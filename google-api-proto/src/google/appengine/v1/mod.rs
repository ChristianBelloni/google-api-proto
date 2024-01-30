/// An Instance resource is the computing unit that App Engine uses to
/// automatically scale an application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Output only. Full path to the Instance resource in the API.
    /// Example: `apps/myapp/services/default/versions/v1/instances/instance-1`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Relative name of the instance within the version.
    /// Example: `instance-1`.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Output only. App Engine release this instance is running on.
    #[prost(string, tag = "3")]
    pub app_engine_release: ::prost::alloc::string::String,
    /// Output only. Availability of the instance.
    #[prost(enumeration = "instance::Availability", tag = "4")]
    pub availability: i32,
    /// Output only. Name of the virtual machine where this instance lives. Only applicable
    /// for instances in App Engine flexible environment.
    #[prost(string, tag = "5")]
    pub vm_name: ::prost::alloc::string::String,
    /// Output only. Zone where the virtual machine is located. Only applicable for instances
    /// in App Engine flexible environment.
    #[prost(string, tag = "6")]
    pub vm_zone_name: ::prost::alloc::string::String,
    /// Output only. Virtual machine ID of this instance. Only applicable for instances in
    /// App Engine flexible environment.
    #[prost(string, tag = "7")]
    pub vm_id: ::prost::alloc::string::String,
    /// Output only. Time that this instance was started.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "8")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Number of requests since this instance was started.
    #[prost(int32, tag = "9")]
    pub requests: i32,
    /// Output only. Number of errors since this instance was started.
    #[prost(int32, tag = "10")]
    pub errors: i32,
    /// Output only. Average queries per second (QPS) over the last minute.
    #[prost(float, tag = "11")]
    pub qps: f32,
    /// Output only. Average latency (ms) over the last minute.
    #[prost(int32, tag = "12")]
    pub average_latency: i32,
    /// Output only. Total memory in use (bytes).
    #[prost(int64, tag = "13")]
    pub memory_usage: i64,
    /// Output only. Status of the virtual machine where this instance lives. Only applicable
    /// for instances in App Engine flexible environment.
    #[prost(string, tag = "14")]
    pub vm_status: ::prost::alloc::string::String,
    /// Output only. Whether this instance is in debug mode. Only applicable for instances in
    /// App Engine flexible environment.
    #[prost(bool, tag = "15")]
    pub vm_debug_enabled: bool,
    /// Output only. The IP address of this instance. Only applicable for instances in App
    /// Engine flexible environment.
    #[prost(string, tag = "16")]
    pub vm_ip: ::prost::alloc::string::String,
    /// Output only. The liveness health check of this instance. Only applicable for instances
    /// in App Engine flexible environment.
    #[prost(enumeration = "instance::liveness::LivenessState", tag = "17")]
    pub vm_liveness: i32,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Wrapper for LivenessState enum.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Liveness {}
    /// Nested message and enum types in `Liveness`.
    pub mod liveness {
        /// Liveness health check status for Flex instances.
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
        pub enum LivenessState {
            /// There is no liveness health check for the instance. Only applicable for
            /// instances in App Engine standard environment.
            Unspecified = 0,
            /// The health checking system is aware of the instance but its health is
            /// not known at the moment.
            Unknown = 1,
            /// The instance is reachable i.e. a connection to the application health
            /// checking endpoint can be established, and conforms to the requirements
            /// defined by the health check.
            Healthy = 2,
            /// The instance is reachable, but does not conform to the requirements
            /// defined by the health check.
            Unhealthy = 3,
            /// The instance is being drained. The existing connections to the instance
            /// have time to complete, but the new ones are being refused.
            Draining = 4,
            /// The instance is unreachable i.e. a connection to the application health
            /// checking endpoint cannot be established, or the server does not respond
            /// within the specified timeout.
            Timeout = 5,
        }
        impl LivenessState {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    LivenessState::Unspecified => "LIVENESS_STATE_UNSPECIFIED",
                    LivenessState::Unknown => "UNKNOWN",
                    LivenessState::Healthy => "HEALTHY",
                    LivenessState::Unhealthy => "UNHEALTHY",
                    LivenessState::Draining => "DRAINING",
                    LivenessState::Timeout => "TIMEOUT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "LIVENESS_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                    "UNKNOWN" => Some(Self::Unknown),
                    "HEALTHY" => Some(Self::Healthy),
                    "UNHEALTHY" => Some(Self::Unhealthy),
                    "DRAINING" => Some(Self::Draining),
                    "TIMEOUT" => Some(Self::Timeout),
                    _ => None,
                }
            }
        }
    }
    /// Availability of the instance.
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
    pub enum Availability {
        Unspecified = 0,
        Resident = 1,
        Dynamic = 2,
    }
    impl Availability {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Availability::Unspecified => "UNSPECIFIED",
                Availability::Resident => "RESIDENT",
                Availability::Dynamic => "DYNAMIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "RESIDENT" => Some(Self::Resident),
                "DYNAMIC" => Some(Self::Dynamic),
                _ => None,
            }
        }
    }
}
/// An Application resource contains the top-level configuration of an App
/// Engine application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Application {
    /// Full path to the Application resource in the API.
    /// Example: `apps/myapp`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Identifier of the Application resource. This identifier is equivalent
    /// to the project ID of the Google Cloud Platform project where you want to
    /// deploy your application.
    /// Example: `myapp`.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// HTTP path dispatch rules for requests to the application that do not
    /// explicitly target a service or version. Rules are order-dependent.
    /// Up to 20 dispatch rules can be supported.
    #[prost(message, repeated, tag = "3")]
    pub dispatch_rules: ::prost::alloc::vec::Vec<UrlDispatchRule>,
    /// Google Apps authentication domain that controls which users can access
    /// this application.
    ///
    /// Defaults to open access for any Google Account.
    #[prost(string, tag = "6")]
    pub auth_domain: ::prost::alloc::string::String,
    /// Location from which this application runs. Application instances
    /// run out of the data centers in the specified location, which is also where
    /// all of the application's end user content is stored.
    ///
    /// Defaults to `us-central`.
    ///
    /// View the list of
    /// [supported locations](<https://cloud.google.com/appengine/docs/locations>).
    #[prost(string, tag = "7")]
    pub location_id: ::prost::alloc::string::String,
    /// Google Cloud Storage bucket that can be used for storing files
    /// associated with this application. This bucket is associated with the
    /// application and can be used by the gcloud deployment commands.
    ///
    /// @OutputOnly
    #[prost(string, tag = "8")]
    pub code_bucket: ::prost::alloc::string::String,
    /// Cookie expiration policy for this application.
    #[prost(message, optional, tag = "9")]
    pub default_cookie_expiration: ::core::option::Option<::prost_types::Duration>,
    /// Serving status of this application.
    #[prost(enumeration = "application::ServingStatus", tag = "10")]
    pub serving_status: i32,
    /// Hostname used to reach this application, as resolved by App Engine.
    ///
    /// @OutputOnly
    #[prost(string, tag = "11")]
    pub default_hostname: ::prost::alloc::string::String,
    /// Google Cloud Storage bucket that can be used by this application to store
    /// content.
    ///
    /// @OutputOnly
    #[prost(string, tag = "12")]
    pub default_bucket: ::prost::alloc::string::String,
    /// The service account associated with the application.
    /// This is the app-level default identity. If no identity provided during
    /// create version, Admin API will fallback to this one.
    #[prost(string, tag = "13")]
    pub service_account: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "14")]
    pub iap: ::core::option::Option<application::IdentityAwareProxy>,
    /// The Google Container Registry domain used for storing managed build docker
    /// images for this application.
    #[prost(string, tag = "16")]
    pub gcr_domain: ::prost::alloc::string::String,
    /// The type of the Cloud Firestore or Cloud Datastore database associated with
    /// this application.
    #[prost(enumeration = "application::DatabaseType", tag = "17")]
    pub database_type: i32,
    /// The feature specific settings to be used in the application.
    #[prost(message, optional, tag = "18")]
    pub feature_settings: ::core::option::Option<application::FeatureSettings>,
}
/// Nested message and enum types in `Application`.
pub mod application {
    /// Identity-Aware Proxy
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentityAwareProxy {
        /// Whether the serving infrastructure will authenticate and
        /// authorize all incoming requests.
        ///
        /// If true, the `oauth2_client_id` and `oauth2_client_secret`
        /// fields must be non-empty.
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        /// OAuth2 client ID to use for the authentication flow.
        #[prost(string, tag = "2")]
        pub oauth2_client_id: ::prost::alloc::string::String,
        /// OAuth2 client secret to use for the authentication flow.
        ///
        /// For security reasons, this value cannot be retrieved via the API.
        /// Instead, the SHA-256 hash of the value is returned in the
        /// `oauth2_client_secret_sha256` field.
        ///
        /// @InputOnly
        #[prost(string, tag = "3")]
        pub oauth2_client_secret: ::prost::alloc::string::String,
        /// Hex-encoded SHA-256 hash of the client secret.
        ///
        /// @OutputOnly
        #[prost(string, tag = "4")]
        pub oauth2_client_secret_sha256: ::prost::alloc::string::String,
    }
    /// The feature specific settings to be used in the application. These define
    /// behaviors that are user configurable.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FeatureSettings {
        /// Boolean value indicating if split health checks should be used instead
        /// of the legacy health checks. At an app.yaml level, this means defaulting
        /// to 'readiness_check' and 'liveness_check' values instead of
        /// 'health_check' ones. Once the legacy 'health_check' behavior is
        /// deprecated, and this value is always true, this setting can
        /// be removed.
        #[prost(bool, tag = "1")]
        pub split_health_checks: bool,
        /// If true, use [Container-Optimized OS](<https://cloud.google.com/container-optimized-os/>)
        /// base image for VMs, rather than a base Debian image.
        #[prost(bool, tag = "2")]
        pub use_container_optimized_os: bool,
    }
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
    pub enum ServingStatus {
        /// Serving status is unspecified.
        Unspecified = 0,
        /// Application is serving.
        Serving = 1,
        /// Application has been disabled by the user.
        UserDisabled = 2,
        /// Application has been disabled by the system.
        SystemDisabled = 3,
    }
    impl ServingStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ServingStatus::Unspecified => "UNSPECIFIED",
                ServingStatus::Serving => "SERVING",
                ServingStatus::UserDisabled => "USER_DISABLED",
                ServingStatus::SystemDisabled => "SYSTEM_DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "SERVING" => Some(Self::Serving),
                "USER_DISABLED" => Some(Self::UserDisabled),
                "SYSTEM_DISABLED" => Some(Self::SystemDisabled),
                _ => None,
            }
        }
    }
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
    pub enum DatabaseType {
        /// Database type is unspecified.
        Unspecified = 0,
        /// Cloud Datastore
        CloudDatastore = 1,
        /// Cloud Firestore Native
        CloudFirestore = 2,
        /// Cloud Firestore in Datastore Mode
        CloudDatastoreCompatibility = 3,
    }
    impl DatabaseType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DatabaseType::Unspecified => "DATABASE_TYPE_UNSPECIFIED",
                DatabaseType::CloudDatastore => "CLOUD_DATASTORE",
                DatabaseType::CloudFirestore => "CLOUD_FIRESTORE",
                DatabaseType::CloudDatastoreCompatibility => {
                    "CLOUD_DATASTORE_COMPATIBILITY"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATABASE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOUD_DATASTORE" => Some(Self::CloudDatastore),
                "CLOUD_FIRESTORE" => Some(Self::CloudFirestore),
                "CLOUD_DATASTORE_COMPATIBILITY" => {
                    Some(Self::CloudDatastoreCompatibility)
                }
                _ => None,
            }
        }
    }
}
/// Rules to match an HTTP request and dispatch that request to a service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlDispatchRule {
    /// Domain name to match against. The wildcard "`*`" is supported if
    /// specified before a period: "`*.`".
    ///
    /// Defaults to matching all domains: "`*`".
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    /// Pathname within the host. Must start with a "`/`". A
    /// single "`*`" can be included at the end of the path.
    ///
    /// The sum of the lengths of the domain and path may not
    /// exceed 100 characters.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// Resource ID of a service in this application that should
    /// serve the matched request. The service must already
    /// exist. Example: `default`.
    #[prost(string, tag = "3")]
    pub service: ::prost::alloc::string::String,
}
/// A NetworkSettings resource is a container for ingress settings for a version
/// or service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkSettings {
    /// The ingress settings for version or service.
    #[prost(enumeration = "network_settings::IngressTrafficAllowed", tag = "1")]
    pub ingress_traffic_allowed: i32,
}
/// Nested message and enum types in `NetworkSettings`.
pub mod network_settings {
    /// If unspecified, INGRESS_TRAFFIC_ALLOWED_ALL will be used.
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
    pub enum IngressTrafficAllowed {
        /// Unspecified
        Unspecified = 0,
        /// Allow HTTP traffic from public and private sources.
        All = 1,
        /// Allow HTTP traffic from only private VPC sources.
        InternalOnly = 2,
        /// Allow HTTP traffic from private VPC sources and through load balancers.
        InternalAndLb = 3,
    }
    impl IngressTrafficAllowed {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IngressTrafficAllowed::Unspecified => {
                    "INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED"
                }
                IngressTrafficAllowed::All => "INGRESS_TRAFFIC_ALLOWED_ALL",
                IngressTrafficAllowed::InternalOnly => {
                    "INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY"
                }
                IngressTrafficAllowed::InternalAndLb => {
                    "INGRESS_TRAFFIC_ALLOWED_INTERNAL_AND_LB"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED" => Some(Self::Unspecified),
                "INGRESS_TRAFFIC_ALLOWED_ALL" => Some(Self::All),
                "INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY" => Some(Self::InternalOnly),
                "INGRESS_TRAFFIC_ALLOWED_INTERNAL_AND_LB" => Some(Self::InternalAndLb),
                _ => None,
            }
        }
    }
}
/// [Google Cloud Endpoints](<https://cloud.google.com/appengine/docs/python/endpoints/>)
/// configuration for API handlers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiConfigHandler {
    /// Action to take when users access resources that require
    /// authentication. Defaults to `redirect`.
    #[prost(enumeration = "AuthFailAction", tag = "1")]
    pub auth_fail_action: i32,
    /// Level of login required to access this resource. Defaults to
    /// `optional`.
    #[prost(enumeration = "LoginRequirement", tag = "2")]
    pub login: i32,
    /// Path to the script from the application root directory.
    #[prost(string, tag = "3")]
    pub script: ::prost::alloc::string::String,
    /// Security (HTTPS) enforcement for this URL.
    #[prost(enumeration = "SecurityLevel", tag = "4")]
    pub security_level: i32,
    /// URL to serve the endpoint at.
    #[prost(string, tag = "5")]
    pub url: ::prost::alloc::string::String,
}
/// Custom static error page to be served when an error occurs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorHandler {
    /// Error condition this handler applies to.
    #[prost(enumeration = "error_handler::ErrorCode", tag = "1")]
    pub error_code: i32,
    /// Static file content to be served for this error.
    #[prost(string, tag = "2")]
    pub static_file: ::prost::alloc::string::String,
    /// MIME type of file. Defaults to `text/html`.
    #[prost(string, tag = "3")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ErrorHandler`.
pub mod error_handler {
    /// Error codes.
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
    pub enum ErrorCode {
        /// Not specified. ERROR_CODE_DEFAULT is assumed.
        Unspecified = 0,
        /// Application has exceeded a resource quota.
        OverQuota = 1,
        /// Client blocked by the application's Denial of Service protection
        /// configuration.
        DosApiDenial = 2,
        /// Deadline reached before the application responds.
        Timeout = 3,
    }
    impl ErrorCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ErrorCode::Unspecified => "ERROR_CODE_UNSPECIFIED",
                ErrorCode::OverQuota => "ERROR_CODE_OVER_QUOTA",
                ErrorCode::DosApiDenial => "ERROR_CODE_DOS_API_DENIAL",
                ErrorCode::Timeout => "ERROR_CODE_TIMEOUT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ERROR_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "ERROR_CODE_OVER_QUOTA" => Some(Self::OverQuota),
                "ERROR_CODE_DOS_API_DENIAL" => Some(Self::DosApiDenial),
                "ERROR_CODE_TIMEOUT" => Some(Self::Timeout),
                _ => None,
            }
        }
    }
}
/// URL pattern and description of how the URL should be handled. App Engine can
/// handle URLs by executing application code or by serving static files
/// uploaded with the version, such as images, CSS, or JavaScript.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlMap {
    /// URL prefix. Uses regular expression syntax, which means regexp
    /// special characters must be escaped, but should not contain groupings.
    /// All URLs that begin with this prefix are handled by this handler, using the
    /// portion of the URL after the prefix as part of the file path.
    #[prost(string, tag = "1")]
    pub url_regex: ::prost::alloc::string::String,
    /// Security (HTTPS) enforcement for this URL.
    #[prost(enumeration = "SecurityLevel", tag = "5")]
    pub security_level: i32,
    /// Level of login required to access this resource. Not supported for Node.js
    /// in the App Engine standard environment.
    #[prost(enumeration = "LoginRequirement", tag = "6")]
    pub login: i32,
    /// Action to take when users access resources that require
    /// authentication. Defaults to `redirect`.
    #[prost(enumeration = "AuthFailAction", tag = "7")]
    pub auth_fail_action: i32,
    /// `30x` code to use when performing redirects for the `secure` field.
    /// Defaults to `302`.
    #[prost(enumeration = "url_map::RedirectHttpResponseCode", tag = "8")]
    pub redirect_http_response_code: i32,
    /// Type of handler for this URL pattern.
    #[prost(oneof = "url_map::HandlerType", tags = "2, 3, 4")]
    pub handler_type: ::core::option::Option<url_map::HandlerType>,
}
/// Nested message and enum types in `UrlMap`.
pub mod url_map {
    /// Redirect codes.
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
    pub enum RedirectHttpResponseCode {
        /// Not specified. `302` is assumed.
        Unspecified = 0,
        /// `301 Moved Permanently` code.
        RedirectHttpResponseCode301 = 1,
        /// `302 Moved Temporarily` code.
        RedirectHttpResponseCode302 = 2,
        /// `303 See Other` code.
        RedirectHttpResponseCode303 = 3,
        /// `307 Temporary Redirect` code.
        RedirectHttpResponseCode307 = 4,
    }
    impl RedirectHttpResponseCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RedirectHttpResponseCode::Unspecified => {
                    "REDIRECT_HTTP_RESPONSE_CODE_UNSPECIFIED"
                }
                RedirectHttpResponseCode::RedirectHttpResponseCode301 => {
                    "REDIRECT_HTTP_RESPONSE_CODE_301"
                }
                RedirectHttpResponseCode::RedirectHttpResponseCode302 => {
                    "REDIRECT_HTTP_RESPONSE_CODE_302"
                }
                RedirectHttpResponseCode::RedirectHttpResponseCode303 => {
                    "REDIRECT_HTTP_RESPONSE_CODE_303"
                }
                RedirectHttpResponseCode::RedirectHttpResponseCode307 => {
                    "REDIRECT_HTTP_RESPONSE_CODE_307"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REDIRECT_HTTP_RESPONSE_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "REDIRECT_HTTP_RESPONSE_CODE_301" => {
                    Some(Self::RedirectHttpResponseCode301)
                }
                "REDIRECT_HTTP_RESPONSE_CODE_302" => {
                    Some(Self::RedirectHttpResponseCode302)
                }
                "REDIRECT_HTTP_RESPONSE_CODE_303" => {
                    Some(Self::RedirectHttpResponseCode303)
                }
                "REDIRECT_HTTP_RESPONSE_CODE_307" => {
                    Some(Self::RedirectHttpResponseCode307)
                }
                _ => None,
            }
        }
    }
    /// Type of handler for this URL pattern.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HandlerType {
        /// Returns the contents of a file, such as an image, as the response.
        #[prost(message, tag = "2")]
        StaticFiles(super::StaticFilesHandler),
        /// Executes a script to handle the requests that match this URL
        /// pattern. Only the `auto` value is supported for Node.js in the
        /// App Engine standard environment, for example `"script": "auto"`.
        #[prost(message, tag = "3")]
        Script(super::ScriptHandler),
        /// Uses API Endpoints to handle requests.
        #[prost(message, tag = "4")]
        ApiEndpoint(super::ApiEndpointHandler),
    }
}
/// Files served directly to the user for a given URL, such as images, CSS
/// stylesheets, or JavaScript source files. Static file handlers describe which
/// files in the application directory are static files, and which URLs serve
/// them.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticFilesHandler {
    /// Path to the static files matched by the URL pattern, from the
    /// application root directory. The path can refer to text matched in groupings
    /// in the URL pattern.
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// Regular expression that matches the file paths for all files that should be
    /// referenced by this handler.
    #[prost(string, tag = "2")]
    pub upload_path_regex: ::prost::alloc::string::String,
    /// HTTP headers to use for all responses from these URLs.
    #[prost(btree_map = "string, string", tag = "3")]
    pub http_headers: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// MIME type used to serve all files served by this handler.
    ///
    /// Defaults to file-specific MIME types, which are derived from each file's
    /// filename extension.
    #[prost(string, tag = "4")]
    pub mime_type: ::prost::alloc::string::String,
    /// Time a static file served by this handler should be cached
    /// by web proxies and browsers.
    #[prost(message, optional, tag = "5")]
    pub expiration: ::core::option::Option<::prost_types::Duration>,
    /// Whether this handler should match the request if the file
    /// referenced by the handler does not exist.
    #[prost(bool, tag = "6")]
    pub require_matching_file: bool,
    /// Whether files should also be uploaded as code data. By default, files
    /// declared in static file handlers are uploaded as static
    /// data and are only served to end users; they cannot be read by the
    /// application. If enabled, uploads are charged against both your code and
    /// static data storage resource quotas.
    #[prost(bool, tag = "7")]
    pub application_readable: bool,
}
/// Executes a script to handle the request that matches the URL pattern.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScriptHandler {
    /// Path to the script from the application root directory.
    #[prost(string, tag = "1")]
    pub script_path: ::prost::alloc::string::String,
}
/// Uses Google Cloud Endpoints to handle requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiEndpointHandler {
    /// Path to the script from the application root directory.
    #[prost(string, tag = "1")]
    pub script_path: ::prost::alloc::string::String,
}
/// Health checking configuration for VM instances. Unhealthy instances
/// are killed and replaced with new instances. Only applicable for
/// instances in App Engine flexible environment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheck {
    /// Whether to explicitly disable health checks for this instance.
    #[prost(bool, tag = "1")]
    pub disable_health_check: bool,
    /// Host header to send when performing an HTTP health check.
    /// Example: "myapp.appspot.com"
    #[prost(string, tag = "2")]
    pub host: ::prost::alloc::string::String,
    /// Number of consecutive successful health checks required before receiving
    /// traffic.
    #[prost(uint32, tag = "3")]
    pub healthy_threshold: u32,
    /// Number of consecutive failed health checks required before removing
    /// traffic.
    #[prost(uint32, tag = "4")]
    pub unhealthy_threshold: u32,
    /// Number of consecutive failed health checks required before an instance is
    /// restarted.
    #[prost(uint32, tag = "5")]
    pub restart_threshold: u32,
    /// Interval between health checks.
    #[prost(message, optional, tag = "6")]
    pub check_interval: ::core::option::Option<::prost_types::Duration>,
    /// Time before the health check is considered failed.
    #[prost(message, optional, tag = "7")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
}
/// Readiness checking configuration for VM instances. Unhealthy instances
/// are removed from traffic rotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadinessCheck {
    /// The request path.
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// Host header to send when performing a HTTP Readiness check.
    /// Example: "myapp.appspot.com"
    #[prost(string, tag = "2")]
    pub host: ::prost::alloc::string::String,
    /// Number of consecutive failed checks required before removing
    /// traffic.
    #[prost(uint32, tag = "3")]
    pub failure_threshold: u32,
    /// Number of consecutive successful checks required before receiving
    /// traffic.
    #[prost(uint32, tag = "4")]
    pub success_threshold: u32,
    /// Interval between health checks.
    #[prost(message, optional, tag = "5")]
    pub check_interval: ::core::option::Option<::prost_types::Duration>,
    /// Time before the check is considered failed.
    #[prost(message, optional, tag = "6")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// A maximum time limit on application initialization, measured from moment
    /// the application successfully replies to a healthcheck until it is ready to
    /// serve traffic.
    #[prost(message, optional, tag = "7")]
    pub app_start_timeout: ::core::option::Option<::prost_types::Duration>,
}
/// Health checking configuration for VM instances. Unhealthy instances
/// are killed and replaced with new instances.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LivenessCheck {
    /// The request path.
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// Host header to send when performing a HTTP Liveness check.
    /// Example: "myapp.appspot.com"
    #[prost(string, tag = "2")]
    pub host: ::prost::alloc::string::String,
    /// Number of consecutive failed checks required before considering the
    /// VM unhealthy.
    #[prost(uint32, tag = "3")]
    pub failure_threshold: u32,
    /// Number of consecutive successful checks required before considering
    /// the VM healthy.
    #[prost(uint32, tag = "4")]
    pub success_threshold: u32,
    /// Interval between health checks.
    #[prost(message, optional, tag = "5")]
    pub check_interval: ::core::option::Option<::prost_types::Duration>,
    /// Time before the check is considered failed.
    #[prost(message, optional, tag = "6")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// The initial delay before starting to execute the checks.
    #[prost(message, optional, tag = "7")]
    pub initial_delay: ::core::option::Option<::prost_types::Duration>,
}
/// Third-party Python runtime library that is required by the application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Library {
    /// Name of the library. Example: "django".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Version of the library to select, or "latest".
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// Actions to take when the user is not logged in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthFailAction {
    /// Not specified. `AUTH_FAIL_ACTION_REDIRECT` is assumed.
    Unspecified = 0,
    /// Redirects user to "accounts.google.com". The user is redirected back to the
    /// application URL after signing in or creating an account.
    Redirect = 1,
    /// Rejects request with a `401` HTTP status code and an error
    /// message.
    Unauthorized = 2,
}
impl AuthFailAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthFailAction::Unspecified => "AUTH_FAIL_ACTION_UNSPECIFIED",
            AuthFailAction::Redirect => "AUTH_FAIL_ACTION_REDIRECT",
            AuthFailAction::Unauthorized => "AUTH_FAIL_ACTION_UNAUTHORIZED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTH_FAIL_ACTION_UNSPECIFIED" => Some(Self::Unspecified),
            "AUTH_FAIL_ACTION_REDIRECT" => Some(Self::Redirect),
            "AUTH_FAIL_ACTION_UNAUTHORIZED" => Some(Self::Unauthorized),
            _ => None,
        }
    }
}
/// Methods to restrict access to a URL based on login status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoginRequirement {
    /// Not specified. `LOGIN_OPTIONAL` is assumed.
    LoginUnspecified = 0,
    /// Does not require that the user is signed in.
    LoginOptional = 1,
    /// If the user is not signed in, the `auth_fail_action` is taken.
    /// In addition, if the user is not an administrator for the
    /// application, they are given an error message regardless of
    /// `auth_fail_action`. If the user is an administrator, the handler
    /// proceeds.
    LoginAdmin = 2,
    /// If the user has signed in, the handler proceeds normally. Otherwise, the
    /// auth_fail_action is taken.
    LoginRequired = 3,
}
impl LoginRequirement {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LoginRequirement::LoginUnspecified => "LOGIN_UNSPECIFIED",
            LoginRequirement::LoginOptional => "LOGIN_OPTIONAL",
            LoginRequirement::LoginAdmin => "LOGIN_ADMIN",
            LoginRequirement::LoginRequired => "LOGIN_REQUIRED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOGIN_UNSPECIFIED" => Some(Self::LoginUnspecified),
            "LOGIN_OPTIONAL" => Some(Self::LoginOptional),
            "LOGIN_ADMIN" => Some(Self::LoginAdmin),
            "LOGIN_REQUIRED" => Some(Self::LoginRequired),
            _ => None,
        }
    }
}
/// Methods to enforce security (HTTPS) on a URL.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityLevel {
    /// Not specified.
    SecureUnspecified = 0,
    /// Requests for a URL that match this handler that use HTTPS are automatically
    /// redirected to the HTTP equivalent URL.
    SecureNever = 1,
    /// Both HTTP and HTTPS requests with URLs that match the handler succeed
    /// without redirects. The application can examine the request to determine
    /// which protocol was used and respond accordingly.
    SecureOptional = 2,
    /// Requests for a URL that match this handler that do not use HTTPS are
    /// automatically redirected to the HTTPS URL with the same path. Query
    /// parameters are reserved for the redirect.
    SecureAlways = 3,
}
impl SecurityLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SecurityLevel::SecureUnspecified => "SECURE_UNSPECIFIED",
            SecurityLevel::SecureNever => "SECURE_NEVER",
            SecurityLevel::SecureOptional => "SECURE_OPTIONAL",
            SecurityLevel::SecureAlways => "SECURE_ALWAYS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SECURE_UNSPECIFIED" => Some(Self::SecureUnspecified),
            "SECURE_NEVER" => Some(Self::SecureNever),
            "SECURE_OPTIONAL" => Some(Self::SecureOptional),
            "SECURE_ALWAYS" => Some(Self::SecureAlways),
            _ => None,
        }
    }
}
/// Code and application artifacts used to deploy a version to App Engine.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    /// Manifest of the files stored in Google Cloud Storage that are included
    /// as part of this version. All files must be readable using the
    /// credentials supplied with this call.
    #[prost(btree_map = "string, message", tag = "1")]
    pub files: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        FileInfo,
    >,
    /// The Docker image for the container that runs the version.
    /// Only applicable for instances running in the App Engine flexible environment.
    #[prost(message, optional, tag = "2")]
    pub container: ::core::option::Option<ContainerInfo>,
    /// The zip file for this deployment, if this is a zip deployment.
    #[prost(message, optional, tag = "3")]
    pub zip: ::core::option::Option<ZipInfo>,
    /// Options for any Google Cloud Build builds created as a part of this
    /// deployment.
    ///
    /// These options will only be used if a new build is created, such as when
    /// deploying to the App Engine flexible environment using files or zip.
    #[prost(message, optional, tag = "6")]
    pub cloud_build_options: ::core::option::Option<CloudBuildOptions>,
}
/// Single source file that is part of the version to be deployed. Each source
/// file that is deployed must be specified separately.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    /// URL source to use to fetch this file. Must be a URL to a resource in
    /// Google Cloud Storage in the form
    /// 'http(s)://storage.googleapis.com/\<bucket\>/\<object\>'.
    #[prost(string, tag = "1")]
    pub source_url: ::prost::alloc::string::String,
    /// The SHA1 hash of the file, in hex.
    #[prost(string, tag = "2")]
    pub sha1_sum: ::prost::alloc::string::String,
    /// The MIME type of the file.
    ///
    /// Defaults to the value from Google Cloud Storage.
    #[prost(string, tag = "3")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Docker image that is used to create a container and start a VM instance for
/// the version that you deploy. Only applicable for instances running in the App
/// Engine flexible environment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerInfo {
    /// URI to the hosted container image in Google Container Registry. The URI
    /// must be fully qualified and include a tag or digest.
    /// Examples: "gcr.io/my-project/image:tag" or "gcr.io/my-project/image@digest"
    #[prost(string, tag = "1")]
    pub image: ::prost::alloc::string::String,
}
/// Options for the build operations performed as a part of the version
/// deployment. Only applicable for App Engine flexible environment when creating
/// a version using source code directly.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudBuildOptions {
    /// Path to the yaml file used in deployment, used to determine runtime
    /// configuration details.
    ///
    /// Required for flexible environment builds.
    ///
    /// See <https://cloud.google.com/appengine/docs/standard/python/config/appref>
    /// for more details.
    #[prost(string, tag = "1")]
    pub app_yaml_path: ::prost::alloc::string::String,
    /// The Cloud Build timeout used as part of any dependent builds performed by
    /// version creation. Defaults to 10 minutes.
    #[prost(message, optional, tag = "2")]
    pub cloud_build_timeout: ::core::option::Option<::prost_types::Duration>,
}
/// The zip file information for a zip deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZipInfo {
    /// URL of the zip file to deploy from. Must be a URL to a resource in
    /// Google Cloud Storage in the form
    /// 'http(s)://storage.googleapis.com/\<bucket\>/\<object\>'.
    #[prost(string, tag = "3")]
    pub source_url: ::prost::alloc::string::String,
    /// An estimate of the number of files in a zip for a zip deployment.
    /// If set, must be greater than or equal to the actual number of files.
    /// Used for optimizing performance; if not provided, deployment may be slow.
    #[prost(int32, tag = "4")]
    pub files_count: i32,
}
/// A Version resource is a specific set of source code and configuration files
/// that are deployed into a service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// Full path to the Version resource in the API.  Example:
    /// `apps/myapp/services/default/versions/v1`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Relative name of the version within the service.  Example: `v1`.
    /// Version names can contain only lowercase letters, numbers, or hyphens.
    /// Reserved names: "default", "latest", and any name with the prefix "ah-".
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Before an application can receive email or XMPP messages, the application
    /// must be configured to enable the service.
    #[prost(enumeration = "InboundServiceType", repeated, tag = "6")]
    pub inbound_services: ::prost::alloc::vec::Vec<i32>,
    /// Instance class that is used to run this version. Valid values are:
    ///
    /// * AutomaticScaling: `F1`, `F2`, `F4`, `F4_1G`
    /// * ManualScaling or BasicScaling: `B1`, `B2`, `B4`, `B8`, `B4_1G`
    ///
    /// Defaults to `F1` for AutomaticScaling and `B1` for ManualScaling or
    /// BasicScaling.
    #[prost(string, tag = "7")]
    pub instance_class: ::prost::alloc::string::String,
    /// Extra network settings.
    /// Only applicable in the App Engine flexible environment.
    #[prost(message, optional, tag = "8")]
    pub network: ::core::option::Option<Network>,
    /// The Google Compute Engine zones that are supported by this version in the
    /// App Engine flexible environment. Deprecated.
    #[prost(string, repeated, tag = "118")]
    pub zones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Machine resources for this version.
    /// Only applicable in the App Engine flexible environment.
    #[prost(message, optional, tag = "9")]
    pub resources: ::core::option::Option<Resources>,
    /// Desired runtime. Example: `python27`.
    #[prost(string, tag = "10")]
    pub runtime: ::prost::alloc::string::String,
    /// The channel of the runtime to use. Only available for some
    /// runtimes. Defaults to the `default` channel.
    #[prost(string, tag = "117")]
    pub runtime_channel: ::prost::alloc::string::String,
    /// Whether multiple requests can be dispatched to this version at once.
    #[prost(bool, tag = "11")]
    pub threadsafe: bool,
    /// Whether to deploy this version in a container on a virtual machine.
    #[prost(bool, tag = "12")]
    pub vm: bool,
    /// Allows App Engine second generation runtimes to access the legacy bundled
    /// services.
    #[prost(bool, tag = "128")]
    pub app_engine_apis: bool,
    /// Metadata settings that are supplied to this version to enable
    /// beta runtime features.
    #[prost(btree_map = "string, string", tag = "13")]
    pub beta_settings: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// App Engine execution environment for this version.
    ///
    /// Defaults to `standard`.
    #[prost(string, tag = "14")]
    pub env: ::prost::alloc::string::String,
    /// Current serving status of this version. Only the versions with a
    /// `SERVING` status create instances and can be billed.
    ///
    /// `SERVING_STATUS_UNSPECIFIED` is an invalid value. Defaults to `SERVING`.
    #[prost(enumeration = "ServingStatus", tag = "15")]
    pub serving_status: i32,
    /// Email address of the user who created this version.
    ///
    /// @OutputOnly
    #[prost(string, tag = "16")]
    pub created_by: ::prost::alloc::string::String,
    /// Time that this version was created.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "17")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Total size in bytes of all the files that are included in this version
    /// and currently hosted on the App Engine disk.
    ///
    /// @OutputOnly
    #[prost(int64, tag = "18")]
    pub disk_usage_bytes: i64,
    /// The version of the API in the given runtime environment. Please see the
    /// app.yaml reference for valid values at
    /// <https://cloud.google.com/appengine/docs/standard/<language>/config/appref>
    #[prost(string, tag = "21")]
    pub runtime_api_version: ::prost::alloc::string::String,
    /// The path or name of the app's main executable.
    #[prost(string, tag = "22")]
    pub runtime_main_executable_path: ::prost::alloc::string::String,
    /// The identity that the deployed version will run as.
    /// Admin API will use the App Engine Appspot service account as default if
    /// this field is neither provided in app.yaml file nor through CLI flag.
    #[prost(string, tag = "127")]
    pub service_account: ::prost::alloc::string::String,
    /// An ordered list of URL-matching patterns that should be applied to incoming
    /// requests. The first matching URL handles the request and other request
    /// handlers are not attempted.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, repeated, tag = "100")]
    pub handlers: ::prost::alloc::vec::Vec<UrlMap>,
    /// Custom static error pages. Limited to 10KB per page.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, repeated, tag = "101")]
    pub error_handlers: ::prost::alloc::vec::Vec<ErrorHandler>,
    /// Configuration for third-party Python runtime libraries that are required
    /// by the application.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, repeated, tag = "102")]
    pub libraries: ::prost::alloc::vec::Vec<Library>,
    /// Serving configuration for
    /// [Google Cloud Endpoints](<https://cloud.google.com/appengine/docs/python/endpoints/>).
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "103")]
    pub api_config: ::core::option::Option<ApiConfigHandler>,
    /// Environment variables available to the application.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(btree_map = "string, string", tag = "104")]
    pub env_variables: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Environment variables available to the build environment.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(btree_map = "string, string", tag = "125")]
    pub build_env_variables: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Duration that static files should be cached by web proxies and browsers.
    /// Only applicable if the corresponding
    /// [StaticFilesHandler](<https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StaticFilesHandler>)
    /// does not specify its own expiration time.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "105")]
    pub default_expiration: ::core::option::Option<::prost_types::Duration>,
    /// Configures health checking for instances. Unhealthy instances are
    /// stopped and replaced with new instances.
    /// Only applicable in the App Engine flexible environment.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "106")]
    pub health_check: ::core::option::Option<HealthCheck>,
    /// Configures readiness health checking for instances.
    /// Unhealthy instances are not put into the backend traffic rotation.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "112")]
    pub readiness_check: ::core::option::Option<ReadinessCheck>,
    /// Configures liveness health checking for instances.
    /// Unhealthy instances are stopped and replaced with new instances
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "113")]
    pub liveness_check: ::core::option::Option<LivenessCheck>,
    /// Files that match this pattern will not be built into this version.
    /// Only applicable for Go runtimes.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(string, tag = "107")]
    pub nobuild_files_regex: ::prost::alloc::string::String,
    /// Code and application artifacts that make up this version.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "108")]
    pub deployment: ::core::option::Option<Deployment>,
    /// Serving URL for this version. Example:
    /// "<https://myversion-dot-myservice-dot-myapp.appspot.com">
    ///
    /// @OutputOnly
    #[prost(string, tag = "109")]
    pub version_url: ::prost::alloc::string::String,
    /// Cloud Endpoints configuration.
    ///
    /// If endpoints_api_service is set, the Cloud Endpoints Extensible Service
    /// Proxy will be provided to serve the API implemented by the app.
    #[prost(message, optional, tag = "110")]
    pub endpoints_api_service: ::core::option::Option<EndpointsApiService>,
    /// The entrypoint for the application.
    #[prost(message, optional, tag = "122")]
    pub entrypoint: ::core::option::Option<Entrypoint>,
    /// Enables VPC connectivity for standard apps.
    #[prost(message, optional, tag = "121")]
    pub vpc_access_connector: ::core::option::Option<VpcAccessConnector>,
    /// Controls how instances are created, scaled, and reaped.
    ///
    /// Defaults to `AutomaticScaling`.
    #[prost(oneof = "version::Scaling", tags = "3, 4, 5")]
    pub scaling: ::core::option::Option<version::Scaling>,
}
/// Nested message and enum types in `Version`.
pub mod version {
    /// Controls how instances are created, scaled, and reaped.
    ///
    /// Defaults to `AutomaticScaling`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scaling {
        /// Automatic scaling is based on request rate, response latencies, and other
        /// application metrics. Instances are dynamically created and destroyed as
        /// needed in order to handle traffic.
        #[prost(message, tag = "3")]
        AutomaticScaling(super::AutomaticScaling),
        /// A service with basic scaling will create an instance when the application
        /// receives a request. The instance will be turned down when the app becomes
        /// idle. Basic scaling is ideal for work that is intermittent or driven by
        /// user activity.
        #[prost(message, tag = "4")]
        BasicScaling(super::BasicScaling),
        /// A service with manual scaling runs continuously, allowing you to perform
        /// complex initialization and rely on the state of its memory over time.
        /// Manually scaled versions are sometimes referred to as "backends".
        #[prost(message, tag = "5")]
        ManualScaling(super::ManualScaling),
    }
}
/// [Cloud Endpoints](<https://cloud.google.com/endpoints>) configuration.
/// The Endpoints API Service provides tooling for serving Open API and gRPC
/// endpoints via an NGINX proxy. Only valid for App Engine Flexible environment
/// deployments.
///
/// The fields here refer to the name and configuration ID of a "service"
/// resource in the [Service Management API](<https://cloud.google.com/service-management/overview>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointsApiService {
    /// Endpoints service name which is the name of the "service" resource in the
    /// Service Management API. For example "myapi.endpoints.myproject.cloud.goog"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Endpoints service configuration ID as specified by the Service Management
    /// API. For example "2016-09-19r1".
    ///
    /// By default, the rollout strategy for Endpoints is `RolloutStrategy.FIXED`.
    /// This means that Endpoints starts up with a particular configuration ID.
    /// When a new configuration is rolled out, Endpoints must be given the new
    /// configuration ID. The `config_id` field is used to give the configuration
    /// ID and is required in this case.
    ///
    /// Endpoints also has a rollout strategy called `RolloutStrategy.MANAGED`.
    /// When using this, Endpoints fetches the latest configuration and does not
    /// need the configuration ID. In this case, `config_id` must be omitted.
    #[prost(string, tag = "2")]
    pub config_id: ::prost::alloc::string::String,
    /// Endpoints rollout strategy. If `FIXED`, `config_id` must be specified. If
    /// `MANAGED`, `config_id` must be omitted.
    #[prost(enumeration = "endpoints_api_service::RolloutStrategy", tag = "3")]
    pub rollout_strategy: i32,
    /// Enable or disable trace sampling. By default, this is set to false for
    /// enabled.
    #[prost(bool, tag = "4")]
    pub disable_trace_sampling: bool,
}
/// Nested message and enum types in `EndpointsApiService`.
pub mod endpoints_api_service {
    /// Available rollout strategies.
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
    pub enum RolloutStrategy {
        /// Not specified. Defaults to `FIXED`.
        UnspecifiedRolloutStrategy = 0,
        /// Endpoints service configuration ID will be fixed to the configuration ID
        /// specified by `config_id`.
        Fixed = 1,
        /// Endpoints service configuration ID will be updated with each rollout.
        Managed = 2,
    }
    impl RolloutStrategy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RolloutStrategy::UnspecifiedRolloutStrategy => {
                    "UNSPECIFIED_ROLLOUT_STRATEGY"
                }
                RolloutStrategy::Fixed => "FIXED",
                RolloutStrategy::Managed => "MANAGED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED_ROLLOUT_STRATEGY" => Some(Self::UnspecifiedRolloutStrategy),
                "FIXED" => Some(Self::Fixed),
                "MANAGED" => Some(Self::Managed),
                _ => None,
            }
        }
    }
}
/// Automatic scaling is based on request rate, response latencies, and other
/// application metrics.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomaticScaling {
    /// The time period that the
    /// [Autoscaler](<https://cloud.google.com/compute/docs/autoscaler/>)
    /// should wait before it starts collecting information from a new instance.
    /// This prevents the autoscaler from collecting information when the instance
    /// is initializing, during which the collected usage would not be reliable.
    /// Only applicable in the App Engine flexible environment.
    #[prost(message, optional, tag = "1")]
    pub cool_down_period: ::core::option::Option<::prost_types::Duration>,
    /// Target scaling by CPU usage.
    #[prost(message, optional, tag = "2")]
    pub cpu_utilization: ::core::option::Option<CpuUtilization>,
    /// Number of concurrent requests an automatic scaling instance can accept
    /// before the scheduler spawns a new instance.
    ///
    /// Defaults to a runtime-specific value.
    #[prost(int32, tag = "3")]
    pub max_concurrent_requests: i32,
    /// Maximum number of idle instances that should be maintained for this
    /// version.
    #[prost(int32, tag = "4")]
    pub max_idle_instances: i32,
    /// Maximum number of instances that should be started to handle requests for
    /// this version.
    #[prost(int32, tag = "5")]
    pub max_total_instances: i32,
    /// Maximum amount of time that a request should wait in the pending queue
    /// before starting a new instance to handle it.
    #[prost(message, optional, tag = "6")]
    pub max_pending_latency: ::core::option::Option<::prost_types::Duration>,
    /// Minimum number of idle instances that should be maintained for
    /// this version. Only applicable for the default version of a service.
    #[prost(int32, tag = "7")]
    pub min_idle_instances: i32,
    /// Minimum number of running instances that should be maintained for this
    /// version.
    #[prost(int32, tag = "8")]
    pub min_total_instances: i32,
    /// Minimum amount of time a request should wait in the pending queue before
    /// starting a new instance to handle it.
    #[prost(message, optional, tag = "9")]
    pub min_pending_latency: ::core::option::Option<::prost_types::Duration>,
    /// Target scaling by request utilization.
    #[prost(message, optional, tag = "10")]
    pub request_utilization: ::core::option::Option<RequestUtilization>,
    /// Target scaling by disk usage.
    #[prost(message, optional, tag = "11")]
    pub disk_utilization: ::core::option::Option<DiskUtilization>,
    /// Target scaling by network usage.
    #[prost(message, optional, tag = "12")]
    pub network_utilization: ::core::option::Option<NetworkUtilization>,
    /// Scheduler settings for standard environment.
    #[prost(message, optional, tag = "20")]
    pub standard_scheduler_settings: ::core::option::Option<StandardSchedulerSettings>,
}
/// A service with basic scaling will create an instance when the application
/// receives a request. The instance will be turned down when the app becomes
/// idle. Basic scaling is ideal for work that is intermittent or driven by
/// user activity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicScaling {
    /// Duration of time after the last request that an instance must wait before
    /// the instance is shut down.
    #[prost(message, optional, tag = "1")]
    pub idle_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Maximum number of instances to create for this version.
    #[prost(int32, tag = "2")]
    pub max_instances: i32,
}
/// A service with manual scaling runs continuously, allowing you to perform
/// complex initialization and rely on the state of its memory over time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualScaling {
    /// Number of instances to assign to the service at the start. This number
    /// can later be altered by using the
    /// [Modules API](<https://cloud.google.com/appengine/docs/python/modules/functions>)
    /// `set_num_instances()` function.
    #[prost(int32, tag = "1")]
    pub instances: i32,
}
/// Target scaling by CPU usage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpuUtilization {
    /// Period of time over which CPU utilization is calculated.
    #[prost(message, optional, tag = "1")]
    pub aggregation_window_length: ::core::option::Option<::prost_types::Duration>,
    /// Target CPU utilization ratio to maintain when scaling. Must be between 0
    /// and 1.
    #[prost(double, tag = "2")]
    pub target_utilization: f64,
}
/// Target scaling by request utilization.
/// Only applicable in the App Engine flexible environment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestUtilization {
    /// Target requests per second.
    #[prost(int32, tag = "1")]
    pub target_request_count_per_second: i32,
    /// Target number of concurrent requests.
    #[prost(int32, tag = "2")]
    pub target_concurrent_requests: i32,
}
/// Target scaling by disk usage.
/// Only applicable in the App Engine flexible environment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskUtilization {
    /// Target bytes written per second.
    #[prost(int32, tag = "14")]
    pub target_write_bytes_per_second: i32,
    /// Target ops written per second.
    #[prost(int32, tag = "15")]
    pub target_write_ops_per_second: i32,
    /// Target bytes read per second.
    #[prost(int32, tag = "16")]
    pub target_read_bytes_per_second: i32,
    /// Target ops read per seconds.
    #[prost(int32, tag = "17")]
    pub target_read_ops_per_second: i32,
}
/// Target scaling by network usage.
/// Only applicable in the App Engine flexible environment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkUtilization {
    /// Target bytes sent per second.
    #[prost(int32, tag = "1")]
    pub target_sent_bytes_per_second: i32,
    /// Target packets sent per second.
    #[prost(int32, tag = "11")]
    pub target_sent_packets_per_second: i32,
    /// Target bytes received per second.
    #[prost(int32, tag = "12")]
    pub target_received_bytes_per_second: i32,
    /// Target packets received per second.
    #[prost(int32, tag = "13")]
    pub target_received_packets_per_second: i32,
}
/// Scheduler settings for standard environment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardSchedulerSettings {
    /// Target CPU utilization ratio to maintain when scaling.
    #[prost(double, tag = "1")]
    pub target_cpu_utilization: f64,
    /// Target throughput utilization ratio to maintain when scaling
    #[prost(double, tag = "2")]
    pub target_throughput_utilization: f64,
    /// Minimum number of instances to run for this version. Set to zero to disable
    /// `min_instances` configuration.
    #[prost(int32, tag = "3")]
    pub min_instances: i32,
    /// Maximum number of instances to run for this version. Set to zero to disable
    /// `max_instances` configuration.
    #[prost(int32, tag = "4")]
    pub max_instances: i32,
}
/// Extra network settings.
/// Only applicable in the App Engine flexible environment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    /// List of ports, or port pairs, to forward from the virtual machine to the
    /// application container.
    /// Only applicable in the App Engine flexible environment.
    #[prost(string, repeated, tag = "1")]
    pub forwarded_ports: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Tag to apply to the instance during creation.
    /// Only applicable in the App Engine flexible environment.
    #[prost(string, tag = "2")]
    pub instance_tag: ::prost::alloc::string::String,
    /// Google Compute Engine network where the virtual machines are created.
    /// Specify the short name, not the resource path.
    ///
    /// Defaults to `default`.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Google Cloud Platform sub-network where the virtual machines are created.
    /// Specify the short name, not the resource path.
    ///
    /// If a subnetwork name is specified, a network name will also be required
    /// unless it is for the default network.
    ///
    /// * If the network that the instance is being created in is a Legacy network,
    /// then the IP address is allocated from the IPv4Range.
    /// * If the network that the instance is being created in is an auto Subnet
    /// Mode Network, then only network name should be specified (not the
    /// subnetwork_name) and the IP address is created from the IPCidrRange of the
    /// subnetwork that exists in that zone for that network.
    /// * If the network that the instance is being created in is a custom Subnet
    /// Mode Network, then the subnetwork_name must be specified and the
    /// IP address is created from the IPCidrRange of the subnetwork.
    ///
    /// If specified, the subnetwork must exist in the same region as the
    /// App Engine flexible environment application.
    #[prost(string, tag = "4")]
    pub subnetwork_name: ::prost::alloc::string::String,
    /// Enable session affinity.
    /// Only applicable in the App Engine flexible environment.
    #[prost(bool, tag = "5")]
    pub session_affinity: bool,
}
/// Volumes mounted within the app container.
/// Only applicable in the App Engine flexible environment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    /// Unique name for the volume.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Underlying volume type, e.g. 'tmpfs'.
    #[prost(string, tag = "2")]
    pub volume_type: ::prost::alloc::string::String,
    /// Volume size in gigabytes.
    #[prost(double, tag = "3")]
    pub size_gb: f64,
}
/// Machine resources for a version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resources {
    /// Number of CPU cores needed.
    #[prost(double, tag = "1")]
    pub cpu: f64,
    /// Disk size (GB) needed.
    #[prost(double, tag = "2")]
    pub disk_gb: f64,
    /// Memory (GB) needed.
    #[prost(double, tag = "3")]
    pub memory_gb: f64,
    /// User specified volumes.
    #[prost(message, repeated, tag = "4")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// The name of the encryption key that is stored in Google Cloud KMS.
    /// Only should be used by Cloud Composer to encrypt the vm disk
    #[prost(string, tag = "5")]
    pub kms_key_reference: ::prost::alloc::string::String,
}
/// VPC access connector specification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpcAccessConnector {
    /// Full Serverless VPC Access Connector name e.g.
    /// /projects/my-project/locations/us-central1/connectors/c1.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The egress setting for the connector, controlling what traffic is diverted
    /// through it.
    #[prost(enumeration = "vpc_access_connector::EgressSetting", tag = "2")]
    pub egress_setting: i32,
}
/// Nested message and enum types in `VpcAccessConnector`.
pub mod vpc_access_connector {
    /// Available egress settings.
    ///
    /// This controls what traffic is diverted through the VPC Access Connector
    /// resource. By default PRIVATE_IP_RANGES will be used.
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
    pub enum EgressSetting {
        Unspecified = 0,
        /// Force the use of VPC Access for all egress traffic from the function.
        AllTraffic = 1,
        /// Use the VPC Access Connector for private IP space from RFC1918.
        PrivateIpRanges = 2,
    }
    impl EgressSetting {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EgressSetting::Unspecified => "EGRESS_SETTING_UNSPECIFIED",
                EgressSetting::AllTraffic => "ALL_TRAFFIC",
                EgressSetting::PrivateIpRanges => "PRIVATE_IP_RANGES",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EGRESS_SETTING_UNSPECIFIED" => Some(Self::Unspecified),
                "ALL_TRAFFIC" => Some(Self::AllTraffic),
                "PRIVATE_IP_RANGES" => Some(Self::PrivateIpRanges),
                _ => None,
            }
        }
    }
}
/// The entrypoint for the application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entrypoint {
    /// The command to run.
    #[prost(oneof = "entrypoint::Command", tags = "1")]
    pub command: ::core::option::Option<entrypoint::Command>,
}
/// Nested message and enum types in `Entrypoint`.
pub mod entrypoint {
    /// The command to run.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        /// The format should be a shell command that can be fed to `bash -c`.
        #[prost(string, tag = "1")]
        Shell(::prost::alloc::string::String),
    }
}
/// Available inbound services.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InboundServiceType {
    /// Not specified.
    InboundServiceUnspecified = 0,
    /// Allows an application to receive mail.
    InboundServiceMail = 1,
    /// Allows an application to receive email-bound notifications.
    InboundServiceMailBounce = 2,
    /// Allows an application to receive error stanzas.
    InboundServiceXmppError = 3,
    /// Allows an application to receive instant messages.
    InboundServiceXmppMessage = 4,
    /// Allows an application to receive user subscription POSTs.
    InboundServiceXmppSubscribe = 5,
    /// Allows an application to receive a user's chat presence.
    InboundServiceXmppPresence = 6,
    /// Registers an application for notifications when a client connects or
    /// disconnects from a channel.
    InboundServiceChannelPresence = 7,
    /// Enables warmup requests.
    InboundServiceWarmup = 9,
}
impl InboundServiceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InboundServiceType::InboundServiceUnspecified => {
                "INBOUND_SERVICE_UNSPECIFIED"
            }
            InboundServiceType::InboundServiceMail => "INBOUND_SERVICE_MAIL",
            InboundServiceType::InboundServiceMailBounce => "INBOUND_SERVICE_MAIL_BOUNCE",
            InboundServiceType::InboundServiceXmppError => "INBOUND_SERVICE_XMPP_ERROR",
            InboundServiceType::InboundServiceXmppMessage => {
                "INBOUND_SERVICE_XMPP_MESSAGE"
            }
            InboundServiceType::InboundServiceXmppSubscribe => {
                "INBOUND_SERVICE_XMPP_SUBSCRIBE"
            }
            InboundServiceType::InboundServiceXmppPresence => {
                "INBOUND_SERVICE_XMPP_PRESENCE"
            }
            InboundServiceType::InboundServiceChannelPresence => {
                "INBOUND_SERVICE_CHANNEL_PRESENCE"
            }
            InboundServiceType::InboundServiceWarmup => "INBOUND_SERVICE_WARMUP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INBOUND_SERVICE_UNSPECIFIED" => Some(Self::InboundServiceUnspecified),
            "INBOUND_SERVICE_MAIL" => Some(Self::InboundServiceMail),
            "INBOUND_SERVICE_MAIL_BOUNCE" => Some(Self::InboundServiceMailBounce),
            "INBOUND_SERVICE_XMPP_ERROR" => Some(Self::InboundServiceXmppError),
            "INBOUND_SERVICE_XMPP_MESSAGE" => Some(Self::InboundServiceXmppMessage),
            "INBOUND_SERVICE_XMPP_SUBSCRIBE" => Some(Self::InboundServiceXmppSubscribe),
            "INBOUND_SERVICE_XMPP_PRESENCE" => Some(Self::InboundServiceXmppPresence),
            "INBOUND_SERVICE_CHANNEL_PRESENCE" => {
                Some(Self::InboundServiceChannelPresence)
            }
            "INBOUND_SERVICE_WARMUP" => Some(Self::InboundServiceWarmup),
            _ => None,
        }
    }
}
/// Run states of a version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServingStatus {
    /// Not specified.
    Unspecified = 0,
    /// Currently serving. Instances are created according to the
    /// scaling settings of the version.
    Serving = 1,
    /// Disabled. No instances will be created and the scaling
    /// settings are ignored until the state of the version changes
    /// to `SERVING`.
    Stopped = 2,
}
impl ServingStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServingStatus::Unspecified => "SERVING_STATUS_UNSPECIFIED",
            ServingStatus::Serving => "SERVING",
            ServingStatus::Stopped => "STOPPED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SERVING_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SERVING" => Some(Self::Serving),
            "STOPPED" => Some(Self::Stopped),
            _ => None,
        }
    }
}
/// Metadata for the given [google.longrunning.Operation][google.longrunning.Operation].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadataV1 {
    /// API method that initiated this operation. Example:
    /// `google.appengine.v1.Versions.CreateVersion`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub method: ::prost::alloc::string::String,
    /// Time that this operation was created.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "2")]
    pub insert_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time that this operation completed.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User who requested this operation.
    ///
    /// @OutputOnly
    #[prost(string, tag = "4")]
    pub user: ::prost::alloc::string::String,
    /// Name of the resource that this operation is acting on. Example:
    /// `apps/myapp/services/default`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "5")]
    pub target: ::prost::alloc::string::String,
    /// Ephemeral message that may change every time the operation is polled.
    /// @OutputOnly
    #[prost(string, tag = "6")]
    pub ephemeral_message: ::prost::alloc::string::String,
    /// Durable messages that persist on every operation poll.
    /// @OutputOnly
    #[prost(string, repeated, tag = "7")]
    pub warning: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Metadata specific to the type of operation in progress.
    /// @OutputOnly
    #[prost(oneof = "operation_metadata_v1::MethodMetadata", tags = "8")]
    pub method_metadata: ::core::option::Option<operation_metadata_v1::MethodMetadata>,
}
/// Nested message and enum types in `OperationMetadataV1`.
pub mod operation_metadata_v1 {
    /// Metadata specific to the type of operation in progress.
    /// @OutputOnly
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MethodMetadata {
        #[prost(message, tag = "8")]
        CreateVersionMetadata(super::CreateVersionMetadataV1),
    }
}
/// Metadata for the given [google.longrunning.Operation][google.longrunning.Operation] during a
/// [google.appengine.v1.CreateVersionRequest][google.appengine.v1.CreateVersionRequest].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVersionMetadataV1 {
    /// The Cloud Build ID if one was created as part of the version create.
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub cloud_build_id: ::prost::alloc::string::String,
}
/// A Service resource is a logical component of an application that can share
/// state and communicate in a secure fashion with other services.
/// For example, an application that handles customer requests might
/// include separate services to handle tasks such as backend data
/// analysis or API requests from mobile devices. Each service has a
/// collection of versions that define a specific set of code used to
/// implement the functionality of that service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// Full path to the Service resource in the API.
    /// Example: `apps/myapp/services/default`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Relative name of the service within the application.
    /// Example: `default`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Mapping that defines fractional HTTP traffic diversion to
    /// different versions within the service.
    #[prost(message, optional, tag = "3")]
    pub split: ::core::option::Option<TrafficSplit>,
    /// A set of labels to apply to this service. Labels are key/value pairs that
    /// describe the service and all resources that belong to it (e.g.,
    /// versions). The labels can be used to search and group resources, and are
    /// propagated to the usage and billing reports, enabling fine-grain analysis
    /// of costs. An example of using labels is to tag resources belonging to
    /// different environments (e.g., "env=prod", "env=qa").
    ///
    /// <p>Label keys and values can be no longer than 63 characters and can only
    /// contain lowercase letters, numeric characters, underscores, dashes, and
    /// international characters. Label keys must start with a lowercase letter
    /// or an international character. Each service can have at most 32 labels.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Ingress settings for this service. Will apply to all versions.
    #[prost(message, optional, tag = "6")]
    pub network_settings: ::core::option::Option<NetworkSettings>,
}
/// Traffic routing configuration for versions within a single service. Traffic
/// splits define how traffic directed to the service is assigned to versions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficSplit {
    /// Mechanism used to determine which version a request is sent to.
    /// The traffic selection algorithm will
    /// be stable for either type until allocations are changed.
    #[prost(enumeration = "traffic_split::ShardBy", tag = "1")]
    pub shard_by: i32,
    /// Mapping from version IDs within the service to fractional
    /// (0.000, 1] allocations of traffic for that version. Each version can
    /// be specified only once, but some versions in the service may not
    /// have any traffic allocation. Services that have traffic allocated
    /// cannot be deleted until either the service is deleted or
    /// their traffic allocation is removed. Allocations must sum to 1.
    /// Up to two decimal place precision is supported for IP-based splits and
    /// up to three decimal places is supported for cookie-based splits.
    #[prost(btree_map = "string, double", tag = "2")]
    pub allocations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        f64,
    >,
}
/// Nested message and enum types in `TrafficSplit`.
pub mod traffic_split {
    /// Available sharding mechanisms.
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
    pub enum ShardBy {
        /// Diversion method unspecified.
        Unspecified = 0,
        /// Diversion based on a specially named cookie, "GOOGAPPUID." The cookie
        /// must be set by the application itself or no diversion will occur.
        Cookie = 1,
        /// Diversion based on applying the modulus operation to a fingerprint
        /// of the IP address.
        Ip = 2,
        /// Diversion based on weighted random assignment. An incoming request is
        /// randomly routed to a version in the traffic split, with probability
        /// proportional to the version's traffic share.
        Random = 3,
    }
    impl ShardBy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ShardBy::Unspecified => "UNSPECIFIED",
                ShardBy::Cookie => "COOKIE",
                ShardBy::Ip => "IP",
                ShardBy::Random => "RANDOM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "COOKIE" => Some(Self::Cookie),
                "IP" => Some(Self::Ip),
                "RANDOM" => Some(Self::Random),
                _ => None,
            }
        }
    }
}
/// An SSL certificate that a user has been authorized to administer. A user
/// is authorized to administer any certificate that applies to one of their
/// authorized domains.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedCertificate {
    /// Full path to the `AuthorizedCertificate` resource in the API. Example:
    /// `apps/myapp/authorizedCertificates/12345`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Relative name of the certificate. This is a unique value autogenerated
    /// on `AuthorizedCertificate` resource creation. Example: `12345`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// The user-specified display name of the certificate. This is not
    /// guaranteed to be unique. Example: `My Certificate`.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Topmost applicable domains of this certificate. This certificate
    /// applies to these domains and their subdomains. Example: `example.com`.
    ///
    /// @OutputOnly
    #[prost(string, repeated, tag = "4")]
    pub domain_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The time when this certificate expires. To update the renewal time on this
    /// certificate, upload an SSL certificate with a different expiration time
    /// using [`AuthorizedCertificates.UpdateAuthorizedCertificate`]().
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "5")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The SSL certificate serving the `AuthorizedCertificate` resource. This
    /// must be obtained independently from a certificate authority.
    #[prost(message, optional, tag = "6")]
    pub certificate_raw_data: ::core::option::Option<CertificateRawData>,
    /// Only applicable if this certificate is managed by App Engine. Managed
    /// certificates are tied to the lifecycle of a `DomainMapping` and cannot be
    /// updated or deleted via the `AuthorizedCertificates` API. If this
    /// certificate is manually administered by the user, this field will be empty.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "7")]
    pub managed_certificate: ::core::option::Option<ManagedCertificate>,
    /// The full paths to user visible Domain Mapping resources that have this
    /// certificate mapped. Example: `apps/myapp/domainMappings/example.com`.
    ///
    /// This may not represent the full list of mapped domain mappings if the user
    /// does not have `VIEWER` permissions on all of the applications that have
    /// this certificate mapped. See `domain_mappings_count` for a complete count.
    ///
    /// Only returned by `GET` or `LIST` requests when specifically requested by
    /// the `view=FULL_CERTIFICATE` option.
    ///
    /// @OutputOnly
    #[prost(string, repeated, tag = "8")]
    pub visible_domain_mappings: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Aggregate count of the domain mappings with this certificate mapped. This
    /// count includes domain mappings on applications for which the user does not
    /// have `VIEWER` permissions.
    ///
    /// Only returned by `GET` or `LIST` requests when specifically requested by
    /// the `view=FULL_CERTIFICATE` option.
    ///
    /// @OutputOnly
    #[prost(int32, tag = "9")]
    pub domain_mappings_count: i32,
}
/// An SSL certificate obtained from a certificate authority.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateRawData {
    /// PEM encoded x.509 public key certificate. This field is set once on
    /// certificate creation. Must include the header and footer. Example:
    /// <pre>
    /// -----BEGIN CERTIFICATE-----
    /// <certificate_value>
    /// -----END CERTIFICATE-----
    /// </pre>
    #[prost(string, tag = "1")]
    pub public_certificate: ::prost::alloc::string::String,
    /// Unencrypted PEM encoded RSA private key. This field is set once on
    /// certificate creation and then encrypted. The key size must be 2048
    /// bits or fewer. Must include the header and footer. Example:
    /// <pre>
    /// -----BEGIN RSA PRIVATE KEY-----
    /// <unencrypted_key_value>
    /// -----END RSA PRIVATE KEY-----
    /// </pre>
    /// @InputOnly
    #[prost(string, tag = "2")]
    pub private_key: ::prost::alloc::string::String,
}
/// A certificate managed by App Engine.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedCertificate {
    /// Time at which the certificate was last renewed. The renewal process is
    /// fully managed. Certificate renewal will automatically occur before the
    /// certificate expires. Renewal errors can be tracked via `ManagementStatus`.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "1")]
    pub last_renewal_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Status of certificate management. Refers to the most recent certificate
    /// acquisition or renewal attempt.
    ///
    /// @OutputOnly
    #[prost(enumeration = "ManagementStatus", tag = "2")]
    pub status: i32,
}
/// State of certificate management. Refers to the most recent certificate
/// acquisition or renewal attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ManagementStatus {
    Unspecified = 0,
    /// Certificate was successfully obtained and inserted into the serving
    /// system.
    Ok = 1,
    /// Certificate is under active attempts to acquire or renew.
    Pending = 2,
    /// Most recent renewal failed due to an invalid DNS setup and will be
    /// retried. Renewal attempts will continue to fail until the certificate
    /// domain's DNS configuration is fixed. The last successfully provisioned
    /// certificate may still be serving.
    FailedRetryingNotVisible = 4,
    /// All renewal attempts have been exhausted, likely due to an invalid DNS
    /// setup.
    FailedPermanent = 6,
    /// Most recent renewal failed due to an explicit CAA record that does not
    /// include one of the in-use CAs (Google CA and Let's Encrypt). Renewals will
    /// continue to fail until the CAA is reconfigured. The last successfully
    /// provisioned certificate may still be serving.
    FailedRetryingCaaForbidden = 7,
    /// Most recent renewal failed due to a CAA retrieval failure. This means that
    /// the domain's DNS provider does not properly handle CAA records, failing
    /// requests for CAA records when no CAA records are defined. Renewals will
    /// continue to fail until the DNS provider is changed or a CAA record is
    /// added for the given domain. The last successfully provisioned certificate
    /// may still be serving.
    FailedRetryingCaaChecking = 8,
}
impl ManagementStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ManagementStatus::Unspecified => "MANAGEMENT_STATUS_UNSPECIFIED",
            ManagementStatus::Ok => "OK",
            ManagementStatus::Pending => "PENDING",
            ManagementStatus::FailedRetryingNotVisible => "FAILED_RETRYING_NOT_VISIBLE",
            ManagementStatus::FailedPermanent => "FAILED_PERMANENT",
            ManagementStatus::FailedRetryingCaaForbidden => {
                "FAILED_RETRYING_CAA_FORBIDDEN"
            }
            ManagementStatus::FailedRetryingCaaChecking => "FAILED_RETRYING_CAA_CHECKING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MANAGEMENT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "OK" => Some(Self::Ok),
            "PENDING" => Some(Self::Pending),
            "FAILED_RETRYING_NOT_VISIBLE" => Some(Self::FailedRetryingNotVisible),
            "FAILED_PERMANENT" => Some(Self::FailedPermanent),
            "FAILED_RETRYING_CAA_FORBIDDEN" => Some(Self::FailedRetryingCaaForbidden),
            "FAILED_RETRYING_CAA_CHECKING" => Some(Self::FailedRetryingCaaChecking),
            _ => None,
        }
    }
}
/// A single firewall rule that is evaluated against incoming traffic
/// and provides an action to take on matched requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallRule {
    /// A positive integer between \[1, Int32.MaxValue-1\] that defines the order of
    /// rule evaluation. Rules with the lowest priority are evaluated first.
    ///
    /// A default rule at priority Int32.MaxValue matches all IPv4 and IPv6 traffic
    /// when no previous rule matches. Only the action of this rule can be modified
    /// by the user.
    #[prost(int32, tag = "1")]
    pub priority: i32,
    /// The action to take on matched requests.
    #[prost(enumeration = "firewall_rule::Action", tag = "2")]
    pub action: i32,
    /// IP address or range, defined using CIDR notation, of requests that this
    /// rule applies to. You can use the wildcard character "*" to match all IPs
    /// equivalent to "0/0" and "::/0" together.
    /// Examples: `192.168.1.1` or `192.168.0.0/16` or `2001:db8::/32`
    ///            or `2001:0db8:0000:0042:0000:8a2e:0370:7334`.
    ///
    ///
    /// <p>Truncation will be silently performed on addresses which are not
    /// properly truncated. For example, `1.2.3.4/24` is accepted as the same
    /// address as `1.2.3.0/24`. Similarly, for IPv6, `2001:db8::1/32` is accepted
    /// as the same address as `2001:db8::/32`.
    #[prost(string, tag = "3")]
    pub source_range: ::prost::alloc::string::String,
    /// An optional string description of this rule.
    /// This field has a maximum length of 100 characters.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `FirewallRule`.
pub mod firewall_rule {
    /// Available actions to take on matching requests.
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
    pub enum Action {
        UnspecifiedAction = 0,
        /// Matching requests are allowed.
        Allow = 1,
        /// Matching requests are denied.
        Deny = 2,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::UnspecifiedAction => "UNSPECIFIED_ACTION",
                Action::Allow => "ALLOW",
                Action::Deny => "DENY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED_ACTION" => Some(Self::UnspecifiedAction),
                "ALLOW" => Some(Self::Allow),
                "DENY" => Some(Self::Deny),
                _ => None,
            }
        }
    }
}
/// A domain serving an App Engine application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainMapping {
    /// Full path to the `DomainMapping` resource in the API. Example:
    /// `apps/myapp/domainMapping/example.com`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Relative name of the domain serving the application. Example:
    /// `example.com`.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// SSL configuration for this domain. If unconfigured, this domain will not
    /// serve with SSL.
    #[prost(message, optional, tag = "3")]
    pub ssl_settings: ::core::option::Option<SslSettings>,
    /// The resource records required to configure this domain mapping. These
    /// records must be added to the domain's DNS configuration in order to
    /// serve the application via this domain mapping.
    ///
    /// @OutputOnly
    #[prost(message, repeated, tag = "4")]
    pub resource_records: ::prost::alloc::vec::Vec<ResourceRecord>,
}
/// SSL configuration for a `DomainMapping` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslSettings {
    /// ID of the `AuthorizedCertificate` resource configuring SSL for the
    /// application. Clearing this field will remove SSL support.
    ///
    /// By default, a managed certificate is automatically created for every
    /// domain mapping. To omit SSL support or to configure SSL manually, specify
    /// `SslManagementType.MANUAL` on a `CREATE` or `UPDATE` request. You must
    /// be authorized to administer the `AuthorizedCertificate` resource to
    /// manually map it to a `DomainMapping` resource.
    /// Example: `12345`.
    #[prost(string, tag = "1")]
    pub certificate_id: ::prost::alloc::string::String,
    /// SSL management type for this domain. If `AUTOMATIC`, a managed certificate
    /// is automatically provisioned. If `MANUAL`, `certificate_id` must be
    /// manually specified in order to configure SSL for this domain.
    #[prost(enumeration = "ssl_settings::SslManagementType", tag = "3")]
    pub ssl_management_type: i32,
    /// ID of the managed `AuthorizedCertificate` resource currently being
    /// provisioned, if applicable. Until the new managed certificate has been
    /// successfully provisioned, the previous SSL state will be preserved. Once
    /// the provisioning process completes, the `certificate_id` field will reflect
    /// the new managed certificate and this field will be left empty. To remove
    /// SSL support while there is still a pending managed certificate, clear the
    /// `certificate_id` field with an `UpdateDomainMappingRequest`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "4")]
    pub pending_managed_certificate_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SslSettings`.
pub mod ssl_settings {
    /// The SSL management type for this domain.
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
    pub enum SslManagementType {
        /// Defaults to `AUTOMATIC`.
        Unspecified = 0,
        /// SSL support for this domain is configured automatically. The mapped SSL
        /// certificate will be automatically renewed.
        Automatic = 1,
        /// SSL support for this domain is configured manually by the user. Either
        /// the domain has no SSL support or a user-obtained SSL certificate has been
        /// explictly mapped to this domain.
        Manual = 2,
    }
    impl SslManagementType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SslManagementType::Unspecified => "SSL_MANAGEMENT_TYPE_UNSPECIFIED",
                SslManagementType::Automatic => "AUTOMATIC",
                SslManagementType::Manual => "MANUAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SSL_MANAGEMENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "AUTOMATIC" => Some(Self::Automatic),
                "MANUAL" => Some(Self::Manual),
                _ => None,
            }
        }
    }
}
/// A DNS resource record.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceRecord {
    /// Relative name of the object affected by this record. Only applicable for
    /// `CNAME` records. Example: 'www'.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Data for this record. Values vary by record type, as defined in RFC 1035
    /// (section 5) and RFC 1034 (section 3.6.1).
    #[prost(string, tag = "2")]
    pub rrdata: ::prost::alloc::string::String,
    /// Resource record type. Example: `AAAA`.
    #[prost(enumeration = "resource_record::RecordType", tag = "3")]
    pub r#type: i32,
}
/// Nested message and enum types in `ResourceRecord`.
pub mod resource_record {
    /// A resource record type.
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
    pub enum RecordType {
        /// An unknown resource record.
        Unspecified = 0,
        /// An A resource record. Data is an IPv4 address.
        A = 1,
        /// An AAAA resource record. Data is an IPv6 address.
        Aaaa = 2,
        /// A CNAME resource record. Data is a domain name to be aliased.
        Cname = 3,
    }
    impl RecordType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RecordType::Unspecified => "RECORD_TYPE_UNSPECIFIED",
                RecordType::A => "A",
                RecordType::Aaaa => "AAAA",
                RecordType::Cname => "CNAME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RECORD_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "A" => Some(Self::A),
                "AAAA" => Some(Self::Aaaa),
                "CNAME" => Some(Self::Cname),
                _ => None,
            }
        }
    }
}
/// Metadata for the given [google.cloud.location.Location][google.cloud.location.Location].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// App Engine standard environment is available in the given location.
    ///
    /// @OutputOnly
    #[prost(bool, tag = "2")]
    pub standard_environment_available: bool,
    /// App Engine flexible environment is available in the given location.
    ///
    /// @OutputOnly
    #[prost(bool, tag = "4")]
    pub flexible_environment_available: bool,
    /// Output only. [Search API](<https://cloud.google.com/appengine/docs/standard/python/search>)
    /// is available in the given location.
    #[prost(bool, tag = "6")]
    pub search_api_available: bool,
}
/// A domain that a user has been authorized to administer. To authorize use
/// of a domain, verify ownership via
/// [Search Console](<https://search.google.com/search-console/welcome>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedDomain {
    /// Full path to the `AuthorizedDomain` resource in the API. Example:
    /// `apps/myapp/authorizedDomains/example.com`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Fully qualified domain name of the domain authorized for use. Example:
    /// `example.com`.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// Request message for `Applications.GetApplication`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApplicationRequest {
    /// Name of the Application resource to get. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `Applications.CreateApplication`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApplicationRequest {
    /// Application configuration.
    #[prost(message, optional, tag = "2")]
    pub application: ::core::option::Option<Application>,
}
/// Request message for `Applications.UpdateApplication`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApplicationRequest {
    /// Name of the Application resource to update. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An Application containing the updated resource.
    #[prost(message, optional, tag = "2")]
    pub application: ::core::option::Option<Application>,
    /// Required. Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for 'Applications.RepairApplication'.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepairApplicationRequest {
    /// Name of the application to repair. Example: `apps/myapp`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `Services.ListServices`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Name of the parent Application resource. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `Services.ListServices`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The services belonging to the requested application.
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `Services.GetService`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Name of the resource requested. Example: `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `Services.UpdateService`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceRequest {
    /// Name of the resource to update. Example: `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A Service resource containing the updated service. Only fields set in the
    /// field mask will be updated.
    #[prost(message, optional, tag = "2")]
    pub service: ::core::option::Option<Service>,
    /// Required. Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Set to `true` to gradually shift traffic to one or more versions that you
    /// specify. By default, traffic is shifted immediately.
    /// For gradual traffic migration, the target versions
    /// must be located within instances that are configured for both
    /// [warmup requests](<https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#InboundServiceType>)
    /// and
    /// [automatic scaling](<https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#AutomaticScaling>).
    /// You must specify the
    /// [`shardBy`](<https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services#ShardBy>)
    /// field in the Service resource. Gradual traffic migration is not
    /// supported in the App Engine flexible environment. For examples, see
    /// [Migrating and Splitting Traffic](<https://cloud.google.com/appengine/docs/admin-api/migrating-splitting-traffic>).
    #[prost(bool, tag = "4")]
    pub migrate_traffic: bool,
}
/// Request message for `Services.DeleteService`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Name of the resource requested. Example: `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `Versions.ListVersions`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsRequest {
    /// Name of the parent Service resource. Example:
    /// `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Controls the set of fields returned in the `List` response.
    #[prost(enumeration = "VersionView", tag = "2")]
    pub view: i32,
    /// Maximum results to return per page.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `Versions.ListVersions`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsResponse {
    /// The versions belonging to the requested service.
    #[prost(message, repeated, tag = "1")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `Versions.GetVersion`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Controls the set of fields returned in the `Get` response.
    #[prost(enumeration = "VersionView", tag = "2")]
    pub view: i32,
}
/// Request message for `Versions.CreateVersion`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVersionRequest {
    /// Name of the parent resource to create this version under. Example:
    /// `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Application deployment configuration.
    #[prost(message, optional, tag = "2")]
    pub version: ::core::option::Option<Version>,
}
/// Request message for `Versions.UpdateVersion`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVersionRequest {
    /// Name of the resource to update. Example:
    /// `apps/myapp/services/default/versions/1`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A Version containing the updated resource. Only fields set in the field
    /// mask will be updated.
    #[prost(message, optional, tag = "2")]
    pub version: ::core::option::Option<Version>,
    /// Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for `Versions.DeleteVersion`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVersionRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `Instances.ListInstances`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Name of the parent Version resource. Example:
    /// `apps/myapp/services/default/versions/v1`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `Instances.ListInstances`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// The instances belonging to the requested version.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `Instances.GetInstance`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1/instances/instance-1`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `Instances.DeleteInstance`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1/instances/instance-1`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `Instances.DebugInstance`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugInstanceRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1/instances/instance-1`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Public SSH key to add to the instance. Examples:
    ///
    /// * `\[USERNAME\]:ssh-rsa \[KEY_VALUE\] [USERNAME]`
    /// * `\[USERNAME\]:ssh-rsa \[KEY_VALUE\] google-ssh {"userName":"\[USERNAME\]","expireOn":"\[EXPIRE_TIME\]"}`
    ///
    /// For more information, see
    /// [Adding and Removing SSH Keys](<https://cloud.google.com/compute/docs/instances/adding-removing-ssh-keys>).
    #[prost(string, tag = "2")]
    pub ssh_key: ::prost::alloc::string::String,
}
/// Request message for `Firewall.ListIngressRules`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIngressRulesRequest {
    /// Name of the Firewall collection to retrieve.
    /// Example: `apps/myapp/firewall/ingressRules`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A valid IP Address. If set, only rules matching this address will be
    /// returned. The first returned rule will be the rule that fires on requests
    /// from this IP.
    #[prost(string, tag = "4")]
    pub matching_address: ::prost::alloc::string::String,
}
/// Response message for `Firewall.ListIngressRules`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIngressRulesResponse {
    /// The ingress FirewallRules for this application.
    #[prost(message, repeated, tag = "1")]
    pub ingress_rules: ::prost::alloc::vec::Vec<FirewallRule>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `Firewall.BatchUpdateIngressRules`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateIngressRulesRequest {
    /// Name of the Firewall collection to set.
    /// Example: `apps/myapp/firewall/ingressRules`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of FirewallRules to replace the existing set.
    #[prost(message, repeated, tag = "2")]
    pub ingress_rules: ::prost::alloc::vec::Vec<FirewallRule>,
}
/// Response message for `Firewall.UpdateAllIngressRules`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateIngressRulesResponse {
    /// The full list of ingress FirewallRules for this application.
    #[prost(message, repeated, tag = "1")]
    pub ingress_rules: ::prost::alloc::vec::Vec<FirewallRule>,
}
/// Request message for `Firewall.CreateIngressRule`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIngressRuleRequest {
    /// Name of the parent Firewall collection in which to create a new rule.
    /// Example: `apps/myapp/firewall/ingressRules`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// A FirewallRule containing the new resource.
    ///
    /// The user may optionally provide a position at which the new rule will be
    /// placed. The positions define a sequential list starting at 1. If a rule
    /// already exists at the given position, rules greater than the provided
    /// position will be moved forward by one.
    ///
    /// If no position is provided, the server will place the rule as the second to
    /// last rule in the sequence before the required default allow-all or deny-all
    /// rule.
    #[prost(message, optional, tag = "2")]
    pub rule: ::core::option::Option<FirewallRule>,
}
/// Request message for `Firewall.GetIngressRule`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIngressRuleRequest {
    /// Name of the Firewall resource to retrieve.
    /// Example: `apps/myapp/firewall/ingressRules/100`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `Firewall.UpdateIngressRule`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIngressRuleRequest {
    /// Name of the Firewall resource to update.
    /// Example: `apps/myapp/firewall/ingressRules/100`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A FirewallRule containing the updated resource
    #[prost(message, optional, tag = "2")]
    pub rule: ::core::option::Option<FirewallRule>,
    /// Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for `Firewall.DeleteIngressRule`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIngressRuleRequest {
    /// Name of the Firewall resource to delete.
    /// Example: `apps/myapp/firewall/ingressRules/100`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `AuthorizedDomains.ListAuthorizedDomains`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizedDomainsRequest {
    /// Name of the parent Application resource. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `AuthorizedDomains.ListAuthorizedDomains`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizedDomainsResponse {
    /// The authorized domains belonging to the user.
    #[prost(message, repeated, tag = "1")]
    pub domains: ::prost::alloc::vec::Vec<AuthorizedDomain>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `AuthorizedCertificates.ListAuthorizedCertificates`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizedCertificatesRequest {
    /// Name of the parent `Application` resource. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Controls the set of fields returned in the `LIST` response.
    #[prost(enumeration = "AuthorizedCertificateView", tag = "4")]
    pub view: i32,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `AuthorizedCertificates.ListAuthorizedCertificates`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizedCertificatesResponse {
    /// The SSL certificates the user is authorized to administer.
    #[prost(message, repeated, tag = "1")]
    pub certificates: ::prost::alloc::vec::Vec<AuthorizedCertificate>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `AuthorizedCertificates.GetAuthorizedCertificate`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthorizedCertificateRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/authorizedCertificates/12345`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Controls the set of fields returned in the `GET` response.
    #[prost(enumeration = "AuthorizedCertificateView", tag = "2")]
    pub view: i32,
}
/// Request message for `AuthorizedCertificates.CreateAuthorizedCertificate`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthorizedCertificateRequest {
    /// Name of the parent `Application` resource. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// SSL certificate data.
    #[prost(message, optional, tag = "2")]
    pub certificate: ::core::option::Option<AuthorizedCertificate>,
}
/// Request message for `AuthorizedCertificates.UpdateAuthorizedCertificate`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAuthorizedCertificateRequest {
    /// Name of the resource to update. Example:
    /// `apps/myapp/authorizedCertificates/12345`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An `AuthorizedCertificate` containing the updated resource. Only fields set
    /// in the field mask will be updated.
    #[prost(message, optional, tag = "2")]
    pub certificate: ::core::option::Option<AuthorizedCertificate>,
    /// Standard field mask for the set of fields to be updated. Updates are only
    /// supported on the `certificate_raw_data` and `display_name` fields.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for `AuthorizedCertificates.DeleteAuthorizedCertificate`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAuthorizedCertificateRequest {
    /// Name of the resource to delete. Example:
    /// `apps/myapp/authorizedCertificates/12345`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `DomainMappings.ListDomainMappings`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDomainMappingsRequest {
    /// Name of the parent Application resource. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `DomainMappings.ListDomainMappings`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDomainMappingsResponse {
    /// The domain mappings for the application.
    #[prost(message, repeated, tag = "1")]
    pub domain_mappings: ::prost::alloc::vec::Vec<DomainMapping>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `DomainMappings.GetDomainMapping`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDomainMappingRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/domainMappings/example.com`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `DomainMappings.CreateDomainMapping`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDomainMappingRequest {
    /// Name of the parent Application resource. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Domain mapping configuration.
    #[prost(message, optional, tag = "2")]
    pub domain_mapping: ::core::option::Option<DomainMapping>,
    /// Whether the domain creation should override any existing mappings for this
    /// domain. By default, overrides are rejected.
    #[prost(enumeration = "DomainOverrideStrategy", tag = "4")]
    pub override_strategy: i32,
}
/// Request message for `DomainMappings.UpdateDomainMapping`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDomainMappingRequest {
    /// Name of the resource to update. Example:
    /// `apps/myapp/domainMappings/example.com`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A domain mapping containing the updated resource. Only fields set
    /// in the field mask will be updated.
    #[prost(message, optional, tag = "2")]
    pub domain_mapping: ::core::option::Option<DomainMapping>,
    /// Required. Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for `DomainMappings.DeleteDomainMapping`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDomainMappingRequest {
    /// Name of the resource to delete. Example:
    /// `apps/myapp/domainMappings/example.com`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Fields that should be returned when [Version][google.appengine.v1.Version] resources
/// are retrieved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VersionView {
    /// Basic version information including scaling and inbound services,
    /// but not detailed deployment information.
    Basic = 0,
    /// The information from `BASIC`, plus detailed information about the
    /// deployment. This format is required when creating resources, but
    /// is not returned in `Get` or `List` by default.
    Full = 1,
}
impl VersionView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VersionView::Basic => "BASIC",
            VersionView::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BASIC" => Some(Self::Basic),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Fields that should be returned when an AuthorizedCertificate resource is
/// retrieved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthorizedCertificateView {
    /// Basic certificate information, including applicable domains and expiration
    /// date.
    BasicCertificate = 0,
    /// The information from `BASIC_CERTIFICATE`, plus detailed information on the
    /// domain mappings that have this certificate mapped.
    FullCertificate = 1,
}
impl AuthorizedCertificateView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthorizedCertificateView::BasicCertificate => "BASIC_CERTIFICATE",
            AuthorizedCertificateView::FullCertificate => "FULL_CERTIFICATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BASIC_CERTIFICATE" => Some(Self::BasicCertificate),
            "FULL_CERTIFICATE" => Some(Self::FullCertificate),
            _ => None,
        }
    }
}
/// Override strategy for mutating an existing mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DomainOverrideStrategy {
    /// Strategy unspecified. Defaults to `STRICT`.
    UnspecifiedDomainOverrideStrategy = 0,
    /// Overrides not allowed. If a mapping already exists for the
    /// specified domain, the request will return an ALREADY_EXISTS (409).
    Strict = 1,
    /// Overrides allowed. If a mapping already exists for the specified domain,
    /// the request will overwrite it. Note that this might stop another
    /// Google product from serving. For example, if the domain is
    /// mapped to another App Engine application, that app will no
    /// longer serve from that domain.
    Override = 2,
}
impl DomainOverrideStrategy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DomainOverrideStrategy::UnspecifiedDomainOverrideStrategy => {
                "UNSPECIFIED_DOMAIN_OVERRIDE_STRATEGY"
            }
            DomainOverrideStrategy::Strict => "STRICT",
            DomainOverrideStrategy::Override => "OVERRIDE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED_DOMAIN_OVERRIDE_STRATEGY" => {
                Some(Self::UnspecifiedDomainOverrideStrategy)
            }
            "STRICT" => Some(Self::Strict),
            "OVERRIDE" => Some(Self::Override),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod applications_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages App Engine applications.
    #[derive(Debug, Clone)]
    pub struct ApplicationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ApplicationsClient<T>
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
        ) -> ApplicationsClient<InterceptedService<T, F>>
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
            ApplicationsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets information about an application.
        pub async fn get_application(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApplicationRequest>,
        ) -> std::result::Result<tonic::Response<super::Application>, tonic::Status> {
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
                "/google.appengine.v1.Applications/GetApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Applications", "GetApplication"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an App Engine application for a Google Cloud Platform project.
        /// Required fields:
        ///
        /// * `id` - The ID of the target Cloud Platform project.
        /// * *location* - The [region](https://cloud.google.com/appengine/docs/locations) where you want the App Engine application located.
        ///
        /// For more information about App Engine applications, see [Managing Projects, Applications, and Billing](https://cloud.google.com/appengine/docs/standard/python/console/).
        pub async fn create_application(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.appengine.v1.Applications/CreateApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.Applications",
                        "CreateApplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified Application resource.
        /// You can update the following fields:
        ///
        /// * `auth_domain` - Google authentication domain for controlling user access to the application.
        /// * `default_cookie_expiration` - Cookie expiration policy for the application.
        /// * `iap` - Identity-Aware Proxy properties for the application.
        pub async fn update_application(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.appengine.v1.Applications/UpdateApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.Applications",
                        "UpdateApplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Recreates the required App Engine features for the specified App Engine
        /// application, for example a Cloud Storage bucket or App Engine service
        /// account.
        /// Use this method if you receive an error message about a missing feature,
        /// for example, *Error retrieving the App Engine service account*.
        /// If you have deleted your App Engine service account, this will
        /// not be able to recreate it. Instead, you should attempt to use the
        /// IAM undelete API if possible at https://cloud.google.com/iam/reference/rest/v1/projects.serviceAccounts/undelete?apix_params=%7B"name"%3A"projects%2F-%2FserviceAccounts%2Funique_id"%2C"resource"%3A%7B%7D%7D .
        /// If the deletion was recent, the numeric ID can be found in the Cloud
        /// Console Activity Log.
        pub async fn repair_application(
            &mut self,
            request: impl tonic::IntoRequest<super::RepairApplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.appengine.v1.Applications/RepairApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.Applications",
                        "RepairApplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod services_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages services of an application.
    #[derive(Debug, Clone)]
    pub struct ServicesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ServicesClient<T>
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
        ) -> ServicesClient<InterceptedService<T, F>>
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
            ServicesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all the services in the application.
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
                "/google.appengine.v1.Services/ListServices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.appengine.v1.Services", "ListServices"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the current configuration of the specified service.
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
                "/google.appengine.v1.Services/GetService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.appengine.v1.Services", "GetService"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates the configuration of the specified service.
        pub async fn update_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.appengine.v1.Services/UpdateService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Services", "UpdateService"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified service and all enclosed versions.
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.appengine.v1.Services/DeleteService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Services", "DeleteService"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod versions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages versions of a service.
    #[derive(Debug, Clone)]
    pub struct VersionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VersionsClient<T>
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
        ) -> VersionsClient<InterceptedService<T, F>>
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
            VersionsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists the versions of a service.
        pub async fn list_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVersionsResponse>,
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
                "/google.appengine.v1.Versions/ListVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.appengine.v1.Versions", "ListVersions"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the specified Version resource.
        /// By default, only a `BASIC_VIEW` will be returned.
        /// Specify the `FULL_VIEW` parameter to get the full resource.
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVersionRequest>,
        ) -> std::result::Result<tonic::Response<super::Version>, tonic::Status> {
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
                "/google.appengine.v1.Versions/GetVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.appengine.v1.Versions", "GetVersion"));
            self.inner.unary(req, path, codec).await
        }
        /// Deploys code and resource files to a new version.
        pub async fn create_version(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.appengine.v1.Versions/CreateVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Versions", "CreateVersion"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified Version resource.
        /// You can specify the following fields depending on the App Engine
        /// environment and type of scaling that the version resource uses:
        ///
        /// **Standard environment**
        ///
        /// * [`instance_class`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.instance_class)
        ///
        /// *automatic scaling* in the standard environment:
        ///
        /// * [`automatic_scaling.min_idle_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
        /// * [`automatic_scaling.max_idle_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
        /// * [`automaticScaling.standard_scheduler_settings.max_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings)
        /// * [`automaticScaling.standard_scheduler_settings.min_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings)
        /// * [`automaticScaling.standard_scheduler_settings.target_cpu_utilization`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings)
        /// * [`automaticScaling.standard_scheduler_settings.target_throughput_utilization`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings)
        ///
        /// *basic scaling* or *manual scaling* in the standard environment:
        ///
        /// * [`serving_status`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.serving_status)
        /// * [`manual_scaling.instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#manualscaling)
        ///
        /// **Flexible environment**
        ///
        /// * [`serving_status`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.serving_status)
        ///
        /// *automatic scaling* in the flexible environment:
        ///
        /// * [`automatic_scaling.min_total_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
        /// * [`automatic_scaling.max_total_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
        /// * [`automatic_scaling.cool_down_period_sec`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
        /// * [`automatic_scaling.cpu_utilization.target_utilization`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
        ///
        /// *manual scaling* in the flexible environment:
        ///
        /// * [`manual_scaling.instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#manualscaling)
        pub async fn update_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.appengine.v1.Versions/UpdateVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Versions", "UpdateVersion"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an existing Version resource.
        pub async fn delete_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.appengine.v1.Versions/DeleteVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Versions", "DeleteVersion"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod instances_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages instances of a version.
    #[derive(Debug, Clone)]
    pub struct InstancesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> InstancesClient<T>
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
        ) -> InstancesClient<InterceptedService<T, F>>
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
            InstancesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists the instances of a version.
        ///
        /// Tip: To aggregate details about instances over time, see the
        /// [Stackdriver Monitoring API](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list).
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInstancesResponse>,
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
                "/google.appengine.v1.Instances/ListInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Instances", "ListInstances"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets instance information.
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> std::result::Result<tonic::Response<super::Instance>, tonic::Status> {
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
                "/google.appengine.v1.Instances/GetInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.appengine.v1.Instances", "GetInstance"));
            self.inner.unary(req, path, codec).await
        }
        /// Stops a running instance.
        ///
        /// The instance might be automatically recreated based on the scaling settings
        /// of the version. For more information, see "How Instances are Managed"
        /// ([standard environment](https://cloud.google.com/appengine/docs/standard/python/how-instances-are-managed) |
        /// [flexible environment](https://cloud.google.com/appengine/docs/flexible/python/how-instances-are-managed)).
        ///
        /// To ensure that instances are not re-created and avoid getting billed, you
        /// can stop all instances within the target version by changing the serving
        /// status of the version to `STOPPED` with the
        /// [`apps.services.versions.patch`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions/patch)
        /// method.
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.appengine.v1.Instances/DeleteInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Instances", "DeleteInstance"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Enables debugging on a VM instance. This allows you to use the SSH
        /// command to connect to the virtual machine where the instance lives.
        /// While in "debug mode", the instance continues to serve live traffic.
        /// You should delete the instance when you are done debugging and then
        /// allow the system to take over and determine if another instance
        /// should be started.
        ///
        /// Only applicable for instances in App Engine flexible environment.
        pub async fn debug_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DebugInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.appengine.v1.Instances/DebugInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Instances", "DebugInstance"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod firewall_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Firewall resources are used to define a collection of access control rules
    /// for an Application. Each rule is defined with a position which specifies
    /// the rule's order in the sequence of rules, an IP range to be matched against
    /// requests, and an action to take upon matching requests.
    ///
    /// Every request is evaluated against the Firewall rules in priority order.
    /// Processesing stops at the first rule which matches the request's IP address.
    /// A final rule always specifies an action that applies to all remaining
    /// IP addresses. The default final rule for a newly-created application will be
    /// set to "allow" if not otherwise specified by the user.
    #[derive(Debug, Clone)]
    pub struct FirewallClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FirewallClient<T>
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
        ) -> FirewallClient<InterceptedService<T, F>>
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
            FirewallClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists the firewall rules of an application.
        pub async fn list_ingress_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIngressRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListIngressRulesResponse>,
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
                "/google.appengine.v1.Firewall/ListIngressRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Firewall", "ListIngressRules"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Replaces the entire firewall ruleset in one bulk operation. This overrides
        /// and replaces the rules of an existing firewall with the new rules.
        ///
        /// If the final rule does not match traffic with the '*' wildcard IP range,
        /// then an "allow all" rule is explicitly added to the end of the list.
        pub async fn batch_update_ingress_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateIngressRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchUpdateIngressRulesResponse>,
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
                "/google.appengine.v1.Firewall/BatchUpdateIngressRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.Firewall",
                        "BatchUpdateIngressRules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a firewall rule for the application.
        pub async fn create_ingress_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateIngressRuleRequest>,
        ) -> std::result::Result<tonic::Response<super::FirewallRule>, tonic::Status> {
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
                "/google.appengine.v1.Firewall/CreateIngressRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Firewall", "CreateIngressRule"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the specified firewall rule.
        pub async fn get_ingress_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIngressRuleRequest>,
        ) -> std::result::Result<tonic::Response<super::FirewallRule>, tonic::Status> {
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
                "/google.appengine.v1.Firewall/GetIngressRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Firewall", "GetIngressRule"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified firewall rule.
        pub async fn update_ingress_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIngressRuleRequest>,
        ) -> std::result::Result<tonic::Response<super::FirewallRule>, tonic::Status> {
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
                "/google.appengine.v1.Firewall/UpdateIngressRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Firewall", "UpdateIngressRule"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified firewall rule.
        pub async fn delete_ingress_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIngressRuleRequest>,
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
                "/google.appengine.v1.Firewall/DeleteIngressRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.appengine.v1.Firewall", "DeleteIngressRule"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod authorized_domains_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages domains a user is authorized to administer. To authorize use of a
    /// domain, verify ownership via
    /// [Webmaster Central](https://www.google.com/webmasters/verification/home).
    #[derive(Debug, Clone)]
    pub struct AuthorizedDomainsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AuthorizedDomainsClient<T>
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
        ) -> AuthorizedDomainsClient<InterceptedService<T, F>>
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
            AuthorizedDomainsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all domains the user is authorized to administer.
        pub async fn list_authorized_domains(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAuthorizedDomainsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAuthorizedDomainsResponse>,
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
                "/google.appengine.v1.AuthorizedDomains/ListAuthorizedDomains",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.AuthorizedDomains",
                        "ListAuthorizedDomains",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod authorized_certificates_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages SSL certificates a user is authorized to administer. A user can
    /// administer any SSL certificates applicable to their authorized domains.
    #[derive(Debug, Clone)]
    pub struct AuthorizedCertificatesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AuthorizedCertificatesClient<T>
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
        ) -> AuthorizedCertificatesClient<InterceptedService<T, F>>
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
            AuthorizedCertificatesClient::new(
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
        /// Lists all SSL certificates the user is authorized to administer.
        pub async fn list_authorized_certificates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAuthorizedCertificatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAuthorizedCertificatesResponse>,
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
                "/google.appengine.v1.AuthorizedCertificates/ListAuthorizedCertificates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.AuthorizedCertificates",
                        "ListAuthorizedCertificates",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the specified SSL certificate.
        pub async fn get_authorized_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAuthorizedCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthorizedCertificate>,
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
                "/google.appengine.v1.AuthorizedCertificates/GetAuthorizedCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.AuthorizedCertificates",
                        "GetAuthorizedCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Uploads the specified SSL certificate.
        pub async fn create_authorized_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAuthorizedCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthorizedCertificate>,
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
                "/google.appengine.v1.AuthorizedCertificates/CreateAuthorizedCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.AuthorizedCertificates",
                        "CreateAuthorizedCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified SSL certificate. To renew a certificate and maintain
        /// its existing domain mappings, update `certificate_data` with a new
        /// certificate. The new certificate must be applicable to the same domains as
        /// the original certificate. The certificate `display_name` may also be
        /// updated.
        pub async fn update_authorized_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAuthorizedCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthorizedCertificate>,
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
                "/google.appengine.v1.AuthorizedCertificates/UpdateAuthorizedCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.AuthorizedCertificates",
                        "UpdateAuthorizedCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified SSL certificate.
        pub async fn delete_authorized_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAuthorizedCertificateRequest>,
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
                "/google.appengine.v1.AuthorizedCertificates/DeleteAuthorizedCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.AuthorizedCertificates",
                        "DeleteAuthorizedCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod domain_mappings_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages domains serving an application.
    #[derive(Debug, Clone)]
    pub struct DomainMappingsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DomainMappingsClient<T>
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
        ) -> DomainMappingsClient<InterceptedService<T, F>>
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
            DomainMappingsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists the domain mappings on an application.
        pub async fn list_domain_mappings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDomainMappingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDomainMappingsResponse>,
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
                "/google.appengine.v1.DomainMappings/ListDomainMappings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.DomainMappings",
                        "ListDomainMappings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the specified domain mapping.
        pub async fn get_domain_mapping(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDomainMappingRequest>,
        ) -> std::result::Result<tonic::Response<super::DomainMapping>, tonic::Status> {
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
                "/google.appengine.v1.DomainMappings/GetDomainMapping",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.DomainMappings",
                        "GetDomainMapping",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Maps a domain to an application. A user must be authorized to administer a
        /// domain in order to map it to an application. For a list of available
        /// authorized domains, see [`AuthorizedDomains.ListAuthorizedDomains`]().
        pub async fn create_domain_mapping(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDomainMappingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.appengine.v1.DomainMappings/CreateDomainMapping",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.DomainMappings",
                        "CreateDomainMapping",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified domain mapping. To map an SSL certificate to a
        /// domain mapping, update `certificate_id` to point to an `AuthorizedCertificate`
        /// resource. A user must be authorized to administer the associated domain
        /// in order to update a `DomainMapping` resource.
        pub async fn update_domain_mapping(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDomainMappingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.appengine.v1.DomainMappings/UpdateDomainMapping",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.DomainMappings",
                        "UpdateDomainMapping",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified domain mapping. A user must be authorized to
        /// administer the associated domain in order to delete a `DomainMapping`
        /// resource.
        pub async fn delete_domain_mapping(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDomainMappingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.appengine.v1.DomainMappings/DeleteDomainMapping",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.appengine.v1.DomainMappings",
                        "DeleteDomainMapping",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// App Engine admin service audit log.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditData {
    /// Detailed information about methods that require it. Does not include
    /// simple Get, List or Delete methods because all significant information
    /// (resource name, number of returned elements for List operations) is already
    /// included in parent audit log message.
    #[prost(oneof = "audit_data::Method", tags = "1, 2")]
    pub method: ::core::option::Option<audit_data::Method>,
}
/// Nested message and enum types in `AuditData`.
pub mod audit_data {
    /// Detailed information about methods that require it. Does not include
    /// simple Get, List or Delete methods because all significant information
    /// (resource name, number of returned elements for List operations) is already
    /// included in parent audit log message.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        /// Detailed information about UpdateService call.
        #[prost(message, tag = "1")]
        UpdateService(super::UpdateServiceMethod),
        /// Detailed information about CreateVersion call.
        #[prost(message, tag = "2")]
        CreateVersion(super::CreateVersionMethod),
    }
}
/// Detailed information about UpdateService call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceMethod {
    /// Update service request.
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<UpdateServiceRequest>,
}
/// Detailed information about CreateVersion call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVersionMethod {
    /// Create version request.
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<CreateVersionRequest>,
}
