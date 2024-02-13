/// gRPC request payload for tether.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgressRequest {
    /// Unique identifier for the request.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Actual payload to send to agent.
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<Payload>,
    /// Tether Endpoint.
    #[prost(enumeration = "TetherEndpoint", tag = "3")]
    pub endpoint: i32,
    /// GCP Project.
    /// Format: `projects/{project_number}`.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
    /// Unique identifier for clients to trace their request/response.
    #[prost(string, tag = "5")]
    pub trace_id: ::prost::alloc::string::String,
    /// Timeout for the HTTP request.
    #[prost(message, optional, tag = "6")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
}
/// Payload for EgressRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payload {
    /// The kind of payload.
    #[prost(oneof = "payload::Kind", tags = "1, 2, 3")]
    pub kind: ::core::option::Option<payload::Kind>,
}
/// Nested message and enum types in `Payload`.
pub mod payload {
    /// The kind of payload.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// The HttpRequest proto.
        #[prost(message, tag = "1")]
        HttpRequest(super::HttpRequest),
        /// The information of stream.
        #[prost(message, tag = "2")]
        StreamInfo(super::StreamInfo),
        /// The action taken by agent.
        #[prost(enumeration = "super::Action", tag = "3")]
        Action(i32),
    }
}
/// The Information of bi-directional stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamInfo {
    /// Unique identifier for the stream.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// gRPC response payload for tether.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgressResponse {
    /// Unique identifier for the response. Matches the EgressRequest's id.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// HttpResponse.
    #[prost(message, optional, tag = "2")]
    pub http_response: ::core::option::Option<HttpResponse>,
    /// Errors from application when handling the http request.
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// GCP Project.
    /// Format: `projects/{project_number}`.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
    /// Unique identifier for clients to trace their request/response. Matches the
    /// EgressRequest's trace id
    #[prost(string, tag = "5")]
    pub trace_id: ::prost::alloc::string::String,
    /// Tether Endpoint.
    #[prost(enumeration = "TetherEndpoint", tag = "6")]
    pub endpoint: i32,
    /// Name is the full resource path of endpoint.
    /// Format: `projects/{project_number or project_id}/endpoints/{endpoint}`
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
}
/// The proto definition of http request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRequest {
    /// A unique identifier for the request.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The HTTP request method.
    /// Valid methods: "GET", "HEAD", "POST", "PUT", "PATCH","DELETE".
    #[prost(string, tag = "2")]
    pub method: ::prost::alloc::string::String,
    /// The HTTP request URL.
    #[prost(message, optional, tag = "3")]
    pub url: ::core::option::Option<Url>,
    /// The HTTP request headers.
    #[prost(message, repeated, tag = "4")]
    pub headers: ::prost::alloc::vec::Vec<Header>,
    /// HTTP request body.
    #[prost(bytes = "bytes", tag = "5")]
    pub body: ::prost::bytes::Bytes,
}
/// The proto definition of url.
/// A url represents a URL and the general form represented is:
///
///   `[scheme://][google.cloud.apigeeconnect.v1.Url.host][path]`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Url {
    /// Scheme.
    #[prost(enumeration = "Scheme", tag = "1")]
    pub scheme: i32,
    /// Host or Host:Port.
    #[prost(string, tag = "2")]
    pub host: ::prost::alloc::string::String,
    /// Path starts with `/`.
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
}
/// The http headers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The proto definition of http response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpResponse {
    /// A unique identifier that matches the request ID.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Status of http response, e.g. "200 OK".
    #[prost(string, tag = "2")]
    pub status: ::prost::alloc::string::String,
    /// Status code of http response, e.g. 200.
    #[prost(int32, tag = "3")]
    pub status_code: i32,
    /// The HTTP 1.1 response body.
    #[prost(bytes = "bytes", tag = "4")]
    pub body: ::prost::bytes::Bytes,
    /// The HTTP response headers.
    #[prost(message, repeated, tag = "5")]
    pub headers: ::prost::alloc::vec::Vec<Header>,
    /// Content length records the length of the associated content. The
    /// value -1 indicates that the length is unknown. Unless http method
    /// is "HEAD", values >= 0 indicate that the given number of bytes may
    /// be read from Body.
    #[prost(int64, tag = "6")]
    pub content_length: i64,
}
/// The action taken by agent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    /// Unspecified Action.
    Unspecified = 0,
    /// Indicates that agent should open a new stream.
    OpenNewStream = 1,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::Unspecified => "ACTION_UNSPECIFIED",
            Action::OpenNewStream => "OPEN_NEW_STREAM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTION_UNSPECIFIED" => Some(Self::Unspecified),
            "OPEN_NEW_STREAM" => Some(Self::OpenNewStream),
            _ => None,
        }
    }
}
/// Endpoint indicates where the messages will be delivered.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TetherEndpoint {
    /// Unspecified tether endpoint.
    Unspecified = 0,
    /// Apigee MART endpoint.
    ApigeeMart = 1,
    /// Apigee Runtime endpoint.
    ApigeeRuntime = 2,
    /// Apigee Mint Rating endpoint.
    ApigeeMintRating = 3,
}
impl TetherEndpoint {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TetherEndpoint::Unspecified => "TETHER_ENDPOINT_UNSPECIFIED",
            TetherEndpoint::ApigeeMart => "APIGEE_MART",
            TetherEndpoint::ApigeeRuntime => "APIGEE_RUNTIME",
            TetherEndpoint::ApigeeMintRating => "APIGEE_MINT_RATING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TETHER_ENDPOINT_UNSPECIFIED" => Some(Self::Unspecified),
            "APIGEE_MART" => Some(Self::ApigeeMart),
            "APIGEE_RUNTIME" => Some(Self::ApigeeRuntime),
            "APIGEE_MINT_RATING" => Some(Self::ApigeeMintRating),
            _ => None,
        }
    }
}
/// HTTP Scheme.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Scheme {
    /// Unspecified scheme.
    Unspecified = 0,
    /// HTTPS protocol.
    Https = 1,
}
impl Scheme {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Scheme::Unspecified => "SCHEME_UNSPECIFIED",
            Scheme::Https => "HTTPS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SCHEME_UNSPECIFIED" => Some(Self::Unspecified),
            "HTTPS" => Some(Self::Https),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod tether_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Tether provides a way for the control plane to send HTTP API requests to
    /// services in data planes that runs in a remote datacenter without
    /// requiring customers to open firewalls on their runtime plane.
    #[derive(Debug, Clone)]
    pub struct TetherClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TetherClient<T>
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
        ) -> TetherClient<InterceptedService<T, F>>
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
            TetherClient::new(InterceptedService::new(inner, interceptor))
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
        /// Egress streams egress requests and responses. Logically, this is not
        /// actually a streaming request, but uses streaming as a mechanism to flip
        /// the client-server relationship of gRPC so that the server can act as a
        /// client.
        /// The listener, the RPC server, accepts connections from the dialer,
        /// the RPC client.
        /// The listener streams http requests and the dialer streams http responses.
        pub async fn egress(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::EgressResponse>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::EgressRequest>>,
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
                "/google.cloud.apigeeconnect.v1.Tether/Egress",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apigeeconnect.v1.Tether", "Egress"),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// The request for [ListConnections][Management.ListConnections].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsRequest {
    /// Required. Parent name of the form:
    ///      `projects/{project_number or project_id}/endpoints/{endpoint}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of connections to return. The service may return fewer
    /// than this value. If unspecified, at most 100 connections will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListConnections` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListConnections` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for
/// [ListConnections][Management.ListConnections].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsResponse {
    /// A list of clients.
    #[prost(message, repeated, tag = "1")]
    pub connections: ::prost::alloc::vec::Vec<Connection>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    /// The endpoint that the connection is made against.
    /// Format: `projects/{project_number}/endpoints/{endpoint}`
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    /// Cluster information.
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
    /// The count of streams.
    #[prost(int32, tag = "3")]
    pub stream_count: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// The name of the cluster.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The region of the cluster.
    #[prost(string, tag = "2")]
    pub region: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod connection_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service Interface for the Apigee Connect connection management APIs.
    #[derive(Debug, Clone)]
    pub struct ConnectionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConnectionServiceClient<T>
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
        ) -> ConnectionServiceClient<InterceptedService<T, F>>
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
            ConnectionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists connections that are currently active for the given Apigee Connect
        /// endpoint.
        pub async fn list_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConnectionsResponse>,
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
                "/google.cloud.apigeeconnect.v1.ConnectionService/ListConnections",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.apigeeconnect.v1.ConnectionService",
                        "ListConnections",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
