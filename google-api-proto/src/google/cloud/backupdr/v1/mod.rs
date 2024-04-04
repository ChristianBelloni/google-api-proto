/// Network configuration for ManagementServer instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    /// Optional. The resource name of the Google Compute Engine VPC network to
    /// which the ManagementServer instance is connected.
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// Optional. The network connect mode of the ManagementServer instance. For
    /// this version, only PRIVATE_SERVICE_ACCESS is supported.
    #[prost(enumeration = "network_config::PeeringMode", tag = "2")]
    pub peering_mode: i32,
}
/// Nested message and enum types in `NetworkConfig`.
pub mod network_config {
    /// VPC peering modes supported by Cloud BackupDR.
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
    pub enum PeeringMode {
        /// Peering mode not set.
        Unspecified = 0,
        /// Connect using Private Service Access to the Management Server. Private
        /// services access provides an IP address range for multiple Google Cloud
        /// services, including Cloud BackupDR.
        PrivateServiceAccess = 1,
    }
    impl PeeringMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PeeringMode::Unspecified => "PEERING_MODE_UNSPECIFIED",
                PeeringMode::PrivateServiceAccess => "PRIVATE_SERVICE_ACCESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PEERING_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "PRIVATE_SERVICE_ACCESS" => Some(Self::PrivateServiceAccess),
                _ => None,
            }
        }
    }
}
/// ManagementURI for the Management Server resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagementUri {
    /// Output only. The ManagementServer AGM/RD WebUI URL.
    #[prost(string, tag = "1")]
    pub web_ui: ::prost::alloc::string::String,
    /// Output only. The ManagementServer AGM/RD API URL.
    #[prost(string, tag = "2")]
    pub api: ::prost::alloc::string::String,
}
/// ManagementURI depending on the Workforce Identity i.e. either 1p or 3p.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkforceIdentityBasedManagementUri {
    /// Output only. First party Management URI for Google Identities.
    #[prost(string, tag = "1")]
    pub first_party_management_uri: ::prost::alloc::string::String,
    /// Output only. Third party Management URI for External Identity Providers.
    #[prost(string, tag = "2")]
    pub third_party_management_uri: ::prost::alloc::string::String,
}
/// OAuth Client ID depending on the Workforce Identity i.e. either 1p or 3p,
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkforceIdentityBasedOAuth2ClientId {
    /// Output only. First party OAuth Client ID for Google Identities.
    #[prost(string, tag = "1")]
    pub first_party_oauth2_client_id: ::prost::alloc::string::String,
    /// Output only. Third party OAuth Client ID for External Identity Providers.
    #[prost(string, tag = "2")]
    pub third_party_oauth2_client_id: ::prost::alloc::string::String,
}
/// ManagementServer describes a single BackupDR ManagementServer instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagementServer {
    /// Output only. Identifier. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The description of the ManagementServer instance (2048 characters
    /// or less).
    #[prost(string, tag = "9")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Resource labels to represent user provided metadata.
    /// Labels currently defined:
    /// 1. migrate_from_go=<false|true>
    ///     If set to true, the MS is created in migration ready mode.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The time when the instance was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the instance was updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The type of the ManagementServer resource.
    #[prost(enumeration = "management_server::InstanceType", tag = "14")]
    pub r#type: i32,
    /// Output only. The hostname or ip address of the exposed AGM endpoints, used
    /// by clients to connect to AGM/RD graphical user interface and APIs.
    #[prost(message, optional, tag = "11")]
    pub management_uri: ::core::option::Option<ManagementUri>,
    /// Output only. The hostnames of the exposed AGM endpoints for both types of
    /// user i.e. 1p and 3p, used to connect AGM/RM UI.
    #[prost(message, optional, tag = "16")]
    pub workforce_identity_based_management_uri: ::core::option::Option<
        WorkforceIdentityBasedManagementUri,
    >,
    /// Output only. The ManagementServer state.
    #[prost(enumeration = "management_server::InstanceState", tag = "7")]
    pub state: i32,
    /// Required. VPC networks to which the ManagementServer instance is connected.
    /// For this version, only a single network is supported.
    #[prost(message, repeated, tag = "8")]
    pub networks: ::prost::alloc::vec::Vec<NetworkConfig>,
    /// Optional. Server specified ETag for the ManagementServer resource to
    /// prevent simultaneous updates from overwiting each other.
    #[prost(string, tag = "13")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. The OAuth 2.0 client id is required to make API calls to the
    /// BackupDR instance API of this ManagementServer. This is the value that
    /// should be provided in the ‘aud’ field of the OIDC ID Token (see openid
    /// specification
    /// <https://openid.net/specs/openid-connect-core-1_0.html#IDToken>).
    #[prost(string, tag = "15")]
    pub oauth2_client_id: ::prost::alloc::string::String,
    /// Output only. The OAuth client IDs for both types of user i.e. 1p and 3p.
    #[prost(message, optional, tag = "17")]
    pub workforce_identity_based_oauth2_client_id: ::core::option::Option<
        WorkforceIdentityBasedOAuth2ClientId,
    >,
    /// Output only. The hostname or ip address of the exposed AGM endpoints, used
    /// by BAs to connect to BA proxy.
    #[prost(string, repeated, tag = "18")]
    pub ba_proxy_uri: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ManagementServer`.
pub mod management_server {
    /// Type of backup service resource.
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
    pub enum InstanceType {
        /// Instance type is not mentioned.
        Unspecified = 0,
        /// Instance for backup and restore management (i.e., AGM).
        BackupRestore = 1,
    }
    impl InstanceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InstanceType::Unspecified => "INSTANCE_TYPE_UNSPECIFIED",
                InstanceType::BackupRestore => "BACKUP_RESTORE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INSTANCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BACKUP_RESTORE" => Some(Self::BackupRestore),
                _ => None,
            }
        }
    }
    /// State of Management server instance.
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
    pub enum InstanceState {
        /// State not set.
        Unspecified = 0,
        /// The instance is being created.
        Creating = 1,
        /// The instance has been created and is fully usable.
        Ready = 2,
        /// The instance configuration is being updated. Certain kinds of updates
        /// may cause the instance to become unusable while the update is in
        /// progress.
        Updating = 3,
        /// The instance is being deleted.
        Deleting = 4,
        /// The instance is being repaired and may be unstable.
        Repairing = 5,
        /// Maintenance is being performed on this instance.
        Maintenance = 6,
        /// The instance is experiencing an issue and might be unusable. You can get
        /// further details from the statusMessage field of Instance resource.
        Error = 7,
    }
    impl InstanceState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InstanceState::Unspecified => "INSTANCE_STATE_UNSPECIFIED",
                InstanceState::Creating => "CREATING",
                InstanceState::Ready => "READY",
                InstanceState::Updating => "UPDATING",
                InstanceState::Deleting => "DELETING",
                InstanceState::Repairing => "REPAIRING",
                InstanceState::Maintenance => "MAINTENANCE",
                InstanceState::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INSTANCE_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "REPAIRING" => Some(Self::Repairing),
                "MAINTENANCE" => Some(Self::Maintenance),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// Request message for listing management servers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListManagementServersRequest {
    /// Required. The project and location for which to retrieve management servers
    /// information, in the format `projects/{project_id}/locations/{location}`. In
    /// Cloud BackupDR, locations map to GCP regions, for example **us-central1**.
    /// To retrieve management servers for all locations, use "-" for the
    /// `{location}` value.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results.
    #[prost(string, optional, tag = "4")]
    pub filter: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. Hint for how to order the results.
    #[prost(string, optional, tag = "5")]
    pub order_by: ::core::option::Option<::prost::alloc::string::String>,
}
/// Response message for listing management servers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListManagementServersResponse {
    /// The list of ManagementServer instances in the project for the specified
    /// location.
    ///
    /// If the `{location}` value in the request is "-", the response contains a
    /// list of instances from all locations. In case any location is unreachable,
    /// the response will only return management servers in reachable locations and
    /// the 'unreachable' field will be populated with a list of unreachable
    /// locations.
    #[prost(message, repeated, tag = "1")]
    pub management_servers: ::prost::alloc::vec::Vec<ManagementServer>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for getting a management server instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManagementServerRequest {
    /// Required. Name of the management server resource name, in the format
    /// `projects/{project_id}/locations/{location}/managementServers/{resource_name}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for creating a management server instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateManagementServerRequest {
    /// Required. The management server project and location in the format
    /// `projects/{project_id}/locations/{location}`. In Cloud Backup and DR
    /// locations map to GCP regions, for example **us-central1**.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The name of the management server to create. The name must be
    /// unique for the specified project and location.
    #[prost(string, tag = "2")]
    pub management_server_id: ::prost::alloc::string::String,
    /// Required. A [management server
    /// resource][google.cloud.backupdr.v1.ManagementServer]
    #[prost(message, optional, tag = "3")]
    pub management_server: ::core::option::Option<ManagementServer>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for deleting a management server instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteManagementServerRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
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
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
    /// Output only. AdditionalInfo contains additional Info related to backup plan
    /// association resource.
    #[prost(btree_map = "string, string", tag = "8")]
    pub additional_info: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Generated client implementations.
pub mod backup_dr_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The BackupDR Service
    #[derive(Debug, Clone)]
    pub struct BackupDrClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BackupDrClient<T>
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
        ) -> BackupDrClient<InterceptedService<T, F>>
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
            BackupDrClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists ManagementServers in a given project and location.
        pub async fn list_management_servers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListManagementServersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListManagementServersResponse>,
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
                "/google.cloud.backupdr.v1.BackupDR/ListManagementServers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.backupdr.v1.BackupDR",
                        "ListManagementServers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single ManagementServer.
        pub async fn get_management_server(
            &mut self,
            request: impl tonic::IntoRequest<super::GetManagementServerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ManagementServer>,
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
                "/google.cloud.backupdr.v1.BackupDR/GetManagementServer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.backupdr.v1.BackupDR",
                        "GetManagementServer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new ManagementServer in a given project and location.
        pub async fn create_management_server(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateManagementServerRequest>,
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
                "/google.cloud.backupdr.v1.BackupDR/CreateManagementServer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.backupdr.v1.BackupDR",
                        "CreateManagementServer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single ManagementServer.
        pub async fn delete_management_server(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteManagementServerRequest>,
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
                "/google.cloud.backupdr.v1.BackupDR/DeleteManagementServer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.backupdr.v1.BackupDR",
                        "DeleteManagementServer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
