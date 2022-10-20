/// Request message for Connectors.GetRuntimeConfig.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRuntimeConfigRequest {
    /// Required. Resource name of the form:
    /// `projects/*/locations/*/runtimeConfig`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// RuntimeConfig is the singleton resource of each location.
/// It includes generic resource configs consumed by control plane and runtime
/// plane like: pub/sub topic/subscription resource name, Cloud Storage location
/// storing schema etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeConfig {
    /// Output only. location_id of the runtime location. E.g. "us-west1".
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
    /// Output only. Pub/Sub topic for connd to send message.
    /// E.g. projects/{project-id}/topics/{topic-id}
    #[prost(string, tag="2")]
    pub connd_topic: ::prost::alloc::string::String,
    /// Output only. Pub/Sub subscription for connd to receive message.
    /// E.g. projects/{project-id}/subscriptions/{topic-id}
    #[prost(string, tag="3")]
    pub connd_subscription: ::prost::alloc::string::String,
    /// Output only. Pub/Sub topic for control plne to send message.
    /// communication.
    /// E.g. projects/{project-id}/topics/{topic-id}
    #[prost(string, tag="4")]
    pub control_plane_topic: ::prost::alloc::string::String,
    /// Output only. Pub/Sub subscription for control plane to receive message.
    /// E.g. projects/{project-id}/subscriptions/{topic-id}
    #[prost(string, tag="5")]
    pub control_plane_subscription: ::prost::alloc::string::String,
    /// Output only. The endpoint of the connectors runtime ingress.
    #[prost(string, tag="6")]
    pub runtime_endpoint: ::prost::alloc::string::String,
    /// Output only. The state of the location.
    #[prost(enumeration="runtime_config::State", tag="7")]
    pub state: i32,
    /// Output only. The Cloud Storage bucket that stores connector's schema reports.
    #[prost(string, tag="8")]
    pub schema_gcs_bucket: ::prost::alloc::string::String,
    /// Output only. The name of the Service Directory service name.
    #[prost(string, tag="9")]
    pub service_directory: ::prost::alloc::string::String,
    /// Output only. Name of the runtimeConfig resource.
    /// Format: projects/{project}/locations/{location}/runtimeConfig
    #[prost(string, tag="11")]
    pub name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RuntimeConfig`.
pub mod runtime_config {
    /// State of the location.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// STATE_UNSPECIFIED.
        Unspecified = 0,
        /// INACTIVE.
        Inactive = 1,
        /// ACTIVATING.
        Activating = 2,
        /// ACTIVE.
        Active = 3,
        /// CREATING.
        Creating = 4,
        /// DELETING.
        Deleting = 5,
        /// UPDATING.
        Updating = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Inactive => "INACTIVE",
                State::Activating => "ACTIVATING",
                State::Active => "ACTIVE",
                State::Creating => "CREATING",
                State::Deleting => "DELETING",
                State::Updating => "UPDATING",
            }
        }
    }
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag="3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag="4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag="5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag="6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag="7")]
    pub api_version: ::prost::alloc::string::String,
}
/// ConfigVariableTemplate provides metadata about a `ConfigVariable` that is
/// used in a Connection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigVariableTemplate {
    /// Key of the config variable.
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    /// Type of the parameter: string, int, bool etc.
    /// consider custom type for the benefit for the validation.
    #[prost(enumeration="config_variable_template::ValueType", tag="2")]
    pub value_type: i32,
    /// Display name of the parameter.
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    /// Description.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// Regular expression in RE2 syntax used for validating the `value` of a
    /// `ConfigVariable`.
    #[prost(string, tag="5")]
    pub validation_regex: ::prost::alloc::string::String,
    /// Flag represents that this `ConfigVariable` must be provided for a
    /// connection.
    #[prost(bool, tag="6")]
    pub required: bool,
    /// Role grant configuration for the config variable.
    #[prost(message, optional, tag="7")]
    pub role_grant: ::core::option::Option<RoleGrant>,
    /// Enum options. To be populated if `ValueType` is `ENUM`
    #[prost(message, repeated, tag="8")]
    pub enum_options: ::prost::alloc::vec::Vec<EnumOption>,
    /// Authorization code link options. To be populated if `ValueType` is
    /// `AUTHORIZATION_CODE`
    #[prost(message, optional, tag="9")]
    pub authorization_code_link: ::core::option::Option<AuthorizationCodeLink>,
    /// State of the config variable.
    #[prost(enumeration="config_variable_template::State", tag="10")]
    pub state: i32,
}
/// Nested message and enum types in `ConfigVariableTemplate`.
pub mod config_variable_template {
    /// ValueType indicates the data type of the value.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ValueType {
        /// Value type is not specified.
        Unspecified = 0,
        /// Value type is string.
        String = 1,
        /// Value type is integer.
        Int = 2,
        /// Value type is boolean.
        Bool = 3,
        /// Value type is secret.
        Secret = 4,
        /// Value type is enum.
        Enum = 5,
        /// Value type is authorization code.
        AuthorizationCode = 6,
    }
    impl ValueType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueType::Unspecified => "VALUE_TYPE_UNSPECIFIED",
                ValueType::String => "STRING",
                ValueType::Int => "INT",
                ValueType::Bool => "BOOL",
                ValueType::Secret => "SECRET",
                ValueType::Enum => "ENUM",
                ValueType::AuthorizationCode => "AUTHORIZATION_CODE",
            }
        }
    }
    /// Indicates the state of the config variable.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Status is unspecified.
        Unspecified = 0,
        /// Config variable is active
        Active = 1,
        /// Config variable is deprecated.
        Deprecated = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
                State::Deprecated => "DEPRECATED",
            }
        }
    }
}
/// Secret provides a reference to entries in Secret Manager.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {
    /// The resource name of the secret version in the format,
    /// format as: `projects/*/secrets/*/versions/*`.
    #[prost(string, tag="1")]
    pub secret_version: ::prost::alloc::string::String,
}
/// EnumOption definition
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumOption {
    /// Id of the option.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Display name of the option.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
}
/// ConfigVariable represents a configuration variable present in a Connection.
/// or AuthConfig.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigVariable {
    /// Key of the config variable.
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    /// Value type of the config variable.
    #[prost(oneof="config_variable::Value", tags="2, 3, 4, 5")]
    pub value: ::core::option::Option<config_variable::Value>,
}
/// Nested message and enum types in `ConfigVariable`.
pub mod config_variable {
    /// Value type of the config variable.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// Value is an integer
        #[prost(int64, tag="2")]
        IntValue(i64),
        /// Value is a bool.
        #[prost(bool, tag="3")]
        BoolValue(bool),
        /// Value is a string.
        #[prost(string, tag="4")]
        StringValue(::prost::alloc::string::String),
        /// Value is a secret.
        #[prost(message, tag="5")]
        SecretValue(super::Secret),
    }
}
/// This configuration defines all the Cloud IAM roles that needs to be granted
/// to a particular GCP resource for the selected prinicpal like service
/// account. These configurations will let UI display to customers what
/// IAM roles need to be granted by them. Or these configurations can be used
/// by the UI to render a 'grant' button to do the same on behalf of the user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleGrant {
    /// Prinicipal/Identity for whom the role need to assigned.
    #[prost(enumeration="role_grant::Principal", tag="1")]
    pub principal: i32,
    /// List of roles that need to be granted.
    #[prost(string, repeated, tag="2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Resource on which the roles needs to be granted for the principal.
    #[prost(message, optional, tag="3")]
    pub resource: ::core::option::Option<role_grant::Resource>,
    /// Template that UI can use to provide helper text to customers.
    #[prost(string, tag="4")]
    pub helper_text_template: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RoleGrant`.
pub mod role_grant {
    /// Resource definition
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        /// Different types of resource supported.
        #[prost(enumeration="resource::Type", tag="1")]
        pub r#type: i32,
        /// Template to uniquely represent a GCP resource in a format IAM expects
        /// This is a template that can have references to other values provided in
        /// the config variable template.
        #[prost(string, tag="3")]
        pub path_template: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Resource`.
    pub mod resource {
        /// Resource Type definition.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Type {
            /// Value type is not specified.
            Unspecified = 0,
            /// GCP Project Resource.
            GcpProject = 1,
            /// Any GCP Resource which is identified uniquely by IAM.
            GcpResource = 2,
            /// GCP Secret Resource.
            GcpSecretmanagerSecret = 3,
            /// GCP Secret Version Resource.
            GcpSecretmanagerSecretVersion = 4,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unspecified => "TYPE_UNSPECIFIED",
                    Type::GcpProject => "GCP_PROJECT",
                    Type::GcpResource => "GCP_RESOURCE",
                    Type::GcpSecretmanagerSecret => "GCP_SECRETMANAGER_SECRET",
                    Type::GcpSecretmanagerSecretVersion => "GCP_SECRETMANAGER_SECRET_VERSION",
                }
            }
        }
    }
    /// Supported Principal values.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Principal {
        /// Value type is not specified.
        Unspecified = 0,
        /// Service Account used for Connector workload identity
        /// This is either the default service account if unspecified or Service
        /// Account provided by Customers through BYOSA.
        ConnectorSa = 1,
    }
    impl Principal {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Principal::Unspecified => "PRINCIPAL_UNSPECIFIED",
                Principal::ConnectorSa => "CONNECTOR_SA",
            }
        }
    }
}
/// This configuration captures the details required to render an authorization
/// link for the OAuth Authorization Code Flow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationCodeLink {
    /// The base URI the user must click to trigger the authorization code login
    /// flow.
    #[prost(string, tag="1")]
    pub uri: ::prost::alloc::string::String,
    /// The scopes for which the user will authorize GCP Connectors on the
    /// connector data source.
    #[prost(string, repeated, tag="2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The client ID assigned to the GCP Connectors OAuth app for the connector
    /// data source.
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    /// Whether to enable PKCE for the auth code flow.
    #[prost(bool, tag="4")]
    pub enable_pkce: bool,
}
/// LaunchStage is a enum to indicate launch stage:
/// PREVIEW, GA, DEPRECATED.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LaunchStage {
    /// LAUNCH_STAGE_UNSPECIFIED.
    Unspecified = 0,
    /// PREVIEW.
    Preview = 1,
    /// GA.
    Ga = 2,
    /// DEPRECATED.
    Deprecated = 3,
}
impl LaunchStage {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LaunchStage::Unspecified => "LAUNCH_STAGE_UNSPECIFIED",
            LaunchStage::Preview => "PREVIEW",
            LaunchStage::Ga => "GA",
            LaunchStage::Deprecated => "DEPRECATED",
        }
    }
}
/// AuthConfig defines details of a authentication type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthConfig {
    /// The type of authentication configured.
    #[prost(enumeration="AuthType", tag="1")]
    pub auth_type: i32,
    /// List containing additional auth configs.
    #[prost(message, repeated, tag="5")]
    pub additional_variables: ::prost::alloc::vec::Vec<ConfigVariable>,
    /// Supported auth types.
    #[prost(oneof="auth_config::Type", tags="2, 3, 4, 6")]
    pub r#type: ::core::option::Option<auth_config::Type>,
}
/// Nested message and enum types in `AuthConfig`.
pub mod auth_config {
    /// Parameters to support Username and Password Authentication.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserPassword {
        /// Username.
        #[prost(string, tag="1")]
        pub username: ::prost::alloc::string::String,
        /// Secret version reference containing the password.
        #[prost(message, optional, tag="2")]
        pub password: ::core::option::Option<super::Secret>,
    }
    /// Parameters to support JSON Web Token (JWT) Profile for Oauth 2.0
    /// Authorization Grant based authentication.
    /// See <https://tools.ietf.org/html/rfc7523> for more details.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Oauth2JwtBearer {
        /// Secret version reference containing a PKCS#8 PEM-encoded private
        /// key associated with the Client Certificate. This private key will be
        /// used to sign JWTs used for the jwt-bearer authorization grant.
        /// Specified in the form as: `projects/*/secrets/*/versions/*`.
        #[prost(message, optional, tag="1")]
        pub client_key: ::core::option::Option<super::Secret>,
        /// JwtClaims providers fields to generate the token.
        #[prost(message, optional, tag="2")]
        pub jwt_claims: ::core::option::Option<oauth2_jwt_bearer::JwtClaims>,
    }
    /// Nested message and enum types in `Oauth2JwtBearer`.
    pub mod oauth2_jwt_bearer {
        /// JWT claims used for the jwt-bearer authorization grant.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct JwtClaims {
            /// Value for the "iss" claim.
            #[prost(string, tag="1")]
            pub issuer: ::prost::alloc::string::String,
            /// Value for the "sub" claim.
            #[prost(string, tag="2")]
            pub subject: ::prost::alloc::string::String,
            /// Value for the "aud" claim.
            #[prost(string, tag="3")]
            pub audience: ::prost::alloc::string::String,
        }
    }
    /// Parameters to support Oauth 2.0 Client Credentials Grant Authentication.
    /// See <https://tools.ietf.org/html/rfc6749#section-1.3.4> for more details.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Oauth2ClientCredentials {
        /// The client identifier.
        #[prost(string, tag="1")]
        pub client_id: ::prost::alloc::string::String,
        /// Secret version reference containing the client secret.
        #[prost(message, optional, tag="2")]
        pub client_secret: ::core::option::Option<super::Secret>,
    }
    /// Parameters to support Ssh public key Authentication.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SshPublicKey {
        /// The user account used to authenticate.
        #[prost(string, tag="1")]
        pub username: ::prost::alloc::string::String,
        /// This is an optional field used in case client has enabled multi-factor
        /// authentication
        #[prost(message, optional, tag="2")]
        pub password: ::core::option::Option<super::Secret>,
        /// SSH Client Cert. It should contain both public and private key.
        #[prost(message, optional, tag="3")]
        pub ssh_client_cert: ::core::option::Option<super::Secret>,
        /// Format of SSH Client cert.
        #[prost(string, tag="4")]
        pub cert_type: ::prost::alloc::string::String,
        /// Password (passphrase) for ssh client certificate if it has one.
        #[prost(message, optional, tag="5")]
        pub ssh_client_cert_pass: ::core::option::Option<super::Secret>,
    }
    /// Supported auth types.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// UserPassword.
        #[prost(message, tag="2")]
        UserPassword(UserPassword),
        /// Oauth2JwtBearer.
        #[prost(message, tag="3")]
        Oauth2JwtBearer(Oauth2JwtBearer),
        /// Oauth2ClientCredentials.
        #[prost(message, tag="4")]
        Oauth2ClientCredentials(Oauth2ClientCredentials),
        /// SSH Public Key.
        #[prost(message, tag="6")]
        SshPublicKey(SshPublicKey),
    }
}
/// AuthConfigTemplate defines required field over an authentication type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthConfigTemplate {
    /// The type of authentication configured.
    #[prost(enumeration="AuthType", tag="1")]
    pub auth_type: i32,
    /// Config variables to describe an `AuthConfig` for a `Connection`.
    #[prost(message, repeated, tag="2")]
    pub config_variable_templates: ::prost::alloc::vec::Vec<ConfigVariableTemplate>,
}
/// AuthType defines different authentication types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthType {
    /// Authentication type not specified.
    Unspecified = 0,
    /// Username and Password Authentication.
    UserPassword = 1,
    /// JSON Web Token (JWT) Profile for Oauth 2.0
    /// Authorization Grant based authentication
    Oauth2JwtBearer = 2,
    /// Oauth 2.0 Client Credentials Grant Authentication
    Oauth2ClientCredentials = 3,
    /// SSH Public Key Authentication
    SshPublicKey = 4,
}
impl AuthType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthType::Unspecified => "AUTH_TYPE_UNSPECIFIED",
            AuthType::UserPassword => "USER_PASSWORD",
            AuthType::Oauth2JwtBearer => "OAUTH2_JWT_BEARER",
            AuthType::Oauth2ClientCredentials => "OAUTH2_CLIENT_CREDENTIALS",
            AuthType::SshPublicKey => "SSH_PUBLIC_KEY",
        }
    }
}
/// ConnectorVersion indicates a specific version of a connector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectorVersion {
    /// Output only. Resource name of the Version.
    /// Format:
    /// projects/{project}/locations/{location}/providers/{provider}/connectors/{connector}/versions/{version}
    /// Only global location is supported for Connector resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Created time.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Updated time.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Resource labels to represent user-provided metadata.
    /// Refer to cloud documentation on labels for more details.
    /// <https://cloud.google.com/compute/docs/labeling-resources>
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Flag to mark the version indicating the launch stage.
    #[prost(enumeration="LaunchStage", tag="6")]
    pub launch_stage: i32,
    /// Output only. ReleaseVersion of the connector, for example: "1.0.1-alpha".
    #[prost(string, tag="7")]
    pub release_version: ::prost::alloc::string::String,
    /// Output only. List of auth configs supported by the Connector Version.
    #[prost(message, repeated, tag="8")]
    pub auth_config_templates: ::prost::alloc::vec::Vec<AuthConfigTemplate>,
    /// Output only. List of config variables needed to create a connection.
    #[prost(message, repeated, tag="9")]
    pub config_variable_templates: ::prost::alloc::vec::Vec<ConfigVariableTemplate>,
    /// Output only. Information about the runtime features supported by the Connector.
    #[prost(message, optional, tag="10")]
    pub supported_runtime_features: ::core::option::Option<SupportedRuntimeFeatures>,
    /// Output only. Display name.
    #[prost(string, tag="11")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Configuration for Egress Control.
    #[prost(message, optional, tag="12")]
    pub egress_control_config: ::core::option::Option<EgressControlConfig>,
    /// Output only. Role grant configurations for this connector version.
    #[prost(message, repeated, tag="14")]
    pub role_grants: ::prost::alloc::vec::Vec<RoleGrant>,
    /// Output only. Role grant configuration for this config variable. It will be DEPRECATED
    /// soon.
    #[prost(message, optional, tag="15")]
    pub role_grant: ::core::option::Option<RoleGrant>,
}
/// Request message for Connectors.GetConnectorVersion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectorVersionRequest {
    /// Required. Resource name of the form:
    /// `projects/*/locations/*/providers/*/connectors/*/versions/*`
    /// Only global location is supported for ConnectorVersion resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Specifies which fields of the ConnectorVersion are returned in the
    /// response. Defaults to `CUSTOMER` view.
    #[prost(enumeration="ConnectorVersionView", tag="2")]
    pub view: i32,
}
/// Request message for Connectors.ListConnectorVersions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectorVersionsRequest {
    /// Required. Parent resource of the connectors, of the form:
    /// `projects/*/locations/*/providers/*/connectors/*`
    /// Only global location is supported for ConnectorVersion resource.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Page size.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Specifies which fields of the ConnectorVersion are returned in the
    /// response. Defaults to `BASIC` view.
    #[prost(enumeration="ConnectorVersionView", tag="4")]
    pub view: i32,
}
/// Response message for Connectors.ListConnectorVersions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectorVersionsResponse {
    /// A list of connector versions.
    #[prost(message, repeated, tag="1")]
    pub connector_versions: ::prost::alloc::vec::Vec<ConnectorVersion>,
    /// Next page token.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Supported runtime features of a connector version. This is passed to the
/// management layer to add a new connector version by the connector developer.
/// Details about how this proto is passed to the management layer is covered in
/// this doc - go/runtime-manifest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupportedRuntimeFeatures {
    /// Specifies if the connector supports entity apis like 'createEntity'.
    #[prost(bool, tag="1")]
    pub entity_apis: bool,
    /// Specifies if the connector supports action apis like 'executeAction'.
    #[prost(bool, tag="2")]
    pub action_apis: bool,
    /// Specifies if the connector supports 'ExecuteSqlQuery' operation.
    #[prost(bool, tag="3")]
    pub sql_query: bool,
}
/// Egress control config for connector runtime. These configurations define the
/// rules to identify which outbound domains/hosts needs to be whitelisted. It
/// may be a static information for a particular connector version or it is
/// derived from the configurations provided by the customer in Connection
/// resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EgressControlConfig {
    #[prost(oneof="egress_control_config::OneofBackends", tags="1, 2")]
    pub oneof_backends: ::core::option::Option<egress_control_config::OneofBackends>,
}
/// Nested message and enum types in `EgressControlConfig`.
pub mod egress_control_config {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneofBackends {
        /// Static Comma separated backends which are common for all Connection
        /// resources. Supported formats for each backend are host:port or just
        /// host (host can be ip address or domain name).
        #[prost(string, tag="1")]
        Backends(::prost::alloc::string::String),
        /// Extractions Rules to extract the backends from customer provided
        /// configuration.
        #[prost(message, tag="2")]
        ExtractionRules(super::ExtractionRules),
    }
}
/// Extraction Rules to identity the backends from customer provided
/// configuration in Connection resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtractionRules {
    /// Collection of Extraction Rule.
    #[prost(message, repeated, tag="1")]
    pub extraction_rule: ::prost::alloc::vec::Vec<ExtractionRule>,
}
/// Extraction Rule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtractionRule {
    /// Source on which the rule is applied.
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<extraction_rule::Source>,
    /// Regex used to extract backend details from source. If empty, whole source
    /// value will be used.
    #[prost(string, tag="2")]
    pub extraction_regex: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ExtractionRule`.
pub mod extraction_rule {
    /// Source to extract the backend from.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Source {
        /// Type of the source.
        #[prost(enumeration="SourceType", tag="1")]
        pub source_type: i32,
        /// Field identifier. For example config vaiable name.
        #[prost(string, tag="2")]
        pub field_id: ::prost::alloc::string::String,
    }
    /// Supported Source types for extraction.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SourceType {
        /// Default SOURCE.
        Unspecified = 0,
        /// Config Variable source type.
        ConfigVariable = 1,
    }
    impl SourceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SourceType::Unspecified => "SOURCE_TYPE_UNSPECIFIED",
                SourceType::ConfigVariable => "CONFIG_VARIABLE",
            }
        }
    }
}
/// Enum to control which fields should be included in the response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConnectorVersionView {
    /// CONNECTOR_VERSION_VIEW_UNSPECIFIED.
    Unspecified = 0,
    /// Do not include role grant configs.
    Basic = 1,
    /// Include role grant configs.
    Full = 2,
}
impl ConnectorVersionView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConnectorVersionView::Unspecified => "CONNECTOR_VERSION_VIEW_UNSPECIFIED",
            ConnectorVersionView::Basic => "CONNECTOR_VERSION_VIEW_BASIC",
            ConnectorVersionView::Full => "CONNECTOR_VERSION_VIEW_FULL",
        }
    }
}
/// Define the Connectors target endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationConfig {
    /// The key is the destination identifier that is supported by the Connector.
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    /// The destinations for the key.
    #[prost(message, repeated, tag="2")]
    pub destinations: ::prost::alloc::vec::Vec<Destination>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Destination {
    /// The port is the target port number that is accepted by the destination.
    #[prost(int32, tag="3")]
    pub port: i32,
    #[prost(oneof="destination::Destination", tags="1, 2")]
    pub destination: ::core::option::Option<destination::Destination>,
}
/// Nested message and enum types in `Destination`.
pub mod destination {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// PSC service attachments.
        /// Format: projects/*/regions/*/serviceAttachments/*
        #[prost(string, tag="1")]
        ServiceAttachment(::prost::alloc::string::String),
        /// For publicly routable host.
        #[prost(string, tag="2")]
        Host(::prost::alloc::string::String),
    }
}
/// Connection represents an instance of connector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    /// Output only. Resource name of the Connection.
    /// Format: projects/{project}/locations/{location}/connections/{connection}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Created time.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Updated time.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Resource labels to represent user-provided metadata.
    /// Refer to cloud documentation on labels for more details.
    /// <https://cloud.google.com/compute/docs/labeling-resources>
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Description of the resource.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Required. Connector version on which the connection is created.
    /// The format is:
    /// projects/*/locations/*/providers/*/connectors/*/versions/*
    /// Only global location is supported for ConnectorVersion resource.
    #[prost(string, tag="6")]
    pub connector_version: ::prost::alloc::string::String,
    /// Output only. Current status of the connection.
    #[prost(message, optional, tag="7")]
    pub status: ::core::option::Option<ConnectionStatus>,
    /// Optional. Configuration for configuring the connection with an external system.
    #[prost(message, repeated, tag="8")]
    pub config_variables: ::prost::alloc::vec::Vec<ConfigVariable>,
    /// Optional. Configuration for establishing the connection's authentication with an
    /// external system.
    #[prost(message, optional, tag="9")]
    pub auth_config: ::core::option::Option<AuthConfig>,
    /// Optional. Configuration that indicates whether or not the Connection can be edited.
    #[prost(message, optional, tag="10")]
    pub lock_config: ::core::option::Option<LockConfig>,
    /// Optional. Configuration of the Connector's destination. Only accepted for Connectors
    /// that accepts user defined destination(s).
    #[prost(message, repeated, tag="18")]
    pub destination_configs: ::prost::alloc::vec::Vec<DestinationConfig>,
    /// Output only. GCR location where the runtime image is stored.
    /// formatted like: gcr.io/{bucketName}/{imageName}
    #[prost(string, tag="11")]
    pub image_location: ::prost::alloc::string::String,
    /// Optional. Service account needed for runtime plane to access GCP resources.
    #[prost(string, tag="12")]
    pub service_account: ::prost::alloc::string::String,
    /// Output only. The name of the Service Directory service name. Used for
    /// Private Harpoon to resolve the ILB address.
    /// e.g.
    /// "projects/cloud-connectors-e2e-testing/locations/us-central1/namespaces/istio-system/services/istio-ingressgateway-connectors"
    #[prost(string, tag="13")]
    pub service_directory: ::prost::alloc::string::String,
    /// Output only. GCR location where the envoy image is stored.
    /// formatted like: gcr.io/{bucketName}/{imageName}
    #[prost(string, tag="15")]
    pub envoy_image_location: ::prost::alloc::string::String,
    /// Optional. Suspended indicates if a user has suspended a connection or not.
    #[prost(bool, tag="17")]
    pub suspended: bool,
    /// Optional. Configuration for the connection.
    #[prost(message, optional, tag="19")]
    pub node_config: ::core::option::Option<NodeConfig>,
}
/// Configuration for the connection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeConfig {
    /// Minimum number of nodes in the runtime nodes.
    #[prost(int32, tag="1")]
    pub min_node_count: i32,
    /// Maximum number of nodes in the runtime nodes.
    #[prost(int32, tag="2")]
    pub max_node_count: i32,
}
/// Metadata of connection schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionSchemaMetadata {
    /// Output only. List of entity names.
    #[prost(string, repeated, tag="1")]
    pub entities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. List of actions.
    #[prost(string, repeated, tag="2")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Schema of a runtime entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeEntitySchema {
    /// Output only. Name of the entity.
    #[prost(string, tag="1")]
    pub entity: ::prost::alloc::string::String,
    /// Output only. List of fields in the entity.
    #[prost(message, repeated, tag="2")]
    pub fields: ::prost::alloc::vec::Vec<runtime_entity_schema::Field>,
}
/// Nested message and enum types in `RuntimeEntitySchema`.
pub mod runtime_entity_schema {
    /// Metadata of an entity field.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Field {
        /// Name of the Field.
        #[prost(string, tag="1")]
        pub field: ::prost::alloc::string::String,
        /// A brief description of the Field.
        #[prost(string, tag="2")]
        pub description: ::prost::alloc::string::String,
        /// The data type of the Field.
        #[prost(enumeration="super::DataType", tag="3")]
        pub data_type: i32,
        /// The following boolean field specifies if the current Field acts
        /// as a primary key or id if the parent is of type entity.
        #[prost(bool, tag="4")]
        pub key: bool,
        /// Specifies if the Field is readonly.
        #[prost(bool, tag="5")]
        pub readonly: bool,
        /// Specifies whether a null value is allowed.
        #[prost(bool, tag="6")]
        pub nullable: bool,
        /// The following field specifies the default value of the Field provided
        /// by the external system if a value is not provided.
        #[prost(message, optional, tag="7")]
        pub default_value: ::core::option::Option<::prost_types::Value>,
        /// The following map contains fields that are not explicitly mentioned
        /// above,this give connectors the flexibility to add new metadata
        /// fields.
        #[prost(message, optional, tag="8")]
        pub additional_details: ::core::option::Option<::prost_types::Struct>,
    }
}
/// Schema of a runtime action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeActionSchema {
    /// Output only. Name of the action.
    #[prost(string, tag="1")]
    pub action: ::prost::alloc::string::String,
    /// Output only. List of input parameter metadata for the action.
    #[prost(message, repeated, tag="2")]
    pub input_parameters: ::prost::alloc::vec::Vec<runtime_action_schema::InputParameter>,
    /// Output only. List of result field metadata.
    #[prost(message, repeated, tag="3")]
    pub result_metadata: ::prost::alloc::vec::Vec<runtime_action_schema::ResultMetadata>,
}
/// Nested message and enum types in `RuntimeActionSchema`.
pub mod runtime_action_schema {
    /// Metadata of an input parameter.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InputParameter {
        /// Name of the Parameter.
        #[prost(string, tag="1")]
        pub parameter: ::prost::alloc::string::String,
        /// A brief description of the Parameter.
        #[prost(string, tag="2")]
        pub description: ::prost::alloc::string::String,
        /// The data type of the Parameter.
        #[prost(enumeration="super::DataType", tag="3")]
        pub data_type: i32,
        /// Specifies whether a null value is allowed.
        #[prost(bool, tag="4")]
        pub nullable: bool,
        /// The following field specifies the default value of the Parameter
        /// provided by the external system if a value is not provided.
        #[prost(message, optional, tag="5")]
        pub default_value: ::core::option::Option<::prost_types::Value>,
    }
    /// Metadata of result field.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResultMetadata {
        /// Name of the result field.
        #[prost(string, tag="1")]
        pub field: ::prost::alloc::string::String,
        /// A brief description of the field.
        #[prost(string, tag="2")]
        pub description: ::prost::alloc::string::String,
        /// The data type of the field.
        #[prost(enumeration="super::DataType", tag="3")]
        pub data_type: i32,
    }
}
/// Determines whether or no a connection is locked. If locked, a reason must be
/// specified.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockConfig {
    /// Indicates whether or not the connection is locked.
    #[prost(bool, tag="1")]
    pub locked: bool,
    /// Describes why a connection is locked.
    #[prost(string, tag="2")]
    pub reason: ::prost::alloc::string::String,
}
/// Request message for ConnectorsService.ListConnections
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsRequest {
    /// Required. Parent resource of the Connection, of the form:
    /// `projects/*/locations/*`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Page size.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Order by parameters.
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
    /// Specifies which fields of the Connection are returned in the response.
    /// Defaults to `BASIC` view.
    #[prost(enumeration="ConnectionView", tag="6")]
    pub view: i32,
}
/// Response message for ConnectorsService.ListConnections
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsResponse {
    /// Connections.
    #[prost(message, repeated, tag="1")]
    pub connections: ::prost::alloc::vec::Vec<Connection>,
    /// Next page token.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for ConnectorsService.GetConnection
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectionRequest {
    /// Required. Resource name of the form:
    /// `projects/*/locations/*/connections/*`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Specifies which fields of the Connection are returned in the response.
    /// Defaults to `BASIC` view.
    #[prost(enumeration="ConnectionView", tag="2")]
    pub view: i32,
}
/// Request message for ConnectorsService.CreateConnection
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectionRequest {
    /// Required. Parent resource of the Connection, of the form:
    /// `projects/*/locations/*`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Identifier to assign to the Connection. Must be unique within scope of
    /// the parent resource.
    #[prost(string, tag="2")]
    pub connection_id: ::prost::alloc::string::String,
    /// Required. Connection resource.
    #[prost(message, optional, tag="3")]
    pub connection: ::core::option::Option<Connection>,
}
/// Request message for ConnectorsService.UpdateConnection
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConnectionRequest {
    /// Required. Connection resource.
    #[prost(message, optional, tag="1")]
    pub connection: ::core::option::Option<Connection>,
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Connection resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ConnectorsService.DeleteConnection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectionRequest {
    /// Required. Resource name of the form:
    /// `projects/*/locations/*/connections/*`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ConnectorsService.GetConnectionSchemaMetadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectionSchemaMetadataRequest {
    /// Required. Connection name
    /// Format:
    /// projects/{project}/locations/{location}/connections/{connection}/connectionSchemaMetadata
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ConnectorsService.ListRuntimeEntitySchemas.
/// For filter, only entity field is supported with literal equality operator.
/// Accepted filter example: entity="Order"
/// Wildcards are not supported in the filter currently.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimeEntitySchemasRequest {
    /// Required. Parent resource of RuntimeEntitySchema
    /// Format:
    /// projects/{project}/locations/{location}/connections/{connection}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Page size.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Required. Filter
    /// Format:
    /// entity="{entityId}"
    /// Only entity field is supported with literal equality operator.
    /// Accepted filter example: entity="Order"
    /// Wildcards are not supported in the filter currently.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for ConnectorsService.ListRuntimeEntitySchemas.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimeEntitySchemasResponse {
    /// Runtime entity schemas.
    #[prost(message, repeated, tag="1")]
    pub runtime_entity_schemas: ::prost::alloc::vec::Vec<RuntimeEntitySchema>,
    /// Next page token.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ConnectorsService.ListRuntimeActionSchemas.
/// For filter, only action field is supported with literal equality operator.
/// Accepted filter example: action="approveOrder"
/// Wildcards are not supported in the filter currently.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimeActionSchemasRequest {
    /// Required. Parent resource of RuntimeActionSchema
    /// Format:
    /// projects/{project}/locations/{location}/connections/{connection}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Page size.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Required. Filter
    /// Format:
    /// action="{actionId}"
    /// Only action field is supported with literal equality operator.
    /// Accepted filter example: action="CancelOrder"
    /// Wildcards are not supported in the filter currently.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for ConnectorsService.ListRuntimeActionSchemas.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimeActionSchemasResponse {
    /// Runtime action schemas.
    #[prost(message, repeated, tag="1")]
    pub runtime_action_schemas: ::prost::alloc::vec::Vec<RuntimeActionSchema>,
    /// Next page token.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// ConnectionStatus indicates the state of the connection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionStatus {
    /// State.
    #[prost(enumeration="connection_status::State", tag="1")]
    pub state: i32,
    /// Description.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Status provides detailed information for the state.
    #[prost(string, tag="3")]
    pub status: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ConnectionStatus`.
pub mod connection_status {
    /// All the possible Connection State.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Connection does not have a state yet.
        Unspecified = 0,
        /// Connection is being created.
        Creating = 1,
        /// Connection is running and ready for requests.
        Active = 2,
        /// Connection is stopped.
        Inactive = 3,
        /// Connection is being deleted.
        Deleting = 4,
        /// Connection is being updated.
        Updating = 5,
        /// Connection is not running due to an error.
        Error = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Active => "ACTIVE",
                State::Inactive => "INACTIVE",
                State::Deleting => "DELETING",
                State::Updating => "UPDATING",
                State::Error => "ERROR",
            }
        }
    }
}
/// All possible data types of a entity or action field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataType {
    /// Data type is not specified.
    Unspecified = 0,
    /// DEPRECATED! Use DATA_TYPE_INTEGER.
    Int = 1,
    /// Short integer(int16) data type.
    Smallint = 2,
    /// Double data type.
    Double = 3,
    /// Date data type.
    Date = 4,
    /// DEPRECATED! Use DATA_TYPE_TIMESTAMP.
    Datetime = 5,
    /// Time data type.
    Time = 6,
    /// DEPRECATED! Use DATA_TYPE_VARCHAR.
    String = 7,
    /// DEPRECATED! Use DATA_TYPE_BIGINT.
    Long = 8,
    /// Boolean data type.
    Boolean = 9,
    /// Decimal data type.
    Decimal = 10,
    /// DEPRECATED! Use DATA_TYPE_VARCHAR.
    Uuid = 11,
    /// UNSUPPORTED! Binary data type.
    Blob = 12,
    /// Bit data type.
    Bit = 13,
    /// Small integer(int8) data type.
    Tinyint = 14,
    /// Integer(int32) data type.
    Integer = 15,
    /// Long integer(int64) data type.
    Bigint = 16,
    /// Float data type.
    Float = 17,
    /// Real data type.
    Real = 18,
    /// Numeric data type.
    Numeric = 19,
    /// Char data type.
    Char = 20,
    /// Varchar data type.
    Varchar = 21,
    /// Longvarchar data type.
    Longvarchar = 22,
    /// Timestamp data type.
    Timestamp = 23,
    /// Nchar data type.
    Nchar = 24,
    /// Nvarchar data type.
    Nvarchar = 25,
    /// Longnvarchar data type.
    Longnvarchar = 26,
    /// Null data type.
    Null = 27,
    /// UNSUPPORTED! Binary data type.
    Other = 28,
    /// UNSUPPORTED! Binary data type.
    JavaObject = 29,
    /// UNSUPPORTED! Binary data type.
    Distinct = 30,
    /// UNSUPPORTED! Binary data type.
    Struct = 31,
    /// UNSUPPORTED! Binary data type.
    Array = 32,
    /// UNSUPPORTED! Binary data type.
    Clob = 33,
    /// UNSUPPORTED! Binary data type.
    Ref = 34,
    /// UNSUPPORTED! Binary data type.
    Datalink = 35,
    /// UNSUPPORTED! Row id data type.
    Rowid = 36,
    /// UNSUPPORTED! Binary data type.
    Binary = 37,
    /// UNSUPPORTED! Variable binary data type.
    Varbinary = 38,
    /// UNSUPPORTED! Long variable binary data type.
    Longvarbinary = 39,
    /// UNSUPPORTED! NCLOB data type.
    Nclob = 40,
    /// UNSUPPORTED! SQL XML data type is not supported.
    Sqlxml = 41,
    /// UNSUPPORTED! Cursor reference type is not supported.
    RefCursor = 42,
    /// UNSUPPORTED! Use TIME or TIMESTAMP instead.
    TimeWithTimezone = 43,
    /// UNSUPPORTED! Use TIMESTAMP instead.
    TimestampWithTimezone = 44,
}
impl DataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataType::Unspecified => "DATA_TYPE_UNSPECIFIED",
            DataType::Int => "DATA_TYPE_INT",
            DataType::Smallint => "DATA_TYPE_SMALLINT",
            DataType::Double => "DATA_TYPE_DOUBLE",
            DataType::Date => "DATA_TYPE_DATE",
            DataType::Datetime => "DATA_TYPE_DATETIME",
            DataType::Time => "DATA_TYPE_TIME",
            DataType::String => "DATA_TYPE_STRING",
            DataType::Long => "DATA_TYPE_LONG",
            DataType::Boolean => "DATA_TYPE_BOOLEAN",
            DataType::Decimal => "DATA_TYPE_DECIMAL",
            DataType::Uuid => "DATA_TYPE_UUID",
            DataType::Blob => "DATA_TYPE_BLOB",
            DataType::Bit => "DATA_TYPE_BIT",
            DataType::Tinyint => "DATA_TYPE_TINYINT",
            DataType::Integer => "DATA_TYPE_INTEGER",
            DataType::Bigint => "DATA_TYPE_BIGINT",
            DataType::Float => "DATA_TYPE_FLOAT",
            DataType::Real => "DATA_TYPE_REAL",
            DataType::Numeric => "DATA_TYPE_NUMERIC",
            DataType::Char => "DATA_TYPE_CHAR",
            DataType::Varchar => "DATA_TYPE_VARCHAR",
            DataType::Longvarchar => "DATA_TYPE_LONGVARCHAR",
            DataType::Timestamp => "DATA_TYPE_TIMESTAMP",
            DataType::Nchar => "DATA_TYPE_NCHAR",
            DataType::Nvarchar => "DATA_TYPE_NVARCHAR",
            DataType::Longnvarchar => "DATA_TYPE_LONGNVARCHAR",
            DataType::Null => "DATA_TYPE_NULL",
            DataType::Other => "DATA_TYPE_OTHER",
            DataType::JavaObject => "DATA_TYPE_JAVA_OBJECT",
            DataType::Distinct => "DATA_TYPE_DISTINCT",
            DataType::Struct => "DATA_TYPE_STRUCT",
            DataType::Array => "DATA_TYPE_ARRAY",
            DataType::Clob => "DATA_TYPE_CLOB",
            DataType::Ref => "DATA_TYPE_REF",
            DataType::Datalink => "DATA_TYPE_DATALINK",
            DataType::Rowid => "DATA_TYPE_ROWID",
            DataType::Binary => "DATA_TYPE_BINARY",
            DataType::Varbinary => "DATA_TYPE_VARBINARY",
            DataType::Longvarbinary => "DATA_TYPE_LONGVARBINARY",
            DataType::Nclob => "DATA_TYPE_NCLOB",
            DataType::Sqlxml => "DATA_TYPE_SQLXML",
            DataType::RefCursor => "DATA_TYPE_REF_CURSOR",
            DataType::TimeWithTimezone => "DATA_TYPE_TIME_WITH_TIMEZONE",
            DataType::TimestampWithTimezone => "DATA_TYPE_TIMESTAMP_WITH_TIMEZONE",
        }
    }
}
/// Enum to control which fields should be included in the response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConnectionView {
    /// CONNECTION_UNSPECIFIED.
    Unspecified = 0,
    /// Do not include runtime required configs.
    Basic = 1,
    /// Include runtime required configs.
    Full = 2,
}
impl ConnectionView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConnectionView::Unspecified => "CONNECTION_VIEW_UNSPECIFIED",
            ConnectionView::Basic => "BASIC",
            ConnectionView::Full => "FULL",
        }
    }
}
/// Provider indicates the owner who provides the connectors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Provider {
    /// Output only. Resource name of the Provider.
    /// Format: projects/{project}/locations/{location}/providers/{provider}
    /// Only global location is supported for Provider resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Created time.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Updated time.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Resource labels to represent user-provided metadata.
    /// Refer to cloud documentation on labels for more details.
    /// <https://cloud.google.com/compute/docs/labeling-resources>
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Link to documentation page.
    #[prost(string, tag="6")]
    pub documentation_uri: ::prost::alloc::string::String,
    /// Output only. Link to external page.
    #[prost(string, tag="7")]
    pub external_uri: ::prost::alloc::string::String,
    /// Output only. Description of the resource.
    #[prost(string, tag="8")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Cloud storage location of icons etc consumed by UI.
    #[prost(string, tag="9")]
    pub web_assets_location: ::prost::alloc::string::String,
    /// Output only. Display name.
    #[prost(string, tag="10")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Flag to mark the version indicating the launch stage.
    #[prost(enumeration="LaunchStage", tag="11")]
    pub launch_stage: i32,
}
/// Request message for Connectors.GetProvider.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProviderRequest {
    /// Required. Resource name of the form:
    /// `projects/*/locations/*/providers/*`
    /// Only global location is supported for Provider resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for Connectors.ListProviders.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProvidersRequest {
    /// Required. Parent resource of the API, of the form:
    /// `projects/*/locations/*`
    /// Only global location is supported for Provider resource.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Page size.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for Connectors.ListProviders.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProvidersResponse {
    /// A list of providers.
    #[prost(message, repeated, tag="1")]
    pub providers: ::prost::alloc::vec::Vec<Provider>,
    /// Next page token.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Connectors indicates a specific connector type, e.x. Salesforce, SAP etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connector {
    /// Output only. Resource name of the Connector.
    /// Format:
    /// projects/{project}/locations/{location}/providers/{provider}/connectors/{connector}
    /// Only global location is supported for Connector resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Created time.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Updated time.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Resource labels to represent user-provided metadata.
    /// Refer to cloud documentation on labels for more details.
    /// <https://cloud.google.com/compute/docs/labeling-resources>
    #[prost(btree_map="string, string", tag="4")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Link to documentation page.
    #[prost(string, tag="6")]
    pub documentation_uri: ::prost::alloc::string::String,
    /// Output only. Link to external page.
    #[prost(string, tag="7")]
    pub external_uri: ::prost::alloc::string::String,
    /// Output only. Description of the resource.
    #[prost(string, tag="8")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Cloud storage location of icons etc consumed by UI.
    #[prost(string, tag="9")]
    pub web_assets_location: ::prost::alloc::string::String,
    /// Output only. Display name.
    #[prost(string, tag="10")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Flag to mark the version indicating the launch stage.
    #[prost(enumeration="LaunchStage", tag="11")]
    pub launch_stage: i32,
}
/// Request message for Connectors.GetConnector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectorRequest {
    /// Required. Resource name of the form:
    /// `projects/*/locations/*/providers/*/connectors/*`
    /// Only global location is supported for Connector resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for Connectors.ListConnectors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectorsRequest {
    /// Required. Parent resource of the connectors, of the form:
    /// `projects/*/locations/*/providers/*`
    /// Only global location is supported for Connector resource.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Page size.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for Connectors.ListConnectors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectorsResponse {
    /// A list of connectors.
    #[prost(message, repeated, tag="1")]
    pub connectors: ::prost::alloc::vec::Vec<Connector>,
    /// Next page token.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod connectors_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Connectors is the interface for managing Connectors.
    #[derive(Debug, Clone)]
    pub struct ConnectorsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConnectorsClient<T>
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
        ) -> ConnectorsClient<InterceptedService<T, F>>
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
            ConnectorsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists Connections in a given project and location.
        pub async fn list_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectionsRequest>,
        ) -> Result<tonic::Response<super::ListConnectionsResponse>, tonic::Status> {
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
                "/google.cloud.connectors.v1.Connectors/ListConnections",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Connection.
        pub async fn get_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectionRequest>,
        ) -> Result<tonic::Response<super::Connection>, tonic::Status> {
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
                "/google.cloud.connectors.v1.Connectors/GetConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Connection in a given project and location.
        pub async fn create_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.connectors.v1.Connectors/CreateConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Connection.
        pub async fn update_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConnectionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.connectors.v1.Connectors/UpdateConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Connection.
        pub async fn delete_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.connectors.v1.Connectors/DeleteConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Providers in a given project and location.
        pub async fn list_providers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProvidersRequest>,
        ) -> Result<tonic::Response<super::ListProvidersResponse>, tonic::Status> {
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
                "/google.cloud.connectors.v1.Connectors/ListProviders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a provider.
        pub async fn get_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProviderRequest>,
        ) -> Result<tonic::Response<super::Provider>, tonic::Status> {
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
                "/google.cloud.connectors.v1.Connectors/GetProvider",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Connectors in a given project and location.
        pub async fn list_connectors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectorsRequest>,
        ) -> Result<tonic::Response<super::ListConnectorsResponse>, tonic::Status> {
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
                "/google.cloud.connectors.v1.Connectors/ListConnectors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Connector.
        pub async fn get_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectorRequest>,
        ) -> Result<tonic::Response<super::Connector>, tonic::Status> {
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
                "/google.cloud.connectors.v1.Connectors/GetConnector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Connector Versions in a given project and location.
        pub async fn list_connector_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectorVersionsRequest>,
        ) -> Result<
            tonic::Response<super::ListConnectorVersionsResponse>,
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
                "/google.cloud.connectors.v1.Connectors/ListConnectorVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single connector version.
        pub async fn get_connector_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectorVersionRequest>,
        ) -> Result<tonic::Response<super::ConnectorVersion>, tonic::Status> {
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
                "/google.cloud.connectors.v1.Connectors/GetConnectorVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets schema metadata of a connection.
        /// SchemaMetadata is a singleton resource for each connection.
        pub async fn get_connection_schema_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectionSchemaMetadataRequest>,
        ) -> Result<tonic::Response<super::ConnectionSchemaMetadata>, tonic::Status> {
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
                "/google.cloud.connectors.v1.Connectors/GetConnectionSchemaMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List schema of a runtime entities filtered by entity name.
        pub async fn list_runtime_entity_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuntimeEntitySchemasRequest>,
        ) -> Result<
            tonic::Response<super::ListRuntimeEntitySchemasResponse>,
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
                "/google.cloud.connectors.v1.Connectors/ListRuntimeEntitySchemas",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List schema of a runtime actions filtered by action name.
        pub async fn list_runtime_action_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuntimeActionSchemasRequest>,
        ) -> Result<
            tonic::Response<super::ListRuntimeActionSchemasResponse>,
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
                "/google.cloud.connectors.v1.Connectors/ListRuntimeActionSchemas",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the runtimeConfig of a location.
        /// RuntimeConfig is a singleton resource for each location.
        pub async fn get_runtime_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRuntimeConfigRequest>,
        ) -> Result<tonic::Response<super::RuntimeConfig>, tonic::Status> {
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
                "/google.cloud.connectors.v1.Connectors/GetRuntimeConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
