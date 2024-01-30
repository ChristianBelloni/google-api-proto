/// Type indicates the type of the log entry and can be used as a filter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    /// Type is unspecified.
    Unspecified = 0,
    /// A Pub/Sub notification failed to be sent.
    PubsubNotificationFailure = 1,
    /// Resource state changed.
    ResourceStateChange = 3,
    /// A process aborted.
    ProcessAborted = 4,
    /// Restriction check failed.
    RestrictionViolated = 5,
    /// Resource deleted.
    ResourceDeleted = 6,
    /// Rollout updated.
    RolloutUpdate = 7,
    /// Deprecated: This field is never used. Use release_render log type instead.
    RenderStatuesChange = 2,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::Unspecified => "TYPE_UNSPECIFIED",
            Type::PubsubNotificationFailure => "TYPE_PUBSUB_NOTIFICATION_FAILURE",
            Type::ResourceStateChange => "TYPE_RESOURCE_STATE_CHANGE",
            Type::ProcessAborted => "TYPE_PROCESS_ABORTED",
            Type::RestrictionViolated => "TYPE_RESTRICTION_VIOLATED",
            Type::ResourceDeleted => "TYPE_RESOURCE_DELETED",
            Type::RolloutUpdate => "TYPE_ROLLOUT_UPDATE",
            Type::RenderStatuesChange => "TYPE_RENDER_STATUES_CHANGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TYPE_PUBSUB_NOTIFICATION_FAILURE" => Some(Self::PubsubNotificationFailure),
            "TYPE_RESOURCE_STATE_CHANGE" => Some(Self::ResourceStateChange),
            "TYPE_PROCESS_ABORTED" => Some(Self::ProcessAborted),
            "TYPE_RESTRICTION_VIOLATED" => Some(Self::RestrictionViolated),
            "TYPE_RESOURCE_DELETED" => Some(Self::ResourceDeleted),
            "TYPE_ROLLOUT_UPDATE" => Some(Self::RolloutUpdate),
            "TYPE_RENDER_STATUES_CHANGE" => Some(Self::RenderStatuesChange),
            _ => None,
        }
    }
}
/// Payload proto for "clouddeploy.googleapis.com/automation_run"
/// Platform Log event that describes the AutomationRun related events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomationRunEvent {
    /// Debug message for when there is an update on the AutomationRun.
    /// Provides further details about the resource creation or state change.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// The name of the `AutomationRun`.
    #[prost(string, tag = "2")]
    pub automation_run: ::prost::alloc::string::String,
    /// Unique identifier of the `DeliveryPipeline`.
    #[prost(string, tag = "3")]
    pub pipeline_uid: ::prost::alloc::string::String,
    /// Identifier of the `Automation`.
    #[prost(string, tag = "4")]
    pub automation_id: ::prost::alloc::string::String,
    /// Identifier of the `Automation` rule.
    #[prost(string, tag = "5")]
    pub rule_id: ::prost::alloc::string::String,
    /// ID of the `Target` to which the `AutomationRun` is created.
    #[prost(string, tag = "6")]
    pub destination_target_id: ::prost::alloc::string::String,
    /// Type of this notification, e.g. for a Pub/Sub failure.
    #[prost(enumeration = "Type", tag = "7")]
    pub r#type: i32,
}
/// Payload proto for "clouddeploy.googleapis.com/automation"
/// Platform Log event that describes the Automation related events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomationEvent {
    /// Debug message for when there is an update on the AutomationRun.
    /// Provides further details about the resource creation or state change.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// The name of the `AutomationRun`.
    #[prost(string, tag = "2")]
    pub automation: ::prost::alloc::string::String,
    /// Unique identifier of the `DeliveryPipeline`.
    #[prost(string, tag = "3")]
    pub pipeline_uid: ::prost::alloc::string::String,
    /// Type of this notification, e.g. for a Pub/Sub failure.
    #[prost(enumeration = "Type", tag = "4")]
    pub r#type: i32,
}
/// A `DeliveryPipeline` resource in the Cloud Deploy API.
///
/// A `DeliveryPipeline` defines a pipeline through which a Skaffold
/// configuration can progress.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliveryPipeline {
    /// Optional. Name of the `DeliveryPipeline`. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/[a-z][a-z0-9\-]{0,62}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Unique identifier of the `DeliveryPipeline`.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Description of the `DeliveryPipeline`. Max length is 255 characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// User annotations. These attributes can only be set and used by the
    /// user, and not by Cloud Deploy.
    #[prost(btree_map = "string, string", tag = "4")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Labels are attributes that can be set and used by both the
    /// user and by Cloud Deploy. Labels must meet the following constraints:
    ///
    /// * Keys and values can contain only lowercase letters, numeric characters,
    /// underscores, and dashes.
    /// * All characters must use UTF-8 encoding, and international characters are
    /// allowed.
    /// * Keys must start with a lowercase letter or international character.
    /// * Each resource is limited to a maximum of 64 labels.
    ///
    /// Both keys and values are additionally constrained to be <= 128 bytes.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Time at which the pipeline was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Most recent time at which the pipeline was updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Information around the state of the Delivery Pipeline.
    #[prost(message, optional, tag = "11")]
    pub condition: ::core::option::Option<PipelineCondition>,
    /// This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "10")]
    pub etag: ::prost::alloc::string::String,
    /// When suspended, no new releases or rollouts can be created,
    /// but in-progress ones will complete.
    #[prost(bool, tag = "12")]
    pub suspended: bool,
    /// The ordering configuration of the `DeliveryPipeline`.
    #[prost(oneof = "delivery_pipeline::Pipeline", tags = "8")]
    pub pipeline: ::core::option::Option<delivery_pipeline::Pipeline>,
}
/// Nested message and enum types in `DeliveryPipeline`.
pub mod delivery_pipeline {
    /// The ordering configuration of the `DeliveryPipeline`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Pipeline {
        /// SerialPipeline defines a sequential set of stages for a
        /// `DeliveryPipeline`.
        #[prost(message, tag = "8")]
        SerialPipeline(super::SerialPipeline),
    }
}
/// SerialPipeline defines a sequential set of stages for a `DeliveryPipeline`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerialPipeline {
    /// Each stage specifies configuration for a `Target`. The ordering
    /// of this list defines the promotion flow.
    #[prost(message, repeated, tag = "1")]
    pub stages: ::prost::alloc::vec::Vec<Stage>,
}
/// Stage specifies a location to which to deploy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stage {
    /// The target_id to which this stage points. This field refers exclusively to
    /// the last segment of a target name. For example, this field would just be
    /// `my-target` (rather than
    /// `projects/project/locations/location/targets/my-target`). The location of
    /// the `Target` is inferred to be the same as the location of the
    /// `DeliveryPipeline` that contains this `Stage`.
    #[prost(string, tag = "1")]
    pub target_id: ::prost::alloc::string::String,
    /// Skaffold profiles to use when rendering the manifest for this stage's
    /// `Target`.
    #[prost(string, repeated, tag = "2")]
    pub profiles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The strategy to use for a `Rollout` to this stage.
    #[prost(message, optional, tag = "5")]
    pub strategy: ::core::option::Option<Strategy>,
    /// Optional. The deploy parameters to use for the target in this stage.
    #[prost(message, repeated, tag = "6")]
    pub deploy_parameters: ::prost::alloc::vec::Vec<DeployParameters>,
}
/// DeployParameters contains deploy parameters information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployParameters {
    /// Required. Values are deploy parameters in key-value pairs.
    #[prost(btree_map = "string, string", tag = "1")]
    pub values: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. Deploy parameters are applied to targets with match labels.
    /// If unspecified, deploy parameters are applied to all targets (including
    /// child targets of a multi-target).
    #[prost(btree_map = "string, string", tag = "2")]
    pub match_target_labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Strategy contains deployment strategy information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Strategy {
    /// Deployment strategy details.
    #[prost(oneof = "strategy::DeploymentStrategy", tags = "1, 2")]
    pub deployment_strategy: ::core::option::Option<strategy::DeploymentStrategy>,
}
/// Nested message and enum types in `Strategy`.
pub mod strategy {
    /// Deployment strategy details.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DeploymentStrategy {
        /// Standard deployment strategy executes a single deploy and allows
        /// verifying the deployment.
        #[prost(message, tag = "1")]
        Standard(super::Standard),
        /// Canary deployment strategy provides progressive percentage based
        /// deployments to a Target.
        #[prost(message, tag = "2")]
        Canary(super::Canary),
    }
}
/// Predeploy contains the predeploy job configuration information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Predeploy {
    /// Optional. A sequence of Skaffold custom actions to invoke during execution
    /// of the predeploy job.
    #[prost(string, repeated, tag = "1")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Postdeploy contains the postdeploy job configuration information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Postdeploy {
    /// Optional. A sequence of Skaffold custom actions to invoke during execution
    /// of the postdeploy job.
    #[prost(string, repeated, tag = "1")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Standard represents the standard deployment strategy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Standard {
    /// Whether to verify a deployment.
    #[prost(bool, tag = "1")]
    pub verify: bool,
    /// Optional. Configuration for the predeploy job. If this is not configured,
    /// predeploy job will not be present.
    #[prost(message, optional, tag = "2")]
    pub predeploy: ::core::option::Option<Predeploy>,
    /// Optional. Configuration for the postdeploy job. If this is not configured,
    /// postdeploy job will not be present.
    #[prost(message, optional, tag = "3")]
    pub postdeploy: ::core::option::Option<Postdeploy>,
}
/// Canary represents the canary deployment strategy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Canary {
    /// Optional. Runtime specific configurations for the deployment strategy. The
    /// runtime configuration is used to determine how Cloud Deploy will split
    /// traffic to enable a progressive deployment.
    #[prost(message, optional, tag = "1")]
    pub runtime_config: ::core::option::Option<RuntimeConfig>,
    /// The mode to use for the canary deployment strategy.
    #[prost(oneof = "canary::Mode", tags = "2, 3")]
    pub mode: ::core::option::Option<canary::Mode>,
}
/// Nested message and enum types in `Canary`.
pub mod canary {
    /// The mode to use for the canary deployment strategy.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mode {
        /// Configures the progressive based deployment for a Target.
        #[prost(message, tag = "2")]
        CanaryDeployment(super::CanaryDeployment),
        /// Configures the progressive based deployment for a Target, but allows
        /// customizing at the phase level where a phase represents each of the
        /// percentage deployments.
        #[prost(message, tag = "3")]
        CustomCanaryDeployment(super::CustomCanaryDeployment),
    }
}
/// CanaryDeployment represents the canary deployment configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanaryDeployment {
    /// Required. The percentage based deployments that will occur as a part of a
    /// `Rollout`. List is expected in ascending order and each integer n is
    /// 0 <= n < 100.
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub percentages: ::prost::alloc::vec::Vec<i32>,
    /// Whether to run verify tests after each percentage deployment.
    #[prost(bool, tag = "2")]
    pub verify: bool,
    /// Optional. Configuration for the predeploy job of the first phase. If this
    /// is not configured, there will be no predeploy job for this phase.
    #[prost(message, optional, tag = "3")]
    pub predeploy: ::core::option::Option<Predeploy>,
    /// Optional. Configuration for the postdeploy job of the last phase. If this
    /// is not configured, there will be no postdeploy job for this phase.
    #[prost(message, optional, tag = "4")]
    pub postdeploy: ::core::option::Option<Postdeploy>,
}
/// CustomCanaryDeployment represents the custom canary deployment
/// configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomCanaryDeployment {
    /// Required. Configuration for each phase in the canary deployment in the
    /// order executed.
    #[prost(message, repeated, tag = "1")]
    pub phase_configs: ::prost::alloc::vec::Vec<custom_canary_deployment::PhaseConfig>,
}
/// Nested message and enum types in `CustomCanaryDeployment`.
pub mod custom_canary_deployment {
    /// PhaseConfig represents the configuration for a phase in the custom
    /// canary deployment.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PhaseConfig {
        /// Required. The ID to assign to the `Rollout` phase.
        /// This value must consist of lower-case letters, numbers, and hyphens,
        /// start with a letter and end with a letter or a number, and have a max
        /// length of 63 characters. In other words, it must match the following
        /// regex: `^[a-z](\[a-z0-9-\]{0,61}\[a-z0-9\])?$`.
        #[prost(string, tag = "1")]
        pub phase_id: ::prost::alloc::string::String,
        /// Required. Percentage deployment for the phase.
        #[prost(int32, tag = "2")]
        pub percentage: i32,
        /// Skaffold profiles to use when rendering the manifest for this phase.
        /// These are in addition to the profiles list specified in the
        /// `DeliveryPipeline` stage.
        #[prost(string, repeated, tag = "3")]
        pub profiles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Whether to run verify tests after the deployment.
        #[prost(bool, tag = "4")]
        pub verify: bool,
        /// Optional. Configuration for the predeploy job of this phase. If this is
        /// not configured, there will be no predeploy job for this phase.
        #[prost(message, optional, tag = "5")]
        pub predeploy: ::core::option::Option<super::Predeploy>,
        /// Optional. Configuration for the postdeploy job of this phase. If this is
        /// not configured, there will be no postdeploy job for this phase.
        #[prost(message, optional, tag = "6")]
        pub postdeploy: ::core::option::Option<super::Postdeploy>,
    }
}
/// KubernetesConfig contains the Kubernetes runtime configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesConfig {
    /// The service definition configuration.
    #[prost(oneof = "kubernetes_config::ServiceDefinition", tags = "1, 2")]
    pub service_definition: ::core::option::Option<kubernetes_config::ServiceDefinition>,
}
/// Nested message and enum types in `KubernetesConfig`.
pub mod kubernetes_config {
    /// Information about the Kubernetes Gateway API service mesh configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GatewayServiceMesh {
        /// Required. Name of the Gateway API HTTPRoute.
        #[prost(string, tag = "1")]
        pub http_route: ::prost::alloc::string::String,
        /// Required. Name of the Kubernetes Service.
        #[prost(string, tag = "2")]
        pub service: ::prost::alloc::string::String,
        /// Required. Name of the Kubernetes Deployment whose traffic is managed by
        /// the specified HTTPRoute and Service.
        #[prost(string, tag = "3")]
        pub deployment: ::prost::alloc::string::String,
        /// Optional. The time to wait for route updates to propagate. The maximum
        /// configurable time is 3 hours, in seconds format. If unspecified, there is
        /// no wait time.
        #[prost(message, optional, tag = "4")]
        pub route_update_wait_time: ::core::option::Option<::prost_types::Duration>,
        /// Optional. The amount of time to migrate traffic back from the canary
        /// Service to the original Service during the stable phase deployment. If
        /// specified, must be between 15s and 3600s. If unspecified, there is no
        /// cutback time.
        #[prost(message, optional, tag = "5")]
        pub stable_cutback_duration: ::core::option::Option<::prost_types::Duration>,
    }
    /// Information about the Kubernetes Service networking configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServiceNetworking {
        /// Required. Name of the Kubernetes Service.
        #[prost(string, tag = "1")]
        pub service: ::prost::alloc::string::String,
        /// Required. Name of the Kubernetes Deployment whose traffic is managed by
        /// the specified Service.
        #[prost(string, tag = "2")]
        pub deployment: ::prost::alloc::string::String,
        /// Optional. Whether to disable Pod overprovisioning. If Pod
        /// overprovisioning is disabled then Cloud Deploy will limit the number of
        /// total Pods used for the deployment strategy to the number of Pods the
        /// Deployment has on the cluster.
        #[prost(bool, tag = "3")]
        pub disable_pod_overprovisioning: bool,
    }
    /// The service definition configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ServiceDefinition {
        /// Kubernetes Gateway API service mesh configuration.
        #[prost(message, tag = "1")]
        GatewayServiceMesh(GatewayServiceMesh),
        /// Kubernetes Service networking configuration.
        #[prost(message, tag = "2")]
        ServiceNetworking(ServiceNetworking),
    }
}
/// CloudRunConfig contains the Cloud Run runtime configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudRunConfig {
    /// Whether Cloud Deploy should update the traffic stanza in a Cloud Run
    /// Service on the user's behalf to facilitate traffic splitting. This is
    /// required to be true for CanaryDeployments, but optional for
    /// CustomCanaryDeployments.
    #[prost(bool, tag = "1")]
    pub automatic_traffic_control: bool,
    /// Optional. A list of tags that are added to the canary revision while the
    /// canary phase is in progress.
    #[prost(string, repeated, tag = "2")]
    pub canary_revision_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. A list of tags that are added to the prior revision while the
    /// canary phase is in progress.
    #[prost(string, repeated, tag = "3")]
    pub prior_revision_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. A list of tags that are added to the final stable revision when
    /// the stable phase is applied.
    #[prost(string, repeated, tag = "4")]
    pub stable_revision_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// RuntimeConfig contains the runtime specific configurations for a deployment
/// strategy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeConfig {
    /// The runtime configuration details.
    #[prost(oneof = "runtime_config::RuntimeConfig", tags = "1, 2")]
    pub runtime_config: ::core::option::Option<runtime_config::RuntimeConfig>,
}
/// Nested message and enum types in `RuntimeConfig`.
pub mod runtime_config {
    /// The runtime configuration details.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RuntimeConfig {
        /// Kubernetes runtime configuration.
        #[prost(message, tag = "1")]
        Kubernetes(super::KubernetesConfig),
        /// Cloud Run runtime configuration.
        #[prost(message, tag = "2")]
        CloudRun(super::CloudRunConfig),
    }
}
/// PipelineReadyCondition contains information around the status of the
/// Pipeline.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineReadyCondition {
    /// True if the Pipeline is in a valid state. Otherwise at least one condition
    /// in `PipelineCondition` is in an invalid state. Iterate over those
    /// conditions and see which condition(s) has status = false to find out what
    /// is wrong with the Pipeline.
    #[prost(bool, tag = "3")]
    pub status: bool,
    /// Last time the condition was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// `TargetsPresentCondition` contains information on any Targets referenced in
/// the Delivery Pipeline that do not actually exist.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetsPresentCondition {
    /// True if there aren't any missing Targets.
    #[prost(bool, tag = "1")]
    pub status: bool,
    /// The list of Target names that do not exist. For example,
    /// `projects/{project_id}/locations/{location_name}/targets/{target_name}`.
    #[prost(string, repeated, tag = "2")]
    pub missing_targets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Last time the condition was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// TargetsTypeCondition contains information on whether the Targets defined in
/// the Delivery Pipeline are of the same type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetsTypeCondition {
    /// True if the targets are all a comparable type. For example this is true if
    /// all targets are GKE clusters. This is false if some targets are Cloud Run
    /// targets and others are GKE clusters.
    #[prost(bool, tag = "1")]
    pub status: bool,
    /// Human readable error message.
    #[prost(string, tag = "2")]
    pub error_details: ::prost::alloc::string::String,
}
/// PipelineCondition contains all conditions relevant to a Delivery Pipeline.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineCondition {
    /// Details around the Pipeline's overall status.
    #[prost(message, optional, tag = "1")]
    pub pipeline_ready_condition: ::core::option::Option<PipelineReadyCondition>,
    /// Details around targets enumerated in the pipeline.
    #[prost(message, optional, tag = "3")]
    pub targets_present_condition: ::core::option::Option<TargetsPresentCondition>,
    /// Details on the whether the targets enumerated in the pipeline are of the
    /// same type.
    #[prost(message, optional, tag = "4")]
    pub targets_type_condition: ::core::option::Option<TargetsTypeCondition>,
}
/// The request object for `ListDeliveryPipelines`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeliveryPipelinesRequest {
    /// Required. The parent, which owns this collection of pipelines. Format must
    /// be `projects/{project_id}/locations/{location_name}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of pipelines to return. The service may return
    /// fewer than this value. If unspecified, at most 50 pipelines will
    /// be returned. The maximum value is 1000; values above 1000 will be set
    /// to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListDeliveryPipelines` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other provided parameters match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter pipelines to be returned. See <https://google.aip.dev/160> for more
    /// details.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to sort by. See <https://google.aip.dev/132#ordering> for more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response object from `ListDeliveryPipelines`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeliveryPipelinesResponse {
    /// The `DeliveryPipeline` objects.
    #[prost(message, repeated, tag = "1")]
    pub delivery_pipelines: ::prost::alloc::vec::Vec<DeliveryPipeline>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request object for `GetDeliveryPipeline`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeliveryPipelineRequest {
    /// Required. Name of the `DeliveryPipeline`. Format must be
    /// `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request object for `CreateDeliveryPipeline`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDeliveryPipelineRequest {
    /// Required. The parent collection in which the `DeliveryPipeline` should be
    /// created. Format should be
    /// `projects/{project_id}/locations/{location_name}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the `DeliveryPipeline`.
    #[prost(string, tag = "2")]
    pub delivery_pipeline_id: ::prost::alloc::string::String,
    /// Required. The `DeliveryPipeline` to create.
    #[prost(message, optional, tag = "3")]
    pub delivery_pipeline: ::core::option::Option<DeliveryPipeline>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
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
    /// Optional. If set to true, the request is validated and the user is provided
    /// with an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// The request object for `UpdateDeliveryPipeline`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDeliveryPipelineRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `DeliveryPipeline` resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it's in the mask. If the
    /// user doesn't provide a mask then all fields are overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The `DeliveryPipeline` to update.
    #[prost(message, optional, tag = "2")]
    pub delivery_pipeline: ::core::option::Option<DeliveryPipeline>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
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
    /// Optional. If set to true, updating a `DeliveryPipeline` that does not exist
    /// will result in the creation of a new `DeliveryPipeline`.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
    /// Optional. If set to true, the request is validated and the user is provided
    /// with an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// The request object for `DeleteDeliveryPipeline`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDeliveryPipelineRequest {
    /// Required. The name of the `DeliveryPipeline` to delete. Format should be
    /// `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, then deleting an already deleted or non-existing
    /// `DeliveryPipeline` will succeed.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// Optional. If set, validate the request and preview the review, but do not
    /// actually post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// Optional. If set to true, all child resources under this pipeline will also
    /// be deleted. Otherwise, the request will only work if the pipeline has no
    /// child resources.
    #[prost(bool, tag = "6")]
    pub force: bool,
    /// Optional. This checksum is computed by the server based on the value of
    /// other fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "5")]
    pub etag: ::prost::alloc::string::String,
}
/// Configs for the Rollback rollout.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackTargetConfig {
    /// Optional. The rollback `Rollout` to create.
    #[prost(message, optional, tag = "1")]
    pub rollout: ::core::option::Option<Rollout>,
    /// Optional. The starting phase ID for the `Rollout`. If unspecified, the
    /// `Rollout` will start in the stable phase.
    #[prost(string, tag = "2")]
    pub starting_phase_id: ::prost::alloc::string::String,
}
/// The request object for `RollbackTarget`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackTargetRequest {
    /// Required. The `DeliveryPipeline` for which the rollback `Rollout` should be
    /// created. Format should be
    /// `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. ID of the `Target` that is being rolled back.
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    /// Required. ID of the rollback `Rollout` to create.
    #[prost(string, tag = "3")]
    pub rollout_id: ::prost::alloc::string::String,
    /// Optional. ID of the `Release` to roll back to. If this isn't specified, the
    /// previous successful `Rollout` to the specified target will be used to
    /// determine the `Release`.
    #[prost(string, tag = "4")]
    pub release_id: ::prost::alloc::string::String,
    /// Optional. If provided, this must be the latest `Rollout` that is on the
    /// `Target`.
    #[prost(string, tag = "5")]
    pub rollout_to_roll_back: ::prost::alloc::string::String,
    /// Optional. Configs for the rollback `Rollout`.
    #[prost(message, optional, tag = "6")]
    pub rollback_config: ::core::option::Option<RollbackTargetConfig>,
    /// Optional. If set to true, the request is validated and the user is provided
    /// with a `RollbackTargetResponse`.
    #[prost(bool, tag = "7")]
    pub validate_only: bool,
}
/// The response object from `RollbackTarget`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackTargetResponse {
    /// The config of the rollback `Rollout` created or will be created.
    #[prost(message, optional, tag = "1")]
    pub rollback_config: ::core::option::Option<RollbackTargetConfig>,
}
/// A `Target` resource in the Cloud Deploy API.
///
/// A `Target` defines a location to which a Skaffold configuration
/// can be deployed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Target {
    /// Optional. Name of the `Target`. Format is
    /// `projects/{project}/locations/{location}/targets/[a-z][a-z0-9\-]{0,62}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Resource id of the `Target`.
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    /// Output only. Unique identifier of the `Target`.
    #[prost(string, tag = "3")]
    pub uid: ::prost::alloc::string::String,
    /// Optional. Description of the `Target`. Max length is 255 characters.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Optional. User annotations. These attributes can only be set and used by
    /// the user, and not by Cloud Deploy. See
    /// <https://google.aip.dev/128#annotations> for more details such as format and
    /// size limitations.
    #[prost(btree_map = "string, string", tag = "5")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. Labels are attributes that can be set and used by both the
    /// user and by Cloud Deploy. Labels must meet the following constraints:
    ///
    /// * Keys and values can contain only lowercase letters, numeric characters,
    /// underscores, and dashes.
    /// * All characters must use UTF-8 encoding, and international characters are
    /// allowed.
    /// * Keys must start with a lowercase letter or international character.
    /// * Each resource is limited to a maximum of 64 labels.
    ///
    /// Both keys and values are additionally constrained to be <= 128 bytes.
    #[prost(btree_map = "string, string", tag = "6")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. Whether or not the `Target` requires approval.
    #[prost(bool, tag = "13")]
    pub require_approval: bool,
    /// Output only. Time at which the `Target` was created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Most recent time at which the `Target` was updated.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. This checksum is computed by the server based on the value of
    /// other fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "12")]
    pub etag: ::prost::alloc::string::String,
    /// Configurations for all execution that relates to this `Target`.
    /// Each `ExecutionEnvironmentUsage` value may only be used in a single
    /// configuration; using the same value multiple times is an error.
    /// When one or more configurations are specified, they must include the
    /// `RENDER` and `DEPLOY` `ExecutionEnvironmentUsage` values.
    /// When no configurations are specified, execution will use the default
    /// specified in `DefaultPool`.
    #[prost(message, repeated, tag = "16")]
    pub execution_configs: ::prost::alloc::vec::Vec<ExecutionConfig>,
    /// Optional. The deploy parameters to use for this target.
    #[prost(btree_map = "string, string", tag = "20")]
    pub deploy_parameters: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Destination to which the Skaffold configuration is applied during a
    /// rollout.
    #[prost(oneof = "target::DeploymentTarget", tags = "15, 17, 18, 19, 21")]
    pub deployment_target: ::core::option::Option<target::DeploymentTarget>,
}
/// Nested message and enum types in `Target`.
pub mod target {
    /// Destination to which the Skaffold configuration is applied during a
    /// rollout.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DeploymentTarget {
        /// Optional. Information specifying a GKE Cluster.
        #[prost(message, tag = "15")]
        Gke(super::GkeCluster),
        /// Optional. Information specifying an Anthos Cluster.
        #[prost(message, tag = "17")]
        AnthosCluster(super::AnthosCluster),
        /// Optional. Information specifying a Cloud Run deployment target.
        #[prost(message, tag = "18")]
        Run(super::CloudRunLocation),
        /// Optional. Information specifying a multiTarget.
        #[prost(message, tag = "19")]
        MultiTarget(super::MultiTarget),
        /// Optional. Information specifying a Custom Target.
        #[prost(message, tag = "21")]
        CustomTarget(super::CustomTarget),
    }
}
/// Configuration of the environment to use when calling Skaffold.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionConfig {
    /// Required. Usages when this configuration should be applied.
    #[prost(
        enumeration = "execution_config::ExecutionEnvironmentUsage",
        repeated,
        packed = "false",
        tag = "1"
    )]
    pub usages: ::prost::alloc::vec::Vec<i32>,
    /// Optional. The resource name of the `WorkerPool`, with the format
    /// `projects/{project}/locations/{location}/workerPools/{worker_pool}`.
    /// If this optional field is unspecified, the default Cloud Build pool will be
    /// used.
    #[prost(string, tag = "4")]
    pub worker_pool: ::prost::alloc::string::String,
    /// Optional. Google service account to use for execution. If unspecified,
    /// the project execution service account
    /// (<PROJECT_NUMBER>-compute@developer.gserviceaccount.com) is used.
    #[prost(string, tag = "5")]
    pub service_account: ::prost::alloc::string::String,
    /// Optional. Cloud Storage location in which to store execution outputs. This
    /// can either be a bucket ("gs://my-bucket") or a path within a bucket
    /// ("gs://my-bucket/my-dir").
    /// If unspecified, a default bucket located in the same region will be used.
    #[prost(string, tag = "6")]
    pub artifact_storage: ::prost::alloc::string::String,
    /// Optional. Execution timeout for a Cloud Build Execution. This must be
    /// between 10m and 24h in seconds format. If unspecified, a default timeout of
    /// 1h is used.
    #[prost(message, optional, tag = "7")]
    pub execution_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Details of the environment.
    #[prost(oneof = "execution_config::ExecutionEnvironment", tags = "2, 3")]
    pub execution_environment: ::core::option::Option<
        execution_config::ExecutionEnvironment,
    >,
}
/// Nested message and enum types in `ExecutionConfig`.
pub mod execution_config {
    /// Possible usages of this configuration.
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
    pub enum ExecutionEnvironmentUsage {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Use for rendering.
        Render = 1,
        /// Use for deploying and deployment hooks.
        Deploy = 2,
        /// Use for deployment verification.
        Verify = 3,
        /// Use for predeploy job execution.
        Predeploy = 4,
        /// Use for postdeploy job execution.
        Postdeploy = 5,
    }
    impl ExecutionEnvironmentUsage {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExecutionEnvironmentUsage::Unspecified => {
                    "EXECUTION_ENVIRONMENT_USAGE_UNSPECIFIED"
                }
                ExecutionEnvironmentUsage::Render => "RENDER",
                ExecutionEnvironmentUsage::Deploy => "DEPLOY",
                ExecutionEnvironmentUsage::Verify => "VERIFY",
                ExecutionEnvironmentUsage::Predeploy => "PREDEPLOY",
                ExecutionEnvironmentUsage::Postdeploy => "POSTDEPLOY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EXECUTION_ENVIRONMENT_USAGE_UNSPECIFIED" => Some(Self::Unspecified),
                "RENDER" => Some(Self::Render),
                "DEPLOY" => Some(Self::Deploy),
                "VERIFY" => Some(Self::Verify),
                "PREDEPLOY" => Some(Self::Predeploy),
                "POSTDEPLOY" => Some(Self::Postdeploy),
                _ => None,
            }
        }
    }
    /// Details of the environment.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ExecutionEnvironment {
        /// Optional. Use default Cloud Build pool.
        #[prost(message, tag = "2")]
        DefaultPool(super::DefaultPool),
        /// Optional. Use private Cloud Build pool.
        #[prost(message, tag = "3")]
        PrivatePool(super::PrivatePool),
    }
}
/// Execution using the default Cloud Build pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefaultPool {
    /// Optional. Google service account to use for execution. If unspecified,
    /// the project execution service account
    /// (<PROJECT_NUMBER>-compute@developer.gserviceaccount.com) will be used.
    #[prost(string, tag = "1")]
    pub service_account: ::prost::alloc::string::String,
    /// Optional. Cloud Storage location where execution outputs should be stored.
    /// This can either be a bucket ("gs://my-bucket") or a path within a bucket
    /// ("gs://my-bucket/my-dir").
    /// If unspecified, a default bucket located in the same region will be used.
    #[prost(string, tag = "2")]
    pub artifact_storage: ::prost::alloc::string::String,
}
/// Execution using a private Cloud Build pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivatePool {
    /// Required. Resource name of the Cloud Build worker pool to use. The format
    /// is `projects/{project}/locations/{location}/workerPools/{pool}`.
    #[prost(string, tag = "1")]
    pub worker_pool: ::prost::alloc::string::String,
    /// Optional. Google service account to use for execution. If unspecified,
    /// the project execution service account
    /// (<PROJECT_NUMBER>-compute@developer.gserviceaccount.com) will be used.
    #[prost(string, tag = "2")]
    pub service_account: ::prost::alloc::string::String,
    /// Optional. Cloud Storage location where execution outputs should be stored.
    /// This can either be a bucket ("gs://my-bucket") or a path within a bucket
    /// ("gs://my-bucket/my-dir").
    /// If unspecified, a default bucket located in the same region will be used.
    #[prost(string, tag = "3")]
    pub artifact_storage: ::prost::alloc::string::String,
}
/// Information specifying a GKE Cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GkeCluster {
    /// Information specifying a GKE Cluster. Format is
    /// `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}`.
    #[prost(string, tag = "1")]
    pub cluster: ::prost::alloc::string::String,
    /// Optional. If true, `cluster` is accessed using the private IP address of
    /// the control plane endpoint. Otherwise, the default IP address of the
    /// control plane endpoint is used. The default IP address is the private IP
    /// address for clusters with private control-plane endpoints and the public IP
    /// address otherwise.
    ///
    /// Only specify this option when `cluster` is a [private GKE
    /// cluster](<https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept>).
    #[prost(bool, tag = "2")]
    pub internal_ip: bool,
}
/// Information specifying an Anthos Cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnthosCluster {
    /// Membership of the GKE Hub-registered cluster to which to apply the Skaffold
    /// configuration. Format is
    /// `projects/{project}/locations/{location}/memberships/{membership_name}`.
    #[prost(string, tag = "1")]
    pub membership: ::prost::alloc::string::String,
}
/// Information specifying where to deploy a Cloud Run Service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudRunLocation {
    /// Required. The location for the Cloud Run Service. Format must be
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
}
/// Information specifying a multiTarget.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiTarget {
    /// Required. The target_ids of this multiTarget.
    #[prost(string, repeated, tag = "1")]
    pub target_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Information specifying a Custom Target.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomTarget {
    /// Required. The name of the CustomTargetType. Format must be
    /// `projects/{project}/locations/{location}/customTargetTypes/{custom_target_type}`.
    #[prost(string, tag = "1")]
    pub custom_target_type: ::prost::alloc::string::String,
}
/// The request object for `ListTargets`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTargetsRequest {
    /// Required. The parent, which owns this collection of targets. Format must be
    /// `projects/{project_id}/locations/{location_name}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `Target` objects to return. The service may
    /// return fewer than this value. If unspecified, at most 50 `Target` objects
    /// will be returned. The maximum value is 1000; values above 1000 will be set
    /// to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListTargets` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other provided parameters match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter targets to be returned. See <https://google.aip.dev/160> for
    /// more details.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Field to sort by. See <https://google.aip.dev/132#ordering> for
    /// more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response object from `ListTargets`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTargetsResponse {
    /// The `Target` objects.
    #[prost(message, repeated, tag = "1")]
    pub targets: ::prost::alloc::vec::Vec<Target>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request object for `GetTarget`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTargetRequest {
    /// Required. Name of the `Target`. Format must be
    /// `projects/{project_id}/locations/{location_name}/targets/{target_name}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request object for `CreateTarget`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTargetRequest {
    /// Required. The parent collection in which the `Target` should be created.
    /// Format should be
    /// `projects/{project_id}/locations/{location_name}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the `Target`.
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    /// Required. The `Target` to create.
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
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
    /// Optional. If set to true, the request is validated and the user is provided
    /// with an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// The request object for `UpdateTarget`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTargetRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Target resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it's in the mask. If the
    /// user doesn't provide a mask then all fields are overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The `Target` to update.
    #[prost(message, optional, tag = "2")]
    pub target: ::core::option::Option<Target>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
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
    /// Optional. If set to true, updating a `Target` that does not exist will
    /// result in the creation of a new `Target`.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
    /// Optional. If set to true, the request is validated and the user is provided
    /// with an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// The request object for `DeleteTarget`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTargetRequest {
    /// Required. The name of the `Target` to delete. Format should be
    /// `projects/{project_id}/locations/{location_name}/targets/{target_name}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, then deleting an already deleted or non-existing
    /// `Target` will succeed.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// Optional. If set, validate the request and preview the review, but do not
    /// actually post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// Optional. This checksum is computed by the server based on the value of
    /// other fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "5")]
    pub etag: ::prost::alloc::string::String,
}
/// A `CustomTargetType` resource in the Cloud Deploy API.
///
/// A `CustomTargetType` defines a type of custom target that can be referenced
/// in a `Target` in order to facilitate deploying to other systems besides the
/// supported runtimes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomTargetType {
    /// Optional. Name of the `CustomTargetType`. Format is
    /// `projects/{project}/locations/{location}/customTargetTypes/[a-z][a-z0-9\-]{0,62}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Resource id of the `CustomTargetType`.
    #[prost(string, tag = "2")]
    pub custom_target_type_id: ::prost::alloc::string::String,
    /// Output only. Unique identifier of the `CustomTargetType`.
    #[prost(string, tag = "3")]
    pub uid: ::prost::alloc::string::String,
    /// Optional. Description of the `CustomTargetType`. Max length is 255
    /// characters.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Optional. User annotations. These attributes can only be set and used by
    /// the user, and not by Cloud Deploy. See
    /// <https://google.aip.dev/128#annotations> for more details such as format and
    /// size limitations.
    #[prost(btree_map = "string, string", tag = "5")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. Labels are attributes that can be set and used by both the
    /// user and by Cloud Deploy. Labels must meet the following constraints:
    ///
    /// * Keys and values can contain only lowercase letters, numeric characters,
    /// underscores, and dashes.
    /// * All characters must use UTF-8 encoding, and international characters are
    /// allowed.
    /// * Keys must start with a lowercase letter or international character.
    /// * Each resource is limited to a maximum of 64 labels.
    ///
    /// Both keys and values are additionally constrained to be <= 128 bytes.
    #[prost(btree_map = "string, string", tag = "6")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Time at which the `CustomTargetType` was created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Most recent time at which the `CustomTargetType` was updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. This checksum is computed by the server based on the value of
    /// other fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "9")]
    pub etag: ::prost::alloc::string::String,
    /// Defines the `CustomTargetType` renderer and deployer.
    #[prost(oneof = "custom_target_type::Definition", tags = "10")]
    pub definition: ::core::option::Option<custom_target_type::Definition>,
}
/// Nested message and enum types in `CustomTargetType`.
pub mod custom_target_type {
    /// Defines the `CustomTargetType` renderer and deployer.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Definition {
        /// Configures render and deploy for the `CustomTargetType` using Skaffold
        /// custom actions.
        #[prost(message, tag = "10")]
        CustomActions(super::CustomTargetSkaffoldActions),
    }
}
/// CustomTargetSkaffoldActions represents the `CustomTargetType` configuration
/// using Skaffold custom actions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomTargetSkaffoldActions {
    /// Optional. The Skaffold custom action responsible for render operations. If
    /// not provided then Cloud Deploy will perform the render operations via
    /// `skaffold render`.
    #[prost(string, tag = "1")]
    pub render_action: ::prost::alloc::string::String,
    /// Required. The Skaffold custom action responsible for deploy operations.
    #[prost(string, tag = "2")]
    pub deploy_action: ::prost::alloc::string::String,
    /// Optional. List of Skaffold modules Cloud Deploy will include in the
    /// Skaffold Config as required before performing diagnose.
    #[prost(message, repeated, tag = "3")]
    pub include_skaffold_modules: ::prost::alloc::vec::Vec<SkaffoldModules>,
}
/// Skaffold Config modules and their remote source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkaffoldModules {
    /// Optional. The Skaffold Config modules to use from the specified source.
    #[prost(string, repeated, tag = "1")]
    pub configs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The source that contains the Skaffold Config modules.
    #[prost(oneof = "skaffold_modules::Source", tags = "2, 3")]
    pub source: ::core::option::Option<skaffold_modules::Source>,
}
/// Nested message and enum types in `SkaffoldModules`.
pub mod skaffold_modules {
    /// Git repository containing Skaffold Config modules.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SkaffoldGitSource {
        /// Required. Git repository the package should be cloned from.
        #[prost(string, tag = "1")]
        pub repo: ::prost::alloc::string::String,
        /// Optional. Relative path from the repository root to the Skaffold file.
        #[prost(string, tag = "2")]
        pub path: ::prost::alloc::string::String,
        /// Optional. Git ref the package should be cloned from.
        #[prost(string, tag = "3")]
        pub r#ref: ::prost::alloc::string::String,
    }
    /// Cloud Storage bucket containing Skaffold Config modules.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SkaffoldGcsSource {
        /// Required. Cloud Storage source paths to copy recursively. For example,
        /// providing "gs://my-bucket/dir/configs/*" will result in Skaffold copying
        /// all files within the "dir/configs" directory in the bucket "my-bucket".
        #[prost(string, tag = "1")]
        pub source: ::prost::alloc::string::String,
        /// Optional. Relative path from the source to the Skaffold file.
        #[prost(string, tag = "2")]
        pub path: ::prost::alloc::string::String,
    }
    /// The source that contains the Skaffold Config modules.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Remote git repository containing the Skaffold Config modules.
        #[prost(message, tag = "2")]
        Git(SkaffoldGitSource),
        /// Cloud Storage bucket containing the Skaffold Config modules.
        #[prost(message, tag = "3")]
        GoogleCloudStorage(SkaffoldGcsSource),
    }
}
/// The request object for `ListCustomTargetTypes`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomTargetTypesRequest {
    /// Required. The parent that owns this collection of custom target types.
    /// Format must be `projects/{project_id}/locations/{location_name}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `CustomTargetType` objects to return. The
    /// service may return fewer than this value. If unspecified, at most 50
    /// `CustomTargetType` objects will be returned. The maximum value is 1000;
    /// values above 1000 will be set to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListCustomTargetTypes`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other provided parameters match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter custom target types to be returned. See
    /// <https://google.aip.dev/160> for more details.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Field to sort by. See <https://google.aip.dev/132#ordering> for
    /// more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response object from `ListCustomTargetTypes.`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomTargetTypesResponse {
    /// The `CustomTargetType` objects.
    #[prost(message, repeated, tag = "1")]
    pub custom_target_types: ::prost::alloc::vec::Vec<CustomTargetType>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request object for `GetCustomTargetType`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomTargetTypeRequest {
    /// Required. Name of the `CustomTargetType`. Format must be
    /// `projects/{project_id}/locations/{location_name}/customTargetTypes/{custom_target_type}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request object for `CreateCustomTargetType`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomTargetTypeRequest {
    /// Required. The parent collection in which the `CustomTargetType` should be
    /// created. Format should be
    /// `projects/{project_id}/locations/{location_name}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the `CustomTargetType`.
    #[prost(string, tag = "2")]
    pub custom_target_type_id: ::prost::alloc::string::String,
    /// Required. The `CustomTargetType` to create.
    #[prost(message, optional, tag = "3")]
    pub custom_target_type: ::core::option::Option<CustomTargetType>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
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
    /// Optional. If set to true, the request is validated and the user is provided
    /// with an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// The request object for `UpdateCustomTargetType`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomTargetTypeRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `CustomTargetType` resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it's in the mask. If the
    /// user doesn't provide a mask then all fields are overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The `CustomTargetType` to update.
    #[prost(message, optional, tag = "2")]
    pub custom_target_type: ::core::option::Option<CustomTargetType>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
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
    /// Optional. If set to true, updating a `CustomTargetType` that does not exist
    /// will result in the creation of a new `CustomTargetType`.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
    /// Optional. If set to true, the request is validated and the user is provided
    /// with an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// The request object for `DeleteCustomTargetType`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCustomTargetTypeRequest {
    /// Required. The name of the `CustomTargetType` to delete. Format must be
    /// `projects/{project_id}/locations/{location_name}/customTargetTypes/{custom_target_type}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, then deleting an already deleted or non-existing
    /// `CustomTargetType` will succeed.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// Optional. If set to true, the request is validated but no actual change is
    /// made.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// Optional. This checksum is computed by the server based on the value of
    /// other fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "5")]
    pub etag: ::prost::alloc::string::String,
}
/// Contains criteria for selecting Targets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetAttribute {
    /// ID of the `Target`. The value of this field could be one of the
    /// following:
    /// * The last segment of a target name. It only needs the ID to determine
    /// which target is being referred to
    /// * "*", all targets in a location.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Target labels.
    #[prost(btree_map = "string, string", tag = "2")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// A `Release` resource in the Cloud Deploy API.
///
/// A `Release` defines a specific Skaffold configuration instance
/// that can be deployed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Release {
    /// Optional. Name of the `Release`. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/[a-z][a-z0-9\-]{0,62}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Unique identifier of the `Release`.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Description of the `Release`. Max length is 255 characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// User annotations. These attributes can only be set and used by the
    /// user, and not by Cloud Deploy. See <https://google.aip.dev/128#annotations>
    /// for more details such as format and size limitations.
    #[prost(btree_map = "string, string", tag = "4")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Labels are attributes that can be set and used by both the
    /// user and by Cloud Deploy. Labels must meet the following constraints:
    ///
    /// * Keys and values can contain only lowercase letters, numeric characters,
    /// underscores, and dashes.
    /// * All characters must use UTF-8 encoding, and international characters are
    /// allowed.
    /// * Keys must start with a lowercase letter or international character.
    /// * Each resource is limited to a maximum of 64 labels.
    ///
    /// Both keys and values are additionally constrained to be <= 128 bytes.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Indicates whether this is an abandoned release.
    #[prost(bool, tag = "23")]
    pub abandoned: bool,
    /// Output only. Time at which the `Release` was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the render began.
    #[prost(message, optional, tag = "7")]
    pub render_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the render completed.
    #[prost(message, optional, tag = "8")]
    pub render_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Cloud Storage URI of tar.gz archive containing Skaffold configuration.
    #[prost(string, tag = "17")]
    pub skaffold_config_uri: ::prost::alloc::string::String,
    /// Filepath of the Skaffold config inside of the config URI.
    #[prost(string, tag = "9")]
    pub skaffold_config_path: ::prost::alloc::string::String,
    /// List of artifacts to pass through to Skaffold command.
    #[prost(message, repeated, tag = "10")]
    pub build_artifacts: ::prost::alloc::vec::Vec<BuildArtifact>,
    /// Output only. Snapshot of the parent pipeline taken at release creation
    /// time.
    #[prost(message, optional, tag = "11")]
    pub delivery_pipeline_snapshot: ::core::option::Option<DeliveryPipeline>,
    /// Output only. Snapshot of the targets taken at release creation time.
    #[prost(message, repeated, tag = "12")]
    pub target_snapshots: ::prost::alloc::vec::Vec<Target>,
    /// Output only. Snapshot of the custom target types referenced by the targets
    /// taken at release creation time.
    #[prost(message, repeated, tag = "27")]
    pub custom_target_type_snapshots: ::prost::alloc::vec::Vec<CustomTargetType>,
    /// Output only. Current state of the render operation.
    #[prost(enumeration = "release::RenderState", tag = "13")]
    pub render_state: i32,
    /// This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "16")]
    pub etag: ::prost::alloc::string::String,
    /// The Skaffold version to use when operating on this release, such as
    /// "1.20.0". Not all versions are valid; Cloud Deploy supports a specific set
    /// of versions.
    ///
    /// If unset, the most recent supported Skaffold version will be used.
    #[prost(string, tag = "19")]
    pub skaffold_version: ::prost::alloc::string::String,
    /// Output only. Map from target ID to the target artifacts created
    /// during the render operation.
    #[prost(btree_map = "string, message", tag = "20")]
    pub target_artifacts: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        TargetArtifact,
    >,
    /// Output only. Map from target ID to details of the render operation for that
    /// target.
    #[prost(btree_map = "string, message", tag = "22")]
    pub target_renders: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        release::TargetRender,
    >,
    /// Output only. Information around the state of the Release.
    #[prost(message, optional, tag = "24")]
    pub condition: ::core::option::Option<release::ReleaseCondition>,
    /// Optional. The deploy parameters to use for all targets in this release.
    #[prost(btree_map = "string, string", tag = "25")]
    pub deploy_parameters: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `Release`.
pub mod release {
    /// Details of rendering for a single target.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetRender {
        /// Output only. The resource name of the Cloud Build `Build` object that is
        /// used to render the manifest for this target. Format is
        /// `projects/{project}/locations/{location}/builds/{build}`.
        #[prost(string, tag = "1")]
        pub rendering_build: ::prost::alloc::string::String,
        /// Output only. Current state of the render operation for this Target.
        #[prost(enumeration = "target_render::TargetRenderState", tag = "2")]
        pub rendering_state: i32,
        /// Output only. Metadata related to the `Release` render for this Target.
        #[prost(message, optional, tag = "6")]
        pub metadata: ::core::option::Option<super::RenderMetadata>,
        /// Output only. Reason this render failed. This will always be unspecified
        /// while the render in progress.
        #[prost(enumeration = "target_render::FailureCause", tag = "4")]
        pub failure_cause: i32,
        /// Output only. Additional information about the render failure, if
        /// available.
        #[prost(string, tag = "5")]
        pub failure_message: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `TargetRender`.
    pub mod target_render {
        /// Valid states of the render operation.
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
        pub enum TargetRenderState {
            /// The render operation state is unspecified.
            Unspecified = 0,
            /// The render operation has completed successfully.
            Succeeded = 1,
            /// The render operation has failed.
            Failed = 2,
            /// The render operation is in progress.
            InProgress = 3,
        }
        impl TargetRenderState {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    TargetRenderState::Unspecified => "TARGET_RENDER_STATE_UNSPECIFIED",
                    TargetRenderState::Succeeded => "SUCCEEDED",
                    TargetRenderState::Failed => "FAILED",
                    TargetRenderState::InProgress => "IN_PROGRESS",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TARGET_RENDER_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                    "SUCCEEDED" => Some(Self::Succeeded),
                    "FAILED" => Some(Self::Failed),
                    "IN_PROGRESS" => Some(Self::InProgress),
                    _ => None,
                }
            }
        }
        /// Well-known rendering failures.
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
        pub enum FailureCause {
            /// No reason for failure is specified.
            Unspecified = 0,
            /// Cloud Build is not available, either because it is not enabled or
            /// because Cloud Deploy has insufficient permissions. See [required
            /// permission](<https://cloud.google.com/deploy/docs/cloud-deploy-service-account#required_permissions>).
            CloudBuildUnavailable = 1,
            /// The render operation did not complete successfully; check Cloud Build
            /// logs.
            ExecutionFailed = 2,
            /// Cloud Build failed to fulfill Cloud Deploy's request. See
            /// failure_message for additional details.
            CloudBuildRequestFailed = 3,
            /// The render operation did not complete successfully because the
            /// verification stanza required for verify was not found on the Skaffold
            /// configuration.
            VerificationConfigNotFound = 4,
            /// The render operation did not complete successfully because the custom
            /// action required for predeploy or postdeploy was not found in the
            /// Skaffold configuration. See failure_message for additional details.
            CustomActionNotFound = 5,
            /// Release failed during rendering because the release configuration is
            /// not supported with the specified deployment strategy.
            DeploymentStrategyNotSupported = 6,
            /// The render operation had a feature configured that is not supported.
            RenderFeatureNotSupported = 7,
        }
        impl FailureCause {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    FailureCause::Unspecified => "FAILURE_CAUSE_UNSPECIFIED",
                    FailureCause::CloudBuildUnavailable => "CLOUD_BUILD_UNAVAILABLE",
                    FailureCause::ExecutionFailed => "EXECUTION_FAILED",
                    FailureCause::CloudBuildRequestFailed => "CLOUD_BUILD_REQUEST_FAILED",
                    FailureCause::VerificationConfigNotFound => {
                        "VERIFICATION_CONFIG_NOT_FOUND"
                    }
                    FailureCause::CustomActionNotFound => "CUSTOM_ACTION_NOT_FOUND",
                    FailureCause::DeploymentStrategyNotSupported => {
                        "DEPLOYMENT_STRATEGY_NOT_SUPPORTED"
                    }
                    FailureCause::RenderFeatureNotSupported => {
                        "RENDER_FEATURE_NOT_SUPPORTED"
                    }
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "FAILURE_CAUSE_UNSPECIFIED" => Some(Self::Unspecified),
                    "CLOUD_BUILD_UNAVAILABLE" => Some(Self::CloudBuildUnavailable),
                    "EXECUTION_FAILED" => Some(Self::ExecutionFailed),
                    "CLOUD_BUILD_REQUEST_FAILED" => Some(Self::CloudBuildRequestFailed),
                    "VERIFICATION_CONFIG_NOT_FOUND" => {
                        Some(Self::VerificationConfigNotFound)
                    }
                    "CUSTOM_ACTION_NOT_FOUND" => Some(Self::CustomActionNotFound),
                    "DEPLOYMENT_STRATEGY_NOT_SUPPORTED" => {
                        Some(Self::DeploymentStrategyNotSupported)
                    }
                    "RENDER_FEATURE_NOT_SUPPORTED" => {
                        Some(Self::RenderFeatureNotSupported)
                    }
                    _ => None,
                }
            }
        }
    }
    /// ReleaseReadyCondition contains information around the status of the
    /// Release. If a release is not ready, you cannot create a rollout with the
    /// release.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReleaseReadyCondition {
        /// True if the Release is in a valid state. Otherwise at least one condition
        /// in `ReleaseCondition` is in an invalid state. Iterate over those
        /// conditions and see which condition(s) has status = false to find out what
        /// is wrong with the Release.
        #[prost(bool, tag = "1")]
        pub status: bool,
    }
    /// SkaffoldSupportedCondition contains information about when support for the
    /// release's version of Skaffold ends.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SkaffoldSupportedCondition {
        /// True if the version of Skaffold used by this release is supported.
        #[prost(bool, tag = "1")]
        pub status: bool,
        /// The Skaffold support state for this release's version of Skaffold.
        #[prost(enumeration = "super::SkaffoldSupportState", tag = "2")]
        pub skaffold_support_state: i32,
        /// The time at which this release's version of Skaffold will enter
        /// maintenance mode.
        #[prost(message, optional, tag = "3")]
        pub maintenance_mode_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The time at which this release's version of Skaffold will no longer be
        /// supported.
        #[prost(message, optional, tag = "4")]
        pub support_expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// ReleaseCondition contains all conditions relevant to a Release.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReleaseCondition {
        /// Details around the Releases's overall status.
        #[prost(message, optional, tag = "1")]
        pub release_ready_condition: ::core::option::Option<ReleaseReadyCondition>,
        /// Details around the support state of the release's Skaffold
        /// version.
        #[prost(message, optional, tag = "2")]
        pub skaffold_supported_condition: ::core::option::Option<
            SkaffoldSupportedCondition,
        >,
    }
    /// Valid states of the render operation.
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
    pub enum RenderState {
        /// The render state is unspecified.
        Unspecified = 0,
        /// All rendering operations have completed successfully.
        Succeeded = 1,
        /// All rendering operations have completed, and one or more have failed.
        Failed = 2,
        /// Rendering has started and is not complete.
        InProgress = 3,
    }
    impl RenderState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RenderState::Unspecified => "RENDER_STATE_UNSPECIFIED",
                RenderState::Succeeded => "SUCCEEDED",
                RenderState::Failed => "FAILED",
                RenderState::InProgress => "IN_PROGRESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RENDER_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "IN_PROGRESS" => Some(Self::InProgress),
                _ => None,
            }
        }
    }
}
/// Description of an a image to use during Skaffold rendering.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildArtifact {
    /// Image name in Skaffold configuration.
    #[prost(string, tag = "3")]
    pub image: ::prost::alloc::string::String,
    /// Image tag to use. This will generally be the full path to an image, such
    /// as "gcr.io/my-project/busybox:1.2.3" or
    /// "gcr.io/my-project/busybox@sha256:abc123".
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
}
/// The artifacts produced by a target render operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetArtifact {
    /// Output only. File path of the resolved Skaffold configuration relative to
    /// the URI.
    #[prost(string, tag = "2")]
    pub skaffold_config_path: ::prost::alloc::string::String,
    /// Output only. File path of the rendered manifest relative to the URI.
    #[prost(string, tag = "3")]
    pub manifest_path: ::prost::alloc::string::String,
    /// Output only. Map from the phase ID to the phase artifacts for the `Target`.
    #[prost(btree_map = "string, message", tag = "5")]
    pub phase_artifacts: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        target_artifact::PhaseArtifact,
    >,
    #[prost(oneof = "target_artifact::Uri", tags = "4")]
    pub uri: ::core::option::Option<target_artifact::Uri>,
}
/// Nested message and enum types in `TargetArtifact`.
pub mod target_artifact {
    /// Contains the paths to the artifacts, relative to the URI, for a phase.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PhaseArtifact {
        /// Output only. File path of the resolved Skaffold configuration relative to
        /// the URI.
        #[prost(string, tag = "1")]
        pub skaffold_config_path: ::prost::alloc::string::String,
        /// Output only. File path of the rendered manifest relative to the URI.
        #[prost(string, tag = "3")]
        pub manifest_path: ::prost::alloc::string::String,
        /// Output only. File path of the directory of rendered job manifests
        /// relative to the URI. This is only set if it is applicable.
        #[prost(string, tag = "4")]
        pub job_manifests_path: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Uri {
        /// Output only. URI of a directory containing the artifacts. This contains
        /// deployment configuration used by Skaffold during a rollout, and all
        /// paths are relative to this location.
        #[prost(string, tag = "4")]
        ArtifactUri(::prost::alloc::string::String),
    }
}
/// The artifacts produced by a deploy operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployArtifact {
    /// Output only. URI of a directory containing the artifacts. All paths are
    /// relative to this location.
    #[prost(string, tag = "1")]
    pub artifact_uri: ::prost::alloc::string::String,
    /// Output only. File paths of the manifests applied during the deploy
    /// operation relative to the URI.
    #[prost(string, repeated, tag = "2")]
    pub manifest_paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CloudRunRenderMetadata contains Cloud Run information associated with a
/// `Release` render.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudRunRenderMetadata {
    /// Output only. The name of the Cloud Run Service in the rendered manifest.
    /// Format is `projects/{project}/locations/{location}/services/{service}`.
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
}
/// RenderMetadata includes information associated with a `Release` render.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenderMetadata {
    /// Output only. Metadata associated with rendering for Cloud Run.
    #[prost(message, optional, tag = "1")]
    pub cloud_run: ::core::option::Option<CloudRunRenderMetadata>,
    /// Output only. Custom metadata provided by user-defined render operation.
    #[prost(message, optional, tag = "2")]
    pub custom: ::core::option::Option<CustomMetadata>,
}
/// The request object for `ListReleases`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReleasesRequest {
    /// Required. The `DeliveryPipeline` which owns this collection of `Release`
    /// objects.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `Release` objects to return. The service
    /// may return fewer than this value. If unspecified, at most 50 `Release`
    /// objects will be returned. The maximum value is 1000; values above 1000 will
    /// be set to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListReleases` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other provided parameters match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter releases to be returned. See <https://google.aip.dev/160>
    /// for more details.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Field to sort by. See <https://google.aip.dev/132#ordering> for
    /// more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response object from `ListReleases`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReleasesResponse {
    /// The `Release` objects.
    #[prost(message, repeated, tag = "1")]
    pub releases: ::prost::alloc::vec::Vec<Release>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request object for `GetRelease`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReleaseRequest {
    /// Required. Name of the `Release`. Format must be
    /// `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/releases/{release_name}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request object for `CreateRelease`,
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReleaseRequest {
    /// Required. The parent collection in which the `Release` should be created.
    /// Format should be
    /// `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the `Release`.
    #[prost(string, tag = "2")]
    pub release_id: ::prost::alloc::string::String,
    /// Required. The `Release` to create.
    #[prost(message, optional, tag = "3")]
    pub release: ::core::option::Option<Release>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
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
    /// Optional. If set to true, the request is validated and the user is provided
    /// with an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// A `Rollout` resource in the Cloud Deploy API.
///
/// A `Rollout` contains information around a specific deployment to a `Target`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rollout {
    /// Optional. Name of the `Rollout`. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/[a-z][a-z0-9\-]{0,62}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Unique identifier of the `Rollout`.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Description of the `Rollout` for user purposes. Max length is 255
    /// characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// User annotations. These attributes can only be set and used by the
    /// user, and not by Cloud Deploy. See <https://google.aip.dev/128#annotations>
    /// for more details such as format and size limitations.
    #[prost(btree_map = "string, string", tag = "4")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Labels are attributes that can be set and used by both the
    /// user and by Cloud Deploy. Labels must meet the following constraints:
    ///
    /// * Keys and values can contain only lowercase letters, numeric characters,
    /// underscores, and dashes.
    /// * All characters must use UTF-8 encoding, and international characters are
    /// allowed.
    /// * Keys must start with a lowercase letter or international character.
    /// * Each resource is limited to a maximum of 64 labels.
    ///
    /// Both keys and values are additionally constrained to be <= 128 bytes.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Time at which the `Rollout` was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the `Rollout` was approved.
    #[prost(message, optional, tag = "7")]
    pub approve_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the `Rollout` was enqueued.
    #[prost(message, optional, tag = "8")]
    pub enqueue_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the `Rollout` started deploying.
    #[prost(message, optional, tag = "9")]
    pub deploy_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the `Rollout` finished deploying.
    #[prost(message, optional, tag = "10")]
    pub deploy_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The ID of Target to which this `Rollout` is deploying.
    #[prost(string, tag = "18")]
    pub target_id: ::prost::alloc::string::String,
    /// Output only. Approval state of the `Rollout`.
    #[prost(enumeration = "rollout::ApprovalState", tag = "12")]
    pub approval_state: i32,
    /// Output only. Current state of the `Rollout`.
    #[prost(enumeration = "rollout::State", tag = "13")]
    pub state: i32,
    /// Output only. Additional information about the rollout failure, if
    /// available.
    #[prost(string, tag = "14")]
    pub failure_reason: ::prost::alloc::string::String,
    /// Output only. The resource name of the Cloud Build `Build` object that is
    /// used to deploy the Rollout. Format is
    /// `projects/{project}/locations/{location}/builds/{build}`.
    #[prost(string, tag = "17")]
    pub deploying_build: ::prost::alloc::string::String,
    /// This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "16")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. The reason this rollout failed. This will always be
    /// unspecified while the rollout is in progress.
    #[prost(enumeration = "rollout::FailureCause", tag = "19")]
    pub deploy_failure_cause: i32,
    /// Output only. The phases that represent the workflows of this `Rollout`.
    #[prost(message, repeated, tag = "23")]
    pub phases: ::prost::alloc::vec::Vec<Phase>,
    /// Output only. Metadata contains information about the rollout.
    #[prost(message, optional, tag = "24")]
    pub metadata: ::core::option::Option<Metadata>,
    /// Output only. Name of the `ControllerRollout`. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/[a-z][a-z0-9\-]{0,62}`.
    #[prost(string, tag = "25")]
    pub controller_rollout: ::prost::alloc::string::String,
    /// Output only. Name of the `Rollout` that is rolled back by this `Rollout`.
    /// Empty if this `Rollout` wasn't created as a rollback.
    #[prost(string, tag = "26")]
    pub rollback_of_rollout: ::prost::alloc::string::String,
    /// Output only. Names of `Rollouts` that rolled back this `Rollout`.
    #[prost(string, repeated, tag = "27")]
    pub rolled_back_by_rollouts: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `Rollout`.
pub mod rollout {
    /// Valid approval states of a `Rollout`.
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
    pub enum ApprovalState {
        /// The `Rollout` has an unspecified approval state.
        Unspecified = 0,
        /// The `Rollout` requires approval.
        NeedsApproval = 1,
        /// The `Rollout` does not require approval.
        DoesNotNeedApproval = 2,
        /// The `Rollout` has been approved.
        Approved = 3,
        /// The `Rollout` has been rejected.
        Rejected = 4,
    }
    impl ApprovalState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ApprovalState::Unspecified => "APPROVAL_STATE_UNSPECIFIED",
                ApprovalState::NeedsApproval => "NEEDS_APPROVAL",
                ApprovalState::DoesNotNeedApproval => "DOES_NOT_NEED_APPROVAL",
                ApprovalState::Approved => "APPROVED",
                ApprovalState::Rejected => "REJECTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "APPROVAL_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "NEEDS_APPROVAL" => Some(Self::NeedsApproval),
                "DOES_NOT_NEED_APPROVAL" => Some(Self::DoesNotNeedApproval),
                "APPROVED" => Some(Self::Approved),
                "REJECTED" => Some(Self::Rejected),
                _ => None,
            }
        }
    }
    /// Valid states of a `Rollout`.
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
        /// The `Rollout` has an unspecified state.
        Unspecified = 0,
        /// The `Rollout` has completed successfully.
        Succeeded = 1,
        /// The `Rollout` has failed.
        Failed = 2,
        /// The `Rollout` is being deployed.
        InProgress = 3,
        /// The `Rollout` needs approval.
        PendingApproval = 4,
        /// An approver rejected the `Rollout`.
        ApprovalRejected = 5,
        /// The `Rollout` is waiting for an earlier Rollout(s) to complete on this
        /// `Target`.
        Pending = 6,
        /// The `Rollout` is waiting for the `Release` to be fully rendered.
        PendingRelease = 7,
        /// The `Rollout` is in the process of being cancelled.
        Cancelling = 8,
        /// The `Rollout` has been cancelled.
        Cancelled = 9,
        /// The `Rollout` is halted.
        Halted = 10,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::InProgress => "IN_PROGRESS",
                State::PendingApproval => "PENDING_APPROVAL",
                State::ApprovalRejected => "APPROVAL_REJECTED",
                State::Pending => "PENDING",
                State::PendingRelease => "PENDING_RELEASE",
                State::Cancelling => "CANCELLING",
                State::Cancelled => "CANCELLED",
                State::Halted => "HALTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "IN_PROGRESS" => Some(Self::InProgress),
                "PENDING_APPROVAL" => Some(Self::PendingApproval),
                "APPROVAL_REJECTED" => Some(Self::ApprovalRejected),
                "PENDING" => Some(Self::Pending),
                "PENDING_RELEASE" => Some(Self::PendingRelease),
                "CANCELLING" => Some(Self::Cancelling),
                "CANCELLED" => Some(Self::Cancelled),
                "HALTED" => Some(Self::Halted),
                _ => None,
            }
        }
    }
    /// Well-known rollout failures.
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
    pub enum FailureCause {
        /// No reason for failure is specified.
        Unspecified = 0,
        /// Cloud Build is not available, either because it is not enabled or because
        /// Cloud Deploy has insufficient permissions. See [required
        /// permission](<https://cloud.google.com/deploy/docs/cloud-deploy-service-account#required_permissions>).
        CloudBuildUnavailable = 1,
        /// The deploy operation did not complete successfully; check Cloud Build
        /// logs.
        ExecutionFailed = 2,
        /// Deployment did not complete within the alloted time.
        DeadlineExceeded = 3,
        /// Release is in a failed state.
        ReleaseFailed = 4,
        /// Release is abandoned.
        ReleaseAbandoned = 5,
        /// No Skaffold verify configuration was found.
        VerificationConfigNotFound = 6,
        /// Cloud Build failed to fulfill Cloud Deploy's request. See failure_message
        /// for additional details.
        CloudBuildRequestFailed = 7,
        /// A Rollout operation had a feature configured that is not supported.
        OperationFeatureNotSupported = 8,
    }
    impl FailureCause {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FailureCause::Unspecified => "FAILURE_CAUSE_UNSPECIFIED",
                FailureCause::CloudBuildUnavailable => "CLOUD_BUILD_UNAVAILABLE",
                FailureCause::ExecutionFailed => "EXECUTION_FAILED",
                FailureCause::DeadlineExceeded => "DEADLINE_EXCEEDED",
                FailureCause::ReleaseFailed => "RELEASE_FAILED",
                FailureCause::ReleaseAbandoned => "RELEASE_ABANDONED",
                FailureCause::VerificationConfigNotFound => {
                    "VERIFICATION_CONFIG_NOT_FOUND"
                }
                FailureCause::CloudBuildRequestFailed => "CLOUD_BUILD_REQUEST_FAILED",
                FailureCause::OperationFeatureNotSupported => {
                    "OPERATION_FEATURE_NOT_SUPPORTED"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FAILURE_CAUSE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOUD_BUILD_UNAVAILABLE" => Some(Self::CloudBuildUnavailable),
                "EXECUTION_FAILED" => Some(Self::ExecutionFailed),
                "DEADLINE_EXCEEDED" => Some(Self::DeadlineExceeded),
                "RELEASE_FAILED" => Some(Self::ReleaseFailed),
                "RELEASE_ABANDONED" => Some(Self::ReleaseAbandoned),
                "VERIFICATION_CONFIG_NOT_FOUND" => Some(Self::VerificationConfigNotFound),
                "CLOUD_BUILD_REQUEST_FAILED" => Some(Self::CloudBuildRequestFailed),
                "OPERATION_FEATURE_NOT_SUPPORTED" => {
                    Some(Self::OperationFeatureNotSupported)
                }
                _ => None,
            }
        }
    }
}
/// Metadata includes information associated with a `Rollout`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// Output only. The name of the Cloud Run Service that is associated with a
    /// `Rollout`.
    #[prost(message, optional, tag = "1")]
    pub cloud_run: ::core::option::Option<CloudRunMetadata>,
    /// Output only. AutomationRolloutMetadata contains the information about the
    /// interactions between Automation service and this rollout.
    #[prost(message, optional, tag = "2")]
    pub automation: ::core::option::Option<AutomationRolloutMetadata>,
    /// Output only. Custom metadata provided by user-defined `Rollout` operations.
    #[prost(message, optional, tag = "3")]
    pub custom: ::core::option::Option<CustomMetadata>,
}
/// DeployJobRunMetadata surfaces information associated with a `DeployJobRun` to
/// the user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployJobRunMetadata {
    /// Output only. The name of the Cloud Run Service that is associated with a
    /// `DeployJobRun`.
    #[prost(message, optional, tag = "1")]
    pub cloud_run: ::core::option::Option<CloudRunMetadata>,
    /// Output only. Custom Target metadata associated with a `DeployJobRun`.
    #[prost(message, optional, tag = "2")]
    pub custom_target: ::core::option::Option<CustomTargetDeployMetadata>,
    /// Output only. Custom metadata provided by user-defined deploy operation.
    #[prost(message, optional, tag = "3")]
    pub custom: ::core::option::Option<CustomMetadata>,
}
/// CloudRunMetadata contains information from a Cloud Run deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudRunMetadata {
    /// Output only. The name of the Cloud Run Service that is associated with a
    /// `Rollout`. Format is
    /// `projects/{project}/locations/{location}/services/{service}`.
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    /// Output only. The Cloud Run Service urls that are associated with a
    /// `Rollout`.
    #[prost(string, repeated, tag = "2")]
    pub service_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The Cloud Run Revision id associated with a `Rollout`.
    #[prost(string, tag = "3")]
    pub revision: ::prost::alloc::string::String,
    /// Output only. The name of the Cloud Run job that is associated with a
    /// `Rollout`. Format is
    /// `projects/{project}/locations/{location}/jobs/{job_name}`.
    #[prost(string, tag = "4")]
    pub job: ::prost::alloc::string::String,
}
/// CustomTargetDeployMetadata contains information from a Custom Target
/// deploy operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomTargetDeployMetadata {
    /// Output only. Skip message provided in the results of a custom deploy
    /// operation.
    #[prost(string, tag = "1")]
    pub skip_message: ::prost::alloc::string::String,
}
/// AutomationRolloutMetadata contains Automation-related actions that
/// were performed on a rollout.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomationRolloutMetadata {
    /// Output only. The ID of the AutomationRun initiated by a promote release
    /// rule.
    #[prost(string, tag = "1")]
    pub promote_automation_run: ::prost::alloc::string::String,
    /// Output only. The IDs of the AutomationRuns initiated by an advance rollout
    /// rule.
    #[prost(string, repeated, tag = "2")]
    pub advance_automation_runs: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Output only. The IDs of the AutomationRuns initiated by a repair rollout
    /// rule.
    #[prost(string, repeated, tag = "3")]
    pub repair_automation_runs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CustomMetadata contains information from a user-defined operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomMetadata {
    /// Output only. Key-value pairs provided by the user-defined operation.
    #[prost(btree_map = "string, string", tag = "1")]
    pub values: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Phase represents a collection of jobs that are logically grouped together
/// for a `Rollout`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Phase {
    /// Output only. The ID of the Phase.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Output only. Current state of the Phase.
    #[prost(enumeration = "phase::State", tag = "3")]
    pub state: i32,
    /// Output only. Additional information on why the Phase was skipped, if
    /// available.
    #[prost(string, tag = "6")]
    pub skip_message: ::prost::alloc::string::String,
    /// The job composition of this Phase.
    #[prost(oneof = "phase::Jobs", tags = "4, 5")]
    pub jobs: ::core::option::Option<phase::Jobs>,
}
/// Nested message and enum types in `Phase`.
pub mod phase {
    /// Valid states of a Phase.
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
        /// The Phase has an unspecified state.
        Unspecified = 0,
        /// The Phase is waiting for an earlier Phase(s) to complete.
        Pending = 1,
        /// The Phase is in progress.
        InProgress = 2,
        /// The Phase has succeeded.
        Succeeded = 3,
        /// The Phase has failed.
        Failed = 4,
        /// The Phase was aborted.
        Aborted = 5,
        /// The Phase was skipped.
        Skipped = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Pending => "PENDING",
                State::InProgress => "IN_PROGRESS",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::Aborted => "ABORTED",
                State::Skipped => "SKIPPED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "IN_PROGRESS" => Some(Self::InProgress),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "ABORTED" => Some(Self::Aborted),
                "SKIPPED" => Some(Self::Skipped),
                _ => None,
            }
        }
    }
    /// The job composition of this Phase.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Jobs {
        /// Output only. Deployment job composition.
        #[prost(message, tag = "4")]
        DeploymentJobs(super::DeploymentJobs),
        /// Output only. ChildRollout job composition.
        #[prost(message, tag = "5")]
        ChildRolloutJobs(super::ChildRolloutJobs),
    }
}
/// Deployment job composition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentJobs {
    /// Output only. The deploy Job. This is the deploy job in the phase.
    #[prost(message, optional, tag = "1")]
    pub deploy_job: ::core::option::Option<Job>,
    /// Output only. The verify Job. Runs after a deploy if the deploy succeeds.
    #[prost(message, optional, tag = "2")]
    pub verify_job: ::core::option::Option<Job>,
    /// Output only. The predeploy Job, which is the first job on the phase.
    #[prost(message, optional, tag = "3")]
    pub predeploy_job: ::core::option::Option<Job>,
    /// Output only. The postdeploy Job, which is the last job on the phase.
    #[prost(message, optional, tag = "4")]
    pub postdeploy_job: ::core::option::Option<Job>,
}
/// ChildRollouts job composition
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChildRolloutJobs {
    /// Output only. List of CreateChildRolloutJobs
    #[prost(message, repeated, tag = "1")]
    pub create_rollout_jobs: ::prost::alloc::vec::Vec<Job>,
    /// Output only. List of AdvanceChildRolloutJobs
    #[prost(message, repeated, tag = "2")]
    pub advance_rollout_jobs: ::prost::alloc::vec::Vec<Job>,
}
/// Job represents an operation for a `Rollout`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// Output only. The ID of the Job.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Output only. The current state of the Job.
    #[prost(enumeration = "job::State", tag = "2")]
    pub state: i32,
    /// Output only. Additional information on why the Job was skipped, if
    /// available.
    #[prost(string, tag = "8")]
    pub skip_message: ::prost::alloc::string::String,
    /// Output only. The name of the `JobRun` responsible for the most recent
    /// invocation of this Job.
    #[prost(string, tag = "3")]
    pub job_run: ::prost::alloc::string::String,
    /// The type of Job.
    #[prost(oneof = "job::JobType", tags = "4, 5, 9, 10, 6, 7")]
    pub job_type: ::core::option::Option<job::JobType>,
}
/// Nested message and enum types in `Job`.
pub mod job {
    /// Valid states of a Job.
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
        /// The Job has an unspecified state.
        Unspecified = 0,
        /// The Job is waiting for an earlier Phase(s) or Job(s) to complete.
        Pending = 1,
        /// The Job is disabled.
        Disabled = 2,
        /// The Job is in progress.
        InProgress = 3,
        /// The Job succeeded.
        Succeeded = 4,
        /// The Job failed.
        Failed = 5,
        /// The Job was aborted.
        Aborted = 6,
        /// The Job was skipped.
        Skipped = 7,
        /// The Job was ignored.
        Ignored = 8,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Pending => "PENDING",
                State::Disabled => "DISABLED",
                State::InProgress => "IN_PROGRESS",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::Aborted => "ABORTED",
                State::Skipped => "SKIPPED",
                State::Ignored => "IGNORED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "DISABLED" => Some(Self::Disabled),
                "IN_PROGRESS" => Some(Self::InProgress),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "ABORTED" => Some(Self::Aborted),
                "SKIPPED" => Some(Self::Skipped),
                "IGNORED" => Some(Self::Ignored),
                _ => None,
            }
        }
    }
    /// The type of Job.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum JobType {
        /// Output only. A deploy Job.
        #[prost(message, tag = "4")]
        DeployJob(super::DeployJob),
        /// Output only. A verify Job.
        #[prost(message, tag = "5")]
        VerifyJob(super::VerifyJob),
        /// Output only. A predeploy Job.
        #[prost(message, tag = "9")]
        PredeployJob(super::PredeployJob),
        /// Output only. A postdeploy Job.
        #[prost(message, tag = "10")]
        PostdeployJob(super::PostdeployJob),
        /// Output only. A createChildRollout Job.
        #[prost(message, tag = "6")]
        CreateChildRolloutJob(super::CreateChildRolloutJob),
        /// Output only. An advanceChildRollout Job.
        #[prost(message, tag = "7")]
        AdvanceChildRolloutJob(super::AdvanceChildRolloutJob),
    }
}
/// A deploy Job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployJob {}
/// A verify Job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyJob {}
/// A predeploy Job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredeployJob {
    /// Output only. The custom actions that the predeploy Job executes.
    #[prost(string, repeated, tag = "1")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A postdeploy Job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostdeployJob {
    /// Output only. The custom actions that the postdeploy Job executes.
    #[prost(string, repeated, tag = "1")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A createChildRollout Job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChildRolloutJob {}
/// An advanceChildRollout Job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvanceChildRolloutJob {}
/// ListRolloutsRequest is the request object used by `ListRollouts`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRolloutsRequest {
    /// Required. The `Release` which owns this collection of `Rollout` objects.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `Rollout` objects to return. The service
    /// may return fewer than this value. If unspecified, at most 50 `Rollout`
    /// objects will be returned. The maximum value is 1000; values above 1000 will
    /// be set to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListRollouts` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other provided parameters match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter rollouts to be returned. See <https://google.aip.dev/160>
    /// for more details.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Field to sort by. See <https://google.aip.dev/132#ordering> for
    /// more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// ListRolloutsResponse is the response object reutrned by `ListRollouts`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRolloutsResponse {
    /// The `Rollout` objects.
    #[prost(message, repeated, tag = "1")]
    pub rollouts: ::prost::alloc::vec::Vec<Rollout>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetRolloutRequest is the request object used by `GetRollout`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRolloutRequest {
    /// Required. Name of the `Rollout`. Format must be
    /// `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/releases/{release_name}/rollouts/{rollout_name}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// CreateRolloutRequest is the request object used by `CreateRollout`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRolloutRequest {
    /// Required. The parent collection in which the `Rollout` should be created.
    /// Format should be
    /// `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/releases/{release_name}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the `Rollout`.
    #[prost(string, tag = "2")]
    pub rollout_id: ::prost::alloc::string::String,
    /// Required. The `Rollout` to create.
    #[prost(message, optional, tag = "3")]
    pub rollout: ::core::option::Option<Rollout>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
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
    /// Optional. If set to true, the request is validated and the user is provided
    /// with an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
    /// Optional. The starting phase ID for the `Rollout`. If empty the `Rollout`
    /// will start at the first phase.
    #[prost(string, tag = "7")]
    pub starting_phase_id: ::prost::alloc::string::String,
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
}
/// The request object used by `ApproveRollout`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveRolloutRequest {
    /// Required. Name of the Rollout. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/{rollout}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. True = approve; false = reject
    #[prost(bool, tag = "2")]
    pub approved: bool,
}
/// The response object from `ApproveRollout`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveRolloutResponse {}
/// The request object used by `AdvanceRollout`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvanceRolloutRequest {
    /// Required. Name of the Rollout. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/{rollout}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The phase ID to advance the `Rollout` to.
    #[prost(string, tag = "2")]
    pub phase_id: ::prost::alloc::string::String,
}
/// The response object from `AdvanceRollout`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvanceRolloutResponse {}
/// The request object used by `CancelRollout`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelRolloutRequest {
    /// Required. Name of the Rollout. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/{rollout}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The response object from `CancelRollout`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelRolloutResponse {}
/// The request object used by `IgnoreJob`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IgnoreJobRequest {
    /// Required. Name of the Rollout. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/{rollout}`.
    #[prost(string, tag = "1")]
    pub rollout: ::prost::alloc::string::String,
    /// Required. The phase ID the Job to ignore belongs to.
    #[prost(string, tag = "2")]
    pub phase_id: ::prost::alloc::string::String,
    /// Required. The job ID for the Job to ignore.
    #[prost(string, tag = "3")]
    pub job_id: ::prost::alloc::string::String,
}
/// The response object from `IgnoreJob`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IgnoreJobResponse {}
/// RetryJobRequest is the request object used by `RetryJob`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryJobRequest {
    /// Required. Name of the Rollout. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/{rollout}`.
    #[prost(string, tag = "1")]
    pub rollout: ::prost::alloc::string::String,
    /// Required. The phase ID the Job to retry belongs to.
    #[prost(string, tag = "2")]
    pub phase_id: ::prost::alloc::string::String,
    /// Required. The job ID for the Job to retry.
    #[prost(string, tag = "3")]
    pub job_id: ::prost::alloc::string::String,
}
/// The response object from 'RetryJob'.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryJobResponse {}
/// The request object used by `AbandonRelease`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbandonReleaseRequest {
    /// Required. Name of the Release. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The response object for `AbandonRelease`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbandonReleaseResponse {}
/// A `JobRun` resource in the Cloud Deploy API.
///
/// A `JobRun` contains information of a single `Rollout` job evaluation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobRun {
    /// Optional. Name of the `JobRun`. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{releases}/rollouts/{rollouts}/jobRuns/{uuid}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Unique identifier of the `JobRun`.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. ID of the `Rollout` phase this `JobRun` belongs in.
    #[prost(string, tag = "3")]
    pub phase_id: ::prost::alloc::string::String,
    /// Output only. ID of the `Rollout` job this `JobRun` corresponds to.
    #[prost(string, tag = "4")]
    pub job_id: ::prost::alloc::string::String,
    /// Output only. Time at which the `JobRun` was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the `JobRun` was started.
    #[prost(message, optional, tag = "6")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the `JobRun` ended.
    #[prost(message, optional, tag = "7")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the `JobRun`.
    #[prost(enumeration = "job_run::State", tag = "8")]
    pub state: i32,
    /// Output only. This checksum is computed by the server based on the value of
    /// other fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "11")]
    pub etag: ::prost::alloc::string::String,
    /// The `JobRun` type and the information for that type.
    #[prost(oneof = "job_run::JobRun", tags = "9, 10, 14, 15, 12, 13")]
    pub job_run: ::core::option::Option<job_run::JobRun>,
}
/// Nested message and enum types in `JobRun`.
pub mod job_run {
    /// Valid states of a `JobRun`.
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
        /// The `JobRun` has an unspecified state.
        Unspecified = 0,
        /// The `JobRun` is in progress.
        InProgress = 1,
        /// The `JobRun` has succeeded.
        Succeeded = 2,
        /// The `JobRun` has failed.
        Failed = 3,
        /// The `JobRun` is terminating.
        Terminating = 4,
        /// The `JobRun` was terminated.
        Terminated = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::InProgress => "IN_PROGRESS",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::Terminating => "TERMINATING",
                State::Terminated => "TERMINATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "IN_PROGRESS" => Some(Self::InProgress),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "TERMINATING" => Some(Self::Terminating),
                "TERMINATED" => Some(Self::Terminated),
                _ => None,
            }
        }
    }
    /// The `JobRun` type and the information for that type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum JobRun {
        /// Output only. Information specific to a deploy `JobRun`.
        #[prost(message, tag = "9")]
        DeployJobRun(super::DeployJobRun),
        /// Output only. Information specific to a verify `JobRun`.
        #[prost(message, tag = "10")]
        VerifyJobRun(super::VerifyJobRun),
        /// Output only. Information specific to a predeploy `JobRun`.
        #[prost(message, tag = "14")]
        PredeployJobRun(super::PredeployJobRun),
        /// Output only. Information specific to a postdeploy `JobRun`.
        #[prost(message, tag = "15")]
        PostdeployJobRun(super::PostdeployJobRun),
        /// Output only. Information specific to a createChildRollout `JobRun`.
        #[prost(message, tag = "12")]
        CreateChildRolloutJobRun(super::CreateChildRolloutJobRun),
        /// Output only. Information specific to an advanceChildRollout `JobRun`
        #[prost(message, tag = "13")]
        AdvanceChildRolloutJobRun(super::AdvanceChildRolloutJobRun),
    }
}
/// DeployJobRun contains information specific to a deploy `JobRun`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployJobRun {
    /// Output only. The resource name of the Cloud Build `Build` object that is
    /// used to deploy. Format is
    /// `projects/{project}/locations/{location}/builds/{build}`.
    #[prost(string, tag = "1")]
    pub build: ::prost::alloc::string::String,
    /// Output only. The reason the deploy failed. This will always be unspecified
    /// while the deploy is in progress or if it succeeded.
    #[prost(enumeration = "deploy_job_run::FailureCause", tag = "2")]
    pub failure_cause: i32,
    /// Output only. Additional information about the deploy failure, if available.
    #[prost(string, tag = "3")]
    pub failure_message: ::prost::alloc::string::String,
    /// Output only. Metadata containing information about the deploy job run.
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<DeployJobRunMetadata>,
    /// Output only. The artifact of a deploy job run, if available.
    #[prost(message, optional, tag = "5")]
    pub artifact: ::core::option::Option<DeployArtifact>,
}
/// Nested message and enum types in `DeployJobRun`.
pub mod deploy_job_run {
    /// Well-known deploy failures.
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
    pub enum FailureCause {
        /// No reason for failure is specified.
        Unspecified = 0,
        /// Cloud Build is not available, either because it is not enabled or because
        /// Cloud Deploy has insufficient permissions. See [Required
        /// permission](<https://cloud.google.com/deploy/docs/cloud-deploy-service-account#required_permissions>).
        CloudBuildUnavailable = 1,
        /// The deploy operation did not complete successfully; check Cloud Build
        /// logs.
        ExecutionFailed = 2,
        /// The deploy job run did not complete within the alloted time.
        DeadlineExceeded = 3,
        /// There were missing resources in the runtime environment required for a
        /// canary deployment. Check the Cloud Build logs for more information.
        MissingResourcesForCanary = 4,
        /// Cloud Build failed to fulfill Cloud Deploy's request. See failure_message
        /// for additional details.
        CloudBuildRequestFailed = 5,
        /// The deploy operation had a feature configured that is not supported.
        DeployFeatureNotSupported = 6,
    }
    impl FailureCause {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FailureCause::Unspecified => "FAILURE_CAUSE_UNSPECIFIED",
                FailureCause::CloudBuildUnavailable => "CLOUD_BUILD_UNAVAILABLE",
                FailureCause::ExecutionFailed => "EXECUTION_FAILED",
                FailureCause::DeadlineExceeded => "DEADLINE_EXCEEDED",
                FailureCause::MissingResourcesForCanary => "MISSING_RESOURCES_FOR_CANARY",
                FailureCause::CloudBuildRequestFailed => "CLOUD_BUILD_REQUEST_FAILED",
                FailureCause::DeployFeatureNotSupported => "DEPLOY_FEATURE_NOT_SUPPORTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FAILURE_CAUSE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOUD_BUILD_UNAVAILABLE" => Some(Self::CloudBuildUnavailable),
                "EXECUTION_FAILED" => Some(Self::ExecutionFailed),
                "DEADLINE_EXCEEDED" => Some(Self::DeadlineExceeded),
                "MISSING_RESOURCES_FOR_CANARY" => Some(Self::MissingResourcesForCanary),
                "CLOUD_BUILD_REQUEST_FAILED" => Some(Self::CloudBuildRequestFailed),
                "DEPLOY_FEATURE_NOT_SUPPORTED" => Some(Self::DeployFeatureNotSupported),
                _ => None,
            }
        }
    }
}
/// VerifyJobRun contains information specific to a verify `JobRun`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyJobRun {
    /// Output only. The resource name of the Cloud Build `Build` object that is
    /// used to verify. Format is
    /// `projects/{project}/locations/{location}/builds/{build}`.
    #[prost(string, tag = "1")]
    pub build: ::prost::alloc::string::String,
    /// Output only. URI of a directory containing the verify artifacts. This
    /// contains the Skaffold event log.
    #[prost(string, tag = "2")]
    pub artifact_uri: ::prost::alloc::string::String,
    /// Output only. File path of the Skaffold event log relative to the artifact
    /// URI.
    #[prost(string, tag = "3")]
    pub event_log_path: ::prost::alloc::string::String,
    /// Output only. The reason the verify failed. This will always be unspecified
    /// while the verify is in progress or if it succeeded.
    #[prost(enumeration = "verify_job_run::FailureCause", tag = "4")]
    pub failure_cause: i32,
    /// Output only. Additional information about the verify failure, if available.
    #[prost(string, tag = "5")]
    pub failure_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `VerifyJobRun`.
pub mod verify_job_run {
    /// Well-known verify failures.
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
    pub enum FailureCause {
        /// No reason for failure is specified.
        Unspecified = 0,
        /// Cloud Build is not available, either because it is not enabled or because
        /// Cloud Deploy has insufficient permissions. See [required
        /// permission](<https://cloud.google.com/deploy/docs/cloud-deploy-service-account#required_permissions>).
        CloudBuildUnavailable = 1,
        /// The verify operation did not complete successfully; check Cloud Build
        /// logs.
        ExecutionFailed = 2,
        /// The verify job run did not complete within the alloted time.
        DeadlineExceeded = 3,
        /// No Skaffold verify configuration was found.
        VerificationConfigNotFound = 4,
        /// Cloud Build failed to fulfill Cloud Deploy's request. See failure_message
        /// for additional details.
        CloudBuildRequestFailed = 5,
    }
    impl FailureCause {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FailureCause::Unspecified => "FAILURE_CAUSE_UNSPECIFIED",
                FailureCause::CloudBuildUnavailable => "CLOUD_BUILD_UNAVAILABLE",
                FailureCause::ExecutionFailed => "EXECUTION_FAILED",
                FailureCause::DeadlineExceeded => "DEADLINE_EXCEEDED",
                FailureCause::VerificationConfigNotFound => {
                    "VERIFICATION_CONFIG_NOT_FOUND"
                }
                FailureCause::CloudBuildRequestFailed => "CLOUD_BUILD_REQUEST_FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FAILURE_CAUSE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOUD_BUILD_UNAVAILABLE" => Some(Self::CloudBuildUnavailable),
                "EXECUTION_FAILED" => Some(Self::ExecutionFailed),
                "DEADLINE_EXCEEDED" => Some(Self::DeadlineExceeded),
                "VERIFICATION_CONFIG_NOT_FOUND" => Some(Self::VerificationConfigNotFound),
                "CLOUD_BUILD_REQUEST_FAILED" => Some(Self::CloudBuildRequestFailed),
                _ => None,
            }
        }
    }
}
/// PredeployJobRun contains information specific to a predeploy `JobRun`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredeployJobRun {
    /// Output only. The resource name of the Cloud Build `Build` object that is
    /// used to execute the custom actions associated with the predeploy Job.
    /// Format is `projects/{project}/locations/{location}/builds/{build}`.
    #[prost(string, tag = "1")]
    pub build: ::prost::alloc::string::String,
    /// Output only. The reason the predeploy failed. This will always be
    /// unspecified while the predeploy is in progress or if it succeeded.
    #[prost(enumeration = "predeploy_job_run::FailureCause", tag = "2")]
    pub failure_cause: i32,
    /// Output only. Additional information about the predeploy failure, if
    /// available.
    #[prost(string, tag = "3")]
    pub failure_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PredeployJobRun`.
pub mod predeploy_job_run {
    /// Well-known predeploy failures.
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
    pub enum FailureCause {
        /// No reason for failure is specified.
        Unspecified = 0,
        /// Cloud Build is not available, either because it is not enabled or because
        /// Cloud Deploy has insufficient permissions. See [required
        /// permission](<https://cloud.google.com/deploy/docs/cloud-deploy-service-account#required_permissions>).
        CloudBuildUnavailable = 1,
        /// The predeploy operation did not complete successfully; check Cloud Build
        /// logs.
        ExecutionFailed = 2,
        /// The predeploy job run did not complete within the alloted time.
        DeadlineExceeded = 3,
        /// Cloud Build failed to fulfill Cloud Deploy's request. See failure_message
        /// for additional details.
        CloudBuildRequestFailed = 4,
    }
    impl FailureCause {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FailureCause::Unspecified => "FAILURE_CAUSE_UNSPECIFIED",
                FailureCause::CloudBuildUnavailable => "CLOUD_BUILD_UNAVAILABLE",
                FailureCause::ExecutionFailed => "EXECUTION_FAILED",
                FailureCause::DeadlineExceeded => "DEADLINE_EXCEEDED",
                FailureCause::CloudBuildRequestFailed => "CLOUD_BUILD_REQUEST_FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FAILURE_CAUSE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOUD_BUILD_UNAVAILABLE" => Some(Self::CloudBuildUnavailable),
                "EXECUTION_FAILED" => Some(Self::ExecutionFailed),
                "DEADLINE_EXCEEDED" => Some(Self::DeadlineExceeded),
                "CLOUD_BUILD_REQUEST_FAILED" => Some(Self::CloudBuildRequestFailed),
                _ => None,
            }
        }
    }
}
/// PostdeployJobRun contains information specific to a postdeploy `JobRun`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostdeployJobRun {
    /// Output only. The resource name of the Cloud Build `Build` object that is
    /// used to execute the custom actions associated with the postdeploy Job.
    /// Format is `projects/{project}/locations/{location}/builds/{build}`.
    #[prost(string, tag = "1")]
    pub build: ::prost::alloc::string::String,
    /// Output only. The reason the postdeploy failed. This will always be
    /// unspecified while the postdeploy is in progress or if it succeeded.
    #[prost(enumeration = "postdeploy_job_run::FailureCause", tag = "2")]
    pub failure_cause: i32,
    /// Output only. Additional information about the postdeploy failure, if
    /// available.
    #[prost(string, tag = "3")]
    pub failure_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PostdeployJobRun`.
pub mod postdeploy_job_run {
    /// Well-known postdeploy failures.
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
    pub enum FailureCause {
        /// No reason for failure is specified.
        Unspecified = 0,
        /// Cloud Build is not available, either because it is not enabled or because
        /// Cloud Deploy has insufficient permissions. See [required
        /// permission](<https://cloud.google.com/deploy/docs/cloud-deploy-service-account#required_permissions>).
        CloudBuildUnavailable = 1,
        /// The postdeploy operation did not complete successfully; check Cloud Build
        /// logs.
        ExecutionFailed = 2,
        /// The postdeploy job run did not complete within the alloted time.
        DeadlineExceeded = 3,
        /// Cloud Build failed to fulfill Cloud Deploy's request. See failure_message
        /// for additional details.
        CloudBuildRequestFailed = 4,
    }
    impl FailureCause {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FailureCause::Unspecified => "FAILURE_CAUSE_UNSPECIFIED",
                FailureCause::CloudBuildUnavailable => "CLOUD_BUILD_UNAVAILABLE",
                FailureCause::ExecutionFailed => "EXECUTION_FAILED",
                FailureCause::DeadlineExceeded => "DEADLINE_EXCEEDED",
                FailureCause::CloudBuildRequestFailed => "CLOUD_BUILD_REQUEST_FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FAILURE_CAUSE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOUD_BUILD_UNAVAILABLE" => Some(Self::CloudBuildUnavailable),
                "EXECUTION_FAILED" => Some(Self::ExecutionFailed),
                "DEADLINE_EXCEEDED" => Some(Self::DeadlineExceeded),
                "CLOUD_BUILD_REQUEST_FAILED" => Some(Self::CloudBuildRequestFailed),
                _ => None,
            }
        }
    }
}
/// CreateChildRolloutJobRun contains information specific to a
/// createChildRollout `JobRun`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChildRolloutJobRun {
    /// Output only. Name of the `ChildRollout`. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/[a-z][a-z0-9\-]{0,62}`.
    #[prost(string, tag = "1")]
    pub rollout: ::prost::alloc::string::String,
    /// Output only. The ID of the childRollout Phase initiated by this JobRun.
    #[prost(string, tag = "2")]
    pub rollout_phase_id: ::prost::alloc::string::String,
}
/// AdvanceChildRolloutJobRun contains information specific to a
/// advanceChildRollout `JobRun`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvanceChildRolloutJobRun {
    /// Output only. Name of the `ChildRollout`. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/[a-z][a-z0-9\-]{0,62}`.
    #[prost(string, tag = "1")]
    pub rollout: ::prost::alloc::string::String,
    /// Output only. the ID of the ChildRollout's Phase.
    #[prost(string, tag = "2")]
    pub rollout_phase_id: ::prost::alloc::string::String,
}
/// ListJobRunsRequest is the request object used by `ListJobRuns`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobRunsRequest {
    /// Required. The `Rollout` which owns this collection of `JobRun` objects.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `JobRun` objects to return. The service may
    /// return fewer than this value. If unspecified, at most 50 `JobRun` objects
    /// will be returned. The maximum value is 1000; values above 1000 will be set
    /// to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListJobRuns` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other provided parameters match the call that provided
    /// the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter results to be returned. See <https://google.aip.dev/160> for
    /// more details.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Field to sort by. See <https://google.aip.dev/132#ordering> for
    /// more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// ListJobRunsResponse is the response object returned by `ListJobRuns`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobRunsResponse {
    /// The `JobRun` objects.
    #[prost(message, repeated, tag = "1")]
    pub job_runs: ::prost::alloc::vec::Vec<JobRun>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If
    /// this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetJobRunRequest is the request object used by `GetJobRun`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobRunRequest {
    /// Required. Name of the `JobRun`. Format must be
    /// `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/releases/{release_name}/rollouts/{rollout_name}/jobRuns/{job_run_name}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request object used by `TerminateJobRun`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerminateJobRunRequest {
    /// Required. Name of the `JobRun`. Format must be
    /// `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/{rollout}/jobRuns/{jobRun}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The response object from `TerminateJobRun`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerminateJobRunResponse {}
/// Service-wide configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// Name of the configuration.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// All supported versions of Skaffold.
    #[prost(message, repeated, tag = "2")]
    pub supported_versions: ::prost::alloc::vec::Vec<SkaffoldVersion>,
    /// Default Skaffold version that is assigned when a Release is created without
    /// specifying a Skaffold version.
    #[prost(string, tag = "3")]
    pub default_skaffold_version: ::prost::alloc::string::String,
}
/// Details of a supported Skaffold version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkaffoldVersion {
    /// Release version number. For example, "1.20.3".
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// The time at which this version of Skaffold will enter maintenance mode.
    #[prost(message, optional, tag = "3")]
    pub maintenance_mode_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which this version of Skaffold will no longer be supported.
    #[prost(message, optional, tag = "4")]
    pub support_expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Date when this version is expected to no longer be supported.
    #[prost(message, optional, tag = "2")]
    pub support_end_date: ::core::option::Option<super::super::super::r#type::Date>,
}
/// Request to get a configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigRequest {
    /// Required. Name of requested configuration.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// An `Automation` resource in the Cloud Deploy API.
///
/// An `Automation` enables the automation of manually driven actions for
/// a Delivery Pipeline, which includes Release promotion among Targets,
/// Rollout repair and Rollout deployment strategy advancement. The intention
/// of Automation is to reduce manual intervention in the continuous delivery
/// process.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Automation {
    /// Output only. Name of the `Automation`. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{delivery_pipeline}/automations/{automation}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Unique identifier of the `Automation`.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Optional. Description of the `Automation`. Max length is 255 characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Time at which the automation was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the automation was updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. User annotations. These attributes can only be set and used by
    /// the user, and not by Cloud Deploy. Annotations must meet the following
    /// constraints:
    ///
    /// * Annotations are key/value pairs.
    /// * Valid annotation keys have two segments: an optional prefix and name,
    /// separated by a slash (`/`).
    /// * The name segment is required and must be 63 characters or less,
    /// beginning and ending with an alphanumeric character (`\[a-z0-9A-Z\]`) with
    /// dashes (`-`), underscores (`_`), dots (`.`), and alphanumerics between.
    /// * The prefix is optional. If specified, the prefix must be a DNS subdomain:
    /// a series of DNS labels separated by dots(`.`), not longer than 253
    /// characters in total, followed by a slash (`/`).
    ///
    /// See
    /// <https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set>
    /// for more details.
    #[prost(btree_map = "string, string", tag = "6")]
    pub annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. Labels are attributes that can be set and used by both the
    /// user and by Cloud Deploy. Labels must meet the following constraints:
    ///
    /// * Keys and values can contain only lowercase letters, numeric characters,
    /// underscores, and dashes.
    /// * All characters must use UTF-8 encoding, and international characters are
    /// allowed.
    /// * Keys must start with a lowercase letter or international character.
    /// * Each resource is limited to a maximum of 64 labels.
    ///
    /// Both keys and values are additionally constrained to be <= 63 characters.
    #[prost(btree_map = "string, string", tag = "7")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. The weak etag of the `Automation` resource.
    /// This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "8")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. When Suspended, automation is deactivated from execution.
    #[prost(bool, tag = "9")]
    pub suspended: bool,
    /// Required. Email address of the user-managed IAM service account that
    /// creates Cloud Deploy release and rollout resources.
    #[prost(string, tag = "10")]
    pub service_account: ::prost::alloc::string::String,
    /// Required. Selected resources to which the automation will be applied.
    #[prost(message, optional, tag = "11")]
    pub selector: ::core::option::Option<AutomationResourceSelector>,
    /// Required. List of Automation rules associated with the Automation resource.
    /// Must have at least one rule and limited to 250 rules per Delivery Pipeline.
    /// Note: the order of the rules here is not the same as the order of
    /// execution.
    #[prost(message, repeated, tag = "14")]
    pub rules: ::prost::alloc::vec::Vec<AutomationRule>,
}
/// AutomationResourceSelector contains the information to select the resources
/// to which an Automation is going to be applied.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomationResourceSelector {
    /// Contains attributes about a target.
    #[prost(message, repeated, tag = "1")]
    pub targets: ::prost::alloc::vec::Vec<TargetAttribute>,
}
/// `AutomationRule` defines the automation activities.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomationRule {
    /// The configuration of the Automation rule.
    #[prost(oneof = "automation_rule::Rule", tags = "1, 2, 3")]
    pub rule: ::core::option::Option<automation_rule::Rule>,
}
/// Nested message and enum types in `AutomationRule`.
pub mod automation_rule {
    /// The configuration of the Automation rule.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rule {
        /// Optional. `PromoteReleaseRule` will automatically promote a release from
        /// the current target to a specified target.
        #[prost(message, tag = "1")]
        PromoteReleaseRule(super::PromoteReleaseRule),
        /// Optional. The `AdvanceRolloutRule` will automatically advance a
        /// successful Rollout.
        #[prost(message, tag = "2")]
        AdvanceRolloutRule(super::AdvanceRolloutRule),
        /// Optional. The `RepairRolloutRule` will automatically repair a failed
        /// rollout.
        #[prost(message, tag = "3")]
        RepairRolloutRule(super::RepairRolloutRule),
    }
}
/// `PromoteRelease` rule will automatically promote a release from the current
/// target to a specified target.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteReleaseRule {
    /// Required. ID of the rule. This id must be unique in the `Automation`
    /// resource to which this rule belongs. The format is `[a-z][a-z0-9\-]{0,62}`.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Optional. How long the release need to be paused until being promoted to
    /// the next target.
    #[prost(message, optional, tag = "2")]
    pub wait: ::core::option::Option<::prost_types::Duration>,
    /// Optional. The ID of the stage in the pipeline to which this `Release` is
    /// deploying. If unspecified, default it to the next stage in the promotion
    /// flow. The value of this field could be one of the following:
    ///
    /// * The last segment of a target name. It only needs the ID to determine
    /// if the target is one of the stages in the promotion sequence defined
    /// in the pipeline.
    /// * "@next", the next target in the promotion sequence.
    #[prost(string, tag = "7")]
    pub destination_target_id: ::prost::alloc::string::String,
    /// Output only. Information around the state of the Automation rule.
    #[prost(message, optional, tag = "5")]
    pub condition: ::core::option::Option<AutomationRuleCondition>,
    /// Optional. The starting phase of the rollout created by this operation.
    /// Default to the first phase.
    #[prost(string, tag = "8")]
    pub destination_phase: ::prost::alloc::string::String,
}
/// The `AdvanceRollout` automation rule will automatically advance a successful
/// Rollout to the next phase.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvanceRolloutRule {
    /// Required. ID of the rule. This id must be unique in the `Automation`
    /// resource to which this rule belongs. The format is `[a-z][a-z0-9\-]{0,62}`.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Optional. Proceeds only after phase name matched any one in the list.
    /// This value must consist of lower-case letters, numbers, and hyphens,
    /// start with a letter and end with a letter or a number, and have a max
    /// length of 63 characters. In other words, it must match the following
    /// regex: `^[a-z](\[a-z0-9-\]{0,61}\[a-z0-9\])?$`.
    #[prost(string, repeated, tag = "6")]
    pub source_phases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. How long to wait after a rollout is finished.
    #[prost(message, optional, tag = "3")]
    pub wait: ::core::option::Option<::prost_types::Duration>,
    /// Output only. Information around the state of the Automation rule.
    #[prost(message, optional, tag = "5")]
    pub condition: ::core::option::Option<AutomationRuleCondition>,
}
/// The `RepairRolloutRule` automation rule will automatically repair a failed
/// `Rollout`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepairRolloutRule {
    /// Required. ID of the rule. This id must be unique in the `Automation`
    /// resource to which this rule belongs. The format is `[a-z][a-z0-9\-]{0,62}`.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Optional. Phases within which jobs are subject to automatic repair actions
    /// on failure. Proceeds only after phase name matched any one in the list, or
    /// for all phases if unspecified. This value must consist of lower-case
    /// letters, numbers, and hyphens, start with a letter and end with a letter or
    /// a number, and have a max length of 63 characters. In other words, it must
    /// match the following regex: `^[a-z](\[a-z0-9-\]{0,61}\[a-z0-9\])?$`.
    #[prost(string, repeated, tag = "2")]
    pub source_phases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Jobs to repair. Proceeds only after job name matched any one in
    /// the list, or for all jobs if unspecified or empty. The phase that includes
    /// the job must match the phase ID specified in `source_phase`. This value
    /// must consist of lower-case letters, numbers, and hyphens, start with a
    /// letter and end with a letter or a number, and have a max length of 63
    /// characters. In other words, it must match the following regex:
    /// `^[a-z](\[a-z0-9-\]{0,61}\[a-z0-9\])?$`.
    #[prost(string, repeated, tag = "3")]
    pub jobs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Defines the types of automatic repair actions for failed jobs.
    #[prost(message, repeated, tag = "4")]
    pub repair_modes: ::prost::alloc::vec::Vec<RepairMode>,
    /// Output only. Information around the state of the 'Automation' rule.
    #[prost(message, optional, tag = "6")]
    pub condition: ::core::option::Option<AutomationRuleCondition>,
}
/// Configuration of the repair action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepairMode {
    /// The repair action to perform.
    #[prost(oneof = "repair_mode::Mode", tags = "1, 2")]
    pub mode: ::core::option::Option<repair_mode::Mode>,
}
/// Nested message and enum types in `RepairMode`.
pub mod repair_mode {
    /// The repair action to perform.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mode {
        /// Optional. Retries a failed job.
        #[prost(message, tag = "1")]
        Retry(super::Retry),
        /// Optional. Rolls back a `Rollout`.
        #[prost(message, tag = "2")]
        Rollback(super::Rollback),
    }
}
/// Retries the failed job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Retry {
    /// Required. Total number of retries. Retry is skipped if set to 0; The
    /// minimum value is 1, and the maximum value is 10.
    #[prost(int64, tag = "1")]
    pub attempts: i64,
    /// Optional. How long to wait for the first retry. Default is 0, and the
    /// maximum value is 14d.
    #[prost(message, optional, tag = "2")]
    pub wait: ::core::option::Option<::prost_types::Duration>,
    /// Optional. The pattern of how wait time will be increased. Default is
    /// linear. Backoff mode will be ignored if `wait` is 0.
    #[prost(enumeration = "BackoffMode", tag = "3")]
    pub backoff_mode: i32,
}
/// Rolls back a `Rollout`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rollback {
    /// Optional. The starting phase ID for the `Rollout`. If unspecified, the
    /// `Rollout` will start in the stable phase.
    #[prost(string, tag = "1")]
    pub destination_phase: ::prost::alloc::string::String,
}
/// `AutomationRuleCondition` contains conditions relevant to an
/// `Automation` rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomationRuleCondition {
    /// Optional. Details around targets enumerated in the rule.
    #[prost(message, optional, tag = "1")]
    pub targets_present_condition: ::core::option::Option<TargetsPresentCondition>,
}
/// The request object for `CreateAutomation`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAutomationRequest {
    /// Required. The parent collection in which the `Automation` should be
    /// created. Format should be
    /// `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the `Automation`.
    #[prost(string, tag = "2")]
    pub automation_id: ::prost::alloc::string::String,
    /// Required. The `Automation` to create.
    #[prost(message, optional, tag = "3")]
    pub automation: ::core::option::Option<Automation>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
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
    /// Optional. If set to true, the request is validated and the user is provided
    /// with an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// The request object for `UpdateAutomation`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAutomationRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `Automation` resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it's in the mask. If the
    /// user doesn't provide a mask then all fields are overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The `Automation` to update.
    #[prost(message, optional, tag = "2")]
    pub automation: ::core::option::Option<Automation>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
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
    /// Optional. If set to true, updating a `Automation` that does not exist will
    /// result in the creation of a new `Automation`.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
    /// Optional. If set to true, the request is validated and the user is provided
    /// with an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// The request object for `DeleteAutomation`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAutomationRequest {
    /// Required. The name of the `Automation` to delete. Format should be
    /// `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/automations/{automation_name}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server knows to ignore the
    /// request if it has already been completed. The server guarantees that for
    /// at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, then deleting an already deleted or non-existing
    /// `Automation` will succeed.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// Optional. If set, validate the request and verify whether the resource
    /// exists, but do not actually post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// Optional. The weak etag of the request.
    /// This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "5")]
    pub etag: ::prost::alloc::string::String,
}
/// The request object for `ListAutomations`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAutomationsRequest {
    /// Required. The parent `Delivery Pipeline`, which owns this collection of
    /// automations. Format must be
    /// `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of automations to return. The service may return
    /// fewer than this value. If unspecified, at most 50 automations will
    /// be returned. The maximum value is 1000; values above 1000 will be set
    /// to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAutomations` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other provided parameters match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter automations to be returned. All fields can be used in the
    /// filter.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to sort by.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response object from `ListAutomations`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAutomationsResponse {
    /// The `Automation` objects.
    #[prost(message, repeated, tag = "1")]
    pub automations: ::prost::alloc::vec::Vec<Automation>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request object for `GetAutomation`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAutomationRequest {
    /// Required. Name of the `Automation`. Format must be
    /// `projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/automations/{automation_name}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// An `AutomationRun` resource in the Cloud Deploy API.
///
/// An `AutomationRun` represents an execution instance of an
/// automation rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomationRun {
    /// Output only. Name of the `AutomationRun`. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{delivery_pipeline}/automationRuns/{automation_run}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time at which the `AutomationRun` was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the automationRun was updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The weak etag of the `AutomationRun` resource.
    /// This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "4")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. Email address of the user-managed IAM service account that
    /// performs the operations against Cloud Deploy resources.
    #[prost(string, tag = "5")]
    pub service_account: ::prost::alloc::string::String,
    /// Output only. Snapshot of the Automation taken at AutomationRun creation
    /// time.
    #[prost(message, optional, tag = "6")]
    pub automation_snapshot: ::core::option::Option<Automation>,
    /// Output only. The ID of the target that represents the promotion stage that
    /// initiates the `AutomationRun`. The value of this field is the last segment
    /// of a target name.
    #[prost(string, tag = "7")]
    pub target_id: ::prost::alloc::string::String,
    /// Output only. Current state of the `AutomationRun`.
    #[prost(enumeration = "automation_run::State", tag = "8")]
    pub state: i32,
    /// Output only. Explains the current state of the `AutomationRun`. Present
    /// only when an explanation is needed.
    #[prost(string, tag = "9")]
    pub state_description: ::prost::alloc::string::String,
    /// Output only. Time the `AutomationRun` expires. An `AutomationRun` expires
    /// after 14 days from its creation date.
    #[prost(message, optional, tag = "11")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The ID of the automation rule that initiated the operation.
    #[prost(string, tag = "12")]
    pub rule_id: ::prost::alloc::string::String,
    /// Output only. The ID of the automation that initiated the operation.
    #[prost(string, tag = "15")]
    pub automation_id: ::prost::alloc::string::String,
    /// Output only. Earliest time the `AutomationRun` will attempt to resume.
    /// Wait-time is configured by `wait` in automation rule.
    #[prost(message, optional, tag = "16")]
    pub wait_until_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The operation that the `AutomationRun` will perform.
    #[prost(oneof = "automation_run::Operation", tags = "13, 14, 17")]
    pub operation: ::core::option::Option<automation_run::Operation>,
}
/// Nested message and enum types in `AutomationRun`.
pub mod automation_run {
    /// Valid state of an `AutomationRun`.
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
        /// The `AutomationRun` has an unspecified state.
        Unspecified = 0,
        /// The `AutomationRun` has succeeded.
        Succeeded = 1,
        /// The `AutomationRun` was cancelled.
        Cancelled = 2,
        /// The `AutomationRun` has failed.
        Failed = 3,
        /// The `AutomationRun` is in progress.
        InProgress = 4,
        /// The `AutomationRun` is pending.
        Pending = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Succeeded => "SUCCEEDED",
                State::Cancelled => "CANCELLED",
                State::Failed => "FAILED",
                State::InProgress => "IN_PROGRESS",
                State::Pending => "PENDING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCEEDED" => Some(Self::Succeeded),
                "CANCELLED" => Some(Self::Cancelled),
                "FAILED" => Some(Self::Failed),
                "IN_PROGRESS" => Some(Self::InProgress),
                "PENDING" => Some(Self::Pending),
                _ => None,
            }
        }
    }
    /// The operation that the `AutomationRun` will perform.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Output only. Promotes a release to a specified 'Target'.
        #[prost(message, tag = "13")]
        PromoteReleaseOperation(super::PromoteReleaseOperation),
        /// Output only. Advances a rollout to the next phase.
        #[prost(message, tag = "14")]
        AdvanceRolloutOperation(super::AdvanceRolloutOperation),
        /// Output only. Repairs a failed 'Rollout'.
        #[prost(message, tag = "17")]
        RepairRolloutOperation(super::RepairRolloutOperation),
    }
}
/// Contains the information of an automated promote-release operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteReleaseOperation {
    /// Output only. The ID of the target that represents the promotion stage to
    /// which the release will be promoted. The value of this field is the last
    /// segment of a target name.
    #[prost(string, tag = "1")]
    pub target_id: ::prost::alloc::string::String,
    /// Output only. How long the operation will be paused.
    #[prost(message, optional, tag = "2")]
    pub wait: ::core::option::Option<::prost_types::Duration>,
    /// Output only. The name of the rollout that initiates the `AutomationRun`.
    #[prost(string, tag = "3")]
    pub rollout: ::prost::alloc::string::String,
    /// Output only. The starting phase of the rollout created by this operation.
    #[prost(string, tag = "4")]
    pub phase: ::prost::alloc::string::String,
}
/// Contains the information of an automated advance-rollout operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvanceRolloutOperation {
    /// Output only. The phase of a deployment that initiated the operation.
    #[prost(string, tag = "5")]
    pub source_phase: ::prost::alloc::string::String,
    /// Output only. How long the operation will be paused.
    #[prost(message, optional, tag = "2")]
    pub wait: ::core::option::Option<::prost_types::Duration>,
    /// Output only. The name of the rollout that initiates the `AutomationRun`.
    #[prost(string, tag = "3")]
    pub rollout: ::prost::alloc::string::String,
    /// Output only. The phase the rollout will be advanced to.
    #[prost(string, tag = "4")]
    pub destination_phase: ::prost::alloc::string::String,
}
/// Contains the information for an automated `repair rollout` operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepairRolloutOperation {
    /// Output only. The name of the rollout that initiates the `AutomationRun`.
    #[prost(string, tag = "1")]
    pub rollout: ::prost::alloc::string::String,
    /// Output only. The index of the current repair action in the repair sequence.
    #[prost(int64, tag = "2")]
    pub current_repair_mode_index: i64,
    /// Output only. Records of the repair attempts. Each repair phase may have
    /// multiple retry attempts or single rollback attempt.
    #[prost(message, repeated, tag = "3")]
    pub repair_phases: ::prost::alloc::vec::Vec<RepairPhase>,
}
/// RepairPhase tracks the repair attempts that have been made for
/// each `RepairMode` specified in the `Automation` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepairPhase {
    #[prost(oneof = "repair_phase::RepairPhase", tags = "1, 2")]
    pub repair_phase: ::core::option::Option<repair_phase::RepairPhase>,
}
/// Nested message and enum types in `RepairPhase`.
pub mod repair_phase {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RepairPhase {
        /// Output only. Records of the retry attempts for retry repair mode.
        #[prost(message, tag = "1")]
        Retry(super::RetryPhase),
        /// Output only. Rollback attempt for rollback repair mode .
        #[prost(message, tag = "2")]
        Rollback(super::RollbackAttempt),
    }
}
/// RetryPhase contains the retry attempts and the metadata for initiating a
/// new attempt.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryPhase {
    /// Output only. The number of attempts that have been made.
    #[prost(int64, tag = "1")]
    pub total_attempts: i64,
    /// Output only. The pattern of how the wait time of the retry attempt is
    /// calculated.
    #[prost(enumeration = "BackoffMode", tag = "2")]
    pub backoff_mode: i32,
    /// Output only. The phase ID of the phase that includes the job being retried.
    #[prost(string, tag = "3")]
    pub phase_id: ::prost::alloc::string::String,
    /// Output only. The job ID for the Job to retry.
    #[prost(string, tag = "4")]
    pub job_id: ::prost::alloc::string::String,
    /// Output only. Detail of a retry action.
    #[prost(message, repeated, tag = "5")]
    pub attempts: ::prost::alloc::vec::Vec<RetryAttempt>,
}
/// RetryAttempt represents an action of retrying the failed Cloud Deploy job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryAttempt {
    /// Output only. The index of this retry attempt.
    #[prost(int64, tag = "1")]
    pub attempt: i64,
    /// Output only. How long the operation will be paused.
    #[prost(message, optional, tag = "2")]
    pub wait: ::core::option::Option<::prost_types::Duration>,
    /// Output only. Valid state of this retry action.
    #[prost(enumeration = "RepairState", tag = "5")]
    pub state: i32,
    /// Output only. Description of the state of the Retry.
    #[prost(string, tag = "6")]
    pub state_desc: ::prost::alloc::string::String,
}
/// RollbackAttempt represents an action of rolling back a Cloud Deploy 'Target'.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackAttempt {
    /// Output only. The phase to which the rollout will be rolled back to.
    #[prost(string, tag = "1")]
    pub destination_phase: ::prost::alloc::string::String,
    /// Output only. ID of the rollback `Rollout` to create.
    #[prost(string, tag = "2")]
    pub rollout_id: ::prost::alloc::string::String,
    /// Output only. Valid state of this rollback action.
    #[prost(enumeration = "RepairState", tag = "3")]
    pub state: i32,
    /// Output only. Description of the state of the Rollback.
    #[prost(string, tag = "4")]
    pub state_desc: ::prost::alloc::string::String,
}
/// The request object for `ListAutomationRuns`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAutomationRunsRequest {
    /// Required. The parent `Delivery Pipeline`, which owns this collection of
    /// automationRuns. Format must be
    /// `projects/{project}/locations/{location}/deliveryPipelines/{delivery_pipeline}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of automationRuns to return. The service may return
    /// fewer than this value. If unspecified, at most 50 automationRuns will
    /// be returned. The maximum value is 1000; values above 1000 will be set
    /// to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAutomationRuns` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other provided parameters match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter automationRuns to be returned. All fields can be used in the
    /// filter.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to sort by.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response object from `ListAutomationRuns`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAutomationRunsResponse {
    /// The `AutomationRuns` objects.
    #[prost(message, repeated, tag = "1")]
    pub automation_runs: ::prost::alloc::vec::Vec<AutomationRun>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request object for `GetAutomationRun`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAutomationRunRequest {
    /// Required. Name of the `AutomationRun`. Format must be
    /// `projects/{project}/locations/{location}/deliveryPipelines/{delivery_pipeline}/automationRuns/{automation_run}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request object used by `CancelAutomationRun`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelAutomationRunRequest {
    /// Required. Name of the `AutomationRun`. Format is
    /// `projects/{project}/locations/{location}/deliveryPipelines/{delivery_pipeline}/automationRuns/{automation_run}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The response object from `CancelAutomationRun`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelAutomationRunResponse {}
/// The support state of a specific Skaffold version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SkaffoldSupportState {
    /// Default value. This value is unused.
    Unspecified = 0,
    /// This Skaffold version is currently supported.
    Supported = 1,
    /// This Skaffold version is in maintenance mode.
    MaintenanceMode = 2,
    /// This Skaffold version is no longer supported.
    Unsupported = 3,
}
impl SkaffoldSupportState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SkaffoldSupportState::Unspecified => "SKAFFOLD_SUPPORT_STATE_UNSPECIFIED",
            SkaffoldSupportState::Supported => "SKAFFOLD_SUPPORT_STATE_SUPPORTED",
            SkaffoldSupportState::MaintenanceMode => {
                "SKAFFOLD_SUPPORT_STATE_MAINTENANCE_MODE"
            }
            SkaffoldSupportState::Unsupported => "SKAFFOLD_SUPPORT_STATE_UNSUPPORTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SKAFFOLD_SUPPORT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "SKAFFOLD_SUPPORT_STATE_SUPPORTED" => Some(Self::Supported),
            "SKAFFOLD_SUPPORT_STATE_MAINTENANCE_MODE" => Some(Self::MaintenanceMode),
            "SKAFFOLD_SUPPORT_STATE_UNSUPPORTED" => Some(Self::Unsupported),
            _ => None,
        }
    }
}
/// The pattern of how wait time is increased.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BackoffMode {
    /// No WaitMode is specified.
    Unspecified = 0,
    /// Increases the wait time linearly.
    Linear = 1,
    /// Increases the wait time exponentially.
    Exponential = 2,
}
impl BackoffMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BackoffMode::Unspecified => "BACKOFF_MODE_UNSPECIFIED",
            BackoffMode::Linear => "BACKOFF_MODE_LINEAR",
            BackoffMode::Exponential => "BACKOFF_MODE_EXPONENTIAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BACKOFF_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "BACKOFF_MODE_LINEAR" => Some(Self::Linear),
            "BACKOFF_MODE_EXPONENTIAL" => Some(Self::Exponential),
            _ => None,
        }
    }
}
/// Valid state of a repair attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RepairState {
    /// The `repair` has an unspecified state.
    Unspecified = 0,
    /// The `repair` action has succeeded.
    Succeeded = 1,
    /// The `repair` action was cancelled.
    Cancelled = 2,
    /// The `repair` action has failed.
    Failed = 3,
    /// The `repair` action is in progress.
    InProgress = 4,
    /// The `repair` action is pending.
    Pending = 5,
    /// The `repair` action was skipped.
    Skipped = 6,
}
impl RepairState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RepairState::Unspecified => "REPAIR_STATE_UNSPECIFIED",
            RepairState::Succeeded => "REPAIR_STATE_SUCCEEDED",
            RepairState::Cancelled => "REPAIR_STATE_CANCELLED",
            RepairState::Failed => "REPAIR_STATE_FAILED",
            RepairState::InProgress => "REPAIR_STATE_IN_PROGRESS",
            RepairState::Pending => "REPAIR_STATE_PENDING",
            RepairState::Skipped => "REPAIR_STATE_SKIPPED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REPAIR_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "REPAIR_STATE_SUCCEEDED" => Some(Self::Succeeded),
            "REPAIR_STATE_CANCELLED" => Some(Self::Cancelled),
            "REPAIR_STATE_FAILED" => Some(Self::Failed),
            "REPAIR_STATE_IN_PROGRESS" => Some(Self::InProgress),
            "REPAIR_STATE_PENDING" => Some(Self::Pending),
            "REPAIR_STATE_SKIPPED" => Some(Self::Skipped),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod cloud_deploy_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// CloudDeploy service creates and manages Continuous Delivery operations
    /// on Google Cloud Platform via Skaffold (https://skaffold.dev).
    #[derive(Debug, Clone)]
    pub struct CloudDeployClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudDeployClient<T>
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
        ) -> CloudDeployClient<InterceptedService<T, F>>
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
            CloudDeployClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists DeliveryPipelines in a given project and location.
        pub async fn list_delivery_pipelines(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeliveryPipelinesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDeliveryPipelinesResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/ListDeliveryPipelines",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "ListDeliveryPipelines",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single DeliveryPipeline.
        pub async fn get_delivery_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeliveryPipelineRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeliveryPipeline>,
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
                "/google.cloud.deploy.v1.CloudDeploy/GetDeliveryPipeline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "GetDeliveryPipeline",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new DeliveryPipeline in a given project and location.
        pub async fn create_delivery_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDeliveryPipelineRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/CreateDeliveryPipeline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "CreateDeliveryPipeline",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single DeliveryPipeline.
        pub async fn update_delivery_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDeliveryPipelineRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/UpdateDeliveryPipeline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "UpdateDeliveryPipeline",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single DeliveryPipeline.
        pub async fn delete_delivery_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDeliveryPipelineRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/DeleteDeliveryPipeline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "DeleteDeliveryPipeline",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Targets in a given project and location.
        pub async fn list_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTargetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTargetsResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/ListTargets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "ListTargets"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a `Rollout` to roll back the specified target.
        pub async fn rollback_target(
            &mut self,
            request: impl tonic::IntoRequest<super::RollbackTargetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RollbackTargetResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/RollbackTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "RollbackTarget",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Target.
        pub async fn get_target(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTargetRequest>,
        ) -> std::result::Result<tonic::Response<super::Target>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/GetTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "GetTarget"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Target in a given project and location.
        pub async fn create_target(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTargetRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/CreateTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "CreateTarget"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single Target.
        pub async fn update_target(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTargetRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/UpdateTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "UpdateTarget"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Target.
        pub async fn delete_target(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTargetRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/DeleteTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "DeleteTarget"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists CustomTargetTypes in a given project and location.
        pub async fn list_custom_target_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomTargetTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCustomTargetTypesResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/ListCustomTargetTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "ListCustomTargetTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single CustomTargetType.
        pub async fn get_custom_target_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomTargetTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CustomTargetType>,
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
                "/google.cloud.deploy.v1.CloudDeploy/GetCustomTargetType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "GetCustomTargetType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new CustomTargetType in a given project and location.
        pub async fn create_custom_target_type(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomTargetTypeRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/CreateCustomTargetType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "CreateCustomTargetType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a single CustomTargetType.
        pub async fn update_custom_target_type(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomTargetTypeRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/UpdateCustomTargetType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "UpdateCustomTargetType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single CustomTargetType.
        pub async fn delete_custom_target_type(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCustomTargetTypeRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/DeleteCustomTargetType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "DeleteCustomTargetType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Releases in a given project and location.
        pub async fn list_releases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReleasesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListReleasesResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/ListReleases",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "ListReleases"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Release.
        pub async fn get_release(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReleaseRequest>,
        ) -> std::result::Result<tonic::Response<super::Release>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/GetRelease",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "GetRelease"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Release in a given project and location.
        pub async fn create_release(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReleaseRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/CreateRelease",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "CreateRelease",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Abandons a Release in the Delivery Pipeline.
        pub async fn abandon_release(
            &mut self,
            request: impl tonic::IntoRequest<super::AbandonReleaseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AbandonReleaseResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/AbandonRelease",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "AbandonRelease",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Approves a Rollout.
        pub async fn approve_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::ApproveRolloutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ApproveRolloutResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/ApproveRollout",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "ApproveRollout",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Advances a Rollout in a given project and location.
        pub async fn advance_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::AdvanceRolloutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AdvanceRolloutResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/AdvanceRollout",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "AdvanceRollout",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Cancels a Rollout in a given project and location.
        pub async fn cancel_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelRolloutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelRolloutResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/CancelRollout",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "CancelRollout",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Rollouts in a given project and location.
        pub async fn list_rollouts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRolloutsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRolloutsResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/ListRollouts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "ListRollouts"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Rollout.
        pub async fn get_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRolloutRequest>,
        ) -> std::result::Result<tonic::Response<super::Rollout>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/GetRollout",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "GetRollout"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Rollout in a given project and location.
        pub async fn create_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRolloutRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/CreateRollout",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "CreateRollout",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Ignores the specified Job in a Rollout.
        pub async fn ignore_job(
            &mut self,
            request: impl tonic::IntoRequest<super::IgnoreJobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IgnoreJobResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/IgnoreJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "IgnoreJob"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retries the specified Job in a Rollout.
        pub async fn retry_job(
            &mut self,
            request: impl tonic::IntoRequest<super::RetryJobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetryJobResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/RetryJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "RetryJob"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists JobRuns in a given project and location.
        pub async fn list_job_runs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobRunsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListJobRunsResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/ListJobRuns",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "ListJobRuns"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single JobRun.
        pub async fn get_job_run(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobRunRequest>,
        ) -> std::result::Result<tonic::Response<super::JobRun>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/GetJobRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "GetJobRun"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Terminates a Job Run in a given project and location.
        pub async fn terminate_job_run(
            &mut self,
            request: impl tonic::IntoRequest<super::TerminateJobRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TerminateJobRunResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/TerminateJobRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "TerminateJobRun",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the configuration for a location.
        pub async fn get_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::Config>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/GetConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.deploy.v1.CloudDeploy", "GetConfig"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Automation in a given project and location.
        pub async fn create_automation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAutomationRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/CreateAutomation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "CreateAutomation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single Automation resource.
        pub async fn update_automation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAutomationRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/UpdateAutomation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "UpdateAutomation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Automation resource.
        pub async fn delete_automation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAutomationRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/DeleteAutomation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "DeleteAutomation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Automation.
        pub async fn get_automation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAutomationRequest>,
        ) -> std::result::Result<tonic::Response<super::Automation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/GetAutomation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "GetAutomation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Automations in a given project and location.
        pub async fn list_automations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAutomationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAutomationsResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/ListAutomations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "ListAutomations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single AutomationRun.
        pub async fn get_automation_run(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAutomationRunRequest>,
        ) -> std::result::Result<tonic::Response<super::AutomationRun>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/GetAutomationRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "GetAutomationRun",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists AutomationRuns in a given project and location.
        pub async fn list_automation_runs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAutomationRunsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAutomationRunsResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/ListAutomationRuns",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "ListAutomationRuns",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Cancels an AutomationRun. The `state` of the `AutomationRun` after
        /// cancelling is `CANCELLED`. `CancelAutomationRun` can be called on
        /// AutomationRun in the state `IN_PROGRESS` and `PENDING`; AutomationRun
        /// in a different state returns an `FAILED_PRECONDITION` error.
        pub async fn cancel_automation_run(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelAutomationRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelAutomationRunResponse>,
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
                "/google.cloud.deploy.v1.CloudDeploy/CancelAutomationRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.deploy.v1.CloudDeploy",
                        "CancelAutomationRun",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Payload proto for "clouddeploy.googleapis.com/rollout_notification"
/// Platform Log event that describes the failure to send rollout status change
/// Pub/Sub notification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RolloutNotificationEvent {
    /// Debug message for when a notification fails to send.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// Unique identifier of the `DeliveryPipeline`.
    #[prost(string, tag = "2")]
    pub pipeline_uid: ::prost::alloc::string::String,
    /// Unique identifier of the `Release`.
    #[prost(string, tag = "3")]
    pub release_uid: ::prost::alloc::string::String,
    /// The name of the `Release`.
    #[prost(string, tag = "7")]
    pub release: ::prost::alloc::string::String,
    /// Unique identifier of the `Rollout`.
    #[prost(string, tag = "8")]
    pub rollout_uid: ::prost::alloc::string::String,
    /// The name of the `Rollout`.
    #[prost(string, tag = "4")]
    pub rollout: ::prost::alloc::string::String,
    /// ID of the `Target` that the rollout is deployed to.
    #[prost(string, tag = "6")]
    pub target_id: ::prost::alloc::string::String,
    /// Type of this notification, e.g. for a Pub/Sub failure.
    #[prost(enumeration = "Type", tag = "5")]
    pub r#type: i32,
}
/// Payload proto for "clouddeploy.googleapis.com/release_notification"
/// Platform Log event that describes the failure to send release status change
/// Pub/Sub notification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseNotificationEvent {
    /// Debug message for when a notification fails to send.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// Unique identifier of the `DeliveryPipeline`.
    #[prost(string, tag = "4")]
    pub pipeline_uid: ::prost::alloc::string::String,
    /// Unique identifier of the `Release`.
    #[prost(string, tag = "5")]
    pub release_uid: ::prost::alloc::string::String,
    /// The name of the `Release`.
    #[prost(string, tag = "2")]
    pub release: ::prost::alloc::string::String,
    /// Type of this notification, e.g. for a Pub/Sub failure.
    #[prost(enumeration = "Type", tag = "3")]
    pub r#type: i32,
}
/// Payload proto for "clouddeploy.googleapis.com/jobrun_notification"
/// Platform Log event that describes the failure to send JobRun resource update
/// Pub/Sub notification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobRunNotificationEvent {
    /// Debug message for when a notification fails to send.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// The name of the `JobRun`.
    #[prost(string, tag = "2")]
    pub job_run: ::prost::alloc::string::String,
    /// Unique identifier of the `DeliveryPipeline`.
    #[prost(string, tag = "3")]
    pub pipeline_uid: ::prost::alloc::string::String,
    /// Unique identifier of the `Release`.
    #[prost(string, tag = "4")]
    pub release_uid: ::prost::alloc::string::String,
    /// The name of the `Release`.
    #[prost(string, tag = "8")]
    pub release: ::prost::alloc::string::String,
    /// Unique identifier of the `Rollout`.
    #[prost(string, tag = "5")]
    pub rollout_uid: ::prost::alloc::string::String,
    /// The name of the `Rollout`.
    #[prost(string, tag = "9")]
    pub rollout: ::prost::alloc::string::String,
    /// ID of the `Target`.
    #[prost(string, tag = "6")]
    pub target_id: ::prost::alloc::string::String,
    /// Type of this notification, e.g. for a Pub/Sub failure.
    #[prost(enumeration = "Type", tag = "7")]
    pub r#type: i32,
}
/// Payload proto for "clouddeploy.googleapis.com/release_render"
/// Platform Log event that describes the render status change.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseRenderEvent {
    /// Debug message for when a render transition occurs. Provides further
    /// details as rendering progresses through render states.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// Unique identifier of the `DeliveryPipeline`.
    #[prost(string, tag = "4")]
    pub pipeline_uid: ::prost::alloc::string::String,
    /// The name of the release.
    /// release_uid is not in this log message because we write some of these log
    /// messages at release creation time, before we've generated the uid.
    #[prost(string, tag = "2")]
    pub release: ::prost::alloc::string::String,
    /// Type of this notification, e.g. for a release render state change event.
    #[prost(enumeration = "Type", tag = "5")]
    pub r#type: i32,
    /// The state of the release render.
    #[prost(enumeration = "release::RenderState", tag = "3")]
    pub release_render_state: i32,
}
/// Payload proto for "clouddeploy.googleapis.com/target_notification"
/// Platform Log event that describes the failure to send target status change
/// Pub/Sub notification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetNotificationEvent {
    /// Debug message for when a notification fails to send.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// The name of the `Target`.
    #[prost(string, tag = "2")]
    pub target: ::prost::alloc::string::String,
    /// Type of this notification, e.g. for a Pub/Sub failure.
    #[prost(enumeration = "Type", tag = "3")]
    pub r#type: i32,
}
/// Payload proto for "clouddeploy.googleapis.com/rollout_update"
/// Platform Log event that describes the rollout update event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RolloutUpdateEvent {
    /// Debug message for when a rollout update event occurs.
    #[prost(string, tag = "6")]
    pub message: ::prost::alloc::string::String,
    /// Unique identifier of the pipeline.
    #[prost(string, tag = "1")]
    pub pipeline_uid: ::prost::alloc::string::String,
    /// Unique identifier of the release.
    #[prost(string, tag = "2")]
    pub release_uid: ::prost::alloc::string::String,
    /// The name of the `Release`.
    #[prost(string, tag = "8")]
    pub release: ::prost::alloc::string::String,
    /// The name of the rollout.
    /// rollout_uid is not in this log message because we write some of these log
    /// messages at rollout creation time, before we've generated the uid.
    #[prost(string, tag = "3")]
    pub rollout: ::prost::alloc::string::String,
    /// ID of the target.
    #[prost(string, tag = "4")]
    pub target_id: ::prost::alloc::string::String,
    /// Type of this notification, e.g. for a rollout update event.
    #[prost(enumeration = "Type", tag = "7")]
    pub r#type: i32,
    /// The type of the rollout update.
    #[prost(enumeration = "rollout_update_event::RolloutUpdateType", tag = "5")]
    pub rollout_update_type: i32,
}
/// Nested message and enum types in `RolloutUpdateEvent`.
pub mod rollout_update_event {
    /// RolloutUpdateType indicates the type of the rollout update.
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
    pub enum RolloutUpdateType {
        /// Rollout update type unspecified.
        Unspecified = 0,
        /// rollout state updated to pending.
        Pending = 1,
        /// Rollout state updated to pending release.
        PendingRelease = 2,
        /// Rollout state updated to in progress.
        InProgress = 3,
        /// Rollout state updated to cancelling.
        Cancelling = 4,
        /// Rollout state updated to cancelled.
        Cancelled = 5,
        /// Rollout state updated to halted.
        Halted = 6,
        /// Rollout state updated to succeeded.
        Succeeded = 7,
        /// Rollout state updated to failed.
        Failed = 8,
        /// Rollout requires approval.
        ApprovalRequired = 9,
        /// Rollout has been approved.
        Approved = 10,
        /// Rollout has been rejected.
        Rejected = 11,
        /// Rollout requires advance to the next phase.
        AdvanceRequired = 12,
        /// Rollout has been advanced.
        Advanced = 13,
    }
    impl RolloutUpdateType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RolloutUpdateType::Unspecified => "ROLLOUT_UPDATE_TYPE_UNSPECIFIED",
                RolloutUpdateType::Pending => "PENDING",
                RolloutUpdateType::PendingRelease => "PENDING_RELEASE",
                RolloutUpdateType::InProgress => "IN_PROGRESS",
                RolloutUpdateType::Cancelling => "CANCELLING",
                RolloutUpdateType::Cancelled => "CANCELLED",
                RolloutUpdateType::Halted => "HALTED",
                RolloutUpdateType::Succeeded => "SUCCEEDED",
                RolloutUpdateType::Failed => "FAILED",
                RolloutUpdateType::ApprovalRequired => "APPROVAL_REQUIRED",
                RolloutUpdateType::Approved => "APPROVED",
                RolloutUpdateType::Rejected => "REJECTED",
                RolloutUpdateType::AdvanceRequired => "ADVANCE_REQUIRED",
                RolloutUpdateType::Advanced => "ADVANCED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROLLOUT_UPDATE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "PENDING_RELEASE" => Some(Self::PendingRelease),
                "IN_PROGRESS" => Some(Self::InProgress),
                "CANCELLING" => Some(Self::Cancelling),
                "CANCELLED" => Some(Self::Cancelled),
                "HALTED" => Some(Self::Halted),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "APPROVAL_REQUIRED" => Some(Self::ApprovalRequired),
                "APPROVED" => Some(Self::Approved),
                "REJECTED" => Some(Self::Rejected),
                "ADVANCE_REQUIRED" => Some(Self::AdvanceRequired),
                "ADVANCED" => Some(Self::Advanced),
                _ => None,
            }
        }
    }
}
/// Payload proto for "clouddeploy.googleapis.com/deliverypipeline_notification"
/// Platform Log event that describes the failure to send delivery pipeline
/// status change Pub/Sub notification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliveryPipelineNotificationEvent {
    /// Debug message for when a notification fails to send.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// Unique identifier of the `DeliveryPipeline`.
    #[prost(string, tag = "4")]
    pub pipeline_uid: ::prost::alloc::string::String,
    /// The name of the `Delivery Pipeline`.
    #[prost(string, tag = "2")]
    pub delivery_pipeline: ::prost::alloc::string::String,
    /// Type of this notification, e.g. for a Pub/Sub failure.
    #[prost(enumeration = "Type", tag = "3")]
    pub r#type: i32,
}
