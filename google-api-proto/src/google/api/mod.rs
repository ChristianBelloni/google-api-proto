#[cfg(any(feature = "google-api-apikeys-v2"))]
pub mod apikeys;
#[cfg(any(feature = "google-api-cloudquotas-v1"))]
pub mod cloudquotas;
#[cfg(
    any(
        feature = "google-api-expr-conformance-v1alpha1",
        feature = "google-api-expr-v1alpha1",
        feature = "google-api-expr-v1beta1",
    )
)]
pub mod expr;
#[cfg(
    any(
        feature = "google-api-servicecontrol-v1",
        feature = "google-api-servicecontrol-v2",
    )
)]
pub mod servicecontrol;
#[cfg(any(feature = "google-api-servicemanagement-v1"))]
pub mod servicemanagement;
#[cfg(
    any(
        feature = "google-api-serviceusage-v1",
        feature = "google-api-serviceusage-v1beta1",
    )
)]
pub mod serviceusage;
/// An indicator of the behavior of a given field (for example, that a field
/// is required in requests, or given as output but ignored as input).
/// This **does not** change the behavior in protocol buffers itself; it only
/// denotes the behavior and may affect how API tooling handles the field.
///
/// Note: This enum **may** receive new values in the future.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FieldBehavior {
    /// Conventional default for enums. Do not use this.
    Unspecified = 0,
    /// Specifically denotes a field as optional.
    /// While all fields in protocol buffers are optional, this may be specified
    /// for emphasis if appropriate.
    Optional = 1,
    /// Denotes a field as required.
    /// This indicates that the field **must** be provided as part of the request,
    /// and failure to do so will cause an error (usually `INVALID_ARGUMENT`).
    Required = 2,
    /// Denotes a field as output only.
    /// This indicates that the field is provided in responses, but including the
    /// field in a request does nothing (the server *must* ignore it and
    /// *must not* throw an error as a result of the field's presence).
    OutputOnly = 3,
    /// Denotes a field as input only.
    /// This indicates that the field is provided in requests, and the
    /// corresponding field is not included in output.
    InputOnly = 4,
    /// Denotes a field as immutable.
    /// This indicates that the field may be set once in a request to create a
    /// resource, but may not be changed thereafter.
    Immutable = 5,
    /// Denotes that a (repeated) field is an unordered list.
    /// This indicates that the service may provide the elements of the list
    /// in any arbitrary  order, rather than the order the user originally
    /// provided. Additionally, the list's order may or may not be stable.
    UnorderedList = 6,
    /// Denotes that this field returns a non-empty default value if not set.
    /// This indicates that if the user provides the empty value in a request,
    /// a non-empty value will be returned. The user will not be aware of what
    /// non-empty value to expect.
    NonEmptyDefault = 7,
    /// Denotes that the field in a resource (a message annotated with
    /// google.api.resource) is used in the resource name to uniquely identify the
    /// resource. For AIP-compliant APIs, this should only be applied to the
    /// `name` field on the resource.
    ///
    /// This behavior should not be applied to references to other resources within
    /// the message.
    ///
    /// The identifier field of resources often have different field behavior
    /// depending on the request it is embedded in (e.g. for Create methods name
    /// is optional and unused, while for Update methods it is required). Instead
    /// of method-specific annotations, only `IDENTIFIER` is required.
    Identifier = 8,
}
impl FieldBehavior {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FieldBehavior::Unspecified => "FIELD_BEHAVIOR_UNSPECIFIED",
            FieldBehavior::Optional => "OPTIONAL",
            FieldBehavior::Required => "REQUIRED",
            FieldBehavior::OutputOnly => "OUTPUT_ONLY",
            FieldBehavior::InputOnly => "INPUT_ONLY",
            FieldBehavior::Immutable => "IMMUTABLE",
            FieldBehavior::UnorderedList => "UNORDERED_LIST",
            FieldBehavior::NonEmptyDefault => "NON_EMPTY_DEFAULT",
            FieldBehavior::Identifier => "IDENTIFIER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FIELD_BEHAVIOR_UNSPECIFIED" => Some(Self::Unspecified),
            "OPTIONAL" => Some(Self::Optional),
            "REQUIRED" => Some(Self::Required),
            "OUTPUT_ONLY" => Some(Self::OutputOnly),
            "INPUT_ONLY" => Some(Self::InputOnly),
            "IMMUTABLE" => Some(Self::Immutable),
            "UNORDERED_LIST" => Some(Self::UnorderedList),
            "NON_EMPTY_DEFAULT" => Some(Self::NonEmptyDefault),
            "IDENTIFIER" => Some(Self::Identifier),
            _ => None,
        }
    }
}
/// Defines the HTTP configuration for an API service. It contains a list of
/// [HttpRule][google.api.HttpRule], each specifying the mapping of an RPC method
/// to one or more HTTP REST API methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http {
    /// A list of HTTP configuration rules that apply to individual API methods.
    ///
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<HttpRule>,
    /// When set to true, URL path parameters will be fully URI-decoded except in
    /// cases of single segment matches in reserved expansion, where "%2F" will be
    /// left encoded.
    ///
    /// The default behavior is to not decode RFC 6570 reserved characters in multi
    /// segment matches.
    #[prost(bool, tag = "2")]
    pub fully_decode_reserved_expansion: bool,
}
/// # gRPC Transcoding
///
/// gRPC Transcoding is a feature for mapping between a gRPC method and one or
/// more HTTP REST endpoints. It allows developers to build a single API service
/// that supports both gRPC APIs and REST APIs. Many systems, including [Google
/// APIs](<https://github.com/googleapis/googleapis>),
/// [Cloud Endpoints](<https://cloud.google.com/endpoints>), [gRPC
/// Gateway](<https://github.com/grpc-ecosystem/grpc-gateway>),
/// and [Envoy](<https://github.com/envoyproxy/envoy>) proxy support this feature
/// and use it for large scale production services.
///
/// `HttpRule` defines the schema of the gRPC/REST mapping. The mapping specifies
/// how different portions of the gRPC request message are mapped to the URL
/// path, URL query parameters, and HTTP request body. It also controls how the
/// gRPC response message is mapped to the HTTP response body. `HttpRule` is
/// typically specified as an `google.api.http` annotation on the gRPC method.
///
/// Each mapping specifies a URL path template and an HTTP method. The path
/// template may refer to one or more fields in the gRPC request message, as long
/// as each field is a non-repeated field with a primitive (non-message) type.
/// The path template controls how fields of the request message are mapped to
/// the URL path.
///
/// Example:
///
///      service Messaging {
///        rpc GetMessage(GetMessageRequest) returns (Message) {
///          option (google.api.http) = {
///              get: "/v1/{name=messages/*}"
///          };
///        }
///      }
///      message GetMessageRequest {
///        string name = 1; // Mapped to URL path.
///      }
///      message Message {
///        string text = 1; // The resource content.
///      }
///
/// This enables an HTTP REST to gRPC mapping as below:
///
/// HTTP | gRPC
/// -----|-----
/// `GET /v1/messages/123456`  | `GetMessage(name: "messages/123456")`
///
/// Any fields in the request message which are not bound by the path template
/// automatically become HTTP query parameters if there is no HTTP request body.
/// For example:
///
///      service Messaging {
///        rpc GetMessage(GetMessageRequest) returns (Message) {
///          option (google.api.http) = {
///              get:"/v1/messages/{message_id}"
///          };
///        }
///      }
///      message GetMessageRequest {
///        message SubMessage {
///          string subfield = 1;
///        }
///        string message_id = 1; // Mapped to URL path.
///        int64 revision = 2;    // Mapped to URL query parameter `revision`.
///        SubMessage sub = 3;    // Mapped to URL query parameter `sub.subfield`.
///      }
///
/// This enables a HTTP JSON to RPC mapping as below:
///
/// HTTP | gRPC
/// -----|-----
/// `GET /v1/messages/123456?revision=2&sub.subfield=foo` |
/// `GetMessage(message_id: "123456" revision: 2 sub: SubMessage(subfield:
/// "foo"))`
///
/// Note that fields which are mapped to URL query parameters must have a
/// primitive type or a repeated primitive type or a non-repeated message type.
/// In the case of a repeated type, the parameter can be repeated in the URL
/// as `...?param=A&param=B`. In the case of a message type, each field of the
/// message is mapped to a separate parameter, such as
/// `...?foo.a=A&foo.b=B&foo.c=C`.
///
/// For HTTP methods that allow a request body, the `body` field
/// specifies the mapping. Consider a REST update method on the
/// message resource collection:
///
///      service Messaging {
///        rpc UpdateMessage(UpdateMessageRequest) returns (Message) {
///          option (google.api.http) = {
///            patch: "/v1/messages/{message_id}"
///            body: "message"
///          };
///        }
///      }
///      message UpdateMessageRequest {
///        string message_id = 1; // mapped to the URL
///        Message message = 2;   // mapped to the body
///      }
///
/// The following HTTP JSON to RPC mapping is enabled, where the
/// representation of the JSON in the request body is determined by
/// protos JSON encoding:
///
/// HTTP | gRPC
/// -----|-----
/// `PATCH /v1/messages/123456 { "text": "Hi!" }` | `UpdateMessage(message_id:
/// "123456" message { text: "Hi!" })`
///
/// The special name `*` can be used in the body mapping to define that
/// every field not bound by the path template should be mapped to the
/// request body.  This enables the following alternative definition of
/// the update method:
///
///      service Messaging {
///        rpc UpdateMessage(Message) returns (Message) {
///          option (google.api.http) = {
///            patch: "/v1/messages/{message_id}"
///            body: "*"
///          };
///        }
///      }
///      message Message {
///        string message_id = 1;
///        string text = 2;
///      }
///
///
/// The following HTTP JSON to RPC mapping is enabled:
///
/// HTTP | gRPC
/// -----|-----
/// `PATCH /v1/messages/123456 { "text": "Hi!" }` | `UpdateMessage(message_id:
/// "123456" text: "Hi!")`
///
/// Note that when using `*` in the body mapping, it is not possible to
/// have HTTP parameters, as all fields not bound by the path end in
/// the body. This makes this option more rarely used in practice when
/// defining REST APIs. The common usage of `*` is in custom methods
/// which don't use the URL at all for transferring data.
///
/// It is possible to define multiple HTTP methods for one RPC by using
/// the `additional_bindings` option. Example:
///
///      service Messaging {
///        rpc GetMessage(GetMessageRequest) returns (Message) {
///          option (google.api.http) = {
///            get: "/v1/messages/{message_id}"
///            additional_bindings {
///              get: "/v1/users/{user_id}/messages/{message_id}"
///            }
///          };
///        }
///      }
///      message GetMessageRequest {
///        string message_id = 1;
///        string user_id = 2;
///      }
///
/// This enables the following two alternative HTTP JSON to RPC mappings:
///
/// HTTP | gRPC
/// -----|-----
/// `GET /v1/messages/123456` | `GetMessage(message_id: "123456")`
/// `GET /v1/users/me/messages/123456` | `GetMessage(user_id: "me" message_id:
/// "123456")`
///
/// ## Rules for HTTP mapping
///
/// 1. Leaf request fields (recursive expansion nested messages in the request
///     message) are classified into three categories:
///     - Fields referred by the path template. They are passed via the URL path.
///     - Fields referred by the [HttpRule.body][google.api.HttpRule.body]. They
///     are passed via the HTTP
///       request body.
///     - All other fields are passed via the URL query parameters, and the
///       parameter name is the field path in the request message. A repeated
///       field can be represented as multiple query parameters under the same
///       name.
///   2. If [HttpRule.body][google.api.HttpRule.body] is "*", there is no URL
///   query parameter, all fields
///      are passed via URL path and HTTP request body.
///   3. If [HttpRule.body][google.api.HttpRule.body] is omitted, there is no HTTP
///   request body, all
///      fields are passed via URL path and URL query parameters.
///
/// ### Path template syntax
///
///      Template = "/" Segments \[ Verb \] ;
///      Segments = Segment { "/" Segment } ;
///      Segment  = "*" | "**" | LITERAL | Variable ;
///      Variable = "{" FieldPath \[ "=" Segments \] "}" ;
///      FieldPath = IDENT { "." IDENT } ;
///      Verb     = ":" LITERAL ;
///
/// The syntax `*` matches a single URL path segment. The syntax `**` matches
/// zero or more URL path segments, which must be the last part of the URL path
/// except the `Verb`.
///
/// The syntax `Variable` matches part of the URL path as specified by its
/// template. A variable template must not contain other variables. If a variable
/// matches a single path segment, its template may be omitted, e.g. `{var}`
/// is equivalent to `{var=*}`.
///
/// The syntax `LITERAL` matches literal text in the URL path. If the `LITERAL`
/// contains any reserved character, such characters should be percent-encoded
/// before the matching.
///
/// If a variable contains exactly one path segment, such as `"{var}"` or
/// `"{var=*}"`, when such a variable is expanded into a URL path on the client
/// side, all characters except `\[-_.~0-9a-zA-Z\]` are percent-encoded. The
/// server side does the reverse decoding. Such variables show up in the
/// [Discovery
/// Document](<https://developers.google.com/discovery/v1/reference/apis>) as
/// `{var}`.
///
/// If a variable contains multiple path segments, such as `"{var=foo/*}"`
/// or `"{var=**}"`, when such a variable is expanded into a URL path on the
/// client side, all characters except `\[-_.~/0-9a-zA-Z\]` are percent-encoded.
/// The server side does the reverse decoding, except "%2F" and "%2f" are left
/// unchanged. Such variables show up in the
/// [Discovery
/// Document](<https://developers.google.com/discovery/v1/reference/apis>) as
/// `{+var}`.
///
/// ## Using gRPC API Service Configuration
///
/// gRPC API Service Configuration (service config) is a configuration language
/// for configuring a gRPC service to become a user-facing product. The
/// service config is simply the YAML representation of the `google.api.Service`
/// proto message.
///
/// As an alternative to annotating your proto file, you can configure gRPC
/// transcoding in your service config YAML files. You do this by specifying a
/// `HttpRule` that maps the gRPC method to a REST endpoint, achieving the same
/// effect as the proto annotation. This can be particularly useful if you
/// have a proto that is reused in multiple services. Note that any transcoding
/// specified in the service config will override any matching transcoding
/// configuration in the proto.
///
/// Example:
///
///      http:
///        rules:
///          # Selects a gRPC method and applies HttpRule to it.
///          - selector: example.v1.Messaging.GetMessage
///            get: /v1/messages/{message_id}/{sub.subfield}
///
/// ## Special notes
///
/// When gRPC Transcoding is used to map a gRPC to JSON REST endpoints, the
/// proto to JSON conversion must follow the [proto3
/// specification](<https://developers.google.com/protocol-buffers/docs/proto3#json>).
///
/// While the single segment variable follows the semantics of
/// [RFC 6570](<https://tools.ietf.org/html/rfc6570>) Section 3.2.2 Simple String
/// Expansion, the multi segment variable **does not** follow RFC 6570 Section
/// 3.2.3 Reserved Expansion. The reason is that the Reserved Expansion
/// does not expand special characters like `?` and `#`, which would lead
/// to invalid URLs. As the result, gRPC Transcoding uses a custom encoding
/// for multi segment variables.
///
/// The path variables **must not** refer to any repeated or mapped field,
/// because client libraries are not capable of handling such variable expansion.
///
/// The path variables **must not** capture the leading "/" character. The reason
/// is that the most common use case "{var}" does not capture the leading "/"
/// character. For consistency, all path variables must share the same behavior.
///
/// Repeated message fields must not be mapped to URL query parameters, because
/// no client library can support such complicated mapping.
///
/// If an API needs to use a JSON array for request or response body, it can map
/// the request or response body to a repeated field. However, some gRPC
/// Transcoding implementations may not support this feature.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRule {
    /// Selects a method to which this rule applies.
    ///
    /// Refer to [selector][google.api.DocumentationRule.selector] for syntax
    /// details.
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    /// The name of the request field whose value is mapped to the HTTP request
    /// body, or `*` for mapping all request fields not captured by the path
    /// pattern to the HTTP body, or omitted for not having any HTTP request body.
    ///
    /// NOTE: the referred field must be present at the top-level of the request
    /// message type.
    #[prost(string, tag = "7")]
    pub body: ::prost::alloc::string::String,
    /// Optional. The name of the response field whose value is mapped to the HTTP
    /// response body. When omitted, the entire response message will be used
    /// as the HTTP response body.
    ///
    /// NOTE: The referred field must be present at the top-level of the response
    /// message type.
    #[prost(string, tag = "12")]
    pub response_body: ::prost::alloc::string::String,
    /// Additional HTTP bindings for the selector. Nested bindings must
    /// not contain an `additional_bindings` field themselves (that is,
    /// the nesting may only be one level deep).
    #[prost(message, repeated, tag = "11")]
    pub additional_bindings: ::prost::alloc::vec::Vec<HttpRule>,
    /// Determines the URL pattern is matched by this rules. This pattern can be
    /// used with any of the {get|put|post|delete|patch} methods. A custom method
    /// can be defined using the 'custom' field.
    #[prost(oneof = "http_rule::Pattern", tags = "2, 3, 4, 5, 6, 8")]
    pub pattern: ::core::option::Option<http_rule::Pattern>,
}
/// Nested message and enum types in `HttpRule`.
pub mod http_rule {
    /// Determines the URL pattern is matched by this rules. This pattern can be
    /// used with any of the {get|put|post|delete|patch} methods. A custom method
    /// can be defined using the 'custom' field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Pattern {
        /// Maps to HTTP GET. Used for listing and getting information about
        /// resources.
        #[prost(string, tag = "2")]
        Get(::prost::alloc::string::String),
        /// Maps to HTTP PUT. Used for replacing a resource.
        #[prost(string, tag = "3")]
        Put(::prost::alloc::string::String),
        /// Maps to HTTP POST. Used for creating a resource or performing an action.
        #[prost(string, tag = "4")]
        Post(::prost::alloc::string::String),
        /// Maps to HTTP DELETE. Used for deleting a resource.
        #[prost(string, tag = "5")]
        Delete(::prost::alloc::string::String),
        /// Maps to HTTP PATCH. Used for updating a resource.
        #[prost(string, tag = "6")]
        Patch(::prost::alloc::string::String),
        /// The custom pattern is used for specifying an HTTP method that is not
        /// included in the `pattern` field, such as HEAD, or "*" to leave the
        /// HTTP method unspecified for this rule. The wild-card rule is useful
        /// for services that provide content to Web (HTML) clients.
        #[prost(message, tag = "8")]
        Custom(super::CustomHttpPattern),
    }
}
/// A custom pattern is used for defining custom HTTP verb.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomHttpPattern {
    /// The name of this custom HTTP verb.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The path matched by this custom verb.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
/// The launch stage as defined by [Google Cloud Platform
/// Launch Stages](<https://cloud.google.com/terms/launch-stages>).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LaunchStage {
    /// Do not use this default value.
    Unspecified = 0,
    /// The feature is not yet implemented. Users can not use it.
    Unimplemented = 6,
    /// Prelaunch features are hidden from users and are only visible internally.
    Prelaunch = 7,
    /// Early Access features are limited to a closed group of testers. To use
    /// these features, you must sign up in advance and sign a Trusted Tester
    /// agreement (which includes confidentiality provisions). These features may
    /// be unstable, changed in backward-incompatible ways, and are not
    /// guaranteed to be released.
    EarlyAccess = 1,
    /// Alpha is a limited availability test for releases before they are cleared
    /// for widespread use. By Alpha, all significant design issues are resolved
    /// and we are in the process of verifying functionality. Alpha customers
    /// need to apply for access, agree to applicable terms, and have their
    /// projects allowlisted. Alpha releases don't have to be feature complete,
    /// no SLAs are provided, and there are no technical support obligations, but
    /// they will be far enough along that customers can actually use them in
    /// test environments or for limited-use tests -- just like they would in
    /// normal production cases.
    Alpha = 2,
    /// Beta is the point at which we are ready to open a release for any
    /// customer to use. There are no SLA or technical support obligations in a
    /// Beta release. Products will be complete from a feature perspective, but
    /// may have some open outstanding issues. Beta releases are suitable for
    /// limited production use cases.
    Beta = 3,
    /// GA features are open to all developers and are considered stable and
    /// fully qualified for production use.
    Ga = 4,
    /// Deprecated features are scheduled to be shut down and removed. For more
    /// information, see the "Deprecation Policy" section of our [Terms of
    /// Service](<https://cloud.google.com/terms/>)
    /// and the [Google Cloud Platform Subject to the Deprecation
    /// Policy](<https://cloud.google.com/terms/deprecation>) documentation.
    Deprecated = 5,
}
impl LaunchStage {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LaunchStage::Unspecified => "LAUNCH_STAGE_UNSPECIFIED",
            LaunchStage::Unimplemented => "UNIMPLEMENTED",
            LaunchStage::Prelaunch => "PRELAUNCH",
            LaunchStage::EarlyAccess => "EARLY_ACCESS",
            LaunchStage::Alpha => "ALPHA",
            LaunchStage::Beta => "BETA",
            LaunchStage::Ga => "GA",
            LaunchStage::Deprecated => "DEPRECATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LAUNCH_STAGE_UNSPECIFIED" => Some(Self::Unspecified),
            "UNIMPLEMENTED" => Some(Self::Unimplemented),
            "PRELAUNCH" => Some(Self::Prelaunch),
            "EARLY_ACCESS" => Some(Self::EarlyAccess),
            "ALPHA" => Some(Self::Alpha),
            "BETA" => Some(Self::Beta),
            "GA" => Some(Self::Ga),
            "DEPRECATED" => Some(Self::Deprecated),
            _ => None,
        }
    }
}
/// Required information for every language.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonLanguageSettings {
    /// Link to automatically generated reference documentation.  Example:
    /// <https://cloud.google.com/nodejs/docs/reference/asset/latest>
    #[deprecated]
    #[prost(string, tag = "1")]
    pub reference_docs_uri: ::prost::alloc::string::String,
    /// The destination where API teams want this client library to be published.
    #[prost(enumeration = "ClientLibraryDestination", repeated, tag = "2")]
    pub destinations: ::prost::alloc::vec::Vec<i32>,
}
/// Details about how and where to publish client libraries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientLibrarySettings {
    /// Version of the API to apply these settings to. This is the full protobuf
    /// package for the API, ending in the version element.
    /// Examples: "google.cloud.speech.v1" and "google.spanner.admin.database.v1".
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// Launch stage of this version of the API.
    #[prost(enumeration = "LaunchStage", tag = "2")]
    pub launch_stage: i32,
    /// When using transport=rest, the client request will encode enums as
    /// numbers rather than strings.
    #[prost(bool, tag = "3")]
    pub rest_numeric_enums: bool,
    /// Settings for legacy Java features, supported in the Service YAML.
    #[prost(message, optional, tag = "21")]
    pub java_settings: ::core::option::Option<JavaSettings>,
    /// Settings for C++ client libraries.
    #[prost(message, optional, tag = "22")]
    pub cpp_settings: ::core::option::Option<CppSettings>,
    /// Settings for PHP client libraries.
    #[prost(message, optional, tag = "23")]
    pub php_settings: ::core::option::Option<PhpSettings>,
    /// Settings for Python client libraries.
    #[prost(message, optional, tag = "24")]
    pub python_settings: ::core::option::Option<PythonSettings>,
    /// Settings for Node client libraries.
    #[prost(message, optional, tag = "25")]
    pub node_settings: ::core::option::Option<NodeSettings>,
    /// Settings for .NET client libraries.
    #[prost(message, optional, tag = "26")]
    pub dotnet_settings: ::core::option::Option<DotnetSettings>,
    /// Settings for Ruby client libraries.
    #[prost(message, optional, tag = "27")]
    pub ruby_settings: ::core::option::Option<RubySettings>,
    /// Settings for Go client libraries.
    #[prost(message, optional, tag = "28")]
    pub go_settings: ::core::option::Option<GoSettings>,
}
/// This message configures the settings for publishing [Google Cloud Client
/// libraries](<https://cloud.google.com/apis/docs/cloud-client-libraries>)
/// generated from the service config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Publishing {
    /// A list of API method settings, e.g. the behavior for methods that use the
    /// long-running operation pattern.
    #[prost(message, repeated, tag = "2")]
    pub method_settings: ::prost::alloc::vec::Vec<MethodSettings>,
    /// Link to a *public* URI where users can report issues.  Example:
    /// <https://issuetracker.google.com/issues/new?component=190865&template=1161103>
    #[prost(string, tag = "101")]
    pub new_issue_uri: ::prost::alloc::string::String,
    /// Link to product home page.  Example:
    /// <https://cloud.google.com/asset-inventory/docs/overview>
    #[prost(string, tag = "102")]
    pub documentation_uri: ::prost::alloc::string::String,
    /// Used as a tracking tag when collecting data about the APIs developer
    /// relations artifacts like docs, packages delivered to package managers,
    /// etc.  Example: "speech".
    #[prost(string, tag = "103")]
    pub api_short_name: ::prost::alloc::string::String,
    /// GitHub label to apply to issues and pull requests opened for this API.
    #[prost(string, tag = "104")]
    pub github_label: ::prost::alloc::string::String,
    /// GitHub teams to be added to CODEOWNERS in the directory in GitHub
    /// containing source code for the client libraries for this API.
    #[prost(string, repeated, tag = "105")]
    pub codeowner_github_teams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A prefix used in sample code when demarking regions to be included in
    /// documentation.
    #[prost(string, tag = "106")]
    pub doc_tag_prefix: ::prost::alloc::string::String,
    /// For whom the client library is being published.
    #[prost(enumeration = "ClientLibraryOrganization", tag = "107")]
    pub organization: i32,
    /// Client library settings.  If the same version string appears multiple
    /// times in this list, then the last one wins.  Settings from earlier
    /// settings with the same version string are discarded.
    #[prost(message, repeated, tag = "109")]
    pub library_settings: ::prost::alloc::vec::Vec<ClientLibrarySettings>,
    /// Optional link to proto reference documentation.  Example:
    /// <https://cloud.google.com/pubsub/lite/docs/reference/rpc>
    #[prost(string, tag = "110")]
    pub proto_reference_documentation_uri: ::prost::alloc::string::String,
}
/// Settings for Java client libraries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JavaSettings {
    /// The package name to use in Java. Clobbers the java_package option
    /// set in the protobuf. This should be used **only** by APIs
    /// who have already set the language_settings.java.package_name" field
    /// in gapic.yaml. API teams should use the protobuf java_package option
    /// where possible.
    ///
    /// Example of a YAML configuration::
    ///
    ///   publishing:
    ///     java_settings:
    ///       library_package: com.google.cloud.pubsub.v1
    #[prost(string, tag = "1")]
    pub library_package: ::prost::alloc::string::String,
    /// Configure the Java class name to use instead of the service's for its
    /// corresponding generated GAPIC client. Keys are fully-qualified
    /// service names as they appear in the protobuf (including the full
    /// the language_settings.java.interface_names" field in gapic.yaml. API
    /// teams should otherwise use the service name as it appears in the
    /// protobuf.
    ///
    /// Example of a YAML configuration::
    ///
    ///   publishing:
    ///     java_settings:
    ///       service_class_names:
    ///         - google.pubsub.v1.Publisher: TopicAdmin
    ///         - google.pubsub.v1.Subscriber: SubscriptionAdmin
    #[prost(btree_map = "string, string", tag = "2")]
    pub service_class_names: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Some settings.
    #[prost(message, optional, tag = "3")]
    pub common: ::core::option::Option<CommonLanguageSettings>,
}
/// Settings for C++ client libraries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CppSettings {
    /// Some settings.
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonLanguageSettings>,
}
/// Settings for Php client libraries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhpSettings {
    /// Some settings.
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonLanguageSettings>,
}
/// Settings for Python client libraries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PythonSettings {
    /// Some settings.
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonLanguageSettings>,
}
/// Settings for Node client libraries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeSettings {
    /// Some settings.
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonLanguageSettings>,
}
/// Settings for Dotnet client libraries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DotnetSettings {
    /// Some settings.
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonLanguageSettings>,
    /// Map from original service names to renamed versions.
    /// This is used when the default generated types
    /// would cause a naming conflict. (Neither name is
    /// fully-qualified.)
    /// Example: Subscriber to SubscriberServiceApi.
    #[prost(btree_map = "string, string", tag = "2")]
    pub renamed_services: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Map from full resource types to the effective short name
    /// for the resource. This is used when otherwise resource
    /// named from different services would cause naming collisions.
    /// Example entry:
    /// "datalabeling.googleapis.com/Dataset": "DataLabelingDataset"
    #[prost(btree_map = "string, string", tag = "3")]
    pub renamed_resources: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// List of full resource types to ignore during generation.
    /// This is typically used for API-specific Location resources,
    /// which should be handled by the generator as if they were actually
    /// the common Location resources.
    /// Example entry: "documentai.googleapis.com/Location"
    #[prost(string, repeated, tag = "4")]
    pub ignored_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Namespaces which must be aliased in snippets due to
    /// a known (but non-generator-predictable) naming collision
    #[prost(string, repeated, tag = "5")]
    pub forced_namespace_aliases: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Method signatures (in the form "service.method(signature)")
    /// which are provided separately, so shouldn't be generated.
    /// Snippets *calling* these methods are still generated, however.
    #[prost(string, repeated, tag = "6")]
    pub handwritten_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Settings for Ruby client libraries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RubySettings {
    /// Some settings.
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonLanguageSettings>,
}
/// Settings for Go client libraries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoSettings {
    /// Some settings.
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonLanguageSettings>,
}
/// Describes the generator configuration for a method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodSettings {
    /// The fully qualified name of the method, for which the options below apply.
    /// This is used to find the method to apply the options.
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    /// Describes settings to use for long-running operations when generating
    /// API methods for RPCs. Complements RPCs that use the annotations in
    /// google/longrunning/operations.proto.
    ///
    /// Example of a YAML configuration::
    ///
    ///   publishing:
    ///     method_settings:
    ///       - selector: google.cloud.speech.v2.Speech.BatchRecognize
    ///         long_running:
    ///           initial_poll_delay:
    ///             seconds: 60 # 1 minute
    ///           poll_delay_multiplier: 1.5
    ///           max_poll_delay:
    ///             seconds: 360 # 6 minutes
    ///           total_poll_timeout:
    ///              seconds: 54000 # 90 minutes
    #[prost(message, optional, tag = "2")]
    pub long_running: ::core::option::Option<method_settings::LongRunning>,
    /// List of top-level fields of the request message, that should be
    /// automatically populated by the client libraries based on their
    /// (google.api.field_info).format. Currently supported format: UUID4.
    ///
    /// Example of a YAML configuration:
    ///
    ///   publishing:
    ///     method_settings:
    ///       - selector: google.example.v1.ExampleService.CreateExample
    ///         auto_populated_fields:
    ///         - request_id
    #[prost(string, repeated, tag = "3")]
    pub auto_populated_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `MethodSettings`.
pub mod method_settings {
    /// Describes settings to use when generating API methods that use the
    /// long-running operation pattern.
    /// All default values below are from those used in the client library
    /// generators (e.g.
    /// [Java](<https://github.com/googleapis/gapic-generator-java/blob/04c2faa191a9b5a10b92392fe8482279c4404803/src/main/java/com/google/api/generator/gapic/composer/common/RetrySettingsComposer.java>)).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LongRunning {
        /// Initial delay after which the first poll request will be made.
        /// Default value: 5 seconds.
        #[prost(message, optional, tag = "1")]
        pub initial_poll_delay: ::core::option::Option<::prost_types::Duration>,
        /// Multiplier to gradually increase delay between subsequent polls until it
        /// reaches max_poll_delay.
        /// Default value: 1.5.
        #[prost(float, tag = "2")]
        pub poll_delay_multiplier: f32,
        /// Maximum time between two subsequent poll requests.
        /// Default value: 45 seconds.
        #[prost(message, optional, tag = "3")]
        pub max_poll_delay: ::core::option::Option<::prost_types::Duration>,
        /// Total polling timeout.
        /// Default value: 5 minutes.
        #[prost(message, optional, tag = "4")]
        pub total_poll_timeout: ::core::option::Option<::prost_types::Duration>,
    }
}
/// The organization for which the client libraries are being published.
/// Affects the url where generated docs are published, etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientLibraryOrganization {
    /// Not useful.
    Unspecified = 0,
    /// Google Cloud Platform Org.
    Cloud = 1,
    /// Ads (Advertising) Org.
    Ads = 2,
    /// Photos Org.
    Photos = 3,
    /// Street View Org.
    StreetView = 4,
    /// Shopping Org.
    Shopping = 5,
    /// Geo Org.
    Geo = 6,
    /// Generative AI - <https://developers.generativeai.google>
    GenerativeAi = 7,
}
impl ClientLibraryOrganization {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClientLibraryOrganization::Unspecified => {
                "CLIENT_LIBRARY_ORGANIZATION_UNSPECIFIED"
            }
            ClientLibraryOrganization::Cloud => "CLOUD",
            ClientLibraryOrganization::Ads => "ADS",
            ClientLibraryOrganization::Photos => "PHOTOS",
            ClientLibraryOrganization::StreetView => "STREET_VIEW",
            ClientLibraryOrganization::Shopping => "SHOPPING",
            ClientLibraryOrganization::Geo => "GEO",
            ClientLibraryOrganization::GenerativeAi => "GENERATIVE_AI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLIENT_LIBRARY_ORGANIZATION_UNSPECIFIED" => Some(Self::Unspecified),
            "CLOUD" => Some(Self::Cloud),
            "ADS" => Some(Self::Ads),
            "PHOTOS" => Some(Self::Photos),
            "STREET_VIEW" => Some(Self::StreetView),
            "SHOPPING" => Some(Self::Shopping),
            "GEO" => Some(Self::Geo),
            "GENERATIVE_AI" => Some(Self::GenerativeAi),
            _ => None,
        }
    }
}
/// To where should client libraries be published?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientLibraryDestination {
    /// Client libraries will neither be generated nor published to package
    /// managers.
    Unspecified = 0,
    /// Generate the client library in a repo under github.com/googleapis,
    /// but don't publish it to package managers.
    Github = 10,
    /// Publish the library to package managers like nuget.org and npmjs.com.
    PackageManager = 20,
}
impl ClientLibraryDestination {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClientLibraryDestination::Unspecified => {
                "CLIENT_LIBRARY_DESTINATION_UNSPECIFIED"
            }
            ClientLibraryDestination::Github => "GITHUB",
            ClientLibraryDestination::PackageManager => "PACKAGE_MANAGER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLIENT_LIBRARY_DESTINATION_UNSPECIFIED" => Some(Self::Unspecified),
            "GITHUB" => Some(Self::Github),
            "PACKAGE_MANAGER" => Some(Self::PackageManager),
            _ => None,
        }
    }
}
/// A simple descriptor of a resource type.
///
/// ResourceDescriptor annotates a resource message (either by means of a
/// protobuf annotation or use in the service config), and associates the
/// resource's schema, the resource type, and the pattern of the resource name.
///
/// Example:
///
///      message Topic {
///        // Indicates this message defines a resource schema.
///        // Declares the resource type in the format of {service}/{kind}.
///        // For Kubernetes resources, the format is {api group}/{kind}.
///        option (google.api.resource) = {
///          type: "pubsub.googleapis.com/Topic"
///          pattern: "projects/{project}/topics/{topic}"
///        };
///      }
///
/// The ResourceDescriptor Yaml config will look like:
///
///      resources:
///      - type: "pubsub.googleapis.com/Topic"
///        pattern: "projects/{project}/topics/{topic}"
///
/// Sometimes, resources have multiple patterns, typically because they can
/// live under multiple parents.
///
/// Example:
///
///      message LogEntry {
///        option (google.api.resource) = {
///          type: "logging.googleapis.com/LogEntry"
///          pattern: "projects/{project}/logs/{log}"
///          pattern: "folders/{folder}/logs/{log}"
///          pattern: "organizations/{organization}/logs/{log}"
///          pattern: "billingAccounts/{billing_account}/logs/{log}"
///        };
///      }
///
/// The ResourceDescriptor Yaml config will look like:
///
///      resources:
///      - type: 'logging.googleapis.com/LogEntry'
///        pattern: "projects/{project}/logs/{log}"
///        pattern: "folders/{folder}/logs/{log}"
///        pattern: "organizations/{organization}/logs/{log}"
///        pattern: "billingAccounts/{billing_account}/logs/{log}"
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceDescriptor {
    /// The resource type. It must be in the format of
    /// {service_name}/{resource_type_kind}. The `resource_type_kind` must be
    /// singular and must not include version numbers.
    ///
    /// Example: `storage.googleapis.com/Bucket`
    ///
    /// The value of the resource_type_kind must follow the regular expression
    /// /[A-Za-z][a-zA-Z0-9]+/. It should start with an upper case character and
    /// should use PascalCase (UpperCamelCase). The maximum number of
    /// characters allowed for the `resource_type_kind` is 100.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Optional. The relative resource name pattern associated with this resource
    /// type. The DNS prefix of the full resource name shouldn't be specified here.
    ///
    /// The path pattern must follow the syntax, which aligns with HTTP binding
    /// syntax:
    ///
    ///      Template = Segment { "/" Segment } ;
    ///      Segment = LITERAL | Variable ;
    ///      Variable = "{" LITERAL "}" ;
    ///
    /// Examples:
    ///
    ///      - "projects/{project}/topics/{topic}"
    ///      - "projects/{project}/knowledgeBases/{knowledge_base}"
    ///
    /// The components in braces correspond to the IDs for each resource in the
    /// hierarchy. It is expected that, if multiple patterns are provided,
    /// the same component name (e.g. "project") refers to IDs of the same
    /// type of resource.
    #[prost(string, repeated, tag = "2")]
    pub pattern: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The field on the resource that designates the resource name
    /// field. If omitted, this is assumed to be "name".
    #[prost(string, tag = "3")]
    pub name_field: ::prost::alloc::string::String,
    /// Optional. The historical or future-looking state of the resource pattern.
    ///
    /// Example:
    ///
    ///      // The InspectTemplate message originally only supported resource
    ///      // names with organization, and project was added later.
    ///      message InspectTemplate {
    ///        option (google.api.resource) = {
    ///          type: "dlp.googleapis.com/InspectTemplate"
    ///          pattern:
    ///          "organizations/{organization}/inspectTemplates/{inspect_template}"
    ///          pattern: "projects/{project}/inspectTemplates/{inspect_template}"
    ///          history: ORIGINALLY_SINGLE_PATTERN
    ///        };
    ///      }
    #[prost(enumeration = "resource_descriptor::History", tag = "4")]
    pub history: i32,
    /// The plural name used in the resource name and permission names, such as
    /// 'projects' for the resource name of 'projects/{project}' and the permission
    /// name of 'cloudresourcemanager.googleapis.com/projects.get'. It is the same
    /// concept of the `plural` field in k8s CRD spec
    /// <https://kubernetes.io/docs/tasks/access-kubernetes-api/custom-resources/custom-resource-definitions/>
    ///
    /// Note: The plural form is required even for singleton resources. See
    /// <https://aip.dev/156>
    #[prost(string, tag = "5")]
    pub plural: ::prost::alloc::string::String,
    /// The same concept of the `singular` field in k8s CRD spec
    /// <https://kubernetes.io/docs/tasks/access-kubernetes-api/custom-resources/custom-resource-definitions/>
    /// Such as "project" for the `resourcemanager.googleapis.com/Project` type.
    #[prost(string, tag = "6")]
    pub singular: ::prost::alloc::string::String,
    /// Style flag(s) for this resource.
    /// These indicate that a resource is expected to conform to a given
    /// style. See the specific style flags for additional information.
    #[prost(enumeration = "resource_descriptor::Style", repeated, tag = "10")]
    pub style: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `ResourceDescriptor`.
pub mod resource_descriptor {
    /// A description of the historical or future-looking state of the
    /// resource pattern.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum History {
        /// The "unset" value.
        Unspecified = 0,
        /// The resource originally had one pattern and launched as such, and
        /// additional patterns were added later.
        OriginallySinglePattern = 1,
        /// The resource has one pattern, but the API owner expects to add more
        /// later. (This is the inverse of ORIGINALLY_SINGLE_PATTERN, and prevents
        /// that from being necessary once there are multiple patterns.)
        FutureMultiPattern = 2,
    }
    impl History {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                History::Unspecified => "HISTORY_UNSPECIFIED",
                History::OriginallySinglePattern => "ORIGINALLY_SINGLE_PATTERN",
                History::FutureMultiPattern => "FUTURE_MULTI_PATTERN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HISTORY_UNSPECIFIED" => Some(Self::Unspecified),
                "ORIGINALLY_SINGLE_PATTERN" => Some(Self::OriginallySinglePattern),
                "FUTURE_MULTI_PATTERN" => Some(Self::FutureMultiPattern),
                _ => None,
            }
        }
    }
    /// A flag representing a specific style that a resource claims to conform to.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Style {
        /// The unspecified value. Do not use.
        Unspecified = 0,
        /// This resource is intended to be "declarative-friendly".
        ///
        /// Declarative-friendly resources must be more strictly consistent, and
        /// setting this to true communicates to tools that this resource should
        /// adhere to declarative-friendly expectations.
        ///
        /// Note: This is used by the API linter (linter.aip.dev) to enable
        /// additional checks.
        DeclarativeFriendly = 1,
    }
    impl Style {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Style::Unspecified => "STYLE_UNSPECIFIED",
                Style::DeclarativeFriendly => "DECLARATIVE_FRIENDLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STYLE_UNSPECIFIED" => Some(Self::Unspecified),
                "DECLARATIVE_FRIENDLY" => Some(Self::DeclarativeFriendly),
                _ => None,
            }
        }
    }
}
/// Defines a proto annotation that describes a string field that refers to
/// an API resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceReference {
    /// The resource type that the annotated field references.
    ///
    /// Example:
    ///
    ///      message Subscription {
    ///        string topic = 2 [(google.api.resource_reference) = {
    ///          type: "pubsub.googleapis.com/Topic"
    ///        }];
    ///      }
    ///
    /// Occasionally, a field may reference an arbitrary resource. In this case,
    /// APIs use the special value * in their resource reference.
    ///
    /// Example:
    ///
    ///      message GetIamPolicyRequest {
    ///        string resource = 2 [(google.api.resource_reference) = {
    ///          type: "*"
    ///        }];
    ///      }
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// The resource type of a child collection that the annotated field
    /// references. This is useful for annotating the `parent` field that
    /// doesn't have a fixed resource type.
    ///
    /// Example:
    ///
    ///      message ListLogEntriesRequest {
    ///        string parent = 1 [(google.api.resource_reference) = {
    ///          child_type: "logging.googleapis.com/LogEntry"
    ///        };
    ///      }
    #[prost(string, tag = "2")]
    pub child_type: ::prost::alloc::string::String,
}
/// Specifies the routing information that should be sent along with the request
/// in the form of routing header.
/// **NOTE:** All service configuration rules follow the "last one wins" order.
///
/// The examples below will apply to an RPC which has the following request type:
///
/// Message Definition:
///
///      message Request {
///        // The name of the Table
///        // Values can be of the following formats:
///        // - `projects/<project>/tables/<table>`
///        // - `projects/<project>/instances/<instance>/tables/<table>`
///        // - `region/<region>/zones/<zone>/tables/<table>`
///        string table_name = 1;
///
///        // This value specifies routing for replication.
///        // It can be in the following formats:
///        // - `profiles/<profile_id>`
///        // - a legacy `profile_id` that can be any string
///        string app_profile_id = 2;
///      }
///
/// Example message:
///
///      {
///        table_name: projects/proj_foo/instances/instance_bar/table/table_baz,
///        app_profile_id: profiles/prof_qux
///      }
///
/// The routing header consists of one or multiple key-value pairs. Every key
/// and value must be percent-encoded, and joined together in the format of
/// `key1=value1&key2=value2`.
/// In the examples below I am skipping the percent-encoding for readablity.
///
/// Example 1
///
/// Extracting a field from the request to put into the routing header
/// unchanged, with the key equal to the field name.
///
/// annotation:
///
///      option (google.api.routing) = {
///        // Take the `app_profile_id`.
///        routing_parameters {
///          field: "app_profile_id"
///        }
///      };
///
/// result:
///
///      x-goog-request-params: app_profile_id=profiles/prof_qux
///
/// Example 2
///
/// Extracting a field from the request to put into the routing header
/// unchanged, with the key different from the field name.
///
/// annotation:
///
///      option (google.api.routing) = {
///        // Take the `app_profile_id`, but name it `routing_id` in the header.
///        routing_parameters {
///          field: "app_profile_id"
///          path_template: "{routing_id=**}"
///        }
///      };
///
/// result:
///
///      x-goog-request-params: routing_id=profiles/prof_qux
///
/// Example 3
///
/// Extracting a field from the request to put into the routing
/// header, while matching a path template syntax on the field's value.
///
/// NB: it is more useful to send nothing than to send garbage for the purpose
/// of dynamic routing, since garbage pollutes cache. Thus the matching.
///
/// Sub-example 3a
///
/// The field matches the template.
///
/// annotation:
///
///      option (google.api.routing) = {
///        // Take the `table_name`, if it's well-formed (with project-based
///        // syntax).
///        routing_parameters {
///          field: "table_name"
///          path_template: "{table_name=projects/*/instances/*/**}"
///        }
///      };
///
/// result:
///
///      x-goog-request-params:
///      table_name=projects/proj_foo/instances/instance_bar/table/table_baz
///
/// Sub-example 3b
///
/// The field does not match the template.
///
/// annotation:
///
///      option (google.api.routing) = {
///        // Take the `table_name`, if it's well-formed (with region-based
///        // syntax).
///        routing_parameters {
///          field: "table_name"
///          path_template: "{table_name=regions/*/zones/*/**}"
///        }
///      };
///
/// result:
///
///      <no routing header will be sent>
///
/// Sub-example 3c
///
/// Multiple alternative conflictingly named path templates are
/// specified. The one that matches is used to construct the header.
///
/// annotation:
///
///      option (google.api.routing) = {
///        // Take the `table_name`, if it's well-formed, whether
///        // using the region- or projects-based syntax.
///
///        routing_parameters {
///          field: "table_name"
///          path_template: "{table_name=regions/*/zones/*/**}"
///        }
///        routing_parameters {
///          field: "table_name"
///          path_template: "{table_name=projects/*/instances/*/**}"
///        }
///      };
///
/// result:
///
///      x-goog-request-params:
///      table_name=projects/proj_foo/instances/instance_bar/table/table_baz
///
/// Example 4
///
/// Extracting a single routing header key-value pair by matching a
/// template syntax on (a part of) a single request field.
///
/// annotation:
///
///      option (google.api.routing) = {
///        // Take just the project id from the `table_name` field.
///        routing_parameters {
///          field: "table_name"
///          path_template: "{routing_id=projects/*}/**"
///        }
///      };
///
/// result:
///
///      x-goog-request-params: routing_id=projects/proj_foo
///
/// Example 5
///
/// Extracting a single routing header key-value pair by matching
/// several conflictingly named path templates on (parts of) a single request
/// field. The last template to match "wins" the conflict.
///
/// annotation:
///
///      option (google.api.routing) = {
///        // If the `table_name` does not have instances information,
///        // take just the project id for routing.
///        // Otherwise take project + instance.
///
///        routing_parameters {
///          field: "table_name"
///          path_template: "{routing_id=projects/*}/**"
///        }
///        routing_parameters {
///          field: "table_name"
///          path_template: "{routing_id=projects/*/instances/*}/**"
///        }
///      };
///
/// result:
///
///      x-goog-request-params:
///      routing_id=projects/proj_foo/instances/instance_bar
///
/// Example 6
///
/// Extracting multiple routing header key-value pairs by matching
/// several non-conflicting path templates on (parts of) a single request field.
///
/// Sub-example 6a
///
/// Make the templates strict, so that if the `table_name` does not
/// have an instance information, nothing is sent.
///
/// annotation:
///
///      option (google.api.routing) = {
///        // The routing code needs two keys instead of one composite
///        // but works only for the tables with the "project-instance" name
///        // syntax.
///
///        routing_parameters {
///          field: "table_name"
///          path_template: "{project_id=projects/*}/instances/*/**"
///        }
///        routing_parameters {
///          field: "table_name"
///          path_template: "projects/*/{instance_id=instances/*}/**"
///        }
///      };
///
/// result:
///
///      x-goog-request-params:
///      project_id=projects/proj_foo&instance_id=instances/instance_bar
///
/// Sub-example 6b
///
/// Make the templates loose, so that if the `table_name` does not
/// have an instance information, just the project id part is sent.
///
/// annotation:
///
///      option (google.api.routing) = {
///        // The routing code wants two keys instead of one composite
///        // but will work with just the `project_id` for tables without
///        // an instance in the `table_name`.
///
///        routing_parameters {
///          field: "table_name"
///          path_template: "{project_id=projects/*}/**"
///        }
///        routing_parameters {
///          field: "table_name"
///          path_template: "projects/*/{instance_id=instances/*}/**"
///        }
///      };
///
/// result (is the same as 6a for our example message because it has the instance
/// information):
///
///      x-goog-request-params:
///      project_id=projects/proj_foo&instance_id=instances/instance_bar
///
/// Example 7
///
/// Extracting multiple routing header key-value pairs by matching
/// several path templates on multiple request fields.
///
/// NB: note that here there is no way to specify sending nothing if one of the
/// fields does not match its template. E.g. if the `table_name` is in the wrong
/// format, the `project_id` will not be sent, but the `routing_id` will be.
/// The backend routing code has to be aware of that and be prepared to not
/// receive a full complement of keys if it expects multiple.
///
/// annotation:
///
///      option (google.api.routing) = {
///        // The routing needs both `project_id` and `routing_id`
///        // (from the `app_profile_id` field) for routing.
///
///        routing_parameters {
///          field: "table_name"
///          path_template: "{project_id=projects/*}/**"
///        }
///        routing_parameters {
///          field: "app_profile_id"
///          path_template: "{routing_id=**}"
///        }
///      };
///
/// result:
///
///      x-goog-request-params:
///      project_id=projects/proj_foo&routing_id=profiles/prof_qux
///
/// Example 8
///
/// Extracting a single routing header key-value pair by matching
/// several conflictingly named path templates on several request fields. The
/// last template to match "wins" the conflict.
///
/// annotation:
///
///      option (google.api.routing) = {
///        // The `routing_id` can be a project id or a region id depending on
///        // the table name format, but only if the `app_profile_id` is not set.
///        // If `app_profile_id` is set it should be used instead.
///
///        routing_parameters {
///          field: "table_name"
///          path_template: "{routing_id=projects/*}/**"
///        }
///        routing_parameters {
///           field: "table_name"
///           path_template: "{routing_id=regions/*}/**"
///        }
///        routing_parameters {
///          field: "app_profile_id"
///          path_template: "{routing_id=**}"
///        }
///      };
///
/// result:
///
///      x-goog-request-params: routing_id=profiles/prof_qux
///
/// Example 9
///
/// Bringing it all together.
///
/// annotation:
///
///      option (google.api.routing) = {
///        // For routing both `table_location` and a `routing_id` are needed.
///        //
///        // table_location can be either an instance id or a region+zone id.
///        //
///        // For `routing_id`, take the value of `app_profile_id`
///        // - If it's in the format `profiles/<profile_id>`, send
///        // just the `<profile_id>` part.
///        // - If it's any other literal, send it as is.
///        // If the `app_profile_id` is empty, and the `table_name` starts with
///        // the project_id, send that instead.
///
///        routing_parameters {
///          field: "table_name"
///          path_template: "projects/*/{table_location=instances/*}/tables/*"
///        }
///        routing_parameters {
///          field: "table_name"
///          path_template: "{table_location=regions/*/zones/*}/tables/*"
///        }
///        routing_parameters {
///          field: "table_name"
///          path_template: "{routing_id=projects/*}/**"
///        }
///        routing_parameters {
///          field: "app_profile_id"
///          path_template: "{routing_id=**}"
///        }
///        routing_parameters {
///          field: "app_profile_id"
///          path_template: "profiles/{routing_id=*}"
///        }
///      };
///
/// result:
///
///      x-goog-request-params:
///      table_location=instances/instance_bar&routing_id=prof_qux
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingRule {
    /// A collection of Routing Parameter specifications.
    /// **NOTE:** If multiple Routing Parameters describe the same key
    /// (via the `path_template` field or via the `field` field when
    /// `path_template` is not provided), "last one wins" rule
    /// determines which Parameter gets used.
    /// See the examples for more details.
    #[prost(message, repeated, tag = "2")]
    pub routing_parameters: ::prost::alloc::vec::Vec<RoutingParameter>,
}
/// A projection from an input message to the GRPC or REST header.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingParameter {
    /// A request field to extract the header key-value pair from.
    #[prost(string, tag = "1")]
    pub field: ::prost::alloc::string::String,
    /// A pattern matching the key-value field. Optional.
    /// If not specified, the whole field specified in the `field` field will be
    /// taken as value, and its name used as key. If specified, it MUST contain
    /// exactly one named segment (along with any number of unnamed segments) The
    /// pattern will be matched over the field specified in the `field` field, then
    /// if the match is successful:
    /// - the name of the single named segment will be used as a header name,
    /// - the match value of the segment will be used as a header value;
    /// if the match is NOT successful, nothing will be sent.
    ///
    /// Example:
    ///
    ///                -- This is a field in the request message
    ///               |   that the header value will be extracted from.
    ///               |
    ///               |                     -- This is the key name in the
    ///               |                    |   routing header.
    ///               V                    |
    ///      field: "table_name"           v
    ///      path_template: "projects/*/{table_location=instances/*}/tables/*"
    ///                                                 ^            ^
    ///                                                 |            |
    ///        In the {} brackets is the pattern that --             |
    ///        specifies what to extract from the                    |
    ///        field as a value to be sent.                          |
    ///                                                              |
    ///       The string in the field must match the whole pattern --
    ///       before brackets, inside brackets, after brackets.
    ///
    /// When looking at this specific example, we can see that:
    /// - A key-value pair with the key `table_location`
    ///    and the value matching `instances/*` should be added
    ///    to the x-goog-request-params routing header.
    /// - The value is extracted from the request message's `table_name` field
    ///    if it matches the full pattern specified:
    ///    `projects/*/instances/*/tables/*`.
    ///
    /// **NB:** If the `path_template` field is not provided, the key name is
    /// equal to the field name, and the whole field should be sent as a value.
    /// This makes the pattern for the field and the value functionally equivalent
    /// to `**`, and the configuration
    ///
    ///      {
    ///        field: "table_name"
    ///      }
    ///
    /// is a functionally equivalent shorthand to:
    ///
    ///      {
    ///        field: "table_name"
    ///        path_template: "{table_name=**}"
    ///      }
    ///
    /// See Example 1 for more details.
    #[prost(string, tag = "2")]
    pub path_template: ::prost::alloc::string::String,
}
/// Message that represents an arbitrary HTTP body. It should only be used for
/// payload formats that can't be represented as JSON, such as raw binary or
/// an HTML page.
///
///
/// This message can be used both in streaming and non-streaming API methods in
/// the request as well as the response.
///
/// It can be used as a top-level request field, which is convenient if one
/// wants to extract parameters from either the URL or HTTP template into the
/// request fields and also want access to the raw HTTP body.
///
/// Example:
///
///      message GetResourceRequest {
///        // A unique request id.
///        string request_id = 1;
///
///        // The raw HTTP body is bound to this field.
///        google.api.HttpBody http_body = 2;
///
///      }
///
///      service ResourceService {
///        rpc GetResource(GetResourceRequest)
///          returns (google.api.HttpBody);
///        rpc UpdateResource(google.api.HttpBody)
///          returns (google.protobuf.Empty);
///
///      }
///
/// Example with streaming methods:
///
///      service CaldavService {
///        rpc GetCalendar(stream google.api.HttpBody)
///          returns (stream google.api.HttpBody);
///        rpc UpdateCalendar(stream google.api.HttpBody)
///          returns (stream google.api.HttpBody);
///
///      }
///
/// Use of this type only changes how the request and response bodies are
/// handled, all other features will continue to work unchanged.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpBody {
    /// The HTTP Content-Type header value specifying the content type of the body.
    #[prost(string, tag = "1")]
    pub content_type: ::prost::alloc::string::String,
    /// The HTTP request/response body as raw binary.
    #[prost(bytes = "bytes", tag = "2")]
    pub data: ::prost::bytes::Bytes,
    /// Application specific response metadata. Must be set in the first response
    /// for streaming APIs.
    #[prost(message, repeated, tag = "3")]
    pub extensions: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// `Distribution` contains summary statistics for a population of values. It
/// optionally contains a histogram representing the distribution of those values
/// across a set of buckets.
///
/// The summary statistics are the count, mean, sum of the squared deviation from
/// the mean, the minimum, and the maximum of the set of population of values.
/// The histogram is based on a sequence of buckets and gives a count of values
/// that fall into each bucket. The boundaries of the buckets are given either
/// explicitly or by formulas for buckets of fixed or exponentially increasing
/// widths.
///
/// Although it is not forbidden, it is generally a bad idea to include
/// non-finite values (infinities or NaNs) in the population of values, as this
/// will render the `mean` and `sum_of_squared_deviation` fields meaningless.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Distribution {
    /// The number of values in the population. Must be non-negative. This value
    /// must equal the sum of the values in `bucket_counts` if a histogram is
    /// provided.
    #[prost(int64, tag = "1")]
    pub count: i64,
    /// The arithmetic mean of the values in the population. If `count` is zero
    /// then this field must be zero.
    #[prost(double, tag = "2")]
    pub mean: f64,
    /// The sum of squared deviations from the mean of the values in the
    /// population. For values x_i this is:
    ///
    ///      Sum[i=1..n]((x_i - mean)^2)
    ///
    /// Knuth, "The Art of Computer Programming", Vol. 2, page 232, 3rd edition
    /// describes Welford's method for accumulating this sum in one pass.
    ///
    /// If `count` is zero then this field must be zero.
    #[prost(double, tag = "3")]
    pub sum_of_squared_deviation: f64,
    /// If specified, contains the range of the population values. The field
    /// must not be present if the `count` is zero.
    #[prost(message, optional, tag = "4")]
    pub range: ::core::option::Option<distribution::Range>,
    /// Defines the histogram bucket boundaries. If the distribution does not
    /// contain a histogram, then omit this field.
    #[prost(message, optional, tag = "6")]
    pub bucket_options: ::core::option::Option<distribution::BucketOptions>,
    /// The number of values in each bucket of the histogram, as described in
    /// `bucket_options`. If the distribution does not have a histogram, then omit
    /// this field. If there is a histogram, then the sum of the values in
    /// `bucket_counts` must equal the value in the `count` field of the
    /// distribution.
    ///
    /// If present, `bucket_counts` should contain N values, where N is the number
    /// of buckets specified in `bucket_options`. If you supply fewer than N
    /// values, the remaining values are assumed to be 0.
    ///
    /// The order of the values in `bucket_counts` follows the bucket numbering
    /// schemes described for the three bucket types. The first value must be the
    /// count for the underflow bucket (number 0). The next N-2 values are the
    /// counts for the finite buckets (number 1 through N-2). The N'th value in
    /// `bucket_counts` is the count for the overflow bucket (number N-1).
    #[prost(int64, repeated, tag = "7")]
    pub bucket_counts: ::prost::alloc::vec::Vec<i64>,
    /// Must be in increasing order of `value` field.
    #[prost(message, repeated, tag = "10")]
    pub exemplars: ::prost::alloc::vec::Vec<distribution::Exemplar>,
}
/// Nested message and enum types in `Distribution`.
pub mod distribution {
    /// The range of the population values.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Range {
        /// The minimum of the population values.
        #[prost(double, tag = "1")]
        pub min: f64,
        /// The maximum of the population values.
        #[prost(double, tag = "2")]
        pub max: f64,
    }
    /// `BucketOptions` describes the bucket boundaries used to create a histogram
    /// for the distribution. The buckets can be in a linear sequence, an
    /// exponential sequence, or each bucket can be specified explicitly.
    /// `BucketOptions` does not include the number of values in each bucket.
    ///
    /// A bucket has an inclusive lower bound and exclusive upper bound for the
    /// values that are counted for that bucket. The upper bound of a bucket must
    /// be strictly greater than the lower bound. The sequence of N buckets for a
    /// distribution consists of an underflow bucket (number 0), zero or more
    /// finite buckets (number 1 through N - 2) and an overflow bucket (number N -
    /// 1). The buckets are contiguous: the lower bound of bucket i (i > 0) is the
    /// same as the upper bound of bucket i - 1. The buckets span the whole range
    /// of finite values: lower bound of the underflow bucket is -infinity and the
    /// upper bound of the overflow bucket is +infinity. The finite buckets are
    /// so-called because both bounds are finite.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BucketOptions {
        /// Exactly one of these three fields must be set.
        #[prost(oneof = "bucket_options::Options", tags = "1, 2, 3")]
        pub options: ::core::option::Option<bucket_options::Options>,
    }
    /// Nested message and enum types in `BucketOptions`.
    pub mod bucket_options {
        /// Specifies a linear sequence of buckets that all have the same width
        /// (except overflow and underflow). Each bucket represents a constant
        /// absolute uncertainty on the specific value in the bucket.
        ///
        /// There are `num_finite_buckets + 2` (= N) buckets. Bucket `i` has the
        /// following boundaries:
        ///
        ///     Upper bound (0 <= i < N-1):     offset + (width * i).
        ///
        ///     Lower bound (1 <= i < N):       offset + (width * (i - 1)).
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Linear {
            /// Must be greater than 0.
            #[prost(int32, tag = "1")]
            pub num_finite_buckets: i32,
            /// Must be greater than 0.
            #[prost(double, tag = "2")]
            pub width: f64,
            /// Lower bound of the first bucket.
            #[prost(double, tag = "3")]
            pub offset: f64,
        }
        /// Specifies an exponential sequence of buckets that have a width that is
        /// proportional to the value of the lower bound. Each bucket represents a
        /// constant relative uncertainty on a specific value in the bucket.
        ///
        /// There are `num_finite_buckets + 2` (= N) buckets. Bucket `i` has the
        /// following boundaries:
        ///
        ///     Upper bound (0 <= i < N-1):     scale * (growth_factor ^ i).
        ///
        ///     Lower bound (1 <= i < N):       scale * (growth_factor ^ (i - 1)).
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Exponential {
            /// Must be greater than 0.
            #[prost(int32, tag = "1")]
            pub num_finite_buckets: i32,
            /// Must be greater than 1.
            #[prost(double, tag = "2")]
            pub growth_factor: f64,
            /// Must be greater than 0.
            #[prost(double, tag = "3")]
            pub scale: f64,
        }
        /// Specifies a set of buckets with arbitrary widths.
        ///
        /// There are `size(bounds) + 1` (= N) buckets. Bucket `i` has the following
        /// boundaries:
        ///
        ///     Upper bound (0 <= i < N-1):     bounds\[i\]
        ///     Lower bound (1 <= i < N);       bounds\[i - 1\]
        ///
        /// The `bounds` field must contain at least one element. If `bounds` has
        /// only one element, then there are no finite buckets, and that single
        /// element is the common boundary of the overflow and underflow buckets.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Explicit {
            /// The values must be monotonically increasing.
            #[prost(double, repeated, tag = "1")]
            pub bounds: ::prost::alloc::vec::Vec<f64>,
        }
        /// Exactly one of these three fields must be set.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Options {
            /// The linear bucket.
            #[prost(message, tag = "1")]
            LinearBuckets(Linear),
            /// The exponential buckets.
            #[prost(message, tag = "2")]
            ExponentialBuckets(Exponential),
            /// The explicit buckets.
            #[prost(message, tag = "3")]
            ExplicitBuckets(Explicit),
        }
    }
    /// Exemplars are example points that may be used to annotate aggregated
    /// distribution values. They are metadata that gives information about a
    /// particular value added to a Distribution bucket, such as a trace ID that
    /// was active when a value was added. They may contain further information,
    /// such as a example values and timestamps, origin, etc.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Exemplar {
        /// Value of the exemplar point. This value determines to which bucket the
        /// exemplar belongs.
        #[prost(double, tag = "1")]
        pub value: f64,
        /// The observation (sampling) time of the above value.
        #[prost(message, optional, tag = "2")]
        pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
        /// Contextual information about the example value. Examples are:
        ///
        ///    Trace: type.googleapis.com/google.monitoring.v3.SpanContext
        ///
        ///    Literal string: type.googleapis.com/google.protobuf.StringValue
        ///
        ///    Labels dropped during aggregation:
        ///      type.googleapis.com/google.monitoring.v3.DroppedLabels
        ///
        /// There may be only a single attachment of any given message type in a
        /// single exemplar, and this is enforced by the system.
        #[prost(message, repeated, tag = "3")]
        pub attachments: ::prost::alloc::vec::Vec<::prost_types::Any>,
    }
}
/// A description of a label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelDescriptor {
    /// The label key.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The type of data that can be assigned to the label.
    #[prost(enumeration = "label_descriptor::ValueType", tag = "2")]
    pub value_type: i32,
    /// A human-readable description for the label.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LabelDescriptor`.
pub mod label_descriptor {
    /// Value types that can be used as label values.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ValueType {
        /// A variable-length string. This is the default.
        String = 0,
        /// Boolean; true or false.
        Bool = 1,
        /// A 64-bit signed integer.
        Int64 = 2,
    }
    impl ValueType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueType::String => "STRING",
                ValueType::Bool => "BOOL",
                ValueType::Int64 => "INT64",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STRING" => Some(Self::String),
                "BOOL" => Some(Self::Bool),
                "INT64" => Some(Self::Int64),
                _ => None,
            }
        }
    }
}
/// Defines a metric type and its schema. Once a metric descriptor is created,
/// deleting or altering it stops data collection and makes the metric type's
/// existing data unusable.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricDescriptor {
    /// The resource name of the metric descriptor.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The metric type, including its DNS name prefix. The type is not
    /// URL-encoded. All user-defined metric types have the DNS name
    /// `custom.googleapis.com` or `external.googleapis.com`. Metric types should
    /// use a natural hierarchical grouping. For example:
    ///
    ///      "custom.googleapis.com/invoice/paid/amount"
    ///      "external.googleapis.com/prometheus/up"
    ///      "appengine.googleapis.com/http/server/response_latencies"
    #[prost(string, tag = "8")]
    pub r#type: ::prost::alloc::string::String,
    /// The set of labels that can be used to describe a specific
    /// instance of this metric type. For example, the
    /// `appengine.googleapis.com/http/server/response_latencies` metric
    /// type has a label for the HTTP response code, `response_code`, so
    /// you can look at latencies for successful responses or just
    /// for responses that failed.
    #[prost(message, repeated, tag = "2")]
    pub labels: ::prost::alloc::vec::Vec<LabelDescriptor>,
    /// Whether the metric records instantaneous values, changes to a value, etc.
    /// Some combinations of `metric_kind` and `value_type` might not be supported.
    #[prost(enumeration = "metric_descriptor::MetricKind", tag = "3")]
    pub metric_kind: i32,
    /// Whether the measurement is an integer, a floating-point number, etc.
    /// Some combinations of `metric_kind` and `value_type` might not be supported.
    #[prost(enumeration = "metric_descriptor::ValueType", tag = "4")]
    pub value_type: i32,
    /// The units in which the metric value is reported. It is only applicable
    /// if the `value_type` is `INT64`, `DOUBLE`, or `DISTRIBUTION`. The `unit`
    /// defines the representation of the stored metric values.
    ///
    /// Different systems might scale the values to be more easily displayed (so a
    /// value of `0.02kBy` _might_ be displayed as `20By`, and a value of
    /// `3523kBy` _might_ be displayed as `3.5MBy`). However, if the `unit` is
    /// `kBy`, then the value of the metric is always in thousands of bytes, no
    /// matter how it might be displayed.
    ///
    /// If you want a custom metric to record the exact number of CPU-seconds used
    /// by a job, you can create an `INT64 CUMULATIVE` metric whose `unit` is
    /// `s{CPU}` (or equivalently `1s{CPU}` or just `s`). If the job uses 12,005
    /// CPU-seconds, then the value is written as `12005`.
    ///
    /// Alternatively, if you want a custom metric to record data in a more
    /// granular way, you can create a `DOUBLE CUMULATIVE` metric whose `unit` is
    /// `ks{CPU}`, and then write the value `12.005` (which is `12005/1000`),
    /// or use `Kis{CPU}` and write `11.723` (which is `12005/1024`).
    ///
    /// The supported units are a subset of [The Unified Code for Units of
    /// Measure](<https://unitsofmeasure.org/ucum.html>) standard:
    ///
    /// **Basic units (UNIT)**
    ///
    /// * `bit`   bit
    /// * `By`    byte
    /// * `s`     second
    /// * `min`   minute
    /// * `h`     hour
    /// * `d`     day
    /// * `1`     dimensionless
    ///
    /// **Prefixes (PREFIX)**
    ///
    /// * `k`     kilo    (10^3)
    /// * `M`     mega    (10^6)
    /// * `G`     giga    (10^9)
    /// * `T`     tera    (10^12)
    /// * `P`     peta    (10^15)
    /// * `E`     exa     (10^18)
    /// * `Z`     zetta   (10^21)
    /// * `Y`     yotta   (10^24)
    ///
    /// * `m`     milli   (10^-3)
    /// * `u`     micro   (10^-6)
    /// * `n`     nano    (10^-9)
    /// * `p`     pico    (10^-12)
    /// * `f`     femto   (10^-15)
    /// * `a`     atto    (10^-18)
    /// * `z`     zepto   (10^-21)
    /// * `y`     yocto   (10^-24)
    ///
    /// * `Ki`    kibi    (2^10)
    /// * `Mi`    mebi    (2^20)
    /// * `Gi`    gibi    (2^30)
    /// * `Ti`    tebi    (2^40)
    /// * `Pi`    pebi    (2^50)
    ///
    /// **Grammar**
    ///
    /// The grammar also includes these connectors:
    ///
    /// * `/`    division or ratio (as an infix operator). For examples,
    ///           `kBy/{email}` or `MiBy/10ms` (although you should almost never
    ///           have `/s` in a metric `unit`; rates should always be computed at
    ///           query time from the underlying cumulative or delta value).
    /// * `.`    multiplication or composition (as an infix operator). For
    ///           examples, `GBy.d` or `k{watt}.h`.
    ///
    /// The grammar for a unit is as follows:
    ///
    ///      Expression = Component { "." Component } { "/" Component } ;
    ///
    ///      Component = ( \[ PREFIX \] UNIT | "%" ) \[ Annotation \]
    ///                | Annotation
    ///                | "1"
    ///                ;
    ///
    ///      Annotation = "{" NAME "}" ;
    ///
    /// Notes:
    ///
    /// * `Annotation` is just a comment if it follows a `UNIT`. If the annotation
    ///     is used alone, then the unit is equivalent to `1`. For examples,
    ///     `{request}/s == 1/s`, `By{transmitted}/s == By/s`.
    /// * `NAME` is a sequence of non-blank printable ASCII characters not
    ///     containing `{` or `}`.
    /// * `1` represents a unitary [dimensionless
    ///     unit](<https://en.wikipedia.org/wiki/Dimensionless_quantity>) of 1, such
    ///     as in `1/s`. It is typically used when none of the basic units are
    ///     appropriate. For example, "new users per day" can be represented as
    ///     `1/d` or `{new-users}/d` (and a metric value `5` would mean "5 new
    ///     users). Alternatively, "thousands of page views per day" would be
    ///     represented as `1000/d` or `k1/d` or `k{page_views}/d` (and a metric
    ///     value of `5.3` would mean "5300 page views per day").
    /// * `%` represents dimensionless value of 1/100, and annotates values giving
    ///     a percentage (so the metric values are typically in the range of 0..100,
    ///     and a metric value `3` means "3 percent").
    /// * `10^2.%` indicates a metric contains a ratio, typically in the range
    ///     0..1, that will be multiplied by 100 and displayed as a percentage
    ///     (so a metric value `0.03` means "3 percent").
    #[prost(string, tag = "5")]
    pub unit: ::prost::alloc::string::String,
    /// A detailed description of the metric, which can be used in documentation.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// A concise name for the metric, which can be displayed in user interfaces.
    /// Use sentence case without an ending period, for example "Request count".
    /// This field is optional but it is recommended to be set for any metrics
    /// associated with user-visible concepts, such as Quota.
    #[prost(string, tag = "7")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Metadata which can be used to guide usage of the metric.
    #[prost(message, optional, tag = "10")]
    pub metadata: ::core::option::Option<metric_descriptor::MetricDescriptorMetadata>,
    /// Optional. The launch stage of the metric definition.
    #[prost(enumeration = "LaunchStage", tag = "12")]
    pub launch_stage: i32,
    /// Read-only. If present, then a [time
    /// series][google.monitoring.v3.TimeSeries], which is identified partially by
    /// a metric type and a
    /// [MonitoredResourceDescriptor][google.api.MonitoredResourceDescriptor], that
    /// is associated with this metric type can only be associated with one of the
    /// monitored resource types listed here.
    #[prost(string, repeated, tag = "13")]
    pub monitored_resource_types: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `MetricDescriptor`.
pub mod metric_descriptor {
    /// Additional annotations that can be used to guide the usage of a metric.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetricDescriptorMetadata {
        /// Deprecated. Must use the
        /// [MetricDescriptor.launch_stage][google.api.MetricDescriptor.launch_stage]
        /// instead.
        #[deprecated]
        #[prost(enumeration = "super::LaunchStage", tag = "1")]
        pub launch_stage: i32,
        /// The sampling period of metric data points. For metrics which are written
        /// periodically, consecutive data points are stored at this time interval,
        /// excluding data loss due to errors. Metrics with a higher granularity have
        /// a smaller sampling period.
        #[prost(message, optional, tag = "2")]
        pub sample_period: ::core::option::Option<::prost_types::Duration>,
        /// The delay of data points caused by ingestion. Data points older than this
        /// age are guaranteed to be ingested and available to be read, excluding
        /// data loss due to errors.
        #[prost(message, optional, tag = "3")]
        pub ingest_delay: ::core::option::Option<::prost_types::Duration>,
    }
    /// The kind of measurement. It describes how the data is reported.
    /// For information on setting the start time and end time based on
    /// the MetricKind, see [TimeInterval][google.monitoring.v3.TimeInterval].
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum MetricKind {
        /// Do not use this default value.
        Unspecified = 0,
        /// An instantaneous measurement of a value.
        Gauge = 1,
        /// The change in a value during a time interval.
        Delta = 2,
        /// A value accumulated over a time interval.  Cumulative
        /// measurements in a time series should have the same start time
        /// and increasing end times, until an event resets the cumulative
        /// value to zero and sets a new start time for the following
        /// points.
        Cumulative = 3,
    }
    impl MetricKind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MetricKind::Unspecified => "METRIC_KIND_UNSPECIFIED",
                MetricKind::Gauge => "GAUGE",
                MetricKind::Delta => "DELTA",
                MetricKind::Cumulative => "CUMULATIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "METRIC_KIND_UNSPECIFIED" => Some(Self::Unspecified),
                "GAUGE" => Some(Self::Gauge),
                "DELTA" => Some(Self::Delta),
                "CUMULATIVE" => Some(Self::Cumulative),
                _ => None,
            }
        }
    }
    /// The value type of a metric.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ValueType {
        /// Do not use this default value.
        Unspecified = 0,
        /// The value is a boolean.
        /// This value type can be used only if the metric kind is `GAUGE`.
        Bool = 1,
        /// The value is a signed 64-bit integer.
        Int64 = 2,
        /// The value is a double precision floating point number.
        Double = 3,
        /// The value is a text string.
        /// This value type can be used only if the metric kind is `GAUGE`.
        String = 4,
        /// The value is a [`Distribution`][google.api.Distribution].
        Distribution = 5,
        /// The value is money.
        Money = 6,
    }
    impl ValueType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueType::Unspecified => "VALUE_TYPE_UNSPECIFIED",
                ValueType::Bool => "BOOL",
                ValueType::Int64 => "INT64",
                ValueType::Double => "DOUBLE",
                ValueType::String => "STRING",
                ValueType::Distribution => "DISTRIBUTION",
                ValueType::Money => "MONEY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VALUE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BOOL" => Some(Self::Bool),
                "INT64" => Some(Self::Int64),
                "DOUBLE" => Some(Self::Double),
                "STRING" => Some(Self::String),
                "DISTRIBUTION" => Some(Self::Distribution),
                "MONEY" => Some(Self::Money),
                _ => None,
            }
        }
    }
}
/// A specific metric, identified by specifying values for all of the
/// labels of a [`MetricDescriptor`][google.api.MetricDescriptor].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metric {
    /// An existing metric type, see
    /// [google.api.MetricDescriptor][google.api.MetricDescriptor]. For example,
    /// `custom.googleapis.com/invoice/paid/amount`.
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    /// The set of label values that uniquely identify this metric. All
    /// labels listed in the `MetricDescriptor` must be assigned values.
    #[prost(btree_map = "string, string", tag = "2")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Output generated from semantically comparing two versions of a service
/// configuration.
///
/// Includes detailed information about a field that have changed with
/// applicable advice about potential consequences for the change, such as
/// backwards-incompatibility.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigChange {
    /// Object hierarchy path to the change, with levels separated by a '.'
    /// character. For repeated fields, an applicable unique identifier field is
    /// used for the index (usually selector, name, or id). For maps, the term
    /// 'key' is used. If the field has no unique identifier, the numeric index
    /// is used.
    /// Examples:
    /// - visibility.rules\[selector=="google.LibraryService.ListBooks"\].restriction
    /// - quota.metric_rules\[selector=="google"\].metric_costs\[key=="reads"\].value
    /// - logging.producer_destinations\[0\]
    #[prost(string, tag = "1")]
    pub element: ::prost::alloc::string::String,
    /// Value of the changed object in the old Service configuration,
    /// in JSON format. This field will not be populated if ChangeType == ADDED.
    #[prost(string, tag = "2")]
    pub old_value: ::prost::alloc::string::String,
    /// Value of the changed object in the new Service configuration,
    /// in JSON format. This field will not be populated if ChangeType == REMOVED.
    #[prost(string, tag = "3")]
    pub new_value: ::prost::alloc::string::String,
    /// The type for this change, either ADDED, REMOVED, or MODIFIED.
    #[prost(enumeration = "ChangeType", tag = "4")]
    pub change_type: i32,
    /// Collection of advice provided for this change, useful for determining the
    /// possible impact of this change.
    #[prost(message, repeated, tag = "5")]
    pub advices: ::prost::alloc::vec::Vec<Advice>,
}
/// Generated advice about this change, used for providing more
/// information about how a change will affect the existing service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Advice {
    /// Useful description for why this advice was applied and what actions should
    /// be taken to mitigate any implied risks.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
/// Classifies set of possible modifications to an object in the service
/// configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChangeType {
    /// No value was provided.
    Unspecified = 0,
    /// The changed object exists in the 'new' service configuration, but not
    /// in the 'old' service configuration.
    Added = 1,
    /// The changed object exists in the 'old' service configuration, but not
    /// in the 'new' service configuration.
    Removed = 2,
    /// The changed object exists in both service configurations, but its value
    /// is different.
    Modified = 3,
}
impl ChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChangeType::Unspecified => "CHANGE_TYPE_UNSPECIFIED",
            ChangeType::Added => "ADDED",
            ChangeType::Removed => "REMOVED",
            ChangeType::Modified => "MODIFIED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ADDED" => Some(Self::Added),
            "REMOVED" => Some(Self::Removed),
            "MODIFIED" => Some(Self::Modified),
            _ => None,
        }
    }
}
/// `Context` defines which contexts an API requests.
///
/// Example:
///
///      context:
///        rules:
///        - selector: "*"
///          requested:
///          - google.rpc.context.ProjectContext
///          - google.rpc.context.OriginContext
///
/// The above specifies that all methods in the API request
/// `google.rpc.context.ProjectContext` and
/// `google.rpc.context.OriginContext`.
///
/// Available context types are defined in package
/// `google.rpc.context`.
///
/// This also provides mechanism to allowlist any protobuf message extension that
/// can be sent in grpc metadata using “x-goog-ext-<extension_id>-bin” and
/// “x-goog-ext-<extension_id>-jspb” format. For example, list any service
/// specific protobuf types that can appear in grpc metadata as follows in your
/// yaml file:
///
/// Example:
///
///      context:
///        rules:
///         - selector: "google.example.library.v1.LibraryService.CreateBook"
///           allowed_request_extensions:
///           - google.foo.v1.NewExtension
///           allowed_response_extensions:
///           - google.foo.v1.NewExtension
///
/// You can also specify extension ID instead of fully qualified extension name
/// here.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Context {
    /// A list of RPC context rules that apply to individual API methods.
    ///
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<ContextRule>,
}
/// A context rule provides information about the context for an individual API
/// element.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextRule {
    /// Selects the methods to which this rule applies.
    ///
    /// Refer to [selector][google.api.DocumentationRule.selector] for syntax
    /// details.
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    /// A list of full type names of requested contexts.
    #[prost(string, repeated, tag = "2")]
    pub requested: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of full type names of provided contexts.
    #[prost(string, repeated, tag = "3")]
    pub provided: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of full type names or extension IDs of extensions allowed in grpc
    /// side channel from client to backend.
    #[prost(string, repeated, tag = "4")]
    pub allowed_request_extensions: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// A list of full type names or extension IDs of extensions allowed in grpc
    /// side channel from backend to client.
    #[prost(string, repeated, tag = "5")]
    pub allowed_response_extensions: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// `Authentication` defines the authentication configuration for API methods
/// provided by an API service.
///
/// Example:
///
///      name: calendar.googleapis.com
///      authentication:
///        providers:
///        - id: google_calendar_auth
///          jwks_uri: <https://www.googleapis.com/oauth2/v1/certs>
///          issuer: <https://securetoken.google.com>
///        rules:
///        - selector: "*"
///          requirements:
///            provider_id: google_calendar_auth
///        - selector: google.calendar.Delegate
///          oauth:
///            canonical_scopes: <https://www.googleapis.com/auth/calendar.read>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authentication {
    /// A list of authentication rules that apply to individual API methods.
    ///
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    #[prost(message, repeated, tag = "3")]
    pub rules: ::prost::alloc::vec::Vec<AuthenticationRule>,
    /// Defines a set of authentication providers that a service supports.
    #[prost(message, repeated, tag = "4")]
    pub providers: ::prost::alloc::vec::Vec<AuthProvider>,
}
/// Authentication rules for the service.
///
/// By default, if a method has any authentication requirements, every request
/// must include a valid credential matching one of the requirements.
/// It's an error to include more than one kind of credential in a single
/// request.
///
/// If a method doesn't have any auth requirements, request credentials will be
/// ignored.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationRule {
    /// Selects the methods to which this rule applies.
    ///
    /// Refer to [selector][google.api.DocumentationRule.selector] for syntax
    /// details.
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    /// The requirements for OAuth credentials.
    #[prost(message, optional, tag = "2")]
    pub oauth: ::core::option::Option<OAuthRequirements>,
    /// If true, the service accepts API keys without any other credential.
    /// This flag only applies to HTTP and gRPC requests.
    #[prost(bool, tag = "5")]
    pub allow_without_credential: bool,
    /// Requirements for additional authentication providers.
    #[prost(message, repeated, tag = "7")]
    pub requirements: ::prost::alloc::vec::Vec<AuthRequirement>,
}
/// Specifies a location to extract JWT from an API request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtLocation {
    /// The value prefix. The value format is "value_prefix{token}"
    /// Only applies to "in" header type. Must be empty for "in" query type.
    /// If not empty, the header value has to match (case sensitive) this prefix.
    /// If not matched, JWT will not be extracted. If matched, JWT will be
    /// extracted after the prefix is removed.
    ///
    /// For example, for "Authorization: Bearer {JWT}",
    /// value_prefix="Bearer " with a space at the end.
    #[prost(string, tag = "3")]
    pub value_prefix: ::prost::alloc::string::String,
    #[prost(oneof = "jwt_location::In", tags = "1, 2, 4")]
    pub r#in: ::core::option::Option<jwt_location::In>,
}
/// Nested message and enum types in `JwtLocation`.
pub mod jwt_location {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum In {
        /// Specifies HTTP header name to extract JWT token.
        #[prost(string, tag = "1")]
        Header(::prost::alloc::string::String),
        /// Specifies URL query parameter name to extract JWT token.
        #[prost(string, tag = "2")]
        Query(::prost::alloc::string::String),
        /// Specifies cookie name to extract JWT token.
        #[prost(string, tag = "4")]
        Cookie(::prost::alloc::string::String),
    }
}
/// Configuration for an authentication provider, including support for
/// [JSON Web Token
/// (JWT)](<https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthProvider {
    /// The unique identifier of the auth provider. It will be referred to by
    /// `AuthRequirement.provider_id`.
    ///
    /// Example: "bookstore_auth".
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Identifies the principal that issued the JWT. See
    /// <https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.1>
    /// Usually a URL or an email address.
    ///
    /// Example: <https://securetoken.google.com>
    /// Example: 1234567-compute@developer.gserviceaccount.com
    #[prost(string, tag = "2")]
    pub issuer: ::prost::alloc::string::String,
    /// URL of the provider's public key set to validate signature of the JWT. See
    /// [OpenID
    /// Discovery](<https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata>).
    /// Optional if the key set document:
    ///   - can be retrieved from
    ///     [OpenID
    ///     Discovery](<https://openid.net/specs/openid-connect-discovery-1_0.html>)
    ///     of the issuer.
    ///   - can be inferred from the email domain of the issuer (e.g. a Google
    ///   service account).
    ///
    /// Example: <https://www.googleapis.com/oauth2/v1/certs>
    #[prost(string, tag = "3")]
    pub jwks_uri: ::prost::alloc::string::String,
    /// The list of JWT
    /// [audiences](<https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3>).
    /// that are allowed to access. A JWT containing any of these audiences will
    /// be accepted. When this setting is absent, JWTs with audiences:
    ///    - "<https://\[service.name\]/[google.protobuf.Api.name]">
    ///    - "<https://\[service.name\]/">
    /// will be accepted.
    /// For example, if no audiences are in the setting, LibraryService API will
    /// accept JWTs with the following audiences:
    ///    -
    ///    <https://library-example.googleapis.com/google.example.library.v1.LibraryService>
    ///    - <https://library-example.googleapis.com/>
    ///
    /// Example:
    ///
    ///      audiences: bookstore_android.apps.googleusercontent.com,
    ///                 bookstore_web.apps.googleusercontent.com
    #[prost(string, tag = "4")]
    pub audiences: ::prost::alloc::string::String,
    /// Redirect URL if JWT token is required but not present or is expired.
    /// Implement authorizationUrl of securityDefinitions in OpenAPI spec.
    #[prost(string, tag = "5")]
    pub authorization_url: ::prost::alloc::string::String,
    /// Defines the locations to extract the JWT.  For now it is only used by the
    /// Cloud Endpoints to store the OpenAPI extension \[x-google-jwt-locations\]
    /// (<https://cloud.google.com/endpoints/docs/openapi/openapi-extensions#x-google-jwt-locations>)
    ///
    /// JWT locations can be one of HTTP headers, URL query parameters or
    /// cookies. The rule is that the first match wins.
    ///
    /// If not specified,  default to use following 3 locations:
    ///     1) Authorization: Bearer
    ///     2) x-goog-iap-jwt-assertion
    ///     3) access_token query parameter
    ///
    /// Default locations can be specified as followings:
    ///     jwt_locations:
    ///     - header: Authorization
    ///       value_prefix: "Bearer "
    ///     - header: x-goog-iap-jwt-assertion
    ///     - query: access_token
    #[prost(message, repeated, tag = "6")]
    pub jwt_locations: ::prost::alloc::vec::Vec<JwtLocation>,
}
/// OAuth scopes are a way to define data and permissions on data. For example,
/// there are scopes defined for "Read-only access to Google Calendar" and
/// "Access to Cloud Platform". Users can consent to a scope for an application,
/// giving it permission to access that data on their behalf.
///
/// OAuth scope specifications should be fairly coarse grained; a user will need
/// to see and understand the text description of what your scope means.
///
/// In most cases: use one or at most two OAuth scopes for an entire family of
/// products. If your product has multiple APIs, you should probably be sharing
/// the OAuth scope across all of those APIs.
///
/// When you need finer grained OAuth consent screens: talk with your product
/// management about how developers will use them in practice.
///
/// Please note that even though each of the canonical scopes is enough for a
/// request to be accepted and passed to the backend, a request can still fail
/// due to the backend requiring additional scopes or permissions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuthRequirements {
    /// The list of publicly documented OAuth scopes that are allowed access. An
    /// OAuth token containing any of these scopes will be accepted.
    ///
    /// Example:
    ///
    ///       canonical_scopes: <https://www.googleapis.com/auth/calendar,>
    ///                         <https://www.googleapis.com/auth/calendar.read>
    #[prost(string, tag = "1")]
    pub canonical_scopes: ::prost::alloc::string::String,
}
/// User-defined authentication requirements, including support for
/// [JSON Web Token
/// (JWT)](<https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequirement {
    /// [id][google.api.AuthProvider.id] from authentication provider.
    ///
    /// Example:
    ///
    ///      provider_id: bookstore_auth
    #[prost(string, tag = "1")]
    pub provider_id: ::prost::alloc::string::String,
    /// NOTE: This will be deprecated soon, once AuthProvider.audiences is
    /// implemented and accepted in all the runtime components.
    ///
    /// The list of JWT
    /// [audiences](<https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3>).
    /// that are allowed to access. A JWT containing any of these audiences will
    /// be accepted. When this setting is absent, only JWTs with audience
    /// "<https://[Service_name][google.api.Service.name]/[API_name][google.protobuf.Api.name]">
    /// will be accepted. For example, if no audiences are in the setting,
    /// LibraryService API will only accept JWTs with the following audience
    /// "<https://library-example.googleapis.com/google.example.library.v1.LibraryService".>
    ///
    /// Example:
    ///
    ///      audiences: bookstore_android.apps.googleusercontent.com,
    ///                 bookstore_web.apps.googleusercontent.com
    #[prost(string, tag = "2")]
    pub audiences: ::prost::alloc::string::String,
}
/// `Backend` defines the backend configuration for a service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backend {
    /// A list of API backend rules that apply to individual API methods.
    ///
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<BackendRule>,
}
/// A backend rule provides configuration for an individual API element.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackendRule {
    /// Selects the methods to which this rule applies.
    ///
    /// Refer to [selector][google.api.DocumentationRule.selector] for syntax
    /// details.
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    /// The address of the API backend.
    ///
    /// The scheme is used to determine the backend protocol and security.
    /// The following schemes are accepted:
    ///
    ///     SCHEME        PROTOCOL    SECURITY
    ///     http://       HTTP        None
    ///     https://      HTTP        TLS
    ///     grpc://       gRPC        None
    ///     grpcs://      gRPC        TLS
    ///
    /// It is recommended to explicitly include a scheme. Leaving out the scheme
    /// may cause constrasting behaviors across platforms.
    ///
    /// If the port is unspecified, the default is:
    /// - 80 for schemes without TLS
    /// - 443 for schemes with TLS
    ///
    /// For HTTP backends, use [protocol][google.api.BackendRule.protocol]
    /// to specify the protocol version.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// The number of seconds to wait for a response from a request. The default
    /// varies based on the request protocol and deployment environment.
    #[prost(double, tag = "3")]
    pub deadline: f64,
    /// Deprecated, do not use.
    #[deprecated]
    #[prost(double, tag = "4")]
    pub min_deadline: f64,
    /// The number of seconds to wait for the completion of a long running
    /// operation. The default is no deadline.
    #[prost(double, tag = "5")]
    pub operation_deadline: f64,
    #[prost(enumeration = "backend_rule::PathTranslation", tag = "6")]
    pub path_translation: i32,
    /// The protocol used for sending a request to the backend.
    /// The supported values are "http/1.1" and "h2".
    ///
    /// The default value is inferred from the scheme in the
    /// [address][google.api.BackendRule.address] field:
    ///
    ///     SCHEME        PROTOCOL
    ///     http://       http/1.1
    ///     https://      http/1.1
    ///     grpc://       h2
    ///     grpcs://      h2
    ///
    /// For secure HTTP backends (https://) that support HTTP/2, set this field
    /// to "h2" for improved performance.
    ///
    /// Configuring this field to non-default values is only supported for secure
    /// HTTP backends. This field will be ignored for all other backends.
    ///
    /// See
    /// <https://www.iana.org/assignments/tls-extensiontype-values/tls-extensiontype-values.xhtml#alpn-protocol-ids>
    /// for more details on the supported values.
    #[prost(string, tag = "9")]
    pub protocol: ::prost::alloc::string::String,
    /// The map between request protocol and the backend address.
    #[prost(btree_map = "string, message", tag = "10")]
    pub overrides_by_request_protocol: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        BackendRule,
    >,
    /// Authentication settings used by the backend.
    ///
    /// These are typically used to provide service management functionality to
    /// a backend served on a publicly-routable URL. The `authentication`
    /// details should match the authentication behavior used by the backend.
    ///
    /// For example, specifying `jwt_audience` implies that the backend expects
    /// authentication via a JWT.
    ///
    /// When authentication is unspecified, the resulting behavior is the same
    /// as `disable_auth` set to `true`.
    ///
    /// Refer to <https://developers.google.com/identity/protocols/OpenIDConnect> for
    /// JWT ID token.
    #[prost(oneof = "backend_rule::Authentication", tags = "7, 8")]
    pub authentication: ::core::option::Option<backend_rule::Authentication>,
}
/// Nested message and enum types in `BackendRule`.
pub mod backend_rule {
    /// Path Translation specifies how to combine the backend address with the
    /// request path in order to produce the appropriate forwarding URL for the
    /// request.
    ///
    /// Path Translation is applicable only to HTTP-based backends. Backends which
    /// do not accept requests over HTTP/HTTPS should leave `path_translation`
    /// unspecified.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PathTranslation {
        Unspecified = 0,
        /// Use the backend address as-is, with no modification to the path. If the
        /// URL pattern contains variables, the variable names and values will be
        /// appended to the query string. If a query string parameter and a URL
        /// pattern variable have the same name, this may result in duplicate keys in
        /// the query string.
        ///
        /// # Examples
        ///
        /// Given the following operation config:
        ///
        ///      Method path:        /api/company/{cid}/user/{uid}
        ///      Backend address:    <https://example.cloudfunctions.net/getUser>
        ///
        /// Requests to the following request paths will call the backend at the
        /// translated path:
        ///
        ///      Request path: /api/company/widgetworks/user/johndoe
        ///      Translated:
        ///      <https://example.cloudfunctions.net/getUser?cid=widgetworks&uid=johndoe>
        ///
        ///      Request path: /api/company/widgetworks/user/johndoe?timezone=EST
        ///      Translated:
        ///      <https://example.cloudfunctions.net/getUser?timezone=EST&cid=widgetworks&uid=johndoe>
        ConstantAddress = 1,
        /// The request path will be appended to the backend address.
        ///
        /// # Examples
        ///
        /// Given the following operation config:
        ///
        ///      Method path:        /api/company/{cid}/user/{uid}
        ///      Backend address:    <https://example.appspot.com>
        ///
        /// Requests to the following request paths will call the backend at the
        /// translated path:
        ///
        ///      Request path: /api/company/widgetworks/user/johndoe
        ///      Translated:
        ///      <https://example.appspot.com/api/company/widgetworks/user/johndoe>
        ///
        ///      Request path: /api/company/widgetworks/user/johndoe?timezone=EST
        ///      Translated:
        ///      <https://example.appspot.com/api/company/widgetworks/user/johndoe?timezone=EST>
        AppendPathToAddress = 2,
    }
    impl PathTranslation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PathTranslation::Unspecified => "PATH_TRANSLATION_UNSPECIFIED",
                PathTranslation::ConstantAddress => "CONSTANT_ADDRESS",
                PathTranslation::AppendPathToAddress => "APPEND_PATH_TO_ADDRESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PATH_TRANSLATION_UNSPECIFIED" => Some(Self::Unspecified),
                "CONSTANT_ADDRESS" => Some(Self::ConstantAddress),
                "APPEND_PATH_TO_ADDRESS" => Some(Self::AppendPathToAddress),
                _ => None,
            }
        }
    }
    /// Authentication settings used by the backend.
    ///
    /// These are typically used to provide service management functionality to
    /// a backend served on a publicly-routable URL. The `authentication`
    /// details should match the authentication behavior used by the backend.
    ///
    /// For example, specifying `jwt_audience` implies that the backend expects
    /// authentication via a JWT.
    ///
    /// When authentication is unspecified, the resulting behavior is the same
    /// as `disable_auth` set to `true`.
    ///
    /// Refer to <https://developers.google.com/identity/protocols/OpenIDConnect> for
    /// JWT ID token.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Authentication {
        /// The JWT audience is used when generating a JWT ID token for the backend.
        /// This ID token will be added in the HTTP "authorization" header, and sent
        /// to the backend.
        #[prost(string, tag = "7")]
        JwtAudience(::prost::alloc::string::String),
        /// When disable_auth is true, a JWT ID token won't be generated and the
        /// original "Authorization" HTTP header will be preserved. If the header is
        /// used to carry the original token and is expected by the backend, this
        /// field must be set to true to preserve the header.
        #[prost(bool, tag = "8")]
        DisableAuth(bool),
    }
}
/// Billing related configuration of the service.
///
/// The following example shows how to configure monitored resources and metrics
/// for billing, `consumer_destinations` is the only supported destination and
/// the monitored resources need at least one label key
/// `cloud.googleapis.com/location` to indicate the location of the billing
/// usage, using different monitored resources between monitoring and billing is
/// recommended so they can be evolved independently:
///
///
///      monitored_resources:
///      - type: library.googleapis.com/billing_branch
///        labels:
///        - key: cloud.googleapis.com/location
///          description: |
///            Predefined label to support billing location restriction.
///        - key: city
///          description: |
///            Custom label to define the city where the library branch is located
///            in.
///        - key: name
///          description: Custom label to define the name of the library branch.
///      metrics:
///      - name: library.googleapis.com/book/borrowed_count
///        metric_kind: DELTA
///        value_type: INT64
///        unit: "1"
///      billing:
///        consumer_destinations:
///        - monitored_resource: library.googleapis.com/billing_branch
///          metrics:
///          - library.googleapis.com/book/borrowed_count
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Billing {
    /// Billing configurations for sending metrics to the consumer project.
    /// There can be multiple consumer destinations per service, each one must have
    /// a different monitored resource type. A metric can be used in at most
    /// one consumer destination.
    #[prost(message, repeated, tag = "8")]
    pub consumer_destinations: ::prost::alloc::vec::Vec<billing::BillingDestination>,
}
/// Nested message and enum types in `Billing`.
pub mod billing {
    /// Configuration of a specific billing destination (Currently only support
    /// bill against consumer project).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BillingDestination {
        /// The monitored resource type. The type must be defined in
        /// [Service.monitored_resources][google.api.Service.monitored_resources]
        /// section.
        #[prost(string, tag = "1")]
        pub monitored_resource: ::prost::alloc::string::String,
        /// Names of the metrics to report to this billing destination.
        /// Each name must be defined in
        /// [Service.metrics][google.api.Service.metrics] section.
        #[prost(string, repeated, tag = "2")]
        pub metrics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Google API Policy Annotation
///
/// This message defines a simple API policy annotation that can be used to
/// annotate API request and response message fields with applicable policies.
/// One field may have multiple applicable policies that must all be satisfied
/// before a request can be processed. This policy annotation is used to
/// generate the overall policy that will be used for automatic runtime
/// policy enforcement and documentation generation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldPolicy {
    /// Selects one or more request or response message fields to apply this
    /// `FieldPolicy`.
    ///
    /// When a `FieldPolicy` is used in proto annotation, the selector must
    /// be left as empty. The service config generator will automatically fill
    /// the correct value.
    ///
    /// When a `FieldPolicy` is used in service config, the selector must be a
    /// comma-separated string with valid request or response field paths,
    /// such as "foo.bar" or "foo.bar,foo.baz".
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    /// Specifies the required permission(s) for the resource referred to by the
    /// field. It requires the field contains a valid resource reference, and
    /// the request must pass the permission checks to proceed. For example,
    /// "resourcemanager.projects.get".
    #[prost(string, tag = "2")]
    pub resource_permission: ::prost::alloc::string::String,
    /// Specifies the resource type for the resource referred to by the field.
    #[prost(string, tag = "3")]
    pub resource_type: ::prost::alloc::string::String,
}
/// Defines policies applying to an RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodPolicy {
    /// Selects a method to which these policies should be enforced, for example,
    /// "google.pubsub.v1.Subscriber.CreateSubscription".
    ///
    /// Refer to [selector][google.api.DocumentationRule.selector] for syntax
    /// details.
    ///
    /// NOTE: This field must not be set in the proto annotation. It will be
    /// automatically filled by the service config compiler .
    #[prost(string, tag = "9")]
    pub selector: ::prost::alloc::string::String,
    /// Policies that are applicable to the request message.
    #[prost(message, repeated, tag = "2")]
    pub request_policies: ::prost::alloc::vec::Vec<FieldPolicy>,
}
/// Selects and configures the service controller used by the service.
///
/// Example:
///
///      control:
///        environment: servicecontrol.googleapis.com
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Control {
    /// The service controller environment to use. If empty, no control plane
    /// feature (like quota and billing) will be enabled. The recommended value for
    /// most services is servicecontrol.googleapis.com
    #[prost(string, tag = "1")]
    pub environment: ::prost::alloc::string::String,
    /// Defines policies applying to the API methods of the service.
    #[prost(message, repeated, tag = "4")]
    pub method_policies: ::prost::alloc::vec::Vec<MethodPolicy>,
}
/// `Documentation` provides the information for describing a service.
///
/// Example:
/// <pre><code>documentation:
///    summary: >
///      The Google Calendar API gives access
///      to most calendar features.
///    pages:
///    - name: Overview
///      content: &#40;== include google/foo/overview.md ==&#41;
///    - name: Tutorial
///      content: &#40;== include google/foo/tutorial.md ==&#41;
///      subpages;
///      - name: Java
///        content: &#40;== include google/foo/tutorial_java.md ==&#41;
///    rules:
///    - selector: google.calendar.Calendar.Get
///      description: >
///        ...
///    - selector: google.calendar.Calendar.Put
///      description: >
///        ...
/// </code></pre>
/// Documentation is provided in markdown syntax. In addition to
/// standard markdown features, definition lists, tables and fenced
/// code blocks are supported. Section headers can be provided and are
/// interpreted relative to the section nesting of the context where
/// a documentation fragment is embedded.
///
/// Documentation from the IDL is merged with documentation defined
/// via the config at normalization time, where documentation provided
/// by config rules overrides IDL provided.
///
/// A number of constructs specific to the API platform are supported
/// in documentation text.
///
/// In order to reference a proto element, the following
/// notation can be used:
/// <pre><code>&#91;fully.qualified.proto.name]&#91;]</code></pre>
/// To override the display text used for the link, this can be used:
/// <pre><code>&#91;display text]&#91;fully.qualified.proto.name]</code></pre>
/// Text can be excluded from doc using the following notation:
/// <pre><code>&#40;-- internal comment --&#41;</code></pre>
///
/// A few directives are available in documentation. Note that
/// directives must appear on a single line to be properly
/// identified. The `include` directive includes a markdown file from
/// an external source:
/// <pre><code>&#40;== include path/to/file ==&#41;</code></pre>
/// The `resource_for` directive marks a message to be the resource of
/// a collection in REST view. If it is not specified, tools attempt
/// to infer the resource from the operations in a collection:
/// <pre><code>&#40;== resource_for v1.shelves.books ==&#41;</code></pre>
/// The directive `suppress_warning` does not directly affect documentation
/// and is documented together with service config validation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Documentation {
    /// A short description of what the service does. The summary must be plain
    /// text. It becomes the overview of the service displayed in Google Cloud
    /// Console.
    /// NOTE: This field is equivalent to the standard field `description`.
    #[prost(string, tag = "1")]
    pub summary: ::prost::alloc::string::String,
    /// The top level pages for the documentation set.
    #[prost(message, repeated, tag = "5")]
    pub pages: ::prost::alloc::vec::Vec<Page>,
    /// A list of documentation rules that apply to individual API elements.
    ///
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    #[prost(message, repeated, tag = "3")]
    pub rules: ::prost::alloc::vec::Vec<DocumentationRule>,
    /// The URL to the root of documentation.
    #[prost(string, tag = "4")]
    pub documentation_root_url: ::prost::alloc::string::String,
    /// Specifies the service root url if the default one (the service name
    /// from the yaml file) is not suitable. This can be seen in any fully
    /// specified service urls as well as sections that show a base that other
    /// urls are relative to.
    #[prost(string, tag = "6")]
    pub service_root_url: ::prost::alloc::string::String,
    /// Declares a single overview page. For example:
    /// <pre><code>documentation:
    ///    summary: ...
    ///    overview: &#40;== include overview.md ==&#41;
    /// </code></pre>
    /// This is a shortcut for the following declaration (using pages style):
    /// <pre><code>documentation:
    ///    summary: ...
    ///    pages:
    ///    - name: Overview
    ///      content: &#40;== include overview.md ==&#41;
    /// </code></pre>
    /// Note: you cannot specify both `overview` field and `pages` field.
    #[prost(string, tag = "2")]
    pub overview: ::prost::alloc::string::String,
}
/// A documentation rule provides information about individual API elements.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentationRule {
    /// The selector is a comma-separated list of patterns for any element such as
    /// a method, a field, an enum value. Each pattern is a qualified name of the
    /// element which may end in "*", indicating a wildcard. Wildcards are only
    /// allowed at the end and for a whole component of the qualified name,
    /// i.e. "foo.*" is ok, but not "foo.b*" or "foo.*.bar". A wildcard will match
    /// one or more components. To specify a default for all applicable elements,
    /// the whole pattern "*" is used.
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    /// Description of the selected proto element (e.g. a message, a method, a
    /// 'service' definition, or a field). Defaults to leading & trailing comments
    /// taken from the proto source definition of the proto element.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Deprecation description of the selected element(s). It can be provided if
    /// an element is marked as `deprecated`.
    #[prost(string, tag = "3")]
    pub deprecation_description: ::prost::alloc::string::String,
}
/// Represents a documentation page. A page can contain subpages to represent
/// nested documentation set structure.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Page {
    /// The name of the page. It will be used as an identity of the page to
    /// generate URI of the page, text of the link to this page in navigation,
    /// etc. The full page name (start from the root page name to this page
    /// concatenated with `.`) can be used as reference to the page in your
    /// documentation. For example:
    /// <pre><code>pages:
    /// - name: Tutorial
    ///    content: &#40;== include tutorial.md ==&#41;
    ///    subpages:
    ///    - name: Java
    ///      content: &#40;== include tutorial_java.md ==&#41;
    /// </code></pre>
    /// You can reference `Java` page using Markdown reference link syntax:
    /// `[Java][Tutorial.Java]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The Markdown content of the page. You can use <code>&#40;== include {path}
    /// ==&#41;</code> to include content from a Markdown file. The content can be
    /// used to produce the documentation page such as HTML format page.
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    /// Subpages of this page. The order of subpages specified here will be
    /// honored in the generated docset.
    #[prost(message, repeated, tag = "3")]
    pub subpages: ::prost::alloc::vec::Vec<Page>,
}
/// `Endpoint` describes a network address of a service that serves a set of
/// APIs. It is commonly known as a service endpoint. A service may expose
/// any number of service endpoints, and all service endpoints share the same
/// service definition, such as quota limits and monitoring metrics.
///
/// Example:
///
///      type: google.api.Service
///      name: library-example.googleapis.com
///      endpoints:
///        # Declares network address `<https://library-example.googleapis.com`>
///        # for service `library-example.googleapis.com`. The `https` scheme
///        # is implicit for all service endpoints. Other schemes may be
///        # supported in the future.
///      - name: library-example.googleapis.com
///        allow_cors: false
///      - name: content-staging-library-example.googleapis.com
///        # Allows HTTP OPTIONS calls to be passed to the API frontend, for it
///        # to decide whether the subsequent cross-origin request is allowed
///        # to proceed.
///        allow_cors: true
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    /// The canonical name of this endpoint.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Unimplemented. Dot not use.
    ///
    /// DEPRECATED: This field is no longer supported. Instead of using aliases,
    /// please specify multiple [google.api.Endpoint][google.api.Endpoint] for each
    /// of the intended aliases.
    ///
    /// Additional names that this endpoint will be hosted on.
    #[deprecated]
    #[prost(string, repeated, tag = "2")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The specification of an Internet routable address of API frontend that will
    /// handle requests to this [API
    /// Endpoint](<https://cloud.google.com/apis/design/glossary>). It should be
    /// either a valid IPv4 address or a fully-qualified domain name. For example,
    /// "8.8.8.8" or "myservice.appspot.com".
    #[prost(string, tag = "101")]
    pub target: ::prost::alloc::string::String,
    /// Allowing
    /// [CORS](<https://en.wikipedia.org/wiki/Cross-origin_resource_sharing>), aka
    /// cross-domain traffic, would allow the backends served from this endpoint to
    /// receive and respond to HTTP OPTIONS requests. The response will be used by
    /// the browser to determine whether the subsequent cross-origin request is
    /// allowed to proceed.
    #[prost(bool, tag = "5")]
    pub allow_cors: bool,
}
/// A description of a log type. Example in YAML format:
///
///      - name: library.googleapis.com/activity_history
///        description: The history of borrowing and returning library items.
///        display_name: Activity
///        labels:
///        - key: /customer_id
///          description: Identifier of a library customer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogDescriptor {
    /// The name of the log. It must be less than 512 characters long and can
    /// include the following characters: upper- and lower-case alphanumeric
    /// characters \[A-Za-z0-9\], and punctuation characters including
    /// slash, underscore, hyphen, period \[/_-.\].
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The set of labels that are available to describe a specific log entry.
    /// Runtime requests that contain labels not specified here are
    /// considered invalid.
    #[prost(message, repeated, tag = "2")]
    pub labels: ::prost::alloc::vec::Vec<LabelDescriptor>,
    /// A human-readable description of this log. This information appears in
    /// the documentation and can contain details.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The human-readable name for this log. This information appears on
    /// the user interface and should be concise.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
}
/// Logging configuration of the service.
///
/// The following example shows how to configure logs to be sent to the
/// producer and consumer projects. In the example, the `activity_history`
/// log is sent to both the producer and consumer projects, whereas the
/// `purchase_history` log is only sent to the producer project.
///
///      monitored_resources:
///      - type: library.googleapis.com/branch
///        labels:
///        - key: /city
///          description: The city where the library branch is located in.
///        - key: /name
///          description: The name of the branch.
///      logs:
///      - name: activity_history
///        labels:
///        - key: /customer_id
///      - name: purchase_history
///      logging:
///        producer_destinations:
///        - monitored_resource: library.googleapis.com/branch
///          logs:
///          - activity_history
///          - purchase_history
///        consumer_destinations:
///        - monitored_resource: library.googleapis.com/branch
///          logs:
///          - activity_history
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Logging {
    /// Logging configurations for sending logs to the producer project.
    /// There can be multiple producer destinations, each one must have a
    /// different monitored resource type. A log can be used in at most
    /// one producer destination.
    #[prost(message, repeated, tag = "1")]
    pub producer_destinations: ::prost::alloc::vec::Vec<logging::LoggingDestination>,
    /// Logging configurations for sending logs to the consumer project.
    /// There can be multiple consumer destinations, each one must have a
    /// different monitored resource type. A log can be used in at most
    /// one consumer destination.
    #[prost(message, repeated, tag = "2")]
    pub consumer_destinations: ::prost::alloc::vec::Vec<logging::LoggingDestination>,
}
/// Nested message and enum types in `Logging`.
pub mod logging {
    /// Configuration of a specific logging destination (the producer project
    /// or the consumer project).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LoggingDestination {
        /// The monitored resource type. The type must be defined in the
        /// [Service.monitored_resources][google.api.Service.monitored_resources]
        /// section.
        #[prost(string, tag = "3")]
        pub monitored_resource: ::prost::alloc::string::String,
        /// Names of the logs to be sent to this destination. Each name must
        /// be defined in the [Service.logs][google.api.Service.logs] section. If the
        /// log name is not a domain scoped name, it will be automatically prefixed
        /// with the service name followed by "/".
        #[prost(string, repeated, tag = "1")]
        pub logs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// An object that describes the schema of a
/// [MonitoredResource][google.api.MonitoredResource] object using a type name
/// and a set of labels.  For example, the monitored resource descriptor for
/// Google Compute Engine VM instances has a type of
/// `"gce_instance"` and specifies the use of the labels `"instance_id"` and
/// `"zone"` to identify particular VM instances.
///
/// Different APIs can support different monitored resource types. APIs generally
/// provide a `list` method that returns the monitored resource descriptors used
/// by the API.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoredResourceDescriptor {
    /// Optional. The resource name of the monitored resource descriptor:
    /// `"projects/{project_id}/monitoredResourceDescriptors/{type}"` where
    /// {type} is the value of the `type` field in this object and
    /// {project_id} is a project ID that provides API-specific context for
    /// accessing the type.  APIs that do not use project information can use the
    /// resource name format `"monitoredResourceDescriptors/{type}"`.
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// Required. The monitored resource type. For example, the type
    /// `"cloudsql_database"` represents databases in Google Cloud SQL.
    ///   For a list of types, see [Monitoring resource
    ///   types](<https://cloud.google.com/monitoring/api/resources>)
    /// and [Logging resource
    /// types](<https://cloud.google.com/logging/docs/api/v2/resource-list>).
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Optional. A concise name for the monitored resource type that might be
    /// displayed in user interfaces. It should be a Title Cased Noun Phrase,
    /// without any article or other determiners. For example,
    /// `"Google Cloud SQL Database"`.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. A detailed description of the monitored resource type that might
    /// be used in documentation.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Required. A set of labels used to describe instances of this monitored
    /// resource type. For example, an individual Google Cloud SQL database is
    /// identified by values for the labels `"database_id"` and `"zone"`.
    #[prost(message, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<LabelDescriptor>,
    /// Optional. The launch stage of the monitored resource definition.
    #[prost(enumeration = "LaunchStage", tag = "7")]
    pub launch_stage: i32,
}
/// An object representing a resource that can be used for monitoring, logging,
/// billing, or other purposes. Examples include virtual machine instances,
/// databases, and storage devices such as disks. The `type` field identifies a
/// [MonitoredResourceDescriptor][google.api.MonitoredResourceDescriptor] object
/// that describes the resource's schema. Information in the `labels` field
/// identifies the actual resource and its attributes according to the schema.
/// For example, a particular Compute Engine VM instance could be represented by
/// the following object, because the
/// [MonitoredResourceDescriptor][google.api.MonitoredResourceDescriptor] for
/// `"gce_instance"` has labels
/// `"project_id"`, `"instance_id"` and `"zone"`:
///
///      { "type": "gce_instance",
///        "labels": { "project_id": "my-project",
///                    "instance_id": "12345678901234",
///                    "zone": "us-central1-a" }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoredResource {
    /// Required. The monitored resource type. This field must match
    /// the `type` field of a
    /// [MonitoredResourceDescriptor][google.api.MonitoredResourceDescriptor]
    /// object. For example, the type of a Compute Engine VM instance is
    /// `gce_instance`. Some descriptors include the service name in the type; for
    /// example, the type of a Datastream stream is
    /// `datastream.googleapis.com/Stream`.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Required. Values for all of the labels listed in the associated monitored
    /// resource descriptor. For example, Compute Engine VM instances use the
    /// labels `"project_id"`, `"instance_id"`, and `"zone"`.
    #[prost(btree_map = "string, string", tag = "2")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Auxiliary metadata for a [MonitoredResource][google.api.MonitoredResource]
/// object. [MonitoredResource][google.api.MonitoredResource] objects contain the
/// minimum set of information to uniquely identify a monitored resource
/// instance. There is some other useful auxiliary metadata. Monitoring and
/// Logging use an ingestion pipeline to extract metadata for cloud resources of
/// all types, and store the metadata in this message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoredResourceMetadata {
    /// Output only. Values for predefined system metadata labels.
    /// System labels are a kind of metadata extracted by Google, including
    /// "machine_image", "vpc", "subnet_id",
    /// "security_group", "name", etc.
    /// System label values can be only strings, Boolean values, or a list of
    /// strings. For example:
    ///
    ///      { "name": "my-test-instance",
    ///        "security_group": \["a", "b", "c"\],
    ///        "spot_instance": false }
    #[prost(message, optional, tag = "1")]
    pub system_labels: ::core::option::Option<::prost_types::Struct>,
    /// Output only. A map of user-defined metadata labels.
    #[prost(btree_map = "string, string", tag = "2")]
    pub user_labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Monitoring configuration of the service.
///
/// The example below shows how to configure monitored resources and metrics
/// for monitoring. In the example, a monitored resource and two metrics are
/// defined. The `library.googleapis.com/book/returned_count` metric is sent
/// to both producer and consumer projects, whereas the
/// `library.googleapis.com/book/num_overdue` metric is only sent to the
/// consumer project.
///
///      monitored_resources:
///      - type: library.googleapis.com/Branch
///        display_name: "Library Branch"
///        description: "A branch of a library."
///        launch_stage: GA
///        labels:
///        - key: resource_container
///          description: "The Cloud container (ie. project id) for the Branch."
///        - key: location
///          description: "The location of the library branch."
///        - key: branch_id
///          description: "The id of the branch."
///      metrics:
///      - name: library.googleapis.com/book/returned_count
///        display_name: "Books Returned"
///        description: "The count of books that have been returned."
///        launch_stage: GA
///        metric_kind: DELTA
///        value_type: INT64
///        unit: "1"
///        labels:
///        - key: customer_id
///          description: "The id of the customer."
///      - name: library.googleapis.com/book/num_overdue
///        display_name: "Books Overdue"
///        description: "The current number of overdue books."
///        launch_stage: GA
///        metric_kind: GAUGE
///        value_type: INT64
///        unit: "1"
///        labels:
///        - key: customer_id
///          description: "The id of the customer."
///      monitoring:
///        producer_destinations:
///        - monitored_resource: library.googleapis.com/Branch
///          metrics:
///          - library.googleapis.com/book/returned_count
///        consumer_destinations:
///        - monitored_resource: library.googleapis.com/Branch
///          metrics:
///          - library.googleapis.com/book/returned_count
///          - library.googleapis.com/book/num_overdue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Monitoring {
    /// Monitoring configurations for sending metrics to the producer project.
    /// There can be multiple producer destinations. A monitored resource type may
    /// appear in multiple monitoring destinations if different aggregations are
    /// needed for different sets of metrics associated with that monitored
    /// resource type. A monitored resource and metric pair may only be used once
    /// in the Monitoring configuration.
    #[prost(message, repeated, tag = "1")]
    pub producer_destinations: ::prost::alloc::vec::Vec<
        monitoring::MonitoringDestination,
    >,
    /// Monitoring configurations for sending metrics to the consumer project.
    /// There can be multiple consumer destinations. A monitored resource type may
    /// appear in multiple monitoring destinations if different aggregations are
    /// needed for different sets of metrics associated with that monitored
    /// resource type. A monitored resource and metric pair may only be used once
    /// in the Monitoring configuration.
    #[prost(message, repeated, tag = "2")]
    pub consumer_destinations: ::prost::alloc::vec::Vec<
        monitoring::MonitoringDestination,
    >,
}
/// Nested message and enum types in `Monitoring`.
pub mod monitoring {
    /// Configuration of a specific monitoring destination (the producer project
    /// or the consumer project).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MonitoringDestination {
        /// The monitored resource type. The type must be defined in
        /// [Service.monitored_resources][google.api.Service.monitored_resources]
        /// section.
        #[prost(string, tag = "1")]
        pub monitored_resource: ::prost::alloc::string::String,
        /// Types of the metrics to report to this monitoring destination.
        /// Each type must be defined in
        /// [Service.metrics][google.api.Service.metrics] section.
        #[prost(string, repeated, tag = "2")]
        pub metrics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Quota configuration helps to achieve fairness and budgeting in service
/// usage.
///
/// The metric based quota configuration works this way:
/// - The service configuration defines a set of metrics.
/// - For API calls, the quota.metric_rules maps methods to metrics with
///    corresponding costs.
/// - The quota.limits defines limits on the metrics, which will be used for
///    quota checks at runtime.
///
/// An example quota configuration in yaml format:
///
///     quota:
///       limits:
///
///       - name: apiWriteQpsPerProject
///         metric: library.googleapis.com/write_calls
///         unit: "1/min/{project}"  # rate limit for consumer projects
///         values:
///           STANDARD: 10000
///
///
///       (The metric rules bind all methods to the read_calls metric,
///        except for the UpdateBook and DeleteBook methods. These two methods
///        are mapped to the write_calls metric, with the UpdateBook method
///        consuming at twice rate as the DeleteBook method.)
///       metric_rules:
///       - selector: "*"
///         metric_costs:
///           library.googleapis.com/read_calls: 1
///       - selector: google.example.library.v1.LibraryService.UpdateBook
///         metric_costs:
///           library.googleapis.com/write_calls: 2
///       - selector: google.example.library.v1.LibraryService.DeleteBook
///         metric_costs:
///           library.googleapis.com/write_calls: 1
///
///   Corresponding Metric definition:
///
///       metrics:
///       - name: library.googleapis.com/read_calls
///         display_name: Read requests
///         metric_kind: DELTA
///         value_type: INT64
///
///       - name: library.googleapis.com/write_calls
///         display_name: Write requests
///         metric_kind: DELTA
///         value_type: INT64
///
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quota {
    /// List of QuotaLimit definitions for the service.
    #[prost(message, repeated, tag = "3")]
    pub limits: ::prost::alloc::vec::Vec<QuotaLimit>,
    /// List of MetricRule definitions, each one mapping a selected method to one
    /// or more metrics.
    #[prost(message, repeated, tag = "4")]
    pub metric_rules: ::prost::alloc::vec::Vec<MetricRule>,
}
/// Bind API methods to metrics. Binding a method to a metric causes that
/// metric's configured quota behaviors to apply to the method call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricRule {
    /// Selects the methods to which this rule applies.
    ///
    /// Refer to [selector][google.api.DocumentationRule.selector] for syntax
    /// details.
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    /// Metrics to update when the selected methods are called, and the associated
    /// cost applied to each metric.
    ///
    /// The key of the map is the metric name, and the values are the amount
    /// increased for the metric against which the quota limits are defined.
    /// The value must not be negative.
    #[prost(btree_map = "string, int64", tag = "2")]
    pub metric_costs: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        i64,
    >,
}
/// `QuotaLimit` defines a specific limit that applies over a specified duration
/// for a limit type. There can be at most one limit for a duration and limit
/// type combination defined within a `QuotaGroup`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaLimit {
    /// Name of the quota limit.
    ///
    /// The name must be provided, and it must be unique within the service. The
    /// name can only include alphanumeric characters as well as '-'.
    ///
    /// The maximum length of the limit name is 64 characters.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// Optional. User-visible, extended description for this quota limit.
    /// Should be used only when more context is needed to understand this limit
    /// than provided by the limit's display name (see: `display_name`).
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Default number of tokens that can be consumed during the specified
    /// duration. This is the number of tokens assigned when a client
    /// application developer activates the service for his/her project.
    ///
    /// Specifying a value of 0 will block all requests. This can be used if you
    /// are provisioning quota to selected consumers and blocking others.
    /// Similarly, a value of -1 will indicate an unlimited quota. No other
    /// negative values are allowed.
    ///
    /// Used by group-based quotas only.
    #[prost(int64, tag = "3")]
    pub default_limit: i64,
    /// Maximum number of tokens that can be consumed during the specified
    /// duration. Client application developers can override the default limit up
    /// to this maximum. If specified, this value cannot be set to a value less
    /// than the default limit. If not specified, it is set to the default limit.
    ///
    /// To allow clients to apply overrides with no upper bound, set this to -1,
    /// indicating unlimited maximum quota.
    ///
    /// Used by group-based quotas only.
    #[prost(int64, tag = "4")]
    pub max_limit: i64,
    /// Free tier value displayed in the Developers Console for this limit.
    /// The free tier is the number of tokens that will be subtracted from the
    /// billed amount when billing is enabled.
    /// This field can only be set on a limit with duration "1d", in a billable
    /// group; it is invalid on any other limit. If this field is not set, it
    /// defaults to 0, indicating that there is no free tier for this service.
    ///
    /// Used by group-based quotas only.
    #[prost(int64, tag = "7")]
    pub free_tier: i64,
    /// Duration of this limit in textual notation. Must be "100s" or "1d".
    ///
    /// Used by group-based quotas only.
    #[prost(string, tag = "5")]
    pub duration: ::prost::alloc::string::String,
    /// The name of the metric this quota limit applies to. The quota limits with
    /// the same metric will be checked together during runtime. The metric must be
    /// defined within the service config.
    #[prost(string, tag = "8")]
    pub metric: ::prost::alloc::string::String,
    /// Specify the unit of the quota limit. It uses the same syntax as
    /// [Metric.unit][]. The supported unit kinds are determined by the quota
    /// backend system.
    ///
    /// Here are some examples:
    /// * "1/min/{project}" for quota per minute per project.
    ///
    /// Note: the order of unit components is insignificant.
    /// The "1" at the beginning is required to follow the metric unit syntax.
    #[prost(string, tag = "9")]
    pub unit: ::prost::alloc::string::String,
    /// Tiered limit values. You must specify this as a key:value pair, with an
    /// integer value that is the maximum number of requests allowed for the
    /// specified unit. Currently only STANDARD is supported.
    #[prost(btree_map = "string, int64", tag = "10")]
    pub values: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        i64,
    >,
    /// User-visible display name for this limit.
    /// Optional. If not set, the UI will provide a default display name based on
    /// the quota configuration. This field can be used to override the default
    /// display name generated from the configuration.
    #[prost(string, tag = "12")]
    pub display_name: ::prost::alloc::string::String,
}
/// Source information used to create a Service Config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceInfo {
    /// All files used during config generation.
    #[prost(message, repeated, tag = "1")]
    pub source_files: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// ### System parameter configuration
///
/// A system parameter is a special kind of parameter defined by the API
/// system, not by an individual API. It is typically mapped to an HTTP header
/// and/or a URL query parameter. This configuration specifies which methods
/// change the names of the system parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemParameters {
    /// Define system parameters.
    ///
    /// The parameters defined here will override the default parameters
    /// implemented by the system. If this field is missing from the service
    /// config, default system parameters will be used. Default system parameters
    /// and names is implementation-dependent.
    ///
    /// Example: define api key for all methods
    ///
    ///      system_parameters
    ///        rules:
    ///          - selector: "*"
    ///            parameters:
    ///              - name: api_key
    ///                url_query_parameter: api_key
    ///
    ///
    /// Example: define 2 api key names for a specific method.
    ///
    ///      system_parameters
    ///        rules:
    ///          - selector: "/ListShelves"
    ///            parameters:
    ///              - name: api_key
    ///                http_header: Api-Key1
    ///              - name: api_key
    ///                http_header: Api-Key2
    ///
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<SystemParameterRule>,
}
/// Define a system parameter rule mapping system parameter definitions to
/// methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemParameterRule {
    /// Selects the methods to which this rule applies. Use '*' to indicate all
    /// methods in all APIs.
    ///
    /// Refer to [selector][google.api.DocumentationRule.selector] for syntax
    /// details.
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    /// Define parameters. Multiple names may be defined for a parameter.
    /// For a given method call, only one of them should be used. If multiple
    /// names are used the behavior is implementation-dependent.
    /// If none of the specified names are present the behavior is
    /// parameter-dependent.
    #[prost(message, repeated, tag = "2")]
    pub parameters: ::prost::alloc::vec::Vec<SystemParameter>,
}
/// Define a parameter's name and location. The parameter may be passed as either
/// an HTTP header or a URL query parameter, and if both are passed the behavior
/// is implementation-dependent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemParameter {
    /// Define the name of the parameter, such as "api_key" . It is case sensitive.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Define the HTTP header name to use for the parameter. It is case
    /// insensitive.
    #[prost(string, tag = "2")]
    pub http_header: ::prost::alloc::string::String,
    /// Define the URL query parameter name to use for the parameter. It is case
    /// sensitive.
    #[prost(string, tag = "3")]
    pub url_query_parameter: ::prost::alloc::string::String,
}
/// Configuration controlling usage of a service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Usage {
    /// Requirements that must be satisfied before a consumer project can use the
    /// service. Each requirement is of the form <service.name>/<requirement-id>;
    /// for example 'serviceusage.googleapis.com/billing-enabled'.
    ///
    /// For Google APIs, a Terms of Service requirement must be included here.
    /// Google Cloud APIs must include "serviceusage.googleapis.com/tos/cloud".
    /// Other Google APIs should include
    /// "serviceusage.googleapis.com/tos/universal". Additional ToS can be
    /// included based on the business needs.
    #[prost(string, repeated, tag = "1")]
    pub requirements: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of usage rules that apply to individual API methods.
    ///
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    #[prost(message, repeated, tag = "6")]
    pub rules: ::prost::alloc::vec::Vec<UsageRule>,
    /// The full resource name of a channel used for sending notifications to the
    /// service producer.
    ///
    /// Google Service Management currently only supports
    /// [Google Cloud Pub/Sub](<https://cloud.google.com/pubsub>) as a notification
    /// channel. To use Google Cloud Pub/Sub as the channel, this must be the name
    /// of a Cloud Pub/Sub topic that uses the Cloud Pub/Sub topic name format
    /// documented in <https://cloud.google.com/pubsub/docs/overview.>
    #[prost(string, tag = "7")]
    pub producer_notification_channel: ::prost::alloc::string::String,
}
/// Usage configuration rules for the service.
///
/// NOTE: Under development.
///
///
/// Use this rule to configure unregistered calls for the service. Unregistered
/// calls are calls that do not contain consumer project identity.
/// (Example: calls that do not contain an API key).
/// By default, API methods do not allow unregistered calls, and each method call
/// must be identified by a consumer project identity. Use this rule to
/// allow/disallow unregistered calls.
///
/// Example of an API that wants to allow unregistered calls for entire service.
///
///      usage:
///        rules:
///        - selector: "*"
///          allow_unregistered_calls: true
///
/// Example of a method that wants to allow unregistered calls.
///
///      usage:
///        rules:
///        - selector: "google.example.library.v1.LibraryService.CreateBook"
///          allow_unregistered_calls: true
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsageRule {
    /// Selects the methods to which this rule applies. Use '*' to indicate all
    /// methods in all APIs.
    ///
    /// Refer to [selector][google.api.DocumentationRule.selector] for syntax
    /// details.
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    /// If true, the selected method allows unregistered calls, e.g. calls
    /// that don't identify any user or application.
    #[prost(bool, tag = "2")]
    pub allow_unregistered_calls: bool,
    /// If true, the selected method should skip service control and the control
    /// plane features, such as quota and billing, will not be available.
    /// This flag is used by Google Cloud Endpoints to bypass checks for internal
    /// methods, such as service health check methods.
    #[prost(bool, tag = "3")]
    pub skip_service_control: bool,
}
/// `Service` is the root object of Google API service configuration (service
/// config). It describes the basic information about a logical service,
/// such as the service name and the user-facing title, and delegates other
/// aspects to sub-sections. Each sub-section is either a proto message or a
/// repeated proto message that configures a specific aspect, such as auth.
/// For more information, see each proto message definition.
///
/// Example:
///
///      type: google.api.Service
///      name: calendar.googleapis.com
///      title: Google Calendar API
///      apis:
///      - name: google.calendar.v3.Calendar
///
///      visibility:
///        rules:
///        - selector: "google.calendar.v3.*"
///          restriction: PREVIEW
///      backend:
///        rules:
///        - selector: "google.calendar.v3.*"
///          address: calendar.example.com
///
///      authentication:
///        providers:
///        - id: google_calendar_auth
///          jwks_uri: <https://www.googleapis.com/oauth2/v1/certs>
///          issuer: <https://securetoken.google.com>
///        rules:
///        - selector: "*"
///          requirements:
///            provider_id: google_calendar_auth
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// The service name, which is a DNS-like logical identifier for the
    /// service, such as `calendar.googleapis.com`. The service name
    /// typically goes through DNS verification to make sure the owner
    /// of the service also owns the DNS name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The product title for this service, it is the name displayed in Google
    /// Cloud Console.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// The Google project that owns this service.
    #[prost(string, tag = "22")]
    pub producer_project_id: ::prost::alloc::string::String,
    /// A unique ID for a specific instance of this message, typically assigned
    /// by the client for tracking purpose. Must be no longer than 63 characters
    /// and only lower case letters, digits, '.', '_' and '-' are allowed. If
    /// empty, the server may choose to generate one instead.
    #[prost(string, tag = "33")]
    pub id: ::prost::alloc::string::String,
    /// A list of API interfaces exported by this service. Only the `name` field
    /// of the [google.protobuf.Api][google.protobuf.Api] needs to be provided by
    /// the configuration author, as the remaining fields will be derived from the
    /// IDL during the normalization process. It is an error to specify an API
    /// interface here which cannot be resolved against the associated IDL files.
    #[prost(message, repeated, tag = "3")]
    pub apis: ::prost::alloc::vec::Vec<::prost_types::Api>,
    /// A list of all proto message types included in this API service.
    /// Types referenced directly or indirectly by the `apis` are automatically
    /// included.  Messages which are not referenced but shall be included, such as
    /// types used by the `google.protobuf.Any` type, should be listed here by
    /// name by the configuration author. Example:
    ///
    ///      types:
    ///      - name: google.protobuf.Int32
    #[prost(message, repeated, tag = "4")]
    pub types: ::prost::alloc::vec::Vec<::prost_types::Type>,
    /// A list of all enum types included in this API service.  Enums referenced
    /// directly or indirectly by the `apis` are automatically included.  Enums
    /// which are not referenced but shall be included should be listed here by
    /// name by the configuration author. Example:
    ///
    ///      enums:
    ///      - name: google.someapi.v1.SomeEnum
    #[prost(message, repeated, tag = "5")]
    pub enums: ::prost::alloc::vec::Vec<::prost_types::Enum>,
    /// Additional API documentation.
    #[prost(message, optional, tag = "6")]
    pub documentation: ::core::option::Option<Documentation>,
    /// API backend configuration.
    #[prost(message, optional, tag = "8")]
    pub backend: ::core::option::Option<Backend>,
    /// HTTP configuration.
    #[prost(message, optional, tag = "9")]
    pub http: ::core::option::Option<Http>,
    /// Quota configuration.
    #[prost(message, optional, tag = "10")]
    pub quota: ::core::option::Option<Quota>,
    /// Auth configuration.
    #[prost(message, optional, tag = "11")]
    pub authentication: ::core::option::Option<Authentication>,
    /// Context configuration.
    #[prost(message, optional, tag = "12")]
    pub context: ::core::option::Option<Context>,
    /// Configuration controlling usage of this service.
    #[prost(message, optional, tag = "15")]
    pub usage: ::core::option::Option<Usage>,
    /// Configuration for network endpoints.  If this is empty, then an endpoint
    /// with the same name as the service is automatically generated to service all
    /// defined APIs.
    #[prost(message, repeated, tag = "18")]
    pub endpoints: ::prost::alloc::vec::Vec<Endpoint>,
    /// Configuration for the service control plane.
    #[prost(message, optional, tag = "21")]
    pub control: ::core::option::Option<Control>,
    /// Defines the logs used by this service.
    #[prost(message, repeated, tag = "23")]
    pub logs: ::prost::alloc::vec::Vec<LogDescriptor>,
    /// Defines the metrics used by this service.
    #[prost(message, repeated, tag = "24")]
    pub metrics: ::prost::alloc::vec::Vec<MetricDescriptor>,
    /// Defines the monitored resources used by this service. This is required
    /// by the [Service.monitoring][google.api.Service.monitoring] and
    /// [Service.logging][google.api.Service.logging] configurations.
    #[prost(message, repeated, tag = "25")]
    pub monitored_resources: ::prost::alloc::vec::Vec<MonitoredResourceDescriptor>,
    /// Billing configuration.
    #[prost(message, optional, tag = "26")]
    pub billing: ::core::option::Option<Billing>,
    /// Logging configuration.
    #[prost(message, optional, tag = "27")]
    pub logging: ::core::option::Option<Logging>,
    /// Monitoring configuration.
    #[prost(message, optional, tag = "28")]
    pub monitoring: ::core::option::Option<Monitoring>,
    /// System parameter configuration.
    #[prost(message, optional, tag = "29")]
    pub system_parameters: ::core::option::Option<SystemParameters>,
    /// Output only. The source information for this configuration if available.
    #[prost(message, optional, tag = "37")]
    pub source_info: ::core::option::Option<SourceInfo>,
    /// Settings for [Google Cloud Client
    /// libraries](<https://cloud.google.com/apis/docs/cloud-client-libraries>)
    /// generated from APIs defined as protocol buffers.
    #[prost(message, optional, tag = "45")]
    pub publishing: ::core::option::Option<Publishing>,
    /// Obsolete. Do not use.
    ///
    /// This field has no semantic meaning. The service config compiler always
    /// sets this field to `3`.
    #[prost(message, optional, tag = "20")]
    pub config_version: ::core::option::Option<u32>,
}
/// Rich semantic information of an API field beyond basic typing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldInfo {
    /// The standard format of a field value. This does not explicitly configure
    /// any API consumer, just documents the API's format for the field it is
    /// applied to.
    #[prost(enumeration = "field_info::Format", tag = "1")]
    pub format: i32,
}
/// Nested message and enum types in `FieldInfo`.
pub mod field_info {
    /// The standard format of a field value. The supported formats are all backed
    /// by either an RFC defined by the IETF or a Google-defined AIP.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Format {
        /// Default, unspecified value.
        Unspecified = 0,
        /// Universally Unique Identifier, version 4, value as defined by
        /// <https://datatracker.ietf.org/doc/html/rfc4122.> The value may be
        /// normalized to entirely lowercase letters. For example, the value
        /// `F47AC10B-58CC-0372-8567-0E02B2C3D479` would be normalized to
        /// `f47ac10b-58cc-0372-8567-0e02b2c3d479`.
        Uuid4 = 1,
        /// Internet Protocol v4 value as defined by [RFC
        /// 791](<https://datatracker.ietf.org/doc/html/rfc791>). The value may be
        /// condensed, with leading zeros in each octet stripped. For example,
        /// `001.022.233.040` would be condensed to `1.22.233.40`.
        Ipv4 = 2,
        /// Internet Protocol v6 value as defined by [RFC
        /// 2460](<https://datatracker.ietf.org/doc/html/rfc2460>). The value may be
        /// normalized to entirely lowercase letters, and zero-padded partial and
        /// empty octets. For example, the value `2001:DB8::` would be normalized to
        /// `2001:0db8:0:0`.
        Ipv6 = 3,
        /// An IP address in either v4 or v6 format as described by the individual
        /// values defined herein. See the comments on the IPV4 and IPV6 types for
        /// allowed normalizations of each.
        Ipv4OrIpv6 = 4,
    }
    impl Format {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Format::Unspecified => "FORMAT_UNSPECIFIED",
                Format::Uuid4 => "UUID4",
                Format::Ipv4 => "IPV4",
                Format::Ipv6 => "IPV6",
                Format::Ipv4OrIpv6 => "IPV4_OR_IPV6",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
                "UUID4" => Some(Self::Uuid4),
                "IPV4" => Some(Self::Ipv4),
                "IPV6" => Some(Self::Ipv6),
                "IPV4_OR_IPV6" => Some(Self::Ipv4OrIpv6),
                _ => None,
            }
        }
    }
}
/// Defines the supported values for `google.rpc.ErrorInfo.reason` for the
/// `googleapis.com` error domain. This error domain is reserved for [Service
/// Infrastructure](<https://cloud.google.com/service-infrastructure/docs/overview>).
/// For each error info of this domain, the metadata key "service" refers to the
/// logical identifier of an API service, such as "pubsub.googleapis.com". The
/// "consumer" refers to the entity that consumes an API Service. It typically is
/// a Google project that owns the client application or the server resource,
/// such as "projects/123". Other metadata keys are specific to each error
/// reason. For more information, see the definition of the specific error
/// reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorReason {
    /// Do not use this default value.
    Unspecified = 0,
    /// The request is calling a disabled service for a consumer.
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" contacting
    /// "pubsub.googleapis.com" service which is disabled:
    ///
    ///      { "reason": "SERVICE_DISABLED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "pubsub.googleapis.com"
    ///        }
    ///      }
    ///
    /// This response indicates the "pubsub.googleapis.com" has been disabled in
    /// "projects/123".
    ServiceDisabled = 1,
    /// The request whose associated billing account is disabled.
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" fails to contact
    /// "pubsub.googleapis.com" service because the associated billing account is
    /// disabled:
    ///
    ///      { "reason": "BILLING_DISABLED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "pubsub.googleapis.com"
    ///        }
    ///      }
    ///
    /// This response indicates the billing account associated has been disabled.
    BillingDisabled = 2,
    /// The request is denied because the provided [API
    /// key](<https://cloud.google.com/docs/authentication/api-keys>) is invalid. It
    /// may be in a bad format, cannot be found, or has been expired).
    ///
    /// Example of an ErrorInfo when the request is contacting
    /// "storage.googleapis.com" service with an invalid API key:
    ///
    ///      { "reason": "API_KEY_INVALID",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "service": "storage.googleapis.com",
    ///        }
    ///      }
    ApiKeyInvalid = 3,
    /// The request is denied because it violates [API key API
    /// restrictions](<https://cloud.google.com/docs/authentication/api-keys#adding_api_restrictions>).
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" fails to call the
    /// "storage.googleapis.com" service because this service is restricted in the
    /// API key:
    ///
    ///      { "reason": "API_KEY_SERVICE_BLOCKED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "storage.googleapis.com"
    ///        }
    ///      }
    ApiKeyServiceBlocked = 4,
    /// The request is denied because it violates [API key HTTP
    /// restrictions](<https://cloud.google.com/docs/authentication/api-keys#adding_http_restrictions>).
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" fails to call
    /// "storage.googleapis.com" service because the http referrer of the request
    /// violates API key HTTP restrictions:
    ///
    ///      { "reason": "API_KEY_HTTP_REFERRER_BLOCKED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "storage.googleapis.com",
    ///        }
    ///      }
    ApiKeyHttpReferrerBlocked = 7,
    /// The request is denied because it violates [API key IP address
    /// restrictions](<https://cloud.google.com/docs/authentication/api-keys#adding_application_restrictions>).
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" fails to call
    /// "storage.googleapis.com" service because the caller IP of the request
    /// violates API key IP address restrictions:
    ///
    ///      { "reason": "API_KEY_IP_ADDRESS_BLOCKED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "storage.googleapis.com",
    ///        }
    ///      }
    ApiKeyIpAddressBlocked = 8,
    /// The request is denied because it violates [API key Android application
    /// restrictions](<https://cloud.google.com/docs/authentication/api-keys#adding_application_restrictions>).
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" fails to call
    /// "storage.googleapis.com" service because the request from the Android apps
    /// violates the API key Android application restrictions:
    ///
    ///      { "reason": "API_KEY_ANDROID_APP_BLOCKED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "storage.googleapis.com"
    ///        }
    ///      }
    ApiKeyAndroidAppBlocked = 9,
    /// The request is denied because it violates [API key iOS application
    /// restrictions](<https://cloud.google.com/docs/authentication/api-keys#adding_application_restrictions>).
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" fails to call
    /// "storage.googleapis.com" service because the request from the iOS apps
    /// violates the API key iOS application restrictions:
    ///
    ///      { "reason": "API_KEY_IOS_APP_BLOCKED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "storage.googleapis.com"
    ///        }
    ///      }
    ApiKeyIosAppBlocked = 13,
    /// The request is denied because there is not enough rate quota for the
    /// consumer.
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" fails to contact
    /// "pubsub.googleapis.com" service because consumer's rate quota usage has
    /// reached the maximum value set for the quota limit
    /// "ReadsPerMinutePerProject" on the quota metric
    /// "pubsub.googleapis.com/read_requests":
    ///
    ///      { "reason": "RATE_LIMIT_EXCEEDED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "pubsub.googleapis.com",
    ///          "quota_metric": "pubsub.googleapis.com/read_requests",
    ///          "quota_limit": "ReadsPerMinutePerProject"
    ///        }
    ///      }
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" checks quota on
    /// the service "dataflow.googleapis.com" and hits the organization quota
    /// limit "DefaultRequestsPerMinutePerOrganization" on the metric
    /// "dataflow.googleapis.com/default_requests".
    ///
    ///      { "reason": "RATE_LIMIT_EXCEEDED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "dataflow.googleapis.com",
    ///          "quota_metric": "dataflow.googleapis.com/default_requests",
    ///          "quota_limit": "DefaultRequestsPerMinutePerOrganization"
    ///        }
    ///      }
    RateLimitExceeded = 5,
    /// The request is denied because there is not enough resource quota for the
    /// consumer.
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" fails to contact
    /// "compute.googleapis.com" service because consumer's resource quota usage
    /// has reached the maximum value set for the quota limit "VMsPerProject"
    /// on the quota metric "compute.googleapis.com/vms":
    ///
    ///      { "reason": "RESOURCE_QUOTA_EXCEEDED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "compute.googleapis.com",
    ///          "quota_metric": "compute.googleapis.com/vms",
    ///          "quota_limit": "VMsPerProject"
    ///        }
    ///      }
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" checks resource
    /// quota on the service "dataflow.googleapis.com" and hits the organization
    /// quota limit "jobs-per-organization" on the metric
    /// "dataflow.googleapis.com/job_count".
    ///
    ///      { "reason": "RESOURCE_QUOTA_EXCEEDED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "dataflow.googleapis.com",
    ///          "quota_metric": "dataflow.googleapis.com/job_count",
    ///          "quota_limit": "jobs-per-organization"
    ///        }
    ///      }
    ResourceQuotaExceeded = 6,
    /// The request whose associated billing account address is in a tax restricted
    /// location, violates the local tax restrictions when creating resources in
    /// the restricted region.
    ///
    /// Example of an ErrorInfo when creating the Cloud Storage Bucket in the
    /// container "projects/123" under a tax restricted region
    /// "locations/asia-northeast3":
    ///
    ///      { "reason": "LOCATION_TAX_POLICY_VIOLATED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "storage.googleapis.com",
    ///          "location": "locations/asia-northeast3"
    ///        }
    ///      }
    ///
    /// This response indicates creating the Cloud Storage Bucket in
    /// "locations/asia-northeast3" violates the location tax restriction.
    LocationTaxPolicyViolated = 10,
    /// The request is denied because the caller does not have required permission
    /// on the user project "projects/123" or the user project is invalid. For more
    /// information, check the [userProject System
    /// Parameters](<https://cloud.google.com/apis/docs/system-parameters>).
    ///
    /// Example of an ErrorInfo when the caller is calling Cloud Storage service
    /// with insufficient permissions on the user project:
    ///
    ///      { "reason": "USER_PROJECT_DENIED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "storage.googleapis.com"
    ///        }
    ///      }
    UserProjectDenied = 11,
    /// The request is denied because the consumer "projects/123" is suspended due
    /// to Terms of Service(Tos) violations. Check [Project suspension
    /// guidelines](<https://cloud.google.com/resource-manager/docs/project-suspension-guidelines>)
    /// for more information.
    ///
    /// Example of an ErrorInfo when calling Cloud Storage service with the
    /// suspended consumer "projects/123":
    ///
    ///      { "reason": "CONSUMER_SUSPENDED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "storage.googleapis.com"
    ///        }
    ///      }
    ConsumerSuspended = 12,
    /// The request is denied because the associated consumer is invalid. It may be
    /// in a bad format, cannot be found, or have been deleted.
    ///
    /// Example of an ErrorInfo when calling Cloud Storage service with the
    /// invalid consumer "projects/123":
    ///
    ///      { "reason": "CONSUMER_INVALID",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "storage.googleapis.com"
    ///        }
    ///      }
    ConsumerInvalid = 14,
    /// The request is denied because it violates [VPC Service
    /// Controls](<https://cloud.google.com/vpc-service-controls/docs/overview>).
    /// The 'uid' field is a random generated identifier that customer can use it
    /// to search the audit log for a request rejected by VPC Service Controls. For
    /// more information, please refer [VPC Service Controls
    /// Troubleshooting](<https://cloud.google.com/vpc-service-controls/docs/troubleshooting#unique-id>)
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" fails to call
    /// Cloud Storage service because the request is prohibited by the VPC Service
    /// Controls.
    ///
    ///      { "reason": "SECURITY_POLICY_VIOLATED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "uid": "123456789abcde",
    ///          "consumer": "projects/123",
    ///          "service": "storage.googleapis.com"
    ///        }
    ///      }
    SecurityPolicyViolated = 15,
    /// The request is denied because the provided access token has expired.
    ///
    /// Example of an ErrorInfo when the request is calling Cloud Storage service
    /// with an expired access token:
    ///
    ///      { "reason": "ACCESS_TOKEN_EXPIRED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "service": "storage.googleapis.com",
    ///          "method": "google.storage.v1.Storage.GetObject"
    ///        }
    ///      }
    AccessTokenExpired = 16,
    /// The request is denied because the provided access token doesn't have at
    /// least one of the acceptable scopes required for the API. Please check
    /// [OAuth 2.0 Scopes for Google
    /// APIs](<https://developers.google.com/identity/protocols/oauth2/scopes>) for
    /// the list of the OAuth 2.0 scopes that you might need to request to access
    /// the API.
    ///
    /// Example of an ErrorInfo when the request is calling Cloud Storage service
    /// with an access token that is missing required scopes:
    ///
    ///      { "reason": "ACCESS_TOKEN_SCOPE_INSUFFICIENT",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "service": "storage.googleapis.com",
    ///          "method": "google.storage.v1.Storage.GetObject"
    ///        }
    ///      }
    AccessTokenScopeInsufficient = 17,
    /// The request is denied because the account associated with the provided
    /// access token is in an invalid state, such as disabled or deleted.
    /// For more information, see <https://cloud.google.com/docs/authentication.>
    ///
    /// Warning: For privacy reasons, the server may not be able to disclose the
    /// email address for some accounts. The client MUST NOT depend on the
    /// availability of the `email` attribute.
    ///
    /// Example of an ErrorInfo when the request is to the Cloud Storage API with
    /// an access token that is associated with a disabled or deleted [service
    /// account](<http://cloud/iam/docs/service-accounts>):
    ///
    ///      { "reason": "ACCOUNT_STATE_INVALID",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "service": "storage.googleapis.com",
    ///          "method": "google.storage.v1.Storage.GetObject",
    ///          "email": "user@123.iam.gserviceaccount.com"
    ///        }
    ///      }
    AccountStateInvalid = 18,
    /// The request is denied because the type of the provided access token is not
    /// supported by the API being called.
    ///
    /// Example of an ErrorInfo when the request is to the Cloud Storage API with
    /// an unsupported token type.
    ///
    ///      { "reason": "ACCESS_TOKEN_TYPE_UNSUPPORTED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "service": "storage.googleapis.com",
    ///          "method": "google.storage.v1.Storage.GetObject"
    ///        }
    ///      }
    AccessTokenTypeUnsupported = 19,
    /// The request is denied because the request doesn't have any authentication
    /// credentials. For more information regarding the supported authentication
    /// strategies for Google Cloud APIs, see
    /// <https://cloud.google.com/docs/authentication.>
    ///
    /// Example of an ErrorInfo when the request is to the Cloud Storage API
    /// without any authentication credentials.
    ///
    ///      { "reason": "CREDENTIALS_MISSING",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "service": "storage.googleapis.com",
    ///          "method": "google.storage.v1.Storage.GetObject"
    ///        }
    ///      }
    CredentialsMissing = 20,
    /// The request is denied because the provided project owning the resource
    /// which acts as the [API
    /// consumer](<https://cloud.google.com/apis/design/glossary#api_consumer>) is
    /// invalid. It may be in a bad format or empty.
    ///
    /// Example of an ErrorInfo when the request is to the Cloud Functions API,
    /// but the offered resource project in the request in a bad format which can't
    /// perform the ListFunctions method.
    ///
    ///      { "reason": "RESOURCE_PROJECT_INVALID",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "service": "cloudfunctions.googleapis.com",
    ///          "method":
    ///          "google.cloud.functions.v1.CloudFunctionsService.ListFunctions"
    ///        }
    ///      }
    ResourceProjectInvalid = 21,
    /// The request is denied because the provided session cookie is missing,
    /// invalid or failed to decode.
    ///
    /// Example of an ErrorInfo when the request is calling Cloud Storage service
    /// with a SID cookie which can't be decoded.
    ///
    ///      { "reason": "SESSION_COOKIE_INVALID",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "service": "storage.googleapis.com",
    ///          "method": "google.storage.v1.Storage.GetObject",
    ///          "cookie": "SID"
    ///        }
    ///      }
    SessionCookieInvalid = 23,
    /// The request is denied because the user is from a Google Workspace customer
    /// that blocks their users from accessing a particular service.
    ///
    /// Example scenario: <https://support.google.com/a/answer/9197205?hl=en>
    ///
    /// Example of an ErrorInfo when access to Google Cloud Storage service is
    /// blocked by the Google Workspace administrator:
    ///
    ///      { "reason": "USER_BLOCKED_BY_ADMIN",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "service": "storage.googleapis.com",
    ///          "method": "google.storage.v1.Storage.GetObject",
    ///        }
    ///      }
    UserBlockedByAdmin = 24,
    /// The request is denied because the resource service usage is restricted
    /// by administrators according to the organization policy constraint.
    /// For more information see
    /// <https://cloud.google.com/resource-manager/docs/organization-policy/restricting-services.>
    ///
    /// Example of an ErrorInfo when access to Google Cloud Storage service is
    /// restricted by Resource Usage Restriction policy:
    ///
    ///      { "reason": "RESOURCE_USAGE_RESTRICTION_VIOLATED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/project-123",
    ///          "service": "storage.googleapis.com"
    ///        }
    ///      }
    ResourceUsageRestrictionViolated = 25,
    /// Unimplemented. Do not use.
    ///
    /// The request is denied because it contains unsupported system parameters in
    /// URL query parameters or HTTP headers. For more information,
    /// see <https://cloud.google.com/apis/docs/system-parameters>
    ///
    /// Example of an ErrorInfo when access "pubsub.googleapis.com" service with
    /// a request header of "x-goog-user-ip":
    ///
    ///      { "reason": "SYSTEM_PARAMETER_UNSUPPORTED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "service": "pubsub.googleapis.com"
    ///          "parameter": "x-goog-user-ip"
    ///        }
    ///      }
    SystemParameterUnsupported = 26,
    /// The request is denied because it violates Org Restriction: the requested
    /// resource does not belong to allowed organizations specified in
    /// "X-Goog-Allowed-Resources" header.
    ///
    /// Example of an ErrorInfo when accessing a GCP resource that is restricted by
    /// Org Restriction for "pubsub.googleapis.com" service.
    ///
    /// {
    ///    reason: "ORG_RESTRICTION_VIOLATION"
    ///    domain: "googleapis.com"
    ///    metadata {
    ///      "consumer":"projects/123456"
    ///      "service": "pubsub.googleapis.com"
    ///    }
    /// }
    OrgRestrictionViolation = 27,
    /// The request is denied because "X-Goog-Allowed-Resources" header is in a bad
    /// format.
    ///
    /// Example of an ErrorInfo when
    /// accessing "pubsub.googleapis.com" service with an invalid
    /// "X-Goog-Allowed-Resources" request header.
    ///
    /// {
    ///    reason: "ORG_RESTRICTION_HEADER_INVALID"
    ///    domain: "googleapis.com"
    ///    metadata {
    ///      "consumer":"projects/123456"
    ///      "service": "pubsub.googleapis.com"
    ///    }
    /// }
    OrgRestrictionHeaderInvalid = 28,
    /// Unimplemented. Do not use.
    ///
    /// The request is calling a service that is not visible to the consumer.
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" contacting
    ///   "pubsub.googleapis.com" service which is not visible to the consumer.
    ///
    ///      { "reason": "SERVICE_NOT_VISIBLE",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "pubsub.googleapis.com"
    ///        }
    ///      }
    ///
    /// This response indicates the "pubsub.googleapis.com" is not visible to
    /// "projects/123" (or it may not exist).
    ServiceNotVisible = 29,
    /// The request is related to a project for which GCP access is suspended.
    ///
    /// Example of an ErrorInfo when the consumer "projects/123" fails to contact
    /// "pubsub.googleapis.com" service because GCP access is suspended:
    ///
    ///      { "reason": "GCP_SUSPENDED",
    ///        "domain": "googleapis.com",
    ///        "metadata": {
    ///          "consumer": "projects/123",
    ///          "service": "pubsub.googleapis.com"
    ///        }
    ///      }
    ///
    /// This response indicates the associated GCP account has been suspended.
    GcpSuspended = 30,
}
impl ErrorReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ErrorReason::Unspecified => "ERROR_REASON_UNSPECIFIED",
            ErrorReason::ServiceDisabled => "SERVICE_DISABLED",
            ErrorReason::BillingDisabled => "BILLING_DISABLED",
            ErrorReason::ApiKeyInvalid => "API_KEY_INVALID",
            ErrorReason::ApiKeyServiceBlocked => "API_KEY_SERVICE_BLOCKED",
            ErrorReason::ApiKeyHttpReferrerBlocked => "API_KEY_HTTP_REFERRER_BLOCKED",
            ErrorReason::ApiKeyIpAddressBlocked => "API_KEY_IP_ADDRESS_BLOCKED",
            ErrorReason::ApiKeyAndroidAppBlocked => "API_KEY_ANDROID_APP_BLOCKED",
            ErrorReason::ApiKeyIosAppBlocked => "API_KEY_IOS_APP_BLOCKED",
            ErrorReason::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            ErrorReason::ResourceQuotaExceeded => "RESOURCE_QUOTA_EXCEEDED",
            ErrorReason::LocationTaxPolicyViolated => "LOCATION_TAX_POLICY_VIOLATED",
            ErrorReason::UserProjectDenied => "USER_PROJECT_DENIED",
            ErrorReason::ConsumerSuspended => "CONSUMER_SUSPENDED",
            ErrorReason::ConsumerInvalid => "CONSUMER_INVALID",
            ErrorReason::SecurityPolicyViolated => "SECURITY_POLICY_VIOLATED",
            ErrorReason::AccessTokenExpired => "ACCESS_TOKEN_EXPIRED",
            ErrorReason::AccessTokenScopeInsufficient => {
                "ACCESS_TOKEN_SCOPE_INSUFFICIENT"
            }
            ErrorReason::AccountStateInvalid => "ACCOUNT_STATE_INVALID",
            ErrorReason::AccessTokenTypeUnsupported => "ACCESS_TOKEN_TYPE_UNSUPPORTED",
            ErrorReason::CredentialsMissing => "CREDENTIALS_MISSING",
            ErrorReason::ResourceProjectInvalid => "RESOURCE_PROJECT_INVALID",
            ErrorReason::SessionCookieInvalid => "SESSION_COOKIE_INVALID",
            ErrorReason::UserBlockedByAdmin => "USER_BLOCKED_BY_ADMIN",
            ErrorReason::ResourceUsageRestrictionViolated => {
                "RESOURCE_USAGE_RESTRICTION_VIOLATED"
            }
            ErrorReason::SystemParameterUnsupported => "SYSTEM_PARAMETER_UNSUPPORTED",
            ErrorReason::OrgRestrictionViolation => "ORG_RESTRICTION_VIOLATION",
            ErrorReason::OrgRestrictionHeaderInvalid => "ORG_RESTRICTION_HEADER_INVALID",
            ErrorReason::ServiceNotVisible => "SERVICE_NOT_VISIBLE",
            ErrorReason::GcpSuspended => "GCP_SUSPENDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ERROR_REASON_UNSPECIFIED" => Some(Self::Unspecified),
            "SERVICE_DISABLED" => Some(Self::ServiceDisabled),
            "BILLING_DISABLED" => Some(Self::BillingDisabled),
            "API_KEY_INVALID" => Some(Self::ApiKeyInvalid),
            "API_KEY_SERVICE_BLOCKED" => Some(Self::ApiKeyServiceBlocked),
            "API_KEY_HTTP_REFERRER_BLOCKED" => Some(Self::ApiKeyHttpReferrerBlocked),
            "API_KEY_IP_ADDRESS_BLOCKED" => Some(Self::ApiKeyIpAddressBlocked),
            "API_KEY_ANDROID_APP_BLOCKED" => Some(Self::ApiKeyAndroidAppBlocked),
            "API_KEY_IOS_APP_BLOCKED" => Some(Self::ApiKeyIosAppBlocked),
            "RATE_LIMIT_EXCEEDED" => Some(Self::RateLimitExceeded),
            "RESOURCE_QUOTA_EXCEEDED" => Some(Self::ResourceQuotaExceeded),
            "LOCATION_TAX_POLICY_VIOLATED" => Some(Self::LocationTaxPolicyViolated),
            "USER_PROJECT_DENIED" => Some(Self::UserProjectDenied),
            "CONSUMER_SUSPENDED" => Some(Self::ConsumerSuspended),
            "CONSUMER_INVALID" => Some(Self::ConsumerInvalid),
            "SECURITY_POLICY_VIOLATED" => Some(Self::SecurityPolicyViolated),
            "ACCESS_TOKEN_EXPIRED" => Some(Self::AccessTokenExpired),
            "ACCESS_TOKEN_SCOPE_INSUFFICIENT" => Some(Self::AccessTokenScopeInsufficient),
            "ACCOUNT_STATE_INVALID" => Some(Self::AccountStateInvalid),
            "ACCESS_TOKEN_TYPE_UNSUPPORTED" => Some(Self::AccessTokenTypeUnsupported),
            "CREDENTIALS_MISSING" => Some(Self::CredentialsMissing),
            "RESOURCE_PROJECT_INVALID" => Some(Self::ResourceProjectInvalid),
            "SESSION_COOKIE_INVALID" => Some(Self::SessionCookieInvalid),
            "USER_BLOCKED_BY_ADMIN" => Some(Self::UserBlockedByAdmin),
            "RESOURCE_USAGE_RESTRICTION_VIOLATED" => {
                Some(Self::ResourceUsageRestrictionViolated)
            }
            "SYSTEM_PARAMETER_UNSUPPORTED" => Some(Self::SystemParameterUnsupported),
            "ORG_RESTRICTION_VIOLATION" => Some(Self::OrgRestrictionViolation),
            "ORG_RESTRICTION_HEADER_INVALID" => Some(Self::OrgRestrictionHeaderInvalid),
            "SERVICE_NOT_VISIBLE" => Some(Self::ServiceNotVisible),
            "GCP_SUSPENDED" => Some(Self::GcpSuspended),
            _ => None,
        }
    }
}
/// `Visibility` restricts service consumer's access to service elements,
/// such as whether an application can call a visibility-restricted method.
/// The restriction is expressed by applying visibility labels on service
/// elements. The visibility labels are elsewhere linked to service consumers.
///
/// A service can define multiple visibility labels, but a service consumer
/// should be granted at most one visibility label. Multiple visibility
/// labels for a single service consumer are not supported.
///
/// If an element and all its parents have no visibility label, its visibility
/// is unconditionally granted.
///
/// Example:
///
///      visibility:
///        rules:
///        - selector: google.calendar.Calendar.EnhancedSearch
///          restriction: PREVIEW
///        - selector: google.calendar.Calendar.Delegate
///          restriction: INTERNAL
///
/// Here, all methods are publicly visible except for the restricted methods
/// EnhancedSearch and Delegate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Visibility {
    /// A list of visibility rules that apply to individual API elements.
    ///
    /// **NOTE:** All service configuration rules follow "last one wins" order.
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<VisibilityRule>,
}
/// A visibility rule provides visibility configuration for an individual API
/// element.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisibilityRule {
    /// Selects methods, messages, fields, enums, etc. to which this rule applies.
    ///
    /// Refer to [selector][google.api.DocumentationRule.selector] for syntax
    /// details.
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    /// A comma-separated list of visibility labels that apply to the `selector`.
    /// Any of the listed labels can be used to grant the visibility.
    ///
    /// If a rule has multiple labels, removing one of the labels but not all of
    /// them can break clients.
    ///
    /// Example:
    ///
    ///      visibility:
    ///        rules:
    ///        - selector: google.calendar.Calendar.EnhancedSearch
    ///          restriction: INTERNAL, PREVIEW
    ///
    /// Removing INTERNAL from this restriction will break clients that rely on
    /// this method and only had access to it through INTERNAL.
    #[prost(string, tag = "2")]
    pub restriction: ::prost::alloc::string::String,
}
/// A descriptor for defining project properties for a service. One service may
/// have many consumer projects, and the service may want to behave differently
/// depending on some properties on the project. For example, a project may be
/// associated with a school, or a business, or a government agency, a business
/// type property on the project may affect how a service responds to the client.
/// This descriptor defines which properties are allowed to be set on a project.
///
/// Example:
///
///     project_properties:
///       properties:
///       - name: NO_WATERMARK
///         type: BOOL
///         description: Allows usage of the API without watermarks.
///       - name: EXTENDED_TILE_CACHE_PERIOD
///         type: INT64
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectProperties {
    /// List of per consumer project-specific properties.
    #[prost(message, repeated, tag = "1")]
    pub properties: ::prost::alloc::vec::Vec<Property>,
}
/// Defines project properties.
///
/// API services can define properties that can be assigned to consumer projects
/// so that backends can perform response customization without having to make
/// additional calls or maintain additional storage. For example, Maps API
/// defines properties that controls map tile cache period, or whether to embed a
/// watermark in a result.
///
/// These values can be set via API producer console. Only API providers can
/// define and set these properties.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// The name of the property (a.k.a key).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The type of this property.
    #[prost(enumeration = "property::PropertyType", tag = "2")]
    pub r#type: i32,
    /// The description of the property
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Property`.
pub mod property {
    /// Supported data type of the property values
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PropertyType {
        /// The type is unspecified, and will result in an error.
        Unspecified = 0,
        /// The type is `int64`.
        Int64 = 1,
        /// The type is `bool`.
        Bool = 2,
        /// The type is `string`.
        String = 3,
        /// The type is 'double'.
        Double = 4,
    }
    impl PropertyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PropertyType::Unspecified => "UNSPECIFIED",
                PropertyType::Int64 => "INT64",
                PropertyType::Bool => "BOOL",
                PropertyType::String => "STRING",
                PropertyType::Double => "DOUBLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "INT64" => Some(Self::Int64),
                "BOOL" => Some(Self::Bool),
                "STRING" => Some(Self::String),
                "DOUBLE" => Some(Self::Double),
                _ => None,
            }
        }
    }
}
