/// Generated client implementations.
pub mod cerbos_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CerbosServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CerbosServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CerbosServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
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
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn check_resource_set(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::CheckResourceSetRequest,
            >,
        ) -> Result<
                tonic::Response<
                    super::super::super::response::v1::CheckResourceSetResponse,
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
                "/cerbos.svc.v1.CerbosService/CheckResourceSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn check_resource_batch(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::CheckResourceBatchRequest,
            >,
        ) -> Result<
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn check_resources(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::CheckResourcesRequest,
            >,
        ) -> Result<
                tonic::Response<
                    super::super::super::response::v1::CheckResourcesResponse,
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
                "/cerbos.svc.v1.CerbosService/CheckResources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn server_info(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::ServerInfoRequest,
            >,
        ) -> Result<
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn resources_query_plan(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::ResourcesQueryPlanRequest,
            >,
        ) -> Result<
                tonic::Response<
                    super::super::super::response::v1::ResourcesQueryPlanResponse,
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
                "/cerbos.svc.v1.CerbosService/ResourcesQueryPlan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod cerbos_admin_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CerbosAdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CerbosAdminServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CerbosAdminServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
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
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn add_or_update_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::AddOrUpdatePolicyRequest,
            >,
        ) -> Result<
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_policies(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::ListPoliciesRequest,
            >,
        ) -> Result<
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::GetPolicyRequest,
            >,
        ) -> Result<
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_audit_log_entries(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::ListAuditLogEntriesRequest,
            >,
        ) -> Result<
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
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn add_or_update_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::AddOrUpdateSchemaRequest,
            >,
        ) -> Result<
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_schemas(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::ListSchemasRequest,
            >,
        ) -> Result<
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::GetSchemaRequest,
            >,
        ) -> Result<
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::DeleteSchemaRequest,
            >,
        ) -> Result<
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn reload_store(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::ReloadStoreRequest,
            >,
        ) -> Result<
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
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod cerbos_playground_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CerbosPlaygroundServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CerbosPlaygroundServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CerbosPlaygroundServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
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
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn playground_validate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::PlaygroundValidateRequest,
            >,
        ) -> Result<
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn playground_test(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::PlaygroundTestRequest,
            >,
        ) -> Result<
                tonic::Response<
                    super::super::super::response::v1::PlaygroundTestResponse,
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
                "/cerbos.svc.v1.CerbosPlaygroundService/PlaygroundTest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn playground_evaluate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::PlaygroundEvaluateRequest,
            >,
        ) -> Result<
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn playground_proxy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::request::v1::PlaygroundProxyRequest,
            >,
        ) -> Result<
                tonic::Response<
                    super::super::super::response::v1::PlaygroundProxyResponse,
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
                "/cerbos.svc.v1.CerbosPlaygroundService/PlaygroundProxy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
