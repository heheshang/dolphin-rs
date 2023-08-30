#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsRelationRuleInputEntryBean {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int32, optional, tag = "2")]
    pub rule_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub rule_input_entry_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub values_map: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "5")]
    pub index: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDsRelationRuleInputEntryBeansRequest {
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
pub struct ListDsRelationRuleInputEntryBeansResponse {
    /// The field name should match the noun "DsRelationRuleInputEntryBean" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub ds_relation_rule_input_entry_beans: ::prost::alloc::vec::Vec<DsRelationRuleInputEntryBean>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDsRelationRuleInputEntryBeanRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDsRelationRuleInputEntryBeanRequest {
    /// The parent resource name where the DsRelationRuleInputEntryBean is to be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The DsRelationRuleInputEntryBean id to use for this DsRelationRuleInputEntryBean.
    #[prost(string, tag = "2")]
    pub ds_relation_rule_input_entry_bean_id: ::prost::alloc::string::String,
    /// The DsRelationRuleInputEntryBean resource to create.
    /// The field name should match the Noun in the method name.
    #[prost(message, optional, tag = "3")]
    pub ds_relation_rule_input_entry_bean: ::core::option::Option<DsRelationRuleInputEntryBean>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDsRelationRuleInputEntryBeanRequest {
    /// The DsRelationRuleInputEntryBean resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub ds_relation_rule_input_entry_bean: ::core::option::Option<DsRelationRuleInputEntryBean>,
    /// The update mask applies to the resource. For the `google.protobuf.FieldMask` definition,
    /// see <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDsRelationRuleInputEntryBeanRequest {
    /// The resource name of the DsRelationRuleInputEntryBean to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod ds_relation_rule_input_entry_bean_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug, Clone)]
    pub struct DsRelationRuleInputEntryBeanServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DsRelationRuleInputEntryBeanServiceClient<tonic::transport::Channel> {
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
    impl<T> DsRelationRuleInputEntryBeanServiceClient<T>
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
        ) -> DsRelationRuleInputEntryBeanServiceClient<InterceptedService<T, F>>
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
            DsRelationRuleInputEntryBeanServiceClient::new(InterceptedService::new(
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

        pub async fn list_ds_relation_rule_input_entry_beans(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDsRelationRuleInputEntryBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDsRelationRuleInputEntryBeansResponse>,
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
                "/ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService/\
                 ListDsRelationRuleInputEntryBeans",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService",
                "ListDsRelationRuleInputEntryBeans",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn get_ds_relation_rule_input_entry_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDsRelationRuleInputEntryBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsRelationRuleInputEntryBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService/\
                 GetDsRelationRuleInputEntryBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService",
                "GetDsRelationRuleInputEntryBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn create_ds_relation_rule_input_entry_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDsRelationRuleInputEntryBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsRelationRuleInputEntryBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService/\
                 CreateDsRelationRuleInputEntryBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService",
                "CreateDsRelationRuleInputEntryBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn update_ds_relation_rule_input_entry_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDsRelationRuleInputEntryBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsRelationRuleInputEntryBean>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService/\
                 UpdateDsRelationRuleInputEntryBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService",
                "UpdateDsRelationRuleInputEntryBean",
            ));
            self.inner.unary(req, path, codec).await
        }

        pub async fn delete_ds_relation_rule_input_entry_bean(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDsRelationRuleInputEntryBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService/\
                 DeleteDsRelationRuleInputEntryBean",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService",
                "DeleteDsRelationRuleInputEntryBean",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ds_relation_rule_input_entry_bean_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DsRelationRuleInputEntryBeanServiceServer.
    #[async_trait]
    pub trait DsRelationRuleInputEntryBeanService: Send + Sync + 'static {
        async fn list_ds_relation_rule_input_entry_beans(
            &self,
            request: tonic::Request<super::ListDsRelationRuleInputEntryBeansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDsRelationRuleInputEntryBeansResponse>,
            tonic::Status,
        >;
        async fn get_ds_relation_rule_input_entry_bean(
            &self,
            request: tonic::Request<super::GetDsRelationRuleInputEntryBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsRelationRuleInputEntryBean>, tonic::Status>;
        async fn create_ds_relation_rule_input_entry_bean(
            &self,
            request: tonic::Request<super::CreateDsRelationRuleInputEntryBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsRelationRuleInputEntryBean>, tonic::Status>;
        async fn update_ds_relation_rule_input_entry_bean(
            &self,
            request: tonic::Request<super::UpdateDsRelationRuleInputEntryBeanRequest>,
        ) -> std::result::Result<tonic::Response<super::DsRelationRuleInputEntryBean>, tonic::Status>;
        async fn delete_ds_relation_rule_input_entry_bean(
            &self,
            request: tonic::Request<super::DeleteDsRelationRuleInputEntryBeanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Generated according to https://cloud.google.com/apis/design/standard_methods
    #[derive(Debug)]
    pub struct DsRelationRuleInputEntryBeanServiceServer<T: DsRelationRuleInputEntryBeanService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DsRelationRuleInputEntryBeanService> DsRelationRuleInputEntryBeanServiceServer<T> {
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
        for DsRelationRuleInputEntryBeanServiceServer<T>
    where
        T: DsRelationRuleInputEntryBeanService,
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
                "/ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService/\
                 ListDsRelationRuleInputEntryBeans" => {
                    #[allow(non_camel_case_types)]
                    struct ListDsRelationRuleInputEntryBeansSvc<T: DsRelationRuleInputEntryBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsRelationRuleInputEntryBeanService>
                        tonic::server::UnaryService<super::ListDsRelationRuleInputEntryBeansRequest>
                        for ListDsRelationRuleInputEntryBeansSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::ListDsRelationRuleInputEntryBeansResponse;

                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListDsRelationRuleInputEntryBeansRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .list_ds_relation_rule_input_entry_beans(request)
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
                        let method = ListDsRelationRuleInputEntryBeansSvc(inner);
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
                "/ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService/\
                 GetDsRelationRuleInputEntryBean" => {
                    #[allow(non_camel_case_types)]
                    struct GetDsRelationRuleInputEntryBeanSvc<T: DsRelationRuleInputEntryBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsRelationRuleInputEntryBeanService>
                        tonic::server::UnaryService<super::GetDsRelationRuleInputEntryBeanRequest>
                        for GetDsRelationRuleInputEntryBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsRelationRuleInputEntryBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDsRelationRuleInputEntryBeanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_ds_relation_rule_input_entry_bean(request)
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
                        let method = GetDsRelationRuleInputEntryBeanSvc(inner);
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
                "/ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService/\
                 CreateDsRelationRuleInputEntryBean" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDsRelationRuleInputEntryBeanSvc<T: DsRelationRuleInputEntryBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsRelationRuleInputEntryBeanService>
                        tonic::server::UnaryService<
                            super::CreateDsRelationRuleInputEntryBeanRequest,
                        > for CreateDsRelationRuleInputEntryBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsRelationRuleInputEntryBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateDsRelationRuleInputEntryBeanRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .create_ds_relation_rule_input_entry_bean(request)
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
                        let method = CreateDsRelationRuleInputEntryBeanSvc(inner);
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
                "/ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService/\
                 UpdateDsRelationRuleInputEntryBean" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDsRelationRuleInputEntryBeanSvc<T: DsRelationRuleInputEntryBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsRelationRuleInputEntryBeanService>
                        tonic::server::UnaryService<
                            super::UpdateDsRelationRuleInputEntryBeanRequest,
                        > for UpdateDsRelationRuleInputEntryBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::DsRelationRuleInputEntryBean;

                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateDsRelationRuleInputEntryBeanRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .update_ds_relation_rule_input_entry_bean(request)
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
                        let method = UpdateDsRelationRuleInputEntryBeanSvc(inner);
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
                "/ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService/\
                 DeleteDsRelationRuleInputEntryBean" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDsRelationRuleInputEntryBeanSvc<T: DsRelationRuleInputEntryBeanService>(
                        pub Arc<T>,
                    );
                    impl<T: DsRelationRuleInputEntryBeanService>
                        tonic::server::UnaryService<
                            super::DeleteDsRelationRuleInputEntryBeanRequest,
                        > for DeleteDsRelationRuleInputEntryBeanSvc<T>
                    {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = ();

                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteDsRelationRuleInputEntryBeanRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .delete_ds_relation_rule_input_entry_bean(request)
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
                        let method = DeleteDsRelationRuleInputEntryBeanSvc(inner);
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
    impl<T: DsRelationRuleInputEntryBeanService> Clone
        for DsRelationRuleInputEntryBeanServiceServer<T>
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
    impl<T: DsRelationRuleInputEntryBeanService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DsRelationRuleInputEntryBeanService> tonic::server::NamedService
        for DsRelationRuleInputEntryBeanServiceServer<T>
    {
        const NAME: &'static str =
            "ds_relation_rule_input_entry.DsRelationRuleInputEntryBeanService";
    }
}
