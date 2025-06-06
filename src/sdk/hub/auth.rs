use crate::genpb::cerbos::cloud::apikey::v1::{
    api_key_service_client::ApiKeyServiceClient, IssueAccessTokenRequest,
};
use crate::sdk::hub::Credentials;
use anyhow::Result;
use http::{HeaderValue, Response};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tonic::body::Body;
use tonic::transport::Channel;
use tonic::Request;
use tower::Service;

const AUTH_TOKEN_HEADER: &str = "x-cerbos-auth";

#[derive(Clone)]
pub struct AuthMiddleware {
    inner: Channel,
    auth_client: Arc<AuthClient>,
}
impl AuthMiddleware {
    pub fn new(inner: Channel, auth_client: Arc<AuthClient>) -> Self {
        AuthMiddleware { inner, auth_client }
    }
}

impl Service<http::Request<Body>> for AuthMiddleware {
    type Response = Response<Body>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut req: http::Request<Body>) -> Self::Future {
        // Clone the service for the async block
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        let auth_client = self.auth_client.clone();

        Box::pin(async move {
            // Get authentication token
            let token = auth_client.authenticate().await?;

            // Add auth header to request
            let headers = req.headers_mut();
            headers.insert(
                AUTH_TOKEN_HEADER,
                HeaderValue::from_str(&token).map_err(|e| Box::new(e) as Self::Error)?,
            );

            // Forward the request
            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}

const EARLY_EXPIRY: Duration = Duration::from_secs(300); // 5 minutes

struct TokenInfo {
    token: String,
    expires_at: Instant,
}

pub struct AuthClient {
    api_key_client: ApiKeyServiceClient<Channel>,
    credentials: Arc<Credentials>,
    token_info: Arc<RwLock<Option<TokenInfo>>>,
}

impl AuthClient {
    pub fn new(channel: Channel, credentials: Arc<Credentials>) -> Self {
        let api_key_client = ApiKeyServiceClient::new(channel);

        Self {
            api_key_client,
            credentials,
            token_info: Arc::new(RwLock::new(None)),
        }
    }

    /// Get a valid authentication token, refreshing if necessary
    pub async fn authenticate(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Try to use existing token first
        {
            let token_guard = self.token_info.read().await;
            if let Some(ref token_info) = *token_guard {
                if token_info.expires_at > Instant::now() {
                    return Ok(token_info.token.clone());
                }
            }
        }

        // Need to get a new token - acquire write lock
        let mut token_guard = self.token_info.write().await;

        // Double-check after acquiring write lock (another thread might have refreshed)
        if let Some(ref token_info) = *token_guard {
            if token_info.expires_at > Instant::now() {
                return Ok(token_info.token.clone());
            }
        }

        let request = Request::new(IssueAccessTokenRequest {
            client_id: self.credentials.client_id.clone(),
            client_secret: self.credentials.client_secret.clone(),
        });
        let response = self
            .api_key_client
            .clone()
            .issue_access_token(request)
            .await;

        let token_response = response?.into_inner();

        let expires_in_duration = token_response
            .expires_in
            .as_ref()
            .map(|duration| Duration::new(duration.seconds as u64, duration.nanos as u32))
            .unwrap_or(Duration::from_secs(3600)); // Default 1 hour

        let mut effective_duration = expires_in_duration;
        if effective_duration > EARLY_EXPIRY {
            effective_duration -= EARLY_EXPIRY;
        }

        // Store the new token
        let token_info = TokenInfo {
            token: token_response.access_token.clone(),
            expires_at: Instant::now() + effective_duration,
        };

        *token_guard = Some(token_info);
        Ok(token_response.access_token)
    }
}
