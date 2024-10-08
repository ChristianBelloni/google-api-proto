// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAccessTokenRequest {
    /// Required. The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The sequence of service accounts in a delegation chain. Each service
    /// account must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on its next service account in the chain. The last service account in the
    /// chain must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on the service account that is specified in the `name` field of the
    /// request.
    ///
    /// The delegates must have the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, repeated, tag = "2")]
    pub delegates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Code to identify the scopes to be included in the OAuth 2.0 access token.
    /// See <https://developers.google.com/identity/protocols/googlescopes> for more
    /// information.
    /// At least one value required.
    #[prost(string, repeated, tag = "4")]
    pub scope: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The desired lifetime duration of the access token in seconds.
    /// Must be set to a value less than or equal to 3600 (1 hour). If a value is
    /// not specified, the token's lifetime will be set to a default value of one
    /// hour.
    #[prost(message, optional, tag = "7")]
    pub lifetime: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAccessTokenResponse {
    /// The OAuth 2.0 access token.
    #[prost(string, tag = "1")]
    pub access_token: ::prost::alloc::string::String,
    /// Token expiration time.
    /// The expiration time is always set.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignBlobRequest {
    /// Required. The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The sequence of service accounts in a delegation chain. Each service
    /// account must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on its next service account in the chain. The last service account in the
    /// chain must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on the service account that is specified in the `name` field of the
    /// request.
    ///
    /// The delegates must have the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, repeated, tag = "3")]
    pub delegates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The bytes to sign.
    #[prost(bytes = "bytes", tag = "5")]
    pub payload: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignBlobResponse {
    /// The ID of the key used to sign the blob.
    #[prost(string, tag = "1")]
    pub key_id: ::prost::alloc::string::String,
    /// The signed blob.
    #[prost(bytes = "bytes", tag = "4")]
    pub signed_blob: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignJwtRequest {
    /// Required. The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The sequence of service accounts in a delegation chain. Each service
    /// account must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on its next service account in the chain. The last service account in the
    /// chain must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on the service account that is specified in the `name` field of the
    /// request.
    ///
    /// The delegates must have the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, repeated, tag = "3")]
    pub delegates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The JWT payload to sign: a JSON object that contains a JWT Claims Set.
    #[prost(string, tag = "5")]
    pub payload: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignJwtResponse {
    /// The ID of the key used to sign the JWT.
    #[prost(string, tag = "1")]
    pub key_id: ::prost::alloc::string::String,
    /// The signed JWT.
    #[prost(string, tag = "2")]
    pub signed_jwt: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateIdTokenRequest {
    /// Required. The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The sequence of service accounts in a delegation chain. Each service
    /// account must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on its next service account in the chain. The last service account in the
    /// chain must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on the service account that is specified in the `name` field of the
    /// request.
    ///
    /// The delegates must have the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, repeated, tag = "2")]
    pub delegates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The audience for the token, such as the API or account that this token
    /// grants access to.
    #[prost(string, tag = "3")]
    pub audience: ::prost::alloc::string::String,
    /// Include the service account email in the token. If set to `true`, the
    /// token will contain `email` and `email_verified` claims.
    #[prost(bool, tag = "4")]
    pub include_email: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateIdTokenResponse {
    /// The OpenId Connect ID token.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod iam_credentials_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service account is a special type of Google account that belongs to your
    /// application or a virtual machine (VM), instead of to an individual end user.
    /// Your application assumes the identity of the service account to call Google
    /// APIs, so that the users aren't directly involved.
    ///
    /// Service account credentials are used to temporarily assume the identity
    /// of the service account. Supported credential types include OAuth 2.0 access
    /// tokens, OpenID Connect ID tokens, self-signed JSON Web Tokens (JWTs), and
    /// more.
    #[derive(Debug, Clone)]
    pub struct IamCredentialsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> IamCredentialsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> IamCredentialsClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            IamCredentialsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Generates an OAuth 2.0 access token for a service account.
        pub async fn generate_access_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateAccessTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateAccessTokenResponse>,
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
                "/google.iam.credentials.v1.IAMCredentials/GenerateAccessToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.iam.credentials.v1.IAMCredentials",
                        "GenerateAccessToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates an OpenID Connect ID token for a service account.
        pub async fn generate_id_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateIdTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateIdTokenResponse>,
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
                "/google.iam.credentials.v1.IAMCredentials/GenerateIdToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.iam.credentials.v1.IAMCredentials",
                        "GenerateIdToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Signs a blob using a service account's system-managed private key.
        pub async fn sign_blob(
            &mut self,
            request: impl tonic::IntoRequest<super::SignBlobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignBlobResponse>,
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
                "/google.iam.credentials.v1.IAMCredentials/SignBlob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.iam.credentials.v1.IAMCredentials",
                        "SignBlob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Signs a JWT using a service account's system-managed private key.
        pub async fn sign_jwt(
            &mut self,
            request: impl tonic::IntoRequest<super::SignJwtRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignJwtResponse>,
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
                "/google.iam.credentials.v1.IAMCredentials/SignJwt",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.iam.credentials.v1.IAMCredentials",
                        "SignJwt",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod iam_credentials_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with IamCredentialsServer.
    #[async_trait]
    pub trait IamCredentials: std::marker::Send + std::marker::Sync + 'static {
        /// Generates an OAuth 2.0 access token for a service account.
        async fn generate_access_token(
            &self,
            request: tonic::Request<super::GenerateAccessTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateAccessTokenResponse>,
            tonic::Status,
        >;
        /// Generates an OpenID Connect ID token for a service account.
        async fn generate_id_token(
            &self,
            request: tonic::Request<super::GenerateIdTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateIdTokenResponse>,
            tonic::Status,
        >;
        /// Signs a blob using a service account's system-managed private key.
        async fn sign_blob(
            &self,
            request: tonic::Request<super::SignBlobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignBlobResponse>,
            tonic::Status,
        >;
        /// Signs a JWT using a service account's system-managed private key.
        async fn sign_jwt(
            &self,
            request: tonic::Request<super::SignJwtRequest>,
        ) -> std::result::Result<tonic::Response<super::SignJwtResponse>, tonic::Status>;
    }
    /// A service account is a special type of Google account that belongs to your
    /// application or a virtual machine (VM), instead of to an individual end user.
    /// Your application assumes the identity of the service account to call Google
    /// APIs, so that the users aren't directly involved.
    ///
    /// Service account credentials are used to temporarily assume the identity
    /// of the service account. Supported credential types include OAuth 2.0 access
    /// tokens, OpenID Connect ID tokens, self-signed JSON Web Tokens (JWTs), and
    /// more.
    #[derive(Debug)]
    pub struct IamCredentialsServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> IamCredentialsServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for IamCredentialsServer<T>
    where
        T: IamCredentials,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
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
            match req.uri().path() {
                "/google.iam.credentials.v1.IAMCredentials/GenerateAccessToken" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateAccessTokenSvc<T: IamCredentials>(pub Arc<T>);
                    impl<
                        T: IamCredentials,
                    > tonic::server::UnaryService<super::GenerateAccessTokenRequest>
                    for GenerateAccessTokenSvc<T> {
                        type Response = super::GenerateAccessTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateAccessTokenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IamCredentials>::generate_access_token(
                                        &inner,
                                        request,
                                    )
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
                        let method = GenerateAccessTokenSvc(inner);
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
                "/google.iam.credentials.v1.IAMCredentials/GenerateIdToken" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateIdTokenSvc<T: IamCredentials>(pub Arc<T>);
                    impl<
                        T: IamCredentials,
                    > tonic::server::UnaryService<super::GenerateIdTokenRequest>
                    for GenerateIdTokenSvc<T> {
                        type Response = super::GenerateIdTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateIdTokenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IamCredentials>::generate_id_token(&inner, request)
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
                        let method = GenerateIdTokenSvc(inner);
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
                "/google.iam.credentials.v1.IAMCredentials/SignBlob" => {
                    #[allow(non_camel_case_types)]
                    struct SignBlobSvc<T: IamCredentials>(pub Arc<T>);
                    impl<
                        T: IamCredentials,
                    > tonic::server::UnaryService<super::SignBlobRequest>
                    for SignBlobSvc<T> {
                        type Response = super::SignBlobResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignBlobRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IamCredentials>::sign_blob(&inner, request).await
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
                        let method = SignBlobSvc(inner);
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
                "/google.iam.credentials.v1.IAMCredentials/SignJwt" => {
                    #[allow(non_camel_case_types)]
                    struct SignJwtSvc<T: IamCredentials>(pub Arc<T>);
                    impl<
                        T: IamCredentials,
                    > tonic::server::UnaryService<super::SignJwtRequest>
                    for SignJwtSvc<T> {
                        type Response = super::SignJwtResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignJwtRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as IamCredentials>::sign_jwt(&inner, request).await
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
                        let method = SignJwtSvc(inner);
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
                                .header("grpc-status", tonic::Code::Unimplemented as i32)
                                .header(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                )
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T> Clone for IamCredentialsServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "google.iam.credentials.v1.IAMCredentials";
    impl<T> tonic::server::NamedService for IamCredentialsServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
