/// A single application container.
/// This specifies both the container to run, the command to run in the container
/// and the arguments to supply to it.
/// Note that additional arguments can be supplied by the system to the container
/// at runtime.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    /// Name of the container specified as a DNS_LABEL (RFC 1123).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Name of the container image in Dockerhub, Google Artifact
    /// Registry, or Google Container Registry. If the host is not provided,
    /// Dockerhub is assumed.
    #[prost(string, tag = "2")]
    pub image: ::prost::alloc::string::String,
    /// Entrypoint array. Not executed within a shell.
    /// The docker image's ENTRYPOINT is used if this is not provided.
    #[prost(string, repeated, tag = "3")]
    pub command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Arguments to the entrypoint.
    /// The docker image's CMD is used if this is not provided.
    #[prost(string, repeated, tag = "4")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of environment variables to set in the container.
    #[prost(message, repeated, tag = "5")]
    pub env: ::prost::alloc::vec::Vec<EnvVar>,
    /// Compute Resource requirements by this container.
    #[prost(message, optional, tag = "6")]
    pub resources: ::core::option::Option<ResourceRequirements>,
    /// List of ports to expose from the container. Only a single port can be
    /// specified. The specified ports must be listening on all interfaces
    /// (0.0.0.0) within the container to be accessible.
    ///
    /// If omitted, a port number will be chosen and passed to the container
    /// through the PORT environment variable for the container to listen on.
    #[prost(message, repeated, tag = "7")]
    pub ports: ::prost::alloc::vec::Vec<ContainerPort>,
    /// Volume to mount into the container's filesystem.
    #[prost(message, repeated, tag = "8")]
    pub volume_mounts: ::prost::alloc::vec::Vec<VolumeMount>,
    /// Container's working directory.
    /// If not specified, the container runtime's default will be used, which
    /// might be configured in the container image.
    #[prost(string, tag = "9")]
    pub working_dir: ::prost::alloc::string::String,
    /// Periodic probe of container liveness.
    /// Container will be restarted if the probe fails.
    #[prost(message, optional, tag = "10")]
    pub liveness_probe: ::core::option::Option<Probe>,
    /// Startup probe of application within the container.
    /// All other probes are disabled if a startup probe is provided, until it
    /// succeeds. Container will not be added to service endpoints if the probe
    /// fails.
    #[prost(message, optional, tag = "11")]
    pub startup_probe: ::core::option::Option<Probe>,
    /// Names of the containers that must start before this container.
    #[prost(string, repeated, tag = "12")]
    pub depends_on: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ResourceRequirements describes the compute resource requirements.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceRequirements {
    /// Only `memory` and `cpu` keys in the map are supported.
    ///
    /// <p>Notes:
    ///   * The only supported values for CPU are '1', '2', '4', and '8'. Setting 4
    /// CPU requires at least 2Gi of memory. For more information, go to
    /// <https://cloud.google.com/run/docs/configuring/cpu.>
    ///    * For supported 'memory' values and syntax, go to
    ///   <https://cloud.google.com/run/docs/configuring/memory-limits>
    #[prost(btree_map = "string, string", tag = "1")]
    pub limits: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Determines whether CPU is only allocated during requests (true by default).
    /// However, if ResourceRequirements is set, the caller must explicitly
    /// set this field to true to preserve the default behavior.
    #[prost(bool, tag = "2")]
    pub cpu_idle: bool,
    /// Determines whether CPU should be boosted on startup of a new container
    /// instance above the requested CPU threshold, this can help reduce cold-start
    /// latency.
    #[prost(bool, tag = "3")]
    pub startup_cpu_boost: bool,
}
/// EnvVar represents an environment variable present in a Container.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvVar {
    /// Required. Name of the environment variable. Must not exceed 32768
    /// characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "env_var::Values", tags = "2, 3")]
    pub values: ::core::option::Option<env_var::Values>,
}
/// Nested message and enum types in `EnvVar`.
pub mod env_var {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Values {
        /// Variable references $(VAR_NAME) are expanded
        /// using the previous defined environment variables in the container and
        /// any route environment variables. If a variable cannot be resolved,
        /// the reference in the input string will be unchanged. The $(VAR_NAME)
        /// syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped
        /// references will never be expanded, regardless of whether the variable
        /// exists or not.
        /// Defaults to "", and the maximum length is 32768 bytes.
        #[prost(string, tag = "2")]
        Value(::prost::alloc::string::String),
        /// Source for the environment variable's value.
        #[prost(message, tag = "3")]
        ValueSource(super::EnvVarSource),
    }
}
/// EnvVarSource represents a source for the value of an EnvVar.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvVarSource {
    /// Selects a secret and a specific version from Cloud Secret Manager.
    #[prost(message, optional, tag = "1")]
    pub secret_key_ref: ::core::option::Option<SecretKeySelector>,
}
/// SecretEnvVarSource represents a source for the value of an EnvVar.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretKeySelector {
    /// Required. The name of the secret in Cloud Secret Manager.
    /// Format: {secret_name} if the secret is in the same project.
    /// projects/{project}/secrets/{secret_name} if the secret is
    /// in a different project.
    #[prost(string, tag = "1")]
    pub secret: ::prost::alloc::string::String,
    /// The Cloud Secret Manager secret version.
    /// Can be 'latest' for the latest version, an integer for a specific version,
    /// or a version alias.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// ContainerPort represents a network port in a single container.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerPort {
    /// If specified, used to specify which protocol to use.
    /// Allowed values are "http1" and "h2c".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Port number the container listens on.
    /// This must be a valid TCP port number, 0 < container_port < 65536.
    #[prost(int32, tag = "3")]
    pub container_port: i32,
}
/// VolumeMount describes a mounting of a Volume within a container.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeMount {
    /// Required. This must match the Name of a Volume.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Path within the container at which the volume should be mounted.
    /// Must not contain ':'. For Cloud SQL volumes, it can be left empty, or must
    /// otherwise be `/cloudsql`. All instances defined in the Volume will be
    /// available as `/cloudsql/\[instance\]`. For more information on Cloud SQL
    /// volumes, visit <https://cloud.google.com/sql/docs/mysql/connect-run>
    #[prost(string, tag = "3")]
    pub mount_path: ::prost::alloc::string::String,
}
/// Volume represents a named volume in a container.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    /// Required. Volume's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "volume::VolumeType", tags = "2, 3, 4, 5, 6")]
    pub volume_type: ::core::option::Option<volume::VolumeType>,
}
/// Nested message and enum types in `Volume`.
pub mod volume {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VolumeType {
        /// Secret represents a secret that should populate this volume.
        #[prost(message, tag = "2")]
        Secret(super::SecretVolumeSource),
        /// For Cloud SQL volumes, contains the specific instances that should be
        /// mounted. Visit <https://cloud.google.com/sql/docs/mysql/connect-run> for
        /// more information on how to connect Cloud SQL and Cloud Run.
        #[prost(message, tag = "3")]
        CloudSqlInstance(super::CloudSqlInstance),
        /// Ephemeral storage used as a shared volume.
        #[prost(message, tag = "4")]
        EmptyDir(super::EmptyDirVolumeSource),
        /// For NFS Voumes, contains the path to the nfs Volume
        #[prost(message, tag = "5")]
        Nfs(super::NfsVolumeSource),
        /// Persistent storage backed by a Google Cloud Storage bucket.
        #[prost(message, tag = "6")]
        Gcs(super::GcsVolumeSource),
    }
}
/// The secret's value will be presented as the content of a file whose
/// name is defined in the item path. If no items are defined, the name of
/// the file is the secret.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretVolumeSource {
    /// Required. The name of the secret in Cloud Secret Manager.
    /// Format: {secret} if the secret is in the same project.
    /// projects/{project}/secrets/{secret} if the secret is
    /// in a different project.
    #[prost(string, tag = "1")]
    pub secret: ::prost::alloc::string::String,
    /// If unspecified, the volume will expose a file whose name is the
    /// secret, relative to VolumeMount.mount_path.
    /// If specified, the key will be used as the version to fetch from Cloud
    /// Secret Manager and the path will be the name of the file exposed in the
    /// volume. When items are defined, they must specify a path and a version.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<VersionToPath>,
    /// Integer representation of mode bits to use on created files by default.
    /// Must be a value between 0000 and 0777 (octal), defaulting to 0444.
    /// Directories within the path are not affected by  this setting.
    ///
    /// Notes
    ///
    /// * Internally, a umask of 0222 will be applied to any non-zero value.
    /// * This is an integer representation of the mode bits. So, the octal
    /// integer value should look exactly as the chmod numeric notation with a
    /// leading zero. Some examples: for chmod 777 (a=rwx), set to 0777 (octal) or
    /// 511 (base-10). For chmod 640 (u=rw,g=r), set to 0640 (octal) or
    /// 416 (base-10). For chmod 755 (u=rwx,g=rx,o=rx), set to 0755 (octal) or 493
    /// (base-10).
    /// * This might be in conflict with other options that affect the
    /// file mode, like fsGroup, and the result can be other mode bits set.
    ///
    /// This might be in conflict with other options that affect the
    /// file mode, like fsGroup, and as a result, other mode bits could be set.
    #[prost(int32, tag = "3")]
    pub default_mode: i32,
}
/// VersionToPath maps a specific version of a secret to a relative file to mount
/// to, relative to VolumeMount's mount_path.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionToPath {
    /// Required. The relative path of the secret in the container.
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// The Cloud Secret Manager secret version.
    /// Can be 'latest' for the latest value, or an integer or a secret alias for a
    /// specific version.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// Integer octal mode bits to use on this file, must be a value between
    /// 01 and 0777 (octal). If 0 or not set, the Volume's default mode will be
    /// used.
    ///
    /// Notes
    ///
    /// * Internally, a umask of 0222 will be applied to any non-zero value.
    /// * This is an integer representation of the mode bits. So, the octal
    /// integer value should look exactly as the chmod numeric notation with a
    /// leading zero. Some examples: for chmod 777 (a=rwx), set to 0777 (octal) or
    /// 511 (base-10). For chmod 640 (u=rw,g=r), set to 0640 (octal) or
    /// 416 (base-10). For chmod 755 (u=rwx,g=rx,o=rx), set to 0755 (octal) or 493
    /// (base-10).
    /// * This might be in conflict with other options that affect the
    /// file mode, like fsGroup, and the result can be other mode bits set.
    #[prost(int32, tag = "3")]
    pub mode: i32,
}
/// Represents a set of Cloud SQL instances. Each one will be available under
/// /cloudsql/\[instance\]. Visit
/// <https://cloud.google.com/sql/docs/mysql/connect-run> for more information on
/// how to connect Cloud SQL and Cloud Run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlInstance {
    /// The Cloud SQL instance connection names, as can be found in
    /// <https://console.cloud.google.com/sql/instances.> Visit
    /// <https://cloud.google.com/sql/docs/mysql/connect-run> for more information on
    /// how to connect Cloud SQL and Cloud Run. Format:
    /// {project}:{location}:{instance}
    #[prost(string, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// In memory (tmpfs) ephemeral storage.
/// It is ephemeral in the sense that when the sandbox is taken down, the data is
/// destroyed with it (it does not persist across sandbox runs).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyDirVolumeSource {
    /// The medium on which the data is stored. Acceptable values today is only
    /// MEMORY or none. When none, the default will currently be backed by memory
    /// but could change over time. +optional
    #[prost(enumeration = "empty_dir_volume_source::Medium", tag = "1")]
    pub medium: i32,
    /// Limit on the storage usable by this EmptyDir volume.
    /// The size limit is also applicable for memory medium.
    /// The maximum usage on memory medium EmptyDir would be the minimum value
    /// between the SizeLimit specified here and the sum of memory limits of all
    /// containers. The default is nil which means that the limit is undefined.
    /// More info:
    /// <https://cloud.google.com/run/docs/configuring/in-memory-volumes#configure-volume.>
    /// Info in Kubernetes:
    /// <https://kubernetes.io/docs/concepts/storage/volumes/#emptydir>
    #[prost(string, tag = "2")]
    pub size_limit: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EmptyDirVolumeSource`.
pub mod empty_dir_volume_source {
    /// The different types of medium supported for EmptyDir.
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
    pub enum Medium {
        /// When not specified, falls back to the default implementation which
        /// is currently in memory (this may change over time).
        Unspecified = 0,
        /// Explicitly set the EmptyDir to be in memory. Uses tmpfs.
        Memory = 1,
    }
    impl Medium {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Medium::Unspecified => "MEDIUM_UNSPECIFIED",
                Medium::Memory => "MEMORY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MEDIUM_UNSPECIFIED" => Some(Self::Unspecified),
                "MEMORY" => Some(Self::Memory),
                _ => None,
            }
        }
    }
}
/// Represents an NFS mount.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfsVolumeSource {
    /// Hostname or IP address of the NFS server
    #[prost(string, tag = "1")]
    pub server: ::prost::alloc::string::String,
    /// Path that is exported by the NFS server.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// If true, mount the NFS volume as read only
    #[prost(bool, tag = "3")]
    pub read_only: bool,
}
/// Represents a GCS Bucket mounted as a volume.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsVolumeSource {
    /// GCS Bucket name
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    /// If true, mount the GCS bucket as read-only
    #[prost(bool, tag = "2")]
    pub read_only: bool,
}
/// Probe describes a health check to be performed against a container to
/// determine whether it is alive or ready to receive traffic.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Probe {
    /// Number of seconds after the container has started before the probe is
    /// initiated.
    /// Defaults to 0 seconds. Minimum value is 0. Maximum value for liveness probe
    /// is 3600. Maximum value for startup probe is 240.
    #[prost(int32, tag = "1")]
    pub initial_delay_seconds: i32,
    /// Number of seconds after which the probe times out.
    /// Defaults to 1 second. Minimum value is 1. Maximum value is 3600.
    /// Must be smaller than period_seconds.
    #[prost(int32, tag = "2")]
    pub timeout_seconds: i32,
    /// How often (in seconds) to perform the probe.
    /// Default to 10 seconds. Minimum value is 1. Maximum value for liveness probe
    /// is 3600. Maximum value for startup probe is 240.
    /// Must be greater or equal than timeout_seconds.
    #[prost(int32, tag = "3")]
    pub period_seconds: i32,
    /// Minimum consecutive failures for the probe to be considered failed after
    /// having succeeded. Defaults to 3. Minimum value is 1.
    #[prost(int32, tag = "4")]
    pub failure_threshold: i32,
    #[prost(oneof = "probe::ProbeType", tags = "5, 6, 7")]
    pub probe_type: ::core::option::Option<probe::ProbeType>,
}
/// Nested message and enum types in `Probe`.
pub mod probe {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProbeType {
        /// HTTPGet specifies the http request to perform.
        /// Exactly one of httpGet, tcpSocket, or grpc must be specified.
        #[prost(message, tag = "5")]
        HttpGet(super::HttpGetAction),
        /// TCPSocket specifies an action involving a TCP port.
        /// Exactly one of httpGet, tcpSocket, or grpc must be specified.
        #[prost(message, tag = "6")]
        TcpSocket(super::TcpSocketAction),
        /// GRPC specifies an action involving a gRPC port.
        /// Exactly one of httpGet, tcpSocket, or grpc must be specified.
        #[prost(message, tag = "7")]
        Grpc(super::GrpcAction),
    }
}
/// HTTPGetAction describes an action based on HTTP Get requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpGetAction {
    /// Path to access on the HTTP server. Defaults to '/'.
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[prost(message, repeated, tag = "4")]
    pub http_headers: ::prost::alloc::vec::Vec<HttpHeader>,
    /// Port number to access on the container. Must be in the range 1 to 65535.
    /// If not specified, defaults to the exposed port of the container, which is
    /// the value of container.ports\[0\].containerPort.
    #[prost(int32, tag = "5")]
    pub port: i32,
}
/// HTTPHeader describes a custom header to be used in HTTP probes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpHeader {
    /// Required. The header field name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The header field value
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// TCPSocketAction describes an action based on opening a socket
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpSocketAction {
    /// Port number to access on the container. Must be in the range 1 to 65535.
    /// If not specified, defaults to the exposed port of the container, which is
    /// the value of container.ports\[0\].containerPort.
    #[prost(int32, tag = "1")]
    pub port: i32,
}
/// GRPCAction describes an action involving a GRPC port.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcAction {
    /// Port number of the gRPC service. Number must be in the range 1 to 65535.
    /// If not specified, defaults to the exposed port of the container, which is
    /// the value of container.ports\[0\].containerPort.
    #[prost(int32, tag = "1")]
    pub port: i32,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest
    /// (see <https://github.com/grpc/grpc/blob/master/doc/health-checking.md> ). If
    /// this is not specified, the default behavior is defined by gRPC.
    #[prost(string, tag = "2")]
    pub service: ::prost::alloc::string::String,
}
/// VPC Access settings. For more information on sending traffic to a VPC
/// network, visit <https://cloud.google.com/run/docs/configuring/connecting-vpc.>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpcAccess {
    /// VPC Access connector name.
    /// Format: projects/{project}/locations/{location}/connectors/{connector},
    /// where {project} can be project id or number.
    /// For more information on sending traffic to a VPC network via a connector,
    /// visit <https://cloud.google.com/run/docs/configuring/vpc-connectors.>
    #[prost(string, tag = "1")]
    pub connector: ::prost::alloc::string::String,
    /// Traffic VPC egress settings. If not provided, it defaults to
    /// PRIVATE_RANGES_ONLY.
    #[prost(enumeration = "vpc_access::VpcEgress", tag = "2")]
    pub egress: i32,
    /// Direct VPC egress settings. Currently only single network interface is
    /// supported.
    #[prost(message, repeated, tag = "3")]
    pub network_interfaces: ::prost::alloc::vec::Vec<vpc_access::NetworkInterface>,
}
/// Nested message and enum types in `VpcAccess`.
pub mod vpc_access {
    /// Direct VPC egress settings.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetworkInterface {
        /// The VPC network that the Cloud Run resource will be able to send traffic
        /// to. At least one of network or subnetwork must be specified. If both
        /// network and subnetwork are specified, the given VPC subnetwork must
        /// belong to the given VPC network. If network is not specified, it will be
        /// looked up from the subnetwork.
        #[prost(string, tag = "1")]
        pub network: ::prost::alloc::string::String,
        /// The VPC subnetwork that the Cloud Run resource will get IPs from. At
        /// least one of network or subnetwork must be specified. If both
        /// network and subnetwork are specified, the given VPC subnetwork must
        /// belong to the given VPC network. If subnetwork is not specified, the
        /// subnetwork with the same name with the network will be used.
        #[prost(string, tag = "2")]
        pub subnetwork: ::prost::alloc::string::String,
        /// Network tags applied to this Cloud Run resource.
        #[prost(string, repeated, tag = "3")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Egress options for VPC access.
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
    pub enum VpcEgress {
        /// Unspecified
        Unspecified = 0,
        /// All outbound traffic is routed through the VPC connector.
        AllTraffic = 1,
        /// Only private IP ranges are routed through the VPC connector.
        PrivateRangesOnly = 2,
    }
    impl VpcEgress {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VpcEgress::Unspecified => "VPC_EGRESS_UNSPECIFIED",
                VpcEgress::AllTraffic => "ALL_TRAFFIC",
                VpcEgress::PrivateRangesOnly => "PRIVATE_RANGES_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VPC_EGRESS_UNSPECIFIED" => Some(Self::Unspecified),
                "ALL_TRAFFIC" => Some(Self::AllTraffic),
                "PRIVATE_RANGES_ONLY" => Some(Self::PrivateRangesOnly),
                _ => None,
            }
        }
    }
}
/// Settings for Binary Authorization feature.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryAuthorization {
    /// If present, indicates to use Breakglass using this justification.
    /// If use_default is False, then it must be empty.
    /// For more information on breakglass, see
    /// <https://cloud.google.com/binary-authorization/docs/using-breakglass>
    #[prost(string, tag = "2")]
    pub breakglass_justification: ::prost::alloc::string::String,
    #[prost(oneof = "binary_authorization::BinauthzMethod", tags = "1")]
    pub binauthz_method: ::core::option::Option<binary_authorization::BinauthzMethod>,
}
/// Nested message and enum types in `BinaryAuthorization`.
pub mod binary_authorization {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BinauthzMethod {
        /// If True, indicates to use the default project's binary authorization
        /// policy. If False, binary authorization will be disabled.
        #[prost(bool, tag = "1")]
        UseDefault(bool),
    }
}
/// Settings for revision-level scaling settings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevisionScaling {
    /// Minimum number of serving instances that this resource should have.
    #[prost(int32, tag = "1")]
    pub min_instance_count: i32,
    /// Maximum number of serving instances that this resource should have.
    #[prost(int32, tag = "2")]
    pub max_instance_count: i32,
}
/// Scaling settings applied at the service level rather than
/// at the revision level.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceScaling {
    /// total min instances for the service. This number of instances is
    /// divided among all revisions with specified traffic based on the percent
    /// of traffic they are receiving. (BETA)
    #[prost(int32, tag = "1")]
    pub min_instance_count: i32,
}
/// Allowed ingress traffic for the Container.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IngressTraffic {
    /// Unspecified
    Unspecified = 0,
    /// All inbound traffic is allowed.
    All = 1,
    /// Only internal traffic is allowed.
    InternalOnly = 2,
    /// Both internal and Google Cloud Load Balancer traffic is allowed.
    InternalLoadBalancer = 3,
}
impl IngressTraffic {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IngressTraffic::Unspecified => "INGRESS_TRAFFIC_UNSPECIFIED",
            IngressTraffic::All => "INGRESS_TRAFFIC_ALL",
            IngressTraffic::InternalOnly => "INGRESS_TRAFFIC_INTERNAL_ONLY",
            IngressTraffic::InternalLoadBalancer => {
                "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INGRESS_TRAFFIC_UNSPECIFIED" => Some(Self::Unspecified),
            "INGRESS_TRAFFIC_ALL" => Some(Self::All),
            "INGRESS_TRAFFIC_INTERNAL_ONLY" => Some(Self::InternalOnly),
            "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER" => Some(Self::InternalLoadBalancer),
            _ => None,
        }
    }
}
/// Alternatives for execution environments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionEnvironment {
    /// Unspecified
    Unspecified = 0,
    /// Uses the First Generation environment.
    Gen1 = 1,
    /// Uses Second Generation environment.
    Gen2 = 2,
}
impl ExecutionEnvironment {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionEnvironment::Unspecified => "EXECUTION_ENVIRONMENT_UNSPECIFIED",
            ExecutionEnvironment::Gen1 => "EXECUTION_ENVIRONMENT_GEN1",
            ExecutionEnvironment::Gen2 => "EXECUTION_ENVIRONMENT_GEN2",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_ENVIRONMENT_UNSPECIFIED" => Some(Self::Unspecified),
            "EXECUTION_ENVIRONMENT_GEN1" => Some(Self::Gen1),
            "EXECUTION_ENVIRONMENT_GEN2" => Some(Self::Gen2),
            _ => None,
        }
    }
}
/// Specifies behavior if an encryption key used by a resource is revoked.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncryptionKeyRevocationAction {
    /// Unspecified
    Unspecified = 0,
    /// Prevents the creation of new instances.
    PreventNew = 1,
    /// Shuts down existing instances, and prevents creation of new ones.
    Shutdown = 2,
}
impl EncryptionKeyRevocationAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EncryptionKeyRevocationAction::Unspecified => {
                "ENCRYPTION_KEY_REVOCATION_ACTION_UNSPECIFIED"
            }
            EncryptionKeyRevocationAction::PreventNew => "PREVENT_NEW",
            EncryptionKeyRevocationAction::Shutdown => "SHUTDOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENCRYPTION_KEY_REVOCATION_ACTION_UNSPECIFIED" => Some(Self::Unspecified),
            "PREVENT_NEW" => Some(Self::PreventNew),
            "SHUTDOWN" => Some(Self::Shutdown),
            _ => None,
        }
    }
}
/// TaskTemplate describes the data a task should have when created
/// from a template.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskTemplate {
    /// Holds the single container that defines the unit of execution for this
    /// task.
    #[prost(message, repeated, tag = "1")]
    pub containers: ::prost::alloc::vec::Vec<Container>,
    /// A list of Volumes to make available to containers.
    #[prost(message, repeated, tag = "2")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// Max allowed time duration the Task may be active before the system will
    /// actively try to mark it failed and kill associated containers. This applies
    /// per attempt of a task, meaning each retry can run for the full timeout.
    /// Defaults to 600 seconds.
    #[prost(message, optional, tag = "4")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Email address of the IAM service account associated with the Task of a
    /// Job. The service account represents the identity of the
    /// running task, and determines what permissions the task has. If
    /// not provided, the task will use the project's default service account.
    #[prost(string, tag = "5")]
    pub service_account: ::prost::alloc::string::String,
    /// The execution environment being used to host this Task.
    #[prost(enumeration = "ExecutionEnvironment", tag = "6")]
    pub execution_environment: i32,
    /// A reference to a customer managed encryption key (CMEK) to use to encrypt
    /// this container image. For more information, go to
    /// <https://cloud.google.com/run/docs/securing/using-cmek>
    #[prost(string, tag = "7")]
    pub encryption_key: ::prost::alloc::string::String,
    /// VPC Access configuration to use for this Task. For more information,
    /// visit <https://cloud.google.com/run/docs/configuring/connecting-vpc.>
    #[prost(message, optional, tag = "8")]
    pub vpc_access: ::core::option::Option<VpcAccess>,
    #[prost(oneof = "task_template::Retries", tags = "3")]
    pub retries: ::core::option::Option<task_template::Retries>,
}
/// Nested message and enum types in `TaskTemplate`.
pub mod task_template {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Retries {
        /// Number of retries allowed per Task, before marking this Task failed.
        /// Defaults to 3.
        #[prost(int32, tag = "3")]
        MaxRetries(i32),
    }
}
/// Holds a single traffic routing entry for the Service. Allocations can be done
/// to a specific Revision name, or pointing to the latest Ready Revision.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficTarget {
    /// The allocation type for this traffic target.
    #[prost(enumeration = "TrafficTargetAllocationType", tag = "1")]
    pub r#type: i32,
    /// Revision to which to send this portion of traffic, if traffic allocation is
    /// by revision.
    #[prost(string, tag = "2")]
    pub revision: ::prost::alloc::string::String,
    /// Specifies percent of the traffic to this Revision.
    /// This defaults to zero if unspecified.
    #[prost(int32, tag = "3")]
    pub percent: i32,
    /// Indicates a string to be part of the URI to exclusively reference this
    /// target.
    #[prost(string, tag = "4")]
    pub tag: ::prost::alloc::string::String,
}
/// Represents the observed state of a single `TrafficTarget` entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficTargetStatus {
    /// The allocation type for this traffic target.
    #[prost(enumeration = "TrafficTargetAllocationType", tag = "1")]
    pub r#type: i32,
    /// Revision to which this traffic is sent.
    #[prost(string, tag = "2")]
    pub revision: ::prost::alloc::string::String,
    /// Specifies percent of the traffic to this Revision.
    #[prost(int32, tag = "3")]
    pub percent: i32,
    /// Indicates the string used in the URI to exclusively reference this target.
    #[prost(string, tag = "4")]
    pub tag: ::prost::alloc::string::String,
    /// Displays the target URI.
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
}
/// The type of instance allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrafficTargetAllocationType {
    /// Unspecified instance allocation type.
    Unspecified = 0,
    /// Allocates instances to the Service's latest ready Revision.
    Latest = 1,
    /// Allocates instances to a Revision by name.
    Revision = 2,
}
impl TrafficTargetAllocationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrafficTargetAllocationType::Unspecified => {
                "TRAFFIC_TARGET_ALLOCATION_TYPE_UNSPECIFIED"
            }
            TrafficTargetAllocationType::Latest => {
                "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST"
            }
            TrafficTargetAllocationType::Revision => {
                "TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRAFFIC_TARGET_ALLOCATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST" => Some(Self::Latest),
            "TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION" => Some(Self::Revision),
            _ => None,
        }
    }
}
/// Defines a status condition for a resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    /// type is used to communicate the status of the reconciliation process.
    /// See also:
    /// <https://github.com/knative/serving/blob/main/docs/spec/errors.md#error-conditions-and-reporting>
    /// Types common to all resources include:
    /// * "Ready": True when the Resource is ready.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// State of the condition.
    #[prost(enumeration = "condition::State", tag = "2")]
    pub state: i32,
    /// Human readable message indicating details about the current status.
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    /// Last time the condition transitioned from one status to another.
    #[prost(message, optional, tag = "4")]
    pub last_transition_time: ::core::option::Option<::prost_types::Timestamp>,
    /// How to interpret failures of this condition, one of Error, Warning, Info
    #[prost(enumeration = "condition::Severity", tag = "5")]
    pub severity: i32,
    /// The reason for this condition. Depending on the condition type,
    /// it will populate one of these fields.
    /// Successful conditions cannot have a reason.
    #[prost(oneof = "condition::Reasons", tags = "6, 9, 11")]
    pub reasons: ::core::option::Option<condition::Reasons>,
}
/// Nested message and enum types in `Condition`.
pub mod condition {
    /// Represents the possible Condition states.
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
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// Transient state: Reconciliation has not started yet.
        ConditionPending = 1,
        /// Transient state: reconciliation is still in progress.
        ConditionReconciling = 2,
        /// Terminal state: Reconciliation did not succeed.
        ConditionFailed = 3,
        /// Terminal state: Reconciliation completed successfully.
        ConditionSucceeded = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::ConditionPending => "CONDITION_PENDING",
                State::ConditionReconciling => "CONDITION_RECONCILING",
                State::ConditionFailed => "CONDITION_FAILED",
                State::ConditionSucceeded => "CONDITION_SUCCEEDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CONDITION_PENDING" => Some(Self::ConditionPending),
                "CONDITION_RECONCILING" => Some(Self::ConditionReconciling),
                "CONDITION_FAILED" => Some(Self::ConditionFailed),
                "CONDITION_SUCCEEDED" => Some(Self::ConditionSucceeded),
                _ => None,
            }
        }
    }
    /// Represents the severity of the condition failures.
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
    pub enum Severity {
        /// Unspecified severity
        Unspecified = 0,
        /// Error severity.
        Error = 1,
        /// Warning severity.
        Warning = 2,
        /// Info severity.
        Info = 3,
    }
    impl Severity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Severity::Unspecified => "SEVERITY_UNSPECIFIED",
                Severity::Error => "ERROR",
                Severity::Warning => "WARNING",
                Severity::Info => "INFO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
                "ERROR" => Some(Self::Error),
                "WARNING" => Some(Self::Warning),
                "INFO" => Some(Self::Info),
                _ => None,
            }
        }
    }
    /// Reasons common to all types of conditions.
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
    pub enum CommonReason {
        /// Default value.
        Undefined = 0,
        /// Reason unknown. Further details will be in message.
        Unknown = 1,
        /// Revision creation process failed.
        RevisionFailed = 3,
        /// Timed out waiting for completion.
        ProgressDeadlineExceeded = 4,
        /// The container image path is incorrect.
        ContainerMissing = 6,
        /// Insufficient permissions on the container image.
        ContainerPermissionDenied = 7,
        /// Container image is not authorized by policy.
        ContainerImageUnauthorized = 8,
        /// Container image policy authorization check failed.
        ContainerImageAuthorizationCheckFailed = 9,
        /// Insufficient permissions on encryption key.
        EncryptionKeyPermissionDenied = 10,
        /// Permission check on encryption key failed.
        EncryptionKeyCheckFailed = 11,
        /// At least one Access check on secrets failed.
        SecretsAccessCheckFailed = 12,
        /// Waiting for operation to complete.
        WaitingForOperation = 13,
        /// System will retry immediately.
        ImmediateRetry = 14,
        /// System will retry later; current attempt failed.
        PostponedRetry = 15,
        /// An internal error occurred. Further information may be in the message.
        Internal = 16,
    }
    impl CommonReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CommonReason::Undefined => "COMMON_REASON_UNDEFINED",
                CommonReason::Unknown => "UNKNOWN",
                CommonReason::RevisionFailed => "REVISION_FAILED",
                CommonReason::ProgressDeadlineExceeded => "PROGRESS_DEADLINE_EXCEEDED",
                CommonReason::ContainerMissing => "CONTAINER_MISSING",
                CommonReason::ContainerPermissionDenied => "CONTAINER_PERMISSION_DENIED",
                CommonReason::ContainerImageUnauthorized => {
                    "CONTAINER_IMAGE_UNAUTHORIZED"
                }
                CommonReason::ContainerImageAuthorizationCheckFailed => {
                    "CONTAINER_IMAGE_AUTHORIZATION_CHECK_FAILED"
                }
                CommonReason::EncryptionKeyPermissionDenied => {
                    "ENCRYPTION_KEY_PERMISSION_DENIED"
                }
                CommonReason::EncryptionKeyCheckFailed => "ENCRYPTION_KEY_CHECK_FAILED",
                CommonReason::SecretsAccessCheckFailed => "SECRETS_ACCESS_CHECK_FAILED",
                CommonReason::WaitingForOperation => "WAITING_FOR_OPERATION",
                CommonReason::ImmediateRetry => "IMMEDIATE_RETRY",
                CommonReason::PostponedRetry => "POSTPONED_RETRY",
                CommonReason::Internal => "INTERNAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMMON_REASON_UNDEFINED" => Some(Self::Undefined),
                "UNKNOWN" => Some(Self::Unknown),
                "REVISION_FAILED" => Some(Self::RevisionFailed),
                "PROGRESS_DEADLINE_EXCEEDED" => Some(Self::ProgressDeadlineExceeded),
                "CONTAINER_MISSING" => Some(Self::ContainerMissing),
                "CONTAINER_PERMISSION_DENIED" => Some(Self::ContainerPermissionDenied),
                "CONTAINER_IMAGE_UNAUTHORIZED" => Some(Self::ContainerImageUnauthorized),
                "CONTAINER_IMAGE_AUTHORIZATION_CHECK_FAILED" => {
                    Some(Self::ContainerImageAuthorizationCheckFailed)
                }
                "ENCRYPTION_KEY_PERMISSION_DENIED" => {
                    Some(Self::EncryptionKeyPermissionDenied)
                }
                "ENCRYPTION_KEY_CHECK_FAILED" => Some(Self::EncryptionKeyCheckFailed),
                "SECRETS_ACCESS_CHECK_FAILED" => Some(Self::SecretsAccessCheckFailed),
                "WAITING_FOR_OPERATION" => Some(Self::WaitingForOperation),
                "IMMEDIATE_RETRY" => Some(Self::ImmediateRetry),
                "POSTPONED_RETRY" => Some(Self::PostponedRetry),
                "INTERNAL" => Some(Self::Internal),
                _ => None,
            }
        }
    }
    /// Reasons specific to Revision resource.
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
    pub enum RevisionReason {
        /// Default value.
        Undefined = 0,
        /// Revision in Pending state.
        Pending = 1,
        /// Revision is in Reserve state.
        Reserve = 2,
        /// Revision is Retired.
        Retired = 3,
        /// Revision is being retired.
        Retiring = 4,
        /// Revision is being recreated.
        Recreating = 5,
        /// There was a health check error.
        HealthCheckContainerError = 6,
        /// Health check failed due to user error from customized path of the
        /// container. System will retry.
        CustomizedPathResponsePending = 7,
        /// A revision with min_instance_count > 0 was created and is reserved, but
        /// it was not configured to serve traffic, so it's not live. This can also
        /// happen momentarily during traffic migration.
        MinInstancesNotProvisioned = 8,
        /// The maximum allowed number of active revisions has been reached.
        ActiveRevisionLimitReached = 9,
        /// There was no deployment defined.
        /// This value is no longer used, but Services created in older versions of
        /// the API might contain this value.
        NoDeployment = 10,
        /// A revision's container has no port specified since the revision is of a
        /// manually scaled service with 0 instance count
        HealthCheckSkipped = 11,
        /// A revision with min_instance_count > 0 was created and is waiting for
        /// enough instances to begin a traffic migration.
        MinInstancesWarming = 12,
    }
    impl RevisionReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RevisionReason::Undefined => "REVISION_REASON_UNDEFINED",
                RevisionReason::Pending => "PENDING",
                RevisionReason::Reserve => "RESERVE",
                RevisionReason::Retired => "RETIRED",
                RevisionReason::Retiring => "RETIRING",
                RevisionReason::Recreating => "RECREATING",
                RevisionReason::HealthCheckContainerError => {
                    "HEALTH_CHECK_CONTAINER_ERROR"
                }
                RevisionReason::CustomizedPathResponsePending => {
                    "CUSTOMIZED_PATH_RESPONSE_PENDING"
                }
                RevisionReason::MinInstancesNotProvisioned => {
                    "MIN_INSTANCES_NOT_PROVISIONED"
                }
                RevisionReason::ActiveRevisionLimitReached => {
                    "ACTIVE_REVISION_LIMIT_REACHED"
                }
                RevisionReason::NoDeployment => "NO_DEPLOYMENT",
                RevisionReason::HealthCheckSkipped => "HEALTH_CHECK_SKIPPED",
                RevisionReason::MinInstancesWarming => "MIN_INSTANCES_WARMING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REVISION_REASON_UNDEFINED" => Some(Self::Undefined),
                "PENDING" => Some(Self::Pending),
                "RESERVE" => Some(Self::Reserve),
                "RETIRED" => Some(Self::Retired),
                "RETIRING" => Some(Self::Retiring),
                "RECREATING" => Some(Self::Recreating),
                "HEALTH_CHECK_CONTAINER_ERROR" => Some(Self::HealthCheckContainerError),
                "CUSTOMIZED_PATH_RESPONSE_PENDING" => {
                    Some(Self::CustomizedPathResponsePending)
                }
                "MIN_INSTANCES_NOT_PROVISIONED" => Some(Self::MinInstancesNotProvisioned),
                "ACTIVE_REVISION_LIMIT_REACHED" => Some(Self::ActiveRevisionLimitReached),
                "NO_DEPLOYMENT" => Some(Self::NoDeployment),
                "HEALTH_CHECK_SKIPPED" => Some(Self::HealthCheckSkipped),
                "MIN_INSTANCES_WARMING" => Some(Self::MinInstancesWarming),
                _ => None,
            }
        }
    }
    /// Reasons specific to Execution resource.
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
    pub enum ExecutionReason {
        /// Default value.
        Undefined = 0,
        /// Internal system error getting execution status. System will retry.
        JobStatusServicePollingError = 1,
        /// A task reached its retry limit and the last attempt failed due to the
        /// user container exiting with a non-zero exit code.
        NonZeroExitCode = 2,
        /// The execution was cancelled by users.
        Cancelled = 3,
        /// The execution is in the process of being cancelled.
        Cancelling = 4,
        /// The execution was deleted.
        Deleted = 5,
    }
    impl ExecutionReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExecutionReason::Undefined => "EXECUTION_REASON_UNDEFINED",
                ExecutionReason::JobStatusServicePollingError => {
                    "JOB_STATUS_SERVICE_POLLING_ERROR"
                }
                ExecutionReason::NonZeroExitCode => "NON_ZERO_EXIT_CODE",
                ExecutionReason::Cancelled => "CANCELLED",
                ExecutionReason::Cancelling => "CANCELLING",
                ExecutionReason::Deleted => "DELETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EXECUTION_REASON_UNDEFINED" => Some(Self::Undefined),
                "JOB_STATUS_SERVICE_POLLING_ERROR" => {
                    Some(Self::JobStatusServicePollingError)
                }
                "NON_ZERO_EXIT_CODE" => Some(Self::NonZeroExitCode),
                "CANCELLED" => Some(Self::Cancelled),
                "CANCELLING" => Some(Self::Cancelling),
                "DELETED" => Some(Self::Deleted),
                _ => None,
            }
        }
    }
    /// The reason for this condition. Depending on the condition type,
    /// it will populate one of these fields.
    /// Successful conditions cannot have a reason.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Reasons {
        /// A common (service-level) reason for this condition.
        #[prost(enumeration = "CommonReason", tag = "6")]
        Reason(i32),
        /// A reason for the revision condition.
        #[prost(enumeration = "RevisionReason", tag = "9")]
        RevisionReason(i32),
        /// A reason for the execution condition.
        #[prost(enumeration = "ExecutionReason", tag = "11")]
        ExecutionReason(i32),
    }
}
/// Request message for obtaining a Task by its full name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskRequest {
    /// Required. The full name of the Task.
    /// Format:
    /// projects/{project}/locations/{location}/jobs/{job}/executions/{execution}/tasks/{task}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for retrieving a list of Tasks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksRequest {
    /// Required. The Execution from which the Tasks should be listed.
    /// To list all Tasks across Executions of a Job, use "-" instead of Execution
    /// name. To list all Tasks across Jobs, use "-" instead of Job name. Format:
    /// projects/{project}/locations/{location}/jobs/{job}/executions/{execution}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of Tasks to return in this call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token received from a previous call to ListTasks.
    /// All other parameters must match.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// If true, returns deleted (but unexpired) resources along with active ones.
    #[prost(bool, tag = "4")]
    pub show_deleted: bool,
}
/// Response message containing a list of Tasks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksResponse {
    /// The resulting list of Tasks.
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
    /// A token indicating there are more items than page_size. Use it in the next
    /// ListTasks request to continue.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Task represents a single run of a container to completion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// Output only. The unique name of this Task.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server assigned unique identifier for the Task. The value is a
    /// UUID4 string and guaranteed to remain unchanged until the resource is
    /// deleted.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. A number that monotonically increases every time the user
    /// modifies the desired state.
    #[prost(int64, tag = "3")]
    pub generation: i64,
    /// Output only. Unstructured key value map that can be used to organize and
    /// categorize objects. User-provided labels are shared with Google's billing
    /// system, so they can be used to filter, or break down billing charges by
    /// team, component, environment, state, etc. For more information, visit
    /// <https://cloud.google.com/resource-manager/docs/creating-managing-labels> or
    /// <https://cloud.google.com/run/docs/configuring/labels>
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Unstructured key value map that may
    /// be set by external tools to store and arbitrary metadata.
    /// They are not queryable and should be preserved
    /// when modifying objects.
    #[prost(btree_map = "string, string", tag = "5")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Represents time when the task was created by the system.
    /// It is not guaranteed to be set in happens-before order across separate
    /// operations.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Represents time when the task was scheduled to run by the
    /// system. It is not guaranteed to be set in happens-before order across
    /// separate operations.
    #[prost(message, optional, tag = "34")]
    pub scheduled_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Represents time when the task started to run.
    /// It is not guaranteed to be set in happens-before order across separate
    /// operations.
    #[prost(message, optional, tag = "27")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Represents time when the Task was completed. It is not
    /// guaranteed to be set in happens-before order across separate operations.
    #[prost(message, optional, tag = "7")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. For a deleted resource, the deletion time. It is only
    /// populated as a response to a Delete request.
    #[prost(message, optional, tag = "9")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. For a deleted resource, the time after which it will be
    /// permamently deleted. It is only populated as a response to a Delete
    /// request.
    #[prost(message, optional, tag = "10")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The name of the parent Job.
    #[prost(string, tag = "12")]
    pub job: ::prost::alloc::string::String,
    /// Output only. The name of the parent Execution.
    #[prost(string, tag = "13")]
    pub execution: ::prost::alloc::string::String,
    /// Holds the single container that defines the unit of execution for this
    /// task.
    #[prost(message, repeated, tag = "14")]
    pub containers: ::prost::alloc::vec::Vec<Container>,
    /// A list of Volumes to make available to containers.
    #[prost(message, repeated, tag = "15")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// Number of retries allowed per Task, before marking this Task failed.
    #[prost(int32, tag = "16")]
    pub max_retries: i32,
    /// Max allowed time duration the Task may be active before the system will
    /// actively try to mark it failed and kill associated containers. This applies
    /// per attempt of a task, meaning each retry can run for the full timeout.
    #[prost(message, optional, tag = "17")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Email address of the IAM service account associated with the Task of a
    /// Job. The service account represents the identity of the
    /// running task, and determines what permissions the task has. If
    /// not provided, the task will use the project's default service account.
    #[prost(string, tag = "18")]
    pub service_account: ::prost::alloc::string::String,
    /// The execution environment being used to host this Task.
    #[prost(enumeration = "ExecutionEnvironment", tag = "20")]
    pub execution_environment: i32,
    /// Output only. Indicates whether the resource's reconciliation is still in
    /// progress. See comments in `Job.reconciling` for additional information on
    /// reconciliation process in Cloud Run.
    #[prost(bool, tag = "21")]
    pub reconciling: bool,
    /// Output only. The Condition of this Task, containing its readiness status,
    /// and detailed error information in case it did not reach the desired state.
    #[prost(message, repeated, tag = "22")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    /// Output only. The generation of this Task. See comments in `Job.reconciling`
    /// for additional information on reconciliation process in Cloud Run.
    #[prost(int64, tag = "23")]
    pub observed_generation: i64,
    /// Output only. Index of the Task, unique per execution, and beginning at 0.
    #[prost(int32, tag = "24")]
    pub index: i32,
    /// Output only. The number of times this Task was retried.
    /// Tasks are retried when they fail up to the maxRetries limit.
    #[prost(int32, tag = "25")]
    pub retried: i32,
    /// Output only. Result of the last attempt of this Task.
    #[prost(message, optional, tag = "26")]
    pub last_attempt_result: ::core::option::Option<TaskAttemptResult>,
    /// Output only. A reference to a customer managed encryption key (CMEK) to use
    /// to encrypt this container image. For more information, go to
    /// <https://cloud.google.com/run/docs/securing/using-cmek>
    #[prost(string, tag = "28")]
    pub encryption_key: ::prost::alloc::string::String,
    /// Output only. VPC Access configuration to use for this Task. For more
    /// information, visit
    /// <https://cloud.google.com/run/docs/configuring/connecting-vpc.>
    #[prost(message, optional, tag = "29")]
    pub vpc_access: ::core::option::Option<VpcAccess>,
    /// Output only. URI where logs for this execution can be found in Cloud
    /// Console.
    #[prost(string, tag = "32")]
    pub log_uri: ::prost::alloc::string::String,
    /// Output only. Reserved for future use.
    #[prost(bool, tag = "33")]
    pub satisfies_pzs: bool,
    /// Output only. A system-generated fingerprint for this version of the
    /// resource. May be used to detect modification conflict during updates.
    #[prost(string, tag = "99")]
    pub etag: ::prost::alloc::string::String,
}
/// Result of a task attempt.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskAttemptResult {
    /// Output only. The status of this attempt.
    /// If the status code is OK, then the attempt succeeded.
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Output only. The exit code of this attempt.
    /// This may be unset if the container was unable to exit cleanly with a code
    /// due to some other failure.
    /// See status field for possible failure details.
    #[prost(int32, tag = "2")]
    pub exit_code: i32,
}
/// Generated client implementations.
pub mod tasks_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Cloud Run Task Control Plane API.
    #[derive(Debug, Clone)]
    pub struct TasksClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TasksClient<T>
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
        ) -> TasksClient<InterceptedService<T, F>>
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
            TasksClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets information about a Task.
        pub async fn get_task(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTaskRequest>,
        ) -> std::result::Result<tonic::Response<super::Task>, tonic::Status> {
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
                "/google.cloud.run.v2.Tasks/GetTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Tasks", "GetTask"));
            self.inner.unary(req, path, codec).await
        }
        /// Lists Tasks from an Execution of a Job.
        pub async fn list_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTasksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTasksResponse>,
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
                "/google.cloud.run.v2.Tasks/ListTasks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Tasks", "ListTasks"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Effective settings for the current revision
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevisionScalingStatus {
    /// The current number of min instances provisioned for this revision.
    #[prost(int32, tag = "1")]
    pub desired_min_instance_count: i32,
}
/// Request message for obtaining a Execution by its full name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExecutionRequest {
    /// Required. The full name of the Execution.
    /// Format:
    /// `projects/{project}/locations/{location}/jobs/{job}/executions/{execution}`,
    /// where `{project}` can be project id or number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for retrieving a list of Executions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionsRequest {
    /// Required. The Execution from which the Executions should be listed.
    /// To list all Executions across Jobs, use "-" instead of Job name.
    /// Format: `projects/{project}/locations/{location}/jobs/{job}`, where
    /// `{project}` can be project id or number.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of Executions to return in this call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token received from a previous call to ListExecutions.
    /// All other parameters must match.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// If true, returns deleted (but unexpired) resources along with active ones.
    #[prost(bool, tag = "4")]
    pub show_deleted: bool,
}
/// Response message containing a list of Executions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionsResponse {
    /// The resulting list of Executions.
    #[prost(message, repeated, tag = "1")]
    pub executions: ::prost::alloc::vec::Vec<Execution>,
    /// A token indicating there are more items than page_size. Use it in the next
    /// ListExecutions request to continue.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for deleting an Execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteExecutionRequest {
    /// Required. The name of the Execution to delete.
    /// Format:
    /// `projects/{project}/locations/{location}/jobs/{job}/executions/{execution}`,
    /// where `{project}` can be project id or number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates that the request should be validated without actually
    /// deleting any resources.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// A system-generated fingerprint for this version of the resource.
    /// This may be used to detect modification conflict during updates.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for deleting an Execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelExecutionRequest {
    /// Required. The name of the Execution to cancel.
    /// Format:
    /// `projects/{project}/locations/{location}/jobs/{job}/executions/{execution}`,
    /// where `{project}` can be project id or number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates that the request should be validated without actually
    /// cancelling any resources.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// A system-generated fingerprint for this version of the resource.
    /// This may be used to detect modification conflict during updates.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
/// Execution represents the configuration of a single execution. A execution an
/// immutable resource that references a container image which is run to
/// completion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Execution {
    /// Output only. The unique name of this Execution.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server assigned unique identifier for the Execution. The value
    /// is a UUID4 string and guaranteed to remain unchanged until the resource is
    /// deleted.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. A number that monotonically increases every time the user
    /// modifies the desired state.
    #[prost(int64, tag = "3")]
    pub generation: i64,
    /// Output only. Unstructured key value map that can be used to organize and
    /// categorize objects. User-provided labels are shared with Google's billing
    /// system, so they can be used to filter, or break down billing charges by
    /// team, component, environment, state, etc. For more information, visit
    /// <https://cloud.google.com/resource-manager/docs/creating-managing-labels> or
    /// <https://cloud.google.com/run/docs/configuring/labels>
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Unstructured key value map that may
    /// be set by external tools to store and arbitrary metadata.
    /// They are not queryable and should be preserved
    /// when modifying objects.
    #[prost(btree_map = "string, string", tag = "5")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Represents time when the execution was acknowledged by the
    /// execution controller. It is not guaranteed to be set in happens-before
    /// order across separate operations.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Represents time when the execution started to run.
    /// It is not guaranteed to be set in happens-before order across separate
    /// operations.
    #[prost(message, optional, tag = "22")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Represents time when the execution was completed. It is not
    /// guaranteed to be set in happens-before order across separate operations.
    #[prost(message, optional, tag = "7")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. For a deleted resource, the deletion time. It is only
    /// populated as a response to a Delete request.
    #[prost(message, optional, tag = "9")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. For a deleted resource, the time after which it will be
    /// permamently deleted. It is only populated as a response to a Delete
    /// request.
    #[prost(message, optional, tag = "10")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The least stable launch stage needed to create this resource, as defined by
    /// [Google Cloud Platform Launch
    /// Stages](<https://cloud.google.com/terms/launch-stages>). Cloud Run supports
    /// `ALPHA`, `BETA`, and `GA`.
    /// <p>Note that this value might not be what was used
    /// as input. For example, if ALPHA was provided as input in the parent
    /// resource, but only BETA and GA-level features are were, this field will be
    /// BETA.
    #[prost(enumeration = "super::super::super::api::LaunchStage", tag = "11")]
    pub launch_stage: i32,
    /// Output only. The name of the parent Job.
    #[prost(string, tag = "12")]
    pub job: ::prost::alloc::string::String,
    /// Output only. Specifies the maximum desired number of tasks the execution
    /// should run at any given time. Must be <= task_count. The actual number of
    /// tasks running in steady state will be less than this number when
    /// ((.spec.task_count - .status.successful) < .spec.parallelism), i.e. when
    /// the work left to do is less than max parallelism.
    #[prost(int32, tag = "13")]
    pub parallelism: i32,
    /// Output only. Specifies the desired number of tasks the execution should
    /// run. Setting to 1 means that parallelism is limited to 1 and the success of
    /// that task signals the success of the execution.
    #[prost(int32, tag = "14")]
    pub task_count: i32,
    /// Output only. The template used to create tasks for this execution.
    #[prost(message, optional, tag = "15")]
    pub template: ::core::option::Option<TaskTemplate>,
    /// Output only. Indicates whether the resource's reconciliation is still in
    /// progress. See comments in `Job.reconciling` for additional information on
    /// reconciliation process in Cloud Run.
    #[prost(bool, tag = "16")]
    pub reconciling: bool,
    /// Output only. The Condition of this Execution, containing its readiness
    /// status, and detailed error information in case it did not reach the desired
    /// state.
    #[prost(message, repeated, tag = "17")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    /// Output only. The generation of this Execution. See comments in
    /// `reconciling` for additional information on reconciliation process in Cloud
    /// Run.
    #[prost(int64, tag = "18")]
    pub observed_generation: i64,
    /// Output only. The number of actively running tasks.
    #[prost(int32, tag = "19")]
    pub running_count: i32,
    /// Output only. The number of tasks which reached phase Succeeded.
    #[prost(int32, tag = "20")]
    pub succeeded_count: i32,
    /// Output only. The number of tasks which reached phase Failed.
    #[prost(int32, tag = "21")]
    pub failed_count: i32,
    /// Output only. The number of tasks which reached phase Cancelled.
    #[prost(int32, tag = "24")]
    pub cancelled_count: i32,
    /// Output only. The number of tasks which have retried at least once.
    #[prost(int32, tag = "25")]
    pub retried_count: i32,
    /// Output only. URI where logs for this execution can be found in Cloud
    /// Console.
    #[prost(string, tag = "26")]
    pub log_uri: ::prost::alloc::string::String,
    /// Output only. Reserved for future use.
    #[prost(bool, tag = "27")]
    pub satisfies_pzs: bool,
    /// Output only. A system-generated fingerprint for this version of the
    /// resource. May be used to detect modification conflict during updates.
    #[prost(string, tag = "99")]
    pub etag: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod executions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Cloud Run Execution Control Plane API.
    #[derive(Debug, Clone)]
    pub struct ExecutionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ExecutionsClient<T>
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
        ) -> ExecutionsClient<InterceptedService<T, F>>
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
            ExecutionsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets information about an Execution.
        pub async fn get_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExecutionRequest>,
        ) -> std::result::Result<tonic::Response<super::Execution>, tonic::Status> {
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
                "/google.cloud.run.v2.Executions/GetExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.run.v2.Executions", "GetExecution"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Executions from a Job.
        pub async fn list_executions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExecutionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExecutionsResponse>,
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
                "/google.cloud.run.v2.Executions/ListExecutions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.run.v2.Executions", "ListExecutions"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an Execution.
        pub async fn delete_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteExecutionRequest>,
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
                "/google.cloud.run.v2.Executions/DeleteExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.run.v2.Executions", "DeleteExecution"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Cancels an Execution.
        pub async fn cancel_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelExecutionRequest>,
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
                "/google.cloud.run.v2.Executions/CancelExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.run.v2.Executions", "CancelExecution"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// ExecutionTemplate describes the data an execution should have when created
/// from a template.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionTemplate {
    /// Unstructured key value map that can be used to organize and categorize
    /// objects.
    /// User-provided labels are shared with Google's billing system, so they can
    /// be used to filter, or break down billing charges by team, component,
    /// environment, state, etc. For more information, visit
    /// <https://cloud.google.com/resource-manager/docs/creating-managing-labels> or
    /// <https://cloud.google.com/run/docs/configuring/labels.>
    ///
    /// <p>Cloud Run API v2 does not support labels with `run.googleapis.com`,
    /// `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev`
    /// namespaces, and they will be rejected. All system labels in v1 now have a
    /// corresponding field in v2 ExecutionTemplate.
    #[prost(btree_map = "string, string", tag = "1")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Unstructured key value map that may be set by external tools to store and
    /// arbitrary metadata. They are not queryable and should be preserved
    /// when modifying objects.
    ///
    /// <p>Cloud Run API v2 does not support annotations with `run.googleapis.com`,
    /// `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev`
    /// namespaces, and they will be rejected. All system annotations in v1 now
    /// have a corresponding field in v2 ExecutionTemplate.
    ///
    /// <p>This field follows Kubernetes annotations' namespacing, limits, and
    /// rules.
    #[prost(btree_map = "string, string", tag = "2")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Specifies the maximum desired number of tasks the execution should run at
    /// given time. Must be <= task_count.
    /// When the job is run, if this field is 0 or unset, the maximum possible
    /// value will be used for that execution.
    /// The actual number of tasks running in steady state will be less than this
    /// number when there are fewer tasks waiting to be completed remaining,
    /// i.e. when the work left to do is less than max parallelism.
    #[prost(int32, tag = "3")]
    pub parallelism: i32,
    /// Specifies the desired number of tasks the execution should run.
    /// Setting to 1 means that parallelism is limited to 1 and the success of
    /// that task signals the success of the execution. Defaults to 1.
    #[prost(int32, tag = "4")]
    pub task_count: i32,
    /// Required. Describes the task(s) that will be created when executing an
    /// execution.
    #[prost(message, optional, tag = "5")]
    pub template: ::core::option::Option<TaskTemplate>,
}
/// Request message for creating a Job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateJobRequest {
    /// Required. The location and project in which this Job should be created.
    /// Format: projects/{project}/locations/{location}, where {project} can be
    /// project id or number.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Job instance to create.
    #[prost(message, optional, tag = "2")]
    pub job: ::core::option::Option<Job>,
    /// Required. The unique identifier for the Job. The name of the job becomes
    /// {parent}/jobs/{job_id}.
    #[prost(string, tag = "3")]
    pub job_id: ::prost::alloc::string::String,
    /// Indicates that the request should be validated and default values
    /// populated, without persisting the request or creating any resources.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for obtaining a Job by its full name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobRequest {
    /// Required. The full name of the Job.
    /// Format: projects/{project}/locations/{location}/jobs/{job}, where {project}
    /// can be project id or number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for updating a Job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateJobRequest {
    /// Required. The Job to be updated.
    #[prost(message, optional, tag = "1")]
    pub job: ::core::option::Option<Job>,
    /// Indicates that the request should be validated and default values
    /// populated, without persisting the request or updating any resources.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// If set to true, and if the Job does not exist, it will create a new
    /// one. Caller must have both create and update permissions for this call if
    /// this is set to true.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// Request message for retrieving a list of Jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsRequest {
    /// Required. The location and project to list resources on.
    /// Format: projects/{project}/locations/{location}, where {project} can be
    /// project id or number.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of Jobs to return in this call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token received from a previous call to ListJobs.
    /// All other parameters must match.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// If true, returns deleted (but unexpired) resources along with active ones.
    #[prost(bool, tag = "4")]
    pub show_deleted: bool,
}
/// Response message containing a list of Jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsResponse {
    /// The resulting list of Jobs.
    #[prost(message, repeated, tag = "1")]
    pub jobs: ::prost::alloc::vec::Vec<Job>,
    /// A token indicating there are more items than page_size. Use it in the next
    /// ListJobs request to continue.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message to delete a Job by its full name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteJobRequest {
    /// Required. The full name of the Job.
    /// Format: projects/{project}/locations/{location}/jobs/{job}, where {project}
    /// can be project id or number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates that the request should be validated without actually
    /// deleting any resources.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// A system-generated fingerprint for this version of the
    /// resource. May be used to detect modification conflict during updates.
    #[prost(string, tag = "4")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message to create a new Execution of a Job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunJobRequest {
    /// Required. The full name of the Job.
    /// Format: projects/{project}/locations/{location}/jobs/{job}, where {project}
    /// can be project id or number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates that the request should be validated without actually
    /// deleting any resources.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// A system-generated fingerprint for this version of the
    /// resource. May be used to detect modification conflict during updates.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
    /// Overrides specification for a given execution of a job. If provided,
    /// overrides will be applied to update the execution or task spec.
    #[prost(message, optional, tag = "4")]
    pub overrides: ::core::option::Option<run_job_request::Overrides>,
}
/// Nested message and enum types in `RunJobRequest`.
pub mod run_job_request {
    /// RunJob Overrides that contains Execution fields to be overridden.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Overrides {
        /// Per container override specification.
        #[prost(message, repeated, tag = "1")]
        pub container_overrides: ::prost::alloc::vec::Vec<overrides::ContainerOverride>,
        /// Optional. The desired number of tasks the execution should run. Will
        /// replace existing task_count value.
        #[prost(int32, tag = "2")]
        pub task_count: i32,
        /// Duration in seconds the task may be active before the system will
        /// actively try to mark it failed and kill associated containers. Will
        /// replace existing timeout_seconds value.
        #[prost(message, optional, tag = "4")]
        pub timeout: ::core::option::Option<::prost_types::Duration>,
    }
    /// Nested message and enum types in `Overrides`.
    pub mod overrides {
        /// Per-container override specification.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ContainerOverride {
            /// The name of the container specified as a DNS_LABEL.
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            /// Optional. Arguments to the entrypoint. Will replace existing args for
            /// override.
            #[prost(string, repeated, tag = "2")]
            pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// List of environment variables to set in the container. Will be merged
            /// with existing env for override.
            #[prost(message, repeated, tag = "3")]
            pub env: ::prost::alloc::vec::Vec<super::super::EnvVar>,
            /// Optional. True if the intention is to clear out existing args list.
            #[prost(bool, tag = "4")]
            pub clear_args: bool,
        }
    }
}
/// Job represents the configuration of a single job, which references a
/// container image that is run to completion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// The fully qualified name of this Job.
    ///
    /// Format:
    /// projects/{project}/locations/{location}/jobs/{job}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server assigned unique identifier for the Execution. The value
    /// is a UUID4 string and guaranteed to remain unchanged until the resource is
    /// deleted.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. A number that monotonically increases every time the user
    /// modifies the desired state.
    #[prost(int64, tag = "3")]
    pub generation: i64,
    /// Unstructured key value map that can be used to organize and categorize
    /// objects.
    /// User-provided labels are shared with Google's billing system, so they can
    /// be used to filter, or break down billing charges by team, component,
    /// environment, state, etc. For more information, visit
    /// <https://cloud.google.com/resource-manager/docs/creating-managing-labels> or
    /// <https://cloud.google.com/run/docs/configuring/labels.>
    ///
    /// <p>Cloud Run API v2 does not support labels with `run.googleapis.com`,
    /// `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev`
    /// namespaces, and they will be rejected. All system labels in v1 now have a
    /// corresponding field in v2 Job.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Unstructured key value map that may
    /// be set by external tools to store and arbitrary metadata.
    /// They are not queryable and should be preserved
    /// when modifying objects.
    ///
    /// <p>Cloud Run API v2 does not support annotations with `run.googleapis.com`,
    /// `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev`
    /// namespaces, and they will be rejected on new resources. All system
    /// annotations in v1 now have a corresponding field in v2 Job.
    ///
    /// <p>This field follows Kubernetes annotations' namespacing, limits, and
    /// rules.
    #[prost(btree_map = "string, string", tag = "5")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The deletion time.
    #[prost(message, optional, tag = "8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. For a deleted resource, the time after which it will be
    /// permamently deleted.
    #[prost(message, optional, tag = "9")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Email address of the authenticated creator.
    #[prost(string, tag = "10")]
    pub creator: ::prost::alloc::string::String,
    /// Output only. Email address of the last authenticated modifier.
    #[prost(string, tag = "11")]
    pub last_modifier: ::prost::alloc::string::String,
    /// Arbitrary identifier for the API client.
    #[prost(string, tag = "12")]
    pub client: ::prost::alloc::string::String,
    /// Arbitrary version identifier for the API client.
    #[prost(string, tag = "13")]
    pub client_version: ::prost::alloc::string::String,
    /// The launch stage as defined by [Google Cloud Platform
    /// Launch Stages](<https://cloud.google.com/terms/launch-stages>).
    /// Cloud Run supports `ALPHA`, `BETA`, and `GA`. If no value is specified, GA
    /// is assumed.
    /// Set the launch stage to a preview stage on input to allow use of preview
    /// features in that stage. On read (or output), describes whether the resource
    /// uses preview features.
    /// <p>
    /// For example, if ALPHA is provided as input, but only BETA and GA-level
    /// features are used, this field will be BETA on output.
    #[prost(enumeration = "super::super::super::api::LaunchStage", tag = "14")]
    pub launch_stage: i32,
    /// Settings for the Binary Authorization feature.
    #[prost(message, optional, tag = "15")]
    pub binary_authorization: ::core::option::Option<BinaryAuthorization>,
    /// Required. The template used to create executions for this Job.
    #[prost(message, optional, tag = "16")]
    pub template: ::core::option::Option<ExecutionTemplate>,
    /// Output only. The generation of this Job. See comments in `reconciling` for
    /// additional information on reconciliation process in Cloud Run.
    #[prost(int64, tag = "17")]
    pub observed_generation: i64,
    /// Output only. The Condition of this Job, containing its readiness status,
    /// and detailed error information in case it did not reach the desired state.
    #[prost(message, optional, tag = "18")]
    pub terminal_condition: ::core::option::Option<Condition>,
    /// Output only. The Conditions of all other associated sub-resources. They
    /// contain additional diagnostics information in case the Job does not reach
    /// its desired state. See comments in `reconciling` for additional information
    /// on reconciliation process in Cloud Run.
    #[prost(message, repeated, tag = "19")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    /// Output only. Number of executions created for this job.
    #[prost(int32, tag = "20")]
    pub execution_count: i32,
    /// Output only. Name of the last created execution.
    #[prost(message, optional, tag = "22")]
    pub latest_created_execution: ::core::option::Option<ExecutionReference>,
    /// Output only. Returns true if the Job is currently being acted upon by the
    /// system to bring it into the desired state.
    ///
    /// When a new Job is created, or an existing one is updated, Cloud Run
    /// will asynchronously perform all necessary steps to bring the Job to the
    /// desired state. This process is called reconciliation.
    /// While reconciliation is in process, `observed_generation` and
    /// `latest_succeeded_execution`, will have transient values that might
    /// mismatch the intended state: Once reconciliation is over (and this field is
    /// false), there are two possible outcomes: reconciliation succeeded and the
    /// state matches the Job, or there was an error,  and reconciliation failed.
    /// This state can be found in `terminal_condition.state`.
    ///
    /// If reconciliation succeeded, the following fields will match:
    /// `observed_generation` and `generation`, `latest_succeeded_execution` and
    /// `latest_created_execution`.
    ///
    /// If reconciliation failed, `observed_generation` and
    /// `latest_succeeded_execution` will have the state of the last succeeded
    /// execution or empty for newly created Job. Additional information on the
    /// failure can be found in `terminal_condition` and `conditions`.
    #[prost(bool, tag = "23")]
    pub reconciling: bool,
    /// Output only. Reserved for future use.
    #[prost(bool, tag = "25")]
    pub satisfies_pzs: bool,
    /// Output only. A system-generated fingerprint for this version of the
    /// resource. May be used to detect modification conflict during updates.
    #[prost(string, tag = "99")]
    pub etag: ::prost::alloc::string::String,
}
/// Reference to an Execution. Use /Executions.GetExecution with the given name
/// to get full execution including the latest status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionReference {
    /// Name of the execution.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Creation timestamp of the execution.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Creation timestamp of the execution.
    #[prost(message, optional, tag = "3")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod jobs_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Cloud Run Job Control Plane API.
    #[derive(Debug, Clone)]
    pub struct JobsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> JobsClient<T>
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
        ) -> JobsClient<InterceptedService<T, F>>
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
            JobsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a Job.
        pub async fn create_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateJobRequest>,
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
                "/google.cloud.run.v2.Jobs/CreateJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Jobs", "CreateJob"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets information about a Job.
        pub async fn get_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobRequest>,
        ) -> std::result::Result<tonic::Response<super::Job>, tonic::Status> {
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
                "/google.cloud.run.v2.Jobs/GetJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Jobs", "GetJob"));
            self.inner.unary(req, path, codec).await
        }
        /// Lists Jobs.
        pub async fn list_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListJobsResponse>,
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
                "/google.cloud.run.v2.Jobs/ListJobs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Jobs", "ListJobs"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates a Job.
        pub async fn update_job(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateJobRequest>,
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
                "/google.cloud.run.v2.Jobs/UpdateJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Jobs", "UpdateJob"));
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Job.
        pub async fn delete_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteJobRequest>,
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
                "/google.cloud.run.v2.Jobs/DeleteJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Jobs", "DeleteJob"));
            self.inner.unary(req, path, codec).await
        }
        /// Triggers creation of a new Execution of this Job.
        pub async fn run_job(
            &mut self,
            request: impl tonic::IntoRequest<super::RunJobRequest>,
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
                "/google.cloud.run.v2.Jobs/RunJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Jobs", "RunJob"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the IAM Access Control policy currently in effect for the given Job.
        /// This result does not include any inherited policies.
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
                "/google.cloud.run.v2.Jobs/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Jobs", "GetIamPolicy"));
            self.inner.unary(req, path, codec).await
        }
        /// Sets the IAM Access control policy for the specified Job. Overwrites
        /// any existing policy.
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
                "/google.cloud.run.v2.Jobs/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Jobs", "SetIamPolicy"));
            self.inner.unary(req, path, codec).await
        }
        /// Returns permissions that a caller has on the specified Project.
        ///
        /// There are no permissions required for making this API call.
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
                "/google.cloud.run.v2.Jobs/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.run.v2.Jobs", "TestIamPermissions"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// RevisionTemplate describes the data a revision should have when created from
/// a template.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevisionTemplate {
    /// The unique name for the revision. If this field is omitted, it will be
    /// automatically generated based on the Service name.
    #[prost(string, tag = "1")]
    pub revision: ::prost::alloc::string::String,
    /// Unstructured key value map that can be used to organize and categorize
    /// objects.
    /// User-provided labels are shared with Google's billing system, so they can
    /// be used to filter, or break down billing charges by team, component,
    /// environment, state, etc. For more information, visit
    /// <https://cloud.google.com/resource-manager/docs/creating-managing-labels> or
    /// <https://cloud.google.com/run/docs/configuring/labels.>
    ///
    /// <p>Cloud Run API v2 does not support labels with `run.googleapis.com`,
    /// `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev`
    /// namespaces, and they will be rejected. All system labels in v1 now have a
    /// corresponding field in v2 RevisionTemplate.
    #[prost(btree_map = "string, string", tag = "2")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Unstructured key value map that may be set by external tools to store and
    /// arbitrary metadata. They are not queryable and should be preserved
    /// when modifying objects.
    ///
    /// <p>Cloud Run API v2 does not support annotations with `run.googleapis.com`,
    /// `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev`
    /// namespaces, and they will be rejected. All system annotations in v1 now
    /// have a corresponding field in v2 RevisionTemplate.
    ///
    /// <p>This field follows Kubernetes annotations' namespacing, limits, and
    /// rules.
    #[prost(btree_map = "string, string", tag = "3")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Scaling settings for this Revision.
    #[prost(message, optional, tag = "4")]
    pub scaling: ::core::option::Option<RevisionScaling>,
    /// VPC Access configuration to use for this Revision. For more information,
    /// visit <https://cloud.google.com/run/docs/configuring/connecting-vpc.>
    #[prost(message, optional, tag = "6")]
    pub vpc_access: ::core::option::Option<VpcAccess>,
    /// Max allowed time for an instance to respond to a request.
    #[prost(message, optional, tag = "8")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Email address of the IAM service account associated with the revision of
    /// the service. The service account represents the identity of the running
    /// revision, and determines what permissions the revision has. If not
    /// provided, the revision will use the project's default service account.
    #[prost(string, tag = "9")]
    pub service_account: ::prost::alloc::string::String,
    /// Holds the single container that defines the unit of execution for this
    /// Revision.
    #[prost(message, repeated, tag = "10")]
    pub containers: ::prost::alloc::vec::Vec<Container>,
    /// A list of Volumes to make available to containers.
    #[prost(message, repeated, tag = "11")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// The sandbox environment to host this Revision.
    #[prost(enumeration = "ExecutionEnvironment", tag = "13")]
    pub execution_environment: i32,
    /// A reference to a customer managed encryption key (CMEK) to use to encrypt
    /// this container image. For more information, go to
    /// <https://cloud.google.com/run/docs/securing/using-cmek>
    #[prost(string, tag = "14")]
    pub encryption_key: ::prost::alloc::string::String,
    /// Sets the maximum number of requests that each serving instance can receive.
    #[prost(int32, tag = "15")]
    pub max_instance_request_concurrency: i32,
    /// Optional. Enable session affinity.
    #[prost(bool, tag = "19")]
    pub session_affinity: bool,
    /// Optional. Disables health checking containers during deployment.
    #[prost(bool, tag = "20")]
    pub health_check_disabled: bool,
}
/// Request message for obtaining a Revision by its full name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRevisionRequest {
    /// Required. The full name of the Revision.
    /// Format:
    /// projects/{project}/locations/{location}/services/{service}/revisions/{revision}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for retrieving a list of Revisions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRevisionsRequest {
    /// Required. The Service from which the Revisions should be listed.
    /// To list all Revisions across Services, use "-" instead of Service name.
    /// Format:
    /// projects/{project}/locations/{location}/services/{service}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of revisions to return in this call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token received from a previous call to ListRevisions.
    /// All other parameters must match.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// If true, returns deleted (but unexpired) resources along with active ones.
    #[prost(bool, tag = "4")]
    pub show_deleted: bool,
}
/// Response message containing a list of Revisions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRevisionsResponse {
    /// The resulting list of Revisions.
    #[prost(message, repeated, tag = "1")]
    pub revisions: ::prost::alloc::vec::Vec<Revision>,
    /// A token indicating there are more items than page_size. Use it in the next
    /// ListRevisions request to continue.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for deleting a retired Revision.
/// Revision lifecycle is usually managed by making changes to the parent
/// Service. Only retired revisions can be deleted with this API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRevisionRequest {
    /// Required. The name of the Revision to delete.
    /// Format:
    /// projects/{project}/locations/{location}/services/{service}/revisions/{revision}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates that the request should be validated without actually
    /// deleting any resources.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// A system-generated fingerprint for this version of the
    /// resource. This may be used to detect modification conflict during updates.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
/// A Revision is an immutable snapshot of code and configuration.  A Revision
/// references a container image. Revisions are only created by updates to its
/// parent Service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Revision {
    /// Output only. The unique name of this Revision.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server assigned unique identifier for the Revision. The value
    /// is a UUID4 string and guaranteed to remain unchanged until the resource is
    /// deleted.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. A number that monotonically increases every time the user
    /// modifies the desired state.
    #[prost(int64, tag = "3")]
    pub generation: i64,
    /// Output only. Unstructured key value map that can be used to organize and
    /// categorize objects. User-provided labels are shared with Google's billing
    /// system, so they can be used to filter, or break down billing charges by
    /// team, component, environment, state, etc. For more information, visit
    /// <https://cloud.google.com/resource-manager/docs/creating-managing-labels> or
    /// <https://cloud.google.com/run/docs/configuring/labels.>
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Unstructured key value map that may
    /// be set by external tools to store and arbitrary metadata.
    /// They are not queryable and should be preserved
    /// when modifying objects.
    #[prost(btree_map = "string, string", tag = "5")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. For a deleted resource, the deletion time. It is only
    /// populated as a response to a Delete request.
    #[prost(message, optional, tag = "8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. For a deleted resource, the time after which it will be
    /// permamently deleted. It is only populated as a response to a Delete
    /// request.
    #[prost(message, optional, tag = "9")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The least stable launch stage needed to create this resource, as defined by
    /// [Google Cloud Platform Launch
    /// Stages](<https://cloud.google.com/terms/launch-stages>). Cloud Run supports
    /// `ALPHA`, `BETA`, and `GA`.
    /// <p>Note that this value might not be what was used
    /// as input. For example, if ALPHA was provided as input in the parent
    /// resource, but only BETA and GA-level features are were, this field will be
    /// BETA.
    #[prost(enumeration = "super::super::super::api::LaunchStage", tag = "10")]
    pub launch_stage: i32,
    /// Output only. The name of the parent service.
    #[prost(string, tag = "11")]
    pub service: ::prost::alloc::string::String,
    /// Scaling settings for this revision.
    #[prost(message, optional, tag = "12")]
    pub scaling: ::core::option::Option<RevisionScaling>,
    /// VPC Access configuration for this Revision. For more information, visit
    /// <https://cloud.google.com/run/docs/configuring/connecting-vpc.>
    #[prost(message, optional, tag = "13")]
    pub vpc_access: ::core::option::Option<VpcAccess>,
    /// Sets the maximum number of requests that each serving instance can receive.
    #[prost(int32, tag = "34")]
    pub max_instance_request_concurrency: i32,
    /// Max allowed time for an instance to respond to a request.
    #[prost(message, optional, tag = "15")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Email address of the IAM service account associated with the revision of
    /// the service. The service account represents the identity of the running
    /// revision, and determines what permissions the revision has.
    #[prost(string, tag = "16")]
    pub service_account: ::prost::alloc::string::String,
    /// Holds the single container that defines the unit of execution for this
    /// Revision.
    #[prost(message, repeated, tag = "17")]
    pub containers: ::prost::alloc::vec::Vec<Container>,
    /// A list of Volumes to make available to containers.
    #[prost(message, repeated, tag = "18")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// The execution environment being used to host this Revision.
    #[prost(enumeration = "ExecutionEnvironment", tag = "20")]
    pub execution_environment: i32,
    /// A reference to a customer managed encryption key (CMEK) to use to encrypt
    /// this container image. For more information, go to
    /// <https://cloud.google.com/run/docs/securing/using-cmek>
    #[prost(string, tag = "21")]
    pub encryption_key: ::prost::alloc::string::String,
    /// The action to take if the encryption key is revoked.
    #[prost(enumeration = "EncryptionKeyRevocationAction", tag = "23")]
    pub encryption_key_revocation_action: i32,
    /// If encryption_key_revocation_action is SHUTDOWN, the duration before
    /// shutting down all instances. The minimum increment is 1 hour.
    #[prost(message, optional, tag = "24")]
    pub encryption_key_shutdown_duration: ::core::option::Option<
        ::prost_types::Duration,
    >,
    /// Output only. Indicates whether the resource's reconciliation is still in
    /// progress. See comments in `Service.reconciling` for additional information
    /// on reconciliation process in Cloud Run.
    #[prost(bool, tag = "30")]
    pub reconciling: bool,
    /// Output only. The Condition of this Revision, containing its readiness
    /// status, and detailed error information in case it did not reach a serving
    /// state.
    #[prost(message, repeated, tag = "31")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    /// Output only. The generation of this Revision currently serving traffic. See
    /// comments in `reconciling` for additional information on reconciliation
    /// process in Cloud Run.
    #[prost(int64, tag = "32")]
    pub observed_generation: i64,
    /// Output only. The Google Console URI to obtain logs for the Revision.
    #[prost(string, tag = "33")]
    pub log_uri: ::prost::alloc::string::String,
    /// Output only. Reserved for future use.
    #[prost(bool, tag = "37")]
    pub satisfies_pzs: bool,
    /// Enable session affinity.
    #[prost(bool, tag = "38")]
    pub session_affinity: bool,
    /// Output only. The current effective scaling settings for the revision.
    #[prost(message, optional, tag = "39")]
    pub scaling_status: ::core::option::Option<RevisionScalingStatus>,
    /// Output only. A system-generated fingerprint for this version of the
    /// resource. May be used to detect modification conflict during updates.
    #[prost(string, tag = "99")]
    pub etag: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod revisions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Cloud Run Revision Control Plane API.
    #[derive(Debug, Clone)]
    pub struct RevisionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RevisionsClient<T>
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
        ) -> RevisionsClient<InterceptedService<T, F>>
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
            RevisionsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets information about a Revision.
        pub async fn get_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRevisionRequest>,
        ) -> std::result::Result<tonic::Response<super::Revision>, tonic::Status> {
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
                "/google.cloud.run.v2.Revisions/GetRevision",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Revisions", "GetRevision"));
            self.inner.unary(req, path, codec).await
        }
        /// Lists Revisions from a given Service, or from a given location.
        pub async fn list_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRevisionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRevisionsResponse>,
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
                "/google.cloud.run.v2.Revisions/ListRevisions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.run.v2.Revisions", "ListRevisions"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Revision.
        pub async fn delete_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRevisionRequest>,
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
                "/google.cloud.run.v2.Revisions/DeleteRevision",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.run.v2.Revisions", "DeleteRevision"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request message for creating a Service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceRequest {
    /// Required. The location and project in which this service should be created.
    /// Format: projects/{project}/locations/{location}, where {project} can be
    /// project id or number. Only lowercase characters, digits, and hyphens.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Service instance to create.
    #[prost(message, optional, tag = "2")]
    pub service: ::core::option::Option<Service>,
    /// Required. The unique identifier for the Service. It must begin with letter,
    /// and cannot end with hyphen; must contain fewer than 50 characters.
    /// The name of the service becomes {parent}/services/{service_id}.
    #[prost(string, tag = "3")]
    pub service_id: ::prost::alloc::string::String,
    /// Indicates that the request should be validated and default values
    /// populated, without persisting the request or creating any resources.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for updating a service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceRequest {
    /// Required. The Service to be updated.
    #[prost(message, optional, tag = "1")]
    pub service: ::core::option::Option<Service>,
    /// Indicates that the request should be validated and default values
    /// populated, without persisting the request or updating any resources.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// If set to true, and if the Service does not exist, it will create a new
    /// one. The caller must have 'run.services.create' permissions if this is set
    /// to true and the Service does not exist.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// Request message for retrieving a list of Services.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Required. The location and project to list resources on.
    /// Location must be a valid Google Cloud region, and cannot be the "-"
    /// wildcard. Format: projects/{project}/locations/{location}, where {project}
    /// can be project id or number.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of Services to return in this call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token received from a previous call to ListServices.
    /// All other parameters must match.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// If true, returns deleted (but unexpired) resources along with active ones.
    #[prost(bool, tag = "4")]
    pub show_deleted: bool,
}
/// Response message containing a list of Services.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The resulting list of Services.
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// A token indicating there are more items than page_size. Use it in the next
    /// ListServices request to continue.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for obtaining a Service by its full name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Required. The full name of the Service.
    /// Format: projects/{project}/locations/{location}/services/{service}, where
    /// {project} can be project id or number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message to delete a Service by its full name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Required. The full name of the Service.
    /// Format: projects/{project}/locations/{location}/services/{service}, where
    /// {project} can be project id or number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates that the request should be validated without actually
    /// deleting any resources.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// A system-generated fingerprint for this version of the
    /// resource. May be used to detect modification conflict during updates.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
/// Service acts as a top-level container that manages a set of
/// configurations and revision templates which implement a network service.
/// Service exists to provide a singular abstraction which can be access
/// controlled, reasoned about, and which encapsulates software lifecycle
/// decisions such as rollout policy and team resource ownership.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// The fully qualified name of this Service. In CreateServiceRequest, this
    /// field is ignored, and instead composed from CreateServiceRequest.parent and
    /// CreateServiceRequest.service_id.
    ///
    /// Format:
    /// projects/{project}/locations/{location}/services/{service_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User-provided description of the Service. This field currently has a
    /// 512-character limit.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Server assigned unique identifier for the trigger. The value
    /// is a UUID4 string and guaranteed to remain unchanged until the resource is
    /// deleted.
    #[prost(string, tag = "3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. A number that monotonically increases every time the user
    /// modifies the desired state.
    /// Please note that unlike v1, this is an int64 value. As with most Google
    /// APIs, its JSON representation will be a `string` instead of an `integer`.
    #[prost(int64, tag = "4")]
    pub generation: i64,
    /// Optional. Unstructured key value map that can be used to organize and
    /// categorize objects. User-provided labels are shared with Google's billing
    /// system, so they can be used to filter, or break down billing charges by
    /// team, component, environment, state, etc. For more information, visit
    /// <https://cloud.google.com/resource-manager/docs/creating-managing-labels> or
    /// <https://cloud.google.com/run/docs/configuring/labels.>
    ///
    /// <p>Cloud Run API v2 does not support labels with  `run.googleapis.com`,
    /// `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev`
    /// namespaces, and they will be rejected. All system labels in v1 now have a
    /// corresponding field in v2 Service.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. Unstructured key value map that may be set by external tools to
    /// store and arbitrary metadata. They are not queryable and should be
    /// preserved when modifying objects.
    ///
    /// <p>Cloud Run API v2 does not support annotations with `run.googleapis.com`,
    /// `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev`
    /// namespaces, and they will be rejected in new resources. All system
    /// annotations in v1 now have a corresponding field in v2 Service.
    ///
    /// <p>This field follows Kubernetes
    /// annotations' namespacing, limits, and rules.
    #[prost(btree_map = "string, string", tag = "6")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The deletion time.
    #[prost(message, optional, tag = "9")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. For a deleted resource, the time after which it will be
    /// permamently deleted.
    #[prost(message, optional, tag = "10")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Email address of the authenticated creator.
    #[prost(string, tag = "11")]
    pub creator: ::prost::alloc::string::String,
    /// Output only. Email address of the last authenticated modifier.
    #[prost(string, tag = "12")]
    pub last_modifier: ::prost::alloc::string::String,
    /// Arbitrary identifier for the API client.
    #[prost(string, tag = "13")]
    pub client: ::prost::alloc::string::String,
    /// Arbitrary version identifier for the API client.
    #[prost(string, tag = "14")]
    pub client_version: ::prost::alloc::string::String,
    /// Provides the ingress settings for this Service. On output, returns the
    /// currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no
    /// revision is active.
    #[prost(enumeration = "IngressTraffic", tag = "15")]
    pub ingress: i32,
    /// The launch stage as defined by [Google Cloud Platform
    /// Launch Stages](<https://cloud.google.com/terms/launch-stages>).
    /// Cloud Run supports `ALPHA`, `BETA`, and `GA`. If no value is specified, GA
    /// is assumed.
    /// Set the launch stage to a preview stage on input to allow use of preview
    /// features in that stage. On read (or output), describes whether the resource
    /// uses preview features.
    /// <p>
    /// For example, if ALPHA is provided as input, but only BETA and GA-level
    /// features are used, this field will be BETA on output.
    #[prost(enumeration = "super::super::super::api::LaunchStage", tag = "16")]
    pub launch_stage: i32,
    /// Settings for the Binary Authorization feature.
    #[prost(message, optional, tag = "17")]
    pub binary_authorization: ::core::option::Option<BinaryAuthorization>,
    /// Required. The template used to create revisions for this Service.
    #[prost(message, optional, tag = "18")]
    pub template: ::core::option::Option<RevisionTemplate>,
    /// Specifies how to distribute traffic over a collection of Revisions
    /// belonging to the Service. If traffic is empty or not provided, defaults to
    /// 100% traffic to the latest `Ready` Revision.
    #[prost(message, repeated, tag = "19")]
    pub traffic: ::prost::alloc::vec::Vec<TrafficTarget>,
    /// Optional. Specifies service-level scaling settings
    #[prost(message, optional, tag = "20")]
    pub scaling: ::core::option::Option<ServiceScaling>,
    /// Optional. Disables public resolution of the default URI of this service.
    #[prost(bool, tag = "22")]
    pub default_uri_disabled: bool,
    /// Output only. The generation of this Service currently serving traffic. See
    /// comments in `reconciling` for additional information on reconciliation
    /// process in Cloud Run. Please note that unlike v1, this is an int64 value.
    /// As with most Google APIs, its JSON representation will be a `string`
    /// instead of an `integer`.
    #[prost(int64, tag = "30")]
    pub observed_generation: i64,
    /// Output only. The Condition of this Service, containing its readiness
    /// status, and detailed error information in case it did not reach a serving
    /// state. See comments in `reconciling` for additional information on
    /// reconciliation process in Cloud Run.
    #[prost(message, optional, tag = "31")]
    pub terminal_condition: ::core::option::Option<Condition>,
    /// Output only. The Conditions of all other associated sub-resources. They
    /// contain additional diagnostics information in case the Service does not
    /// reach its Serving state. See comments in `reconciling` for additional
    /// information on reconciliation process in Cloud Run.
    #[prost(message, repeated, tag = "32")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    /// Output only. Name of the latest revision that is serving traffic. See
    /// comments in `reconciling` for additional information on reconciliation
    /// process in Cloud Run.
    #[prost(string, tag = "33")]
    pub latest_ready_revision: ::prost::alloc::string::String,
    /// Output only. Name of the last created revision. See comments in
    /// `reconciling` for additional information on reconciliation process in Cloud
    /// Run.
    #[prost(string, tag = "34")]
    pub latest_created_revision: ::prost::alloc::string::String,
    /// Output only. Detailed status information for corresponding traffic targets.
    /// See comments in `reconciling` for additional information on reconciliation
    /// process in Cloud Run.
    #[prost(message, repeated, tag = "35")]
    pub traffic_statuses: ::prost::alloc::vec::Vec<TrafficTargetStatus>,
    /// Output only. The main URI in which this Service is serving traffic.
    #[prost(string, tag = "36")]
    pub uri: ::prost::alloc::string::String,
    /// One or more custom audiences that you want this service to support. Specify
    /// each custom audience as the full URL in a string. The custom audiences are
    /// encoded in the token and used to authenticate requests. For more
    /// information, see
    /// <https://cloud.google.com/run/docs/configuring/custom-audiences.>
    #[prost(string, repeated, tag = "37")]
    pub custom_audiences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Reserved for future use.
    #[prost(bool, tag = "38")]
    pub satisfies_pzs: bool,
    /// Output only. Returns true if the Service is currently being acted upon by
    /// the system to bring it into the desired state.
    ///
    /// When a new Service is created, or an existing one is updated, Cloud Run
    /// will asynchronously perform all necessary steps to bring the Service to the
    /// desired serving state. This process is called reconciliation.
    /// While reconciliation is in process, `observed_generation`,
    /// `latest_ready_revison`, `traffic_statuses`, and `uri` will have transient
    /// values that might mismatch the intended state: Once reconciliation is over
    /// (and this field is false), there are two possible outcomes: reconciliation
    /// succeeded and the serving state matches the Service, or there was an error,
    /// and reconciliation failed. This state can be found in
    /// `terminal_condition.state`.
    ///
    /// If reconciliation succeeded, the following fields will match: `traffic` and
    /// `traffic_statuses`, `observed_generation` and `generation`,
    /// `latest_ready_revision` and `latest_created_revision`.
    ///
    /// If reconciliation failed, `traffic_statuses`, `observed_generation`, and
    /// `latest_ready_revision` will have the state of the last serving revision,
    /// or empty for newly created Services. Additional information on the failure
    /// can be found in `terminal_condition` and `conditions`.
    #[prost(bool, tag = "98")]
    pub reconciling: bool,
    /// Output only. A system-generated fingerprint for this version of the
    /// resource. May be used to detect modification conflict during updates.
    #[prost(string, tag = "99")]
    pub etag: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod services_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Cloud Run Service Control Plane API
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
        /// Creates a new Service in a given project and location.
        pub async fn create_service(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceRequest>,
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
                "/google.cloud.run.v2.Services/CreateService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.run.v2.Services", "CreateService"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets information about a Service.
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
                "/google.cloud.run.v2.Services/GetService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Services", "GetService"));
            self.inner.unary(req, path, codec).await
        }
        /// Lists Services.
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
                "/google.cloud.run.v2.Services/ListServices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Services", "ListServices"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates a Service.
        pub async fn update_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServiceRequest>,
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
                "/google.cloud.run.v2.Services/UpdateService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.run.v2.Services", "UpdateService"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Service.
        /// This will cause the Service to stop serving traffic and will delete all
        /// revisions.
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceRequest>,
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
                "/google.cloud.run.v2.Services/DeleteService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.run.v2.Services", "DeleteService"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the IAM Access Control policy currently in effect for the given
        /// Cloud Run Service. This result does not include any inherited policies.
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
                "/google.cloud.run.v2.Services/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Services", "GetIamPolicy"));
            self.inner.unary(req, path, codec).await
        }
        /// Sets the IAM Access control policy for the specified Service. Overwrites
        /// any existing policy.
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
                "/google.cloud.run.v2.Services/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.run.v2.Services", "SetIamPolicy"));
            self.inner.unary(req, path, codec).await
        }
        /// Returns permissions that a caller has on the specified Project.
        ///
        /// There are no permissions required for making this API call.
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
                "/google.cloud.run.v2.Services/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.run.v2.Services", "TestIamPermissions"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
