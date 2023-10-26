/// A service that is available for use by the consumer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// The resource name of the consumer and service.
    ///
    /// A valid name would be:
    /// - `projects/123/services/serviceusage.googleapis.com`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The resource name of the consumer.
    ///
    /// A valid name would be:
    /// - `projects/123`
    #[prost(string, tag = "5")]
    pub parent: ::prost::alloc::string::String,
    /// The service configuration of the available service.
    /// Some fields may be filtered out of the configuration in responses to
    /// the `ListServices` method. These fields are present only in responses to
    /// the `GetService` method.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<ServiceConfig>,
    /// Whether or not the service has been enabled for use by the consumer.
    #[prost(enumeration = "State", tag = "4")]
    pub state: i32,
}
/// The configuration of the service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceConfig {
    /// The DNS address at which this service is available.
    ///
    /// An example DNS address would be:
    /// `calendar.googleapis.com`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The product title for this service.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// A list of API interfaces exported by this service. Contains only the names,
    /// versions, and method names of the interfaces.
    #[prost(message, repeated, tag = "3")]
    pub apis: ::prost::alloc::vec::Vec<::prost_types::Api>,
    /// Additional API documentation. Contains only the summary and the
    /// documentation URL.
    #[prost(message, optional, tag = "6")]
    pub documentation: ::core::option::Option<super::super::Documentation>,
    /// Quota configuration.
    #[prost(message, optional, tag = "10")]
    pub quota: ::core::option::Option<super::super::Quota>,
    /// Auth configuration. Contains only the OAuth rules.
    #[prost(message, optional, tag = "11")]
    pub authentication: ::core::option::Option<super::super::Authentication>,
    /// Configuration controlling usage of this service.
    #[prost(message, optional, tag = "15")]
    pub usage: ::core::option::Option<super::super::Usage>,
    /// Configuration for network endpoints. Contains only the names and aliases
    /// of the endpoints.
    #[prost(message, repeated, tag = "18")]
    pub endpoints: ::prost::alloc::vec::Vec<super::super::Endpoint>,
    /// Defines the monitored resources used by this service. This is required
    /// by the [Service.monitoring][google.api.Service.monitoring] and
    /// [Service.logging][google.api.Service.logging] configurations.
    #[prost(message, repeated, tag = "25")]
    pub monitored_resources: ::prost::alloc::vec::Vec<
        super::super::MonitoredResourceDescriptor,
    >,
    /// Monitoring configuration.
    /// This should not include the 'producer_destinations' field.
    #[prost(message, optional, tag = "28")]
    pub monitoring: ::core::option::Option<super::super::Monitoring>,
}
/// The operation metadata returned for the batchend services operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The full name of the resources that this operation is directly
    /// associated with.
    #[prost(string, repeated, tag = "2")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Consumer quota settings for a quota metric.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumerQuotaMetric {
    /// The resource name of the quota settings on this metric for this consumer.
    ///
    /// An example name would be:
    /// `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus`
    ///
    /// The resource name is intended to be opaque and should not be parsed for
    /// its component strings, since its representation could change in the future.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the metric.
    ///
    /// An example name would be:
    /// `compute.googleapis.com/cpus`
    #[prost(string, tag = "4")]
    pub metric: ::prost::alloc::string::String,
    /// The display name of the metric.
    ///
    /// An example name would be:
    /// `CPUs`
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The consumer quota for each quota limit defined on the metric.
    #[prost(message, repeated, tag = "3")]
    pub consumer_quota_limits: ::prost::alloc::vec::Vec<ConsumerQuotaLimit>,
    /// The quota limits targeting the descendant containers of the
    /// consumer in request.
    ///
    /// If the consumer in request is of type `organizations`
    /// or `folders`, the field will list per-project limits in the metric; if the
    /// consumer in request is of type `project`, the field will be empty.
    ///
    /// The `quota_buckets` field of each descendant consumer quota limit will not
    /// be populated.
    #[prost(message, repeated, tag = "6")]
    pub descendant_consumer_quota_limits: ::prost::alloc::vec::Vec<ConsumerQuotaLimit>,
    /// The units in which the metric value is reported.
    #[prost(string, tag = "5")]
    pub unit: ::prost::alloc::string::String,
}
/// Consumer quota settings for a quota limit.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumerQuotaLimit {
    /// The resource name of the quota limit.
    ///
    /// An example name would be:
    /// `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion`
    ///
    /// The resource name is intended to be opaque and should not be parsed for
    /// its component strings, since its representation could change in the future.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the parent metric of this limit.
    ///
    /// An example name would be:
    /// `compute.googleapis.com/cpus`
    #[prost(string, tag = "8")]
    pub metric: ::prost::alloc::string::String,
    /// The limit unit.
    ///
    /// An example unit would be
    /// `1/{project}/{region}`
    /// Note that `{project}` and `{region}` are not placeholders in this example;
    /// the literal characters `{` and `}` occur in the string.
    #[prost(string, tag = "2")]
    pub unit: ::prost::alloc::string::String,
    /// Whether this limit is precise or imprecise.
    #[prost(bool, tag = "3")]
    pub is_precise: bool,
    /// Whether admin overrides are allowed on this limit
    #[prost(bool, tag = "7")]
    pub allows_admin_overrides: bool,
    /// Summary of the enforced quota buckets, organized by quota dimension,
    /// ordered from least specific to most specific (for example, the global
    /// default bucket, with no quota dimensions, will always appear first).
    #[prost(message, repeated, tag = "9")]
    pub quota_buckets: ::prost::alloc::vec::Vec<QuotaBucket>,
    /// List of all supported locations.
    /// This field is present only if the limit has a {region} or {zone} dimension.
    #[prost(string, repeated, tag = "11")]
    pub supported_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A quota bucket is a quota provisioning unit for a specific set of dimensions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaBucket {
    /// The effective limit of this quota bucket. Equal to default_limit if there
    /// are no overrides.
    #[prost(int64, tag = "1")]
    pub effective_limit: i64,
    /// The default limit of this quota bucket, as specified by the service
    /// configuration.
    #[prost(int64, tag = "2")]
    pub default_limit: i64,
    /// Producer override on this quota bucket.
    #[prost(message, optional, tag = "3")]
    pub producer_override: ::core::option::Option<QuotaOverride>,
    /// Consumer override on this quota bucket.
    #[prost(message, optional, tag = "4")]
    pub consumer_override: ::core::option::Option<QuotaOverride>,
    /// Admin override on this quota bucket.
    #[prost(message, optional, tag = "5")]
    pub admin_override: ::core::option::Option<QuotaOverride>,
    /// Producer policy inherited from the closet ancestor of the current consumer.
    #[prost(message, optional, tag = "7")]
    pub producer_quota_policy: ::core::option::Option<ProducerQuotaPolicy>,
    /// The dimensions of this quota bucket.
    ///
    /// If this map is empty, this is the global bucket, which is the default quota
    /// value applied to all requests that do not have a more specific override.
    ///
    /// If this map is nonempty, the default limit, effective limit, and quota
    /// overrides apply only to requests that have the dimensions given in the map.
    ///
    /// For example, if the map has key `region` and value `us-east-1`, then the
    /// specified effective limit is only effective in that region, and the
    /// specified overrides apply only in that region.
    #[prost(btree_map = "string, string", tag = "6")]
    pub dimensions: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// A quota override
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaOverride {
    /// The resource name of the override.
    /// This name is generated by the server when the override is created.
    ///
    /// Example names would be:
    /// `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/adminOverrides/4a3f2c1d`
    /// `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/consumerOverrides/4a3f2c1d`
    ///
    /// The resource name is intended to be opaque and should not be parsed for
    /// its component strings, since its representation could change in the future.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The overriding quota limit value.
    /// Can be any nonnegative integer, or -1 (unlimited quota).
    #[prost(int64, tag = "2")]
    pub override_value: i64,
    /// If this map is nonempty, then this override applies only to specific values
    /// for dimensions defined in the limit unit.
    ///
    /// For example, an override on a limit with the unit `1/{project}/{region}`
    /// could contain an entry with the key `region` and the value `us-east-1`;
    /// the override is only applied to quota consumed in that region.
    ///
    /// This map has the following restrictions:
    ///
    /// *   Keys that are not defined in the limit's unit are not valid keys.
    ///      Any string appearing in `{brackets}` in the unit (besides `{project}`
    ///      or
    ///      `{user}`) is a defined key.
    /// *   `project` is not a valid key; the project is already specified in
    ///      the parent resource name.
    /// *   `user` is not a valid key; the API does not support quota overrides
    ///      that apply only to a specific user.
    /// *   If `region` appears as a key, its value must be a valid Cloud region.
    /// *   If `zone` appears as a key, its value must be a valid Cloud zone.
    /// *   If any valid key other than `region` or `zone` appears in the map, then
    ///      all valid keys other than `region` or `zone` must also appear in the
    ///      map.
    #[prost(btree_map = "string, string", tag = "3")]
    pub dimensions: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The name of the metric to which this override applies.
    ///
    /// An example name would be:
    /// `compute.googleapis.com/cpus`
    #[prost(string, tag = "4")]
    pub metric: ::prost::alloc::string::String,
    /// The limit unit of the limit to which this override applies.
    ///
    /// An example unit would be:
    /// `1/{project}/{region}`
    /// Note that `{project}` and `{region}` are not placeholders in this example;
    /// the literal characters `{` and `}` occur in the string.
    #[prost(string, tag = "5")]
    pub unit: ::prost::alloc::string::String,
    /// The resource name of the ancestor that requested the override. For example:
    /// `organizations/12345` or `folders/67890`.
    /// Used by admin overrides only.
    #[prost(string, tag = "6")]
    pub admin_override_ancestor: ::prost::alloc::string::String,
}
/// Import data embedded in the request message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverrideInlineSource {
    /// The overrides to create.
    /// Each override must have a value for 'metric' and 'unit', to specify
    /// which metric and which limit the override should be applied to.
    /// The 'name' field of the override does not need to be set; it is ignored.
    #[prost(message, repeated, tag = "1")]
    pub overrides: ::prost::alloc::vec::Vec<QuotaOverride>,
}
/// Quota policy created by service producer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProducerQuotaPolicy {
    /// The resource name of the policy.
    /// This name is generated by the server when the policy is created.
    ///
    /// Example names would be:
    /// `organizations/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/producerQuotaPolicies/4a3f2c1d`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The quota policy value.
    /// Can be any nonnegative integer, or -1 (unlimited quota).
    #[prost(int64, tag = "2")]
    pub policy_value: i64,
    ///
    /// If this map is nonempty, then this policy applies only to specific values
    /// for dimensions defined in the limit unit.
    ///
    /// For example, a policy on a limit with the unit `1/{project}/{region}`
    /// could contain an entry with the key `region` and the value `us-east-1`;
    /// the policy is only applied to quota consumed in that region.
    ///
    /// This map has the following restrictions:
    ///
    /// *   Keys that are not defined in the limit's unit are not valid keys.
    ///      Any string appearing in {brackets} in the unit (besides {project} or
    ///      {user}) is a defined key.
    /// *   `project` is not a valid key; the project is already specified in
    ///      the parent resource name.
    /// *   `user` is not a valid key; the API does not support quota policies
    ///      that apply only to a specific user.
    /// *   If `region` appears as a key, its value must be a valid Cloud region.
    /// *   If `zone` appears as a key, its value must be a valid Cloud zone.
    /// *   If any valid key other than `region` or `zone` appears in the map, then
    ///      all valid keys other than `region` or `zone` must also appear in the
    ///      map.
    #[prost(btree_map = "string, string", tag = "3")]
    pub dimensions: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The name of the metric to which this policy applies.
    ///
    /// An example name would be:
    /// `compute.googleapis.com/cpus`
    #[prost(string, tag = "4")]
    pub metric: ::prost::alloc::string::String,
    /// The limit unit of the limit to which this policy applies.
    ///
    /// An example unit would be:
    /// `1/{project}/{region}`
    /// Note that `{project}` and `{region}` are not placeholders in this example;
    /// the literal characters `{` and `}` occur in the string.
    #[prost(string, tag = "5")]
    pub unit: ::prost::alloc::string::String,
    /// The cloud resource container at which the quota policy is created. The
    /// format is `{container_type}/{container_number}`
    #[prost(string, tag = "6")]
    pub container: ::prost::alloc::string::String,
}
/// Quota policy created by quota administrator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminQuotaPolicy {
    /// The resource name of the policy.
    /// This name is generated by the server when the policy is created.
    ///
    /// Example names would be:
    /// `organizations/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/adminQuotaPolicies/4a3f2c1d`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The quota policy value.
    /// Can be any nonnegative integer, or -1 (unlimited quota).
    #[prost(int64, tag = "2")]
    pub policy_value: i64,
    ///
    /// If this map is nonempty, then this policy applies only to specific values
    /// for dimensions defined in the limit unit.
    ///
    /// For example, a policy on a limit with the unit `1/{project}/{region}`
    /// could contain an entry with the key `region` and the value `us-east-1`;
    /// the policy is only applied to quota consumed in that region.
    ///
    /// This map has the following restrictions:
    ///
    /// *   If `region` appears as a key, its value must be a valid Cloud region.
    /// *   If `zone` appears as a key, its value must be a valid Cloud zone.
    /// *   Keys other than `region` or `zone` are not valid.
    #[prost(btree_map = "string, string", tag = "3")]
    pub dimensions: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The name of the metric to which this policy applies.
    ///
    /// An example name would be:
    /// `compute.googleapis.com/cpus`
    #[prost(string, tag = "4")]
    pub metric: ::prost::alloc::string::String,
    /// The limit unit of the limit to which this policy applies.
    ///
    /// An example unit would be:
    /// `1/{project}/{region}`
    /// Note that `{project}` and `{region}` are not placeholders in this example;
    /// the literal characters `{` and `}` occur in the string.
    #[prost(string, tag = "5")]
    pub unit: ::prost::alloc::string::String,
    /// The cloud resource container at which the quota policy is created. The
    /// format is `{container_type}/{container_number}`
    #[prost(string, tag = "6")]
    pub container: ::prost::alloc::string::String,
}
/// Service identity for a service. This is the identity that service producer
/// should use to access consumer resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceIdentity {
    /// The email address of the service account that a service producer would use
    /// to access consumer resources.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    /// The unique and stable id of the service account.
    /// <https://cloud.google.com/iam/reference/rest/v1/projects.serviceAccounts#ServiceAccount>
    #[prost(string, tag = "2")]
    pub unique_id: ::prost::alloc::string::String,
}
/// Whether or not a service has been enabled for use by a consumer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// The default value, which indicates that the enabled state of the service
    /// is unspecified or not meaningful. Currently, all consumers other than
    /// projects (such as folders and organizations) are always in this state.
    Unspecified = 0,
    /// The service cannot be used by this consumer. It has either been explicitly
    /// disabled, or has never been enabled.
    Disabled = 1,
    /// The service has been explicitly enabled for use by this consumer.
    Enabled = 2,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::Unspecified => "STATE_UNSPECIFIED",
            State::Disabled => "DISABLED",
            State::Enabled => "ENABLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "DISABLED" => Some(Self::Disabled),
            "ENABLED" => Some(Self::Enabled),
            _ => None,
        }
    }
}
/// Selected view of quota. Can be used to request more detailed quota
/// information when retrieving quota metrics and limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QuotaView {
    /// No quota view specified. Requests that do not specify a quota view will
    /// typically default to the BASIC view.
    Unspecified = 0,
    /// Only buckets with overrides are shown in the response.
    Basic = 1,
    /// Include per-location buckets even if they do not have overrides.
    /// When the view is FULL, and a limit has regional or zonal quota, the limit
    /// will include buckets for all regions or zones that could support
    /// overrides, even if none are currently present. In some cases this will
    /// cause the response to become very large; callers that do not need this
    /// extra information should use the BASIC view instead.
    Full = 2,
}
impl QuotaView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            QuotaView::Unspecified => "QUOTA_VIEW_UNSPECIFIED",
            QuotaView::Basic => "BASIC",
            QuotaView::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "QUOTA_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "BASIC" => Some(Self::Basic),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Enumerations of quota safety checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QuotaSafetyCheck {
    /// Unspecified quota safety check.
    Unspecified = 0,
    /// Validates that a quota mutation would not cause the consumer's effective
    /// limit to be lower than the consumer's quota usage.
    LimitDecreaseBelowUsage = 1,
    /// Validates that a quota mutation would not cause the consumer's effective
    /// limit to decrease by more than 10 percent.
    LimitDecreasePercentageTooHigh = 2,
}
impl QuotaSafetyCheck {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            QuotaSafetyCheck::Unspecified => "QUOTA_SAFETY_CHECK_UNSPECIFIED",
            QuotaSafetyCheck::LimitDecreaseBelowUsage => "LIMIT_DECREASE_BELOW_USAGE",
            QuotaSafetyCheck::LimitDecreasePercentageTooHigh => {
                "LIMIT_DECREASE_PERCENTAGE_TOO_HIGH"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "QUOTA_SAFETY_CHECK_UNSPECIFIED" => Some(Self::Unspecified),
            "LIMIT_DECREASE_BELOW_USAGE" => Some(Self::LimitDecreaseBelowUsage),
            "LIMIT_DECREASE_PERCENTAGE_TOO_HIGH" => {
                Some(Self::LimitDecreasePercentageTooHigh)
            }
            _ => None,
        }
    }
}
/// Request message for the `EnableService` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableServiceRequest {
    /// Name of the consumer and service to enable the service on.
    ///
    /// The `EnableService` and `DisableService` methods currently only support
    /// projects.
    ///
    /// Enabling a service requires that the service is public or is shared with
    /// the user enabling the service.
    ///
    /// An example name would be:
    /// `projects/123/services/serviceusage.googleapis.com`
    /// where `123` is the project number (not project ID).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the `DisableService` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableServiceRequest {
    /// Name of the consumer and service to disable the service on.
    ///
    /// The enable and disable methods currently only support projects.
    ///
    /// An example name would be:
    /// `projects/123/services/serviceusage.googleapis.com`
    /// where `123` is the project number (not project ID).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the `GetService` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Name of the consumer and service to get the `ConsumerState` for.
    ///
    /// An example name would be:
    /// `projects/123/services/serviceusage.googleapis.com`
    /// where `123` is the project number (not project ID).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the `ListServices` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Parent to search for services on.
    ///
    /// An example name would be:
    /// `projects/123`
    /// where `123` is the project number (not project ID).
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested size of the next page of data.
    /// Requested page size cannot exceed 200.
    ///   If not set, the default page size is 50.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token identifying which result to start with, which is returned by a
    /// previous list call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Only list services that conform to the given filter.
    /// The allowed filter strings are `state:ENABLED` and `state:DISABLED`.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for the `ListServices` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The available services for the requested project.
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// Token that can be passed to `ListServices` to resume a paginated
    /// query.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the `BatchEnableServices` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchEnableServicesRequest {
    /// Parent to enable services on.
    ///
    /// An example name would be:
    /// `projects/123`
    /// where `123` is the project number (not project ID).
    ///
    /// The `BatchEnableServices` method currently only supports projects.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The identifiers of the services to enable on the project.
    ///
    /// A valid identifier would be:
    /// serviceusage.googleapis.com
    ///
    /// Enabling services requires that each service is public or is shared with
    /// the user enabling the service.
    ///
    /// Two or more services must be specified. To enable a single service,
    /// use the `EnableService` method instead.
    ///
    /// A single request can enable a maximum of 20 services at a time. If more
    /// than 20 services are specified, the request will fail, and no state changes
    /// will occur.
    #[prost(string, repeated, tag = "2")]
    pub service_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for ListConsumerQuotaMetrics
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConsumerQuotaMetricsRequest {
    /// Parent of the quotas resource.
    ///
    /// Some example names would be:
    /// `projects/123/services/serviceconsumermanagement.googleapis.com`
    /// `folders/345/services/serviceconsumermanagement.googleapis.com`
    /// `organizations/456/services/serviceconsumermanagement.googleapis.com`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested size of the next page of data.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token identifying which result to start with; returned by a previous list
    /// call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Specifies the level of detail for quota information in the response.
    #[prost(enumeration = "QuotaView", tag = "4")]
    pub view: i32,
}
/// Response message for ListConsumerQuotaMetrics
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConsumerQuotaMetricsResponse {
    /// Quota settings for the consumer, organized by quota metric.
    #[prost(message, repeated, tag = "1")]
    pub metrics: ::prost::alloc::vec::Vec<ConsumerQuotaMetric>,
    /// Token identifying which result to start with; returned by a previous list
    /// call.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetConsumerQuotaMetric
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConsumerQuotaMetricRequest {
    /// The resource name of the quota limit.
    ///
    /// An example name would be:
    /// `projects/123/services/serviceusage.googleapis.com/quotas/metrics/serviceusage.googleapis.com%2Fmutate_requests`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Specifies the level of detail for quota information in the response.
    #[prost(enumeration = "QuotaView", tag = "2")]
    pub view: i32,
}
/// Request message for GetConsumerQuotaLimit
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConsumerQuotaLimitRequest {
    /// The resource name of the quota limit.
    ///
    /// Use the quota limit resource name returned by previous
    /// ListConsumerQuotaMetrics and GetConsumerQuotaMetric API calls.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Specifies the level of detail for quota information in the response.
    #[prost(enumeration = "QuotaView", tag = "2")]
    pub view: i32,
}
/// Request message for CreateAdminOverride.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAdminOverrideRequest {
    /// The resource name of the parent quota limit, returned by a
    /// ListConsumerQuotaMetrics or GetConsumerQuotaMetric call.
    ///
    /// An example name would be:
    /// `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The admin override to create.
    #[prost(message, optional, tag = "2")]
    pub r#override: ::core::option::Option<QuotaOverride>,
    /// Whether to force the creation of the quota override.
    /// Setting the force parameter to 'true' ignores all quota safety checks that
    /// would fail the request. QuotaSafetyCheck lists all such validations.
    #[prost(bool, tag = "3")]
    pub force: bool,
    /// The list of quota safety checks to ignore before the override mutation.
    /// Unlike 'force' field that ignores all the quota safety checks, the
    /// 'force_only' field ignores only the specified checks; other checks are
    /// still enforced. The 'force' and 'force_only' fields cannot both be set.
    #[prost(enumeration = "QuotaSafetyCheck", repeated, tag = "4")]
    pub force_only: ::prost::alloc::vec::Vec<i32>,
}
/// Request message for UpdateAdminOverride.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAdminOverrideRequest {
    /// The resource name of the override to update.
    ///
    /// An example name would be:
    /// `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/adminOverrides/4a3f2c1d`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The new override.
    /// Only the override_value is updated; all other fields are ignored.
    #[prost(message, optional, tag = "2")]
    pub r#override: ::core::option::Option<QuotaOverride>,
    /// Whether to force the update of the quota override.
    /// Setting the force parameter to 'true' ignores all quota safety checks that
    /// would fail the request. QuotaSafetyCheck lists all such validations.
    #[prost(bool, tag = "3")]
    pub force: bool,
    /// Update only the specified fields of the override.
    /// If unset, all fields will be updated.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The list of quota safety checks to ignore before the override mutation.
    /// Unlike 'force' field that ignores all the quota safety checks, the
    /// 'force_only' field ignores only the specified checks; other checks are
    /// still enforced. The 'force' and 'force_only' fields cannot both be set.
    #[prost(enumeration = "QuotaSafetyCheck", repeated, tag = "5")]
    pub force_only: ::prost::alloc::vec::Vec<i32>,
}
/// Request message for DeleteAdminOverride.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAdminOverrideRequest {
    /// The resource name of the override to delete.
    ///
    /// An example name would be:
    /// `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/adminOverrides/4a3f2c1d`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Whether to force the deletion of the quota override.
    /// Setting the force parameter to 'true' ignores all quota safety checks that
    /// would fail the request. QuotaSafetyCheck lists all such validations.
    #[prost(bool, tag = "2")]
    pub force: bool,
    /// The list of quota safety checks to ignore before the override mutation.
    /// Unlike 'force' field that ignores all the quota safety checks, the
    /// 'force_only' field ignores only the specified checks; other checks are
    /// still enforced. The 'force' and 'force_only' fields cannot both be set.
    #[prost(enumeration = "QuotaSafetyCheck", repeated, tag = "3")]
    pub force_only: ::prost::alloc::vec::Vec<i32>,
}
/// Request message for ListAdminOverrides
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAdminOverridesRequest {
    /// The resource name of the parent quota limit, returned by a
    /// ListConsumerQuotaMetrics or GetConsumerQuotaMetric call.
    ///
    /// An example name would be:
    /// `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested size of the next page of data.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token identifying which result to start with; returned by a previous list
    /// call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListAdminOverrides.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAdminOverridesResponse {
    /// Admin overrides on this limit.
    #[prost(message, repeated, tag = "1")]
    pub overrides: ::prost::alloc::vec::Vec<QuotaOverride>,
    /// Token identifying which result to start with; returned by a previous list
    /// call.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Response message for BatchCreateAdminOverrides
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateAdminOverridesResponse {
    /// The overrides that were created.
    #[prost(message, repeated, tag = "1")]
    pub overrides: ::prost::alloc::vec::Vec<QuotaOverride>,
}
/// Request message for ImportAdminOverrides
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAdminOverridesRequest {
    /// The resource name of the consumer.
    ///
    /// An example name would be:
    /// `projects/123/services/compute.googleapis.com`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Whether to force the creation of the quota overrides.
    /// Setting the force parameter to 'true' ignores all quota safety checks that
    /// would fail the request. QuotaSafetyCheck lists all such validations.
    #[prost(bool, tag = "3")]
    pub force: bool,
    /// The list of quota safety checks to ignore before the override mutation.
    /// Unlike 'force' field that ignores all the quota safety checks, the
    /// 'force_only' field ignores only the specified checks; other checks are
    /// still enforced. The 'force' and 'force_only' fields cannot both be set.
    #[prost(enumeration = "QuotaSafetyCheck", repeated, tag = "4")]
    pub force_only: ::prost::alloc::vec::Vec<i32>,
    /// Source of import data
    #[prost(oneof = "import_admin_overrides_request::Source", tags = "2")]
    pub source: ::core::option::Option<import_admin_overrides_request::Source>,
}
/// Nested message and enum types in `ImportAdminOverridesRequest`.
pub mod import_admin_overrides_request {
    /// Source of import data
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The import data is specified in the request message itself
        #[prost(message, tag = "2")]
        InlineSource(super::OverrideInlineSource),
    }
}
/// Response message for ImportAdminOverrides
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAdminOverridesResponse {
    /// The overrides that were created from the imported data.
    #[prost(message, repeated, tag = "1")]
    pub overrides: ::prost::alloc::vec::Vec<QuotaOverride>,
}
/// Metadata message that provides information such as progress,
/// partial failures, and similar information on each GetOperation call
/// of LRO returned by ImportAdminOverrides.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAdminOverridesMetadata {}
/// Request message for CreateConsumerOverride.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConsumerOverrideRequest {
    /// The resource name of the parent quota limit, returned by a
    /// ListConsumerQuotaMetrics or GetConsumerQuotaMetric call.
    ///
    /// An example name would be:
    /// `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The override to create.
    #[prost(message, optional, tag = "2")]
    pub r#override: ::core::option::Option<QuotaOverride>,
    /// Whether to force the creation of the quota override.
    /// Setting the force parameter to 'true' ignores all quota safety checks that
    /// would fail the request. QuotaSafetyCheck lists all such validations.
    #[prost(bool, tag = "3")]
    pub force: bool,
    /// The list of quota safety checks to ignore before the override mutation.
    /// Unlike 'force' field that ignores all the quota safety checks, the
    /// 'force_only' field ignores only the specified checks; other checks are
    /// still enforced. The 'force' and 'force_only' fields cannot both be set.
    #[prost(enumeration = "QuotaSafetyCheck", repeated, tag = "4")]
    pub force_only: ::prost::alloc::vec::Vec<i32>,
}
/// Request message for UpdateConsumerOverride.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConsumerOverrideRequest {
    /// The resource name of the override to update.
    ///
    /// An example name would be:
    /// `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/consumerOverrides/4a3f2c1d`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The new override.
    /// Only the override_value is updated; all other fields are ignored.
    #[prost(message, optional, tag = "2")]
    pub r#override: ::core::option::Option<QuotaOverride>,
    /// Whether to force the update of the quota override.
    /// Setting the force parameter to 'true' ignores all quota safety checks that
    /// would fail the request. QuotaSafetyCheck lists all such validations.
    #[prost(bool, tag = "3")]
    pub force: bool,
    /// Update only the specified fields of the override.
    /// If unset, all fields will be updated.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The list of quota safety checks to ignore before the override mutation.
    /// Unlike 'force' field that ignores all the quota safety checks, the
    /// 'force_only' field ignores only the specified checks; other checks are
    /// still enforced. The 'force' and 'force_only' fields cannot both be set.
    #[prost(enumeration = "QuotaSafetyCheck", repeated, tag = "5")]
    pub force_only: ::prost::alloc::vec::Vec<i32>,
}
/// Request message for DeleteConsumerOverride.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConsumerOverrideRequest {
    /// The resource name of the override to delete.
    ///
    /// An example name would be:
    /// `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/consumerOverrides/4a3f2c1d`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Whether to force the deletion of the quota override.
    /// Setting the force parameter to 'true' ignores all quota safety checks that
    /// would fail the request. QuotaSafetyCheck lists all such validations.
    #[prost(bool, tag = "2")]
    pub force: bool,
    /// The list of quota safety checks to ignore before the override mutation.
    /// Unlike 'force' field that ignores all the quota safety checks, the
    /// 'force_only' field ignores only the specified checks; other checks are
    /// still enforced. The 'force' and 'force_only' fields cannot both be set.
    #[prost(enumeration = "QuotaSafetyCheck", repeated, tag = "3")]
    pub force_only: ::prost::alloc::vec::Vec<i32>,
}
/// Request message for ListConsumerOverrides
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConsumerOverridesRequest {
    /// The resource name of the parent quota limit, returned by a
    /// ListConsumerQuotaMetrics or GetConsumerQuotaMetric call.
    ///
    /// An example name would be:
    /// `projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested size of the next page of data.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token identifying which result to start with; returned by a previous list
    /// call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListConsumerOverrides.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConsumerOverridesResponse {
    /// Consumer overrides on this limit.
    #[prost(message, repeated, tag = "1")]
    pub overrides: ::prost::alloc::vec::Vec<QuotaOverride>,
    /// Token identifying which result to start with; returned by a previous list
    /// call.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Response message for BatchCreateConsumerOverrides
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateConsumerOverridesResponse {
    /// The overrides that were created.
    #[prost(message, repeated, tag = "1")]
    pub overrides: ::prost::alloc::vec::Vec<QuotaOverride>,
}
/// Request message for ImportConsumerOverrides
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportConsumerOverridesRequest {
    /// The resource name of the consumer.
    ///
    /// An example name would be:
    /// `projects/123/services/compute.googleapis.com`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Whether to force the creation of the quota overrides.
    /// Setting the force parameter to 'true' ignores all quota safety checks that
    /// would fail the request. QuotaSafetyCheck lists all such validations.
    #[prost(bool, tag = "3")]
    pub force: bool,
    /// The list of quota safety checks to ignore before the override mutation.
    /// Unlike 'force' field that ignores all the quota safety checks, the
    /// 'force_only' field ignores only the specified checks; other checks are
    /// still enforced. The 'force' and 'force_only' fields cannot both be set.
    #[prost(enumeration = "QuotaSafetyCheck", repeated, tag = "4")]
    pub force_only: ::prost::alloc::vec::Vec<i32>,
    /// Source of import data
    #[prost(oneof = "import_consumer_overrides_request::Source", tags = "2")]
    pub source: ::core::option::Option<import_consumer_overrides_request::Source>,
}
/// Nested message and enum types in `ImportConsumerOverridesRequest`.
pub mod import_consumer_overrides_request {
    /// Source of import data
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The import data is specified in the request message itself
        #[prost(message, tag = "2")]
        InlineSource(super::OverrideInlineSource),
    }
}
/// Response message for ImportConsumerOverrides
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportConsumerOverridesResponse {
    /// The overrides that were created from the imported data.
    #[prost(message, repeated, tag = "1")]
    pub overrides: ::prost::alloc::vec::Vec<QuotaOverride>,
}
/// Metadata message that provides information such as progress,
/// partial failures, and similar information on each GetOperation call
/// of LRO returned by ImportConsumerOverrides.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportConsumerOverridesMetadata {}
/// Response message for ImportAdminQuotaPolicies
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAdminQuotaPoliciesResponse {
    /// The policies that were created from the imported data.
    #[prost(message, repeated, tag = "1")]
    pub policies: ::prost::alloc::vec::Vec<AdminQuotaPolicy>,
}
/// Metadata message that provides information such as progress,
/// partial failures, and similar information on each GetOperation call
/// of LRO returned by ImportAdminQuotaPolicies.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAdminQuotaPoliciesMetadata {}
/// Metadata message that provides information such as progress,
/// partial failures, and similar information on each GetOperation call
/// of LRO returned by CreateAdminQuotaPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAdminQuotaPolicyMetadata {}
/// Metadata message that provides information such as progress,
/// partial failures, and similar information on each GetOperation call
/// of LRO returned by UpdateAdminQuotaPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAdminQuotaPolicyMetadata {}
/// Metadata message that provides information such as progress,
/// partial failures, and similar information on each GetOperation call
/// of LRO returned by DeleteAdminQuotaPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAdminQuotaPolicyMetadata {}
/// Request message for generating service identity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateServiceIdentityRequest {
    /// Name of the consumer and service to generate an identity for.
    ///
    /// The `GenerateServiceIdentity` methods currently support projects, folders,
    /// organizations.
    ///
    /// Example parents would be:
    /// `projects/123/services/example.googleapis.com`
    /// `folders/123/services/example.googleapis.com`
    /// `organizations/123/services/example.googleapis.com`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// Response message for getting service identity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceIdentityResponse {
    /// Service identity that service producer can use to access consumer
    /// resources. If exists is true, it contains email and unique_id. If exists is
    /// false, it contains pre-constructed email and empty unique_id.
    #[prost(message, optional, tag = "1")]
    pub identity: ::core::option::Option<ServiceIdentity>,
    /// Service identity state.
    #[prost(enumeration = "get_service_identity_response::IdentityState", tag = "2")]
    pub state: i32,
}
/// Nested message and enum types in `GetServiceIdentityResponse`.
pub mod get_service_identity_response {
    /// Enum for service identity state.
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
    pub enum IdentityState {
        /// Default service identity state. This value is used if the state is
        /// omitted.
        Unspecified = 0,
        /// Service identity has been created and can be used.
        Active = 1,
    }
    impl IdentityState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IdentityState::Unspecified => "IDENTITY_STATE_UNSPECIFIED",
                IdentityState::Active => "ACTIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IDENTITY_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                _ => None,
            }
        }
    }
}
/// Metadata for the `GetServiceIdentity` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceIdentityMetadata {}
/// Generated client implementations.
pub mod service_usage_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// [Service Usage API](https://cloud.google.com/service-usage/docs/overview)
    #[derive(Debug, Clone)]
    pub struct ServiceUsageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ServiceUsageClient<T>
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
        ) -> ServiceUsageClient<InterceptedService<T, F>>
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
            ServiceUsageClient::new(InterceptedService::new(inner, interceptor))
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
        /// Enables a service so that it can be used with a project.
        ///
        /// Operation response type: `google.protobuf.Empty`
        pub async fn enable_service(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableServiceRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/EnableService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "EnableService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Disables a service so that it can no longer be used with a project.
        /// This prevents unintended usage that may cause unexpected billing
        /// charges or security leaks.
        ///
        /// It is not valid to call the disable method on a service that is not
        /// currently enabled. Callers will receive a `FAILED_PRECONDITION` status if
        /// the target service is not currently enabled.
        ///
        /// Operation response type: `google.protobuf.Empty`
        pub async fn disable_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableServiceRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/DisableService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "DisableService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the service configuration and enabled state for a given service.
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/GetService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "GetService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all services available to the specified project, and the current
        /// state of those services with respect to the project. The list includes
        /// all public services, all services for which the calling user has the
        /// `servicemanagement.services.bind` permission, and all services that have
        /// already been enabled on the project. The list can be filtered to
        /// only include services in a specific state, for example to only include
        /// services enabled on the project.
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ListServices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "ListServices",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Enables multiple services on a project. The operation is atomic: if
        /// enabling any service fails, then the entire batch fails, and no state
        /// changes occur.
        ///
        /// Operation response type: `google.protobuf.Empty`
        pub async fn batch_enable_services(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchEnableServicesRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/BatchEnableServices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "BatchEnableServices",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a summary of all quota information visible to the service
        /// consumer, organized by service metric. Each metric includes information
        /// about all of its defined limits. Each limit includes the limit
        /// configuration (quota unit, preciseness, default value), the current
        /// effective limit value, and all of the overrides applied to the limit.
        pub async fn list_consumer_quota_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConsumerQuotaMetricsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConsumerQuotaMetricsResponse>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ListConsumerQuotaMetrics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "ListConsumerQuotaMetrics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a summary of quota information for a specific quota metric
        pub async fn get_consumer_quota_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConsumerQuotaMetricRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConsumerQuotaMetric>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/GetConsumerQuotaMetric",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "GetConsumerQuotaMetric",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a summary of quota information for a specific quota limit.
        pub async fn get_consumer_quota_limit(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConsumerQuotaLimitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConsumerQuotaLimit>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/GetConsumerQuotaLimit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "GetConsumerQuotaLimit",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an admin override.
        /// An admin override is applied by an administrator of a parent folder or
        /// parent organization of the consumer receiving the override. An admin
        /// override is intended to limit the amount of quota the consumer can use out
        /// of the total quota pool allocated to all children of the folder or
        /// organization.
        pub async fn create_admin_override(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAdminOverrideRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/CreateAdminOverride",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "CreateAdminOverride",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an admin override.
        pub async fn update_admin_override(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAdminOverrideRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/UpdateAdminOverride",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "UpdateAdminOverride",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an admin override.
        pub async fn delete_admin_override(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAdminOverrideRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/DeleteAdminOverride",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "DeleteAdminOverride",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all admin overrides on this limit.
        pub async fn list_admin_overrides(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAdminOverridesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAdminOverridesResponse>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ListAdminOverrides",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "ListAdminOverrides",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates or updates multiple admin overrides atomically, all on the
        /// same consumer, but on many different metrics or limits.
        /// The name field in the quota override message should not be set.
        pub async fn import_admin_overrides(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportAdminOverridesRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ImportAdminOverrides",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "ImportAdminOverrides",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a consumer override.
        /// A consumer override is applied to the consumer on its own authority to
        /// limit its own quota usage. Consumer overrides cannot be used to grant more
        /// quota than would be allowed by admin overrides, producer overrides, or the
        /// default limit of the service.
        pub async fn create_consumer_override(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConsumerOverrideRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/CreateConsumerOverride",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "CreateConsumerOverride",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a consumer override.
        pub async fn update_consumer_override(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConsumerOverrideRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/UpdateConsumerOverride",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "UpdateConsumerOverride",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a consumer override.
        pub async fn delete_consumer_override(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConsumerOverrideRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/DeleteConsumerOverride",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "DeleteConsumerOverride",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all consumer overrides on this limit.
        pub async fn list_consumer_overrides(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConsumerOverridesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConsumerOverridesResponse>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ListConsumerOverrides",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "ListConsumerOverrides",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates or updates multiple consumer overrides atomically, all on the
        /// same consumer, but on many different metrics or limits.
        /// The name field in the quota override message should not be set.
        pub async fn import_consumer_overrides(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportConsumerOverridesRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ImportConsumerOverrides",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "ImportConsumerOverrides",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates service identity for service.
        pub async fn generate_service_identity(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateServiceIdentityRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/GenerateServiceIdentity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.serviceusage.v1beta1.ServiceUsage",
                        "GenerateServiceIdentity",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
