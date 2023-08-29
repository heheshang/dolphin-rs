#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsDqComparisonTypeBean {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub execute_sql: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub output_table: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "8")]
    pub is_inner_source: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsDqComparisonTypeBeansRequest {
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
pub struct ListDsDqComparisonTypeBeansResponse {
    /// The field name should match the noun "DsDqComparisonTypeBean" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub ds_dq_comparison_type_beans: ::prost::alloc::vec::Vec<DsDqComparisonTypeBean>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsDqComparisonTypeBeanRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDsDqComparisonTypeBeanRequest {
    /// The parent resource name where the DsDqComparisonTypeBean is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The DsDqComparisonTypeBean id to use for this DsDqComparisonTypeBean.
    #[prost(string, tag = "2")]
    pub ds_dq_comparison_type_bean_id: ::prost::alloc::string::String,
    /// The DsDqComparisonTypeBean resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub ds_dq_comparison_type_bean: ::core::option::Option<DsDqComparisonTypeBean>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDsDqComparisonTypeBeanRequest {
    /// The DsDqComparisonTypeBean resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub ds_dq_comparison_type_bean: ::core::option::Option<DsDqComparisonTypeBean>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsDqComparisonTypeBeanRequest {
    /// The resource name of the DsDqComparisonTypeBean to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod ds_dq_comparison_type_bean_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct DsDqComparisonTypeBeanServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsDqComparisonTypeBeanServiceClient<tonic::transport::Channel> {
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
    impl<T> DsDqComparisonTypeBeanServiceClient<T>
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
        ) -> DsDqComparisonTypeBeanServiceClient<InterceptedService<T, F>>
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
            DsDqComparisonTypeBeanServiceClient::new(InterceptedService::new(inner, interceptor))
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

        pub async fn list_ds_dq_comparison_type_beans(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDsDqComparisonTypeBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDsDqComparisonTypeBeansResponse>,
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
                "/ds_dq_comparison_type.DsDqComparisonTypeBeanService/ListDsDqComparisonTypeBeans",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_dq_comparison_type.DsDqComparisonTypeBeanService",
                "ListDsDqComparisonTypeBeans",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn get_ds_dq_comparison_type_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDsDqComparisonTypeBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsDqComparisonTypeBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_dq_comparison_type.DsDqComparisonTypeBeanService/GetDsDqComparisonTypeBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_dq_comparison_type.DsDqComparisonTypeBeanService",
                "GetDsDqComparisonTypeBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn create_ds_dq_comparison_type_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDsDqComparisonTypeBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsDqComparisonTypeBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_dq_comparison_type.DsDqComparisonTypeBeanService/CreateDsDqComparisonTypeBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_dq_comparison_type.DsDqComparisonTypeBeanService",
                "CreateDsDqComparisonTypeBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn update_ds_dq_comparison_type_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDsDqComparisonTypeBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsDqComparisonTypeBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_dq_comparison_type.DsDqComparisonTypeBeanService/UpdateDsDqComparisonTypeBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_dq_comparison_type.DsDqComparisonTypeBeanService",
                "UpdateDsDqComparisonTypeBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn delete_ds_dq_comparison_type_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDsDqComparisonTypeBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_dq_comparison_type.DsDqComparisonTypeBeanService/DeleteDsDqComparisonTypeBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_dq_comparison_type.DsDqComparisonTypeBeanService",
                "DeleteDsDqComparisonTypeBean",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ds_dq_comparison_type_bean_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DsDqComparisonTypeBeanServiceServer.
    #[async_trait]
    pub trait DsDqComparisonTypeBeanService: Send + Sync + 'static {
        async fn list_ds_dq_comparison_type_beans(
            &self,
            request: tonic::Request<super::ListDsDqComparisonTypeBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDsDqComparisonTypeBeansResponse>,
            tonic::Status,
        >;
        async fn get_ds_dq_comparison_type_bean(
            &self,
            request: tonic::Request<super::GetDsDqComparisonTypeBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsDqComparisonTypeBean>, tonic::Status>;
        async fn create_ds_dq_comparison_type_bean(
            &self,
            request: tonic::Request<super::CreateDsDqComparisonTypeBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsDqComparisonTypeBean>, tonic::Status>;
        async fn update_ds_dq_comparison_type_bean(
            &self,
            request: tonic::Request<super::UpdateDsDqComparisonTypeBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsDqComparisonTypeBean>, tonic::Status>;
        async fn delete_ds_dq_comparison_type_bean(
            &self,
            request: tonic::Request<super::DeleteDsDqComparisonTypeBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct DsDqComparisonTypeBeanServiceServer<T: DsDqComparisonTypeBeanService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DsDqComparisonTypeBeanService> DsDqComparisonTypeBeanServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DsDqComparisonTypeBeanServiceServer<T>
    where
        T: DsDqComparisonTypeBeanService,
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
                "/ds_dq_comparison_type.DsDqComparisonTypeBeanService/\
                 ListDsDqComparisonTypeBeans" => {
                    #[allow(non_camel_case_types)]
                    struct ListDsDqComparisonTypeBeansSvc<T: DsDqComparisonTypeBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsDqComparisonTypeBeanService>
                        tonic::server::UnaryService<super::ListDsDqComparisonTypeBeansRequest>
                        for ListDsDqComparisonTypeBeansSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::ListDsDqComparisonTypeBeansResponse;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDsDqComparisonTypeBeansRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_ds_dq_comparison_type_beans(request).await
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
                        let method = ListDsDqComparisonTypeBeansSvc(inner);
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
                "/ds_dq_comparison_type.DsDqComparisonTypeBeanService/\
                 GetDsDqComparisonTypeBean" => {
                    #[allow(non_camel_case_types)]
                    struct GetDsDqComparisonTypeBeanSvc<T: DsDqComparisonTypeBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsDqComparisonTypeBeanService>
                        tonic::server::UnaryService<super::GetDsDqComparisonTypeBeanRequest>
                        for GetDsDqComparisonTypeBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsDqComparisonTypeBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDsDqComparisonTypeBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_ds_dq_comparison_type_bean(request).await
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
                        let method = GetDsDqComparisonTypeBeanSvc(inner);
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
                "/ds_dq_comparison_type.DsDqComparisonTypeBeanService/\
                 CreateDsDqComparisonTypeBean" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDsDqComparisonTypeBeanSvc<T: DsDqComparisonTypeBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsDqComparisonTypeBeanService>
                        tonic::server::UnaryService<super::CreateDsDqComparisonTypeBeanRequest>
                        for CreateDsDqComparisonTypeBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsDqComparisonTypeBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDsDqComparisonTypeBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_ds_dq_comparison_type_bean(request).await
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
                        let method = CreateDsDqComparisonTypeBeanSvc(inner);
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
                "/ds_dq_comparison_type.DsDqComparisonTypeBeanService/\
                 UpdateDsDqComparisonTypeBean" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDsDqComparisonTypeBeanSvc<T: DsDqComparisonTypeBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsDqComparisonTypeBeanService>
                        tonic::server::UnaryService<super::UpdateDsDqComparisonTypeBeanRequest>
                        for UpdateDsDqComparisonTypeBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsDqComparisonTypeBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDsDqComparisonTypeBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_ds_dq_comparison_type_bean(request).await
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
                        let method = UpdateDsDqComparisonTypeBeanSvc(inner);
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
                "/ds_dq_comparison_type.DsDqComparisonTypeBeanService/\
                 DeleteDsDqComparisonTypeBean" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDsDqComparisonTypeBeanSvc<T: DsDqComparisonTypeBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsDqComparisonTypeBeanService>
                        tonic::server::UnaryService<super::DeleteDsDqComparisonTypeBeanRequest>
                        for DeleteDsDqComparisonTypeBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = ();

                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDsDqComparisonTypeBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_ds_dq_comparison_type_bean(request).await
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
                        let method = DeleteDsDqComparisonTypeBeanSvc(inner);
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
    impl<T: DsDqComparisonTypeBeanService> Clone for DsDqComparisonTypeBeanServiceServer<T> {
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
    impl<T: DsDqComparisonTypeBeanService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DsDqComparisonTypeBeanService> tonic::server::NamedService
        for DsDqComparisonTypeBeanServiceServer<T>
    {
        const NAME: &'static str = "ds_dq_comparison_type.DsDqComparisonTypeBeanService";
    }
}
