use std::time::Duration;

use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint};

use crate::genpb::cerbos::{
    request::v1::CheckResourcesRequest, svc::v1::cerbos_service_client::CerbosServiceClient,
};

use self::model::ProtobufWrapper;

pub mod model;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = StdError> = std::result::Result<T, E>;

pub struct CerbosClientOptions {
    host: &'static str,
    tls_config: Option<ClientTlsConfig>,
    timeout: Duration,
}

impl CerbosClientOptions {
    pub fn new(host: &'static str) -> Self {
        Self {
            host,
            tls_config: Some(ClientTlsConfig::new()),
            timeout: Duration::from_secs(2),
        }
    }

    pub fn with_plaintext(mut self) -> Self {
        self.tls_config = None;
        self
    }

    pub fn with_timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }

    pub fn with_tls_domain_name(mut self, domain: impl Into<String>) -> Self {
        self.tls_config = self
            .tls_config
            .or_else(|| Some(ClientTlsConfig::new()))
            .map(|c| c.domain_name(domain));
        self
    }

    pub fn with_tls_ca_cert_pem(mut self, pem: impl AsRef<[u8]>) -> Self {
        let cert = Certificate::from_pem(pem);

        self.tls_config = self
            .tls_config
            .or_else(|| Some(ClientTlsConfig::new()))
            .map(|c| c.ca_certificate(cert));
        self
    }

    pub(crate) fn build_endpoint(self) -> Result<Endpoint> {
        let channel = Channel::from_static(&self.host).timeout(self.timeout);
        match self.tls_config {
            Some(tc) => Ok(channel.tls_config(tc)?),
            None => Ok(channel),
        }
    }
}

pub struct CerbosAsyncClient {
    stub: CerbosServiceClient<Channel>,
}

impl CerbosAsyncClient {
    pub async fn new(conf: CerbosClientOptions) -> Result<Self> {
        let endpoint = conf.build_endpoint()?;
        let channel = endpoint.connect_lazy();
        Ok(Self {
            stub: CerbosServiceClient::new(channel),
        })
    }

    pub async fn check_resources(
        &mut self,
        principal: model::Principal,
        resources: model::ResourceList,
        aux_data: Option<model::AuxData>,
    ) -> Result<model::CheckResourcesResponse> {
        let mut req = CheckResourcesRequest::default();
        req.request_id = "test".to_string();
        req.principal = Some(principal.to_pb());
        req.resources = resources.resources;
        req.aux_data = aux_data.map(|a| a.to_pb());

        let resp = self.stub.check_resources(req).await?;

        Ok(model::CheckResourcesResponse {
            response: resp.get_ref().to_owned(),
        })
    }
}
