/// Request for
/// [TroubleshootIamPolicy][google.cloud.policytroubleshooter.iam.v3beta.PolicyTroubleshooter.TroubleshootIamPolicy].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TroubleshootIamPolicyRequest {
    /// The information to use for checking whether a principal has a permission
    /// for a resource.
    #[prost(message, optional, tag = "1")]
    pub access_tuple: ::core::option::Option<AccessTuple>,
}
/// Response for
/// [TroubleshootIamPolicy][google.cloud.policytroubleshooter.iam.v3beta.PolicyTroubleshooter.TroubleshootIamPolicy].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TroubleshootIamPolicyResponse {
    /// Indicates whether the principal has the specified permission for the
    /// specified resource, based on evaluating all types of the applicable IAM
    /// policies.
    #[prost(
        enumeration = "troubleshoot_iam_policy_response::OverallAccessState",
        tag = "1"
    )]
    pub overall_access_state: i32,
    /// The access tuple from the request, including any provided context used to
    /// evaluate the condition.
    #[prost(message, optional, tag = "2")]
    pub access_tuple: ::core::option::Option<AccessTuple>,
    /// An explanation of how the applicable IAM allow policies affect the final
    /// access state.
    #[prost(message, optional, tag = "3")]
    pub allow_policy_explanation: ::core::option::Option<AllowPolicyExplanation>,
    /// An explanation of how the applicable IAM deny policies affect the final
    /// access state.
    #[prost(message, optional, tag = "4")]
    pub deny_policy_explanation: ::core::option::Option<DenyPolicyExplanation>,
}
/// Nested message and enum types in `TroubleshootIamPolicyResponse`.
pub mod troubleshoot_iam_policy_response {
    /// Whether the principal has the permission on the resource.
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
    pub enum OverallAccessState {
        /// Not specified.
        Unspecified = 0,
        /// The principal has the permission.
        CanAccess = 1,
        /// The principal doesn't have the permission.
        CannotAccess = 2,
        /// The principal might have the permission, but the sender can't access all
        /// of the information needed to fully evaluate the principal's access.
        UnknownInfo = 3,
        /// The principal might have the permission, but Policy Troubleshooter can't
        /// fully evaluate the principal's access because the sender didn't provide
        /// the required context to evaluate the condition.
        UnknownConditional = 4,
    }
    impl OverallAccessState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OverallAccessState::Unspecified => "OVERALL_ACCESS_STATE_UNSPECIFIED",
                OverallAccessState::CanAccess => "CAN_ACCESS",
                OverallAccessState::CannotAccess => "CANNOT_ACCESS",
                OverallAccessState::UnknownInfo => "UNKNOWN_INFO",
                OverallAccessState::UnknownConditional => "UNKNOWN_CONDITIONAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OVERALL_ACCESS_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CAN_ACCESS" => Some(Self::CanAccess),
                "CANNOT_ACCESS" => Some(Self::CannotAccess),
                "UNKNOWN_INFO" => Some(Self::UnknownInfo),
                "UNKNOWN_CONDITIONAL" => Some(Self::UnknownConditional),
                _ => None,
            }
        }
    }
}
/// Information about the principal, resource, and permission to check.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessTuple {
    /// Required. The email address of the principal whose access you want to
    /// check. For example, `alice@example.com` or
    /// `my-service-account@my-project.iam.gserviceaccount.com`.
    ///
    /// The principal must be a Google Account or a service account. Other types of
    /// principals are not supported.
    #[prost(string, tag = "1")]
    pub principal: ::prost::alloc::string::String,
    /// Required. The full resource name that identifies the resource. For example,
    /// `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`.
    ///
    /// For examples of full resource names for Google Cloud services, see
    /// <https://cloud.google.com/iam/help/troubleshooter/full-resource-names.>
    #[prost(string, tag = "2")]
    pub full_resource_name: ::prost::alloc::string::String,
    /// Required. The IAM permission to check for, either in the `v1` permission
    /// format or the `v2` permission format.
    ///
    /// For a complete list of IAM permissions in the `v1` format, see
    /// <https://cloud.google.com/iam/help/permissions/reference.>
    ///
    /// For a list of IAM permissions in the `v2` format, see
    /// <https://cloud.google.com/iam/help/deny/supported-permissions.>
    ///
    /// For a complete list of predefined IAM roles and the permissions in each
    /// role, see <https://cloud.google.com/iam/help/roles/reference.>
    #[prost(string, tag = "3")]
    pub permission: ::prost::alloc::string::String,
    /// Output only. The permission that Policy Troubleshooter checked for, in
    /// the `v2` format.
    #[prost(string, tag = "4")]
    pub permission_fqdn: ::prost::alloc::string::String,
    /// Optional. Additional context for the request, such as the request time or
    /// IP address. This context allows Policy Troubleshooter to troubleshoot
    /// conditional role bindings and deny rules.
    #[prost(message, optional, tag = "5")]
    pub condition_context: ::core::option::Option<ConditionContext>,
}
/// Additional context for troubleshooting conditional role bindings and deny
/// rules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionContext {
    /// Represents a target resource that is involved with a network activity.
    /// If multiple resources are involved with an activity, this must be the
    /// primary one.
    #[prost(message, optional, tag = "1")]
    pub resource: ::core::option::Option<condition_context::Resource>,
    /// The destination of a network activity, such as accepting a TCP connection.
    /// In a multi-hop network activity, the destination represents the receiver of
    /// the last hop.
    #[prost(message, optional, tag = "2")]
    pub destination: ::core::option::Option<condition_context::Peer>,
    /// Represents a network request, such as an HTTP request.
    #[prost(message, optional, tag = "3")]
    pub request: ::core::option::Option<condition_context::Request>,
    /// Output only. The effective tags on the resource. The effective tags are
    /// fetched during troubleshooting.
    #[prost(message, repeated, tag = "4")]
    pub effective_tags: ::prost::alloc::vec::Vec<condition_context::EffectiveTag>,
}
/// Nested message and enum types in `ConditionContext`.
pub mod condition_context {
    /// Core attributes for a resource. A resource is an
    /// addressable (named) entity provided by the destination service. For
    /// example, a Compute Engine instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        /// The name of the service that this resource belongs to, such as
        /// `compute.googleapis.com`. The service name might not match the DNS
        /// hostname that actually serves the request.
        ///
        /// For a full list of resource service values, see
        /// <https://cloud.google.com/iam/help/conditions/resource-services>
        #[prost(string, tag = "1")]
        pub service: ::prost::alloc::string::String,
        /// The stable identifier (name) of a resource on the `service`. A resource
        /// can be logically identified as `//{resource.service}/{resource.name}`.
        /// Unlike the resource URI, the resource name doesn't contain any protocol
        /// and version information.
        ///
        /// For a list of full resource name formats, see
        /// <https://cloud.google.com/iam/help/troubleshooter/full-resource-names>
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        /// The type of the resource, in the format `{service}/{kind}`.
        ///
        /// For a full list of resource type values, see
        /// <https://cloud.google.com/iam/help/conditions/resource-types>
        #[prost(string, tag = "3")]
        pub r#type: ::prost::alloc::string::String,
    }
    /// This message defines attributes for a node that handles a network request.
    /// The node can be either a service or an application that sends, forwards,
    /// or receives the request. Service peers should fill in
    /// `principal` and `labels` as appropriate.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Peer {
        /// The IPv4 or IPv6 address of the peer.
        #[prost(string, tag = "1")]
        pub ip: ::prost::alloc::string::String,
        /// The network port of the peer.
        #[prost(int64, tag = "2")]
        pub port: i64,
    }
    /// This message defines attributes for an HTTP request. If the actual
    /// request is not an HTTP request, the runtime system should try to map
    /// the actual request to an equivalent HTTP request.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        /// Optional. The timestamp when the destination service receives the first
        /// byte of the request.
        #[prost(message, optional, tag = "1")]
        pub receive_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// A tag that applies to a resource during policy evaluation. Tags can be
    /// either directly bound to a resource or inherited from its ancestor.
    /// `EffectiveTag` contains the `name` and `namespaced_name` of the tag value
    /// and tag key, with additional fields of `inherited` to indicate the
    /// inheritance status of the effective tag.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EffectiveTag {
        /// Output only. Resource name for TagValue in the format `tagValues/456`.
        #[prost(string, tag = "1")]
        pub tag_value: ::prost::alloc::string::String,
        /// Output only. The namespaced name of the TagValue. Can be in the form
        /// `{organization_id}/{tag_key_short_name}/{tag_value_short_name}` or
        /// `{project_id}/{tag_key_short_name}/{tag_value_short_name}` or
        /// `{project_number}/{tag_key_short_name}/{tag_value_short_name}`.
        #[prost(string, tag = "2")]
        pub namespaced_tag_value: ::prost::alloc::string::String,
        /// Output only. The name of the TagKey, in the format `tagKeys/{id}`, such
        /// as `tagKeys/123`.
        #[prost(string, tag = "3")]
        pub tag_key: ::prost::alloc::string::String,
        /// Output only. The namespaced name of the TagKey. Can be in the form
        /// `{organization_id}/{tag_key_short_name}` or
        /// `{project_id}/{tag_key_short_name}` or
        /// `{project_number}/{tag_key_short_name}`.
        #[prost(string, tag = "4")]
        pub namespaced_tag_key: ::prost::alloc::string::String,
        /// The parent name of the tag key.
        /// Must be in the format `organizations/{organization_id}` or
        /// `projects/{project_number}`
        #[prost(string, tag = "6")]
        pub tag_key_parent_name: ::prost::alloc::string::String,
        /// Output only. Indicates the inheritance status of a tag value
        /// attached to the given resource. If the tag value is inherited from one of
        /// the resource's ancestors, inherited will be true. If false, then the tag
        /// value is directly attached to the resource, inherited will be false.
        #[prost(bool, tag = "5")]
        pub inherited: bool,
    }
}
/// Details about how the relevant IAM allow policies affect the final access
/// state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowPolicyExplanation {
    /// Indicates whether the principal has the specified permission for the
    /// specified resource, based on evaluating all applicable IAM allow policies.
    #[prost(enumeration = "AllowAccessState", tag = "1")]
    pub allow_access_state: i32,
    /// List of IAM allow policies that were evaluated to check the principal's
    /// permissions, with annotations to indicate how each policy contributed to
    /// the final result.
    ///
    /// The list of policies includes the policy for the resource itself, as well
    /// as allow policies that are inherited from higher levels of the resource
    /// hierarchy, including the organization, the folder, and the project.
    ///
    /// To learn more about the resource hierarchy, see
    /// <https://cloud.google.com/iam/help/resource-hierarchy.>
    #[prost(message, repeated, tag = "2")]
    pub explained_policies: ::prost::alloc::vec::Vec<ExplainedAllowPolicy>,
    /// The relevance of the allow policy type to the overall access state.
    #[prost(enumeration = "HeuristicRelevance", tag = "3")]
    pub relevance: i32,
}
/// Details about how a specific IAM allow policy contributed to the final access
/// state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplainedAllowPolicy {
    /// Required. Indicates whether _this policy_ provides the specified permission
    /// to the specified principal for the specified resource.
    ///
    /// This field does _not_ indicate whether the principal actually has the
    /// permission for the resource. There might be another policy that overrides
    /// this policy. To determine whether the principal actually has the
    /// permission, use the `overall_access_state` field in the
    /// [TroubleshootIamPolicyResponse][google.cloud.policytroubleshooter.iam.v3beta.TroubleshootIamPolicyResponse].
    #[prost(enumeration = "AllowAccessState", tag = "1")]
    pub allow_access_state: i32,
    /// The full resource name that identifies the resource. For example,
    /// `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`.
    ///
    /// If the sender of the request does not have access to the policy, this field
    /// is omitted.
    ///
    /// For examples of full resource names for Google Cloud services, see
    /// <https://cloud.google.com/iam/help/troubleshooter/full-resource-names.>
    #[prost(string, tag = "2")]
    pub full_resource_name: ::prost::alloc::string::String,
    /// Details about how each role binding in the policy affects the principal's
    /// ability, or inability, to use the permission for the resource. The order of
    /// the role bindings matches the role binding order in the policy.
    ///
    /// If the sender of the request does not have access to the policy, this field
    /// is omitted.
    #[prost(message, repeated, tag = "3")]
    pub binding_explanations: ::prost::alloc::vec::Vec<AllowBindingExplanation>,
    /// The relevance of this policy to the overall access state in the
    /// [TroubleshootIamPolicyResponse][google.cloud.policytroubleshooter.iam.v3beta.TroubleshootIamPolicyResponse].
    ///
    /// If the sender of the request does not have access to the policy, this field
    /// is omitted.
    #[prost(enumeration = "HeuristicRelevance", tag = "4")]
    pub relevance: i32,
    /// The IAM allow policy attached to the resource.
    ///
    /// If the sender of the request does not have access to the policy, this field
    /// is empty.
    #[prost(message, optional, tag = "5")]
    pub policy: ::core::option::Option<super::super::super::super::iam::v1::Policy>,
}
/// Details about how a role binding in an allow policy affects a principal's
/// ability to use a permission.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowBindingExplanation {
    /// Required. Indicates whether _this role binding_ gives the specified
    /// permission to the specified principal on the specified resource.
    ///
    /// This field does _not_ indicate whether the principal actually has the
    /// permission on the resource. There might be another role binding that
    /// overrides this role binding. To determine whether the principal actually
    /// has the permission, use the `overall_access_state` field in the
    /// [TroubleshootIamPolicyResponse][google.cloud.policytroubleshooter.iam.v3beta.TroubleshootIamPolicyResponse].
    #[prost(enumeration = "AllowAccessState", tag = "1")]
    pub allow_access_state: i32,
    /// The role that this role binding grants. For example,
    /// `roles/compute.admin`.
    ///
    /// For a complete list of predefined IAM roles, as well as the permissions in
    /// each role, see <https://cloud.google.com/iam/help/roles/reference.>
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
    /// Indicates whether the role granted by this role binding contains the
    /// specified permission.
    #[prost(enumeration = "RolePermissionInclusionState", tag = "3")]
    pub role_permission: i32,
    /// The relevance of the permission's existence, or nonexistence, in the role
    /// to the overall determination for the entire policy.
    #[prost(enumeration = "HeuristicRelevance", tag = "4")]
    pub role_permission_relevance: i32,
    /// The combined result of all memberships. Indicates if the principal is
    /// included in any role binding, either directly or indirectly.
    #[prost(message, optional, tag = "5")]
    pub combined_membership: ::core::option::Option<
        allow_binding_explanation::AnnotatedAllowMembership,
    >,
    /// Indicates whether each role binding includes the principal specified in the
    /// request, either directly or indirectly. Each key identifies a principal in
    /// the role binding, and each value indicates whether the principal in the
    /// role binding includes the principal in the request.
    ///
    /// For example, suppose that a role binding includes the following principals:
    ///
    /// * `user:alice@example.com`
    /// * `group:product-eng@example.com`
    ///
    /// You want to troubleshoot access for `user:bob@example.com`. This user is a
    /// member of the group `group:product-eng@example.com`.
    ///
    /// For the first principal in the role binding, the key is
    /// `user:alice@example.com`, and the `membership` field in the value is set to
    /// `NOT_INCLUDED`.
    ///
    /// For the second principal in the role binding, the key is
    /// `group:product-eng@example.com`, and the `membership` field in the value is
    /// set to `INCLUDED`.
    #[prost(btree_map = "string, message", tag = "6")]
    pub memberships: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        allow_binding_explanation::AnnotatedAllowMembership,
    >,
    /// The relevance of this role binding to the overall determination for the
    /// entire policy.
    #[prost(enumeration = "HeuristicRelevance", tag = "7")]
    pub relevance: i32,
    /// A condition expression that specifies when the role binding grants access.
    ///
    /// To learn about IAM Conditions, see
    /// <https://cloud.google.com/iam/help/conditions/overview.>
    #[prost(message, optional, tag = "8")]
    pub condition: ::core::option::Option<super::super::super::super::r#type::Expr>,
    /// Condition evaluation state for this role binding.
    #[prost(message, optional, tag = "9")]
    pub condition_explanation: ::core::option::Option<ConditionExplanation>,
}
/// Nested message and enum types in `AllowBindingExplanation`.
pub mod allow_binding_explanation {
    /// Details about whether the role binding includes the principal.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AnnotatedAllowMembership {
        /// Indicates whether the role binding includes the principal.
        #[prost(enumeration = "super::MembershipMatchingState", tag = "1")]
        pub membership: i32,
        /// The relevance of the principal's status to the overall determination for
        /// the role binding.
        #[prost(enumeration = "super::HeuristicRelevance", tag = "2")]
        pub relevance: i32,
    }
}
/// Details about how the relevant IAM deny policies affect the final access
/// state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenyPolicyExplanation {
    /// Indicates whether the principal is denied the specified permission for
    /// the specified resource, based on evaluating all applicable IAM deny
    /// policies.
    #[prost(enumeration = "DenyAccessState", tag = "1")]
    pub deny_access_state: i32,
    /// List of resources with IAM deny policies that were evaluated to check the
    /// principal's denied permissions, with annotations to indicate how each
    /// policy contributed to the final result.
    ///
    /// The list of resources includes the policy for the resource itself, as well
    /// as policies that are inherited from higher levels of the resource
    /// hierarchy, including the organization, the folder, and the project. The
    /// order of the resources starts from the resource and climbs up the resource
    /// hierarchy.
    ///
    /// To learn more about the resource hierarchy, see
    /// <https://cloud.google.com/iam/help/resource-hierarchy.>
    #[prost(message, repeated, tag = "2")]
    pub explained_resources: ::prost::alloc::vec::Vec<ExplainedDenyResource>,
    /// The relevance of the deny policy result to the overall access state.
    #[prost(enumeration = "HeuristicRelevance", tag = "3")]
    pub relevance: i32,
    /// Indicates whether the permission to troubleshoot is supported in deny
    /// policies.
    #[prost(bool, tag = "4")]
    pub permission_deniable: bool,
}
/// Details about how a specific resource contributed to the deny policy
/// evaluation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplainedDenyResource {
    /// Required. Indicates whether any policies attached to _this resource_ deny
    /// the specific permission to the specified principal for the specified
    /// resource.
    ///
    /// This field does _not_ indicate whether the principal actually has the
    /// permission for the resource. There might be another policy that overrides
    /// this policy. To determine whether the principal actually has the
    /// permission, use the `overall_access_state` field in the
    /// [TroubleshootIamPolicyResponse][google.cloud.policytroubleshooter.iam.v3beta.TroubleshootIamPolicyResponse].
    #[prost(enumeration = "DenyAccessState", tag = "1")]
    pub deny_access_state: i32,
    /// The full resource name that identifies the resource. For example,
    /// `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`.
    ///
    /// If the sender of the request does not have access to the policy, this field
    /// is omitted.
    ///
    /// For examples of full resource names for Google Cloud services, see
    /// <https://cloud.google.com/iam/help/troubleshooter/full-resource-names.>
    #[prost(string, tag = "2")]
    pub full_resource_name: ::prost::alloc::string::String,
    /// List of IAM deny policies that were evaluated to check the principal's
    /// denied permissions, with annotations to indicate how each policy
    /// contributed to the final result.
    #[prost(message, repeated, tag = "3")]
    pub explained_policies: ::prost::alloc::vec::Vec<ExplainedDenyPolicy>,
    /// The relevance of this policy to the overall access state in the
    /// [TroubleshootIamPolicyResponse][google.cloud.policytroubleshooter.iam.v3beta.TroubleshootIamPolicyResponse].
    ///
    /// If the sender of the request does not have access to the policy, this field
    /// is omitted.
    #[prost(enumeration = "HeuristicRelevance", tag = "4")]
    pub relevance: i32,
}
/// Details about how a specific IAM deny policy [Policy][google.iam.v2.Policy]
/// contributed to the access check.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplainedDenyPolicy {
    /// Required. Indicates whether _this policy_ denies the specified permission
    /// to the specified principal for the specified resource.
    ///
    /// This field does _not_ indicate whether the principal actually has the
    /// permission for the resource. There might be another policy that overrides
    /// this policy. To determine whether the principal actually has the
    /// permission, use the `overall_access_state` field in the
    /// [TroubleshootIamPolicyResponse][google.cloud.policytroubleshooter.iam.v3beta.TroubleshootIamPolicyResponse].
    #[prost(enumeration = "DenyAccessState", tag = "1")]
    pub deny_access_state: i32,
    /// The IAM deny policy attached to the resource.
    ///
    /// If the sender of the request does not have access to the policy, this field
    /// is omitted.
    #[prost(message, optional, tag = "2")]
    pub policy: ::core::option::Option<super::super::super::super::iam::v2::Policy>,
    /// Details about how each rule in the policy affects the principal's inability
    /// to use the permission for the resource. The order of the deny rule matches
    /// the order of the rules in the deny policy.
    ///
    /// If the sender of the request does not have access to the policy, this field
    /// is omitted.
    #[prost(message, repeated, tag = "3")]
    pub rule_explanations: ::prost::alloc::vec::Vec<DenyRuleExplanation>,
    /// The relevance of this policy to the overall access state in the
    /// [TroubleshootIamPolicyResponse][google.cloud.policytroubleshooter.iam.v3beta.TroubleshootIamPolicyResponse].
    ///
    /// If the sender of the request does not have access to the policy, this field
    /// is omitted.
    #[prost(enumeration = "HeuristicRelevance", tag = "4")]
    pub relevance: i32,
}
/// Details about how a deny rule in a deny policy affects a principal's ability
/// to use a permission.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenyRuleExplanation {
    /// Required. Indicates whether _this rule_ denies the specified permission to
    /// the specified principal for the specified resource.
    ///
    /// This field does _not_ indicate whether the principal is actually denied on
    /// the permission for the resource. There might be another rule that overrides
    /// this rule. To determine whether the principal actually has the permission,
    /// use the `overall_access_state` field in the
    /// [TroubleshootIamPolicyResponse][google.cloud.policytroubleshooter.iam.v3beta.TroubleshootIamPolicyResponse].
    #[prost(enumeration = "DenyAccessState", tag = "1")]
    pub deny_access_state: i32,
    /// Indicates whether the permission in the request is listed as a denied
    /// permission in the deny rule.
    #[prost(message, optional, tag = "2")]
    pub combined_denied_permission: ::core::option::Option<
        deny_rule_explanation::AnnotatedPermissionMatching,
    >,
    /// Lists all denied permissions in the deny rule and indicates whether each
    /// permission matches the permission in the request.
    ///
    /// Each key identifies a denied permission in the rule, and each value
    /// indicates whether the denied permission matches the permission in the
    /// request.
    #[prost(btree_map = "string, message", tag = "3")]
    pub denied_permissions: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        deny_rule_explanation::AnnotatedPermissionMatching,
    >,
    /// Indicates whether the permission in the request is listed as an exception
    /// permission in the deny rule.
    #[prost(message, optional, tag = "4")]
    pub combined_exception_permission: ::core::option::Option<
        deny_rule_explanation::AnnotatedPermissionMatching,
    >,
    /// Lists all exception permissions in the deny rule and indicates whether each
    /// permission matches the permission in the request.
    ///
    /// Each key identifies a exception permission in the rule, and each value
    /// indicates whether the exception permission matches the permission in the
    /// request.
    #[prost(btree_map = "string, message", tag = "5")]
    pub exception_permissions: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        deny_rule_explanation::AnnotatedPermissionMatching,
    >,
    /// Indicates whether the principal is listed as a denied principal in the
    /// deny rule, either directly or through membership in a principal set.
    #[prost(message, optional, tag = "6")]
    pub combined_denied_principal: ::core::option::Option<
        deny_rule_explanation::AnnotatedDenyPrincipalMatching,
    >,
    /// Lists all denied principals in the deny rule and indicates whether each
    /// principal matches the principal in the request, either directly or through
    /// membership in a principal set.
    ///
    /// Each key identifies a denied principal in the rule, and each value
    /// indicates whether the denied principal matches the principal in the
    /// request.
    #[prost(btree_map = "string, message", tag = "7")]
    pub denied_principals: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        deny_rule_explanation::AnnotatedDenyPrincipalMatching,
    >,
    /// Indicates whether the principal is listed as an exception principal in the
    /// deny rule, either directly or through membership in a principal set.
    #[prost(message, optional, tag = "8")]
    pub combined_exception_principal: ::core::option::Option<
        deny_rule_explanation::AnnotatedDenyPrincipalMatching,
    >,
    /// Lists all exception principals in the deny rule and indicates whether each
    /// principal matches the principal in the request, either directly or through
    /// membership in a principal set.
    ///
    /// Each key identifies a exception principal in the rule, and each value
    /// indicates whether the exception principal matches the principal in the
    /// request.
    #[prost(btree_map = "string, message", tag = "9")]
    pub exception_principals: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        deny_rule_explanation::AnnotatedDenyPrincipalMatching,
    >,
    /// The relevance of this role binding to the overall determination for the
    /// entire policy.
    #[prost(enumeration = "HeuristicRelevance", tag = "10")]
    pub relevance: i32,
    /// A condition expression that specifies when the deny rule denies the
    /// principal access.
    ///
    /// To learn about IAM Conditions, see
    /// <https://cloud.google.com/iam/help/conditions/overview.>
    #[prost(message, optional, tag = "11")]
    pub condition: ::core::option::Option<super::super::super::super::r#type::Expr>,
    /// Condition evaluation state for this role binding.
    #[prost(message, optional, tag = "12")]
    pub condition_explanation: ::core::option::Option<ConditionExplanation>,
}
/// Nested message and enum types in `DenyRuleExplanation`.
pub mod deny_rule_explanation {
    /// Details about whether the permission in the request is denied by the
    /// deny rule.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AnnotatedPermissionMatching {
        /// Indicates whether the permission in the request is denied by the deny
        /// rule.
        #[prost(enumeration = "super::PermissionPatternMatchingState", tag = "1")]
        pub permission_matching_state: i32,
        /// The relevance of the permission status to the overall determination for
        /// the rule.
        #[prost(enumeration = "super::HeuristicRelevance", tag = "2")]
        pub relevance: i32,
    }
    /// Details about whether the principal in the request is listed as a denied
    /// principal in the deny rule, either directly or through membership in a
    /// principal set.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AnnotatedDenyPrincipalMatching {
        /// Indicates whether the principal is listed as a denied principal in the
        /// deny rule, either directly or through membership in a principal set.
        #[prost(enumeration = "super::MembershipMatchingState", tag = "1")]
        pub membership: i32,
        /// The relevance of the principal's status to the overall determination for
        /// the role binding.
        #[prost(enumeration = "super::HeuristicRelevance", tag = "2")]
        pub relevance: i32,
    }
}
/// Explanation for how a condition affects a principal's access
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionExplanation {
    /// Value of the condition.
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<::prost_types::Value>,
    /// Any errors that prevented complete evaluation of the condition expression.
    #[prost(message, repeated, tag = "3")]
    pub errors: ::prost::alloc::vec::Vec<super::super::super::super::rpc::Status>,
    /// The value of each statement of the condition expression. The value can be
    /// `true`, `false`, or `null`. The value is `null` if the statement can't be
    /// evaluated.
    #[prost(message, repeated, tag = "2")]
    pub evaluation_states: ::prost::alloc::vec::Vec<
        condition_explanation::EvaluationState,
    >,
}
/// Nested message and enum types in `ConditionExplanation`.
pub mod condition_explanation {
    /// Evaluated state of a condition expression.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EvaluationState {
        /// Start position of an expression in the condition, by character.
        #[prost(int32, tag = "1")]
        pub start: i32,
        /// End position of an expression in the condition, by character,
        /// end included, for example: the end position of the first part of
        /// `a==b || c==d` would be 4.
        #[prost(int32, tag = "2")]
        pub end: i32,
        /// Value of this expression.
        #[prost(message, optional, tag = "3")]
        pub value: ::core::option::Option<::prost_types::Value>,
        /// Any errors that prevented complete evaluation of the condition
        /// expression.
        #[prost(message, repeated, tag = "4")]
        pub errors: ::prost::alloc::vec::Vec<
            super::super::super::super::super::rpc::Status,
        >,
    }
}
/// Whether IAM allow policies gives the principal the permission.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AllowAccessState {
    /// Not specified.
    Unspecified = 0,
    /// The allow policy gives the principal the permission.
    Granted = 1,
    /// The allow policy doesn't give the principal the permission.
    NotGranted = 2,
    /// The allow policy gives the principal the permission if a condition
    /// expression evaluate to `true`. However, the sender of the request didn't
    /// provide enough context for Policy Troubleshooter to evaluate the condition
    /// expression.
    UnknownConditional = 3,
    /// The sender of the request doesn't have access to all of the allow policies
    /// that Policy Troubleshooter needs to evaluate the principal's access.
    UnknownInfo = 4,
}
impl AllowAccessState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AllowAccessState::Unspecified => "ALLOW_ACCESS_STATE_UNSPECIFIED",
            AllowAccessState::Granted => "ALLOW_ACCESS_STATE_GRANTED",
            AllowAccessState::NotGranted => "ALLOW_ACCESS_STATE_NOT_GRANTED",
            AllowAccessState::UnknownConditional => {
                "ALLOW_ACCESS_STATE_UNKNOWN_CONDITIONAL"
            }
            AllowAccessState::UnknownInfo => "ALLOW_ACCESS_STATE_UNKNOWN_INFO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALLOW_ACCESS_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "ALLOW_ACCESS_STATE_GRANTED" => Some(Self::Granted),
            "ALLOW_ACCESS_STATE_NOT_GRANTED" => Some(Self::NotGranted),
            "ALLOW_ACCESS_STATE_UNKNOWN_CONDITIONAL" => Some(Self::UnknownConditional),
            "ALLOW_ACCESS_STATE_UNKNOWN_INFO" => Some(Self::UnknownInfo),
            _ => None,
        }
    }
}
/// Whether IAM deny policies deny the principal the permission.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DenyAccessState {
    /// Not specified.
    Unspecified = 0,
    /// The deny policy denies the principal the permission.
    Denied = 1,
    /// The deny policy doesn't deny the principal the permission.
    NotDenied = 2,
    /// The deny policy denies the principal the permission if a condition
    /// expression evaluates to `true`. However, the sender of the request didn't
    /// provide enough context for Policy Troubleshooter to evaluate the condition
    /// expression.
    UnknownConditional = 3,
    /// The sender of the request does not have access to all of the deny policies
    /// that Policy Troubleshooter needs to evaluate the principal's access.
    UnknownInfo = 4,
}
impl DenyAccessState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DenyAccessState::Unspecified => "DENY_ACCESS_STATE_UNSPECIFIED",
            DenyAccessState::Denied => "DENY_ACCESS_STATE_DENIED",
            DenyAccessState::NotDenied => "DENY_ACCESS_STATE_NOT_DENIED",
            DenyAccessState::UnknownConditional => {
                "DENY_ACCESS_STATE_UNKNOWN_CONDITIONAL"
            }
            DenyAccessState::UnknownInfo => "DENY_ACCESS_STATE_UNKNOWN_INFO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DENY_ACCESS_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "DENY_ACCESS_STATE_DENIED" => Some(Self::Denied),
            "DENY_ACCESS_STATE_NOT_DENIED" => Some(Self::NotDenied),
            "DENY_ACCESS_STATE_UNKNOWN_CONDITIONAL" => Some(Self::UnknownConditional),
            "DENY_ACCESS_STATE_UNKNOWN_INFO" => Some(Self::UnknownInfo),
            _ => None,
        }
    }
}
/// Whether a role includes a specific permission.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RolePermissionInclusionState {
    /// Not specified.
    Unspecified = 0,
    /// The permission is included in the role.
    RolePermissionIncluded = 1,
    /// The permission is not included in the role.
    RolePermissionNotIncluded = 2,
    /// The sender of the request is not allowed to access the role definition.
    RolePermissionUnknownInfo = 3,
}
impl RolePermissionInclusionState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RolePermissionInclusionState::Unspecified => {
                "ROLE_PERMISSION_INCLUSION_STATE_UNSPECIFIED"
            }
            RolePermissionInclusionState::RolePermissionIncluded => {
                "ROLE_PERMISSION_INCLUDED"
            }
            RolePermissionInclusionState::RolePermissionNotIncluded => {
                "ROLE_PERMISSION_NOT_INCLUDED"
            }
            RolePermissionInclusionState::RolePermissionUnknownInfo => {
                "ROLE_PERMISSION_UNKNOWN_INFO"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ROLE_PERMISSION_INCLUSION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "ROLE_PERMISSION_INCLUDED" => Some(Self::RolePermissionIncluded),
            "ROLE_PERMISSION_NOT_INCLUDED" => Some(Self::RolePermissionNotIncluded),
            "ROLE_PERMISSION_UNKNOWN_INFO" => Some(Self::RolePermissionUnknownInfo),
            _ => None,
        }
    }
}
/// Whether the permission in the request matches the permission in the policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PermissionPatternMatchingState {
    /// Not specified.
    Unspecified = 0,
    /// The permission in the request matches the permission in the policy.
    PermissionPatternMatched = 1,
    /// The permission in the request matches the permission in the policy.
    PermissionPatternNotMatched = 2,
}
impl PermissionPatternMatchingState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PermissionPatternMatchingState::Unspecified => {
                "PERMISSION_PATTERN_MATCHING_STATE_UNSPECIFIED"
            }
            PermissionPatternMatchingState::PermissionPatternMatched => {
                "PERMISSION_PATTERN_MATCHED"
            }
            PermissionPatternMatchingState::PermissionPatternNotMatched => {
                "PERMISSION_PATTERN_NOT_MATCHED"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PERMISSION_PATTERN_MATCHING_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "PERMISSION_PATTERN_MATCHED" => Some(Self::PermissionPatternMatched),
            "PERMISSION_PATTERN_NOT_MATCHED" => Some(Self::PermissionPatternNotMatched),
            _ => None,
        }
    }
}
/// Whether the principal in the request matches the principal in the policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MembershipMatchingState {
    /// Not specified.
    Unspecified = 0,
    /// The principal in the request matches the principal in the policy. The
    /// principal can be included directly or indirectly:
    ///
    /// * A principal is included directly if that principal is listed in the
    ///    role binding.
    /// * A principal is included indirectly if that principal is in a Google
    ///    group, Google Workspace account, or Cloud Identity domain that is listed
    ///    in the policy.
    MembershipMatched = 1,
    /// The principal in the request doesn't match the principal in the policy.
    MembershipNotMatched = 2,
    /// The principal in the policy is a group or domain, and the sender of the
    /// request doesn't have permission to view whether the principal in the
    /// request is a member of the group or domain.
    MembershipUnknownInfo = 3,
    /// The principal is an unsupported type.
    MembershipUnknownUnsupported = 4,
}
impl MembershipMatchingState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MembershipMatchingState::Unspecified => {
                "MEMBERSHIP_MATCHING_STATE_UNSPECIFIED"
            }
            MembershipMatchingState::MembershipMatched => "MEMBERSHIP_MATCHED",
            MembershipMatchingState::MembershipNotMatched => "MEMBERSHIP_NOT_MATCHED",
            MembershipMatchingState::MembershipUnknownInfo => "MEMBERSHIP_UNKNOWN_INFO",
            MembershipMatchingState::MembershipUnknownUnsupported => {
                "MEMBERSHIP_UNKNOWN_UNSUPPORTED"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MEMBERSHIP_MATCHING_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "MEMBERSHIP_MATCHED" => Some(Self::MembershipMatched),
            "MEMBERSHIP_NOT_MATCHED" => Some(Self::MembershipNotMatched),
            "MEMBERSHIP_UNKNOWN_INFO" => Some(Self::MembershipUnknownInfo),
            "MEMBERSHIP_UNKNOWN_UNSUPPORTED" => Some(Self::MembershipUnknownUnsupported),
            _ => None,
        }
    }
}
/// The extent to which a single data point contributes to an overall
/// determination.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HeuristicRelevance {
    /// Not specified.
    Unspecified = 0,
    /// The data point has a limited effect on the result. Changing the data point
    /// is unlikely to affect the overall determination.
    Normal = 1,
    /// The data point has a strong effect on the result. Changing the data point
    /// is likely to affect the overall determination.
    High = 2,
}
impl HeuristicRelevance {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HeuristicRelevance::Unspecified => "HEURISTIC_RELEVANCE_UNSPECIFIED",
            HeuristicRelevance::Normal => "HEURISTIC_RELEVANCE_NORMAL",
            HeuristicRelevance::High => "HEURISTIC_RELEVANCE_HIGH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HEURISTIC_RELEVANCE_UNSPECIFIED" => Some(Self::Unspecified),
            "HEURISTIC_RELEVANCE_NORMAL" => Some(Self::Normal),
            "HEURISTIC_RELEVANCE_HIGH" => Some(Self::High),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod policy_troubleshooter_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// IAM Policy Troubleshooter service.
    ///
    /// This service helps you troubleshoot access issues for Google Cloud resources.
    #[derive(Debug, Clone)]
    pub struct PolicyTroubleshooterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PolicyTroubleshooterClient<T>
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
        ) -> PolicyTroubleshooterClient<InterceptedService<T, F>>
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
            PolicyTroubleshooterClient::new(InterceptedService::new(inner, interceptor))
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
        /// Checks whether a principal has a specific permission for a specific
        /// resource, and explains why the principal does or doesn't have that
        /// permission.
        pub async fn troubleshoot_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::TroubleshootIamPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TroubleshootIamPolicyResponse>,
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
                "/google.cloud.policytroubleshooter.iam.v3beta.PolicyTroubleshooter/TroubleshootIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.policytroubleshooter.iam.v3beta.PolicyTroubleshooter",
                        "TroubleshootIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
