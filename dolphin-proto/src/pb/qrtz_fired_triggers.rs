#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QrtzFiredTriggers {
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
pub struct ListQrtzFiredTriggerssRequest {
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
pub struct ListQrtzFiredTriggerssResponse {
    /// The field name should match the noun "QrtzFiredTriggers" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub qrtz_fired_triggerss: ::prost::alloc::vec::Vec<QrtzFiredTriggers>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQrtzFiredTriggersRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateQrtzFiredTriggersRequest {
    /// The parent resource name where the QrtzFiredTriggers is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The QrtzFiredTriggers id to use for this QrtzFiredTriggers.
    #[prost(string, tag = "2")]
    pub qrtz_fired_triggers_id: ::prost::alloc::string::String,
    /// The QrtzFiredTriggers resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub qrtz_fired_triggers: ::core::option::Option<QrtzFiredTriggers>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateQrtzFiredTriggersRequest {
    /// The QrtzFiredTriggers resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub qrtz_fired_triggers: ::core::option::Option<QrtzFiredTriggers>,
    /// The update mask applies to the resource. For the `FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteQrtzFiredTriggersRequest {
    /// The resource name of the QrtzFiredTriggers to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod qrtz_fired_triggers_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct QrtzFiredTriggersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QrtzFiredTriggersServiceClient<tonic::transport::Channel> {
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
    impl<T> QrtzFiredTriggersServiceClient<T>
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
        ) -> QrtzFiredTriggersServiceClient<InterceptedService<T, F>>
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
            QrtzFiredTriggersServiceClient::new(
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
        pub async fn list_qrtz_fired_triggerss(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQrtzFiredTriggerssRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQrtzFiredTriggerssResponse>,
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
                "/qrtz_fired_triggers.QrtzFiredTriggersService/ListQrtzFiredTriggerss",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "qrtz_fired_triggers.QrtzFiredTriggersService",
                        "ListQrtzFiredTriggerss",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_qrtz_fired_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQrtzFiredTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QrtzFiredTriggers>,
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
                "/qrtz_fired_triggers.QrtzFiredTriggersService/GetQrtzFiredTriggers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "qrtz_fired_triggers.QrtzFiredTriggersService",
                        "GetQrtzFiredTriggers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_qrtz_fired_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateQrtzFiredTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QrtzFiredTriggers>,
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
                "/qrtz_fired_triggers.QrtzFiredTriggersService/CreateQrtzFiredTriggers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "qrtz_fired_triggers.QrtzFiredTriggersService",
                        "CreateQrtzFiredTriggers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_qrtz_fired_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateQrtzFiredTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QrtzFiredTriggers>,
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
                "/qrtz_fired_triggers.QrtzFiredTriggersService/UpdateQrtzFiredTriggers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "qrtz_fired_triggers.QrtzFiredTriggersService",
                        "UpdateQrtzFiredTriggers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_qrtz_fired_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteQrtzFiredTriggersRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/qrtz_fired_triggers.QrtzFiredTriggersService/DeleteQrtzFiredTriggers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "qrtz_fired_triggers.QrtzFiredTriggersService",
                        "DeleteQrtzFiredTriggers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod qrtz_fired_triggers_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QrtzFiredTriggersServiceServer.
    #[async_trait]
    pub trait QrtzFiredTriggersService: Send + Sync + 'static {
        async fn list_qrtz_fired_triggerss(
            &self,
            request: tonic::Request<super::ListQrtzFiredTriggerssRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQrtzFiredTriggerssResponse>,
            tonic::Status,
        >;
        async fn get_qrtz_fired_triggers(
            &self,
            request: tonic::Request<super::GetQrtzFiredTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QrtzFiredTriggers>,
            tonic::Status,
        >;
        async fn create_qrtz_fired_triggers(
            &self,
            request: tonic::Request<super::CreateQrtzFiredTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QrtzFiredTriggers>,
            tonic::Status,
        >;
        async fn update_qrtz_fired_triggers(
            &self,
            request: tonic::Request<super::UpdateQrtzFiredTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QrtzFiredTriggers>,
            tonic::Status,
        >;
        async fn delete_qrtz_fired_triggers(
            &self,
            request: tonic::Request<super::DeleteQrtzFiredTriggersRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct QrtzFiredTriggersServiceServer<T: QrtzFiredTriggersService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: QrtzFiredTriggersService> QrtzFiredTriggersServiceServer<T> {
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
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
    for QrtzFiredTriggersServiceServer<T>
    where
        T: QrtzFiredTriggersService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/qrtz_fired_triggers.QrtzFiredTriggersService/ListQrtzFiredTriggerss" => {
                    #[allow(non_camel_case_types)]
                    struct ListQrtzFiredTriggerssSvc<T: QrtzFiredTriggersService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: QrtzFiredTriggersService,
                    > tonic::server::UnaryService<super::ListQrtzFiredTriggerssRequest>
                    for ListQrtzFiredTriggerssSvc<T> {
                        type Response = super::ListQrtzFiredTriggerssResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListQrtzFiredTriggerssRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_qrtz_fired_triggerss(request).await
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
                        let method = ListQrtzFiredTriggerssSvc(inner);
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
                "/qrtz_fired_triggers.QrtzFiredTriggersService/GetQrtzFiredTriggers" => {
                    #[allow(non_camel_case_types)]
                    struct GetQrtzFiredTriggersSvc<T: QrtzFiredTriggersService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: QrtzFiredTriggersService,
                    > tonic::server::UnaryService<super::GetQrtzFiredTriggersRequest>
                    for GetQrtzFiredTriggersSvc<T> {
                        type Response = super::QrtzFiredTriggers;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetQrtzFiredTriggersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_qrtz_fired_triggers(request).await
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
                        let method = GetQrtzFiredTriggersSvc(inner);
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
                "/qrtz_fired_triggers.QrtzFiredTriggersService/CreateQrtzFiredTriggers" => {
                    #[allow(non_camel_case_types)]
                    struct CreateQrtzFiredTriggersSvc<T: QrtzFiredTriggersService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: QrtzFiredTriggersService,
                    > tonic::server::UnaryService<super::CreateQrtzFiredTriggersRequest>
                    for CreateQrtzFiredTriggersSvc<T> {
                        type Response = super::QrtzFiredTriggers;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateQrtzFiredTriggersRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_qrtz_fired_triggers(request).await
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
                        let method = CreateQrtzFiredTriggersSvc(inner);
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
                "/qrtz_fired_triggers.QrtzFiredTriggersService/UpdateQrtzFiredTriggers" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateQrtzFiredTriggersSvc<T: QrtzFiredTriggersService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: QrtzFiredTriggersService,
                    > tonic::server::UnaryService<super::UpdateQrtzFiredTriggersRequest>
                    for UpdateQrtzFiredTriggersSvc<T> {
                        type Response = super::QrtzFiredTriggers;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateQrtzFiredTriggersRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_qrtz_fired_triggers(request).await
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
                        let method = UpdateQrtzFiredTriggersSvc(inner);
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
                "/qrtz_fired_triggers.QrtzFiredTriggersService/DeleteQrtzFiredTriggers" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteQrtzFiredTriggersSvc<T: QrtzFiredTriggersService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: QrtzFiredTriggersService,
                    > tonic::server::UnaryService<super::DeleteQrtzFiredTriggersRequest>
                    for DeleteQrtzFiredTriggersSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteQrtzFiredTriggersRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_qrtz_fired_triggers(request).await
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
                        let method = DeleteQrtzFiredTriggersSvc(inner);
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
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: QrtzFiredTriggersService> Clone for QrtzFiredTriggersServiceServer<T> {
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
    impl<T: QrtzFiredTriggersService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: QrtzFiredTriggersService> tonic::server::NamedService
    for QrtzFiredTriggersServiceServer<T> {
        const NAME: &'static str = "qrtz_fired_triggers.QrtzFiredTriggersService";
    }
}
