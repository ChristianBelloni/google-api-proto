/// Describes the state of a job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobState {
    /// The job state is unspecified.
    Unspecified = 0,
    /// The service is preparing to run the job.
    Pending = 9,
    /// The job is in progress.
    Running = 1,
    /// The job completed successfully.
    Succeeded = 2,
    /// The job failed.
    Failed = 3,
    /// The job has been cancelled.
    Cancelled = 4,
    /// Entity Recon API: The knowledge extraction job is running.
    KnowledgeExtraction = 5,
    /// Entity Recon API: The preprocessing job is running.
    ReconPreprocessing = 6,
    /// Entity Recon API: The clustering job is running.
    Clustering = 7,
    /// Entity Recon API: The exporting clusters job is running.
    ExportingClusters = 8,
}
impl JobState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JobState::Unspecified => "JOB_STATE_UNSPECIFIED",
            JobState::Pending => "JOB_STATE_PENDING",
            JobState::Running => "JOB_STATE_RUNNING",
            JobState::Succeeded => "JOB_STATE_SUCCEEDED",
            JobState::Failed => "JOB_STATE_FAILED",
            JobState::Cancelled => "JOB_STATE_CANCELLED",
            JobState::KnowledgeExtraction => "JOB_STATE_KNOWLEDGE_EXTRACTION",
            JobState::ReconPreprocessing => "JOB_STATE_RECON_PREPROCESSING",
            JobState::Clustering => "JOB_STATE_CLUSTERING",
            JobState::ExportingClusters => "JOB_STATE_EXPORTING_CLUSTERS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "JOB_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "JOB_STATE_PENDING" => Some(Self::Pending),
            "JOB_STATE_RUNNING" => Some(Self::Running),
            "JOB_STATE_SUCCEEDED" => Some(Self::Succeeded),
            "JOB_STATE_FAILED" => Some(Self::Failed),
            "JOB_STATE_CANCELLED" => Some(Self::Cancelled),
            "JOB_STATE_KNOWLEDGE_EXTRACTION" => Some(Self::KnowledgeExtraction),
            "JOB_STATE_RECON_PREPROCESSING" => Some(Self::ReconPreprocessing),
            "JOB_STATE_CLUSTERING" => Some(Self::Clustering),
            "JOB_STATE_EXPORTING_CLUSTERS" => Some(Self::ExportingClusters),
            _ => None,
        }
    }
}
/// The common metadata for long running operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonOperationMetadata {
    /// The state of the operation.
    #[prost(enumeration = "common_operation_metadata::State", tag = "1")]
    pub state: i32,
    /// The creation time of the operation.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The last update time of the operation.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `CommonOperationMetadata`.
pub mod common_operation_metadata {
    /// State of the longrunning operation.
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
        /// Operation is still running.
        Running = 1,
        /// Operation is being cancelled.
        Cancelling = 2,
        /// Operation succeeded.
        Succeeded = 3,
        /// Operation failed.
        Failed = 4,
        /// Operation is cancelled.
        Cancelled = 5,
        /// Operation is pending not running yet.
        Pending = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Running => "RUNNING",
                State::Cancelling => "CANCELLING",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::Cancelled => "CANCELLED",
                State::Pending => "PENDING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "RUNNING" => Some(Self::Running),
                "CANCELLING" => Some(Self::Cancelling),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "CANCELLED" => Some(Self::Cancelled),
                "PENDING" => Some(Self::Pending),
                _ => None,
            }
        }
    }
}
/// The desired input location and metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputConfig {
    /// Set of input BigQuery tables.
    #[prost(message, repeated, tag = "1")]
    pub bigquery_input_configs: ::prost::alloc::vec::Vec<BigQueryInputConfig>,
    /// Entity type
    #[prost(enumeration = "input_config::EntityType", tag = "2")]
    pub entity_type: i32,
    /// Optional. Provide the bigquery table containing the previous results if
    /// cluster ID stability is desired. Format is
    /// `projects/*/datasets/*/tables/*`.
    #[prost(string, tag = "3")]
    pub previous_result_bigquery_table: ::prost::alloc::string::String,
}
/// Nested message and enum types in `InputConfig`.
pub mod input_config {
    /// The type of entities we will support. Currently, we only support people,
    /// establishment, property, and product types. If the type is
    /// unspecified, it will be generic type.
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
    pub enum EntityType {
        /// The default value.
        Unspecified = 0,
        /// People entity.
        People = 1,
        /// Establishment entity.
        Establishment = 2,
        /// Property entity. e.g. real estate property.
        Property = 3,
        /// Product entity.
        Product = 4,
        /// Organization entity.
        Organization = 5,
        /// Local Business entity.
        LocalBusiness = 6,
        /// Person entity.
        Person = 7,
    }
    impl EntityType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EntityType::Unspecified => "ENTITY_TYPE_UNSPECIFIED",
                EntityType::People => "PEOPLE",
                EntityType::Establishment => "ESTABLISHMENT",
                EntityType::Property => "PROPERTY",
                EntityType::Product => "PRODUCT",
                EntityType::Organization => "ORGANIZATION",
                EntityType::LocalBusiness => "LOCAL_BUSINESS",
                EntityType::Person => "PERSON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENTITY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PEOPLE" => Some(Self::People),
                "ESTABLISHMENT" => Some(Self::Establishment),
                "PROPERTY" => Some(Self::Property),
                "PRODUCT" => Some(Self::Product),
                "ORGANIZATION" => Some(Self::Organization),
                "LOCAL_BUSINESS" => Some(Self::LocalBusiness),
                "PERSON" => Some(Self::Person),
                _ => None,
            }
        }
    }
}
/// The input config for BigQuery tables.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryInputConfig {
    /// Required. Format is `projects/*/datasets/*/tables/*`.
    #[prost(string, tag = "1")]
    pub bigquery_table: ::prost::alloc::string::String,
    /// Required. Schema mapping file
    #[prost(string, tag = "2")]
    pub gcs_uri: ::prost::alloc::string::String,
}
/// The desired output location and metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// Format is “projects/*/datasets/*”.
    #[prost(string, tag = "1")]
    pub bigquery_dataset: ::prost::alloc::string::String,
}
/// Recon configs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconConfig {
    /// Extra options that affect entity clustering behavior.
    #[prost(message, optional, tag = "3")]
    pub options: ::core::option::Option<recon_config::Options>,
    /// Model Configs
    #[prost(message, optional, tag = "4")]
    pub model_config: ::core::option::Option<recon_config::ModelConfig>,
    /// Choice of clustering algorithm. Default is ConnectedComponentsConfig.
    #[prost(oneof = "recon_config::ClusteringConfig", tags = "1, 2")]
    pub clustering_config: ::core::option::Option<recon_config::ClusteringConfig>,
}
/// Nested message and enum types in `ReconConfig`.
pub mod recon_config {
    /// Options for experimental changes on entity clustering behavior.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        /// If true, separate clusters by their geographic region (from geocoding).
        /// Uses the following entity features:
        ///
        /// - schema.org/addressLocality
        /// - schema.org/addressRegion
        /// - schema.org/addressCountry
        /// Warning: processing will no longer be regionalized!
        #[prost(bool, tag = "100")]
        pub enable_geocoding_separation: bool,
    }
    /// Model Configs
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ModelConfig {
        /// Model name. Refer to external documentation for valid names.
        /// If unspecified, it defaults to the one mentioned in the documentation.
        #[prost(string, tag = "1")]
        pub model_name: ::prost::alloc::string::String,
        /// Model version tag. Refer to external documentation for valid tags.
        /// If unspecified, it defaults to the one mentioned in the documentation.
        #[prost(string, tag = "2")]
        pub version_tag: ::prost::alloc::string::String,
    }
    /// Choice of clustering algorithm. Default is ConnectedComponentsConfig.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusteringConfig {
        /// Configs for connected components.
        #[prost(message, tag = "1")]
        ConnectedComponentsConfig(super::ConnectedComponentsConfig),
        /// Configs for affinity clustering.
        #[prost(message, tag = "2")]
        AffinityClusteringConfig(super::AffinityClusteringConfig),
    }
}
/// Options for connected components.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectedComponentsConfig {
    /// Threshold used for connected components. Default value is 0.85.
    #[prost(float, tag = "1")]
    pub weight_threshold: f32,
}
/// Options for affinity clustering.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffinityClusteringConfig {
    /// Number of iterations to perform. Default value is 1.
    #[prost(int64, tag = "1")]
    pub compression_round_count: i64,
}
/// Details of operations that perform deletes of any entities.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOperationMetadata {
    /// The common part of the operation metadata.
    #[prost(message, optional, tag = "1")]
    pub common_metadata: ::core::option::Option<CommonOperationMetadata>,
}
/// Request message for CreateEntityReconciliationJob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntityReconciliationJobRequest {
    /// Required. The resource name of the Location to create the
    /// EntityReconciliationJob in. Format:
    /// `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The EntityReconciliationJob to create.
    #[prost(message, optional, tag = "2")]
    pub entity_reconciliation_job: ::core::option::Option<EntityReconciliationJob>,
}
/// Request message for GetEntityReconciliationJob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntityReconciliationJobRequest {
    /// Required. The name of the EntityReconciliationJob resource.
    /// Format:
    /// `projects/{project}/locations/{location}/entityReconciliationJobs/{entity_reconciliation_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [EnterpriseKnowledgeGraphService.ListEntityReconciliationJobs][google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService.ListEntityReconciliationJobs].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityReconciliationJobsRequest {
    /// Required. The name of the EntityReconciliationJob's parent resource.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// An expression for filtering the results of the request. For field names
    /// both snake_case and camelCase are supported.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [EnterpriseKnowledgeGraphService.ListEntityReconciliationJobs][google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService.ListEntityReconciliationJobs].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityReconciliationJobsResponse {
    /// A list of EntityReconciliationJobs that matches the specified filter in the
    /// request.
    #[prost(message, repeated, tag = "1")]
    pub entity_reconciliation_jobs: ::prost::alloc::vec::Vec<EntityReconciliationJob>,
    /// The standard List next-page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CancelEntityReconciliationJob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelEntityReconciliationJobRequest {
    /// Required. The name of the EntityReconciliationJob resource.
    /// Format:
    /// `projects/{project}/locations/{location}/entityReconciliationJobs/{entity_reconciliation_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteEntityReconciliationJob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntityReconciliationJobRequest {
    /// Required. The name of the EntityReconciliationJob resource.
    /// Format:
    /// `projects/{project}/locations/{location}/entityReconciliationJobs/{entity_reconciliation_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Entity reconciliation job message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityReconciliationJob {
    /// Output only. Resource name of the EntityReconciliationJob.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Information about the input BigQuery tables.
    #[prost(message, optional, tag = "2")]
    pub input_config: ::core::option::Option<InputConfig>,
    /// Required. The desired output location.
    #[prost(message, optional, tag = "3")]
    pub output_config: ::core::option::Option<OutputConfig>,
    /// Output only. The detailed state of the job.
    #[prost(enumeration = "JobState", tag = "4")]
    pub state: i32,
    /// Output only. Only populated when the job's state is JOB_STATE_FAILED or
    /// JOB_STATE_CANCELLED.
    #[prost(message, optional, tag = "5")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Output only. Time when the EntityReconciliationJob was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the EntityReconciliationJob entered any of the
    /// following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`,
    /// `JOB_STATE_CANCELLED`.
    #[prost(message, optional, tag = "7")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the EntityReconciliationJob was most recently
    /// updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Recon configs to adjust the clustering behavior.
    #[prost(message, optional, tag = "9")]
    pub recon_config: ::core::option::Option<ReconConfig>,
}
/// Request message for
/// [EnterpriseKnowledgeGraphService.Lookup][google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService.Lookup].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupRequest {
    /// Required. The name of the Entity's parent resource.
    /// Format:
    /// `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The list of entity ids to be used for lookup.
    #[prost(string, repeated, tag = "2")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of language codes (defined in ISO 693) to run the query with,
    /// e.g. 'en'.
    #[prost(string, repeated, tag = "3")]
    pub languages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response message for
/// [EnterpriseKnowledgeGraphService.Lookup][google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService.Lookup].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupResponse {
    /// The local context applicable for the response. See more details at
    /// <http://www.w3.org/TR/json-ld/#context-definitions.>
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<::prost_types::Value>,
    /// The schema type of top-level JSON-LD object, e.g. ItemList.
    #[prost(message, optional, tag = "2")]
    pub r#type: ::core::option::Option<::prost_types::Value>,
    /// The item list of search results.
    #[prost(message, optional, tag = "3")]
    pub item_list_element: ::core::option::Option<::prost_types::ListValue>,
}
/// Request message for
/// [EnterpriseKnowledgeGraphService.Search][google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService.Search].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRequest {
    /// Required. The name of the Entity's parent resource.
    /// Format:
    /// `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The literal query string for search.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// The list of language codes (defined in ISO 693) to run the query with,
    /// e.g. 'en'.
    #[prost(string, repeated, tag = "3")]
    pub languages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Restricts returned entities with these types, e.g. Person
    /// (as defined in <http://schema.org/Person>). If multiple types are specified,
    /// returned entities will contain one or more of these types.
    #[prost(string, repeated, tag = "4")]
    pub types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Limits the number of entities to be returned.
    #[prost(message, optional, tag = "6")]
    pub limit: ::core::option::Option<i32>,
}
/// Response message for
/// [EnterpriseKnowledgeGraphService.Search][google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService.Search].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResponse {
    /// The local context applicable for the response. See more details at
    /// <http://www.w3.org/TR/json-ld/#context-definitions.>
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<::prost_types::Value>,
    /// The schema type of top-level JSON-LD object, e.g. ItemList.
    #[prost(message, optional, tag = "2")]
    pub r#type: ::core::option::Option<::prost_types::Value>,
    /// The item list of search results.
    #[prost(message, optional, tag = "3")]
    pub item_list_element: ::core::option::Option<::prost_types::ListValue>,
}
/// Request message for
/// [EnterpriseKnowledgeGraphService.LookupPublicKg][google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService.LookupPublicKg].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupPublicKgRequest {
    /// Required. The name of the Entity's parent resource.
    /// Format:
    /// `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The list of entity ids to be used for lookup.
    #[prost(string, repeated, tag = "2")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of language codes (defined in ISO 693) to run the query with,
    /// e.g. 'en'.
    #[prost(string, repeated, tag = "3")]
    pub languages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response message for
/// [EnterpriseKnowledgeGraphService.LookupPublicKg][google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService.LookupPublicKg].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupPublicKgResponse {
    /// The local context applicable for the response. See more details at
    /// <http://www.w3.org/TR/json-ld/#context-definitions.>
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<::prost_types::Value>,
    /// The schema type of top-level JSON-LD object, e.g. ItemList.
    #[prost(message, optional, tag = "2")]
    pub r#type: ::core::option::Option<::prost_types::Value>,
    /// The item list of search results.
    #[prost(message, optional, tag = "3")]
    pub item_list_element: ::core::option::Option<::prost_types::ListValue>,
}
/// Request message for
/// [EnterpriseKnowledgeGraphService.Search][google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService.Search].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchPublicKgRequest {
    /// Required. The name of the Entity's parent resource.
    /// Format:
    /// `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The literal query string for search.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// The list of language codes (defined in ISO 693) to run the query with,
    /// e.g. 'en'.
    #[prost(string, repeated, tag = "3")]
    pub languages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Restricts returned entities with these types, e.g. Person
    /// (as defined in <http://schema.org/Person>). If multiple types are specified,
    /// returned entities will contain one or more of these types.
    #[prost(string, repeated, tag = "4")]
    pub types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Limits the number of entities to be returned.
    #[prost(message, optional, tag = "6")]
    pub limit: ::core::option::Option<i32>,
}
/// Response message for
/// [EnterpriseKnowledgeGraphService.Search][google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService.Search].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchPublicKgResponse {
    /// The local context applicable for the response. See more details at
    /// <http://www.w3.org/TR/json-ld/#context-definitions.>
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<::prost_types::Value>,
    /// The schema type of top-level JSON-LD object, e.g. ItemList.
    #[prost(message, optional, tag = "2")]
    pub r#type: ::core::option::Option<::prost_types::Value>,
    /// The item list of search results.
    #[prost(message, optional, tag = "3")]
    pub item_list_element: ::core::option::Option<::prost_types::ListValue>,
}
/// Generated client implementations.
pub mod enterprise_knowledge_graph_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// APIs for enterprise knowledge graph product.
    #[derive(Debug, Clone)]
    pub struct EnterpriseKnowledgeGraphServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EnterpriseKnowledgeGraphServiceClient<T>
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
        ) -> EnterpriseKnowledgeGraphServiceClient<InterceptedService<T, F>>
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
            EnterpriseKnowledgeGraphServiceClient::new(
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
        /// Creates a EntityReconciliationJob. A EntityReconciliationJob once created
        /// will right away be attempted to start.
        pub async fn create_entity_reconciliation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntityReconciliationJobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EntityReconciliationJob>,
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
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/CreateEntityReconciliationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService",
                        "CreateEntityReconciliationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a EntityReconciliationJob.
        pub async fn get_entity_reconciliation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntityReconciliationJobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EntityReconciliationJob>,
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
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/GetEntityReconciliationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService",
                        "GetEntityReconciliationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Entity Reconciliation Jobs.
        pub async fn list_entity_reconciliation_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntityReconciliationJobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEntityReconciliationJobsResponse>,
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
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/ListEntityReconciliationJobs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService",
                        "ListEntityReconciliationJobs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Cancels a EntityReconciliationJob. Success of cancellation is not
        /// guaranteed.
        pub async fn cancel_entity_reconciliation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelEntityReconciliationJobRequest>,
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
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/CancelEntityReconciliationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService",
                        "CancelEntityReconciliationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a EntityReconciliationJob.
        /// It only deletes the job when the job state is in FAILED, SUCCEEDED, and
        /// CANCELLED.
        pub async fn delete_entity_reconciliation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntityReconciliationJobRequest>,
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
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/DeleteEntityReconciliationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService",
                        "DeleteEntityReconciliationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Finds the Cloud KG entities with CKG ID(s).
        pub async fn lookup(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupRequest>,
        ) -> std::result::Result<tonic::Response<super::LookupResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/Lookup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService",
                        "Lookup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Searches the Cloud KG entities with entity name.
        pub async fn search(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRequest>,
        ) -> std::result::Result<tonic::Response<super::SearchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/Search",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService",
                        "Search",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Finds the public KG entities with public KG ID(s).
        pub async fn lookup_public_kg(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupPublicKgRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LookupPublicKgResponse>,
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
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/LookupPublicKg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService",
                        "LookupPublicKg",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Searches the public KG entities with entity name.
        pub async fn search_public_kg(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchPublicKgRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchPublicKgResponse>,
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
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/SearchPublicKg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService",
                        "SearchPublicKg",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
