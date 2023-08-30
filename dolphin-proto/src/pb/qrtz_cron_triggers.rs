#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QrtzCronTriggersBean {
    #[prost(string, tag = "1")]
    pub sched_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub trigger_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub trigger_group: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub cron_expression: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub time_zone_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQrtzCronTriggersBeansRequest {
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
pub struct ListQrtzCronTriggersBeansResponse {
    /// The field name should match the noun "QrtzCronTriggersBean" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub qrtz_cron_triggers_beans: ::prost::alloc::vec::Vec<QrtzCronTriggersBean>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQrtzCronTriggersBeanRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateQrtzCronTriggersBeanRequest {
    /// The parent resource name where the QrtzCronTriggersBean is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The QrtzCronTriggersBean id to use for this QrtzCronTriggersBean.
    #[prost(string, tag = "2")]
    pub qrtz_cron_triggers_bean_id: ::prost::alloc::string::String,
    /// The QrtzCronTriggersBean resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub qrtz_cron_triggers_bean: ::core::option::Option<QrtzCronTriggersBean>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateQrtzCronTriggersBeanRequest {
    /// The QrtzCronTriggersBean resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub qrtz_cron_triggers_bean: ::core::option::Option<QrtzCronTriggersBean>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteQrtzCronTriggersBeanRequest {
    /// The resource name of the QrtzCronTriggersBean to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod qrtz_cron_triggers_bean_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct QrtzCronTriggersBeanServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QrtzCronTriggersBeanServiceClient<tonic::transport::Channel> {
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
    impl<T> QrtzCronTriggersBeanServiceClient<T>
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
        ) -> QrtzCronTriggersBeanServiceClient<InterceptedService<T, F>>
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
            QrtzCronTriggersBeanServiceClient::new(InterceptedService::new(inner, interceptor))
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

        pub async fn list_qrtz_cron_triggers_beans(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQrtzCronTriggersBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQrtzCronTriggersBeansResponse>,
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
                "/qrtz_cron_triggers.QrtzCronTriggersBeanService/ListQrtzCronTriggersBeans",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_cron_triggers.QrtzCronTriggersBeanService",
                "ListQrtzCronTriggersBeans",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn get_qrtz_cron_triggers_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQrtzCronTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzCronTriggersBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_cron_triggers.QrtzCronTriggersBeanService/GetQrtzCronTriggersBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_cron_triggers.QrtzCronTriggersBeanService",
                "GetQrtzCronTriggersBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn create_qrtz_cron_triggers_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateQrtzCronTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzCronTriggersBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_cron_triggers.QrtzCronTriggersBeanService/CreateQrtzCronTriggersBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_cron_triggers.QrtzCronTriggersBeanService",
                "CreateQrtzCronTriggersBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn update_qrtz_cron_triggers_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateQrtzCronTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzCronTriggersBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_cron_triggers.QrtzCronTriggersBeanService/UpdateQrtzCronTriggersBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_cron_triggers.QrtzCronTriggersBeanService",
                "UpdateQrtzCronTriggersBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn delete_qrtz_cron_triggers_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteQrtzCronTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qrtz_cron_triggers.QrtzCronTriggersBeanService/DeleteQrtzCronTriggersBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "qrtz_cron_triggers.QrtzCronTriggersBeanService",
                "DeleteQrtzCronTriggersBean",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod qrtz_cron_triggers_bean_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QrtzCronTriggersBeanServiceServer.
    #[async_trait]
    pub trait QrtzCronTriggersBeanService: Send + Sync + 'static {
        async fn list_qrtz_cron_triggers_beans(
            &self,
            request: tonic::Request<super::ListQrtzCronTriggersBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQrtzCronTriggersBeansResponse>,
            tonic::Status,
        >;
        async fn get_qrtz_cron_triggers_bean(
            &self,
            request: tonic::Request<super::GetQrtzCronTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzCronTriggersBean>, tonic::Status>;
        async fn create_qrtz_cron_triggers_bean(
            &self,
            request: tonic::Request<super::CreateQrtzCronTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzCronTriggersBean>, tonic::Status>;
        async fn update_qrtz_cron_triggers_bean(
            &self,
            request: tonic::Request<super::UpdateQrtzCronTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::QrtzCronTriggersBean>, tonic::Status>;
        async fn delete_qrtz_cron_triggers_bean(
            &self,
            request: tonic::Request<super::DeleteQrtzCronTriggersBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct QrtzCronTriggersBeanServiceServer<T: QrtzCronTriggersBeanService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: QrtzCronTriggersBeanService> QrtzCronTriggersBeanServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QrtzCronTriggersBeanServiceServer<T>
    where
        T: QrtzCronTriggersBeanService,
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
                "/qrtz_cron_triggers.QrtzCronTriggersBeanService/ListQrtzCronTriggersBeans" => {
                    #[allow(non_camel_case_types)]
                    struct ListQrtzCronTriggersBeansSvc<T: QrtzCronTriggersBeanService>(pub Arc<T>);
                    impl<T: QrtzCronTriggersBeanService>
                        tonic::server::UnaryService<super::ListQrtzCronTriggersBeansRequest>
                        for ListQrtzCronTriggersBeansSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::ListQrtzCronTriggersBeansResponse;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListQrtzCronTriggersBeansRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_qrtz_cron_triggers_beans(request).await
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
                        let method = ListQrtzCronTriggersBeansSvc(inner);
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
                "/qrtz_cron_triggers.QrtzCronTriggersBeanService/GetQrtzCronTriggersBean" => {
                    #[allow(non_camel_case_types)]
                    struct GetQrtzCronTriggersBeanSvc<T: QrtzCronTriggersBeanService>(pub Arc<T>);
                    impl<T: QrtzCronTriggersBeanService>
                        tonic::server::UnaryService<super::GetQrtzCronTriggersBeanRequest>
                        for GetQrtzCronTriggersBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::QrtzCronTriggersBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetQrtzCronTriggersBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).get_qrtz_cron_triggers_bean(request).await };
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
                        let method = GetQrtzCronTriggersBeanSvc(inner);
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
                "/qrtz_cron_triggers.QrtzCronTriggersBeanService/CreateQrtzCronTriggersBean" => {
                    #[allow(non_camel_case_types)]
                    struct CreateQrtzCronTriggersBeanSvc<T: QrtzCronTriggersBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: QrtzCronTriggersBeanService>
                        tonic::server::UnaryService<super::CreateQrtzCronTriggersBeanRequest>
                        for CreateQrtzCronTriggersBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::QrtzCronTriggersBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateQrtzCronTriggersBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_qrtz_cron_triggers_bean(request).await
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
                        let method = CreateQrtzCronTriggersBeanSvc(inner);
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
                "/qrtz_cron_triggers.QrtzCronTriggersBeanService/UpdateQrtzCronTriggersBean" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateQrtzCronTriggersBeanSvc<T: QrtzCronTriggersBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: QrtzCronTriggersBeanService>
                        tonic::server::UnaryService<super::UpdateQrtzCronTriggersBeanRequest>
                        for UpdateQrtzCronTriggersBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::QrtzCronTriggersBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateQrtzCronTriggersBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_qrtz_cron_triggers_bean(request).await
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
                        let method = UpdateQrtzCronTriggersBeanSvc(inner);
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
                "/qrtz_cron_triggers.QrtzCronTriggersBeanService/DeleteQrtzCronTriggersBean" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteQrtzCronTriggersBeanSvc<T: QrtzCronTriggersBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: QrtzCronTriggersBeanService>
                        tonic::server::UnaryService<super::DeleteQrtzCronTriggersBeanRequest>
                        for DeleteQrtzCronTriggersBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = ();

                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteQrtzCronTriggersBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_qrtz_cron_triggers_bean(request).await
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
                        let method = DeleteQrtzCronTriggersBeanSvc(inner);
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
    impl<T: QrtzCronTriggersBeanService> Clone for QrtzCronTriggersBeanServiceServer<T> {
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
    impl<T: QrtzCronTriggersBeanService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: QrtzCronTriggersBeanService> tonic::server::NamedService
        for QrtzCronTriggersBeanServiceServer<T>
    {
        const NAME: &'static str = "qrtz_cron_triggers.QrtzCronTriggersBeanService";
    }
}
