/// Request for \[CreateCluster][CloudRedis.CreateCluster\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterRequest {
    /// Required. The resource name of the cluster location using the form:
    ///      `projects/{project_id}/locations/{location_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The logical name of the Redis cluster in the customer project
    /// with the following restrictions:
    ///
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-63 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the customer project / location
    #[prost(string, tag = "2")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The cluster that is to be created.
    #[prost(message, optional, tag = "3")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Idempotent request UUID.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for \[ListClusters][CloudRedis.ListClusters\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersRequest {
    /// Required. The resource name of the cluster location using the form:
    ///      `projects/{project_id}/locations/{location_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    ///
    /// If not specified, a default value of 1000 will be used by the service.
    /// Regardless of the page_size value, the response may include a partial list
    /// and a caller should only rely on response's
    /// \[`next_page_token`][google.cloud.redis.cluster.v1beta1.ListClustersResponse.next_page_token\]
    /// to determine if there are more clusters left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The `next_page_token` value returned from a previous
    /// \[ListClusters][CloudRedis.ListClusters\] request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for \[ListClusters][CloudRedis.ListClusters\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersResponse {
    /// A list of Redis clusters in the project in the specified location,
    /// or across all locations.
    ///
    /// If the `location_id` in the parent field of the request is "-", all regions
    /// available to the project are queried, and the results aggregated.
    /// If in such an aggregated query a location is unavailable, a placeholder
    /// Redis entry is included in the response with the `name` field set to a
    /// value of the form
    /// `projects/{project_id}/locations/{location_id}/clusters/`- and the
    /// `status` field set to ERROR and `status_message` field set to "location not
    /// available for ListClusters".
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<Cluster>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for \[UpdateCluster][CloudRedis.UpdateCluster\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClusterRequest {
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. The elements of the repeated paths field may only include these
    /// fields from \[Cluster][google.cloud.redis.cluster.v1beta1.Cluster\]:
    ///
    ///   *   `size_gb`
    ///   *   `replica_count`
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Update description.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Idempotent request UUID.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for \[GetCluster][CloudRedis.GetCluster\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterRequest {
    /// Required. Redis cluster resource name using the form:
    ///      `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[DeleteCluster][CloudRedis.DeleteCluster\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClusterRequest {
    /// Required. Redis cluster resource name using the form:
    ///      `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Idempotent request UUID.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for
/// \[GetClusterCertificateAuthorityRequest][CloudRedis.GetClusterCertificateAuthorityRequest\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterCertificateAuthorityRequest {
    /// Required. Redis cluster certificate authority resource name using the form:
    ///      `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}/certificateAuthority`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A cluster instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// Required. Unique name of the resource in this scope including project and
    /// location using the form:
    ///      `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp associated with the cluster creation request.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of this cluster.
    /// Can be CREATING, READY, UPDATING, DELETING and SUSPENDED
    #[prost(enumeration = "cluster::State", tag = "4")]
    pub state: i32,
    /// Output only. System assigned, unique identifier for the cluster.
    #[prost(string, tag = "5")]
    pub uid: ::prost::alloc::string::String,
    /// Optional. The number of replica nodes per shard.
    #[prost(int32, optional, tag = "8")]
    pub replica_count: ::core::option::Option<i32>,
    /// Optional. The authorization mode of the Redis cluster.
    /// If not provided, auth feature is disabled for the cluster.
    #[prost(enumeration = "AuthorizationMode", tag = "11")]
    pub authorization_mode: i32,
    /// Optional. The in-transit encryption for the Redis cluster.
    /// If not provided, encryption  is disabled for the cluster.
    #[prost(enumeration = "TransitEncryptionMode", tag = "12")]
    pub transit_encryption_mode: i32,
    /// Output only. Redis memory size in GB for the entire cluster.
    #[prost(int32, optional, tag = "13")]
    pub size_gb: ::core::option::Option<i32>,
    /// Required. Number of shards for the Redis cluster.
    #[prost(int32, optional, tag = "14")]
    pub shard_count: ::core::option::Option<i32>,
    /// Required. Each PscConfig configures the consumer network where IPs will
    /// be designated to the cluster for client access through Private Service
    /// Connect Automation. Currently, only one PscConfig is supported.
    #[prost(message, repeated, tag = "15")]
    pub psc_configs: ::prost::alloc::vec::Vec<PscConfig>,
    /// Output only. Endpoints created on each given network, for Redis clients to
    /// connect to the cluster. Currently only one discovery endpoint is supported.
    #[prost(message, repeated, tag = "16")]
    pub discovery_endpoints: ::prost::alloc::vec::Vec<DiscoveryEndpoint>,
    /// Output only. PSC connections for discovery of the cluster topology and
    /// accessing the cluster.
    #[prost(message, repeated, tag = "17")]
    pub psc_connections: ::prost::alloc::vec::Vec<PscConnection>,
    /// Output only. Additional information about the current state of the cluster.
    #[prost(message, optional, tag = "18")]
    pub state_info: ::core::option::Option<cluster::StateInfo>,
}
/// Nested message and enum types in `Cluster`.
pub mod cluster {
    /// Represents additional information about the state of the cluster.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StateInfo {
        #[prost(oneof = "state_info::Info", tags = "1")]
        pub info: ::core::option::Option<state_info::Info>,
    }
    /// Nested message and enum types in `StateInfo`.
    pub mod state_info {
        /// Represents information about an updating cluster.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct UpdateInfo {
            /// Target number of shards for redis cluster
            #[prost(int32, optional, tag = "1")]
            pub target_shard_count: ::core::option::Option<i32>,
            /// Target number of replica nodes per shard.
            #[prost(int32, optional, tag = "2")]
            pub target_replica_count: ::core::option::Option<i32>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Info {
            /// Describes ongoing update on the cluster when cluster state is UPDATING.
            #[prost(message, tag = "1")]
            UpdateInfo(UpdateInfo),
        }
    }
    /// Represents the different states of a Redis cluster.
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
    pub enum State {
        /// Not set.
        Unspecified = 0,
        /// Redis cluster is being created.
        Creating = 1,
        /// Redis cluster has been created and is fully usable.
        Active = 2,
        /// Redis cluster configuration is being updated.
        Updating = 3,
        /// Redis cluster is being deleted.
        Deleting = 4,
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
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PscConfig {
    /// Required. The network where the IP address of the discovery endpoint will
    /// be reserved, in the form of
    /// projects/{network_project}/global/networks/{network_id}.
    #[prost(string, tag = "2")]
    pub network: ::prost::alloc::string::String,
}
/// Endpoints on each network, for Redis clients to connect to the cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryEndpoint {
    /// Output only. Address of the exposed Redis endpoint used by clients to
    /// connect to the service. The address could be either IP or hostname.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Output only. The port number of the exposed Redis endpoint.
    #[prost(int32, tag = "2")]
    pub port: i32,
    /// Output only. Customer configuration for where the endpoint is created and
    /// accessed from.
    #[prost(message, optional, tag = "3")]
    pub psc_config: ::core::option::Option<PscConfig>,
}
/// Details of consumer resources in a PSC connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PscConnection {
    /// Output only. The PSC connection id of the forwarding rule connected to the
    /// service attachment.
    #[prost(string, tag = "1")]
    pub psc_connection_id: ::prost::alloc::string::String,
    /// Output only. The IP allocated on the consumer network for the PSC
    /// forwarding rule.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// Output only. The URI of the consumer side forwarding rule.
    /// Example:
    /// projects/{projectNumOrId}/regions/us-east1/forwardingRules/{resourceId}.
    #[prost(string, tag = "3")]
    pub forwarding_rule: ::prost::alloc::string::String,
    /// Output only. The consumer project_id where the forwarding rule is created
    /// from.
    #[prost(string, tag = "4")]
    pub project_id: ::prost::alloc::string::String,
    /// The consumer network where the IP address resides, in the form of
    /// projects/{project_id}/global/networks/{network_id}.
    #[prost(string, tag = "5")]
    pub network: ::prost::alloc::string::String,
}
/// Pre-defined metadata fields.
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
/// Redis cluster certificate authority
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateAuthority {
    /// Identifier. Unique name of the resource in this scope including project,
    /// location and cluster using the form:
    ///      `projects/{project}/locations/{location}/clusters/{cluster}/certificateAuthority`
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// server ca information
    #[prost(oneof = "certificate_authority::ServerCa", tags = "1")]
    pub server_ca: ::core::option::Option<certificate_authority::ServerCa>,
}
/// Nested message and enum types in `CertificateAuthority`.
pub mod certificate_authority {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ManagedCertificateAuthority {
        /// The PEM encoded CA certificate chains for redis managed
        /// server authentication
        #[prost(message, repeated, tag = "1")]
        pub ca_certs: ::prost::alloc::vec::Vec<managed_certificate_authority::CertChain>,
    }
    /// Nested message and enum types in `ManagedCertificateAuthority`.
    pub mod managed_certificate_authority {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CertChain {
            /// The certificates that form the CA chain, from leaf to root order.
            #[prost(string, repeated, tag = "1")]
            pub certificates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
    }
    /// server ca information
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ServerCa {
        #[prost(message, tag = "1")]
        ManagedServerCa(ManagedCertificateAuthority),
    }
}
/// Available authorization mode of a Redis cluster.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthorizationMode {
    /// Not set.
    AuthModeUnspecified = 0,
    /// IAM basic authorization mode
    AuthModeIamAuth = 1,
    /// Authorization disabled mode
    AuthModeDisabled = 2,
}
impl AuthorizationMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthorizationMode::AuthModeUnspecified => "AUTH_MODE_UNSPECIFIED",
            AuthorizationMode::AuthModeIamAuth => "AUTH_MODE_IAM_AUTH",
            AuthorizationMode::AuthModeDisabled => "AUTH_MODE_DISABLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTH_MODE_UNSPECIFIED" => Some(Self::AuthModeUnspecified),
            "AUTH_MODE_IAM_AUTH" => Some(Self::AuthModeIamAuth),
            "AUTH_MODE_DISABLED" => Some(Self::AuthModeDisabled),
            _ => None,
        }
    }
}
/// Available mode of in-transit encryption.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransitEncryptionMode {
    /// In-transit encryption not set.
    Unspecified = 0,
    /// In-transit encryption disabled.
    Disabled = 1,
    /// Use server managed encryption for in-transit encryption.
    ServerAuthentication = 2,
}
impl TransitEncryptionMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransitEncryptionMode::Unspecified => "TRANSIT_ENCRYPTION_MODE_UNSPECIFIED",
            TransitEncryptionMode::Disabled => "TRANSIT_ENCRYPTION_MODE_DISABLED",
            TransitEncryptionMode::ServerAuthentication => {
                "TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRANSIT_ENCRYPTION_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "TRANSIT_ENCRYPTION_MODE_DISABLED" => Some(Self::Disabled),
            "TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION" => {
                Some(Self::ServerAuthentication)
            }
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod cloud_redis_cluster_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Configures and manages Cloud Memorystore for Redis clusters
    ///
    /// Google Cloud Memorystore for Redis Cluster
    ///
    /// The `redis.googleapis.com` service implements the Google Cloud Memorystore
    /// for Redis API and defines the following resource model for managing Redis
    /// clusters:
    /// * The service works with a collection of cloud projects, named: `/projects/*`
    /// * Each project has a collection of available locations, named: `/locations/*`
    /// * Each location has a collection of Redis clusters, named: `/clusters/*`
    /// * As such, Redis clusters are resources of the form:
    ///   `/projects/{project_id}/locations/{location_id}/clusters/{instance_id}`
    ///
    /// Note that location_id must be a GCP `region`; for example:
    /// * `projects/redpepper-1290/locations/us-central1/clusters/my-redis`
    ///
    /// We use API version selector for Flex APIs
    /// * The versioning strategy is release-based versioning
    /// * Our backend CLH only deals with the superset version (called v1main)
    /// * Existing backend for Redis Gen1 and MRR is not touched.
    /// * More details in go/redis-flex-api-versioning
    #[derive(Debug, Clone)]
    pub struct CloudRedisClusterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudRedisClusterClient<T>
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
        ) -> CloudRedisClusterClient<InterceptedService<T, F>>
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
            CloudRedisClusterClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all Redis clusters owned by a project in either the specified
        /// location (region) or all locations.
        ///
        /// The location should have the following format:
        ///
        /// * `projects/{project_id}/locations/{location_id}`
        ///
        /// If `location_id` is specified as `-` (wildcard), then all regions
        /// available to the project are queried, and the results are aggregated.
        pub async fn list_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListClustersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListClustersResponse>,
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
                "/google.cloud.redis.cluster.v1beta1.CloudRedisCluster/ListClusters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.redis.cluster.v1beta1.CloudRedisCluster",
                        "ListClusters",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a specific Redis cluster.
        pub async fn get_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClusterRequest>,
        ) -> std::result::Result<tonic::Response<super::Cluster>, tonic::Status> {
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
                "/google.cloud.redis.cluster.v1beta1.CloudRedisCluster/GetCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.redis.cluster.v1beta1.CloudRedisCluster",
                        "GetCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the metadata and configuration of a specific Redis cluster.
        ///
        /// Completed longrunning.Operation will contain the new cluster object
        /// in the response field. The returned operation is automatically deleted
        /// after a few hours, so there is no need to call DeleteOperation.
        pub async fn update_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateClusterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.redis.cluster.v1beta1.CloudRedisCluster/UpdateCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.redis.cluster.v1beta1.CloudRedisCluster",
                        "UpdateCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a specific Redis cluster. Cluster stops serving and data is
        /// deleted.
        pub async fn delete_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClusterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.redis.cluster.v1beta1.CloudRedisCluster/DeleteCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.redis.cluster.v1beta1.CloudRedisCluster",
                        "DeleteCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Redis cluster based on the specified properties.
        /// The creation is executed asynchronously and callers may check the returned
        /// operation to track its progress. Once the operation is completed the Redis
        /// cluster will be fully functional. The completed longrunning.Operation will
        /// contain the new cluster object in the response field.
        ///
        /// The returned operation is automatically deleted after a few hours, so there
        /// is no need to call DeleteOperation.
        pub async fn create_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClusterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.redis.cluster.v1beta1.CloudRedisCluster/CreateCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.redis.cluster.v1beta1.CloudRedisCluster",
                        "CreateCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of certificate authority information for Redis cluster.
        pub async fn get_cluster_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetClusterCertificateAuthorityRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CertificateAuthority>,
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
                "/google.cloud.redis.cluster.v1beta1.CloudRedisCluster/GetClusterCertificateAuthority",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.redis.cluster.v1beta1.CloudRedisCluster",
                        "GetClusterCertificateAuthority",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
