/// A `Service` is a discrete, autonomous, and network-accessible unit, designed
/// to solve an individual concern
/// ([Wikipedia](<https://en.wikipedia.org/wiki/Service-orientation>)). In
/// Cloud Monitoring, a `Service` acts as the root resource under which
/// operational aspects of the service are accessible.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// Resource name for this Service. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/services/\[SERVICE_ID\]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Name used for UI elements listing this Service.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Configuration for how to query telemetry on a Service.
    #[prost(message, optional, tag = "13")]
    pub telemetry: ::core::option::Option<service::Telemetry>,
    /// Labels which have been used to annotate the service. Label keys must start
    /// with a letter. Label keys and values may contain lowercase letters,
    /// numbers, underscores, and dashes. Label keys and values have a maximum
    /// length of 63 characters, and must be less than 128 bytes in size. Up to 64
    /// label entries may be stored. For labels which do not have a semantic value,
    /// the empty string may be supplied for the label value.
    #[prost(btree_map = "string, string", tag = "14")]
    pub user_labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// REQUIRED. Service-identifying atoms specifying the underlying service.
    #[prost(oneof = "service::Identifier", tags = "6, 7, 8, 9, 10, 11")]
    pub identifier: ::core::option::Option<service::Identifier>,
}
/// Nested message and enum types in `Service`.
pub mod service {
    /// Custom view of service telemetry. Currently a place-holder pending final
    /// design.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Custom {}
    /// App Engine service. Learn more at <https://cloud.google.com/appengine.>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppEngine {
        /// The ID of the App Engine module underlying this service. Corresponds to
        /// the `module_id` resource label in the `gae_app` monitored resource:
        /// <https://cloud.google.com/monitoring/api/resources#tag_gae_app>
        #[prost(string, tag = "1")]
        pub module_id: ::prost::alloc::string::String,
    }
    /// Cloud Endpoints service. Learn more at <https://cloud.google.com/endpoints.>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CloudEndpoints {
        /// The name of the Cloud Endpoints service underlying this service.
        /// Corresponds to the `service` resource label in the `api` monitored
        /// resource: <https://cloud.google.com/monitoring/api/resources#tag_api>
        #[prost(string, tag = "1")]
        pub service: ::prost::alloc::string::String,
    }
    /// Istio service scoped to a single Kubernetes cluster. Learn more at
    /// <https://istio.io.> Clusters running OSS Istio will have their services
    /// ingested as this type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusterIstio {
        /// The location of the Kubernetes cluster in which this Istio service is
        /// defined. Corresponds to the `location` resource label in `k8s_cluster`
        /// resources.
        #[prost(string, tag = "1")]
        pub location: ::prost::alloc::string::String,
        /// The name of the Kubernetes cluster in which this Istio service is
        /// defined. Corresponds to the `cluster_name` resource label in
        /// `k8s_cluster` resources.
        #[prost(string, tag = "2")]
        pub cluster_name: ::prost::alloc::string::String,
        /// The namespace of the Istio service underlying this service. Corresponds
        /// to the `destination_service_namespace` metric label in Istio metrics.
        #[prost(string, tag = "3")]
        pub service_namespace: ::prost::alloc::string::String,
        /// The name of the Istio service underlying this service. Corresponds to the
        /// `destination_service_name` metric label in Istio metrics.
        #[prost(string, tag = "4")]
        pub service_name: ::prost::alloc::string::String,
    }
    /// Istio service scoped to an Istio mesh. Anthos clusters running ASM < 1.6.8
    /// will have their services ingested as this type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MeshIstio {
        /// Identifier for the mesh in which this Istio service is defined.
        /// Corresponds to the `mesh_uid` metric label in Istio metrics.
        #[prost(string, tag = "1")]
        pub mesh_uid: ::prost::alloc::string::String,
        /// The namespace of the Istio service underlying this service. Corresponds
        /// to the `destination_service_namespace` metric label in Istio metrics.
        #[prost(string, tag = "3")]
        pub service_namespace: ::prost::alloc::string::String,
        /// The name of the Istio service underlying this service. Corresponds to the
        /// `destination_service_name` metric label in Istio metrics.
        #[prost(string, tag = "4")]
        pub service_name: ::prost::alloc::string::String,
    }
    /// Canonical service scoped to an Istio mesh. Anthos clusters running ASM >=
    /// 1.6.8 will have their services ingested as this type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IstioCanonicalService {
        /// Identifier for the Istio mesh in which this canonical service is defined.
        /// Corresponds to the `mesh_uid` metric label in
        /// [Istio metrics](<https://cloud.google.com/monitoring/api/metrics_istio>).
        #[prost(string, tag = "1")]
        pub mesh_uid: ::prost::alloc::string::String,
        /// The namespace of the canonical service underlying this service.
        /// Corresponds to the `destination_canonical_service_namespace` metric
        /// label in [Istio
        /// metrics](<https://cloud.google.com/monitoring/api/metrics_istio>).
        #[prost(string, tag = "3")]
        pub canonical_service_namespace: ::prost::alloc::string::String,
        /// The name of the canonical service underlying this service.
        /// Corresponds to the `destination_canonical_service_name` metric label in
        /// label in [Istio
        /// metrics](<https://cloud.google.com/monitoring/api/metrics_istio>).
        #[prost(string, tag = "4")]
        pub canonical_service: ::prost::alloc::string::String,
    }
    /// Configuration for how to query telemetry on a Service.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Telemetry {
        /// The full name of the resource that defines this service. Formatted as
        /// described in <https://cloud.google.com/apis/design/resource_names.>
        #[prost(string, tag = "1")]
        pub resource_name: ::prost::alloc::string::String,
    }
    /// REQUIRED. Service-identifying atoms specifying the underlying service.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Identifier {
        /// Custom service type.
        #[prost(message, tag = "6")]
        Custom(Custom),
        /// Type used for App Engine services.
        #[prost(message, tag = "7")]
        AppEngine(AppEngine),
        /// Type used for Cloud Endpoints services.
        #[prost(message, tag = "8")]
        CloudEndpoints(CloudEndpoints),
        /// Type used for Istio services that live in a Kubernetes cluster.
        #[prost(message, tag = "9")]
        ClusterIstio(ClusterIstio),
        /// Type used for Istio services scoped to an Istio mesh.
        #[prost(message, tag = "10")]
        MeshIstio(MeshIstio),
        /// Type used for canonical services scoped to an Istio mesh.
        /// Metrics for Istio are
        /// [documented here](<https://istio.io/latest/docs/reference/config/metrics/>)
        #[prost(message, tag = "11")]
        IstioCanonicalService(IstioCanonicalService),
    }
}
/// A Service-Level Objective (SLO) describes a level of desired good service. It
/// consists of a service-level indicator (SLI), a performance goal, and a period
/// over which the objective is to be evaluated against that goal. The SLO can
/// use SLIs defined in a number of different manners. Typical SLOs might include
/// "99% of requests in each rolling week have latency below 200 milliseconds" or
/// "99.5% of requests in each calendar month return successfully."
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceLevelObjective {
    /// Resource name for this `ServiceLevelObjective`. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/services/\[SERVICE_ID\]/serviceLevelObjectives/\[SLO_NAME\]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Name used for UI elements listing this SLO.
    #[prost(string, tag = "11")]
    pub display_name: ::prost::alloc::string::String,
    /// The definition of good service, used to measure and calculate the quality
    /// of the `Service`'s performance with respect to a single aspect of service
    /// quality.
    #[prost(message, optional, tag = "3")]
    pub service_level_indicator: ::core::option::Option<ServiceLevelIndicator>,
    /// The fraction of service that must be good in order for this objective to be
    /// met. `0 < goal <= 0.999`.
    #[prost(double, tag = "4")]
    pub goal: f64,
    /// Labels which have been used to annotate the service-level objective. Label
    /// keys must start with a letter. Label keys and values may contain lowercase
    /// letters, numbers, underscores, and dashes. Label keys and values have a
    /// maximum length of 63 characters, and must be less than 128 bytes in size.
    /// Up to 64 label entries may be stored. For labels which do not have a
    /// semantic value, the empty string may be supplied for the label value.
    #[prost(btree_map = "string, string", tag = "12")]
    pub user_labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The time period over which the objective will be evaluated.
    #[prost(oneof = "service_level_objective::Period", tags = "5, 6")]
    pub period: ::core::option::Option<service_level_objective::Period>,
}
/// Nested message and enum types in `ServiceLevelObjective`.
pub mod service_level_objective {
    /// `ServiceLevelObjective.View` determines what form of
    /// `ServiceLevelObjective` is returned from `GetServiceLevelObjective`,
    /// `ListServiceLevelObjectives`, and `ListServiceLevelObjectiveVersions` RPCs.
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
    pub enum View {
        /// Same as FULL.
        Unspecified = 0,
        /// Return the embedded `ServiceLevelIndicator` in the form in which it was
        /// defined. If it was defined using a `BasicSli`, return that `BasicSli`.
        Full = 2,
        /// For `ServiceLevelIndicator`s using `BasicSli` articulation, instead
        /// return the `ServiceLevelIndicator` with its mode of computation fully
        /// spelled out as a `RequestBasedSli`. For `ServiceLevelIndicator`s using
        /// `RequestBasedSli` or `WindowsBasedSli`, return the
        /// `ServiceLevelIndicator` as it was provided.
        Explicit = 1,
    }
    impl View {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                View::Unspecified => "VIEW_UNSPECIFIED",
                View::Full => "FULL",
                View::Explicit => "EXPLICIT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VIEW_UNSPECIFIED" => Some(Self::Unspecified),
                "FULL" => Some(Self::Full),
                "EXPLICIT" => Some(Self::Explicit),
                _ => None,
            }
        }
    }
    /// The time period over which the objective will be evaluated.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Period {
        /// A rolling time period, semantically "in the past `<rolling_period>`".
        /// Must be an integer multiple of 1 day no larger than 30 days.
        #[prost(message, tag = "5")]
        RollingPeriod(::prost_types::Duration),
        /// A calendar period, semantically "since the start of the current
        /// `<calendar_period>`". At this time, only `DAY`, `WEEK`, `FORTNIGHT`, and
        /// `MONTH` are supported.
        #[prost(enumeration = "super::super::super::r#type::CalendarPeriod", tag = "6")]
        CalendarPeriod(i32),
    }
}
/// A Service-Level Indicator (SLI) describes the "performance" of a service. For
/// some services, the SLI is well-defined. In such cases, the SLI can be
/// described easily by referencing the well-known SLI and providing the needed
/// parameters. Alternatively, a "custom" SLI can be defined with a query to the
/// underlying metric store. An SLI is defined to be `good_service /
/// total_service` over any queried time interval. The value of performance
/// always falls into the range `0 <= performance <= 1`. A custom SLI describes
/// how to compute this ratio, whether this is by dividing values from a pair of
/// time series, cutting a `Distribution` into good and bad counts, or counting
/// time windows in which the service complies with a criterion. For separation
/// of concerns, a single Service-Level Indicator measures performance for only
/// one aspect of service quality, such as fraction of successful queries or
/// fast-enough queries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceLevelIndicator {
    /// Service level indicators can be grouped by whether the "unit" of service
    /// being measured is based on counts of good requests or on counts of good
    /// time windows
    #[prost(oneof = "service_level_indicator::Type", tags = "4, 1, 2")]
    pub r#type: ::core::option::Option<service_level_indicator::Type>,
}
/// Nested message and enum types in `ServiceLevelIndicator`.
pub mod service_level_indicator {
    /// Service level indicators can be grouped by whether the "unit" of service
    /// being measured is based on counts of good requests or on counts of good
    /// time windows
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Basic SLI on a well-known service type.
        #[prost(message, tag = "4")]
        BasicSli(super::BasicSli),
        /// Request-based SLIs
        #[prost(message, tag = "1")]
        RequestBased(super::RequestBasedSli),
        /// Windows-based SLIs
        #[prost(message, tag = "2")]
        WindowsBased(super::WindowsBasedSli),
    }
}
/// An SLI measuring performance on a well-known service type. Performance will
/// be computed on the basis of pre-defined metrics. The type of the
/// `service_resource` determines the metrics to use and the
/// `service_resource.labels` and `metric_labels` are used to construct a
/// monitoring filter to filter that metric down to just the data relevant to
/// this service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicSli {
    /// OPTIONAL: The set of RPCs to which this SLI is relevant. Telemetry from
    /// other methods will not be used to calculate performance for this SLI. If
    /// omitted, this SLI applies to all the Service's methods. For service types
    /// that don't support breaking down by method, setting this field will result
    /// in an error.
    #[prost(string, repeated, tag = "7")]
    pub method: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// OPTIONAL: The set of locations to which this SLI is relevant. Telemetry
    /// from other locations will not be used to calculate performance for this
    /// SLI. If omitted, this SLI applies to all locations in which the Service has
    /// activity. For service types that don't support breaking down by location,
    /// setting this field will result in an error.
    #[prost(string, repeated, tag = "8")]
    pub location: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// OPTIONAL: The set of API versions to which this SLI is relevant. Telemetry
    /// from other API versions will not be used to calculate performance for this
    /// SLI. If omitted, this SLI applies to all API versions. For service types
    /// that don't support breaking down by version, setting this field will result
    /// in an error.
    #[prost(string, repeated, tag = "9")]
    pub version: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// This SLI can be evaluated on the basis of availability or latency.
    #[prost(oneof = "basic_sli::SliCriteria", tags = "2, 3")]
    pub sli_criteria: ::core::option::Option<basic_sli::SliCriteria>,
}
/// Nested message and enum types in `BasicSli`.
pub mod basic_sli {
    /// Future parameters for the availability SLI.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AvailabilityCriteria {}
    /// Parameters for a latency threshold SLI.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LatencyCriteria {
        /// Good service is defined to be the count of requests made to this service
        /// that return in no more than `threshold`.
        #[prost(message, optional, tag = "3")]
        pub threshold: ::core::option::Option<::prost_types::Duration>,
    }
    /// This SLI can be evaluated on the basis of availability or latency.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SliCriteria {
        /// Good service is defined to be the count of requests made to this service
        /// that return successfully.
        #[prost(message, tag = "2")]
        Availability(AvailabilityCriteria),
        /// Good service is defined to be the count of requests made to this service
        /// that are fast enough with respect to `latency.threshold`.
        #[prost(message, tag = "3")]
        Latency(LatencyCriteria),
    }
}
/// Range of numerical values within `min` and `max`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    /// Range minimum.
    #[prost(double, tag = "1")]
    pub min: f64,
    /// Range maximum.
    #[prost(double, tag = "2")]
    pub max: f64,
}
/// Service Level Indicators for which atomic units of service are counted
/// directly.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBasedSli {
    /// The means to compute a ratio of `good_service` to `total_service`.
    #[prost(oneof = "request_based_sli::Method", tags = "1, 3")]
    pub method: ::core::option::Option<request_based_sli::Method>,
}
/// Nested message and enum types in `RequestBasedSli`.
pub mod request_based_sli {
    /// The means to compute a ratio of `good_service` to `total_service`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        /// `good_total_ratio` is used when the ratio of `good_service` to
        /// `total_service` is computed from two `TimeSeries`.
        #[prost(message, tag = "1")]
        GoodTotalRatio(super::TimeSeriesRatio),
        /// `distribution_cut` is used when `good_service` is a count of values
        /// aggregated in a `Distribution` that fall into a good range. The
        /// `total_service` is the total count of all values aggregated in the
        /// `Distribution`.
        #[prost(message, tag = "3")]
        DistributionCut(super::DistributionCut),
    }
}
/// A `TimeSeriesRatio` specifies two `TimeSeries` to use for computing the
/// `good_service / total_service` ratio. The specified `TimeSeries` must have
/// `ValueType = DOUBLE` or `ValueType = INT64` and must have `MetricKind =
/// DELTA` or `MetricKind = CUMULATIVE`. The `TimeSeriesRatio` must specify
/// exactly two of good, bad, and total, and the relationship `good_service +
/// bad_service = total_service` will be assumed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesRatio {
    /// A [monitoring filter](<https://cloud.google.com/monitoring/api/v3/filters>)
    /// specifying a `TimeSeries` quantifying good service provided. Must have
    /// `ValueType = DOUBLE` or `ValueType = INT64` and must have `MetricKind =
    /// DELTA` or `MetricKind = CUMULATIVE`.
    #[prost(string, tag = "4")]
    pub good_service_filter: ::prost::alloc::string::String,
    /// A [monitoring filter](<https://cloud.google.com/monitoring/api/v3/filters>)
    /// specifying a `TimeSeries` quantifying bad service, either demanded service
    /// that was not provided or demanded service that was of inadequate quality.
    /// Must have `ValueType = DOUBLE` or `ValueType = INT64` and must have
    /// `MetricKind = DELTA` or `MetricKind = CUMULATIVE`.
    #[prost(string, tag = "5")]
    pub bad_service_filter: ::prost::alloc::string::String,
    /// A [monitoring filter](<https://cloud.google.com/monitoring/api/v3/filters>)
    /// specifying a `TimeSeries` quantifying total demanded service. Must have
    /// `ValueType = DOUBLE` or `ValueType = INT64` and must have `MetricKind =
    /// DELTA` or `MetricKind = CUMULATIVE`.
    #[prost(string, tag = "6")]
    pub total_service_filter: ::prost::alloc::string::String,
}
/// A `DistributionCut` defines a `TimeSeries` and thresholds used for measuring
/// good service and total service. The `TimeSeries` must have `ValueType =
/// DISTRIBUTION` and `MetricKind = DELTA` or `MetricKind = CUMULATIVE`. The
/// computed `good_service` will be the estimated count of values in the
/// `Distribution` that fall within the specified `min` and `max`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistributionCut {
    /// A [monitoring filter](<https://cloud.google.com/monitoring/api/v3/filters>)
    /// specifying a `TimeSeries` aggregating values. Must have `ValueType =
    /// DISTRIBUTION` and `MetricKind = DELTA` or `MetricKind = CUMULATIVE`.
    #[prost(string, tag = "4")]
    pub distribution_filter: ::prost::alloc::string::String,
    /// Range of values considered "good." For a one-sided range, set one bound to
    /// an infinite value.
    #[prost(message, optional, tag = "5")]
    pub range: ::core::option::Option<Range>,
}
/// A `WindowsBasedSli` defines `good_service` as the count of time windows for
/// which the provided service was of good quality. Criteria for determining
/// if service was good are embedded in the `window_criterion`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowsBasedSli {
    /// Duration over which window quality is evaluated. Must be an integer
    /// fraction of a day and at least `60s`.
    #[prost(message, optional, tag = "4")]
    pub window_period: ::core::option::Option<::prost_types::Duration>,
    /// The criterion to use for evaluating window goodness.
    #[prost(oneof = "windows_based_sli::WindowCriterion", tags = "5, 2, 6, 7")]
    pub window_criterion: ::core::option::Option<windows_based_sli::WindowCriterion>,
}
/// Nested message and enum types in `WindowsBasedSli`.
pub mod windows_based_sli {
    /// A `PerformanceThreshold` is used when each window is good when that window
    /// has a sufficiently high `performance`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PerformanceThreshold {
        /// If window `performance >= threshold`, the window is counted as good.
        #[prost(double, tag = "2")]
        pub threshold: f64,
        /// The means, either a request-based SLI or a basic SLI, by which to compute
        /// performance over a window.
        #[prost(oneof = "performance_threshold::Type", tags = "1, 3")]
        pub r#type: ::core::option::Option<performance_threshold::Type>,
    }
    /// Nested message and enum types in `PerformanceThreshold`.
    pub mod performance_threshold {
        /// The means, either a request-based SLI or a basic SLI, by which to compute
        /// performance over a window.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// `RequestBasedSli` to evaluate to judge window quality.
            #[prost(message, tag = "1")]
            Performance(super::super::RequestBasedSli),
            /// `BasicSli` to evaluate to judge window quality.
            #[prost(message, tag = "3")]
            BasicSliPerformance(super::super::BasicSli),
        }
    }
    /// A `MetricRange` is used when each window is good when the value x of a
    /// single `TimeSeries` satisfies `range.min <= x <= range.max`. The provided
    /// `TimeSeries` must have `ValueType = INT64` or `ValueType = DOUBLE` and
    /// `MetricKind = GAUGE`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetricRange {
        /// A [monitoring filter](<https://cloud.google.com/monitoring/api/v3/filters>)
        /// specifying the `TimeSeries` to use for evaluating window quality.
        #[prost(string, tag = "1")]
        pub time_series: ::prost::alloc::string::String,
        /// Range of values considered "good." For a one-sided range, set one bound
        /// to an infinite value.
        #[prost(message, optional, tag = "4")]
        pub range: ::core::option::Option<super::Range>,
    }
    /// The criterion to use for evaluating window goodness.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WindowCriterion {
        /// A [monitoring filter](<https://cloud.google.com/monitoring/api/v3/filters>)
        /// specifying a `TimeSeries` with `ValueType = BOOL`. The window is good if
        /// any `true` values appear in the window.
        #[prost(string, tag = "5")]
        GoodBadMetricFilter(::prost::alloc::string::String),
        /// A window is good if its `performance` is high enough.
        #[prost(message, tag = "2")]
        GoodTotalRatioThreshold(PerformanceThreshold),
        /// A window is good if the metric's value is in a good range, averaged
        /// across returned streams.
        #[prost(message, tag = "6")]
        MetricMeanInRange(MetricRange),
        /// A window is good if the metric's value is in a good range, summed across
        /// returned streams.
        #[prost(message, tag = "7")]
        MetricSumInRange(MetricRange),
    }
}
/// The `CreateService` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceRequest {
    /// Required. Resource [name](<https://cloud.google.com/monitoring/api/v3#project_name>) of
    /// the parent workspace. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The Service id to use for this Service. If omitted, an id will be
    /// generated instead. Must match the pattern `\[a-z0-9\-\]+`
    #[prost(string, tag = "3")]
    pub service_id: ::prost::alloc::string::String,
    /// Required. The `Service` to create.
    #[prost(message, optional, tag = "2")]
    pub service: ::core::option::Option<Service>,
}
/// The `GetService` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Required. Resource name of the `Service`. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/services/\[SERVICE_ID\]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The `ListServices` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Required. Resource name of the parent containing the listed services, either a
    /// [project](<https://cloud.google.com/monitoring/api/v3#project_name>) or a
    /// Monitoring Workspace. The formats are:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    ///      workspaces/\[HOST_PROJECT_ID_OR_NUMBER\]
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// A filter specifying what `Service`s to return. The filter currently
    /// supports the following fields:
    ///
    ///      - `identifier_case`
    ///      - `app_engine.module_id`
    ///      - `cloud_endpoints.service` (reserved for future use)
    ///      - `mesh_istio.mesh_uid`
    ///      - `mesh_istio.service_namespace`
    ///      - `mesh_istio.service_name`
    ///      - `cluster_istio.location` (deprecated)
    ///      - `cluster_istio.cluster_name` (deprecated)
    ///      - `cluster_istio.service_namespace` (deprecated)
    ///      - `cluster_istio.service_name` (deprecated)
    ///
    /// `identifier_case` refers to which option in the identifier oneof is
    /// populated. For example, the filter `identifier_case = "CUSTOM"` would match
    /// all services with a value for the `custom` field. Valid options are
    /// "CUSTOM", "APP_ENGINE", "MESH_ISTIO", plus "CLUSTER_ISTIO" (deprecated)
    /// and "CLOUD_ENDPOINTS" (reserved for future use).
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// A non-negative number that is the maximum number of results to return.
    /// When 0, use default page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The `ListServices` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The `Service`s matching the specified filter.
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The `UpdateService` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceRequest {
    /// Required. The `Service` to draw updates from.
    /// The given `name` specifies the resource to update.
    #[prost(message, optional, tag = "1")]
    pub service: ::core::option::Option<Service>,
    /// A set of field paths defining which fields to use for the update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The `DeleteService` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Required. Resource name of the `Service` to delete. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/services/\[SERVICE_ID\]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The `CreateServiceLevelObjective` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceLevelObjectiveRequest {
    /// Required. Resource name of the parent `Service`. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/services/\[SERVICE_ID\]
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The ServiceLevelObjective id to use for this
    /// ServiceLevelObjective. If omitted, an id will be generated instead. Must
    /// match the pattern `\[a-z0-9\-\]+`
    #[prost(string, tag = "3")]
    pub service_level_objective_id: ::prost::alloc::string::String,
    /// Required. The `ServiceLevelObjective` to create.
    /// The provided `name` will be respected if no `ServiceLevelObjective` exists
    /// with this name.
    #[prost(message, optional, tag = "2")]
    pub service_level_objective: ::core::option::Option<ServiceLevelObjective>,
}
/// The `GetServiceLevelObjective` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceLevelObjectiveRequest {
    /// Required. Resource name of the `ServiceLevelObjective` to get. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/services/\[SERVICE_ID\]/serviceLevelObjectives/\[SLO_NAME\]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// View of the `ServiceLevelObjective` to return. If `DEFAULT`, return the
    /// `ServiceLevelObjective` as originally defined. If `EXPLICIT` and the
    /// `ServiceLevelObjective` is defined in terms of a `BasicSli`, replace the
    /// `BasicSli` with a `RequestBasedSli` spelling out how the SLI is computed.
    #[prost(enumeration = "service_level_objective::View", tag = "2")]
    pub view: i32,
}
/// The `ListServiceLevelObjectives` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceLevelObjectivesRequest {
    /// Required. Resource name of the parent containing the listed SLOs, either a
    /// project or a Monitoring Workspace. The formats are:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/services/\[SERVICE_ID\]
    ///      workspaces/\[HOST_PROJECT_ID_OR_NUMBER\]/services/-
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// A filter specifying what `ServiceLevelObjective`s to return.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// A non-negative number that is the maximum number of results to return.
    /// When 0, use default page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// View of the `ServiceLevelObjective`s to return. If `DEFAULT`, return each
    /// `ServiceLevelObjective` as originally defined. If `EXPLICIT` and the
    /// `ServiceLevelObjective` is defined in terms of a `BasicSli`, replace the
    /// `BasicSli` with a `RequestBasedSli` spelling out how the SLI is computed.
    #[prost(enumeration = "service_level_objective::View", tag = "5")]
    pub view: i32,
}
/// The `ListServiceLevelObjectives` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceLevelObjectivesResponse {
    /// The `ServiceLevelObjective`s matching the specified filter.
    #[prost(message, repeated, tag = "1")]
    pub service_level_objectives: ::prost::alloc::vec::Vec<ServiceLevelObjective>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The `UpdateServiceLevelObjective` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceLevelObjectiveRequest {
    /// Required. The `ServiceLevelObjective` to draw updates from.
    /// The given `name` specifies the resource to update.
    #[prost(message, optional, tag = "1")]
    pub service_level_objective: ::core::option::Option<ServiceLevelObjective>,
    /// A set of field paths defining which fields to use for the update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The `DeleteServiceLevelObjective` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceLevelObjectiveRequest {
    /// Required. Resource name of the `ServiceLevelObjective` to delete. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/services/\[SERVICE_ID\]/serviceLevelObjectives/\[SLO_NAME\]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod service_monitoring_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Cloud Monitoring Service-Oriented Monitoring API has endpoints for
    /// managing and querying aspects of a workspace's services. These include the
    /// `Service`'s monitored resources, its Service-Level Objectives, and a taxonomy
    /// of categorized Health Metrics.
    #[derive(Debug, Clone)]
    pub struct ServiceMonitoringServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ServiceMonitoringServiceClient<T>
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
        ) -> ServiceMonitoringServiceClient<InterceptedService<T, F>>
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
            ServiceMonitoringServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Create a `Service`.
        pub async fn create_service(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceRequest>,
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
                "/google.monitoring.v3.ServiceMonitoringService/CreateService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.ServiceMonitoringService",
                        "CreateService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get the named `Service`.
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
                "/google.monitoring.v3.ServiceMonitoringService/GetService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.ServiceMonitoringService",
                        "GetService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List `Service`s for this workspace.
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
                "/google.monitoring.v3.ServiceMonitoringService/ListServices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.ServiceMonitoringService",
                        "ListServices",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update this `Service`.
        pub async fn update_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServiceRequest>,
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
                "/google.monitoring.v3.ServiceMonitoringService/UpdateService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.ServiceMonitoringService",
                        "UpdateService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Soft delete this `Service`.
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceRequest>,
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
                "/google.monitoring.v3.ServiceMonitoringService/DeleteService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.ServiceMonitoringService",
                        "DeleteService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create a `ServiceLevelObjective` for the given `Service`.
        pub async fn create_service_level_objective(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceLevelObjectiveRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ServiceLevelObjective>,
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
                "/google.monitoring.v3.ServiceMonitoringService/CreateServiceLevelObjective",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.ServiceMonitoringService",
                        "CreateServiceLevelObjective",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get a `ServiceLevelObjective` by name.
        pub async fn get_service_level_objective(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceLevelObjectiveRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ServiceLevelObjective>,
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
                "/google.monitoring.v3.ServiceMonitoringService/GetServiceLevelObjective",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.ServiceMonitoringService",
                        "GetServiceLevelObjective",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List the `ServiceLevelObjective`s for the given `Service`.
        pub async fn list_service_level_objectives(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServiceLevelObjectivesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListServiceLevelObjectivesResponse>,
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
                "/google.monitoring.v3.ServiceMonitoringService/ListServiceLevelObjectives",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.ServiceMonitoringService",
                        "ListServiceLevelObjectives",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update the given `ServiceLevelObjective`.
        pub async fn update_service_level_objective(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServiceLevelObjectiveRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ServiceLevelObjective>,
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
                "/google.monitoring.v3.ServiceMonitoringService/UpdateServiceLevelObjective",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.ServiceMonitoringService",
                        "UpdateServiceLevelObjective",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete the given `ServiceLevelObjective`.
        pub async fn delete_service_level_objective(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceLevelObjectiveRequest>,
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
                "/google.monitoring.v3.ServiceMonitoringService/DeleteServiceLevelObjective",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.ServiceMonitoringService",
                        "DeleteServiceLevelObjective",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// A single strongly-typed value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedValue {
    /// The typed value field.
    #[prost(oneof = "typed_value::Value", tags = "1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<typed_value::Value>,
}
/// Nested message and enum types in `TypedValue`.
pub mod typed_value {
    /// The typed value field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A Boolean value: `true` or `false`.
        #[prost(bool, tag = "1")]
        BoolValue(bool),
        /// A 64-bit integer. Its range is approximately &plusmn;9.2x10<sup>18</sup>.
        #[prost(int64, tag = "2")]
        Int64Value(i64),
        /// A 64-bit double-precision floating-point number. Its magnitude
        /// is approximately &plusmn;10<sup>&plusmn;300</sup> and it has 16
        /// significant digits of precision.
        #[prost(double, tag = "3")]
        DoubleValue(f64),
        /// A variable-length string value.
        #[prost(string, tag = "4")]
        StringValue(::prost::alloc::string::String),
        /// A distribution value.
        #[prost(message, tag = "5")]
        DistributionValue(super::super::super::api::Distribution),
    }
}
/// A closed time interval. It extends from the start time to the end time, and includes both: `\[startTime, endTime\]`. Valid time intervals depend on the [`MetricKind`](<https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors#MetricKind>) of the metric value. The end time must not be earlier than the start time. When writing data points, the start time must not be more than 25 hours in the past and the end time must not be more than five minutes in the future.
///
/// * For `GAUGE` metrics, the `startTime` value is technically optional; if
///    no value is specified, the start time defaults to the value of the
///    end time, and the interval represents a single point in time. If both
///    start and end times are specified, they must be identical. Such an
///    interval is valid only for `GAUGE` metrics, which are point-in-time
///    measurements. The end time of a new interval must be at least a
///    millisecond after the end time of the previous interval.
///
/// * For `DELTA` metrics, the start time and end time must specify a
///    non-zero interval, with subsequent points specifying contiguous and
///    non-overlapping intervals. For `DELTA` metrics, the start time of
///    the next interval must be at least a millisecond after the end time
///    of the previous interval.
///
/// * For `CUMULATIVE` metrics, the start time and end time must specify a
///    non-zero interval, with subsequent points specifying the same
///    start time and increasing end times, until an event resets the
///    cumulative value to zero and sets a new start time for the following
///    points. The new start time must be at least a millisecond after the
///    end time of the previous interval.
///
/// * The start time of a new interval must be at least a millisecond after the
///    end time of the previous interval because intervals are closed. If the
///    start time of a new interval is the same as the end time of the previous
///    interval, then data written at the new start time could overwrite data
///    written at the previous end time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeInterval {
    /// Required. The end of the time interval.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The beginning of the time interval.  The default value
    /// for the start time is the end time. The start time must not be
    /// later than the end time.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Describes how to combine multiple time series to provide a different view of
/// the data.  Aggregation of time series is done in two steps. First, each time
/// series in the set is _aligned_ to the same time interval boundaries, then the
/// set of time series is optionally _reduced_ in number.
///
/// Alignment consists of applying the `per_series_aligner` operation
/// to each time series after its data has been divided into regular
/// `alignment_period` time intervals. This process takes _all_ of the data
/// points in an alignment period, applies a mathematical transformation such as
/// averaging, minimum, maximum, delta, etc., and converts them into a single
/// data point per period.
///
/// Reduction is when the aligned and transformed time series can optionally be
/// combined, reducing the number of time series through similar mathematical
/// transformations. Reduction involves applying a `cross_series_reducer` to
/// all the time series, optionally sorting the time series into subsets with
/// `group_by_fields`, and applying the reducer to each subset.
///
/// The raw time series data can contain a huge amount of information from
/// multiple sources. Alignment and reduction transforms this mass of data into
/// a more manageable and representative collection of data, for example "the
/// 95% latency across the average of all tasks in a cluster". This
/// representative data can be more easily graphed and comprehended, and the
/// individual time series data is still available for later drilldown. For more
/// details, see [Filtering and
/// aggregation](<https://cloud.google.com/monitoring/api/v3/aggregation>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aggregation {
    /// The `alignment_period` specifies a time interval, in seconds, that is used
    /// to divide the data in all the
    /// [time series][google.monitoring.v3.TimeSeries] into consistent blocks of
    /// time. This will be done before the per-series aligner can be applied to
    /// the data.
    ///
    /// The value must be at least 60 seconds. If a per-series
    /// aligner other than `ALIGN_NONE` is specified, this field is required or an
    /// error is returned. If no per-series aligner is specified, or the aligner
    /// `ALIGN_NONE` is specified, then this field is ignored.
    ///
    /// The maximum value of the `alignment_period` is 104 weeks (2 years) for
    /// charts, and 90,000 seconds (25 hours) for alerting policies.
    #[prost(message, optional, tag = "1")]
    pub alignment_period: ::core::option::Option<::prost_types::Duration>,
    /// An `Aligner` describes how to bring the data points in a single
    /// time series into temporal alignment. Except for `ALIGN_NONE`, all
    /// alignments cause all the data points in an `alignment_period` to be
    /// mathematically grouped together, resulting in a single data point for
    /// each `alignment_period` with end timestamp at the end of the period.
    ///
    /// Not all alignment operations may be applied to all time series. The valid
    /// choices depend on the `metric_kind` and `value_type` of the original time
    /// series. Alignment can change the `metric_kind` or the `value_type` of
    /// the time series.
    ///
    /// Time series data must be aligned in order to perform cross-time
    /// series reduction. If `cross_series_reducer` is specified, then
    /// `per_series_aligner` must be specified and not equal to `ALIGN_NONE`
    /// and `alignment_period` must be specified; otherwise, an error is
    /// returned.
    #[prost(enumeration = "aggregation::Aligner", tag = "2")]
    pub per_series_aligner: i32,
    /// The reduction operation to be used to combine time series into a single
    /// time series, where the value of each data point in the resulting series is
    /// a function of all the already aligned values in the input time series.
    ///
    /// Not all reducer operations can be applied to all time series. The valid
    /// choices depend on the `metric_kind` and the `value_type` of the original
    /// time series. Reduction can yield a time series with a different
    /// `metric_kind` or `value_type` than the input time series.
    ///
    /// Time series data must first be aligned (see `per_series_aligner`) in order
    /// to perform cross-time series reduction. If `cross_series_reducer` is
    /// specified, then `per_series_aligner` must be specified, and must not be
    /// `ALIGN_NONE`. An `alignment_period` must also be specified; otherwise, an
    /// error is returned.
    #[prost(enumeration = "aggregation::Reducer", tag = "4")]
    pub cross_series_reducer: i32,
    /// The set of fields to preserve when `cross_series_reducer` is
    /// specified. The `group_by_fields` determine how the time series are
    /// partitioned into subsets prior to applying the aggregation
    /// operation. Each subset contains time series that have the same
    /// value for each of the grouping fields. Each individual time
    /// series is a member of exactly one subset. The
    /// `cross_series_reducer` is applied to each subset of time series.
    /// It is not possible to reduce across different resource types, so
    /// this field implicitly contains `resource.type`.  Fields not
    /// specified in `group_by_fields` are aggregated away.  If
    /// `group_by_fields` is not specified and all the time series have
    /// the same resource type, then the time series are aggregated into
    /// a single output time series. If `cross_series_reducer` is not
    /// defined, this field is ignored.
    #[prost(string, repeated, tag = "5")]
    pub group_by_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Aggregation`.
pub mod aggregation {
    /// The `Aligner` specifies the operation that will be applied to the data
    /// points in each alignment period in a time series. Except for
    /// `ALIGN_NONE`, which specifies that no operation be applied, each alignment
    /// operation replaces the set of data values in each alignment period with
    /// a single value: the result of applying the operation to the data values.
    /// An aligned time series has a single data value at the end of each
    /// `alignment_period`.
    ///
    /// An alignment operation can change the data type of the values, too. For
    /// example, if you apply a counting operation to boolean values, the data
    /// `value_type` in the original time series is `BOOLEAN`, but the `value_type`
    /// in the aligned result is `INT64`.
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
    pub enum Aligner {
        /// No alignment. Raw data is returned. Not valid if cross-series reduction
        /// is requested. The `value_type` of the result is the same as the
        /// `value_type` of the input.
        AlignNone = 0,
        /// Align and convert to
        /// [DELTA][google.api.MetricDescriptor.MetricKind.DELTA].
        /// The output is `delta = y1 - y0`.
        ///
        /// This alignment is valid for
        /// [CUMULATIVE][google.api.MetricDescriptor.MetricKind.CUMULATIVE] and
        /// `DELTA` metrics. If the selected alignment period results in periods
        /// with no data, then the aligned value for such a period is created by
        /// interpolation. The `value_type`  of the aligned result is the same as
        /// the `value_type` of the input.
        AlignDelta = 1,
        /// Align and convert to a rate. The result is computed as
        /// `rate = (y1 - y0)/(t1 - t0)`, or "delta over time".
        /// Think of this aligner as providing the slope of the line that passes
        /// through the value at the start and at the end of the `alignment_period`.
        ///
        /// This aligner is valid for `CUMULATIVE`
        /// and `DELTA` metrics with numeric values. If the selected alignment
        /// period results in periods with no data, then the aligned value for
        /// such a period is created by interpolation. The output is a `GAUGE`
        /// metric with `value_type` `DOUBLE`.
        ///
        /// If, by "rate", you mean "percentage change", see the
        /// `ALIGN_PERCENT_CHANGE` aligner instead.
        AlignRate = 2,
        /// Align by interpolating between adjacent points around the alignment
        /// period boundary. This aligner is valid for `GAUGE` metrics with
        /// numeric values. The `value_type` of the aligned result is the same as the
        /// `value_type` of the input.
        AlignInterpolate = 3,
        /// Align by moving the most recent data point before the end of the
        /// alignment period to the boundary at the end of the alignment
        /// period. This aligner is valid for `GAUGE` metrics. The `value_type` of
        /// the aligned result is the same as the `value_type` of the input.
        AlignNextOlder = 4,
        /// Align the time series by returning the minimum value in each alignment
        /// period. This aligner is valid for `GAUGE` and `DELTA` metrics with
        /// numeric values. The `value_type` of the aligned result is the same as
        /// the `value_type` of the input.
        AlignMin = 10,
        /// Align the time series by returning the maximum value in each alignment
        /// period. This aligner is valid for `GAUGE` and `DELTA` metrics with
        /// numeric values. The `value_type` of the aligned result is the same as
        /// the `value_type` of the input.
        AlignMax = 11,
        /// Align the time series by returning the mean value in each alignment
        /// period. This aligner is valid for `GAUGE` and `DELTA` metrics with
        /// numeric values. The `value_type` of the aligned result is `DOUBLE`.
        AlignMean = 12,
        /// Align the time series by returning the number of values in each alignment
        /// period. This aligner is valid for `GAUGE` and `DELTA` metrics with
        /// numeric or Boolean values. The `value_type` of the aligned result is
        /// `INT64`.
        AlignCount = 13,
        /// Align the time series by returning the sum of the values in each
        /// alignment period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with numeric and distribution values. The `value_type` of the
        /// aligned result is the same as the `value_type` of the input.
        AlignSum = 14,
        /// Align the time series by returning the standard deviation of the values
        /// in each alignment period. This aligner is valid for `GAUGE` and
        /// `DELTA` metrics with numeric values. The `value_type` of the output is
        /// `DOUBLE`.
        AlignStddev = 15,
        /// Align the time series by returning the number of `True` values in
        /// each alignment period. This aligner is valid for `GAUGE` metrics with
        /// Boolean values. The `value_type` of the output is `INT64`.
        AlignCountTrue = 16,
        /// Align the time series by returning the number of `False` values in
        /// each alignment period. This aligner is valid for `GAUGE` metrics with
        /// Boolean values. The `value_type` of the output is `INT64`.
        AlignCountFalse = 24,
        /// Align the time series by returning the ratio of the number of `True`
        /// values to the total number of values in each alignment period. This
        /// aligner is valid for `GAUGE` metrics with Boolean values. The output
        /// value is in the range \[0.0, 1.0\] and has `value_type` `DOUBLE`.
        AlignFractionTrue = 17,
        /// Align the time series by using [percentile
        /// aggregation](<https://en.wikipedia.org/wiki/Percentile>). The resulting
        /// data point in each alignment period is the 99th percentile of all data
        /// points in the period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with distribution values. The output is a `GAUGE` metric with
        /// `value_type` `DOUBLE`.
        AlignPercentile99 = 18,
        /// Align the time series by using [percentile
        /// aggregation](<https://en.wikipedia.org/wiki/Percentile>). The resulting
        /// data point in each alignment period is the 95th percentile of all data
        /// points in the period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with distribution values. The output is a `GAUGE` metric with
        /// `value_type` `DOUBLE`.
        AlignPercentile95 = 19,
        /// Align the time series by using [percentile
        /// aggregation](<https://en.wikipedia.org/wiki/Percentile>). The resulting
        /// data point in each alignment period is the 50th percentile of all data
        /// points in the period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with distribution values. The output is a `GAUGE` metric with
        /// `value_type` `DOUBLE`.
        AlignPercentile50 = 20,
        /// Align the time series by using [percentile
        /// aggregation](<https://en.wikipedia.org/wiki/Percentile>). The resulting
        /// data point in each alignment period is the 5th percentile of all data
        /// points in the period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with distribution values. The output is a `GAUGE` metric with
        /// `value_type` `DOUBLE`.
        AlignPercentile05 = 21,
        /// Align and convert to a percentage change. This aligner is valid for
        /// `GAUGE` and `DELTA` metrics with numeric values. This alignment returns
        /// `((current - previous)/previous) * 100`, where the value of `previous` is
        /// determined based on the `alignment_period`.
        ///
        /// If the values of `current` and `previous` are both 0, then the returned
        /// value is 0. If only `previous` is 0, the returned value is infinity.
        ///
        /// A 10-minute moving mean is computed at each point of the alignment period
        /// prior to the above calculation to smooth the metric and prevent false
        /// positives from very short-lived spikes. The moving mean is only
        /// applicable for data whose values are `>= 0`. Any values `< 0` are
        /// treated as a missing datapoint, and are ignored. While `DELTA`
        /// metrics are accepted by this alignment, special care should be taken that
        /// the values for the metric will always be positive. The output is a
        /// `GAUGE` metric with `value_type` `DOUBLE`.
        AlignPercentChange = 23,
    }
    impl Aligner {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Aligner::AlignNone => "ALIGN_NONE",
                Aligner::AlignDelta => "ALIGN_DELTA",
                Aligner::AlignRate => "ALIGN_RATE",
                Aligner::AlignInterpolate => "ALIGN_INTERPOLATE",
                Aligner::AlignNextOlder => "ALIGN_NEXT_OLDER",
                Aligner::AlignMin => "ALIGN_MIN",
                Aligner::AlignMax => "ALIGN_MAX",
                Aligner::AlignMean => "ALIGN_MEAN",
                Aligner::AlignCount => "ALIGN_COUNT",
                Aligner::AlignSum => "ALIGN_SUM",
                Aligner::AlignStddev => "ALIGN_STDDEV",
                Aligner::AlignCountTrue => "ALIGN_COUNT_TRUE",
                Aligner::AlignCountFalse => "ALIGN_COUNT_FALSE",
                Aligner::AlignFractionTrue => "ALIGN_FRACTION_TRUE",
                Aligner::AlignPercentile99 => "ALIGN_PERCENTILE_99",
                Aligner::AlignPercentile95 => "ALIGN_PERCENTILE_95",
                Aligner::AlignPercentile50 => "ALIGN_PERCENTILE_50",
                Aligner::AlignPercentile05 => "ALIGN_PERCENTILE_05",
                Aligner::AlignPercentChange => "ALIGN_PERCENT_CHANGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ALIGN_NONE" => Some(Self::AlignNone),
                "ALIGN_DELTA" => Some(Self::AlignDelta),
                "ALIGN_RATE" => Some(Self::AlignRate),
                "ALIGN_INTERPOLATE" => Some(Self::AlignInterpolate),
                "ALIGN_NEXT_OLDER" => Some(Self::AlignNextOlder),
                "ALIGN_MIN" => Some(Self::AlignMin),
                "ALIGN_MAX" => Some(Self::AlignMax),
                "ALIGN_MEAN" => Some(Self::AlignMean),
                "ALIGN_COUNT" => Some(Self::AlignCount),
                "ALIGN_SUM" => Some(Self::AlignSum),
                "ALIGN_STDDEV" => Some(Self::AlignStddev),
                "ALIGN_COUNT_TRUE" => Some(Self::AlignCountTrue),
                "ALIGN_COUNT_FALSE" => Some(Self::AlignCountFalse),
                "ALIGN_FRACTION_TRUE" => Some(Self::AlignFractionTrue),
                "ALIGN_PERCENTILE_99" => Some(Self::AlignPercentile99),
                "ALIGN_PERCENTILE_95" => Some(Self::AlignPercentile95),
                "ALIGN_PERCENTILE_50" => Some(Self::AlignPercentile50),
                "ALIGN_PERCENTILE_05" => Some(Self::AlignPercentile05),
                "ALIGN_PERCENT_CHANGE" => Some(Self::AlignPercentChange),
                _ => None,
            }
        }
    }
    /// A Reducer operation describes how to aggregate data points from multiple
    /// time series into a single time series, where the value of each data point
    /// in the resulting series is a function of all the already aligned values in
    /// the input time series.
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
    pub enum Reducer {
        /// No cross-time series reduction. The output of the `Aligner` is
        /// returned.
        ReduceNone = 0,
        /// Reduce by computing the mean value across time series for each
        /// alignment period. This reducer is valid for
        /// [DELTA][google.api.MetricDescriptor.MetricKind.DELTA] and
        /// [GAUGE][google.api.MetricDescriptor.MetricKind.GAUGE] metrics with
        /// numeric or distribution values. The `value_type` of the output is
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        ReduceMean = 1,
        /// Reduce by computing the minimum value across time series for each
        /// alignment period. This reducer is valid for `DELTA` and `GAUGE` metrics
        /// with numeric values. The `value_type` of the output is the same as the
        /// `value_type` of the input.
        ReduceMin = 2,
        /// Reduce by computing the maximum value across time series for each
        /// alignment period. This reducer is valid for `DELTA` and `GAUGE` metrics
        /// with numeric values. The `value_type` of the output is the same as the
        /// `value_type` of the input.
        ReduceMax = 3,
        /// Reduce by computing the sum across time series for each
        /// alignment period. This reducer is valid for `DELTA` and `GAUGE` metrics
        /// with numeric and distribution values. The `value_type` of the output is
        /// the same as the `value_type` of the input.
        ReduceSum = 4,
        /// Reduce by computing the standard deviation across time series
        /// for each alignment period. This reducer is valid for `DELTA` and
        /// `GAUGE` metrics with numeric or distribution values. The `value_type`
        /// of the output is `DOUBLE`.
        ReduceStddev = 5,
        /// Reduce by computing the number of data points across time series
        /// for each alignment period. This reducer is valid for `DELTA` and
        /// `GAUGE` metrics of numeric, Boolean, distribution, and string
        /// `value_type`. The `value_type` of the output is `INT64`.
        ReduceCount = 6,
        /// Reduce by computing the number of `True`-valued data points across time
        /// series for each alignment period. This reducer is valid for `DELTA` and
        /// `GAUGE` metrics of Boolean `value_type`. The `value_type` of the output
        /// is `INT64`.
        ReduceCountTrue = 7,
        /// Reduce by computing the number of `False`-valued data points across time
        /// series for each alignment period. This reducer is valid for `DELTA` and
        /// `GAUGE` metrics of Boolean `value_type`. The `value_type` of the output
        /// is `INT64`.
        ReduceCountFalse = 15,
        /// Reduce by computing the ratio of the number of `True`-valued data points
        /// to the total number of data points for each alignment period. This
        /// reducer is valid for `DELTA` and `GAUGE` metrics of Boolean `value_type`.
        /// The output value is in the range \[0.0, 1.0\] and has `value_type`
        /// `DOUBLE`.
        ReduceFractionTrue = 8,
        /// Reduce by computing the [99th
        /// percentile](<https://en.wikipedia.org/wiki/Percentile>) of data points
        /// across time series for each alignment period. This reducer is valid for
        /// `GAUGE` and `DELTA` metrics of numeric and distribution type. The value
        /// of the output is `DOUBLE`.
        ReducePercentile99 = 9,
        /// Reduce by computing the [95th
        /// percentile](<https://en.wikipedia.org/wiki/Percentile>) of data points
        /// across time series for each alignment period. This reducer is valid for
        /// `GAUGE` and `DELTA` metrics of numeric and distribution type. The value
        /// of the output is `DOUBLE`.
        ReducePercentile95 = 10,
        /// Reduce by computing the [50th
        /// percentile](<https://en.wikipedia.org/wiki/Percentile>) of data points
        /// across time series for each alignment period. This reducer is valid for
        /// `GAUGE` and `DELTA` metrics of numeric and distribution type. The value
        /// of the output is `DOUBLE`.
        ReducePercentile50 = 11,
        /// Reduce by computing the [5th
        /// percentile](<https://en.wikipedia.org/wiki/Percentile>) of data points
        /// across time series for each alignment period. This reducer is valid for
        /// `GAUGE` and `DELTA` metrics of numeric and distribution type. The value
        /// of the output is `DOUBLE`.
        ReducePercentile05 = 12,
    }
    impl Reducer {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reducer::ReduceNone => "REDUCE_NONE",
                Reducer::ReduceMean => "REDUCE_MEAN",
                Reducer::ReduceMin => "REDUCE_MIN",
                Reducer::ReduceMax => "REDUCE_MAX",
                Reducer::ReduceSum => "REDUCE_SUM",
                Reducer::ReduceStddev => "REDUCE_STDDEV",
                Reducer::ReduceCount => "REDUCE_COUNT",
                Reducer::ReduceCountTrue => "REDUCE_COUNT_TRUE",
                Reducer::ReduceCountFalse => "REDUCE_COUNT_FALSE",
                Reducer::ReduceFractionTrue => "REDUCE_FRACTION_TRUE",
                Reducer::ReducePercentile99 => "REDUCE_PERCENTILE_99",
                Reducer::ReducePercentile95 => "REDUCE_PERCENTILE_95",
                Reducer::ReducePercentile50 => "REDUCE_PERCENTILE_50",
                Reducer::ReducePercentile05 => "REDUCE_PERCENTILE_05",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REDUCE_NONE" => Some(Self::ReduceNone),
                "REDUCE_MEAN" => Some(Self::ReduceMean),
                "REDUCE_MIN" => Some(Self::ReduceMin),
                "REDUCE_MAX" => Some(Self::ReduceMax),
                "REDUCE_SUM" => Some(Self::ReduceSum),
                "REDUCE_STDDEV" => Some(Self::ReduceStddev),
                "REDUCE_COUNT" => Some(Self::ReduceCount),
                "REDUCE_COUNT_TRUE" => Some(Self::ReduceCountTrue),
                "REDUCE_COUNT_FALSE" => Some(Self::ReduceCountFalse),
                "REDUCE_FRACTION_TRUE" => Some(Self::ReduceFractionTrue),
                "REDUCE_PERCENTILE_99" => Some(Self::ReducePercentile99),
                "REDUCE_PERCENTILE_95" => Some(Self::ReducePercentile95),
                "REDUCE_PERCENTILE_50" => Some(Self::ReducePercentile50),
                "REDUCE_PERCENTILE_05" => Some(Self::ReducePercentile05),
                _ => None,
            }
        }
    }
}
/// Specifies an ordering relationship on two arguments, called `left` and
/// `right`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComparisonType {
    /// No ordering relationship is specified.
    ComparisonUnspecified = 0,
    /// True if the left argument is greater than the right argument.
    ComparisonGt = 1,
    /// True if the left argument is greater than or equal to the right argument.
    ComparisonGe = 2,
    /// True if the left argument is less than the right argument.
    ComparisonLt = 3,
    /// True if the left argument is less than or equal to the right argument.
    ComparisonLe = 4,
    /// True if the left argument is equal to the right argument.
    ComparisonEq = 5,
    /// True if the left argument is not equal to the right argument.
    ComparisonNe = 6,
}
impl ComparisonType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ComparisonType::ComparisonUnspecified => "COMPARISON_UNSPECIFIED",
            ComparisonType::ComparisonGt => "COMPARISON_GT",
            ComparisonType::ComparisonGe => "COMPARISON_GE",
            ComparisonType::ComparisonLt => "COMPARISON_LT",
            ComparisonType::ComparisonLe => "COMPARISON_LE",
            ComparisonType::ComparisonEq => "COMPARISON_EQ",
            ComparisonType::ComparisonNe => "COMPARISON_NE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMPARISON_UNSPECIFIED" => Some(Self::ComparisonUnspecified),
            "COMPARISON_GT" => Some(Self::ComparisonGt),
            "COMPARISON_GE" => Some(Self::ComparisonGe),
            "COMPARISON_LT" => Some(Self::ComparisonLt),
            "COMPARISON_LE" => Some(Self::ComparisonLe),
            "COMPARISON_EQ" => Some(Self::ComparisonEq),
            "COMPARISON_NE" => Some(Self::ComparisonNe),
            _ => None,
        }
    }
}
/// The tier of service for a Workspace. Please see the
/// [service tiers
/// documentation](<https://cloud.google.com/monitoring/workspaces/tiers>) for more
/// details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceTier {
    /// An invalid sentinel value, used to indicate that a tier has not
    /// been provided explicitly.
    Unspecified = 0,
    /// The Stackdriver Basic tier, a free tier of service that provides basic
    /// features, a moderate allotment of logs, and access to built-in metrics.
    /// A number of features are not available in this tier. For more details,
    /// see [the service tiers
    /// documentation](<https://cloud.google.com/monitoring/workspaces/tiers>).
    Basic = 1,
    /// The Stackdriver Premium tier, a higher, more expensive tier of service
    /// that provides access to all Stackdriver features, lets you use Stackdriver
    /// with AWS accounts, and has a larger allotments for logs and metrics. For
    /// more details, see [the service tiers
    /// documentation](<https://cloud.google.com/monitoring/workspaces/tiers>).
    Premium = 2,
}
impl ServiceTier {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServiceTier::Unspecified => "SERVICE_TIER_UNSPECIFIED",
            ServiceTier::Basic => "SERVICE_TIER_BASIC",
            ServiceTier::Premium => "SERVICE_TIER_PREMIUM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SERVICE_TIER_UNSPECIFIED" => Some(Self::Unspecified),
            "SERVICE_TIER_BASIC" => Some(Self::Basic),
            "SERVICE_TIER_PREMIUM" => Some(Self::Premium),
            _ => None,
        }
    }
}
/// A single data point in a time series.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    /// The time interval to which the data point applies.  For `GAUGE` metrics,
    /// the start time is optional, but if it is supplied, it must equal the
    /// end time.  For `DELTA` metrics, the start
    /// and end time should specify a non-zero interval, with subsequent points
    /// specifying contiguous and non-overlapping intervals.  For `CUMULATIVE`
    /// metrics, the start and end time should specify a non-zero interval, with
    /// subsequent points specifying the same start time and increasing end times,
    /// until an event resets the cumulative value to zero and sets a new start
    /// time for the following points.
    #[prost(message, optional, tag = "1")]
    pub interval: ::core::option::Option<TimeInterval>,
    /// The value of the data point.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<TypedValue>,
}
/// A collection of data points that describes the time-varying values
/// of a metric. A time series is identified by a combination of a
/// fully-specified monitored resource and a fully-specified metric.
/// This type is used for both listing and creating time series.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeries {
    /// The associated metric. A fully-specified metric used to identify the time
    /// series.
    #[prost(message, optional, tag = "1")]
    pub metric: ::core::option::Option<super::super::api::Metric>,
    /// The associated monitored resource.  Custom metrics can use only certain
    /// monitored resource types in their time series data. For more information,
    /// see [Monitored resources for custom
    /// metrics](<https://cloud.google.com/monitoring/custom-metrics/creating-metrics#custom-metric-resources>).
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<super::super::api::MonitoredResource>,
    /// Output only. The associated monitored resource metadata. When reading a
    /// time series, this field will include metadata labels that are explicitly
    /// named in the reduction. When creating a time series, this field is ignored.
    #[prost(message, optional, tag = "7")]
    pub metadata: ::core::option::Option<super::super::api::MonitoredResourceMetadata>,
    /// The metric kind of the time series. When listing time series, this metric
    /// kind might be different from the metric kind of the associated metric if
    /// this time series is an alignment or reduction of other time series.
    ///
    /// When creating a time series, this field is optional. If present, it must be
    /// the same as the metric kind of the associated metric. If the associated
    /// metric's descriptor must be auto-created, then this field specifies the
    /// metric kind of the new descriptor and must be either `GAUGE` (the default)
    /// or `CUMULATIVE`.
    #[prost(enumeration = "super::super::api::metric_descriptor::MetricKind", tag = "3")]
    pub metric_kind: i32,
    /// The value type of the time series. When listing time series, this value
    /// type might be different from the value type of the associated metric if
    /// this time series is an alignment or reduction of other time series.
    ///
    /// When creating a time series, this field is optional. If present, it must be
    /// the same as the type of the data in the `points` field.
    #[prost(enumeration = "super::super::api::metric_descriptor::ValueType", tag = "4")]
    pub value_type: i32,
    /// The data points of this time series. When listing time series, points are
    /// returned in reverse time order.
    ///
    /// When creating a time series, this field must contain exactly one point and
    /// the point's type must be the same as the value type of the associated
    /// metric. If the associated metric's descriptor must be auto-created, then
    /// the value type of the descriptor is determined by the point's type, which
    /// must be `BOOL`, `INT64`, `DOUBLE`, or `DISTRIBUTION`.
    #[prost(message, repeated, tag = "5")]
    pub points: ::prost::alloc::vec::Vec<Point>,
    /// The units in which the metric value is reported. It is only applicable
    /// if the `value_type` is `INT64`, `DOUBLE`, or `DISTRIBUTION`. The `unit`
    /// defines the representation of the stored metric values.
    #[prost(string, tag = "8")]
    pub unit: ::prost::alloc::string::String,
}
/// A descriptor for the labels and points in a time series.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesDescriptor {
    /// Descriptors for the labels.
    #[prost(message, repeated, tag = "1")]
    pub label_descriptors: ::prost::alloc::vec::Vec<super::super::api::LabelDescriptor>,
    /// Descriptors for the point data value columns.
    #[prost(message, repeated, tag = "5")]
    pub point_descriptors: ::prost::alloc::vec::Vec<
        time_series_descriptor::ValueDescriptor,
    >,
}
/// Nested message and enum types in `TimeSeriesDescriptor`.
pub mod time_series_descriptor {
    /// A descriptor for the value columns in a data point.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValueDescriptor {
        /// The value key.
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        /// The value type.
        #[prost(
            enumeration = "super::super::super::api::metric_descriptor::ValueType",
            tag = "2"
        )]
        pub value_type: i32,
        /// The value stream kind.
        #[prost(
            enumeration = "super::super::super::api::metric_descriptor::MetricKind",
            tag = "3"
        )]
        pub metric_kind: i32,
        /// The unit in which `time_series` point values are reported. `unit`
        /// follows the UCUM format for units as seen in
        /// <https://unitsofmeasure.org/ucum.html.>
        /// `unit` is only valid if `value_type` is INTEGER, DOUBLE, DISTRIBUTION.
        #[prost(string, tag = "4")]
        pub unit: ::prost::alloc::string::String,
    }
}
/// Represents the values of a time series associated with a
/// TimeSeriesDescriptor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesData {
    /// The values of the labels in the time series identifier, given in the same
    /// order as the `label_descriptors` field of the TimeSeriesDescriptor
    /// associated with this object. Each value must have a value of the type
    /// given in the corresponding entry of `label_descriptors`.
    #[prost(message, repeated, tag = "1")]
    pub label_values: ::prost::alloc::vec::Vec<LabelValue>,
    /// The points in the time series.
    #[prost(message, repeated, tag = "2")]
    pub point_data: ::prost::alloc::vec::Vec<time_series_data::PointData>,
}
/// Nested message and enum types in `TimeSeriesData`.
pub mod time_series_data {
    /// A point's value columns and time interval. Each point has one or more
    /// point values corresponding to the entries in `point_descriptors` field in
    /// the TimeSeriesDescriptor associated with this object.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PointData {
        /// The values that make up the point.
        #[prost(message, repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<super::TypedValue>,
        /// The time interval associated with the point.
        #[prost(message, optional, tag = "2")]
        pub time_interval: ::core::option::Option<super::TimeInterval>,
    }
}
/// A label value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelValue {
    /// The label value can be a bool, int64, or string.
    #[prost(oneof = "label_value::Value", tags = "1, 2, 3")]
    pub value: ::core::option::Option<label_value::Value>,
}
/// Nested message and enum types in `LabelValue`.
pub mod label_value {
    /// The label value can be a bool, int64, or string.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A bool label value.
        #[prost(bool, tag = "1")]
        BoolValue(bool),
        /// An int64 label value.
        #[prost(int64, tag = "2")]
        Int64Value(i64),
        /// A string label value.
        #[prost(string, tag = "3")]
        StringValue(::prost::alloc::string::String),
    }
}
/// An error associated with a query in the time series query language format.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryError {
    /// The location of the time series query language text that this error applies
    /// to.
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<TextLocator>,
    /// The error message.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// A locator for text. Indicates a particular part of the text of a request or
/// of an object referenced in the request.
///
/// For example, suppose the request field `text` contains:
///
///    text: "The quick brown fox jumps over the lazy dog."
///
/// Then the locator:
///
///    source: "text"
///    start_position {
///      line: 1
///      column: 17
///    }
///    end_position {
///      line: 1
///      column: 19
///    }
///
/// refers to the part of the text: "fox".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextLocator {
    /// The source of the text. The source may be a field in the request, in which
    /// case its format is the format of the
    /// google.rpc.BadRequest.FieldViolation.field field in
    /// <https://cloud.google.com/apis/design/errors#error_details.> It may also be
    /// be a source other than the request field (e.g. a macro definition
    /// referenced in the text of the query), in which case this is the name of
    /// the source (e.g. the macro name).
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// The position of the first byte within the text.
    #[prost(message, optional, tag = "2")]
    pub start_position: ::core::option::Option<text_locator::Position>,
    /// The position of the last byte within the text.
    #[prost(message, optional, tag = "3")]
    pub end_position: ::core::option::Option<text_locator::Position>,
    /// If `source`, `start_position`, and `end_position` describe a call on
    /// some object (e.g. a macro in the time series query language text) and a
    /// location is to be designated in that object's text, `nested_locator`
    /// identifies the location within that object.
    #[prost(message, optional, boxed, tag = "4")]
    pub nested_locator: ::core::option::Option<::prost::alloc::boxed::Box<TextLocator>>,
    /// When `nested_locator` is set, this field gives the reason for the nesting.
    /// Usually, the reason is a macro invocation. In that case, the macro name
    /// (including the leading '@') signals the location of the macro call
    /// in the text and a macro argument name (including the leading '$') signals
    /// the location of the macro argument inside the macro body that got
    /// substituted away.
    #[prost(string, tag = "5")]
    pub nesting_reason: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TextLocator`.
pub mod text_locator {
    /// The position of a byte within the text.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Position {
        /// The line, starting with 1, where the byte is positioned.
        #[prost(int32, tag = "1")]
        pub line: i32,
        /// The column within the line, starting with 1, where the byte is
        /// positioned. This is a byte index even though the text is UTF-8.
        #[prost(int32, tag = "2")]
        pub column: i32,
    }
}
/// A `Snooze` will prevent any alerts from being opened, and close any that
/// are already open. The `Snooze` will work on alerts that match the
/// criteria defined in the `Snooze`. The `Snooze` will be active from
/// `interval.start_time` through `interval.end_time`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snooze {
    /// Required. The name of the `Snooze`. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/snoozes/\[SNOOZE_ID\]
    ///
    /// The ID of the `Snooze` will be generated by the system.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. This defines the criteria for applying the `Snooze`. See
    /// `Criteria` for more information.
    #[prost(message, optional, tag = "3")]
    pub criteria: ::core::option::Option<snooze::Criteria>,
    /// Required. The `Snooze` will be active from `interval.start_time` through
    /// `interval.end_time`.
    /// `interval.start_time` cannot be in the past. There is a 15 second clock
    /// skew to account for the time it takes for a request to reach the API from
    /// the UI.
    #[prost(message, optional, tag = "4")]
    pub interval: ::core::option::Option<TimeInterval>,
    /// Required. A display name for the `Snooze`. This can be, at most, 512
    /// unicode characters.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Snooze`.
pub mod snooze {
    /// Criteria specific to the `AlertPolicy`s that this `Snooze` applies to. The
    /// `Snooze` will suppress alerts that come from one of the `AlertPolicy`s
    /// whose names are supplied.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Criteria {
        /// The specific `AlertPolicy` names for the alert that should be snoozed.
        /// The format is:
        ///
        ///      projects/\[PROJECT_ID_OR_NUMBER\]/alertPolicies/\[POLICY_ID\]
        ///
        /// There is a limit of 16 policies per snooze. This limit is checked during
        /// snooze creation.
        #[prost(string, repeated, tag = "1")]
        pub policies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// The message definition for creating a `Snooze`. Users must provide the body
/// of the `Snooze` to be created but must omit the `Snooze` field, `name`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSnoozeRequest {
    /// Required. The
    /// [project](<https://cloud.google.com/monitoring/api/v3#project_name>) in which
    /// a `Snooze` should be created. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The `Snooze` to create. Omit the `name` field, as it will be
    /// filled in by the API.
    #[prost(message, optional, tag = "2")]
    pub snooze: ::core::option::Option<Snooze>,
}
/// The message definition for listing `Snooze`s associated with the given
/// `parent`, satisfying the optional `filter`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnoozesRequest {
    /// Required. The
    /// [project](<https://cloud.google.com/monitoring/api/v3#project_name>) whose
    /// `Snooze`s should be listed. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Optional filter to restrict results to the given criteria. The
    /// following fields are supported.
    ///
    ///    * `interval.start_time`
    ///    * `interval.end_time`
    ///
    /// For example:
    ///
    ///      ```
    ///      interval.start_time > "2022-03-11T00:00:00-08:00" AND
    ///          interval.end_time < "2022-03-12T00:00:00-08:00"
    ///      ```
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return for a single query. The
    /// server may further constrain the maximum number of results returned in a
    /// single page. The value should be in the range \[1, 1000\]. If the value given
    /// is outside this range, the server will decide the number of results to be
    /// returned.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. The `next_page_token` from a previous call to
    /// `ListSnoozesRequest` to get the next page of results.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// The results of a successful `ListSnoozes` call, containing the matching
/// `Snooze`s.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnoozesResponse {
    /// `Snooze`s matching this list call.
    #[prost(message, repeated, tag = "1")]
    pub snoozes: ::prost::alloc::vec::Vec<Snooze>,
    /// Page token for repeated calls to `ListSnoozes`, to fetch additional pages
    /// of results. If this is empty or missing, there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The message definition for retrieving a `Snooze`. Users must specify the
/// field, `name`, which identifies the `Snooze`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSnoozeRequest {
    /// Required. The ID of the `Snooze` to retrieve. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/snoozes/\[SNOOZE_ID\]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The message definition for updating a `Snooze`. The field, `snooze.name`
/// identifies the `Snooze` to be updated. The remainder of `snooze` gives the
/// content the `Snooze` in question will be assigned.
///
/// What fields can be updated depends on the start time and end time of the
/// `Snooze`.
///
///    * end time is in the past: These `Snooze`s are considered
///      read-only and cannot be updated.
///    * start time is in the past and end time is in the future: `display_name`
///      and `interval.end_time` can be updated.
///    * start time is in the future: `display_name`, `interval.start_time` and
///      `interval.end_time` can be updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSnoozeRequest {
    /// Required. The `Snooze` to update. Must have the name field present.
    #[prost(message, optional, tag = "1")]
    pub snooze: ::core::option::Option<Snooze>,
    /// Required. The fields to update.
    ///
    /// For each field listed in `update_mask`:
    ///
    ///    * If the `Snooze` object supplied in the `UpdateSnoozeRequest` has a
    ///      value for that field, the value of the field in the existing `Snooze`
    ///      will be set to the value of the field in the supplied `Snooze`.
    ///    * If the field does not have a value in the supplied `Snooze`, the field
    ///      in the existing `Snooze` is set to its default value.
    ///
    /// Fields not listed retain their existing value.
    ///
    /// The following are the field names that are accepted in `update_mask`:
    ///
    ///    * `display_name`
    ///    * `interval.start_time`
    ///    * `interval.end_time`
    ///
    /// That said, the start time and end time of the `Snooze` determines which
    /// fields can legally be updated. Before attempting an update, users should
    /// consult the documentation for `UpdateSnoozeRequest`, which talks about
    /// which fields can be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Generated client implementations.
pub mod snooze_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The SnoozeService API is used to temporarily prevent an alert policy from
    /// generating alerts. A Snooze is a description of the criteria under which one
    /// or more alert policies should not fire alerts for the specified duration.
    #[derive(Debug, Clone)]
    pub struct SnoozeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SnoozeServiceClient<T>
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
        ) -> SnoozeServiceClient<InterceptedService<T, F>>
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
            SnoozeServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a `Snooze` that will prevent alerts, which match the provided
        /// criteria, from being opened. The `Snooze` applies for a specific time
        /// interval.
        pub async fn create_snooze(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSnoozeRequest>,
        ) -> std::result::Result<tonic::Response<super::Snooze>, tonic::Status> {
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
                "/google.monitoring.v3.SnoozeService/CreateSnooze",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.monitoring.v3.SnoozeService", "CreateSnooze"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the `Snooze`s associated with a project. Can optionally pass in
        /// `filter`, which specifies predicates to match `Snooze`s.
        pub async fn list_snoozes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSnoozesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSnoozesResponse>,
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
                "/google.monitoring.v3.SnoozeService/ListSnoozes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.monitoring.v3.SnoozeService", "ListSnoozes"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a `Snooze` by `name`.
        pub async fn get_snooze(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSnoozeRequest>,
        ) -> std::result::Result<tonic::Response<super::Snooze>, tonic::Status> {
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
                "/google.monitoring.v3.SnoozeService/GetSnooze",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.monitoring.v3.SnoozeService", "GetSnooze"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a `Snooze`, identified by its `name`, with the parameters in the
        /// given `Snooze` object.
        pub async fn update_snooze(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSnoozeRequest>,
        ) -> std::result::Result<tonic::Response<super::Snooze>, tonic::Status> {
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
                "/google.monitoring.v3.SnoozeService/UpdateSnooze",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.monitoring.v3.SnoozeService", "UpdateSnooze"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The context of a span. This is attached to an
/// [Exemplar][google.api.Distribution.Exemplar]
/// in [Distribution][google.api.Distribution] values during aggregation.
///
/// It contains the name of a span with format:
///
///      projects/\[PROJECT_ID_OR_NUMBER\]/traces/\[TRACE_ID\]/spans/\[SPAN_ID\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpanContext {
    /// The resource name of the span. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/traces/\[TRACE_ID\]/spans/\[SPAN_ID\]
    ///
    /// `\[TRACE_ID\]` is a unique identifier for a trace within a project;
    /// it is a 32-character hexadecimal encoding of a 16-byte array.
    ///
    /// `\[SPAN_ID\]` is a unique identifier for a span within a trace; it
    /// is a 16-character hexadecimal encoding of an 8-byte array.
    #[prost(string, tag = "1")]
    pub span_name: ::prost::alloc::string::String,
}
/// An internal checker allows Uptime checks to run on private/internal GCP
/// resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalChecker {
    /// A unique resource name for this InternalChecker. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/internalCheckers/\[INTERNAL_CHECKER_ID\]
    ///
    /// `\[PROJECT_ID_OR_NUMBER\]` is the Cloud Monitoring Metrics Scope project for
    /// the Uptime check config associated with the internal checker.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The checker's human-readable name. The display name
    /// should be unique within a Cloud Monitoring Metrics Scope in order to make
    /// it easier to identify; however, uniqueness is not enforced.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The [GCP VPC network](<https://cloud.google.com/vpc/docs/vpc>) where the
    /// internal resource lives (ex: "default").
    #[prost(string, tag = "3")]
    pub network: ::prost::alloc::string::String,
    /// The GCP zone the Uptime check should egress from. Only respected for
    /// internal Uptime checks, where internal_network is specified.
    #[prost(string, tag = "4")]
    pub gcp_zone: ::prost::alloc::string::String,
    /// The GCP project ID where the internal checker lives. Not necessary
    /// the same as the Metrics Scope project.
    #[prost(string, tag = "6")]
    pub peer_project_id: ::prost::alloc::string::String,
    /// The current operational state of the internal checker.
    #[prost(enumeration = "internal_checker::State", tag = "7")]
    pub state: i32,
}
/// Nested message and enum types in `InternalChecker`.
pub mod internal_checker {
    /// Operational states for an internal checker.
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
        /// An internal checker should never be in the unspecified state.
        Unspecified = 0,
        /// The checker is being created, provisioned, and configured. A checker in
        /// this state can be returned by `ListInternalCheckers` or
        /// `GetInternalChecker`, as well as by examining the [long running
        /// Operation](<https://cloud.google.com/apis/design/design_patterns#long_running_operations>)
        /// that created it.
        Creating = 1,
        /// The checker is running and available for use. A checker in this state
        /// can be returned by `ListInternalCheckers` or `GetInternalChecker` as
        /// well as by examining the [long running
        /// Operation](<https://cloud.google.com/apis/design/design_patterns#long_running_operations>)
        /// that created it.
        /// If a checker is being torn down, it is neither visible nor usable, so
        /// there is no "deleting" or "down" state.
        Running = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "UNSPECIFIED",
                State::Creating => "CREATING",
                State::Running => "RUNNING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "RUNNING" => Some(Self::Running),
                _ => None,
            }
        }
    }
}
/// This message configures which resources and services to monitor for
/// availability.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UptimeCheckConfig {
    /// A unique resource name for this Uptime check configuration. The format is:
    ///
    ///       projects/\[PROJECT_ID_OR_NUMBER\]/uptimeCheckConfigs/\[UPTIME_CHECK_ID\]
    ///
    /// `\[PROJECT_ID_OR_NUMBER\]` is the Workspace host project associated with the
    /// Uptime check.
    ///
    /// This field should be omitted when creating the Uptime check configuration;
    /// on create, the resource name is assigned by the server and included in the
    /// response.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A human-friendly name for the Uptime check configuration. The display name
    /// should be unique within a Cloud Monitoring Workspace in order to make it
    /// easier to identify; however, uniqueness is not enforced. Required.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// How often, in seconds, the Uptime check is performed.
    /// Currently, the only supported values are `60s` (1 minute), `300s`
    /// (5 minutes), `600s` (10 minutes), and `900s` (15 minutes). Optional,
    /// defaults to `60s`.
    #[prost(message, optional, tag = "7")]
    pub period: ::core::option::Option<::prost_types::Duration>,
    /// The maximum amount of time to wait for the request to complete (must be
    /// between 1 and 60 seconds). Required.
    #[prost(message, optional, tag = "8")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// The content that is expected to appear in the data returned by the target
    /// server against which the check is run.  Currently, only the first entry
    /// in the `content_matchers` list is supported, and additional entries will
    /// be ignored. This field is optional and should only be specified if a
    /// content match is required as part of the/ Uptime check.
    #[prost(message, repeated, tag = "9")]
    pub content_matchers: ::prost::alloc::vec::Vec<uptime_check_config::ContentMatcher>,
    /// The type of checkers to use to execute the Uptime check.
    #[prost(enumeration = "uptime_check_config::CheckerType", tag = "17")]
    pub checker_type: i32,
    /// The list of regions from which the check will be run.
    /// Some regions contain one location, and others contain more than one.
    /// If this field is specified, enough regions must be provided to include a
    /// minimum of 3 locations.  Not specifying this field will result in Uptime
    /// checks running from all available regions.
    #[prost(enumeration = "UptimeCheckRegion", repeated, tag = "10")]
    pub selected_regions: ::prost::alloc::vec::Vec<i32>,
    /// If this is `true`, then checks are made only from the 'internal_checkers'.
    /// If it is `false`, then checks are made only from the 'selected_regions'.
    /// It is an error to provide 'selected_regions' when is_internal is `true`,
    /// or to provide 'internal_checkers' when is_internal is `false`.
    #[deprecated]
    #[prost(bool, tag = "15")]
    pub is_internal: bool,
    /// The internal checkers that this check will egress from. If `is_internal` is
    /// `true` and this list is empty, the check will egress from all the
    /// InternalCheckers configured for the project that owns this
    /// `UptimeCheckConfig`.
    #[deprecated]
    #[prost(message, repeated, tag = "14")]
    pub internal_checkers: ::prost::alloc::vec::Vec<InternalChecker>,
    /// User-supplied key/value data to be used for organizing and
    /// identifying the `UptimeCheckConfig` objects.
    ///
    /// The field can contain up to 64 entries. Each key and value is limited to
    /// 63 Unicode characters or 128 bytes, whichever is smaller. Labels and
    /// values can contain only lowercase letters, numerals, underscores, and
    /// dashes. Keys must begin with a letter.
    #[prost(btree_map = "string, string", tag = "20")]
    pub user_labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The resource the check is checking. Required.
    #[prost(oneof = "uptime_check_config::Resource", tags = "3, 4")]
    pub resource: ::core::option::Option<uptime_check_config::Resource>,
    /// The type of Uptime check request.
    #[prost(oneof = "uptime_check_config::CheckRequestType", tags = "5, 6")]
    pub check_request_type: ::core::option::Option<
        uptime_check_config::CheckRequestType,
    >,
}
/// Nested message and enum types in `UptimeCheckConfig`.
pub mod uptime_check_config {
    /// The resource submessage for group checks. It can be used instead of a
    /// monitored resource, when multiple resources are being monitored.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceGroup {
        /// The group of resources being monitored. Should be only the `\[GROUP_ID\]`,
        /// and not the full-path
        /// `projects/\[PROJECT_ID_OR_NUMBER\]/groups/\[GROUP_ID\]`.
        #[prost(string, tag = "1")]
        pub group_id: ::prost::alloc::string::String,
        /// The resource type of the group members.
        #[prost(enumeration = "super::GroupResourceType", tag = "2")]
        pub resource_type: i32,
    }
    /// Information involved in sending ICMP pings alongside public HTTP/TCP
    /// checks. For HTTP, the pings are performed for each part of the redirect
    /// chain.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PingConfig {
        /// Number of ICMP pings. A maximum of 3 ICMP pings is currently supported.
        #[prost(int32, tag = "1")]
        pub pings_count: i32,
    }
    /// Information involved in an HTTP/HTTPS Uptime check request.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HttpCheck {
        /// The HTTP request method to use for the check. If set to
        /// `METHOD_UNSPECIFIED` then `request_method` defaults to `GET`.
        #[prost(enumeration = "http_check::RequestMethod", tag = "8")]
        pub request_method: i32,
        /// If `true`, use HTTPS instead of HTTP to run the check.
        #[prost(bool, tag = "1")]
        pub use_ssl: bool,
        /// Optional (defaults to "/"). The path to the page against which to run
        /// the check. Will be combined with the `host` (specified within the
        /// `monitored_resource`) and `port` to construct the full URL. If the
        /// provided path does not begin with "/", a "/" will be prepended
        /// automatically.
        #[prost(string, tag = "2")]
        pub path: ::prost::alloc::string::String,
        /// Optional (defaults to 80 when `use_ssl` is `false`, and 443 when
        /// `use_ssl` is `true`). The TCP port on the HTTP server against which to
        /// run the check. Will be combined with host (specified within the
        /// `monitored_resource`) and `path` to construct the full URL.
        #[prost(int32, tag = "3")]
        pub port: i32,
        /// The authentication information. Optional when creating an HTTP check;
        /// defaults to empty.
        #[prost(message, optional, tag = "4")]
        pub auth_info: ::core::option::Option<http_check::BasicAuthentication>,
        /// Boolean specifying whether to encrypt the header information.
        /// Encryption should be specified for any headers related to authentication
        /// that you do not wish to be seen when retrieving the configuration. The
        /// server will be responsible for encrypting the headers.
        /// On Get/List calls, if `mask_headers` is set to `true` then the headers
        /// will be obscured with `******.`
        #[prost(bool, tag = "5")]
        pub mask_headers: bool,
        /// The list of headers to send as part of the Uptime check request.
        /// If two headers have the same key and different values, they should
        /// be entered as a single header, with the value being a comma-separated
        /// list of all the desired values as described at
        /// <https://www.w3.org/Protocols/rfc2616/rfc2616.txt> (page 31).
        /// Entering two separate headers with the same key in a Create call will
        /// cause the first to be overwritten by the second.
        /// The maximum number of headers allowed is 100.
        #[prost(btree_map = "string, string", tag = "6")]
        pub headers: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// The content type header to use for the check. The following
        /// configurations result in errors:
        /// 1. Content type is specified in both the `headers` field and the
        /// `content_type` field.
        /// 2. Request method is `GET` and `content_type` is not `TYPE_UNSPECIFIED`
        /// 3. Request method is `POST` and `content_type` is `TYPE_UNSPECIFIED`.
        /// 4. Request method is `POST` and a "Content-Type" header is provided via
        /// `headers` field. The `content_type` field should be used instead.
        #[prost(enumeration = "http_check::ContentType", tag = "9")]
        pub content_type: i32,
        /// A user provided content type header to use for the check. The invalid
        /// configurations outlined in the `content_type` field apply to
        /// `custom_content_type`, as well as the following:
        /// 1. `content_type` is `URL_ENCODED` and `custom_content_type` is set.
        /// 2. `content_type` is `USER_PROVIDED` and `custom_content_type` is not
        /// set.
        #[prost(string, tag = "13")]
        pub custom_content_type: ::prost::alloc::string::String,
        /// Boolean specifying whether to include SSL certificate validation as a
        /// part of the Uptime check. Only applies to checks where
        /// `monitored_resource` is set to `uptime_url`. If `use_ssl` is `false`,
        /// setting `validate_ssl` to `true` has no effect.
        #[prost(bool, tag = "7")]
        pub validate_ssl: bool,
        /// The request body associated with the HTTP POST request. If `content_type`
        /// is `URL_ENCODED`, the body passed in must be URL-encoded. Users can
        /// provide a `Content-Length` header via the `headers` field or the API will
        /// do so. If the `request_method` is `GET` and `body` is not empty, the API
        /// will return an error. The maximum byte size is 1 megabyte.
        ///
        /// Note: If client libraries aren't used (which performs the conversion
        /// automatically) base64 encode your `body` data since the field is of
        /// `bytes` type.
        #[prost(bytes = "bytes", tag = "10")]
        pub body: ::prost::bytes::Bytes,
        /// If present, the check will only pass if the HTTP response status code is
        /// in this set of status codes. If empty, the HTTP status code will only
        /// pass if the HTTP status code is 200-299.
        #[prost(message, repeated, tag = "11")]
        pub accepted_response_status_codes: ::prost::alloc::vec::Vec<
            http_check::ResponseStatusCode,
        >,
        /// Contains information needed to add pings to an HTTP check.
        #[prost(message, optional, tag = "12")]
        pub ping_config: ::core::option::Option<PingConfig>,
    }
    /// Nested message and enum types in `HttpCheck`.
    pub mod http_check {
        /// The authentication parameters to provide to the specified resource or
        /// URL that requires a username and password. Currently, only
        /// [Basic HTTP authentication](<https://tools.ietf.org/html/rfc7617>) is
        /// supported in Uptime checks.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct BasicAuthentication {
            /// The username to use when authenticating with the HTTP server.
            #[prost(string, tag = "1")]
            pub username: ::prost::alloc::string::String,
            /// The password to use when authenticating with the HTTP server.
            #[prost(string, tag = "2")]
            pub password: ::prost::alloc::string::String,
        }
        /// A status to accept. Either a status code class like "2xx", or an integer
        /// status code like "200".
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResponseStatusCode {
            /// Either a specific value or a class of status codes.
            #[prost(oneof = "response_status_code::StatusCode", tags = "1, 2")]
            pub status_code: ::core::option::Option<response_status_code::StatusCode>,
        }
        /// Nested message and enum types in `ResponseStatusCode`.
        pub mod response_status_code {
            /// An HTTP status code class.
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
            pub enum StatusClass {
                /// Default value that matches no status codes.
                Unspecified = 0,
                /// The class of status codes between 100 and 199.
                StatusClass1xx = 100,
                /// The class of status codes between 200 and 299.
                StatusClass2xx = 200,
                /// The class of status codes between 300 and 399.
                StatusClass3xx = 300,
                /// The class of status codes between 400 and 499.
                StatusClass4xx = 400,
                /// The class of status codes between 500 and 599.
                StatusClass5xx = 500,
                /// The class of all status codes.
                Any = 1000,
            }
            impl StatusClass {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        StatusClass::Unspecified => "STATUS_CLASS_UNSPECIFIED",
                        StatusClass::StatusClass1xx => "STATUS_CLASS_1XX",
                        StatusClass::StatusClass2xx => "STATUS_CLASS_2XX",
                        StatusClass::StatusClass3xx => "STATUS_CLASS_3XX",
                        StatusClass::StatusClass4xx => "STATUS_CLASS_4XX",
                        StatusClass::StatusClass5xx => "STATUS_CLASS_5XX",
                        StatusClass::Any => "STATUS_CLASS_ANY",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "STATUS_CLASS_UNSPECIFIED" => Some(Self::Unspecified),
                        "STATUS_CLASS_1XX" => Some(Self::StatusClass1xx),
                        "STATUS_CLASS_2XX" => Some(Self::StatusClass2xx),
                        "STATUS_CLASS_3XX" => Some(Self::StatusClass3xx),
                        "STATUS_CLASS_4XX" => Some(Self::StatusClass4xx),
                        "STATUS_CLASS_5XX" => Some(Self::StatusClass5xx),
                        "STATUS_CLASS_ANY" => Some(Self::Any),
                        _ => None,
                    }
                }
            }
            /// Either a specific value or a class of status codes.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum StatusCode {
                /// A status code to accept.
                #[prost(int32, tag = "1")]
                StatusValue(i32),
                /// A class of status codes to accept.
                #[prost(enumeration = "StatusClass", tag = "2")]
                StatusClass(i32),
            }
        }
        /// The HTTP request method options.
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
        pub enum RequestMethod {
            /// No request method specified.
            MethodUnspecified = 0,
            /// GET request.
            Get = 1,
            /// POST request.
            Post = 2,
        }
        impl RequestMethod {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    RequestMethod::MethodUnspecified => "METHOD_UNSPECIFIED",
                    RequestMethod::Get => "GET",
                    RequestMethod::Post => "POST",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "METHOD_UNSPECIFIED" => Some(Self::MethodUnspecified),
                    "GET" => Some(Self::Get),
                    "POST" => Some(Self::Post),
                    _ => None,
                }
            }
        }
        /// Header options corresponding to the content type of a HTTP request body.
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
        pub enum ContentType {
            /// No content type specified.
            TypeUnspecified = 0,
            /// `body` is in URL-encoded form. Equivalent to setting the `Content-Type`
            /// to `application/x-www-form-urlencoded` in the HTTP request.
            UrlEncoded = 1,
            /// `body` is in `custom_content_type` form. Equivalent to setting the
            /// `Content-Type` to the contents of `custom_content_type` in the HTTP
            /// request.
            UserProvided = 2,
        }
        impl ContentType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ContentType::TypeUnspecified => "TYPE_UNSPECIFIED",
                    ContentType::UrlEncoded => "URL_ENCODED",
                    ContentType::UserProvided => "USER_PROVIDED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TYPE_UNSPECIFIED" => Some(Self::TypeUnspecified),
                    "URL_ENCODED" => Some(Self::UrlEncoded),
                    "USER_PROVIDED" => Some(Self::UserProvided),
                    _ => None,
                }
            }
        }
    }
    /// Information required for a TCP Uptime check request.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TcpCheck {
        /// The TCP port on the server against which to run the check. Will be
        /// combined with host (specified within the `monitored_resource`) to
        /// construct the full URL. Required.
        #[prost(int32, tag = "1")]
        pub port: i32,
        /// Contains information needed to add pings to a TCP check.
        #[prost(message, optional, tag = "2")]
        pub ping_config: ::core::option::Option<PingConfig>,
    }
    /// Optional. Used to perform content matching. This allows matching based on
    /// substrings and regular expressions, together with their negations. Only the
    /// first 4&nbsp;MB of an HTTP or HTTPS check's response (and the first
    /// 1&nbsp;MB of a TCP check's response) are examined for purposes of content
    /// matching.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ContentMatcher {
        /// String, regex or JSON content to match. Maximum 1024 bytes. An empty
        /// `content` string indicates no content matching is to be performed.
        #[prost(string, tag = "1")]
        pub content: ::prost::alloc::string::String,
        /// The type of content matcher that will be applied to the server output,
        /// compared to the `content` string when the check is run.
        #[prost(enumeration = "content_matcher::ContentMatcherOption", tag = "2")]
        pub matcher: i32,
        /// Certain `ContentMatcherOption` types require additional information.
        /// `MATCHES_JSON_PATH` or `NOT_MATCHES_JSON_PATH` require a
        /// `JsonPathMatcher`; not used for other options.
        #[prost(oneof = "content_matcher::AdditionalMatcherInfo", tags = "3")]
        pub additional_matcher_info: ::core::option::Option<
            content_matcher::AdditionalMatcherInfo,
        >,
    }
    /// Nested message and enum types in `ContentMatcher`.
    pub mod content_matcher {
        /// Information needed to perform a JSONPath content match.
        /// Used for `ContentMatcherOption::MATCHES_JSON_PATH` and
        /// `ContentMatcherOption::NOT_MATCHES_JSON_PATH`.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct JsonPathMatcher {
            /// JSONPath within the response output pointing to the expected
            /// `ContentMatcher::content` to match against.
            #[prost(string, tag = "1")]
            pub json_path: ::prost::alloc::string::String,
            /// The type of JSONPath match that will be applied to the JSON output
            /// (`ContentMatcher.content`)
            #[prost(enumeration = "json_path_matcher::JsonPathMatcherOption", tag = "2")]
            pub json_matcher: i32,
        }
        /// Nested message and enum types in `JsonPathMatcher`.
        pub mod json_path_matcher {
            /// Options to perform JSONPath content matching.
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
            pub enum JsonPathMatcherOption {
                /// No JSONPath matcher type specified (not valid).
                Unspecified = 0,
                /// Selects 'exact string' matching. The match succeeds if the content at
                /// the `json_path` within the output is exactly the same as the
                /// `content` string.
                ExactMatch = 1,
                /// Selects regular-expression matching. The match succeeds if the
                /// content at the `json_path` within the output matches the regular
                /// expression specified in the `content` string.
                RegexMatch = 2,
            }
            impl JsonPathMatcherOption {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        JsonPathMatcherOption::Unspecified => {
                            "JSON_PATH_MATCHER_OPTION_UNSPECIFIED"
                        }
                        JsonPathMatcherOption::ExactMatch => "EXACT_MATCH",
                        JsonPathMatcherOption::RegexMatch => "REGEX_MATCH",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "JSON_PATH_MATCHER_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
                        "EXACT_MATCH" => Some(Self::ExactMatch),
                        "REGEX_MATCH" => Some(Self::RegexMatch),
                        _ => None,
                    }
                }
            }
        }
        /// Options to perform content matching.
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
        pub enum ContentMatcherOption {
            /// No content matcher type specified (maintained for backward
            /// compatibility, but deprecated for future use).
            /// Treated as `CONTAINS_STRING`.
            Unspecified = 0,
            /// Selects substring matching. The match succeeds if the output contains
            /// the `content` string.  This is the default value for checks without
            /// a `matcher` option, or where the value of `matcher` is
            /// `CONTENT_MATCHER_OPTION_UNSPECIFIED`.
            ContainsString = 1,
            /// Selects negation of substring matching. The match succeeds if the
            /// output does _NOT_ contain the `content` string.
            NotContainsString = 2,
            /// Selects regular-expression matching. The match succeeds if the output
            /// matches the regular expression specified in the `content` string.
            /// Regex matching is only supported for HTTP/HTTPS checks.
            MatchesRegex = 3,
            /// Selects negation of regular-expression matching. The match succeeds if
            /// the output does _NOT_ match the regular expression specified in the
            /// `content` string. Regex matching is only supported for HTTP/HTTPS
            /// checks.
            NotMatchesRegex = 4,
            /// Selects JSONPath matching. See `JsonPathMatcher` for details on when
            /// the match succeeds. JSONPath matching is only supported for HTTP/HTTPS
            /// checks.
            MatchesJsonPath = 5,
            /// Selects JSONPath matching. See `JsonPathMatcher` for details on when
            /// the match succeeds. Succeeds when output does _NOT_ match as specified.
            /// JSONPath is only supported for HTTP/HTTPS checks.
            NotMatchesJsonPath = 6,
        }
        impl ContentMatcherOption {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ContentMatcherOption::Unspecified => {
                        "CONTENT_MATCHER_OPTION_UNSPECIFIED"
                    }
                    ContentMatcherOption::ContainsString => "CONTAINS_STRING",
                    ContentMatcherOption::NotContainsString => "NOT_CONTAINS_STRING",
                    ContentMatcherOption::MatchesRegex => "MATCHES_REGEX",
                    ContentMatcherOption::NotMatchesRegex => "NOT_MATCHES_REGEX",
                    ContentMatcherOption::MatchesJsonPath => "MATCHES_JSON_PATH",
                    ContentMatcherOption::NotMatchesJsonPath => "NOT_MATCHES_JSON_PATH",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "CONTENT_MATCHER_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
                    "CONTAINS_STRING" => Some(Self::ContainsString),
                    "NOT_CONTAINS_STRING" => Some(Self::NotContainsString),
                    "MATCHES_REGEX" => Some(Self::MatchesRegex),
                    "NOT_MATCHES_REGEX" => Some(Self::NotMatchesRegex),
                    "MATCHES_JSON_PATH" => Some(Self::MatchesJsonPath),
                    "NOT_MATCHES_JSON_PATH" => Some(Self::NotMatchesJsonPath),
                    _ => None,
                }
            }
        }
        /// Certain `ContentMatcherOption` types require additional information.
        /// `MATCHES_JSON_PATH` or `NOT_MATCHES_JSON_PATH` require a
        /// `JsonPathMatcher`; not used for other options.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum AdditionalMatcherInfo {
            /// Matcher information for `MATCHES_JSON_PATH` and `NOT_MATCHES_JSON_PATH`
            #[prost(message, tag = "3")]
            JsonPathMatcher(JsonPathMatcher),
        }
    }
    /// What kind of checkers are available to be used by the check.
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
    pub enum CheckerType {
        /// The default checker type. Currently converted to `STATIC_IP_CHECKERS`
        /// on creation, the default conversion behavior may change in the future.
        Unspecified = 0,
        /// `STATIC_IP_CHECKERS` are used for uptime checks that perform egress
        /// across the public internet. `STATIC_IP_CHECKERS` use the static IP
        /// addresses returned by `ListUptimeCheckIps`.
        StaticIpCheckers = 1,
        /// `VPC_CHECKERS` are used for uptime checks that perform egress using
        /// Service Directory and private network access. When using `VPC_CHECKERS`,
        /// the monitored resource type must be `servicedirectory_service`.
        VpcCheckers = 3,
    }
    impl CheckerType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CheckerType::Unspecified => "CHECKER_TYPE_UNSPECIFIED",
                CheckerType::StaticIpCheckers => "STATIC_IP_CHECKERS",
                CheckerType::VpcCheckers => "VPC_CHECKERS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CHECKER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATIC_IP_CHECKERS" => Some(Self::StaticIpCheckers),
                "VPC_CHECKERS" => Some(Self::VpcCheckers),
                _ => None,
            }
        }
    }
    /// The resource the check is checking. Required.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        /// The [monitored
        /// resource](<https://cloud.google.com/monitoring/api/resources>) associated
        /// with the configuration.
        /// The following monitored resource types are valid for this field:
        ///    `uptime_url`,
        ///    `gce_instance`,
        ///    `gae_app`,
        ///    `aws_ec2_instance`,
        ///    `aws_elb_load_balancer`
        ///    `k8s_service`
        ///    `servicedirectory_service`
        ///    `cloud_run_revision`
        #[prost(message, tag = "3")]
        MonitoredResource(super::super::super::api::MonitoredResource),
        /// The group resource associated with the configuration.
        #[prost(message, tag = "4")]
        ResourceGroup(ResourceGroup),
    }
    /// The type of Uptime check request.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CheckRequestType {
        /// Contains information needed to make an HTTP or HTTPS check.
        #[prost(message, tag = "5")]
        HttpCheck(HttpCheck),
        /// Contains information needed to make a TCP check.
        #[prost(message, tag = "6")]
        TcpCheck(TcpCheck),
    }
}
/// Contains the region, location, and list of IP
/// addresses where checkers in the location run from.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UptimeCheckIp {
    /// A broad region category in which the IP address is located.
    #[prost(enumeration = "UptimeCheckRegion", tag = "1")]
    pub region: i32,
    /// A more specific location within the region that typically encodes
    /// a particular city/town/metro (and its containing state/province or country)
    /// within the broader umbrella region category.
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
    /// The IP address from which the Uptime check originates. This is a fully
    /// specified IP address (not an IP address range). Most IP addresses, as of
    /// this publication, are in IPv4 format; however, one should not rely on the
    /// IP addresses being in IPv4 format indefinitely, and should support
    /// interpreting this field in either IPv4 or IPv6 format.
    #[prost(string, tag = "3")]
    pub ip_address: ::prost::alloc::string::String,
}
/// The regions from which an Uptime check can be run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UptimeCheckRegion {
    /// Default value if no region is specified. Will result in Uptime checks
    /// running from all regions.
    RegionUnspecified = 0,
    /// Allows checks to run from locations within the United States of America.
    Usa = 1,
    /// Allows checks to run from locations within the continent of Europe.
    Europe = 2,
    /// Allows checks to run from locations within the continent of South
    /// America.
    SouthAmerica = 3,
    /// Allows checks to run from locations within the Asia Pacific area (ex:
    /// Singapore).
    AsiaPacific = 4,
    /// Allows checks to run from locations within the western United States of
    /// America
    UsaOregon = 5,
    /// Allows checks to run from locations within the central United States of
    /// America
    UsaIowa = 6,
    /// Allows checks to run from locations within the eastern United States of
    /// America
    UsaVirginia = 7,
}
impl UptimeCheckRegion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UptimeCheckRegion::RegionUnspecified => "REGION_UNSPECIFIED",
            UptimeCheckRegion::Usa => "USA",
            UptimeCheckRegion::Europe => "EUROPE",
            UptimeCheckRegion::SouthAmerica => "SOUTH_AMERICA",
            UptimeCheckRegion::AsiaPacific => "ASIA_PACIFIC",
            UptimeCheckRegion::UsaOregon => "USA_OREGON",
            UptimeCheckRegion::UsaIowa => "USA_IOWA",
            UptimeCheckRegion::UsaVirginia => "USA_VIRGINIA",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REGION_UNSPECIFIED" => Some(Self::RegionUnspecified),
            "USA" => Some(Self::Usa),
            "EUROPE" => Some(Self::Europe),
            "SOUTH_AMERICA" => Some(Self::SouthAmerica),
            "ASIA_PACIFIC" => Some(Self::AsiaPacific),
            "USA_OREGON" => Some(Self::UsaOregon),
            "USA_IOWA" => Some(Self::UsaIowa),
            "USA_VIRGINIA" => Some(Self::UsaVirginia),
            _ => None,
        }
    }
}
/// The supported resource types that can be used as values of
/// `group_resource.resource_type`.
/// `INSTANCE` includes `gce_instance` and `aws_ec2_instance` resource types.
/// The resource types `gae_app` and `uptime_url` are not valid here because
/// group checks on App Engine modules and URLs are not allowed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GroupResourceType {
    /// Default value (not valid).
    ResourceTypeUnspecified = 0,
    /// A group of instances from Google Cloud Platform (GCP) or
    /// Amazon Web Services (AWS).
    Instance = 1,
    /// A group of Amazon ELB load balancers.
    AwsElbLoadBalancer = 2,
}
impl GroupResourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GroupResourceType::ResourceTypeUnspecified => "RESOURCE_TYPE_UNSPECIFIED",
            GroupResourceType::Instance => "INSTANCE",
            GroupResourceType::AwsElbLoadBalancer => "AWS_ELB_LOAD_BALANCER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESOURCE_TYPE_UNSPECIFIED" => Some(Self::ResourceTypeUnspecified),
            "INSTANCE" => Some(Self::Instance),
            "AWS_ELB_LOAD_BALANCER" => Some(Self::AwsElbLoadBalancer),
            _ => None,
        }
    }
}
/// The protocol for the `ListUptimeCheckConfigs` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUptimeCheckConfigsRequest {
    /// Required. The
    /// [project](<https://cloud.google.com/monitoring/api/v3#project_name>) whose
    /// Uptime check configurations are listed. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// If provided, this field specifies the criteria that must be met by
    /// uptime checks to be included in the response.
    ///
    /// For more details, see [Filtering
    /// syntax](<https://cloud.google.com/monitoring/api/v3/sorting-and-filtering#filter_syntax>).
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. The server
    /// may further constrain the maximum number of results returned in a single
    /// page. If the page_size is <=0, the server will decide the number of results
    /// to be returned.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return more results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The protocol for the `ListUptimeCheckConfigs` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUptimeCheckConfigsResponse {
    /// The returned Uptime check configurations.
    #[prost(message, repeated, tag = "1")]
    pub uptime_check_configs: ::prost::alloc::vec::Vec<UptimeCheckConfig>,
    /// This field represents the pagination token to retrieve the next page of
    /// results. If the value is empty, it means no further results for the
    /// request. To retrieve the next page of results, the value of the
    /// next_page_token is passed to the subsequent List method call (in the
    /// request message's page_token field).
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of Uptime check configurations for the project,
    /// irrespective of any pagination.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// The protocol for the `GetUptimeCheckConfig` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUptimeCheckConfigRequest {
    /// Required. The Uptime check configuration to retrieve. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/uptimeCheckConfigs/\[UPTIME_CHECK_ID\]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The protocol for the `CreateUptimeCheckConfig` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUptimeCheckConfigRequest {
    /// Required. The
    /// [project](<https://cloud.google.com/monitoring/api/v3#project_name>) in which
    /// to create the Uptime check. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The new Uptime check configuration.
    #[prost(message, optional, tag = "2")]
    pub uptime_check_config: ::core::option::Option<UptimeCheckConfig>,
}
/// The protocol for the `UpdateUptimeCheckConfig` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUptimeCheckConfigRequest {
    /// Optional. If present, only the listed fields in the current Uptime check
    /// configuration are updated with values from the new configuration. If this
    /// field is empty, then the current configuration is completely replaced with
    /// the new configuration.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. If an `updateMask` has been specified, this field gives
    /// the values for the set of fields mentioned in the `updateMask`. If an
    /// `updateMask` has not been given, this Uptime check configuration replaces
    /// the current configuration. If a field is mentioned in `updateMask` but
    /// the corresponding field is omitted in this partial Uptime check
    /// configuration, it has the effect of deleting/clearing the field from the
    /// configuration on the server.
    ///
    /// The following fields can be updated: `display_name`,
    /// `http_check`, `tcp_check`, `timeout`, `content_matchers`, and
    /// `selected_regions`.
    #[prost(message, optional, tag = "3")]
    pub uptime_check_config: ::core::option::Option<UptimeCheckConfig>,
}
/// The protocol for the `DeleteUptimeCheckConfig` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUptimeCheckConfigRequest {
    /// Required. The Uptime check configuration to delete. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/uptimeCheckConfigs/\[UPTIME_CHECK_ID\]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The protocol for the `ListUptimeCheckIps` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUptimeCheckIpsRequest {
    /// The maximum number of results to return in a single response. The server
    /// may further constrain the maximum number of results returned in a single
    /// page. If the page_size is <=0, the server will decide the number of results
    /// to be returned.
    /// NOTE: this field is not yet implemented
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return more results from the previous method call.
    /// NOTE: this field is not yet implemented
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The protocol for the `ListUptimeCheckIps` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUptimeCheckIpsResponse {
    /// The returned list of IP addresses (including region and location) that the
    /// checkers run from.
    #[prost(message, repeated, tag = "1")]
    pub uptime_check_ips: ::prost::alloc::vec::Vec<UptimeCheckIp>,
    /// This field represents the pagination token to retrieve the next page of
    /// results. If the value is empty, it means no further results for the
    /// request. To retrieve the next page of results, the value of the
    /// next_page_token is passed to the subsequent List method call (in the
    /// request message's page_token field).
    /// NOTE: this field is not yet implemented
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod uptime_check_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The UptimeCheckService API is used to manage (list, create, delete, edit)
    /// Uptime check configurations in the Cloud Monitoring product. An Uptime
    /// check is a piece of configuration that determines which resources and
    /// services to monitor for availability. These configurations can also be
    /// configured interactively by navigating to the [Cloud console]
    /// (https://console.cloud.google.com), selecting the appropriate project,
    /// clicking on "Monitoring" on the left-hand side to navigate to Cloud
    /// Monitoring, and then clicking on "Uptime".
    #[derive(Debug, Clone)]
    pub struct UptimeCheckServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UptimeCheckServiceClient<T>
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
        ) -> UptimeCheckServiceClient<InterceptedService<T, F>>
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
            UptimeCheckServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists the existing valid Uptime check configurations for the project
        /// (leaving out any invalid configurations).
        pub async fn list_uptime_check_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUptimeCheckConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUptimeCheckConfigsResponse>,
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
                "/google.monitoring.v3.UptimeCheckService/ListUptimeCheckConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.UptimeCheckService",
                        "ListUptimeCheckConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a single Uptime check configuration.
        pub async fn get_uptime_check_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUptimeCheckConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UptimeCheckConfig>,
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
                "/google.monitoring.v3.UptimeCheckService/GetUptimeCheckConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.UptimeCheckService",
                        "GetUptimeCheckConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Uptime check configuration.
        pub async fn create_uptime_check_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUptimeCheckConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UptimeCheckConfig>,
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
                "/google.monitoring.v3.UptimeCheckService/CreateUptimeCheckConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.UptimeCheckService",
                        "CreateUptimeCheckConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an Uptime check configuration. You can either replace the entire
        /// configuration with a new one or replace only certain fields in the current
        /// configuration by specifying the fields to be updated via `updateMask`.
        /// Returns the updated configuration.
        pub async fn update_uptime_check_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUptimeCheckConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UptimeCheckConfig>,
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
                "/google.monitoring.v3.UptimeCheckService/UpdateUptimeCheckConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.UptimeCheckService",
                        "UpdateUptimeCheckConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an Uptime check configuration. Note that this method will fail
        /// if the Uptime check configuration is referenced by an alert policy or
        /// other dependent configs that would be rendered invalid by the deletion.
        pub async fn delete_uptime_check_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUptimeCheckConfigRequest>,
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
                "/google.monitoring.v3.UptimeCheckService/DeleteUptimeCheckConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.UptimeCheckService",
                        "DeleteUptimeCheckConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the list of IP addresses that checkers run from
        pub async fn list_uptime_check_ips(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUptimeCheckIpsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUptimeCheckIpsResponse>,
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
                "/google.monitoring.v3.UptimeCheckService/ListUptimeCheckIps",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.UptimeCheckService",
                        "ListUptimeCheckIps",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The `ListMonitoredResourceDescriptors` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMonitoredResourceDescriptorsRequest {
    /// Required. The [project](<https://cloud.google.com/monitoring/api/v3#project_name>) on
    /// which to execute the request. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// An optional [filter](<https://cloud.google.com/monitoring/api/v3/filters>)
    /// describing the descriptors to be returned.  The filter can reference the
    /// descriptor's type and labels. For example, the following filter returns
    /// only Google Compute Engine descriptors that have an `id` label:
    ///
    ///      resource.type = starts_with("gce_") AND resource.label:id
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// A positive number that is the maximum number of results to return.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The `ListMonitoredResourceDescriptors` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMonitoredResourceDescriptorsResponse {
    /// The monitored resource descriptors that are available to this project
    /// and that match `filter`, if present.
    #[prost(message, repeated, tag = "1")]
    pub resource_descriptors: ::prost::alloc::vec::Vec<
        super::super::api::MonitoredResourceDescriptor,
    >,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The `GetMonitoredResourceDescriptor` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMonitoredResourceDescriptorRequest {
    /// Required. The monitored resource descriptor to get.  The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/monitoredResourceDescriptors/\[RESOURCE_TYPE\]
    ///
    /// The `\[RESOURCE_TYPE\]` is a predefined type, such as
    /// `cloudsql_database`.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// The `ListMetricDescriptors` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetricDescriptorsRequest {
    /// Required. The [project](<https://cloud.google.com/monitoring/api/v3#project_name>) on
    /// which to execute the request. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// If this field is empty, all custom and
    /// system-defined metric descriptors are returned.
    /// Otherwise, the [filter](<https://cloud.google.com/monitoring/api/v3/filters>)
    /// specifies which metric descriptors are to be
    /// returned. For example, the following filter matches all
    /// [custom metrics](<https://cloud.google.com/monitoring/custom-metrics>):
    ///
    ///      metric.type = starts_with("custom.googleapis.com/")
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// A positive number that is the maximum number of results to return.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The `ListMetricDescriptors` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetricDescriptorsResponse {
    /// The metric descriptors that are available to the project
    /// and that match the value of `filter`, if present.
    #[prost(message, repeated, tag = "1")]
    pub metric_descriptors: ::prost::alloc::vec::Vec<
        super::super::api::MetricDescriptor,
    >,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The `GetMetricDescriptor` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetricDescriptorRequest {
    /// Required. The metric descriptor on which to execute the request. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/metricDescriptors/\[METRIC_ID\]
    ///
    /// An example value of `\[METRIC_ID\]` is
    /// `"compute.googleapis.com/instance/disk/read_bytes_count"`.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// The `CreateMetricDescriptor` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMetricDescriptorRequest {
    /// Required. The [project](<https://cloud.google.com/monitoring/api/v3#project_name>) on
    /// which to execute the request. The format is:
    /// 4
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new [custom metric](<https://cloud.google.com/monitoring/custom-metrics>)
    /// descriptor.
    #[prost(message, optional, tag = "2")]
    pub metric_descriptor: ::core::option::Option<super::super::api::MetricDescriptor>,
}
/// The `DeleteMetricDescriptor` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMetricDescriptorRequest {
    /// Required. The metric descriptor on which to execute the request. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/metricDescriptors/\[METRIC_ID\]
    ///
    /// An example of `\[METRIC_ID\]` is:
    /// `"custom.googleapis.com/my_test_metric"`.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// The `ListTimeSeries` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTimeSeriesRequest {
    /// Required. The [project](<https://cloud.google.com/monitoring/api/v3#project_name>),
    /// organization or folder on which to execute the request. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    ///      organizations/\[ORGANIZATION_ID\]
    ///      folders/\[FOLDER_ID\]
    #[prost(string, tag = "10")]
    pub name: ::prost::alloc::string::String,
    /// Required. A [monitoring filter](<https://cloud.google.com/monitoring/api/v3/filters>)
    /// that specifies which time series should be returned.  The filter must
    /// specify a single metric type, and can additionally specify metric labels
    /// and other information. For example:
    ///
    ///      metric.type = "compute.googleapis.com/instance/cpu/usage_time" AND
    ///          metric.labels.instance_name = "my-instance-name"
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Required. The time interval for which results should be returned. Only time series
    /// that contain data points in the specified interval are included
    /// in the response.
    #[prost(message, optional, tag = "4")]
    pub interval: ::core::option::Option<TimeInterval>,
    /// Specifies the alignment of data points in individual time series as
    /// well as how to combine the retrieved time series across specified labels.
    ///
    /// By default (if no `aggregation` is explicitly specified), the raw time
    /// series data is returned.
    #[prost(message, optional, tag = "5")]
    pub aggregation: ::core::option::Option<Aggregation>,
    /// Apply a second aggregation after `aggregation` is applied. May only be
    /// specified if `aggregation` is specified.
    #[prost(message, optional, tag = "11")]
    pub secondary_aggregation: ::core::option::Option<Aggregation>,
    /// Unsupported: must be left blank. The points in each time series are
    /// currently returned in reverse time order (most recent to oldest).
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
    /// Required. Specifies which information is returned about the time series.
    #[prost(enumeration = "list_time_series_request::TimeSeriesView", tag = "7")]
    pub view: i32,
    /// A positive number that is the maximum number of results to return. If
    /// `page_size` is empty or more than 100,000 results, the effective
    /// `page_size` is 100,000 results. If `view` is set to `FULL`, this is the
    /// maximum number of `Points` returned. If `view` is set to `HEADERS`, this is
    /// the maximum number of `TimeSeries` returned.
    #[prost(int32, tag = "8")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "9")]
    pub page_token: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ListTimeSeriesRequest`.
pub mod list_time_series_request {
    /// Controls which fields are returned by `ListTimeSeries`.
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
    pub enum TimeSeriesView {
        /// Returns the identity of the metric(s), the time series,
        /// and the time series data.
        Full = 0,
        /// Returns the identity of the metric and the time series resource,
        /// but not the time series data.
        Headers = 1,
    }
    impl TimeSeriesView {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimeSeriesView::Full => "FULL",
                TimeSeriesView::Headers => "HEADERS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FULL" => Some(Self::Full),
                "HEADERS" => Some(Self::Headers),
                _ => None,
            }
        }
    }
}
/// The `ListTimeSeries` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTimeSeriesResponse {
    /// One or more time series that match the filter included in the request.
    #[prost(message, repeated, tag = "1")]
    pub time_series: ::prost::alloc::vec::Vec<TimeSeries>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Query execution errors that may have caused the time series data returned
    /// to be incomplete.
    #[prost(message, repeated, tag = "3")]
    pub execution_errors: ::prost::alloc::vec::Vec<super::super::rpc::Status>,
    /// The unit in which all `time_series` point values are reported. `unit`
    /// follows the UCUM format for units as seen in
    /// <https://unitsofmeasure.org/ucum.html.>
    /// If different `time_series` have different units (for example, because they
    /// come from different metric types, or a unit is absent), then `unit` will be
    /// "{not_a_unit}".
    #[prost(string, tag = "5")]
    pub unit: ::prost::alloc::string::String,
}
/// The `CreateTimeSeries` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTimeSeriesRequest {
    /// Required. The [project](<https://cloud.google.com/monitoring/api/v3#project_name>) on
    /// which to execute the request. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new data to be added to a list of time series.
    /// Adds at most one data point to each of several time series.  The new data
    /// point must be more recent than any other point in its time series.  Each
    /// `TimeSeries` value must fully specify a unique time series by supplying
    /// all label values for the metric and the monitored resource.
    ///
    /// The maximum number of `TimeSeries` objects per `Create` request is 200.
    #[prost(message, repeated, tag = "2")]
    pub time_series: ::prost::alloc::vec::Vec<TimeSeries>,
}
/// DEPRECATED. Used to hold per-time-series error status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTimeSeriesError {
    /// DEPRECATED. Time series ID that resulted in the `status` error.
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub time_series: ::core::option::Option<TimeSeries>,
    /// DEPRECATED. The status of the requested write operation for `time_series`.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<super::super::rpc::Status>,
}
/// Summary of the result of a failed request to write data to a time series.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTimeSeriesSummary {
    /// The number of points in the request.
    #[prost(int32, tag = "1")]
    pub total_point_count: i32,
    /// The number of points that were successfully written.
    #[prost(int32, tag = "2")]
    pub success_point_count: i32,
    /// The number of points that failed to be written. Order is not guaranteed.
    #[prost(message, repeated, tag = "3")]
    pub errors: ::prost::alloc::vec::Vec<create_time_series_summary::Error>,
}
/// Nested message and enum types in `CreateTimeSeriesSummary`.
pub mod create_time_series_summary {
    /// Detailed information about an error category.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Error {
        /// The status of the requested write operation.
        #[prost(message, optional, tag = "1")]
        pub status: ::core::option::Option<super::super::super::rpc::Status>,
        /// The number of points that couldn't be written because of `status`.
        #[prost(int32, tag = "2")]
        pub point_count: i32,
    }
}
/// The `QueryTimeSeries` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTimeSeriesRequest {
    /// Required. The [project](<https://cloud.google.com/monitoring/api/v3#project_name>) on
    /// which to execute the request. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The query in the [Monitoring Query
    /// Language](<https://cloud.google.com/monitoring/mql/reference>) format.
    /// The default time zone is in UTC.
    #[prost(string, tag = "7")]
    pub query: ::prost::alloc::string::String,
    /// A positive number that is the maximum number of time_series_data to return.
    #[prost(int32, tag = "9")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "10")]
    pub page_token: ::prost::alloc::string::String,
}
/// The `QueryTimeSeries` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTimeSeriesResponse {
    /// The descriptor for the time series data.
    #[prost(message, optional, tag = "8")]
    pub time_series_descriptor: ::core::option::Option<TimeSeriesDescriptor>,
    /// The time series data.
    #[prost(message, repeated, tag = "9")]
    pub time_series_data: ::prost::alloc::vec::Vec<TimeSeriesData>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results, use that value as
    /// `page_token` in the next call to this method.
    #[prost(string, tag = "10")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Query execution errors that may have caused the time series data returned
    /// to be incomplete. The available data will be available in the
    /// response.
    #[prost(message, repeated, tag = "11")]
    pub partial_errors: ::prost::alloc::vec::Vec<super::super::rpc::Status>,
}
/// This is an error detail intended to be used with INVALID_ARGUMENT errors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErrorList {
    /// Errors in parsing the time series query language text. The number of errors
    /// in the response may be limited.
    #[prost(message, repeated, tag = "1")]
    pub errors: ::prost::alloc::vec::Vec<QueryError>,
    /// A summary of all the errors.
    #[prost(string, tag = "2")]
    pub error_summary: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod metric_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages metric descriptors, monitored resource descriptors, and
    /// time series data.
    #[derive(Debug, Clone)]
    pub struct MetricServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MetricServiceClient<T>
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
        ) -> MetricServiceClient<InterceptedService<T, F>>
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
            MetricServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists monitored resource descriptors that match a filter. This method does not require a Workspace.
        pub async fn list_monitored_resource_descriptors(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListMonitoredResourceDescriptorsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListMonitoredResourceDescriptorsResponse>,
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
                "/google.monitoring.v3.MetricService/ListMonitoredResourceDescriptors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.MetricService",
                        "ListMonitoredResourceDescriptors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a single monitored resource descriptor. This method does not require a Workspace.
        pub async fn get_monitored_resource_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetMonitoredResourceDescriptorRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::api::MonitoredResourceDescriptor>,
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
                "/google.monitoring.v3.MetricService/GetMonitoredResourceDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.MetricService",
                        "GetMonitoredResourceDescriptor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists metric descriptors that match a filter. This method does not require a Workspace.
        pub async fn list_metric_descriptors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMetricDescriptorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMetricDescriptorsResponse>,
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
                "/google.monitoring.v3.MetricService/ListMetricDescriptors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.MetricService",
                        "ListMetricDescriptors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a single metric descriptor. This method does not require a Workspace.
        pub async fn get_metric_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMetricDescriptorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::api::MetricDescriptor>,
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
                "/google.monitoring.v3.MetricService/GetMetricDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.MetricService",
                        "GetMetricDescriptor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new metric descriptor.
        /// The creation is executed asynchronously and callers may check the returned
        /// operation to track its progress.
        /// User-created metric descriptors define
        /// [custom metrics](https://cloud.google.com/monitoring/custom-metrics).
        pub async fn create_metric_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMetricDescriptorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::api::MetricDescriptor>,
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
                "/google.monitoring.v3.MetricService/CreateMetricDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.MetricService",
                        "CreateMetricDescriptor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a metric descriptor. Only user-created
        /// [custom metrics](https://cloud.google.com/monitoring/custom-metrics) can be
        /// deleted.
        pub async fn delete_metric_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMetricDescriptorRequest>,
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
                "/google.monitoring.v3.MetricService/DeleteMetricDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.MetricService",
                        "DeleteMetricDescriptor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists time series that match a filter. This method does not require a Workspace.
        pub async fn list_time_series(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTimeSeriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTimeSeriesResponse>,
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
                "/google.monitoring.v3.MetricService/ListTimeSeries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.MetricService",
                        "ListTimeSeries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates or adds data to one or more time series.
        /// The response is empty if all time series in the request were written.
        /// If any time series could not be written, a corresponding failure message is
        /// included in the error response.
        pub async fn create_time_series(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTimeSeriesRequest>,
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
                "/google.monitoring.v3.MetricService/CreateTimeSeries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.MetricService",
                        "CreateTimeSeries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates or adds data to one or more service time series. A service time
        /// series is a time series for a metric from a Google Cloud service. The
        /// response is empty if all time series in the request were written. If any
        /// time series could not be written, a corresponding failure message is
        /// included in the error response. This endpoint rejects writes to
        /// user-defined metrics.
        /// This method is only for use by Google Cloud services. Use
        /// [projects.timeSeries.create][google.monitoring.v3.MetricService.CreateTimeSeries]
        /// instead.
        pub async fn create_service_time_series(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTimeSeriesRequest>,
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
                "/google.monitoring.v3.MetricService/CreateServiceTimeSeries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.MetricService",
                        "CreateServiceTimeSeries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod query_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The QueryService API is used to manage time series data in Stackdriver
    /// Monitoring. Time series data is a collection of data points that describes
    /// the time-varying values of a metric.
    #[derive(Debug, Clone)]
    pub struct QueryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> QueryServiceClient<T>
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
        ) -> QueryServiceClient<InterceptedService<T, F>>
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
            QueryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Queries time series using Monitoring Query Language. This method does not require a Workspace.
        pub async fn query_time_series(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTimeSeriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTimeSeriesResponse>,
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
                "/google.monitoring.v3.QueryService/QueryTimeSeries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.QueryService",
                        "QueryTimeSeries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Describes a change made to a configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutationRecord {
    /// When the change occurred.
    #[prost(message, optional, tag = "1")]
    pub mutate_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The email address of the user making the change.
    #[prost(string, tag = "2")]
    pub mutated_by: ::prost::alloc::string::String,
}
/// A description of a notification channel. The descriptor includes
/// the properties of the channel and the set of labels or fields that
/// must be specified to configure channels of a given type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationChannelDescriptor {
    /// The full REST resource name for this descriptor. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/notificationChannelDescriptors/\[TYPE\]
    ///
    /// In the above, `\[TYPE\]` is the value of the `type` field.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// The type of notification channel, such as "email" and "sms". To view the
    /// full list of channels, see
    /// [Channel
    /// descriptors](<https://cloud.google.com/monitoring/alerts/using-channels-api#ncd>).
    /// Notification channel types are globally unique.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// A human-readable name for the notification channel type.  This
    /// form of the name is suitable for a user interface.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// A human-readable description of the notification channel
    /// type. The description may include a description of the properties
    /// of the channel and pointers to external documentation.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The set of labels that must be defined to identify a particular
    /// channel of the corresponding type. Each label includes a
    /// description for how that field should be populated.
    #[prost(message, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<super::super::api::LabelDescriptor>,
    /// The tiers that support this notification channel; the project service tier
    /// must be one of the supported_tiers.
    #[deprecated]
    #[prost(enumeration = "ServiceTier", repeated, packed = "false", tag = "5")]
    pub supported_tiers: ::prost::alloc::vec::Vec<i32>,
    /// The product launch stage for channels of this type.
    #[prost(enumeration = "super::super::api::LaunchStage", tag = "7")]
    pub launch_stage: i32,
}
/// A `NotificationChannel` is a medium through which an alert is
/// delivered when a policy violation is detected. Examples of channels
/// include email, SMS, and third-party messaging applications. Fields
/// containing sensitive information like authentication tokens or
/// contact info are only partially populated on retrieval.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationChannel {
    /// The type of the notification channel. This field matches the
    /// value of the
    /// [NotificationChannelDescriptor.type][google.monitoring.v3.NotificationChannelDescriptor.type]
    /// field.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// The full REST resource name for this channel. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/notificationChannels/\[CHANNEL_ID\]
    ///
    /// The `\[CHANNEL_ID\]` is automatically assigned by the server on creation.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// An optional human-readable name for this notification channel. It is
    /// recommended that you specify a non-empty and unique name in order to
    /// make it easier to identify the channels in your project, though this is
    /// not enforced. The display name is limited to 512 Unicode characters.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// An optional human-readable description of this notification channel. This
    /// description may provide additional details, beyond the display
    /// name, for the channel. This may not exceed 1024 Unicode characters.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Configuration fields that define the channel and its behavior. The
    /// permissible and required labels are specified in the
    /// [NotificationChannelDescriptor.labels][google.monitoring.v3.NotificationChannelDescriptor.labels]
    /// of the `NotificationChannelDescriptor` corresponding to the `type` field.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// User-supplied key/value data that does not need to conform to
    /// the corresponding `NotificationChannelDescriptor`'s schema, unlike
    /// the `labels` field. This field is intended to be used for organizing
    /// and identifying the `NotificationChannel` objects.
    ///
    /// The field can contain up to 64 entries. Each key and value is limited to
    /// 63 Unicode characters or 128 bytes, whichever is smaller. Labels and
    /// values can contain only lowercase letters, numerals, underscores, and
    /// dashes. Keys must begin with a letter.
    #[prost(btree_map = "string, string", tag = "8")]
    pub user_labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Indicates whether this channel has been verified or not. On a
    /// [`ListNotificationChannels`][google.monitoring.v3.NotificationChannelService.ListNotificationChannels]
    /// or
    /// [`GetNotificationChannel`][google.monitoring.v3.NotificationChannelService.GetNotificationChannel]
    /// operation, this field is expected to be populated.
    ///
    /// If the value is `UNVERIFIED`, then it indicates that the channel is
    /// non-functioning (it both requires verification and lacks verification);
    /// otherwise, it is assumed that the channel works.
    ///
    /// If the channel is neither `VERIFIED` nor `UNVERIFIED`, it implies that
    /// the channel is of a type that does not require verification or that
    /// this specific channel has been exempted from verification because it was
    /// created prior to verification being required for channels of this type.
    ///
    /// This field cannot be modified using a standard
    /// [`UpdateNotificationChannel`][google.monitoring.v3.NotificationChannelService.UpdateNotificationChannel]
    /// operation. To change the value of this field, you must call
    /// [`VerifyNotificationChannel`][google.monitoring.v3.NotificationChannelService.VerifyNotificationChannel].
    #[prost(enumeration = "notification_channel::VerificationStatus", tag = "9")]
    pub verification_status: i32,
    /// Whether notifications are forwarded to the described channel. This makes
    /// it possible to disable delivery of notifications to a particular channel
    /// without removing the channel from all alerting policies that reference
    /// the channel. This is a more convenient approach when the change is
    /// temporary and you want to receive notifications from the same set
    /// of alerting policies on the channel at some point in the future.
    #[prost(message, optional, tag = "11")]
    pub enabled: ::core::option::Option<bool>,
    /// Record of the creation of this channel.
    #[prost(message, optional, tag = "12")]
    pub creation_record: ::core::option::Option<MutationRecord>,
    /// Records of the modification of this channel.
    #[prost(message, repeated, tag = "13")]
    pub mutation_records: ::prost::alloc::vec::Vec<MutationRecord>,
}
/// Nested message and enum types in `NotificationChannel`.
pub mod notification_channel {
    /// Indicates whether the channel has been verified or not. It is illegal
    /// to specify this field in a
    /// [`CreateNotificationChannel`][google.monitoring.v3.NotificationChannelService.CreateNotificationChannel]
    /// or an
    /// [`UpdateNotificationChannel`][google.monitoring.v3.NotificationChannelService.UpdateNotificationChannel]
    /// operation.
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
    pub enum VerificationStatus {
        /// Sentinel value used to indicate that the state is unknown, omitted, or
        /// is not applicable (as in the case of channels that neither support
        /// nor require verification in order to function).
        Unspecified = 0,
        /// The channel has yet to be verified and requires verification to function.
        /// Note that this state also applies to the case where the verification
        /// process has been initiated by sending a verification code but where
        /// the verification code has not been submitted to complete the process.
        Unverified = 1,
        /// It has been proven that notifications can be received on this
        /// notification channel and that someone on the project has access
        /// to messages that are delivered to that channel.
        Verified = 2,
    }
    impl VerificationStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VerificationStatus::Unspecified => "VERIFICATION_STATUS_UNSPECIFIED",
                VerificationStatus::Unverified => "UNVERIFIED",
                VerificationStatus::Verified => "VERIFIED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VERIFICATION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "UNVERIFIED" => Some(Self::Unverified),
                "VERIFIED" => Some(Self::Verified),
                _ => None,
            }
        }
    }
}
/// The `ListNotificationChannelDescriptors` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationChannelDescriptorsRequest {
    /// Required. The REST resource name of the parent from which to retrieve
    /// the notification channel descriptors. The expected syntax is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    ///
    /// Note that this
    /// [names](<https://cloud.google.com/monitoring/api/v3#project_name>) the parent
    /// container in which to look for the descriptors; to retrieve a single
    /// descriptor by name, use the
    /// [GetNotificationChannelDescriptor][google.monitoring.v3.NotificationChannelService.GetNotificationChannelDescriptor]
    /// operation, instead.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. If
    /// not set to a positive number, a reasonable value will be chosen by the
    /// service.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// If non-empty, `page_token` must contain a value returned as the
    /// `next_page_token` in a previous response to request the next set
    /// of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The `ListNotificationChannelDescriptors` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationChannelDescriptorsResponse {
    /// The monitored resource descriptors supported for the specified
    /// project, optionally filtered.
    #[prost(message, repeated, tag = "1")]
    pub channel_descriptors: ::prost::alloc::vec::Vec<NotificationChannelDescriptor>,
    /// If not empty, indicates that there may be more results that match
    /// the request. Use the value in the `page_token` field in a
    /// subsequent request to fetch the next set of results. If empty,
    /// all results have been returned.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The `GetNotificationChannelDescriptor` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationChannelDescriptorRequest {
    /// Required. The channel type for which to execute the request. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/notificationChannelDescriptors/\[CHANNEL_TYPE\]
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// The `CreateNotificationChannel` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNotificationChannelRequest {
    /// Required. The
    /// [project](<https://cloud.google.com/monitoring/api/v3#project_name>) on which
    /// to execute the request. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    ///
    /// This names the container into which the channel will be
    /// written, this does not name the newly created channel. The resulting
    /// channel's name will have a normalized version of this field as a prefix,
    /// but will add `/notificationChannels/\[CHANNEL_ID\]` to identify the channel.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The definition of the `NotificationChannel` to create.
    #[prost(message, optional, tag = "2")]
    pub notification_channel: ::core::option::Option<NotificationChannel>,
}
/// The `ListNotificationChannels` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationChannelsRequest {
    /// Required. The
    /// [project](<https://cloud.google.com/monitoring/api/v3#project_name>) on which
    /// to execute the request. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    ///
    /// This names the container
    /// in which to look for the notification channels; it does not name a
    /// specific channel. To query a specific channel by REST resource name, use
    /// the
    /// [`GetNotificationChannel`][google.monitoring.v3.NotificationChannelService.GetNotificationChannel]
    /// operation.
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// If provided, this field specifies the criteria that must be met by
    /// notification channels to be included in the response.
    ///
    /// For more details, see [sorting and
    /// filtering](<https://cloud.google.com/monitoring/api/v3/sorting-and-filtering>).
    #[prost(string, tag = "6")]
    pub filter: ::prost::alloc::string::String,
    /// A comma-separated list of fields by which to sort the result. Supports
    /// the same set of fields as in `filter`. Entries can be prefixed with
    /// a minus sign to sort in descending rather than ascending order.
    ///
    /// For more details, see [sorting and
    /// filtering](<https://cloud.google.com/monitoring/api/v3/sorting-and-filtering>).
    #[prost(string, tag = "7")]
    pub order_by: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. If
    /// not set to a positive number, a reasonable value will be chosen by the
    /// service.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If non-empty, `page_token` must contain a value returned as the
    /// `next_page_token` in a previous response to request the next set
    /// of results.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The `ListNotificationChannels` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationChannelsResponse {
    /// The notification channels defined for the specified project.
    #[prost(message, repeated, tag = "3")]
    pub notification_channels: ::prost::alloc::vec::Vec<NotificationChannel>,
    /// If not empty, indicates that there may be more results that match
    /// the request. Use the value in the `page_token` field in a
    /// subsequent request to fetch the next set of results. If empty,
    /// all results have been returned.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of notification channels in all pages. This number is only
    /// an estimate, and may change in subsequent pages. <https://aip.dev/158>
    #[prost(int32, tag = "4")]
    pub total_size: i32,
}
/// The `GetNotificationChannel` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationChannelRequest {
    /// Required. The channel for which to execute the request. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/notificationChannels/\[CHANNEL_ID\]
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// The `UpdateNotificationChannel` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNotificationChannelRequest {
    /// The fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. A description of the changes to be applied to the specified
    /// notification channel. The description must provide a definition for
    /// fields to be updated; the names of these fields should also be
    /// included in the `update_mask`.
    #[prost(message, optional, tag = "3")]
    pub notification_channel: ::core::option::Option<NotificationChannel>,
}
/// The `DeleteNotificationChannel` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNotificationChannelRequest {
    /// Required. The channel for which to execute the request. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/notificationChannels/\[CHANNEL_ID\]
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// If true, the notification channel will be deleted regardless of its
    /// use in alert policies (the policies will be updated to remove the
    /// channel). If false, channels that are still referenced by an existing
    /// alerting policy will fail to be deleted in a delete operation.
    #[prost(bool, tag = "5")]
    pub force: bool,
}
/// The `SendNotificationChannelVerificationCode` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendNotificationChannelVerificationCodeRequest {
    /// Required. The notification channel to which to send a verification code.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The `GetNotificationChannelVerificationCode` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationChannelVerificationCodeRequest {
    /// Required. The notification channel for which a verification code is to be
    /// generated and retrieved. This must name a channel that is already verified;
    /// if the specified channel is not verified, the request will fail.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The desired expiration time. If specified, the API will guarantee that
    /// the returned code will not be valid after the specified timestamp;
    /// however, the API cannot guarantee that the returned code will be
    /// valid for at least as long as the requested time (the API puts an upper
    /// bound on the amount of time for which a code may be valid). If omitted,
    /// a default expiration will be used, which may be less than the max
    /// permissible expiration (so specifying an expiration may extend the
    /// code's lifetime over omitting an expiration, even though the API does
    /// impose an upper limit on the maximum expiration that is permitted).
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The `GetNotificationChannelVerificationCode` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationChannelVerificationCodeResponse {
    /// The verification code, which may be used to verify other channels
    /// that have an equivalent identity (i.e. other channels of the same
    /// type with the same fingerprint such as other email channels with
    /// the same email address or other sms channels with the same number).
    #[prost(string, tag = "1")]
    pub code: ::prost::alloc::string::String,
    /// The expiration time associated with the code that was returned. If
    /// an expiration was provided in the request, this is the minimum of the
    /// requested expiration in the request and the max permitted expiration.
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The `VerifyNotificationChannel` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyNotificationChannelRequest {
    /// Required. The notification channel to verify.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The verification code that was delivered to the channel as
    /// a result of invoking the `SendNotificationChannelVerificationCode` API
    /// method or that was retrieved from a verified channel via
    /// `GetNotificationChannelVerificationCode`. For example, one might have
    /// "G-123456" or "TKNZGhhd2EyN3I1MnRnMjRv" (in general, one is only
    /// guaranteed that the code is valid UTF-8; one should not
    /// make any assumptions regarding the structure or format of the code).
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod notification_channel_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Notification Channel API provides access to configuration that
    /// controls how messages related to incidents are sent.
    #[derive(Debug, Clone)]
    pub struct NotificationChannelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> NotificationChannelServiceClient<T>
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
        ) -> NotificationChannelServiceClient<InterceptedService<T, F>>
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
            NotificationChannelServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Lists the descriptors for supported channel types. The use of descriptors
        /// makes it possible for new channel types to be dynamically added.
        pub async fn list_notification_channel_descriptors(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListNotificationChannelDescriptorsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListNotificationChannelDescriptorsResponse>,
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
                "/google.monitoring.v3.NotificationChannelService/ListNotificationChannelDescriptors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.NotificationChannelService",
                        "ListNotificationChannelDescriptors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a single channel descriptor. The descriptor indicates which fields
        /// are expected / permitted for a notification channel of the given type.
        pub async fn get_notification_channel_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetNotificationChannelDescriptorRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::NotificationChannelDescriptor>,
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
                "/google.monitoring.v3.NotificationChannelService/GetNotificationChannelDescriptor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.NotificationChannelService",
                        "GetNotificationChannelDescriptor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the notification channels that have been created for the project.
        /// To list the types of notification channels that are supported, use
        /// the `ListNotificationChannelDescriptors` method.
        pub async fn list_notification_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotificationChannelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNotificationChannelsResponse>,
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
                "/google.monitoring.v3.NotificationChannelService/ListNotificationChannels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.NotificationChannelService",
                        "ListNotificationChannels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a single notification channel. The channel includes the relevant
        /// configuration details with which the channel was created. However, the
        /// response may truncate or omit passwords, API keys, or other private key
        /// matter and thus the response may not be 100% identical to the information
        /// that was supplied in the call to the create method.
        pub async fn get_notification_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotificationChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NotificationChannel>,
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
                "/google.monitoring.v3.NotificationChannelService/GetNotificationChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.NotificationChannelService",
                        "GetNotificationChannel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new notification channel, representing a single notification
        /// endpoint such as an email address, SMS number, or PagerDuty service.
        ///
        /// Design your application to single-thread API calls that modify the state of
        /// notification channels in a single project. This includes calls to
        /// CreateNotificationChannel, DeleteNotificationChannel and
        /// UpdateNotificationChannel.
        pub async fn create_notification_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNotificationChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NotificationChannel>,
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
                "/google.monitoring.v3.NotificationChannelService/CreateNotificationChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.NotificationChannelService",
                        "CreateNotificationChannel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a notification channel. Fields not specified in the field mask
        /// remain unchanged.
        ///
        /// Design your application to single-thread API calls that modify the state of
        /// notification channels in a single project. This includes calls to
        /// CreateNotificationChannel, DeleteNotificationChannel and
        /// UpdateNotificationChannel.
        pub async fn update_notification_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNotificationChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NotificationChannel>,
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
                "/google.monitoring.v3.NotificationChannelService/UpdateNotificationChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.NotificationChannelService",
                        "UpdateNotificationChannel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a notification channel.
        ///
        /// Design your application to single-thread API calls that modify the state of
        /// notification channels in a single project. This includes calls to
        /// CreateNotificationChannel, DeleteNotificationChannel and
        /// UpdateNotificationChannel.
        pub async fn delete_notification_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNotificationChannelRequest>,
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
                "/google.monitoring.v3.NotificationChannelService/DeleteNotificationChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.NotificationChannelService",
                        "DeleteNotificationChannel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Causes a verification code to be delivered to the channel. The code
        /// can then be supplied in `VerifyNotificationChannel` to verify the channel.
        pub async fn send_notification_channel_verification_code(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SendNotificationChannelVerificationCodeRequest,
            >,
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
                "/google.monitoring.v3.NotificationChannelService/SendNotificationChannelVerificationCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.NotificationChannelService",
                        "SendNotificationChannelVerificationCode",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Requests a verification code for an already verified channel that can then
        /// be used in a call to VerifyNotificationChannel() on a different channel
        /// with an equivalent identity in the same or in a different project. This
        /// makes it possible to copy a channel between projects without requiring
        /// manual reverification of the channel. If the channel is not in the
        /// verified state, this method will fail (in other words, this may only be
        /// used if the SendNotificationChannelVerificationCode and
        /// VerifyNotificationChannel paths have already been used to put the given
        /// channel into the verified state).
        ///
        /// There is no guarantee that the verification codes returned by this method
        /// will be of a similar structure or form as the ones that are delivered
        /// to the channel via SendNotificationChannelVerificationCode; while
        /// VerifyNotificationChannel() will recognize both the codes delivered via
        /// SendNotificationChannelVerificationCode() and returned from
        /// GetNotificationChannelVerificationCode(), it is typically the case that
        /// the verification codes delivered via
        /// SendNotificationChannelVerificationCode() will be shorter and also
        /// have a shorter expiration (e.g. codes such as "G-123456") whereas
        /// GetVerificationCode() will typically return a much longer, websafe base
        /// 64 encoded string that has a longer expiration time.
        pub async fn get_notification_channel_verification_code(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetNotificationChannelVerificationCodeRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetNotificationChannelVerificationCodeResponse>,
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
                "/google.monitoring.v3.NotificationChannelService/GetNotificationChannelVerificationCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.NotificationChannelService",
                        "GetNotificationChannelVerificationCode",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Verifies a `NotificationChannel` by proving receipt of the code
        /// delivered to the channel as a result of calling
        /// `SendNotificationChannelVerificationCode`.
        pub async fn verify_notification_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyNotificationChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NotificationChannel>,
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
                "/google.monitoring.v3.NotificationChannelService/VerifyNotificationChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.NotificationChannelService",
                        "VerifyNotificationChannel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The description of a dynamic collection of monitored resources. Each group
/// has a filter that is matched against monitored resources and their associated
/// metadata. If a group's filter matches an available monitored resource, then
/// that resource is a member of that group.  Groups can contain any number of
/// monitored resources, and each monitored resource can be a member of any
/// number of groups.
///
/// Groups can be nested in parent-child hierarchies. The `parentName` field
/// identifies an optional parent for each group.  If a group has a parent, then
/// the only monitored resources available to be matched by the group's filter
/// are the resources contained in the parent group.  In other words, a group
/// contains the monitored resources that match its filter and the filters of all
/// the group's ancestors.  A group without a parent can contain any monitored
/// resource.
///
/// For example, consider an infrastructure running a set of instances with two
/// user-defined tags: `"environment"` and `"role"`. A parent group has a filter,
/// `environment="production"`.  A child of that parent group has a filter,
/// `role="transcoder"`.  The parent group contains all instances in the
/// production environment, regardless of their roles.  The child group contains
/// instances that have the transcoder role *and* are in the production
/// environment.
///
/// The monitored resources contained in a group can change at any moment,
/// depending on what resources exist and what filters are associated with the
/// group and its ancestors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    /// Output only. The name of this group. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/groups/\[GROUP_ID\]
    ///
    /// When creating a group, this field is ignored and a new name is created
    /// consisting of the project specified in the call to `CreateGroup`
    /// and a unique `\[GROUP_ID\]` that is generated automatically.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A user-assigned name for this group, used only for display purposes.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The name of the group's parent, if it has one. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/groups/\[GROUP_ID\]
    ///
    /// For groups with no parent, `parent_name` is the empty string, `""`.
    #[prost(string, tag = "3")]
    pub parent_name: ::prost::alloc::string::String,
    /// The filter used to determine which monitored resources belong to this
    /// group.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// If true, the members of this group are considered to be a cluster.
    /// The system can perform additional analysis on groups that are clusters.
    #[prost(bool, tag = "6")]
    pub is_cluster: bool,
}
/// The `ListGroup` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupsRequest {
    /// Required. The [project](<https://cloud.google.com/monitoring/api/v3#project_name>)
    /// whose groups are to be listed. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
    /// A positive number that is the maximum number of results to return.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `next_page_token` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "6")]
    pub page_token: ::prost::alloc::string::String,
    /// An optional filter consisting of a single group name.  The filters limit
    /// the groups returned based on their parent-child relationship with the
    /// specified group. If no filter is specified, all groups are returned.
    #[prost(oneof = "list_groups_request::Filter", tags = "2, 3, 4")]
    pub filter: ::core::option::Option<list_groups_request::Filter>,
}
/// Nested message and enum types in `ListGroupsRequest`.
pub mod list_groups_request {
    /// An optional filter consisting of a single group name.  The filters limit
    /// the groups returned based on their parent-child relationship with the
    /// specified group. If no filter is specified, all groups are returned.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// A group name. The format is:
        ///
        ///      projects/\[PROJECT_ID_OR_NUMBER\]/groups/\[GROUP_ID\]
        ///
        /// Returns groups whose `parent_name` field contains the group
        /// name.  If no groups have this parent, the results are empty.
        #[prost(string, tag = "2")]
        ChildrenOfGroup(::prost::alloc::string::String),
        /// A group name. The format is:
        ///
        ///      projects/\[PROJECT_ID_OR_NUMBER\]/groups/\[GROUP_ID\]
        ///
        /// Returns groups that are ancestors of the specified group.
        /// The groups are returned in order, starting with the immediate parent and
        /// ending with the most distant ancestor.  If the specified group has no
        /// immediate parent, the results are empty.
        #[prost(string, tag = "3")]
        AncestorsOfGroup(::prost::alloc::string::String),
        /// A group name. The format is:
        ///
        ///      projects/\[PROJECT_ID_OR_NUMBER\]/groups/\[GROUP_ID\]
        ///
        /// Returns the descendants of the specified group.  This is a superset of
        /// the results returned by the `children_of_group` filter, and includes
        /// children-of-children, and so forth.
        #[prost(string, tag = "4")]
        DescendantsOfGroup(::prost::alloc::string::String),
    }
}
/// The `ListGroups` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupsResponse {
    /// The groups that match the specified filters.
    #[prost(message, repeated, tag = "1")]
    pub group: ::prost::alloc::vec::Vec<Group>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The `GetGroup` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGroupRequest {
    /// Required. The group to retrieve. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/groups/\[GROUP_ID\]
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// The `CreateGroup` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGroupRequest {
    /// Required. The [project](<https://cloud.google.com/monitoring/api/v3#project_name>) in
    /// which to create the group. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// Required. A group definition. It is an error to define the `name` field because
    /// the system assigns the name.
    #[prost(message, optional, tag = "2")]
    pub group: ::core::option::Option<Group>,
    /// If true, validate this request but do not create the group.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// The `UpdateGroup` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroupRequest {
    /// Required. The new definition of the group.  All fields of the existing group,
    /// excepting `name`, are replaced with the corresponding fields of this group.
    #[prost(message, optional, tag = "2")]
    pub group: ::core::option::Option<Group>,
    /// If true, validate this request but do not update the existing group.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// The `DeleteGroup` request. The default behavior is to be able to delete a
/// single group without any descendants.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGroupRequest {
    /// Required. The group to delete. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/groups/\[GROUP_ID\]
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// If this field is true, then the request means to delete a group with all
    /// its descendants. Otherwise, the request means to delete a group only when
    /// it has no descendants. The default value is false.
    #[prost(bool, tag = "4")]
    pub recursive: bool,
}
/// The `ListGroupMembers` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupMembersRequest {
    /// Required. The group whose members are listed. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/groups/\[GROUP_ID\]
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
    /// A positive number that is the maximum number of results to return.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `next_page_token` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// An optional [list
    /// filter](<https://cloud.google.com/monitoring/api/learn_more#filtering>)
    /// describing the members to be returned.  The filter may reference the type,
    /// labels, and metadata of monitored resources that comprise the group. For
    /// example, to return only resources representing Compute Engine VM instances,
    /// use this filter:
    ///
    ///      `resource.type = "gce_instance"`
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// An optional time interval for which results should be returned. Only
    /// members that were part of the group during the specified interval are
    /// included in the response.  If no interval is provided then the group
    /// membership over the last minute is returned.
    #[prost(message, optional, tag = "6")]
    pub interval: ::core::option::Option<TimeInterval>,
}
/// The `ListGroupMembers` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupMembersResponse {
    /// A set of monitored resources in the group.
    #[prost(message, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<super::super::api::MonitoredResource>,
    /// If there are more results than have been returned, then this field is
    /// set to a non-empty value.  To see the additional results, use that value as
    /// `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of elements matching this request.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod group_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Group API lets you inspect and manage your
    /// [groups](#google.monitoring.v3.Group).
    ///
    /// A group is a named filter that is used to identify
    /// a collection of monitored resources. Groups are typically used to
    /// mirror the physical and/or logical topology of the environment.
    /// Because group membership is computed dynamically, monitored
    /// resources that are started in the future are automatically placed
    /// in matching groups. By using a group to name monitored resources in,
    /// for example, an alert policy, the target of that alert policy is
    /// updated automatically as monitored resources are added and removed
    /// from the infrastructure.
    #[derive(Debug, Clone)]
    pub struct GroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GroupServiceClient<T>
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
        ) -> GroupServiceClient<InterceptedService<T, F>>
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
            GroupServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists the existing groups.
        pub async fn list_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGroupsResponse>,
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
                "/google.monitoring.v3.GroupService/ListGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.monitoring.v3.GroupService", "ListGroups"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a single group.
        pub async fn get_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::Group>, tonic::Status> {
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
                "/google.monitoring.v3.GroupService/GetGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.monitoring.v3.GroupService", "GetGroup"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new group.
        pub async fn create_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::Group>, tonic::Status> {
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
                "/google.monitoring.v3.GroupService/CreateGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.monitoring.v3.GroupService", "CreateGroup"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing group.
        /// You can change any group attributes except `name`.
        pub async fn update_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::Group>, tonic::Status> {
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
                "/google.monitoring.v3.GroupService/UpdateGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.monitoring.v3.GroupService", "UpdateGroup"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an existing group.
        pub async fn delete_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGroupRequest>,
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
                "/google.monitoring.v3.GroupService/DeleteGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.monitoring.v3.GroupService", "DeleteGroup"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the monitored resources that are members of a group.
        pub async fn list_group_members(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGroupMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGroupMembersResponse>,
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
                "/google.monitoring.v3.GroupService/ListGroupMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.GroupService",
                        "ListGroupMembers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// A description of the conditions under which some aspect of your system is
/// considered to be "unhealthy" and the ways to notify people or services about
/// this state. For an overview of alert policies, see
/// [Introduction to Alerting](<https://cloud.google.com/monitoring/alerts/>).
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertPolicy {
    /// Required if the policy exists. The resource name for this policy. The
    /// format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/alertPolicies/\[ALERT_POLICY_ID\]
    ///
    /// `\[ALERT_POLICY_ID\]` is assigned by Cloud Monitoring when the policy
    /// is created. When calling the
    /// [alertPolicies.create][google.monitoring.v3.AlertPolicyService.CreateAlertPolicy]
    /// method, do not include the `name` field in the alerting policy passed as
    /// part of the request.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A short name or phrase used to identify the policy in dashboards,
    /// notifications, and incidents. To avoid confusion, don't use the same
    /// display name for multiple policies in the same project. The name is
    /// limited to 512 Unicode characters.
    ///
    /// The convention for the display_name of a PrometheusQueryLanguageCondition
    /// is "{rule group name}/{alert name}", where the {rule group name} and
    /// {alert name} should be taken from the corresponding Prometheus
    /// configuration file. This convention is not enforced.
    /// In any case the display_name is not a unique key of the AlertPolicy.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Documentation that is included with notifications and incidents related to
    /// this policy. Best practice is for the documentation to include information
    /// to help responders understand, mitigate, escalate, and correct the
    /// underlying problems detected by the alerting policy. Notification channels
    /// that have limited capacity might not show this documentation.
    #[prost(message, optional, tag = "13")]
    pub documentation: ::core::option::Option<alert_policy::Documentation>,
    /// User-supplied key/value data to be used for organizing and
    /// identifying the `AlertPolicy` objects.
    ///
    /// The field can contain up to 64 entries. Each key and value is limited to
    /// 63 Unicode characters or 128 bytes, whichever is smaller. Labels and
    /// values can contain only lowercase letters, numerals, underscores, and
    /// dashes. Keys must begin with a letter.
    ///
    /// Note that Prometheus {alert name} is a
    /// [valid Prometheus label
    /// names](<https://prometheus.io/docs/concepts/data_model/#metric-names-and-labels>),
    /// whereas Prometheus {rule group} is an unrestricted UTF-8 string.
    /// This means that they cannot be stored as-is in user labels, because
    /// they may contain characters that are not allowed in user-label values.
    #[prost(btree_map = "string, string", tag = "16")]
    pub user_labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// A list of conditions for the policy. The conditions are combined by AND or
    /// OR according to the `combiner` field. If the combined conditions evaluate
    /// to true, then an incident is created. A policy can have from one to six
    /// conditions.
    /// If `condition_time_series_query_language` is present, it must be the only
    /// `condition`.
    /// If `condition_monitoring_query_language` is present, it must be the only
    /// `condition`.
    #[prost(message, repeated, tag = "12")]
    pub conditions: ::prost::alloc::vec::Vec<alert_policy::Condition>,
    /// How to combine the results of multiple conditions to determine if an
    /// incident should be opened.
    /// If `condition_time_series_query_language` is present, this must be
    /// `COMBINE_UNSPECIFIED`.
    #[prost(enumeration = "alert_policy::ConditionCombinerType", tag = "6")]
    pub combiner: i32,
    /// Whether or not the policy is enabled. On write, the default interpretation
    /// if unset is that the policy is enabled. On read, clients should not make
    /// any assumption about the state if it has not been populated. The
    /// field should always be populated on List and Get operations, unless
    /// a field projection has been specified that strips it out.
    #[prost(message, optional, tag = "17")]
    pub enabled: ::core::option::Option<bool>,
    /// Read-only description of how the alert policy is invalid. This field is
    /// only set when the alert policy is invalid. An invalid alert policy will not
    /// generate incidents.
    #[prost(message, optional, tag = "18")]
    pub validity: ::core::option::Option<super::super::rpc::Status>,
    /// Identifies the notification channels to which notifications should be sent
    /// when incidents are opened or closed or when new violations occur on
    /// an already opened incident. Each element of this array corresponds to
    /// the `name` field in each of the
    /// [`NotificationChannel`][google.monitoring.v3.NotificationChannel]
    /// objects that are returned from the \[`ListNotificationChannels`\]
    /// \[google.monitoring.v3.NotificationChannelService.ListNotificationChannels\]
    /// method. The format of the entries in this field is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/notificationChannels/\[CHANNEL_ID\]
    #[prost(string, repeated, tag = "14")]
    pub notification_channels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A read-only record of the creation of the alerting policy. If provided
    /// in a call to create or update, this field will be ignored.
    #[prost(message, optional, tag = "10")]
    pub creation_record: ::core::option::Option<MutationRecord>,
    /// A read-only record of the most recent change to the alerting policy. If
    /// provided in a call to create or update, this field will be ignored.
    #[prost(message, optional, tag = "11")]
    pub mutation_record: ::core::option::Option<MutationRecord>,
    /// Control over how this alert policy's notification channels are notified.
    #[prost(message, optional, tag = "21")]
    pub alert_strategy: ::core::option::Option<alert_policy::AlertStrategy>,
    /// Optional. The severity of an alert policy indicates how important incidents
    /// generated by that policy are. The severity level will be displayed on the
    /// Incident detail page and in notifications.
    #[prost(enumeration = "alert_policy::Severity", tag = "22")]
    pub severity: i32,
}
/// Nested message and enum types in `AlertPolicy`.
pub mod alert_policy {
    /// A content string and a MIME type that describes the content string's
    /// format.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Documentation {
        /// The body of the documentation, interpreted according to `mime_type`.
        /// The content may not exceed 8,192 Unicode characters and may not exceed
        /// more than 10,240 bytes when encoded in UTF-8 format, whichever is
        /// smaller. This text can be [templatized by using
        /// variables](<https://cloud.google.com/monitoring/alerts/doc-variables>).
        #[prost(string, tag = "1")]
        pub content: ::prost::alloc::string::String,
        /// The format of the `content` field. Presently, only the value
        /// `"text/markdown"` is supported. See
        /// [Markdown](<https://en.wikipedia.org/wiki/Markdown>) for more information.
        #[prost(string, tag = "2")]
        pub mime_type: ::prost::alloc::string::String,
        /// Optional. The subject line of the notification. The subject line may not
        /// exceed 10,240 bytes. In notifications generated by this policy, the
        /// contents of the subject line after variable expansion will be truncated
        /// to 255 bytes or shorter at the latest UTF-8 character boundary. The
        /// 255-byte limit is recommended by [this
        /// thread](<https://stackoverflow.com/questions/1592291/what-is-the-email-subject-length-limit>).
        /// It is both the limit imposed by some third-party ticketing products and
        /// it is common to define textual fields in databases as VARCHAR(255).
        ///
        /// The contents of the subject line can be [templatized by using
        /// variables](<https://cloud.google.com/monitoring/alerts/doc-variables>).
        /// If this field is missing or empty, a default subject line will be
        /// generated.
        #[prost(string, tag = "3")]
        pub subject: ::prost::alloc::string::String,
    }
    /// A condition is a true/false test that determines when an alerting policy
    /// should open an incident. If a condition evaluates to true, it signifies
    /// that something is wrong.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Condition {
        /// Required if the condition exists. The unique resource name for this
        /// condition. Its format is:
        ///
        ///      projects/\[PROJECT_ID_OR_NUMBER\]/alertPolicies/\[POLICY_ID\]/conditions/\[CONDITION_ID\]
        ///
        /// `\[CONDITION_ID\]` is assigned by Cloud Monitoring when the
        /// condition is created as part of a new or updated alerting policy.
        ///
        /// When calling the
        /// [alertPolicies.create][google.monitoring.v3.AlertPolicyService.CreateAlertPolicy]
        /// method, do not include the `name` field in the conditions of the
        /// requested alerting policy. Cloud Monitoring creates the
        /// condition identifiers and includes them in the new policy.
        ///
        /// When calling the
        /// [alertPolicies.update][google.monitoring.v3.AlertPolicyService.UpdateAlertPolicy]
        /// method to update a policy, including a condition `name` causes the
        /// existing condition to be updated. Conditions without names are added to
        /// the updated policy. Existing conditions are deleted if they are not
        /// updated.
        ///
        /// Best practice is to preserve `\[CONDITION_ID\]` if you make only small
        /// changes, such as those to condition thresholds, durations, or trigger
        /// values.  Otherwise, treat the change as a new condition and let the
        /// existing condition be deleted.
        #[prost(string, tag = "12")]
        pub name: ::prost::alloc::string::String,
        /// A short name or phrase used to identify the condition in dashboards,
        /// notifications, and incidents. To avoid confusion, don't use the same
        /// display name for multiple conditions in the same policy.
        #[prost(string, tag = "6")]
        pub display_name: ::prost::alloc::string::String,
        /// Only one of the following condition types will be specified.
        #[prost(oneof = "condition::Condition", tags = "1, 2, 20, 19, 21")]
        pub condition: ::core::option::Option<condition::Condition>,
    }
    /// Nested message and enum types in `Condition`.
    pub mod condition {
        /// Specifies how many time series must fail a predicate to trigger a
        /// condition. If not specified, then a `{count: 1}` trigger is used.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Trigger {
            /// A type of trigger.
            #[prost(oneof = "trigger::Type", tags = "1, 2")]
            pub r#type: ::core::option::Option<trigger::Type>,
        }
        /// Nested message and enum types in `Trigger`.
        pub mod trigger {
            /// A type of trigger.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Type {
                /// The absolute number of time series that must fail
                /// the predicate for the condition to be triggered.
                #[prost(int32, tag = "1")]
                Count(i32),
                /// The percentage of time series that must fail the
                /// predicate for the condition to be triggered.
                #[prost(double, tag = "2")]
                Percent(f64),
            }
        }
        /// A condition type that compares a collection of time series
        /// against a threshold.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MetricThreshold {
            /// Required. A
            /// [filter](<https://cloud.google.com/monitoring/api/v3/filters>) that
            /// identifies which time series should be compared with the threshold.
            ///
            /// The filter is similar to the one that is specified in the
            /// [`ListTimeSeries`
            /// request](<https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list>)
            /// (that call is useful to verify the time series that will be retrieved /
            /// processed). The filter must specify the metric type and the resource
            /// type. Optionally, it can specify resource labels and metric labels.
            /// This field must not exceed 2048 Unicode characters in length.
            #[prost(string, tag = "2")]
            pub filter: ::prost::alloc::string::String,
            /// Specifies the alignment of data points in individual time series as
            /// well as how to combine the retrieved time series together (such as
            /// when aggregating multiple streams on each resource to a single
            /// stream for each resource or when aggregating streams across all
            /// members of a group of resources). Multiple aggregations
            /// are applied in the order specified.
            ///
            /// This field is similar to the one in the [`ListTimeSeries`
            /// request](<https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list>).
            /// It is advisable to use the `ListTimeSeries` method when debugging this
            /// field.
            #[prost(message, repeated, tag = "8")]
            pub aggregations: ::prost::alloc::vec::Vec<super::super::Aggregation>,
            /// A [filter](<https://cloud.google.com/monitoring/api/v3/filters>) that
            /// identifies a time series that should be used as the denominator of a
            /// ratio that will be compared with the threshold. If a
            /// `denominator_filter` is specified, the time series specified by the
            /// `filter` field will be used as the numerator.
            ///
            /// The filter must specify the metric type and optionally may contain
            /// restrictions on resource type, resource labels, and metric labels.
            /// This field may not exceed 2048 Unicode characters in length.
            #[prost(string, tag = "9")]
            pub denominator_filter: ::prost::alloc::string::String,
            /// Specifies the alignment of data points in individual time series
            /// selected by `denominatorFilter` as
            /// well as how to combine the retrieved time series together (such as
            /// when aggregating multiple streams on each resource to a single
            /// stream for each resource or when aggregating streams across all
            /// members of a group of resources).
            ///
            /// When computing ratios, the `aggregations` and
            /// `denominator_aggregations` fields must use the same alignment period
            /// and produce time series that have the same periodicity and labels.
            #[prost(message, repeated, tag = "10")]
            pub denominator_aggregations: ::prost::alloc::vec::Vec<
                super::super::Aggregation,
            >,
            /// When this field is present, the `MetricThreshold` condition forecasts
            /// whether the time series is predicted to violate the threshold within
            /// the `forecast_horizon`. When this field is not set, the
            /// `MetricThreshold` tests the current value of the timeseries against the
            /// threshold.
            #[prost(message, optional, tag = "12")]
            pub forecast_options: ::core::option::Option<
                metric_threshold::ForecastOptions,
            >,
            /// The comparison to apply between the time series (indicated by `filter`
            /// and `aggregation`) and the threshold (indicated by `threshold_value`).
            /// The comparison is applied on each time series, with the time series
            /// on the left-hand side and the threshold on the right-hand side.
            ///
            /// Only `COMPARISON_LT` and `COMPARISON_GT` are supported currently.
            #[prost(enumeration = "super::super::ComparisonType", tag = "4")]
            pub comparison: i32,
            /// A value against which to compare the time series.
            #[prost(double, tag = "5")]
            pub threshold_value: f64,
            /// The amount of time that a time series must violate the
            /// threshold to be considered failing. Currently, only values
            /// that are a multiple of a minute--e.g., 0, 60, 120, or 300
            /// seconds--are supported. If an invalid value is given, an
            /// error will be returned. When choosing a duration, it is useful to
            /// keep in mind the frequency of the underlying time series data
            /// (which may also be affected by any alignments specified in the
            /// `aggregations` field); a good duration is long enough so that a single
            /// outlier does not generate spurious alerts, but short enough that
            /// unhealthy states are detected and alerted on quickly.
            #[prost(message, optional, tag = "6")]
            pub duration: ::core::option::Option<::prost_types::Duration>,
            /// The number/percent of time series for which the comparison must hold
            /// in order for the condition to trigger. If unspecified, then the
            /// condition will trigger if the comparison is true for any of the
            /// time series that have been identified by `filter` and `aggregations`,
            /// or by the ratio, if `denominator_filter` and `denominator_aggregations`
            /// are specified.
            #[prost(message, optional, tag = "7")]
            pub trigger: ::core::option::Option<Trigger>,
            /// A condition control that determines how metric-threshold conditions
            /// are evaluated when data stops arriving.
            #[prost(enumeration = "EvaluationMissingData", tag = "11")]
            pub evaluation_missing_data: i32,
        }
        /// Nested message and enum types in `MetricThreshold`.
        pub mod metric_threshold {
            /// Options used when forecasting the time series and testing
            /// the predicted value against the threshold.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ForecastOptions {
                /// Required. The length of time into the future to forecast whether a
                /// time series will violate the threshold. If the predicted value is
                /// found to violate the threshold, and the violation is observed in all
                /// forecasts made for the configured `duration`, then the time series is
                /// considered to be failing.
                /// The forecast horizon can range from 1 hour to 60 hours.
                #[prost(message, optional, tag = "1")]
                pub forecast_horizon: ::core::option::Option<::prost_types::Duration>,
            }
        }
        /// A condition type that checks that monitored resources
        /// are reporting data. The configuration defines a metric and
        /// a set of monitored resources. The predicate is considered in violation
        /// when a time series for the specified metric of a monitored
        /// resource does not include any data in the specified `duration`.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MetricAbsence {
            /// Required. A
            /// [filter](<https://cloud.google.com/monitoring/api/v3/filters>) that
            /// identifies which time series should be compared with the threshold.
            ///
            /// The filter is similar to the one that is specified in the
            /// [`ListTimeSeries`
            /// request](<https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list>)
            /// (that call is useful to verify the time series that will be retrieved /
            /// processed). The filter must specify the metric type and the resource
            /// type. Optionally, it can specify resource labels and metric labels.
            /// This field must not exceed 2048 Unicode characters in length.
            #[prost(string, tag = "1")]
            pub filter: ::prost::alloc::string::String,
            /// Specifies the alignment of data points in individual time series as
            /// well as how to combine the retrieved time series together (such as
            /// when aggregating multiple streams on each resource to a single
            /// stream for each resource or when aggregating streams across all
            /// members of a group of resources). Multiple aggregations
            /// are applied in the order specified.
            ///
            /// This field is similar to the one in the [`ListTimeSeries`
            /// request](<https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list>).
            /// It is advisable to use the `ListTimeSeries` method when debugging this
            /// field.
            #[prost(message, repeated, tag = "5")]
            pub aggregations: ::prost::alloc::vec::Vec<super::super::Aggregation>,
            /// The amount of time that a time series must fail to report new
            /// data to be considered failing. The minimum value of this field
            /// is 120 seconds. Larger values that are a multiple of a
            /// minute--for example, 240 or 300 seconds--are supported.
            /// If an invalid value is given, an
            /// error will be returned. The `Duration.nanos` field is
            /// ignored.
            #[prost(message, optional, tag = "2")]
            pub duration: ::core::option::Option<::prost_types::Duration>,
            /// The number/percent of time series for which the comparison must hold
            /// in order for the condition to trigger. If unspecified, then the
            /// condition will trigger if the comparison is true for any of the
            /// time series that have been identified by `filter` and `aggregations`.
            #[prost(message, optional, tag = "3")]
            pub trigger: ::core::option::Option<Trigger>,
        }
        /// A condition type that checks whether a log message in the [scoping
        /// project](<https://cloud.google.com/monitoring/api/v3#project_name>)
        /// satisfies the given filter. Logs from other projects in the metrics
        /// scope are not evaluated.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LogMatch {
            /// Required. A logs-based filter. See [Advanced Logs
            /// Queries](<https://cloud.google.com/logging/docs/view/advanced-queries>)
            /// for how this filter should be constructed.
            #[prost(string, tag = "1")]
            pub filter: ::prost::alloc::string::String,
            /// Optional. A map from a label key to an extractor expression, which is
            /// used to extract the value for this label key. Each entry in this map is
            /// a specification for how data should be extracted from log entries that
            /// match `filter`. Each combination of extracted values is treated as a
            /// separate rule for the purposes of triggering notifications. Label keys
            /// and corresponding values can be used in notifications generated by this
            /// condition.
            ///
            /// Please see [the documentation on logs-based metric
            /// `valueExtractor`s](<https://cloud.google.com/logging/docs/reference/v2/rest/v2/projects.metrics#LogMetric.FIELDS.value_extractor>)
            /// for syntax and examples.
            #[prost(btree_map = "string, string", tag = "2")]
            pub label_extractors: ::prost::alloc::collections::BTreeMap<
                ::prost::alloc::string::String,
                ::prost::alloc::string::String,
            >,
        }
        /// A condition type that allows alert policies to be defined using
        /// [Monitoring Query Language](<https://cloud.google.com/monitoring/mql>).
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MonitoringQueryLanguageCondition {
            /// [Monitoring Query Language](<https://cloud.google.com/monitoring/mql>)
            /// query that outputs a boolean stream.
            #[prost(string, tag = "1")]
            pub query: ::prost::alloc::string::String,
            /// The amount of time that a time series must violate the
            /// threshold to be considered failing. Currently, only values
            /// that are a multiple of a minute--e.g., 0, 60, 120, or 300
            /// seconds--are supported. If an invalid value is given, an
            /// error will be returned. When choosing a duration, it is useful to
            /// keep in mind the frequency of the underlying time series data
            /// (which may also be affected by any alignments specified in the
            /// `aggregations` field); a good duration is long enough so that a single
            /// outlier does not generate spurious alerts, but short enough that
            /// unhealthy states are detected and alerted on quickly.
            #[prost(message, optional, tag = "2")]
            pub duration: ::core::option::Option<::prost_types::Duration>,
            /// The number/percent of time series for which the comparison must hold
            /// in order for the condition to trigger. If unspecified, then the
            /// condition will trigger if the comparison is true for any of the
            /// time series that have been identified by `filter` and `aggregations`,
            /// or by the ratio, if `denominator_filter` and `denominator_aggregations`
            /// are specified.
            #[prost(message, optional, tag = "3")]
            pub trigger: ::core::option::Option<Trigger>,
            /// A condition control that determines how metric-threshold conditions
            /// are evaluated when data stops arriving.
            #[prost(enumeration = "EvaluationMissingData", tag = "4")]
            pub evaluation_missing_data: i32,
        }
        /// A condition type that allows alert policies to be defined using
        /// [Prometheus Query Language
        /// (PromQL)](<https://prometheus.io/docs/prometheus/latest/querying/basics/>).
        ///
        /// The PrometheusQueryLanguageCondition message contains information
        /// from a Prometheus alerting rule and its associated rule group.
        ///
        /// A Prometheus alerting rule is described
        /// [here](<https://prometheus.io/docs/prometheus/latest/configuration/alerting_rules/>).
        /// The semantics of a Prometheus alerting rule is described
        /// [here](<https://prometheus.io/docs/prometheus/latest/configuration/recording_rules/#rule>).
        ///
        /// A Prometheus rule group is described
        /// [here](<https://prometheus.io/docs/prometheus/latest/configuration/recording_rules/>).
        /// The semantics of a Prometheus rule group is described
        /// [here](<https://prometheus.io/docs/prometheus/latest/configuration/recording_rules/#rule_group>).
        ///
        /// Because Cloud Alerting has no representation of a Prometheus rule
        /// group resource, we must embed the information of the parent rule
        /// group inside each of the conditions that refer to it. We must also
        /// update the contents of all Prometheus alerts in case the information
        /// of their rule group changes.
        ///
        /// The PrometheusQueryLanguageCondition protocol buffer combines the
        /// information of the corresponding rule group and alerting rule.
        /// The structure of the PrometheusQueryLanguageCondition protocol buffer
        /// does NOT mimic the structure of the Prometheus rule group and alerting
        /// rule YAML declarations. The PrometheusQueryLanguageCondition protocol
        /// buffer may change in the future to support future rule group and/or
        /// alerting rule features. There are no new such features at the present
        /// time (2023-06-26).
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PrometheusQueryLanguageCondition {
            /// Required. The PromQL expression to evaluate. Every evaluation cycle
            /// this expression is evaluated at the current time, and all resultant
            /// time series become pending/firing alerts. This field must not be empty.
            #[prost(string, tag = "1")]
            pub query: ::prost::alloc::string::String,
            /// Optional. Alerts are considered firing once their PromQL expression was
            /// evaluated to be "true" for this long.
            /// Alerts whose PromQL expression was not evaluated to be "true" for
            /// long enough are considered pending.
            /// Must be a non-negative duration or missing.
            /// This field is optional. Its default value is zero.
            #[prost(message, optional, tag = "2")]
            pub duration: ::core::option::Option<::prost_types::Duration>,
            /// Optional. How often this rule should be evaluated.
            /// Must be a positive multiple of 30 seconds or missing.
            /// This field is optional. Its default value is 30 seconds.
            /// If this PrometheusQueryLanguageCondition was generated from a
            /// Prometheus alerting rule, then this value should be taken from the
            /// enclosing rule group.
            #[prost(message, optional, tag = "3")]
            pub evaluation_interval: ::core::option::Option<::prost_types::Duration>,
            /// Optional. Labels to add to or overwrite in the PromQL query result.
            /// Label names [must be
            /// valid](<https://prometheus.io/docs/concepts/data_model/#metric-names-and-labels>).
            /// Label values can be [templatized by using
            /// variables](<https://cloud.google.com/monitoring/alerts/doc-variables>).
            /// The only available variable names are the names of the labels in the
            /// PromQL result, including "__name__" and "value". "labels" may be empty.
            #[prost(btree_map = "string, string", tag = "4")]
            pub labels: ::prost::alloc::collections::BTreeMap<
                ::prost::alloc::string::String,
                ::prost::alloc::string::String,
            >,
            /// Optional. The rule group name of this alert in the corresponding
            /// Prometheus configuration file.
            ///
            /// Some external tools may require this field to be populated correctly
            /// in order to refer to the original Prometheus configuration file.
            /// The rule group name and the alert name are necessary to update the
            /// relevant AlertPolicies in case the definition of the rule group changes
            /// in the future.
            ///
            /// This field is optional. If this field is not empty, then it must
            /// contain a valid UTF-8 string.
            /// This field may not exceed 2048 Unicode characters in length.
            #[prost(string, tag = "5")]
            pub rule_group: ::prost::alloc::string::String,
            /// Optional. The alerting rule name of this alert in the corresponding
            /// Prometheus configuration file.
            ///
            /// Some external tools may require this field to be populated correctly
            /// in order to refer to the original Prometheus configuration file.
            /// The rule group name and the alert name are necessary to update the
            /// relevant AlertPolicies in case the definition of the rule group changes
            /// in the future.
            ///
            /// This field is optional. If this field is not empty, then it must be a
            /// [valid Prometheus label
            /// name](<https://prometheus.io/docs/concepts/data_model/#metric-names-and-labels>).
            /// This field may not exceed 2048 Unicode characters in length.
            #[prost(string, tag = "6")]
            pub alert_rule: ::prost::alloc::string::String,
        }
        /// A condition control that determines how metric-threshold conditions
        /// are evaluated when data stops arriving.
        /// This control doesn't affect metric-absence policies.
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
        pub enum EvaluationMissingData {
            /// An unspecified evaluation missing data option.  Equivalent to
            /// EVALUATION_MISSING_DATA_NO_OP.
            Unspecified = 0,
            /// If there is no data to evaluate the condition, then evaluate the
            /// condition as false.
            Inactive = 1,
            /// If there is no data to evaluate the condition, then evaluate the
            /// condition as true.
            Active = 2,
            /// Do not evaluate the condition to any value if there is no data.
            NoOp = 3,
        }
        impl EvaluationMissingData {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    EvaluationMissingData::Unspecified => {
                        "EVALUATION_MISSING_DATA_UNSPECIFIED"
                    }
                    EvaluationMissingData::Inactive => "EVALUATION_MISSING_DATA_INACTIVE",
                    EvaluationMissingData::Active => "EVALUATION_MISSING_DATA_ACTIVE",
                    EvaluationMissingData::NoOp => "EVALUATION_MISSING_DATA_NO_OP",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "EVALUATION_MISSING_DATA_UNSPECIFIED" => Some(Self::Unspecified),
                    "EVALUATION_MISSING_DATA_INACTIVE" => Some(Self::Inactive),
                    "EVALUATION_MISSING_DATA_ACTIVE" => Some(Self::Active),
                    "EVALUATION_MISSING_DATA_NO_OP" => Some(Self::NoOp),
                    _ => None,
                }
            }
        }
        /// Only one of the following condition types will be specified.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Condition {
            /// A condition that compares a time series against a threshold.
            #[prost(message, tag = "1")]
            ConditionThreshold(MetricThreshold),
            /// A condition that checks that a time series continues to
            /// receive new data points.
            #[prost(message, tag = "2")]
            ConditionAbsent(MetricAbsence),
            /// A condition that checks for log messages matching given constraints. If
            /// set, no other conditions can be present.
            #[prost(message, tag = "20")]
            ConditionMatchedLog(LogMatch),
            /// A condition that uses the Monitoring Query Language to define
            /// alerts.
            #[prost(message, tag = "19")]
            ConditionMonitoringQueryLanguage(MonitoringQueryLanguageCondition),
            /// A condition that uses the Prometheus query language to define alerts.
            #[prost(message, tag = "21")]
            ConditionPrometheusQueryLanguage(PrometheusQueryLanguageCondition),
        }
    }
    /// Control over how the notification channels in `notification_channels`
    /// are notified when this alert fires.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AlertStrategy {
        /// Required for alert policies with a `LogMatch` condition.
        ///
        /// This limit is not implemented for alert policies that are not log-based.
        #[prost(message, optional, tag = "1")]
        pub notification_rate_limit: ::core::option::Option<
            alert_strategy::NotificationRateLimit,
        >,
        /// If an alert policy that was active has no data for this long, any open
        /// incidents will close
        #[prost(message, optional, tag = "3")]
        pub auto_close: ::core::option::Option<::prost_types::Duration>,
        /// Control how notifications will be sent out, on a per-channel basis.
        #[prost(message, repeated, tag = "4")]
        pub notification_channel_strategy: ::prost::alloc::vec::Vec<
            alert_strategy::NotificationChannelStrategy,
        >,
    }
    /// Nested message and enum types in `AlertStrategy`.
    pub mod alert_strategy {
        /// Control over the rate of notifications sent to this alert policy's
        /// notification channels.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct NotificationRateLimit {
            /// Not more than one notification per `period`.
            #[prost(message, optional, tag = "1")]
            pub period: ::core::option::Option<::prost_types::Duration>,
        }
        /// Control over how the notification channels in `notification_channels`
        /// are notified when this alert fires, on a per-channel basis.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct NotificationChannelStrategy {
            /// The full REST resource name for the notification channels that these
            /// settings apply to. Each of these correspond to the name field in one
            /// of the NotificationChannel objects referenced in the
            /// notification_channels field of this AlertPolicy.
            /// The format is:
            ///
            ///      projects/\[PROJECT_ID_OR_NUMBER\]/notificationChannels/\[CHANNEL_ID\]
            #[prost(string, repeated, tag = "1")]
            pub notification_channel_names: ::prost::alloc::vec::Vec<
                ::prost::alloc::string::String,
            >,
            /// The frequency at which to send reminder notifications for open
            /// incidents.
            #[prost(message, optional, tag = "2")]
            pub renotify_interval: ::core::option::Option<::prost_types::Duration>,
        }
    }
    /// Operators for combining conditions.
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
    pub enum ConditionCombinerType {
        /// An unspecified combiner.
        CombineUnspecified = 0,
        /// Combine conditions using the logical `AND` operator. An
        /// incident is created only if all the conditions are met
        /// simultaneously. This combiner is satisfied if all conditions are
        /// met, even if they are met on completely different resources.
        And = 1,
        /// Combine conditions using the logical `OR` operator. An incident
        /// is created if any of the listed conditions is met.
        Or = 2,
        /// Combine conditions using logical `AND` operator, but unlike the regular
        /// `AND` option, an incident is created only if all conditions are met
        /// simultaneously on at least one resource.
        AndWithMatchingResource = 3,
    }
    impl ConditionCombinerType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConditionCombinerType::CombineUnspecified => "COMBINE_UNSPECIFIED",
                ConditionCombinerType::And => "AND",
                ConditionCombinerType::Or => "OR",
                ConditionCombinerType::AndWithMatchingResource => {
                    "AND_WITH_MATCHING_RESOURCE"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMBINE_UNSPECIFIED" => Some(Self::CombineUnspecified),
                "AND" => Some(Self::And),
                "OR" => Some(Self::Or),
                "AND_WITH_MATCHING_RESOURCE" => Some(Self::AndWithMatchingResource),
                _ => None,
            }
        }
    }
    /// An enumeration of possible severity level for an Alert Policy.
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
        /// No severity is specified. This is the default value.
        Unspecified = 0,
        /// This is the highest severity level. Use this if the problem could
        /// cause significant damage or downtime.
        Critical = 1,
        /// This is the medium severity level. Use this if the problem could
        /// cause minor damage or downtime.
        Error = 2,
        /// This is the lowest severity level. Use this if the problem is not causing
        /// any damage or downtime, but could potentially lead to a problem in the
        /// future.
        Warning = 3,
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
                Severity::Error => "ERROR",
                Severity::Warning => "WARNING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
                "CRITICAL" => Some(Self::Critical),
                "ERROR" => Some(Self::Error),
                "WARNING" => Some(Self::Warning),
                _ => None,
            }
        }
    }
}
/// A set of (label, value) pairs that were removed from a Distribution
/// time series during aggregation and then added as an attachment to a
/// Distribution.Exemplar.
///
/// The full label set for the exemplars is constructed by using the dropped
/// pairs in combination with the label values that remain on the aggregated
/// Distribution time series. The constructed full label set can be used to
/// identify the specific entity, such as the instance or job, which might be
/// contributing to a long-tail. However, with dropped labels, the storage
/// requirements are reduced because only the aggregated distribution values for
/// a large group of time series are stored.
///
/// Note that there are no guarantees on ordering of the labels from
/// exemplar-to-exemplar and from distribution-to-distribution in the same
/// stream, and there may be duplicates.  It is up to clients to resolve any
/// ambiguities.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DroppedLabels {
    /// Map from label to its value, for all labels dropped in any aggregation.
    #[prost(btree_map = "string, string", tag = "1")]
    pub label: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// The protocol for the `CreateAlertPolicy` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAlertPolicyRequest {
    /// Required. The
    /// [project](<https://cloud.google.com/monitoring/api/v3#project_name>) in which
    /// to create the alerting policy. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    ///
    /// Note that this field names the parent container in which the alerting
    /// policy will be written, not the name of the created policy. |name| must be
    /// a host project of a Metrics Scope, otherwise INVALID_ARGUMENT error will
    /// return. The alerting policy that is returned will have a name that contains
    /// a normalized representation of this name as a prefix but adds a suffix of
    /// the form `/alertPolicies/\[ALERT_POLICY_ID\]`, identifying the policy in the
    /// container.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The requested alerting policy. You should omit the `name` field
    /// in this policy. The name will be returned in the new policy, including a
    /// new `\[ALERT_POLICY_ID\]` value.
    #[prost(message, optional, tag = "2")]
    pub alert_policy: ::core::option::Option<AlertPolicy>,
}
/// The protocol for the `GetAlertPolicy` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAlertPolicyRequest {
    /// Required. The alerting policy to retrieve. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/alertPolicies/\[ALERT_POLICY_ID\]
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// The protocol for the `ListAlertPolicies` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAlertPoliciesRequest {
    /// Required. The
    /// [project](<https://cloud.google.com/monitoring/api/v3#project_name>) whose
    /// alert policies are to be listed. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]
    ///
    /// Note that this field names the parent container in which the alerting
    /// policies to be listed are stored. To retrieve a single alerting policy
    /// by name, use the
    /// [GetAlertPolicy][google.monitoring.v3.AlertPolicyService.GetAlertPolicy]
    /// operation, instead.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// If provided, this field specifies the criteria that must be met by
    /// alert policies to be included in the response.
    ///
    /// For more details, see [sorting and
    /// filtering](<https://cloud.google.com/monitoring/api/v3/sorting-and-filtering>).
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// A comma-separated list of fields by which to sort the result. Supports
    /// the same set of field references as the `filter` field. Entries can be
    /// prefixed with a minus sign to sort by the field in descending order.
    ///
    /// For more details, see [sorting and
    /// filtering](<https://cloud.google.com/monitoring/api/v3/sorting-and-filtering>).
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return more results from the previous method call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The protocol for the `ListAlertPolicies` response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAlertPoliciesResponse {
    /// The returned alert policies.
    #[prost(message, repeated, tag = "3")]
    pub alert_policies: ::prost::alloc::vec::Vec<AlertPolicy>,
    /// If there might be more results than were returned, then this field is set
    /// to a non-empty value. To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of alert policies in all pages. This number is only an
    /// estimate, and may change in subsequent pages. <https://aip.dev/158>
    #[prost(int32, tag = "4")]
    pub total_size: i32,
}
/// The protocol for the `UpdateAlertPolicy` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAlertPolicyRequest {
    /// Optional. A list of alerting policy field names. If this field is not
    /// empty, each listed field in the existing alerting policy is set to the
    /// value of the corresponding field in the supplied policy (`alert_policy`),
    /// or to the field's default value if the field is not in the supplied
    /// alerting policy.  Fields not listed retain their previous value.
    ///
    /// Examples of valid field masks include `display_name`, `documentation`,
    /// `documentation.content`, `documentation.mime_type`, `user_labels`,
    /// `user_label.nameofkey`, `enabled`, `conditions`, `combiner`, etc.
    ///
    /// If this field is empty, then the supplied alerting policy replaces the
    /// existing policy. It is the same as deleting the existing policy and
    /// adding the supplied policy, except for the following:
    ///
    /// +   The new policy will have the same `\[ALERT_POLICY_ID\]` as the former
    ///      policy. This gives you continuity with the former policy in your
    ///      notifications and incidents.
    /// +   Conditions in the new policy will keep their former `\[CONDITION_ID\]` if
    ///      the supplied condition includes the `name` field with that
    ///      `\[CONDITION_ID\]`. If the supplied condition omits the `name` field,
    ///      then a new `\[CONDITION_ID\]` is created.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The updated alerting policy or the updated values for the
    /// fields listed in `update_mask`.
    /// If `update_mask` is not empty, any fields in this policy that are
    /// not in `update_mask` are ignored.
    #[prost(message, optional, tag = "3")]
    pub alert_policy: ::core::option::Option<AlertPolicy>,
}
/// The protocol for the `DeleteAlertPolicy` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAlertPolicyRequest {
    /// Required. The alerting policy to delete. The format is:
    ///
    ///      projects/\[PROJECT_ID_OR_NUMBER\]/alertPolicies/\[ALERT_POLICY_ID\]
    ///
    /// For more information, see [AlertPolicy][google.monitoring.v3.AlertPolicy].
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod alert_policy_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The AlertPolicyService API is used to manage (list, create, delete,
    /// edit) alert policies in Cloud Monitoring. An alerting policy is
    /// a description of the conditions under which some aspect of your
    /// system is considered to be "unhealthy" and the ways to notify
    /// people or services about this state. In addition to using this API, alert
    /// policies can also be managed through
    /// [Cloud Monitoring](https://cloud.google.com/monitoring/docs/),
    /// which can be reached by clicking the "Monitoring" tab in
    /// [Cloud console](https://console.cloud.google.com/).
    #[derive(Debug, Clone)]
    pub struct AlertPolicyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AlertPolicyServiceClient<T>
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
        ) -> AlertPolicyServiceClient<InterceptedService<T, F>>
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
            AlertPolicyServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists the existing alerting policies for the workspace.
        pub async fn list_alert_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAlertPoliciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAlertPoliciesResponse>,
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
                "/google.monitoring.v3.AlertPolicyService/ListAlertPolicies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.AlertPolicyService",
                        "ListAlertPolicies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a single alerting policy.
        pub async fn get_alert_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAlertPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::AlertPolicy>, tonic::Status> {
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
                "/google.monitoring.v3.AlertPolicyService/GetAlertPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.AlertPolicyService",
                        "GetAlertPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new alerting policy.
        ///
        /// Design your application to single-thread API calls that modify the state of
        /// alerting policies in a single project. This includes calls to
        /// CreateAlertPolicy, DeleteAlertPolicy and UpdateAlertPolicy.
        pub async fn create_alert_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAlertPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::AlertPolicy>, tonic::Status> {
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
                "/google.monitoring.v3.AlertPolicyService/CreateAlertPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.AlertPolicyService",
                        "CreateAlertPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an alerting policy.
        ///
        /// Design your application to single-thread API calls that modify the state of
        /// alerting policies in a single project. This includes calls to
        /// CreateAlertPolicy, DeleteAlertPolicy and UpdateAlertPolicy.
        pub async fn delete_alert_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAlertPolicyRequest>,
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
                "/google.monitoring.v3.AlertPolicyService/DeleteAlertPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.AlertPolicyService",
                        "DeleteAlertPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an alerting policy. You can either replace the entire policy with
        /// a new one or replace only certain fields in the current alerting policy by
        /// specifying the fields to be updated via `updateMask`. Returns the
        /// updated alerting policy.
        ///
        /// Design your application to single-thread API calls that modify the state of
        /// alerting policies in a single project. This includes calls to
        /// CreateAlertPolicy, DeleteAlertPolicy and UpdateAlertPolicy.
        pub async fn update_alert_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAlertPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::AlertPolicy>, tonic::Status> {
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
                "/google.monitoring.v3.AlertPolicyService/UpdateAlertPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.monitoring.v3.AlertPolicyService",
                        "UpdateAlertPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
