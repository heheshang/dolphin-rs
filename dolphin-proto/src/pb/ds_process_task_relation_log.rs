#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsProcessTaskRelationLogBean {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "3")]
    pub project_code: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "4")]
    pub process_definition_code: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "5")]
    pub process_definition_version: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "6")]
    pub pre_task_code: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "7")]
    pub pre_task_version: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "8")]
    pub post_task_code: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "9")]
    pub post_task_version: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub condition_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "11")]
    pub condition_params: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "12")]
    pub operator: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "13")]
    pub operate_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsProcessTaskRelationLogBeansRequest {
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
pub struct ListDsProcessTaskRelationLogBeansResponse {
    /// The field name should match the noun "DsProcessTaskRelationLogBean" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub ds_process_task_relation_log_beans: ::prost::alloc::vec::Vec<DsProcessTaskRelationLogBean>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsProcessTaskRelationLogBeanRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDsProcessTaskRelationLogBeanRequest {
    /// The parent resource name where the DsProcessTaskRelationLogBean is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The DsProcessTaskRelationLogBean id to use for this DsProcessTaskRelationLogBean.
    #[prost(string, tag = "2")]
    pub ds_process_task_relation_log_bean_id: ::prost::alloc::string::String,
    /// The DsProcessTaskRelationLogBean resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub ds_process_task_relation_log_bean: ::core::option::Option<DsProcessTaskRelationLogBean>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDsProcessTaskRelationLogBeanRequest {
    /// The DsProcessTaskRelationLogBean resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub ds_process_task_relation_log_bean: ::core::option::Option<DsProcessTaskRelationLogBean>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsProcessTaskRelationLogBeanRequest {
    /// The resource name of the DsProcessTaskRelationLogBean to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod ds_process_task_relation_log_bean_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct DsProcessTaskRelationLogBeanServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsProcessTaskRelationLogBeanServiceClient<tonic::transport::Channel> {
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
    impl<T> DsProcessTaskRelationLogBeanServiceClient<T>
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
        ) -> DsProcessTaskRelationLogBeanServiceClient<InterceptedService<T, F>>
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
            DsProcessTaskRelationLogBeanServiceClient::new(InterceptedService::new(
                inner,
                interceptor,
            ))
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

        pub async fn list_ds_process_task_relation_log_beans(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDsProcessTaskRelationLogBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDsProcessTaskRelationLogBeansResponse>,
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
                "/ds_process_task_relation_log.DsProcessTaskRelationLogBeanService/\
                 ListDsProcessTaskRelationLogBeans",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_process_task_relation_log.DsProcessTaskRelationLogBeanService",
                "ListDsProcessTaskRelationLogBeans",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn get_ds_process_task_relation_log_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDsProcessTaskRelationLogBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsProcessTaskRelationLogBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_process_task_relation_log.DsProcessTaskRelationLogBeanService/\
                 GetDsProcessTaskRelationLogBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_process_task_relation_log.DsProcessTaskRelationLogBeanService",
                "GetDsProcessTaskRelationLogBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn create_ds_process_task_relation_log_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDsProcessTaskRelationLogBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsProcessTaskRelationLogBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_process_task_relation_log.DsProcessTaskRelationLogBeanService/\
                 CreateDsProcessTaskRelationLogBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_process_task_relation_log.DsProcessTaskRelationLogBeanService",
                "CreateDsProcessTaskRelationLogBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn update_ds_process_task_relation_log_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDsProcessTaskRelationLogBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsProcessTaskRelationLogBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_process_task_relation_log.DsProcessTaskRelationLogBeanService/\
                 UpdateDsProcessTaskRelationLogBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_process_task_relation_log.DsProcessTaskRelationLogBeanService",
                "UpdateDsProcessTaskRelationLogBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn delete_ds_process_task_relation_log_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDsProcessTaskRelationLogBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_process_task_relation_log.DsProcessTaskRelationLogBeanService/\
                 DeleteDsProcessTaskRelationLogBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_process_task_relation_log.DsProcessTaskRelationLogBeanService",
                "DeleteDsProcessTaskRelationLogBean",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ds_process_task_relation_log_bean_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DsProcessTaskRelationLogBeanServiceServer.
    #[async_trait]
    pub trait DsProcessTaskRelationLogBeanService: Send + Sync + 'static {
        async fn list_ds_process_task_relation_log_beans(
            &self,
            request: tonic::Request<super::ListDsProcessTaskRelationLogBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDsProcessTaskRelationLogBeansResponse>,
            tonic::Status,
        >;
        async fn get_ds_process_task_relation_log_bean(
            &self,
            request: tonic::Request<super::GetDsProcessTaskRelationLogBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsProcessTaskRelationLogBean>, tonic::Status>;
        async fn create_ds_process_task_relation_log_bean(
            &self,
            request: tonic::Request<super::CreateDsProcessTaskRelationLogBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsProcessTaskRelationLogBean>, tonic::Status>;
        async fn update_ds_process_task_relation_log_bean(
            &self,
            request: tonic::Request<super::UpdateDsProcessTaskRelationLogBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsProcessTaskRelationLogBean>, tonic::Status>;
        async fn delete_ds_process_task_relation_log_bean(
            &self,
            request: tonic::Request<super::DeleteDsProcessTaskRelationLogBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct DsProcessTaskRelationLogBeanServiceServer<T: DsProcessTaskRelationLogBeanService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DsProcessTaskRelationLogBeanService> DsProcessTaskRelationLogBeanServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
        for DsProcessTaskRelationLogBeanServiceServer<T>
    where
        T: DsProcessTaskRelationLogBeanService,
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
                "/ds_process_task_relation_log.DsProcessTaskRelationLogBeanService/\
                 ListDsProcessTaskRelationLogBeans" => {
                    #[allow(non_camel_case_types)]
                    struct ListDsProcessTaskRelationLogBeansSvc<T: DsProcessTaskRelationLogBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsProcessTaskRelationLogBeanService>
                        tonic::server::UnaryService<super::ListDsProcessTaskRelationLogBeansRequest>
                        for ListDsProcessTaskRelationLogBeansSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::ListDsProcessTaskRelationLogBeansResponse;

                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListDsProcessTaskRelationLogBeansRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .list_ds_process_task_relation_log_beans(request)
                                    .await
                            };
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
                        let method = ListDsProcessTaskRelationLogBeansSvc(inner);
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
                "/ds_process_task_relation_log.DsProcessTaskRelationLogBeanService/\
                 GetDsProcessTaskRelationLogBean" => {
                    #[allow(non_camel_case_types)]
                    struct GetDsProcessTaskRelationLogBeanSvc<T: DsProcessTaskRelationLogBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsProcessTaskRelationLogBeanService>
                        tonic::server::UnaryService<super::GetDsProcessTaskRelationLogBeanRequest>
                        for GetDsProcessTaskRelationLogBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsProcessTaskRelationLogBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDsProcessTaskRelationLogBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_ds_process_task_relation_log_bean(request)
                                    .await
                            };
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
                        let method = GetDsProcessTaskRelationLogBeanSvc(inner);
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
                "/ds_process_task_relation_log.DsProcessTaskRelationLogBeanService/\
                 CreateDsProcessTaskRelationLogBean" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDsProcessTaskRelationLogBeanSvc<T: DsProcessTaskRelationLogBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsProcessTaskRelationLogBeanService>
                        tonic::server::UnaryService<
                            super::CreateDsProcessTaskRelationLogBeanRequest,
                        > for CreateDsProcessTaskRelationLogBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsProcessTaskRelationLogBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateDsProcessTaskRelationLogBeanRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .create_ds_process_task_relation_log_bean(request)
                                    .await
                            };
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
                        let method = CreateDsProcessTaskRelationLogBeanSvc(inner);
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
                "/ds_process_task_relation_log.DsProcessTaskRelationLogBeanService/\
                 UpdateDsProcessTaskRelationLogBean" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDsProcessTaskRelationLogBeanSvc<T: DsProcessTaskRelationLogBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsProcessTaskRelationLogBeanService>
                        tonic::server::UnaryService<
                            super::UpdateDsProcessTaskRelationLogBeanRequest,
                        > for UpdateDsProcessTaskRelationLogBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsProcessTaskRelationLogBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateDsProcessTaskRelationLogBeanRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .update_ds_process_task_relation_log_bean(request)
                                    .await
                            };
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
                        let method = UpdateDsProcessTaskRelationLogBeanSvc(inner);
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
                "/ds_process_task_relation_log.DsProcessTaskRelationLogBeanService/\
                 DeleteDsProcessTaskRelationLogBean" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDsProcessTaskRelationLogBeanSvc<T: DsProcessTaskRelationLogBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsProcessTaskRelationLogBeanService>
                        tonic::server::UnaryService<
                            super::DeleteDsProcessTaskRelationLogBeanRequest,
                        > for DeleteDsProcessTaskRelationLogBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = ();

                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteDsProcessTaskRelationLogBeanRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .delete_ds_process_task_relation_log_bean(request)
                                    .await
                            };
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
                        let method = DeleteDsProcessTaskRelationLogBeanSvc(inner);
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
    impl<T: DsProcessTaskRelationLogBeanService> Clone
        for DsProcessTaskRelationLogBeanServiceServer<T>
    {
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
    impl<T: DsProcessTaskRelationLogBeanService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DsProcessTaskRelationLogBeanService> tonic::server::NamedService
        for DsProcessTaskRelationLogBeanServiceServer<T>
    {
        const NAME: &'static str =
            "ds_process_task_relation_log.DsProcessTaskRelationLogBeanService";
    }
}
