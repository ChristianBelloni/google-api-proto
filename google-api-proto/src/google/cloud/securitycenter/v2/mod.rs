/// Represents database access information, such as queries. A database may be a
/// sub-resource of an instance (as in the case of Cloud SQL instances or Cloud
/// Spanner instances), or the database instance itself. Some database resources
/// might not have the [full resource
/// name](<https://google.aip.dev/122#full-resource-names>) populated because these
/// resource types, such as Cloud SQL databases, are not yet supported by Cloud
/// Asset Inventory. In these cases only the display name is provided.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
    /// Some database resources may not have the [full resource
    /// name](<https://google.aip.dev/122#full-resource-names>) populated because
    /// these resource types are not yet supported by Cloud Asset Inventory (e.g.
    /// Cloud SQL databases). In these cases only the display name will be
    /// provided.
    /// The [full resource name](<https://google.aip.dev/122#full-resource-names>) of
    /// the database that the user connected to, if it is supported by Cloud Asset
    /// Inventory.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The human-readable name of the database that the user connected to.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The username used to connect to the database. The username might not be an
    /// IAM principal and does not have a set format.
    #[prost(string, tag = "3")]
    pub user_name: ::prost::alloc::string::String,
    /// The SQL statement that is associated with the database access.
    #[prost(string, tag = "4")]
    pub query: ::prost::alloc::string::String,
    /// The target usernames, roles, or groups of an SQL privilege grant, which is
    /// not an IAM policy change.
    #[prost(string, repeated, tag = "5")]
    pub grantees: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The version of the database, for example, POSTGRES_14.
    /// See [the complete
    /// list](<https://cloud.google.com/sql/docs/mysql/admin-api/rest/v1/SqlDatabaseVersion>).
    #[prost(string, tag = "6")]
    pub version: ::prost::alloc::string::String,
}
/// The [data profile](<https://cloud.google.com/dlp/docs/data-profiles>)
/// associated with the finding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudDlpDataProfile {
    /// Name of the data profile, for example,
    /// `projects/123/locations/europe/tableProfiles/8383929`.
    #[prost(string, tag = "1")]
    pub data_profile: ::prost::alloc::string::String,
    /// The resource hierarchy level at which the data profile was generated.
    #[prost(enumeration = "cloud_dlp_data_profile::ParentType", tag = "2")]
    pub parent_type: i32,
}
/// Nested message and enum types in `CloudDlpDataProfile`.
pub mod cloud_dlp_data_profile {
    /// Parents for configurations that produce data profile findings.
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
    pub enum ParentType {
        /// Unspecified parent type.
        Unspecified = 0,
        /// Organization-level configurations.
        Organization = 1,
        /// Project-level configurations.
        Project = 2,
    }
    impl ParentType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ParentType::Unspecified => "PARENT_TYPE_UNSPECIFIED",
                ParentType::Organization => "ORGANIZATION",
                ParentType::Project => "PROJECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PARENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ORGANIZATION" => Some(Self::Organization),
                "PROJECT" => Some(Self::Project),
                _ => None,
            }
        }
    }
}
/// Contains information about the org policies associated with the finding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrgPolicy {
    /// The resource name of the org policy.
    /// Example:
    /// "organizations/{organization_id}/policies/{constraint_name}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents an application associated with a finding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Application {
    /// The base URI that identifies the network location of the application in
    /// which the vulnerability was detected. For example, `<http://example.com`.>
    #[prost(string, tag = "1")]
    pub base_uri: ::prost::alloc::string::String,
    /// The full URI with payload that could be used to reproduce the
    /// vulnerability. For example, `<http://example.com?p=aMmYgI6H`.>
    #[prost(string, tag = "2")]
    pub full_uri: ::prost::alloc::string::String,
}
/// Represents a generic name-value label. A label has separate name and value
/// fields to support filtering with the `contains()` function. For more
/// information, see [Filtering on array-type
/// fields](<https://cloud.google.com/security-command-center/docs/how-to-api-list-findings#array-contains-filtering>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    /// Name of the label.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Value that corresponds to the label's name.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Container associated with the finding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    /// Name of the container.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Container image URI provided when configuring a pod or container. This
    /// string can identify a container image version using mutable tags.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Optional container image ID, if provided by the container runtime. Uniquely
    /// identifies the container image launched using a container image digest.
    #[prost(string, tag = "3")]
    pub image_id: ::prost::alloc::string::String,
    /// Container labels, as provided by the container runtime.
    #[prost(message, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    /// The time that the container was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Kubernetes-related attributes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Kubernetes {
    /// Kubernetes
    /// [Pods](<https://cloud.google.com/kubernetes-engine/docs/concepts/pod>)
    /// associated with the finding. This field contains Pod records for each
    /// container that is owned by a Pod.
    #[prost(message, repeated, tag = "1")]
    pub pods: ::prost::alloc::vec::Vec<kubernetes::Pod>,
    /// Provides Kubernetes
    /// [node](<https://cloud.google.com/kubernetes-engine/docs/concepts/cluster-architecture#nodes>)
    /// information.
    #[prost(message, repeated, tag = "2")]
    pub nodes: ::prost::alloc::vec::Vec<kubernetes::Node>,
    /// GKE [node
    /// pools](<https://cloud.google.com/kubernetes-engine/docs/concepts/node-pools>)
    /// associated with the finding. This field contains node pool information for
    /// each node, when it is available.
    #[prost(message, repeated, tag = "3")]
    pub node_pools: ::prost::alloc::vec::Vec<kubernetes::NodePool>,
    /// Provides Kubernetes role information for findings that involve [Roles or
    /// ClusterRoles](<https://cloud.google.com/kubernetes-engine/docs/how-to/role-based-access-control>).
    #[prost(message, repeated, tag = "4")]
    pub roles: ::prost::alloc::vec::Vec<kubernetes::Role>,
    /// Provides Kubernetes role binding information for findings that involve
    /// [RoleBindings or
    /// ClusterRoleBindings](<https://cloud.google.com/kubernetes-engine/docs/how-to/role-based-access-control>).
    #[prost(message, repeated, tag = "5")]
    pub bindings: ::prost::alloc::vec::Vec<kubernetes::Binding>,
    /// Provides information on any Kubernetes access reviews (privilege checks)
    /// relevant to the finding.
    #[prost(message, repeated, tag = "6")]
    pub access_reviews: ::prost::alloc::vec::Vec<kubernetes::AccessReview>,
    /// Kubernetes objects related to the finding.
    #[prost(message, repeated, tag = "7")]
    pub objects: ::prost::alloc::vec::Vec<kubernetes::Object>,
}
/// Nested message and enum types in `Kubernetes`.
pub mod kubernetes {
    /// A Kubernetes Pod.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Pod {
        /// Kubernetes Pod namespace.
        #[prost(string, tag = "1")]
        pub ns: ::prost::alloc::string::String,
        /// Kubernetes Pod name.
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        /// Pod labels.  For Kubernetes containers, these are applied to the
        /// container.
        #[prost(message, repeated, tag = "3")]
        pub labels: ::prost::alloc::vec::Vec<super::Label>,
        /// Pod containers associated with this finding, if any.
        #[prost(message, repeated, tag = "4")]
        pub containers: ::prost::alloc::vec::Vec<super::Container>,
    }
    /// Kubernetes nodes associated with the finding.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        /// [Full resource name](<https://google.aip.dev/122#full-resource-names>) of
        /// the Compute Engine VM running the cluster node.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
    }
    /// Provides GKE node pool information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NodePool {
        /// Kubernetes node pool name.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Nodes associated with the finding.
        #[prost(message, repeated, tag = "2")]
        pub nodes: ::prost::alloc::vec::Vec<Node>,
    }
    /// Kubernetes Role or ClusterRole.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Role {
        /// Role type.
        #[prost(enumeration = "role::Kind", tag = "1")]
        pub kind: i32,
        /// Role namespace.
        #[prost(string, tag = "2")]
        pub ns: ::prost::alloc::string::String,
        /// Role name.
        #[prost(string, tag = "3")]
        pub name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Role`.
    pub mod role {
        /// Types of Kubernetes roles.
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
            /// Role type is not specified.
            Unspecified = 0,
            /// Kubernetes Role.
            Role = 1,
            /// Kubernetes ClusterRole.
            ClusterRole = 2,
        }
        impl Kind {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Kind::Unspecified => "KIND_UNSPECIFIED",
                    Kind::Role => "ROLE",
                    Kind::ClusterRole => "CLUSTER_ROLE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "KIND_UNSPECIFIED" => Some(Self::Unspecified),
                    "ROLE" => Some(Self::Role),
                    "CLUSTER_ROLE" => Some(Self::ClusterRole),
                    _ => None,
                }
            }
        }
    }
    /// Represents a Kubernetes RoleBinding or ClusterRoleBinding.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Binding {
        /// Namespace for the binding.
        #[prost(string, tag = "1")]
        pub ns: ::prost::alloc::string::String,
        /// Name for the binding.
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        /// The Role or ClusterRole referenced by the binding.
        #[prost(message, optional, tag = "3")]
        pub role: ::core::option::Option<Role>,
        /// Represents one or more subjects that are bound to the role. Not always
        /// available for PATCH requests.
        #[prost(message, repeated, tag = "4")]
        pub subjects: ::prost::alloc::vec::Vec<Subject>,
    }
    /// Represents a Kubernetes subject.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Subject {
        /// Authentication type for the subject.
        #[prost(enumeration = "subject::AuthType", tag = "1")]
        pub kind: i32,
        /// Namespace for the subject.
        #[prost(string, tag = "2")]
        pub ns: ::prost::alloc::string::String,
        /// Name for the subject.
        #[prost(string, tag = "3")]
        pub name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Subject`.
    pub mod subject {
        /// Auth types that can be used for the subject's kind field.
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
            /// Authentication is not specified.
            Unspecified = 0,
            /// User with valid certificate.
            User = 1,
            /// Users managed by Kubernetes API with credentials stored as secrets.
            Serviceaccount = 2,
            /// Collection of users.
            Group = 3,
        }
        impl AuthType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    AuthType::Unspecified => "AUTH_TYPE_UNSPECIFIED",
                    AuthType::User => "USER",
                    AuthType::Serviceaccount => "SERVICEACCOUNT",
                    AuthType::Group => "GROUP",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "AUTH_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "USER" => Some(Self::User),
                    "SERVICEACCOUNT" => Some(Self::Serviceaccount),
                    "GROUP" => Some(Self::Group),
                    _ => None,
                }
            }
        }
    }
    /// Conveys information about a Kubernetes access review (such as one returned
    /// by a [`kubectl auth
    /// can-i`](<https://kubernetes.io/docs/reference/access-authn-authz/authorization/#checking-api-access>)
    /// command) that was involved in a finding.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessReview {
        /// The API group of the resource. "*" means all.
        #[prost(string, tag = "1")]
        pub group: ::prost::alloc::string::String,
        /// Namespace of the action being requested. Currently, there is no
        /// distinction between no namespace and all namespaces.  Both
        /// are represented by "" (empty).
        #[prost(string, tag = "2")]
        pub ns: ::prost::alloc::string::String,
        /// The name of the resource being requested. Empty means all.
        #[prost(string, tag = "3")]
        pub name: ::prost::alloc::string::String,
        /// The optional resource type requested. "*" means all.
        #[prost(string, tag = "4")]
        pub resource: ::prost::alloc::string::String,
        /// The optional subresource type.
        #[prost(string, tag = "5")]
        pub subresource: ::prost::alloc::string::String,
        /// A Kubernetes resource API verb, like get, list, watch, create, update,
        /// delete, proxy. "*" means all.
        #[prost(string, tag = "6")]
        pub verb: ::prost::alloc::string::String,
        /// The API version of the resource. "*" means all.
        #[prost(string, tag = "7")]
        pub version: ::prost::alloc::string::String,
    }
    /// Kubernetes object related to the finding, uniquely identified by GKNN.
    /// Used if the object Kind is not one of Pod, Node, NodePool, Binding, or
    /// AccessReview.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Object {
        /// Kubernetes object group, such as "policy.k8s.io/v1".
        #[prost(string, tag = "1")]
        pub group: ::prost::alloc::string::String,
        /// Kubernetes object kind, such as "Namespace".
        #[prost(string, tag = "2")]
        pub kind: ::prost::alloc::string::String,
        /// Kubernetes object namespace. Must be a valid DNS label. Named
        /// "ns" to avoid collision with C++ namespace keyword. For details see
        /// <https://kubernetes.io/docs/tasks/administer-cluster/namespaces/.>
        #[prost(string, tag = "3")]
        pub ns: ::prost::alloc::string::String,
        /// Kubernetes object name. For details see
        /// <https://kubernetes.io/docs/concepts/overview/working-with-objects/names/.>
        #[prost(string, tag = "4")]
        pub name: ::prost::alloc::string::String,
        /// Pod containers associated with this finding, if any.
        #[prost(message, repeated, tag = "5")]
        pub containers: ::prost::alloc::vec::Vec<super::Container>,
    }
}
/// A resource that is determined to have value to a user's system
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValuedResource {
    /// Valued resource name, for example,
    ///   e.g.:
    ///   `organizations/123/simulations/456/valuedResources/789`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The
    /// [full resource
    /// name](<https://cloud.google.com/apis/design/resource_names#full_resource_name>)
    /// of the valued resource.
    #[prost(string, tag = "2")]
    pub resource: ::prost::alloc::string::String,
    /// The [resource
    /// type](<https://cloud.google.com/asset-inventory/docs/supported-asset-types>)
    /// of the valued resource.
    #[prost(string, tag = "3")]
    pub resource_type: ::prost::alloc::string::String,
    /// Human-readable name of the valued resource.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// How valuable this resource is.
    #[prost(enumeration = "valued_resource::ResourceValue", tag = "5")]
    pub resource_value: i32,
    /// Exposed score for this valued resource. A value of 0 means no exposure was
    /// detected exposure.
    #[prost(double, tag = "6")]
    pub exposed_score: f64,
    /// List of resource value configurations' metadata used to determine the value
    /// of this resource. Maximum of 100.
    #[prost(message, repeated, tag = "7")]
    pub resource_value_configs_used: ::prost::alloc::vec::Vec<
        ResourceValueConfigMetadata,
    >,
}
/// Nested message and enum types in `ValuedResource`.
pub mod valued_resource {
    /// How valuable the resource is.
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
    pub enum ResourceValue {
        /// The resource value isn't specified.
        Unspecified = 0,
        /// This is a low-value resource.
        Low = 1,
        /// This is a medium-value resource.
        Medium = 2,
        /// This is a high-value resource.
        High = 3,
    }
    impl ResourceValue {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResourceValue::Unspecified => "RESOURCE_VALUE_UNSPECIFIED",
                ResourceValue::Low => "RESOURCE_VALUE_LOW",
                ResourceValue::Medium => "RESOURCE_VALUE_MEDIUM",
                ResourceValue::High => "RESOURCE_VALUE_HIGH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESOURCE_VALUE_UNSPECIFIED" => Some(Self::Unspecified),
                "RESOURCE_VALUE_LOW" => Some(Self::Low),
                "RESOURCE_VALUE_MEDIUM" => Some(Self::Medium),
                "RESOURCE_VALUE_HIGH" => Some(Self::High),
                _ => None,
            }
        }
    }
}
/// Metadata about a ResourceValueConfig. For example, id and name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceValueConfigMetadata {
    /// Resource value config name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Attack path simulation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Simulation {
    /// Full resource name of the Simulation:
    /// organizations/123/simulations/456
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time simulation was created
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Resource value configurations' metadata used in this simulation. Maximum of
    /// 100.
    #[prost(message, repeated, tag = "3")]
    pub resource_value_configs_metadata: ::prost::alloc::vec::Vec<
        ResourceValueConfigMetadata,
    >,
}
/// Represents a particular IAM binding, which captures a member's role addition,
/// removal, or state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamBinding {
    /// The action that was performed on a Binding.
    #[prost(enumeration = "iam_binding::Action", tag = "1")]
    pub action: i32,
    /// Role that is assigned to "members".
    /// For example, "roles/viewer", "roles/editor", or "roles/owner".
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
    /// A single identity requesting access for a Cloud Platform resource, for
    /// example, "foo@google.com".
    #[prost(string, tag = "3")]
    pub member: ::prost::alloc::string::String,
}
/// Nested message and enum types in `IamBinding`.
pub mod iam_binding {
    /// The type of action performed on a Binding in a policy.
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
        /// Unspecified.
        Unspecified = 0,
        /// Addition of a Binding.
        Add = 1,
        /// Removal of a Binding.
        Remove = 2,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::Unspecified => "ACTION_UNSPECIFIED",
                Action::Add => "ADD",
                Action::Remove => "REMOVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "ADD" => Some(Self::Add),
                "REMOVE" => Some(Self::Remove),
                _ => None,
            }
        }
    }
}
/// Contains information about the IP connection associated with the finding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    /// Destination IP address. Not present for sockets that are listening and not
    /// connected.
    #[prost(string, tag = "1")]
    pub destination_ip: ::prost::alloc::string::String,
    /// Destination port. Not present for sockets that are listening and not
    /// connected.
    #[prost(int32, tag = "2")]
    pub destination_port: i32,
    /// Source IP address.
    #[prost(string, tag = "3")]
    pub source_ip: ::prost::alloc::string::String,
    /// Source port.
    #[prost(int32, tag = "4")]
    pub source_port: i32,
    /// IANA Internet Protocol Number such as TCP(6) and UDP(17).
    #[prost(enumeration = "connection::Protocol", tag = "5")]
    pub protocol: i32,
}
/// Nested message and enum types in `Connection`.
pub mod connection {
    /// IANA Internet Protocol Number such as TCP(6) and UDP(17).
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
        /// Unspecified protocol (not HOPOPT).
        Unspecified = 0,
        /// Internet Control Message Protocol.
        Icmp = 1,
        /// Transmission Control Protocol.
        Tcp = 6,
        /// User Datagram Protocol.
        Udp = 17,
        /// Generic Routing Encapsulation.
        Gre = 47,
        /// Encap Security Payload.
        Esp = 50,
    }
    impl Protocol {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Protocol::Unspecified => "PROTOCOL_UNSPECIFIED",
                Protocol::Icmp => "ICMP",
                Protocol::Tcp => "TCP",
                Protocol::Udp => "UDP",
                Protocol::Gre => "GRE",
                Protocol::Esp => "ESP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROTOCOL_UNSPECIFIED" => Some(Self::Unspecified),
                "ICMP" => Some(Self::Icmp),
                "TCP" => Some(Self::Tcp),
                "UDP" => Some(Self::Udp),
                "GRE" => Some(Self::Gre),
                "ESP" => Some(Self::Esp),
                _ => None,
            }
        }
    }
}
/// Represents what's commonly known as an _indicator of compromise_ (IoC) in
/// computer forensics. This is an artifact observed on a network or in an
/// operating system that, with high confidence, indicates a computer intrusion.
/// For more information, see [Indicator of
/// compromise](<https://en.wikipedia.org/wiki/Indicator_of_compromise>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Indicator {
    /// The list of IP addresses that are associated with the finding.
    #[prost(string, repeated, tag = "1")]
    pub ip_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of domains associated to the Finding.
    #[prost(string, repeated, tag = "2")]
    pub domains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of matched signatures indicating that the given
    /// process is present in the environment.
    #[prost(message, repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<indicator::ProcessSignature>,
    /// The list of URIs associated to the Findings.
    #[prost(string, repeated, tag = "4")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Indicator`.
pub mod indicator {
    /// Indicates what signature matched this process.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProcessSignature {
        /// Describes the type of resource associated with the signature.
        #[prost(enumeration = "process_signature::SignatureType", tag = "8")]
        pub signature_type: i32,
        #[prost(oneof = "process_signature::Signature", tags = "6, 7")]
        pub signature: ::core::option::Option<process_signature::Signature>,
    }
    /// Nested message and enum types in `ProcessSignature`.
    pub mod process_signature {
        /// A signature corresponding to memory page hashes.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MemoryHashSignature {
            /// The binary family.
            #[prost(string, tag = "1")]
            pub binary_family: ::prost::alloc::string::String,
            /// The list of memory hash detections contributing to the binary family
            /// match.
            #[prost(message, repeated, tag = "4")]
            pub detections: ::prost::alloc::vec::Vec<memory_hash_signature::Detection>,
        }
        /// Nested message and enum types in `MemoryHashSignature`.
        pub mod memory_hash_signature {
            /// Memory hash detection contributing to the binary family match.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Detection {
                /// The name of the binary associated with the memory hash
                /// signature detection.
                #[prost(string, tag = "2")]
                pub binary: ::prost::alloc::string::String,
                /// The percentage of memory page hashes in the signature
                /// that were matched.
                #[prost(double, tag = "3")]
                pub percent_pages_matched: f64,
            }
        }
        /// A signature corresponding to a YARA rule.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct YaraRuleSignature {
            /// The name of the YARA rule.
            #[prost(string, tag = "5")]
            pub yara_rule: ::prost::alloc::string::String,
        }
        /// Possible resource types to be associated with a signature.
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
        pub enum SignatureType {
            /// The default signature type.
            Unspecified = 0,
            /// Used for signatures concerning processes.
            Process = 1,
            /// Used for signatures concerning disks.
            File = 2,
        }
        impl SignatureType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    SignatureType::Unspecified => "SIGNATURE_TYPE_UNSPECIFIED",
                    SignatureType::Process => "SIGNATURE_TYPE_PROCESS",
                    SignatureType::File => "SIGNATURE_TYPE_FILE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SIGNATURE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "SIGNATURE_TYPE_PROCESS" => Some(Self::Process),
                    "SIGNATURE_TYPE_FILE" => Some(Self::File),
                    _ => None,
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Signature {
            /// Signature indicating that a binary family was matched.
            #[prost(message, tag = "6")]
            MemoryHashSignature(MemoryHashSignature),
            /// Signature indicating that a YARA rule was matched.
            #[prost(message, tag = "7")]
            YaraRuleSignature(YaraRuleSignature),
        }
    }
}
/// Kernel mode rootkit signatures.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KernelRootkit {
    /// Rootkit name, when available.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// True if unexpected modifications of kernel code memory are present.
    #[prost(bool, tag = "2")]
    pub unexpected_code_modification: bool,
    /// True if unexpected modifications of kernel read-only data memory are
    /// present.
    #[prost(bool, tag = "3")]
    pub unexpected_read_only_data_modification: bool,
    /// True if `ftrace` points are present with callbacks pointing to regions
    /// that are not in the expected kernel or module code range.
    #[prost(bool, tag = "4")]
    pub unexpected_ftrace_handler: bool,
    /// True if `kprobe` points are present with callbacks pointing to regions
    /// that are not in the expected kernel or module code range.
    #[prost(bool, tag = "5")]
    pub unexpected_kprobe_handler: bool,
    /// True if kernel code pages that are not in the expected kernel or module
    /// code regions are present.
    #[prost(bool, tag = "6")]
    pub unexpected_kernel_code_pages: bool,
    /// True if system call handlers that are are not in the expected kernel or
    /// module code regions are present.
    #[prost(bool, tag = "7")]
    pub unexpected_system_call_handler: bool,
    /// True if interrupt handlers that are are not in the expected kernel or
    /// module code regions are present.
    #[prost(bool, tag = "8")]
    pub unexpected_interrupt_handler: bool,
    /// True if unexpected processes in the scheduler run queue are present. Such
    /// processes are in the run queue, but not in the process task list.
    #[prost(bool, tag = "9")]
    pub unexpected_processes_in_runqueue: bool,
}
/// Contains compliance information about a security standard indicating unmet
/// recommendations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Compliance {
    /// Industry-wide compliance standards or benchmarks, such as CIS, PCI, and
    /// OWASP.
    #[prost(string, tag = "1")]
    pub standard: ::prost::alloc::string::String,
    /// Version of the standard or benchmark, for example, 1.1
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// Policies within the standard or benchmark, for example, A.12.4.1
    #[prost(string, repeated, tag = "3")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents an access event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Access {
    /// Associated email, such as "foo@google.com".
    ///
    /// The email address of the authenticated user or a service account acting on
    /// behalf of a third party principal making the request. For third party
    /// identity callers, the `principal_subject` field is populated instead of
    /// this field. For privacy reasons, the principal email address is sometimes
    /// redacted. For more information, see [Caller identities in audit
    /// logs](<https://cloud.google.com/logging/docs/audit#user-id>).
    #[prost(string, tag = "1")]
    pub principal_email: ::prost::alloc::string::String,
    /// Caller's IP address, such as "1.1.1.1".
    #[prost(string, tag = "2")]
    pub caller_ip: ::prost::alloc::string::String,
    /// The caller IP's geolocation, which identifies where the call came from.
    #[prost(message, optional, tag = "3")]
    pub caller_ip_geo: ::core::option::Option<Geolocation>,
    /// Type of user agent associated with the finding. For example, an operating
    /// system shell or an embedded or standalone application.
    #[prost(string, tag = "4")]
    pub user_agent_family: ::prost::alloc::string::String,
    /// The caller's user agent string associated with the finding.
    #[prost(string, tag = "5")]
    pub user_agent: ::prost::alloc::string::String,
    /// This is the API service that the service account made a call to, e.g.
    /// "iam.googleapis.com"
    #[prost(string, tag = "6")]
    pub service_name: ::prost::alloc::string::String,
    /// The method that the service account called, e.g. "SetIamPolicy".
    #[prost(string, tag = "7")]
    pub method_name: ::prost::alloc::string::String,
    /// A string that represents the principal_subject that is associated with the
    /// identity. Unlike `principal_email`, `principal_subject` supports principals
    /// that aren't associated with email addresses, such as third party
    /// principals. For most identities, the format is
    /// `principal://iam.googleapis.com/{identity pool name}/subject/{subject}`.
    /// Some GKE identities, such as GKE_WORKLOAD, FREEFORM, and GKE_HUB_WORKLOAD,
    /// still use the legacy format `serviceAccount:{identity pool
    /// name}\[{subject}\]`.
    #[prost(string, tag = "8")]
    pub principal_subject: ::prost::alloc::string::String,
    /// The name of the service account key that was used to create or exchange
    /// credentials when authenticating the service account that made the request.
    /// This is a scheme-less URI full resource name. For example:
    ///
    /// "//iam.googleapis.com/projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}".
    ///
    #[prost(string, tag = "9")]
    pub service_account_key_name: ::prost::alloc::string::String,
    /// The identity delegation history of an authenticated service account that
    /// made the request. The `serviceAccountDelegationInfo\[\]` object contains
    /// information about the real authorities that try to access Google Cloud
    /// resources by delegating on a service account. When multiple authorities are
    /// present, they are guaranteed to be sorted based on the original ordering of
    /// the identity delegation events.
    #[prost(message, repeated, tag = "10")]
    pub service_account_delegation_info: ::prost::alloc::vec::Vec<
        ServiceAccountDelegationInfo,
    >,
    /// A string that represents a username. The username provided depends on the
    /// type of the finding and is likely not an IAM principal. For example, this
    /// can be a system username if the finding is related to a virtual machine, or
    /// it can be an application login username.
    #[prost(string, tag = "11")]
    pub user_name: ::prost::alloc::string::String,
}
/// Identity delegation history of an authenticated service account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccountDelegationInfo {
    /// The email address of a Google account.
    #[prost(string, tag = "1")]
    pub principal_email: ::prost::alloc::string::String,
    /// A string representing the principal_subject associated with the identity.
    /// As compared to `principal_email`, supports principals that aren't
    /// associated with email addresses, such as third party principals. For most
    /// identities, the format will be `principal://iam.googleapis.com/{identity
    /// pool name}/subjects/{subject}` except for some GKE identities
    /// (GKE_WORKLOAD, FREEFORM, GKE_HUB_WORKLOAD) that are still in the legacy
    /// format `serviceAccount:{identity pool name}\[{subject}\]`
    #[prost(string, tag = "2")]
    pub principal_subject: ::prost::alloc::string::String,
}
/// Represents a geographical location for a given access.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Geolocation {
    /// A CLDR.
    #[prost(string, tag = "1")]
    pub region_code: ::prost::alloc::string::String,
}
/// An attack exposure contains the results of an attack path simulation run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttackExposure {
    /// A number between 0 (inclusive) and infinity that represents how important
    /// this finding is to remediate. The higher the score, the more important it
    /// is to remediate.
    #[prost(double, tag = "1")]
    pub score: f64,
    /// The most recent time the attack exposure was updated on this finding.
    #[prost(message, optional, tag = "2")]
    pub latest_calculation_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The resource name of the attack path simulation result that contains the
    /// details regarding this attack exposure score.
    /// Example: organizations/123/simulations/456/attackExposureResults/789
    #[prost(string, tag = "3")]
    pub attack_exposure_result: ::prost::alloc::string::String,
    /// Output only. What state this AttackExposure is in. This captures whether or
    /// not an attack exposure has been calculated or not.
    #[prost(enumeration = "attack_exposure::State", tag = "4")]
    pub state: i32,
    /// The number of high value resources that are exposed as a result of this
    /// finding.
    #[prost(int32, tag = "5")]
    pub exposed_high_value_resources_count: i32,
    /// The number of medium value resources that are exposed as a result of this
    /// finding.
    #[prost(int32, tag = "6")]
    pub exposed_medium_value_resources_count: i32,
    /// The number of high value resources that are exposed as a result of this
    /// finding.
    #[prost(int32, tag = "7")]
    pub exposed_low_value_resources_count: i32,
}
/// Nested message and enum types in `AttackExposure`.
pub mod attack_exposure {
    /// This enum defines the various states an AttackExposure can be in.
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
        /// The state is not specified.
        Unspecified = 0,
        /// The attack exposure has been calculated.
        Calculated = 1,
        /// The attack exposure has not been calculated.
        NotCalculated = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Calculated => "CALCULATED",
                State::NotCalculated => "NOT_CALCULATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CALCULATED" => Some(Self::Calculated),
                "NOT_CALCULATED" => Some(Self::NotCalculated),
                _ => None,
            }
        }
    }
}
/// Information related to Google Cloud Backup and DR Service findings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupDisasterRecovery {
    /// The name of a Backup and DR template which comprises one or more backup
    /// policies. See the [Backup and DR
    /// documentation](<https://cloud.google.com/backup-disaster-recovery/docs/concepts/backup-plan#temp>)
    /// for more information. For example, `snap-ov`.
    #[prost(string, tag = "1")]
    pub backup_template: ::prost::alloc::string::String,
    /// The names of Backup and DR policies that are associated with a template
    /// and that define when to run a backup, how frequently to run a backup, and
    /// how long to retain the backup image. For example, `onvaults`.
    #[prost(string, repeated, tag = "2")]
    pub policies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The name of a Backup and DR host, which is managed by the backup and
    /// recovery appliance and known to the management console. The host can be of
    /// type Generic (for example, Compute Engine, SQL Server, Oracle DB, SMB file
    /// system, etc.), vCenter, or an ESX server. See the [Backup and DR
    /// documentation on
    /// hosts](<https://cloud.google.com/backup-disaster-recovery/docs/configuration/manage-hosts-and-their-applications>)
    /// for more information. For example, `centos7-01`.
    #[prost(string, tag = "3")]
    pub host: ::prost::alloc::string::String,
    /// The names of Backup and DR applications. An application is a VM, database,
    /// or file system on a managed host monitored by a backup and recovery
    /// appliance. For example, `centos7-01-vol00`, `centos7-01-vol01`,
    /// `centos7-01-vol02`.
    #[prost(string, repeated, tag = "4")]
    pub applications: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The name of the Backup and DR storage pool that the backup and recovery
    /// appliance is storing data in. The storage pool could be of type Cloud,
    /// Primary, Snapshot, or OnVault. See the [Backup and DR documentation on
    /// storage
    /// pools](<https://cloud.google.com/backup-disaster-recovery/docs/concepts/storage-pools>).
    /// For example, `DiskPoolOne`.
    #[prost(string, tag = "5")]
    pub storage_pool: ::prost::alloc::string::String,
    /// The names of Backup and DR advanced policy options of a policy applying to
    /// an application. See the [Backup and DR documentation on policy
    /// options](<https://cloud.google.com/backup-disaster-recovery/docs/create-plan/policy-settings>).
    /// For example, `skipofflineappsincongrp, nounmap`.
    #[prost(string, repeated, tag = "6")]
    pub policy_options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The name of the Backup and DR resource profile that specifies the storage
    /// media for backups of application and VM data. See the [Backup and DR
    /// documentation on
    /// profiles](<https://cloud.google.com/backup-disaster-recovery/docs/concepts/backup-plan#profile>).
    /// For example, `GCP`.
    #[prost(string, tag = "7")]
    pub profile: ::prost::alloc::string::String,
    /// The name of the Backup and DR appliance that captures, moves, and manages
    /// the lifecycle of backup data. For example, `backup-server-57137`.
    #[prost(string, tag = "8")]
    pub appliance: ::prost::alloc::string::String,
    /// The backup type of the Backup and DR image.
    /// For example, `Snapshot`, `Remote Snapshot`, `OnVault`.
    #[prost(string, tag = "9")]
    pub backup_type: ::prost::alloc::string::String,
    /// The timestamp at which the Backup and DR backup was created.
    #[prost(message, optional, tag = "10")]
    pub backup_create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Details about the Cloud Data Loss Prevention (Cloud DLP) [inspection
/// job](<https://cloud.google.com/dlp/docs/concepts-job-triggers>) that produced
/// the finding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudDlpInspection {
    /// Name of the inspection job, for example,
    /// `projects/123/locations/europe/dlpJobs/i-8383929`.
    #[prost(string, tag = "1")]
    pub inspect_job: ::prost::alloc::string::String,
    /// The type of information (or
    /// *[infoType](<https://cloud.google.com/dlp/docs/infotypes-reference>)*) found,
    /// for example, `EMAIL_ADDRESS` or `STREET_ADDRESS`.
    #[prost(string, tag = "2")]
    pub info_type: ::prost::alloc::string::String,
    /// The number of times Cloud DLP found this infoType within this job
    /// and resource.
    #[prost(int64, tag = "3")]
    pub info_type_count: i64,
    /// Whether Cloud DLP scanned the complete resource or a sampled subset.
    #[prost(bool, tag = "4")]
    pub full_scan: bool,
}
/// Details about specific contacts
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContactDetails {
    /// A list of contacts
    #[prost(message, repeated, tag = "1")]
    pub contacts: ::prost::alloc::vec::Vec<Contact>,
}
/// The email address of a contact.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    /// An email address. For example, "`person123@company.com`".
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
}
/// Exfiltration represents a data exfiltration attempt from one or more sources
/// to one or more targets. The `sources` attribute lists the sources of the
/// exfiltrated data. The `targets` attribute lists the destinations the data was
/// copied to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exfiltration {
    /// If there are multiple sources, then the data is considered "joined" between
    /// them. For instance, BigQuery can join multiple tables, and each
    /// table would be considered a source.
    #[prost(message, repeated, tag = "1")]
    pub sources: ::prost::alloc::vec::Vec<ExfilResource>,
    /// If there are multiple targets, each target would get a complete copy of the
    /// "joined" source data.
    #[prost(message, repeated, tag = "2")]
    pub targets: ::prost::alloc::vec::Vec<ExfilResource>,
    /// Total exfiltrated bytes processed for the entire job.
    #[prost(int64, tag = "3")]
    pub total_exfiltrated_bytes: i64,
}
/// Resource where data was exfiltrated from or exfiltrated to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExfilResource {
    /// The resource's [full resource
    /// name](<https://cloud.google.com/apis/design/resource_names#full_resource_name>).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Subcomponents of the asset that was exfiltrated, like URIs used during
    /// exfiltration, table names, databases, and filenames. For example, multiple
    /// tables might have been exfiltrated from the same Cloud SQL instance, or
    /// multiple files might have been exfiltrated from the same Cloud Storage
    /// bucket.
    #[prost(string, repeated, tag = "2")]
    pub components: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Representation of third party SIEM/SOAR fields within SCC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalSystem {
    /// Full resource name of the external system. The following list
    /// shows some examples:
    ///
    /// + `organizations/1234/sources/5678/findings/123456/externalSystems/jira`
    /// +
    /// `organizations/1234/sources/5678/locations/us/findings/123456/externalSystems/jira`
    /// + `folders/1234/sources/5678/findings/123456/externalSystems/jira`
    /// +
    /// `folders/1234/sources/5678/locations/us/findings/123456/externalSystems/jira`
    /// + `projects/1234/sources/5678/findings/123456/externalSystems/jira`
    /// +
    /// `projects/1234/sources/5678/locations/us/findings/123456/externalSystems/jira`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// References primary/secondary etc assignees in the external system.
    #[prost(string, repeated, tag = "2")]
    pub assignees: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The identifier that's used to track the finding's corresponding case in the
    /// external system.
    #[prost(string, tag = "3")]
    pub external_uid: ::prost::alloc::string::String,
    /// The most recent status of the finding's corresponding case, as reported by
    /// the external system.
    #[prost(string, tag = "4")]
    pub status: ::prost::alloc::string::String,
    /// The time when the case was last updated, as reported by the external
    /// system.
    #[prost(message, optional, tag = "5")]
    pub external_system_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The link to the finding's corresponding case in the external system.
    #[prost(string, tag = "6")]
    pub case_uri: ::prost::alloc::string::String,
    /// The priority of the finding's corresponding case in the external system.
    #[prost(string, tag = "7")]
    pub case_priority: ::prost::alloc::string::String,
    /// The SLA of the finding's corresponding case in the external system.
    #[prost(message, optional, tag = "9")]
    pub case_sla: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the case was created, as reported by the external system.
    #[prost(message, optional, tag = "10")]
    pub case_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the case was closed, as reported by the external system.
    #[prost(message, optional, tag = "11")]
    pub case_close_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Information about the ticket, if any, that is being used to track the
    /// resolution of the issue that is identified by this finding.
    #[prost(message, optional, tag = "8")]
    pub ticket_info: ::core::option::Option<external_system::TicketInfo>,
}
/// Nested message and enum types in `ExternalSystem`.
pub mod external_system {
    /// Information about the ticket, if any, that is being used to track the
    /// resolution of the issue that is identified by this finding.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TicketInfo {
        /// The identifier of the ticket in the ticket system.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The assignee of the ticket in the ticket system.
        #[prost(string, tag = "2")]
        pub assignee: ::prost::alloc::string::String,
        /// The description of the ticket in the ticket system.
        #[prost(string, tag = "3")]
        pub description: ::prost::alloc::string::String,
        /// The link to the ticket in the ticket system.
        #[prost(string, tag = "4")]
        pub uri: ::prost::alloc::string::String,
        /// The latest status of the ticket, as reported by the ticket system.
        #[prost(string, tag = "5")]
        pub status: ::prost::alloc::string::String,
        /// The time when the ticket was last updated, as reported by the ticket
        /// system.
        #[prost(message, optional, tag = "6")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    }
}
/// File information about the related binary/library used by an executable, or
/// the script used by a script interpreter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    /// Absolute path of the file as a JSON encoded string.
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// Size of the file in bytes.
    #[prost(int64, tag = "2")]
    pub size: i64,
    /// SHA256 hash of the first hashed_size bytes of the file encoded as a
    /// hex string.  If hashed_size == size, sha256 represents the SHA256 hash
    /// of the entire file.
    #[prost(string, tag = "3")]
    pub sha256: ::prost::alloc::string::String,
    /// The length in bytes of the file prefix that was hashed.  If
    /// hashed_size == size, any hashes reported represent the entire
    /// file.
    #[prost(int64, tag = "4")]
    pub hashed_size: i64,
    /// True when the hash covers only a prefix of the file.
    #[prost(bool, tag = "5")]
    pub partially_hashed: bool,
    /// Prefix of the file contents as a JSON-encoded string.
    #[prost(string, tag = "6")]
    pub contents: ::prost::alloc::string::String,
    /// Path of the file in terms of underlying disk/partition identifiers.
    #[prost(message, optional, tag = "7")]
    pub disk_path: ::core::option::Option<file::DiskPath>,
}
/// Nested message and enum types in `File`.
pub mod file {
    /// Path of the file in terms of underlying disk/partition identifiers.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DiskPath {
        /// UUID of the partition (format
        /// <https://wiki.archlinux.org/title/persistent_block_device_naming#by-uuid>)
        #[prost(string, tag = "1")]
        pub partition_uuid: ::prost::alloc::string::String,
        /// Relative path of the file in the partition as a JSON encoded string.
        /// Example: /home/user1/executable_file.sh
        #[prost(string, tag = "2")]
        pub relative_path: ::prost::alloc::string::String,
    }
}
/// Contains information related to the load balancer associated with the
/// finding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadBalancer {
    /// The name of the load balancer associated with the finding.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// An individual entry in a log.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogEntry {
    #[prost(oneof = "log_entry::LogEntry", tags = "1")]
    pub log_entry: ::core::option::Option<log_entry::LogEntry>,
}
/// Nested message and enum types in `LogEntry`.
pub mod log_entry {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LogEntry {
        /// An individual entry in a log stored in Cloud Logging.
        #[prost(message, tag = "1")]
        CloudLoggingEntry(super::CloudLoggingEntry),
    }
}
/// Metadata taken from a [Cloud Logging
/// LogEntry](<https://cloud.google.com/logging/docs/reference/v2/rest/v2/LogEntry>)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudLoggingEntry {
    /// A unique identifier for the log entry.
    #[prost(string, tag = "1")]
    pub insert_id: ::prost::alloc::string::String,
    /// The type of the log (part of `log_name`. `log_name` is the resource name of
    /// the log to which this log entry belongs). For example:
    /// `cloudresourcemanager.googleapis.com/activity` Note that this field is not
    /// URL-encoded, unlike in `LogEntry`.
    #[prost(string, tag = "2")]
    pub log_id: ::prost::alloc::string::String,
    /// The organization, folder, or project of the monitored resource that
    /// produced this log entry.
    #[prost(string, tag = "3")]
    pub resource_container: ::prost::alloc::string::String,
    /// The time the event described by the log entry occurred.
    #[prost(message, optional, tag = "4")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
/// MITRE ATT&CK tactics and techniques related to this finding.
/// See: <https://attack.mitre.org>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MitreAttack {
    /// The MITRE ATT&CK tactic most closely represented by this finding, if any.
    #[prost(enumeration = "mitre_attack::Tactic", tag = "1")]
    pub primary_tactic: i32,
    /// The MITRE ATT&CK technique most closely represented by this finding, if
    /// any. primary_techniques is a repeated field because there are multiple
    /// levels of MITRE ATT&CK techniques.  If the technique most closely
    /// represented by this finding is a sub-technique (e.g. `SCANNING_IP_BLOCKS`),
    /// both the sub-technique and its parent technique(s) will be listed (e.g.
    /// `SCANNING_IP_BLOCKS`, `ACTIVE_SCANNING`).
    #[prost(enumeration = "mitre_attack::Technique", repeated, tag = "2")]
    pub primary_techniques: ::prost::alloc::vec::Vec<i32>,
    /// Additional MITRE ATT&CK tactics related to this finding, if any.
    #[prost(enumeration = "mitre_attack::Tactic", repeated, tag = "3")]
    pub additional_tactics: ::prost::alloc::vec::Vec<i32>,
    /// Additional MITRE ATT&CK techniques related to this finding, if any, along
    /// with any of their respective parent techniques.
    #[prost(enumeration = "mitre_attack::Technique", repeated, tag = "4")]
    pub additional_techniques: ::prost::alloc::vec::Vec<i32>,
    /// The MITRE ATT&CK version referenced by the above fields. E.g. "8".
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MitreAttack`.
pub mod mitre_attack {
    /// MITRE ATT&CK tactics that can be referenced by SCC findings.
    /// See: <https://attack.mitre.org/tactics/enterprise/>
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
    pub enum Tactic {
        /// Unspecified value.
        Unspecified = 0,
        /// TA0043
        Reconnaissance = 1,
        /// TA0042
        ResourceDevelopment = 2,
        /// TA0001
        InitialAccess = 5,
        /// TA0002
        Execution = 3,
        /// TA0003
        Persistence = 6,
        /// TA0004
        PrivilegeEscalation = 8,
        /// TA0005
        DefenseEvasion = 7,
        /// TA0006
        CredentialAccess = 9,
        /// TA0007
        Discovery = 10,
        /// TA0008
        LateralMovement = 11,
        /// TA0009
        Collection = 12,
        /// TA0011
        CommandAndControl = 4,
        /// TA0010
        Exfiltration = 13,
        /// TA0040
        Impact = 14,
    }
    impl Tactic {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Tactic::Unspecified => "TACTIC_UNSPECIFIED",
                Tactic::Reconnaissance => "RECONNAISSANCE",
                Tactic::ResourceDevelopment => "RESOURCE_DEVELOPMENT",
                Tactic::InitialAccess => "INITIAL_ACCESS",
                Tactic::Execution => "EXECUTION",
                Tactic::Persistence => "PERSISTENCE",
                Tactic::PrivilegeEscalation => "PRIVILEGE_ESCALATION",
                Tactic::DefenseEvasion => "DEFENSE_EVASION",
                Tactic::CredentialAccess => "CREDENTIAL_ACCESS",
                Tactic::Discovery => "DISCOVERY",
                Tactic::LateralMovement => "LATERAL_MOVEMENT",
                Tactic::Collection => "COLLECTION",
                Tactic::CommandAndControl => "COMMAND_AND_CONTROL",
                Tactic::Exfiltration => "EXFILTRATION",
                Tactic::Impact => "IMPACT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TACTIC_UNSPECIFIED" => Some(Self::Unspecified),
                "RECONNAISSANCE" => Some(Self::Reconnaissance),
                "RESOURCE_DEVELOPMENT" => Some(Self::ResourceDevelopment),
                "INITIAL_ACCESS" => Some(Self::InitialAccess),
                "EXECUTION" => Some(Self::Execution),
                "PERSISTENCE" => Some(Self::Persistence),
                "PRIVILEGE_ESCALATION" => Some(Self::PrivilegeEscalation),
                "DEFENSE_EVASION" => Some(Self::DefenseEvasion),
                "CREDENTIAL_ACCESS" => Some(Self::CredentialAccess),
                "DISCOVERY" => Some(Self::Discovery),
                "LATERAL_MOVEMENT" => Some(Self::LateralMovement),
                "COLLECTION" => Some(Self::Collection),
                "COMMAND_AND_CONTROL" => Some(Self::CommandAndControl),
                "EXFILTRATION" => Some(Self::Exfiltration),
                "IMPACT" => Some(Self::Impact),
                _ => None,
            }
        }
    }
    /// MITRE ATT&CK techniques that can be referenced by SCC findings.
    /// See: <https://attack.mitre.org/techniques/enterprise/>
    /// Next ID: 59
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
    pub enum Technique {
        /// Unspecified value.
        Unspecified = 0,
        /// T1036
        Masquerading = 49,
        /// T1036.005
        MatchLegitimateNameOrLocation = 50,
        /// T1037
        BootOrLogonInitializationScripts = 37,
        /// T1037.005
        StartupItems = 38,
        /// T1046
        NetworkServiceDiscovery = 32,
        /// T1057
        ProcessDiscovery = 56,
        /// T1059
        CommandAndScriptingInterpreter = 6,
        /// T1059.004
        UnixShell = 7,
        /// T1069
        PermissionGroupsDiscovery = 18,
        /// T1069.003
        CloudGroups = 19,
        /// T1071
        ApplicationLayerProtocol = 45,
        /// T1071.004
        Dns = 46,
        /// T1072
        SoftwareDeploymentTools = 47,
        /// T1078
        ValidAccounts = 14,
        /// T1078.001
        DefaultAccounts = 35,
        /// T1078.003
        LocalAccounts = 15,
        /// T1078.004
        CloudAccounts = 16,
        /// T1090
        Proxy = 9,
        /// T1090.002
        ExternalProxy = 10,
        /// T1090.003
        MultiHopProxy = 11,
        /// T1098
        AccountManipulation = 22,
        /// T1098.001
        AdditionalCloudCredentials = 40,
        /// T1098.004
        SshAuthorizedKeys = 23,
        /// T1098.006
        AdditionalContainerClusterRoles = 58,
        /// T1105
        IngressToolTransfer = 3,
        /// T1106
        NativeApi = 4,
        /// T1110
        BruteForce = 44,
        /// T1129
        SharedModules = 5,
        /// T1134
        AccessTokenManipulation = 33,
        /// T1134.001
        TokenImpersonationOrTheft = 39,
        /// T1190
        ExploitPublicFacingApplication = 27,
        /// T1484
        DomainPolicyModification = 30,
        /// T1485
        DataDestruction = 29,
        /// T1489
        ServiceStop = 52,
        /// T1490
        InhibitSystemRecovery = 36,
        /// T1496
        ResourceHijacking = 8,
        /// T1498
        NetworkDenialOfService = 17,
        /// T1526
        CloudServiceDiscovery = 48,
        /// T1528
        StealApplicationAccessToken = 42,
        /// T1531
        AccountAccessRemoval = 51,
        /// T1539
        StealWebSessionCookie = 25,
        /// T1543
        CreateOrModifySystemProcess = 24,
        /// T1548
        AbuseElevationControlMechanism = 34,
        /// T1552
        UnsecuredCredentials = 13,
        /// T1556
        ModifyAuthenticationProcess = 28,
        /// T1562
        ImpairDefenses = 31,
        /// T1562.001
        DisableOrModifyTools = 55,
        /// T1567
        ExfiltrationOverWebService = 20,
        /// T1567.002
        ExfiltrationToCloudStorage = 21,
        /// T1568
        DynamicResolution = 12,
        /// T1570
        LateralToolTransfer = 41,
        /// T1578
        ModifyCloudComputeInfrastructure = 26,
        /// T1578.001
        CreateSnapshot = 54,
        /// T1580
        CloudInfrastructureDiscovery = 53,
        /// T1588
        ObtainCapabilities = 43,
        /// T1595
        ActiveScanning = 1,
        /// T1595.001
        ScanningIpBlocks = 2,
        /// T1613
        ContainerAndResourceDiscovery = 57,
    }
    impl Technique {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Technique::Unspecified => "TECHNIQUE_UNSPECIFIED",
                Technique::Masquerading => "MASQUERADING",
                Technique::MatchLegitimateNameOrLocation => {
                    "MATCH_LEGITIMATE_NAME_OR_LOCATION"
                }
                Technique::BootOrLogonInitializationScripts => {
                    "BOOT_OR_LOGON_INITIALIZATION_SCRIPTS"
                }
                Technique::StartupItems => "STARTUP_ITEMS",
                Technique::NetworkServiceDiscovery => "NETWORK_SERVICE_DISCOVERY",
                Technique::ProcessDiscovery => "PROCESS_DISCOVERY",
                Technique::CommandAndScriptingInterpreter => {
                    "COMMAND_AND_SCRIPTING_INTERPRETER"
                }
                Technique::UnixShell => "UNIX_SHELL",
                Technique::PermissionGroupsDiscovery => "PERMISSION_GROUPS_DISCOVERY",
                Technique::CloudGroups => "CLOUD_GROUPS",
                Technique::ApplicationLayerProtocol => "APPLICATION_LAYER_PROTOCOL",
                Technique::Dns => "DNS",
                Technique::SoftwareDeploymentTools => "SOFTWARE_DEPLOYMENT_TOOLS",
                Technique::ValidAccounts => "VALID_ACCOUNTS",
                Technique::DefaultAccounts => "DEFAULT_ACCOUNTS",
                Technique::LocalAccounts => "LOCAL_ACCOUNTS",
                Technique::CloudAccounts => "CLOUD_ACCOUNTS",
                Technique::Proxy => "PROXY",
                Technique::ExternalProxy => "EXTERNAL_PROXY",
                Technique::MultiHopProxy => "MULTI_HOP_PROXY",
                Technique::AccountManipulation => "ACCOUNT_MANIPULATION",
                Technique::AdditionalCloudCredentials => "ADDITIONAL_CLOUD_CREDENTIALS",
                Technique::SshAuthorizedKeys => "SSH_AUTHORIZED_KEYS",
                Technique::AdditionalContainerClusterRoles => {
                    "ADDITIONAL_CONTAINER_CLUSTER_ROLES"
                }
                Technique::IngressToolTransfer => "INGRESS_TOOL_TRANSFER",
                Technique::NativeApi => "NATIVE_API",
                Technique::BruteForce => "BRUTE_FORCE",
                Technique::SharedModules => "SHARED_MODULES",
                Technique::AccessTokenManipulation => "ACCESS_TOKEN_MANIPULATION",
                Technique::TokenImpersonationOrTheft => "TOKEN_IMPERSONATION_OR_THEFT",
                Technique::ExploitPublicFacingApplication => {
                    "EXPLOIT_PUBLIC_FACING_APPLICATION"
                }
                Technique::DomainPolicyModification => "DOMAIN_POLICY_MODIFICATION",
                Technique::DataDestruction => "DATA_DESTRUCTION",
                Technique::ServiceStop => "SERVICE_STOP",
                Technique::InhibitSystemRecovery => "INHIBIT_SYSTEM_RECOVERY",
                Technique::ResourceHijacking => "RESOURCE_HIJACKING",
                Technique::NetworkDenialOfService => "NETWORK_DENIAL_OF_SERVICE",
                Technique::CloudServiceDiscovery => "CLOUD_SERVICE_DISCOVERY",
                Technique::StealApplicationAccessToken => {
                    "STEAL_APPLICATION_ACCESS_TOKEN"
                }
                Technique::AccountAccessRemoval => "ACCOUNT_ACCESS_REMOVAL",
                Technique::StealWebSessionCookie => "STEAL_WEB_SESSION_COOKIE",
                Technique::CreateOrModifySystemProcess => {
                    "CREATE_OR_MODIFY_SYSTEM_PROCESS"
                }
                Technique::AbuseElevationControlMechanism => {
                    "ABUSE_ELEVATION_CONTROL_MECHANISM"
                }
                Technique::UnsecuredCredentials => "UNSECURED_CREDENTIALS",
                Technique::ModifyAuthenticationProcess => "MODIFY_AUTHENTICATION_PROCESS",
                Technique::ImpairDefenses => "IMPAIR_DEFENSES",
                Technique::DisableOrModifyTools => "DISABLE_OR_MODIFY_TOOLS",
                Technique::ExfiltrationOverWebService => "EXFILTRATION_OVER_WEB_SERVICE",
                Technique::ExfiltrationToCloudStorage => "EXFILTRATION_TO_CLOUD_STORAGE",
                Technique::DynamicResolution => "DYNAMIC_RESOLUTION",
                Technique::LateralToolTransfer => "LATERAL_TOOL_TRANSFER",
                Technique::ModifyCloudComputeInfrastructure => {
                    "MODIFY_CLOUD_COMPUTE_INFRASTRUCTURE"
                }
                Technique::CreateSnapshot => "CREATE_SNAPSHOT",
                Technique::CloudInfrastructureDiscovery => {
                    "CLOUD_INFRASTRUCTURE_DISCOVERY"
                }
                Technique::ObtainCapabilities => "OBTAIN_CAPABILITIES",
                Technique::ActiveScanning => "ACTIVE_SCANNING",
                Technique::ScanningIpBlocks => "SCANNING_IP_BLOCKS",
                Technique::ContainerAndResourceDiscovery => {
                    "CONTAINER_AND_RESOURCE_DISCOVERY"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TECHNIQUE_UNSPECIFIED" => Some(Self::Unspecified),
                "MASQUERADING" => Some(Self::Masquerading),
                "MATCH_LEGITIMATE_NAME_OR_LOCATION" => {
                    Some(Self::MatchLegitimateNameOrLocation)
                }
                "BOOT_OR_LOGON_INITIALIZATION_SCRIPTS" => {
                    Some(Self::BootOrLogonInitializationScripts)
                }
                "STARTUP_ITEMS" => Some(Self::StartupItems),
                "NETWORK_SERVICE_DISCOVERY" => Some(Self::NetworkServiceDiscovery),
                "PROCESS_DISCOVERY" => Some(Self::ProcessDiscovery),
                "COMMAND_AND_SCRIPTING_INTERPRETER" => {
                    Some(Self::CommandAndScriptingInterpreter)
                }
                "UNIX_SHELL" => Some(Self::UnixShell),
                "PERMISSION_GROUPS_DISCOVERY" => Some(Self::PermissionGroupsDiscovery),
                "CLOUD_GROUPS" => Some(Self::CloudGroups),
                "APPLICATION_LAYER_PROTOCOL" => Some(Self::ApplicationLayerProtocol),
                "DNS" => Some(Self::Dns),
                "SOFTWARE_DEPLOYMENT_TOOLS" => Some(Self::SoftwareDeploymentTools),
                "VALID_ACCOUNTS" => Some(Self::ValidAccounts),
                "DEFAULT_ACCOUNTS" => Some(Self::DefaultAccounts),
                "LOCAL_ACCOUNTS" => Some(Self::LocalAccounts),
                "CLOUD_ACCOUNTS" => Some(Self::CloudAccounts),
                "PROXY" => Some(Self::Proxy),
                "EXTERNAL_PROXY" => Some(Self::ExternalProxy),
                "MULTI_HOP_PROXY" => Some(Self::MultiHopProxy),
                "ACCOUNT_MANIPULATION" => Some(Self::AccountManipulation),
                "ADDITIONAL_CLOUD_CREDENTIALS" => Some(Self::AdditionalCloudCredentials),
                "SSH_AUTHORIZED_KEYS" => Some(Self::SshAuthorizedKeys),
                "ADDITIONAL_CONTAINER_CLUSTER_ROLES" => {
                    Some(Self::AdditionalContainerClusterRoles)
                }
                "INGRESS_TOOL_TRANSFER" => Some(Self::IngressToolTransfer),
                "NATIVE_API" => Some(Self::NativeApi),
                "BRUTE_FORCE" => Some(Self::BruteForce),
                "SHARED_MODULES" => Some(Self::SharedModules),
                "ACCESS_TOKEN_MANIPULATION" => Some(Self::AccessTokenManipulation),
                "TOKEN_IMPERSONATION_OR_THEFT" => Some(Self::TokenImpersonationOrTheft),
                "EXPLOIT_PUBLIC_FACING_APPLICATION" => {
                    Some(Self::ExploitPublicFacingApplication)
                }
                "DOMAIN_POLICY_MODIFICATION" => Some(Self::DomainPolicyModification),
                "DATA_DESTRUCTION" => Some(Self::DataDestruction),
                "SERVICE_STOP" => Some(Self::ServiceStop),
                "INHIBIT_SYSTEM_RECOVERY" => Some(Self::InhibitSystemRecovery),
                "RESOURCE_HIJACKING" => Some(Self::ResourceHijacking),
                "NETWORK_DENIAL_OF_SERVICE" => Some(Self::NetworkDenialOfService),
                "CLOUD_SERVICE_DISCOVERY" => Some(Self::CloudServiceDiscovery),
                "STEAL_APPLICATION_ACCESS_TOKEN" => {
                    Some(Self::StealApplicationAccessToken)
                }
                "ACCOUNT_ACCESS_REMOVAL" => Some(Self::AccountAccessRemoval),
                "STEAL_WEB_SESSION_COOKIE" => Some(Self::StealWebSessionCookie),
                "CREATE_OR_MODIFY_SYSTEM_PROCESS" => {
                    Some(Self::CreateOrModifySystemProcess)
                }
                "ABUSE_ELEVATION_CONTROL_MECHANISM" => {
                    Some(Self::AbuseElevationControlMechanism)
                }
                "UNSECURED_CREDENTIALS" => Some(Self::UnsecuredCredentials),
                "MODIFY_AUTHENTICATION_PROCESS" => {
                    Some(Self::ModifyAuthenticationProcess)
                }
                "IMPAIR_DEFENSES" => Some(Self::ImpairDefenses),
                "DISABLE_OR_MODIFY_TOOLS" => Some(Self::DisableOrModifyTools),
                "EXFILTRATION_OVER_WEB_SERVICE" => Some(Self::ExfiltrationOverWebService),
                "EXFILTRATION_TO_CLOUD_STORAGE" => Some(Self::ExfiltrationToCloudStorage),
                "DYNAMIC_RESOLUTION" => Some(Self::DynamicResolution),
                "LATERAL_TOOL_TRANSFER" => Some(Self::LateralToolTransfer),
                "MODIFY_CLOUD_COMPUTE_INFRASTRUCTURE" => {
                    Some(Self::ModifyCloudComputeInfrastructure)
                }
                "CREATE_SNAPSHOT" => Some(Self::CreateSnapshot),
                "CLOUD_INFRASTRUCTURE_DISCOVERY" => {
                    Some(Self::CloudInfrastructureDiscovery)
                }
                "OBTAIN_CAPABILITIES" => Some(Self::ObtainCapabilities),
                "ACTIVE_SCANNING" => Some(Self::ActiveScanning),
                "SCANNING_IP_BLOCKS" => Some(Self::ScanningIpBlocks),
                "CONTAINER_AND_RESOURCE_DISCOVERY" => {
                    Some(Self::ContainerAndResourceDiscovery)
                }
                _ => None,
            }
        }
    }
}
/// Represents an operating system process.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Process {
    /// The process name, as displayed in utilities like `top` and `ps`. This name
    /// can be accessed through `/proc/\[pid\]/comm` and changed with
    /// `prctl(PR_SET_NAME)`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// File information for the process executable.
    #[prost(message, optional, tag = "2")]
    pub binary: ::core::option::Option<File>,
    /// File information for libraries loaded by the process.
    #[prost(message, repeated, tag = "3")]
    pub libraries: ::prost::alloc::vec::Vec<File>,
    /// When the process represents the invocation of a script, `binary` provides
    /// information about the interpreter, while `script` provides information
    /// about the script file provided to the interpreter.
    #[prost(message, optional, tag = "4")]
    pub script: ::core::option::Option<File>,
    /// Process arguments as JSON encoded strings.
    #[prost(string, repeated, tag = "5")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// True if `args` is incomplete.
    #[prost(bool, tag = "6")]
    pub arguments_truncated: bool,
    /// Process environment variables.
    #[prost(message, repeated, tag = "7")]
    pub env_variables: ::prost::alloc::vec::Vec<EnvironmentVariable>,
    /// True if `env_variables` is incomplete.
    #[prost(bool, tag = "8")]
    pub env_variables_truncated: bool,
    /// The process ID.
    #[prost(int64, tag = "9")]
    pub pid: i64,
    /// The parent process ID.
    #[prost(int64, tag = "10")]
    pub parent_pid: i64,
}
/// A name-value pair representing an environment variable used in an operating
/// system process.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentVariable {
    /// Environment variable name as a JSON encoded string.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Environment variable value as a JSON encoded string.
    #[prost(string, tag = "2")]
    pub val: ::prost::alloc::string::String,
}
/// User specified security marks that are attached to the parent Security
/// Command Center resource. Security marks are scoped within a Security Command
/// Center organization -- they can be modified and viewed by all users who have
/// proper permissions on the organization.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityMarks {
    /// The relative resource name of the SecurityMarks. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// The following list shows some examples:
    ///
    /// + `organizations/{organization_id}/assets/{asset_id}/securityMarks`
    /// +
    /// `organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks`
    /// +
    /// `organizations/{organization_id}/sources/{source_id}/locations/{location}/findings/{finding_id}/securityMarks`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mutable user specified security marks belonging to the parent resource.
    /// Constraints are as follows:
    ///
    ///    * Keys and values are treated as case insensitive
    ///    * Keys must be between 1 - 256 characters (inclusive)
    ///    * Keys must be letters, numbers, underscores, or dashes
    ///    * Values have leading and trailing whitespace trimmed, remaining
    ///      characters must be between 1 - 4096 characters (inclusive)
    #[prost(btree_map = "string, string", tag = "2")]
    pub marks: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The canonical name of the marks. The following list shows some examples:
    ///
    /// + `organizations/{organization_id}/assets/{asset_id}/securityMarks"
    /// +
    /// `organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks"
    /// +
    /// `organizations/{organization_id}/sources/{source_id}/locations/{location}/findings/{finding_id}/securityMarks"
    /// + `folders/{folder_id}/assets/{asset_id}/securityMarks"
    /// +
    /// `folders/{folder_id}/sources/{source_id}/findings/{finding_id}/securityMarks"
    /// +
    /// `folders/{folder_id}/sources/{source_id}/locations/{location}/findings/{finding_id}/securityMarks"
    /// + `projects/{project_number}/assets/{asset_id}/securityMarks"
    /// +
    /// `projects/{project_number}/sources/{source_id}/findings/{finding_id}/securityMarks"
    /// +
    /// `projects/{project_number}/sources/{source_id}/locations/{location}/findings/{finding_id}/securityMarks"
    #[prost(string, tag = "3")]
    pub canonical_name: ::prost::alloc::string::String,
}
/// Represents a posture that is deployed on Google Cloud by the
/// Security Command Center Posture Management service.
/// A posture contains one or more policy sets. A policy set is a
/// group of policies that enforce a set of security rules on Google
/// Cloud.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityPosture {
    /// Name of the posture, for example, `CIS-Posture`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The version of the posture, for example, `c7cfa2a8`.
    #[prost(string, tag = "2")]
    pub revision_id: ::prost::alloc::string::String,
    /// The project, folder, or organization on which the posture is deployed,
    /// for example, `projects/{project_number}`.
    #[prost(string, tag = "3")]
    pub posture_deployment_resource: ::prost::alloc::string::String,
    /// The name of the posture deployment, for example,
    /// `organizations/{org_id}/posturedeployments/{posture_deployment_id}`.
    #[prost(string, tag = "4")]
    pub posture_deployment: ::prost::alloc::string::String,
    /// The name of the updated policy, for example,
    /// `projects/{project_id}/policies/{constraint_name}`.
    #[prost(string, tag = "5")]
    pub changed_policy: ::prost::alloc::string::String,
    /// The name of the updated policy set, for example, `cis-policyset`.
    #[prost(string, tag = "6")]
    pub policy_set: ::prost::alloc::string::String,
    /// The ID of the updated policy, for example, `compute-policy-1`.
    #[prost(string, tag = "7")]
    pub policy: ::prost::alloc::string::String,
    /// The details about a change in an updated policy that violates the deployed
    /// posture.
    #[prost(message, repeated, tag = "8")]
    pub policy_drift_details: ::prost::alloc::vec::Vec<
        security_posture::PolicyDriftDetails,
    >,
}
/// Nested message and enum types in `SecurityPosture`.
pub mod security_posture {
    /// The policy field that violates the deployed posture and its expected and
    /// detected values.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PolicyDriftDetails {
        /// The name of the updated field, for example
        /// constraint.implementation.policy_rules\[0\].enforce
        #[prost(string, tag = "1")]
        pub field: ::prost::alloc::string::String,
        /// The value of this field that was configured in a posture, for example,
        /// `true` or `allowed_values={"projects/29831892"}`.
        #[prost(string, tag = "2")]
        pub expected_value: ::prost::alloc::string::String,
        /// The detected value that violates the deployed posture, for example,
        /// `false` or `allowed_values={"projects/22831892"}`.
        #[prost(string, tag = "3")]
        pub detected_value: ::prost::alloc::string::String,
    }
}
/// Refers to common vulnerability fields e.g. cve, cvss, cwe etc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vulnerability {
    /// CVE stands for Common Vulnerabilities and Exposures
    /// (<https://cve.mitre.org/about/>)
    #[prost(message, optional, tag = "1")]
    pub cve: ::core::option::Option<Cve>,
    /// The offending package is relevant to the finding.
    #[prost(message, optional, tag = "2")]
    pub offending_package: ::core::option::Option<Package>,
    /// The fixed package is relevant to the finding.
    #[prost(message, optional, tag = "3")]
    pub fixed_package: ::core::option::Option<Package>,
    /// The security bulletin is relevant to this finding.
    #[prost(message, optional, tag = "4")]
    pub security_bulletin: ::core::option::Option<SecurityBulletin>,
}
/// CVE stands for Common Vulnerabilities and Exposures.
/// Information from the [CVE
/// record](<https://www.cve.org/ResourcesSupport/Glossary>) that describes this
/// vulnerability.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cve {
    /// The unique identifier for the vulnerability. e.g. CVE-2021-34527
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Additional information about the CVE.
    /// e.g. <https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-34527>
    #[prost(message, repeated, tag = "2")]
    pub references: ::prost::alloc::vec::Vec<Reference>,
    /// Describe Common Vulnerability Scoring System specified at
    /// <https://www.first.org/cvss/v3.1/specification-document>
    #[prost(message, optional, tag = "3")]
    pub cvssv3: ::core::option::Option<Cvssv3>,
    /// Whether upstream fix is available for the CVE.
    #[prost(bool, tag = "4")]
    pub upstream_fix_available: bool,
    /// The potential impact of the vulnerability if it was to be exploited.
    #[prost(enumeration = "cve::RiskRating", tag = "5")]
    pub impact: i32,
    /// The exploitation activity of the vulnerability in the wild.
    #[prost(enumeration = "cve::ExploitationActivity", tag = "6")]
    pub exploitation_activity: i32,
    /// Whether or not the vulnerability has been observed in the wild.
    #[prost(bool, tag = "7")]
    pub observed_in_the_wild: bool,
    /// Whether or not the vulnerability was zero day when the finding was
    /// published.
    #[prost(bool, tag = "8")]
    pub zero_day: bool,
}
/// Nested message and enum types in `Cve`.
pub mod cve {
    /// The possible values of impact of the vulnerability if it was to be
    /// exploited.
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
    pub enum RiskRating {
        /// Invalid or empty value.
        Unspecified = 0,
        /// Exploitation would have little to no security impact.
        Low = 1,
        /// Exploitation would enable attackers to perform activities, or could allow
        /// attackers to have a direct impact, but would require additional steps.
        Medium = 2,
        /// Exploitation would enable attackers to have a notable direct impact
        /// without needing to overcome any major mitigating factors.
        High = 3,
        /// Exploitation would fundamentally undermine the security of affected
        /// systems, enable actors to perform significant attacks with minimal
        /// effort, with little to no mitigating factors to overcome.
        Critical = 4,
    }
    impl RiskRating {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RiskRating::Unspecified => "RISK_RATING_UNSPECIFIED",
                RiskRating::Low => "LOW",
                RiskRating::Medium => "MEDIUM",
                RiskRating::High => "HIGH",
                RiskRating::Critical => "CRITICAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RISK_RATING_UNSPECIFIED" => Some(Self::Unspecified),
                "LOW" => Some(Self::Low),
                "MEDIUM" => Some(Self::Medium),
                "HIGH" => Some(Self::High),
                "CRITICAL" => Some(Self::Critical),
                _ => None,
            }
        }
    }
    /// The possible values of exploitation activity of the vulnerability in the
    /// wild.
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
    pub enum ExploitationActivity {
        /// Invalid or empty value.
        Unspecified = 0,
        /// Exploitation has been reported or confirmed to widely occur.
        Wide = 1,
        /// Limited reported or confirmed exploitation activities.
        Confirmed = 2,
        /// Exploit is publicly available.
        Available = 3,
        /// No known exploitation activity, but has a high potential for
        /// exploitation.
        Anticipated = 4,
        /// No known exploitation activity.
        NoKnown = 5,
    }
    impl ExploitationActivity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExploitationActivity::Unspecified => "EXPLOITATION_ACTIVITY_UNSPECIFIED",
                ExploitationActivity::Wide => "WIDE",
                ExploitationActivity::Confirmed => "CONFIRMED",
                ExploitationActivity::Available => "AVAILABLE",
                ExploitationActivity::Anticipated => "ANTICIPATED",
                ExploitationActivity::NoKnown => "NO_KNOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EXPLOITATION_ACTIVITY_UNSPECIFIED" => Some(Self::Unspecified),
                "WIDE" => Some(Self::Wide),
                "CONFIRMED" => Some(Self::Confirmed),
                "AVAILABLE" => Some(Self::Available),
                "ANTICIPATED" => Some(Self::Anticipated),
                "NO_KNOWN" => Some(Self::NoKnown),
                _ => None,
            }
        }
    }
}
/// Additional Links
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reference {
    /// Source of the reference e.g. NVD
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// Uri for the mentioned source e.g.
    /// <https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-34527.>
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
/// Common Vulnerability Scoring System version 3.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cvssv3 {
    /// The base score is a function of the base metric scores.
    #[prost(double, tag = "1")]
    pub base_score: f64,
    /// Base Metrics
    /// Represents the intrinsic characteristics of a vulnerability that are
    /// constant over time and across user environments.
    /// This metric reflects the context by which vulnerability exploitation is
    /// possible.
    #[prost(enumeration = "cvssv3::AttackVector", tag = "2")]
    pub attack_vector: i32,
    /// This metric describes the conditions beyond the attacker's control that
    /// must exist in order to exploit the vulnerability.
    #[prost(enumeration = "cvssv3::AttackComplexity", tag = "3")]
    pub attack_complexity: i32,
    /// This metric describes the level of privileges an attacker must possess
    /// before successfully exploiting the vulnerability.
    #[prost(enumeration = "cvssv3::PrivilegesRequired", tag = "4")]
    pub privileges_required: i32,
    /// This metric captures the requirement for a human user, other than the
    /// attacker, to participate in the successful compromise of the vulnerable
    /// component.
    #[prost(enumeration = "cvssv3::UserInteraction", tag = "5")]
    pub user_interaction: i32,
    /// The Scope metric captures whether a vulnerability in one vulnerable
    /// component impacts resources in components beyond its security scope.
    #[prost(enumeration = "cvssv3::Scope", tag = "6")]
    pub scope: i32,
    /// This metric measures the impact to the confidentiality of the information
    /// resources managed by a software component due to a successfully exploited
    /// vulnerability.
    #[prost(enumeration = "cvssv3::Impact", tag = "7")]
    pub confidentiality_impact: i32,
    /// This metric measures the impact to integrity of a successfully exploited
    /// vulnerability.
    #[prost(enumeration = "cvssv3::Impact", tag = "8")]
    pub integrity_impact: i32,
    /// This metric measures the impact to the availability of the impacted
    /// component resulting from a successfully exploited vulnerability.
    #[prost(enumeration = "cvssv3::Impact", tag = "9")]
    pub availability_impact: i32,
}
/// Nested message and enum types in `Cvssv3`.
pub mod cvssv3 {
    /// This metric reflects the context by which vulnerability exploitation is
    /// possible.
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
    pub enum AttackVector {
        /// Invalid value.
        Unspecified = 0,
        /// The vulnerable component is bound to the network stack and the set of
        /// possible attackers extends beyond the other options listed below, up to
        /// and including the entire Internet.
        Network = 1,
        /// The vulnerable component is bound to the network stack, but the attack is
        /// limited at the protocol level to a logically adjacent topology.
        Adjacent = 2,
        /// The vulnerable component is not bound to the network stack and the
        /// attacker's path is via read/write/execute capabilities.
        Local = 3,
        /// The attack requires the attacker to physically touch or manipulate the
        /// vulnerable component.
        Physical = 4,
    }
    impl AttackVector {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AttackVector::Unspecified => "ATTACK_VECTOR_UNSPECIFIED",
                AttackVector::Network => "ATTACK_VECTOR_NETWORK",
                AttackVector::Adjacent => "ATTACK_VECTOR_ADJACENT",
                AttackVector::Local => "ATTACK_VECTOR_LOCAL",
                AttackVector::Physical => "ATTACK_VECTOR_PHYSICAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ATTACK_VECTOR_UNSPECIFIED" => Some(Self::Unspecified),
                "ATTACK_VECTOR_NETWORK" => Some(Self::Network),
                "ATTACK_VECTOR_ADJACENT" => Some(Self::Adjacent),
                "ATTACK_VECTOR_LOCAL" => Some(Self::Local),
                "ATTACK_VECTOR_PHYSICAL" => Some(Self::Physical),
                _ => None,
            }
        }
    }
    /// This metric describes the conditions beyond the attacker's control that
    /// must exist in order to exploit the vulnerability.
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
    pub enum AttackComplexity {
        /// Invalid value.
        Unspecified = 0,
        /// Specialized access conditions or extenuating circumstances do not exist.
        /// An attacker can expect repeatable success when attacking the vulnerable
        /// component.
        Low = 1,
        /// A successful attack depends on conditions beyond the attacker's control.
        /// That is, a successful attack cannot be accomplished at will, but requires
        /// the attacker to invest in some measurable amount of effort in preparation
        /// or execution against the vulnerable component before a successful attack
        /// can be expected.
        High = 2,
    }
    impl AttackComplexity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AttackComplexity::Unspecified => "ATTACK_COMPLEXITY_UNSPECIFIED",
                AttackComplexity::Low => "ATTACK_COMPLEXITY_LOW",
                AttackComplexity::High => "ATTACK_COMPLEXITY_HIGH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ATTACK_COMPLEXITY_UNSPECIFIED" => Some(Self::Unspecified),
                "ATTACK_COMPLEXITY_LOW" => Some(Self::Low),
                "ATTACK_COMPLEXITY_HIGH" => Some(Self::High),
                _ => None,
            }
        }
    }
    /// This metric describes the level of privileges an attacker must possess
    /// before successfully exploiting the vulnerability.
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
    pub enum PrivilegesRequired {
        /// Invalid value.
        Unspecified = 0,
        /// The attacker is unauthorized prior to attack, and therefore does not
        /// require any access to settings or files of the vulnerable system to
        /// carry out an attack.
        None = 1,
        /// The attacker requires privileges that provide basic user capabilities
        /// that could normally affect only settings and files owned by a user.
        /// Alternatively, an attacker with Low privileges has the ability to access
        /// only non-sensitive resources.
        Low = 2,
        /// The attacker requires privileges that provide significant (e.g.,
        /// administrative) control over the vulnerable component allowing access to
        /// component-wide settings and files.
        High = 3,
    }
    impl PrivilegesRequired {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PrivilegesRequired::Unspecified => "PRIVILEGES_REQUIRED_UNSPECIFIED",
                PrivilegesRequired::None => "PRIVILEGES_REQUIRED_NONE",
                PrivilegesRequired::Low => "PRIVILEGES_REQUIRED_LOW",
                PrivilegesRequired::High => "PRIVILEGES_REQUIRED_HIGH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PRIVILEGES_REQUIRED_UNSPECIFIED" => Some(Self::Unspecified),
                "PRIVILEGES_REQUIRED_NONE" => Some(Self::None),
                "PRIVILEGES_REQUIRED_LOW" => Some(Self::Low),
                "PRIVILEGES_REQUIRED_HIGH" => Some(Self::High),
                _ => None,
            }
        }
    }
    /// This metric captures the requirement for a human user, other than the
    /// attacker, to participate in the successful compromise of the vulnerable
    /// component.
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
    pub enum UserInteraction {
        /// Invalid value.
        Unspecified = 0,
        /// The vulnerable system can be exploited without interaction from any user.
        None = 1,
        /// Successful exploitation of this vulnerability requires a user to take
        /// some action before the vulnerability can be exploited.
        Required = 2,
    }
    impl UserInteraction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserInteraction::Unspecified => "USER_INTERACTION_UNSPECIFIED",
                UserInteraction::None => "USER_INTERACTION_NONE",
                UserInteraction::Required => "USER_INTERACTION_REQUIRED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "USER_INTERACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "USER_INTERACTION_NONE" => Some(Self::None),
                "USER_INTERACTION_REQUIRED" => Some(Self::Required),
                _ => None,
            }
        }
    }
    /// The Scope metric captures whether a vulnerability in one vulnerable
    /// component impacts resources in components beyond its security scope.
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
    pub enum Scope {
        /// Invalid value.
        Unspecified = 0,
        /// An exploited vulnerability can only affect resources managed by the same
        /// security authority.
        Unchanged = 1,
        /// An exploited vulnerability can affect resources beyond the security scope
        /// managed by the security authority of the vulnerable component.
        Changed = 2,
    }
    impl Scope {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Scope::Unspecified => "SCOPE_UNSPECIFIED",
                Scope::Unchanged => "SCOPE_UNCHANGED",
                Scope::Changed => "SCOPE_CHANGED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SCOPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SCOPE_UNCHANGED" => Some(Self::Unchanged),
                "SCOPE_CHANGED" => Some(Self::Changed),
                _ => None,
            }
        }
    }
    /// The Impact metrics capture the effects of a successfully exploited
    /// vulnerability on the component that suffers the worst outcome that is most
    /// directly and predictably associated with the attack.
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
    pub enum Impact {
        /// Invalid value.
        Unspecified = 0,
        /// High impact.
        High = 1,
        /// Low impact.
        Low = 2,
        /// No impact.
        None = 3,
    }
    impl Impact {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Impact::Unspecified => "IMPACT_UNSPECIFIED",
                Impact::High => "IMPACT_HIGH",
                Impact::Low => "IMPACT_LOW",
                Impact::None => "IMPACT_NONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IMPACT_UNSPECIFIED" => Some(Self::Unspecified),
                "IMPACT_HIGH" => Some(Self::High),
                "IMPACT_LOW" => Some(Self::Low),
                "IMPACT_NONE" => Some(Self::None),
                _ => None,
            }
        }
    }
}
/// Package is a generic definition of a package.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Package {
    /// The name of the package where the vulnerability was detected.
    #[prost(string, tag = "1")]
    pub package_name: ::prost::alloc::string::String,
    /// The CPE URI where the vulnerability was detected.
    #[prost(string, tag = "2")]
    pub cpe_uri: ::prost::alloc::string::String,
    /// Type of package, for example, os, maven, or go.
    #[prost(string, tag = "3")]
    pub package_type: ::prost::alloc::string::String,
    /// The version of the package.
    #[prost(string, tag = "4")]
    pub package_version: ::prost::alloc::string::String,
}
/// SecurityBulletin are notifications of vulnerabilities of Google products.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityBulletin {
    /// ID of the bulletin corresponding to the vulnerability.
    #[prost(string, tag = "1")]
    pub bulletin_id: ::prost::alloc::string::String,
    /// Submission time of this Security Bulletin.
    #[prost(message, optional, tag = "2")]
    pub submission_time: ::core::option::Option<::prost_types::Timestamp>,
    /// This represents a version that the cluster receiving this notification
    /// should be upgraded to, based on its current version. For example, 1.15.0
    #[prost(string, tag = "3")]
    pub suggested_upgrade_version: ::prost::alloc::string::String,
}
/// Security Command Center finding.
///
/// A finding is a record of assessment data like security, risk, health, or
/// privacy, that is ingested into Security Command Center for presentation,
/// notification, analysis, policy testing, and enforcement. For example, a
/// cross-site scripting (XSS) vulnerability in an App Engine application is a
/// finding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Finding {
    /// The [relative resource
    /// name](<https://cloud.google.com/apis/design/resource_names#relative_resource_name>)
    /// of the finding. The following list shows some examples:
    ///
    /// +
    /// `organizations/{organization_id}/sources/{source_id}/findings/{finding_id}`
    /// +
    /// `organizations/{organization_id}/sources/{source_id}/locations/{location_id}/findings/{finding_id}`
    /// + `folders/{folder_id}/sources/{source_id}/findings/{finding_id}`
    /// +
    /// `folders/{folder_id}/sources/{source_id}/locations/{location_id}/findings/{finding_id}`
    /// + `projects/{project_id}/sources/{source_id}/findings/{finding_id}`
    /// +
    /// `projects/{project_id}/sources/{source_id}/locations/{location_id}/findings/{finding_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The canonical name of the finding. The following list shows
    /// some examples:
    ///
    /// +
    /// `organizations/{organization_id}/sources/{source_id}/findings/{finding_id}`
    /// +
    /// `organizations/{organization_id}/sources/{source_id}/locations/{location_id}/findings/{finding_id}`
    /// + `folders/{folder_id}/sources/{source_id}/findings/{finding_id}`
    /// +
    /// `folders/{folder_id}/sources/{source_id}/locations/{location_id}/findings/{finding_id}`
    /// + `projects/{project_id}/sources/{source_id}/findings/{finding_id}`
    /// +
    /// `projects/{project_id}/sources/{source_id}/locations/{location_id}/findings/{finding_id}`
    ///
    /// The prefix is the closest CRM ancestor of the resource associated with the
    /// finding.
    #[prost(string, tag = "2")]
    pub canonical_name: ::prost::alloc::string::String,
    /// The relative resource name of the source and location the finding belongs
    /// to. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// This field is immutable after creation time. The following list shows some
    /// examples:
    ///
    /// + `organizations/{organization_id}/sources/{source_id}`
    /// + `folders/{folders_id}/sources/{source_id}`
    /// + `projects/{projects_id}/sources/{source_id}`
    /// +
    /// `organizations/{organization_id}/sources/{source_id}/locations/{location_id}`
    /// + `folders/{folders_id}/sources/{source_id}/locations/{location_id}`
    /// + `projects/{projects_id}/sources/{source_id}/locations/{location_id}`
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Immutable. For findings on Google Cloud resources, the full resource
    /// name of the Google Cloud resource this finding is for. See:
    /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
    /// When the finding is for a non-Google Cloud resource, the resourceName can
    /// be a customer or partner defined string.
    #[prost(string, tag = "4")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The state of the finding.
    #[prost(enumeration = "finding::State", tag = "6")]
    pub state: i32,
    /// Immutable. The additional taxonomy group within findings from a given
    /// source. Example: "XSS_FLASH_INJECTION"
    #[prost(string, tag = "7")]
    pub category: ::prost::alloc::string::String,
    /// The URI that, if available, points to a web page outside of Security
    /// Command Center where additional information about the finding can be found.
    /// This field is guaranteed to be either empty or a well formed URL.
    #[prost(string, tag = "8")]
    pub external_uri: ::prost::alloc::string::String,
    /// Source specific properties. These properties are managed by the source
    /// that writes the finding. The key names in the source_properties map must be
    /// between 1 and 255 characters, and must start with a letter and contain
    /// alphanumeric characters or underscores only.
    #[prost(btree_map = "string, message", tag = "9")]
    pub source_properties: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
    /// Output only. User specified security marks. These marks are entirely
    /// managed by the user and come from the SecurityMarks resource that belongs
    /// to the finding.
    #[prost(message, optional, tag = "10")]
    pub security_marks: ::core::option::Option<SecurityMarks>,
    /// The time the finding was first detected. If an existing finding is updated,
    /// then this is the time the update occurred.
    /// For example, if the finding represents an open firewall, this property
    /// captures the time the detector believes the firewall became open. The
    /// accuracy is determined by the detector. If the finding is later resolved,
    /// then this time reflects when the finding was resolved. This must not
    /// be set to a value greater than the current timestamp.
    #[prost(message, optional, tag = "11")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which the finding was created in Security Command
    /// Center.
    #[prost(message, optional, tag = "12")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The severity of the finding. This field is managed by the source that
    /// writes the finding.
    #[prost(enumeration = "finding::Severity", tag = "14")]
    pub severity: i32,
    /// Indicates the mute state of a finding (either muted, unmuted
    /// or undefined). Unlike other attributes of a finding, a finding provider
    /// shouldn't set the value of mute.
    #[prost(enumeration = "finding::Mute", tag = "15")]
    pub mute: i32,
    /// The class of the finding.
    #[prost(enumeration = "finding::FindingClass", tag = "16")]
    pub finding_class: i32,
    /// Represents what's commonly known as an *indicator of compromise* (IoC) in
    /// computer forensics. This is an artifact observed on a network or in an
    /// operating system that, with high confidence, indicates a computer
    /// intrusion. For more information, see [Indicator of
    /// compromise](<https://en.wikipedia.org/wiki/Indicator_of_compromise>).
    #[prost(message, optional, tag = "17")]
    pub indicator: ::core::option::Option<Indicator>,
    /// Represents vulnerability-specific fields like CVE and CVSS scores.
    /// CVE stands for Common Vulnerabilities and Exposures
    /// (<https://cve.mitre.org/about/>)
    #[prost(message, optional, tag = "18")]
    pub vulnerability: ::core::option::Option<Vulnerability>,
    /// Output only. The most recent time this finding was muted or unmuted.
    #[prost(message, optional, tag = "19")]
    pub mute_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Third party SIEM/SOAR fields within SCC, contains external
    /// system information and external system finding fields.
    #[prost(btree_map = "string, message", tag = "20")]
    pub external_systems: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ExternalSystem,
    >,
    /// MITRE ATT&CK tactics and techniques related to this finding.
    /// See: <https://attack.mitre.org>
    #[prost(message, optional, tag = "21")]
    pub mitre_attack: ::core::option::Option<MitreAttack>,
    /// Access details associated with the finding, such as more information on the
    /// caller, which method was accessed, and from where.
    #[prost(message, optional, tag = "22")]
    pub access: ::core::option::Option<Access>,
    /// Contains information about the IP connection associated with the finding.
    #[prost(message, repeated, tag = "23")]
    pub connections: ::prost::alloc::vec::Vec<Connection>,
    /// Records additional information about the mute operation, for example, the
    /// [mute
    /// configuration](<https://cloud.google.com/security-command-center/docs/how-to-mute-findings>)
    /// that muted the finding and the user who muted the finding.
    #[prost(string, tag = "24")]
    pub mute_initiator: ::prost::alloc::string::String,
    /// Represents operating system processes associated with the Finding.
    #[prost(message, repeated, tag = "25")]
    pub processes: ::prost::alloc::vec::Vec<Process>,
    /// Output only. Map containing the points of contact for the given finding.
    /// The key represents the type of contact, while the value contains a list of
    /// all the contacts that pertain. Please refer to:
    /// <https://cloud.google.com/resource-manager/docs/managing-notification-contacts#notification-categories>
    ///
    ///      {
    ///        "security": {
    ///          "contacts": [
    ///            {
    ///              "email": "person1@company.com"
    ///            },
    ///            {
    ///              "email": "person2@company.com"
    ///            }
    ///          ]
    ///        }
    ///      }
    #[prost(btree_map = "string, message", tag = "26")]
    pub contacts: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ContactDetails,
    >,
    /// Contains compliance information for security standards associated to the
    /// finding.
    #[prost(message, repeated, tag = "27")]
    pub compliances: ::prost::alloc::vec::Vec<Compliance>,
    /// Output only. The human readable display name of the finding source such as
    /// "Event Threat Detection" or "Security Health Analytics".
    #[prost(string, tag = "29")]
    pub parent_display_name: ::prost::alloc::string::String,
    /// Contains more details about the finding.
    #[prost(string, tag = "30")]
    pub description: ::prost::alloc::string::String,
    /// Represents exfiltrations associated with the finding.
    #[prost(message, optional, tag = "31")]
    pub exfiltration: ::core::option::Option<Exfiltration>,
    /// Represents IAM bindings associated with the finding.
    #[prost(message, repeated, tag = "32")]
    pub iam_bindings: ::prost::alloc::vec::Vec<IamBinding>,
    /// Steps to address the finding.
    #[prost(string, tag = "33")]
    pub next_steps: ::prost::alloc::string::String,
    /// Unique identifier of the module which generated the finding.
    /// Example:
    /// folders/598186756061/securityHealthAnalyticsSettings/customModules/56799441161885
    #[prost(string, tag = "34")]
    pub module_name: ::prost::alloc::string::String,
    /// Containers associated with the finding. This field provides information for
    /// both Kubernetes and non-Kubernetes containers.
    #[prost(message, repeated, tag = "35")]
    pub containers: ::prost::alloc::vec::Vec<Container>,
    /// Kubernetes resources associated with the finding.
    #[prost(message, optional, tag = "36")]
    pub kubernetes: ::core::option::Option<Kubernetes>,
    /// Database associated with the finding.
    #[prost(message, optional, tag = "37")]
    pub database: ::core::option::Option<Database>,
    /// The results of an attack path simulation relevant to this finding.
    #[prost(message, optional, tag = "38")]
    pub attack_exposure: ::core::option::Option<AttackExposure>,
    /// File associated with the finding.
    #[prost(message, repeated, tag = "39")]
    pub files: ::prost::alloc::vec::Vec<File>,
    /// Cloud Data Loss Prevention (Cloud DLP) inspection results that are
    /// associated with the finding.
    #[prost(message, optional, tag = "40")]
    pub cloud_dlp_inspection: ::core::option::Option<CloudDlpInspection>,
    /// Cloud DLP data profile that is associated with the finding.
    #[prost(message, optional, tag = "41")]
    pub cloud_dlp_data_profile: ::core::option::Option<CloudDlpDataProfile>,
    /// Signature of the kernel rootkit.
    #[prost(message, optional, tag = "42")]
    pub kernel_rootkit: ::core::option::Option<KernelRootkit>,
    /// Contains information about the org policies associated with the finding.
    #[prost(message, repeated, tag = "43")]
    pub org_policies: ::prost::alloc::vec::Vec<OrgPolicy>,
    /// Represents an application associated with the finding.
    #[prost(message, optional, tag = "45")]
    pub application: ::core::option::Option<Application>,
    /// Fields related to Backup and DR findings.
    #[prost(message, optional, tag = "47")]
    pub backup_disaster_recovery: ::core::option::Option<BackupDisasterRecovery>,
    /// The security posture associated with the finding.
    #[prost(message, optional, tag = "48")]
    pub security_posture: ::core::option::Option<SecurityPosture>,
    /// Log entries that are relevant to the finding.
    #[prost(message, repeated, tag = "49")]
    pub log_entries: ::prost::alloc::vec::Vec<LogEntry>,
    /// The load balancers associated with the finding.
    #[prost(message, repeated, tag = "50")]
    pub load_balancers: ::prost::alloc::vec::Vec<LoadBalancer>,
}
/// Nested message and enum types in `Finding`.
pub mod finding {
    /// The state of the finding.
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
        /// The finding requires attention and has not been addressed yet.
        Active = 1,
        /// The finding has been fixed, triaged as a non-issue or otherwise addressed
        /// and is no longer active.
        Inactive = 2,
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
                State::Inactive => "INACTIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "INACTIVE" => Some(Self::Inactive),
                _ => None,
            }
        }
    }
    /// The severity of the finding.
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
        /// This value is used for findings when a source doesn't write a severity
        /// value.
        Unspecified = 0,
        /// Vulnerability:
        /// A critical vulnerability is easily discoverable by an external actor,
        /// exploitable, and results in the direct ability to execute arbitrary code,
        /// exfiltrate data, and otherwise gain additional access and privileges to
        /// cloud resources and workloads. Examples include publicly accessible
        /// unprotected user data and public SSH access with weak or no
        /// passwords.
        ///
        /// Threat:
        /// Indicates a threat that is able to access, modify, or delete data or
        /// execute unauthorized code within existing resources.
        Critical = 1,
        /// Vulnerability:
        /// A high risk vulnerability can be easily discovered and exploited in
        /// combination with other vulnerabilities in order to gain direct access and
        /// the ability to execute arbitrary code, exfiltrate data, and otherwise
        /// gain additional access and privileges to cloud resources and workloads.
        /// An example is a database with weak or no passwords that is only
        /// accessible internally. This database could easily be compromised by an
        /// actor that had access to the internal network.
        ///
        /// Threat:
        /// Indicates a threat that is able to create new computational resources in
        /// an environment but not able to access data or execute code in existing
        /// resources.
        High = 2,
        /// Vulnerability:
        /// A medium risk vulnerability could be used by an actor to gain access to
        /// resources or privileges that enable them to eventually (through multiple
        /// steps or a complex exploit) gain access and the ability to execute
        /// arbitrary code or exfiltrate data. An example is a service account with
        /// access to more projects than it should have. If an actor gains access to
        /// the service account, they could potentially use that access to manipulate
        /// a project the service account was not intended to.
        ///
        /// Threat:
        /// Indicates a threat that is able to cause operational impact but may not
        /// access data or execute unauthorized code.
        Medium = 3,
        /// Vulnerability:
        /// A low risk vulnerability hampers a security organization's ability to
        /// detect vulnerabilities or active threats in their deployment, or prevents
        /// the root cause investigation of security issues. An example is monitoring
        /// and logs being disabled for resource configurations and access.
        ///
        /// Threat:
        /// Indicates a threat that has obtained minimal access to an environment but
        /// is not able to access data, execute code, or create resources.
        Low = 4,
    }
    impl Severity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Severity::Unspecified => "SEVERITY_UNSPECIFIED",
                Severity::Critical => "CRITICAL",
                Severity::High => "HIGH",
                Severity::Medium => "MEDIUM",
                Severity::Low => "LOW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
                "CRITICAL" => Some(Self::Critical),
                "HIGH" => Some(Self::High),
                "MEDIUM" => Some(Self::Medium),
                "LOW" => Some(Self::Low),
                _ => None,
            }
        }
    }
    /// Mute state a finding can be in.
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
    pub enum Mute {
        /// Unspecified.
        Unspecified = 0,
        /// Finding has been muted.
        Muted = 1,
        /// Finding has been unmuted.
        Unmuted = 2,
        /// Finding has never been muted/unmuted.
        Undefined = 3,
    }
    impl Mute {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mute::Unspecified => "MUTE_UNSPECIFIED",
                Mute::Muted => "MUTED",
                Mute::Unmuted => "UNMUTED",
                Mute::Undefined => "UNDEFINED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MUTE_UNSPECIFIED" => Some(Self::Unspecified),
                "MUTED" => Some(Self::Muted),
                "UNMUTED" => Some(Self::Unmuted),
                "UNDEFINED" => Some(Self::Undefined),
                _ => None,
            }
        }
    }
    /// Represents what kind of Finding it is.
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
    pub enum FindingClass {
        /// Unspecified finding class.
        Unspecified = 0,
        /// Describes unwanted or malicious activity.
        Threat = 1,
        /// Describes a potential weakness in software that increases risk to
        /// Confidentiality & Integrity & Availability.
        Vulnerability = 2,
        /// Describes a potential weakness in cloud resource/asset configuration that
        /// increases risk.
        Misconfiguration = 3,
        /// Describes a security observation that is for informational purposes.
        Observation = 4,
        /// Describes an error that prevents some SCC functionality.
        SccError = 5,
        /// Describes a potential security risk due to a change in the security
        /// posture.
        PostureViolation = 6,
    }
    impl FindingClass {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FindingClass::Unspecified => "FINDING_CLASS_UNSPECIFIED",
                FindingClass::Threat => "THREAT",
                FindingClass::Vulnerability => "VULNERABILITY",
                FindingClass::Misconfiguration => "MISCONFIGURATION",
                FindingClass::Observation => "OBSERVATION",
                FindingClass::SccError => "SCC_ERROR",
                FindingClass::PostureViolation => "POSTURE_VIOLATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FINDING_CLASS_UNSPECIFIED" => Some(Self::Unspecified),
                "THREAT" => Some(Self::Threat),
                "VULNERABILITY" => Some(Self::Vulnerability),
                "MISCONFIGURATION" => Some(Self::Misconfiguration),
                "OBSERVATION" => Some(Self::Observation),
                "SCC_ERROR" => Some(Self::SccError),
                "POSTURE_VIOLATION" => Some(Self::PostureViolation),
                _ => None,
            }
        }
    }
}
/// Information related to the Google Cloud resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The full resource name of the resource. See:
    /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The human readable name of the resource.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The full resource type of the resource.
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
}
/// Cloud SCC's Notification
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationMessage {
    /// Name of the notification config that generated current notification.
    #[prost(string, tag = "1")]
    pub notification_config_name: ::prost::alloc::string::String,
    /// The Cloud resource tied to this notification's Finding.
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<Resource>,
    /// Notification Event.
    #[prost(oneof = "notification_message::Event", tags = "2")]
    pub event: ::core::option::Option<notification_message::Event>,
}
/// Nested message and enum types in `NotificationMessage`.
pub mod notification_message {
    /// Notification Event.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// If it's a Finding based notification config, this field will be
        /// populated.
        #[prost(message, tag = "2")]
        Finding(super::Finding),
    }
}
/// A path that an attacker could take to reach an exposed resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttackPath {
    /// The attack path name, for example,
    ///   `organizations/12/simulations/34/valuedResources/56/attackPaths/78`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of nodes that exist in this attack path.
    #[prost(message, repeated, tag = "2")]
    pub path_nodes: ::prost::alloc::vec::Vec<attack_path::AttackPathNode>,
    /// A list of the edges between nodes in this attack path.
    #[prost(message, repeated, tag = "3")]
    pub edges: ::prost::alloc::vec::Vec<attack_path::AttackPathEdge>,
}
/// Nested message and enum types in `AttackPath`.
pub mod attack_path {
    /// Represents one point that an attacker passes through in this attack path.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AttackPathNode {
        /// The name of the resource at this point in the attack path.
        /// The format of the name follows the Cloud Asset Inventory [resource
        /// name
        /// format]("<https://cloud.google.com/asset-inventory/docs/resource-name-format">)
        #[prost(string, tag = "1")]
        pub resource: ::prost::alloc::string::String,
        /// The [supported resource
        /// type](<https://cloud.google.com/asset-inventory/docs/supported-asset-types">)
        #[prost(string, tag = "2")]
        pub resource_type: ::prost::alloc::string::String,
        /// Human-readable name of this resource.
        #[prost(string, tag = "3")]
        pub display_name: ::prost::alloc::string::String,
        /// The findings associated with this node in the attack path.
        #[prost(message, repeated, tag = "4")]
        pub associated_findings: ::prost::alloc::vec::Vec<
            attack_path_node::PathNodeAssociatedFinding,
        >,
        /// Unique id of the attack path node.
        #[prost(string, tag = "5")]
        pub uuid: ::prost::alloc::string::String,
        /// A list of attack step nodes that exist in this attack path node.
        #[prost(message, repeated, tag = "6")]
        pub attack_steps: ::prost::alloc::vec::Vec<attack_path_node::AttackStepNode>,
    }
    /// Nested message and enum types in `AttackPathNode`.
    pub mod attack_path_node {
        /// A finding that is associated with this node in the attack path.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PathNodeAssociatedFinding {
            /// Canonical name of the associated findings. Example:
            /// organizations/123/sources/456/findings/789
            #[prost(string, tag = "1")]
            pub canonical_finding: ::prost::alloc::string::String,
            /// The additional taxonomy group within findings from a given source.
            #[prost(string, tag = "2")]
            pub finding_category: ::prost::alloc::string::String,
            /// Full resource name of the finding.
            #[prost(string, tag = "3")]
            pub name: ::prost::alloc::string::String,
        }
        /// Detailed steps the attack can take between path nodes.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AttackStepNode {
            /// Unique ID for one Node
            #[prost(string, tag = "1")]
            pub uuid: ::prost::alloc::string::String,
            /// Attack step type. Can be either AND, OR or DEFENSE
            #[prost(enumeration = "NodeType", tag = "2")]
            pub r#type: i32,
            /// User friendly name of the attack step
            #[prost(string, tag = "3")]
            pub display_name: ::prost::alloc::string::String,
            /// Attack step labels for metadata
            #[prost(btree_map = "string, string", tag = "4")]
            pub labels: ::prost::alloc::collections::BTreeMap<
                ::prost::alloc::string::String,
                ::prost::alloc::string::String,
            >,
            /// Attack step description
            #[prost(string, tag = "5")]
            pub description: ::prost::alloc::string::String,
        }
        /// The type of the incoming attack step node.
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
        pub enum NodeType {
            /// Type not specified
            Unspecified = 0,
            /// Incoming edge joined with AND
            And = 1,
            /// Incoming edge joined with OR
            Or = 2,
            /// Incoming edge is defense
            Defense = 3,
            /// Incoming edge is attacker
            Attacker = 4,
        }
        impl NodeType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    NodeType::Unspecified => "NODE_TYPE_UNSPECIFIED",
                    NodeType::And => "NODE_TYPE_AND",
                    NodeType::Or => "NODE_TYPE_OR",
                    NodeType::Defense => "NODE_TYPE_DEFENSE",
                    NodeType::Attacker => "NODE_TYPE_ATTACKER",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "NODE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "NODE_TYPE_AND" => Some(Self::And),
                    "NODE_TYPE_OR" => Some(Self::Or),
                    "NODE_TYPE_DEFENSE" => Some(Self::Defense),
                    "NODE_TYPE_ATTACKER" => Some(Self::Attacker),
                    _ => None,
                }
            }
        }
    }
    /// Represents a connection between a source node and a destination node in
    /// this attack path.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AttackPathEdge {
        /// The attack node uuid of the source node.
        #[prost(string, tag = "1")]
        pub source: ::prost::alloc::string::String,
        /// The attack node uuid of the destination node.
        #[prost(string, tag = "2")]
        pub destination: ::prost::alloc::string::String,
    }
}
/// Configures how to deliver Findings to BigQuery Instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryExport {
    /// The relative resource name of this export. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name.>
    /// The following list shows some examples:
    ///
    /// +
    /// `organizations/{organization_id}/locations/{location_id}/bigQueryExports/{export_id}`
    /// + `folders/{folder_id}/locations/{location_id}/bigQueryExports/{export_id}`
    /// +
    /// `projects/{project_id}/locations/{location_id}/bigQueryExports/{export_id}`
    ///
    /// This field is provided in responses, and is ignored when provided in create
    /// requests.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The description of the export (max of 1024 characters).
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Expression that defines the filter to apply across create/update events
    /// of findings. The expression is a list of zero or more restrictions combined
    /// via logical operators `AND` and `OR`. Parentheses are supported, and `OR`
    /// has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a
    /// `-` character in front of them to indicate negation. The fields map to
    /// those defined in the corresponding resource.
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// The dataset to write findings' updates to. Its format is
    /// "projects/\[project_id\]/datasets/\[bigquery_dataset_id\]".
    /// BigQuery Dataset unique ID  must contain only letters (a-z, A-Z), numbers
    /// (0-9), or underscores (_).
    #[prost(string, tag = "4")]
    pub dataset: ::prost::alloc::string::String,
    /// Output only. The time at which the BigQuery export was created.
    /// This field is set by the server and will be ignored if provided on export
    /// on creation.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The most recent time at which the BigQuery export was updated.
    /// This field is set by the server and will be ignored if provided on export
    /// creation or update.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Email address of the user who last edited the BigQuery export.
    /// This field is set by the server and will be ignored if provided on export
    /// creation or update.
    #[prost(string, tag = "7")]
    pub most_recent_editor: ::prost::alloc::string::String,
    /// Output only. The service account that needs permission to create table and
    /// upload data to the BigQuery dataset.
    #[prost(string, tag = "8")]
    pub principal: ::prost::alloc::string::String,
}
/// A mute config is a Cloud SCC resource that contains the configuration
/// to mute create/update events of findings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuteConfig {
    /// This field will be ignored if provided on config creation. The following
    /// list shows some examples of the format:
    ///
    /// + `organizations/{organization}/muteConfigs/{mute_config}`
    /// +
    /// `organizations/{organization}locations/{location}//muteConfigs/{mute_config}`
    /// + `folders/{folder}/muteConfigs/{mute_config}`
    /// + `folders/{folder}/locations/{location}/muteConfigs/{mute_config}`
    /// + `projects/{project}/muteConfigs/{mute_config}`
    /// + `projects/{project}/locations/{location}/muteConfigs/{mute_config}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A description of the mute config.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. An expression that defines the filter to apply across
    /// create/update events of findings. While creating a filter string, be
    /// mindful of the scope in which the mute configuration is being created.
    /// E.g., If a filter contains project = X but is created under the project = Y
    /// scope, it might not match any findings.
    ///
    /// The following field and operator combinations are supported:
    ///
    /// * severity: `=`, `:`
    /// * category: `=`, `:`
    /// * resource.name: `=`, `:`
    /// * resource.project_name: `=`, `:`
    /// * resource.project_display_name: `=`, `:`
    /// * resource.folders.resource_folder: `=`, `:`
    /// * resource.parent_name: `=`, `:`
    /// * resource.parent_display_name: `=`, `:`
    /// * resource.type: `=`, `:`
    /// * finding_class: `=`, `:`
    /// * indicator.ip_addresses: `=`, `:`
    /// * indicator.domains: `=`, `:`
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// Output only. The time at which the mute config was created.
    /// This field is set by the server and will be ignored if provided on config
    /// creation.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The most recent time at which the mute config was updated.
    /// This field is set by the server and will be ignored if provided on config
    /// creation or update.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Email address of the user who last edited the mute config.
    /// This field is set by the server and will be ignored if provided on config
    /// creation or update.
    #[prost(string, tag = "6")]
    pub most_recent_editor: ::prost::alloc::string::String,
    /// Required. The type of the mute config, which determines what type of mute
    /// state the config affects. Immutable after creation.
    #[prost(enumeration = "mute_config::MuteConfigType", tag = "8")]
    pub r#type: i32,
}
/// Nested message and enum types in `MuteConfig`.
pub mod mute_config {
    /// The type of MuteConfig.
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
    pub enum MuteConfigType {
        /// Unused.
        Unspecified = 0,
        /// A static mute config, which sets the static mute state of future matching
        /// findings to muted. Once the static mute state has been set, finding or
        /// config modifications will not affect the state.
        Static = 1,
    }
    impl MuteConfigType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MuteConfigType::Unspecified => "MUTE_CONFIG_TYPE_UNSPECIFIED",
                MuteConfigType::Static => "STATIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MUTE_CONFIG_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATIC" => Some(Self::Static),
                _ => None,
            }
        }
    }
}
/// Cloud Security Command Center (Cloud SCC) notification configs.
///
/// A notification config is a Cloud SCC resource that contains the configuration
/// to send notifications for create/update events of findings, assets and etc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationConfig {
    /// The relative resource name of this notification config. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// The following list shows some examples:
    /// +
    /// `organizations/{organization_id}/locations/{location_id}/notificationConfigs/notify_public_bucket`
    /// +
    /// `folders/{folder_id}/locations/{location_id}/notificationConfigs/notify_public_bucket`
    /// +
    /// `projects/{project_id}/locations/{location_id}/notificationConfigs/notify_public_bucket`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The description of the notification config (max of 1024 characters).
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// The Pub/Sub topic to send notifications to. Its format is
    /// "projects/\[project_id\]/topics/\[topic\]".
    #[prost(string, tag = "3")]
    pub pubsub_topic: ::prost::alloc::string::String,
    /// Output only. The service account that needs "pubsub.topics.publish"
    /// permission to publish to the Pub/Sub topic.
    #[prost(string, tag = "4")]
    pub service_account: ::prost::alloc::string::String,
    /// The config for triggering notifications.
    #[prost(oneof = "notification_config::NotifyConfig", tags = "5")]
    pub notify_config: ::core::option::Option<notification_config::NotifyConfig>,
}
/// Nested message and enum types in `NotificationConfig`.
pub mod notification_config {
    /// The config for streaming-based notifications, which send each event as soon
    /// as it is detected.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StreamingConfig {
        /// Expression that defines the filter to apply across create/update events
        /// of assets or findings as specified by the event type. The expression is a
        /// list of zero or more restrictions combined via logical operators `AND`
        /// and `OR`. Parentheses are supported, and `OR` has higher precedence than
        /// `AND`.
        ///
        /// Restrictions have the form `<field> <operator> <value>` and may have a
        /// `-` character in front of them to indicate negation. The fields map to
        /// those defined in the corresponding resource.
        ///
        /// The supported operators are:
        ///
        /// * `=` for all value types.
        /// * `>`, `<`, `>=`, `<=` for integer values.
        /// * `:`, meaning substring matching, for strings.
        ///
        /// The supported value types are:
        ///
        /// * string literals in quotes.
        /// * integer literals without quotes.
        /// * boolean literals `true` and `false` without quotes.
        #[prost(string, tag = "1")]
        pub filter: ::prost::alloc::string::String,
    }
    /// The config for triggering notifications.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum NotifyConfig {
        /// The config for triggering streaming-based notifications.
        #[prost(message, tag = "5")]
        StreamingConfig(StreamingConfig),
    }
}
/// A resource value config (RVC) is a mapping configuration of user's resources
/// to resource values. Used in Attack path simulations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceValueConfig {
    /// Name for the resource value config
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Resource value level this expression represents
    /// Only required when there is no SDP mapping in the request
    #[prost(enumeration = "ResourceValue", tag = "2")]
    pub resource_value: i32,
    /// Required. Tag values combined with AND to check against.
    /// Values in the form "tagValues/123"
    /// E.g. \[ "tagValues/123", "tagValues/456", "tagValues/789" \]
    /// <https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing>
    #[prost(string, repeated, tag = "3")]
    pub tag_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Apply resource_value only to resources that match resource_type.
    /// resource_type will be checked with "AND" of other resources.
    /// E.g. "storage.googleapis.com/Bucket" with resource_value "HIGH" will
    /// apply "HIGH" value only to "storage.googleapis.com/Bucket" resources.
    #[prost(string, tag = "4")]
    pub resource_type: ::prost::alloc::string::String,
    /// Project or folder to scope this config to.
    /// For example, "project/456" would apply this config only to resources in
    /// "project/456"
    /// scope will be checked with "AND" of other resources.
    #[prost(string, tag = "5")]
    pub scope: ::prost::alloc::string::String,
    /// List of resource labels to search for, evaluated with AND.
    /// E.g. "resource_labels_selector": {"key": "value", "env": "prod"}
    /// will match resources with labels "key": "value" AND "env": "prod"
    /// <https://cloud.google.com/resource-manager/docs/creating-managing-labels>
    #[prost(btree_map = "string, string", tag = "6")]
    pub resource_labels_selector: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Description of the resource value config.
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Timestamp this resource value config was created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp this resource value config was last updated.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A mapping of the sensitivity on Sensitive Data Protection finding to
    /// resource values. This mapping can only be used in combination with a
    /// resource_type that is related to BigQuery, e.g.
    /// "bigquery.googleapis.com/Dataset".
    #[prost(message, optional, tag = "11")]
    pub sensitive_data_protection_mapping: ::core::option::Option<
        resource_value_config::SensitiveDataProtectionMapping,
    >,
}
/// Nested message and enum types in `ResourceValueConfig`.
pub mod resource_value_config {
    /// Resource value mapping for Sensitive Data Protection findings
    /// If any of these mappings have a resource value that is not unspecified,
    /// the resource_value field will be ignored when reading this configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SensitiveDataProtectionMapping {
        /// Resource value mapping for high-sensitivity Sensitive Data Protection
        /// findings
        #[prost(enumeration = "super::ResourceValue", tag = "1")]
        pub high_sensitivity_mapping: i32,
        /// Resource value mapping for medium-sensitivity Sensitive Data Protection
        /// findings
        #[prost(enumeration = "super::ResourceValue", tag = "2")]
        pub medium_sensitivity_mapping: i32,
    }
}
/// Value enum to map to a resource
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceValue {
    /// Unspecific value
    Unspecified = 0,
    /// High resource value
    High = 1,
    /// Medium resource value
    Medium = 2,
    /// Low resource value
    Low = 3,
    /// No resource value, e.g. ignore these resources
    None = 4,
}
impl ResourceValue {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceValue::Unspecified => "RESOURCE_VALUE_UNSPECIFIED",
            ResourceValue::High => "HIGH",
            ResourceValue::Medium => "MEDIUM",
            ResourceValue::Low => "LOW",
            ResourceValue::None => "NONE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESOURCE_VALUE_UNSPECIFIED" => Some(Self::Unspecified),
            "HIGH" => Some(Self::High),
            "MEDIUM" => Some(Self::Medium),
            "LOW" => Some(Self::Low),
            "NONE" => Some(Self::None),
            _ => None,
        }
    }
}
/// Security Command Center finding source. A finding source
/// is an entity or a mechanism that can produce a finding. A source is like a
/// container of findings that come from the same scanner, logger, monitor, and
/// other tools.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    /// The relative resource name of this source. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Example:
    /// "organizations/{organization_id}/sources/{source_id}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The source's display name.
    /// A source's display name must be unique amongst its siblings, for example,
    /// two sources with the same parent can't share the same display name.
    /// The display name must have a length between 1 and 64 characters
    /// (inclusive).
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The description of the source (max of 1024 characters).
    /// Example:
    /// "Web Security Scanner is a web security scanner for common
    /// vulnerabilities in App Engine applications. It can automatically
    /// scan and detect four common vulnerabilities, including cross-site-scripting
    /// (XSS), Flash injection, mixed content (HTTP in HTTPS), and
    /// outdated or insecure libraries."
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The canonical name of the finding source. It's either
    /// "organizations/{organization_id}/sources/{source_id}",
    /// "folders/{folder_id}/sources/{source_id}", or
    /// "projects/{project_number}/sources/{source_id}",
    /// depending on the closest CRM ancestor of the resource associated with the
    /// finding.
    #[prost(string, tag = "4")]
    pub canonical_name: ::prost::alloc::string::String,
}
/// Request message to create multiple resource value configs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateResourceValueConfigsRequest {
    /// Required. Resource name of the new ResourceValueConfig's parent.
    /// The parent field in the CreateResourceValueConfigRequest
    /// messages must either be empty or match this field.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The resource value configs to be created.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<CreateResourceValueConfigRequest>,
}
/// Response message for BatchCreateResourceValueConfigs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateResourceValueConfigsResponse {
    /// The resource value configs created
    #[prost(message, repeated, tag = "1")]
    pub resource_value_configs: ::prost::alloc::vec::Vec<ResourceValueConfig>,
}
/// Request message for bulk findings update.
///
/// Note:
/// 1. If multiple bulk update requests match the same resource, the order in
/// which they get executed is not defined.
/// 2. Once a bulk operation is started, there is no way to stop it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkMuteFindingsRequest {
    /// Required. The parent, at which bulk action needs to be applied. If no
    /// location is specified, findings are updated in global. The following list
    /// shows some examples:
    ///
    /// + `organizations/\[organization_id\]`
    /// + `organizations/\[organization_id\]/locations/\[location_id\]`
    /// + `folders/\[folder_id\]`
    /// + `folders/\[folder_id\]/locations/\[location_id\]`
    /// + `projects/\[project_id\]`
    /// + `projects/\[project_id\]/locations/\[location_id\]`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Expression that identifies findings that should be updated.
    /// The expression is a list of zero or more restrictions combined
    /// via logical operators `AND` and `OR`. Parentheses are supported, and `OR`
    /// has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a
    /// `-` character in front of them to indicate negation. The fields map to
    /// those defined in the corresponding resource.
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
}
/// The response to a BulkMute request. Contains the LRO information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkMuteFindingsResponse {}
/// Request message for creating a BigQuery export.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBigQueryExportRequest {
    /// Required. The name of the parent resource of the new BigQuery export. Its
    /// format is "organizations/\[organization_id\]/locations/\[location_id\]",
    /// "folders/\[folder_id\]/locations/\[location_id\]", or
    /// "projects/\[project_id\]/locations/\[location_id\]".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The BigQuery export being created.
    #[prost(message, optional, tag = "2")]
    pub big_query_export: ::core::option::Option<BigQueryExport>,
    /// Required. Unique identifier provided by the client within the parent scope.
    /// It must consist of only lowercase letters, numbers, and hyphens, must start
    /// with a letter, must end with either a letter or a number, and must be 63
    /// characters or less.
    #[prost(string, tag = "3")]
    pub big_query_export_id: ::prost::alloc::string::String,
}
/// Request message for creating a finding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFindingRequest {
    /// Required. Resource name of the new finding's parent. The following list
    /// shows some examples of the format:
    /// +
    /// `organizations/\[organization_id\]/sources/\[source_id\]`
    /// +
    /// `organizations/\[organization_id\]/sources/\[source_id\]/locations/\[location_id\]`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Unique identifier provided by the client within the parent scope.
    /// It must be alphanumeric and less than or equal to 32 characters and
    /// greater than 0 characters in length.
    #[prost(string, tag = "2")]
    pub finding_id: ::prost::alloc::string::String,
    /// Required. The Finding being created. The name and security_marks will be
    /// ignored as they are both output only fields on this resource.
    #[prost(message, optional, tag = "3")]
    pub finding: ::core::option::Option<Finding>,
}
/// Request message for creating a mute config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMuteConfigRequest {
    /// Required. Resource name of the new mute configs's parent. Its format is
    /// "organizations/\[organization_id\]/locations/\[location_id\]",
    /// "folders/\[folder_id\]/locations/\[location_id\]", or
    /// "projects/\[project_id\]/locations/\[location_id\]".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The mute config being created.
    #[prost(message, optional, tag = "2")]
    pub mute_config: ::core::option::Option<MuteConfig>,
    /// Required. Unique identifier provided by the client within the parent scope.
    /// It must consist of only lowercase letters, numbers, and hyphens, must start
    /// with a letter, must end with either a letter or a number, and must be 63
    /// characters or less.
    #[prost(string, tag = "3")]
    pub mute_config_id: ::prost::alloc::string::String,
}
/// Request message for creating a notification config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNotificationConfigRequest {
    /// Required. Resource name of the new notification config's parent. Its format
    /// is "organizations/\[organization_id\]/locations/\[location_id\]",
    /// "folders/\[folder_id\]/locations/\[location_id\]", or
    /// "projects/\[project_id\]/locations/\[location_id\]".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required.
    /// Unique identifier provided by the client within the parent scope.
    /// It must be between 1 and 128 characters and contain alphanumeric
    /// characters, underscores, or hyphens only.
    #[prost(string, tag = "2")]
    pub config_id: ::prost::alloc::string::String,
    /// Required. The notification config being created. The name and the service
    /// account will be ignored as they are both output only fields on this
    /// resource.
    #[prost(message, optional, tag = "3")]
    pub notification_config: ::core::option::Option<NotificationConfig>,
}
/// Request message to create single resource value config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResourceValueConfigRequest {
    /// Required. Resource name of the new ResourceValueConfig's parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The resource value config being created.
    #[prost(message, optional, tag = "2")]
    pub resource_value_config: ::core::option::Option<ResourceValueConfig>,
}
/// Request message for creating a source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSourceRequest {
    /// Required. Resource name of the new source's parent. Its format should be
    /// "organizations/\[organization_id\]".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Source being created, only the display_name and description
    /// will be used. All other fields will be ignored.
    #[prost(message, optional, tag = "2")]
    pub source: ::core::option::Option<Source>,
}
/// Request message for deleting a BigQuery export.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBigQueryExportRequest {
    /// Required. The name of the BigQuery export to delete. The following list
    /// shows some examples of the format:
    ///
    /// +
    /// `organizations/{organization}/locations/{location}/bigQueryExports/{export_id}`
    /// + `folders/{folder}/locations/{location}/bigQueryExports/{export_id}`
    /// + `projects/{project}/locations/{location}/bigQueryExports/{export_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for deleting a mute config. If no location is specified,
/// default is global.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMuteConfigRequest {
    /// Required. Name of the mute config to delete. The following list shows some
    /// examples of the format:
    ///
    /// + `organizations/{organization}/muteConfigs/{config_id}`
    /// +
    /// `organizations/{organization}/locations/{location}/muteConfigs/{config_id}`
    /// + `folders/{folder}/muteConfigs/{config_id}`
    /// + `folders/{folder}/locations/{location}/muteConfigs/{config_id}`
    /// + `projects/{project}/muteConfigs/{config_id}`
    /// + `projects/{project}/locations/{location}/muteConfigs/{config_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for deleting a notification config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNotificationConfigRequest {
    /// Required. Name of the notification config to delete. The following list
    /// shows some examples of the format:
    ///
    /// +
    /// `organizations/\[organization_id\]/locations/\[location_id\]/notificationConfigs/\[config_id\]`
    /// +
    /// `folders/\[folder_id\]/locations/\[location_id\]notificationConfigs/\[config_id\]`
    /// +
    /// `projects/\[project_id\]/locations/\[location_id\]notificationConfigs/\[config_id\]`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message to delete resource value config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResourceValueConfigRequest {
    /// Required. Name of the ResourceValueConfig to delete
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for retrieving a BigQuery export.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBigQueryExportRequest {
    /// Required. Name of the BigQuery export to retrieve. The following list shows
    /// some examples of the format:
    ///
    /// +
    /// `organizations/{organization}/locations/{location}/bigQueryExports/{export_id}`
    /// + `folders/{folder}/locations/{location}/bigQueryExports/{export_id}`
    /// + `projects/{project}locations/{location}//bigQueryExports/{export_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for retrieving a mute config. If no location is specified,
/// default is global.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMuteConfigRequest {
    /// Required. Name of the mute config to retrieve. The following list shows
    /// some examples of the format:
    ///
    /// + `organizations/{organization}/muteConfigs/{config_id}`
    /// +
    /// `organizations/{organization}/locations/{location}/muteConfigs/{config_id}`
    /// + `folders/{folder}/muteConfigs/{config_id}`
    /// + `folders/{folder}/locations/{location}/muteConfigs/{config_id}`
    /// + `projects/{project}/muteConfigs/{config_id}`
    /// + `projects/{project}/locations/{location}/muteConfigs/{config_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for getting a notification config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationConfigRequest {
    /// Required. Name of the notification config to get. The following list shows
    /// some examples of the format:
    ///
    /// +
    /// `organizations/\[organization_id\]/locations/\[location_id\]/notificationConfigs/\[config_id\]`
    /// +
    /// `folders/\[folder_id\]/locations/\[location_id\]/notificationConfigs/\[config_id\]`
    /// +
    /// `projects/\[project_id\]/locations/\[location_id\]/notificationConfigs/\[config_id\]`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message to get resource value config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResourceValueConfigRequest {
    /// Required. Name of the resource value config to retrieve. Its format is
    /// organizations/{organization}/resourceValueConfigs/{config_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for getting a source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSourceRequest {
    /// Required. Relative resource name of the source. Its format is
    /// "organizations/\[organization_id\]/source/\[source_id\]".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for grouping by findings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupFindingsRequest {
    /// Required. Name of the source to groupBy. If no location is specified,
    /// finding is assumed to be in global.
    ///   The following list shows some examples:
    ///
    /// + `organizations/\[organization_id\]/sources/\[source_id\]`
    /// +
    /// `organizations/\[organization_id\]/sources/\[source_id\]/locations/\[location_id\]`
    /// + `folders/\[folder_id\]/sources/\[source_id\]`
    /// + `folders/\[folder_id\]/sources/\[source_id\]/locations/\[location_id\]`
    /// + `projects/\[project_id\]/sources/\[source_id\]`
    /// + `projects/\[project_id\]/sources/\[source_id\]/locations/\[location_id\]`
    ///
    /// To groupBy across all sources provide a source_id of `-`. The following
    /// list shows some examples:
    ///
    /// + `organizations/{organization_id}/sources/-`
    /// + `organizations/{organization_id}/sources/-/locations/\[location_id\]`
    /// + `folders/{folder_id}/sources/-`
    /// + `folders/{folder_id}/sources/-/locations/\[location_id\]`
    /// + `projects/{project_id}/sources/-`
    /// + `projects/{project_id}/sources/-/locations/\[location_id\]`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Expression that defines the filter to apply across findings.
    /// The expression is a list of one or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. Examples include:
    ///
    ///   * name
    ///   * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// The following field and operator combinations are supported:
    ///
    /// * name: `=`
    /// * parent: `=`, `:`
    /// * resource_name: `=`, `:`
    /// * state: `=`, `:`
    /// * category: `=`, `:`
    /// * external_uri: `=`, `:`
    /// * event_time: `=`, `>`, `<`, `>=`, `<=`
    ///
    ///    Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///    Examples:
    ///      `event_time = "2019-06-10T16:07:18-07:00"`
    ///      `event_time = 1560208038000`
    ///
    /// * severity: `=`, `:`
    /// * security_marks.marks: `=`, `:`
    /// * resource:
    ///    * resource.name: `=`, `:`
    ///    * resource.parent_name: `=`, `:`
    ///    * resource.parent_display_name: `=`, `:`
    ///    * resource.project_name: `=`, `:`
    ///    * resource.project_display_name: `=`, `:`
    ///    * resource.type: `=`, `:`
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Required. Expression that defines what assets fields to use for grouping.
    /// The string value should follow SQL syntax: comma separated list of fields.
    /// For example: "parent,resource_name".
    ///
    /// The following fields are supported:
    ///
    /// * resource_name
    /// * category
    /// * state
    /// * parent
    /// * severity
    #[prost(string, tag = "3")]
    pub group_by: ::prost::alloc::string::String,
    /// The value returned by the last `GroupFindingsResponse`; indicates
    /// that this is a continuation of a prior `GroupFindings` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "7")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "8")]
    pub page_size: i32,
}
/// Response message for group by findings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupFindingsResponse {
    /// Group results. There exists an element for each existing unique
    /// combination of property/values. The element contains a count for the number
    /// of times those specific property/values appear.
    #[prost(message, repeated, tag = "1")]
    pub group_by_results: ::prost::alloc::vec::Vec<GroupResult>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of results matching the query.
    #[prost(int32, tag = "4")]
    pub total_size: i32,
}
/// Result containing the properties and count of a groupBy request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupResult {
    /// Properties matching the groupBy fields in the request.
    #[prost(btree_map = "string, message", tag = "1")]
    pub properties: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
    /// Total count of resources for the given properties.
    #[prost(int64, tag = "2")]
    pub count: i64,
}
/// Request message for listing the attack paths for a given simulation or valued
/// resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAttackPathsRequest {
    /// Required. Name of parent to list attack paths.
    ///
    /// Valid formats:
    /// "organizations/{organization}",
    /// "organizations/{organization}/simulations/{simulation}"
    /// "organizations/{organization}/simulations/{simulation}/attackExposureResults/{attack_exposure_result_v2}"
    /// "organizations/{organization}/simulations/{simulation}/valuedResources/{valued_resource}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The filter expression that filters the attack path in the response.
    /// Supported fields:
    ///
    ///    * `valued_resources` supports =
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The value returned by the last `ListAttackPathsResponse`; indicates
    /// that this is a continuation of a prior `ListAttackPaths` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
}
/// Response message for listing the attack paths for a given simulation or
/// valued resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAttackPathsResponse {
    /// The attack paths that the attack path simulation identified.
    #[prost(message, repeated, tag = "1")]
    pub attack_paths: ::prost::alloc::vec::Vec<AttackPath>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for getting simulation.
/// Simulation name can include "latest" to retrieve the latest simulation
/// For example, "organizations/123/simulations/latest"
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSimulationRequest {
    /// Required. The organization name or simulation name of this simulation
    ///
    /// Valid format:
    /// "organizations/{organization}/simulations/latest"
    /// "organizations/{organization}/simulations/{simulation}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for getting a valued resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValuedResourceRequest {
    /// Required. The name of this valued resource
    ///
    /// Valid format:
    /// "organizations/{organization}/simulations/{simulation}/valuedResources/{valued_resource}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for listing BigQuery exports at a given scope e.g.
/// organization, folder or project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBigQueryExportsRequest {
    /// Required. The parent, which owns the collection of BigQuery exports. Its
    /// format is "organizations/\[organization_id\]/locations/\[location_id\]",
    /// "folders/\[folder_id\]/locations/\[location_id\]", or
    /// "projects/\[project_id\]/locations/\[location_id\]".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of configs to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 10 configs will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListBigQueryExports` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListBigQueryExports`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing BigQuery exports.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBigQueryExportsResponse {
    /// The BigQuery exports from the specified parent.
    #[prost(message, repeated, tag = "1")]
    pub big_query_exports: ::prost::alloc::vec::Vec<BigQueryExport>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for listing findings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFindingsRequest {
    /// Required. Name of the source the findings belong to. If no location is
    /// specified, the default is global. The following list shows some examples:
    ///
    /// + `organizations/\[organization_id\]/sources/\[source_id\]`
    /// +
    /// `organizations/\[organization_id\]/sources/\[source_id\]/locations/\[location_id\]`
    /// + `folders/\[folder_id\]/sources/\[source_id\]`
    /// + `folders/\[folder_id\]/sources/\[source_id\]/locations/\[location_id\]`
    /// + `projects/\[project_id\]/sources/\[source_id\]`
    /// + `projects/\[project_id\]/sources/\[source_id\]/locations/\[location_id\]`
    ///
    /// To list across all sources provide a source_id of `-`. The following
    /// list shows some examples:
    ///
    /// + `organizations/{organization_id}/sources/-`
    /// + `organizations/{organization_id}/sources/-/locations/{location_id}`
    /// + `folders/{folder_id}/sources/-`
    /// + `folders/{folder_id}/sources/-locations/{location_id}`
    /// + `projects/{projects_id}/sources/-`
    /// + `projects/{projects_id}/sources/-/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Expression that defines the filter to apply across findings.
    /// The expression is a list of one or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. Examples include:
    ///
    ///   * name
    ///   * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// The following field and operator combinations are supported:
    ///
    /// * name: `=`
    /// * parent: `=`, `:`
    /// * resource_name: `=`, `:`
    /// * state: `=`, `:`
    /// * category: `=`, `:`
    /// * external_uri: `=`, `:`
    /// * event_time: `=`, `>`, `<`, `>=`, `<=`
    ///
    ///    Usage: This should be milliseconds since epoch or an RFC3339 string.
    ///    Examples:
    ///      `event_time = "2019-06-10T16:07:18-07:00"`
    ///      `event_time = 1560208038000`
    ///
    /// * severity: `=`, `:`
    /// * security_marks.marks: `=`, `:`
    /// * resource:
    ///    * resource.name: `=`, `:`
    ///    * resource.parent_name: `=`, `:`
    ///    * resource.parent_display_name: `=`, `:`
    ///    * resource.project_name: `=`, `:`
    ///    * resource.project_display_name: `=`, `:`
    ///    * resource.type: `=`, `:`
    ///    * resource.folders.resource_folder: `=`, `:`
    ///    * resource.display_name: `=`, `:`
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Expression that defines what fields and order to use for sorting. The
    /// string value should follow SQL syntax: comma separated list of fields. For
    /// example: "name,parent". The default sorting order
    /// is ascending. To specify descending order for a field, a suffix " desc"
    /// should be appended to the field name. For example: "name
    /// desc,parent". Redundant space characters in the
    /// syntax are insignificant. "name desc,parent" and "
    /// name     desc  ,   parent  " are equivalent.
    ///
    /// The following fields are supported:
    /// name
    /// parent
    /// state
    /// category
    /// resource_name
    /// event_time
    /// security_marks.marks
    #[prost(string, tag = "3")]
    pub order_by: ::prost::alloc::string::String,
    /// A field mask to specify the Finding fields to be listed in the response.
    /// An empty field mask will list all fields.
    #[prost(message, optional, tag = "7")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The value returned by the last `ListFindingsResponse`; indicates
    /// that this is a continuation of a prior `ListFindings` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "8")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "9")]
    pub page_size: i32,
}
/// Response message for listing findings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFindingsResponse {
    /// Findings matching the list request.
    #[prost(message, repeated, tag = "1")]
    pub list_findings_results: ::prost::alloc::vec::Vec<
        list_findings_response::ListFindingsResult,
    >,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of findings matching the query.
    #[prost(int32, tag = "4")]
    pub total_size: i32,
}
/// Nested message and enum types in `ListFindingsResponse`.
pub mod list_findings_response {
    /// Result containing the Finding.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListFindingsResult {
        /// Finding matching the search request.
        #[prost(message, optional, tag = "1")]
        pub finding: ::core::option::Option<super::Finding>,
        /// Output only. Resource that is associated with this finding.
        #[prost(message, optional, tag = "3")]
        pub resource: ::core::option::Option<list_findings_result::Resource>,
    }
    /// Nested message and enum types in `ListFindingsResult`.
    pub mod list_findings_result {
        /// Information related to the Google Cloud resource that is
        /// associated with this finding.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Resource {
            /// The full resource name of the resource. See:
            /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            /// The human readable name of the resource.
            #[prost(string, tag = "2")]
            pub display_name: ::prost::alloc::string::String,
            /// The full resource type of the resource.
            #[prost(string, tag = "3")]
            pub r#type: ::prost::alloc::string::String,
        }
    }
}
/// Request message for listing  mute configs at a given scope e.g. organization,
/// folder or project. If no location is specified, default is
/// global.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMuteConfigsRequest {
    /// Required. The parent, which owns the collection of mute configs. Its format
    /// is "organizations/\[organization_id\]", "folders/\[folder_id\]",
    /// "projects/\[project_id\]",
    /// "organizations/\[organization_id\]/locations/\[location_id\]",
    /// "folders/\[folder_id\]/locations/\[location_id\]",
    /// "projects/\[project_id\]/locations/\[location_id\]".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of configs to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 10 configs will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListMuteConfigs` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListMuteConfigs` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing mute configs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMuteConfigsResponse {
    /// The mute configs from the specified parent.
    #[prost(message, repeated, tag = "1")]
    pub mute_configs: ::prost::alloc::vec::Vec<MuteConfig>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for listing notification configs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationConfigsRequest {
    /// Required. The name of the parent in which to list the notification
    /// configurations. Its format is
    /// "organizations/\[organization_id\]/locations/\[location_id\]",
    /// "folders/\[folder_id\]/locations/\[location_id\]", or
    /// "projects/\[project_id\]/locations/\[location_id\]".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The value returned by the last `ListNotificationConfigsResponse`; indicates
    /// that this is a continuation of a prior `ListNotificationConfigs` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for listing notification configs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationConfigsResponse {
    /// Notification configs belonging to the requested parent.
    #[prost(message, repeated, tag = "1")]
    pub notification_configs: ::prost::alloc::vec::Vec<NotificationConfig>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message to list resource value configs of a parent
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResourceValueConfigsRequest {
    /// Required. The parent, which owns the collection of resource value configs.
    /// Its format is
    /// "organizations/\[organization_id\]"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of configs to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 10 configs will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListResourceValueConfigs` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListResourceValueConfigs` must match the call that provided the
    /// page token.
    ///
    /// page_size can be specified, and the new page_size will be used.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message to list resource value configs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResourceValueConfigsResponse {
    /// The resource value configs from the specified parent.
    #[prost(message, repeated, tag = "1")]
    pub resource_value_configs: ::prost::alloc::vec::Vec<ResourceValueConfig>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is empty, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for listing sources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSourcesRequest {
    /// Required. Resource name of the parent of sources to list. Its format should
    /// be "organizations/\[organization_id\]", "folders/\[folder_id\]", or
    /// "projects/\[project_id\]".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The value returned by the last `ListSourcesResponse`; indicates
    /// that this is a continuation of a prior `ListSources` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "7")]
    pub page_size: i32,
}
/// Response message for listing sources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSourcesResponse {
    /// Sources belonging to the requested parent.
    #[prost(message, repeated, tag = "1")]
    pub sources: ::prost::alloc::vec::Vec<Source>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for listing the valued resources for a given simulation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListValuedResourcesRequest {
    /// Required. Name of parent to list exposed resources.
    ///
    /// Valid formats:
    /// "organizations/{organization}",
    /// "organizations/{organization}/simulations/{simulation}"
    /// "organizations/{organization}/simulations/{simulation}/attackExposureResults/{attack_exposure_result_v2}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The filter expression that filters the valued resources in the response.
    /// Supported fields:
    ///
    ///    * `resource_value` supports =
    ///    * `resource_type` supports =
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The value returned by the last `ListValuedResourcesResponse`; indicates
    /// that this is a continuation of a prior `ListValuedResources` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. The fields by which to order the valued resources response.
    ///
    /// Supported fields:
    ///
    ///    * `exposed_score`
    ///
    ///    * `resource_value`
    ///
    ///    * `resource_type`
    ///
    /// Values should be a comma separated list of fields. For example:
    /// `exposed_score,resource_value`.
    ///
    /// The default sorting order is descending. To specify ascending or descending
    /// order for a field, append a " ASC" or a " DESC" suffix, respectively; for
    /// example: `exposed_score DESC`.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for listing the valued resources for a given simulation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListValuedResourcesResponse {
    /// The valued resources that the attack path simulation identified.
    #[prost(message, repeated, tag = "1")]
    pub valued_resources: ::prost::alloc::vec::Vec<ValuedResource>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The estimated total number of results matching the query.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for updating a finding's state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFindingStateRequest {
    /// Required. The [relative resource
    /// name](<https://cloud.google.com/apis/design/resource_names#relative_resource_name>)
    /// of the finding. If no location is specified, finding is assumed to be in
    /// global. The following list shows some examples:
    ///
    /// +
    /// `organizations/{organization_id}/sources/{source_id}/findings/{finding_id}`
    /// +
    /// `organizations/{organization_id}/sources/{source_id}/locations/{location_id}/findings/{finding_id}`
    /// + `folders/{folder_id}/sources/{source_id}/findings/{finding_id}`
    /// +
    /// `folders/{folder_id}/sources/{source_id}/locations/{location_id}/findings/{finding_id}`
    /// + `projects/{project_id}/sources/{source_id}/findings/{finding_id}`
    /// +
    /// `projects/{project_id}/sources/{source_id}/locations/{location_id}/findings/{finding_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The desired State of the finding.
    #[prost(enumeration = "finding::State", tag = "2")]
    pub state: i32,
}
/// Request message for updating a finding's mute status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMuteRequest {
    /// Required. The [relative resource
    /// name](<https://cloud.google.com/apis/design/resource_names#relative_resource_name>)
    /// of the finding. If no location is specified, finding is assumed to be in
    /// global. The following list shows some examples:
    ///
    /// +
    /// `organizations/{organization_id}/sources/{source_id}/findings/{finding_id}`
    /// +
    /// `organizations/{organization_id}/sources/{source_id}/locations/{location_id}/findings/{finding_id}`
    /// + `folders/{folder_id}/sources/{source_id}/findings/{finding_id}`
    /// +
    /// `folders/{folder_id}/sources/{source_id}/locations/{location_id}/findings/{finding_id}`
    /// + `projects/{project_id}/sources/{source_id}/findings/{finding_id}`
    /// +
    /// `projects/{project_id}/sources/{source_id}/locations/{location_id}/findings/{finding_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The desired state of the Mute.
    #[prost(enumeration = "finding::Mute", tag = "2")]
    pub mute: i32,
}
/// Request message for updating a BigQuery export.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBigQueryExportRequest {
    /// Required. The BigQuery export being updated.
    #[prost(message, optional, tag = "1")]
    pub big_query_export: ::core::option::Option<BigQueryExport>,
    /// The list of fields to be updated.
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a ExternalSystem resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateExternalSystemRequest {
    /// Required. The external system resource to update.
    #[prost(message, optional, tag = "1")]
    pub external_system: ::core::option::Option<ExternalSystem>,
    /// The FieldMask to use when updating the external system resource.
    ///
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating or creating a finding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFindingRequest {
    /// Required. The finding resource to update or create if it does not already
    /// exist. parent, security_marks, and update_time will be ignored.
    ///
    /// In the case of creation, the finding id portion of the name must be
    /// alphanumeric and less than or equal to 32 characters and greater than 0
    /// characters in length.
    #[prost(message, optional, tag = "1")]
    pub finding: ::core::option::Option<Finding>,
    /// The FieldMask to use when updating the finding resource. This field should
    /// not be specified when creating a finding.
    ///
    /// When updating a finding, an empty mask is treated as updating all mutable
    /// fields and replacing source_properties.  Individual source_properties can
    /// be added/updated by using "source_properties.<property key>" in the field
    /// mask.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a mute config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMuteConfigRequest {
    /// Required. The mute config being updated.
    #[prost(message, optional, tag = "1")]
    pub mute_config: ::core::option::Option<MuteConfig>,
    /// The list of fields to be updated.
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a notification config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNotificationConfigRequest {
    /// Required. The notification config to update.
    #[prost(message, optional, tag = "1")]
    pub notification_config: ::core::option::Option<NotificationConfig>,
    /// The FieldMask to use when updating the notification config.
    ///
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message to update resource value config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResourceValueConfigRequest {
    /// Required. The resource value config being updated.
    #[prost(message, optional, tag = "1")]
    pub resource_value_config: ::core::option::Option<ResourceValueConfig>,
    /// The list of fields to be updated.
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a SecurityMarks resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSecurityMarksRequest {
    /// Required. The security marks resource to update.
    #[prost(message, optional, tag = "1")]
    pub security_marks: ::core::option::Option<SecurityMarks>,
    /// The FieldMask to use when updating the security marks resource.
    ///
    /// The field mask must not contain duplicate fields.
    /// If empty or set to "marks", all marks will be replaced.  Individual
    /// marks can be updated using "marks.<mark_key>".
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSourceRequest {
    /// Required. The source resource to update.
    #[prost(message, optional, tag = "1")]
    pub source: ::core::option::Option<Source>,
    /// The FieldMask to use when updating the source resource.
    ///
    /// If empty all mutable fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Generated client implementations.
pub mod security_center_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// V2 APIs for Security Center service.
    #[derive(Debug, Clone)]
    pub struct SecurityCenterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SecurityCenterClient<T>
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
        ) -> SecurityCenterClient<InterceptedService<T, F>>
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
            SecurityCenterClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a ResourceValueConfig for an organization. Maps user's tags to
        /// difference resource values for use by the attack path simulation.
        pub async fn batch_create_resource_value_configs(
            &mut self,
            request: impl tonic::IntoRequest<
                super::BatchCreateResourceValueConfigsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateResourceValueConfigsResponse>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/BatchCreateResourceValueConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "BatchCreateResourceValueConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Kicks off an LRO to bulk mute findings for a parent based on a filter. If
        /// no location is specified, findings are muted in global. The parent
        /// can be either an organization, folder, or project. The findings matched by
        /// the filter will be muted after the LRO is done.
        pub async fn bulk_mute_findings(
            &mut self,
            request: impl tonic::IntoRequest<super::BulkMuteFindingsRequest>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/BulkMuteFindings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "BulkMuteFindings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a BigQuery export.
        pub async fn create_big_query_export(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBigQueryExportRequest>,
        ) -> std::result::Result<tonic::Response<super::BigQueryExport>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/CreateBigQueryExport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "CreateBigQueryExport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a finding in a location. The corresponding source must exist for
        /// finding creation to succeed.
        pub async fn create_finding(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFindingRequest>,
        ) -> std::result::Result<tonic::Response<super::Finding>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/CreateFinding",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "CreateFinding",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a mute config.
        pub async fn create_mute_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMuteConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::MuteConfig>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/CreateMuteConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "CreateMuteConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a notification config.
        pub async fn create_notification_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNotificationConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NotificationConfig>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/CreateNotificationConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "CreateNotificationConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a source.
        pub async fn create_source(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSourceRequest>,
        ) -> std::result::Result<tonic::Response<super::Source>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/CreateSource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "CreateSource",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an existing BigQuery export.
        pub async fn delete_big_query_export(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBigQueryExportRequest>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/DeleteBigQueryExport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "DeleteBigQueryExport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an existing mute config. If no location is specified, default is
        /// global.
        pub async fn delete_mute_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMuteConfigRequest>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/DeleteMuteConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "DeleteMuteConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a notification config.
        pub async fn delete_notification_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNotificationConfigRequest>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/DeleteNotificationConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "DeleteNotificationConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a ResourceValueConfig.
        pub async fn delete_resource_value_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteResourceValueConfigRequest>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/DeleteResourceValueConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "DeleteResourceValueConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a BigQuery export.
        pub async fn get_big_query_export(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBigQueryExportRequest>,
        ) -> std::result::Result<tonic::Response<super::BigQueryExport>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/GetBigQueryExport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "GetBigQueryExport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get the simulation by name or the latest simulation for the given
        /// organization.
        pub async fn get_simulation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSimulationRequest>,
        ) -> std::result::Result<tonic::Response<super::Simulation>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/GetSimulation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "GetSimulation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get the valued resource by name
        pub async fn get_valued_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::GetValuedResourceRequest>,
        ) -> std::result::Result<tonic::Response<super::ValuedResource>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/GetValuedResource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "GetValuedResource",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the access control policy on the specified Source.
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
                "/google.cloud.securitycenter.v2.SecurityCenter/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a mute config. If no location is specified, default is
        /// global.
        pub async fn get_mute_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMuteConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::MuteConfig>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/GetMuteConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "GetMuteConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a notification config.
        pub async fn get_notification_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotificationConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NotificationConfig>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/GetNotificationConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "GetNotificationConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a ResourceValueConfig.
        pub async fn get_resource_value_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetResourceValueConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResourceValueConfig>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/GetResourceValueConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "GetResourceValueConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a source.
        pub async fn get_source(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSourceRequest>,
        ) -> std::result::Result<tonic::Response<super::Source>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/GetSource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "GetSource",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Filters an organization or source's findings and groups them by their
        /// specified properties in a location. If no location is specified, findings
        /// are assumed to be in global
        ///
        /// To group across all sources provide a `-` as the source id.
        /// The following list shows some examples:
        ///
        /// + `/v2/organizations/{organization_id}/sources/-/findings`
        /// +
        /// `/v2/organizations/{organization_id}/sources/-/locations/{location_id}/findings`
        /// + `/v2/folders/{folder_id}/sources/-/findings`
        /// + `/v2/folders/{folder_id}/sources/-/locations/{location_id}/findings`
        /// + `/v2/projects/{project_id}/sources/-/findings`
        /// + `/v2/projects/{project_id}/sources/-/locations/{location_id}/findings`
        pub async fn group_findings(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupFindingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GroupFindingsResponse>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/GroupFindings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "GroupFindings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the attack paths for a set of simulation results or valued resources
        /// and filter.
        pub async fn list_attack_paths(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAttackPathsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAttackPathsResponse>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/ListAttackPaths",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "ListAttackPaths",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists BigQuery exports. Note that when requesting BigQuery exports at a
        /// given level all exports under that level are also returned e.g. if
        /// requesting BigQuery exports under a folder, then all BigQuery exports
        /// immediately under the folder plus the ones created under the projects
        /// within the folder are returned.
        pub async fn list_big_query_exports(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBigQueryExportsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBigQueryExportsResponse>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/ListBigQueryExports",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "ListBigQueryExports",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists an organization or source's findings.
        ///
        /// To list across all sources for a given location provide a `-` as the source
        /// id. If no location is specified, finding are assumed to be in global.
        /// The following list shows some examples:
        ///
        /// + `/v2/organizations/{organization_id}/sources/-/findings`
        /// +
        /// `/v2/organizations/{organization_id}/sources/-/locations/{location_id}/findings`
        pub async fn list_findings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFindingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFindingsResponse>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/ListFindings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "ListFindings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists mute configs. If no location is specified, default is
        /// global.
        pub async fn list_mute_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMuteConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMuteConfigsResponse>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/ListMuteConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "ListMuteConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists notification configs.
        pub async fn list_notification_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotificationConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNotificationConfigsResponse>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/ListNotificationConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "ListNotificationConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all ResourceValueConfigs.
        pub async fn list_resource_value_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListResourceValueConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListResourceValueConfigsResponse>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/ListResourceValueConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "ListResourceValueConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all sources belonging to an organization.
        pub async fn list_sources(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSourcesResponse>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/ListSources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "ListSources",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the valued resources for a set of simulation results and filter.
        pub async fn list_valued_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::ListValuedResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListValuedResourcesResponse>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/ListValuedResources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "ListValuedResources",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the state of a finding. If no location is specified, finding is
        /// assumed to be in global
        pub async fn set_finding_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SetFindingStateRequest>,
        ) -> std::result::Result<tonic::Response<super::Finding>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/SetFindingState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "SetFindingState",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the access control policy on the specified Source.
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
                "/google.cloud.securitycenter.v2.SecurityCenter/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the mute state of a finding. If no location is specified, finding
        /// is assumed to be in global
        pub async fn set_mute(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMuteRequest>,
        ) -> std::result::Result<tonic::Response<super::Finding>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/SetMute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "SetMute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the permissions that a caller has on the specified source.
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
                "/google.cloud.securitycenter.v2.SecurityCenter/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a BigQuery export.
        pub async fn update_big_query_export(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBigQueryExportRequest>,
        ) -> std::result::Result<tonic::Response<super::BigQueryExport>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/UpdateBigQueryExport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "UpdateBigQueryExport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates external system. This is for a given finding. If no location is
        /// specified, finding is assumed to be in global
        pub async fn update_external_system(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateExternalSystemRequest>,
        ) -> std::result::Result<tonic::Response<super::ExternalSystem>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/UpdateExternalSystem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "UpdateExternalSystem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates or updates a finding. If no location is specified, finding is
        /// assumed to be in global. The corresponding source must exist for a finding
        /// creation to succeed.
        pub async fn update_finding(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFindingRequest>,
        ) -> std::result::Result<tonic::Response<super::Finding>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/UpdateFinding",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "UpdateFinding",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a mute config. If no location is specified, default is
        /// global.
        pub async fn update_mute_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMuteConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::MuteConfig>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/UpdateMuteConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "UpdateMuteConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a notification config. The following update
        /// fields are allowed: description, pubsub_topic, streaming_config.filter
        pub async fn update_notification_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNotificationConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NotificationConfig>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/UpdateNotificationConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "UpdateNotificationConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing ResourceValueConfigs with new rules.
        pub async fn update_resource_value_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateResourceValueConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResourceValueConfig>,
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
                "/google.cloud.securitycenter.v2.SecurityCenter/UpdateResourceValueConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "UpdateResourceValueConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates security marks. For Finding Security marks, if no location is
        /// specified, finding is assumed to be in global. Assets Security Marks can
        /// only be accessed through global endpoint.
        pub async fn update_security_marks(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSecurityMarksRequest>,
        ) -> std::result::Result<tonic::Response<super::SecurityMarks>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/UpdateSecurityMarks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "UpdateSecurityMarks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a source.
        pub async fn update_source(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSourceRequest>,
        ) -> std::result::Result<tonic::Response<super::Source>, tonic::Status> {
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
                "/google.cloud.securitycenter.v2.SecurityCenter/UpdateSource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycenter.v2.SecurityCenter",
                        "UpdateSource",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
