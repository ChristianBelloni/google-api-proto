/// A container for [services][google.cloud.servicedirectory.v1beta1.Service].
/// Namespaces allow administrators to group services together and define
/// permissions for a collection of services.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Namespace {
    /// Immutable. The resource name for the namespace in the format
    /// `projects/*/locations/*/namespaces/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Resource labels associated with this namespace.
    /// No more than 64 user labels can be associated with a given resource. Label
    /// keys and values can be no longer than 63 characters.
    #[prost(btree_map = "string, string", tag = "2")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The timestamp when the namespace was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the namespace was last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A globally unique identifier (in UUID4 format) for this
    /// namespace.
    #[prost(string, tag = "6")]
    pub uid: ::prost::alloc::string::String,
}
/// An individual endpoint that provides a
/// [service][google.cloud.servicedirectory.v1beta1.Service]. The service must
/// already exist to create an endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    /// Immutable. The resource name for the endpoint in the format
    /// `projects/*/locations/*/namespaces/*/services/*/endpoints/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An IPv4 or IPv6 address. Service Directory rejects bad addresses
    /// like:
    ///
    /// *   `8.8.8`
    /// *   `8.8.8.8:53`
    /// *   `test:bad:address`
    /// *   `\[::1\]`
    /// *   `\[::1\]:8080`
    ///
    /// Limited to 45 characters.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// Optional. Service Directory rejects values outside of `\[0, 65535\]`.
    #[prost(int32, tag = "3")]
    pub port: i32,
    /// Optional. Metadata for the endpoint. This data can be consumed by service
    /// clients.
    ///
    /// Restrictions:
    ///
    /// *   The entire metadata dictionary may contain up to 512 characters,
    ///      spread accoss all key-value pairs. Metadata that goes beyond this
    ///      limit are rejected
    /// *   Valid metadata keys have two segments: an optional prefix and name,
    ///      separated by a slash (/). The name segment is required and must be 63
    ///      characters or less, beginning and ending with an alphanumeric character
    ///      (\[a-z0-9A-Z\]) with dashes (-), underscores (_), dots (.), and
    ///      alphanumerics between. The prefix is optional. If specified, the prefix
    ///      must be a DNS subdomain: a series of DNS labels separated by dots (.),
    ///      not longer than 253 characters in total, followed by a slash (/).
    ///      Metadata that fails to meet these requirements are rejected
    ///
    /// Note: This field is equivalent to the `annotations` field in the v1 API.
    /// They have the same syntax and read/write to the same location in Service
    /// Directory.
    #[prost(btree_map = "string, string", tag = "4")]
    pub metadata: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Immutable. The Google Compute Engine network (VPC) of the endpoint in the
    /// format `projects/<project number>/locations/global/networks/*`.
    ///
    /// The project must be specified by project number (project id is rejected).
    /// Incorrectly formatted networks are rejected, but no other validation
    /// is performed on this field (ex. network or project existence, reachability,
    /// or permissions).
    #[prost(string, tag = "5")]
    pub network: ::prost::alloc::string::String,
    /// Output only. The timestamp when the endpoint was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the endpoint was last updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A globally unique identifier (in UUID4 format) for this
    /// endpoint.
    #[prost(string, tag = "8")]
    pub uid: ::prost::alloc::string::String,
}
/// An individual service. A service contains a name and optional metadata.
/// A service must exist before
/// [endpoints][google.cloud.servicedirectory.v1beta1.Endpoint] can be
/// added to it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// Immutable. The resource name for the service in the format
    /// `projects/*/locations/*/namespaces/*/services/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Metadata for the service. This data can be consumed by service
    /// clients.
    ///
    /// Restrictions:
    ///
    /// *   The entire metadata dictionary may contain up to 2000 characters,
    ///      spread accoss all key-value pairs. Metadata that goes beyond this
    ///      limit are rejected
    /// *   Valid metadata keys have two segments: an optional prefix and name,
    ///      separated by a slash (/). The name segment is required and must be 63
    ///      characters or less, beginning and ending with an alphanumeric character
    ///      (\[a-z0-9A-Z\]) with dashes (-), underscores (_), dots (.), and
    ///      alphanumerics between. The prefix is optional. If specified, the prefix
    ///      must be a DNS subdomain: a series of DNS labels separated by dots (.),
    ///      not longer than 253 characters in total, followed by a slash (/).
    ///      Metadata that fails to meet these requirements are rejected
    ///
    /// Note: This field is equivalent to the `annotations` field in the v1 API.
    /// They have the same syntax and read/write to the same location in Service
    /// Directory.
    #[prost(btree_map = "string, string", tag = "2")]
    pub metadata: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Endpoints associated with this service. Returned on
    /// [LookupService.ResolveService][google.cloud.servicedirectory.v1beta1.LookupService.ResolveService].
    /// Control plane clients should use
    /// [RegistrationService.ListEndpoints][google.cloud.servicedirectory.v1beta1.RegistrationService.ListEndpoints].
    #[prost(message, repeated, tag = "3")]
    pub endpoints: ::prost::alloc::vec::Vec<Endpoint>,
    /// Output only. The timestamp when the service was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the service was last updated. Note:
    /// endpoints being created/deleted/updated within the service are not
    /// considered service updates for the purpose of this timestamp.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A globally unique identifier (in UUID4 format) for this
    /// service.
    #[prost(string, tag = "8")]
    pub uid: ::prost::alloc::string::String,
}
/// The request message for
/// [LookupService.ResolveService][google.cloud.servicedirectory.v1beta1.LookupService.ResolveService].
/// Looks up a service by its name, returns the service and its endpoints.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveServiceRequest {
    /// Required. The name of the service to resolve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The maximum number of endpoints to return. Defaults to 25.
    /// Maximum is 100. If a value less than one is specified, the Default is used.
    /// If a value greater than the Maximum is specified, the Maximum is used.
    #[prost(int32, tag = "2")]
    pub max_endpoints: i32,
    /// Optional. The filter applied to the endpoints of the resolved service.
    ///
    /// General `filter` string syntax:
    /// `<field> <operator> <value> (<logical connector>)`
    ///
    /// *   `<field>` can be `name`, `address`, `port`, or `metadata.<key>` for
    ///      map field
    /// *   `<operator>` can be `<`, `>`, `<=`, `>=`, `!=`, `=`, `:`. Of which `:`
    ///      means `HAS`, and is roughly the same as `=`
    /// *   `<value>` must be the same data type as field
    /// *   `<logical connector>` can be `AND`, `OR`, `NOT`
    ///
    /// Examples of valid filters:
    ///
    /// *   `metadata.owner` returns endpoints that have a annotation with the key
    ///      `owner`, this is the same as `metadata:owner`
    /// *   `metadata.protocol=gRPC` returns endpoints that have key/value
    ///      `protocol=gRPC`
    /// *   `address=192.108.1.105` returns endpoints that have this address
    /// *   `port>8080` returns endpoints that have port number larger than 8080
    /// *
    /// `name>projects/my-project/locations/us-east1/namespaces/my-namespace/services/my-service/endpoints/endpoint-c`
    ///      returns endpoints that have name that is alphabetically later than the
    ///      string, so "endpoint-e" is returned but "endpoint-a" is not
    /// *
    /// `name=projects/my-project/locations/us-central1/namespaces/my-namespace/services/my-service/endpoints/ep-1`
    ///       returns the endpoint that has an endpoint_id equal to `ep-1`
    /// *   `metadata.owner!=sd AND metadata.foo=bar` returns endpoints that have
    ///      `owner` in annotation key but value is not `sd` AND have key/value
    ///       `foo=bar`
    /// *   `doesnotexist.foo=bar` returns an empty list. Note that endpoint
    ///      doesn't have a field called "doesnotexist". Since the filter does not
    ///      match any endpoint, it returns no results
    ///
    /// For more information about filtering, see
    /// [API Filtering](<https://aip.dev/160>).
    #[prost(string, tag = "3")]
    pub endpoint_filter: ::prost::alloc::string::String,
}
/// The response message for
/// [LookupService.ResolveService][google.cloud.servicedirectory.v1beta1.LookupService.ResolveService].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveServiceResponse {
    #[prost(message, optional, tag = "1")]
    pub service: ::core::option::Option<Service>,
}
/// Generated client implementations.
pub mod lookup_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service Directory API for looking up service data at runtime.
    #[derive(Debug, Clone)]
    pub struct LookupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LookupServiceClient<T>
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
        ) -> LookupServiceClient<InterceptedService<T, F>>
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
            LookupServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns a [service][google.cloud.servicedirectory.v1beta1.Service] and its
        /// associated endpoints.
        /// Resolving a service is not considered an active developer method.
        pub async fn resolve_service(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveServiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveServiceResponse>,
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
                "/google.cloud.servicedirectory.v1beta1.LookupService/ResolveService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.LookupService",
                        "ResolveService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The request message for
/// [RegistrationService.CreateNamespace][google.cloud.servicedirectory.v1beta1.RegistrationService.CreateNamespace].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNamespaceRequest {
    /// Required. The resource name of the project and location the namespace
    /// will be created in.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Resource ID must be 1-63 characters long, and comply with
    /// <a href="<https://www.ietf.org/rfc/rfc1035.txt"> target="_blank">RFC1035</a>.
    /// Specifically, the name must be 1-63 characters long and match the regular
    /// expression `[a-z](?:\[-a-z0-9\]{0,61}\[a-z0-9\])?` which means the first
    /// character must be a lowercase letter, and all following characters must
    /// be a dash, lowercase letter, or digit, except the last character, which
    /// cannot be a dash.
    #[prost(string, tag = "2")]
    pub namespace_id: ::prost::alloc::string::String,
    /// Required. A namespace with initial fields set.
    #[prost(message, optional, tag = "3")]
    pub namespace: ::core::option::Option<Namespace>,
}
/// The request message for
/// [RegistrationService.ListNamespaces][google.cloud.servicedirectory.v1beta1.RegistrationService.ListNamespaces].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNamespacesRequest {
    /// Required. The resource name of the project and location whose namespaces
    /// you'd like to list.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous List request,
    /// if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to list results by.
    ///
    /// General `filter` string syntax:
    /// `<field> <operator> <value> (<logical connector>)`
    ///
    /// *   `<field>` can be `name`, `labels.<key>` for map field, or
    /// `attributes.<field>` for attributes field
    /// *   `<operator>` can be `<`, `>`, `<=`, `>=`, `!=`, `=`, `:`. Of which `:`
    ///      means `HAS`, and is roughly the same as `=`
    /// *   `<value>` must be the same data type as field
    /// *   `<logical connector>` can be `AND`, `OR`, `NOT`
    ///
    /// Examples of valid filters:
    ///
    /// *   `labels.owner` returns namespaces that have a label with the key
    ///      `owner`, this is the same as `labels:owner`
    /// *   `labels.owner=sd` returns namespaces that have key/value `owner=sd`
    /// *   `name>projects/my-project/locations/us-east1/namespaces/namespace-c`
    ///      returns namespaces that have name that is alphabetically later than the
    ///      string, so "namespace-e" is returned but "namespace-a" is not
    /// *   `labels.owner!=sd AND labels.foo=bar` returns namespaces that have
    ///      `owner` in label key but value is not `sd` AND have key/value `foo=bar`
    /// *   `doesnotexist.foo=bar` returns an empty list. Note that namespace
    ///      doesn't have a field called "doesnotexist". Since the filter does not
    ///      match any namespaces, it returns no results
    /// *   `attributes.managed_registration=true` returns namespaces that are
    ///      managed by a GCP product or service
    ///
    /// For more information about filtering, see
    /// [API Filtering](<https://aip.dev/160>).
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The order to list results by.
    ///
    /// General `order_by` string syntax: `<field> (<asc|desc>) (,)`
    ///
    /// *   `<field>` allows value: `name`
    /// *   `<asc|desc>` ascending or descending order by `<field>`. If this is
    ///      left blank, `asc` is used
    ///
    /// Note that an empty `order_by` string results in default order, which is
    /// order by `name` in ascending order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response message for
/// [RegistrationService.ListNamespaces][google.cloud.servicedirectory.v1beta1.RegistrationService.ListNamespaces].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNamespacesResponse {
    /// The list of namespaces.
    #[prost(message, repeated, tag = "1")]
    pub namespaces: ::prost::alloc::vec::Vec<Namespace>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for
/// [RegistrationService.GetNamespace][google.cloud.servicedirectory.v1beta1.RegistrationService.GetNamespace].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNamespaceRequest {
    /// Required. The name of the namespace to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for
/// [RegistrationService.UpdateNamespace][google.cloud.servicedirectory.v1beta1.RegistrationService.UpdateNamespace].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNamespaceRequest {
    /// Required. The updated namespace.
    #[prost(message, optional, tag = "1")]
    pub namespace: ::core::option::Option<Namespace>,
    /// Required. List of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for
/// [RegistrationService.DeleteNamespace][google.cloud.servicedirectory.v1beta1.RegistrationService.DeleteNamespace].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNamespaceRequest {
    /// Required. The name of the namespace to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for
/// [RegistrationService.CreateService][google.cloud.servicedirectory.v1beta1.RegistrationService.CreateService].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceRequest {
    /// Required. The resource name of the namespace this service will belong to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Resource ID must be 1-63 characters long, and comply with
    /// <a href="<https://www.ietf.org/rfc/rfc1035.txt"> target="_blank">RFC1035</a>.
    /// Specifically, the name must be 1-63 characters long and match the regular
    /// expression `[a-z](?:\[-a-z0-9\]{0,61}\[a-z0-9\])?` which means the first
    /// character must be a lowercase letter, and all following characters must
    /// be a dash, lowercase letter, or digit, except the last character, which
    /// cannot be a dash.
    #[prost(string, tag = "2")]
    pub service_id: ::prost::alloc::string::String,
    /// Required. A service  with initial fields set.
    #[prost(message, optional, tag = "3")]
    pub service: ::core::option::Option<Service>,
}
/// The request message for
/// [RegistrationService.ListServices][google.cloud.servicedirectory.v1beta1.RegistrationService.ListServices].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Required. The resource name of the namespace whose services you'd
    /// like to list.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous List request,
    /// if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to list results by.
    ///
    /// General `filter` string syntax:
    /// `<field> <operator> <value> (<logical connector>)`
    ///
    /// *   `<field>` can be `name` or `metadata.<key>` for map field
    /// *   `<operator>` can be `<`, `>`, `<=`, `>=`, `!=`, `=`, `:`. Of which `:`
    ///      means `HAS`, and is roughly the same as `=`
    /// *   `<value>` must be the same data type as field
    /// *   `<logical connector>` can be `AND`, `OR`, `NOT`
    ///
    /// Examples of valid filters:
    ///
    /// *   `metadata.owner` returns services that have a metadata with the key
    ///      `owner`, this is the same as `metadata:owner`
    /// *   `metadata.protocol=gRPC` returns services that have key/value
    ///      `protocol=gRPC`
    /// *
    /// `name>projects/my-project/locations/us-east1/namespaces/my-namespace/services/service-c`
    ///      returns services that have name that is alphabetically later than the
    ///      string, so "service-e" is returned but "service-a" is not
    /// *   `metadata.owner!=sd AND metadata.foo=bar` returns services that have
    ///      `owner` in metadata key but value is not `sd` AND have key/value
    ///      `foo=bar`
    /// *   `doesnotexist.foo=bar` returns an empty list. Note that service
    ///      doesn't have a field called "doesnotexist". Since the filter does not
    ///      match any services, it returns no results
    /// *   `attributes.managed_registration=true` returns services that are
    /// managed
    ///      by a GCP product or service
    ///
    /// For more information about filtering, see
    /// [API Filtering](<https://aip.dev/160>).
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The order to list results by.
    ///
    /// General `order_by` string syntax: `<field> (<asc|desc>) (,)`
    ///
    /// *   `<field>` allows value: `name`
    /// *   `<asc|desc>` ascending or descending order by `<field>`. If this is
    ///      left blank, `asc` is used
    ///
    /// Note that an empty `order_by` string results in default order, which is
    /// order by `name` in ascending order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response message for
/// [RegistrationService.ListServices][google.cloud.servicedirectory.v1beta1.RegistrationService.ListServices].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The list of services.
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for
/// [RegistrationService.GetService][google.cloud.servicedirectory.v1beta1.RegistrationService.GetService].
/// This should not be used for looking up a service. Instead, use the `resolve`
/// method as it contains all endpoints and associated metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Required. The name of the service to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for
/// [RegistrationService.UpdateService][google.cloud.servicedirectory.v1beta1.RegistrationService.UpdateService].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceRequest {
    /// Required. The updated service.
    #[prost(message, optional, tag = "1")]
    pub service: ::core::option::Option<Service>,
    /// Required. List of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for
/// [RegistrationService.DeleteService][google.cloud.servicedirectory.v1beta1.RegistrationService.DeleteService].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Required. The name of the service to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for
/// [RegistrationService.CreateEndpoint][google.cloud.servicedirectory.v1beta1.RegistrationService.CreateEndpoint].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEndpointRequest {
    /// Required. The resource name of the service that this endpoint provides.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Resource ID must be 1-63 characters long, and comply with
    /// <a href="<https://www.ietf.org/rfc/rfc1035.txt"> target="_blank">RFC1035</a>.
    /// Specifically, the name must be 1-63 characters long and match the regular
    /// expression `[a-z](?:\[-a-z0-9\]{0,61}\[a-z0-9\])?` which means the first
    /// character must be a lowercase letter, and all following characters must
    /// be a dash, lowercase letter, or digit, except the last character, which
    /// cannot be a dash.
    #[prost(string, tag = "2")]
    pub endpoint_id: ::prost::alloc::string::String,
    /// Required. A endpoint with initial fields set.
    #[prost(message, optional, tag = "3")]
    pub endpoint: ::core::option::Option<Endpoint>,
}
/// The request message for
/// [RegistrationService.ListEndpoints][google.cloud.servicedirectory.v1beta1.RegistrationService.ListEndpoints].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointsRequest {
    /// Required. The resource name of the service whose endpoints you'd like to
    /// list.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous List request,
    /// if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to list results by.
    ///
    /// General `filter` string syntax:
    /// `<field> <operator> <value> (<logical connector>)`
    ///
    /// *   `<field>` can be `name`, `address`, `port`, `metadata.<key>` for map
    ///      field, or `attributes.<field>` for attributes field
    /// *   `<operator>` can be `<`, `>`, `<=`, `>=`, `!=`, `=`, `:`. Of which `:`
    ///      means `HAS`, and is roughly the same as `=`
    /// *   `<value>` must be the same data type as field
    /// *   `<logical connector>` can be `AND`, `OR`, `NOT`
    ///
    /// Examples of valid filters:
    ///
    /// *   `metadata.owner` returns endpoints that have a metadata with the key
    ///      `owner`, this is the same as `metadata:owner`
    /// *   `metadata.protocol=gRPC` returns endpoints that have key/value
    ///      `protocol=gRPC`
    /// *   `address=192.108.1.105` returns endpoints that have this address
    /// *   `port>8080` returns endpoints that have port number larger than 8080
    /// *
    /// `name>projects/my-project/locations/us-east1/namespaces/my-namespace/services/my-service/endpoints/endpoint-c`
    ///      returns endpoints that have name that is alphabetically later than the
    ///      string, so "endpoint-e" is returned but "endpoint-a" is not
    /// *   `metadata.owner!=sd AND metadata.foo=bar` returns endpoints that have
    ///      `owner` in metadata key but value is not `sd` AND have key/value
    ///       `foo=bar`
    /// *   `doesnotexist.foo=bar` returns an empty list. Note that endpoint
    ///      doesn't have a field called "doesnotexist". Since the filter does not
    ///      match any endpoints, it returns no results
    /// *   `attributes.kubernetes_resource_type=KUBERNETES_RESOURCE_TYPE_CLUSTER_
    ///      IP` returns endpoints with the corresponding kubernetes_resource_type
    ///
    /// For more information about filtering, see
    /// [API Filtering](<https://aip.dev/160>).
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The order to list results by.
    ///
    /// General `order_by` string syntax: `<field> (<asc|desc>) (,)`
    ///
    /// *   `<field>` allows values: `name`, `address`, `port`
    /// *   `<asc|desc>` ascending or descending order by `<field>`. If this is
    ///      left blank, `asc` is used
    ///
    /// Note that an empty `order_by` string results in default order, which is
    /// order by `name` in ascending order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response message for
/// [RegistrationService.ListEndpoints][google.cloud.servicedirectory.v1beta1.RegistrationService.ListEndpoints].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointsResponse {
    /// The list of endpoints.
    #[prost(message, repeated, tag = "1")]
    pub endpoints: ::prost::alloc::vec::Vec<Endpoint>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for
/// [RegistrationService.GetEndpoint][google.cloud.servicedirectory.v1beta1.RegistrationService.GetEndpoint].
/// This should not be used to lookup endpoints at runtime. Instead, use
/// the `resolve` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointRequest {
    /// Required. The name of the endpoint to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for
/// [RegistrationService.UpdateEndpoint][google.cloud.servicedirectory.v1beta1.RegistrationService.UpdateEndpoint].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEndpointRequest {
    /// Required. The updated endpoint.
    #[prost(message, optional, tag = "1")]
    pub endpoint: ::core::option::Option<Endpoint>,
    /// Required. List of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for
/// [RegistrationService.DeleteEndpoint][google.cloud.servicedirectory.v1beta1.RegistrationService.DeleteEndpoint].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEndpointRequest {
    /// Required. The name of the endpoint to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod registration_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service Directory API for registering services. It defines the following
    /// resource model:
    ///
    /// - The API has a collection of
    /// [Namespace][google.cloud.servicedirectory.v1beta1.Namespace]
    /// resources, named `projects/*/locations/*/namespaces/*`.
    ///
    /// - Each Namespace has a collection of
    /// [Service][google.cloud.servicedirectory.v1beta1.Service] resources, named
    /// `projects/*/locations/*/namespaces/*/services/*`.
    ///
    /// - Each Service has a collection of
    /// [Endpoint][google.cloud.servicedirectory.v1beta1.Endpoint]
    /// resources, named
    /// `projects/*/locations/*/namespaces/*/services/*/endpoints/*`.
    #[derive(Debug, Clone)]
    pub struct RegistrationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RegistrationServiceClient<T>
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
        ) -> RegistrationServiceClient<InterceptedService<T, F>>
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
            RegistrationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a namespace, and returns the new namespace.
        pub async fn create_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNamespaceRequest>,
        ) -> std::result::Result<tonic::Response<super::Namespace>, tonic::Status> {
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/CreateNamespace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "CreateNamespace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all namespaces.
        pub async fn list_namespaces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNamespacesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNamespacesResponse>,
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/ListNamespaces",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "ListNamespaces",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a namespace.
        pub async fn get_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNamespaceRequest>,
        ) -> std::result::Result<tonic::Response<super::Namespace>, tonic::Status> {
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/GetNamespace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "GetNamespace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a namespace.
        pub async fn update_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNamespaceRequest>,
        ) -> std::result::Result<tonic::Response<super::Namespace>, tonic::Status> {
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/UpdateNamespace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "UpdateNamespace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a namespace. This also deletes all services and endpoints in
        /// the namespace.
        pub async fn delete_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNamespaceRequest>,
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/DeleteNamespace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "DeleteNamespace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a service, and returns the new service.
        pub async fn create_service(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceRequest>,
        ) -> std::result::Result<tonic::Response<super::Service>, tonic::Status> {
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/CreateService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "CreateService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all services belonging to a namespace.
        pub async fn list_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListServicesResponse>,
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/ListServices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "ListServices",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a service.
        pub async fn get_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRequest>,
        ) -> std::result::Result<tonic::Response<super::Service>, tonic::Status> {
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/GetService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "GetService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a service.
        pub async fn update_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServiceRequest>,
        ) -> std::result::Result<tonic::Response<super::Service>, tonic::Status> {
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/UpdateService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "UpdateService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a service. This also deletes all endpoints associated with
        /// the service.
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceRequest>,
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/DeleteService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "DeleteService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an endpoint, and returns the new endpoint.
        pub async fn create_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEndpointRequest>,
        ) -> std::result::Result<tonic::Response<super::Endpoint>, tonic::Status> {
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/CreateEndpoint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "CreateEndpoint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all endpoints.
        pub async fn list_endpoints(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEndpointsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEndpointsResponse>,
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/ListEndpoints",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "ListEndpoints",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets an endpoint.
        pub async fn get_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEndpointRequest>,
        ) -> std::result::Result<tonic::Response<super::Endpoint>, tonic::Status> {
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/GetEndpoint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "GetEndpoint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an endpoint.
        pub async fn update_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEndpointRequest>,
        ) -> std::result::Result<tonic::Response<super::Endpoint>, tonic::Status> {
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/UpdateEndpoint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "UpdateEndpoint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an endpoint.
        pub async fn delete_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEndpointRequest>,
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/DeleteEndpoint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "DeleteEndpoint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the IAM Policy for a resource
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the IAM Policy for a resource
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Tests IAM permissions for a resource (namespace, service  or
        /// service workload only).
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.servicedirectory.v1beta1.RegistrationService/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicedirectory.v1beta1.RegistrationService",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
