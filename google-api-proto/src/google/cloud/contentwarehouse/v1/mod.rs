/// Meta information is used to improve the performance of the service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMetadata {
    /// Provides user unique identification and groups information.
    #[prost(message, optional, tag = "1")]
    pub user_info: ::core::option::Option<UserInfo>,
}
/// Additional information returned to client, such as debugging information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMetadata {
    /// A unique id associated with this call. This id is logged for tracking
    /// purpose.
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
}
/// The user information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// A unique user identification string, as determined by the client.
    /// The maximum number of allowed characters is 255.
    /// Allowed characters include numbers 0 to 9, uppercase and lowercase letters,
    /// and restricted special symbols (:, @, +, -, _, ~)
    /// The format is "user:xxxx@example.com";
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The unique group identifications which the user is belong to.
    /// The format is "group:yyyy@example.com";
    #[prost(string, repeated, tag = "2")]
    pub group_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Options for Update operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOptions {
    /// Type for update.
    #[prost(enumeration = "UpdateType", tag = "1")]
    pub update_type: i32,
    /// Field mask for merging Document fields.
    /// For the `FieldMask` definition,
    /// see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Options for merging.
    #[prost(message, optional, tag = "3")]
    pub merge_fields_options: ::core::option::Option<MergeFieldsOptions>,
}
/// Options for merging updated fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeFieldsOptions {
    /// When merging message fields, the default behavior is to merge
    /// the content of two message fields together. If you instead want to use
    /// the field from the source message to replace the corresponding field in
    /// the destination message, set this flag to true. When this flag is set,
    /// specified submessage fields that are missing in source will be cleared in
    /// destination.
    #[prost(bool, optional, tag = "1")]
    pub replace_message_fields: ::core::option::Option<bool>,
    /// When merging repeated fields, the default behavior is to append
    /// entries from the source repeated field to the destination repeated field.
    /// If you instead want to keep only the entries from the source repeated
    /// field, set this flag to true.
    ///
    /// If you want to replace a repeated field within a message field on the
    /// destination message, you must set both replace_repeated_fields and
    /// replace_message_fields to true, otherwise the repeated fields will be
    /// appended.
    #[prost(bool, optional, tag = "2")]
    pub replace_repeated_fields: ::core::option::Option<bool>,
}
/// Update type of the requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpdateType {
    /// Defaults to full replace behavior, ie. FULL_REPLACE.
    Unspecified = 0,
    /// Fully replace all the fields (including previously linked raw document).
    /// Any field masks will be ignored.
    Replace = 1,
    /// Merge the fields into the existing entities.
    Merge = 2,
    /// Inserts the properties by names.
    InsertPropertiesByNames = 3,
    /// Replace the properties by names.
    ReplacePropertiesByNames = 4,
    /// Delete the properties by names.
    DeletePropertiesByNames = 5,
    /// For each of the property, replaces the property if the it exists, otherwise
    /// inserts a new property. And for the rest of the fields, merge them based on
    /// update mask and merge fields options.
    MergeAndReplaceOrInsertPropertiesByNames = 6,
}
impl UpdateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpdateType::Unspecified => "UPDATE_TYPE_UNSPECIFIED",
            UpdateType::Replace => "UPDATE_TYPE_REPLACE",
            UpdateType::Merge => "UPDATE_TYPE_MERGE",
            UpdateType::InsertPropertiesByNames => {
                "UPDATE_TYPE_INSERT_PROPERTIES_BY_NAMES"
            }
            UpdateType::ReplacePropertiesByNames => {
                "UPDATE_TYPE_REPLACE_PROPERTIES_BY_NAMES"
            }
            UpdateType::DeletePropertiesByNames => {
                "UPDATE_TYPE_DELETE_PROPERTIES_BY_NAMES"
            }
            UpdateType::MergeAndReplaceOrInsertPropertiesByNames => {
                "UPDATE_TYPE_MERGE_AND_REPLACE_OR_INSERT_PROPERTIES_BY_NAMES"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UPDATE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "UPDATE_TYPE_REPLACE" => Some(Self::Replace),
            "UPDATE_TYPE_MERGE" => Some(Self::Merge),
            "UPDATE_TYPE_INSERT_PROPERTIES_BY_NAMES" => {
                Some(Self::InsertPropertiesByNames)
            }
            "UPDATE_TYPE_REPLACE_PROPERTIES_BY_NAMES" => {
                Some(Self::ReplacePropertiesByNames)
            }
            "UPDATE_TYPE_DELETE_PROPERTIES_BY_NAMES" => {
                Some(Self::DeletePropertiesByNames)
            }
            "UPDATE_TYPE_MERGE_AND_REPLACE_OR_INSERT_PROPERTIES_BY_NAMES" => {
                Some(Self::MergeAndReplaceOrInsertPropertiesByNames)
            }
            _ => None,
        }
    }
}
/// Type of database used by the customer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatabaseType {
    /// This value is required by protobuf best practices
    DbUnknown = 0,
    /// Internal Spanner
    DbInfraSpanner = 1,
    /// Cloud Sql with a Postgres Sql instance
    DbCloudSqlPostgres = 2,
}
impl DatabaseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DatabaseType::DbUnknown => "DB_UNKNOWN",
            DatabaseType::DbInfraSpanner => "DB_INFRA_SPANNER",
            DatabaseType::DbCloudSqlPostgres => "DB_CLOUD_SQL_POSTGRES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DB_UNKNOWN" => Some(Self::DbUnknown),
            "DB_INFRA_SPANNER" => Some(Self::DbInfraSpanner),
            "DB_CLOUD_SQL_POSTGRES" => Some(Self::DbCloudSqlPostgres),
            _ => None,
        }
    }
}
/// Access Control Mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessControlMode {
    /// This value is required by protobuf best practices
    AclModeUnknown = 0,
    /// Universal Access: No document level access control.
    AclModeUniversalAccess = 1,
    /// Document level access control with customer own Identity Service.
    AclModeDocumentLevelAccessControlByoid = 2,
    /// Document level access control using Google Cloud Identity.
    AclModeDocumentLevelAccessControlGci = 3,
}
impl AccessControlMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessControlMode::AclModeUnknown => "ACL_MODE_UNKNOWN",
            AccessControlMode::AclModeUniversalAccess => "ACL_MODE_UNIVERSAL_ACCESS",
            AccessControlMode::AclModeDocumentLevelAccessControlByoid => {
                "ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_BYOID"
            }
            AccessControlMode::AclModeDocumentLevelAccessControlGci => {
                "ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_GCI"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACL_MODE_UNKNOWN" => Some(Self::AclModeUnknown),
            "ACL_MODE_UNIVERSAL_ACCESS" => Some(Self::AclModeUniversalAccess),
            "ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_BYOID" => {
                Some(Self::AclModeDocumentLevelAccessControlByoid)
            }
            "ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_GCI" => {
                Some(Self::AclModeDocumentLevelAccessControlGci)
            }
            _ => None,
        }
    }
}
/// The default role of the document creator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DocumentCreatorDefaultRole {
    /// Unspecified, will be default to document admin role.
    Unspecified = 0,
    /// Document Admin, same as contentwarehouse.googleapis.com/documentAdmin.
    DocumentAdmin = 1,
    /// Document Editor, same as contentwarehouse.googleapis.com/documentEditor.
    DocumentEditor = 2,
    /// Document Viewer, same as contentwarehouse.googleapis.com/documentViewer.
    DocumentViewer = 3,
}
impl DocumentCreatorDefaultRole {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DocumentCreatorDefaultRole::Unspecified => {
                "DOCUMENT_CREATOR_DEFAULT_ROLE_UNSPECIFIED"
            }
            DocumentCreatorDefaultRole::DocumentAdmin => "DOCUMENT_ADMIN",
            DocumentCreatorDefaultRole::DocumentEditor => "DOCUMENT_EDITOR",
            DocumentCreatorDefaultRole::DocumentViewer => "DOCUMENT_VIEWER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOCUMENT_CREATOR_DEFAULT_ROLE_UNSPECIFIED" => Some(Self::Unspecified),
            "DOCUMENT_ADMIN" => Some(Self::DocumentAdmin),
            "DOCUMENT_EDITOR" => Some(Self::DocumentEditor),
            "DOCUMENT_VIEWER" => Some(Self::DocumentViewer),
            _ => None,
        }
    }
}
/// Response message of RunPipeline method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPipelineResponse {}
/// Metadata message of RunPipeline method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPipelineMetadata {
    /// Number of files that were processed by the pipeline.
    #[prost(int32, tag = "1")]
    pub total_file_count: i32,
    /// Number of files that have failed at some point in the pipeline.
    #[prost(int32, tag = "2")]
    pub failed_file_count: i32,
    /// User unique identification and groups information.
    #[prost(message, optional, tag = "3")]
    pub user_info: ::core::option::Option<UserInfo>,
    /// The list of response details of each document.
    #[prost(message, repeated, tag = "5")]
    pub individual_document_statuses: ::prost::alloc::vec::Vec<
        run_pipeline_metadata::IndividualDocumentStatus,
    >,
    /// The pipeline metadata.
    #[prost(oneof = "run_pipeline_metadata::PipelineMetadata", tags = "4, 6, 7")]
    pub pipeline_metadata: ::core::option::Option<
        run_pipeline_metadata::PipelineMetadata,
    >,
}
/// Nested message and enum types in `RunPipelineMetadata`.
pub mod run_pipeline_metadata {
    /// The metadata message for GcsIngest pipeline.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsIngestPipelineMetadata {
        /// The input Cloud Storage folder in this pipeline.
        /// Format: `gs://<bucket-name>/<folder-name>`.
        #[prost(string, tag = "1")]
        pub input_path: ::prost::alloc::string::String,
    }
    /// The metadata message for Export-to-CDW pipeline.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExportToCdwPipelineMetadata {
        /// The input list of all the resource names of the documents to be exported.
        #[prost(string, repeated, tag = "1")]
        pub documents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The output CDW dataset resource name.
        #[prost(string, tag = "2")]
        pub doc_ai_dataset: ::prost::alloc::string::String,
        /// The output Cloud Storage folder in this pipeline.
        #[prost(string, tag = "3")]
        pub output_path: ::prost::alloc::string::String,
    }
    /// The metadata message for Process-with-DocAi pipeline.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProcessWithDocAiPipelineMetadata {
        /// The input list of all the resource names of the documents to be
        /// processed.
        #[prost(string, repeated, tag = "1")]
        pub documents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The DocAI processor to process the documents with.
        #[prost(message, optional, tag = "2")]
        pub processor_info: ::core::option::Option<super::ProcessorInfo>,
    }
    /// The status of processing a document.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IndividualDocumentStatus {
        /// Document identifier of an existing document.
        #[prost(string, tag = "1")]
        pub document_id: ::prost::alloc::string::String,
        /// The status processing the document.
        #[prost(message, optional, tag = "2")]
        pub status: ::core::option::Option<super::super::super::super::rpc::Status>,
    }
    /// The pipeline metadata.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PipelineMetadata {
        /// The pipeline metadata for GcsIngest pipeline.
        #[prost(message, tag = "4")]
        GcsIngestPipelineMetadata(GcsIngestPipelineMetadata),
        /// The pipeline metadata for Export-to-CDW pipeline.
        #[prost(message, tag = "6")]
        ExportToCdwPipelineMetadata(ExportToCdwPipelineMetadata),
        /// The pipeline metadata for Process-with-DocAi pipeline.
        #[prost(message, tag = "7")]
        ProcessWithDocAiPipelineMetadata(ProcessWithDocAiPipelineMetadata),
    }
}
/// The DocAI processor information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessorInfo {
    /// The processor resource name.
    /// Format is `projects/{project}/locations/{location}/processors/{processor}`,
    /// or
    /// `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`
    #[prost(string, tag = "1")]
    pub processor_name: ::prost::alloc::string::String,
    /// The processor will process the documents with this document type.
    #[prost(string, tag = "2")]
    pub document_type: ::prost::alloc::string::String,
    /// The Document schema resource name. All documents processed by this
    /// processor will use this schema.
    /// Format:
    /// projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}.
    #[prost(string, tag = "3")]
    pub schema_name: ::prost::alloc::string::String,
}
/// The ingestion pipeline config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestPipelineConfig {
    /// The document level acl policy config.
    /// This refers to an Identity and Access (IAM) policy, which specifies access
    /// controls for all documents ingested by the pipeline. The
    /// [role][google.iam.v1.Binding.role] and
    /// [members][google.iam.v1.Binding.role] under the policy needs to be
    /// specified.
    ///
    /// The following roles are supported for document level acl control:
    /// * roles/contentwarehouse.documentAdmin
    /// * roles/contentwarehouse.documentEditor
    /// * roles/contentwarehouse.documentViewer
    ///
    /// The following members are supported for document level acl control:
    /// * user:user-email@example.com
    /// * group:group-email@example.com
    /// Note that for documents searched with LLM, only single level user or group
    /// acl check is supported.
    #[prost(message, optional, tag = "1")]
    pub document_acl_policy: ::core::option::Option<
        super::super::super::iam::v1::Policy,
    >,
    /// The document text extraction enabled flag.
    /// If the flag is set to true, DWH will perform text extraction on the raw
    /// document.
    #[prost(bool, tag = "2")]
    pub enable_document_text_extraction: bool,
    /// Optional. The name of the folder to which all ingested documents will be
    /// linked during ingestion process. Format is
    /// `projects/{project}/locations/{location}/documents/{folder_id}`
    #[prost(string, tag = "3")]
    pub folder: ::prost::alloc::string::String,
    /// The Cloud Function resource name. The Cloud Function needs to live inside
    /// consumer project and is accessible to Document AI Warehouse P4SA.
    /// Only Cloud Functions V2 is supported. Cloud function execution should
    /// complete within 5 minutes or this file ingestion may fail due to timeout.
    /// Format: `<https://{region}-{project_id}.cloudfunctions.net/{cloud_function}`>
    /// The following keys are available the request json payload.
    /// * display_name
    /// * properties
    /// * plain_text
    /// * reference_id
    /// * document_schema_name
    /// * raw_document_path
    /// * raw_document_file_type
    ///
    /// The following keys from the cloud function json response payload will be
    /// ingested to the Document AI Warehouse as part of Document proto content
    /// and/or related information. The original values will be overridden if any
    /// key is present in the response.
    /// * display_name
    /// * properties
    /// * plain_text
    /// * document_acl_policy
    /// * folder
    #[prost(string, tag = "4")]
    pub cloud_function: ::prost::alloc::string::String,
}
/// The configuration of the Cloud Storage Ingestion pipeline.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsIngestPipeline {
    /// The input Cloud Storage folder. All files under this folder will be
    /// imported to Document Warehouse.
    /// Format: `gs://<bucket-name>/<folder-name>`.
    #[prost(string, tag = "1")]
    pub input_path: ::prost::alloc::string::String,
    /// The Document Warehouse schema resource name. All documents processed by
    /// this pipeline will use this schema.
    /// Format:
    /// projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}.
    #[prost(string, tag = "2")]
    pub schema_name: ::prost::alloc::string::String,
    /// The Doc AI processor type name. Only used when the format of ingested
    /// files is Doc AI Document proto format.
    #[prost(string, tag = "3")]
    pub processor_type: ::prost::alloc::string::String,
    /// The flag whether to skip ingested documents.
    /// If it is set to true, documents in Cloud Storage contains key "status" with
    /// value "status=ingested" in custom metadata will be skipped to ingest.
    #[prost(bool, tag = "4")]
    pub skip_ingested_documents: bool,
    /// Optional. The config for the Cloud Storage Ingestion pipeline.
    /// It provides additional customization options to run the pipeline and can be
    /// skipped if it is not applicable.
    #[prost(message, optional, tag = "5")]
    pub pipeline_config: ::core::option::Option<IngestPipelineConfig>,
}
/// The configuration of the Cloud Storage Ingestion with DocAI Processors
/// pipeline.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsIngestWithDocAiProcessorsPipeline {
    /// The input Cloud Storage folder. All files under this folder will be
    /// imported to Document Warehouse.
    /// Format: `gs://<bucket-name>/<folder-name>`.
    #[prost(string, tag = "1")]
    pub input_path: ::prost::alloc::string::String,
    /// The split and classify processor information.
    /// The split and classify result will be used to find a matched extract
    /// processor.
    #[prost(message, optional, tag = "2")]
    pub split_classify_processor_info: ::core::option::Option<ProcessorInfo>,
    /// The extract processors information.
    /// One matched extract processor will be used to process documents based on
    /// the classify processor result. If no classify processor is specified, the
    /// first extract processor will be used.
    #[prost(message, repeated, tag = "3")]
    pub extract_processor_infos: ::prost::alloc::vec::Vec<ProcessorInfo>,
    /// The Cloud Storage folder path used to store the raw results from
    /// processors.
    /// Format: `gs://<bucket-name>/<folder-name>`.
    #[prost(string, tag = "4")]
    pub processor_results_folder_path: ::prost::alloc::string::String,
    /// The flag whether to skip ingested documents.
    /// If it is set to true, documents in Cloud Storage contains key "status" with
    /// value "status=ingested" in custom metadata will be skipped to ingest.
    #[prost(bool, tag = "5")]
    pub skip_ingested_documents: bool,
    /// Optional. The config for the Cloud Storage Ingestion with DocAI Processors
    /// pipeline. It provides additional customization options to run the pipeline
    /// and can be skipped if it is not applicable.
    #[prost(message, optional, tag = "6")]
    pub pipeline_config: ::core::option::Option<IngestPipelineConfig>,
}
/// The configuration of exporting documents from the Document Warehouse to CDW
/// pipeline.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportToCdwPipeline {
    /// The list of all the resource names of the documents to be processed.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{document_id}.
    #[prost(string, repeated, tag = "1")]
    pub documents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Cloud Storage folder path used to store the exported documents before
    /// being sent to CDW.
    /// Format: `gs://<bucket-name>/<folder-name>`.
    #[prost(string, tag = "2")]
    pub export_folder_path: ::prost::alloc::string::String,
    /// Optional. The CDW dataset resource name. This field is optional. If not
    /// set, the documents will be exported to Cloud Storage only. Format:
    /// projects/{project}/locations/{location}/processors/{processor}/dataset
    #[prost(string, tag = "3")]
    pub doc_ai_dataset: ::prost::alloc::string::String,
    /// Ratio of training dataset split. When importing into Document AI Workbench,
    /// documents will be automatically split into training and test split category
    /// with the specified ratio. This field is required if doc_ai_dataset is set.
    #[prost(float, tag = "4")]
    pub training_split_ratio: f32,
}
/// The configuration of processing documents in Document Warehouse with DocAi
/// processors pipeline.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessWithDocAiPipeline {
    /// The list of all the resource names of the documents to be processed.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{document_id}.
    #[prost(string, repeated, tag = "1")]
    pub documents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Cloud Storage folder path used to store the exported documents before
    /// being sent to CDW.
    /// Format: `gs://<bucket-name>/<folder-name>`.
    #[prost(string, tag = "2")]
    pub export_folder_path: ::prost::alloc::string::String,
    /// The CDW processor information.
    #[prost(message, optional, tag = "3")]
    pub processor_info: ::core::option::Option<ProcessorInfo>,
    /// The Cloud Storage folder path used to store the raw results from
    /// processors.
    /// Format: `gs://<bucket-name>/<folder-name>`.
    #[prost(string, tag = "4")]
    pub processor_results_folder_path: ::prost::alloc::string::String,
}
/// Represents a list of synonyms for a given context.
/// For example a context "sales" could contain:
/// Synonym 1: sale, invoice, bill, order
/// Synonym 2: money, credit, finance, payment
/// Synonym 3: shipping, freight, transport
/// Each SynonymSets should be disjoint
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynonymSet {
    /// The resource name of the SynonymSet
    /// This is mandatory for google.api.resource.
    /// Format:
    /// projects/{project_number}/locations/{location}/synonymSets/{context}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// This is a freeform field. Example contexts can be "sales," "engineering,"
    /// "real estate," "accounting," etc.
    /// The context can be supplied during search requests.
    #[prost(string, tag = "2")]
    pub context: ::prost::alloc::string::String,
    /// List of Synonyms for the context.
    #[prost(message, repeated, tag = "3")]
    pub synonyms: ::prost::alloc::vec::Vec<synonym_set::Synonym>,
}
/// Nested message and enum types in `SynonymSet`.
pub mod synonym_set {
    /// Represents a list of words given by the customer
    /// All these words are synonyms of each other.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Synonym {
        /// For example: sale, invoice, bill, order
        #[prost(string, repeated, tag = "1")]
        pub words: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Request message for SynonymSetService.CreateSynonymSet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSynonymSetRequest {
    /// Required. The parent name.
    /// Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The synonymSet to be created for a context
    #[prost(message, optional, tag = "2")]
    pub synonym_set: ::core::option::Option<SynonymSet>,
}
/// Request message for SynonymSetService.GetSynonymSet.
/// Will return synonymSet for a certain context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSynonymSetRequest {
    /// Required. The name of the synonymSet to retrieve
    /// Format:
    /// projects/{project_number}/locations/{location}/synonymSets/{context}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for SynonymSetService.ListSynonymSets.
/// Will return all synonymSets belonging to the customer project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSynonymSetsRequest {
    /// Required. The parent name.
    /// Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of synonymSets to return. The service may return
    /// fewer than this value.
    /// If unspecified, at most 50 rule sets will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListSynonymSets` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListSynonymSets`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for SynonymSetService.ListSynonymSets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSynonymSetsResponse {
    /// The synonymSets from the specified parent.
    #[prost(message, repeated, tag = "1")]
    pub synonym_sets: ::prost::alloc::vec::Vec<SynonymSet>,
    /// A page token, received from a previous `ListSynonymSets` call.
    /// Provide this to retrieve the subsequent page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for SynonymSetService.UpdateSynonymSet.
/// Removes the SynonymSet for the specified context and replaces
/// it with the SynonymSet in this request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSynonymSetRequest {
    /// Required. The name of the synonymSet to update
    /// Format:
    /// projects/{project_number}/locations/{location}/synonymSets/{context}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The synonymSet to be updated for the customer
    #[prost(message, optional, tag = "2")]
    pub synonym_set: ::core::option::Option<SynonymSet>,
}
/// Request message for SynonymSetService.DeleteSynonymSet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSynonymSetRequest {
    /// Required. The name of the synonymSet to delete
    /// Format:
    /// projects/{project_number}/locations/{location}/synonymSets/{context}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod synonym_set_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A Service that manage/custom customer specified SynonymSets.
    #[derive(Debug, Clone)]
    pub struct SynonymSetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SynonymSetServiceClient<T>
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
        ) -> SynonymSetServiceClient<InterceptedService<T, F>>
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
            SynonymSetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a SynonymSet for a single context.
        /// Throws an ALREADY_EXISTS exception if a synonymset already exists
        /// for the context.
        pub async fn create_synonym_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSynonymSetRequest>,
        ) -> std::result::Result<tonic::Response<super::SynonymSet>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.contentwarehouse.v1.SynonymSetService/CreateSynonymSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.SynonymSetService",
                        "CreateSynonymSet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a SynonymSet for a particular context.
        /// Throws a NOT_FOUND exception if the Synonymset
        /// does not exist
        pub async fn get_synonym_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSynonymSetRequest>,
        ) -> std::result::Result<tonic::Response<super::SynonymSet>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.contentwarehouse.v1.SynonymSetService/GetSynonymSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.SynonymSetService",
                        "GetSynonymSet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Remove the existing SynonymSet for the context and replaces it
        /// with a new one.
        /// Throws a NOT_FOUND exception if the SynonymSet is not found.
        pub async fn update_synonym_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSynonymSetRequest>,
        ) -> std::result::Result<tonic::Response<super::SynonymSet>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.contentwarehouse.v1.SynonymSetService/UpdateSynonymSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.SynonymSetService",
                        "UpdateSynonymSet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a SynonymSet for a given context.
        /// Throws a NOT_FOUND exception if the SynonymSet is not found.
        pub async fn delete_synonym_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSynonymSetRequest>,
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
                "/google.cloud.contentwarehouse.v1.SynonymSetService/DeleteSynonymSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.SynonymSetService",
                        "DeleteSynonymSet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns all SynonymSets (for all contexts) for the specified location.
        pub async fn list_synonym_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSynonymSetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSynonymSetsResponse>,
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
                "/google.cloud.contentwarehouse.v1.SynonymSetService/ListSynonymSets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.SynonymSetService",
                        "ListSynonymSets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// A document schema used to define document structure.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentSchema {
    /// The resource name of the document schema.
    /// Format:
    /// projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}.
    ///
    /// The name is ignored when creating a document schema.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Name of the schema given by the user. Must be unique per project.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Document details.
    #[prost(message, repeated, tag = "3")]
    pub property_definitions: ::prost::alloc::vec::Vec<PropertyDefinition>,
    /// Document Type, true refers the document is a folder, otherwise it is
    /// a typical document.
    #[prost(bool, tag = "4")]
    pub document_is_folder: bool,
    /// Output only. The time when the document schema is last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the document schema is created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Schema description.
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
}
/// Defines the metadata for a schema property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyDefinition {
    /// Required. The name of the metadata property.
    /// Must be unique within a document schema and is case insensitive.
    /// Names must be non-blank, start with a letter, and can contain alphanumeric
    /// characters and: /, :, -, _, and .
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The display-name for the property, used for front-end.
    #[prost(string, tag = "12")]
    pub display_name: ::prost::alloc::string::String,
    /// Whether the property can have multiple values.
    #[prost(bool, tag = "2")]
    pub is_repeatable: bool,
    /// Whether the property can be filtered. If this is a sub-property, all the
    /// parent properties must be marked filterable.
    #[prost(bool, tag = "3")]
    pub is_filterable: bool,
    /// Indicates that the property should be included in a global search.
    #[prost(bool, tag = "4")]
    pub is_searchable: bool,
    /// Whether the property is user supplied metadata.
    /// This out-of-the box placeholder setting can be used to tag derived
    /// properties. Its value and interpretation logic should be implemented by API
    /// user.
    #[prost(bool, tag = "5")]
    pub is_metadata: bool,
    /// Whether the property is mandatory.
    /// Default is 'false', i.e. populating property value can be skipped.
    /// If 'true' then user must populate the value for this property.
    #[prost(bool, tag = "14")]
    pub is_required: bool,
    /// The retrieval importance of the property during search.
    #[prost(enumeration = "property_definition::RetrievalImportance", tag = "18")]
    pub retrieval_importance: i32,
    /// The mapping information between this property to another schema source.
    #[prost(message, repeated, tag = "19")]
    pub schema_sources: ::prost::alloc::vec::Vec<property_definition::SchemaSource>,
    /// Type of the property.
    #[prost(
        oneof = "property_definition::ValueTypeOptions",
        tags = "7, 8, 9, 10, 11, 13, 15, 16"
    )]
    pub value_type_options: ::core::option::Option<
        property_definition::ValueTypeOptions,
    >,
}
/// Nested message and enum types in `PropertyDefinition`.
pub mod property_definition {
    /// The schema source information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SchemaSource {
        /// The schema name in the source.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The Doc AI processor type name.
        #[prost(string, tag = "2")]
        pub processor_type: ::prost::alloc::string::String,
    }
    /// Stores the retrieval importance.
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
    pub enum RetrievalImportance {
        /// No importance specified. Default medium importance.
        Unspecified = 0,
        /// Highest importance.
        Highest = 1,
        /// Higher importance.
        Higher = 2,
        /// High importance.
        High = 3,
        /// Medium importance.
        Medium = 4,
        /// Low importance (negative).
        Low = 5,
        /// Lowest importance (negative).
        Lowest = 6,
    }
    impl RetrievalImportance {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RetrievalImportance::Unspecified => "RETRIEVAL_IMPORTANCE_UNSPECIFIED",
                RetrievalImportance::Highest => "HIGHEST",
                RetrievalImportance::Higher => "HIGHER",
                RetrievalImportance::High => "HIGH",
                RetrievalImportance::Medium => "MEDIUM",
                RetrievalImportance::Low => "LOW",
                RetrievalImportance::Lowest => "LOWEST",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RETRIEVAL_IMPORTANCE_UNSPECIFIED" => Some(Self::Unspecified),
                "HIGHEST" => Some(Self::Highest),
                "HIGHER" => Some(Self::Higher),
                "HIGH" => Some(Self::High),
                "MEDIUM" => Some(Self::Medium),
                "LOW" => Some(Self::Low),
                "LOWEST" => Some(Self::Lowest),
                _ => None,
            }
        }
    }
    /// Type of the property.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ValueTypeOptions {
        /// Integer property.
        #[prost(message, tag = "7")]
        IntegerTypeOptions(super::IntegerTypeOptions),
        /// Float property.
        #[prost(message, tag = "8")]
        FloatTypeOptions(super::FloatTypeOptions),
        /// Text/string property.
        #[prost(message, tag = "9")]
        TextTypeOptions(super::TextTypeOptions),
        /// Nested structured data property.
        #[prost(message, tag = "10")]
        PropertyTypeOptions(super::PropertyTypeOptions),
        /// Enum/categorical property.
        #[prost(message, tag = "11")]
        EnumTypeOptions(super::EnumTypeOptions),
        /// Date time property.
        /// It is not supported by CMEK compliant deployment.
        #[prost(message, tag = "13")]
        DateTimeTypeOptions(super::DateTimeTypeOptions),
        /// Map property.
        #[prost(message, tag = "15")]
        MapTypeOptions(super::MapTypeOptions),
        /// Timestamp property.
        /// It is not supported by CMEK compliant deployment.
        #[prost(message, tag = "16")]
        TimestampTypeOptions(super::TimestampTypeOptions),
    }
}
/// Configurations for an integer property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegerTypeOptions {}
/// Configurations for a float property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatTypeOptions {}
/// Configurations for a text property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextTypeOptions {}
/// Configurations for a date time property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTimeTypeOptions {}
/// Configurations for a Map property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapTypeOptions {}
/// Configurations for a timestamp property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampTypeOptions {}
/// Configurations for a nested structured data property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyTypeOptions {
    /// Required. List of property definitions.
    #[prost(message, repeated, tag = "1")]
    pub property_definitions: ::prost::alloc::vec::Vec<PropertyDefinition>,
}
/// Configurations for an enum/categorical property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumTypeOptions {
    /// Required. List of possible enum values.
    #[prost(string, repeated, tag = "1")]
    pub possible_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Make sure the Enum property value provided in the document is in the
    /// possile value list during document creation. The validation check runs by
    /// default.
    #[prost(bool, tag = "2")]
    pub validation_check_disabled: bool,
}
/// Represents a set of rules from a single customer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleSet {
    /// The resource name of the rule set. Managed internally.
    /// Format:
    /// projects/{project_number}/locations/{location}/ruleSet/{rule_set_id}.
    ///
    /// The name is ignored when creating a rule set.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// Short description of the rule-set.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// Source of the rules i.e., customer name.
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    /// List of rules given by the customer.
    #[prost(message, repeated, tag = "3")]
    pub rules: ::prost::alloc::vec::Vec<Rule>,
}
/// Represents the rule for a content warehouse trigger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rule {
    /// Short description of the rule and its context.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// ID of the rule. It has to be unique across all the examples.
    /// This is managed internally.
    #[prost(string, tag = "2")]
    pub rule_id: ::prost::alloc::string::String,
    /// Identifies the trigger type for running the policy.
    #[prost(enumeration = "rule::TriggerType", tag = "3")]
    pub trigger_type: i32,
    /// Represents the conditional expression to be evaluated.
    /// Expression should evaluate to a boolean result.
    /// When the condition is true actions are executed.
    /// Example: user_role = "hsbc_role_1" AND doc.salary > 20000
    #[prost(string, tag = "4")]
    pub condition: ::prost::alloc::string::String,
    /// List of actions that are executed when the rule is satisfied.
    #[prost(message, repeated, tag = "5")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
}
/// Nested message and enum types in `Rule`.
pub mod rule {
    /// The trigger types for actions.
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
    pub enum TriggerType {
        /// Trigger for unknown action.
        Unknown = 0,
        /// Trigger for create document action.
        OnCreate = 1,
        /// Trigger for update document action.
        OnUpdate = 4,
        /// Trigger for create link action.
        OnCreateLink = 7,
        /// Trigger for delete link action.
        OnDeleteLink = 8,
    }
    impl TriggerType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TriggerType::Unknown => "UNKNOWN",
                TriggerType::OnCreate => "ON_CREATE",
                TriggerType::OnUpdate => "ON_UPDATE",
                TriggerType::OnCreateLink => "ON_CREATE_LINK",
                TriggerType::OnDeleteLink => "ON_DELETE_LINK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "ON_CREATE" => Some(Self::OnCreate),
                "ON_UPDATE" => Some(Self::OnUpdate),
                "ON_CREATE_LINK" => Some(Self::OnCreateLink),
                "ON_DELETE_LINK" => Some(Self::OnDeleteLink),
                _ => None,
            }
        }
    }
}
/// Represents the action triggered by Rule Engine when the rule is true.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    /// ID of the action. Managed internally.
    #[prost(string, tag = "1")]
    pub action_id: ::prost::alloc::string::String,
    #[prost(oneof = "action::Action", tags = "2, 3, 4, 5, 6, 9, 10")]
    pub action: ::core::option::Option<action::Action>,
}
/// Nested message and enum types in `Action`.
pub mod action {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// Action triggering access control operations.
        #[prost(message, tag = "2")]
        AccessControl(super::AccessControlAction),
        /// Action triggering data validation operations.
        #[prost(message, tag = "3")]
        DataValidation(super::DataValidationAction),
        /// Action triggering data update operations.
        #[prost(message, tag = "4")]
        DataUpdate(super::DataUpdateAction),
        /// Action triggering create document link operation.
        #[prost(message, tag = "5")]
        AddToFolder(super::AddToFolderAction),
        /// Action publish to Pub/Sub operation.
        #[prost(message, tag = "6")]
        PublishToPubSub(super::PublishAction),
        /// Action removing a document from a folder.
        #[prost(message, tag = "9")]
        RemoveFromFolderAction(super::RemoveFromFolderAction),
        /// Action deleting the document.
        #[prost(message, tag = "10")]
        DeleteDocumentAction(super::DeleteDocumentAction),
    }
}
/// Represents the action responsible for access control list management
/// operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessControlAction {
    /// Identifies the type of operation.
    #[prost(enumeration = "access_control_action::OperationType", tag = "1")]
    pub operation_type: i32,
    /// Represents the new policy from which bindings are added, removed or
    /// replaced based on the type of the operation. the policy is limited to a few
    /// 10s of KB.
    #[prost(message, optional, tag = "2")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
}
/// Nested message and enum types in `AccessControlAction`.
pub mod access_control_action {
    /// Type of ACL modification operation.
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
    pub enum OperationType {
        /// The unknown operation type.
        Unknown = 0,
        /// Adds newly given policy bindings in the existing bindings list.
        AddPolicyBinding = 1,
        /// Removes newly given policy bindings from the existing bindings list.
        RemovePolicyBinding = 2,
        /// Replaces existing policy bindings with the given policy binding list
        ReplacePolicyBinding = 3,
    }
    impl OperationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OperationType::Unknown => "UNKNOWN",
                OperationType::AddPolicyBinding => "ADD_POLICY_BINDING",
                OperationType::RemovePolicyBinding => "REMOVE_POLICY_BINDING",
                OperationType::ReplacePolicyBinding => "REPLACE_POLICY_BINDING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "ADD_POLICY_BINDING" => Some(Self::AddPolicyBinding),
                "REMOVE_POLICY_BINDING" => Some(Self::RemovePolicyBinding),
                "REPLACE_POLICY_BINDING" => Some(Self::ReplacePolicyBinding),
                _ => None,
            }
        }
    }
}
/// Represents the action responsible for data validation operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataValidationAction {
    /// Map of (K, V) -> (field, string condition to be evaluated on the field)
    /// E.g., ("age", "age > 18  && age < 60") entry triggers validation of field
    /// age with the given condition. Map entries will be ANDed during validation.
    #[prost(btree_map = "string, string", tag = "1")]
    pub conditions: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Represents the action responsible for properties update operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataUpdateAction {
    /// Map of (K, V) -> (valid name of the field, new value of the field)
    /// E.g., ("age", "60") entry triggers update of field age with a value of 60.
    /// If the field is not present then new entry is added.
    /// During update action execution, value strings will be casted to
    /// appropriate types.
    #[prost(btree_map = "string, string", tag = "1")]
    pub entries: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Represents the action responsible for adding document under a folder.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToFolderAction {
    /// Names of the folder under which new document is to be added.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{document_id}.
    #[prost(string, repeated, tag = "1")]
    pub folders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents the action responsible for remove a document from a specific
/// folder.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFromFolderAction {
    /// Condition of the action to be executed.
    #[prost(string, tag = "1")]
    pub condition: ::prost::alloc::string::String,
    /// Name of the folder under which new document is to be added.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{document_id}.
    #[prost(string, tag = "2")]
    pub folder: ::prost::alloc::string::String,
}
/// Represents the action responsible for publishing messages to a Pub/Sub topic.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishAction {
    /// The topic id in the Pub/Sub service for which messages will be published
    /// to.
    #[prost(string, tag = "1")]
    pub topic_id: ::prost::alloc::string::String,
    /// Messages to be published.
    #[prost(string, repeated, tag = "2")]
    pub messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents the action responsible for deleting the document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentAction {
    /// Boolean field to select between hard vs soft delete options.
    /// Set 'true' for 'hard delete' and 'false' for 'soft delete'.
    #[prost(bool, tag = "1")]
    pub enable_hard_delete: bool,
}
/// Records the output of Rule Engine including rule evaluation and actions
/// result.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleEngineOutput {
    /// Name of the document against which the rules and actions were evaluated.
    #[prost(string, tag = "3")]
    pub document_name: ::prost::alloc::string::String,
    /// Output from Rule Evaluator containing matched, unmatched and invalid rules.
    #[prost(message, optional, tag = "1")]
    pub rule_evaluator_output: ::core::option::Option<RuleEvaluatorOutput>,
    /// Output from Action Executor containing rule and corresponding actions
    /// execution result.
    #[prost(message, optional, tag = "2")]
    pub action_executor_output: ::core::option::Option<ActionExecutorOutput>,
}
/// Represents the output of the Rule Evaluator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleEvaluatorOutput {
    /// List of rules fetched from database for the given request trigger type.
    #[prost(message, repeated, tag = "1")]
    pub triggered_rules: ::prost::alloc::vec::Vec<Rule>,
    /// A subset of triggered rules that are evaluated true for a given request.
    #[prost(message, repeated, tag = "2")]
    pub matched_rules: ::prost::alloc::vec::Vec<Rule>,
    /// A subset of triggered rules that failed the validation check(s) after
    /// parsing.
    #[prost(message, repeated, tag = "3")]
    pub invalid_rules: ::prost::alloc::vec::Vec<InvalidRule>,
}
/// A triggered rule that failed the validation check(s) after parsing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidRule {
    /// Triggered rule.
    #[prost(message, optional, tag = "1")]
    pub rule: ::core::option::Option<Rule>,
    /// Validation error on a parsed expression.
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
/// Represents the output of the Action Executor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionExecutorOutput {
    /// List of rule and corresponding actions result.
    #[prost(message, repeated, tag = "1")]
    pub rule_actions_pairs: ::prost::alloc::vec::Vec<RuleActionsPair>,
}
/// Represents a rule and outputs of associated actions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleActionsPair {
    /// Represents the rule.
    #[prost(message, optional, tag = "1")]
    pub rule: ::core::option::Option<Rule>,
    /// Outputs of executing the actions associated with the above rule.
    #[prost(message, repeated, tag = "2")]
    pub action_outputs: ::prost::alloc::vec::Vec<ActionOutput>,
}
/// Represents the result of executing an action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionOutput {
    /// ID of the action.
    #[prost(string, tag = "1")]
    pub action_id: ::prost::alloc::string::String,
    /// State of an action.
    #[prost(enumeration = "action_output::State", tag = "2")]
    pub action_state: i32,
    /// Action execution output message.
    #[prost(string, tag = "3")]
    pub output_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ActionOutput`.
pub mod action_output {
    /// Represents execution state of the action.
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
        /// The unknown state.
        Unknown = 0,
        /// State indicating action executed successfully.
        ActionSucceeded = 1,
        /// State indicating action failed.
        ActionFailed = 2,
        /// State indicating action timed out.
        ActionTimedOut = 3,
        /// State indicating action is pending.
        ActionPending = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unknown => "UNKNOWN",
                State::ActionSucceeded => "ACTION_SUCCEEDED",
                State::ActionFailed => "ACTION_FAILED",
                State::ActionTimedOut => "ACTION_TIMED_OUT",
                State::ActionPending => "ACTION_PENDING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "ACTION_SUCCEEDED" => Some(Self::ActionSucceeded),
                "ACTION_FAILED" => Some(Self::ActionFailed),
                "ACTION_TIMED_OUT" => Some(Self::ActionTimedOut),
                "ACTION_PENDING" => Some(Self::ActionPending),
                _ => None,
            }
        }
    }
}
/// Request message for RuleSetService.CreateRuleSet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRuleSetRequest {
    /// Required. The parent name.
    /// Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The rule set to create.
    #[prost(message, optional, tag = "2")]
    pub rule_set: ::core::option::Option<RuleSet>,
}
/// Request message for RuleSetService.GetRuleSet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRuleSetRequest {
    /// Required. The name of the rule set to retrieve.
    /// Format:
    /// projects/{project_number}/locations/{location}/ruleSets/{rule_set_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for RuleSetService.UpdateRuleSet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRuleSetRequest {
    /// Required. The name of the rule set to update.
    /// Format:
    /// projects/{project_number}/locations/{location}/ruleSets/{rule_set_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The rule set to update.
    #[prost(message, optional, tag = "2")]
    pub rule_set: ::core::option::Option<RuleSet>,
}
/// Request message for RuleSetService.DeleteRuleSet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRuleSetRequest {
    /// Required. The name of the rule set to delete.
    /// Format:
    /// projects/{project_number}/locations/{location}/ruleSets/{rule_set_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for RuleSetService.ListRuleSets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleSetsRequest {
    /// Required. The parent, which owns this collection of document.
    /// Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of rule sets to return. The service may return
    /// fewer than this value.
    /// If unspecified, at most 50 rule sets will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListRuleSets` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListRuleSets`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for RuleSetService.ListRuleSets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleSetsResponse {
    /// The rule sets from the specified parent.
    #[prost(message, repeated, tag = "1")]
    pub rule_sets: ::prost::alloc::vec::Vec<RuleSet>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DocumentService.RunPipeline.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPipelineRequest {
    /// Required. The resource name which owns the resources of the pipeline.
    /// Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The meta information collected about the end user, used to enforce access
    /// control for the service.
    #[prost(message, optional, tag = "6")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    /// The predefined pipelines.
    #[prost(oneof = "run_pipeline_request::Pipeline", tags = "2, 3, 4, 5")]
    pub pipeline: ::core::option::Option<run_pipeline_request::Pipeline>,
}
/// Nested message and enum types in `RunPipelineRequest`.
pub mod run_pipeline_request {
    /// The predefined pipelines.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Pipeline {
        /// Cloud Storage ingestion pipeline.
        #[prost(message, tag = "2")]
        GcsIngestPipeline(super::GcsIngestPipeline),
        /// Use DocAI processors to process documents in Cloud Storage and ingest
        /// them to Document Warehouse.
        #[prost(message, tag = "3")]
        GcsIngestWithDocAiProcessorsPipeline(
            super::GcsIngestWithDocAiProcessorsPipeline,
        ),
        /// Export docuemnts from Document Warehouse to CDW for training purpose.
        #[prost(message, tag = "4")]
        ExportCdwPipeline(super::ExportToCdwPipeline),
        /// Use a DocAI processor to process documents in Document Warehouse, and
        /// re-ingest the updated results into Document Warehouse.
        #[prost(message, tag = "5")]
        ProcessWithDocAiPipeline(super::ProcessWithDocAiPipeline),
    }
}
/// Generated client implementations.
pub mod pipeline_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// This service lets you manage pipelines.
    #[derive(Debug, Clone)]
    pub struct PipelineServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PipelineServiceClient<T>
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
        ) -> PipelineServiceClient<InterceptedService<T, F>>
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
            PipelineServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Run a predefined pipeline.
        pub async fn run_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::RunPipelineRequest>,
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
                "/google.cloud.contentwarehouse.v1.PipelineService/RunPipeline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.PipelineService",
                        "RunPipeline",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentQuery {
    /// The query string that matches against the full text of the document and
    /// the searchable properties.
    ///
    /// The query partially supports [Google AIP style
    /// syntax](<https://google.aip.dev/160>). Specifically, the query supports
    /// literals, logical operators, negation operators, comparison operators, and
    /// functions.
    ///
    /// Literals: A bare literal value (examples: "42", "Hugo") is a value to be
    /// matched against. It searches over the full text of the document and the
    /// searchable properties.
    ///
    /// Logical operators: "AND", "and", "OR", and "or" are binary logical
    /// operators (example: "engineer OR developer").
    ///
    /// Negation operators: "NOT" and "!" are negation operators (example: "NOT
    /// software").
    ///
    /// Comparison operators: support the binary comparison operators =, !=, <, >,
    /// <= and >= for string, numeric, enum, boolean. Also support like operator
    /// `~~` for string. It provides semantic search functionality by parsing,
    /// stemming and doing synonyms expansion against the input query.
    ///
    /// To specify a property in the query, the left hand side expression in the
    /// comparison must be the property ID including the parent. The right hand
    /// side must be literals. For example:
    /// "\"projects/123/locations/us\".property_a < 1" matches results whose
    /// "property_a" is less than 1 in project 123 and us location.
    /// The literals and comparison expression can be connected in a single query
    /// (example: "software engineer \"projects/123/locations/us\".salary > 100").
    ///
    /// Functions: supported functions are `LOWER(\[property_name\])` to perform a
    /// case insensitive match and `EMPTY(\[property_name\])` to filter on the
    /// existence of a key.
    ///
    /// Support nested expressions connected using parenthesis and logical
    /// operators. The default logical operators is `AND` if there is no operators
    /// between expressions.
    ///
    /// The query can be used with other filters e.g. `time_filters` and
    /// `folder_name_filter`. They are connected with `AND` operator under the
    /// hood.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// Experimental, do not use.
    /// If the query is a natural language question. False by default. If true,
    /// then the question-answering feature will be used instead of search, and
    /// `result_count` in
    /// [SearchDocumentsRequest][google.cloud.contentwarehouse.v1.SearchDocumentsRequest]
    /// must be set. In addition, all other input fields related to search
    /// (pagination, histograms, etc.) will be ignored.
    #[prost(bool, tag = "12")]
    pub is_nl_query: bool,
    /// This filter specifies a structured syntax to match against the
    /// \[PropertyDefinition\].[is_filterable][] marked as `true`. The syntax for
    /// this expression is a subset of SQL syntax.
    ///
    /// Supported operators are: `=`, `!=`, `<`, `<=`, `>`, and `>=` where the left
    /// of the operator is a property name and the right of the operator is a
    /// number or a quoted string. You must escape backslash (\\) and quote (\")
    /// characters. Supported functions are `LOWER(\[property_name\])` to perform a
    /// case insensitive match and `EMPTY(\[property_name\])` to filter on the
    /// existence of a key.
    ///
    /// Boolean expressions (AND/OR/NOT) are supported up to 3 levels of nesting
    /// (for example, "((A AND B AND C) OR NOT D) AND E"), a maximum of 100
    /// comparisons or functions are allowed in the expression. The expression must
    /// be < 6000 bytes in length.
    ///
    /// Sample Query:
    /// `(LOWER(driving_license)="class \"a\"" OR EMPTY(driving_license)) AND
    /// driving_years > 10`
    #[deprecated]
    #[prost(string, tag = "4")]
    pub custom_property_filter: ::prost::alloc::string::String,
    /// Documents created/updated within a range specified by this filter are
    /// searched against.
    #[prost(message, repeated, tag = "5")]
    pub time_filters: ::prost::alloc::vec::Vec<TimeFilter>,
    /// This filter specifies the exact document schema
    /// [Document.document_schema_name][google.cloud.contentwarehouse.v1.Document.document_schema_name]
    /// of the documents to search against.
    ///
    /// If a value isn't specified, documents within the search results are
    /// associated with any schema. If multiple values are specified, documents
    /// within the search results may be associated with any of the specified
    /// schemas.
    ///
    /// At most 20 document schema names are allowed.
    #[prost(string, repeated, tag = "6")]
    pub document_schema_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// This filter specifies a structured syntax to match against the
    /// [PropertyDefinition.is_filterable][google.cloud.contentwarehouse.v1.PropertyDefinition.is_filterable]
    /// marked as `true`. The relationship between the PropertyFilters is OR.
    #[prost(message, repeated, tag = "7")]
    pub property_filter: ::prost::alloc::vec::Vec<PropertyFilter>,
    /// This filter specifies the types of files to return: ALL, FOLDER, or FILE.
    /// If FOLDER or FILE is specified, then only either folders or files will be
    /// returned, respectively. If ALL is specified, both folders and files will be
    /// returned.
    ///
    /// If no value is specified, ALL files will be returned.
    #[prost(message, optional, tag = "8")]
    pub file_type_filter: ::core::option::Option<FileTypeFilter>,
    /// Search all the documents under this specified folder.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{document_id}.
    #[prost(string, tag = "9")]
    pub folder_name_filter: ::prost::alloc::string::String,
    /// Search the documents in the list.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{document_id}.
    #[prost(string, repeated, tag = "14")]
    pub document_name_filter: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// For custom synonyms.
    /// Customers provide the synonyms based on context. One customer can provide
    /// multiple set of synonyms based on different context. The search query will
    /// be expanded based on the custom synonyms of the query context set.
    /// By default, no custom synonyms wll be applied if no query context is
    /// provided.
    /// It is not supported for CMEK compliant deployment.
    #[prost(string, repeated, tag = "10")]
    pub query_context: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The exact creator(s) of the documents to search against.
    ///
    /// If a value isn't specified, documents within the search results are
    /// associated with any creator. If multiple values are specified, documents
    /// within the search results may be associated with any of the specified
    /// creators.
    #[prost(string, repeated, tag = "11")]
    pub document_creator_filter: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// To support the custom weighting across document schemas, customers need to
    /// provide the properties to be used to boost the ranking in the search
    /// request. For a search query with CustomWeightsMetadata specified, only the
    /// RetrievalImportance for the properties in the CustomWeightsMetadata will
    /// be honored.
    #[prost(message, optional, tag = "13")]
    pub custom_weights_metadata: ::core::option::Option<CustomWeightsMetadata>,
}
/// Filter on create timestamp or update timestamp of documents.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeFilter {
    #[prost(message, optional, tag = "1")]
    pub time_range: ::core::option::Option<super::super::super::r#type::Interval>,
    /// Specifies which time field to filter documents on.
    ///
    /// Defaults to [TimeField.UPLOAD_TIME][].
    #[prost(enumeration = "time_filter::TimeField", tag = "2")]
    pub time_field: i32,
}
/// Nested message and enum types in `TimeFilter`.
pub mod time_filter {
    /// Time field used in TimeFilter.
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
    pub enum TimeField {
        /// Default value.
        Unspecified = 0,
        /// Earliest document create time.
        CreateTime = 1,
        /// Latest document update time.
        UpdateTime = 2,
        /// Time when document becomes mutable again.
        DispositionTime = 3,
    }
    impl TimeField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimeField::Unspecified => "TIME_FIELD_UNSPECIFIED",
                TimeField::CreateTime => "CREATE_TIME",
                TimeField::UpdateTime => "UPDATE_TIME",
                TimeField::DispositionTime => "DISPOSITION_TIME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TIME_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATE_TIME" => Some(Self::CreateTime),
                "UPDATE_TIME" => Some(Self::UpdateTime),
                "DISPOSITION_TIME" => Some(Self::DispositionTime),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyFilter {
    /// The Document schema name
    /// [Document.document_schema_name][google.cloud.contentwarehouse.v1.Document.document_schema_name].
    /// Format:
    /// projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}.
    #[prost(string, tag = "1")]
    pub document_schema_name: ::prost::alloc::string::String,
    /// The filter condition.
    /// The syntax for this expression is a subset of SQL syntax.
    ///
    /// Supported operators are: `=`, `!=`, `<`, `<=`, `>`, `>=`, and `~~` where
    /// the left of the operator is a property name and the right of the operator
    /// is a number or a quoted string. You must escape backslash (\\) and quote
    /// (\") characters.
    ///
    /// `~~` is the LIKE operator. The right of the operator must be a string. The
    /// only supported property data type for LIKE is text_values. It provides
    /// semantic search functionality by parsing, stemming and doing synonyms
    /// expansion against the input query. It matches if the property contains
    /// semantic similar content to the query. It is not regex matching or wildcard
    /// matching. For example, "property.company ~~ \"google\"" will match records
    /// whose property `property.compnay` have values like "Google Inc.", "Google
    /// LLC" or "Google Company".
    ///
    /// Supported functions are `LOWER(\[property_name\])` to perform a
    /// case insensitive match and `EMPTY(\[property_name\])` to filter on the
    /// existence of a key.
    ///
    /// Boolean expressions (AND/OR/NOT) are supported up to 3 levels of nesting
    /// (for example, "((A AND B AND C) OR NOT D) AND E"), a maximum of 100
    /// comparisons or functions are allowed in the expression. The expression must
    /// be < 6000 bytes in length.
    ///
    /// Only properties that are marked filterable are allowed
    /// ([PropertyDefinition.is_filterable][google.cloud.contentwarehouse.v1.PropertyDefinition.is_filterable]).
    /// Property names do not need to be prefixed by the document schema id (as is
    /// the case with histograms), however property names will need to be prefixed
    /// by its parent hierarchy, if any.  For example:
    /// top_property_name.sub_property_name.
    ///
    /// Sample Query:
    /// `(LOWER(driving_license)="class \"a\"" OR EMPTY(driving_license)) AND
    /// driving_years > 10`
    ///
    ///
    /// CMEK compliant deployment only supports:
    ///
    /// * Operators: `=`, `<`, `<=`, `>`, and `>=`.
    /// * Boolean expressions: AND and OR.
    #[prost(string, tag = "2")]
    pub condition: ::prost::alloc::string::String,
}
/// Filter for the specific types of documents returned.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileTypeFilter {
    /// The type of files to return.
    #[prost(enumeration = "file_type_filter::FileType", tag = "1")]
    pub file_type: i32,
}
/// Nested message and enum types in `FileTypeFilter`.
pub mod file_type_filter {
    /// Representation of the types of files.
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
    pub enum FileType {
        /// Default document type. If set, disables the filter.
        Unspecified = 0,
        /// Returns all document types, including folders.
        All = 1,
        /// Returns only folders.
        Folder = 2,
        /// Returns only non-folder documents.
        Document = 3,
        /// Returns only root folders
        RootFolder = 4,
    }
    impl FileType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FileType::Unspecified => "FILE_TYPE_UNSPECIFIED",
                FileType::All => "ALL",
                FileType::Folder => "FOLDER",
                FileType::Document => "DOCUMENT",
                FileType::RootFolder => "ROOT_FOLDER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FILE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ALL" => Some(Self::All),
                "FOLDER" => Some(Self::Folder),
                "DOCUMENT" => Some(Self::Document),
                "ROOT_FOLDER" => Some(Self::RootFolder),
                _ => None,
            }
        }
    }
}
/// To support the custom weighting across document schemas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomWeightsMetadata {
    /// List of schema and property name. Allows a maximum of 10 schemas to be
    /// specified for relevance boosting.
    #[prost(message, repeated, tag = "1")]
    pub weighted_schema_properties: ::prost::alloc::vec::Vec<WeightedSchemaProperty>,
}
/// Specifies the schema property name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightedSchemaProperty {
    /// The document schema name.
    #[prost(string, tag = "1")]
    pub document_schema_name: ::prost::alloc::string::String,
    /// The property definition names in the schema.
    #[prost(string, repeated, tag = "2")]
    pub property_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Defines the structure for content warehouse document proto.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// The resource name of the document.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{document_id}.
    ///
    /// The name is ignored when creating a document.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The reference ID set by customers. Must be unique per project and location.
    #[prost(string, tag = "11")]
    pub reference_id: ::prost::alloc::string::String,
    /// Required. Display name of the document given by the user. This name will be
    /// displayed in the UI. Customer can populate this field with the name of the
    /// document. This differs from the 'title' field as 'title' is optional and
    /// stores the top heading in the document.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Title that describes the document.
    /// This can be the top heading or text that describes the document.
    #[prost(string, tag = "18")]
    pub title: ::prost::alloc::string::String,
    /// Uri to display the document, for example, in the UI.
    #[prost(string, tag = "17")]
    pub display_uri: ::prost::alloc::string::String,
    /// The Document schema name.
    /// Format:
    /// projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}.
    #[prost(string, tag = "3")]
    pub document_schema_name: ::prost::alloc::string::String,
    /// A path linked to structured content file.
    #[deprecated]
    #[prost(string, tag = "16")]
    pub structured_content_uri: ::prost::alloc::string::String,
    /// List of values that are user supplied metadata.
    #[prost(message, repeated, tag = "7")]
    pub properties: ::prost::alloc::vec::Vec<Property>,
    /// Output only. The time when the document is last updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the document is created.
    #[prost(message, optional, tag = "9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// This is used when DocAI was not used to load the document and parsing/
    /// extracting is needed for the inline_raw_document.  For example, if
    /// inline_raw_document is the byte representation of a PDF file, then
    /// this should be set to: RAW_DOCUMENT_FILE_TYPE_PDF.
    #[prost(enumeration = "RawDocumentFileType", tag = "10")]
    pub raw_document_file_type: i32,
    /// If true, makes the document visible to asynchronous policies and rules.
    #[deprecated]
    #[prost(bool, tag = "12")]
    pub async_enabled: bool,
    /// Indicates the category (image, audio, video etc.) of the original content.
    #[prost(enumeration = "ContentCategory", tag = "20")]
    pub content_category: i32,
    /// If true, text extraction will not be performed.
    #[deprecated]
    #[prost(bool, tag = "19")]
    pub text_extraction_disabled: bool,
    /// If true, text extraction will be performed.
    #[prost(bool, tag = "21")]
    pub text_extraction_enabled: bool,
    /// The user who creates the document.
    #[prost(string, tag = "13")]
    pub creator: ::prost::alloc::string::String,
    /// The user who lastly updates the document.
    #[prost(string, tag = "14")]
    pub updater: ::prost::alloc::string::String,
    /// Output only. If linked to a Collection with RetentionPolicy, the date when
    /// the document becomes mutable.
    #[prost(message, optional, tag = "22")]
    pub disposition_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Indicates if the document has a legal hold on it.
    #[prost(bool, tag = "23")]
    pub legal_hold: bool,
    #[prost(oneof = "document::StructuredContent", tags = "15, 4")]
    pub structured_content: ::core::option::Option<document::StructuredContent>,
    /// Raw document file.
    #[prost(oneof = "document::RawDocument", tags = "5, 6")]
    pub raw_document: ::core::option::Option<document::RawDocument>,
}
/// Nested message and enum types in `Document`.
pub mod document {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StructuredContent {
        /// Other document format, such as PPTX, XLXS
        #[prost(string, tag = "15")]
        PlainText(::prost::alloc::string::String),
        /// Document AI format to save the structured content, including OCR.
        #[prost(message, tag = "4")]
        CloudAiDocument(super::super::super::documentai::v1::Document),
    }
    /// Raw document file.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RawDocument {
        /// Raw document file in Cloud Storage path.
        #[prost(string, tag = "5")]
        RawDocumentPath(::prost::alloc::string::String),
        /// Raw document content.
        #[prost(bytes, tag = "6")]
        InlineRawDocument(::prost::bytes::Bytes),
    }
}
/// References to the documents.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentReference {
    /// Required. Name of the referenced document.
    #[prost(string, tag = "1")]
    pub document_name: ::prost::alloc::string::String,
    /// display_name of the referenced document; this name does not need to be
    /// consistent to the display_name in the Document proto, depending on the ACL
    /// constraint.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Stores the subset of the referenced document's content.
    /// This is useful to allow user peek the information of the referenced
    /// document.
    #[prost(string, tag = "3")]
    pub snippet: ::prost::alloc::string::String,
    /// The document type of the document being referenced.
    #[prost(bool, tag = "4")]
    pub document_is_folder: bool,
    /// Output only. The time when the document is last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the document is created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the document is deleted.
    #[prost(message, optional, tag = "7")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Document is a folder with retention policy.
    #[prost(bool, tag = "8")]
    pub document_is_retention_folder: bool,
    /// Document is a folder with legal hold.
    #[prost(bool, tag = "9")]
    pub document_is_legal_hold_folder: bool,
}
/// Property of a document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// Required. Must match the name of a PropertyDefinition in the
    /// DocumentSchema.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Type of the property.
    /// Must match the property_options type of the matching PropertyDefinition.
    /// Value of the Property parsed into a specific data type.
    /// Specific type value(s) obtained from Document AIs Property.mention_text
    /// field.
    #[prost(oneof = "property::Values", tags = "2, 3, 4, 5, 6, 7, 8, 9")]
    pub values: ::core::option::Option<property::Values>,
}
/// Nested message and enum types in `Property`.
pub mod property {
    /// Type of the property.
    /// Must match the property_options type of the matching PropertyDefinition.
    /// Value of the Property parsed into a specific data type.
    /// Specific type value(s) obtained from Document AIs Property.mention_text
    /// field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Values {
        /// Integer property values.
        #[prost(message, tag = "2")]
        IntegerValues(super::IntegerArray),
        /// Float property values.
        #[prost(message, tag = "3")]
        FloatValues(super::FloatArray),
        /// String/text property values.
        #[prost(message, tag = "4")]
        TextValues(super::TextArray),
        /// Enum property values.
        #[prost(message, tag = "5")]
        EnumValues(super::EnumArray),
        /// Nested structured data property values.
        #[prost(message, tag = "6")]
        PropertyValues(super::PropertyArray),
        /// Date time property values.
        /// It is not supported by CMEK compliant deployment.
        #[prost(message, tag = "7")]
        DateTimeValues(super::DateTimeArray),
        /// Map property values.
        #[prost(message, tag = "8")]
        MapProperty(super::MapProperty),
        /// Timestamp property values.
        /// It is not supported by CMEK compliant deployment.
        #[prost(message, tag = "9")]
        TimestampValues(super::TimestampArray),
    }
}
/// Integer values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegerArray {
    /// List of integer values.
    #[prost(int32, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<i32>,
}
/// Float values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatArray {
    /// List of float values.
    #[prost(float, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<f32>,
}
/// String/text values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextArray {
    /// List of text values.
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Enum values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumArray {
    /// List of enum values.
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// DateTime values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTimeArray {
    /// List of datetime values.
    /// Both OffsetDateTime and ZonedDateTime are supported.
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<super::super::super::r#type::DateTime>,
}
/// Timestamp values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampArray {
    /// List of timestamp values.
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<TimestampValue>,
}
/// Timestamp value type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampValue {
    #[prost(oneof = "timestamp_value::Value", tags = "1, 2")]
    pub value: ::core::option::Option<timestamp_value::Value>,
}
/// Nested message and enum types in `TimestampValue`.
pub mod timestamp_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// Timestamp value
        #[prost(message, tag = "1")]
        TimestampValue(::prost_types::Timestamp),
        /// The string must represent a valid instant in UTC and is parsed using
        /// java.time.format.DateTimeFormatter.ISO_INSTANT.
        /// e.g. "2013-09-29T18:46:19Z"
        #[prost(string, tag = "2")]
        TextValue(::prost::alloc::string::String),
    }
}
/// Property values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyArray {
    /// List of property values.
    #[prost(message, repeated, tag = "1")]
    pub properties: ::prost::alloc::vec::Vec<Property>,
}
/// Map property value.
/// Represents a structured entries of key value pairs, consisting of field names
/// which map to dynamically typed values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapProperty {
    /// Unordered map of dynamically typed values.
    #[prost(btree_map = "string, message", tag = "1")]
    pub fields: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        Value,
    >,
}
/// `Value` represents a dynamically typed value which can be either be
/// a float, a integer, a string, or a datetime value. A producer of value is
/// expected to set one of these variants. Absence of any variant indicates an
/// error.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// The kind of value.
    #[prost(oneof = "value::Kind", tags = "1, 2, 3, 4, 5, 6, 7")]
    pub kind: ::core::option::Option<value::Kind>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// The kind of value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Represents a float value.
        #[prost(float, tag = "1")]
        FloatValue(f32),
        /// Represents a integer value.
        #[prost(int32, tag = "2")]
        IntValue(i32),
        /// Represents a string value.
        #[prost(string, tag = "3")]
        StringValue(::prost::alloc::string::String),
        /// Represents an enum value.
        #[prost(message, tag = "4")]
        EnumValue(super::EnumValue),
        /// Represents a datetime value.
        #[prost(message, tag = "5")]
        DatetimeValue(super::super::super::super::r#type::DateTime),
        /// Represents a timestamp value.
        #[prost(message, tag = "6")]
        TimestampValue(super::TimestampValue),
        /// Represents a boolean value.
        #[prost(bool, tag = "7")]
        BooleanValue(bool),
    }
}
/// Represents the string value of the enum field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValue {
    /// String value of the enum field. This must match defined set of enums
    /// in document schema using EnumTypeOptions.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// When a raw document is supplied, this indicates the file format
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RawDocumentFileType {
    /// No raw document specified or it is non-parsable
    Unspecified = 0,
    /// Adobe PDF format
    Pdf = 1,
    /// Microsoft Word format
    Docx = 2,
    /// Microsoft Excel format
    Xlsx = 3,
    /// Microsoft Powerpoint format
    Pptx = 4,
    /// UTF-8 encoded text format
    Text = 5,
    /// TIFF or TIF image file format
    Tiff = 6,
}
impl RawDocumentFileType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RawDocumentFileType::Unspecified => "RAW_DOCUMENT_FILE_TYPE_UNSPECIFIED",
            RawDocumentFileType::Pdf => "RAW_DOCUMENT_FILE_TYPE_PDF",
            RawDocumentFileType::Docx => "RAW_DOCUMENT_FILE_TYPE_DOCX",
            RawDocumentFileType::Xlsx => "RAW_DOCUMENT_FILE_TYPE_XLSX",
            RawDocumentFileType::Pptx => "RAW_DOCUMENT_FILE_TYPE_PPTX",
            RawDocumentFileType::Text => "RAW_DOCUMENT_FILE_TYPE_TEXT",
            RawDocumentFileType::Tiff => "RAW_DOCUMENT_FILE_TYPE_TIFF",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RAW_DOCUMENT_FILE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "RAW_DOCUMENT_FILE_TYPE_PDF" => Some(Self::Pdf),
            "RAW_DOCUMENT_FILE_TYPE_DOCX" => Some(Self::Docx),
            "RAW_DOCUMENT_FILE_TYPE_XLSX" => Some(Self::Xlsx),
            "RAW_DOCUMENT_FILE_TYPE_PPTX" => Some(Self::Pptx),
            "RAW_DOCUMENT_FILE_TYPE_TEXT" => Some(Self::Text),
            "RAW_DOCUMENT_FILE_TYPE_TIFF" => Some(Self::Tiff),
            _ => None,
        }
    }
}
/// When a raw document or structured content is supplied, this stores the
/// content category.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContentCategory {
    /// No category is specified.
    Unspecified = 0,
    /// Content is of image type.
    Image = 1,
    /// Content is of audio type.
    Audio = 2,
    /// Content is of video type.
    Video = 3,
}
impl ContentCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContentCategory::Unspecified => "CONTENT_CATEGORY_UNSPECIFIED",
            ContentCategory::Image => "CONTENT_CATEGORY_IMAGE",
            ContentCategory::Audio => "CONTENT_CATEGORY_AUDIO",
            ContentCategory::Video => "CONTENT_CATEGORY_VIDEO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTENT_CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
            "CONTENT_CATEGORY_IMAGE" => Some(Self::Image),
            "CONTENT_CATEGORY_AUDIO" => Some(Self::Audio),
            "CONTENT_CATEGORY_VIDEO" => Some(Self::Video),
            _ => None,
        }
    }
}
/// The histogram request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramQuery {
    /// An expression specifies a histogram request against matching documents for
    /// searches.
    ///
    /// See
    /// [SearchDocumentsRequest.histogram_queries][google.cloud.contentwarehouse.v1.SearchDocumentsRequest.histogram_queries]
    /// for details about syntax.
    #[prost(string, tag = "1")]
    pub histogram_query: ::prost::alloc::string::String,
    /// Controls if the histogram query requires the return of a precise count.
    /// Enable this flag may adversely impact performance.
    ///
    /// Defaults to true.
    #[prost(bool, tag = "2")]
    pub require_precise_result_size: bool,
    /// Optional. Filter the result of histogram query by the property names. It
    /// only works with histogram query count('FilterableProperties'). It is an
    /// optional. It will perform histogram on all the property names for all the
    /// document schemas. Setting this field will have a better performance.
    #[prost(message, optional, tag = "3")]
    pub filters: ::core::option::Option<HistogramQueryPropertyNameFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramQueryPropertyNameFilter {
    /// This filter specifies the exact document schema(s)
    /// [Document.document_schema_name][google.cloud.contentwarehouse.v1.Document.document_schema_name]
    /// to run histogram query against. It is optional. It will perform histogram
    /// for property names for all the document schemas if it is not set.
    ///
    /// At most 10 document schema names are allowed.
    /// Format:
    /// projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}.
    #[prost(string, repeated, tag = "1")]
    pub document_schemas: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// It is optional. It will perform histogram for all the property names if it
    /// is not set.
    /// The properties need to be defined with the is_filterable flag set to
    /// true and the name of the property should be in the format:
    /// "schemaId.propertyName". The property needs to be defined in the schema.
    /// Example: the schema id is abc. Then the name of property for property
    /// MORTGAGE_TYPE will be "abc.MORTGAGE_TYPE".
    #[prost(string, repeated, tag = "2")]
    pub property_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// By default, the y_axis is HISTOGRAM_YAXIS_DOCUMENT if this field is not
    /// set.
    #[prost(
        enumeration = "histogram_query_property_name_filter::HistogramYAxis",
        tag = "3"
    )]
    pub y_axis: i32,
}
/// Nested message and enum types in `HistogramQueryPropertyNameFilter`.
pub mod histogram_query_property_name_filter {
    /// The result of the histogram query count('FilterableProperties') using
    /// HISTOGRAM_YAXIS_DOCUMENT will be:
    /// invoice_id: 2
    /// address: 1
    /// payment_method: 2
    /// line_item_description: 1
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
    pub enum HistogramYAxis {
        /// Count the documents per property name.
        HistogramYaxisDocument = 0,
        /// Count the properties per property name.
        HistogramYaxisProperty = 1,
    }
    impl HistogramYAxis {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HistogramYAxis::HistogramYaxisDocument => "HISTOGRAM_YAXIS_DOCUMENT",
                HistogramYAxis::HistogramYaxisProperty => "HISTOGRAM_YAXIS_PROPERTY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HISTOGRAM_YAXIS_DOCUMENT" => Some(Self::HistogramYaxisDocument),
                "HISTOGRAM_YAXIS_PROPERTY" => Some(Self::HistogramYaxisProperty),
                _ => None,
            }
        }
    }
}
/// Histogram result that matches
/// [HistogramQuery][google.cloud.contentwarehouse.v1.HistogramQuery] specified
/// in searches.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramQueryResult {
    /// Requested histogram expression.
    #[prost(string, tag = "1")]
    pub histogram_query: ::prost::alloc::string::String,
    /// A map from the values of the facet associated with distinct values to the
    /// number of matching entries with corresponding value.
    ///
    /// The key format is:
    ///
    /// * (for string histogram) string values stored in the field.
    #[prost(btree_map = "string, int64", tag = "2")]
    pub histogram: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        i64,
    >,
}
/// Request Option for processing Cloud AI Document in CW Document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudAiDocumentOption {
    /// Whether to convert all the entities to properties.
    #[prost(bool, tag = "1")]
    pub enable_entities_conversions: bool,
    /// If set, only selected entities will be converted to properties.
    #[prost(btree_map = "string, string", tag = "2")]
    pub customized_entities_properties_conversions: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Request message for DocumentService.CreateDocument.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentRequest {
    /// Required. The parent name.
    /// Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The document to create.
    #[prost(message, optional, tag = "2")]
    pub document: ::core::option::Option<Document>,
    /// The meta information collected about the end user, used to enforce access
    /// control for the service.
    #[prost(message, optional, tag = "3")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    /// Default document policy during creation.
    /// This refers to an Identity and Access (IAM) policy, which specifies access
    /// controls for the Document.
    /// Conditions defined in the policy will be ignored.
    #[prost(message, optional, tag = "4")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    /// Request Option for processing Cloud AI Document in Document Warehouse.
    /// This field offers limited support for mapping entities from Cloud AI
    /// Document to Warehouse Document. Please consult with product team before
    /// using this field and other available options.
    #[prost(message, optional, tag = "5")]
    pub cloud_ai_document_option: ::core::option::Option<CloudAiDocumentOption>,
    /// Field mask for creating Document fields. If mask path is empty,
    /// it means all fields are masked.
    /// For the `FieldMask` definition,
    /// see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask.>
    #[prost(message, optional, tag = "6")]
    pub create_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DocumentService.GetDocument.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentRequest {
    /// Required. The name of the document to retrieve.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{document_id} or
    /// projects/{project_number}/locations/{location}/documents/referenceId/{reference_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The meta information collected about the end user, used to enforce access
    /// control for the service.
    #[prost(message, optional, tag = "2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
}
/// Request message for DocumentService.UpdateDocument.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentRequest {
    /// Required. The name of the document to update.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{document_id}
    /// or
    /// projects/{project_number}/locations/{location}/documents/referenceId/{reference_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The document to update.
    #[prost(message, optional, tag = "2")]
    pub document: ::core::option::Option<Document>,
    /// The meta information collected about the end user, used to enforce access
    /// control for the service.
    #[prost(message, optional, tag = "3")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    /// Request Option for processing Cloud AI Document in Document Warehouse.
    /// This field offers limited support for mapping entities from Cloud AI
    /// Document to Warehouse Document. Please consult with product team before
    /// using this field and other available options.
    #[prost(message, optional, tag = "5")]
    pub cloud_ai_document_option: ::core::option::Option<CloudAiDocumentOption>,
    /// Options for the update operation.
    #[prost(message, optional, tag = "6")]
    pub update_options: ::core::option::Option<UpdateOptions>,
}
/// Request message for DocumentService.DeleteDocument.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentRequest {
    /// Required. The name of the document to delete.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{document_id}
    /// or
    /// projects/{project_number}/locations/{location}/documents/referenceId/{reference_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The meta information collected about the end user, used to enforce access
    /// control for the service.
    #[prost(message, optional, tag = "2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
}
/// Request message for DocumentService.SearchDocuments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchDocumentsRequest {
    /// Required. The parent, which owns this collection of documents.
    /// Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The meta information collected about the end user, used to enforce access
    /// control and improve the search quality of the service.
    #[prost(message, optional, tag = "3")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    /// Query used to search against documents (keyword, filters, etc.).
    #[prost(message, optional, tag = "4")]
    pub document_query: ::core::option::Option<DocumentQuery>,
    /// An integer that specifies the current offset (that is, starting result
    /// location, amongst the documents deemed by the API as relevant) in search
    /// results. This field is only considered if
    /// [page_token][google.cloud.contentwarehouse.v1.SearchDocumentsRequest.page_token]
    /// is unset.
    ///
    /// The maximum allowed value is 5000. Otherwise an error is thrown.
    ///
    /// For example, 0 means to  return results starting from the first matching
    /// document, and 10 means to return from the 11th document. This can be used
    /// for pagination, (for example, pageSize = 10 and offset = 10 means to return
    /// from the second page).
    #[prost(int32, tag = "5")]
    pub offset: i32,
    /// A limit on the number of documents returned in the search results.
    /// Increasing this value above the default value of 10 can increase search
    /// response time. The value can be between 1 and 100.
    #[prost(int32, tag = "6")]
    pub page_size: i32,
    /// The token specifying the current offset within search results.
    /// See
    /// [SearchDocumentsResponse.next_page_token][google.cloud.contentwarehouse.v1.SearchDocumentsResponse.next_page_token]
    /// for an explanation of how to obtain the next set of query results.
    #[prost(string, tag = "7")]
    pub page_token: ::prost::alloc::string::String,
    /// The criteria determining how search results are sorted. For non-empty
    /// query, default is `"relevance desc"`. For empty query, default is
    /// `"upload_date desc"`.
    ///
    /// Supported options are:
    ///
    /// * `"relevance desc"`: By relevance descending, as determined by the API
    ///    algorithms.
    /// * `"upload_date desc"`: By upload date descending.
    /// * `"upload_date"`: By upload date ascending.
    /// * `"update_date desc"`: By last updated date descending.
    /// * `"update_date"`: By last updated date ascending.
    /// * `"retrieval_importance desc"`: By retrieval importance of properties
    ///    descending. This feature is still under development, please do not use
    ///    unless otherwise instructed to do so.
    #[prost(string, tag = "8")]
    pub order_by: ::prost::alloc::string::String,
    /// An expression specifying a histogram request against matching
    /// documents. Expression syntax is an aggregation function call with
    /// histogram facets and other options.
    ///
    /// The following aggregation functions are supported:
    ///
    /// * `count(string_histogram_facet)`: Count the number of matching entities
    /// for each distinct attribute value.
    ///
    /// Data types:
    ///
    /// * Histogram facet (aka filterable properties): Facet names with format
    /// &lt;schema id&gt;.&lt;facet&gt;. Facets will have the
    /// format of: `[a-zA-Z][a-zA-Z0-9_:/-.]`. If the facet is a child
    /// facet, then the parent hierarchy needs to be specified separated by
    /// dots in the prefix after the schema id. Thus, the format for a multi-
    /// level facet is: &lt;schema id&gt;.&lt;parent facet name&gt;.
    /// &lt;child facet name&gt;. Example:
    /// schema123.root_parent_facet.middle_facet.child_facet
    /// * DocumentSchemaId: (with no schema id prefix) to get
    /// histograms for each document type (returns the schema id path, e.g.
    /// projects/12345/locations/us-west/documentSchemas/abc123).
    ///
    /// Example expression:
    ///
    /// * Document type counts:
    ///    count('DocumentSchemaId')
    ///
    /// * For schema id, abc123, get the counts for MORTGAGE_TYPE:
    ///    count('abc123.MORTGAGE_TYPE')
    #[prost(message, repeated, tag = "9")]
    pub histogram_queries: ::prost::alloc::vec::Vec<HistogramQuery>,
    /// Controls if the search document request requires the return of a total size
    /// of matched documents. See
    /// [SearchDocumentsResponse.total_size][google.cloud.contentwarehouse.v1.SearchDocumentsResponse.total_size].
    ///
    /// Enabling this flag may adversely impact performance. Hint: If this is
    /// used with pagination, set this flag on the initial query but set this
    /// to false on subsequent page calls (keep the total count locally).
    ///
    /// Defaults to false.
    #[prost(bool, tag = "10")]
    pub require_total_size: bool,
    /// Controls if the search document request requires the return of a total size
    /// of matched documents. See
    /// [SearchDocumentsResponse.total_size][google.cloud.contentwarehouse.v1.SearchDocumentsResponse.total_size].
    #[prost(enumeration = "search_documents_request::TotalResultSize", tag = "12")]
    pub total_result_size: i32,
    /// Experimental, do not use.
    /// The limit on the number of documents returned for the question-answering
    /// feature. To enable the question-answering feature, set
    /// \[DocumentQuery\].[is_nl_query][] to true.
    #[prost(int32, tag = "11")]
    pub qa_size_limit: i32,
}
/// Nested message and enum types in `SearchDocumentsRequest`.
pub mod search_documents_request {
    /// The total number of matching documents.
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
    pub enum TotalResultSize {
        /// Total number calculation will be skipped.
        Unspecified = 0,
        /// Estimate total number. The total result size will be accurated up to
        /// 10,000. This option will add cost and latency to your request.
        EstimatedSize = 1,
        /// It may adversely impact performance. The limit is 1000,000.
        ActualSize = 2,
    }
    impl TotalResultSize {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TotalResultSize::Unspecified => "TOTAL_RESULT_SIZE_UNSPECIFIED",
                TotalResultSize::EstimatedSize => "ESTIMATED_SIZE",
                TotalResultSize::ActualSize => "ACTUAL_SIZE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TOTAL_RESULT_SIZE_UNSPECIFIED" => Some(Self::Unspecified),
                "ESTIMATED_SIZE" => Some(Self::EstimatedSize),
                "ACTUAL_SIZE" => Some(Self::ActualSize),
                _ => None,
            }
        }
    }
}
/// Request message for DocumentService.LockDocument.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockDocumentRequest {
    /// Required. The name of the document to lock.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{document}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The collection the document connects to.
    #[prost(string, tag = "2")]
    pub collection_id: ::prost::alloc::string::String,
    /// The user information who locks the document.
    #[prost(message, optional, tag = "3")]
    pub locking_user: ::core::option::Option<UserInfo>,
}
/// Request message for DocumentService.FetchAcl
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchAclRequest {
    /// Required. REQUIRED: The resource for which the policy is being requested.
    /// Format for document:
    /// projects/{project_number}/locations/{location}/documents/{document_id}.
    /// Format for collection:
    /// projects/{project_number}/locations/{location}/collections/{collection_id}.
    /// Format for project: projects/{project_number}.
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    /// The meta information collected about the end user, used to enforce access
    /// control for the service.
    #[prost(message, optional, tag = "2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    /// For Get Project ACL only. Authorization check for end user will be ignored
    /// when project_owner=true.
    #[prost(bool, tag = "3")]
    pub project_owner: bool,
}
/// Request message for DocumentService.SetAcl.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAclRequest {
    /// Required. REQUIRED: The resource for which the policy is being requested.
    /// Format for document:
    /// projects/{project_number}/locations/{location}/documents/{document_id}.
    /// Format for collection:
    /// projects/{project_number}/locations/{location}/collections/{collection_id}.
    /// Format for project: projects/{project_number}.
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    /// Required. REQUIRED: The complete policy to be applied to the `resource`.
    /// The size of the policy is limited to a few 10s of KB. This refers to an
    /// Identity and Access (IAM) policy, which specifies access controls for the
    /// Document.
    ///
    /// You can set ACL with condition for projects only.
    ///
    /// Supported operators are: `=`, `!=`, `<`, `<=`, `>`, and `>=` where
    /// the left of the operator is `DocumentSchemaId` or property name and the
    /// right of the operator is a number or a quoted string. You must escape
    /// backslash (\\) and quote (\") characters.
    ///
    /// Boolean expressions (AND/OR) are supported up to 3 levels of nesting (for
    /// example, "((A AND B AND C) OR D) AND E"), a maximum of 10 comparisons are
    /// allowed in the expression. The expression must be < 6000 bytes in length.
    ///
    /// Sample condition:
    ///      `"DocumentSchemaId = \"some schema id\" OR SchemaId.floatPropertyName
    ///      >= 10"`
    #[prost(message, optional, tag = "2")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    /// The meta information collected about the end user, used to enforce access
    /// control for the service.
    #[prost(message, optional, tag = "3")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    /// For Set Project ACL only. Authorization check for end user will be ignored
    /// when project_owner=true.
    #[prost(bool, tag = "4")]
    pub project_owner: bool,
}
/// Response message for DocumentService.CreateDocument.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentResponse {
    /// Document created after executing create request.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// Output from Rule Engine recording the rule evaluator and action executor's
    /// output.
    ///
    /// Refer format in: google/cloud/contentwarehouse/v1/rule_engine.proto
    #[prost(message, optional, tag = "2")]
    pub rule_engine_output: ::core::option::Option<RuleEngineOutput>,
    /// Additional information for the API invocation, such as the request tracking
    /// id.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
    /// post-processing LROs
    #[prost(message, repeated, tag = "4")]
    pub long_running_operations: ::prost::alloc::vec::Vec<
        super::super::super::longrunning::Operation,
    >,
}
/// Response message for DocumentService.UpdateDocument.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentResponse {
    /// Updated document after executing update request.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// Output from Rule Engine recording the rule evaluator and action executor's
    /// output.
    ///
    /// Refer format in: google/cloud/contentwarehouse/v1/rule_engine.proto
    #[prost(message, optional, tag = "2")]
    pub rule_engine_output: ::core::option::Option<RuleEngineOutput>,
    /// Additional information for the API invocation, such as the request tracking
    /// id.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
/// Additional result info for the question-answering feature.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QaResult {
    /// Highlighted sections in the snippet.
    #[prost(message, repeated, tag = "1")]
    pub highlights: ::prost::alloc::vec::Vec<qa_result::Highlight>,
    /// The calibrated confidence score for this document, in the range
    /// \[0., 1.\]. This represents the confidence level for whether the returned
    /// document and snippet answers the user's query.
    #[prost(float, tag = "2")]
    pub confidence_score: f32,
}
/// Nested message and enum types in `QAResult`.
pub mod qa_result {
    /// A text span in the search text snippet that represents a highlighted
    /// section (answer context, highly relevant sentence, etc.).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Highlight {
        /// Start index of the highlight.
        #[prost(int32, tag = "1")]
        pub start_index: i32,
        /// End index of the highlight, exclusive.
        #[prost(int32, tag = "2")]
        pub end_index: i32,
    }
}
/// Response message for DocumentService.SearchDocuments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchDocumentsResponse {
    /// The document entities that match the specified
    /// [SearchDocumentsRequest][google.cloud.contentwarehouse.v1.SearchDocumentsRequest].
    #[prost(message, repeated, tag = "1")]
    pub matching_documents: ::prost::alloc::vec::Vec<
        search_documents_response::MatchingDocument,
    >,
    /// The token that specifies the starting position of the next page of results.
    /// This field is empty if there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of matched documents which is available only if the client
    /// set
    /// [SearchDocumentsRequest.require_total_size][google.cloud.contentwarehouse.v1.SearchDocumentsRequest.require_total_size]
    /// to `true` or set
    /// [SearchDocumentsRequest.total_result_size][google.cloud.contentwarehouse.v1.SearchDocumentsRequest.total_result_size]
    /// to `ESTIMATED_SIZE` or `ACTUAL_SIZE`. Otherwise, the value will be `-1`.
    /// Typically a UI would handle this condition by displaying &quot;of
    /// many&quot;, for example: &quot;Displaying 10 of many&quot;.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
    /// Additional information for the API invocation, such as the request tracking
    /// id.
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
    /// The histogram results that match with the specified
    /// [SearchDocumentsRequest.histogram_queries][google.cloud.contentwarehouse.v1.SearchDocumentsRequest.histogram_queries].
    #[prost(message, repeated, tag = "6")]
    pub histogram_query_results: ::prost::alloc::vec::Vec<HistogramQueryResult>,
    /// Experimental.
    /// Question answer from the query against the document.
    #[prost(string, tag = "7")]
    pub question_answer: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SearchDocumentsResponse`.
pub mod search_documents_response {
    /// Document entry with metadata inside
    /// [SearchDocumentsResponse][google.cloud.contentwarehouse.v1.SearchDocumentsResponse]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchingDocument {
        /// Document that matches the specified
        /// [SearchDocumentsRequest][google.cloud.contentwarehouse.v1.SearchDocumentsRequest].
        /// This document only contains indexed metadata information.
        #[prost(message, optional, tag = "1")]
        pub document: ::core::option::Option<super::Document>,
        /// Contains snippets of text from the document full raw text that most
        /// closely match a search query's keywords, if available. All HTML tags in
        /// the original fields are stripped when returned in this field, and
        /// matching query keywords are enclosed in HTML bold tags.
        ///
        /// If the question-answering feature is enabled, this field will instead
        /// contain a snippet that answers the user's natural-language query. No HTML
        /// bold tags will be present, and highlights in the answer snippet can be
        /// found in
        /// [QAResult.highlights][google.cloud.contentwarehouse.v1.QAResult.highlights].
        #[prost(string, tag = "2")]
        pub search_text_snippet: ::prost::alloc::string::String,
        /// Experimental.
        /// Additional result info if the question-answering feature is enabled.
        #[prost(message, optional, tag = "3")]
        pub qa_result: ::core::option::Option<super::QaResult>,
        /// Return the 1-based page indices where those pages have one or more
        /// matched tokens.
        #[prost(int64, repeated, tag = "4")]
        pub matched_token_page_indices: ::prost::alloc::vec::Vec<i64>,
    }
}
/// Response message for DocumentService.FetchAcl.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchAclResponse {
    /// The IAM policy.
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    /// Additional information for the API invocation, such as the request tracking
    /// id.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
/// Response message for DocumentService.SetAcl.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAclResponse {
    /// The policy will be attached to a resource (e.g. projecct, document).
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    /// Additional information for the API invocation, such as the request tracking
    /// id.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
/// Generated client implementations.
pub mod document_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// This service lets you manage document.
    #[derive(Debug, Clone)]
    pub struct DocumentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DocumentServiceClient<T>
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
        ) -> DocumentServiceClient<InterceptedService<T, F>>
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
            DocumentServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a document.
        pub async fn create_document(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateDocumentResponse>,
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
                "/google.cloud.contentwarehouse.v1.DocumentService/CreateDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentService",
                        "CreateDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a document. Returns NOT_FOUND if the document does not exist.
        pub async fn get_document(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDocumentRequest>,
        ) -> std::result::Result<tonic::Response<super::Document>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.contentwarehouse.v1.DocumentService/GetDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentService",
                        "GetDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a document. Returns INVALID_ARGUMENT if the name of the document
        /// is non-empty and does not equal the existing name.
        pub async fn update_document(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateDocumentResponse>,
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
                "/google.cloud.contentwarehouse.v1.DocumentService/UpdateDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentService",
                        "UpdateDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a document. Returns NOT_FOUND if the document does not exist.
        pub async fn delete_document(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDocumentRequest>,
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
                "/google.cloud.contentwarehouse.v1.DocumentService/DeleteDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentService",
                        "DeleteDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Searches for documents using provided
        /// [SearchDocumentsRequest][google.cloud.contentwarehouse.v1.SearchDocumentsRequest].
        /// This call only returns documents that the caller has permission to search
        /// against.
        pub async fn search_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchDocumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchDocumentsResponse>,
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
                "/google.cloud.contentwarehouse.v1.DocumentService/SearchDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentService",
                        "SearchDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lock the document so the document cannot be updated by other users.
        pub async fn lock_document(
            &mut self,
            request: impl tonic::IntoRequest<super::LockDocumentRequest>,
        ) -> std::result::Result<tonic::Response<super::Document>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.contentwarehouse.v1.DocumentService/LockDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentService",
                        "LockDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the access control policy for a resource. Returns NOT_FOUND error if
        /// the resource does not exist. Returns an empty policy if the resource exists
        /// but does not have a policy set.
        pub async fn fetch_acl(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchAclRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchAclResponse>,
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
                "/google.cloud.contentwarehouse.v1.DocumentService/FetchAcl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentService",
                        "FetchAcl",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the access control policy for a resource. Replaces any existing
        /// policy.
        pub async fn set_acl(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAclRequest>,
        ) -> std::result::Result<tonic::Response<super::SetAclResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.contentwarehouse.v1.DocumentService/SetAcl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentService",
                        "SetAcl",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request message for DocumentSchemaService.CreateDocumentSchema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentSchemaRequest {
    /// Required. The parent name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The document schema to create.
    #[prost(message, optional, tag = "2")]
    pub document_schema: ::core::option::Option<DocumentSchema>,
}
/// Request message for DocumentSchemaService.GetDocumentSchema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentSchemaRequest {
    /// Required. The name of the document schema to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DocumentSchemaService.UpdateDocumentSchema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentSchemaRequest {
    /// Required. The name of the document schema to update.
    /// Format:
    /// projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The document schema to update with.
    #[prost(message, optional, tag = "2")]
    pub document_schema: ::core::option::Option<DocumentSchema>,
}
/// Request message for DocumentSchemaService.DeleteDocumentSchema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentSchemaRequest {
    /// Required. The name of the document schema to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DocumentSchemaService.ListDocumentSchemas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentSchemasRequest {
    /// Required. The parent, which owns this collection of document schemas.
    /// Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of document schemas to return. The service may return
    /// fewer than this value.
    /// If unspecified, at most 50 document schemas will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListDocumentSchemas` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListDocumentSchemas`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for DocumentSchemaService.ListDocumentSchemas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentSchemasResponse {
    /// The document schemas from the specified parent.
    #[prost(message, repeated, tag = "1")]
    pub document_schemas: ::prost::alloc::vec::Vec<DocumentSchema>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod document_schema_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// This service lets you manage document schema.
    #[derive(Debug, Clone)]
    pub struct DocumentSchemaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DocumentSchemaServiceClient<T>
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
        ) -> DocumentSchemaServiceClient<InterceptedService<T, F>>
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
            DocumentSchemaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a document schema.
        pub async fn create_document_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDocumentSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::DocumentSchema>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.contentwarehouse.v1.DocumentSchemaService/CreateDocumentSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentSchemaService",
                        "CreateDocumentSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a Document Schema. Returns INVALID_ARGUMENT if the name of the
        /// Document Schema is non-empty and does not equal the existing name.
        /// Supports only appending new properties, adding new ENUM possible values,
        /// and updating the
        /// [EnumTypeOptions.validation_check_disabled][google.cloud.contentwarehouse.v1.EnumTypeOptions.validation_check_disabled]
        /// flag for ENUM possible values. Updating existing properties will result
        /// into INVALID_ARGUMENT.
        pub async fn update_document_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDocumentSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::DocumentSchema>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.contentwarehouse.v1.DocumentSchemaService/UpdateDocumentSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentSchemaService",
                        "UpdateDocumentSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a document schema. Returns NOT_FOUND if the document schema does not
        /// exist.
        pub async fn get_document_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDocumentSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::DocumentSchema>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.contentwarehouse.v1.DocumentSchemaService/GetDocumentSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentSchemaService",
                        "GetDocumentSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a document schema. Returns NOT_FOUND if the document schema does
        /// not exist. Returns BAD_REQUEST if the document schema has documents
        /// depending on it.
        pub async fn delete_document_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDocumentSchemaRequest>,
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
                "/google.cloud.contentwarehouse.v1.DocumentSchemaService/DeleteDocumentSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentSchemaService",
                        "DeleteDocumentSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists document schemas.
        pub async fn list_document_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDocumentSchemasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDocumentSchemasResponse>,
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
                "/google.cloud.contentwarehouse.v1.DocumentSchemaService/ListDocumentSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentSchemaService",
                        "ListDocumentSchemas",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod rule_set_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage customer specific RuleSets.
    #[derive(Debug, Clone)]
    pub struct RuleSetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RuleSetServiceClient<T>
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
        ) -> RuleSetServiceClient<InterceptedService<T, F>>
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
            RuleSetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a ruleset.
        pub async fn create_rule_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRuleSetRequest>,
        ) -> std::result::Result<tonic::Response<super::RuleSet>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.contentwarehouse.v1.RuleSetService/CreateRuleSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.RuleSetService",
                        "CreateRuleSet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a ruleset. Returns NOT_FOUND if the ruleset does not exist.
        pub async fn get_rule_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRuleSetRequest>,
        ) -> std::result::Result<tonic::Response<super::RuleSet>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.contentwarehouse.v1.RuleSetService/GetRuleSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.RuleSetService",
                        "GetRuleSet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a ruleset. Returns INVALID_ARGUMENT if the name of the ruleset
        /// is non-empty and does not equal the existing name.
        pub async fn update_rule_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRuleSetRequest>,
        ) -> std::result::Result<tonic::Response<super::RuleSet>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.contentwarehouse.v1.RuleSetService/UpdateRuleSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.RuleSetService",
                        "UpdateRuleSet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a ruleset. Returns NOT_FOUND if the document does not exist.
        pub async fn delete_rule_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRuleSetRequest>,
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
                "/google.cloud.contentwarehouse.v1.RuleSetService/DeleteRuleSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.RuleSetService",
                        "DeleteRuleSet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists rulesets.
        pub async fn list_rule_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuleSetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRuleSetsResponse>,
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
                "/google.cloud.contentwarehouse.v1.RuleSetService/ListRuleSets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.RuleSetService",
                        "ListRuleSets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Response message for DocumentLinkService.ListLinkedTargets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLinkedTargetsResponse {
    /// Target document-links.
    #[prost(message, repeated, tag = "1")]
    pub document_links: ::prost::alloc::vec::Vec<DocumentLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DocumentLinkService.ListLinkedTargets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLinkedTargetsRequest {
    /// Required. The name of the document, for which all target links are
    /// returned. Format:
    /// projects/{project_number}/locations/{location}/documents/{target_document_id}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The meta information collected about the document creator, used to enforce
    /// access control for the service.
    #[prost(message, optional, tag = "2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
}
/// Response message for DocumentLinkService.ListLinkedSources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLinkedSourcesResponse {
    /// Source document-links.
    #[prost(message, repeated, tag = "1")]
    pub document_links: ::prost::alloc::vec::Vec<DocumentLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Response message for DocumentLinkService.ListLinkedSources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLinkedSourcesRequest {
    /// Required. The name of the document, for which all source links are
    /// returned. Format:
    /// projects/{project_number}/locations/{location}/documents/{source_document_id}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of document-links to return. The service may return
    /// fewer than this value.
    ///
    /// If unspecified, at most 50 document-links will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// A page token, received from a previous `ListLinkedSources` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListLinkedSources`
    /// must match the call that provided the page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// The meta information collected about the document creator, used to enforce
    /// access control for the service.
    #[prost(message, optional, tag = "2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
}
/// A document-link between source and target document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentLink {
    /// Name of this document-link.
    /// It is required that the parent derived form the name to be consistent with
    /// the source document reference. Otherwise an exception will be thrown.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{source_document_id}/documentLinks/{document_link_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Document references of the source document.
    #[prost(message, optional, tag = "2")]
    pub source_document_reference: ::core::option::Option<DocumentReference>,
    /// Document references of the target document.
    #[prost(message, optional, tag = "3")]
    pub target_document_reference: ::core::option::Option<DocumentReference>,
    /// Description of this document-link.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The time when the documentLink is last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the documentLink is created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The state of the documentlink. If target node has been deleted, the
    /// link is marked as invalid. Removing a source node will result in removal
    /// of all associated links.
    #[prost(enumeration = "document_link::State", tag = "7")]
    pub state: i32,
}
/// Nested message and enum types in `DocumentLink`.
pub mod document_link {
    /// The state of a document-link.
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
        /// Unknown state of documentlink.
        Unspecified = 0,
        /// The documentlink has both source and target documents detected.
        Active = 1,
        /// Target document is deleted, and mark the documentlink as soft-deleted.
        SoftDeleted = 2,
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
                State::SoftDeleted => "SOFT_DELETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "SOFT_DELETED" => Some(Self::SoftDeleted),
                _ => None,
            }
        }
    }
}
/// Request message for DocumentLinkService.CreateDocumentLink.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentLinkRequest {
    /// Required. Parent of the document-link to be created.
    /// parent of document-link should be a document.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{source_document_id}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Document links associated with the source documents
    /// (source_document_id).
    #[prost(message, optional, tag = "2")]
    pub document_link: ::core::option::Option<DocumentLink>,
    /// The meta information collected about the document creator, used to enforce
    /// access control for the service.
    #[prost(message, optional, tag = "3")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
}
/// Request message for DocumentLinkService.DeleteDocumentLink.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentLinkRequest {
    /// Required. The name of the document-link to be deleted.
    /// Format:
    /// projects/{project_number}/locations/{location}/documents/{source_document_id}/documentLinks/{document_link_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The meta information collected about the document creator, used to enforce
    /// access control for the service.
    #[prost(message, optional, tag = "2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
}
/// Generated client implementations.
pub mod document_link_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// This service lets you manage document-links.
    /// Document-Links are treated as sub-resources under source documents.
    #[derive(Debug, Clone)]
    pub struct DocumentLinkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DocumentLinkServiceClient<T>
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
        ) -> DocumentLinkServiceClient<InterceptedService<T, F>>
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
            DocumentLinkServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Return all target document-links from the document.
        pub async fn list_linked_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLinkedTargetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLinkedTargetsResponse>,
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
                "/google.cloud.contentwarehouse.v1.DocumentLinkService/ListLinkedTargets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentLinkService",
                        "ListLinkedTargets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Return all source document-links from the document.
        pub async fn list_linked_sources(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLinkedSourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLinkedSourcesResponse>,
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
                "/google.cloud.contentwarehouse.v1.DocumentLinkService/ListLinkedSources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentLinkService",
                        "ListLinkedSources",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create a link between a source document and a target document.
        pub async fn create_document_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDocumentLinkRequest>,
        ) -> std::result::Result<tonic::Response<super::DocumentLink>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.contentwarehouse.v1.DocumentLinkService/CreateDocumentLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentLinkService",
                        "CreateDocumentLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Remove the link between the source and target documents.
        pub async fn delete_document_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDocumentLinkRequest>,
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
                "/google.cloud.contentwarehouse.v1.DocumentLinkService/DeleteDocumentLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.contentwarehouse.v1.DocumentLinkService",
                        "DeleteDocumentLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Metadata object for CreateDocument request (currently empty).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentMetadata {}
/// Metadata object for UpdateDocument request (currently empty).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentMetadata {}
