use crate::genpb::cerbos::cloud::apikey::v1::{
    api_key_service_client::ApiKeyServiceClient, IssueAccessTokenRequest,
};
use crate::sdk::hub::Credentials;
use anyhow::Result;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tonic::transport::Channel;
use tonic::Request;

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

    pub async fn has_valid_token(&self) -> bool {
        let token_guard = self.token_info.read().await;
        token_guard
            .as_ref()
            .map(|token_info| token_info.expires_at > Instant::now())
            .unwrap_or(false)
    }
    pub async fn current_token(&self) -> Option<String> {
        let token_guard = self.token_info.read().await;
        token_guard
            .as_ref()
            .filter(|token_info| token_info.expires_at > Instant::now())
            .map(|token_info| token_info.token.clone())
    }

    pub async fn time_until_expiry(&self) -> Option<Duration> {
        let token_guard = self.token_info.read().await;
        token_guard.as_ref().and_then(|token_info| {
            let now = Instant::now();
            if token_info.expires_at > now {
                Some(token_info.expires_at - now)
            } else {
                None
            }
        })
    }
}

// Clone implementation for use in middleware
impl Clone for AuthClient {
    fn clone(&self) -> Self {
        Self {
            api_key_client: self.api_key_client.clone(),
            credentials: self.credentials.clone(),
            token_info: self.token_info.clone(),
        }
    }
}
