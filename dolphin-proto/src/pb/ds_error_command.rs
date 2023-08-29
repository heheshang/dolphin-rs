#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsErrorCommandBean {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int32, optional, tag = "2")]
    pub command_type: ::core::option::Option<i32>,
    #[prost(int64, tag = "3")]
    pub process_definition_code: i64,
    #[prost(string, optional, tag = "4")]
    pub command_param: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "5")]
    pub task_depend_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub failure_strategy: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub warning_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub warning_group_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "9")]
    pub schedule_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub start_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "11")]
    pub executor_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "13")]
    pub process_instance_priority: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "14")]
    pub worker_group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "15")]
    pub environment_code: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "16")]
    pub dry_run: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "17")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "18")]
    pub process_instance_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "19")]
    pub process_definition_version: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsErrorCommandBeansRequest {
    /// The parent resource name, for example, "shelves/shelf1"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsErrorCommandBeansResponse {
    /// The field name should match the noun "DsErrorCommandBean" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub ds_error_command_beans: ::prost::alloc::vec::Vec<DsErrorCommandBean>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsErrorCommandBeanRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDsErrorCommandBeanRequest {
    /// The parent resource name where the DsErrorCommandBean is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The DsErrorCommandBean id to use for this DsErrorCommandBean.
    #[prost(string, tag = "2")]
    pub ds_error_command_bean_id: ::prost::alloc::string::String,
    /// The DsErrorCommandBean resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub ds_error_command_bean: ::core::option::Option<DsErrorCommandBean>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDsErrorCommandBeanRequest {
    /// The DsErrorCommandBean resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub ds_error_command_bean: ::core::option::Option<DsErrorCommandBean>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsErrorCommandBeanRequest {
    /// The resource name of the DsErrorCommandBean to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod ds_error_command_bean_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct DsErrorCommandBeanServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsErrorCommandBeanServiceClient<tonic::transport::Channel> {
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
    impl<T> DsErrorCommandBeanServiceClient<T>
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
        ) -> DsErrorCommandBeanServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DsErrorCommandBeanServiceClient::new(InterceptedService::new(inner, interceptor))
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

        pub async fn list_ds_error_command_beans(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDsErrorCommandBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDsErrorCommandBeansResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_error_command.DsErrorCommandBeanService/ListDsErrorCommandBeans",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_error_command.DsErrorCommandBeanService",
                "ListDsErrorCommandBeans",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn get_ds_error_command_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDsErrorCommandBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsErrorCommandBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_error_command.DsErrorCommandBeanService/GetDsErrorCommandBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_error_command.DsErrorCommandBeanService",
                "GetDsErrorCommandBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn create_ds_error_command_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDsErrorCommandBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsErrorCommandBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_error_command.DsErrorCommandBeanService/CreateDsErrorCommandBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_error_command.DsErrorCommandBeanService",
                "CreateDsErrorCommandBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn update_ds_error_command_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDsErrorCommandBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsErrorCommandBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_error_command.DsErrorCommandBeanService/UpdateDsErrorCommandBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_error_command.DsErrorCommandBeanService",
                "UpdateDsErrorCommandBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn delete_ds_error_command_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDsErrorCommandBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_error_command.DsErrorCommandBeanService/DeleteDsErrorCommandBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_error_command.DsErrorCommandBeanService",
                "DeleteDsErrorCommandBean",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ds_error_command_bean_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DsErrorCommandBeanServiceServer.
    #[async_trait]
    pub trait DsErrorCommandBeanService: Send + Sync + 'static {
        async fn list_ds_error_command_beans(
            &self,
            request: tonic::Request<super::ListDsErrorCommandBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDsErrorCommandBeansResponse>,
            tonic::Status,
        >;
        async fn get_ds_error_command_bean(
            &self,
            request: tonic::Request<super::GetDsErrorCommandBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsErrorCommandBean>, tonic::Status>;
        async fn create_ds_error_command_bean(
            &self,
            request: tonic::Request<super::CreateDsErrorCommandBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsErrorCommandBean>, tonic::Status>;
        async fn update_ds_error_command_bean(
            &self,
            request: tonic::Request<super::UpdateDsErrorCommandBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsErrorCommandBean>, tonic::Status>;
        async fn delete_ds_error_command_bean(
            &self,
            request: tonic::Request<super::DeleteDsErrorCommandBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct DsErrorCommandBeanServiceServer<T: DsErrorCommandBeanService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DsErrorCommandBeanService> DsErrorCommandBeanServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }

        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }

        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where F: tonic::service::Interceptor {
            InterceptedService::new(Self::new(inner), interceptor)
        }

        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }

        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }

        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }

        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DsErrorCommandBeanServiceServer<T>
    where
        T: DsErrorCommandBeanService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        type Response = http::Response<tonic::body::BoxBody>;

        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/ds_error_command.DsErrorCommandBeanService/ListDsErrorCommandBeans" => {
                    #[allow(non_camel_case_types)]
                    struct ListDsErrorCommandBeansSvc<T: DsErrorCommandBeanService>(pub Arc<T>);
                    impl<T: DsErrorCommandBeanService>
                        tonic::server::UnaryService<super::ListDsErrorCommandBeansRequest>
                        for ListDsErrorCommandBeansSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::ListDsErrorCommandBeansResponse;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDsErrorCommandBeansRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).list_ds_error_command_beans(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDsErrorCommandBeansSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_error_command.DsErrorCommandBeanService/GetDsErrorCommandBean" => {
                    #[allow(non_camel_case_types)]
                    struct GetDsErrorCommandBeanSvc<T: DsErrorCommandBeanService>(pub Arc<T>);
                    impl<T: DsErrorCommandBeanService>
                        tonic::server::UnaryService<super::GetDsErrorCommandBeanRequest>
                        for GetDsErrorCommandBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsErrorCommandBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDsErrorCommandBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).get_ds_error_command_bean(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDsErrorCommandBeanSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_error_command.DsErrorCommandBeanService/CreateDsErrorCommandBean" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDsErrorCommandBeanSvc<T: DsErrorCommandBeanService>(pub Arc<T>);
                    impl<T: DsErrorCommandBeanService>
                        tonic::server::UnaryService<super::CreateDsErrorCommandBeanRequest>
                        for CreateDsErrorCommandBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsErrorCommandBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDsErrorCommandBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).create_ds_error_command_bean(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateDsErrorCommandBeanSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_error_command.DsErrorCommandBeanService/UpdateDsErrorCommandBean" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDsErrorCommandBeanSvc<T: DsErrorCommandBeanService>(pub Arc<T>);
                    impl<T: DsErrorCommandBeanService>
                        tonic::server::UnaryService<super::UpdateDsErrorCommandBeanRequest>
                        for UpdateDsErrorCommandBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsErrorCommandBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDsErrorCommandBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).update_ds_error_command_bean(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateDsErrorCommandBeanSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ds_error_command.DsErrorCommandBeanService/DeleteDsErrorCommandBean" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDsErrorCommandBeanSvc<T: DsErrorCommandBeanService>(pub Arc<T>);
                    impl<T: DsErrorCommandBeanService>
                        tonic::server::UnaryService<super::DeleteDsErrorCommandBeanRequest>
                        for DeleteDsErrorCommandBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = ();

                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDsErrorCommandBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).delete_ds_error_command_bean(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteDsErrorCommandBeanSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: DsErrorCommandBeanService> Clone for DsErrorCommandBeanServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: DsErrorCommandBeanService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DsErrorCommandBeanService> tonic::server::NamedService
        for DsErrorCommandBeanServiceServer<T>
    {
        const NAME: &'static str = "ds_error_command.DsErrorCommandBeanService";
    }
}
