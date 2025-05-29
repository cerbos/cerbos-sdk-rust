// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use super::auth::AuthMiddleware;
use super::auth_client::AuthClient;
use super::store::StoreClient;
use anyhow::{Context, Result};
use std::time::Duration;
use std::{env, sync::Arc};
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};
use tower::ServiceBuilder;

pub struct HubClient<T> {
    channel: T,
}
type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

impl<T> HubClient<T>
where
    T: Clone,
    T: tonic::client::GrpcService<tonic::body::Body>,
    T::Error: Into<StdError>,
    T::ResponseBody: http_body::Body<Data = prost::bytes::Bytes> + std::marker::Send + 'static,
    <T::ResponseBody as http_body::Body>::Error: Into<StdError> + std::marker::Send,
{
    pub fn store_client(&self) -> StoreClient<T> {
        StoreClient::new(self.channel.clone())
    }
}

#[derive(Debug)]
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
    credentials: Option<Credentials>,
    connect_timeout: Duration,
    request_timeout: Duration,
}

impl HubClientBuilder {
    pub fn new() -> Self {
        println!("ctor");
        Self {
            endpoint: "https://api.cerbos.cloud".to_string(),
            connect_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            credentials: if let (Ok(id), Ok(secret)) = (
                env::var("CERBOS_HUB_CLIENT_ID"),
                env::var("CERBOS_HUB_CLIENT_SECRET"),
            ) {
                Some(Credentials::new(id, secret))
            } else {
                None
            },
        }
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
        self.credentials = Some(Credentials::new(client_id.into(), client_secret.into()));
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

    pub async fn build(self) -> Result<HubClient<AuthMiddleware>> {
        let endpoint = Endpoint::from_shared(self.endpoint.clone())
            .with_context(|| format!("Failed to create endpoint for {}", self.endpoint))?
            .tls_config(ClientTlsConfig::new().with_native_roots())
            .with_context(|| "Failed to apply TLS configuration")?
            .connect_timeout(self.connect_timeout)
            .timeout(self.request_timeout);

        println!("connect!");
        let channel = endpoint
            .connect()
            .await
            .with_context(|| format!("Failed to connect to {}", self.endpoint))?;

        println!("Credentials {:?}", self.credentials);
        let credentials = Arc::new(self.credentials.with_context(|| "invalid credentials!")?);

        let auth_client = Arc::new(AuthClient::new(channel.clone(), credentials));

        let authenticated_channel = ServiceBuilder::new()
            .layer(tower::layer::layer_fn(move |inner| {
                AuthMiddleware::new(inner, auth_client.clone())
            }))
            .service(channel);
        Ok(HubClient {
            channel: authenticated_channel,
        })
    }
}
