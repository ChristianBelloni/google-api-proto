/// Message used by AlloyDB connectors to exchange client and connection metadata
/// with the server after a successful TLS handshake. This metadata includes an
/// IAM token, which is used to authenticate users based on their IAM identity.
/// The sole purpose of this message is for the use of AlloyDB connectors.
/// Clients should not rely on this message directly as there can be breaking
/// changes in the future.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataExchangeRequest {
    /// Optional. Connector information.
    #[prost(string, tag = "1")]
    pub user_agent: ::prost::alloc::string::String,
    /// Authentication type.
    #[prost(enumeration = "metadata_exchange_request::AuthType", tag = "2")]
    pub auth_type: i32,
    /// IAM token used for both IAM user authentiation and
    /// `alloydb.instances.connect` permission check.
    #[prost(string, tag = "3")]
    pub oauth2_token: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MetadataExchangeRequest`.
pub mod metadata_exchange_request {
    /// AuthType contains all supported authentication types.
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
    pub enum AuthType {
        /// Authentication type is unspecified and DB_NATIVE is used by default
        Unspecified = 0,
        /// Database native authentication (user/password)
        DbNative = 1,
        /// Automatic IAM authentication
        AutoIam = 2,
    }
    impl AuthType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AuthType::Unspecified => "AUTH_TYPE_UNSPECIFIED",
                AuthType::DbNative => "DB_NATIVE",
                AuthType::AutoIam => "AUTO_IAM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "AUTH_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DB_NATIVE" => Some(Self::DbNative),
                "AUTO_IAM" => Some(Self::AutoIam),
                _ => None,
            }
        }
    }
}
/// Message for response to metadata exchange request. The sole purpose of this
/// message is for the use of AlloyDB connectors. Clients should not rely on this
/// message directly as there can be breaking changes in the future.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataExchangeResponse {
    /// Response code.
    #[prost(enumeration = "metadata_exchange_response::ResponseCode", tag = "1")]
    pub response_code: i32,
    /// Optional. Error message.
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MetadataExchangeResponse`.
pub mod metadata_exchange_response {
    /// Response code.
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
    pub enum ResponseCode {
        /// Unknown response code
        Unspecified = 0,
        /// Success
        Ok = 1,
        /// Failure
        Error = 2,
    }
    impl ResponseCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResponseCode::Unspecified => "RESPONSE_CODE_UNSPECIFIED",
                ResponseCode::Ok => "OK",
                ResponseCode::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESPONSE_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "OK" => Some(Self::Ok),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
