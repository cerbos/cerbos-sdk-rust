// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use super::store::StoreClient;
use anyhow::{Context, Result};
use std::env;
use std::time::Duration;
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};

pub struct HubClient {
    channel: Channel,
}

impl HubClient {
    pub fn store_client(&self) -> StoreClient {
        StoreClient::new(self.channel.clone())
    }
}

pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
}

impl Credentials {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
        }
    }
}

pub struct HubClientBuilder {
    endpoint: String,
    credentials: Credentials,
    connect_timeout: Duration,
    request_timeout: Duration,
}

impl HubClientBuilder {
    pub fn new() -> Result<Self> {
        Ok(Self {
            endpoint: "https://api.cerbos.cloud".to_string(),
            credentials: Credentials {
                client_id: env::var("CERBOS_HUB_CLIENT_ID")?,
                client_secret: env::var("CERBOS_HUB_CLIENT_SECRET")?,
            },
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
        self.credentials = Credentials::new(client_id.into(), client_secret.into());
        self
    }

    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    pub fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

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
