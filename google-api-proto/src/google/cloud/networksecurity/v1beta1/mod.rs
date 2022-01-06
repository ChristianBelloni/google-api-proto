/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Specification of the GRPC Endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcEndpoint {
    /// Required. The target URI of the gRPC endpoint. Only UDS path is supported, and
    /// should start with “unix:”.
    #[prost(string, tag = "1")]
    pub target_uri: ::prost::alloc::string::String,
}
/// Specification of ValidationCA. Defines the mechanism to obtain the
/// Certificate Authority certificate to validate the peer certificate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationCa {
    /// The type of certificate provider which provides the CA certificate.
    #[prost(oneof = "validation_ca::Type", tags = "2, 3")]
    pub r#type: ::core::option::Option<validation_ca::Type>,
}
/// Nested message and enum types in `ValidationCA`.
pub mod validation_ca {
    /// The type of certificate provider which provides the CA certificate.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// gRPC specific configuration to access the gRPC server to
        /// obtain the CA certificate.
        #[prost(message, tag = "2")]
        GrpcEndpoint(super::GrpcEndpoint),
        /// The certificate provider instance specification that will be passed to
        /// the data plane, which will be used to load necessary credential
        /// information.
        #[prost(message, tag = "3")]
        CertificateProviderInstance(super::CertificateProviderInstance),
    }
}
/// Specification of a TLS certificate provider instance. Workloads may have one
/// or more CertificateProvider instances (plugins) and one of them is enabled
/// and configured by specifying this message. Workloads use the values from this
/// message to locate and load the CertificateProvider instance configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateProviderInstance {
    /// Required. Plugin instance name, used to locate and load CertificateProvider instance
    /// configuration. Set to "google_cloud_private_spiffe" to use Certificate
    /// Authority Service certificate provider instance.
    #[prost(string, tag = "1")]
    pub plugin_instance: ::prost::alloc::string::String,
}
/// Specification of certificate provider. Defines the mechanism to obtain the
/// certificate and private key for peer to peer authentication.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateProvider {
    /// The type of certificate provider which provides the certificates and
    /// private keys.
    #[prost(oneof = "certificate_provider::Type", tags = "2, 3")]
    pub r#type: ::core::option::Option<certificate_provider::Type>,
}
/// Nested message and enum types in `CertificateProvider`.
pub mod certificate_provider {
    /// The type of certificate provider which provides the certificates and
    /// private keys.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// gRPC specific configuration to access the gRPC server to
        /// obtain the cert and private key.
        #[prost(message, tag = "2")]
        GrpcEndpoint(super::GrpcEndpoint),
        /// The certificate provider instance specification that will be passed to
        /// the data plane, which will be used to load necessary credential
        /// information.
        #[prost(message, tag = "3")]
        CertificateProviderInstance(super::CertificateProviderInstance),
    }
}
/// ClientTlsPolicy is a resource that specifies how a client should authenticate
/// connections to backends of a service. This resource itself does not affect
/// configuration unless it is attached to a backend service resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientTlsPolicy {
    /// Required. Name of the ClientTlsPolicy resource. It matches the pattern
    /// `projects/*/locations/{location}/clientTlsPolicies/{client_tls_policy}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Free-text description of the resource.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Set of label tags associated with the resource.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. Server Name Indication string to present to the server during TLS
    /// handshake. E.g: "secure.example.com".
    #[prost(string, tag = "6")]
    pub sni: ::prost::alloc::string::String,
    /// Optional. Defines a mechanism to provision client identity (public and private keys)
    /// for peer to peer authentication. The presence of this dictates mTLS.
    #[prost(message, optional, tag = "7")]
    pub client_certificate: ::core::option::Option<CertificateProvider>,
    /// Optional. Defines the mechanism to obtain the Certificate Authority certificate to
    /// validate the server certificate. If empty, client does not validate the
    /// server certificate.
    #[prost(message, repeated, tag = "8")]
    pub server_validation_ca: ::prost::alloc::vec::Vec<ValidationCa>,
}
/// Request used by the ListClientTlsPolicies method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClientTlsPoliciesRequest {
    /// Required. The project and location from which the ClientTlsPolicies should
    /// be listed, specified in the format `projects/*/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of ClientTlsPolicies to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListClientTlsPoliciesResponse`
    /// Indicates that this is a continuation of a prior
    /// `ListClientTlsPolicies` call, and that the system
    /// should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response returned by the ListClientTlsPolicies method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClientTlsPoliciesResponse {
    /// List of ClientTlsPolicy resources.
    #[prost(message, repeated, tag = "1")]
    pub client_tls_policies: ::prost::alloc::vec::Vec<ClientTlsPolicy>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request used by the GetClientTlsPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClientTlsPolicyRequest {
    /// Required. A name of the ClientTlsPolicy to get. Must be in the format
    /// `projects/*/locations/{location}/clientTlsPolicies/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request used by the CreateClientTlsPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClientTlsPolicyRequest {
    /// Required. The parent resource of the ClientTlsPolicy. Must be in
    /// the format `projects/*/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Short name of the ClientTlsPolicy resource to be created. This value should
    /// be 1-63 characters long, containing only letters, numbers, hyphens, and
    /// underscores, and should not start with a number. E.g. "client_mtls_policy".
    #[prost(string, tag = "2")]
    pub client_tls_policy_id: ::prost::alloc::string::String,
    /// Required. ClientTlsPolicy resource to be created.
    #[prost(message, optional, tag = "3")]
    pub client_tls_policy: ::core::option::Option<ClientTlsPolicy>,
}
/// Request used by UpdateClientTlsPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClientTlsPolicyRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// ClientTlsPolicy resource by the update.  The fields
    /// specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the
    /// mask. If the user does not provide a mask then all fields will be
    /// overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Updated ClientTlsPolicy resource.
    #[prost(message, optional, tag = "2")]
    pub client_tls_policy: ::core::option::Option<ClientTlsPolicy>,
}
/// Request used by the DeleteClientTlsPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClientTlsPolicyRequest {
    /// Required. A name of the ClientTlsPolicy to delete. Must be in
    /// the format `projects/*/locations/{location}/clientTlsPolicies/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// AuthorizationPolicy is a resource that specifies how a server
/// should authorize incoming connections. This resource in itself does
/// not change the configuration unless it's attached to a target https
/// proxy or endpoint config selector resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationPolicy {
    /// Required. Name of the AuthorizationPolicy resource. It matches pattern
    /// `projects/{project}/locations/{location}/authorizationPolicies/<authorization_policy>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Free-text description of the resource.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Set of label tags associated with the AuthorizationPolicy resource.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. The action to take when a rule match is found. Possible values
    /// are "ALLOW" or "DENY".
    #[prost(enumeration = "authorization_policy::Action", tag = "6")]
    pub action: i32,
    /// Optional. List of rules to match. Note that at least one of the rules must match in
    /// order for the action specified in the 'action' field to be taken. A rule is
    /// a match if there is a matching source and destination. If left blank, the
    /// action specified in the `action` field will be applied on every request.
    #[prost(message, repeated, tag = "7")]
    pub rules: ::prost::alloc::vec::Vec<authorization_policy::Rule>,
}
/// Nested message and enum types in `AuthorizationPolicy`.
pub mod authorization_policy {
    /// Specification of rules.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rule {
        /// Optional. List of attributes for the traffic source. All of the sources must match.
        /// A source is a match if both principals and ip_blocks match. If not set,
        /// the action specified in the 'action' field will be applied without any
        /// rule checks for the source.
        #[prost(message, repeated, tag = "1")]
        pub sources: ::prost::alloc::vec::Vec<rule::Source>,
        /// Optional. List of attributes for the traffic destination. All of the destinations
        /// must match. A destination is a match if a request matches all the
        /// specified hosts, ports, methods and headers. If not set, the
        /// action specified in the 'action' field will be applied without any rule
        /// checks for the destination.
        #[prost(message, repeated, tag = "2")]
        pub destinations: ::prost::alloc::vec::Vec<rule::Destination>,
    }
    /// Nested message and enum types in `Rule`.
    pub mod rule {
        /// Specification of traffic source attributes.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Source {
            /// Optional. List of peer identities to match for authorization. At least one
            /// principal should match. Each peer can be an exact match, or a prefix
            /// match (example, "namespace/*") or a suffix match (example, //
            /// */service-account") or a presence match "*". Authorization based on the
            /// principal name without certificate validation (configured by
            /// ServerTlsPolicy resource) is considered insecure.
            #[prost(string, repeated, tag = "1")]
            pub principals: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Optional. List of CIDR ranges to match based on source IP address. At least one
            /// IP block should match. Single IP (e.g., "1.2.3.4") and CIDR (e.g.,
            /// "1.2.3.0/24") are supported. Authorization based on source IP alone
            /// should be avoided. The IP addresses of any load balancers or proxies
            /// should be considered untrusted.
            #[prost(string, repeated, tag = "2")]
            pub ip_blocks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        /// Specification of traffic destination attributes.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Destination {
            /// Required. List of host names to match. Matched against the ":authority" header in
            /// http requests. At least one host should match. Each host can be an
            /// exact match, or a prefix match (example "mydomain.*") or a suffix
            /// match (example // *.myorg.com") or a presence(any) match "*".
            #[prost(string, repeated, tag = "1")]
            pub hosts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Required. List of destination ports to match. At least one port should match.
            #[prost(uint32, repeated, packed = "false", tag = "2")]
            pub ports: ::prost::alloc::vec::Vec<u32>,
            /// Optional. A list of HTTP methods to match. At least one method should
            /// match. Should not be set for gRPC services.
            #[prost(string, repeated, tag = "4")]
            pub methods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Optional. Match against key:value pair in http header. Provides a flexible match
            /// based on HTTP headers, for potentially advanced use cases. At least one
            /// header should match. Avoid using header matches to make authorization
            /// decisions unless there is a strong guarantee that requests arrive
            /// through a trusted client or proxy.
            #[prost(message, optional, tag = "5")]
            pub http_header_match: ::core::option::Option<destination::HttpHeaderMatch>,
        }
        /// Nested message and enum types in `Destination`.
        pub mod destination {
            /// Specification of HTTP header match atrributes.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct HttpHeaderMatch {
                /// Required. The name of the HTTP header to match. For matching
                /// against the HTTP request's authority, use a headerMatch
                /// with the header name ":authority". For matching a
                /// request's method, use the headerName ":method".
                #[prost(string, tag = "1")]
                pub header_name: ::prost::alloc::string::String,
                #[prost(oneof = "http_header_match::Type", tags = "2")]
                pub r#type: ::core::option::Option<http_header_match::Type>,
            }
            /// Nested message and enum types in `HttpHeaderMatch`.
            pub mod http_header_match {
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Type {
                    /// Required. The value of the header must match the regular expression
                    /// specified in regexMatch. For regular expression grammar,
                    /// please see: en.cppreference.com/w/cpp/regex/ecmascript
                    /// For matching against a port specified in the HTTP
                    /// request, use a headerMatch with headerName set to Host
                    /// and a regular expression that satisfies the RFC2616 Host
                    /// header's port specifier.
                    #[prost(string, tag = "2")]
                    RegexMatch(::prost::alloc::string::String),
                }
            }
        }
    }
    /// Possible values that define what action to take.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        /// Default value.
        Unspecified = 0,
        /// Grant access.
        Allow = 1,
        /// Deny access.
        /// Deny rules should be avoided unless they are used to provide a default
        /// "deny all" fallback.
        Deny = 2,
    }
}
/// Request used with the ListAuthorizationPolicies method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizationPoliciesRequest {
    /// Required. The project and location from which the AuthorizationPolicies
    /// should be listed, specified in the format
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of AuthorizationPolicies to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last
    /// `ListAuthorizationPoliciesResponse` Indicates that this is a
    /// continuation of a prior `ListAuthorizationPolicies` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response returned by the ListAuthorizationPolicies method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizationPoliciesResponse {
    /// List of AuthorizationPolicies resources.
    #[prost(message, repeated, tag = "1")]
    pub authorization_policies: ::prost::alloc::vec::Vec<AuthorizationPolicy>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request used by the GetAuthorizationPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthorizationPolicyRequest {
    /// Required. A name of the AuthorizationPolicy to get. Must be in the format
    /// `projects/{project}/locations/{location}/authorizationPolicies/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request used by the CreateAuthorizationPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthorizationPolicyRequest {
    /// Required. The parent resource of the AuthorizationPolicy. Must be in the
    /// format `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Short name of the AuthorizationPolicy resource to be created.
    /// This value should be 1-63 characters long, containing only
    /// letters, numbers, hyphens, and underscores, and should not start
    /// with a number. E.g. "authz_policy".
    #[prost(string, tag = "2")]
    pub authorization_policy_id: ::prost::alloc::string::String,
    /// Required. AuthorizationPolicy resource to be created.
    #[prost(message, optional, tag = "3")]
    pub authorization_policy: ::core::option::Option<AuthorizationPolicy>,
}
/// Request used by the UpdateAuthorizationPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAuthorizationPolicyRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// AuthorizationPolicy resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Updated AuthorizationPolicy resource.
    #[prost(message, optional, tag = "2")]
    pub authorization_policy: ::core::option::Option<AuthorizationPolicy>,
}
/// Request used by the DeleteAuthorizationPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAuthorizationPolicyRequest {
    /// Required. A name of the AuthorizationPolicy to delete. Must be in the format
    /// `projects/{project}/locations/{location}/authorizationPolicies/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// ServerTlsPolicy is a resource that specifies how a server should authenticate
/// incoming requests. This resource itself does not affect configuration unless
/// it is attached to a target https proxy or endpoint config selector resource.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerTlsPolicy {
    /// Required. Name of the ServerTlsPolicy resource. It matches the pattern
    /// `projects/*/locations/{location}/serverTlsPolicies/{server_tls_policy}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Free-text description of the resource.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Set of label tags associated with the resource.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    ///
    /// Determines if server allows plaintext connections. If set to true, server
    /// allows plain text connections. By default, it is set to false. This setting
    /// is not exclusive of other encryption modes. For example, if `allow_open`
    /// and `mtls_policy` are set, server allows both plain text and mTLS
    /// connections. See documentation of other encryption modes to confirm
    /// compatibility.
    #[prost(bool, tag = "6")]
    pub allow_open: bool,
    ///
    /// Defines a mechanism to provision server identity (public and private keys).
    /// Cannot be combined with `allow_open` as a permissive mode that allows both
    /// plain text and TLS is not supported.
    #[prost(message, optional, tag = "7")]
    pub server_certificate: ::core::option::Option<CertificateProvider>,
    ///
    /// Defines a mechanism to provision peer validation certificates for peer to
    /// peer authentication (Mutual TLS - mTLS). If not specified, client
    /// certificate will not be requested. The connection is treated as TLS and not
    /// mTLS. If `allow_open` and `mtls_policy` are set, server allows both plain
    /// text and mTLS connections.
    #[prost(message, optional, tag = "8")]
    pub mtls_policy: ::core::option::Option<server_tls_policy::MtlsPolicy>,
}
/// Nested message and enum types in `ServerTlsPolicy`.
pub mod server_tls_policy {
    /// Specification of the MTLSPolicy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MtlsPolicy {
        ///
        /// Defines the mechanism to obtain the Certificate Authority certificate to
        /// validate the client certificate.
        #[prost(message, repeated, tag = "1")]
        pub client_validation_ca: ::prost::alloc::vec::Vec<super::ValidationCa>,
    }
}
/// Request used by the ListServerTlsPolicies method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServerTlsPoliciesRequest {
    /// Required. The project and location from which the ServerTlsPolicies should
    /// be listed, specified in the format `projects/*/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of ServerTlsPolicies to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListServerTlsPoliciesResponse`
    /// Indicates that this is a continuation of a prior
    /// `ListServerTlsPolicies` call, and that the system
    /// should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response returned by the ListServerTlsPolicies method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServerTlsPoliciesResponse {
    /// List of ServerTlsPolicy resources.
    #[prost(message, repeated, tag = "1")]
    pub server_tls_policies: ::prost::alloc::vec::Vec<ServerTlsPolicy>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request used by the GetServerTlsPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerTlsPolicyRequest {
    /// Required. A name of the ServerTlsPolicy to get. Must be in the format
    /// `projects/*/locations/{location}/serverTlsPolicies/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request used by the CreateServerTlsPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServerTlsPolicyRequest {
    /// Required. The parent resource of the ServerTlsPolicy. Must be in
    /// the format `projects/*/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Short name of the ServerTlsPolicy resource to be created. This value should
    /// be 1-63 characters long, containing only letters, numbers, hyphens, and
    /// underscores, and should not start with a number. E.g. "server_mtls_policy".
    #[prost(string, tag = "2")]
    pub server_tls_policy_id: ::prost::alloc::string::String,
    /// Required. ServerTlsPolicy resource to be created.
    #[prost(message, optional, tag = "3")]
    pub server_tls_policy: ::core::option::Option<ServerTlsPolicy>,
}
/// Request used by UpdateServerTlsPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServerTlsPolicyRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// ServerTlsPolicy resource by the update.  The fields
    /// specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the
    /// mask. If the user does not provide a mask then all fields will be
    /// overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Updated ServerTlsPolicy resource.
    #[prost(message, optional, tag = "2")]
    pub server_tls_policy: ::core::option::Option<ServerTlsPolicy>,
}
/// Request used by the DeleteServerTlsPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServerTlsPolicyRequest {
    /// Required. A name of the ServerTlsPolicy to delete. Must be in
    /// the format `projects/*/locations/{location}/serverTlsPolicies/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod network_security_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Network Security API provides resources to configure authentication and"]
    #[doc = " authorization policies. Refer to per API resource documentation for more"]
    #[doc = " information."]
    #[derive(Debug, Clone)]
    pub struct NetworkSecurityClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> NetworkSecurityClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> NetworkSecurityClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            NetworkSecurityClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Lists AuthorizationPolicies in a given project and location."]
        pub async fn list_authorization_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAuthorizationPoliciesRequest>,
        ) -> Result<tonic::Response<super::ListAuthorizationPoliciesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/ListAuthorizationPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single AuthorizationPolicy."]
        pub async fn get_authorization_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAuthorizationPolicyRequest>,
        ) -> Result<tonic::Response<super::AuthorizationPolicy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/GetAuthorizationPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new AuthorizationPolicy in a given project and location."]
        pub async fn create_authorization_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAuthorizationPolicyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/CreateAuthorizationPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the parameters of a single AuthorizationPolicy."]
        pub async fn update_authorization_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAuthorizationPolicyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/UpdateAuthorizationPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single AuthorizationPolicy."]
        pub async fn delete_authorization_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAuthorizationPolicyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/DeleteAuthorizationPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists ServerTlsPolicies in a given project and location."]
        pub async fn list_server_tls_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServerTlsPoliciesRequest>,
        ) -> Result<tonic::Response<super::ListServerTlsPoliciesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/ListServerTlsPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single ServerTlsPolicy."]
        pub async fn get_server_tls_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServerTlsPolicyRequest>,
        ) -> Result<tonic::Response<super::ServerTlsPolicy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/GetServerTlsPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new ServerTlsPolicy in a given project and location."]
        pub async fn create_server_tls_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServerTlsPolicyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/CreateServerTlsPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the parameters of a single ServerTlsPolicy."]
        pub async fn update_server_tls_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServerTlsPolicyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/UpdateServerTlsPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single ServerTlsPolicy."]
        pub async fn delete_server_tls_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServerTlsPolicyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/DeleteServerTlsPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists ClientTlsPolicies in a given project and location."]
        pub async fn list_client_tls_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListClientTlsPoliciesRequest>,
        ) -> Result<tonic::Response<super::ListClientTlsPoliciesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/ListClientTlsPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single ClientTlsPolicy."]
        pub async fn get_client_tls_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClientTlsPolicyRequest>,
        ) -> Result<tonic::Response<super::ClientTlsPolicy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/GetClientTlsPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new ClientTlsPolicy in a given project and location."]
        pub async fn create_client_tls_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClientTlsPolicyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/CreateClientTlsPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the parameters of a single ClientTlsPolicy."]
        pub async fn update_client_tls_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateClientTlsPolicyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/UpdateClientTlsPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single ClientTlsPolicy."]
        pub async fn delete_client_tls_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClientTlsPolicyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.networksecurity.v1beta1.NetworkSecurity/DeleteClientTlsPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
