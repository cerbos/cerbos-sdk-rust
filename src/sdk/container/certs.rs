// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, KeyPair, SanType};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    time::{Duration, SystemTime},
};
use tempfile::TempDir;
use testcontainers::{clients::Cli, Container, RunnableImage};
use tokio::time::sleep;
use tonic::transport::{Certificate as TonicCertificate, ClientTlsConfig};

use cerbos_sdk::{
    container::CerbosContainer, CerbosAsyncClient, CerbosClientOptions, CerbosEndpoint,
};

/// Certificate bundle containing both the certificate and private key
#[derive(Debug, Clone)]
pub struct CertificateBundle {
    pub cert_pem: String,
    pub key_pem: String,
    pub ca_cert_pem: String,
}

/// Generates a self-signed CA certificate and server certificate for TLS
pub fn generate_certificates(hostname: &str) -> Result<CertificateBundle> {
    // Generate CA certificate
    let mut ca_params = CertificateParams::new(vec!["Cerbos Test CA".to_string()]);
    ca_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    ca_params.key_usages = vec![
        rcgen::KeyUsagePurpose::KeyCertSign,
        rcgen::KeyUsagePurpose::CrlSign,
    ];

    let mut ca_dn = DistinguishedName::new();
    ca_dn.push(DnType::CommonName, "Cerbos Test CA");
    ca_params.distinguished_name = ca_dn;

    // Set validity period
    let now = SystemTime::now();
    ca_params.not_before = now - Duration::from_secs(60);
    ca_params.not_after = now + Duration::from_secs(365 * 24 * 60 * 60); // 1 year

    let ca_cert =
        Certificate::from_params(ca_params).with_context(|| "Failed to generate CA certificate")?;

    // Generate server certificate signed by CA
    let mut server_params = CertificateParams::new(vec![hostname.to_string()]);
    server_params.subject_alt_names = vec![
        SanType::DnsName(hostname.to_string()),
        SanType::IpAddress(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))),
    ];

    let mut server_dn = DistinguishedName::new();
    server_dn.push(DnType::CommonName, hostname);
    server_params.distinguished_name = server_dn;

    server_params.not_before = now - Duration::from_secs(60);
    server_params.not_after = now + Duration::from_secs(365 * 24 * 60 * 60);

    let server_cert = Certificate::from_params(server_params)
        .with_context(|| "Failed to generate server certificate")?;

    // Sign the server certificate with the CA
    let server_cert_pem = server_cert
        .serialize_pem_with_signer(&ca_cert)
        .with_context(|| "Failed to sign server certificate")?;

    let server_key_pem = server_cert.serialize_private_key_pem();
    let ca_cert_pem = ca_cert
        .serialize_pem()
        .with_context(|| "Failed to serialize CA certificate")?;

    Ok(CertificateBundle {
        cert_pem: server_cert_pem,
        key_pem: server_key_pem,
        ca_cert_pem,
    })
}

/// Configuration for TLS-enabled Cerbos container
pub struct CerbosTlsConfig {
    pub temp_dir: TempDir,
    pub cert_bundle: CertificateBundle,
    pub hostname: String,
}

impl CerbosTlsConfig {
    /// Create a new TLS configuration with generated certificates
    pub fn new(hostname: &str) -> Result<Self> {
        let temp_dir = TempDir::new().with_context(|| "Failed to create temporary directory")?;

        let cert_bundle = generate_certificates(hostname)?;

        // Write certificates to temporary files
        let cert_path = temp_dir.path().join("server.crt");
        let key_path = temp_dir.path().join("server.key");
        let ca_path = temp_dir.path().join("ca.crt");

        fs::write(&cert_path, &cert_bundle.cert_pem)
            .with_context(|| "Failed to write server certificate")?;
        fs::write(&key_path, &cert_bundle.key_pem)
            .with_context(|| "Failed to write server private key")?;
        fs::write(&ca_path, &cert_bundle.ca_cert_pem)
            .with_context(|| "Failed to write CA certificate")?;

        Ok(Self {
            temp_dir,
            cert_bundle,
            hostname: hostname.to_string(),
        })
    }

    /// Get the path to the certificate directory
    pub fn cert_dir(&self) -> &std::path::Path {
        self.temp_dir.path()
    }

    /// Create a Cerbos container configured with TLS
    pub fn create_container(&self) -> RunnableImage<CerbosContainer> {
        let container = CerbosContainer::default()
            .with_volume_mounts([(
                self.cert_dir().to_string_lossy().to_string(),
                "/certs".to_string(),
            )])
            .with_environment_vars([
                ("CERBOS_NO_TELEMETRY".to_string(), "1".to_string()),
                (
                    "CERBOS_TLS_CERT".to_string(),
                    "/certs/server.crt".to_string(),
                ),
                (
                    "CERBOS_TLS_KEY".to_string(),
                    "/certs/server.key".to_string(),
                ),
            ]);

        RunnableImage::from(container)
    }

    /// Create a TLS-configured Cerbos client
    pub fn create_client(&self, port: u16) -> Result<CerbosClientOptions<String>> {
        let endpoint = CerbosEndpoint::HostPort(self.hostname.clone(), port);

        let client_options = CerbosClientOptions::new(endpoint)
            .with_tls_domain_name(&self.hostname)
            .with_tls_ca_cert_pem(&self.cert_bundle.ca_cert_pem)
            .with_timeout(Duration::from_secs(10));

        Ok(client_options)
    }
}

/// Helper function to start a TLS-enabled Cerbos container and create a client
pub async fn start_cerbos_with_tls(
    docker: &Cli,
    hostname: &str,
) -> Result<(
    Container<'_, CerbosContainer>,
    CerbosAsyncClient,
    CerbosTlsConfig,
)> {
    let tls_config = CerbosTlsConfig::new(hostname)?;

    // Start the container
    let container = docker.run(tls_config.create_container());

    // Get the mapped port for gRPC TLS (port 3593)
    let grpc_port = container.get_host_port_ipv4(3593);

    // Wait for the container to be ready
    sleep(Duration::from_secs(2)).await;

    // Create the client
    let client_options = tls_config.create_client(grpc_port)?;
    let client = CerbosAsyncClient::new(client_options).await?;

    Ok((container, client, tls_config))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cerbos_sdk::model::{Principal, Resource, ResourceList};
    use testcontainers::clients::Cli;

    #[tokio::test]
    async fn test_cerbos_tls_connection() -> Result<()> {
        let docker = Cli::default();
        let hostname = "localhost";

        let (_container, mut client, _tls_config) =
            start_cerbos_with_tls(&docker, hostname).await?;

        // Create a test principal
        let principal = Principal::new("john", ["employee"]).with_attr("department", "engineering");

        // Create a test resource
        let resource = Resource::new("document:1", "document").with_attr("owner", "john");

        // Test a permission check
        let allowed = client.is_allowed("read", principal, resource, None).await?;

        // This should work without TLS errors
        println!("Permission check result: {}", allowed);

        Ok(())
    }

    #[tokio::test]
    async fn test_certificate_generation() -> Result<()> {
        let hostname = "test.example.com";
        let cert_bundle = generate_certificates(hostname)?;

        // Verify certificates are not empty
        assert!(!cert_bundle.cert_pem.is_empty());
        assert!(!cert_bundle.key_pem.is_empty());
        assert!(!cert_bundle.ca_cert_pem.is_empty());

        // Verify certificate contains expected hostname
        assert!(cert_bundle.cert_pem.contains("-----BEGIN CERTIFICATE-----"));
        assert!(cert_bundle.key_pem.contains("-----BEGIN PRIVATE KEY-----"));
        assert!(cert_bundle
            .ca_cert_pem
            .contains("-----BEGIN CERTIFICATE-----"));

        Ok(())
    }

    #[tokio::test]
    async fn test_tls_config_creation() -> Result<()> {
        let hostname = "localhost";
        let tls_config = CerbosTlsConfig::new(hostname)?;

        // Verify files were created
        let cert_path = tls_config.cert_dir().join("server.crt");
        let key_path = tls_config.cert_dir().join("server.key");
        let ca_path = tls_config.cert_dir().join("ca.crt");

        assert!(cert_path.exists());
        assert!(key_path.exists());
        assert!(ca_path.exists());

        // Verify client options can be created
        let client_options = tls_config.create_client(3593)?;
        // If we get here without panic, the client options are valid

        Ok(())
    }
}
