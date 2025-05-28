// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use super::store::StoreClient;
use anyhow::{Context, Result};
use std::env;
use std::time::Duration;
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};
/// Hub client for interacting with Cerbos Hub services
pub struct HubClient {
    channel: Channel,
}

impl HubClient {
    /// Create a new Hub client with the given endpoint
    pub async fn new(endpoint: impl Into<String>) -> Result<Self> {
        let endpoint = endpoint.into();
        let channel = Endpoint::from_shared(endpoint.clone())
            .with_context(|| format!("Failed to create endpoint for {}", endpoint))?
            .tls_config(ClientTlsConfig::new())
            .with_context(|| "Failed to create TLS configuration")?
            .connect_timeout(Duration::from_secs(30))
            .timeout(Duration::from_secs(30))
            .connect()
            .await
            .with_context(|| format!("Failed to connect to {}", endpoint))?;

        Ok(Self { channel })
    }

    /// Create a new Hub client with custom TLS configuration
    pub async fn new_with_tls(
        endpoint: impl Into<String>,
        tls_config: ClientTlsConfig,
    ) -> Result<Self> {
        let endpoint = endpoint.into();
        let channel = Endpoint::from_shared(endpoint.clone())
            .with_context(|| format!("Failed to create endpoint for {}", endpoint))?
            .tls_config(tls_config)
            .with_context(|| "Failed to apply TLS configuration")?
            .connect_timeout(Duration::from_secs(30))
            .timeout(Duration::from_secs(30))
            .connect()
            .await
            .with_context(|| format!("Failed to connect to {}", endpoint))?;

        Ok(Self { channel })
    }

    /// Create a new Hub client without TLS (for testing)
    pub async fn new_insecure(endpoint: impl Into<String>) -> Result<Self> {
        let endpoint = endpoint.into();
        let channel = Endpoint::from_shared(endpoint.clone())
            .with_context(|| format!("Failed to create endpoint for {}", endpoint))?
            .connect_timeout(Duration::from_secs(30))
            .timeout(Duration::from_secs(30))
            .connect()
            .await
            .with_context(|| format!("Failed to connect to {}", endpoint))?;

        Ok(Self { channel })
    }

    /// Get a store client for file operations
    pub fn store_client(&self) -> StoreClient {
        StoreClient::new(self.channel.clone())
    }
}

/// Builder for creating Hub clients with custom configuration
pub struct HubClientBuilder {
    endpoint: String,
    client_id: String,
    client_secret: String,
    connect_timeout: Duration,
    request_timeout: Duration,
}

impl HubClientBuilder {
    pub fn new() -> Result<Self> {
        Ok(Self {
            endpoint: "https://api.cerbos.cloud".to_string(),
            client_id: env::var("CERBOS_HUB_CLIENT_ID")?,
            client_secret: env::var("CERBOS_HUB_CLIENT_SECRET")?,
            connect_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
        })
    }
    pub fn with_api_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    pub fn with_client_credentials(
        mut self,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        self.client_id = client_id.into();
        self.client_secret = client_secret.into();
        self
    }
    /// Set connection timeout
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set request timeout
    pub fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Build the Hub client
    pub async fn build(self) -> Result<HubClient> {
        let endpoint = Endpoint::from_shared(self.endpoint.clone())
            .with_context(|| format!("Failed to create endpoint for {}", self.endpoint))?
            .tls_config(ClientTlsConfig::new().with_native_roots())
            .with_context(|| "Failed to apply TLS configuration")?
            .connect_timeout(self.connect_timeout)
            .timeout(self.request_timeout);

        let channel = endpoint
            .connect()
            .await
            .with_context(|| format!("Failed to connect to {}", self.endpoint))?;

        Ok(HubClient { channel })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hub_client_builder() -> Result<()> {
        // This test would require a running Hub instance to connect to
        // For now, just test that the builder constructs properly
        let builder = HubClientBuilder::new()?
            .with_connect_timeout(Duration::from_secs(10))
            .with_request_timeout(Duration::from_secs(5));

        // We can't actually connect without a real endpoint, but we can verify the builder works
        assert_eq!(builder.connect_timeout, Duration::from_secs(10));
        assert_eq!(builder.request_timeout, Duration::from_secs(5));
        Ok(())
    }
}
