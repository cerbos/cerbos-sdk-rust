/// Generated client implementations.
pub mod cerbos_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CerbosServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CerbosServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CerbosServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CerbosServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CerbosServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn check_resource_set(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::CheckResourceSetRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::CheckResourceSetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosService/CheckResourceSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cerbos.svc.v1.CerbosService", "CheckResourceSet"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn check_resource_batch(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::CheckResourceBatchRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::response::v1::CheckResourceBatchResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosService/CheckResourceBatch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cerbos.svc.v1.CerbosService", "CheckResourceBatch"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn check_resources(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::CheckResourcesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::CheckResourcesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosService/CheckResources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cerbos.svc.v1.CerbosService", "CheckResources"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn server_info(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::ServerInfoRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::ServerInfoResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosService/ServerInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cerbos.svc.v1.CerbosService", "ServerInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn plan_resources(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::PlanResourcesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::PlanResourcesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosService/PlanResources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cerbos.svc.v1.CerbosService", "PlanResources"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod cerbos_admin_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CerbosAdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CerbosAdminServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CerbosAdminServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CerbosAdminServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CerbosAdminServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn add_or_update_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::AddOrUpdatePolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::response::v1::AddOrUpdatePolicyResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosAdminService/AddOrUpdatePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cerbos.svc.v1.CerbosAdminService",
                        "AddOrUpdatePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_policies(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::ListPoliciesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::ListPoliciesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosAdminService/ListPolicies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cerbos.svc.v1.CerbosAdminService", "ListPolicies"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::GetPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::GetPolicyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosAdminService/GetPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cerbos.svc.v1.CerbosAdminService", "GetPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn disable_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::DisablePolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::DisablePolicyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosAdminService/DisablePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cerbos.svc.v1.CerbosAdminService", "DisablePolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn enable_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::EnablePolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::EnablePolicyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosAdminService/EnablePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cerbos.svc.v1.CerbosAdminService", "EnablePolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_audit_log_entries(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::ListAuditLogEntriesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::response::v1::ListAuditLogEntriesResponse,
                >,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosAdminService/ListAuditLogEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cerbos.svc.v1.CerbosAdminService",
                        "ListAuditLogEntries",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn add_or_update_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::AddOrUpdateSchemaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::response::v1::AddOrUpdateSchemaResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosAdminService/AddOrUpdateSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cerbos.svc.v1.CerbosAdminService",
                        "AddOrUpdateSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_schemas(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::ListSchemasRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::ListSchemasResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosAdminService/ListSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cerbos.svc.v1.CerbosAdminService", "ListSchemas"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::GetSchemaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::GetSchemaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosAdminService/GetSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cerbos.svc.v1.CerbosAdminService", "GetSchema"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::DeleteSchemaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::DeleteSchemaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosAdminService/DeleteSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cerbos.svc.v1.CerbosAdminService", "DeleteSchema"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reload_store(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::ReloadStoreRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::ReloadStoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosAdminService/ReloadStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cerbos.svc.v1.CerbosAdminService", "ReloadStore"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod cerbos_playground_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CerbosPlaygroundServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CerbosPlaygroundServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CerbosPlaygroundServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CerbosPlaygroundServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CerbosPlaygroundServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn playground_validate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::PlaygroundValidateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::response::v1::PlaygroundValidateResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosPlaygroundService/PlaygroundValidate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cerbos.svc.v1.CerbosPlaygroundService",
                        "PlaygroundValidate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn playground_test(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::PlaygroundTestRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::PlaygroundTestResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosPlaygroundService/PlaygroundTest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cerbos.svc.v1.CerbosPlaygroundService",
                        "PlaygroundTest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn playground_evaluate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::PlaygroundEvaluateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::response::v1::PlaygroundEvaluateResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosPlaygroundService/PlaygroundEvaluate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cerbos.svc.v1.CerbosPlaygroundService",
                        "PlaygroundEvaluate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn playground_proxy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::PlaygroundProxyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::response::v1::PlaygroundProxyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cerbos.svc.v1.CerbosPlaygroundService/PlaygroundProxy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cerbos.svc.v1.CerbosPlaygroundService",
                        "PlaygroundProxy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
