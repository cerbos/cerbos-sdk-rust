use http::{HeaderValue, Request, Response};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tonic::body::Body;
use tonic::transport::Channel;
use tower::Service;

use super::auth_client::AuthClient;

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

impl Service<Request<Body>> for AuthMiddleware {
    type Response = Response<Body>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
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
                "User-Agent",
                HeaderValue::from_str("cerbos-sdk-rust/1.0.0 (linux; x86_64)")?,
            );

            headers.insert(
                AUTH_TOKEN_HEADER,
                HeaderValue::from_str(dbg!(&token))
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?,
            );

            // Forward the request
            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}

// Convenience function to create the middleware layer
pub fn auth_layer(
    auth_client: Arc<AuthClient>,
) -> impl tower::Layer<Channel, Service = AuthMiddleware> {
    tower::layer::layer_fn(move |inner| AuthMiddleware::new(inner, auth_client.clone()))
}
