// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0
use std::time::Duration;

use tokio::{
    net::UnixStream,
    runtime::{Builder, Runtime},
};
use tonic::{
    codegen::InterceptedService,
    metadata::Ascii,
    metadata::MetadataValue,
    service::Interceptor,
    transport::{Certificate, Channel, ClientTlsConfig, Uri},
    Request, Status,
};
use tower::service_fn;
use uuid::Uuid;

use crate::genpb::cerbos::{
    request::v1::{CheckResourcesRequest, PlanResourcesRequest},
    svc::v1::cerbos_service_client::CerbosServiceClient,
};

use self::model::{ProtobufWrapper, Resource, ResourceList};
use anyhow::Context;
use hyper_util::rt::TokioIo;

pub mod attr;
pub mod auth;
pub mod auth_client;
#[cfg(feature = "testcontainers")]
pub mod container;
pub mod hub;
pub mod model;
pub mod store;

pub type Result<T> = anyhow::Result<T>;

/// Cerbos gRPC endpoint kind.
#[derive(Debug)]
pub enum CerbosEndpoint<S>
where
    S: Into<String> + Send,
{
    HostPort(S, u16),
    UnixDomainSocket(S),
}

/// Options for constructing the Cerbos client.
pub struct CerbosClientOptions<S>
where
    S: Into<String> + Send,
{
    endpoint: CerbosEndpoint<S>,
    tls_config: Option<ClientTlsConfig>,
    timeout: Duration,
    request_id_gen: fn() -> String,
    playground_instance: Option<String>,
    user_agent: String,
}

impl<S> CerbosClientOptions<S>
where
    S: Into<String> + Send,
{
    pub fn new(endpoint: CerbosEndpoint<S>) -> Self {
        Self {
            endpoint,
            tls_config: Some(ClientTlsConfig::new()),
            timeout: Duration::from_secs(2),
            request_id_gen: gen_uuid,
            playground_instance: None,
            user_agent: "cerbos-rs".to_string(),
        }
    }

    /// Disable TLS
    pub fn with_plaintext(mut self) -> Self {
        self.tls_config = None;
        self
    }

    /// Set timeout for API calls
    pub fn with_timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }

    /// Domain name in the TLS certificate.
    pub fn with_tls_domain_name(mut self, domain: impl Into<String>) -> Self {
        self.tls_config = self
            .tls_config
            .or_else(|| Some(ClientTlsConfig::new()))
            .map(|c| c.domain_name(domain));
        self
    }

    /// CA cert to verify the server TLS certificate.
    pub fn with_tls_ca_cert_pem(mut self, pem: impl AsRef<[u8]>) -> Self {
        let cert = Certificate::from_pem(pem);

        self.tls_config = self
            .tls_config
            .or_else(|| Some(ClientTlsConfig::new()))
            .map(|c| c.ca_certificate(cert));
        self
    }

    /// Request ID generator to use. Defaults to UUID.
    pub fn with_request_id_gen(mut self, id_gen: fn() -> String) -> Self {
        self.request_id_gen = id_gen;
        self
    }

    /// Configure the client to use the Cerbos playground.
    pub fn with_playground_instance(mut self, id: impl Into<String>) -> Self {
        self.playground_instance = Some(id.into());
        self
    }

    /// Set a custom user agent for the client.
    pub fn with_user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = ua.into();
        self
    }

    pub(crate) fn build_channel(self) -> Result<Channel> {
        match self.endpoint {
            CerbosEndpoint::HostPort(host, port) => {
                let protocol = self.tls_config.as_ref().map_or_else(|| "http", |_| "https");
                let endpoint_addr = format!("{}://{}:{}", protocol, host.into(), port);
                let mut endpoint = Channel::from_shared(endpoint_addr.clone())
                    .with_context(|| format!("Failed to create channel for {}", endpoint_addr))?
                    .connect_timeout(self.timeout)
                    .timeout(self.timeout)
                    .user_agent(self.user_agent.clone())
                    .with_context(|| "Failed to create channel")?;

                endpoint = match self.tls_config {
                    Some(tc) => endpoint
                        .tls_config(tc)
                        .with_context(|| "Failed to create TLS configuration")?,
                    None => endpoint,
                };

                Ok(endpoint.connect_lazy())
            }
            CerbosEndpoint::UnixDomainSocket(path) => {
                let mut endpoint = Channel::from_static("https://127.0.0.1:3593")
                    .connect_timeout(self.timeout)
                    .timeout(self.timeout)
                    .user_agent(self.user_agent.clone())
                    .with_context(|| "Failed to create channel")?;

                endpoint = match self.tls_config {
                    Some(tc) => endpoint
                        .tls_config(tc)
                        .with_context(|| "Failed to create TLS configuration")?,
                    None => endpoint,
                };

                let uds: &'static str = Box::leak(path.into().into_boxed_str());
                let connect = move |_: Uri| async {
                    UnixStream::connect(uds.to_string()).await.map(TokioIo::new)
                };
                Ok(endpoint.connect_with_connector_lazy(service_fn(connect)))
            }
        }
    }
}

/// Asynchronous Cerbos client
pub struct CerbosAsyncClient {
    stub: CerbosServiceClient<InterceptedService<Channel, CerbosInterceptor>>,
    request_id_gen: fn() -> String,
}

impl CerbosAsyncClient {
    /// Create a new Cerbos client using client options
    pub async fn new<S>(conf: CerbosClientOptions<S>) -> Result<Self>
    where
        S: Into<String> + Send,
    {
        let playground_instance = match conf.playground_instance {
            Some(ref instance) => Some(instance.parse()?),
            None => None,
        };

        let request_timeout = conf.timeout;
        let request_id_gen = conf.request_id_gen;
        let channel = conf.build_channel()?;
        let stub = CerbosServiceClient::with_interceptor(
            channel,
            CerbosInterceptor {
                playground_instance,
                request_timeout,
            },
        );

        Ok(Self {
            stub,
            request_id_gen,
        })
    }

    /// Check access to multiple resources
    pub async fn check_resources(
        &mut self,
        principal: model::Principal,
        resources: model::ResourceList,
        aux_data: Option<model::AuxData>,
    ) -> Result<model::CheckResourcesResponse> {
        let req = CheckResourcesRequest {
            request_id: (self.request_id_gen)(),
            principal: Some(principal.to_pb()),
            resources: resources.resources,
            aux_data: aux_data.map(|a| a.to_pb()),
            ..Default::default()
        };

        let resp = self
            .stub
            .check_resources(req)
            .await
            .with_context(|| "CheckResources call failed")?;

        Ok(model::CheckResourcesResponse {
            response: resp.get_ref().to_owned(),
        })
    }

    /// Check access to a single resource
    pub async fn is_allowed<S>(
        &mut self,
        action: S,
        principal: model::Principal,
        resource: Resource,
        aux_data: Option<model::AuxData>,
    ) -> Result<bool>
    where
        S: Into<String> + Clone,
    {
        let resp = self
            .check_resources(
                principal,
                ResourceList::new().add(resource, [action.clone()]),
                aux_data,
            )
            .await?;
        Ok(resp
            .iter()
            .next()
            .map(|r| r.is_allowed(action.into()))
            .unwrap_or(false))
    }

    /// Produce a query plan for selecting resources that the principal can perform the given
    /// action on.
    pub async fn plan_resources<S>(
        &mut self,
        action: S,
        principal: model::Principal,
        resource: model::ResourceKind,
        aux_data: Option<model::AuxData>,
    ) -> Result<model::PlanResourcesResponse>
    where
        S: Into<String> + Clone,
    {
        let req = PlanResourcesRequest {
            request_id: (self.request_id_gen)(),
            action: action.into(),
            principal: Some(principal.to_pb()),
            resource: Some(resource.to_pb()),
            aux_data: aux_data.map(|a| a.to_pb()),
            ..Default::default()
        };

        let resp = self
            .stub
            .plan_resources(req)
            .await
            .with_context(|| "PlanResources call failed")?;

        Ok(model::PlanResourcesResponse {
            response: resp.get_ref().to_owned(),
        })
    }
}

pub struct CerbosSyncClient {
    runtime: Runtime,
    client: CerbosAsyncClient,
}

impl CerbosSyncClient {
    pub fn new<S>(conf: CerbosClientOptions<S>) -> Result<Self>
    where
        S: Into<String> + Send,
    {
        let runtime = Builder::new_multi_thread().enable_all().build()?;
        let client = runtime.block_on(CerbosAsyncClient::new(conf))?;
        Ok(Self { runtime, client })
    }

    pub fn check_resources(
        &mut self,
        principal: model::Principal,
        resources: model::ResourceList,
        aux_data: Option<model::AuxData>,
    ) -> Result<model::CheckResourcesResponse> {
        self.runtime
            .block_on(self.client.check_resources(principal, resources, aux_data))
    }

    pub fn is_allowed<S>(
        &mut self,
        action: S,
        principal: model::Principal,
        resource: Resource,
        aux_data: Option<model::AuxData>,
    ) -> Result<bool>
    where
        S: Into<String> + Clone,
    {
        self.runtime.block_on(
            self.client
                .is_allowed(action, principal, resource, aux_data),
        )
    }

    pub fn plan_resources<S>(
        &mut self,
        action: S,
        principal: model::Principal,
        resource: model::ResourceKind,
        aux_data: Option<model::AuxData>,
    ) -> Result<model::PlanResourcesResponse>
    where
        S: Into<String> + Clone,
    {
        self.runtime.block_on(
            self.client
                .plan_resources(action, principal, resource, aux_data),
        )
    }
}

fn gen_uuid() -> String {
    Uuid::new_v4().hyphenated().to_string()
}

struct CerbosInterceptor {
    request_timeout: Duration,
    playground_instance: Option<MetadataValue<Ascii>>,
}

impl Interceptor for CerbosInterceptor {
    fn call(&mut self, mut request: Request<()>) -> std::result::Result<Request<()>, Status> {
        if let Some(ref playground_md) = self.playground_instance {
            request
                .metadata_mut()
                .insert("playground-instance", playground_md.clone());
        }

        request.set_timeout(self.request_timeout);
        Ok(request)
    }
}
