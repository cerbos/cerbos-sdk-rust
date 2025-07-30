// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use rcgen::{
    BasicConstraints, Certificate, CertificateParams, DnType, ExtendedKeyUsagePurpose, IsCa,
    Issuer, KeyPair, KeyUsagePurpose,
};
use std::fs;
use tempfile::TempDir;
use time::{Duration, OffsetDateTime};

pub struct CerbosTestTlsConfig<'a> {
    ca_cert: Certificate,
    temp_dir: &'a TempDir,
}

impl<'a> CerbosTestTlsConfig<'a> {
    pub const CERT_NAME: &'static str = "server.crt";
    pub const CERT_KEY: &'static str = "server.key";

    pub fn new(hostname: impl Into<String>, temp_dir: &'a TempDir) -> Result<Self> {
        let (ca_cert, issuer) = Self::new_ca()?;
        let (cert, key) = Self::new_end_entity(hostname.into(), &issuer)?;

        let cert_path = temp_dir.path().join(Self::CERT_NAME);
        fs::write(&cert_path, cert.pem()).context("Failed to write server certificate")?;

        let key_path = temp_dir.path().join(Self::CERT_KEY);
        fs::write(&key_path, key.serialize_pem()).context("Failed to write server private key")?;

        let ca_cert_path = temp_dir.path().join("ca.crt");
        let _ = fs::write(&ca_cert_path, ca_cert.pem());

        Ok(Self { ca_cert, temp_dir })
    }
    pub fn get_ca_cert(&self) -> tonic::transport::Certificate {
        tonic::transport::Certificate::from_pem(self.ca_cert.pem())
    }
    fn new_ca() -> anyhow::Result<(Certificate, Issuer<'static, KeyPair>)> {
        let mut params = CertificateParams::new([])?;
        let (yesterday, tomorrow) = Self::validity_period();
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        params.distinguished_name.push(DnType::CountryName, "AU");
        params
            .distinguished_name
            .push(DnType::OrganizationName, "Cerbos crabs");
        params.key_usages.push(KeyUsagePurpose::DigitalSignature);
        params.key_usages.push(KeyUsagePurpose::KeyCertSign);
        params.key_usages.push(KeyUsagePurpose::CrlSign);

        params.not_before = yesterday;
        params.not_after = tomorrow;

        let key_pair = KeyPair::generate()?;
        let cert = params.self_signed(&key_pair)?;

        Ok((cert, Issuer::new(params, key_pair)))
    }
    fn new_end_entity(
        name: String,
        issuer: &Issuer<'static, KeyPair>,
    ) -> anyhow::Result<(Certificate, KeyPair)> {
        let mut params = CertificateParams::new([name.clone()])?;
        let (yesterday, tomorrow) = Self::validity_period();
        params.distinguished_name.push(DnType::CommonName, name);
        params.use_authority_key_identifier_extension = true;
        params.key_usages.push(KeyUsagePurpose::DigitalSignature);
        params
            .extended_key_usages
            .push(ExtendedKeyUsagePurpose::ServerAuth);
        params.not_before = yesterday;
        params.not_after = tomorrow;

        let key_pair = KeyPair::generate()?;
        Ok((params.signed_by(&key_pair, issuer)?, key_pair))
    }
    fn validity_period() -> (OffsetDateTime, OffsetDateTime) {
        let day = Duration::days(1);
        let yesterday = OffsetDateTime::now_utc().checked_sub(day).unwrap();
        let tomorrow = OffsetDateTime::now_utc().checked_add(day).unwrap();
        (yesterday, tomorrow)
    }
    pub fn get_temp_dir(&self) -> &TempDir {
        self.temp_dir
    }
}
