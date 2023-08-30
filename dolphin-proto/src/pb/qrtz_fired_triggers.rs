#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QrtzFiredTriggersBean {
    #[prost(string, tag = "1")]
    pub sched_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub entry_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub trigger_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub trigger_group: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub instance_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub fired_time: i64,
    #[prost(int64, tag = "7")]
    pub sched_time: i64,
    #[prost(int32, tag = "8")]
    pub priority: i32,
    #[prost(string, tag = "9")]
    pub state: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "10")]
    pub job_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub job_group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "12")]
    pub is_nonconcurrent: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "13")]
    pub requests_recovery: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQrtzFiredTriggersBeansRequest {
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
pub struct ListQrtzFiredTriggersBeansResponse {
    /// The field name should match the noun "QrtzFiredTriggersBean" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub qrtz_fired_triggers_beans: ::prost::alloc::vec::Vec<QrtzFiredTriggersBean>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQrtzFiredTriggersBeanRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateQrtzFiredTriggersBeanRequest {
    /// The parent resource name where the QrtzFiredTriggersBean is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The QrtzFiredTriggersBean id to use for this QrtzFiredTriggersBean.
    #[prost(string, tag = "2")]
    pub qrtz_fired_triggers_bean_id: ::prost::alloc::string::String,
    /// The QrtzFiredTriggersBean resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub qrtz_fired_triggers_bean: ::core::option::Option<QrtzFiredTriggersBean>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateQrtzFiredTriggersBeanRequest {
    /// The QrtzFiredTriggersBean resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub qrtz_fired_triggers_bean: ::core::option::Option<QrtzFiredTriggersBean>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteQrtzFiredTriggersBeanRequest {
    /// The resource name of the QrtzFiredTriggersBean to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod qrtz_fired_triggers_bean_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct QrtzFiredTriggersBeanServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QrtzFiredTriggersBeanServiceClient<tonic::transport::Channel> {
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
    impl<T> QrtzFiredTriggersBeanServiceClient<T>
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
        ) -> QrtzFiredTriggersBeanServiceClient<InterceptedService<T, F>>
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
            QrtzFiredTriggersBeanServiceClient::new(InterceptedService::new(inner, interceptor))
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

        pub async fn list_qrtz_fired_triggers_beans(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQrtzFiredTriggersBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQrtzFiredTriggersBeansResponse>,
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
                "/qrtz_fired_triggers.QrtzFiredTriggersBeanService/ListQrtzFiredTriggersBeans",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_fired_triggers.QrtzFiredTriggersBeanService",
                "ListQrtzFiredTriggersBeans",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn get_qrtz_fired_triggers_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQrtzFiredTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzFiredTriggersBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_fired_triggers.QrtzFiredTriggersBeanService/GetQrtzFiredTriggersBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_fired_triggers.QrtzFiredTriggersBeanService",
                "GetQrtzFiredTriggersBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn create_qrtz_fired_triggers_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateQrtzFiredTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzFiredTriggersBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_fired_triggers.QrtzFiredTriggersBeanService/CreateQrtzFiredTriggersBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_fired_triggers.QrtzFiredTriggersBeanService",
                "CreateQrtzFiredTriggersBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn update_qrtz_fired_triggers_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateQrtzFiredTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzFiredTriggersBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_fired_triggers.QrtzFiredTriggersBeanService/UpdateQrtzFiredTriggersBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_fired_triggers.QrtzFiredTriggersBeanService",
                "UpdateQrtzFiredTriggersBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn delete_qrtz_fired_triggers_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteQrtzFiredTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_fired_triggers.QrtzFiredTriggersBeanService/DeleteQrtzFiredTriggersBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_fired_triggers.QrtzFiredTriggersBeanService",
                "DeleteQrtzFiredTriggersBean",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod qrtz_fired_triggers_bean_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QrtzFiredTriggersBeanServiceServer.
    #[async_trait]
    pub trait QrtzFiredTriggersBeanService: Send + Sync + 'static {
        async fn list_qrtz_fired_triggers_beans(
            &self,
            request: tonic::Request<super::ListQrtzFiredTriggersBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQrtzFiredTriggersBeansResponse>,
            tonic::Status,
        >;
        async fn get_qrtz_fired_triggers_bean(
            &self,
            request: tonic::Request<super::GetQrtzFiredTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzFiredTriggersBean>, tonic::Status>;
        async fn create_qrtz_fired_triggers_bean(
            &self,
            request: tonic::Request<super::CreateQrtzFiredTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzFiredTriggersBean>, tonic::Status>;
        async fn update_qrtz_fired_triggers_bean(
            &self,
            request: tonic::Request<super::UpdateQrtzFiredTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzFiredTriggersBean>, tonic::Status>;
        async fn delete_qrtz_fired_triggers_bean(
            &self,
            request: tonic::Request<super::DeleteQrtzFiredTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct QrtzFiredTriggersBeanServiceServer<T: QrtzFiredTriggersBeanService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: QrtzFiredTriggersBeanService> QrtzFiredTriggersBeanServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QrtzFiredTriggersBeanServiceServer<T>
    where
        T: QrtzFiredTriggersBeanService,
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
                "/qrtz_fired_triggers.QrtzFiredTriggersBeanService/ListQrtzFiredTriggersBeans" => {
                    #[allow(non_camel_case_types)]
                    struct ListQrtzFiredTriggersBeansSvc<T: QrtzFiredTriggersBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: QrtzFiredTriggersBeanService>
                        tonic::server::UnaryService<super::ListQrtzFiredTriggersBeansRequest>
                        for ListQrtzFiredTriggersBeansSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::ListQrtzFiredTriggersBeansResponse;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListQrtzFiredTriggersBeansRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_qrtz_fired_triggers_beans(request).await
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
                        let method = ListQrtzFiredTriggersBeansSvc(inner);
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
                "/qrtz_fired_triggers.QrtzFiredTriggersBeanService/GetQrtzFiredTriggersBean" => {
                    #[allow(non_camel_case_types)]
                    struct GetQrtzFiredTriggersBeanSvc<T: QrtzFiredTriggersBeanService>(pub Arc<T>);
                    impl<T: QrtzFiredTriggersBeanService>
                        tonic::server::UnaryService<super::GetQrtzFiredTriggersBeanRequest>
                        for GetQrtzFiredTriggersBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::QrtzFiredTriggersBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetQrtzFiredTriggersBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).get_qrtz_fired_triggers_bean(request).await };
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
                        let method = GetQrtzFiredTriggersBeanSvc(inner);
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
                "/qrtz_fired_triggers.QrtzFiredTriggersBeanService/CreateQrtzFiredTriggersBean" => {
                    #[allow(non_camel_case_types)]
                    struct CreateQrtzFiredTriggersBeanSvc<T: QrtzFiredTriggersBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: QrtzFiredTriggersBeanService>
                        tonic::server::UnaryService<super::CreateQrtzFiredTriggersBeanRequest>
                        for CreateQrtzFiredTriggersBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::QrtzFiredTriggersBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateQrtzFiredTriggersBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_qrtz_fired_triggers_bean(request).await
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
                        let method = CreateQrtzFiredTriggersBeanSvc(inner);
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
                "/qrtz_fired_triggers.QrtzFiredTriggersBeanService/UpdateQrtzFiredTriggersBean" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateQrtzFiredTriggersBeanSvc<T: QrtzFiredTriggersBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: QrtzFiredTriggersBeanService>
                        tonic::server::UnaryService<super::UpdateQrtzFiredTriggersBeanRequest>
                        for UpdateQrtzFiredTriggersBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::QrtzFiredTriggersBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateQrtzFiredTriggersBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_qrtz_fired_triggers_bean(request).await
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
                        let method = UpdateQrtzFiredTriggersBeanSvc(inner);
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
                "/qrtz_fired_triggers.QrtzFiredTriggersBeanService/DeleteQrtzFiredTriggersBean" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteQrtzFiredTriggersBeanSvc<T: QrtzFiredTriggersBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: QrtzFiredTriggersBeanService>
                        tonic::server::UnaryService<super::DeleteQrtzFiredTriggersBeanRequest>
                        for DeleteQrtzFiredTriggersBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = ();

                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteQrtzFiredTriggersBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_qrtz_fired_triggers_bean(request).await
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
                        let method = DeleteQrtzFiredTriggersBeanSvc(inner);
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
    impl<T: QrtzFiredTriggersBeanService> Clone for QrtzFiredTriggersBeanServiceServer<T> {
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
    impl<T: QrtzFiredTriggersBeanService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: QrtzFiredTriggersBeanService> tonic::server::NamedService
        for QrtzFiredTriggersBeanServiceServer<T>
    {
        const NAME: &'static str = "qrtz_fired_triggers.QrtzFiredTriggersBeanService";
    }
}
