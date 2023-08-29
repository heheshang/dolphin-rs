#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsSchedulesBean {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int64, tag = "2")]
    pub process_definition_code: i64,
    #[prost(string, tag = "3")]
    pub start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub end_time: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub timezone_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "6")]
    pub crontab: ::prost::alloc::string::String,
    #[prost(int32, tag = "7")]
    pub failure_strategy: i32,
    #[prost(int32, tag = "8")]
    pub user_id: i32,
    #[prost(int32, tag = "9")]
    pub release_state: i32,
    #[prost(int32, tag = "10")]
    pub warning_type: i32,
    #[prost(int32, optional, tag = "11")]
    pub warning_group_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub process_instance_priority: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "13")]
    pub worker_group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "14")]
    pub environment_code: ::core::option::Option<i64>,
    #[prost(string, tag = "15")]
    pub create_time: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub update_time: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsSchedulesBeansRequest {
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
pub struct ListDsSchedulesBeansResponse {
    /// The field name should match the noun "DsSchedulesBean" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub ds_schedules_beans: ::prost::alloc::vec::Vec<DsSchedulesBean>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsSchedulesBeanRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDsSchedulesBeanRequest {
    /// The parent resource name where the DsSchedulesBean is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The DsSchedulesBean id to use for this DsSchedulesBean.
    #[prost(string, tag = "2")]
    pub ds_schedules_bean_id: ::prost::alloc::string::String,
    /// The DsSchedulesBean resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub ds_schedules_bean: ::core::option::Option<DsSchedulesBean>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDsSchedulesBeanRequest {
    /// The DsSchedulesBean resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub ds_schedules_bean: ::core::option::Option<DsSchedulesBean>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsSchedulesBeanRequest {
    /// The resource name of the DsSchedulesBean to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod ds_schedules_bean_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct DsSchedulesBeanServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsSchedulesBeanServiceClient<tonic::transport::Channel> {
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
    impl<T> DsSchedulesBeanServiceClient<T>
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
        ) -> DsSchedulesBeanServiceClient<InterceptedService<T, F>>
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
            DsSchedulesBeanServiceClient::new(InterceptedService::new(inner, interceptor))
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

        pub async fn list_ds_schedules_beans(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDsSchedulesBeansRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsSchedulesBeansResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_schedules.DsSchedulesBeanService/ListDsSchedulesBeans",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_schedules.DsSchedulesBeanService",
                "ListDsSchedulesBeans",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn get_ds_schedules_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDsSchedulesBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsSchedulesBean>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_schedules.DsSchedulesBeanService/GetDsSchedulesBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_schedules.DsSchedulesBeanService",
                "GetDsSchedulesBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn create_ds_schedules_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDsSchedulesBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsSchedulesBean>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_schedules.DsSchedulesBeanService/CreateDsSchedulesBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_schedules.DsSchedulesBeanService",
                "CreateDsSchedulesBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn update_ds_schedules_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDsSchedulesBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsSchedulesBean>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_schedules.DsSchedulesBeanService/UpdateDsSchedulesBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_schedules.DsSchedulesBeanService",
                "UpdateDsSchedulesBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn delete_ds_schedules_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDsSchedulesBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_schedules.DsSchedulesBeanService/DeleteDsSchedulesBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_schedules.DsSchedulesBeanService",
                "DeleteDsSchedulesBean",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ds_schedules_bean_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DsSchedulesBeanServiceServer.
    #[async_trait]
    pub trait DsSchedulesBeanService: Send + Sync + 'static {
        async fn list_ds_schedules_beans(
            &self,
            request: tonic::Request<super::ListDsSchedulesBeansRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsSchedulesBeansResponse>, tonic::Status>;
        async fn get_ds_schedules_bean(
            &self,
            request: tonic::Request<super::GetDsSchedulesBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsSchedulesBean>, tonic::Status>;
        async fn create_ds_schedules_bean(
            &self,
            request: tonic::Request<super::CreateDsSchedulesBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsSchedulesBean>, tonic::Status>;
        async fn update_ds_schedules_bean(
            &self,
            request: tonic::Request<super::UpdateDsSchedulesBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsSchedulesBean>, tonic::Status>;
        async fn delete_ds_schedules_bean(
            &self,
            request: tonic::Request<super::DeleteDsSchedulesBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct DsSchedulesBeanServiceServer<T: DsSchedulesBeanService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DsSchedulesBeanService> DsSchedulesBeanServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DsSchedulesBeanServiceServer<T>
    where
        T: DsSchedulesBeanService,
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
                "/ds_schedules.DsSchedulesBeanService/ListDsSchedulesBeans" => {
                    #[allow(non_camel_case_types)]
                    struct ListDsSchedulesBeansSvc<T: DsSchedulesBeanService>(pub Arc<T>);
                    impl<T: DsSchedulesBeanService>
                        tonic::server::UnaryService<super::ListDsSchedulesBeansRequest>
                        for ListDsSchedulesBeansSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::ListDsSchedulesBeansResponse;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDsSchedulesBeansRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).list_ds_schedules_beans(request).await };
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
                        let method = ListDsSchedulesBeansSvc(inner);
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
                "/ds_schedules.DsSchedulesBeanService/GetDsSchedulesBean" => {
                    #[allow(non_camel_case_types)]
                    struct GetDsSchedulesBeanSvc<T: DsSchedulesBeanService>(pub Arc<T>);
                    impl<T: DsSchedulesBeanService>
                        tonic::server::UnaryService<super::GetDsSchedulesBeanRequest>
                        for GetDsSchedulesBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsSchedulesBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDsSchedulesBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_ds_schedules_bean(request).await };
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
                        let method = GetDsSchedulesBeanSvc(inner);
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
                "/ds_schedules.DsSchedulesBeanService/CreateDsSchedulesBean" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDsSchedulesBeanSvc<T: DsSchedulesBeanService>(pub Arc<T>);
                    impl<T: DsSchedulesBeanService>
                        tonic::server::UnaryService<super::CreateDsSchedulesBeanRequest>
                        for CreateDsSchedulesBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsSchedulesBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDsSchedulesBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).create_ds_schedules_bean(request).await };
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
                        let method = CreateDsSchedulesBeanSvc(inner);
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
                "/ds_schedules.DsSchedulesBeanService/UpdateDsSchedulesBean" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDsSchedulesBeanSvc<T: DsSchedulesBeanService>(pub Arc<T>);
                    impl<T: DsSchedulesBeanService>
                        tonic::server::UnaryService<super::UpdateDsSchedulesBeanRequest>
                        for UpdateDsSchedulesBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsSchedulesBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDsSchedulesBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).update_ds_schedules_bean(request).await };
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
                        let method = UpdateDsSchedulesBeanSvc(inner);
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
                "/ds_schedules.DsSchedulesBeanService/DeleteDsSchedulesBean" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDsSchedulesBeanSvc<T: DsSchedulesBeanService>(pub Arc<T>);
                    impl<T: DsSchedulesBeanService>
                        tonic::server::UnaryService<super::DeleteDsSchedulesBeanRequest>
                        for DeleteDsSchedulesBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = ();

                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDsSchedulesBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).delete_ds_schedules_bean(request).await };
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
                        let method = DeleteDsSchedulesBeanSvc(inner);
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
    impl<T: DsSchedulesBeanService> Clone for DsSchedulesBeanServiceServer<T> {
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
    impl<T: DsSchedulesBeanService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DsSchedulesBeanService> tonic::server::NamedService for DsSchedulesBeanServiceServer<T> {
        const NAME: &'static str = "ds_schedules.DsSchedulesBeanService";
    }
}
