/// Network configuration in the consumer project
/// with which the peering has to be done.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    /// Required. Management CIDR used by VMware management appliances.
    #[prost(string, tag = "4")]
    pub management_cidr: ::prost::alloc::string::String,
    /// Optional. The relative resource name of the VMware Engine network attached
    /// to the private cloud. Specify the name in the following form:
    /// `projects/{project}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}`
    /// where `{project}` can either be a project number or a project ID.
    #[prost(string, tag = "5")]
    pub vmware_engine_network: ::prost::alloc::string::String,
    /// Output only. The canonical name of the VMware Engine network in the form:
    /// `projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}`
    #[prost(string, tag = "6")]
    pub vmware_engine_network_canonical: ::prost::alloc::string::String,
    /// Output only. The IP address layout version of the management IP address
    /// range. Possible versions include:
    /// * `managementIpAddressLayoutVersion=1`: Indicates the legacy IP address
    /// layout used by some existing private clouds. This is no longer supported
    /// for new private clouds as it does not support all features.
    /// * `managementIpAddressLayoutVersion=2`: Indicates the latest IP address
    /// layout used by all newly created private clouds. This version supports all
    /// current features.
    #[prost(int32, tag = "8")]
    pub management_ip_address_layout_version: i32,
    /// Output only. DNS Server IP of the Private Cloud.
    /// All DNS queries can be forwarded to this address for name resolution of
    /// Private Cloud's management entities like vCenter, NSX-T Manager and
    /// ESXi hosts.
    #[prost(string, tag = "9")]
    pub dns_server_ip: ::prost::alloc::string::String,
}
/// Information about the type and number of nodes associated with the cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeTypeConfig {
    /// Required. The number of nodes of this type in the cluster
    #[prost(int32, tag = "1")]
    pub node_count: i32,
    /// Optional. Customized number of cores available to each node of the type.
    /// This number must always be one of `nodeType.availableCustomCoreCounts`.
    /// If zero is provided max value from `nodeType.availableCustomCoreCounts`
    /// will be used.
    #[prost(int32, tag = "2")]
    pub custom_core_count: i32,
}
/// Configuration of a stretched cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StretchedClusterConfig {
    /// Required. Zone that will remain operational when connection between the two
    /// zones is lost. Specify the resource name of a zone that belongs to the
    /// region of the private cloud. For example:
    /// `projects/{project}/locations/europe-west3-a` where `{project}` can either
    /// be a project number or a project ID.
    #[prost(string, tag = "1")]
    pub preferred_location: ::prost::alloc::string::String,
    /// Required. Additional zone for a higher level of availability and load
    /// balancing. Specify the resource name of a zone that belongs to the region
    /// of the private cloud. For example:
    /// `projects/{project}/locations/europe-west3-b` where `{project}` can either
    /// be a project number or a project ID.
    #[prost(string, tag = "2")]
    pub secondary_location: ::prost::alloc::string::String,
}
/// Represents a private cloud resource. Private clouds of type `STANDARD` and
/// `TIME_LIMITED` are zonal resources, `STRETCHED` private clouds are
/// regional.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateCloud {
    /// Output only. The resource name of this private cloud.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation time of this resource.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update time of this resource.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the resource was scheduled for deletion.
    #[prost(message, optional, tag = "4")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the resource will be irreversibly deleted.
    #[prost(message, optional, tag = "5")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. State of the resource. New values may be added to this enum
    /// when appropriate.
    #[prost(enumeration = "private_cloud::State", tag = "8")]
    pub state: i32,
    /// Required. Network configuration of the private cloud.
    #[prost(message, optional, tag = "9")]
    pub network_config: ::core::option::Option<NetworkConfig>,
    /// Required. Input only. The management cluster for this private cloud.
    /// This field is required during creation of the private cloud to provide
    /// details for the default cluster.
    ///
    /// The following fields can't be changed after private cloud creation:
    /// `ManagementCluster.clusterId`, `ManagementCluster.nodeTypeId`.
    #[prost(message, optional, tag = "10")]
    pub management_cluster: ::core::option::Option<private_cloud::ManagementCluster>,
    /// User-provided description for this private cloud.
    #[prost(string, tag = "11")]
    pub description: ::prost::alloc::string::String,
    /// Output only. HCX appliance.
    #[prost(message, optional, tag = "17")]
    pub hcx: ::core::option::Option<Hcx>,
    /// Output only. NSX appliance.
    #[prost(message, optional, tag = "18")]
    pub nsx: ::core::option::Option<Nsx>,
    /// Output only. Vcenter appliance.
    #[prost(message, optional, tag = "19")]
    pub vcenter: ::core::option::Option<Vcenter>,
    /// Output only. System-generated unique identifier for the resource.
    #[prost(string, tag = "20")]
    pub uid: ::prost::alloc::string::String,
    /// Optional. Type of the private cloud. Defaults to STANDARD.
    #[prost(enumeration = "private_cloud::Type", tag = "22")]
    pub r#type: i32,
}
/// Nested message and enum types in `PrivateCloud`.
pub mod private_cloud {
    /// Management cluster configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ManagementCluster {
        /// Required. The user-provided identifier of the new `Cluster`.
        /// The identifier must meet the following requirements:
        ///
        /// * Only contains 1-63 alphanumeric characters and hyphens
        /// * Begins with an alphabetical character
        /// * Ends with a non-hyphen character
        /// * Not formatted as a UUID
        /// * Complies with [RFC
        /// 1034](<https://datatracker.ietf.org/doc/html/rfc1034>) (section 3.5)
        #[prost(string, tag = "1")]
        pub cluster_id: ::prost::alloc::string::String,
        /// Required. The map of cluster node types in this cluster, where the key is
        /// canonical identifier of the node type (corresponds to the `NodeType`).
        #[prost(btree_map = "string, message", tag = "7")]
        pub node_type_configs: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            super::NodeTypeConfig,
        >,
        /// Optional. Configuration of a stretched cluster. Required for STRETCHED
        /// private clouds.
        #[prost(message, optional, tag = "8")]
        pub stretched_cluster_config: ::core::option::Option<
            super::StretchedClusterConfig,
        >,
    }
    /// Enum State defines possible states of private clouds.
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
        /// The default value. This value should never be used.
        Unspecified = 0,
        /// The private cloud is ready.
        Active = 1,
        /// The private cloud is being created.
        Creating = 2,
        /// The private cloud is being updated.
        Updating = 3,
        /// The private cloud is in failed state.
        Failed = 5,
        /// The private cloud is scheduled for deletion. The deletion process can be
        /// cancelled by using the corresponding undelete method.
        Deleted = 6,
        /// The private cloud is irreversibly deleted and is being removed from the
        /// system.
        Purging = 7,
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
                State::Creating => "CREATING",
                State::Updating => "UPDATING",
                State::Failed => "FAILED",
                State::Deleted => "DELETED",
                State::Purging => "PURGING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "CREATING" => Some(Self::Creating),
                "UPDATING" => Some(Self::Updating),
                "FAILED" => Some(Self::Failed),
                "DELETED" => Some(Self::Deleted),
                "PURGING" => Some(Self::Purging),
                _ => None,
            }
        }
    }
    /// Enum Type defines private cloud type.
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
        /// Standard private is a zonal resource, with 3+ nodes. Default type.
        Standard = 0,
        /// Time limited private cloud is a zonal resource, can have only 1 node and
        /// has limited life span. Will be deleted after defined period of time,
        /// can be converted into standard private cloud by expanding it up to 3
        /// or more nodes.
        TimeLimited = 1,
        /// Stretched private cloud is a regional resource with redundancy,
        /// with a minimum of 6 nodes, nodes count has to be even.
        Stretched = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Standard => "STANDARD",
                Type::TimeLimited => "TIME_LIMITED",
                Type::Stretched => "STRETCHED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STANDARD" => Some(Self::Standard),
                "TIME_LIMITED" => Some(Self::TimeLimited),
                "STRETCHED" => Some(Self::Stretched),
                _ => None,
            }
        }
    }
}
/// A cluster in a private cloud.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// Output only. The resource name of this cluster.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/clusters/my-cluster`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation time of this resource.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update time of this resource.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. State of the resource.
    #[prost(enumeration = "cluster::State", tag = "6")]
    pub state: i32,
    /// Output only. True if the cluster is a management cluster; false otherwise.
    /// There can only be one management cluster in a private cloud
    /// and it has to be the first one.
    #[prost(bool, tag = "7")]
    pub management: bool,
    /// Output only. System-generated unique identifier for the resource.
    #[prost(string, tag = "14")]
    pub uid: ::prost::alloc::string::String,
    /// Required. The map of cluster node types in this cluster, where the key is
    /// canonical identifier of the node type (corresponds to the `NodeType`).
    #[prost(btree_map = "string, message", tag = "16")]
    pub node_type_configs: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        NodeTypeConfig,
    >,
    /// Optional. Configuration of a stretched cluster. Required for clusters that
    /// belong to a STRETCHED private cloud.
    #[prost(message, optional, tag = "17")]
    pub stretched_cluster_config: ::core::option::Option<StretchedClusterConfig>,
}
/// Nested message and enum types in `Cluster`.
pub mod cluster {
    /// Enum State defines possible states of private cloud clusters.
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
        /// The default value. This value should never be used.
        Unspecified = 0,
        /// The Cluster is operational and can be used by the user.
        Active = 1,
        /// The Cluster is being deployed.
        Creating = 2,
        /// Adding or removing of a node to the cluster, any other cluster specific
        /// updates.
        Updating = 3,
        /// The Cluster is being deleted.
        Deleting = 4,
        /// The Cluster is undergoing maintenance, for example: a failed node is
        /// getting replaced.
        Repairing = 5,
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
                State::Creating => "CREATING",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Repairing => "REPAIRING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "CREATING" => Some(Self::Creating),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "REPAIRING" => Some(Self::Repairing),
                _ => None,
            }
        }
    }
}
/// Node in a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    /// Output only. The resource name of this node.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// projects/my-project/locations/us-central1-a/privateClouds/my-cloud/clusters/my-cluster/nodes/my-node
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Fully qualified domain name of the node.
    #[prost(string, tag = "2")]
    pub fqdn: ::prost::alloc::string::String,
    /// Output only. Internal IP address of the node.
    #[prost(string, tag = "3")]
    pub internal_ip: ::prost::alloc::string::String,
    /// Output only. The canonical identifier of the node type (corresponds to the
    /// `NodeType`).
    /// For example: standard-72.
    #[prost(string, tag = "4")]
    pub node_type_id: ::prost::alloc::string::String,
    /// Output only. The version number of the VMware ESXi
    /// management component in this cluster.
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    /// Output only. Customized number of cores
    #[prost(int64, tag = "6")]
    pub custom_core_count: i64,
    /// Output only. The state of the appliance.
    #[prost(enumeration = "node::State", tag = "7")]
    pub state: i32,
}
/// Nested message and enum types in `Node`.
pub mod node {
    /// Enum State defines possible states of a node in a cluster.
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
        /// The default value. This value should never be used.
        Unspecified = 0,
        /// Node is operational and can be used by the user.
        Active = 1,
        /// Node is being provisioned.
        Creating = 2,
        /// Node is in a failed state.
        Failed = 3,
        /// Node is undergoing maintenance, e.g.: during private cloud upgrade.
        Upgrading = 4,
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
                State::Creating => "CREATING",
                State::Failed => "FAILED",
                State::Upgrading => "UPGRADING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "CREATING" => Some(Self::Creating),
                "FAILED" => Some(Self::Failed),
                "UPGRADING" => Some(Self::Upgrading),
                _ => None,
            }
        }
    }
}
/// Represents an allocated external IP address and its corresponding internal IP
/// address in a private cloud.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAddress {
    /// Output only. The resource name of this external IP address.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/externalAddresses/my-address`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation time of this resource.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update time of this resource.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The internal IP address of a workload VM.
    #[prost(string, tag = "6")]
    pub internal_ip: ::prost::alloc::string::String,
    /// Output only. The external IP address of a workload VM.
    #[prost(string, tag = "7")]
    pub external_ip: ::prost::alloc::string::String,
    /// Output only. The state of the resource.
    #[prost(enumeration = "external_address::State", tag = "8")]
    pub state: i32,
    /// Output only. System-generated unique identifier for the resource.
    #[prost(string, tag = "9")]
    pub uid: ::prost::alloc::string::String,
    /// User-provided description for this resource.
    #[prost(string, tag = "11")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ExternalAddress`.
pub mod external_address {
    /// Enum State defines possible states of external addresses.
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
        /// The default value. This value should never be used.
        Unspecified = 0,
        /// The address is ready.
        Active = 1,
        /// The address is being created.
        Creating = 2,
        /// The address is being updated.
        Updating = 3,
        /// The address is being deleted.
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
                State::Active => "ACTIVE",
                State::Creating => "CREATING",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "CREATING" => Some(Self::Creating),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
}
/// Subnet in a private cloud. Either `management` subnets (such as vMotion) that
/// are read-only, or `userDefined`, which can also be updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subnet {
    /// Output only. The resource name of this subnet.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/subnets/my-subnet`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The IP address range of the subnet in CIDR format '10.0.0.0/24'.
    #[prost(string, tag = "7")]
    pub ip_cidr_range: ::prost::alloc::string::String,
    /// The IP address of the gateway of this subnet.
    /// Must fall within the IP prefix defined above.
    #[prost(string, tag = "8")]
    pub gateway_ip: ::prost::alloc::string::String,
    /// Output only. The type of the subnet. For example "management" or
    /// "userDefined".
    #[prost(string, tag = "11")]
    pub r#type: ::prost::alloc::string::String,
    /// Output only. The state of the resource.
    #[prost(enumeration = "subnet::State", tag = "13")]
    pub state: i32,
    /// Output only. VLAN ID of the VLAN on which the subnet is configured
    #[prost(int32, tag = "16")]
    pub vlan_id: i32,
}
/// Nested message and enum types in `Subnet`.
pub mod subnet {
    /// Defines possible states of subnets.
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
        /// The default value. This value should never be used.
        Unspecified = 0,
        /// The subnet is ready.
        Active = 1,
        /// The subnet is being created.
        Creating = 2,
        /// The subnet is being updated.
        Updating = 3,
        /// The subnet is being deleted.
        Deleting = 4,
        /// Changes requested in the last operation are being propagated.
        Reconciling = 5,
        /// Last operation on the subnet did not succeed. Subnet's payload is
        /// reverted back to its most recent working state.
        Failed = 6,
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
                State::Creating => "CREATING",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Reconciling => "RECONCILING",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "CREATING" => Some(Self::Creating),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "RECONCILING" => Some(Self::Reconciling),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// External access firewall rules for filtering incoming traffic destined to
/// `ExternalAddress` resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAccessRule {
    /// Output only. The resource name of this external access rule.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/networkPolicies/my-policy/externalAccessRules/my-rule`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation time of this resource.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update time of this resource.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-provided description for this external access rule.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// External access rule priority, which determines the external access rule to
    /// use when multiple rules apply. If multiple rules have the same priority,
    /// their ordering is non-deterministic. If specific ordering is required,
    /// assign unique priorities to enforce such ordering. The external access rule
    /// priority is an integer from 100 to 4096, both inclusive. Lower integers
    /// indicate higher precedence. For example, a rule with priority `100` has
    /// higher precedence than a rule with priority `101`.
    #[prost(int32, tag = "6")]
    pub priority: i32,
    /// The action that the external access rule performs.
    #[prost(enumeration = "external_access_rule::Action", tag = "7")]
    pub action: i32,
    /// The IP protocol to which the external access rule applies. This value can
    /// be one of the following three protocol strings (not case-sensitive):
    /// `tcp`, `udp`, or `icmp`.
    #[prost(string, tag = "8")]
    pub ip_protocol: ::prost::alloc::string::String,
    /// If source ranges are specified, the external access rule applies only to
    /// traffic that has a source IP address in these ranges. These ranges can
    /// either be expressed in the CIDR format or as an IP address. As only inbound
    /// rules are supported, `ExternalAddress` resources cannot be the source IP
    /// addresses of an external access rule. To match all source addresses,
    /// specify `0.0.0.0/0`.
    #[prost(message, repeated, tag = "9")]
    pub source_ip_ranges: ::prost::alloc::vec::Vec<external_access_rule::IpRange>,
    /// A list of source ports to which the external access rule applies. This
    /// field is only applicable for the UDP or TCP protocol.
    /// Each entry must be either an integer or a range. For example: `\["22"\]`,
    /// `\["80","443"\]`, or `\["12345-12349"\]`. To match all source ports, specify
    /// `\["0-65535"\]`.
    #[prost(string, repeated, tag = "10")]
    pub source_ports: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If destination ranges are specified, the external access rule applies only
    /// to the traffic that has a destination IP address in these ranges. The
    /// specified IP addresses must have reserved external IP addresses in the
    /// scope of the parent network policy. To match all external IP addresses in
    /// the scope of the parent network policy, specify `0.0.0.0/0`. To match a
    /// specific external IP address, specify it using the
    /// `IpRange.external_address` property.
    #[prost(message, repeated, tag = "11")]
    pub destination_ip_ranges: ::prost::alloc::vec::Vec<external_access_rule::IpRange>,
    /// A list of destination ports to which the external access rule applies. This
    /// field is only applicable for the UDP or TCP protocol.
    /// Each entry must be either an integer or a range. For example: `\["22"\]`,
    /// `\["80","443"\]`, or `\["12345-12349"\]`. To match all destination ports,
    /// specify `\["0-65535"\]`.
    #[prost(string, repeated, tag = "12")]
    pub destination_ports: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The state of the resource.
    #[prost(enumeration = "external_access_rule::State", tag = "13")]
    pub state: i32,
    /// Output only. System-generated unique identifier for the resource.
    #[prost(string, tag = "14")]
    pub uid: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ExternalAccessRule`.
pub mod external_access_rule {
    /// An IP range provided in any one of the supported formats.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IpRange {
        #[prost(oneof = "ip_range::IpRange", tags = "1, 2, 3")]
        pub ip_range: ::core::option::Option<ip_range::IpRange>,
    }
    /// Nested message and enum types in `IpRange`.
    pub mod ip_range {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum IpRange {
            /// A single IP address. For example: `10.0.0.5`.
            #[prost(string, tag = "1")]
            IpAddress(::prost::alloc::string::String),
            /// An IP address range in the CIDR format. For example: `10.0.0.0/24`.
            #[prost(string, tag = "2")]
            IpAddressRange(::prost::alloc::string::String),
            /// The name of an `ExternalAddress` resource. The external address must
            /// have been reserved in the scope of this external access rule's parent
            /// network policy.  Provide the external address name in the form of
            /// `projects/{project}/locations/{location}/privateClouds/{private_cloud}/externalAddresses/{external_address}`.
            /// For example:
            /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/externalAddresses/my-address`.
            #[prost(string, tag = "3")]
            ExternalAddress(::prost::alloc::string::String),
        }
    }
    /// Action determines whether the external access rule permits or blocks
    /// traffic, subject to the other components of the rule matching the traffic.
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
        /// Defaults to allow.
        Unspecified = 0,
        /// Allows connections that match the other specified components.
        Allow = 1,
        /// Blocks connections that match the other specified components.
        Deny = 2,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::Unspecified => "ACTION_UNSPECIFIED",
                Action::Allow => "ALLOW",
                Action::Deny => "DENY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "ALLOW" => Some(Self::Allow),
                "DENY" => Some(Self::Deny),
                _ => None,
            }
        }
    }
    /// Defines possible states of external access firewall rules.
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
        /// The rule is ready.
        Active = 1,
        /// The rule is being created.
        Creating = 2,
        /// The rule is being updated.
        Updating = 3,
        /// The rule is being deleted.
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
                State::Active => "ACTIVE",
                State::Creating => "CREATING",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "CREATING" => Some(Self::Creating),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
}
/// Logging server to receive vCenter or ESXi logs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggingServer {
    /// Output only. The resource name of this logging server.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/loggingServers/my-logging-server`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation time of this resource.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update time of this resource.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Fully-qualified domain name (FQDN) or IP Address of the logging
    /// server.
    #[prost(string, tag = "5")]
    pub hostname: ::prost::alloc::string::String,
    /// Required. Port number at which the logging server receives logs.
    #[prost(int32, tag = "7")]
    pub port: i32,
    /// Required. Protocol used by vCenter to send logs to a logging server.
    #[prost(enumeration = "logging_server::Protocol", tag = "6")]
    pub protocol: i32,
    /// Required. The type of component that produces logs that will be forwarded
    /// to this logging server.
    #[prost(enumeration = "logging_server::SourceType", tag = "10")]
    pub source_type: i32,
    /// Output only. System-generated unique identifier for the resource.
    #[prost(string, tag = "8")]
    pub uid: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LoggingServer`.
pub mod logging_server {
    /// Defines possible protocols used to send logs to
    /// a logging server.
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
        /// Unspecified communications protocol. This is the default value.
        Unspecified = 0,
        /// UDP
        Udp = 1,
        /// TCP
        Tcp = 2,
    }
    impl Protocol {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Protocol::Unspecified => "PROTOCOL_UNSPECIFIED",
                Protocol::Udp => "UDP",
                Protocol::Tcp => "TCP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROTOCOL_UNSPECIFIED" => Some(Self::Unspecified),
                "UDP" => Some(Self::Udp),
                "TCP" => Some(Self::Tcp),
                _ => None,
            }
        }
    }
    /// Defines possible types of component that produces logs.
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
    pub enum SourceType {
        /// The default value. This value should never be used.
        Unspecified = 0,
        /// Logs produced by ESXI hosts
        Esxi = 1,
        /// Logs produced by vCenter server
        Vcsa = 2,
    }
    impl SourceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SourceType::Unspecified => "SOURCE_TYPE_UNSPECIFIED",
                SourceType::Esxi => "ESXI",
                SourceType::Vcsa => "VCSA",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SOURCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ESXI" => Some(Self::Esxi),
                "VCSA" => Some(Self::Vcsa),
                _ => None,
            }
        }
    }
}
/// Describes node type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeType {
    /// Output only. The resource name of this node type.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-proj/locations/us-central1-a/nodeTypes/standard-72`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The canonical identifier of the node type
    /// (corresponds to the `NodeType`). For example: standard-72.
    #[prost(string, tag = "2")]
    pub node_type_id: ::prost::alloc::string::String,
    /// Output only. The friendly name for this node type.
    /// For example: ve1-standard-72
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The total number of virtual CPUs in a single node.
    #[prost(int32, tag = "4")]
    pub virtual_cpu_count: i32,
    /// Output only. The total number of CPU cores in a single node.
    #[prost(int32, tag = "5")]
    pub total_core_count: i32,
    /// Output only. The amount of physical memory available, defined in GB.
    #[prost(int32, tag = "7")]
    pub memory_gb: i32,
    /// Output only. The amount of storage available, defined in GB.
    #[prost(int32, tag = "8")]
    pub disk_size_gb: i32,
    /// Output only. List of possible values of custom core count.
    #[prost(int32, repeated, packed = "false", tag = "11")]
    pub available_custom_core_counts: ::prost::alloc::vec::Vec<i32>,
    /// Output only. The type of the resource.
    #[prost(enumeration = "node_type::Kind", tag = "12")]
    pub kind: i32,
    /// Output only. Families of the node type.
    /// For node types to be in the same cluster
    /// they must share at least one element in the `families`.
    #[prost(string, repeated, tag = "13")]
    pub families: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Capabilities of this node type.
    #[prost(
        enumeration = "node_type::Capability",
        repeated,
        packed = "false",
        tag = "14"
    )]
    pub capabilities: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `NodeType`.
pub mod node_type {
    /// Enum Kind defines possible types of a NodeType.
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
    pub enum Kind {
        /// The default value. This value should never be used.
        Unspecified = 0,
        /// Standard HCI node.
        Standard = 1,
        /// Storage only Node.
        StorageOnly = 2,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::Unspecified => "KIND_UNSPECIFIED",
                Kind::Standard => "STANDARD",
                Kind::StorageOnly => "STORAGE_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "KIND_UNSPECIFIED" => Some(Self::Unspecified),
                "STANDARD" => Some(Self::Standard),
                "STORAGE_ONLY" => Some(Self::StorageOnly),
                _ => None,
            }
        }
    }
    /// Capability of a node type.
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
    pub enum Capability {
        /// The default value. This value is used if the capability is omitted or
        /// unknown.
        Unspecified = 0,
        /// This node type supports stretch clusters.
        StretchedClusters = 1,
    }
    impl Capability {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Capability::Unspecified => "CAPABILITY_UNSPECIFIED",
                Capability::StretchedClusters => "STRETCHED_CLUSTERS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CAPABILITY_UNSPECIFIED" => Some(Self::Unspecified),
                "STRETCHED_CLUSTERS" => Some(Self::StretchedClusters),
                _ => None,
            }
        }
    }
}
/// Credentials for a private cloud.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Credentials {
    /// Initial username.
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    /// Initial password.
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
/// HCX activation key. A default key is created during
/// private cloud provisioning, but this behavior is subject to change
/// and you should always verify active keys.
/// Use
/// [VmwareEngine.ListHcxActivationKeys][google.cloud.vmwareengine.v1.VmwareEngine.ListHcxActivationKeys]
/// to retrieve existing keys and
/// [VmwareEngine.CreateHcxActivationKey][google.cloud.vmwareengine.v1.VmwareEngine.CreateHcxActivationKey]
/// to create new ones.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HcxActivationKey {
    /// Output only. The resource name of this HcxActivationKey.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/privateClouds/my-cloud/hcxActivationKeys/my-key`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation time of HCX activation key.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. State of HCX activation key.
    #[prost(enumeration = "hcx_activation_key::State", tag = "3")]
    pub state: i32,
    /// Output only. HCX activation key.
    #[prost(string, tag = "4")]
    pub activation_key: ::prost::alloc::string::String,
    /// Output only. System-generated unique identifier for the resource.
    #[prost(string, tag = "5")]
    pub uid: ::prost::alloc::string::String,
}
/// Nested message and enum types in `HcxActivationKey`.
pub mod hcx_activation_key {
    /// State of HCX activation key
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
        /// Unspecified state.
        Unspecified = 0,
        /// State of a newly generated activation key.
        Available = 1,
        /// State of key when it has been used to activate HCX appliance.
        Consumed = 2,
        /// State of key when it is being created.
        Creating = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Available => "AVAILABLE",
                State::Consumed => "CONSUMED",
                State::Creating => "CREATING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "AVAILABLE" => Some(Self::Available),
                "CONSUMED" => Some(Self::Consumed),
                "CREATING" => Some(Self::Creating),
                _ => None,
            }
        }
    }
}
/// Details about a HCX Cloud Manager appliance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hcx {
    /// Internal IP address of the appliance.
    #[prost(string, tag = "2")]
    pub internal_ip: ::prost::alloc::string::String,
    /// Version of the appliance.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// Output only. The state of the appliance.
    #[prost(enumeration = "hcx::State", tag = "5")]
    pub state: i32,
    /// Fully qualified domain name of the appliance.
    #[prost(string, tag = "6")]
    pub fqdn: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Hcx`.
pub mod hcx {
    /// State of the appliance
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
        /// Unspecified appliance state. This is the default value.
        Unspecified = 0,
        /// The appliance is operational and can be used.
        Active = 1,
        /// The appliance is being deployed.
        Creating = 2,
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
                State::Creating => "CREATING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "CREATING" => Some(Self::Creating),
                _ => None,
            }
        }
    }
}
/// Details about a NSX Manager appliance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nsx {
    /// Internal IP address of the appliance.
    #[prost(string, tag = "2")]
    pub internal_ip: ::prost::alloc::string::String,
    /// Version of the appliance.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// Output only. The state of the appliance.
    #[prost(enumeration = "nsx::State", tag = "5")]
    pub state: i32,
    /// Fully qualified domain name of the appliance.
    #[prost(string, tag = "6")]
    pub fqdn: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Nsx`.
pub mod nsx {
    /// State of the appliance
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
        /// Unspecified appliance state. This is the default value.
        Unspecified = 0,
        /// The appliance is operational and can be used.
        Active = 1,
        /// The appliance is being deployed.
        Creating = 2,
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
                State::Creating => "CREATING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "CREATING" => Some(Self::Creating),
                _ => None,
            }
        }
    }
}
/// Details about a vCenter Server management appliance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vcenter {
    /// Internal IP address of the appliance.
    #[prost(string, tag = "2")]
    pub internal_ip: ::prost::alloc::string::String,
    /// Version of the appliance.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// Output only. The state of the appliance.
    #[prost(enumeration = "vcenter::State", tag = "5")]
    pub state: i32,
    /// Fully qualified domain name of the appliance.
    #[prost(string, tag = "6")]
    pub fqdn: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Vcenter`.
pub mod vcenter {
    /// State of the appliance
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
        /// Unspecified appliance state. This is the default value.
        Unspecified = 0,
        /// The appliance is operational and can be used.
        Active = 1,
        /// The appliance is being deployed.
        Creating = 2,
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
                State::Creating => "CREATING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "CREATING" => Some(Self::Creating),
                _ => None,
            }
        }
    }
}
/// DNS forwarding config.
/// This config defines a list of domain to name server mappings,
/// and is attached to the private cloud for custom domain resolution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsForwarding {
    /// Output only. The resource name of this DNS profile.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/dnsForwarding`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation time of this resource.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update time of this resource.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. List of domain mappings to configure
    #[prost(message, repeated, tag = "4")]
    pub forwarding_rules: ::prost::alloc::vec::Vec<dns_forwarding::ForwardingRule>,
}
/// Nested message and enum types in `DnsForwarding`.
pub mod dns_forwarding {
    /// A forwarding rule is a mapping of a `domain` to `name_servers`.
    /// This mapping allows VMware Engine to resolve domains for attached private
    /// clouds by forwarding DNS requests for a given domain to the specified
    /// nameservers.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ForwardingRule {
        /// Required. Domain used to resolve a `name_servers` list.
        #[prost(string, tag = "1")]
        pub domain: ::prost::alloc::string::String,
        /// Required. List of DNS servers to use for domain resolution
        #[prost(string, repeated, tag = "2")]
        pub name_servers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Details of a network peering.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPeering {
    /// Output only. The resource name of the network peering. NetworkPeering is a
    /// global resource and location can only be global. Resource names are
    /// scheme-less URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/global/networkPeerings/my-peering`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation time of this resource.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update time of this resource.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The relative resource name of the network to peer with
    /// a standard VMware Engine network. The provided network can be a
    /// consumer VPC network or another standard VMware Engine network. If the
    /// `peer_network_type` is VMWARE_ENGINE_NETWORK, specify the name in the form:
    /// `projects/{project}/locations/global/vmwareEngineNetworks/{vmware_engine_network_id}`.
    /// Otherwise specify the name in the form:
    /// `projects/{project}/global/networks/{network_id}`, where
    /// `{project}` can either be a project number or a project ID.
    #[prost(string, tag = "5")]
    pub peer_network: ::prost::alloc::string::String,
    /// Optional. True if custom routes are exported to the peered network;
    /// false otherwise. The default value is true.
    #[prost(bool, optional, tag = "8")]
    pub export_custom_routes: ::core::option::Option<bool>,
    /// Optional. True if custom routes are imported from the peered network;
    /// false otherwise. The default value is true.
    #[prost(bool, optional, tag = "9")]
    pub import_custom_routes: ::core::option::Option<bool>,
    /// Optional. True if full mesh connectivity is created and managed
    /// automatically between peered networks; false otherwise. Currently this
    /// field is always true because Google Compute Engine automatically creates
    /// and manages subnetwork routes between two VPC networks when peering state
    /// is 'ACTIVE'.
    #[prost(bool, optional, tag = "10")]
    pub exchange_subnet_routes: ::core::option::Option<bool>,
    /// Optional. True if all subnet routes with a public IP address range are
    /// exported; false otherwise. The default value is true. IPv4 special-use
    /// ranges (<https://en.wikipedia.org/wiki/IPv4#Special_addresses>) are always
    /// exported to peers and are not controlled by this field.
    #[prost(bool, optional, tag = "11")]
    pub export_custom_routes_with_public_ip: ::core::option::Option<bool>,
    /// Optional. True if all subnet routes with public IP address range are
    /// imported; false otherwise. The default value is true. IPv4 special-use
    /// ranges (<https://en.wikipedia.org/wiki/IPv4#Special_addresses>) are always
    /// imported to peers and are not controlled by this field.
    #[prost(bool, optional, tag = "12")]
    pub import_custom_routes_with_public_ip: ::core::option::Option<bool>,
    /// Output only. State of the network peering. This field
    /// has a value of 'ACTIVE' when there's a matching configuration in the peer
    /// network. New values may be added to this enum when appropriate.
    #[prost(enumeration = "network_peering::State", tag = "13")]
    pub state: i32,
    /// Output only. Output Only. Details about the current state of the network
    /// peering.
    #[prost(string, tag = "7")]
    pub state_details: ::prost::alloc::string::String,
    /// Optional. Maximum transmission unit (MTU) in bytes.
    /// The default value is `1500`. If a value of `0` is provided for this field,
    /// VMware Engine uses the default value instead.
    #[prost(int32, tag = "14")]
    pub peer_mtu: i32,
    /// Required. The type of the network to peer with the VMware Engine network.
    #[prost(enumeration = "network_peering::PeerNetworkType", tag = "16")]
    pub peer_network_type: i32,
    /// Output only. System-generated unique identifier for the resource.
    #[prost(string, tag = "17")]
    pub uid: ::prost::alloc::string::String,
    /// Required. The relative resource name of the VMware Engine network.
    /// Specify the name in the following form:
    /// `projects/{project}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}`
    /// where `{project}` can either be a project number or a project ID.
    #[prost(string, tag = "20")]
    pub vmware_engine_network: ::prost::alloc::string::String,
    /// Optional. User-provided description for this network peering.
    #[prost(string, tag = "21")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `NetworkPeering`.
pub mod network_peering {
    /// Possible states of a network peering.
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
        /// Unspecified network peering state. This is the default value.
        Unspecified = 0,
        /// The peering is not active.
        Inactive = 1,
        /// The peering is active.
        Active = 2,
        /// The peering is being created.
        Creating = 3,
        /// The peering is being deleted.
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
                State::Inactive => "INACTIVE",
                State::Active => "ACTIVE",
                State::Creating => "CREATING",
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "INACTIVE" => Some(Self::Inactive),
                "ACTIVE" => Some(Self::Active),
                "CREATING" => Some(Self::Creating),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
    /// Type or purpose of the network peering connection.
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
    pub enum PeerNetworkType {
        /// Unspecified
        Unspecified = 0,
        /// Peering connection used for connecting to another VPC network established
        /// by the same user. For example, a peering connection to another VPC
        /// network in the same project or to an on-premises network.
        Standard = 1,
        /// Peering connection used for connecting to another VMware Engine network.
        VmwareEngineNetwork = 2,
        /// Peering connection used for establishing [private services
        /// access](<https://cloud.google.com/vpc/docs/private-services-access>).
        PrivateServicesAccess = 3,
        /// Peering connection used for connecting to NetApp Cloud Volumes.
        NetappCloudVolumes = 4,
        /// Peering connection used for connecting to third-party services. Most
        /// third-party services require manual setup of reverse peering on the VPC
        /// network associated with the third-party service.
        ThirdPartyService = 5,
        /// Peering connection used for connecting to Dell PowerScale Filers
        DellPowerscale = 6,
    }
    impl PeerNetworkType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PeerNetworkType::Unspecified => "PEER_NETWORK_TYPE_UNSPECIFIED",
                PeerNetworkType::Standard => "STANDARD",
                PeerNetworkType::VmwareEngineNetwork => "VMWARE_ENGINE_NETWORK",
                PeerNetworkType::PrivateServicesAccess => "PRIVATE_SERVICES_ACCESS",
                PeerNetworkType::NetappCloudVolumes => "NETAPP_CLOUD_VOLUMES",
                PeerNetworkType::ThirdPartyService => "THIRD_PARTY_SERVICE",
                PeerNetworkType::DellPowerscale => "DELL_POWERSCALE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PEER_NETWORK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "STANDARD" => Some(Self::Standard),
                "VMWARE_ENGINE_NETWORK" => Some(Self::VmwareEngineNetwork),
                "PRIVATE_SERVICES_ACCESS" => Some(Self::PrivateServicesAccess),
                "NETAPP_CLOUD_VOLUMES" => Some(Self::NetappCloudVolumes),
                "THIRD_PARTY_SERVICE" => Some(Self::ThirdPartyService),
                "DELL_POWERSCALE" => Some(Self::DellPowerscale),
                _ => None,
            }
        }
    }
}
/// Exchanged network peering route.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeeringRoute {
    /// Output only. Destination range of the peering route in CIDR notation.
    #[prost(string, tag = "1")]
    pub dest_range: ::prost::alloc::string::String,
    /// Output only. Type of the route in the peer VPC network.
    #[prost(enumeration = "peering_route::Type", tag = "2")]
    pub r#type: i32,
    /// Output only. Region containing the next hop of the peering route. This
    /// field only applies to dynamic routes in the peer VPC network.
    #[prost(string, tag = "3")]
    pub next_hop_region: ::prost::alloc::string::String,
    /// Output only. The priority of the peering route.
    #[prost(int64, tag = "4")]
    pub priority: i64,
    /// Output only. True if the peering route has been imported from a peered
    /// VPC network; false otherwise. The import happens if the field
    /// `NetworkPeering.importCustomRoutes` is true for this network,
    /// `NetworkPeering.exportCustomRoutes` is true for the peer VPC network, and
    /// the import does not result in a route conflict.
    #[prost(bool, tag = "5")]
    pub imported: bool,
    /// Output only. Direction of the routes exchanged with the peer network, from
    /// the VMware Engine network perspective:
    ///
    /// * Routes of direction `INCOMING` are imported from the peer network.
    /// * Routes of direction `OUTGOING` are exported from the intranet VPC network
    /// of the VMware Engine network.
    #[prost(enumeration = "peering_route::Direction", tag = "6")]
    pub direction: i32,
}
/// Nested message and enum types in `PeeringRoute`.
pub mod peering_route {
    /// The type of the peering route.
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
        /// Unspecified peering route type. This is the default value.
        Unspecified = 0,
        /// Dynamic routes in the peer network.
        DynamicPeeringRoute = 1,
        /// Static routes in the peer network.
        StaticPeeringRoute = 2,
        /// Created, updated, and removed automatically by Google Cloud when subnets
        /// are created, modified, or deleted in the peer network.
        SubnetPeeringRoute = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::DynamicPeeringRoute => "DYNAMIC_PEERING_ROUTE",
                Type::StaticPeeringRoute => "STATIC_PEERING_ROUTE",
                Type::SubnetPeeringRoute => "SUBNET_PEERING_ROUTE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DYNAMIC_PEERING_ROUTE" => Some(Self::DynamicPeeringRoute),
                "STATIC_PEERING_ROUTE" => Some(Self::StaticPeeringRoute),
                "SUBNET_PEERING_ROUTE" => Some(Self::SubnetPeeringRoute),
                _ => None,
            }
        }
    }
    /// The direction of the exchanged routes.
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
    pub enum Direction {
        /// Unspecified exchanged routes direction. This is default.
        Unspecified = 0,
        /// Routes imported from the peer network.
        Incoming = 1,
        /// Routes exported to the peer network.
        Outgoing = 2,
    }
    impl Direction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Direction::Unspecified => "DIRECTION_UNSPECIFIED",
                Direction::Incoming => "INCOMING",
                Direction::Outgoing => "OUTGOING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
                "INCOMING" => Some(Self::Incoming),
                "OUTGOING" => Some(Self::Outgoing),
                _ => None,
            }
        }
    }
}
/// Represents a network policy resource. Network policies are regional
/// resources. You can use a network policy to enable or disable internet access
/// and external IP access. Network policies are associated with a VMware Engine
/// network, which might span across regions. For a given region, a network
/// policy applies to all private clouds in the VMware Engine network associated
/// with the policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPolicy {
    /// Output only. The resource name of this network policy.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/networkPolicies/my-network-policy`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation time of this resource.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update time of this resource.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Network service that allows VMware workloads to access the internet.
    #[prost(message, optional, tag = "6")]
    pub internet_access: ::core::option::Option<network_policy::NetworkService>,
    /// Network service that allows External IP addresses to be assigned to VMware
    /// workloads. This service can only be enabled when `internet_access` is also
    /// enabled.
    #[prost(message, optional, tag = "7")]
    pub external_ip: ::core::option::Option<network_policy::NetworkService>,
    /// Required. IP address range in CIDR notation used to create internet access
    /// and external IP access. An RFC 1918 CIDR block, with a "/26" prefix, is
    /// required. The range cannot overlap with any prefixes either in the consumer
    /// VPC network or in use by the private clouds attached to that VPC network.
    #[prost(string, tag = "9")]
    pub edge_services_cidr: ::prost::alloc::string::String,
    /// Output only. System-generated unique identifier for the resource.
    #[prost(string, tag = "10")]
    pub uid: ::prost::alloc::string::String,
    /// Optional. The relative resource name of the VMware Engine network.
    /// Specify the name in the following form:
    /// `projects/{project}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}`
    /// where `{project}` can either be a project number or a project ID.
    #[prost(string, tag = "12")]
    pub vmware_engine_network: ::prost::alloc::string::String,
    /// Optional. User-provided description for this network policy.
    #[prost(string, tag = "13")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The canonical name of the VMware Engine network in the form:
    /// `projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}`
    #[prost(string, tag = "14")]
    pub vmware_engine_network_canonical: ::prost::alloc::string::String,
}
/// Nested message and enum types in `NetworkPolicy`.
pub mod network_policy {
    /// Represents a network service that is managed by a `NetworkPolicy` resource.
    /// A network service provides a way to control an aspect of external access to
    /// VMware workloads. For example, whether the VMware workloads in the
    /// private clouds governed by a network policy can access or be accessed from
    /// the internet.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetworkService {
        /// True if the service is enabled; false otherwise.
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        /// Output only. State of the service. New values may be added to this enum
        /// when appropriate.
        #[prost(enumeration = "network_service::State", tag = "2")]
        pub state: i32,
    }
    /// Nested message and enum types in `NetworkService`.
    pub mod network_service {
        /// Enum State defines possible states of a network policy controlled
        /// service.
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
            /// Unspecified service state. This is the default value.
            Unspecified = 0,
            /// Service is not provisioned.
            Unprovisioned = 1,
            /// Service is in the process of being provisioned/deprovisioned.
            Reconciling = 2,
            /// Service is active.
            Active = 3,
        }
        impl State {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    State::Unspecified => "STATE_UNSPECIFIED",
                    State::Unprovisioned => "UNPROVISIONED",
                    State::Reconciling => "RECONCILING",
                    State::Active => "ACTIVE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                    "UNPROVISIONED" => Some(Self::Unprovisioned),
                    "RECONCILING" => Some(Self::Reconciling),
                    "ACTIVE" => Some(Self::Active),
                    _ => None,
                }
            }
        }
    }
}
/// Represents a binding between a network and the management DNS zone.
/// A management DNS zone is the Cloud DNS cross-project binding zone that
/// VMware Engine creates for each private cloud. It contains FQDNs and
/// corresponding IP addresses for the private cloud's ESXi hosts and management
/// VM appliances like vCenter and NSX Manager.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagementDnsZoneBinding {
    /// Output only. The resource name of this binding.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/managementDnsZoneBindings/my-management-dns-zone-binding`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation time of this resource.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update time of this resource.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The state of the resource.
    #[prost(enumeration = "management_dns_zone_binding::State", tag = "8")]
    pub state: i32,
    /// User-provided description for this resource.
    #[prost(string, tag = "13")]
    pub description: ::prost::alloc::string::String,
    /// Output only. System-generated unique identifier for the resource.
    #[prost(string, tag = "9")]
    pub uid: ::prost::alloc::string::String,
    /// Required. The relative resource name of the network to bind to the
    /// management DNS zone. This network can be a consumer VPC network or a
    /// VMware engine network.
    #[prost(oneof = "management_dns_zone_binding::BindNetwork", tags = "14, 15")]
    pub bind_network: ::core::option::Option<management_dns_zone_binding::BindNetwork>,
}
/// Nested message and enum types in `ManagementDnsZoneBinding`.
pub mod management_dns_zone_binding {
    /// Enum State defines possible states of binding between the consumer VPC
    /// network and the management DNS zone.
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
        /// The default value. This value should never be used.
        Unspecified = 0,
        /// The binding is ready.
        Active = 1,
        /// The binding is being created.
        Creating = 2,
        /// The binding is being updated.
        Updating = 3,
        /// The binding is being deleted.
        Deleting = 4,
        /// The binding has failed.
        Failed = 5,
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
                State::Creating => "CREATING",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "CREATING" => Some(Self::Creating),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// Required. The relative resource name of the network to bind to the
    /// management DNS zone. This network can be a consumer VPC network or a
    /// VMware engine network.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BindNetwork {
        /// Network to bind is a standard consumer VPC.
        /// Specify the name in the following form for consumer
        /// VPC network: `projects/{project}/global/networks/{network_id}`.
        /// `{project}` can either be a project number or a project ID.
        #[prost(string, tag = "14")]
        VpcNetwork(::prost::alloc::string::String),
        /// Network to bind is a VMware Engine network.
        /// Specify the name in the following form for VMware engine network:
        /// `projects/{project}/locations/global/vmwareEngineNetworks/{vmware_engine_network_id}`.
        /// `{project}` can either be a project number or a project ID.
        #[prost(string, tag = "15")]
        VmwareEngineNetwork(::prost::alloc::string::String),
    }
}
/// VMware Engine network resource that provides connectivity for VMware Engine
/// private clouds.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmwareEngineNetwork {
    /// Output only. The resource name of the VMware Engine network.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/global/vmwareEngineNetworks/my-network`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation time of this resource.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update time of this resource.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-provided description for this VMware Engine network.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Output only. VMware Engine service VPC networks that provide connectivity
    /// from a private cloud to customer projects, the internet, and other Google
    /// Cloud services.
    #[prost(message, repeated, tag = "6")]
    pub vpc_networks: ::prost::alloc::vec::Vec<vmware_engine_network::VpcNetwork>,
    /// Output only. State of the VMware Engine network.
    #[prost(enumeration = "vmware_engine_network::State", tag = "7")]
    pub state: i32,
    /// Required. VMware Engine network type.
    #[prost(enumeration = "vmware_engine_network::Type", tag = "8")]
    pub r#type: i32,
    /// Output only. System-generated unique identifier for the resource.
    #[prost(string, tag = "9")]
    pub uid: ::prost::alloc::string::String,
    /// Checksum that may be sent on update and delete requests to ensure that the
    /// user-provided value is up to date before the server processes a request.
    /// The server computes checksums based on the value of other fields in the
    /// request.
    #[prost(string, tag = "10")]
    pub etag: ::prost::alloc::string::String,
}
/// Nested message and enum types in `VmwareEngineNetwork`.
pub mod vmware_engine_network {
    /// Represents a VMware Engine VPC network that is managed by a
    /// VMware Engine network resource.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VpcNetwork {
        /// Output only. Type of VPC network (INTRANET, INTERNET, or
        /// GOOGLE_CLOUD)
        #[prost(enumeration = "vpc_network::Type", tag = "1")]
        pub r#type: i32,
        /// Output only. The relative resource name of the service VPC network this
        /// VMware Engine network is attached to. For example:
        /// `projects/123123/global/networks/my-network`
        #[prost(string, tag = "2")]
        pub network: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `VpcNetwork`.
    pub mod vpc_network {
        /// Enum Type defines possible types of a VMware Engine network controlled
        /// service.
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
            /// The default value. This value should never be used.
            Unspecified = 0,
            /// VPC network that will be peered with a consumer VPC network or the
            /// intranet VPC of another VMware Engine network. Access a private cloud
            /// through Compute Engine VMs on a peered VPC network or an on-premises
            /// resource connected to a peered consumer VPC network.
            Intranet = 1,
            /// VPC network used for internet access to and from a private cloud.
            Internet = 2,
            /// VPC network used for access to Google Cloud services like
            /// Cloud Storage.
            GoogleCloud = 3,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unspecified => "TYPE_UNSPECIFIED",
                    Type::Intranet => "INTRANET",
                    Type::Internet => "INTERNET",
                    Type::GoogleCloud => "GOOGLE_CLOUD",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "INTRANET" => Some(Self::Intranet),
                    "INTERNET" => Some(Self::Internet),
                    "GOOGLE_CLOUD" => Some(Self::GoogleCloud),
                    _ => None,
                }
            }
        }
    }
    /// Enum State defines possible states of VMware Engine network.
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
        /// The VMware Engine network is being created.
        Creating = 1,
        /// The VMware Engine network is ready.
        Active = 2,
        /// The VMware Engine network is being updated.
        Updating = 3,
        /// The VMware Engine network is being deleted.
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
    /// Enum Type defines possible types of VMware Engine network.
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
        /// The default value. This value should never be used.
        Unspecified = 0,
        /// Network type used by private clouds created in projects without a network
        /// of type `STANDARD`. This network type is no longer used for new VMware
        /// Engine private cloud deployments.
        Legacy = 1,
        /// Standard network type used for private cloud connectivity.
        Standard = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Legacy => "LEGACY",
                Type::Standard => "STANDARD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "LEGACY" => Some(Self::Legacy),
                "STANDARD" => Some(Self::Standard),
                _ => None,
            }
        }
    }
}
/// Private connection resource that provides connectivity for VMware Engine
/// private clouds.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateConnection {
    /// Output only. The resource name of the private connection.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/privateConnections/my-connection`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation time of this resource.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update time of this resource.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. User-provided description for this private connection.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Output only. State of the private connection.
    #[prost(enumeration = "private_connection::State", tag = "5")]
    pub state: i32,
    /// Required. The relative resource name of Legacy VMware Engine network.
    /// Specify the name in the following form:
    /// `projects/{project}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}`
    /// where `{project}`, `{location}` will be same as specified in private
    /// connection resource name and `{vmware_engine_network_id}` will be in the
    /// form of `{location}`-default e.g.
    /// projects/project/locations/us-central1/vmwareEngineNetworks/us-central1-default.
    #[prost(string, tag = "8")]
    pub vmware_engine_network: ::prost::alloc::string::String,
    /// Output only. The canonical name of the VMware Engine network in the form:
    /// `projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmware_engine_network_id}`
    #[prost(string, tag = "9")]
    pub vmware_engine_network_canonical: ::prost::alloc::string::String,
    /// Required. Private connection type.
    #[prost(enumeration = "private_connection::Type", tag = "10")]
    pub r#type: i32,
    /// Output only. VPC network peering id between given network VPC and
    /// VMwareEngineNetwork.
    #[prost(string, tag = "12")]
    pub peering_id: ::prost::alloc::string::String,
    /// Optional. Routing Mode.
    /// Default value is set to GLOBAL.
    /// For type = PRIVATE_SERVICE_ACCESS, this field can be set to GLOBAL or
    /// REGIONAL, for other types only GLOBAL is supported.
    #[prost(enumeration = "private_connection::RoutingMode", tag = "13")]
    pub routing_mode: i32,
    /// Output only. System-generated unique identifier for the resource.
    #[prost(string, tag = "14")]
    pub uid: ::prost::alloc::string::String,
    /// Required. Service network to create private connection.
    /// Specify the name in the following form:
    /// `projects/{project}/global/networks/{network_id}`
    /// For type = PRIVATE_SERVICE_ACCESS, this field represents servicenetworking
    /// VPC, e.g. projects/project-tp/global/networks/servicenetworking.
    /// For type = NETAPP_CLOUD_VOLUME, this field represents NetApp service VPC,
    /// e.g. projects/project-tp/global/networks/netapp-tenant-vpc.
    /// For type = DELL_POWERSCALE, this field represent Dell service VPC, e.g.
    /// projects/project-tp/global/networks/dell-tenant-vpc.
    /// For type= THIRD_PARTY_SERVICE, this field could represent a consumer VPC or
    /// any other producer VPC to which the VMware Engine Network needs to be
    /// connected, e.g. projects/project/global/networks/vpc.
    #[prost(string, tag = "16")]
    pub service_network: ::prost::alloc::string::String,
    /// Output only. Peering state between service network and VMware Engine
    /// network.
    #[prost(enumeration = "private_connection::PeeringState", tag = "17")]
    pub peering_state: i32,
}
/// Nested message and enum types in `PrivateConnection`.
pub mod private_connection {
    /// Enum State defines possible states of private connection.
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
        /// The private connection is being created.
        Creating = 1,
        /// The private connection is ready.
        Active = 2,
        /// The private connection is being updated.
        Updating = 3,
        /// The private connection is being deleted.
        Deleting = 4,
        /// The private connection is not provisioned, since no private cloud is
        /// present for which this private connection is needed.
        Unprovisioned = 5,
        /// The private connection is in failed state.
        Failed = 6,
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
                State::Unprovisioned => "UNPROVISIONED",
                State::Failed => "FAILED",
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
                "UNPROVISIONED" => Some(Self::Unprovisioned),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// Enum Type defines possible types of private connection.
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
        /// The default value. This value should never be used.
        Unspecified = 0,
        /// Connection used for establishing [private services
        /// access](<https://cloud.google.com/vpc/docs/private-services-access>).
        PrivateServiceAccess = 1,
        /// Connection used for connecting to NetApp Cloud Volumes.
        NetappCloudVolumes = 2,
        /// Connection used for connecting to Dell PowerScale.
        DellPowerscale = 3,
        /// Connection used for connecting to third-party services.
        ThirdPartyService = 4,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::PrivateServiceAccess => "PRIVATE_SERVICE_ACCESS",
                Type::NetappCloudVolumes => "NETAPP_CLOUD_VOLUMES",
                Type::DellPowerscale => "DELL_POWERSCALE",
                Type::ThirdPartyService => "THIRD_PARTY_SERVICE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PRIVATE_SERVICE_ACCESS" => Some(Self::PrivateServiceAccess),
                "NETAPP_CLOUD_VOLUMES" => Some(Self::NetappCloudVolumes),
                "DELL_POWERSCALE" => Some(Self::DellPowerscale),
                "THIRD_PARTY_SERVICE" => Some(Self::ThirdPartyService),
                _ => None,
            }
        }
    }
    /// Possible types for RoutingMode
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
    pub enum RoutingMode {
        /// The default value. This value should never be used.
        Unspecified = 0,
        /// Global Routing Mode
        Global = 1,
        /// Regional Routing Mode
        Regional = 2,
    }
    impl RoutingMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RoutingMode::Unspecified => "ROUTING_MODE_UNSPECIFIED",
                RoutingMode::Global => "GLOBAL",
                RoutingMode::Regional => "REGIONAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROUTING_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "GLOBAL" => Some(Self::Global),
                "REGIONAL" => Some(Self::Regional),
                _ => None,
            }
        }
    }
    /// Enum PeeringState defines the possible states of peering between service
    /// network and the vpc network peered to service network
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
    pub enum PeeringState {
        /// The default value. This value is used if the peering state is omitted or
        /// unknown.
        Unspecified = 0,
        /// The peering is in active state.
        PeeringActive = 1,
        /// The peering is in inactive state.
        PeeringInactive = 2,
    }
    impl PeeringState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PeeringState::Unspecified => "PEERING_STATE_UNSPECIFIED",
                PeeringState::PeeringActive => "PEERING_ACTIVE",
                PeeringState::PeeringInactive => "PEERING_INACTIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PEERING_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PEERING_ACTIVE" => Some(Self::PeeringActive),
                "PEERING_INACTIVE" => Some(Self::PeeringInactive),
                _ => None,
            }
        }
    }
}
/// VmwareEngine specific metadata for the given
/// [google.cloud.location.Location][google.cloud.location.Location]. It is
/// returned as a content of the `google.cloud.location.Location.metadata` field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// Output only. Capabilities of this location.
    #[prost(
        enumeration = "location_metadata::Capability",
        repeated,
        packed = "false",
        tag = "1"
    )]
    pub capabilities: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `LocationMetadata`.
pub mod location_metadata {
    /// Capability of a location.
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
    pub enum Capability {
        /// The default value. This value is used if the capability is omitted or
        /// unknown.
        Unspecified = 0,
        /// Stretch clusters are supported in this location.
        StretchedClusters = 1,
    }
    impl Capability {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Capability::Unspecified => "CAPABILITY_UNSPECIFIED",
                Capability::StretchedClusters => "STRETCHED_CLUSTERS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CAPABILITY_UNSPECIFIED" => Some(Self::Unspecified),
                "STRETCHED_CLUSTERS" => Some(Self::StretchedClusters),
                _ => None,
            }
        }
    }
}
/// DnsBindPermission resource that contains the accounts having the consumer DNS
/// bind permission on the corresponding intranet VPC of the consumer project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsBindPermission {
    /// Required. Output only. The name of the resource which stores the
    /// users/service accounts having the permission to bind to the corresponding
    /// intranet VPC of the consumer project. DnsBindPermission is a global
    /// resource and location can only be global. Resource names are schemeless
    /// URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.> For example:
    /// `projects/my-project/locations/global/dnsBindPermission`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Users/Service accounts which have access for binding on the
    /// intranet VPC project corresponding to the consumer project.
    #[prost(message, repeated, tag = "2")]
    pub principals: ::prost::alloc::vec::Vec<Principal>,
}
/// Users/Service accounts which have access for DNS binding on the intranet
/// VPC corresponding to the consumer project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Principal {
    /// The consumer provided user/service account which needs to be
    /// granted permission to DNS bind with the intranet VPC corresponding to the
    /// consumer project.
    #[prost(oneof = "principal::Principal", tags = "1, 2")]
    pub principal: ::core::option::Option<principal::Principal>,
}
/// Nested message and enum types in `Principal`.
pub mod principal {
    /// The consumer provided user/service account which needs to be
    /// granted permission to DNS bind with the intranet VPC corresponding to the
    /// consumer project.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Principal {
        /// The user who needs to be granted permission.
        #[prost(string, tag = "1")]
        User(::prost::alloc::string::String),
        /// The service account which needs to be granted the permission.
        #[prost(string, tag = "2")]
        ServiceAccount(::prost::alloc::string::String),
    }
}
/// Request message for
/// [VmwareEngine.ListPrivateClouds][google.cloud.vmwareengine.v1.VmwareEngine.ListPrivateClouds]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrivateCloudsRequest {
    /// Required. The resource name of the private cloud to be queried for
    /// clusters. Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of private clouds to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListPrivateClouds` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListPrivateClouds` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that matches resources returned in the response.
    /// The expression must specify the field name, a comparison operator, and the
    /// value that you want to use for filtering. The value must be a string, a
    /// number, or a boolean. The comparison operator must be `=`, `!=`, `>`, or
    /// `<`.
    ///
    /// For example, if you are filtering a list of private clouds, you can exclude
    /// the ones named `example-pc` by specifying `name != "example-pc"`.
    ///
    /// You can also filter nested fields. For example, you could specify
    /// `networkConfig.managementCidr = "192.168.0.0/24"` to include private clouds
    /// only if they have a matching address in their network configuration.
    ///
    /// To filter on multiple expressions, provide each separate expression within
    /// parentheses. For example:
    /// ```
    /// (name = "example-pc")
    /// (createTime > "2021-04-12T08:15:10.40Z")
    /// ```
    ///
    /// By default, each expression is an `AND` expression. However, you can
    /// include `AND` and `OR` expressions explicitly. For example:
    /// ```
    /// (name = "private-cloud-1") AND
    /// (createTime > "2021-04-12T08:15:10.40Z") OR
    /// (name = "private-cloud-2")
    /// ```
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sorts list results by a certain order. By default, returned results are
    /// ordered by `name` in ascending order. You can also sort results in
    /// descending order based on the `name` value using `orderBy="name desc"`.
    /// Currently, only ordering by `name` is supported.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListPrivateClouds][google.cloud.vmwareengine.v1.VmwareEngine.ListPrivateClouds]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrivateCloudsResponse {
    /// A list of private clouds.
    #[prost(message, repeated, tag = "1")]
    pub private_clouds: ::prost::alloc::vec::Vec<PrivateCloud>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached when making an aggregated query using
    /// wildcards.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [VmwareEngine.GetPrivateCloud][google.cloud.vmwareengine.v1.VmwareEngine.GetPrivateCloud]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivateCloudRequest {
    /// Required. The resource name of the private cloud to retrieve.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.CreatePrivateCloud][google.cloud.vmwareengine.v1.VmwareEngine.CreatePrivateCloud]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePrivateCloudRequest {
    /// Required. The resource name of the location to create the new
    /// private cloud in. Resource names are schemeless URIs that follow the
    /// conventions in <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The user-provided identifier of the private cloud to be created.
    /// This identifier must be unique among each `PrivateCloud` within the parent
    /// and becomes the final token in the name URI.
    /// The identifier must meet the following requirements:
    ///
    /// * Only contains 1-63 alphanumeric characters and hyphens
    /// * Begins with an alphabetical character
    /// * Ends with a non-hyphen character
    /// * Not formatted as a UUID
    /// * Complies with [RFC 1034](<https://datatracker.ietf.org/doc/html/rfc1034>)
    /// (section 3.5)
    #[prost(string, tag = "2")]
    pub private_cloud_id: ::prost::alloc::string::String,
    /// Required. The initial description of the new private cloud.
    #[prost(message, optional, tag = "3")]
    pub private_cloud: ::core::option::Option<PrivateCloud>,
    /// Optional. The request ID must be a valid UUID with the exception that zero
    /// UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. True if you want the request to be validated and not executed;
    /// false otherwise.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Request message for
/// [VmwareEngine.UpdatePrivateCloud][google.cloud.vmwareengine.v1.VmwareEngine.UpdatePrivateCloud]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePrivateCloudRequest {
    /// Required. Private cloud description.
    #[prost(message, optional, tag = "1")]
    pub private_cloud: ::core::option::Option<PrivateCloud>,
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `PrivateCloud` resource by the update. The fields specified in `updateMask`
    /// are relative to the resource, not the full request. A field will be
    /// overwritten if it is in the mask. If the user does not provide a mask then
    /// all fields will be overwritten.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. The request ID must be a valid UUID with the exception that zero
    /// UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.DeletePrivateCloud][google.cloud.vmwareengine.v1.VmwareEngine.DeletePrivateCloud]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePrivateCloudRequest {
    /// Required. The resource name of the private cloud to delete.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The request ID must be a valid UUID with the exception that zero
    /// UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, cascade delete is enabled and all children of
    /// this private cloud resource are also deleted. When this flag is set to
    /// false, the private cloud will not be deleted if there are any children
    /// other than the management cluster. The management cluster is always
    /// deleted.
    #[prost(bool, tag = "3")]
    pub force: bool,
    /// Optional. Time delay of the deletion specified in hours. The default value
    /// is `3`. Specifying a non-zero value for this field changes the value of
    /// `PrivateCloud.state` to `DELETED` and sets `expire_time` to the planned
    /// deletion time. Deletion can be cancelled before `expire_time` elapses using
    /// [VmwareEngine.UndeletePrivateCloud][google.cloud.vmwareengine.v1.VmwareEngine.UndeletePrivateCloud].
    /// Specifying a value of `0` for this field instead begins the deletion
    /// process and ceases billing immediately. During the final deletion process,
    /// the value of `PrivateCloud.state` becomes `PURGING`.
    #[prost(int32, optional, tag = "4")]
    pub delay_hours: ::core::option::Option<i32>,
}
/// Request message for
/// [VmwareEngine.UndeletePrivateCloud][google.cloud.vmwareengine.v1.VmwareEngine.UndeletePrivateCloud]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeletePrivateCloudRequest {
    /// Required. The resource name of the private cloud scheduled for deletion.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The request ID must be a valid UUID with the exception that zero
    /// UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ListClusters][google.cloud.vmwareengine.v1.VmwareEngine.ListClusters]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersRequest {
    /// Required. The resource name of the private cloud to query for clusters.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of clusters to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListClusters` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListClusters`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    ///
    /// To filter on multiple expressions, provide each separate expression within
    /// parentheses. For example:
    /// ```
    /// (name = "example-cluster")
    /// (nodeCount = "3")
    /// ```
    ///
    /// By default, each expression is an `AND` expression. However, you can
    /// include `AND` and `OR` expressions explicitly. For example:
    /// ```
    /// (name = "example-cluster-1") AND
    /// (createTime > "2021-04-12T08:15:10.40Z") OR
    /// (name = "example-cluster-2")
    /// ```
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sorts list results by a certain order. By default, returned results are
    /// ordered by `name` in ascending order. You can also sort results in
    /// descending order based on the `name` value using `orderBy="name desc"`.
    /// Currently, only ordering by `name` is supported.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListClusters][google.cloud.vmwareengine.v1.VmwareEngine.ListClusters]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersResponse {
    /// A list of private cloud clusters.
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<Cluster>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached when making an aggregated query using
    /// wildcards.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [VmwareEngine.GetCluster][google.cloud.vmwareengine.v1.VmwareEngine.GetCluster]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterRequest {
    /// Required. The cluster resource name to retrieve.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/clusters/my-cluster`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.CreateCluster][google.cloud.vmwareengine.v1.VmwareEngine.CreateCluster]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterRequest {
    /// Required. The resource name of the private cloud to create a new cluster
    /// in. Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The user-provided identifier of the new `Cluster`.
    /// This identifier must be unique among clusters within the parent and becomes
    /// the final token in the name URI.
    /// The identifier must meet the following requirements:
    ///
    /// * Only contains 1-63 alphanumeric characters and hyphens
    /// * Begins with an alphabetical character
    /// * Ends with a non-hyphen character
    /// * Not formatted as a UUID
    /// * Complies with [RFC 1034](<https://datatracker.ietf.org/doc/html/rfc1034>)
    /// (section 3.5)
    #[prost(string, tag = "2")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The initial description of the new cluster.
    #[prost(message, optional, tag = "3")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Optional. The request ID must be a valid UUID with the exception that zero
    /// UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. True if you want the request to be validated and not executed;
    /// false otherwise.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Request message for
/// [VmwareEngine.UpdateCluster][google.cloud.vmwareengine.v1.VmwareEngine.UpdateCluster]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClusterRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `Cluster` resource by the update. The fields specified in the `updateMask`
    /// are relative to the resource, not the full request. A field will be
    /// overwritten if it is in the mask. If the user does not provide a mask then
    /// all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The description of the cluster.
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Optional. The request ID must be a valid UUID with the exception that
    /// zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. True if you want the request to be validated and not executed;
    /// false otherwise.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for
/// [VmwareEngine.DeleteCluster][google.cloud.vmwareengine.v1.VmwareEngine.DeleteCluster]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClusterRequest {
    /// Required. The resource name of the cluster to delete.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/clusters/my-cluster`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The request ID must be a valid UUID with the exception that zero
    /// UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ListNodes][google.cloud.vmwareengine.v1.VmwareEngine.ListNodes]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodesRequest {
    /// Required. The resource name of the cluster to be queried for nodes.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/clusters/my-cluster`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of nodes to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListNodes` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListNodes` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListNodes][google.cloud.vmwareengine.v1.VmwareEngine.ListNodes]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodesResponse {
    /// The nodes.
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<Node>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.GetNode][google.cloud.vmwareengine.v1.VmwareEngine.GetNode]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeRequest {
    /// Required. The resource name of the node to retrieve.
    /// For example:
    /// `projects/{project}/locations/{location}/privateClouds/{private_cloud}/clusters/{cluster}/nodes/{node}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ListExternalAddresses][google.cloud.vmwareengine.v1.VmwareEngine.ListExternalAddresses]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExternalAddressesRequest {
    /// Required. The resource name of the private cloud to be queried for
    /// external IP addresses.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of external IP addresses to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListExternalAddresses` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListExternalAddresses` must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that matches resources returned in the response.
    /// The expression must specify the field name, a comparison
    /// operator, and the value that you want to use for filtering. The value
    /// must be a string, a number, or a boolean. The comparison operator
    /// must be `=`, `!=`, `>`, or `<`.
    ///
    /// For example, if you are filtering a list of IP addresses, you can
    /// exclude the ones named `example-ip` by specifying
    /// `name != "example-ip"`.
    ///
    /// To filter on multiple expressions, provide each separate expression within
    /// parentheses. For example:
    /// ```
    /// (name = "example-ip")
    /// (createTime > "2021-04-12T08:15:10.40Z")
    /// ```
    ///
    /// By default, each expression is an `AND` expression. However, you
    /// can include `AND` and `OR` expressions explicitly.
    /// For example:
    /// ```
    /// (name = "example-ip-1") AND
    /// (createTime > "2021-04-12T08:15:10.40Z") OR
    /// (name = "example-ip-2")
    /// ```
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sorts list results by a certain order. By default, returned results
    /// are ordered by `name` in ascending order.
    /// You can also sort results in descending order based on the `name` value
    /// using `orderBy="name desc"`.
    /// Currently, only ordering by `name` is supported.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListExternalAddresses][google.cloud.vmwareengine.v1.VmwareEngine.ListExternalAddresses]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExternalAddressesResponse {
    /// A list of external IP addresses.
    #[prost(message, repeated, tag = "1")]
    pub external_addresses: ::prost::alloc::vec::Vec<ExternalAddress>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached when making an aggregated query using
    /// wildcards.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [VmwareEngine.FetchNetworkPolicyExternalAddresses][google.cloud.vmwareengine.v1.VmwareEngine.FetchNetworkPolicyExternalAddresses]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchNetworkPolicyExternalAddressesRequest {
    /// Required. The resource name of the network policy to query for assigned
    /// external IP addresses. Resource names are schemeless URIs that follow the
    /// conventions in <https://cloud.google.com/apis/design/resource_names.> For
    /// example:
    /// `projects/my-project/locations/us-central1/networkPolicies/my-policy`
    #[prost(string, tag = "1")]
    pub network_policy: ::prost::alloc::string::String,
    /// The maximum number of external IP addresses to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous
    /// `FetchNetworkPolicyExternalAddresses` call. Provide this to retrieve the
    /// subsequent page.
    ///
    /// When paginating, all parameters provided to
    /// `FetchNetworkPolicyExternalAddresses`, except for `page_size` and
    /// `page_token`, must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.FetchNetworkPolicyExternalAddresses][google.cloud.vmwareengine.v1.VmwareEngine.FetchNetworkPolicyExternalAddresses]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchNetworkPolicyExternalAddressesResponse {
    /// A list of external IP addresses assigned to VMware workload VMs within the
    /// scope of the given network policy.
    #[prost(message, repeated, tag = "1")]
    pub external_addresses: ::prost::alloc::vec::Vec<ExternalAddress>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.GetExternalAddress][google.cloud.vmwareengine.v1.VmwareEngine.GetExternalAddress]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExternalAddressRequest {
    /// Required. The resource name of the external IP address to retrieve.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/externalAddresses/my-ip`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.CreateExternalAddress][google.cloud.vmwareengine.v1.VmwareEngine.CreateExternalAddress]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateExternalAddressRequest {
    /// Required. The resource name of the private cloud
    /// to create a new external IP address in.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The initial description of a new external IP address.
    #[prost(message, optional, tag = "2")]
    pub external_address: ::core::option::Option<ExternalAddress>,
    /// Required. The user-provided identifier of the `ExternalAddress` to be
    /// created. This identifier must be unique among `ExternalAddress` resources
    /// within the parent and becomes the final token in the name URI. The
    /// identifier must meet the following requirements:
    ///
    /// * Only contains 1-63 alphanumeric characters and hyphens
    /// * Begins with an alphabetical character
    /// * Ends with a non-hyphen character
    /// * Not formatted as a UUID
    /// * Complies with [RFC 1034](<https://datatracker.ietf.org/doc/html/rfc1034>)
    /// (section 3.5)
    #[prost(string, tag = "3")]
    pub external_address_id: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if the original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.UpdateExternalAddress][google.cloud.vmwareengine.v1.VmwareEngine.UpdateExternalAddress]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateExternalAddressRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `ExternalAddress` resource by the update.
    /// The fields specified in the `update_mask` are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. External IP address description.
    #[prost(message, optional, tag = "2")]
    pub external_address: ::core::option::Option<ExternalAddress>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if the original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.DeleteExternalAddress][google.cloud.vmwareengine.v1.VmwareEngine.DeleteExternalAddress]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteExternalAddressRequest {
    /// Required. The resource name of the external IP address to delete.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/externalAddresses/my-ip`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if the original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ListSubnets][google.cloud.vmwareengine.v1.VmwareEngine.ListSubnets]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubnetsRequest {
    /// Required. The resource name of the private cloud to be queried for
    /// subnets.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of subnets to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListSubnetsRequest` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListSubnetsRequest` must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListSubnets][google.cloud.vmwareengine.v1.VmwareEngine.ListSubnets]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubnetsResponse {
    /// A list of subnets.
    #[prost(message, repeated, tag = "1")]
    pub subnets: ::prost::alloc::vec::Vec<Subnet>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached when making an aggregated query using
    /// wildcards.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [VmwareEngine.GetSubnet][google.cloud.vmwareengine.v1.VmwareEngine.GetSubnet]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSubnetRequest {
    /// Required. The resource name of the subnet to retrieve.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/subnets/my-subnet`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.UpdateSubnet][google.cloud.vmwareengine.v1.VmwareEngine.UpdateSubnet]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSubnetRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `Subnet` resource by the update.
    /// The fields specified in the `update_mask` are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Subnet description.
    #[prost(message, optional, tag = "2")]
    pub subnet: ::core::option::Option<Subnet>,
}
/// Request message for
/// [VmwareEngine.ListExternalAccessRules][google.cloud.vmwareengine.v1.VmwareEngine.ListExternalAccessRules]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExternalAccessRulesRequest {
    /// Required. The resource name of the network policy to query for external
    /// access firewall rules. Resource names are schemeless URIs that follow the
    /// conventions in <https://cloud.google.com/apis/design/resource_names.> For
    /// example:
    /// `projects/my-project/locations/us-central1/networkPolicies/my-policy`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of external access rules to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListExternalAccessRulesRequest`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListExternalAccessRulesRequest` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that matches resources returned in the response.
    /// The expression must specify the field name, a comparison
    /// operator, and the value that you want to use for filtering. The value
    /// must be a string, a number, or a boolean. The comparison operator
    /// must be `=`, `!=`, `>`, or `<`.
    ///
    /// For example, if you are filtering a list of external access rules, you can
    /// exclude the ones named `example-rule` by specifying
    /// `name != "example-rule"`.
    ///
    /// To filter on multiple expressions, provide each separate expression within
    /// parentheses. For example:
    /// ```
    /// (name = "example-rule")
    /// (createTime > "2021-04-12T08:15:10.40Z")
    /// ```
    ///
    /// By default, each expression is an `AND` expression. However, you
    /// can include `AND` and `OR` expressions explicitly.
    /// For example:
    /// ```
    /// (name = "example-rule-1") AND
    /// (createTime > "2021-04-12T08:15:10.40Z") OR
    /// (name = "example-rule-2")
    /// ```
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sorts list results by a certain order. By default, returned results
    /// are ordered by `name` in ascending order.
    /// You can also sort results in descending order based on the `name` value
    /// using `orderBy="name desc"`.
    /// Currently, only ordering by `name` is supported.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListExternalAccessRules][google.cloud.vmwareengine.v1.VmwareEngine.ListExternalAccessRules]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExternalAccessRulesResponse {
    /// A list of external access firewall rules.
    #[prost(message, repeated, tag = "1")]
    pub external_access_rules: ::prost::alloc::vec::Vec<ExternalAccessRule>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached when making an aggregated query using
    /// wildcards.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [VmwareEngine.GetExternalAccessRule][google.cloud.vmwareengine.v1.VmwareEngine.GetExternalAccessRule]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExternalAccessRuleRequest {
    /// Required. The resource name of the external access firewall rule to
    /// retrieve. Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/networkPolicies/my-policy/externalAccessRules/my-rule`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.CreateExternalAccessRule][google.cloud.vmwareengine.v1.VmwareEngine.CreateExternalAccessRule]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateExternalAccessRuleRequest {
    /// Required. The resource name of the network policy
    /// to create a new external access firewall rule in.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/networkPolicies/my-policy`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The initial description of a new external access rule.
    #[prost(message, optional, tag = "2")]
    pub external_access_rule: ::core::option::Option<ExternalAccessRule>,
    /// Required. The user-provided identifier of the `ExternalAccessRule` to be
    /// created. This identifier must be unique among `ExternalAccessRule`
    /// resources within the parent and becomes the final token in the name URI.
    /// The identifier must meet the following requirements:
    ///
    /// * Only contains 1-63 alphanumeric characters and hyphens
    /// * Begins with an alphabetical character
    /// * Ends with a non-hyphen character
    /// * Not formatted as a UUID
    /// * Complies with [RFC 1034](<https://datatracker.ietf.org/doc/html/rfc1034>)
    /// (section 3.5)
    #[prost(string, tag = "3")]
    pub external_access_rule_id: ::prost::alloc::string::String,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if the original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.UpdateExternalAccessRule][google.cloud.vmwareengine.v1.VmwareEngine.UpdateExternalAccessRule]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateExternalAccessRuleRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `ExternalAccessRule` resource by the update.
    /// The fields specified in the `update_mask` are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Description of the external access rule.
    #[prost(message, optional, tag = "2")]
    pub external_access_rule: ::core::option::Option<ExternalAccessRule>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if the original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.DeleteExternalAccessRule][google.cloud.vmwareengine.v1.VmwareEngine.DeleteExternalAccessRule]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteExternalAccessRuleRequest {
    /// Required. The resource name of the external access firewall rule to delete.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/networkPolicies/my-policy/externalAccessRules/my-rule`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if the original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ListLoggingServers][google.cloud.vmwareengine.v1.VmwareEngine.ListLoggingServers]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLoggingServersRequest {
    /// Required. The resource name of the private cloud to be queried for
    /// logging servers.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of logging servers to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListLoggingServersRequest` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListLoggingServersRequest` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that matches resources returned in the response.
    /// The expression must specify the field name, a comparison
    /// operator, and the value that you want to use for filtering. The value
    /// must be a string, a number, or a boolean. The comparison operator
    /// must be `=`, `!=`, `>`, or `<`.
    ///
    /// For example, if you are filtering a list of logging servers, you can
    /// exclude the ones named `example-server` by specifying
    /// `name != "example-server"`.
    ///
    /// To filter on multiple expressions, provide each separate expression within
    /// parentheses. For example:
    /// ```
    /// (name = "example-server")
    /// (createTime > "2021-04-12T08:15:10.40Z")
    /// ```
    ///
    /// By default, each expression is an `AND` expression. However, you
    /// can include `AND` and `OR` expressions explicitly.
    /// For example:
    /// ```
    /// (name = "example-server-1") AND
    /// (createTime > "2021-04-12T08:15:10.40Z") OR
    /// (name = "example-server-2")
    /// ```
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sorts list results by a certain order. By default, returned results
    /// are ordered by `name` in ascending order.
    /// You can also sort results in descending order based on the `name` value
    /// using `orderBy="name desc"`.
    /// Currently, only ordering by `name` is supported.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListLoggingServers][google.cloud.vmwareengine.v1.VmwareEngine.ListLoggingServers]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLoggingServersResponse {
    /// A list of Logging Servers.
    #[prost(message, repeated, tag = "1")]
    pub logging_servers: ::prost::alloc::vec::Vec<LoggingServer>,
    /// A token, which can be send as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached when making an aggregated query using
    /// wildcards.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [VmwareEngine.GetLoggingServer][google.cloud.vmwareengine.v1.VmwareEngine.GetLoggingServer]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoggingServerRequest {
    /// Required. The resource name of the Logging Server to retrieve.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/loggingServers/my-logging-server`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.CreateLoggingServer][google.cloud.vmwareengine.v1.VmwareEngine.CreateLoggingServer]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLoggingServerRequest {
    /// Required. The resource name of the private cloud
    /// to create a new Logging Server in.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The initial description of a new logging server.
    #[prost(message, optional, tag = "2")]
    pub logging_server: ::core::option::Option<LoggingServer>,
    /// Required. The user-provided identifier of the `LoggingServer` to be
    /// created. This identifier must be unique among `LoggingServer` resources
    /// within the parent and becomes the final token in the name URI.
    /// The identifier must meet the following requirements:
    ///
    /// * Only contains 1-63 alphanumeric characters and hyphens
    /// * Begins with an alphabetical character
    /// * Ends with a non-hyphen character
    /// * Not formatted as a UUID
    /// * Complies with [RFC 1034](<https://datatracker.ietf.org/doc/html/rfc1034>)
    /// (section 3.5)
    #[prost(string, tag = "3")]
    pub logging_server_id: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.UpdateLoggingServer][google.cloud.vmwareengine.v1.VmwareEngine.UpdateLoggingServer]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLoggingServerRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `LoggingServer` resource by the update.
    /// The fields specified in the `update_mask` are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Logging server description.
    #[prost(message, optional, tag = "2")]
    pub logging_server: ::core::option::Option<LoggingServer>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.DeleteLoggingServer][google.cloud.vmwareengine.v1.VmwareEngine.DeleteLoggingServer]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLoggingServerRequest {
    /// Required. The resource name of the logging server to delete.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/loggingServers/my-logging-server`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
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
    /// Output only. True if the user has requested cancellation
    /// of the operation; false otherwise.
    /// Operations that have successfully been cancelled
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ListNodeTypes][google.cloud.vmwareengine.v1.VmwareEngine.ListNodeTypes]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodeTypesRequest {
    /// Required. The resource name of the location to be queried for node types.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of node types to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListNodeTypes` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListNodeTypes` must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that matches resources returned in the response.
    /// The expression must specify the field name, a comparison
    /// operator, and the value that you want to use for filtering. The value
    /// must be a string, a number, or a boolean. The comparison operator
    /// must be `=`, `!=`, `>`, or `<`.
    ///
    /// For example, if you are filtering a list of node types, you can
    /// exclude the ones named `standard-72` by specifying
    /// `name != "standard-72"`.
    ///
    /// To filter on multiple expressions, provide each separate expression within
    /// parentheses. For example:
    /// ```
    /// (name = "standard-72")
    /// (virtual_cpu_count > 2)
    /// ```
    ///
    /// By default, each expression is an `AND` expression. However, you
    /// can include `AND` and `OR` expressions explicitly.
    /// For example:
    /// ```
    /// (name = "standard-96") AND
    /// (virtual_cpu_count > 2) OR
    /// (name = "standard-72")
    /// ```
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListNodeTypes][google.cloud.vmwareengine.v1.VmwareEngine.ListNodeTypes]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodeTypesResponse {
    /// A list of Node Types.
    #[prost(message, repeated, tag = "1")]
    pub node_types: ::prost::alloc::vec::Vec<NodeType>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached when making an aggregated query using
    /// wildcards.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [VmwareEngine.GetNodeType][google.cloud.vmwareengine.v1.VmwareEngine.GetNodeType]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeTypeRequest {
    /// Required. The resource name of the node type to retrieve.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-proj/locations/us-central1-a/nodeTypes/standard-72`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ShowNsxCredentials][google.cloud.vmwareengine.v1.VmwareEngine.ShowNsxCredentials]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowNsxCredentialsRequest {
    /// Required. The resource name of the private cloud
    /// to be queried for credentials.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub private_cloud: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ShowVcenterCredentials][google.cloud.vmwareengine.v1.VmwareEngine.ShowVcenterCredentials]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowVcenterCredentialsRequest {
    /// Required. The resource name of the private cloud
    /// to be queried for credentials.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub private_cloud: ::prost::alloc::string::String,
    /// Optional. The username of the user to be queried for credentials.
    /// The default value of this field is CloudOwner@gve.local.
    /// The provided value must be one of the following:
    /// CloudOwner@gve.local,
    /// solution-user-01@gve.local,
    /// solution-user-02@gve.local,
    /// solution-user-03@gve.local,
    /// solution-user-04@gve.local,
    /// solution-user-05@gve.local,
    /// zertoadmin@gve.local.
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ResetNsxCredentials][google.cloud.vmwareengine.v1.VmwareEngine.ResetNsxCredentials]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetNsxCredentialsRequest {
    /// Required. The resource name of the private cloud
    /// to reset credentials for.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub private_cloud: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ResetVcenterCredentials][google.cloud.vmwareengine.v1.VmwareEngine.ResetVcenterCredentials]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetVcenterCredentialsRequest {
    /// Required. The resource name of the private cloud
    /// to reset credentials for.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub private_cloud: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. The username of the user to be to reset the credentials.
    /// The default value of this field is CloudOwner@gve.local.
    /// The provided value should be one of the following:
    /// solution-user-01@gve.local,
    /// solution-user-02@gve.local,
    /// solution-user-03@gve.local,
    /// solution-user-04@gve.local,
    /// solution-user-05@gve.local,
    /// zertoadmin@gve.local.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListHcxActivationKeys][google.cloud.vmwareengine.v1.VmwareEngine.ListHcxActivationKeys]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHcxActivationKeysResponse {
    /// List of HCX activation keys.
    #[prost(message, repeated, tag = "1")]
    pub hcx_activation_keys: ::prost::alloc::vec::Vec<HcxActivationKey>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached when making an aggregated query using
    /// wildcards.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [VmwareEngine.ListHcxActivationKeys][google.cloud.vmwareengine.v1.VmwareEngine.ListHcxActivationKeys]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHcxActivationKeysRequest {
    /// Required. The resource name of the private cloud
    /// to be queried for HCX activation keys.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of HCX activation keys to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListHcxActivationKeys` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListHcxActivationKeys` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Request message for [VmwareEngine.GetHcxActivationKeys][]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHcxActivationKeyRequest {
    /// Required. The resource name of the HCX activation key to retrieve.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/privateClouds/my-cloud/hcxActivationKeys/my-key`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.CreateHcxActivationKey][google.cloud.vmwareengine.v1.VmwareEngine.CreateHcxActivationKey]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateHcxActivationKeyRequest {
    /// Required. The resource name of the private cloud to create the key for.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The initial description of a new HCX activation key. When
    /// creating a new key, this field must be an empty object.
    #[prost(message, optional, tag = "2")]
    pub hcx_activation_key: ::core::option::Option<HcxActivationKey>,
    /// Required. The user-provided identifier of the `HcxActivationKey` to be
    /// created. This identifier must be unique among `HcxActivationKey` resources
    /// within the parent and becomes the final token in the name URI.
    /// The identifier must meet the following requirements:
    ///
    /// * Only contains 1-63 alphanumeric characters and hyphens
    /// * Begins with an alphabetical character
    /// * Ends with a non-hyphen character
    /// * Not formatted as a UUID
    /// * Complies with [RFC 1034](<https://datatracker.ietf.org/doc/html/rfc1034>)
    /// (section 3.5)
    #[prost(string, tag = "3")]
    pub hcx_activation_key_id: ::prost::alloc::string::String,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.GetDnsForwarding][google.cloud.vmwareengine.v1.VmwareEngine.GetDnsForwarding]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDnsForwardingRequest {
    /// Required. The resource name of a `DnsForwarding` to retrieve.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/dnsForwarding`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.UpdateDnsForwarding][google.cloud.vmwareengine.v1.VmwareEngine.UpdateDnsForwarding]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDnsForwardingRequest {
    /// Required. DnsForwarding config details.
    #[prost(message, optional, tag = "1")]
    pub dns_forwarding: ::core::option::Option<DnsForwarding>,
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `DnsForwarding` resource by the update.
    /// The fields specified in the `update_mask` are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.CreateNetworkPeering][google.cloud.vmwareengine.v1.VmwareEngine.CreateNetworkPeering]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNetworkPeeringRequest {
    /// Required. The resource name of the location to create the new network
    /// peering in. This value is always `global`, because `NetworkPeering` is a
    /// global resource. Resource names are schemeless URIs that follow the
    /// conventions in <https://cloud.google.com/apis/design/resource_names.> For
    /// example: `projects/my-project/locations/global`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The user-provided identifier of the new `NetworkPeering`.
    /// This identifier must be unique among `NetworkPeering` resources within the
    /// parent and becomes the final token in the name URI.
    /// The identifier must meet the following requirements:
    ///
    /// * Only contains 1-63 alphanumeric characters and hyphens
    /// * Begins with an alphabetical character
    /// * Ends with a non-hyphen character
    /// * Not formatted as a UUID
    /// * Complies with [RFC 1034](<https://datatracker.ietf.org/doc/html/rfc1034>)
    /// (section 3.5)
    #[prost(string, tag = "2")]
    pub network_peering_id: ::prost::alloc::string::String,
    /// Required. The initial description of the new network peering.
    #[prost(message, optional, tag = "3")]
    pub network_peering: ::core::option::Option<NetworkPeering>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.DeleteNetworkPeering][google.cloud.vmwareengine.v1.VmwareEngine.DeleteNetworkPeering]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNetworkPeeringRequest {
    /// Required. The resource name of the network peering to be deleted.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/global/networkPeerings/my-peering`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.GetNetworkPeering][google.cloud.vmwareengine.v1.VmwareEngine.GetNetworkPeering]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNetworkPeeringRequest {
    /// Required. The resource name of the network peering to retrieve.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/global/networkPeerings/my-peering`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ListNetworkPeerings][google.cloud.vmwareengine.v1.VmwareEngine.ListNetworkPeerings]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNetworkPeeringsRequest {
    /// Required. The resource name of the location (global) to query for
    /// network peerings. Resource names are schemeless URIs that follow the
    /// conventions in <https://cloud.google.com/apis/design/resource_names.> For
    /// example: `projects/my-project/locations/global`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of network peerings to return in one page.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListNetworkPeerings` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListNetworkPeerings` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that matches resources returned in the response.
    /// The expression must specify the field name, a comparison
    /// operator, and the value that you want to use for filtering. The value
    /// must be a string, a number, or a boolean. The comparison operator
    /// must be `=`, `!=`, `>`, or `<`.
    ///
    /// For example, if you are filtering a list of network peerings, you can
    /// exclude the ones named `example-peering` by specifying
    /// `name != "example-peering"`.
    ///
    /// To filter on multiple expressions, provide each separate expression within
    /// parentheses. For example:
    /// ```
    /// (name = "example-peering")
    /// (createTime > "2021-04-12T08:15:10.40Z")
    /// ```
    ///
    /// By default, each expression is an `AND` expression. However, you
    /// can include `AND` and `OR` expressions explicitly.
    /// For example:
    /// ```
    /// (name = "example-peering-1") AND
    /// (createTime > "2021-04-12T08:15:10.40Z") OR
    /// (name = "example-peering-2")
    /// ```
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sorts list results by a certain order. By default, returned results
    /// are ordered by `name` in ascending order.
    /// You can also sort results in descending order based on the `name` value
    /// using `orderBy="name desc"`.
    /// Currently, only ordering by `name` is supported.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.UpdateNetworkPeering][google.cloud.vmwareengine.v1.VmwareEngine.UpdateNetworkPeering]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNetworkPeeringRequest {
    /// Required. Network peering description.
    #[prost(message, optional, tag = "1")]
    pub network_peering: ::core::option::Option<NetworkPeering>,
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `NetworkPeering` resource by the update.
    /// The fields specified in the `update_mask` are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListNetworkPeerings][google.cloud.vmwareengine.v1.VmwareEngine.ListNetworkPeerings]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNetworkPeeringsResponse {
    /// A list of network peerings.
    #[prost(message, repeated, tag = "1")]
    pub network_peerings: ::prost::alloc::vec::Vec<NetworkPeering>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [VmwareEngine.ListPeeringRoutes][google.cloud.vmwareengine.v1.VmwareEngine.ListPeeringRoutes]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPeeringRoutesRequest {
    /// Required. The resource name of the network peering to retrieve peering
    /// routes from. Resource names are schemeless URIs that follow the conventions
    /// in <https://cloud.google.com/apis/design/resource_names.> For example:
    /// `projects/my-project/locations/global/networkPeerings/my-peering`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of peering routes to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListPeeringRoutes` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListPeeringRoutes` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that matches resources returned in the response.
    /// Currently, only filtering on the `direction` field is supported. To return
    /// routes imported from the peer network, provide "direction=INCOMING". To
    /// return routes exported from the VMware Engine network, provide
    /// "direction=OUTGOING". Other filter expressions return an error.
    #[prost(string, tag = "6")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListPeeringRoutes][google.cloud.vmwareengine.v1.VmwareEngine.ListPeeringRoutes]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPeeringRoutesResponse {
    /// A list of peering routes.
    #[prost(message, repeated, tag = "1")]
    pub peering_routes: ::prost::alloc::vec::Vec<PeeringRoute>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ListNetworkPolicies][google.cloud.vmwareengine.v1.VmwareEngine.ListNetworkPolicies]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNetworkPoliciesRequest {
    /// Required. The resource name of the location (region) to query for
    /// network policies. Resource names are schemeless URIs that follow the
    /// conventions in <https://cloud.google.com/apis/design/resource_names.> For
    /// example: `projects/my-project/locations/us-central1`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of network policies to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListNetworkPolicies` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListNetworkPolicies` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that matches resources returned in the response.
    /// The expression must specify the field name, a comparison
    /// operator, and the value that you want to use for filtering. The value
    /// must be a string, a number, or a boolean. The comparison operator
    /// must be `=`, `!=`, `>`, or `<`.
    ///
    /// For example, if you are filtering a list of network policies, you can
    /// exclude the ones named `example-policy` by specifying
    /// `name != "example-policy"`.
    ///
    /// To filter on multiple expressions, provide each separate expression within
    /// parentheses. For example:
    /// ```
    /// (name = "example-policy")
    /// (createTime > "2021-04-12T08:15:10.40Z")
    /// ```
    ///
    /// By default, each expression is an `AND` expression. However, you
    /// can include `AND` and `OR` expressions explicitly.
    /// For example:
    /// ```
    /// (name = "example-policy-1") AND
    /// (createTime > "2021-04-12T08:15:10.40Z") OR
    /// (name = "example-policy-2")
    /// ```
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sorts list results by a certain order. By default, returned results
    /// are ordered by `name` in ascending order.
    /// You can also sort results in descending order based on the `name` value
    /// using `orderBy="name desc"`.
    /// Currently, only ordering by `name` is supported.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListNetworkPolicies][google.cloud.vmwareengine.v1.VmwareEngine.ListNetworkPolicies]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNetworkPoliciesResponse {
    /// A list of network policies.
    #[prost(message, repeated, tag = "1")]
    pub network_policies: ::prost::alloc::vec::Vec<NetworkPolicy>,
    /// A token, which can be send as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached when making an aggregated query using
    /// wildcards.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [VmwareEngine.GetNetworkPolicy][google.cloud.vmwareengine.v1.VmwareEngine.GetNetworkPolicy]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNetworkPolicyRequest {
    /// Required. The resource name of the network policy to retrieve.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/networkPolicies/my-network-policy`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.UpdateNetworkPolicy][google.cloud.vmwareengine.v1.VmwareEngine.UpdateNetworkPolicy]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNetworkPolicyRequest {
    /// Required. Network policy description.
    #[prost(message, optional, tag = "1")]
    pub network_policy: ::core::option::Option<NetworkPolicy>,
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `NetworkPolicy` resource by the update.
    /// The fields specified in the `update_mask` are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.CreateNetworkPolicy][google.cloud.vmwareengine.v1.VmwareEngine.CreateNetworkPolicy]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNetworkPolicyRequest {
    /// Required. The resource name of the location (region)
    /// to create the new network policy in.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    ///   `projects/my-project/locations/us-central1`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The user-provided identifier of the network policy to be created.
    /// This identifier must be unique within parent
    /// `projects/{my-project}/locations/{us-central1}/networkPolicies` and becomes
    /// the final token in the name URI.
    /// The identifier must meet the following requirements:
    ///
    /// * Only contains 1-63 alphanumeric characters and hyphens
    /// * Begins with an alphabetical character
    /// * Ends with a non-hyphen character
    /// * Not formatted as a UUID
    /// * Complies with [RFC 1034](<https://datatracker.ietf.org/doc/html/rfc1034>)
    /// (section 3.5)
    #[prost(string, tag = "2")]
    pub network_policy_id: ::prost::alloc::string::String,
    /// Required. The network policy configuration to use in the request.
    #[prost(message, optional, tag = "3")]
    pub network_policy: ::core::option::Option<NetworkPolicy>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.DeleteNetworkPolicy][google.cloud.vmwareengine.v1.VmwareEngine.DeleteNetworkPolicy]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNetworkPolicyRequest {
    /// Required. The resource name of the network policy to delete.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/networkPolicies/my-network-policy`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ListManagementDnsZoneBindings][google.cloud.vmwareengine.v1.VmwareEngine.ListManagementDnsZoneBindings]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListManagementDnsZoneBindingsRequest {
    /// Required. The resource name of the private cloud to be queried for
    /// management DNS zone bindings.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of management DNS zone bindings to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListManagementDnsZoneBindings`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListManagementDnsZoneBindings` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that matches resources returned in the response.
    /// The expression must specify the field name, a comparison
    /// operator, and the value that you want to use for filtering. The value
    /// must be a string, a number, or a boolean. The comparison operator
    /// must be `=`, `!=`, `>`, or `<`.
    ///
    /// For example, if you are filtering a list of Management DNS Zone Bindings,
    /// you can exclude the ones named `example-management-dns-zone-binding` by
    /// specifying `name != "example-management-dns-zone-binding"`.
    ///
    /// To filter on multiple expressions, provide each separate expression within
    /// parentheses. For example:
    /// ```
    /// (name = "example-management-dns-zone-binding")
    /// (createTime > "2021-04-12T08:15:10.40Z")
    /// ```
    ///
    /// By default, each expression is an `AND` expression. However, you
    /// can include `AND` and `OR` expressions explicitly.
    /// For example:
    /// ```
    /// (name = "example-management-dns-zone-binding-1") AND
    /// (createTime > "2021-04-12T08:15:10.40Z") OR
    /// (name = "example-management-dns-zone-binding-2")
    /// ```
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sorts list results by a certain order. By default, returned results
    /// are ordered by `name` in ascending order.
    /// You can also sort results in descending order based on the `name` value
    /// using `orderBy="name desc"`.
    /// Currently, only ordering by `name` is supported.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListManagementDnsZoneBindings][google.cloud.vmwareengine.v1.VmwareEngine.ListManagementDnsZoneBindings]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListManagementDnsZoneBindingsResponse {
    /// A list of management DNS zone bindings.
    #[prost(message, repeated, tag = "1")]
    pub management_dns_zone_bindings: ::prost::alloc::vec::Vec<ManagementDnsZoneBinding>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached when making an aggregated query using
    /// wildcards.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [VmwareEngine.GetManagementDnsZoneBinding][google.cloud.vmwareengine.v1.VmwareEngine.GetManagementDnsZoneBinding]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManagementDnsZoneBindingRequest {
    /// Required. The resource name of the management DNS zone binding to
    /// retrieve. Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/managementDnsZoneBindings/my-management-dns-zone-binding`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [VmwareEngine.CreateManagementDnsZoneBindings][]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateManagementDnsZoneBindingRequest {
    /// Required. The resource name of the private cloud
    /// to create a new management DNS zone binding for.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The initial values for a new management DNS zone binding.
    #[prost(message, optional, tag = "2")]
    pub management_dns_zone_binding: ::core::option::Option<ManagementDnsZoneBinding>,
    /// Required. The user-provided identifier of the `ManagementDnsZoneBinding`
    /// resource to be created. This identifier must be unique among
    /// `ManagementDnsZoneBinding` resources within the parent and becomes the
    /// final token in the name URI. The identifier must meet the following
    /// requirements:
    ///
    /// * Only contains 1-63 alphanumeric characters and hyphens
    /// * Begins with an alphabetical character
    /// * Ends with a non-hyphen character
    /// * Not formatted as a UUID
    /// * Complies with [RFC 1034](<https://datatracker.ietf.org/doc/html/rfc1034>)
    /// (section 3.5)
    #[prost(string, tag = "3")]
    pub management_dns_zone_binding_id: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if the original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.UpdateManagementDnsZoneBinding][google.cloud.vmwareengine.v1.VmwareEngine.UpdateManagementDnsZoneBinding]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateManagementDnsZoneBindingRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `ManagementDnsZoneBinding` resource by the update.
    /// The fields specified in the `update_mask` are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. New values to update the management DNS zone binding with.
    #[prost(message, optional, tag = "2")]
    pub management_dns_zone_binding: ::core::option::Option<ManagementDnsZoneBinding>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if the original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.DeleteManagementDnsZoneBinding][google.cloud.vmwareengine.v1.VmwareEngine.DeleteManagementDnsZoneBinding]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteManagementDnsZoneBindingRequest {
    /// Required. The resource name of the management DNS zone binding to delete.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/managementDnsZoneBindings/my-management-dns-zone-binding`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if the original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for [VmwareEngine.RepairManagementDnsZoneBindings][]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepairManagementDnsZoneBindingRequest {
    /// Required. The resource name of the management DNS zone binding to repair.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1-a/privateClouds/my-cloud/managementDnsZoneBindings/my-management-dns-zone-binding`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if the original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.CreateVmwareEngineNetwork][google.cloud.vmwareengine.v1.VmwareEngine.CreateVmwareEngineNetwork]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVmwareEngineNetworkRequest {
    /// Required. The resource name of the location to create the new VMware Engine
    /// network in. A VMware Engine network of type
    /// `LEGACY` is a regional resource, and a VMware
    /// Engine network of type `STANDARD` is a global resource.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.> For example:
    /// `projects/my-project/locations/global`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The user-provided identifier of the new VMware Engine network.
    /// This identifier must be unique among VMware Engine network resources
    /// within the parent and becomes the final token in the name URI. The
    /// identifier must meet the following requirements:
    ///
    /// * For networks of type LEGACY, adheres to the format:
    /// `{region-id}-default`. Replace `{region-id}` with the region where you want
    /// to create the VMware Engine network. For example, "us-central1-default".
    /// * Only contains 1-63 alphanumeric characters and hyphens
    /// * Begins with an alphabetical character
    /// * Ends with a non-hyphen character
    /// * Not formatted as a UUID
    /// * Complies with [RFC 1034](<https://datatracker.ietf.org/doc/html/rfc1034>)
    /// (section 3.5)
    #[prost(string, tag = "2")]
    pub vmware_engine_network_id: ::prost::alloc::string::String,
    /// Required. The initial description of the new VMware Engine network.
    #[prost(message, optional, tag = "3")]
    pub vmware_engine_network: ::core::option::Option<VmwareEngineNetwork>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.UpdateVmwareEngineNetwork][google.cloud.vmwareengine.v1.VmwareEngine.UpdateVmwareEngineNetwork]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVmwareEngineNetworkRequest {
    /// Required. VMware Engine network description.
    #[prost(message, optional, tag = "1")]
    pub vmware_engine_network: ::core::option::Option<VmwareEngineNetwork>,
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// VMware Engine network resource by the update.
    /// The fields specified in the `update_mask` are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten. Only the
    /// following fields can be updated: `description`.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.DeleteVmwareEngineNetwork][google.cloud.vmwareengine.v1.VmwareEngine.DeleteVmwareEngineNetwork]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVmwareEngineNetworkRequest {
    /// Required. The resource name of the VMware Engine network to be deleted.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/global/vmwareEngineNetworks/my-network`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. Checksum used to ensure that the user-provided value is up to
    /// date before the server processes the request. The server compares provided
    /// checksum with the current checksum of the resource. If the user-provided
    /// value is out of date, this request returns an `ABORTED` error.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.GetVmwareEngineNetwork][google.cloud.vmwareengine.v1.VmwareEngine.GetVmwareEngineNetwork]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVmwareEngineNetworkRequest {
    /// Required. The resource name of the VMware Engine network to retrieve.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/global/vmwareEngineNetworks/my-network`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ListVmwareEngineNetworks][google.cloud.vmwareengine.v1.VmwareEngine.ListVmwareEngineNetworks]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVmwareEngineNetworksRequest {
    /// Required. The resource name of the location to query for
    /// VMware Engine networks. Resource names are schemeless URIs that follow the
    /// conventions in <https://cloud.google.com/apis/design/resource_names.> For
    /// example: `projects/my-project/locations/global`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results to return in one page.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListVmwareEngineNetworks` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListVmwareEngineNetworks` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that matches resources returned in the response.
    /// The expression must specify the field name, a comparison
    /// operator, and the value that you want to use for filtering. The value
    /// must be a string, a number, or a boolean. The comparison operator
    /// must be `=`, `!=`, `>`, or `<`.
    ///
    /// For example, if you are filtering a list of network peerings, you can
    /// exclude the ones named `example-network` by specifying
    /// `name != "example-network"`.
    ///
    /// To filter on multiple expressions, provide each separate expression within
    /// parentheses. For example:
    /// ```
    /// (name = "example-network")
    /// (createTime > "2021-04-12T08:15:10.40Z")
    /// ```
    ///
    /// By default, each expression is an `AND` expression. However, you
    /// can include `AND` and `OR` expressions explicitly.
    /// For example:
    /// ```
    /// (name = "example-network-1") AND
    /// (createTime > "2021-04-12T08:15:10.40Z") OR
    /// (name = "example-network-2")
    /// ```
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sorts list results by a certain order. By default, returned results
    /// are ordered by `name` in ascending order.
    /// You can also sort results in descending order based on the `name` value
    /// using `orderBy="name desc"`.
    /// Currently, only ordering by `name` is supported.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListVmwareEngineNetworks][google.cloud.vmwareengine.v1.VmwareEngine.ListVmwareEngineNetworks]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVmwareEngineNetworksResponse {
    /// A list of VMware Engine networks.
    #[prost(message, repeated, tag = "1")]
    pub vmware_engine_networks: ::prost::alloc::vec::Vec<VmwareEngineNetwork>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [VmwareEngine.CreatePrivateConnection][google.cloud.vmwareengine.v1.VmwareEngine.CreatePrivateConnection]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePrivateConnectionRequest {
    /// Required. The resource name of the location to create the new private
    /// connection in. Private connection is a regional resource.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.> For example:
    /// `projects/my-project/locations/us-central1`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The user-provided identifier of the new private connection.
    /// This identifier must be unique among private connection resources
    /// within the parent and becomes the final token in the name URI. The
    /// identifier must meet the following requirements:
    ///
    /// * Only contains 1-63 alphanumeric characters and hyphens
    /// * Begins with an alphabetical character
    /// * Ends with a non-hyphen character
    /// * Not formatted as a UUID
    /// * Complies with [RFC 1034](<https://datatracker.ietf.org/doc/html/rfc1034>)
    /// (section 3.5)
    #[prost(string, tag = "2")]
    pub private_connection_id: ::prost::alloc::string::String,
    /// Required. The initial description of the new private connection.
    #[prost(message, optional, tag = "3")]
    pub private_connection: ::core::option::Option<PrivateConnection>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.GetPrivateConnection][google.cloud.vmwareengine.v1.VmwareEngine.GetPrivateConnection]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivateConnectionRequest {
    /// Required. The resource name of the private connection to retrieve.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/privateConnections/my-connection`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ListPrivateConnections][google.cloud.vmwareengine.v1.VmwareEngine.ListPrivateConnections]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrivateConnectionsRequest {
    /// Required. The resource name of the location to query for
    /// private connections. Resource names are schemeless URIs that follow the
    /// conventions in <https://cloud.google.com/apis/design/resource_names.> For
    /// example: `projects/my-project/locations/us-central1`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of private connections to return in one page.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListPrivateConnections` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListPrivateConnections` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that matches resources returned in the response.
    /// The expression must specify the field name, a comparison
    /// operator, and the value that you want to use for filtering. The value
    /// must be a string, a number, or a boolean. The comparison operator
    /// must be `=`, `!=`, `>`, or `<`.
    ///
    /// For example, if you are filtering a list of private connections, you can
    /// exclude the ones named `example-connection` by specifying
    /// `name != "example-connection"`.
    ///
    /// To filter on multiple expressions, provide each separate expression within
    /// parentheses. For example:
    /// ```
    /// (name = "example-connection")
    /// (createTime > "2022-09-22T08:15:10.40Z")
    /// ```
    ///
    /// By default, each expression is an `AND` expression. However, you
    /// can include `AND` and `OR` expressions explicitly.
    /// For example:
    /// ```
    /// (name = "example-connection-1") AND
    /// (createTime > "2021-04-12T08:15:10.40Z") OR
    /// (name = "example-connection-2")
    /// ```
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sorts list results by a certain order. By default, returned results
    /// are ordered by `name` in ascending order.
    /// You can also sort results in descending order based on the `name` value
    /// using `orderBy="name desc"`.
    /// Currently, only ordering by `name` is supported.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListPrivateConnections][google.cloud.vmwareengine.v1.VmwareEngine.ListPrivateConnections]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrivateConnectionsResponse {
    /// A list of private connections.
    #[prost(message, repeated, tag = "1")]
    pub private_connections: ::prost::alloc::vec::Vec<PrivateConnection>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [VmwareEngine.UpdatePrivateConnection][google.cloud.vmwareengine.v1.VmwareEngine.UpdatePrivateConnection]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePrivateConnectionRequest {
    /// Required. Private connection description.
    #[prost(message, optional, tag = "1")]
    pub private_connection: ::core::option::Option<PrivateConnection>,
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `PrivateConnection` resource by the update.
    /// The fields specified in the `update_mask` are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.DeletePrivateConnection][google.cloud.vmwareengine.v1.VmwareEngine.DeletePrivateConnection]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePrivateConnectionRequest {
    /// Required. The resource name of the private connection to be deleted.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// For example:
    /// `projects/my-project/locations/us-central1/privateConnections/my-connection`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.ListPrivateConnectionPeeringRoutes][google.cloud.vmwareengine.v1.VmwareEngine.ListPrivateConnectionPeeringRoutes]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrivateConnectionPeeringRoutesRequest {
    /// Required. The resource name of the private connection to retrieve peering
    /// routes from. Resource names are schemeless URIs that follow the conventions
    /// in <https://cloud.google.com/apis/design/resource_names.> For example:
    /// `projects/my-project/locations/us-west1/privateConnections/my-connection`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of peering routes to return in one page.
    /// The service may return fewer than this value.
    /// The maximum value is coerced to 1000.
    /// The default value of this field is 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListPrivateConnectionPeeringRoutes`
    /// call. Provide this to retrieve the subsequent page. When paginating, all
    /// other parameters provided to `ListPrivateConnectionPeeringRoutes` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [VmwareEngine.ListPrivateConnectionPeeringRoutes][google.cloud.vmwareengine.v1.VmwareEngine.ListPrivateConnectionPeeringRoutes]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrivateConnectionPeeringRoutesResponse {
    /// A list of peering routes.
    #[prost(message, repeated, tag = "1")]
    pub peering_routes: ::prost::alloc::vec::Vec<PeeringRoute>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.GrantDnsBindPermission][google.cloud.vmwareengine.v1.VmwareEngine.GrantDnsBindPermission]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantDnsBindPermissionRequest {
    /// Required. The name of the resource which stores the users/service accounts
    /// having the permission to bind to the corresponding intranet VPC of the
    /// consumer project. DnsBindPermission is a global resource. Resource names
    /// are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.> For example:
    /// `projects/my-project/locations/global/dnsBindPermission`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The consumer provided user/service account which needs to be
    /// granted permission to bind with the intranet VPC corresponding to the
    /// consumer project.
    #[prost(message, optional, tag = "2")]
    pub principal: ::core::option::Option<Principal>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.RevokeDnsBindPermission][google.cloud.vmwareengine.v1.VmwareEngine.RevokeDnsBindPermission]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeDnsBindPermissionRequest {
    /// Required. The name of the resource which stores the users/service accounts
    /// having the permission to bind to the corresponding intranet VPC of the
    /// consumer project. DnsBindPermission is a global resource. Resource names
    /// are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.> For example:
    /// `projects/my-project/locations/global/dnsBindPermission`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The consumer provided user/service account which needs to be
    /// granted permission to bind with the intranet VPC corresponding to the
    /// consumer project.
    #[prost(message, optional, tag = "2")]
    pub principal: ::core::option::Option<Principal>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server guarantees that a
    /// request doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [VmwareEngine.GetDnsBindPermission][google.cloud.vmwareengine.v1.VmwareEngine.GetDnsBindPermission]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDnsBindPermissionRequest {
    /// Required. The name of the resource which stores the users/service accounts
    /// having the permission to bind to the corresponding intranet VPC of the
    /// consumer project. DnsBindPermission is a global resource. Resource names
    /// are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.> For example:
    /// `projects/my-project/locations/global/dnsBindPermission`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod vmware_engine_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// VMwareEngine manages VMware's private clusters in the Cloud.
    #[derive(Debug, Clone)]
    pub struct VmwareEngineClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VmwareEngineClient<T>
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
        ) -> VmwareEngineClient<InterceptedService<T, F>>
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
            VmwareEngineClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists `PrivateCloud` resources in a given project and location.
        pub async fn list_private_clouds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPrivateCloudsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPrivateCloudsResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListPrivateClouds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListPrivateClouds",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a `PrivateCloud` resource by its resource name.
        pub async fn get_private_cloud(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPrivateCloudRequest>,
        ) -> std::result::Result<tonic::Response<super::PrivateCloud>, tonic::Status> {
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetPrivateCloud",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetPrivateCloud",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new `PrivateCloud` resource in a given project and location.
        /// Private clouds of type `STANDARD` and
        /// `TIME_LIMITED` are zonal resources, `STRETCHED` private clouds are
        /// regional.
        /// Creating a private cloud also creates a [management
        /// cluster](https://cloud.google.com/vmware-engine/docs/concepts-vmware-components)
        /// for that private cloud.
        pub async fn create_private_cloud(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePrivateCloudRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/CreatePrivateCloud",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "CreatePrivateCloud",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Modifies a `PrivateCloud` resource. Only the following fields can be
        /// updated: `description`.
        /// Only fields specified in `updateMask` are applied.
        ///
        /// During operation processing, the resource is temporarily in the `ACTIVE`
        /// state before the operation fully completes. For that period of time, you
        /// can't update the resource. Use the operation status to determine when the
        /// processing fully completes.
        pub async fn update_private_cloud(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePrivateCloudRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/UpdatePrivateCloud",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "UpdatePrivateCloud",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Schedules a `PrivateCloud` resource for deletion.
        ///
        /// A `PrivateCloud` resource scheduled for deletion has `PrivateCloud.state`
        /// set to `DELETED` and `expireTime` set to the time when deletion is final
        /// and can no longer be reversed. The delete operation is marked as done
        /// as soon as the `PrivateCloud` is successfully scheduled for deletion
        /// (this also applies when `delayHours` is set to zero), and the operation is
        /// not kept in pending state until `PrivateCloud` is purged.
        /// `PrivateCloud` can be restored using `UndeletePrivateCloud` method before
        /// the `expireTime` elapses. When `expireTime` is reached, deletion is final
        /// and all private cloud resources are irreversibly removed and billing stops.
        /// During the final removal process, `PrivateCloud.state` is set to `PURGING`.
        /// `PrivateCloud` can be polled using standard `GET` method for the whole
        /// period of deletion and purging. It will not be returned only
        /// when it is completely purged.
        pub async fn delete_private_cloud(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePrivateCloudRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/DeletePrivateCloud",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "DeletePrivateCloud",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Restores a private cloud that was previously scheduled for deletion by
        /// `DeletePrivateCloud`. A `PrivateCloud` resource scheduled for deletion has
        /// `PrivateCloud.state` set to `DELETED` and `PrivateCloud.expireTime` set to
        /// the time when deletion can no longer be reversed.
        pub async fn undelete_private_cloud(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeletePrivateCloudRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/UndeletePrivateCloud",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "UndeletePrivateCloud",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists `Cluster` resources in a given private cloud.
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListClusters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListClusters",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a `Cluster` resource by its resource name.
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new cluster in a given private cloud.
        /// Creating a new cluster provides additional nodes for
        /// use in the parent private cloud and requires sufficient [node
        /// quota](https://cloud.google.com/vmware-engine/quotas).
        pub async fn create_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClusterRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/CreateCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "CreateCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Modifies a `Cluster` resource. Only fields specified in `updateMask` are
        /// applied.
        ///
        /// During operation processing, the resource is temporarily in the `ACTIVE`
        /// state before the operation fully completes. For that period of time, you
        /// can't update the resource. Use the operation status to determine when the
        /// processing fully completes.
        pub async fn update_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateClusterRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/UpdateCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "UpdateCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a `Cluster` resource. To avoid unintended data loss, migrate or
        /// gracefully shut down any workloads running on the cluster before deletion.
        /// You cannot delete the management cluster of a private cloud using this
        /// method.
        pub async fn delete_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClusterRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/DeleteCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "DeleteCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists nodes in a given cluster.
        pub async fn list_nodes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNodesResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListNodes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListNodes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single node.
        pub async fn get_node(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodeRequest>,
        ) -> std::result::Result<tonic::Response<super::Node>, tonic::Status> {
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetNode",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists external IP addresses assigned to VMware workload VMs in a given
        /// private cloud.
        pub async fn list_external_addresses(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExternalAddressesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExternalAddressesResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListExternalAddresses",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListExternalAddresses",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists external IP addresses assigned to VMware workload VMs within the
        /// scope of the given network policy.
        pub async fn fetch_network_policy_external_addresses(
            &mut self,
            request: impl tonic::IntoRequest<
                super::FetchNetworkPolicyExternalAddressesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::FetchNetworkPolicyExternalAddressesResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/FetchNetworkPolicyExternalAddresses",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "FetchNetworkPolicyExternalAddresses",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single external IP address.
        pub async fn get_external_address(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExternalAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExternalAddress>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetExternalAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetExternalAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new `ExternalAddress` resource in a given private cloud. The
        /// network policy that corresponds to the private cloud must have the external
        /// IP address network service enabled (`NetworkPolicy.external_ip`).
        pub async fn create_external_address(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExternalAddressRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/CreateExternalAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "CreateExternalAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single external IP address.
        /// Only fields specified in `update_mask` are applied.
        ///
        /// During operation processing, the resource is temporarily in the `ACTIVE`
        /// state before the operation fully completes. For that period of time, you
        /// can't update the resource. Use the operation status to determine when the
        /// processing fully completes.
        pub async fn update_external_address(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateExternalAddressRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/UpdateExternalAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "UpdateExternalAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single external IP address. When you delete an external IP
        /// address, connectivity between the external IP address and the corresponding
        /// internal IP address is lost.
        pub async fn delete_external_address(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteExternalAddressRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/DeleteExternalAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "DeleteExternalAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists subnets in a given private cloud.
        pub async fn list_subnets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSubnetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSubnetsResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListSubnets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListSubnets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single subnet.
        pub async fn get_subnet(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSubnetRequest>,
        ) -> std::result::Result<tonic::Response<super::Subnet>, tonic::Status> {
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetSubnet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetSubnet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single subnet. Only fields specified in
        /// `update_mask` are applied.
        ///
        /// *Note*: This API is synchronous and always returns a successful
        /// `google.longrunning.Operation` (LRO). The returned LRO will only have
        /// `done` and `response` fields.
        pub async fn update_subnet(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSubnetRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/UpdateSubnet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "UpdateSubnet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists `ExternalAccessRule` resources in the specified network policy.
        pub async fn list_external_access_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExternalAccessRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExternalAccessRulesResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListExternalAccessRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListExternalAccessRules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single external access rule.
        pub async fn get_external_access_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExternalAccessRuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExternalAccessRule>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetExternalAccessRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetExternalAccessRule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new external access rule in a given network policy.
        pub async fn create_external_access_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExternalAccessRuleRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/CreateExternalAccessRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "CreateExternalAccessRule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single external access rule.
        /// Only fields specified in `update_mask` are applied.
        pub async fn update_external_access_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateExternalAccessRuleRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/UpdateExternalAccessRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "UpdateExternalAccessRule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single external access rule.
        pub async fn delete_external_access_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteExternalAccessRuleRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/DeleteExternalAccessRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "DeleteExternalAccessRule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists logging servers configured for a given private
        /// cloud.
        pub async fn list_logging_servers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLoggingServersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLoggingServersResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListLoggingServers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListLoggingServers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a logging server.
        pub async fn get_logging_server(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLoggingServerRequest>,
        ) -> std::result::Result<tonic::Response<super::LoggingServer>, tonic::Status> {
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetLoggingServer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetLoggingServer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create a new logging server for a given private cloud.
        pub async fn create_logging_server(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLoggingServerRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/CreateLoggingServer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "CreateLoggingServer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single logging server.
        /// Only fields specified in `update_mask` are applied.
        pub async fn update_logging_server(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLoggingServerRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/UpdateLoggingServer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "UpdateLoggingServer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single logging server.
        pub async fn delete_logging_server(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLoggingServerRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/DeleteLoggingServer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "DeleteLoggingServer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists node types
        pub async fn list_node_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNodeTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNodeTypesResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListNodeTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListNodeTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single `NodeType`.
        pub async fn get_node_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodeTypeRequest>,
        ) -> std::result::Result<tonic::Response<super::NodeType>, tonic::Status> {
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetNodeType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetNodeType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of credentials for NSX appliance.
        pub async fn show_nsx_credentials(
            &mut self,
            request: impl tonic::IntoRequest<super::ShowNsxCredentialsRequest>,
        ) -> std::result::Result<tonic::Response<super::Credentials>, tonic::Status> {
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ShowNsxCredentials",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ShowNsxCredentials",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of credentials for Vcenter appliance.
        pub async fn show_vcenter_credentials(
            &mut self,
            request: impl tonic::IntoRequest<super::ShowVcenterCredentialsRequest>,
        ) -> std::result::Result<tonic::Response<super::Credentials>, tonic::Status> {
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ShowVcenterCredentials",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ShowVcenterCredentials",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Resets credentials of the NSX appliance.
        pub async fn reset_nsx_credentials(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetNsxCredentialsRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ResetNsxCredentials",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ResetNsxCredentials",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Resets credentials of the Vcenter appliance.
        pub async fn reset_vcenter_credentials(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetVcenterCredentialsRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ResetVcenterCredentials",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ResetVcenterCredentials",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of the `DnsForwarding` config.
        pub async fn get_dns_forwarding(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDnsForwardingRequest>,
        ) -> std::result::Result<tonic::Response<super::DnsForwarding>, tonic::Status> {
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetDnsForwarding",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetDnsForwarding",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of the `DnsForwarding` config, like associated
        /// domains. Only fields specified in `update_mask` are applied.
        pub async fn update_dns_forwarding(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDnsForwardingRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/UpdateDnsForwarding",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "UpdateDnsForwarding",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a `NetworkPeering` resource by its resource name. The resource
        /// contains details of the network peering, such as peered
        /// networks, import and export custom route configurations, and peering state.
        /// NetworkPeering is a global resource and location can only be global.
        pub async fn get_network_peering(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNetworkPeeringRequest>,
        ) -> std::result::Result<tonic::Response<super::NetworkPeering>, tonic::Status> {
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetNetworkPeering",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetNetworkPeering",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists `NetworkPeering` resources in a given project. NetworkPeering is a
        /// global resource and location can only be global.
        pub async fn list_network_peerings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNetworkPeeringsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNetworkPeeringsResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListNetworkPeerings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListNetworkPeerings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new network peering between the peer network and VMware Engine
        /// network provided in a `NetworkPeering` resource. NetworkPeering is a
        /// global resource and location can only be global.
        pub async fn create_network_peering(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNetworkPeeringRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/CreateNetworkPeering",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "CreateNetworkPeering",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a `NetworkPeering` resource. When a network peering is deleted for
        /// a VMware Engine network, the peer network becomes inaccessible to that
        /// VMware Engine network. NetworkPeering is a global resource and location can
        /// only be global.
        pub async fn delete_network_peering(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNetworkPeeringRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/DeleteNetworkPeering",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "DeleteNetworkPeering",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Modifies a `NetworkPeering` resource. Only the `description` field can be
        /// updated. Only fields specified in `updateMask` are applied. NetworkPeering
        /// is a global resource and location can only be global.
        pub async fn update_network_peering(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNetworkPeeringRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/UpdateNetworkPeering",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "UpdateNetworkPeering",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the network peering routes exchanged over a peering connection.
        /// NetworkPeering is a global resource and location can only be global.
        pub async fn list_peering_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPeeringRoutesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPeeringRoutesResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListPeeringRoutes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListPeeringRoutes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new HCX activation key in a given private cloud.
        pub async fn create_hcx_activation_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateHcxActivationKeyRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/CreateHcxActivationKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "CreateHcxActivationKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists `HcxActivationKey` resources in a given private cloud.
        pub async fn list_hcx_activation_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHcxActivationKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListHcxActivationKeysResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListHcxActivationKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListHcxActivationKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a `HcxActivationKey` resource by its resource name.
        pub async fn get_hcx_activation_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHcxActivationKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HcxActivationKey>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetHcxActivationKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetHcxActivationKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a `NetworkPolicy` resource by its resource name.
        pub async fn get_network_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNetworkPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::NetworkPolicy>, tonic::Status> {
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetNetworkPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetNetworkPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists `NetworkPolicy` resources in a specified project and location.
        pub async fn list_network_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNetworkPoliciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNetworkPoliciesResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListNetworkPolicies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListNetworkPolicies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new network policy in a given VMware Engine network of a
        /// project and location (region). A new network policy cannot be created if
        /// another network policy already exists in the same scope.
        pub async fn create_network_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNetworkPolicyRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/CreateNetworkPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "CreateNetworkPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Modifies a `NetworkPolicy` resource. Only the following fields can be
        /// updated: `internet_access`, `external_ip`, `edge_services_cidr`.
        /// Only fields specified in `updateMask` are applied. When updating a network
        /// policy, the external IP network service can only be disabled if there are
        /// no external IP addresses present in the scope of the policy. Also, a
        /// `NetworkService` cannot be updated when `NetworkService.state` is set
        /// to `RECONCILING`.
        ///
        /// During operation processing, the resource is temporarily in the `ACTIVE`
        /// state before the operation fully completes. For that period of time, you
        /// can't update the resource. Use the operation status to determine when the
        /// processing fully completes.
        pub async fn update_network_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNetworkPolicyRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/UpdateNetworkPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "UpdateNetworkPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a `NetworkPolicy` resource. A network policy cannot be deleted
        /// when `NetworkService.state` is set to `RECONCILING` for either its external
        /// IP or internet access service.
        pub async fn delete_network_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNetworkPolicyRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/DeleteNetworkPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "DeleteNetworkPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Consumer VPCs bound to Management DNS Zone of a given private cloud.
        pub async fn list_management_dns_zone_bindings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListManagementDnsZoneBindingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListManagementDnsZoneBindingsResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListManagementDnsZoneBindings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListManagementDnsZoneBindings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a 'ManagementDnsZoneBinding' resource by its resource name.
        pub async fn get_management_dns_zone_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::GetManagementDnsZoneBindingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ManagementDnsZoneBinding>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetManagementDnsZoneBinding",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetManagementDnsZoneBinding",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new `ManagementDnsZoneBinding` resource in a private cloud.
        /// This RPC creates the DNS binding and the resource that represents the
        /// DNS binding of the consumer VPC network to the management DNS zone. A
        /// management DNS zone is the Cloud DNS cross-project binding zone that
        /// VMware Engine creates for each private cloud. It contains FQDNs and
        /// corresponding IP addresses for the private cloud's ESXi hosts and
        /// management VM appliances like vCenter and NSX Manager.
        pub async fn create_management_dns_zone_binding(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateManagementDnsZoneBindingRequest,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/CreateManagementDnsZoneBinding",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "CreateManagementDnsZoneBinding",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a `ManagementDnsZoneBinding` resource.
        /// Only fields specified in `update_mask` are applied.
        pub async fn update_management_dns_zone_binding(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateManagementDnsZoneBindingRequest,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/UpdateManagementDnsZoneBinding",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "UpdateManagementDnsZoneBinding",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a `ManagementDnsZoneBinding` resource. When a management DNS zone
        /// binding is deleted, the corresponding consumer VPC network is no longer
        /// bound to the management DNS zone.
        pub async fn delete_management_dns_zone_binding(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DeleteManagementDnsZoneBindingRequest,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/DeleteManagementDnsZoneBinding",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "DeleteManagementDnsZoneBinding",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retries to create a `ManagementDnsZoneBinding` resource that is
        /// in failed state.
        pub async fn repair_management_dns_zone_binding(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RepairManagementDnsZoneBindingRequest,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/RepairManagementDnsZoneBinding",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "RepairManagementDnsZoneBinding",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new VMware Engine network that can be used by a private cloud.
        pub async fn create_vmware_engine_network(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVmwareEngineNetworkRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/CreateVmwareEngineNetwork",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "CreateVmwareEngineNetwork",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Modifies a VMware Engine network resource. Only the following fields can be
        /// updated: `description`. Only fields specified in `updateMask` are
        /// applied.
        pub async fn update_vmware_engine_network(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVmwareEngineNetworkRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/UpdateVmwareEngineNetwork",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "UpdateVmwareEngineNetwork",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a `VmwareEngineNetwork` resource. You can only delete a VMware
        /// Engine network after all resources that refer to it are deleted. For
        /// example, a private cloud, a network peering, and a network policy can all
        /// refer to the same VMware Engine network.
        pub async fn delete_vmware_engine_network(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVmwareEngineNetworkRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/DeleteVmwareEngineNetwork",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "DeleteVmwareEngineNetwork",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a `VmwareEngineNetwork` resource by its resource name. The
        /// resource contains details of the VMware Engine network, such as its VMware
        /// Engine network type, peered networks in a service project, and state
        /// (for example, `CREATING`, `ACTIVE`, `DELETING`).
        pub async fn get_vmware_engine_network(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVmwareEngineNetworkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VmwareEngineNetwork>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetVmwareEngineNetwork",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetVmwareEngineNetwork",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists `VmwareEngineNetwork` resources in a given project and location.
        pub async fn list_vmware_engine_networks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVmwareEngineNetworksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVmwareEngineNetworksResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListVmwareEngineNetworks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListVmwareEngineNetworks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new private connection that can be used for accessing private
        /// Clouds.
        pub async fn create_private_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePrivateConnectionRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/CreatePrivateConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "CreatePrivateConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a `PrivateConnection` resource by its resource name. The resource
        /// contains details of the private connection, such as connected
        /// network, routing mode and state.
        pub async fn get_private_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPrivateConnectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PrivateConnection>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetPrivateConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetPrivateConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists `PrivateConnection` resources in a given project and location.
        pub async fn list_private_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPrivateConnectionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPrivateConnectionsResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListPrivateConnections",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListPrivateConnections",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Modifies a `PrivateConnection` resource. Only `description` and
        /// `routing_mode` fields can be updated. Only fields specified in `updateMask`
        /// are applied.
        pub async fn update_private_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePrivateConnectionRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/UpdatePrivateConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "UpdatePrivateConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a `PrivateConnection` resource. When a private connection is
        /// deleted for a VMware Engine network, the connected network becomes
        /// inaccessible to that VMware Engine network.
        pub async fn delete_private_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePrivateConnectionRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/DeletePrivateConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "DeletePrivateConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the private connection routes exchanged over a peering connection.
        pub async fn list_private_connection_peering_routes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListPrivateConnectionPeeringRoutesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListPrivateConnectionPeeringRoutesResponse>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/ListPrivateConnectionPeeringRoutes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "ListPrivateConnectionPeeringRoutes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Grants the bind permission to the customer provided principal(user /
        /// service account) to bind their DNS zone with the intranet VPC associated
        /// with the project. DnsBindPermission is a global resource and location can
        /// only be global.
        pub async fn grant_dns_bind_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::GrantDnsBindPermissionRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GrantDnsBindPermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GrantDnsBindPermission",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets all the principals having bind permission on the intranet VPC
        /// associated with the consumer project granted by the Grant API.
        /// DnsBindPermission is a global resource and location can only be global.
        pub async fn get_dns_bind_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDnsBindPermissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DnsBindPermission>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/GetDnsBindPermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "GetDnsBindPermission",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Revokes the bind permission from the customer provided principal(user /
        /// service account) on the intranet VPC associated with the consumer project.
        /// DnsBindPermission is a global resource and location can only be global.
        pub async fn revoke_dns_bind_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeDnsBindPermissionRequest>,
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
                "/google.cloud.vmwareengine.v1.VmwareEngine/RevokeDnsBindPermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.vmwareengine.v1.VmwareEngine",
                        "RevokeDnsBindPermission",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
