/// Operation System image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsImage {
    /// Output only. OS Image's unique name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// OS Image code.
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    /// OS Image description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Instance types this image is applicable to.
    /// [Available
    /// types](<https://cloud.google.com/bare-metal/docs/bms-planning#server_configurations>)
    #[prost(string, repeated, tag = "4")]
    pub applicable_instance_types: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Network templates that can be used with this OS Image.
    #[prost(string, repeated, tag = "5")]
    pub supported_network_templates: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// Request for getting all available OS images.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOsImagesRequest {
    /// Required. Parent value for ListProvisioningQuotasRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server might return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    /// Notice that page_size field is not supported and won't be respected in
    /// the API request for now, will be updated when pagination is supported.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Request for getting all available OS images.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOsImagesResponse {
    /// The OS images available.
    #[prost(message, repeated, tag = "1")]
    pub os_images: ::prost::alloc::vec::Vec<OsImage>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Performance tier of the Volume.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VolumePerformanceTier {
    /// Value is not specified.
    Unspecified = 0,
    /// Regular volumes, shared aggregates.
    Shared = 1,
    /// Assigned aggregates.
    Assigned = 2,
    /// High throughput aggregates.
    Ht = 3,
}
impl VolumePerformanceTier {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VolumePerformanceTier::Unspecified => "VOLUME_PERFORMANCE_TIER_UNSPECIFIED",
            VolumePerformanceTier::Shared => "VOLUME_PERFORMANCE_TIER_SHARED",
            VolumePerformanceTier::Assigned => "VOLUME_PERFORMANCE_TIER_ASSIGNED",
            VolumePerformanceTier::Ht => "VOLUME_PERFORMANCE_TIER_HT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VOLUME_PERFORMANCE_TIER_UNSPECIFIED" => Some(Self::Unspecified),
            "VOLUME_PERFORMANCE_TIER_SHARED" => Some(Self::Shared),
            "VOLUME_PERFORMANCE_TIER_ASSIGNED" => Some(Self::Assigned),
            "VOLUME_PERFORMANCE_TIER_HT" => Some(Self::Ht),
            _ => None,
        }
    }
}
/// The possible values for a workload profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WorkloadProfile {
    /// The workload profile is in an unknown state.
    Unspecified = 0,
    /// The workload profile is generic.
    Generic = 1,
    /// The workload profile is hana.
    Hana = 2,
}
impl WorkloadProfile {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WorkloadProfile::Unspecified => "WORKLOAD_PROFILE_UNSPECIFIED",
            WorkloadProfile::Generic => "WORKLOAD_PROFILE_GENERIC",
            WorkloadProfile::Hana => "WORKLOAD_PROFILE_HANA",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WORKLOAD_PROFILE_UNSPECIFIED" => Some(Self::Unspecified),
            "WORKLOAD_PROFILE_GENERIC" => Some(Self::Generic),
            "WORKLOAD_PROFILE_HANA" => Some(Self::Hana),
            _ => None,
        }
    }
}
/// A Network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    /// Output only. The resource name of this `Network`.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// Format:
    /// `projects/{project}/locations/{location}/networks/{network}`
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// An identifier for the `Network`, generated by the backend.
    #[prost(string, tag = "10")]
    pub id: ::prost::alloc::string::String,
    /// The type of this network.
    #[prost(enumeration = "network::Type", tag = "2")]
    pub r#type: i32,
    /// IP address configured.
    #[prost(string, tag = "3")]
    pub ip_address: ::prost::alloc::string::String,
    /// List of physical interfaces.
    #[prost(string, repeated, tag = "4")]
    pub mac_address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Network state.
    #[prost(enumeration = "network::State", tag = "6")]
    pub state: i32,
    /// The vlan id of the Network.
    #[prost(string, tag = "7")]
    pub vlan_id: ::prost::alloc::string::String,
    /// The cidr of the Network.
    #[prost(string, tag = "8")]
    pub cidr: ::prost::alloc::string::String,
    /// The vrf for the Network.
    #[prost(message, optional, tag = "9")]
    pub vrf: ::core::option::Option<Vrf>,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "11")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// IP range for reserved for services (e.g. NFS).
    #[prost(string, tag = "12")]
    pub services_cidr: ::prost::alloc::string::String,
    /// List of IP address reservations in this network.
    /// When updating this field, an error will be generated if a reservation
    /// conflicts with an IP address already allocated to a physical server.
    #[prost(message, repeated, tag = "13")]
    pub reservations: ::prost::alloc::vec::Vec<NetworkAddressReservation>,
    /// Output only. Pod name.
    #[prost(string, tag = "14")]
    pub pod: ::prost::alloc::string::String,
    /// Input only. List of mount points to attach the network to.
    #[prost(message, repeated, tag = "15")]
    pub mount_points: ::prost::alloc::vec::Vec<NetworkMountPoint>,
    /// Whether network uses standard frames or jumbo ones.
    #[prost(bool, tag = "16")]
    pub jumbo_frames_enabled: bool,
    /// Output only. Gateway ip address.
    #[prost(string, tag = "17")]
    pub gateway_ip: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Network`.
pub mod network {
    /// Network type.
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
    pub enum Type {
        /// Unspecified value.
        Unspecified = 0,
        /// Client network, a network peered to a Google Cloud VPC.
        Client = 1,
        /// Private network, a network local to the Bare Metal Solution environment.
        Private = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Client => "CLIENT",
                Type::Private => "PRIVATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLIENT" => Some(Self::Client),
                "PRIVATE" => Some(Self::Private),
                _ => None,
            }
        }
    }
    /// The possible states for this Network.
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
        /// The Network is in an unknown state.
        Unspecified = 0,
        /// The Network is provisioning.
        Provisioning = 1,
        /// The Network has been provisioned.
        Provisioned = 2,
        /// The Network is being deprovisioned.
        Deprovisioning = 3,
        /// The Network is being updated.
        Updating = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Provisioning => "PROVISIONING",
                State::Provisioned => "PROVISIONED",
                State::Deprovisioning => "DEPROVISIONING",
                State::Updating => "UPDATING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PROVISIONING" => Some(Self::Provisioning),
                "PROVISIONED" => Some(Self::Provisioned),
                "DEPROVISIONING" => Some(Self::Deprovisioning),
                "UPDATING" => Some(Self::Updating),
                _ => None,
            }
        }
    }
}
/// A reservation of one or more addresses in a network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkAddressReservation {
    /// The first address of this reservation block.
    /// Must be specified as a single IPv4 address, e.g. 10.1.2.2.
    #[prost(string, tag = "1")]
    pub start_address: ::prost::alloc::string::String,
    /// The last address of this reservation block, inclusive. I.e., for cases when
    /// reservations are only single addresses, end_address and start_address will
    /// be the same.
    /// Must be specified as a single IPv4 address, e.g. 10.1.2.2.
    #[prost(string, tag = "2")]
    pub end_address: ::prost::alloc::string::String,
    /// A note about this reservation, intended for human consumption.
    #[prost(string, tag = "3")]
    pub note: ::prost::alloc::string::String,
}
/// A network VRF.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vrf {
    /// The name of the VRF.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The possible state of VRF.
    #[prost(enumeration = "vrf::State", tag = "5")]
    pub state: i32,
    /// The QOS policy applied to this VRF.
    /// The value is only meaningful when all the vlan attachments have the same
    /// QoS. This field should not be used for new integrations, use vlan
    /// attachment level qos instead. The field is left for backward-compatibility.
    #[prost(message, optional, tag = "6")]
    pub qos_policy: ::core::option::Option<vrf::QosPolicy>,
    /// The list of VLAN attachments for the VRF.
    #[prost(message, repeated, tag = "7")]
    pub vlan_attachments: ::prost::alloc::vec::Vec<vrf::VlanAttachment>,
}
/// Nested message and enum types in `VRF`.
pub mod vrf {
    /// QOS policy parameters.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QosPolicy {
        /// The bandwidth permitted by the QOS policy, in gbps.
        #[prost(double, tag = "1")]
        pub bandwidth_gbps: f64,
    }
    /// VLAN attachment details.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VlanAttachment {
        /// The peer vlan ID of the attachment.
        #[prost(int64, tag = "1")]
        pub peer_vlan_id: i64,
        /// The peer IP of the attachment.
        #[prost(string, tag = "2")]
        pub peer_ip: ::prost::alloc::string::String,
        /// The router IP of the attachment.
        #[prost(string, tag = "3")]
        pub router_ip: ::prost::alloc::string::String,
        /// Input only. Pairing key.
        #[prost(string, tag = "4")]
        pub pairing_key: ::prost::alloc::string::String,
        /// The QOS policy applied to this VLAN attachment.
        /// This value should be preferred to using qos at vrf level.
        #[prost(message, optional, tag = "5")]
        pub qos_policy: ::core::option::Option<QosPolicy>,
        /// Immutable. The identifier of the attachment within vrf.
        #[prost(string, tag = "6")]
        pub id: ::prost::alloc::string::String,
        /// Optional. The name of the vlan attachment within vrf. This is of the form
        /// projects/{project_number}/regions/{region}/interconnectAttachments/{interconnect_attachment}
        #[prost(string, tag = "7")]
        pub interconnect_attachment: ::prost::alloc::string::String,
    }
    /// The possible states for this VRF.
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
        /// The unspecified state.
        Unspecified = 0,
        /// The vrf is provisioning.
        Provisioning = 1,
        /// The vrf is provisioned.
        Provisioned = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Provisioning => "PROVISIONING",
                State::Provisioned => "PROVISIONED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PROVISIONING" => Some(Self::Provisioning),
                "PROVISIONED" => Some(Self::Provisioned),
                _ => None,
            }
        }
    }
}
/// Each logical interface represents a logical abstraction of the underlying
/// physical interface (for eg. bond, nic) of the instance. Each logical
/// interface can effectively map to multiple network-IP pairs and still be
/// mapped to one underlying physical interface.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicalInterface {
    /// List of logical network interfaces within a logical interface.
    #[prost(message, repeated, tag = "1")]
    pub logical_network_interfaces: ::prost::alloc::vec::Vec<
        logical_interface::LogicalNetworkInterface,
    >,
    /// Interface name. This is of syntax <bond><bond_mode> or <nic> and
    /// forms part of the network template name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The index of the logical interface mapping to the index of the hardware
    /// bond or nic on the chosen network template. This field is deprecated.
    #[deprecated]
    #[prost(int32, tag = "3")]
    pub interface_index: i32,
}
/// Nested message and enum types in `LogicalInterface`.
pub mod logical_interface {
    /// Each logical network interface is effectively a network and IP pair.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LogicalNetworkInterface {
        /// Name of the network
        #[prost(string, tag = "1")]
        pub network: ::prost::alloc::string::String,
        /// IP address in the network
        #[prost(string, tag = "2")]
        pub ip_address: ::prost::alloc::string::String,
        /// Whether this interface is the default gateway for the instance. Only
        /// one interface can be the default gateway for the instance.
        #[prost(bool, tag = "3")]
        pub default_gateway: bool,
        /// Type of network.
        #[prost(enumeration = "super::network::Type", tag = "4")]
        pub network_type: i32,
        /// An identifier for the `Network`, generated by the backend.
        #[prost(string, tag = "5")]
        pub id: ::prost::alloc::string::String,
    }
}
/// Message for requesting network information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNetworkRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting a list of networks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNetworksRequest {
    /// Required. Parent value for ListNetworksRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server might return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message containing the list of networks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNetworksResponse {
    /// The list of networks.
    #[prost(message, repeated, tag = "1")]
    pub networks: ::prost::alloc::vec::Vec<Network>,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message requesting to updating a network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNetworkRequest {
    /// Required. The network to update.
    ///
    /// The `name` field is used to identify the instance to update.
    /// Format: projects/{project}/locations/{location}/networks/{network}
    #[prost(message, optional, tag = "1")]
    pub network: ::core::option::Option<Network>,
    /// The list of fields to update.
    /// The only currently supported fields are:
    ///    `labels`, `reservations`, `vrf.vlan_attachments`
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Network with all used IP addresses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkUsage {
    /// Network.
    #[prost(message, optional, tag = "1")]
    pub network: ::core::option::Option<Network>,
    /// All used IP addresses in this network.
    #[prost(string, repeated, tag = "2")]
    pub used_ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request to get networks with IPs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNetworkUsageRequest {
    /// Required. Parent value (project and location).
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
}
/// Response with Networks with IPs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNetworkUsageResponse {
    /// Networks with IPs.
    #[prost(message, repeated, tag = "1")]
    pub networks: ::prost::alloc::vec::Vec<NetworkUsage>,
}
/// Mount point for a network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkMountPoint {
    /// Instance to attach network to.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Logical interface to detach from.
    #[prost(string, tag = "2")]
    pub logical_interface: ::prost::alloc::string::String,
    /// Network should be a default gateway.
    #[prost(bool, tag = "3")]
    pub default_gateway: bool,
    /// Ip address of the server.
    #[prost(string, tag = "4")]
    pub ip_address: ::prost::alloc::string::String,
}
/// Message requesting rename of a server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameNetworkRequest {
    /// Required. The `name` field is used to identify the network.
    /// Format: projects/{project}/locations/{location}/networks/{network}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new `id` of the network.
    #[prost(string, tag = "2")]
    pub new_network_id: ::prost::alloc::string::String,
}
/// A provisioning configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisioningConfig {
    /// Output only. The system-generated name of the provisioning config. This
    /// follows the UUID format.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Instances to be created.
    #[prost(message, repeated, tag = "2")]
    pub instances: ::prost::alloc::vec::Vec<InstanceConfig>,
    /// Networks to be created.
    #[prost(message, repeated, tag = "3")]
    pub networks: ::prost::alloc::vec::Vec<NetworkConfig>,
    /// Volumes to be created.
    #[prost(message, repeated, tag = "4")]
    pub volumes: ::prost::alloc::vec::Vec<VolumeConfig>,
    /// A generated ticket id to track provisioning request.
    #[prost(string, tag = "5")]
    pub ticket_id: ::prost::alloc::string::String,
    /// A service account to enable customers to access instance credentials upon
    /// handover.
    #[prost(string, tag = "6")]
    pub handover_service_account: ::prost::alloc::string::String,
    /// Email provided to send a confirmation with provisioning config to.
    /// Deprecated in favour of email field in request messages.
    #[deprecated]
    #[prost(string, tag = "7")]
    pub email: ::prost::alloc::string::String,
    /// Output only. State of ProvisioningConfig.
    #[prost(enumeration = "provisioning_config::State", tag = "8")]
    pub state: i32,
    /// Optional. Location name of this ProvisioningConfig.
    /// It is optional only for Intake UI transition period.
    #[prost(string, tag = "9")]
    pub location: ::prost::alloc::string::String,
    /// Output only. Last update timestamp.
    #[prost(message, optional, tag = "10")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. URI to Cloud Console UI view of this provisioning config.
    #[prost(string, tag = "11")]
    pub cloud_console_uri: ::prost::alloc::string::String,
    /// If true, VPC SC is enabled for the cluster.
    #[prost(bool, tag = "12")]
    pub vpc_sc_enabled: bool,
    /// Optional status messages associated with the FAILED state.
    #[prost(string, tag = "13")]
    pub status_message: ::prost::alloc::string::String,
    /// Optional. The user-defined identifier of the provisioning config.
    #[prost(string, tag = "14")]
    pub custom_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ProvisioningConfig`.
pub mod provisioning_config {
    /// The possible states for this ProvisioningConfig.
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
        /// State wasn't specified.
        Unspecified = 0,
        /// ProvisioningConfig is a draft and can be freely modified.
        Draft = 1,
        /// ProvisioningConfig was already submitted and cannot be modified.
        Submitted = 2,
        /// ProvisioningConfig was in the provisioning state.  Initially this state
        /// comes from the work order table in big query when SNOW is used.  Later
        /// this field can be set by the work order API.
        Provisioning = 3,
        /// ProvisioningConfig was provisioned, meaning the resources exist.
        Provisioned = 4,
        /// ProvisioningConfig was validated.  A validation tool will be run to
        /// set this state.
        Validated = 5,
        /// ProvisioningConfig was canceled.
        Cancelled = 6,
        /// The request is submitted for provisioning, with error return.
        Failed = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Draft => "DRAFT",
                State::Submitted => "SUBMITTED",
                State::Provisioning => "PROVISIONING",
                State::Provisioned => "PROVISIONED",
                State::Validated => "VALIDATED",
                State::Cancelled => "CANCELLED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "DRAFT" => Some(Self::Draft),
                "SUBMITTED" => Some(Self::Submitted),
                "PROVISIONING" => Some(Self::Provisioning),
                "PROVISIONED" => Some(Self::Provisioned),
                "VALIDATED" => Some(Self::Validated),
                "CANCELLED" => Some(Self::Cancelled),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// Request for SubmitProvisioningConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitProvisioningConfigRequest {
    /// Required. The parent project and location containing the
    /// ProvisioningConfig.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ProvisioningConfig to create.
    #[prost(message, optional, tag = "2")]
    pub provisioning_config: ::core::option::Option<ProvisioningConfig>,
    /// Optional. Email provided to send a confirmation with provisioning config
    /// to.
    #[prost(string, tag = "3")]
    pub email: ::prost::alloc::string::String,
}
/// Response for SubmitProvisioningConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitProvisioningConfigResponse {
    /// The submitted provisioning config.
    #[prost(message, optional, tag = "1")]
    pub provisioning_config: ::core::option::Option<ProvisioningConfig>,
}
/// A provisioning quota for a given project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisioningQuota {
    /// Output only. The name of the provisioning quota.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The asset type of this provisioning quota.
    #[prost(enumeration = "provisioning_quota::AssetType", tag = "2")]
    pub asset_type: i32,
    /// The gcp service of the provisioning quota.
    #[prost(string, tag = "3")]
    pub gcp_service: ::prost::alloc::string::String,
    /// The specific location of the provisioining quota.
    #[prost(string, tag = "4")]
    pub location: ::prost::alloc::string::String,
    /// The available count of the provisioning quota.
    #[prost(int32, tag = "5")]
    pub available_count: i32,
    /// The quota of one asset type.
    #[prost(oneof = "provisioning_quota::Quota", tags = "6")]
    pub quota: ::core::option::Option<provisioning_quota::Quota>,
    /// Available quantity based on asset type.
    #[prost(oneof = "provisioning_quota::Availability", tags = "7, 8, 9")]
    pub availability: ::core::option::Option<provisioning_quota::Availability>,
}
/// Nested message and enum types in `ProvisioningQuota`.
pub mod provisioning_quota {
    /// The available asset types for intake.
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
    pub enum AssetType {
        /// The unspecified type.
        Unspecified = 0,
        /// The server asset type.
        Server = 1,
        /// The storage asset type.
        Storage = 2,
        /// The network asset type.
        Network = 3,
    }
    impl AssetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetType::Unspecified => "ASSET_TYPE_UNSPECIFIED",
                AssetType::Server => "ASSET_TYPE_SERVER",
                AssetType::Storage => "ASSET_TYPE_STORAGE",
                AssetType::Network => "ASSET_TYPE_NETWORK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ASSET_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ASSET_TYPE_SERVER" => Some(Self::Server),
                "ASSET_TYPE_STORAGE" => Some(Self::Storage),
                "ASSET_TYPE_NETWORK" => Some(Self::Network),
                _ => None,
            }
        }
    }
    /// The quota of one asset type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Quota {
        /// Instance quota.
        #[prost(message, tag = "6")]
        InstanceQuota(super::InstanceQuota),
    }
    /// Available quantity based on asset type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Availability {
        /// Server count.
        #[prost(int64, tag = "7")]
        ServerCount(i64),
        /// Network bandwidth, Gbps
        #[prost(int64, tag = "8")]
        NetworkBandwidth(i64),
        /// Storage size (GB).
        #[prost(int64, tag = "9")]
        StorageGib(i64),
    }
}
/// Message for requesting the list of provisioning quotas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProvisioningQuotasRequest {
    /// Required. Parent value for ListProvisioningQuotasRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server might return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    /// Notice that page_size field is not supported and won't be respected in
    /// the API request for now, will be updated when pagination is supported.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the list of provisioning quotas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProvisioningQuotasResponse {
    /// The provisioning quotas registered in this project.
    #[prost(message, repeated, tag = "1")]
    pub provisioning_quotas: ::prost::alloc::vec::Vec<ProvisioningQuota>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Configuration parameters for a new instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceConfig {
    /// Output only. The name of the instance config.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A transient unique identifier to idenfity an instance within an
    /// ProvisioningConfig request.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Instance type.
    /// [Available
    /// types](<https://cloud.google.com/bare-metal/docs/bms-planning#server_configurations>)
    #[prost(string, tag = "3")]
    pub instance_type: ::prost::alloc::string::String,
    /// Whether the instance should be provisioned with Hyperthreading enabled.
    #[prost(bool, tag = "4")]
    pub hyperthreading: bool,
    /// OS image to initialize the instance.
    /// [Available
    /// images](<https://cloud.google.com/bare-metal/docs/bms-planning#server_configurations>)
    #[prost(string, tag = "5")]
    pub os_image: ::prost::alloc::string::String,
    /// Client network address. Filled if InstanceConfig.multivlan_config is false.
    #[deprecated]
    #[prost(message, optional, tag = "6")]
    pub client_network: ::core::option::Option<instance_config::NetworkAddress>,
    /// Private network address, if any. Filled if InstanceConfig.multivlan_config
    /// is false.
    #[deprecated]
    #[prost(message, optional, tag = "7")]
    pub private_network: ::core::option::Option<instance_config::NetworkAddress>,
    /// User note field, it can be used by customers to add additional information
    /// for the BMS Ops team .
    #[prost(string, tag = "8")]
    pub user_note: ::prost::alloc::string::String,
    /// If true networks can be from different projects of the same vendor account.
    #[prost(bool, tag = "9")]
    pub account_networks_enabled: bool,
    /// The type of network configuration on the instance.
    #[prost(enumeration = "instance_config::NetworkConfig", tag = "10")]
    pub network_config: i32,
    /// Server network template name. Filled if InstanceConfig.multivlan_config is
    /// true.
    #[prost(string, tag = "11")]
    pub network_template: ::prost::alloc::string::String,
    /// List of logical interfaces for the instance. The number of logical
    /// interfaces will be the same as number of hardware bond/nic on the chosen
    /// network template. Filled if InstanceConfig.multivlan_config is true.
    #[prost(message, repeated, tag = "12")]
    pub logical_interfaces: ::prost::alloc::vec::Vec<LogicalInterface>,
    /// List of names of ssh keys used to provision the instance.
    #[prost(string, repeated, tag = "13")]
    pub ssh_key_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `InstanceConfig`.
pub mod instance_config {
    /// A network.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetworkAddress {
        /// Id of the network to use, within the same ProvisioningConfig request.
        #[prost(string, tag = "1")]
        pub network_id: ::prost::alloc::string::String,
        /// IPv4 address to be assigned to the server.
        #[prost(string, tag = "2")]
        pub address: ::prost::alloc::string::String,
        /// Name of the existing network to use.
        #[prost(string, tag = "3")]
        pub existing_network_id: ::prost::alloc::string::String,
    }
    /// The network configuration of the instance.
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
    pub enum NetworkConfig {
        /// The unspecified network configuration.
        NetworkconfigUnspecified = 0,
        /// Instance part of single client network and single private network.
        SingleVlan = 1,
        /// Instance part of multiple (or single) client networks and private
        /// networks.
        MultiVlan = 2,
    }
    impl NetworkConfig {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NetworkConfig::NetworkconfigUnspecified => "NETWORKCONFIG_UNSPECIFIED",
                NetworkConfig::SingleVlan => "SINGLE_VLAN",
                NetworkConfig::MultiVlan => "MULTI_VLAN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NETWORKCONFIG_UNSPECIFIED" => Some(Self::NetworkconfigUnspecified),
                "SINGLE_VLAN" => Some(Self::SingleVlan),
                "MULTI_VLAN" => Some(Self::MultiVlan),
                _ => None,
            }
        }
    }
}
/// Configuration parameters for a new volume.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeConfig {
    /// Output only. The name of the volume config.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A transient unique identifier to identify a volume within an
    /// ProvisioningConfig request.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Whether snapshots should be enabled.
    #[prost(bool, tag = "3")]
    pub snapshots_enabled: bool,
    /// The type of this Volume.
    #[prost(enumeration = "volume_config::Type", tag = "4")]
    pub r#type: i32,
    /// Volume protocol.
    #[prost(enumeration = "volume_config::Protocol", tag = "5")]
    pub protocol: i32,
    /// The requested size of this volume, in GB.
    #[prost(int32, tag = "6")]
    pub size_gb: i32,
    /// LUN ranges to be configured. Set only when protocol is PROTOCOL_FC.
    #[prost(message, repeated, tag = "7")]
    pub lun_ranges: ::prost::alloc::vec::Vec<volume_config::LunRange>,
    /// Machine ids connected to this volume. Set only when protocol is
    /// PROTOCOL_FC.
    #[prost(string, repeated, tag = "8")]
    pub machine_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// NFS exports. Set only when protocol is PROTOCOL_NFS.
    #[prost(message, repeated, tag = "9")]
    pub nfs_exports: ::prost::alloc::vec::Vec<volume_config::NfsExport>,
    /// User note field, it can be used by customers to add additional information
    /// for the BMS Ops team .
    #[prost(string, tag = "10")]
    pub user_note: ::prost::alloc::string::String,
    /// The GCP service of the storage volume. Available gcp_service are in
    /// <https://cloud.google.com/bare-metal/docs/bms-planning.>
    #[prost(string, tag = "11")]
    pub gcp_service: ::prost::alloc::string::String,
    /// Performance tier of the Volume.
    /// Default is SHARED.
    #[prost(enumeration = "VolumePerformanceTier", tag = "12")]
    pub performance_tier: i32,
}
/// Nested message and enum types in `VolumeConfig`.
pub mod volume_config {
    /// A LUN(Logical Unit Number) range.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LunRange {
        /// Number of LUNs to create.
        #[prost(int32, tag = "1")]
        pub quantity: i32,
        /// The requested size of each LUN, in GB.
        #[prost(int32, tag = "2")]
        pub size_gb: i32,
    }
    /// A NFS export entry.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NfsExport {
        /// Network to use to publish the export.
        #[prost(string, tag = "1")]
        pub network_id: ::prost::alloc::string::String,
        /// Export permissions.
        #[prost(enumeration = "nfs_export::Permissions", tag = "4")]
        pub permissions: i32,
        /// Disable root squashing, which is a feature of NFS.
        /// Root squash is a special mapping of the remote superuser (root) identity
        /// when using identity authentication.
        #[prost(bool, tag = "5")]
        pub no_root_squash: bool,
        /// Allow the setuid flag.
        #[prost(bool, tag = "6")]
        pub allow_suid: bool,
        /// Allow dev flag in NfsShare AllowedClientsRequest.
        #[prost(bool, tag = "7")]
        pub allow_dev: bool,
        /// A client object.
        #[prost(oneof = "nfs_export::Client", tags = "2, 3")]
        pub client: ::core::option::Option<nfs_export::Client>,
    }
    /// Nested message and enum types in `NfsExport`.
    pub mod nfs_export {
        /// Permissions that can granted for an export.
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
        pub enum Permissions {
            /// Unspecified value.
            Unspecified = 0,
            /// Read-only permission.
            ReadOnly = 1,
            /// Read-write permission.
            ReadWrite = 2,
        }
        impl Permissions {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Permissions::Unspecified => "PERMISSIONS_UNSPECIFIED",
                    Permissions::ReadOnly => "READ_ONLY",
                    Permissions::ReadWrite => "READ_WRITE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "PERMISSIONS_UNSPECIFIED" => Some(Self::Unspecified),
                    "READ_ONLY" => Some(Self::ReadOnly),
                    "READ_WRITE" => Some(Self::ReadWrite),
                    _ => None,
                }
            }
        }
        /// A client object.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Client {
            /// Either a single machine, identified by an ID, or a comma-separated
            /// list of machine IDs.
            #[prost(string, tag = "2")]
            MachineId(::prost::alloc::string::String),
            /// A CIDR range.
            #[prost(string, tag = "3")]
            Cidr(::prost::alloc::string::String),
        }
    }
    /// The types of Volumes.
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
    pub enum Type {
        /// The unspecified type.
        Unspecified = 0,
        /// This Volume is on flash.
        Flash = 1,
        /// This Volume is on disk.
        Disk = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Flash => "FLASH",
                Type::Disk => "DISK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "FLASH" => Some(Self::Flash),
                "DISK" => Some(Self::Disk),
                _ => None,
            }
        }
    }
    /// The protocol used to access the volume.
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
    pub enum Protocol {
        /// Unspecified value.
        Unspecified = 0,
        /// Fibre channel.
        Fc = 1,
        /// Network file system.
        Nfs = 2,
    }
    impl Protocol {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Protocol::Unspecified => "PROTOCOL_UNSPECIFIED",
                Protocol::Fc => "PROTOCOL_FC",
                Protocol::Nfs => "PROTOCOL_NFS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROTOCOL_UNSPECIFIED" => Some(Self::Unspecified),
                "PROTOCOL_FC" => Some(Self::Fc),
                "PROTOCOL_NFS" => Some(Self::Nfs),
                _ => None,
            }
        }
    }
}
/// Configuration parameters for a new network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    /// Output only. The name of the network config.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A transient unique identifier to identify a volume within an
    /// ProvisioningConfig request.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// The type of this network, either Client or Private.
    #[prost(enumeration = "network_config::Type", tag = "3")]
    pub r#type: i32,
    /// Interconnect bandwidth. Set only when type is CLIENT.
    #[prost(enumeration = "network_config::Bandwidth", tag = "4")]
    pub bandwidth: i32,
    /// List of VLAN attachments. As of now there are always 2 attachments, but it
    /// is going to change in  the future (multi vlan).
    #[prost(message, repeated, tag = "5")]
    pub vlan_attachments: ::prost::alloc::vec::Vec<network_config::IntakeVlanAttachment>,
    /// CIDR range of the network.
    #[prost(string, tag = "6")]
    pub cidr: ::prost::alloc::string::String,
    /// Service CIDR, if any.
    #[prost(enumeration = "network_config::ServiceCidr", tag = "7")]
    pub service_cidr: i32,
    /// User note field, it can be used by customers to add additional information
    /// for the BMS Ops team .
    #[prost(string, tag = "8")]
    pub user_note: ::prost::alloc::string::String,
    /// The GCP service of the network. Available gcp_service are in
    /// <https://cloud.google.com/bare-metal/docs/bms-planning.>
    #[prost(string, tag = "9")]
    pub gcp_service: ::prost::alloc::string::String,
    /// Whether the VLAN attachment pair is located in the same project.
    #[prost(bool, tag = "10")]
    pub vlan_same_project: bool,
    /// The JumboFramesEnabled option for customer to set.
    #[prost(bool, tag = "11")]
    pub jumbo_frames_enabled: bool,
}
/// Nested message and enum types in `NetworkConfig`.
pub mod network_config {
    /// A GCP vlan attachment.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntakeVlanAttachment {
        /// Identifier of the VLAN attachment.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Attachment pairing key.
        #[prost(string, tag = "2")]
        pub pairing_key: ::prost::alloc::string::String,
    }
    /// Network type.
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
    pub enum Type {
        /// Unspecified value.
        Unspecified = 0,
        /// Client network, that is a network peered to a GCP VPC.
        Client = 1,
        /// Private network, that is a network local to the BMS POD.
        Private = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Client => "CLIENT",
                Type::Private => "PRIVATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLIENT" => Some(Self::Client),
                "PRIVATE" => Some(Self::Private),
                _ => None,
            }
        }
    }
    /// Interconnect bandwidth.
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
    pub enum Bandwidth {
        /// Unspecified value.
        Unspecified = 0,
        /// 1 Gbps.
        Bw1Gbps = 1,
        /// 2 Gbps.
        Bw2Gbps = 2,
        /// 5 Gbps.
        Bw5Gbps = 3,
        /// 10 Gbps.
        Bw10Gbps = 4,
    }
    impl Bandwidth {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Bandwidth::Unspecified => "BANDWIDTH_UNSPECIFIED",
                Bandwidth::Bw1Gbps => "BW_1_GBPS",
                Bandwidth::Bw2Gbps => "BW_2_GBPS",
                Bandwidth::Bw5Gbps => "BW_5_GBPS",
                Bandwidth::Bw10Gbps => "BW_10_GBPS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BANDWIDTH_UNSPECIFIED" => Some(Self::Unspecified),
                "BW_1_GBPS" => Some(Self::Bw1Gbps),
                "BW_2_GBPS" => Some(Self::Bw2Gbps),
                "BW_5_GBPS" => Some(Self::Bw5Gbps),
                "BW_10_GBPS" => Some(Self::Bw10Gbps),
                _ => None,
            }
        }
    }
    /// Service network block.
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
    pub enum ServiceCidr {
        /// Unspecified value.
        Unspecified = 0,
        /// Services are disabled for the given network.
        Disabled = 1,
        /// Use the highest /26 block of the network to host services.
        High26 = 2,
        /// Use the highest /27 block of the network to host services.
        High27 = 3,
        /// Use the highest /28 block of the network to host services.
        High28 = 4,
    }
    impl ServiceCidr {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ServiceCidr::Unspecified => "SERVICE_CIDR_UNSPECIFIED",
                ServiceCidr::Disabled => "DISABLED",
                ServiceCidr::High26 => "HIGH_26",
                ServiceCidr::High27 => "HIGH_27",
                ServiceCidr::High28 => "HIGH_28",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SERVICE_CIDR_UNSPECIFIED" => Some(Self::Unspecified),
                "DISABLED" => Some(Self::Disabled),
                "HIGH_26" => Some(Self::High26),
                "HIGH_27" => Some(Self::High27),
                "HIGH_28" => Some(Self::High28),
                _ => None,
            }
        }
    }
}
/// A resource budget.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceQuota {
    /// Output only. The name of the instance quota.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Instance type.
    /// Deprecated: use gcp_service.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub instance_type: ::prost::alloc::string::String,
    /// The gcp service of the provisioning quota.
    #[prost(string, tag = "5")]
    pub gcp_service: ::prost::alloc::string::String,
    /// Location where the quota applies.
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
    /// Number of machines than can be created for the given location and
    /// instance_type.
    #[prost(int32, tag = "4")]
    pub available_machine_count: i32,
}
/// Request for GetProvisioningConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProvisioningConfigRequest {
    /// Required. Name of the ProvisioningConfig.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for CreateProvisioningConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProvisioningConfigRequest {
    /// Required. The parent project and location containing the
    /// ProvisioningConfig.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ProvisioningConfig to create.
    #[prost(message, optional, tag = "2")]
    pub provisioning_config: ::core::option::Option<ProvisioningConfig>,
    /// Optional. Email provided to send a confirmation with provisioning config
    /// to.
    #[prost(string, tag = "3")]
    pub email: ::prost::alloc::string::String,
}
/// Message for updating a ProvisioningConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProvisioningConfigRequest {
    /// Required. The ProvisioningConfig to update.
    #[prost(message, optional, tag = "1")]
    pub provisioning_config: ::core::option::Option<ProvisioningConfig>,
    /// Required. The list of fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. Email provided to send a confirmation with provisioning config
    /// to.
    #[prost(string, tag = "3")]
    pub email: ::prost::alloc::string::String,
}
/// A storage volume.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    /// Output only. The resource name of this `Volume`.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// Format:
    /// `projects/{project}/locations/{location}/volumes/{volume}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An identifier for the `Volume`, generated by the backend.
    #[prost(string, tag = "11")]
    pub id: ::prost::alloc::string::String,
    /// The storage type for this volume.
    #[prost(enumeration = "volume::StorageType", tag = "2")]
    pub storage_type: i32,
    /// The state of this storage volume.
    #[prost(enumeration = "volume::State", tag = "3")]
    pub state: i32,
    /// The requested size of this storage volume, in GiB.
    #[prost(int64, tag = "4")]
    pub requested_size_gib: i64,
    /// Originally requested size, in GiB.
    #[prost(int64, tag = "16")]
    pub originally_requested_size_gib: i64,
    /// The current size of this storage volume, in GiB, including space reserved
    /// for snapshots. This size might be different than the requested size if the
    /// storage volume has been configured with auto grow or auto shrink.
    #[prost(int64, tag = "5")]
    pub current_size_gib: i64,
    /// Additional emergency size that was requested for this Volume, in GiB.
    /// current_size_gib includes this value.
    #[prost(int64, tag = "14")]
    pub emergency_size_gib: i64,
    /// Maximum size volume can be expanded to in case of evergency, in GiB.
    #[prost(int64, tag = "17")]
    pub max_size_gib: i64,
    /// The size, in GiB, that this storage volume has expanded as a result of an
    /// auto grow policy. In the absence of auto-grow, the value is 0.
    #[prost(int64, tag = "6")]
    pub auto_grown_size_gib: i64,
    /// The space remaining in the storage volume for new LUNs, in GiB, excluding
    /// space reserved for snapshots.
    #[prost(int64, tag = "7")]
    pub remaining_space_gib: i64,
    /// Details about snapshot space reservation and usage on the storage volume.
    #[prost(message, optional, tag = "8")]
    pub snapshot_reservation_detail: ::core::option::Option<
        volume::SnapshotReservationDetail,
    >,
    /// The behavior to use when snapshot reserved space is full.
    #[prost(enumeration = "volume::SnapshotAutoDeleteBehavior", tag = "9")]
    pub snapshot_auto_delete_behavior: i32,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "12")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Whether snapshots are enabled.
    #[prost(bool, tag = "13")]
    pub snapshot_enabled: bool,
    /// Immutable. Pod name.
    #[prost(string, tag = "15")]
    pub pod: ::prost::alloc::string::String,
    /// Output only. Storage protocol for the Volume.
    #[prost(enumeration = "volume::Protocol", tag = "18")]
    pub protocol: i32,
    /// Output only. Whether this volume is a boot volume. A boot volume is one
    /// which contains a boot LUN.
    #[prost(bool, tag = "19")]
    pub boot_volume: bool,
    /// Immutable. Performance tier of the Volume.
    /// Default is SHARED.
    #[prost(enumeration = "VolumePerformanceTier", tag = "20")]
    pub performance_tier: i32,
    /// Input only. User-specified notes for new Volume.
    /// Used to provision Volumes that require manual intervention.
    #[prost(string, tag = "21")]
    pub notes: ::prost::alloc::string::String,
    /// The workload profile for the volume.
    #[prost(enumeration = "volume::WorkloadProfile", tag = "22")]
    pub workload_profile: i32,
    /// Output only. Time after which volume will be fully deleted.
    /// It is filled only for volumes in COOLOFF state.
    #[prost(message, optional, tag = "24")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Instances this Volume is attached to.
    /// This field is set only in Get requests.
    #[prost(string, repeated, tag = "25")]
    pub instances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Is the Volume attached at at least one instance.
    /// This field is a lightweight counterpart of `instances` field.
    /// It is filled in List responses as well.
    #[prost(bool, tag = "26")]
    pub attached: bool,
}
/// Nested message and enum types in `Volume`.
pub mod volume {
    /// Details about snapshot space reservation and usage on the storage volume.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SnapshotReservationDetail {
        /// The space on this storage volume reserved for snapshots, shown in GiB.
        #[prost(int64, tag = "1")]
        pub reserved_space_gib: i64,
        /// The percent of snapshot space on this storage volume actually being used
        /// by the snapshot copies. This value might be higher than 100% if the
        /// snapshot copies have overflowed into the data portion of the storage
        /// volume.
        #[prost(int32, tag = "2")]
        pub reserved_space_used_percent: i32,
        /// The amount, in GiB, of available space in this storage volume's reserved
        /// snapshot space.
        #[prost(int64, tag = "3")]
        pub reserved_space_remaining_gib: i64,
        /// Percent of the total Volume size reserved for snapshot copies.
        /// Enabling snapshots requires reserving 20% or more of
        /// the storage volume space for snapshots. Maximum reserved space for
        /// snapshots is 40%.
        /// Setting this field will effectively set snapshot_enabled to true.
        #[prost(int32, tag = "4")]
        pub reserved_space_percent: i32,
    }
    /// The storage type for a volume.
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
    pub enum StorageType {
        /// The storage type for this volume is unknown.
        Unspecified = 0,
        /// The storage type for this volume is SSD.
        Ssd = 1,
        /// This storage type for this volume is HDD.
        Hdd = 2,
    }
    impl StorageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StorageType::Unspecified => "STORAGE_TYPE_UNSPECIFIED",
                StorageType::Ssd => "SSD",
                StorageType::Hdd => "HDD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STORAGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SSD" => Some(Self::Ssd),
                "HDD" => Some(Self::Hdd),
                _ => None,
            }
        }
    }
    /// The possible states for a storage volume.
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
        /// The storage volume is in an unknown state.
        Unspecified = 0,
        /// The storage volume is being created.
        Creating = 1,
        /// The storage volume is ready for use.
        Ready = 2,
        /// The storage volume has been requested to be deleted.
        Deleting = 3,
        /// The storage volume is being updated.
        Updating = 4,
        /// The storage volume is in cool off state. It will be deleted after
        /// `expire_time`.
        CoolOff = 5,
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
                State::Ready => "READY",
                State::Deleting => "DELETING",
                State::Updating => "UPDATING",
                State::CoolOff => "COOL_OFF",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "DELETING" => Some(Self::Deleting),
                "UPDATING" => Some(Self::Updating),
                "COOL_OFF" => Some(Self::CoolOff),
                _ => None,
            }
        }
    }
    /// The kinds of auto delete behavior to use when snapshot reserved space is
    /// full.
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
    pub enum SnapshotAutoDeleteBehavior {
        /// The unspecified behavior.
        Unspecified = 0,
        /// Don't delete any snapshots. This disables new snapshot creation, as
        /// long as the snapshot reserved space is full.
        Disabled = 1,
        /// Delete the oldest snapshots first.
        OldestFirst = 2,
        /// Delete the newest snapshots first.
        NewestFirst = 3,
    }
    impl SnapshotAutoDeleteBehavior {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SnapshotAutoDeleteBehavior::Unspecified => {
                    "SNAPSHOT_AUTO_DELETE_BEHAVIOR_UNSPECIFIED"
                }
                SnapshotAutoDeleteBehavior::Disabled => "DISABLED",
                SnapshotAutoDeleteBehavior::OldestFirst => "OLDEST_FIRST",
                SnapshotAutoDeleteBehavior::NewestFirst => "NEWEST_FIRST",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SNAPSHOT_AUTO_DELETE_BEHAVIOR_UNSPECIFIED" => Some(Self::Unspecified),
                "DISABLED" => Some(Self::Disabled),
                "OLDEST_FIRST" => Some(Self::OldestFirst),
                "NEWEST_FIRST" => Some(Self::NewestFirst),
                _ => None,
            }
        }
    }
    /// Storage protocol.
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
    pub enum Protocol {
        /// Value is not specified.
        Unspecified = 0,
        /// Fibre Channel protocol.
        FibreChannel = 1,
        /// NFS protocol means Volume is a NFS Share volume.
        /// Such volumes cannot be manipulated via Volumes API.
        Nfs = 2,
    }
    impl Protocol {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Protocol::Unspecified => "PROTOCOL_UNSPECIFIED",
                Protocol::FibreChannel => "FIBRE_CHANNEL",
                Protocol::Nfs => "NFS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROTOCOL_UNSPECIFIED" => Some(Self::Unspecified),
                "FIBRE_CHANNEL" => Some(Self::FibreChannel),
                "NFS" => Some(Self::Nfs),
                _ => None,
            }
        }
    }
    /// The possible values for a workload profile.
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
    pub enum WorkloadProfile {
        /// The workload profile is in an unknown state.
        Unspecified = 0,
        /// The workload profile is generic.
        Generic = 1,
        /// The workload profile is hana.
        Hana = 2,
    }
    impl WorkloadProfile {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WorkloadProfile::Unspecified => "WORKLOAD_PROFILE_UNSPECIFIED",
                WorkloadProfile::Generic => "GENERIC",
                WorkloadProfile::Hana => "HANA",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "WORKLOAD_PROFILE_UNSPECIFIED" => Some(Self::Unspecified),
                "GENERIC" => Some(Self::Generic),
                "HANA" => Some(Self::Hana),
                _ => None,
            }
        }
    }
}
/// Message for requesting storage volume information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVolumeRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting a list of storage volumes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumesRequest {
    /// Required. Parent value for ListVolumesRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server might return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message containing the list of storage volumes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumesResponse {
    /// The list of storage volumes.
    #[prost(message, repeated, tag = "1")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for updating a volume.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVolumeRequest {
    /// Required. The volume to update.
    ///
    /// The `name` field is used to identify the volume to update.
    /// Format: projects/{project}/locations/{location}/volumes/{volume}
    #[prost(message, optional, tag = "1")]
    pub volume: ::core::option::Option<Volume>,
    /// The list of fields to update.
    /// The only currently supported fields are:
    ///    'labels'
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Message requesting rename of a server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameVolumeRequest {
    /// Required. The `name` field is used to identify the volume.
    /// Format: projects/{project}/locations/{location}/volumes/{volume}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new `id` of the volume.
    #[prost(string, tag = "2")]
    pub new_volume_id: ::prost::alloc::string::String,
}
/// Request for skip volume cooloff and delete it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvictVolumeRequest {
    /// Required. The name of the Volume.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for emergency resize Volume.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResizeVolumeRequest {
    /// Required. Volume to resize.
    #[prost(string, tag = "1")]
    pub volume: ::prost::alloc::string::String,
    /// New Volume size, in GiB.
    #[prost(int64, tag = "2")]
    pub size_gib: i64,
}
/// A storage volume logical unit number (LUN).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lun {
    /// Output only. The name of the LUN.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An identifier for the LUN, generated by the backend.
    #[prost(string, tag = "10")]
    pub id: ::prost::alloc::string::String,
    /// The state of this storage volume.
    #[prost(enumeration = "lun::State", tag = "2")]
    pub state: i32,
    /// The size of this LUN, in gigabytes.
    #[prost(int64, tag = "3")]
    pub size_gb: i64,
    /// The LUN multiprotocol type ensures the characteristics of the LUN are
    /// optimized for each operating system.
    #[prost(enumeration = "lun::MultiprotocolType", tag = "4")]
    pub multiprotocol_type: i32,
    /// Display the storage volume for this LUN.
    #[prost(string, tag = "5")]
    pub storage_volume: ::prost::alloc::string::String,
    /// Display if this LUN can be shared between multiple physical servers.
    #[prost(bool, tag = "6")]
    pub shareable: bool,
    /// Display if this LUN is a boot LUN.
    #[prost(bool, tag = "7")]
    pub boot_lun: bool,
    /// The storage type for this LUN.
    #[prost(enumeration = "lun::StorageType", tag = "8")]
    pub storage_type: i32,
    /// The WWID for this LUN.
    #[prost(string, tag = "9")]
    pub wwid: ::prost::alloc::string::String,
    /// Output only. Time after which LUN will be fully deleted.
    /// It is filled only for LUNs in COOL_OFF state.
    #[prost(message, optional, tag = "11")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Instances this Lun is attached to.
    #[prost(string, repeated, tag = "12")]
    pub instances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Lun`.
pub mod lun {
    /// The possible states for the LUN.
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
        /// The LUN is in an unknown state.
        Unspecified = 0,
        /// The LUN is being created.
        Creating = 1,
        /// The LUN is being updated.
        Updating = 2,
        /// The LUN is ready for use.
        Ready = 3,
        /// The LUN has been requested to be deleted.
        Deleting = 4,
        /// The LUN is in cool off state. It will be deleted after `expire_time`.
        CoolOff = 5,
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
                State::Updating => "UPDATING",
                State::Ready => "READY",
                State::Deleting => "DELETING",
                State::CoolOff => "COOL_OFF",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "UPDATING" => Some(Self::Updating),
                "READY" => Some(Self::Ready),
                "DELETING" => Some(Self::Deleting),
                "COOL_OFF" => Some(Self::CoolOff),
                _ => None,
            }
        }
    }
    /// Display the operating systems present for the LUN multiprotocol type.
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
    pub enum MultiprotocolType {
        /// Server has no OS specified.
        Unspecified = 0,
        /// Server with Linux OS.
        Linux = 1,
    }
    impl MultiprotocolType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MultiprotocolType::Unspecified => "MULTIPROTOCOL_TYPE_UNSPECIFIED",
                MultiprotocolType::Linux => "LINUX",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MULTIPROTOCOL_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "LINUX" => Some(Self::Linux),
                _ => None,
            }
        }
    }
    /// The storage types for a LUN.
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
    pub enum StorageType {
        /// The storage type for this LUN is unknown.
        Unspecified = 0,
        /// This storage type for this LUN is SSD.
        Ssd = 1,
        /// This storage type for this LUN is HDD.
        Hdd = 2,
    }
    impl StorageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StorageType::Unspecified => "STORAGE_TYPE_UNSPECIFIED",
                StorageType::Ssd => "SSD",
                StorageType::Hdd => "HDD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STORAGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SSD" => Some(Self::Ssd),
                "HDD" => Some(Self::Hdd),
                _ => None,
            }
        }
    }
}
/// Message for requesting storage lun information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLunRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting a list of storage volume luns.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLunsRequest {
    /// Required. Parent value for ListLunsRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server might return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message containing the list of storage volume luns.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLunsResponse {
    /// The list of luns.
    #[prost(message, repeated, tag = "1")]
    pub luns: ::prost::alloc::vec::Vec<Lun>,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for skip lun cooloff and delete it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvictLunRequest {
    /// Required. The name of the lun.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Immutable. The resource name of this `Instance`.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// Format:
    /// `projects/{project}/locations/{location}/instances/{instance}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. An identifier for the `Instance`, generated by the backend.
    #[prost(string, tag = "11")]
    pub id: ::prost::alloc::string::String,
    /// Output only. Create a time stamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update a time stamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. The server type.
    /// [Available server
    /// types](<https://cloud.google.com/bare-metal/docs/bms-planning#server_configurations>)
    #[prost(string, tag = "4")]
    pub machine_type: ::prost::alloc::string::String,
    /// Output only. The state of the server.
    #[prost(enumeration = "instance::State", tag = "5")]
    pub state: i32,
    /// True if you enable hyperthreading for the server, otherwise false.
    /// The default value is false.
    #[prost(bool, tag = "6")]
    pub hyperthreading_enabled: bool,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "7")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Immutable. List of LUNs associated with this server.
    #[prost(message, repeated, tag = "8")]
    pub luns: ::prost::alloc::vec::Vec<Lun>,
    /// Input only. List of Volumes to attach to this Instance on creation.
    /// This field won't be populated in Get/List responses.
    #[prost(message, repeated, tag = "16")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// Output only. List of networks associated with this server.
    #[prost(message, repeated, tag = "9")]
    pub networks: ::prost::alloc::vec::Vec<Network>,
    /// Output only. True if the interactive serial console feature is enabled for
    /// the instance, false otherwise. The default value is false.
    #[prost(bool, tag = "10")]
    pub interactive_serial_console_enabled: bool,
    /// The OS image currently installed on the server.
    #[prost(string, tag = "12")]
    pub os_image: ::prost::alloc::string::String,
    /// Immutable. Pod name.
    /// Pod is an independent part of infrastructure.
    /// Instance can be connected to the assets (networks, volumes) allocated
    /// in the same pod only.
    #[prost(string, tag = "13")]
    pub pod: ::prost::alloc::string::String,
    /// Instance network template name. For eg, bondaa-bondaa, bondab-nic, etc.
    /// Generally, the template name follows the syntax of
    /// "bond<bond_mode>" or "nic".
    #[prost(string, tag = "14")]
    pub network_template: ::prost::alloc::string::String,
    /// List of logical interfaces for the instance. The number of logical
    /// interfaces will be the same as number of hardware bond/nic on the chosen
    /// network template. For the non-multivlan configurations (for eg, existing
    /// servers) that use existing default network template (bondaa-bondaa), both
    /// the Instance.networks field and the Instance.logical_interfaces fields will
    /// be filled to ensure backward compatibility. For the others, only
    /// Instance.logical_interfaces will be filled.
    #[prost(message, repeated, tag = "15")]
    pub logical_interfaces: ::prost::alloc::vec::Vec<LogicalInterface>,
    /// Output only. Text field about info for logging in.
    #[prost(string, tag = "17")]
    pub login_info: ::prost::alloc::string::String,
    /// The workload profile for the instance.
    #[prost(enumeration = "WorkloadProfile", tag = "18")]
    pub workload_profile: i32,
    /// Output only. The firmware version for the instance.
    #[prost(string, tag = "19")]
    pub firmware_version: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// The possible states for this server.
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
        /// The server is in an unknown state.
        Unspecified = 0,
        /// The server is being provisioned.
        Provisioning = 1,
        /// The server is running.
        Running = 2,
        /// The server has been deleted.
        Deleted = 3,
        /// The server is being updated.
        Updating = 4,
        /// The server is starting.
        Starting = 5,
        /// The server is stopping.
        Stopping = 6,
        /// The server is shutdown.
        Shutdown = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Provisioning => "PROVISIONING",
                State::Running => "RUNNING",
                State::Deleted => "DELETED",
                State::Updating => "UPDATING",
                State::Starting => "STARTING",
                State::Stopping => "STOPPING",
                State::Shutdown => "SHUTDOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PROVISIONING" => Some(Self::Provisioning),
                "RUNNING" => Some(Self::Running),
                "DELETED" => Some(Self::Deleted),
                "UPDATING" => Some(Self::Updating),
                "STARTING" => Some(Self::Starting),
                "STOPPING" => Some(Self::Stopping),
                "SHUTDOWN" => Some(Self::Shutdown),
                _ => None,
            }
        }
    }
}
/// Message for requesting server information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting the list of servers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. Parent value for ListInstancesRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, the server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for the list of servers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// The list of servers.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message requesting to updating a server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    /// Required. The server to update.
    ///
    /// The `name` field is used to identify the instance to update.
    /// Format: projects/{project}/locations/{location}/instances/{instance}
    #[prost(message, optional, tag = "1")]
    pub instance: ::core::option::Option<Instance>,
    /// The list of fields to update.
    /// The currently supported fields are:
    ///    `labels`
    ///    `hyperthreading_enabled`
    ///    `os_image`
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Message requesting rename of a server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameInstanceRequest {
    /// Required. The `name` field is used to identify the instance.
    /// Format: projects/{project}/locations/{location}/instances/{instance}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new `id` of the instance.
    #[prost(string, tag = "2")]
    pub new_instance_id: ::prost::alloc::string::String,
}
/// Message requesting to reset a server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetInstanceRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message requesting to start a server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartInstanceRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message from starting a server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartInstanceResponse {}
/// Message requesting to stop a server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstanceRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message from stopping a server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstanceResponse {}
/// Message for enabling the interactive serial console on an instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableInteractiveSerialConsoleRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for response of EnableInteractiveSerialConsole.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableInteractiveSerialConsoleResponse {}
/// Message for disabling the interactive serial console on an instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableInteractiveSerialConsoleRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for response of DisableInteractiveSerialConsole.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableInteractiveSerialConsoleResponse {}
/// Message for detach specific LUN from an Instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetachLunRequest {
    /// Required. Name of the instance.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Required. Name of the Lun to detach.
    #[prost(string, tag = "2")]
    pub lun: ::prost::alloc::string::String,
    /// If true, performs lun unmapping without instance reboot.
    #[prost(bool, tag = "3")]
    pub skip_reboot: bool,
}
/// Network template.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerNetworkTemplate {
    /// Output only. Template's unique name. The full resource name follows the
    /// pattern:
    /// `projects/{project}/locations/{location}/serverNetworkTemplate/{server_network_template}`
    /// Generally, the {server_network_template} follows the syntax of
    /// "bond<interface_type_index><bond_mode>" or "nic<interface_type_index>".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Instance types this template is applicable to.
    #[prost(string, repeated, tag = "2")]
    pub applicable_instance_types: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Logical interfaces.
    #[prost(message, repeated, tag = "3")]
    pub logical_interfaces: ::prost::alloc::vec::Vec<
        server_network_template::LogicalInterface,
    >,
}
/// Nested message and enum types in `ServerNetworkTemplate`.
pub mod server_network_template {
    /// Logical interface.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LogicalInterface {
        /// Interface name.
        /// This is not a globally unique identifier.
        /// Name is unique only inside the ServerNetworkTemplate. This is of syntax
        /// <bond><interface_type_index><bond_mode> or <nic><interface_type_index>
        /// and forms part of the network template name.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Interface type.
        #[prost(enumeration = "logical_interface::InterfaceType", tag = "2")]
        pub r#type: i32,
        /// If true, interface must have network connected.
        #[prost(bool, tag = "3")]
        pub required: bool,
    }
    /// Nested message and enum types in `LogicalInterface`.
    pub mod logical_interface {
        /// Interface type.
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
        pub enum InterfaceType {
            /// Unspecified value.
            Unspecified = 0,
            /// Bond interface type.
            Bond = 1,
            /// NIC interface type.
            Nic = 2,
        }
        impl InterfaceType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    InterfaceType::Unspecified => "INTERFACE_TYPE_UNSPECIFIED",
                    InterfaceType::Bond => "BOND",
                    InterfaceType::Nic => "NIC",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "INTERFACE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "BOND" => Some(Self::Bond),
                    "NIC" => Some(Self::Nic),
                    _ => None,
                }
            }
        }
    }
}
/// An NFS share.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfsShare {
    /// Immutable. The name of the NFS share.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. An identifier for the NFS share, generated by the backend.
    /// This field will be deprecated in the future, use `id` instead.
    #[prost(string, tag = "2")]
    pub nfs_share_id: ::prost::alloc::string::String,
    /// Output only. An identifier for the NFS share, generated by the backend.
    /// This is the same value as nfs_share_id and will replace it in the future.
    #[prost(string, tag = "8")]
    pub id: ::prost::alloc::string::String,
    /// Output only. The state of the NFS share.
    #[prost(enumeration = "nfs_share::State", tag = "3")]
    pub state: i32,
    /// Output only. The underlying volume of the share. Created automatically
    /// during provisioning.
    #[prost(string, tag = "4")]
    pub volume: ::prost::alloc::string::String,
    /// List of allowed access points.
    #[prost(message, repeated, tag = "5")]
    pub allowed_clients: ::prost::alloc::vec::Vec<nfs_share::AllowedClient>,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "6")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The requested size, in GiB.
    #[prost(int64, tag = "7")]
    pub requested_size_gib: i64,
    /// Immutable. The storage type of the underlying volume.
    #[prost(enumeration = "nfs_share::StorageType", tag = "9")]
    pub storage_type: i32,
}
/// Nested message and enum types in `NfsShare`.
pub mod nfs_share {
    /// Represents an 'access point' for the share.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AllowedClient {
        /// The network the access point sits on.
        #[prost(string, tag = "1")]
        pub network: ::prost::alloc::string::String,
        /// Output only. The IP address of the share on this network. Assigned
        /// automatically during provisioning based on the network's services_cidr.
        #[prost(string, tag = "2")]
        pub share_ip: ::prost::alloc::string::String,
        /// The subnet of IP addresses permitted to access the share.
        #[prost(string, tag = "3")]
        pub allowed_clients_cidr: ::prost::alloc::string::String,
        /// Mount permissions.
        #[prost(enumeration = "MountPermissions", tag = "4")]
        pub mount_permissions: i32,
        /// Allow dev flag.  Which controls whether to allow creation of devices.
        #[prost(bool, tag = "5")]
        pub allow_dev: bool,
        /// Allow the setuid flag.
        #[prost(bool, tag = "6")]
        pub allow_suid: bool,
        /// Disable root squashing, which is a feature of NFS.
        /// Root squash is a special mapping of the remote superuser (root) identity
        /// when using identity authentication.
        #[prost(bool, tag = "7")]
        pub no_root_squash: bool,
        /// Output only. The path to access NFS, in format shareIP:/InstanceID
        /// InstanceID is the generated ID instead of customer provided name.
        /// example like "10.0.0.0:/g123456789-nfs001"
        #[prost(string, tag = "8")]
        pub nfs_path: ::prost::alloc::string::String,
    }
    /// The possible states for this NFS share.
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
        /// The share is in an unknown state.
        Unspecified = 0,
        /// The share has been provisioned.
        Provisioned = 1,
        /// The NFS Share is being created.
        Creating = 2,
        /// The NFS Share is being updated.
        Updating = 3,
        /// The NFS Share has been requested to be deleted.
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
                State::Provisioned => "PROVISIONED",
                State::Creating => "CREATING",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PROVISIONED" => Some(Self::Provisioned),
                "CREATING" => Some(Self::Creating),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
    /// The possible mount permissions.
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
    pub enum MountPermissions {
        /// Permissions were not specified.
        Unspecified = 0,
        /// NFS share can be mount with read-only permissions.
        Read = 1,
        /// NFS share can be mount with read-write permissions.
        ReadWrite = 2,
    }
    impl MountPermissions {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MountPermissions::Unspecified => "MOUNT_PERMISSIONS_UNSPECIFIED",
                MountPermissions::Read => "READ",
                MountPermissions::ReadWrite => "READ_WRITE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MOUNT_PERMISSIONS_UNSPECIFIED" => Some(Self::Unspecified),
                "READ" => Some(Self::Read),
                "READ_WRITE" => Some(Self::ReadWrite),
                _ => None,
            }
        }
    }
    /// The storage type for a volume.
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
    pub enum StorageType {
        /// The storage type for this volume is unknown.
        Unspecified = 0,
        /// The storage type for this volume is SSD.
        Ssd = 1,
        /// This storage type for this volume is HDD.
        Hdd = 2,
    }
    impl StorageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StorageType::Unspecified => "STORAGE_TYPE_UNSPECIFIED",
                StorageType::Ssd => "SSD",
                StorageType::Hdd => "HDD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STORAGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SSD" => Some(Self::Ssd),
                "HDD" => Some(Self::Hdd),
                _ => None,
            }
        }
    }
}
/// Message for requesting NFS share information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNfsShareRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting a list of NFS shares.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNfsSharesRequest {
    /// Required. Parent value for ListNfsSharesRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server might return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message containing the list of NFS shares.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNfsSharesResponse {
    /// The list of NFS shares.
    #[prost(message, repeated, tag = "1")]
    pub nfs_shares: ::prost::alloc::vec::Vec<NfsShare>,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message requesting to updating an NFS share.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNfsShareRequest {
    /// Required. The NFS share to update.
    ///
    /// The `name` field is used to identify the NFS share to update.
    /// Format: projects/{project}/locations/{location}/nfsShares/{nfs_share}
    #[prost(message, optional, tag = "1")]
    pub nfs_share: ::core::option::Option<NfsShare>,
    /// The list of fields to update.
    /// The only currently supported fields are:
    ///    `labels`
    ///    `allowed_clients`
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Message requesting rename of a server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameNfsShareRequest {
    /// Required. The `name` field is used to identify the nfsshare.
    /// Format: projects/{project}/locations/{location}/nfsshares/{nfsshare}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new `id` of the nfsshare.
    #[prost(string, tag = "2")]
    pub new_nfsshare_id: ::prost::alloc::string::String,
}
/// Message for creating an NFS share.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNfsShareRequest {
    /// Required. The parent project and location.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The NfsShare to create.
    #[prost(message, optional, tag = "2")]
    pub nfs_share: ::core::option::Option<NfsShare>,
}
/// Message for deleting an NFS share.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNfsShareRequest {
    /// Required. The name of the NFS share to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// An SSH key, used for authorizing with the interactive serial console feature.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SshKey {
    /// Output only. The name of this SSH key.
    /// Currently, the only valid value for the location is "global".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The public SSH key. This must be in OpenSSH .authorized_keys format.
    #[prost(string, tag = "2")]
    pub public_key: ::prost::alloc::string::String,
}
/// Message for listing the public SSH keys in a project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSshKeysRequest {
    /// Required. The parent containing the SSH keys.
    /// Currently, the only valid value for the location is "global".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Message for response of ListSSHKeys.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSshKeysResponse {
    /// The SSH keys registered in the project.
    #[prost(message, repeated, tag = "1")]
    pub ssh_keys: ::prost::alloc::vec::Vec<SshKey>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "90")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for registering a public SSH key in a project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSshKeyRequest {
    /// Required. The parent containing the SSH keys.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The SSH key to register.
    #[prost(message, optional, tag = "2")]
    pub ssh_key: ::core::option::Option<SshKey>,
    /// Required. The ID to use for the key, which will become the final component
    /// of the key's resource name.
    ///
    /// This value must match the regex:
    ///    \[a-zA-Z0-9@.\-_\]{1,64}
    #[prost(string, tag = "3")]
    pub ssh_key_id: ::prost::alloc::string::String,
}
/// Message for deleting an SSH key from a project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSshKeyRequest {
    /// Required. The name of the SSH key to delete.
    /// Currently, the only valid value for the location is "global".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A snapshot of a volume. Only boot volumes can have snapshots.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeSnapshot {
    /// The name of the snapshot.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. An identifier for the snapshot, generated by the backend.
    #[prost(string, tag = "6")]
    pub id: ::prost::alloc::string::String,
    /// The description of the snapshot.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The creation time of the snapshot.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The name of the volume which this snapshot belongs to.
    #[prost(string, tag = "5")]
    pub storage_volume: ::prost::alloc::string::String,
    /// Output only. The type of the snapshot which indicates whether it was
    /// scheduled or manual/ad-hoc.
    #[prost(enumeration = "volume_snapshot::SnapshotType", tag = "7")]
    pub r#type: i32,
}
/// Nested message and enum types in `VolumeSnapshot`.
pub mod volume_snapshot {
    /// Represents the type of a snapshot.
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
    pub enum SnapshotType {
        /// Type is not specified.
        Unspecified = 0,
        /// Snapshot was taken manually by user.
        AdHoc = 1,
        /// Snapshot was taken automatically as a part of a snapshot schedule.
        Scheduled = 2,
    }
    impl SnapshotType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SnapshotType::Unspecified => "SNAPSHOT_TYPE_UNSPECIFIED",
                SnapshotType::AdHoc => "AD_HOC",
                SnapshotType::Scheduled => "SCHEDULED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SNAPSHOT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "AD_HOC" => Some(Self::AdHoc),
                "SCHEDULED" => Some(Self::Scheduled),
                _ => None,
            }
        }
    }
}
/// Message for requesting volume snapshot information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVolumeSnapshotRequest {
    /// Required. The name of the snapshot.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting a list of volume snapshots.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumeSnapshotsRequest {
    /// Required. Parent value for ListVolumesRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server might return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message containing the list of volume snapshots.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumeSnapshotsResponse {
    /// The list of snapshots.
    #[prost(message, repeated, tag = "1")]
    pub volume_snapshots: ::prost::alloc::vec::Vec<VolumeSnapshot>,
    /// A token identifying a page of results from the server.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for deleting named Volume snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVolumeSnapshotRequest {
    /// Required. The name of the snapshot to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a volume snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVolumeSnapshotRequest {
    /// Required. The volume to snapshot.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The snapshot to create.
    #[prost(message, optional, tag = "2")]
    pub volume_snapshot: ::core::option::Option<VolumeSnapshot>,
}
/// Message for restoring a volume snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreVolumeSnapshotRequest {
    /// Required. Name of the snapshot which will be used to restore its parent
    /// volume.
    #[prost(string, tag = "1")]
    pub volume_snapshot: ::prost::alloc::string::String,
}
/// Represents the metadata from a long-running operation.
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
    /// Output only. Name of the action executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user requested the cancellation
    /// of the operation. Operations that have been successfully cancelled
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used with the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Response message from resetting a server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetInstanceResponse {}
/// Generated client implementations.
pub mod bare_metal_solution_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Performs management operations on Bare Metal Solution servers.
    ///
    /// The `baremetalsolution.googleapis.com` service provides management
    /// capabilities for Bare Metal Solution servers. To access the API methods, you
    /// must assign Bare Metal Solution IAM roles containing the desired permissions
    /// to your staff in your Google Cloud project. You must also enable the Bare
    /// Metal Solution API. Once enabled, the methods act
    /// upon specific servers in your Bare Metal Solution environment.
    #[derive(Debug, Clone)]
    pub struct BareMetalSolutionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BareMetalSolutionClient<T>
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
        ) -> BareMetalSolutionClient<InterceptedService<T, F>>
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
            BareMetalSolutionClient::new(InterceptedService::new(inner, interceptor))
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
        /// List servers in a given project and location.
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "ListInstances",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get details about a single server.
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/GetInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "GetInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update details of a single server.
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/UpdateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "UpdateInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// RenameInstance sets a new name for an instance.
        /// Use with caution, previous names become immediately invalidated.
        pub async fn rename_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameInstanceRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/RenameInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "RenameInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Perform an ungraceful, hard reset on a server. Equivalent to shutting the
        /// power off and then turning it back on.
        pub async fn reset_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetInstanceRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ResetInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "ResetInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Starts a server that was shutdown.
        pub async fn start_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::StartInstanceRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/StartInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "StartInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Stop a running server.
        pub async fn stop_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::StopInstanceRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/StopInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "StopInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Enable the interactive serial console feature on an instance.
        pub async fn enable_interactive_serial_console(
            &mut self,
            request: impl tonic::IntoRequest<
                super::EnableInteractiveSerialConsoleRequest,
            >,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/EnableInteractiveSerialConsole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "EnableInteractiveSerialConsole",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Disable the interactive serial console feature on an instance.
        pub async fn disable_interactive_serial_console(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DisableInteractiveSerialConsoleRequest,
            >,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/DisableInteractiveSerialConsole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "DisableInteractiveSerialConsole",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Detach LUN from Instance.
        pub async fn detach_lun(
            &mut self,
            request: impl tonic::IntoRequest<super::DetachLunRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/DetachLun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "DetachLun",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the public SSH keys registered for the specified project.
        /// These SSH keys are used only for the interactive serial console feature.
        pub async fn list_ssh_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSshKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSshKeysResponse>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListSSHKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "ListSSHKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Register a public SSH key in the specified project for use with the
        /// interactive serial console feature.
        pub async fn create_ssh_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSshKeyRequest>,
        ) -> std::result::Result<tonic::Response<super::SshKey>, tonic::Status> {
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/CreateSSHKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "CreateSSHKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a public SSH key registered in the specified project.
        pub async fn delete_ssh_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSshKeyRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/DeleteSSHKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "DeleteSSHKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List storage volumes in a given project and location.
        pub async fn list_volumes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVolumesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVolumesResponse>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListVolumes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "ListVolumes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get details of a single storage volume.
        pub async fn get_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVolumeRequest>,
        ) -> std::result::Result<tonic::Response<super::Volume>, tonic::Status> {
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/GetVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "GetVolume",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update details of a single storage volume.
        pub async fn update_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVolumeRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/UpdateVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "UpdateVolume",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// RenameVolume sets a new name for a volume.
        /// Use with caution, previous names become immediately invalidated.
        pub async fn rename_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameVolumeRequest>,
        ) -> std::result::Result<tonic::Response<super::Volume>, tonic::Status> {
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/RenameVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "RenameVolume",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Skips volume's cooloff and deletes it now.
        /// Volume must be in cooloff state.
        pub async fn evict_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::EvictVolumeRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/EvictVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "EvictVolume",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Emergency Volume resize.
        pub async fn resize_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::ResizeVolumeRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ResizeVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "ResizeVolume",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List network in a given project and location.
        pub async fn list_networks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNetworksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNetworksResponse>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListNetworks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "ListNetworks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List all Networks (and used IPs for each Network) in the vendor account
        /// associated with the specified project.
        pub async fn list_network_usage(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNetworkUsageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNetworkUsageResponse>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListNetworkUsage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "ListNetworkUsage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get details of a single network.
        pub async fn get_network(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNetworkRequest>,
        ) -> std::result::Result<tonic::Response<super::Network>, tonic::Status> {
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/GetNetwork",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "GetNetwork",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update details of a single network.
        pub async fn update_network(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNetworkRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/UpdateNetwork",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "UpdateNetwork",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Takes a snapshot of a boot volume.
        /// Returns INVALID_ARGUMENT if called for a non-boot volume.
        pub async fn create_volume_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVolumeSnapshotRequest>,
        ) -> std::result::Result<tonic::Response<super::VolumeSnapshot>, tonic::Status> {
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/CreateVolumeSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "CreateVolumeSnapshot",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Uses the specified snapshot to restore its parent volume.
        /// Returns INVALID_ARGUMENT if called for a non-boot volume.
        pub async fn restore_volume_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreVolumeSnapshotRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/RestoreVolumeSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "RestoreVolumeSnapshot",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a volume snapshot.
        /// Returns INVALID_ARGUMENT if called for a non-boot volume.
        pub async fn delete_volume_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVolumeSnapshotRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/DeleteVolumeSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "DeleteVolumeSnapshot",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the specified snapshot resource.
        /// Returns INVALID_ARGUMENT if called for a non-boot volume.
        pub async fn get_volume_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVolumeSnapshotRequest>,
        ) -> std::result::Result<tonic::Response<super::VolumeSnapshot>, tonic::Status> {
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/GetVolumeSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "GetVolumeSnapshot",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the list of snapshots for the specified volume.
        /// Returns a response with an empty list of snapshots if called
        /// for a non-boot volume.
        pub async fn list_volume_snapshots(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVolumeSnapshotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVolumeSnapshotsResponse>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListVolumeSnapshots",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "ListVolumeSnapshots",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get details of a single storage logical unit number(LUN).
        pub async fn get_lun(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLunRequest>,
        ) -> std::result::Result<tonic::Response<super::Lun>, tonic::Status> {
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/GetLun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "GetLun",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List storage volume luns for given storage volume.
        pub async fn list_luns(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLunsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLunsResponse>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListLuns",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "ListLuns",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Skips lun's cooloff and deletes it now.
        /// Lun must be in cooloff state.
        pub async fn evict_lun(
            &mut self,
            request: impl tonic::IntoRequest<super::EvictLunRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/EvictLun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "EvictLun",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get details of a single NFS share.
        pub async fn get_nfs_share(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNfsShareRequest>,
        ) -> std::result::Result<tonic::Response<super::NfsShare>, tonic::Status> {
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/GetNfsShare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "GetNfsShare",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List NFS shares.
        pub async fn list_nfs_shares(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNfsSharesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNfsSharesResponse>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListNfsShares",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "ListNfsShares",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update details of a single NFS share.
        pub async fn update_nfs_share(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNfsShareRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/UpdateNfsShare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "UpdateNfsShare",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create an NFS share.
        pub async fn create_nfs_share(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNfsShareRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/CreateNfsShare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "CreateNfsShare",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// RenameNfsShare sets a new name for an nfsshare.
        /// Use with caution, previous names become immediately invalidated.
        pub async fn rename_nfs_share(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameNfsShareRequest>,
        ) -> std::result::Result<tonic::Response<super::NfsShare>, tonic::Status> {
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/RenameNfsShare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "RenameNfsShare",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete an NFS share. The underlying volume is automatically deleted.
        pub async fn delete_nfs_share(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNfsShareRequest>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/DeleteNfsShare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "DeleteNfsShare",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List the budget details to provision resources on a given project.
        pub async fn list_provisioning_quotas(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProvisioningQuotasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProvisioningQuotasResponse>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListProvisioningQuotas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "ListProvisioningQuotas",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Submit a provisiong configuration for a given project.
        pub async fn submit_provisioning_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitProvisioningConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubmitProvisioningConfigResponse>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/SubmitProvisioningConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "SubmitProvisioningConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get ProvisioningConfig by name.
        pub async fn get_provisioning_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProvisioningConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProvisioningConfig>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/GetProvisioningConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "GetProvisioningConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create new ProvisioningConfig.
        pub async fn create_provisioning_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProvisioningConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProvisioningConfig>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/CreateProvisioningConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "CreateProvisioningConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update existing ProvisioningConfig.
        pub async fn update_provisioning_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProvisioningConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProvisioningConfig>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/UpdateProvisioningConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "UpdateProvisioningConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// RenameNetwork sets a new name for a network.
        /// Use with caution, previous names become immediately invalidated.
        pub async fn rename_network(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameNetworkRequest>,
        ) -> std::result::Result<tonic::Response<super::Network>, tonic::Status> {
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/RenameNetwork",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "RenameNetwork",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the list of OS images which are currently approved.
        pub async fn list_os_images(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOsImagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOsImagesResponse>,
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
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListOSImages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.baremetalsolution.v2.BareMetalSolution",
                        "ListOSImages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
