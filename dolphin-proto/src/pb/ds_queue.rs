#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsQueueBean {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, optional, tag = "2")]
    pub queue_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub queue: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsQueueBeansRequest {
    /// The maximum number of items to return.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    #[prost(int32, tag = "2")]
    pub page_num: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsQueueBeansResponse {
    /// The field name should match the noun "DsQueueBean" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub ds_queue_beans: ::prost::alloc::vec::Vec<DsQueueBean>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsQueueBeanRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDsQueueBeanRequest {
    /// The parent resource name where the DsQueueBean is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The DsQueueBean id to use for this DsQueueBean.
    #[prost(string, tag = "2")]
    pub ds_queue_bean_id: ::prost::alloc::string::String,
    /// The DsQueueBean resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub ds_queue_bean: ::core::option::Option<DsQueueBean>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDsQueueBeanRequest {
    /// The DsQueueBean resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub ds_queue_bean: ::core::option::Option<DsQueueBean>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsQueueBeanRequest {
    /// The resource name of the DsQueueBean to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod ds_queue_bean_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct DsQueueBeanServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsQueueBeanServiceClient<tonic::transport::Channel> {
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
    impl<T> DsQueueBeanServiceClient<T>
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
        ) -> DsQueueBeanServiceClient<InterceptedService<T, F>>
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
            DsQueueBeanServiceClient::new(InterceptedService::new(inner, interceptor))
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

        pub async fn list_ds_queue_beans(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDsQueueBeansRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsQueueBeansResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_queue.DsQueueBeanService/ListDsQueueBeans",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_queue.DsQueueBeanService",
                "ListDsQueueBeans",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn get_ds_queue_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDsQueueBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsQueueBean>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/ds_queue.DsQueueBeanService/GetDsQueueBean");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_queue.DsQueueBeanService",
                "GetDsQueueBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn create_ds_queue_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDsQueueBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsQueueBean>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_queue.DsQueueBeanService/CreateDsQueueBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_queue.DsQueueBeanService",
                "CreateDsQueueBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn update_ds_queue_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDsQueueBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsQueueBean>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_queue.DsQueueBeanService/UpdateDsQueueBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_queue.DsQueueBeanService",
                "UpdateDsQueueBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn delete_ds_queue_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDsQueueBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_queue.DsQueueBeanService/DeleteDsQueueBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_queue.DsQueueBeanService",
                "DeleteDsQueueBean",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ds_queue_bean_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DsQueueBeanServiceServer.
    #[async_trait]
    pub trait DsQueueBeanService: Send + Sync + 'static {
        async fn list_ds_queue_beans(
            &self,
            request: tonic::Request<super::ListDsQueueBeansRequest>,
        ) -> std::result::Result<tonic::Response<super::ListDsQueueBeansResponse>, tonic::Status>;
        async fn get_ds_queue_bean(
            &self,
            request: tonic::Request<super::GetDsQueueBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsQueueBean>, tonic::Status>;
        async fn create_ds_queue_bean(
            &self,
            request: tonic::Request<super::CreateDsQueueBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsQueueBean>, tonic::Status>;
        async fn update_ds_queue_bean(
            &self,
            request: tonic::Request<super::UpdateDsQueueBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsQueueBean>, tonic::Status>;
        async fn delete_ds_queue_bean(
            &self,
            request: tonic::Request<super::DeleteDsQueueBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct DsQueueBeanServiceServer<T: DsQueueBeanService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DsQueueBeanService> DsQueueBeanServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DsQueueBeanServiceServer<T>
    where
        T: DsQueueBeanService,
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
                "/ds_queue.DsQueueBeanService/ListDsQueueBeans" => {
                    #[allow(non_camel_case_types)]
                    struct ListDsQueueBeansSvc<T: DsQueueBeanService>(pub Arc<T>);
                    impl<T: DsQueueBeanService>
                        tonic::server::UnaryService<super::ListDsQueueBeansRequest>
                        for ListDsQueueBeansSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::ListDsQueueBeansResponse;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDsQueueBeansRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_ds_queue_beans(request).await };
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
                        let method = ListDsQueueBeansSvc(inner);
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
                "/ds_queue.DsQueueBeanService/GetDsQueueBean" => {
                    #[allow(non_camel_case_types)]
                    struct GetDsQueueBeanSvc<T: DsQueueBeanService>(pub Arc<T>);
                    impl<T: DsQueueBeanService>
                        tonic::server::UnaryService<super::GetDsQueueBeanRequest>
                        for GetDsQueueBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsQueueBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDsQueueBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_ds_queue_bean(request).await };
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
                        let method = GetDsQueueBeanSvc(inner);
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
                "/ds_queue.DsQueueBeanService/CreateDsQueueBean" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDsQueueBeanSvc<T: DsQueueBeanService>(pub Arc<T>);
                    impl<T: DsQueueBeanService>
                        tonic::server::UnaryService<super::CreateDsQueueBeanRequest>
                        for CreateDsQueueBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsQueueBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDsQueueBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_ds_queue_bean(request).await };
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
                        let method = CreateDsQueueBeanSvc(inner);
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
                "/ds_queue.DsQueueBeanService/UpdateDsQueueBean" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDsQueueBeanSvc<T: DsQueueBeanService>(pub Arc<T>);
                    impl<T: DsQueueBeanService>
                        tonic::server::UnaryService<super::UpdateDsQueueBeanRequest>
                        for UpdateDsQueueBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsQueueBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDsQueueBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_ds_queue_bean(request).await };
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
                        let method = UpdateDsQueueBeanSvc(inner);
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
                "/ds_queue.DsQueueBeanService/DeleteDsQueueBean" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDsQueueBeanSvc<T: DsQueueBeanService>(pub Arc<T>);
                    impl<T: DsQueueBeanService>
                        tonic::server::UnaryService<super::DeleteDsQueueBeanRequest>
                        for DeleteDsQueueBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = ();

                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDsQueueBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_ds_queue_bean(request).await };
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
                        let method = DeleteDsQueueBeanSvc(inner);
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
    impl<T: DsQueueBeanService> Clone for DsQueueBeanServiceServer<T> {
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
    impl<T: DsQueueBeanService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DsQueueBeanService> tonic::server::NamedService for DsQueueBeanServiceServer<T> {
        const NAME: &'static str = "ds_queue.DsQueueBeanService";
    }
}
