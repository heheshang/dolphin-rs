#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QrtzSimpropTriggers {
    #[prost(string, tag = "1")]
    pub sched_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub trigger_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub trigger_group: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "4")]
    pub str_prop_1: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub str_prop_2: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub str_prop_3: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "7")]
    pub int_prop_1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub int_prop_2: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "9")]
    pub long_prop_1: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "10")]
    pub long_prop_2: ::core::option::Option<i64>,
    #[prost(double, optional, tag = "11")]
    pub dec_prop_1: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "12")]
    pub dec_prop_2: ::core::option::Option<f64>,
    #[prost(bool, tag = "13")]
    pub bool_prop_1: bool,
    #[prost(bool, tag = "14")]
    pub bool_prop_2: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQrtzSimpropTriggerssRequest {
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
pub struct ListQrtzSimpropTriggerssResponse {
    /// The field name should match the noun "QrtzSimpropTriggers" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub qrtz_simprop_triggerss: ::prost::alloc::vec::Vec<QrtzSimpropTriggers>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQrtzSimpropTriggersRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateQrtzSimpropTriggersRequest {
    /// The parent resource name where the QrtzSimpropTriggers is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The QrtzSimpropTriggers id to use for this QrtzSimpropTriggers.
    #[prost(string, tag = "2")]
    pub qrtz_simprop_triggers_id: ::prost::alloc::string::String,
    /// The QrtzSimpropTriggers resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub qrtz_simprop_triggers: ::core::option::Option<QrtzSimpropTriggers>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateQrtzSimpropTriggersRequest {
    /// The QrtzSimpropTriggers resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub qrtz_simprop_triggers: ::core::option::Option<QrtzSimpropTriggers>,
    /// The update mask applies to the resource. For the `FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteQrtzSimpropTriggersRequest {
    /// The resource name of the QrtzSimpropTriggers to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod qrtz_simprop_triggers_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct QrtzSimpropTriggersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QrtzSimpropTriggersServiceClient<tonic::transport::Channel> {
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
    impl<T> QrtzSimpropTriggersServiceClient<T>
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
        ) -> QrtzSimpropTriggersServiceClient<InterceptedService<T, F>>
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
            QrtzSimpropTriggersServiceClient::new(
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
        pub async fn list_qrtz_simprop_triggerss(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQrtzSimpropTriggerssRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQrtzSimpropTriggerssResponse>,
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
                "/qrtz_simprop_triggers.QrtzSimpropTriggersService/ListQrtzSimpropTriggerss",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "qrtz_simprop_triggers.QrtzSimpropTriggersService",
                        "ListQrtzSimpropTriggerss",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_qrtz_simprop_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQrtzSimpropTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QrtzSimpropTriggers>,
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
                "/qrtz_simprop_triggers.QrtzSimpropTriggersService/GetQrtzSimpropTriggers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "qrtz_simprop_triggers.QrtzSimpropTriggersService",
                        "GetQrtzSimpropTriggers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_qrtz_simprop_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateQrtzSimpropTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QrtzSimpropTriggers>,
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
                "/qrtz_simprop_triggers.QrtzSimpropTriggersService/CreateQrtzSimpropTriggers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "qrtz_simprop_triggers.QrtzSimpropTriggersService",
                        "CreateQrtzSimpropTriggers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_qrtz_simprop_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateQrtzSimpropTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QrtzSimpropTriggers>,
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
                "/qrtz_simprop_triggers.QrtzSimpropTriggersService/UpdateQrtzSimpropTriggers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "qrtz_simprop_triggers.QrtzSimpropTriggersService",
                        "UpdateQrtzSimpropTriggers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_qrtz_simprop_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteQrtzSimpropTriggersRequest>,
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
                "/qrtz_simprop_triggers.QrtzSimpropTriggersService/DeleteQrtzSimpropTriggers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "qrtz_simprop_triggers.QrtzSimpropTriggersService",
                        "DeleteQrtzSimpropTriggers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod qrtz_simprop_triggers_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QrtzSimpropTriggersServiceServer.
    #[async_trait]
    pub trait QrtzSimpropTriggersService: Send + Sync + 'static {
        async fn list_qrtz_simprop_triggerss(
            &self,
            request: tonic::Request<super::ListQrtzSimpropTriggerssRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQrtzSimpropTriggerssResponse>,
            tonic::Status,
        >;
        async fn get_qrtz_simprop_triggers(
            &self,
            request: tonic::Request<super::GetQrtzSimpropTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QrtzSimpropTriggers>,
            tonic::Status,
        >;
        async fn create_qrtz_simprop_triggers(
            &self,
            request: tonic::Request<super::CreateQrtzSimpropTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QrtzSimpropTriggers>,
            tonic::Status,
        >;
        async fn update_qrtz_simprop_triggers(
            &self,
            request: tonic::Request<super::UpdateQrtzSimpropTriggersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QrtzSimpropTriggers>,
            tonic::Status,
        >;
        async fn delete_qrtz_simprop_triggers(
            &self,
            request: tonic::Request<super::DeleteQrtzSimpropTriggersRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct QrtzSimpropTriggersServiceServer<T: QrtzSimpropTriggersService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: QrtzSimpropTriggersService> QrtzSimpropTriggersServiceServer<T> {
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
    for QrtzSimpropTriggersServiceServer<T>
    where
        T: QrtzSimpropTriggersService,
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
                "/qrtz_simprop_triggers.QrtzSimpropTriggersService/ListQrtzSimpropTriggerss" => {
                    #[allow(non_camel_case_types)]
                    struct ListQrtzSimpropTriggerssSvc<T: QrtzSimpropTriggersService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: QrtzSimpropTriggersService,
                    > tonic::server::UnaryService<super::ListQrtzSimpropTriggerssRequest>
                    for ListQrtzSimpropTriggerssSvc<T> {
                        type Response = super::ListQrtzSimpropTriggerssResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListQrtzSimpropTriggerssRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_qrtz_simprop_triggerss(request).await
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
                        let method = ListQrtzSimpropTriggerssSvc(inner);
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
                "/qrtz_simprop_triggers.QrtzSimpropTriggersService/GetQrtzSimpropTriggers" => {
                    #[allow(non_camel_case_types)]
                    struct GetQrtzSimpropTriggersSvc<T: QrtzSimpropTriggersService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: QrtzSimpropTriggersService,
                    > tonic::server::UnaryService<super::GetQrtzSimpropTriggersRequest>
                    for GetQrtzSimpropTriggersSvc<T> {
                        type Response = super::QrtzSimpropTriggers;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetQrtzSimpropTriggersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_qrtz_simprop_triggers(request).await
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
                        let method = GetQrtzSimpropTriggersSvc(inner);
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
                "/qrtz_simprop_triggers.QrtzSimpropTriggersService/CreateQrtzSimpropTriggers" => {
                    #[allow(non_camel_case_types)]
                    struct CreateQrtzSimpropTriggersSvc<T: QrtzSimpropTriggersService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: QrtzSimpropTriggersService,
                    > tonic::server::UnaryService<
                        super::CreateQrtzSimpropTriggersRequest,
                    > for CreateQrtzSimpropTriggersSvc<T> {
                        type Response = super::QrtzSimpropTriggers;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateQrtzSimpropTriggersRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_qrtz_simprop_triggers(request).await
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
                        let method = CreateQrtzSimpropTriggersSvc(inner);
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
                "/qrtz_simprop_triggers.QrtzSimpropTriggersService/UpdateQrtzSimpropTriggers" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateQrtzSimpropTriggersSvc<T: QrtzSimpropTriggersService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: QrtzSimpropTriggersService,
                    > tonic::server::UnaryService<
                        super::UpdateQrtzSimpropTriggersRequest,
                    > for UpdateQrtzSimpropTriggersSvc<T> {
                        type Response = super::QrtzSimpropTriggers;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateQrtzSimpropTriggersRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_qrtz_simprop_triggers(request).await
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
                        let method = UpdateQrtzSimpropTriggersSvc(inner);
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
                "/qrtz_simprop_triggers.QrtzSimpropTriggersService/DeleteQrtzSimpropTriggers" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteQrtzSimpropTriggersSvc<T: QrtzSimpropTriggersService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: QrtzSimpropTriggersService,
                    > tonic::server::UnaryService<
                        super::DeleteQrtzSimpropTriggersRequest,
                    > for DeleteQrtzSimpropTriggersSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteQrtzSimpropTriggersRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_qrtz_simprop_triggers(request).await
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
                        let method = DeleteQrtzSimpropTriggersSvc(inner);
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
    impl<T: QrtzSimpropTriggersService> Clone for QrtzSimpropTriggersServiceServer<T> {
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
    impl<T: QrtzSimpropTriggersService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: QrtzSimpropTriggersService> tonic::server::NamedService
    for QrtzSimpropTriggersServiceServer<T> {
        const NAME: &'static str = "qrtz_simprop_triggers.QrtzSimpropTriggersService";
    }
}
