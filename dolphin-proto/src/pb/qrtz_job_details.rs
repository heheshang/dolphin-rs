#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QrtzJobDetailsBean {
    #[prost(string, tag = "1")]
    pub sched_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub job_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub job_group: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "5")]
    pub job_class_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub is_durable: bool,
    #[prost(bool, tag = "7")]
    pub is_nonconcurrent: bool,
    #[prost(bool, tag = "8")]
    pub is_update_data: bool,
    #[prost(bool, tag = "9")]
    pub requests_recovery: bool,
    #[prost(bytes = "vec", optional, tag = "10")]
    pub job_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQrtzJobDetailsBeansRequest {
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
pub struct ListQrtzJobDetailsBeansResponse {
    /// The field name should match the noun "QrtzJobDetailsBean" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub qrtz_job_details_beans: ::prost::alloc::vec::Vec<QrtzJobDetailsBean>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQrtzJobDetailsBeanRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateQrtzJobDetailsBeanRequest {
    /// The parent resource name where the QrtzJobDetailsBean is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The QrtzJobDetailsBean id to use for this QrtzJobDetailsBean.
    #[prost(string, tag = "2")]
    pub qrtz_job_details_bean_id: ::prost::alloc::string::String,
    /// The QrtzJobDetailsBean resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub qrtz_job_details_bean: ::core::option::Option<QrtzJobDetailsBean>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateQrtzJobDetailsBeanRequest {
    /// The QrtzJobDetailsBean resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub qrtz_job_details_bean: ::core::option::Option<QrtzJobDetailsBean>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteQrtzJobDetailsBeanRequest {
    /// The resource name of the QrtzJobDetailsBean to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod qrtz_job_details_bean_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct QrtzJobDetailsBeanServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QrtzJobDetailsBeanServiceClient<tonic::transport::Channel> {
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
    impl<T> QrtzJobDetailsBeanServiceClient<T>
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
        ) -> QrtzJobDetailsBeanServiceClient<InterceptedService<T, F>>
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
            QrtzJobDetailsBeanServiceClient::new(InterceptedService::new(inner, interceptor))
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

        pub async fn list_qrtz_job_details_beans(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQrtzJobDetailsBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQrtzJobDetailsBeansResponse>,
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
                "/qrtz_job_details.QrtzJobDetailsBeanService/ListQrtzJobDetailsBeans",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_job_details.QrtzJobDetailsBeanService",
                "ListQrtzJobDetailsBeans",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn get_qrtz_job_details_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQrtzJobDetailsBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzJobDetailsBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_job_details.QrtzJobDetailsBeanService/GetQrtzJobDetailsBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_job_details.QrtzJobDetailsBeanService",
                "GetQrtzJobDetailsBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn create_qrtz_job_details_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateQrtzJobDetailsBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzJobDetailsBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_job_details.QrtzJobDetailsBeanService/CreateQrtzJobDetailsBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_job_details.QrtzJobDetailsBeanService",
                "CreateQrtzJobDetailsBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn update_qrtz_job_details_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateQrtzJobDetailsBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzJobDetailsBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_job_details.QrtzJobDetailsBeanService/UpdateQrtzJobDetailsBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_job_details.QrtzJobDetailsBeanService",
                "UpdateQrtzJobDetailsBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn delete_qrtz_job_details_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteQrtzJobDetailsBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_job_details.QrtzJobDetailsBeanService/DeleteQrtzJobDetailsBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_job_details.QrtzJobDetailsBeanService",
                "DeleteQrtzJobDetailsBean",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod qrtz_job_details_bean_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QrtzJobDetailsBeanServiceServer.
    #[async_trait]
    pub trait QrtzJobDetailsBeanService: Send + Sync + 'static {
        async fn list_qrtz_job_details_beans(
            &self,
            request: tonic::Request<super::ListQrtzJobDetailsBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQrtzJobDetailsBeansResponse>,
            tonic::Status,
        >;
        async fn get_qrtz_job_details_bean(
            &self,
            request: tonic::Request<super::GetQrtzJobDetailsBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzJobDetailsBean>, tonic::Status>;
        async fn create_qrtz_job_details_bean(
            &self,
            request: tonic::Request<super::CreateQrtzJobDetailsBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzJobDetailsBean>, tonic::Status>;
        async fn update_qrtz_job_details_bean(
            &self,
            request: tonic::Request<super::UpdateQrtzJobDetailsBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzJobDetailsBean>, tonic::Status>;
        async fn delete_qrtz_job_details_bean(
            &self,
            request: tonic::Request<super::DeleteQrtzJobDetailsBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct QrtzJobDetailsBeanServiceServer<T: QrtzJobDetailsBeanService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: QrtzJobDetailsBeanService> QrtzJobDetailsBeanServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QrtzJobDetailsBeanServiceServer<T>
    where
        T: QrtzJobDetailsBeanService,
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
                "/qrtz_job_details.QrtzJobDetailsBeanService/ListQrtzJobDetailsBeans" => {
                    #[allow(non_camel_case_types)]
                    struct ListQrtzJobDetailsBeansSvc<T: QrtzJobDetailsBeanService>(pub Arc<T>);
                    impl<T: QrtzJobDetailsBeanService>
                        tonic::server::UnaryService<super::ListQrtzJobDetailsBeansRequest>
                        for ListQrtzJobDetailsBeansSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::ListQrtzJobDetailsBeansResponse;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListQrtzJobDetailsBeansRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).list_qrtz_job_details_beans(request).await };
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
                        let method = ListQrtzJobDetailsBeansSvc(inner);
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
                "/qrtz_job_details.QrtzJobDetailsBeanService/GetQrtzJobDetailsBean" => {
                    #[allow(non_camel_case_types)]
                    struct GetQrtzJobDetailsBeanSvc<T: QrtzJobDetailsBeanService>(pub Arc<T>);
                    impl<T: QrtzJobDetailsBeanService>
                        tonic::server::UnaryService<super::GetQrtzJobDetailsBeanRequest>
                        for GetQrtzJobDetailsBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::QrtzJobDetailsBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetQrtzJobDetailsBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).get_qrtz_job_details_bean(request).await };
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
                        let method = GetQrtzJobDetailsBeanSvc(inner);
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
                "/qrtz_job_details.QrtzJobDetailsBeanService/CreateQrtzJobDetailsBean" => {
                    #[allow(non_camel_case_types)]
                    struct CreateQrtzJobDetailsBeanSvc<T: QrtzJobDetailsBeanService>(pub Arc<T>);
                    impl<T: QrtzJobDetailsBeanService>
                        tonic::server::UnaryService<super::CreateQrtzJobDetailsBeanRequest>
                        for CreateQrtzJobDetailsBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::QrtzJobDetailsBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateQrtzJobDetailsBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).create_qrtz_job_details_bean(request).await };
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
                        let method = CreateQrtzJobDetailsBeanSvc(inner);
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
                "/qrtz_job_details.QrtzJobDetailsBeanService/UpdateQrtzJobDetailsBean" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateQrtzJobDetailsBeanSvc<T: QrtzJobDetailsBeanService>(pub Arc<T>);
                    impl<T: QrtzJobDetailsBeanService>
                        tonic::server::UnaryService<super::UpdateQrtzJobDetailsBeanRequest>
                        for UpdateQrtzJobDetailsBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::QrtzJobDetailsBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateQrtzJobDetailsBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).update_qrtz_job_details_bean(request).await };
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
                        let method = UpdateQrtzJobDetailsBeanSvc(inner);
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
                "/qrtz_job_details.QrtzJobDetailsBeanService/DeleteQrtzJobDetailsBean" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteQrtzJobDetailsBeanSvc<T: QrtzJobDetailsBeanService>(pub Arc<T>);
                    impl<T: QrtzJobDetailsBeanService>
                        tonic::server::UnaryService<super::DeleteQrtzJobDetailsBeanRequest>
                        for DeleteQrtzJobDetailsBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = ();

                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteQrtzJobDetailsBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).delete_qrtz_job_details_bean(request).await };
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
                        let method = DeleteQrtzJobDetailsBeanSvc(inner);
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
    impl<T: QrtzJobDetailsBeanService> Clone for QrtzJobDetailsBeanServiceServer<T> {
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
    impl<T: QrtzJobDetailsBeanService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: QrtzJobDetailsBeanService> tonic::server::NamedService
        for QrtzJobDetailsBeanServiceServer<T>
    {
        const NAME: &'static str = "qrtz_job_details.QrtzJobDetailsBeanService";
    }
}
