/// Defines a Cloud Organization `Policy` which is used to specify `Constraints`
/// for configurations of Cloud Platform resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    /// Version of the `Policy`. Default version is 0;
    #[prost(int32, tag = "1")]
    pub version: i32,
    /// The name of the `Constraint` the `Policy` is configuring, for example,
    /// `constraints/serviceuser.services`.
    ///
    /// Immutable after creation.
    #[prost(string, tag = "2")]
    pub constraint: ::prost::alloc::string::String,
    /// An opaque tag indicating the current version of the `Policy`, used for
    /// concurrency control.
    ///
    /// When the `Policy` is returned from either a `GetPolicy` or a
    /// `ListOrgPolicy` request, this `etag` indicates the version of the current
    /// `Policy` to use when executing a read-modify-write loop.
    ///
    /// When the `Policy` is returned from a `GetEffectivePolicy` request, the
    /// `etag` will be unset.
    ///
    /// When the `Policy` is used in a `SetOrgPolicy` method, use the `etag` value
    /// that was returned from a `GetOrgPolicy` request as part of a
    /// read-modify-write loop for concurrency control. Not setting the `etag`in a
    /// `SetOrgPolicy` request will result in an unconditional write of the
    /// `Policy`.
    #[prost(bytes = "bytes", tag = "3")]
    pub etag: ::prost::bytes::Bytes,
    /// The time stamp the `Policy` was previously updated. This is set by the
    /// server, not specified by the caller, and represents the last time a call to
    /// `SetOrgPolicy` was made for that `Policy`. Any value set by the client will
    /// be ignored.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The field to populate is based on the `constraint_type` value in the
    /// `Constraint`.
    ///    `list_constraint` => `list_policy`
    ///    `boolean_constraint` => `boolean_policy`
    ///
    ///   A `restore_default` message may be used with any `Constraint` type.
    ///
    /// Providing a *_policy that is incompatible with the `constraint_type` will
    /// result in an `invalid_argument` error.
    ///
    /// Attempting to set a `Policy` with a `policy_type` not set will result in an
    /// `invalid_argument` error.
    #[prost(oneof = "policy::PolicyType", tags = "5, 6, 7")]
    pub policy_type: ::core::option::Option<policy::PolicyType>,
}
/// Nested message and enum types in `Policy`.
pub mod policy {
    /// Used in `policy_type` to specify how `list_policy` behaves at this
    /// resource.
    ///
    /// `ListPolicy` can define specific values and subtrees of Cloud Resource
    /// Manager resource hierarchy (`Organizations`, `Folders`, `Projects`) that
    /// are allowed or denied by setting the `allowed_values` and `denied_values`
    /// fields. This is achieved by using the `under:` and optional `is:` prefixes.
    /// The `under:` prefix is used to denote resource subtree values.
    /// The `is:` prefix is used to denote specific values, and is required only
    /// if the value contains a ":". Values prefixed with "is:" are treated the
    /// same as values with no prefix.
    /// Ancestry subtrees must be in one of the following formats:
    ///      - "projects/<project-id>", e.g. "projects/tokyo-rain-123"
    ///      - "folders/<folder-id>", e.g. "folders/1234"
    ///      - "organizations/<organization-id>", e.g. "organizations/1234"
    /// The `supports_under` field of the associated `Constraint`  defines whether
    /// ancestry prefixes can be used. You can set `allowed_values` and
    /// `denied_values` in the same `Policy` if `all_values` is
    /// `ALL_VALUES_UNSPECIFIED`. `ALLOW` or `DENY` are used to allow or deny all
    /// values. If `all_values` is set to either `ALLOW` or `DENY`,
    /// `allowed_values` and `denied_values` must be unset.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListPolicy {
        /// List of values allowed  at this resource. Can only be set if `all_values`
        /// is set to `ALL_VALUES_UNSPECIFIED`.
        #[prost(string, repeated, tag = "1")]
        pub allowed_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// List of values denied at this resource. Can only be set if `all_values`
        /// is set to `ALL_VALUES_UNSPECIFIED`.
        #[prost(string, repeated, tag = "2")]
        pub denied_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The policy all_values state.
        #[prost(enumeration = "list_policy::AllValues", tag = "3")]
        pub all_values: i32,
        /// Optional. The Google Cloud Console will try to default to a configuration
        /// that matches the value specified in this `Policy`. If `suggested_value`
        /// is not set, it will inherit the value specified higher in the hierarchy,
        /// unless `inherit_from_parent` is `false`.
        #[prost(string, tag = "4")]
        pub suggested_value: ::prost::alloc::string::String,
        /// Determines the inheritance behavior for this `Policy`.
        ///
        /// By default, a `ListPolicy` set at a resource supercedes any `Policy` set
        /// anywhere up the resource hierarchy. However, if `inherit_from_parent` is
        /// set to `true`, then the values from the effective `Policy` of the parent
        /// resource are inherited, meaning the values set in this `Policy` are
        /// added to the values inherited up the hierarchy.
        ///
        /// Setting `Policy` hierarchies that inherit both allowed values and denied
        /// values isn't recommended in most circumstances to keep the configuration
        /// simple and understandable. However, it is possible to set a `Policy` with
        /// `allowed_values` set that inherits a `Policy` with `denied_values` set.
        /// In this case, the values that are allowed must be in `allowed_values` and
        /// not present in `denied_values`.
        ///
        /// For example, suppose you have a `Constraint`
        /// `constraints/serviceuser.services`, which has a `constraint_type` of
        /// `list_constraint`, and with `constraint_default` set to `ALLOW`.
        /// Suppose that at the Organization level, a `Policy` is applied that
        /// restricts the allowed API activations to {`E1`, `E2`}. Then, if a
        /// `Policy` is applied to a project below the Organization that has
        /// `inherit_from_parent` set to `false` and field all_values set to DENY,
        /// then an attempt to activate any API will be denied.
        ///
        /// The following examples demonstrate different possible layerings for
        /// `projects/bar` parented by `organizations/foo`:
        ///
        /// Example 1 (no inherited values):
        ///    `organizations/foo` has a `Policy` with values:
        ///      {allowed_values: "E1" allowed_values:"E2"}
        ///    `projects/bar` has `inherit_from_parent` `false` and values:
        ///      {allowed_values: "E3" allowed_values: "E4"}
        /// The accepted values at `organizations/foo` are `E1`, `E2`.
        /// The accepted values at `projects/bar` are `E3`, and `E4`.
        ///
        /// Example 2 (inherited values):
        ///    `organizations/foo` has a `Policy` with values:
        ///      {allowed_values: "E1" allowed_values:"E2"}
        ///    `projects/bar` has a `Policy` with values:
        ///      {value: "E3" value: "E4" inherit_from_parent: true}
        /// The accepted values at `organizations/foo` are `E1`, `E2`.
        /// The accepted values at `projects/bar` are `E1`, `E2`, `E3`, and `E4`.
        ///
        /// Example 3 (inheriting both allowed and denied values):
        ///    `organizations/foo` has a `Policy` with values:
        ///      {allowed_values: "E1" allowed_values: "E2"}
        ///    `projects/bar` has a `Policy` with:
        ///      {denied_values: "E1"}
        /// The accepted values at `organizations/foo` are `E1`, `E2`.
        /// The value accepted at `projects/bar` is `E2`.
        ///
        /// Example 4 (RestoreDefault):
        ///    `organizations/foo` has a `Policy` with values:
        ///      {allowed_values: "E1" allowed_values:"E2"}
        ///    `projects/bar` has a `Policy` with values:
        ///      {RestoreDefault: {}}
        /// The accepted values at `organizations/foo` are `E1`, `E2`.
        /// The accepted values at `projects/bar` are either all or none depending on
        /// the value of `constraint_default` (if `ALLOW`, all; if
        /// `DENY`, none).
        ///
        /// Example 5 (no policy inherits parent policy):
        ///    `organizations/foo` has no `Policy` set.
        ///    `projects/bar` has no `Policy` set.
        /// The accepted values at both levels are either all or none depending on
        /// the value of `constraint_default` (if `ALLOW`, all; if
        /// `DENY`, none).
        ///
        /// Example 6 (ListConstraint allowing all):
        ///    `organizations/foo` has a `Policy` with values:
        ///      {allowed_values: "E1" allowed_values: "E2"}
        ///    `projects/bar` has a `Policy` with:
        ///      {all: ALLOW}
        /// The accepted values at `organizations/foo` are `E1`, E2`.
        /// Any value is accepted at `projects/bar`.
        ///
        /// Example 7 (ListConstraint allowing none):
        ///    `organizations/foo` has a `Policy` with values:
        ///      {allowed_values: "E1" allowed_values: "E2"}
        ///    `projects/bar` has a `Policy` with:
        ///      {all: DENY}
        /// The accepted values at `organizations/foo` are `E1`, E2`.
        /// No value is accepted at `projects/bar`.
        ///
        /// Example 10 (allowed and denied subtrees of Resource Manager hierarchy):
        /// Given the following resource hierarchy
        ///    O1->{F1, F2}; F1->{P1}; F2->{P2, P3},
        ///    `organizations/foo` has a `Policy` with values:
        ///      {allowed_values: "under:organizations/O1"}
        ///    `projects/bar` has a `Policy` with:
        ///      {allowed_values: "under:projects/P3"}
        ///      {denied_values: "under:folders/F2"}
        /// The accepted values at `organizations/foo` are `organizations/O1`,
        ///    `folders/F1`, `folders/F2`, `projects/P1`, `projects/P2`,
        ///    `projects/P3`.
        /// The accepted values at `projects/bar` are `organizations/O1`,
        ///    `folders/F1`, `projects/P1`.
        #[prost(bool, tag = "5")]
        pub inherit_from_parent: bool,
    }
    /// Nested message and enum types in `ListPolicy`.
    pub mod list_policy {
        /// This enum can be used to set `Policies` that apply to all possible
        /// configuration values rather than specific values in `allowed_values` or
        /// `denied_values`.
        ///
        /// Settting this to `ALLOW` will mean this `Policy` allows all values.
        /// Similarly, setting it to `DENY` will mean no values are allowed. If
        /// set to either `ALLOW` or `DENY,  `allowed_values` and `denied_values`
        /// must be unset. Setting this to `ALL_VALUES_UNSPECIFIED` allows for
        /// setting `allowed_values` and `denied_values`.
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
        pub enum AllValues {
            /// Indicates that allowed_values or denied_values must be set.
            Unspecified = 0,
            /// A policy with this set allows all values.
            Allow = 1,
            /// A policy with this set denies all values.
            Deny = 2,
        }
        impl AllValues {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    AllValues::Unspecified => "ALL_VALUES_UNSPECIFIED",
                    AllValues::Allow => "ALLOW",
                    AllValues::Deny => "DENY",
                }
            }
        }
    }
    /// Used in `policy_type` to specify how `boolean_policy` will behave at this
    /// resource.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BooleanPolicy {
        /// If `true`, then the `Policy` is enforced. If `false`, then any
        /// configuration is acceptable.
        ///
        /// Suppose you have a `Constraint`
        /// `constraints/compute.disableSerialPortAccess` with `constraint_default`
        /// set to `ALLOW`. A `Policy` for that `Constraint` exhibits the following
        /// behavior:
        ///    - If the `Policy` at this resource has enforced set to `false`, serial
        ///      port connection attempts will be allowed.
        ///    - If the `Policy` at this resource has enforced set to `true`, serial
        ///      port connection attempts will be refused.
        ///    - If the `Policy` at this resource is `RestoreDefault`, serial port
        ///      connection attempts will be allowed.
        ///    - If no `Policy` is set at this resource or anywhere higher in the
        ///      resource hierarchy, serial port connection attempts will be allowed.
        ///    - If no `Policy` is set at this resource, but one exists higher in the
        ///      resource hierarchy, the behavior is as if the`Policy` were set at
        ///      this resource.
        ///
        /// The following examples demonstrate the different possible layerings:
        ///
        /// Example 1 (nearest `Constraint` wins):
        ///    `organizations/foo` has a `Policy` with:
        ///      {enforced: false}
        ///    `projects/bar` has no `Policy` set.
        /// The constraint at `projects/bar` and `organizations/foo` will not be
        /// enforced.
        ///
        /// Example 2 (enforcement gets replaced):
        ///    `organizations/foo` has a `Policy` with:
        ///      {enforced: false}
        ///    `projects/bar` has a `Policy` with:
        ///      {enforced: true}
        /// The constraint at `organizations/foo` is not enforced.
        /// The constraint at `projects/bar` is enforced.
        ///
        /// Example 3 (RestoreDefault):
        ///    `organizations/foo` has a `Policy` with:
        ///      {enforced: true}
        ///    `projects/bar` has a `Policy` with:
        ///      {RestoreDefault: {}}
        /// The constraint at `organizations/foo` is enforced.
        /// The constraint at `projects/bar` is not enforced, because
        /// `constraint_default` for the `Constraint` is `ALLOW`.
        #[prost(bool, tag = "1")]
        pub enforced: bool,
    }
    /// Ignores policies set above this resource and restores the
    /// `constraint_default` enforcement behavior of the specific `Constraint` at
    /// this resource.
    ///
    /// Suppose that `constraint_default` is set to `ALLOW` for the
    /// `Constraint` `constraints/serviceuser.services`. Suppose that organization
    /// foo.com sets a `Policy` at their Organization resource node that restricts
    /// the allowed service activations to deny all service activations. They
    /// could then set a `Policy` with the `policy_type` `restore_default` on
    /// several experimental projects, restoring the `constraint_default`
    /// enforcement of the `Constraint` for only those projects, allowing those
    /// projects to have all services activated.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RestoreDefault {}
    /// The field to populate is based on the `constraint_type` value in the
    /// `Constraint`.
    ///    `list_constraint` => `list_policy`
    ///    `boolean_constraint` => `boolean_policy`
    ///
    ///   A `restore_default` message may be used with any `Constraint` type.
    ///
    /// Providing a *_policy that is incompatible with the `constraint_type` will
    /// result in an `invalid_argument` error.
    ///
    /// Attempting to set a `Policy` with a `policy_type` not set will result in an
    /// `invalid_argument` error.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PolicyType {
        /// List of values either allowed or disallowed.
        #[prost(message, tag = "5")]
        ListPolicy(ListPolicy),
        /// For boolean `Constraints`, whether to enforce the `Constraint` or not.
        #[prost(message, tag = "6")]
        BooleanPolicy(BooleanPolicy),
        /// Restores the default behavior of the constraint; independent of
        /// `Constraint` type.
        #[prost(message, tag = "7")]
        RestoreDefault(RestoreDefault),
    }
}
