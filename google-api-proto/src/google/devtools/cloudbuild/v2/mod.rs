/// A connection to a SCM like GitHub, GitHub Enterprise, Bitbucket Server or
/// GitLab.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    /// Immutable. The resource name of the connection, in the format
    /// `projects/{project}/locations/{location}/connections/{connection_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server assigned timestamp for when the connection was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server assigned timestamp for when the connection was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Installation state of the Connection.
    #[prost(message, optional, tag = "12")]
    pub installation_state: ::core::option::Option<InstallationState>,
    /// If disabled is set to true, functionality is disabled for this connection.
    /// Repository based API methods and webhooks processing for repositories in
    /// this connection will be disabled.
    #[prost(bool, tag = "13")]
    pub disabled: bool,
    /// Output only. Set to true when the connection is being set up or updated in
    /// the background.
    #[prost(bool, tag = "14")]
    pub reconciling: bool,
    /// Allows clients to store small amounts of arbitrary data.
    #[prost(btree_map = "string, string", tag = "15")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "16")]
    pub etag: ::prost::alloc::string::String,
    /// Configuration for the connection depending on the type of provider.
    #[prost(oneof = "connection::ConnectionConfig", tags = "5, 6, 7")]
    pub connection_config: ::core::option::Option<connection::ConnectionConfig>,
}
/// Nested message and enum types in `Connection`.
pub mod connection {
    /// Configuration for the connection depending on the type of provider.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConnectionConfig {
        /// Configuration for connections to github.com.
        #[prost(message, tag = "5")]
        GithubConfig(super::GitHubConfig),
        /// Configuration for connections to an instance of GitHub Enterprise.
        #[prost(message, tag = "6")]
        GithubEnterpriseConfig(super::GitHubEnterpriseConfig),
        /// Configuration for connections to gitlab.com or an instance of GitLab
        /// Enterprise.
        #[prost(message, tag = "7")]
        GitlabConfig(super::GitLabConfig),
    }
}
/// Describes stage and necessary actions to be taken by the
/// user to complete the installation. Used for GitHub and GitHub Enterprise
/// based connections.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallationState {
    /// Output only. Current step of the installation process.
    #[prost(enumeration = "installation_state::Stage", tag = "1")]
    pub stage: i32,
    /// Output only. Message of what the user should do next to continue the
    /// installation. Empty string if the installation is already complete.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// Output only. Link to follow for next action. Empty string if the
    /// installation is already complete.
    #[prost(string, tag = "3")]
    pub action_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `InstallationState`.
pub mod installation_state {
    /// Stage of the installation process.
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
    pub enum Stage {
        /// No stage specified.
        Unspecified = 0,
        /// Only for GitHub Enterprise. An App creation has been requested.
        /// The user needs to confirm the creation in their GitHub enterprise host.
        PendingCreateApp = 1,
        /// User needs to authorize the GitHub (or Enterprise) App via OAuth.
        PendingUserOauth = 2,
        /// User needs to follow the link to install the GitHub (or Enterprise) App.
        PendingInstallApp = 3,
        /// Installation process has been completed.
        Complete = 10,
    }
    impl Stage {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Stage::Unspecified => "STAGE_UNSPECIFIED",
                Stage::PendingCreateApp => "PENDING_CREATE_APP",
                Stage::PendingUserOauth => "PENDING_USER_OAUTH",
                Stage::PendingInstallApp => "PENDING_INSTALL_APP",
                Stage::Complete => "COMPLETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STAGE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING_CREATE_APP" => Some(Self::PendingCreateApp),
                "PENDING_USER_OAUTH" => Some(Self::PendingUserOauth),
                "PENDING_INSTALL_APP" => Some(Self::PendingInstallApp),
                "COMPLETE" => Some(Self::Complete),
                _ => None,
            }
        }
    }
}
/// Request message for FetchLinkableRepositories.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchLinkableRepositoriesRequest {
    /// Required. The name of the Connection.
    /// Format: `projects/*/locations/*/connections/*`.
    #[prost(string, tag = "1")]
    pub connection: ::prost::alloc::string::String,
    /// Number of results to return in the list. Default to 20.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page start.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for FetchLinkableRepositories.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchLinkableRepositoriesResponse {
    /// repositories ready to be created.
    #[prost(message, repeated, tag = "1")]
    pub repositories: ::prost::alloc::vec::Vec<Repository>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Configuration for connections to github.com.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitHubConfig {
    /// OAuth credential of the account that authorized the Cloud Build GitHub App.
    /// It is recommended to use a robot account instead of a human user account.
    /// The OAuth token must be tied to the Cloud Build GitHub App.
    #[prost(message, optional, tag = "1")]
    pub authorizer_credential: ::core::option::Option<OAuthCredential>,
    /// GitHub App installation id.
    #[prost(int64, tag = "2")]
    pub app_installation_id: i64,
}
/// Configuration for connections to an instance of GitHub Enterprise.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitHubEnterpriseConfig {
    /// Required. The URI of the GitHub Enterprise host this connection is for.
    #[prost(string, tag = "1")]
    pub host_uri: ::prost::alloc::string::String,
    /// Required. API Key used for authentication of webhook events.
    #[prost(string, tag = "12")]
    pub api_key: ::prost::alloc::string::String,
    /// Id of the GitHub App created from the manifest.
    #[prost(int64, tag = "2")]
    pub app_id: i64,
    /// The URL-friendly name of the GitHub App.
    #[prost(string, tag = "13")]
    pub app_slug: ::prost::alloc::string::String,
    /// SecretManager resource containing the private key of the GitHub App,
    /// formatted as `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "4")]
    pub private_key_secret_version: ::prost::alloc::string::String,
    /// SecretManager resource containing the webhook secret of the GitHub App,
    /// formatted as `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "5")]
    pub webhook_secret_secret_version: ::prost::alloc::string::String,
    /// ID of the installation of the GitHub App.
    #[prost(int64, tag = "9")]
    pub app_installation_id: i64,
    /// Configuration for using Service Directory to privately connect to a GitHub
    /// Enterprise server. This should only be set if the GitHub Enterprise server
    /// is hosted on-premises and not reachable by public internet. If this field
    /// is left empty, calls to the GitHub Enterprise server will be made over the
    /// public internet.
    #[prost(message, optional, tag = "10")]
    pub service_directory_config: ::core::option::Option<ServiceDirectoryConfig>,
    /// SSL certificate to use for requests to GitHub Enterprise.
    #[prost(string, tag = "11")]
    pub ssl_ca: ::prost::alloc::string::String,
    /// Output only. GitHub Enterprise version installed at the host_uri.
    #[prost(string, tag = "14")]
    pub server_version: ::prost::alloc::string::String,
}
/// Configuration for connections to gitlab.com or an instance of GitLab
/// Enterprise.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitLabConfig {
    /// The URI of the GitLab Enterprise host this connection is for.
    /// If not specified, the default value is <https://gitlab.com.>
    #[prost(string, tag = "1")]
    pub host_uri: ::prost::alloc::string::String,
    /// Required. Immutable. SecretManager resource containing the webhook secret
    /// of a GitLab Enterprise project, formatted as
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "2")]
    pub webhook_secret_secret_version: ::prost::alloc::string::String,
    /// Required. A GitLab personal access token with the minimum `read_api` scope
    /// access.
    #[prost(message, optional, tag = "3")]
    pub read_authorizer_credential: ::core::option::Option<UserCredential>,
    /// Required. A GitLab personal access token with the `api` scope access.
    #[prost(message, optional, tag = "4")]
    pub authorizer_credential: ::core::option::Option<UserCredential>,
    /// Configuration for using Service Directory to privately connect to a GitLab
    /// Enterprise server. This should only be set if the GitLab Enterprise server
    /// is hosted on-premises and not reachable by public internet. If this field
    /// is left empty, calls to the GitLab Enterprise server will be made over the
    /// public internet.
    #[prost(message, optional, tag = "5")]
    pub service_directory_config: ::core::option::Option<ServiceDirectoryConfig>,
    /// SSL certificate to use for requests to GitLab Enterprise.
    #[prost(string, tag = "6")]
    pub ssl_ca: ::prost::alloc::string::String,
    /// Output only. Version of the GitLab Enterprise server running on the
    /// `host_uri`.
    #[prost(string, tag = "7")]
    pub server_version: ::prost::alloc::string::String,
}
/// ServiceDirectoryConfig represents Service Directory configuration for a
/// connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceDirectoryConfig {
    /// Required. The Service Directory service name.
    /// Format:
    /// projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}.
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
}
/// A repository associated to a parent connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Repository {
    /// Immutable. Resource name of the repository, in the format
    /// `projects/*/locations/*/connections/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Git Clone HTTPS URI.
    #[prost(string, tag = "2")]
    pub remote_uri: ::prost::alloc::string::String,
    /// Output only. Server assigned timestamp for when the connection was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server assigned timestamp for when the connection was updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Allows clients to store small amounts of arbitrary data.
    #[prost(btree_map = "string, string", tag = "6")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "7")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. External ID of the webhook created for the repository.
    #[prost(string, tag = "8")]
    pub webhook_id: ::prost::alloc::string::String,
}
/// Represents an OAuth token of the account that authorized the Connection,
/// and associated metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuthCredential {
    /// A SecretManager resource containing the OAuth token that authorizes
    /// the Cloud Build connection. Format: `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub oauth_token_secret_version: ::prost::alloc::string::String,
    /// Output only. The username associated to this token.
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
}
/// Represents a personal access token that authorized the Connection,
/// and associated metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCredential {
    /// Required. A SecretManager resource containing the user token that
    /// authorizes the Cloud Build connection. Format:
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub user_token_secret_version: ::prost::alloc::string::String,
    /// Output only. The username associated to this token.
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
}
/// Message for creating a Connection
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectionRequest {
    /// Required. Project and location where the connection will be created.
    /// Format: `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Connection to create.
    #[prost(message, optional, tag = "2")]
    pub connection: ::core::option::Option<Connection>,
    /// Required. The ID to use for the Connection, which will become the final
    /// component of the Connection's resource name. Names must be unique
    /// per-project per-location. Allows alphanumeric characters and any of
    /// -._~%!$&'()*+,;=@.
    #[prost(string, tag = "3")]
    pub connection_id: ::prost::alloc::string::String,
}
/// Message for getting the details of a Connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectionRequest {
    /// Required. The name of the Connection to retrieve.
    /// Format: `projects/*/locations/*/connections/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting list of Connections.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsRequest {
    /// Required. The parent, which owns this collection of Connections.
    /// Format: `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Number of results to return in the list.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page start.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Message for response to listing Connections.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsResponse {
    /// The list of Connections.
    #[prost(message, repeated, tag = "1")]
    pub connections: ::prost::alloc::vec::Vec<Connection>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for updating a Connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConnectionRequest {
    /// Required. The Connection to update.
    #[prost(message, optional, tag = "1")]
    pub connection: ::core::option::Option<Connection>,
    /// The list of fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set to true, and the connection is not found a new connection
    /// will be created. In this situation `update_mask` is ignored.
    /// The creation will succeed only if the input connection has all the
    /// necessary information (e.g a github_config with both  user_oauth_token and
    /// installation_id properties).
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// The current etag of the connection.
    /// If an etag is provided and does not match the current etag of the
    /// connection, update will be blocked and an ABORTED error will be returned.
    #[prost(string, tag = "4")]
    pub etag: ::prost::alloc::string::String,
}
/// Message for deleting a Connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectionRequest {
    /// Required. The name of the Connection to delete.
    /// Format: `projects/*/locations/*/connections/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The current etag of the connection.
    /// If an etag is provided and does not match the current etag of the
    /// connection, deletion will be blocked and an ABORTED error will be returned.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
    /// If set, validate the request, but do not actually post it.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Message for creating a Repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRepositoryRequest {
    /// Required. The connection to contain the repository. If the request is part
    /// of a BatchCreateRepositoriesRequest, this field should be empty or match
    /// the parent specified there.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The repository to create.
    #[prost(message, optional, tag = "2")]
    pub repository: ::core::option::Option<Repository>,
    /// Required. The ID to use for the repository, which will become the final
    /// component of the repository's resource name. This ID should be unique in
    /// the connection. Allows alphanumeric characters and any of
    /// -._~%!$&'()*+,;=@.
    #[prost(string, tag = "3")]
    pub repository_id: ::prost::alloc::string::String,
}
/// Message for creating repositoritories in batch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateRepositoriesRequest {
    /// Required. The connection to contain all the repositories being created.
    /// Format: projects/*/locations/*/connections/*
    /// The parent field in the CreateRepositoryRequest messages
    /// must either be empty or match this field.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request messages specifying the repositories to create.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<CreateRepositoryRequest>,
}
/// Message for response of creating repositories in batch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateRepositoriesResponse {
    /// Repository resources created.
    #[prost(message, repeated, tag = "1")]
    pub repositories: ::prost::alloc::vec::Vec<Repository>,
}
/// Message for getting the details of a Repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRepositoryRequest {
    /// Required. The name of the Repository to retrieve.
    /// Format: `projects/*/locations/*/connections/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting list of Repositories.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepositoriesRequest {
    /// Required. The parent, which owns this collection of Repositories.
    /// Format: `projects/*/locations/*/connections/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Number of results to return in the list.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page start.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters resources listed in the response.
    /// Expressions must follow API improvement proposal
    /// \[AIP-160\](<https://google.aip.dev/160>). e.g.
    /// `remote_uri:"<https://github.com*"`.>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Message for response to listing Repositories.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepositoriesResponse {
    /// The list of Repositories.
    #[prost(message, repeated, tag = "1")]
    pub repositories: ::prost::alloc::vec::Vec<Repository>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for deleting a Repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRepositoryRequest {
    /// Required. The name of the Repository to delete.
    /// Format: `projects/*/locations/*/connections/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The current etag of the repository.
    /// If an etag is provided and does not match the current etag of the
    /// repository, deletion will be blocked and an ABORTED error will be returned.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
    /// If set, validate the request, but do not actually post it.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Message for fetching SCM read/write token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchReadWriteTokenRequest {
    /// Required. The resource name of the repository in the format
    /// `projects/*/locations/*/connections/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub repository: ::prost::alloc::string::String,
}
/// Message for fetching SCM read token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchReadTokenRequest {
    /// Required. The resource name of the repository in the format
    /// `projects/*/locations/*/connections/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub repository: ::prost::alloc::string::String,
}
/// Message for responding to get read token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchReadTokenResponse {
    /// The token content.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Expiration timestamp. Can be empty if unknown or non-expiring.
    #[prost(message, optional, tag = "2")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Message for responding to get read/write token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchReadWriteTokenResponse {
    /// The token content.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Expiration timestamp. Can be empty if unknown or non-expiring.
    #[prost(message, optional, tag = "2")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// RPC request object accepted by the ProcessWebhook RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessWebhookRequest {
    /// Required. Project and location where the webhook will be received.
    /// Format: `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// HTTP request body.
    #[prost(message, optional, tag = "2")]
    pub body: ::core::option::Option<super::super::super::api::HttpBody>,
    /// Arbitrary additional key to find the maching repository for a webhook event
    /// if needed.
    #[prost(string, tag = "3")]
    pub webhook_key: ::prost::alloc::string::String,
}
/// Request for fetching git refs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchGitRefsRequest {
    /// Required. The resource name of the repository in the format
    /// `projects/*/locations/*/connections/*/repositories/*`.
    #[prost(string, tag = "1")]
    pub repository: ::prost::alloc::string::String,
    /// Type of refs to fetch
    #[prost(enumeration = "fetch_git_refs_request::RefType", tag = "2")]
    pub ref_type: i32,
}
/// Nested message and enum types in `FetchGitRefsRequest`.
pub mod fetch_git_refs_request {
    /// Type of refs
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
    pub enum RefType {
        /// No type specified.
        Unspecified = 0,
        /// To fetch tags.
        Tag = 1,
        /// To fetch branches.
        Branch = 2,
    }
    impl RefType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RefType::Unspecified => "REF_TYPE_UNSPECIFIED",
                RefType::Tag => "TAG",
                RefType::Branch => "BRANCH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REF_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "TAG" => Some(Self::Tag),
                "BRANCH" => Some(Self::Branch),
                _ => None,
            }
        }
    }
}
/// Response for fetching git refs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchGitRefsResponse {
    /// Name of the refs fetched.
    #[prost(string, repeated, tag = "1")]
    pub ref_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod repository_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages connections to source code repositories.
    #[derive(Debug, Clone)]
    pub struct RepositoryManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RepositoryManagerClient<T>
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
        ) -> RepositoryManagerClient<InterceptedService<T, F>>
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
            RepositoryManagerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a Connection.
        pub async fn create_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectionRequest>,
        ) -> std::result::Result<
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/CreateConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "CreateConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single connection.
        pub async fn get_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectionRequest>,
        ) -> std::result::Result<tonic::Response<super::Connection>, tonic::Status> {
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/GetConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "GetConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Connections in a given project and location.
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/ListConnections",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "ListConnections",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a single connection.
        pub async fn update_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConnectionRequest>,
        ) -> std::result::Result<
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/UpdateConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "UpdateConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single connection.
        pub async fn delete_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectionRequest>,
        ) -> std::result::Result<
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/DeleteConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "DeleteConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Repository.
        pub async fn create_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRepositoryRequest>,
        ) -> std::result::Result<
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/CreateRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "CreateRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates multiple repositories inside a connection.
        pub async fn batch_create_repositories(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateRepositoriesRequest>,
        ) -> std::result::Result<
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/BatchCreateRepositories",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "BatchCreateRepositories",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single repository.
        pub async fn get_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRepositoryRequest>,
        ) -> std::result::Result<tonic::Response<super::Repository>, tonic::Status> {
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/GetRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "GetRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Repositories in a given connection.
        pub async fn list_repositories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRepositoriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRepositoriesResponse>,
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/ListRepositories",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "ListRepositories",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single repository.
        pub async fn delete_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRepositoryRequest>,
        ) -> std::result::Result<
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/DeleteRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "DeleteRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches read/write token of a given repository.
        pub async fn fetch_read_write_token(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchReadWriteTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchReadWriteTokenResponse>,
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/FetchReadWriteToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "FetchReadWriteToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches read token of a given repository.
        pub async fn fetch_read_token(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchReadTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchReadTokenResponse>,
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/FetchReadToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "FetchReadToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// FetchLinkableRepositories get repositories from SCM that are
        /// accessible and could be added to the connection.
        pub async fn fetch_linkable_repositories(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchLinkableRepositoriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchLinkableRepositoriesResponse>,
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/FetchLinkableRepositories",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "FetchLinkableRepositories",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch the list of branches or tags for a given repository.
        pub async fn fetch_git_refs(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchGitRefsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchGitRefsResponse>,
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
                "/google.devtools.cloudbuild.v2.RepositoryManager/FetchGitRefs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudbuild.v2.RepositoryManager",
                        "FetchGitRefs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
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
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Represents the custom metadata of the RunWorkflow long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunWorkflowCustomOperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "3")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "4")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "5")]
    pub api_version: ::prost::alloc::string::String,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "6")]
    pub target: ::prost::alloc::string::String,
    /// Output only. ID of the pipeline run created by RunWorkflow.
    #[prost(string, tag = "7")]
    pub pipeline_run_id: ::prost::alloc::string::String,
}
