/// An asset represents a resource in your environment. Asset types include
/// virtual machines and databases.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// Output only. The full name of the asset.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the asset was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the asset was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Generic asset attributes.
    #[prost(btree_map = "string, string", tag = "5")]
    pub attributes: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The list of insights associated with the asset.
    #[prost(message, optional, tag = "20")]
    pub insight_list: ::core::option::Option<InsightList>,
    /// Output only. Performance data for the asset.
    #[prost(message, optional, tag = "21")]
    pub performance_data: ::core::option::Option<AssetPerformanceData>,
    /// Output only. The list of sources contributing to the asset.
    #[prost(string, repeated, tag = "22")]
    pub sources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The list of groups that the asset is assigned to.
    #[prost(string, repeated, tag = "23")]
    pub assigned_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The internal information of an asset. Only one field can be specified
    /// with different attributes based on the type of the asset.
    #[prost(oneof = "asset::AssetDetails", tags = "6")]
    pub asset_details: ::core::option::Option<asset::AssetDetails>,
}
/// Nested message and enum types in `Asset`.
pub mod asset {
    /// The internal information of an asset. Only one field can be specified
    /// with different attributes based on the type of the asset.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AssetDetails {
        /// Output only. Asset information specific for virtual and physical
        /// machines.
        #[prost(message, tag = "6")]
        MachineDetails(super::MachineDetails),
    }
}
/// The preferences that apply to all assets in a given context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreferenceSet {
    /// Output only. Name of the preference set.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the preference set was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the preference set was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-friendly display name. Maximum length is 63 characters.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// A description of the preference set.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// A set of preferences that applies to all virtual machines in the context.
    #[prost(message, optional, tag = "6")]
    pub virtual_machine_preferences: ::core::option::Option<VirtualMachinePreferences>,
}
/// A resource that represents the background job that imports asset frames.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportJob {
    /// Output only. The full name of the import job.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User-friendly display name. Maximum length is 63 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the import job was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the import job was last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the import job was completed.
    #[prost(message, optional, tag = "5")]
    pub complete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The state of the import job.
    #[prost(enumeration = "import_job::ImportJobState", tag = "6")]
    pub state: i32,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "7")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. Reference to a source.
    #[prost(string, tag = "8")]
    pub asset_source: ::prost::alloc::string::String,
    /// Holds the various report types of an import job.
    #[prost(oneof = "import_job::Report", tags = "10, 11")]
    pub report: ::core::option::Option<import_job::Report>,
}
/// Nested message and enum types in `ImportJob`.
pub mod import_job {
    /// Enumerates possible states of an import job.
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
    pub enum ImportJobState {
        /// Default value.
        Unspecified = 0,
        /// The import job is pending.
        Pending = 1,
        /// The processing of the import job is ongoing.
        Running = 2,
        /// The import job processing has completed.
        Completed = 3,
        /// The import job failed to be processed.
        Failed = 4,
        /// The import job is being validated.
        Validating = 5,
        /// The import job contains blocking errors.
        FailedValidation = 6,
        /// The validation of the job completed with no blocking errors.
        Ready = 7,
    }
    impl ImportJobState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ImportJobState::Unspecified => "IMPORT_JOB_STATE_UNSPECIFIED",
                ImportJobState::Pending => "IMPORT_JOB_STATE_PENDING",
                ImportJobState::Running => "IMPORT_JOB_STATE_RUNNING",
                ImportJobState::Completed => "IMPORT_JOB_STATE_COMPLETED",
                ImportJobState::Failed => "IMPORT_JOB_STATE_FAILED",
                ImportJobState::Validating => "IMPORT_JOB_STATE_VALIDATING",
                ImportJobState::FailedValidation => "IMPORT_JOB_STATE_FAILED_VALIDATION",
                ImportJobState::Ready => "IMPORT_JOB_STATE_READY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IMPORT_JOB_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "IMPORT_JOB_STATE_PENDING" => Some(Self::Pending),
                "IMPORT_JOB_STATE_RUNNING" => Some(Self::Running),
                "IMPORT_JOB_STATE_COMPLETED" => Some(Self::Completed),
                "IMPORT_JOB_STATE_FAILED" => Some(Self::Failed),
                "IMPORT_JOB_STATE_VALIDATING" => Some(Self::Validating),
                "IMPORT_JOB_STATE_FAILED_VALIDATION" => Some(Self::FailedValidation),
                "IMPORT_JOB_STATE_READY" => Some(Self::Ready),
                _ => None,
            }
        }
    }
    /// Holds the various report types of an import job.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Report {
        /// Output only. The report with the validation results of the import job.
        #[prost(message, tag = "10")]
        ValidationReport(super::ValidationReport),
        /// Output only. The report with the results of running the import job.
        #[prost(message, tag = "11")]
        ExecutionReport(super::ExecutionReport),
    }
}
/// A resource that represents a payload file in an import job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataFile {
    /// Output only. The name of the file.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User-friendly display name. Maximum length is 63 characters.
    #[prost(string, tag = "6")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The payload format.
    #[prost(enumeration = "ImportJobFormat", tag = "2")]
    pub format: i32,
    /// Output only. The timestamp when the file was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The state of the import data file.
    #[prost(enumeration = "import_data_file::State", tag = "4")]
    pub state: i32,
    #[prost(oneof = "import_data_file::FileInfo", tags = "5")]
    pub file_info: ::core::option::Option<import_data_file::FileInfo>,
}
/// Nested message and enum types in `ImportDataFile`.
pub mod import_data_file {
    /// Enumerates possible states of an import data file.
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
        /// Default value.
        Unspecified = 0,
        /// The data file is being created.
        Creating = 1,
        /// The data file completed initialization.
        Active = 2,
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
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FileInfo {
        /// Information about a file that is uploaded to a storage service.
        #[prost(message, tag = "5")]
        UploadFileInfo(super::UploadFileInfo),
    }
}
/// A resource that represents an asset group.
/// The purpose of an asset group is to bundle a set of assets that have
/// something in common, while allowing users to add annotations to the group.
/// An asset can belong to multiple groups.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    /// Output only. The name of the group.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the group was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the group was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// User-friendly display name.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// The description of the resource.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
}
/// Message representing a frame which failed to be processed due to an error.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorFrame {
    /// Output only. The identifier of the ErrorFrame.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. All the violations that were detected for the frame.
    #[prost(message, repeated, tag = "2")]
    pub violations: ::prost::alloc::vec::Vec<FrameViolationEntry>,
    /// Output only. The frame that was originally reported.
    #[prost(message, optional, tag = "3")]
    pub original_frame: ::core::option::Option<AssetFrame>,
    /// Output only. Frame ingestion time.
    #[prost(message, optional, tag = "4")]
    pub ingestion_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Source represents an object from which asset information is
/// streamed to Migration Center.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    /// Output only. The full name of the source.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the source was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the source was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-friendly display name.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// Free-text description.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Data source type.
    #[prost(enumeration = "source::SourceType", tag = "6")]
    pub r#type: i32,
    /// The information confidence of the source.
    /// The higher the value, the higher the confidence.
    #[prost(int32, tag = "7")]
    pub priority: i32,
    /// If `true`, the source is managed by other service(s).
    #[prost(bool, tag = "8")]
    pub managed: bool,
    /// Output only. Number of frames that are still being processed.
    #[prost(int32, tag = "9")]
    pub pending_frame_count: i32,
    /// Output only. The number of frames that were reported by the source and
    /// contained errors.
    #[prost(int32, tag = "10")]
    pub error_frame_count: i32,
    /// Output only. The state of the source.
    #[prost(enumeration = "source::State", tag = "11")]
    pub state: i32,
}
/// Nested message and enum types in `Source`.
pub mod source {
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
        /// Unspecified
        Unknown = 0,
        /// Manually uploaded file (e.g. CSV)
        Upload = 1,
        /// Guest-level info
        GuestOsScan = 2,
        /// Inventory-level scan
        InventoryScan = 3,
        /// Third-party owned sources.
        Custom = 4,
    }
    impl SourceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SourceType::Unknown => "SOURCE_TYPE_UNKNOWN",
                SourceType::Upload => "SOURCE_TYPE_UPLOAD",
                SourceType::GuestOsScan => "SOURCE_TYPE_GUEST_OS_SCAN",
                SourceType::InventoryScan => "SOURCE_TYPE_INVENTORY_SCAN",
                SourceType::Custom => "SOURCE_TYPE_CUSTOM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SOURCE_TYPE_UNKNOWN" => Some(Self::Unknown),
                "SOURCE_TYPE_UPLOAD" => Some(Self::Upload),
                "SOURCE_TYPE_GUEST_OS_SCAN" => Some(Self::GuestOsScan),
                "SOURCE_TYPE_INVENTORY_SCAN" => Some(Self::InventoryScan),
                "SOURCE_TYPE_CUSTOM" => Some(Self::Custom),
                _ => None,
            }
        }
    }
    /// Enumerates possible states of a source.
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
        /// Unspecified.
        Unspecified = 0,
        /// The source is active and ready to be used.
        Active = 1,
        /// In the process of being deleted.
        Deleting = 2,
        /// Source is in an invalid state. Asset frames reported to it will be
        /// ignored.
        Invalid = 3,
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
                State::Deleting => "DELETING",
                State::Invalid => "INVALID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "DELETING" => Some(Self::Deleting),
                "INVALID" => Some(Self::Invalid),
                _ => None,
            }
        }
    }
}
/// The groups and associated preference sets on which
/// we can generate reports.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportConfig {
    /// Output only. Name of resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-friendly display name. Maximum length is 63 characters.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// Free-text description.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Required. Collection of combinations of groups and preference sets.
    #[prost(message, repeated, tag = "6")]
    pub group_preferenceset_assignments: ::prost::alloc::vec::Vec<
        report_config::GroupPreferenceSetAssignment,
    >,
}
/// Nested message and enum types in `ReportConfig`.
pub mod report_config {
    /// Represents a combination of a group with a preference set.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupPreferenceSetAssignment {
        /// Required. Name of the group.
        #[prost(string, tag = "1")]
        pub group: ::prost::alloc::string::String,
        /// Required. Name of the Preference Set.
        #[prost(string, tag = "2")]
        pub preference_set: ::prost::alloc::string::String,
    }
}
/// Report represents a point-in-time rendering of the ReportConfig results.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Report {
    /// Output only. Name of resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Creation timestamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update timestamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-friendly display name. Maximum length is 63 characters.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// Free-text description.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Report type.
    #[prost(enumeration = "report::Type", tag = "6")]
    pub r#type: i32,
    /// Report creation state.
    #[prost(enumeration = "report::State", tag = "7")]
    pub state: i32,
    /// Output only. Summary view of the Report.
    #[prost(message, optional, tag = "8")]
    pub summary: ::core::option::Option<ReportSummary>,
}
/// Nested message and enum types in `Report`.
pub mod report {
    /// Report type.
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
        /// Default Report type.
        Unspecified = 0,
        /// Total cost of ownership Report type.
        TotalCostOfOwnership = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::TotalCostOfOwnership => "TOTAL_COST_OF_OWNERSHIP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "TOTAL_COST_OF_OWNERSHIP" => Some(Self::TotalCostOfOwnership),
                _ => None,
            }
        }
    }
    /// Report creation state.
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
        /// Default Report creation state.
        Unspecified = 0,
        /// Creating Report.
        Pending = 1,
        /// Successfully created Report.
        Succeeded = 2,
        /// Failed to create Report.
        Failed = 3,
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
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
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
    /// of the operation. Operations that have been cancelled successfully
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Message for requesting a list of assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsRequest {
    /// Required. Parent value for `ListAssetsRequest`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to sort by. See <https://google.aip.dev/132#ordering> for more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// View of the assets. Defaults to BASIC.
    #[prost(enumeration = "AssetView", tag = "6")]
    pub view: i32,
}
/// Response message for listing assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsResponse {
    /// A list of assets.
    #[prost(message, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// View of the assets. Defaults to BASIC.
    #[prost(enumeration = "AssetView", tag = "2")]
    pub view: i32,
}
/// A request to update an asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAssetRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `Asset` resource by the update.
    /// The values specified in the `update_mask` field are relative to the
    /// resource, not the full request.
    /// A field will be overwritten if it is in the mask.
    /// A single * value in the mask lets you to overwrite all fields.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag = "2")]
    pub asset: ::core::option::Option<Asset>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to update a list of assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateAssetsRequest {
    /// Required. Parent value for batch asset update.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request message specifying the resources to update.
    /// A maximum of 1000 assets can be modified in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<UpdateAssetRequest>,
}
/// Response for updating a list of assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateAssetsResponse {
    /// Update asset content.
    /// The content only includes values after field mask being applied.
    #[prost(message, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
}
/// A request to delete an asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAssetRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to delete a list of  asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteAssetsRequest {
    /// Required. Parent value for batch asset delete.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The IDs of the assets to delete.
    /// A maximum of 1000 assets can be deleted in a batch.
    /// Format: projects/{project}/locations/{location}/assets/{name}.
    #[prost(string, repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. When this value is set to `true` the request is a no-op for
    /// non-existing assets. See <https://google.aip.dev/135#delete-if-existing> for
    /// additional details. Default value is `false`.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
}
/// A request to report a set of asset frames.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportAssetFramesRequest {
    /// Required. Parent of the resource.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Collection of frames data.
    #[prost(message, optional, tag = "2")]
    pub frames: ::core::option::Option<Frames>,
    /// Required. Reference to a source.
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
}
/// A response to a call to `ReportAssetFrame`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportAssetFramesResponse {}
/// A request to aggregate one or more values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateAssetsValuesRequest {
    /// Required. Parent value for `AggregateAssetsValuesRequest`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Array of aggregations to perform.
    /// Up to 25 aggregations can be defined.
    #[prost(message, repeated, tag = "2")]
    pub aggregations: ::prost::alloc::vec::Vec<Aggregation>,
    /// The aggregation will be performed on assets that match the provided filter.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
}
/// A response to a request to aggregated assets values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateAssetsValuesResponse {
    /// The aggregation results.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<AggregationResult>,
}
/// A request to create an import job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateImportJobRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the import job.
    #[prost(string, tag = "2")]
    pub import_job_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub import_job: ::core::option::Option<ImportJob>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to list import jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImportJobsRequest {
    /// Required. Parent value for `ListImportJobsRequest`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to sort by. See <https://google.aip.dev/132#ordering> for more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. The level of details of each import job.
    /// Default value is BASIC.
    #[prost(enumeration = "ImportJobView", tag = "6")]
    pub view: i32,
}
/// A response for listing import jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImportJobsResponse {
    /// The list of import jobs.
    #[prost(message, repeated, tag = "1")]
    pub import_jobs: ::prost::alloc::vec::Vec<ImportJob>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A request to get an import job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetImportJobRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The level of details of the import job.
    /// Default value is FULL.
    #[prost(enumeration = "ImportJobView", tag = "2")]
    pub view: i32,
}
/// A request to delete an import job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteImportJobRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to `true`, any `ImportDataFiles` of this job will also be
    /// deleted If set to `false`, the request only works if the job has no data
    /// files.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// A request to update an import job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateImportJobRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `Asset` resource by the update.
    /// The values specified in the `update_mask` field are relative to the
    /// resource, not the full request.
    /// A field will be overwritten if it is in the mask.
    /// A single * value in the mask lets you to overwrite all fields.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated
    #[prost(message, optional, tag = "2")]
    pub import_job: ::core::option::Option<ImportJob>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to validate an import job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateImportJobRequest {
    /// Required. The name of the import job to validate.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to run an import job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunImportJobRequest {
    /// Required. The name of the import job to run.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to get an import data file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetImportDataFileRequest {
    /// Required. Name of the ImportDataFile.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to list import data files of an import job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImportDataFilesRequest {
    /// Required. Name of the parent of the `ImportDataFiles` resource.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of data files to return. The service may return
    /// fewer than this value. If unspecified, at most 500 data files will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListImportDataFiles` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListImportDataFiles`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to sort by. See <https://google.aip.dev/132#ordering> for more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for listing payload files of an import job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImportDataFilesResponse {
    /// The list of import data files.
    #[prost(message, repeated, tag = "1")]
    pub import_data_files: ::prost::alloc::vec::Vec<ImportDataFile>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A request to create an `ImportDataFile` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateImportDataFileRequest {
    /// Required. Name of the parent of the ImportDataFile.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the new data file.
    #[prost(string, tag = "2")]
    pub import_data_file_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub import_data_file: ::core::option::Option<ImportDataFile>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to delete an `ImportDataFile` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteImportDataFileRequest {
    /// Required. Name of the ImportDataFile to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to list groups.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupsRequest {
    /// Required. Parent value for `ListGroupsRequest`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to sort by. See <https://google.aip.dev/132#ordering> for more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// A response for listing groups.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupsResponse {
    /// The list of Group
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<Group>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A request to get a group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGroupRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to create a group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGroupRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User specified ID for the group. It will become the last
    /// component of the group name. The ID must be unique within the project, must
    /// conform with RFC-1034, is restricted to lower-cased letters, and has a
    /// maximum length of 63 characters. The ID must match the regular expression:
    /// `\[a-z]([a-z0-9-]{0,61}[a-z0-9\])?`.
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    /// Required. The group resource being created.
    #[prost(message, optional, tag = "3")]
    pub group: ::core::option::Option<Group>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to update a group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroupRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `Group` resource by the update.
    /// The values specified in the `update_mask` are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask.
    /// A single * value in the mask lets you to overwrite all fields.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The group resource being updated.
    #[prost(message, optional, tag = "2")]
    pub group: ::core::option::Option<Group>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to delete a group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGroupRequest {
    /// Required. Name of the group resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to add assets to a group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAssetsToGroupRequest {
    /// Required. Group reference.
    #[prost(string, tag = "1")]
    pub group: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Required. List of assets to be added.
    /// The maximum number of assets that can be added in a single request is
    /// 1000.
    #[prost(message, optional, tag = "3")]
    pub assets: ::core::option::Option<AssetList>,
    /// Optional. When this value is set to `false` and one of the given assets is
    /// already an existing member of the group, the operation fails with an
    /// `Already Exists` error. When set to `true` this situation is silently
    /// ignored by the server.
    ///
    /// Default value is `false`.
    #[prost(bool, tag = "4")]
    pub allow_existing: bool,
}
/// A request to remove assets from a group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAssetsFromGroupRequest {
    /// Required. Group reference.
    #[prost(string, tag = "1")]
    pub group: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Required. List of assets to be removed.
    /// The maximum number of assets that can be removed in a single request is
    /// 1000.
    #[prost(message, optional, tag = "3")]
    pub assets: ::core::option::Option<AssetList>,
    /// Optional. When this value is set to `false` and one of the given assets is
    /// not an existing member of the group, the operation fails with a `Not Found`
    /// error. When set to `true` this situation is silently ignored by the server.
    ///
    /// Default value is `false`.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// A request to list error frames for a source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListErrorFramesRequest {
    /// Required. Parent value (the source) for `ListErrorFramesRequest`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. An optional view mode to control the level of details of each
    /// error frame. The default is a BASIC frame view.
    #[prost(enumeration = "ErrorFrameView", tag = "4")]
    pub view: i32,
}
/// A response for listing error frames.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListErrorFramesResponse {
    /// The list of error frames.
    #[prost(message, repeated, tag = "1")]
    pub error_frames: ::prost::alloc::vec::Vec<ErrorFrame>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetErrorFrameRequest {
    /// Required. The name of the frame to retrieve.
    /// Format:
    /// projects/{project}/locations/{location}/sources/{source}/errorFrames/{error_frame}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional view mode to control the level of details for the
    /// frame. The default is a basic frame view.
    #[prost(enumeration = "ErrorFrameView", tag = "2")]
    pub view: i32,
}
/// A request for a list of sources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSourcesRequest {
    /// Required. Parent value for `ListSourcesRequest`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server may return fewer items than requested.
    /// If unspecified, the server will pick an appropriate default value.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results that the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to sort by. See <https://google.aip.dev/132#ordering> for more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for listing sources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSourcesResponse {
    /// The list of sources.
    #[prost(message, repeated, tag = "1")]
    pub sources: ::prost::alloc::vec::Vec<Source>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A request to get a source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSourceRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to create a source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSourceRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User specified ID for the source. It will become the last
    /// component of the source name. The ID must be unique within the project,
    /// must conform with RFC-1034, is restricted to lower-cased letters, and has a
    /// maximum length of 63 characters. The ID must match the regular expression:
    /// `\[a-z]([a-z0-9-]{0,61}[a-z0-9\])?`.
    #[prost(string, tag = "2")]
    pub source_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub source: ::core::option::Option<Source>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to update a source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSourceRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `Source` resource by the update.
    /// The values specified in the `update_mask` field are relative to the
    /// resource, not the full request.
    /// A field will be overwritten if it is in the mask.
    /// A single * value in the mask lets you to overwrite all fields.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated
    #[prost(message, optional, tag = "2")]
    pub source: ::core::option::Option<Source>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to delete a source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSourceRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for listing preference sets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPreferenceSetsRequest {
    /// Required. Parent value for `ListPreferenceSetsRequest`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, at most 500 preference sets will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Field to sort by. See <https://google.aip.dev/132#ordering> for more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for listing preference sets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPreferenceSetsResponse {
    /// The list of PreferenceSets
    #[prost(message, repeated, tag = "1")]
    pub preference_sets: ::prost::alloc::vec::Vec<PreferenceSet>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A request to get a preference set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPreferenceSetRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to create a preference set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePreferenceSetRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User specified ID for the preference set. It will become the last
    /// component of the preference set name. The ID must be unique within the
    /// project, must conform with RFC-1034, is restricted to lower-cased letters,
    /// and has a maximum length of 63 characters. The ID must match the regular
    /// expression
    /// `\[a-z]([a-z0-9-]{0,61}[a-z0-9\])?`.
    #[prost(string, tag = "2")]
    pub preference_set_id: ::prost::alloc::string::String,
    /// Required. The preference set resource being created.
    #[prost(message, optional, tag = "3")]
    pub preference_set: ::core::option::Option<PreferenceSet>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to update a preference set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePreferenceSetRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `PreferenceSet` resource by the update.
    /// The values specified in the `update_mask` field are relative to the
    /// resource, not the full request.
    /// A field will be overwritten if it is in the mask.
    /// A single * value in the mask lets you to overwrite all fields.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The preference set resource being updated.
    #[prost(message, optional, tag = "2")]
    pub preference_set: ::core::option::Option<PreferenceSet>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to delete a preference set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePreferenceSetRequest {
    /// Required. Name of the group resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to get the settings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingsRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to update the settings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingsRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `Settings` resource by the update.
    /// The values specified in the `update_mask` field are relative to the
    /// resource, not the full request.
    /// A field will be overwritten if it is in the mask.
    /// A single * value in the mask lets you to overwrite all fields.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The project settings resource being updated.
    #[prost(message, optional, tag = "2")]
    pub settings: ::core::option::Option<Settings>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to create a `ReportConfig` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportConfigRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User specified ID for the report config. It will become the last
    /// component of the report config name. The ID must be unique within the
    /// project, must conform with RFC-1034, is restricted to lower-cased letters,
    /// and has a maximum length of 63 characters. The ID must match the regular
    /// expression: \[a-z]([a-z0-9-]{0,61}[a-z0-9\])?.
    #[prost(string, tag = "2")]
    pub report_config_id: ::prost::alloc::string::String,
    /// Required. The report config set resource being created.
    #[prost(message, optional, tag = "3")]
    pub report_config: ::core::option::Option<ReportConfig>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to delete a ReportConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReportConfigRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to `true`, any child `Reports` of this entity will also be
    /// deleted. If set to `false`, the request only works if the resource has no
    /// children.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// A request to get a Report.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReportRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Determines what information to retrieve for the Report.
    #[prost(enumeration = "ReportView", tag = "6")]
    pub view: i32,
}
/// A request for a list of Reports.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportsRequest {
    /// Required. Parent value for `ListReportsRequest`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server may return fewer items than requested.
    /// If unspecified, the server will pick an appropriate default value.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results that the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to sort by. See <https://google.aip.dev/132#ordering> for more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Determines what information to retrieve for each Report.
    #[prost(enumeration = "ReportView", tag = "6")]
    pub view: i32,
}
/// Response message for listing Reports.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportsResponse {
    /// The list of Reports.
    #[prost(message, repeated, tag = "1")]
    pub reports: ::prost::alloc::vec::Vec<Report>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A request to delete a Report.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReportRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to get a `ReportConfig` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReportConfigRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to get a list of `ReportConfig` resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportConfigsRequest {
    /// Required. Parent value for `ListReportConfigsRequest`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to sort by. See <https://google.aip.dev/132#ordering> for more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for listing report configs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportConfigsResponse {
    /// A list of report configs.
    #[prost(message, repeated, tag = "1")]
    pub report_configs: ::prost::alloc::vec::Vec<ReportConfig>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for creating a Report.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User specified id for the report. It will become the last
    /// component of the report name. The id must be unique within the project,
    /// must conform with RFC-1034, is restricted to lower-cased letters, and has a
    /// maximum length of 63 characters. The id must match the regular expression:
    /// \[a-z]([a-z0-9-]{0,61}[a-z0-9\])?.
    #[prost(string, tag = "2")]
    pub report_id: ::prost::alloc::string::String,
    /// Required. The report resource being created.
    #[prost(message, optional, tag = "3")]
    pub report: ::core::option::Option<Report>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Collection of frame data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Frames {
    /// A repeated field of asset data.
    #[prost(message, repeated, tag = "1")]
    pub frames_data: ::prost::alloc::vec::Vec<AssetFrame>,
}
/// Contains data reported from an inventory source on an asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetFrame {
    /// The time the data was reported.
    #[prost(message, optional, tag = "10")]
    pub report_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "11")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Generic asset attributes.
    #[prost(btree_map = "string, string", tag = "12")]
    pub attributes: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Asset performance data samples.
    #[prost(message, repeated, tag = "13")]
    pub performance_samples: ::prost::alloc::vec::Vec<PerformanceSample>,
    /// Optional. Trace token is optionally provided to assist with debugging and
    /// traceability.
    #[prost(string, tag = "14")]
    pub trace_token: ::prost::alloc::string::String,
    /// The internal data of the frame is a oneof field depending on the type
    /// of asset information in the frame.
    #[prost(oneof = "asset_frame::FrameData", tags = "1")]
    pub frame_data: ::core::option::Option<asset_frame::FrameData>,
}
/// Nested message and enum types in `AssetFrame`.
pub mod asset_frame {
    /// The internal data of the frame is a oneof field depending on the type
    /// of asset information in the frame.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FrameData {
        /// Asset information specific for virtual machines.
        #[prost(message, tag = "1")]
        MachineDetails(super::MachineDetails),
    }
}
/// Details of a machine.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MachineDetails {
    /// Machine unique identifier.
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    /// Machine name.
    #[prost(string, tag = "2")]
    pub machine_name: ::prost::alloc::string::String,
    /// Machine creation time.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Number of CPU cores in the machine. Must be non-negative.
    #[prost(int32, tag = "4")]
    pub core_count: i32,
    /// The amount of memory in the machine. Must be non-negative.
    #[prost(int32, tag = "5")]
    pub memory_mb: i32,
    /// Power state of the machine.
    #[prost(enumeration = "machine_details::PowerState", tag = "6")]
    pub power_state: i32,
    /// Architecture details (vendor, CPU architecture).
    #[prost(message, optional, tag = "7")]
    pub architecture: ::core::option::Option<MachineArchitectureDetails>,
    /// Guest OS information.
    #[prost(message, optional, tag = "8")]
    pub guest_os: ::core::option::Option<GuestOsDetails>,
    /// Network details.
    #[prost(message, optional, tag = "9")]
    pub network: ::core::option::Option<MachineNetworkDetails>,
    /// Disk details.
    #[prost(message, optional, tag = "10")]
    pub disks: ::core::option::Option<MachineDiskDetails>,
    /// Platform specific information.
    #[prost(message, optional, tag = "11")]
    pub platform: ::core::option::Option<PlatformDetails>,
}
/// Nested message and enum types in `MachineDetails`.
pub mod machine_details {
    /// Machine power state.
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
    pub enum PowerState {
        /// Power state is unknown.
        Unspecified = 0,
        /// The machine is preparing to enter the ACTIVE state. An instance may enter
        /// the PENDING state when it launches for the first time, or when it is
        /// started after being in the SUSPENDED state.
        Pending = 1,
        /// The machine is active.
        Active = 2,
        /// The machine is being turned off.
        Suspending = 3,
        /// The machine is off.
        Suspended = 4,
        /// The machine is being deleted from the hosting platform.
        Deleting = 5,
        /// The machine is deleted from the hosting platform.
        Deleted = 6,
    }
    impl PowerState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PowerState::Unspecified => "POWER_STATE_UNSPECIFIED",
                PowerState::Pending => "PENDING",
                PowerState::Active => "ACTIVE",
                PowerState::Suspending => "SUSPENDING",
                PowerState::Suspended => "SUSPENDED",
                PowerState::Deleting => "DELETING",
                PowerState::Deleted => "DELETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "POWER_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "ACTIVE" => Some(Self::Active),
                "SUSPENDING" => Some(Self::Suspending),
                "SUSPENDED" => Some(Self::Suspended),
                "DELETING" => Some(Self::Deleting),
                "DELETED" => Some(Self::Deleted),
                _ => None,
            }
        }
    }
}
/// Details of the machine architecture.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MachineArchitectureDetails {
    /// CPU architecture, e.g., "x64-based PC", "x86_64", "i686" etc.
    #[prost(string, tag = "1")]
    pub cpu_architecture: ::prost::alloc::string::String,
    /// CPU name, e.g., "Intel Xeon E5-2690", "AMD EPYC 7571" etc.
    #[prost(string, tag = "2")]
    pub cpu_name: ::prost::alloc::string::String,
    /// Hardware vendor.
    #[prost(string, tag = "3")]
    pub vendor: ::prost::alloc::string::String,
    /// Number of CPU threads allocated to the machine.
    #[prost(int32, tag = "4")]
    pub cpu_thread_count: i32,
    /// Number of processor sockets allocated to the machine.
    #[prost(int32, tag = "5")]
    pub cpu_socket_count: i32,
    /// BIOS Details.
    #[prost(message, optional, tag = "6")]
    pub bios: ::core::option::Option<BiosDetails>,
    /// Firmware type.
    #[prost(enumeration = "machine_architecture_details::FirmwareType", tag = "7")]
    pub firmware_type: i32,
    /// CPU hyper-threading support.
    #[prost(enumeration = "machine_architecture_details::CpuHyperThreading", tag = "8")]
    pub hyperthreading: i32,
}
/// Nested message and enum types in `MachineArchitectureDetails`.
pub mod machine_architecture_details {
    /// Firmware type.
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
    pub enum FirmwareType {
        /// Unspecified or unknown.
        Unspecified = 0,
        /// BIOS firmware.
        Bios = 1,
        /// EFI firmware.
        Efi = 2,
    }
    impl FirmwareType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FirmwareType::Unspecified => "FIRMWARE_TYPE_UNSPECIFIED",
                FirmwareType::Bios => "BIOS",
                FirmwareType::Efi => "EFI",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FIRMWARE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BIOS" => Some(Self::Bios),
                "EFI" => Some(Self::Efi),
                _ => None,
            }
        }
    }
    /// CPU hyper-threading support.
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
    pub enum CpuHyperThreading {
        /// Unspecified or unknown.
        Unspecified = 0,
        /// Hyper-threading is disabled.
        Disabled = 1,
        /// Hyper-threading is enabled.
        Enabled = 2,
    }
    impl CpuHyperThreading {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CpuHyperThreading::Unspecified => "CPU_HYPER_THREADING_UNSPECIFIED",
                CpuHyperThreading::Disabled => "DISABLED",
                CpuHyperThreading::Enabled => "ENABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CPU_HYPER_THREADING_UNSPECIFIED" => Some(Self::Unspecified),
                "DISABLED" => Some(Self::Disabled),
                "ENABLED" => Some(Self::Enabled),
                _ => None,
            }
        }
    }
}
/// Details about the BIOS.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiosDetails {
    /// BIOS name.
    #[prost(string, tag = "1")]
    pub bios_name: ::prost::alloc::string::String,
    /// BIOS ID.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// BIOS manufacturer.
    #[prost(string, tag = "3")]
    pub manufacturer: ::prost::alloc::string::String,
    /// BIOS version.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// BIOS release date.
    #[prost(message, optional, tag = "5")]
    pub release_date: ::core::option::Option<super::super::super::r#type::Date>,
    /// SMBIOS UUID.
    #[prost(string, tag = "6")]
    pub smbios_uuid: ::prost::alloc::string::String,
}
/// Details of network adapters and settings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MachineNetworkDetails {
    /// The primary IP address of the machine.
    #[prost(string, tag = "1")]
    pub primary_ip_address: ::prost::alloc::string::String,
    /// The public IP address of the machine.
    #[prost(string, tag = "2")]
    pub public_ip_address: ::prost::alloc::string::String,
    /// MAC address of the machine.
    /// This property is used to uniqly identify the machine.
    #[prost(string, tag = "3")]
    pub primary_mac_address: ::prost::alloc::string::String,
    /// List of network adapters.
    #[prost(message, optional, tag = "4")]
    pub adapters: ::core::option::Option<NetworkAdapterList>,
}
/// List of network adapters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkAdapterList {
    /// Network adapter entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<NetworkAdapterDetails>,
}
/// Details of network adapter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkAdapterDetails {
    /// Network adapter type (e.g. VMXNET3).
    #[prost(string, tag = "1")]
    pub adapter_type: ::prost::alloc::string::String,
    /// MAC address.
    #[prost(string, tag = "2")]
    pub mac_address: ::prost::alloc::string::String,
    /// NetworkAddressList
    #[prost(message, optional, tag = "3")]
    pub addresses: ::core::option::Option<NetworkAddressList>,
}
/// List of allocated/assigned network addresses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkAddressList {
    /// Network address entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<NetworkAddress>,
}
/// Details of network address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkAddress {
    /// Assigned or configured IP Address.
    #[prost(string, tag = "1")]
    pub ip_address: ::prost::alloc::string::String,
    /// Subnet mask.
    #[prost(string, tag = "2")]
    pub subnet_mask: ::prost::alloc::string::String,
    /// Broadcast address.
    #[prost(string, tag = "3")]
    pub bcast: ::prost::alloc::string::String,
    /// Fully qualified domain name.
    #[prost(string, tag = "4")]
    pub fqdn: ::prost::alloc::string::String,
    /// Whether DHCP is used to assign addresses.
    #[prost(enumeration = "network_address::AddressAssignment", tag = "5")]
    pub assignment: i32,
}
/// Nested message and enum types in `NetworkAddress`.
pub mod network_address {
    /// Network address assignment.
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
    pub enum AddressAssignment {
        /// Unknown (default value).
        Unspecified = 0,
        /// Staticly assigned IP.
        Static = 1,
        /// Dynamically assigned IP (DHCP).
        Dhcp = 2,
    }
    impl AddressAssignment {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AddressAssignment::Unspecified => "ADDRESS_ASSIGNMENT_UNSPECIFIED",
                AddressAssignment::Static => "ADDRESS_ASSIGNMENT_STATIC",
                AddressAssignment::Dhcp => "ADDRESS_ASSIGNMENT_DHCP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ADDRESS_ASSIGNMENT_UNSPECIFIED" => Some(Self::Unspecified),
                "ADDRESS_ASSIGNMENT_STATIC" => Some(Self::Static),
                "ADDRESS_ASSIGNMENT_DHCP" => Some(Self::Dhcp),
                _ => None,
            }
        }
    }
}
/// Details of machine disks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MachineDiskDetails {
    /// Disk total Capacity.
    #[prost(int64, tag = "1")]
    pub total_capacity_bytes: i64,
    /// Total disk free space.
    #[prost(int64, tag = "2")]
    pub total_free_bytes: i64,
    /// List of disks.
    #[prost(message, optional, tag = "3")]
    pub disks: ::core::option::Option<DiskEntryList>,
}
/// VM disks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskEntryList {
    /// Disk entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<DiskEntry>,
}
/// Single disk entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskEntry {
    /// Disk capacity.
    #[prost(int64, tag = "1")]
    pub capacity_bytes: i64,
    /// Disk free space.
    #[prost(int64, tag = "2")]
    pub free_bytes: i64,
    /// Disk label.
    #[prost(string, tag = "3")]
    pub disk_label: ::prost::alloc::string::String,
    /// Disk label type (e.g. BIOS/GPT)
    #[prost(string, tag = "4")]
    pub disk_label_type: ::prost::alloc::string::String,
    /// Disks interface type.
    #[prost(enumeration = "disk_entry::InterfaceType", tag = "5")]
    pub interface_type: i32,
    /// Partition layout.
    #[prost(message, optional, tag = "6")]
    pub partitions: ::core::option::Option<DiskPartitionList>,
    /// Disk hardware address (e.g. 0:1 for SCSI).
    #[prost(string, tag = "7")]
    pub hw_address: ::prost::alloc::string::String,
    /// Additional details for specific platforms.
    #[prost(oneof = "disk_entry::PlatformSpecific", tags = "20")]
    pub platform_specific: ::core::option::Option<disk_entry::PlatformSpecific>,
}
/// Nested message and enum types in `DiskEntry`.
pub mod disk_entry {
    /// Disks interface type.
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
        /// Interface type unknown or unspecified.
        Unspecified = 0,
        /// IDE interface type.
        Ide = 1,
        /// SATA interface type.
        Sata = 2,
        /// SAS interface type.
        Sas = 3,
        /// SCSI interface type.
        Scsi = 4,
        /// NVME interface type.
        Nvme = 5,
        /// FC interface type.
        Fc = 6,
        /// iSCSI interface type.
        Iscsi = 7,
    }
    impl InterfaceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InterfaceType::Unspecified => "INTERFACE_TYPE_UNSPECIFIED",
                InterfaceType::Ide => "IDE",
                InterfaceType::Sata => "SATA",
                InterfaceType::Sas => "SAS",
                InterfaceType::Scsi => "SCSI",
                InterfaceType::Nvme => "NVME",
                InterfaceType::Fc => "FC",
                InterfaceType::Iscsi => "ISCSI",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INTERFACE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "IDE" => Some(Self::Ide),
                "SATA" => Some(Self::Sata),
                "SAS" => Some(Self::Sas),
                "SCSI" => Some(Self::Scsi),
                "NVME" => Some(Self::Nvme),
                "FC" => Some(Self::Fc),
                "ISCSI" => Some(Self::Iscsi),
                _ => None,
            }
        }
    }
    /// Additional details for specific platforms.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PlatformSpecific {
        /// VMware disk details.
        #[prost(message, tag = "20")]
        Vmware(super::VmwareDiskConfig),
    }
}
/// Disk partition list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskPartitionList {
    /// Partition entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<DiskPartition>,
}
/// Disk Partition details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskPartition {
    /// Partition type.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Partition file system.
    #[prost(string, tag = "2")]
    pub file_system: ::prost::alloc::string::String,
    /// Mount pount (Linux/Windows) or drive letter (Windows).
    #[prost(string, tag = "3")]
    pub mount_point: ::prost::alloc::string::String,
    /// Partition capacity.
    #[prost(int64, tag = "4")]
    pub capacity_bytes: i64,
    /// Partition free space.
    #[prost(int64, tag = "5")]
    pub free_bytes: i64,
    /// Partition UUID.
    #[prost(string, tag = "6")]
    pub uuid: ::prost::alloc::string::String,
    /// Sub-partitions.
    #[prost(message, optional, tag = "7")]
    pub sub_partitions: ::core::option::Option<DiskPartitionList>,
}
/// VMware disk config details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmwareDiskConfig {
    /// VMDK backing type.
    #[prost(enumeration = "vmware_disk_config::BackingType", tag = "1")]
    pub backing_type: i32,
    /// Is VMDK shared with other VMs.
    #[prost(bool, tag = "2")]
    pub shared: bool,
    /// VMDK disk mode.
    #[prost(enumeration = "vmware_disk_config::VmdkMode", tag = "3")]
    pub vmdk_mode: i32,
    /// RDM compatibility mode.
    #[prost(enumeration = "vmware_disk_config::RdmCompatibility", tag = "4")]
    pub rdm_compatibility: i32,
}
/// Nested message and enum types in `VmwareDiskConfig`.
pub mod vmware_disk_config {
    /// VMDK backing type possible values.
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
    pub enum BackingType {
        /// Default value.
        Unspecified = 0,
        /// Flat v1.
        FlatV1 = 1,
        /// Flat v2.
        FlatV2 = 2,
        /// Persistent memory, also known as Non-Volatile Memory (NVM).
        Pmem = 3,
        /// Raw Disk Memory v1.
        RdmV1 = 4,
        /// Raw Disk Memory v2.
        RdmV2 = 5,
        /// SEsparse is a snapshot format introduced in vSphere 5.5 for large disks.
        Sesparse = 6,
        /// SEsparse v1.
        SesparseV1 = 7,
        /// SEsparse v1.
        SesparseV2 = 8,
    }
    impl BackingType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BackingType::Unspecified => "BACKING_TYPE_UNSPECIFIED",
                BackingType::FlatV1 => "BACKING_TYPE_FLAT_V1",
                BackingType::FlatV2 => "BACKING_TYPE_FLAT_V2",
                BackingType::Pmem => "BACKING_TYPE_PMEM",
                BackingType::RdmV1 => "BACKING_TYPE_RDM_V1",
                BackingType::RdmV2 => "BACKING_TYPE_RDM_V2",
                BackingType::Sesparse => "BACKING_TYPE_SESPARSE",
                BackingType::SesparseV1 => "BACKING_TYPE_SESPARSE_V1",
                BackingType::SesparseV2 => "BACKING_TYPE_SESPARSE_V2",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BACKING_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BACKING_TYPE_FLAT_V1" => Some(Self::FlatV1),
                "BACKING_TYPE_FLAT_V2" => Some(Self::FlatV2),
                "BACKING_TYPE_PMEM" => Some(Self::Pmem),
                "BACKING_TYPE_RDM_V1" => Some(Self::RdmV1),
                "BACKING_TYPE_RDM_V2" => Some(Self::RdmV2),
                "BACKING_TYPE_SESPARSE" => Some(Self::Sesparse),
                "BACKING_TYPE_SESPARSE_V1" => Some(Self::SesparseV1),
                "BACKING_TYPE_SESPARSE_V2" => Some(Self::SesparseV2),
                _ => None,
            }
        }
    }
    /// VMDK disk mode.
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
    pub enum VmdkMode {
        /// VMDK disk mode unspecified or unknown.
        Unspecified = 0,
        /// Dependent disk mode.
        Dependent = 1,
        /// Independent - Persistent disk mode.
        IndependentPersistent = 2,
        /// Independent - Nonpersistent disk mode.
        IndependentNonpersistent = 3,
    }
    impl VmdkMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VmdkMode::Unspecified => "VMDK_MODE_UNSPECIFIED",
                VmdkMode::Dependent => "DEPENDENT",
                VmdkMode::IndependentPersistent => "INDEPENDENT_PERSISTENT",
                VmdkMode::IndependentNonpersistent => "INDEPENDENT_NONPERSISTENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VMDK_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "DEPENDENT" => Some(Self::Dependent),
                "INDEPENDENT_PERSISTENT" => Some(Self::IndependentPersistent),
                "INDEPENDENT_NONPERSISTENT" => Some(Self::IndependentNonpersistent),
                _ => None,
            }
        }
    }
    /// RDM compatibility mode.
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
    pub enum RdmCompatibility {
        /// Compatibility mode unspecified or unknown.
        Unspecified = 0,
        /// Physical compatibility mode.
        PhysicalCompatibility = 1,
        /// Virtual compatibility mode.
        VirtualCompatibility = 2,
    }
    impl RdmCompatibility {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RdmCompatibility::Unspecified => "RDM_COMPATIBILITY_UNSPECIFIED",
                RdmCompatibility::PhysicalCompatibility => "PHYSICAL_COMPATIBILITY",
                RdmCompatibility::VirtualCompatibility => "VIRTUAL_COMPATIBILITY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RDM_COMPATIBILITY_UNSPECIFIED" => Some(Self::Unspecified),
                "PHYSICAL_COMPATIBILITY" => Some(Self::PhysicalCompatibility),
                "VIRTUAL_COMPATIBILITY" => Some(Self::VirtualCompatibility),
                _ => None,
            }
        }
    }
}
/// Information from Guest-level collections.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestOsDetails {
    /// The name of the operating system.
    #[prost(string, tag = "1")]
    pub os_name: ::prost::alloc::string::String,
    /// What family the OS belong to, if known.
    #[prost(enumeration = "OperatingSystemFamily", tag = "2")]
    pub family: i32,
    /// The version of the operating system.
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// OS and app configuration.
    #[prost(message, optional, tag = "4")]
    pub config: ::core::option::Option<GuestConfigDetails>,
    /// Runtime information.
    #[prost(message, optional, tag = "5")]
    pub runtime: ::core::option::Option<GuestRuntimeDetails>,
}
/// Guest OS config information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestConfigDetails {
    /// OS issue (typically /etc/issue in Linux).
    #[prost(string, tag = "1")]
    pub issue: ::prost::alloc::string::String,
    /// Mount list (Linux fstab).
    #[prost(message, optional, tag = "2")]
    pub fstab: ::core::option::Option<FstabEntryList>,
    /// Hosts file (/etc/hosts).
    #[prost(message, optional, tag = "3")]
    pub hosts: ::core::option::Option<HostsEntryList>,
    /// NFS exports.
    #[prost(message, optional, tag = "4")]
    pub nfs_exports: ::core::option::Option<NfsExportList>,
    /// Security-Enhanced Linux (SELinux) mode.
    #[prost(enumeration = "guest_config_details::SeLinuxMode", tag = "5")]
    pub selinux_mode: i32,
}
/// Nested message and enum types in `GuestConfigDetails`.
pub mod guest_config_details {
    /// Security-Enhanced Linux (SELinux) mode.
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
    pub enum SeLinuxMode {
        /// SELinux mode unknown or unspecified.
        Unspecified = 0,
        /// SELinux is disabled.
        Disabled = 1,
        /// SELinux permissive mode.
        Permissive = 2,
        /// SELinux enforcing mode.
        Enforcing = 3,
    }
    impl SeLinuxMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SeLinuxMode::Unspecified => "SE_LINUX_MODE_UNSPECIFIED",
                SeLinuxMode::Disabled => "SE_LINUX_MODE_DISABLED",
                SeLinuxMode::Permissive => "SE_LINUX_MODE_PERMISSIVE",
                SeLinuxMode::Enforcing => "SE_LINUX_MODE_ENFORCING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SE_LINUX_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "SE_LINUX_MODE_DISABLED" => Some(Self::Disabled),
                "SE_LINUX_MODE_PERMISSIVE" => Some(Self::Permissive),
                "SE_LINUX_MODE_ENFORCING" => Some(Self::Enforcing),
                _ => None,
            }
        }
    }
}
/// Fstab content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FstabEntryList {
    /// Fstab entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<FstabEntry>,
}
/// Single fstab entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FstabEntry {
    /// The block special device or remote filesystem to be mounted.
    #[prost(string, tag = "1")]
    pub spec: ::prost::alloc::string::String,
    /// The mount point for the filesystem.
    #[prost(string, tag = "2")]
    pub file: ::prost::alloc::string::String,
    /// The type of the filesystem.
    #[prost(string, tag = "3")]
    pub vfstype: ::prost::alloc::string::String,
    /// Mount options associated with the filesystem.
    #[prost(string, tag = "4")]
    pub mntops: ::prost::alloc::string::String,
    /// Used by dump to determine which filesystems need to be dumped.
    #[prost(int32, tag = "5")]
    pub freq: i32,
    /// Used by the fsck(8) program to determine the order in which filesystem
    /// checks are done at reboot time.
    #[prost(int32, tag = "6")]
    pub passno: i32,
}
/// Hosts content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostsEntryList {
    /// Hosts entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<HostsEntry>,
}
/// Single /etc/hosts entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostsEntry {
    /// IP (raw, IPv4/6 agnostic).
    #[prost(string, tag = "1")]
    pub ip: ::prost::alloc::string::String,
    /// List of host names / aliases.
    #[prost(string, repeated, tag = "2")]
    pub host_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// NFS exports.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfsExportList {
    /// NFS export entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<NfsExport>,
}
/// NFS export.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfsExport {
    /// The directory being exported.
    #[prost(string, tag = "1")]
    pub export_directory: ::prost::alloc::string::String,
    /// The hosts or networks to which the export is being shared.
    #[prost(string, repeated, tag = "2")]
    pub hosts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Guest OS runtime information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestRuntimeDetails {
    /// Running background services.
    #[prost(message, optional, tag = "1")]
    pub services: ::core::option::Option<RunningServiceList>,
    /// Running processes.
    #[prost(message, optional, tag = "2")]
    pub processes: ::core::option::Option<RunningProcessList>,
    /// Runtime network information (connections, ports).
    #[prost(message, optional, tag = "3")]
    pub network: ::core::option::Option<RuntimeNetworkInfo>,
    /// Last time the OS was booted.
    #[prost(message, optional, tag = "4")]
    pub last_boot_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Domain, e.g. c.stratozone-development.internal.
    #[prost(string, tag = "5")]
    pub domain: ::prost::alloc::string::String,
    /// Machine name.
    #[prost(string, tag = "6")]
    pub machine_name: ::prost::alloc::string::String,
    /// Installed applications information.
    #[prost(message, optional, tag = "7")]
    pub installed_apps: ::core::option::Option<GuestInstalledApplicationList>,
    /// Open files information.
    #[prost(message, optional, tag = "8")]
    pub open_file_list: ::core::option::Option<OpenFileList>,
}
/// List of running guest OS services.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunningServiceList {
    /// Running service entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<RunningService>,
}
/// Guest OS running service details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunningService {
    /// Service name.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// Service state (OS-agnostic).
    #[prost(enumeration = "running_service::State", tag = "2")]
    pub state: i32,
    /// Service start mode (OS-agnostic).
    #[prost(enumeration = "running_service::StartMode", tag = "3")]
    pub start_mode: i32,
    /// Service binary path.
    #[prost(string, tag = "4")]
    pub exe_path: ::prost::alloc::string::String,
    /// Service command line.
    #[prost(string, tag = "5")]
    pub cmdline: ::prost::alloc::string::String,
    /// Service pid.
    #[prost(int64, tag = "6")]
    pub pid: i64,
}
/// Nested message and enum types in `RunningService`.
pub mod running_service {
    /// Service state (OS-agnostic).
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
        /// Service state unspecified.
        Unspecified = 0,
        /// Service is active.
        Active = 1,
        /// Service is paused.
        Paused = 2,
        /// Service is stopped.
        Stopped = 3,
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
                State::Paused => "PAUSED",
                State::Stopped => "STOPPED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "PAUSED" => Some(Self::Paused),
                "STOPPED" => Some(Self::Stopped),
                _ => None,
            }
        }
    }
    /// Service start mode (OS-agnostic).
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
    pub enum StartMode {
        /// Start mode unspecified.
        Unspecified = 0,
        /// The service is a device driver started by the system loader.
        Boot = 1,
        /// The service is a device driver started by the IOInitSystem function.
        System = 2,
        /// The service is started by the operating system, at system start-up
        Auto = 3,
        /// The service is started only manually, by a user.
        Manual = 4,
        /// The service is disabled.
        Disabled = 5,
    }
    impl StartMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StartMode::Unspecified => "START_MODE_UNSPECIFIED",
                StartMode::Boot => "BOOT",
                StartMode::System => "SYSTEM",
                StartMode::Auto => "AUTO",
                StartMode::Manual => "MANUAL",
                StartMode::Disabled => "DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "START_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "BOOT" => Some(Self::Boot),
                "SYSTEM" => Some(Self::System),
                "AUTO" => Some(Self::Auto),
                "MANUAL" => Some(Self::Manual),
                "DISABLED" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
}
/// List of running guest OS processes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunningProcessList {
    /// Running process entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<RunningProcess>,
}
/// Guest OS running process details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunningProcess {
    /// Process ID.
    #[prost(int64, tag = "1")]
    pub pid: i64,
    /// Process binary path.
    #[prost(string, tag = "2")]
    pub exe_path: ::prost::alloc::string::String,
    /// Process full command line.
    #[prost(string, tag = "3")]
    pub cmdline: ::prost::alloc::string::String,
    /// User running the process.
    #[prost(string, tag = "4")]
    pub user: ::prost::alloc::string::String,
    /// Process extended attributes.
    #[prost(btree_map = "string, string", tag = "100")]
    pub attributes: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Runtime networking information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeNetworkInfo {
    /// Time of the last network scan.
    #[prost(message, optional, tag = "1")]
    pub scan_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Network connections.
    #[prost(message, optional, tag = "2")]
    pub connections: ::core::option::Option<NetworkConnectionList>,
}
/// Network connection list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConnectionList {
    /// Network connection entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<NetworkConnection>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConnection {
    /// Connection protocol (e.g. TCP/UDP).
    #[prost(string, tag = "1")]
    pub protocol: ::prost::alloc::string::String,
    /// Local IP address.
    #[prost(string, tag = "2")]
    pub local_ip_address: ::prost::alloc::string::String,
    /// Local port.
    #[prost(int32, tag = "3")]
    pub local_port: i32,
    /// Remote IP address.
    #[prost(string, tag = "4")]
    pub remote_ip_address: ::prost::alloc::string::String,
    /// Remote port.
    #[prost(int32, tag = "5")]
    pub remote_port: i32,
    /// Network connection state.
    #[prost(enumeration = "network_connection::State", tag = "6")]
    pub state: i32,
    /// Process ID.
    #[prost(int64, tag = "7")]
    pub pid: i64,
    /// Process or service name.
    #[prost(string, tag = "8")]
    pub process_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `NetworkConnection`.
pub mod network_connection {
    /// Network connection state.
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
        /// Connection state is unknown or unspecified.
        Unspecified = 0,
        /// The connection is being opened.
        Opening = 1,
        /// The connection is open.
        Open = 2,
        /// Listening for incoming connections.
        Listen = 3,
        /// The connection is being closed.
        Closing = 4,
        /// The connection is closed.
        Closed = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Opening => "OPENING",
                State::Open => "OPEN",
                State::Listen => "LISTEN",
                State::Closing => "CLOSING",
                State::Closed => "CLOSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "OPENING" => Some(Self::Opening),
                "OPEN" => Some(Self::Open),
                "LISTEN" => Some(Self::Listen),
                "CLOSING" => Some(Self::Closing),
                "CLOSED" => Some(Self::Closed),
                _ => None,
            }
        }
    }
}
/// Guest installed application list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestInstalledApplicationList {
    /// Application entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<GuestInstalledApplication>,
}
/// Guest installed application information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestInstalledApplication {
    /// Installed application name.
    #[prost(string, tag = "1")]
    pub application_name: ::prost::alloc::string::String,
    /// Installed application vendor.
    #[prost(string, tag = "2")]
    pub vendor: ::prost::alloc::string::String,
    /// The time when the application was installed.
    #[prost(message, optional, tag = "3")]
    pub install_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Source path.
    #[prost(string, tag = "4")]
    pub path: ::prost::alloc::string::String,
    /// Installed application version.
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
}
/// Open file list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenFileList {
    /// Open file details entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<OpenFileDetails>,
}
/// Open file Information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenFileDetails {
    /// Opened file command.
    #[prost(string, tag = "1")]
    pub command: ::prost::alloc::string::String,
    /// Opened file user.
    #[prost(string, tag = "2")]
    pub user: ::prost::alloc::string::String,
    /// Opened file file type.
    #[prost(string, tag = "3")]
    pub file_type: ::prost::alloc::string::String,
    /// Opened file file path.
    #[prost(string, tag = "4")]
    pub file_path: ::prost::alloc::string::String,
}
/// Information about the platform.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlatformDetails {
    #[prost(oneof = "platform_details::VendorDetails", tags = "1, 2, 3, 4, 5")]
    pub vendor_details: ::core::option::Option<platform_details::VendorDetails>,
}
/// Nested message and enum types in `PlatformDetails`.
pub mod platform_details {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VendorDetails {
        /// VMware specific details.
        #[prost(message, tag = "1")]
        VmwareDetails(super::VmwarePlatformDetails),
        /// AWS EC2 specific details.
        #[prost(message, tag = "2")]
        AwsEc2Details(super::AwsEc2PlatformDetails),
        /// Azure VM specific details.
        #[prost(message, tag = "3")]
        AzureVmDetails(super::AzureVmPlatformDetails),
        /// Generic platform details.
        #[prost(message, tag = "4")]
        GenericDetails(super::GenericPlatformDetails),
        /// Physical machines platform details.
        #[prost(message, tag = "5")]
        PhysicalDetails(super::PhysicalPlatformDetails),
    }
}
/// VMware specific details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmwarePlatformDetails {
    /// vCenter version.
    #[prost(string, tag = "1")]
    pub vcenter_version: ::prost::alloc::string::String,
    /// ESX version.
    #[prost(string, tag = "2")]
    pub esx_version: ::prost::alloc::string::String,
    /// VMware os enum -
    /// <https://vdc-repo.vmware.com/vmwb-repository/dcr-public/da47f910-60ac-438b-8b9b-6122f4d14524/16b7274a-bf8b-4b4c-a05e-746f2aa93c8c/doc/vim.vm.GuestOsDescriptor.GuestOsIdentifier.html.>
    #[prost(string, tag = "3")]
    pub osid: ::prost::alloc::string::String,
    /// Folder name in vCenter where asset resides.
    #[prost(string, tag = "4")]
    pub vcenter_folder: ::prost::alloc::string::String,
    /// vCenter URI used in collection.
    #[prost(string, tag = "5")]
    pub vcenter_uri: ::prost::alloc::string::String,
    /// vCenter VM ID.
    #[prost(string, tag = "6")]
    pub vcenter_vm_id: ::prost::alloc::string::String,
}
/// AWS EC2 specific details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsEc2PlatformDetails {
    /// AWS platform's machine type label.
    #[prost(string, tag = "1")]
    pub machine_type_label: ::prost::alloc::string::String,
    /// The location of the machine in the AWS format.
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
}
/// Azure VM specific details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureVmPlatformDetails {
    /// Azure platform's machine type label.
    #[prost(string, tag = "1")]
    pub machine_type_label: ::prost::alloc::string::String,
    /// The location of the machine in the Azure format.
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
    /// Azure platform's provisioning state.
    #[prost(string, tag = "3")]
    pub provisioning_state: ::prost::alloc::string::String,
}
/// Generic platform details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericPlatformDetails {
    /// Free text representation of the machine location.
    /// The format of this field should not be relied on. Different VMs in the same
    /// location may have different string values for this field.
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
}
/// Platform specific details for Physical Machines.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalPlatformDetails {
    /// Free text representation of the machine location.
    /// The format of this field should not be relied on. Different machines in the
    /// same location may have different string values for this field.
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
}
/// Memory usage sample.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemoryUsageSample {
    /// Percentage of system memory utilized. Must be in the interval [0, 100].
    #[prost(float, tag = "1")]
    pub utilized_percentage: f32,
}
/// CPU usage sample.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpuUsageSample {
    /// Percentage of total CPU capacity utilized. Must be in the interval [0,
    /// 100]. On most systems can be calculated using 100 - idle percentage.
    #[prost(float, tag = "1")]
    pub utilized_percentage: f32,
}
/// Network usage sample. Values are across all network interfaces.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkUsageSample {
    /// Average network ingress in B/s sampled over a short window.
    /// Must be non-negative.
    #[prost(float, tag = "1")]
    pub average_ingress_bps: f32,
    /// Average network egress in B/s sampled over a short window.
    /// Must be non-negative.
    #[prost(float, tag = "2")]
    pub average_egress_bps: f32,
}
/// Disk usage sample. Values are across all disks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskUsageSample {
    /// Average IOPS sampled over a short window. Must be non-negative.
    #[prost(float, tag = "1")]
    pub average_iops: f32,
}
/// Performance data sample.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerformanceSample {
    /// Time the sample was collected.
    #[prost(message, optional, tag = "1")]
    pub sample_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Memory usage sample.
    #[prost(message, optional, tag = "2")]
    pub memory: ::core::option::Option<MemoryUsageSample>,
    /// CPU usage sample.
    #[prost(message, optional, tag = "3")]
    pub cpu: ::core::option::Option<CpuUsageSample>,
    /// Network usage sample.
    #[prost(message, optional, tag = "4")]
    pub network: ::core::option::Option<NetworkUsageSample>,
    /// Disk usage sample.
    #[prost(message, optional, tag = "5")]
    pub disk: ::core::option::Option<DiskUsageSample>,
}
/// Performance data for an asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetPerformanceData {
    /// Daily resource usage aggregations.
    /// Contains all of the data available for an asset, up to the last 420 days.
    /// Aggregations are sorted from oldest to most recent.
    #[prost(message, repeated, tag = "1")]
    pub daily_resource_usage_aggregations: ::prost::alloc::vec::Vec<
        DailyResourceUsageAggregation,
    >,
}
/// Usage data aggregation for a single day.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyResourceUsageAggregation {
    /// Aggregation date. Day boundaries are at midnight UTC.
    #[prost(message, optional, tag = "1")]
    pub date: ::core::option::Option<super::super::super::r#type::Date>,
    /// CPU usage.
    #[prost(message, optional, tag = "2")]
    pub cpu: ::core::option::Option<daily_resource_usage_aggregation::Cpu>,
    /// Memory usage.
    #[prost(message, optional, tag = "3")]
    pub memory: ::core::option::Option<daily_resource_usage_aggregation::Memory>,
    /// Network usage.
    #[prost(message, optional, tag = "4")]
    pub network: ::core::option::Option<daily_resource_usage_aggregation::Network>,
    /// Disk usage.
    #[prost(message, optional, tag = "5")]
    pub disk: ::core::option::Option<daily_resource_usage_aggregation::Disk>,
}
/// Nested message and enum types in `DailyResourceUsageAggregation`.
pub mod daily_resource_usage_aggregation {
    /// Statistical aggregation of samples for a single resource usage.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Stats {
        /// Average usage value.
        #[prost(float, tag = "1")]
        pub average: f32,
        /// Median usage value.
        #[prost(float, tag = "2")]
        pub median: f32,
        /// 95th percentile usage value.
        #[prost(float, tag = "3")]
        pub nintey_fifth_percentile: f32,
        /// Peak usage value.
        #[prost(float, tag = "4")]
        pub peak: f32,
    }
    /// Statistical aggregation of CPU usage.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Cpu {
        /// CPU utilization percentage.
        #[prost(message, optional, tag = "1")]
        pub utilization_percentage: ::core::option::Option<Stats>,
    }
    /// Statistical aggregation of memory usage.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Memory {
        /// Memory utilization percentage.
        #[prost(message, optional, tag = "1")]
        pub utilization_percentage: ::core::option::Option<Stats>,
    }
    /// Statistical aggregation of network usage.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Network {
        /// Network ingress in B/s.
        #[prost(message, optional, tag = "1")]
        pub ingress_bps: ::core::option::Option<Stats>,
        /// Network egress in B/s.
        #[prost(message, optional, tag = "2")]
        pub egress_bps: ::core::option::Option<Stats>,
    }
    /// Statistical aggregation of disk usage.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Disk {
        /// Disk I/O operations per second.
        #[prost(message, optional, tag = "1")]
        pub iops: ::core::option::Option<Stats>,
    }
}
/// Message containing insights list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsightList {
    /// Output only. Insights of the list.
    #[prost(message, repeated, tag = "1")]
    pub insights: ::prost::alloc::vec::Vec<Insight>,
    /// Output only. Update timestamp.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// An insight about an asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Insight {
    #[prost(oneof = "insight::Insight", tags = "1")]
    pub insight: ::core::option::Option<insight::Insight>,
}
/// Nested message and enum types in `Insight`.
pub mod insight {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Insight {
        /// Output only. An insight about potential migrations for an asset.
        #[prost(message, tag = "1")]
        MigrationInsight(super::MigrationInsight),
    }
}
/// An insight about potential migrations for an asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationInsight {
    /// Output only. Description of how well the asset this insight is associated
    /// with fits the proposed migration.
    #[prost(message, optional, tag = "1")]
    pub fit: ::core::option::Option<FitDescriptor>,
    /// A target for the migration.
    #[prost(oneof = "migration_insight::MigrationTarget", tags = "10")]
    pub migration_target: ::core::option::Option<migration_insight::MigrationTarget>,
}
/// Nested message and enum types in `MigrationInsight`.
pub mod migration_insight {
    /// A target for the migration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MigrationTarget {
        /// Output only. A Google Compute Engine target.
        #[prost(message, tag = "10")]
        ComputeEngineTarget(super::ComputeEngineMigrationTarget),
    }
}
/// Compute engine migration target.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeEngineMigrationTarget {
    /// Description of the suggested shape for the migration target.
    #[prost(message, optional, tag = "1")]
    pub shape: ::core::option::Option<ComputeEngineShapeDescriptor>,
}
/// Describes the fit level of an asset for migration to a specific target.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FitDescriptor {
    /// Fit level.
    #[prost(enumeration = "fit_descriptor::FitLevel", tag = "1")]
    pub fit_level: i32,
}
/// Nested message and enum types in `FitDescriptor`.
pub mod fit_descriptor {
    /// Fit level.
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
    pub enum FitLevel {
        /// Not enough information.
        Unspecified = 0,
        /// Fit.
        Fit = 1,
        /// No Fit.
        NoFit = 2,
        /// Fit with effort.
        RequiresEffort = 3,
    }
    impl FitLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FitLevel::Unspecified => "FIT_LEVEL_UNSPECIFIED",
                FitLevel::Fit => "FIT",
                FitLevel::NoFit => "NO_FIT",
                FitLevel::RequiresEffort => "REQUIRES_EFFORT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FIT_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
                "FIT" => Some(Self::Fit),
                "NO_FIT" => Some(Self::NoFit),
                "REQUIRES_EFFORT" => Some(Self::RequiresEffort),
                _ => None,
            }
        }
    }
}
/// Compute Engine target shape descriptor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeEngineShapeDescriptor {
    /// Memory in mebibytes.
    #[prost(int32, tag = "1")]
    pub memory_mb: i32,
    /// Number of physical cores.
    #[prost(int32, tag = "2")]
    pub physical_core_count: i32,
    /// Number of logical cores.
    #[prost(int32, tag = "3")]
    pub logical_core_count: i32,
    /// Compute Engine machine series.
    #[prost(string, tag = "4")]
    pub series: ::prost::alloc::string::String,
    /// Compute Engine machine type.
    #[prost(string, tag = "5")]
    pub machine_type: ::prost::alloc::string::String,
}
/// Message describing an aggregation. The message includes the aggregation type,
/// parameters, and the field on which to perform the aggregation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aggregation {
    /// The name of the field on which to aggregate.
    #[prost(string, tag = "1")]
    pub field: ::prost::alloc::string::String,
    #[prost(oneof = "aggregation::AggregationFunction", tags = "2, 3, 4, 5")]
    pub aggregation_function: ::core::option::Option<aggregation::AggregationFunction>,
}
/// Nested message and enum types in `Aggregation`.
pub mod aggregation {
    /// Object count.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Count {}
    /// Sum of field values.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Sum {}
    /// Histogram of bucketed assets counts by field value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Histogram {
        /// Lower bounds of buckets.
        /// The response will contain `n+1` buckets for `n` bounds.
        /// The first bucket will count all assets for which the field value is
        /// smaller than the first bound.
        /// Subsequent buckets will count assets for which the field value is
        /// greater or equal to a lower bound and smaller than the next one.
        /// The last bucket will count assets for which the field value is greater or
        /// equal to the final lower bound.
        /// You can define up to 20 lower bounds.
        #[prost(double, repeated, tag = "1")]
        pub lower_bounds: ::prost::alloc::vec::Vec<f64>,
    }
    /// Frequency distribution of all field values.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Frequency {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AggregationFunction {
        /// Count the number of matching objects.
        #[prost(message, tag = "2")]
        Count(Count),
        /// Sum over a numeric field.
        #[prost(message, tag = "3")]
        Sum(Sum),
        /// Creates a bucketed histogram of field values.
        #[prost(message, tag = "4")]
        Histogram(Histogram),
        /// Creates a frequency distribution of all field values.
        #[prost(message, tag = "5")]
        Frequency(Frequency),
    }
}
/// Message describing a result of an aggregation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregationResult {
    #[prost(string, tag = "1")]
    pub field: ::prost::alloc::string::String,
    #[prost(oneof = "aggregation_result::Result", tags = "2, 3, 4, 5")]
    pub result: ::core::option::Option<aggregation_result::Result>,
}
/// Nested message and enum types in `AggregationResult`.
pub mod aggregation_result {
    /// The result of a count aggregation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Count {
        #[prost(int64, tag = "1")]
        pub value: i64,
    }
    /// The result of a sum aggregation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Sum {
        #[prost(double, tag = "1")]
        pub value: f64,
    }
    /// The result of a bucketed histogram aggregation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Histogram {
        /// Buckets in the histogram.
        /// There will be `n+1` buckets matching `n` lower bounds in the request.
        /// The first bucket will be from -infinity to the first bound.
        /// Subsequent buckets will be between one bound and the next.
        /// The final bucket will be from the final bound to infinity.
        #[prost(message, repeated, tag = "1")]
        pub buckets: ::prost::alloc::vec::Vec<histogram::Bucket>,
    }
    /// Nested message and enum types in `Histogram`.
    pub mod histogram {
        /// A histogram bucket with a lower and upper bound, and a count of items
        /// with a field value between those bounds.
        /// The lower bound is inclusive and the upper bound is exclusive.
        /// Lower bound may be -infinity and upper bound may be infinity.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Bucket {
            /// Lower bound - inclusive.
            #[prost(double, tag = "1")]
            pub lower_bound: f64,
            /// Upper bound - exclusive.
            #[prost(double, tag = "2")]
            pub upper_bound: f64,
            /// Count of items in the bucket.
            #[prost(int64, tag = "3")]
            pub count: i64,
        }
    }
    /// The result of a frequency distribution aggregation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Frequency {
        #[prost(btree_map = "string, int64", tag = "1")]
        pub values: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            i64,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "2")]
        Count(Count),
        #[prost(message, tag = "3")]
        Sum(Sum),
        #[prost(message, tag = "4")]
        Histogram(Histogram),
        #[prost(message, tag = "5")]
        Frequency(Frequency),
    }
}
/// A resource that aggregates the validation errors found in an import job file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileValidationReport {
    /// The name of the file.
    #[prost(string, tag = "1")]
    pub file_name: ::prost::alloc::string::String,
    /// Partial list of rows that encountered validation error.
    #[prost(message, repeated, tag = "2")]
    pub row_errors: ::prost::alloc::vec::Vec<ImportRowError>,
    /// Flag indicating that processing was aborted due to maximum number of
    /// errors.
    #[prost(bool, tag = "3")]
    pub partial_report: bool,
    /// List of file level errors.
    #[prost(message, repeated, tag = "4")]
    pub file_errors: ::prost::alloc::vec::Vec<ImportError>,
}
/// A resource that aggregates errors across import job files.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationReport {
    /// List of errors found in files.
    #[prost(message, repeated, tag = "1")]
    pub file_validations: ::prost::alloc::vec::Vec<FileValidationReport>,
    /// List of job level errors.
    #[prost(message, repeated, tag = "2")]
    pub job_errors: ::prost::alloc::vec::Vec<ImportError>,
}
/// A resource that reports result of the import job execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionReport {
    /// Total number of asset frames reported for the import job.
    #[prost(int32, tag = "1")]
    pub frames_reported: i32,
    /// Validation errors encountered during the execution of the import job.
    #[prost(message, optional, tag = "2")]
    pub execution_errors: ::core::option::Option<ValidationReport>,
    /// Total number of rows in the import job.
    #[prost(int32, tag = "3")]
    pub total_rows_count: i32,
}
/// A resource that reports the errors encountered while processing an
/// import job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportError {
    /// The error information.
    #[prost(string, tag = "1")]
    pub error_details: ::prost::alloc::string::String,
    /// The severity of the error.
    #[prost(enumeration = "import_error::Severity", tag = "2")]
    pub severity: i32,
}
/// Nested message and enum types in `ImportError`.
pub mod import_error {
    /// Enumerate possible error severity.
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
        Unspecified = 0,
        Error = 1,
        Warning = 2,
        Info = 3,
    }
    impl Severity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Severity::Unspecified => "SEVERITY_UNSPECIFIED",
                Severity::Error => "ERROR",
                Severity::Warning => "WARNING",
                Severity::Info => "INFO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
                "ERROR" => Some(Self::Error),
                "WARNING" => Some(Self::Warning),
                "INFO" => Some(Self::Info),
                _ => None,
            }
        }
    }
}
/// A resource that reports the import job errors at row level.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportRowError {
    /// The row number where the error was detected.
    #[deprecated]
    #[prost(int32, tag = "1")]
    pub row_number: i32,
    /// The name of the VM in the row.
    #[prost(string, tag = "2")]
    pub vm_name: ::prost::alloc::string::String,
    /// The VM UUID.
    #[prost(string, tag = "3")]
    pub vm_uuid: ::prost::alloc::string::String,
    /// The list of errors detected in the row.
    #[prost(message, repeated, tag = "4")]
    pub errors: ::prost::alloc::vec::Vec<ImportError>,
}
/// A resource that contains a URI to which a data file can be uploaded.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileInfo {
    /// Output only. Upload URI for the file.
    #[prost(string, tag = "1")]
    pub signed_uri: ::prost::alloc::string::String,
    /// Output only. The headers that were used to sign the URI.
    #[prost(btree_map = "string, string", tag = "2")]
    pub headers: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Expiration time of the upload URI.
    #[prost(message, optional, tag = "3")]
    pub uri_expiration_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Lists the asset IDs of all assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetList {
    /// Required. A list of asset IDs
    #[prost(string, repeated, tag = "1")]
    pub asset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A resource that contains a single violation of a reported `AssetFrame`
/// resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrameViolationEntry {
    /// The field of the original frame where the violation occurred.
    #[prost(string, tag = "1")]
    pub field: ::prost::alloc::string::String,
    /// A message describing the violation.
    #[prost(string, tag = "2")]
    pub violation: ::prost::alloc::string::String,
}
/// VirtualMachinePreferences enables you to create sets of assumptions, for
/// example, a geographical location and pricing track, for your migrated virtual
/// machines. The set of preferences influence recommendations for migrating
/// virtual machine assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualMachinePreferences {
    /// Region preferences for assets using this preference set.
    /// If you are unsure which value to set, the migration service API region is
    /// often a good value to start with.
    #[prost(message, optional, tag = "3")]
    pub region_preferences: ::core::option::Option<RegionPreferences>,
    /// Commitment plan to consider when calculating costs for virtual machine
    /// insights and recommendations.
    /// If you are unsure which value to set, a 3 year commitment plan is often a
    /// good value to start with.
    #[prost(enumeration = "CommitmentPlan", tag = "4")]
    pub commitment_plan: i32,
    /// Sizing optimization strategy specifies the preferred strategy used when
    /// extrapolating usage data to calculate insights and recommendations for a
    /// virtual machine.
    /// If you are unsure which value to set, a moderate sizing optimization
    /// strategy is often a good value to start with.
    #[prost(enumeration = "SizingOptimizationStrategy", tag = "5")]
    pub sizing_optimization_strategy: i32,
    /// Compute Engine preferences concern insights and recommendations for Compute
    /// Engine target.
    #[prost(message, optional, tag = "6")]
    pub compute_engine_preferences: ::core::option::Option<ComputeEnginePreferences>,
}
/// The user preferences relating to Compute Engine target platform.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeEnginePreferences {
    /// Preferences concerning the machine types to consider on Compute Engine.
    #[prost(message, optional, tag = "2")]
    pub machine_preferences: ::core::option::Option<MachinePreferences>,
    /// License type to consider when calculating costs for virtual machine
    /// insights and recommendations. If unspecified, costs are calculated
    /// based on the default licensing plan.
    #[prost(enumeration = "LicenseType", tag = "3")]
    pub license_type: i32,
}
/// The type of machines to consider when calculating virtual machine migration
/// insights and recommendations.
/// Not all machine types are available in all zones and regions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MachinePreferences {
    /// Compute Engine machine series to consider for insights and recommendations.
    /// If empty, no restriction is applied on the machine series.
    #[prost(message, repeated, tag = "1")]
    pub allowed_machine_series: ::prost::alloc::vec::Vec<MachineSeries>,
}
/// A Compute Engine machine series.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MachineSeries {
    /// Code to identify a Compute Engine machine series. Consult
    /// <https://cloud.google.com/compute/docs/machine-resource#machine_type_comparison>
    /// for more details on the available series.
    #[prost(string, tag = "1")]
    pub code: ::prost::alloc::string::String,
}
/// The user preferences relating to target regions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionPreferences {
    /// A list of preferred regions,
    /// ordered by the most preferred region first.
    /// Set only valid Google Cloud region names.
    /// See <https://cloud.google.com/compute/docs/regions-zones>
    /// for available regions.
    #[prost(string, repeated, tag = "1")]
    pub preferred_regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Describes the Migration Center settings related to the project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Settings {
    /// Output only. The name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The preference set used by default for a project.
    #[prost(string, tag = "2")]
    pub preference_set: ::prost::alloc::string::String,
}
/// Describes the Summary view of a Report, which contains aggregated values
/// for all the groups and preference sets included in this Report.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportSummary {
    /// Aggregate statistics for all the assets across all the groups.
    #[prost(message, optional, tag = "1")]
    pub all_assets_stats: ::core::option::Option<report_summary::AssetAggregateStats>,
    /// Findings for each Group included in this report.
    #[prost(message, repeated, tag = "2")]
    pub group_findings: ::prost::alloc::vec::Vec<report_summary::GroupFinding>,
}
/// Nested message and enum types in `ReportSummary`.
pub mod report_summary {
    /// Describes a collection of data points rendered as a Chart.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChartData {
        /// Each data point in the chart is represented as a name-value pair
        /// with the name being the x-axis label, and the value being the y-axis
        /// value.
        #[prost(message, repeated, tag = "1")]
        pub data_points: ::prost::alloc::vec::Vec<chart_data::DataPoint>,
    }
    /// Nested message and enum types in `ChartData`.
    pub mod chart_data {
        /// Describes a single data point in the Chart.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DataPoint {
            /// The X-axis label for this data point.
            #[prost(string, tag = "1")]
            pub label: ::prost::alloc::string::String,
            /// The Y-axis value for this data point.
            #[prost(double, tag = "2")]
            pub value: f64,
        }
    }
    /// Utilization Chart is a specific type of visualization which displays
    /// a metric classified into "Used" and "Free" buckets.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UtilizationChartData {
        /// Aggregate value which falls into the "Used" bucket.
        #[prost(int64, tag = "1")]
        pub used: i64,
        /// Aggregate value which falls into the "Free" bucket.
        #[prost(int64, tag = "2")]
        pub free: i64,
    }
    /// A Histogram Chart shows a distribution of values into buckets, showing
    /// a count of values which fall into a bucket.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HistogramChartData {
        /// Buckets in the histogram.
        /// There will be `n+1` buckets matching `n` lower bounds in the request.
        /// The first bucket will be from -infinity to the first bound.
        /// Subsequent buckets will be between one bound and the next.
        /// The final bucket will be from the final bound to infinity.
        #[prost(message, repeated, tag = "1")]
        pub buckets: ::prost::alloc::vec::Vec<histogram_chart_data::Bucket>,
    }
    /// Nested message and enum types in `HistogramChartData`.
    pub mod histogram_chart_data {
        /// A histogram bucket with a lower and upper bound, and a count of items
        /// with a field value between those bounds.
        /// The lower bound is inclusive and the upper bound is exclusive.
        /// Lower bound may be -infinity and upper bound may be infinity.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Bucket {
            /// Lower bound - inclusive.
            #[prost(int64, tag = "1")]
            pub lower_bound: i64,
            /// Upper bound - exclusive.
            #[prost(int64, tag = "2")]
            pub upper_bound: i64,
            /// Count of items in the bucket.
            #[prost(int64, tag = "3")]
            pub count: i64,
        }
    }
    /// Aggregate statistics for a collection of assets.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AssetAggregateStats {
        /// Sum of the memory in bytes of all the assets in this collection.
        #[prost(int64, tag = "1")]
        pub total_memory_bytes: i64,
        /// Sum of persistent storage in bytes of all the assets in this collection.
        #[prost(int64, tag = "2")]
        pub total_storage_bytes: i64,
        /// Sum of the CPU core count of all the assets in this collection.
        #[prost(int64, tag = "3")]
        pub total_cores: i64,
        /// Count of the number of unique assets in this collection.
        #[prost(int64, tag = "4")]
        pub total_assets: i64,
        /// Total memory split into Used/Free buckets.
        #[prost(message, optional, tag = "5")]
        pub memory_utilization_chart: ::core::option::Option<UtilizationChartData>,
        /// Total memory split into Used/Free buckets.
        #[prost(message, optional, tag = "6")]
        pub storage_utilization_chart: ::core::option::Option<UtilizationChartData>,
        /// Count of assets grouped by Operating System families.
        #[prost(message, optional, tag = "7")]
        pub operating_system: ::core::option::Option<ChartData>,
        /// Histogram showing a distribution of CPU core counts.
        #[prost(message, optional, tag = "8")]
        pub core_count_histogram: ::core::option::Option<HistogramChartData>,
        /// Histogram showing a distribution of memory sizes.
        #[prost(message, optional, tag = "9")]
        pub memory_bytes_histogram: ::core::option::Option<HistogramChartData>,
        /// Histogram showing a distribution of memory sizes.
        #[prost(message, optional, tag = "10")]
        pub storage_bytes_histogram: ::core::option::Option<HistogramChartData>,
    }
    /// Represents a data point tracking the count of assets allocated for a
    /// specific Machine Series.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MachineSeriesAllocation {
        /// The Machine Series (e.g. "E2", "N2")
        #[prost(message, optional, tag = "1")]
        pub machine_series: ::core::option::Option<super::MachineSeries>,
        /// Count of assets allocated to this machine series.
        #[prost(int64, tag = "2")]
        pub allocated_asset_count: i64,
    }
    /// A set of findings that applies to assets destined for Compute Engine.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ComputeEngineFinding {
        /// Set of regions in which the assets were allocated.
        #[prost(string, repeated, tag = "1")]
        pub allocated_regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Count of assets which were allocated.
        #[prost(int64, tag = "2")]
        pub allocated_asset_count: i64,
        /// Distribution of assets based on the Machine Series.
        #[prost(message, repeated, tag = "3")]
        pub machine_series_allocations: ::prost::alloc::vec::Vec<
            MachineSeriesAllocation,
        >,
        /// Set of disk types allocated to assets.
        #[prost(enumeration = "super::PersistentDiskType", repeated, tag = "4")]
        pub allocated_disk_types: ::prost::alloc::vec::Vec<i32>,
    }
    /// Summary Findings for a specific Group/PreferenceSet combination.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupPreferenceSetFinding {
        /// Display Name of the Preference Set
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// Description for the Preference Set.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// A set of preferences that applies to all machines in the context.
        #[prost(message, optional, tag = "3")]
        pub machine_preferences: ::core::option::Option<
            super::VirtualMachinePreferences,
        >,
        /// Total monthly cost for this preference set.
        #[prost(message, optional, tag = "4")]
        pub monthly_cost_total: ::core::option::Option<
            super::super::super::super::r#type::Money,
        >,
        /// Compute monthly cost for this preference set.
        #[prost(message, optional, tag = "5")]
        pub monthly_cost_compute: ::core::option::Option<
            super::super::super::super::r#type::Money,
        >,
        /// Licensing monthly cost for this preference set.
        #[prost(message, optional, tag = "6")]
        pub monthly_cost_os_license: ::core::option::Option<
            super::super::super::super::r#type::Money,
        >,
        /// Network Egress monthly cost for this preference set.
        #[prost(message, optional, tag = "7")]
        pub monthly_cost_network_egress: ::core::option::Option<
            super::super::super::super::r#type::Money,
        >,
        /// Storage monthly cost for this preference set.
        #[prost(message, optional, tag = "8")]
        pub monthly_cost_storage: ::core::option::Option<
            super::super::super::super::r#type::Money,
        >,
        /// Miscellaneous monthly cost for this preference set.
        #[prost(message, optional, tag = "9")]
        pub monthly_cost_other: ::core::option::Option<
            super::super::super::super::r#type::Money,
        >,
        /// A set of findings that applies to Compute Engine machines in the input.
        #[prost(message, optional, tag = "10")]
        pub compute_engine_finding: ::core::option::Option<ComputeEngineFinding>,
    }
    /// Summary Findings for a specific Group.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupFinding {
        /// Display Name for the Group.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// Description for the Group.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// Summary statistics for all the assets in this group.
        #[prost(message, optional, tag = "3")]
        pub asset_aggregate_stats: ::core::option::Option<AssetAggregateStats>,
        /// Count of the number of assets in this group which are also included
        /// in another group within the same report.
        #[prost(int64, tag = "4")]
        pub overlapping_asset_count: i64,
        /// Findings for each of the PreferenceSets for this group.
        #[prost(message, repeated, tag = "5")]
        pub preference_set_findings: ::prost::alloc::vec::Vec<GroupPreferenceSetFinding>,
    }
}
/// Specifies the types of asset views that provide complete or partial details
/// of an asset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetView {
    /// The asset view is not specified. The API displays the basic view by
    /// default.
    Unspecified = 0,
    /// The asset view includes only basic metadata of the asset.
    Basic = 1,
    /// The asset view includes all the metadata of an asset and performance data.
    Full = 2,
}
impl AssetView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AssetView::Unspecified => "ASSET_VIEW_UNSPECIFIED",
            AssetView::Basic => "ASSET_VIEW_BASIC",
            AssetView::Full => "ASSET_VIEW_FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASSET_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "ASSET_VIEW_BASIC" => Some(Self::Basic),
            "ASSET_VIEW_FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Known categories of operating systems.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperatingSystemFamily {
    OsFamilyUnknown = 0,
    /// Microsoft Windows Server and Desktop.
    OsFamilyWindows = 1,
    /// Various Linux flavors.
    OsFamilyLinux = 2,
    /// Non-Linux Unix flavors.
    OsFamilyUnix = 3,
}
impl OperatingSystemFamily {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperatingSystemFamily::OsFamilyUnknown => "OS_FAMILY_UNKNOWN",
            OperatingSystemFamily::OsFamilyWindows => "OS_FAMILY_WINDOWS",
            OperatingSystemFamily::OsFamilyLinux => "OS_FAMILY_LINUX",
            OperatingSystemFamily::OsFamilyUnix => "OS_FAMILY_UNIX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OS_FAMILY_UNKNOWN" => Some(Self::OsFamilyUnknown),
            "OS_FAMILY_WINDOWS" => Some(Self::OsFamilyWindows),
            "OS_FAMILY_LINUX" => Some(Self::OsFamilyLinux),
            "OS_FAMILY_UNIX" => Some(Self::OsFamilyUnix),
            _ => None,
        }
    }
}
/// Specifies the data formats supported by Migration Center.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ImportJobFormat {
    /// Default value.
    Unspecified = 0,
    /// RVTools format (XLSX).
    RvtoolsXlsx = 1,
    /// RVTools format (CSV).
    RvtoolsCsv = 2,
    /// CSV format exported from AWS using the
    /// [AWS collection
    /// script]\[<https://github.com/GoogleCloudPlatform/aws-to-stratozone-export\].>
    ExportedAwsCsv = 4,
    /// CSV format exported from Azure using the
    /// [Azure collection
    /// script]\[<https://github.com/GoogleCloudPlatform/azure-to-stratozone-export\].>
    ExportedAzureCsv = 5,
    /// CSV format created manually and following the StratoZone format. For more
    /// information, see [Manually create and upload data
    /// tables]\[<https://cloud.google.com/migrate/stratozone/docs/import-data-portal\].>
    StratozoneCsv = 6,
}
impl ImportJobFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ImportJobFormat::Unspecified => "IMPORT_JOB_FORMAT_UNSPECIFIED",
            ImportJobFormat::RvtoolsXlsx => "IMPORT_JOB_FORMAT_RVTOOLS_XLSX",
            ImportJobFormat::RvtoolsCsv => "IMPORT_JOB_FORMAT_RVTOOLS_CSV",
            ImportJobFormat::ExportedAwsCsv => "IMPORT_JOB_FORMAT_EXPORTED_AWS_CSV",
            ImportJobFormat::ExportedAzureCsv => "IMPORT_JOB_FORMAT_EXPORTED_AZURE_CSV",
            ImportJobFormat::StratozoneCsv => "IMPORT_JOB_FORMAT_STRATOZONE_CSV",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IMPORT_JOB_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
            "IMPORT_JOB_FORMAT_RVTOOLS_XLSX" => Some(Self::RvtoolsXlsx),
            "IMPORT_JOB_FORMAT_RVTOOLS_CSV" => Some(Self::RvtoolsCsv),
            "IMPORT_JOB_FORMAT_EXPORTED_AWS_CSV" => Some(Self::ExportedAwsCsv),
            "IMPORT_JOB_FORMAT_EXPORTED_AZURE_CSV" => Some(Self::ExportedAzureCsv),
            "IMPORT_JOB_FORMAT_STRATOZONE_CSV" => Some(Self::StratozoneCsv),
            _ => None,
        }
    }
}
/// Specifies the types of import job views that provide complete or partial
/// details of an import job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ImportJobView {
    /// The import job view is not specified. The API displays the basic view by
    /// default.
    Unspecified = 0,
    /// The import job view includes basic metadata of an import job.
    /// This view does not include payload information.
    Basic = 1,
    /// The import job view includes all metadata of an import job.
    Full = 2,
}
impl ImportJobView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ImportJobView::Unspecified => "IMPORT_JOB_VIEW_UNSPECIFIED",
            ImportJobView::Basic => "IMPORT_JOB_VIEW_BASIC",
            ImportJobView::Full => "IMPORT_JOB_VIEW_FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IMPORT_JOB_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "IMPORT_JOB_VIEW_BASIC" => Some(Self::Basic),
            "IMPORT_JOB_VIEW_FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// ErrorFrameView can be specified in ErrorFrames List and Get requests to
/// control the level of details that is returned for the original frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorFrameView {
    /// Value is unset. The system will fallback to the default value.
    Unspecified = 0,
    /// Include basic frame data, but not the full contents.
    Basic = 1,
    /// Include everything.
    Full = 2,
}
impl ErrorFrameView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ErrorFrameView::Unspecified => "ERROR_FRAME_VIEW_UNSPECIFIED",
            ErrorFrameView::Basic => "ERROR_FRAME_VIEW_BASIC",
            ErrorFrameView::Full => "ERROR_FRAME_VIEW_FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ERROR_FRAME_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "ERROR_FRAME_VIEW_BASIC" => Some(Self::Basic),
            "ERROR_FRAME_VIEW_FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// The persistent disk (PD) types of Compute Engine virtual machines.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PersistentDiskType {
    /// Unspecified (default value).
    /// Selecting this value allows the system to use any disk type according
    /// to reported usage. This a good value to start with.
    Unspecified = 0,
    /// Standard HDD Persistent Disk.
    Standard = 1,
    /// Balanced Persistent Disk.
    Balanced = 2,
    /// SSD Persistent Disk.
    Ssd = 3,
}
impl PersistentDiskType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PersistentDiskType::Unspecified => "PERSISTENT_DISK_TYPE_UNSPECIFIED",
            PersistentDiskType::Standard => "PERSISTENT_DISK_TYPE_STANDARD",
            PersistentDiskType::Balanced => "PERSISTENT_DISK_TYPE_BALANCED",
            PersistentDiskType::Ssd => "PERSISTENT_DISK_TYPE_SSD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PERSISTENT_DISK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PERSISTENT_DISK_TYPE_STANDARD" => Some(Self::Standard),
            "PERSISTENT_DISK_TYPE_BALANCED" => Some(Self::Balanced),
            "PERSISTENT_DISK_TYPE_SSD" => Some(Self::Ssd),
            _ => None,
        }
    }
}
/// The License type for premium images (RHEL, RHEL for SAP, SLES, SLES for SAP,
/// Windows Server).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LicenseType {
    /// Unspecified (default value).
    Unspecified = 0,
    /// Default Google Cloud licensing plan. Licensing is charged per usage.
    /// This a good value to start with.
    Default = 1,
    /// Bring-your-own-license (BYOL) plan. User provides the OS license.
    BringYourOwnLicense = 2,
}
impl LicenseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LicenseType::Unspecified => "LICENSE_TYPE_UNSPECIFIED",
            LicenseType::Default => "LICENSE_TYPE_DEFAULT",
            LicenseType::BringYourOwnLicense => "LICENSE_TYPE_BRING_YOUR_OWN_LICENSE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LICENSE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "LICENSE_TYPE_DEFAULT" => Some(Self::Default),
            "LICENSE_TYPE_BRING_YOUR_OWN_LICENSE" => Some(Self::BringYourOwnLicense),
            _ => None,
        }
    }
}
/// The sizing optimization strategy preferences of a virtual machine. This
/// strategy, in addition to actual usage data of the virtual machine, can help
/// determine the recommended shape on the target platform.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SizingOptimizationStrategy {
    /// Unspecified (default value).
    Unspecified = 0,
    /// No optimization applied. Virtual machine sizing matches as closely as
    /// possible the machine shape on the source site, not considering any actual
    /// performance data.
    SameAsSource = 1,
    /// Virtual machine sizing will match the reported usage and shape, with some
    /// slack. This a good value to start with.
    Moderate = 2,
    /// Virtual machine sizing will match the reported usage, with little slack.
    /// Using this option can help reduce costs.
    Aggressive = 3,
}
impl SizingOptimizationStrategy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SizingOptimizationStrategy::Unspecified => {
                "SIZING_OPTIMIZATION_STRATEGY_UNSPECIFIED"
            }
            SizingOptimizationStrategy::SameAsSource => {
                "SIZING_OPTIMIZATION_STRATEGY_SAME_AS_SOURCE"
            }
            SizingOptimizationStrategy::Moderate => {
                "SIZING_OPTIMIZATION_STRATEGY_MODERATE"
            }
            SizingOptimizationStrategy::Aggressive => {
                "SIZING_OPTIMIZATION_STRATEGY_AGGRESSIVE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIZING_OPTIMIZATION_STRATEGY_UNSPECIFIED" => Some(Self::Unspecified),
            "SIZING_OPTIMIZATION_STRATEGY_SAME_AS_SOURCE" => Some(Self::SameAsSource),
            "SIZING_OPTIMIZATION_STRATEGY_MODERATE" => Some(Self::Moderate),
            "SIZING_OPTIMIZATION_STRATEGY_AGGRESSIVE" => Some(Self::Aggressive),
            _ => None,
        }
    }
}
/// The plan of commitments for VM resource-based committed use discount (CUD).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommitmentPlan {
    /// Unspecified commitment plan.
    Unspecified = 0,
    /// No commitment plan.
    None = 1,
    /// 1 year commitment.
    OneYear = 2,
    /// 3 years commitment.
    ThreeYears = 3,
}
impl CommitmentPlan {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CommitmentPlan::Unspecified => "COMMITMENT_PLAN_UNSPECIFIED",
            CommitmentPlan::None => "COMMITMENT_PLAN_NONE",
            CommitmentPlan::OneYear => "COMMITMENT_PLAN_ONE_YEAR",
            CommitmentPlan::ThreeYears => "COMMITMENT_PLAN_THREE_YEARS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMMITMENT_PLAN_UNSPECIFIED" => Some(Self::Unspecified),
            "COMMITMENT_PLAN_NONE" => Some(Self::None),
            "COMMITMENT_PLAN_ONE_YEAR" => Some(Self::OneYear),
            "COMMITMENT_PLAN_THREE_YEARS" => Some(Self::ThreeYears),
            _ => None,
        }
    }
}
/// Specifies the types of views that provide complete or partial details
/// of a Report.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReportView {
    /// The report view is not specified. The API displays the basic view by
    /// default.
    Unspecified = 0,
    /// The report view includes only basic metadata of the Report. Useful for
    /// list views.
    Basic = 1,
    /// The report view includes all the metadata of the Report. Useful for
    /// preview.
    Full = 2,
    /// The report view includes the standard metadata of an report. Useful for
    /// detail view.
    Standard = 3,
}
impl ReportView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReportView::Unspecified => "REPORT_VIEW_UNSPECIFIED",
            ReportView::Basic => "REPORT_VIEW_BASIC",
            ReportView::Full => "REPORT_VIEW_FULL",
            ReportView::Standard => "REPORT_VIEW_STANDARD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REPORT_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "REPORT_VIEW_BASIC" => Some(Self::Basic),
            "REPORT_VIEW_FULL" => Some(Self::Full),
            "REPORT_VIEW_STANDARD" => Some(Self::Standard),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod migration_center_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources.
    #[derive(Debug, Clone)]
    pub struct MigrationCenterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MigrationCenterClient<T>
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
        ) -> MigrationCenterClient<InterceptedService<T, F>>
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
            MigrationCenterClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all the assets in a given project and location.
        pub async fn list_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAssetsResponse>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/ListAssets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "ListAssets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of an asset.
        pub async fn get_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAssetRequest>,
        ) -> std::result::Result<tonic::Response<super::Asset>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.migrationcenter.v1.MigrationCenter/GetAsset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "GetAsset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of an asset.
        pub async fn update_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAssetRequest>,
        ) -> std::result::Result<tonic::Response<super::Asset>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.migrationcenter.v1.MigrationCenter/UpdateAsset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "UpdateAsset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a list of assets.
        pub async fn batch_update_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateAssetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchUpdateAssetsResponse>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/BatchUpdateAssets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "BatchUpdateAssets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an asset.
        pub async fn delete_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAssetRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/DeleteAsset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "DeleteAsset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes list of Assets.
        pub async fn batch_delete_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteAssetsRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/BatchDeleteAssets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "BatchDeleteAssets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Reports a set of frames.
        pub async fn report_asset_frames(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportAssetFramesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReportAssetFramesResponse>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/ReportAssetFrames",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "ReportAssetFrames",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Aggregates the requested fields based on provided function.
        pub async fn aggregate_assets_values(
            &mut self,
            request: impl tonic::IntoRequest<super::AggregateAssetsValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AggregateAssetsValuesResponse>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/AggregateAssetsValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "AggregateAssetsValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an import job.
        pub async fn create_import_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateImportJobRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/CreateImportJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "CreateImportJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all import jobs.
        pub async fn list_import_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListImportJobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListImportJobsResponse>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/ListImportJobs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "ListImportJobs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of an import job.
        pub async fn get_import_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetImportJobRequest>,
        ) -> std::result::Result<tonic::Response<super::ImportJob>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.migrationcenter.v1.MigrationCenter/GetImportJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "GetImportJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an import job.
        pub async fn delete_import_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteImportJobRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/DeleteImportJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "DeleteImportJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an import job.
        pub async fn update_import_job(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateImportJobRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/UpdateImportJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "UpdateImportJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Validates an import job.
        pub async fn validate_import_job(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateImportJobRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/ValidateImportJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "ValidateImportJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Runs an import job.
        pub async fn run_import_job(
            &mut self,
            request: impl tonic::IntoRequest<super::RunImportJobRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/RunImportJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "RunImportJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets an import data file.
        pub async fn get_import_data_file(
            &mut self,
            request: impl tonic::IntoRequest<super::GetImportDataFileRequest>,
        ) -> std::result::Result<tonic::Response<super::ImportDataFile>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.migrationcenter.v1.MigrationCenter/GetImportDataFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "GetImportDataFile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List import data files.
        pub async fn list_import_data_files(
            &mut self,
            request: impl tonic::IntoRequest<super::ListImportDataFilesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListImportDataFilesResponse>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/ListImportDataFiles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "ListImportDataFiles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an import data file.
        pub async fn create_import_data_file(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateImportDataFileRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/CreateImportDataFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "CreateImportDataFile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete an import data file.
        pub async fn delete_import_data_file(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteImportDataFileRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/DeleteImportDataFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "DeleteImportDataFile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all groups in a given project and location.
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/ListGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "ListGroups",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a group.
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/GetGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "GetGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new group in a given project and location.
        pub async fn create_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGroupRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/CreateGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "CreateGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a group.
        pub async fn update_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGroupRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/UpdateGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "UpdateGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a group.
        pub async fn delete_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGroupRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/DeleteGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "DeleteGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Adds assets to a group.
        pub async fn add_assets_to_group(
            &mut self,
            request: impl tonic::IntoRequest<super::AddAssetsToGroupRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/AddAssetsToGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "AddAssetsToGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Removes assets from a group.
        pub async fn remove_assets_from_group(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveAssetsFromGroupRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/RemoveAssetsFromGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "RemoveAssetsFromGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all error frames in a given source and location.
        pub async fn list_error_frames(
            &mut self,
            request: impl tonic::IntoRequest<super::ListErrorFramesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListErrorFramesResponse>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/ListErrorFrames",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "ListErrorFrames",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of an error frame.
        pub async fn get_error_frame(
            &mut self,
            request: impl tonic::IntoRequest<super::GetErrorFrameRequest>,
        ) -> std::result::Result<tonic::Response<super::ErrorFrame>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.migrationcenter.v1.MigrationCenter/GetErrorFrame",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "GetErrorFrame",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all the sources in a given project and location.
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/ListSources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "ListSources",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a source.
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/GetSource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "GetSource",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new source in a given project and location.
        pub async fn create_source(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSourceRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/CreateSource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "CreateSource",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a source.
        pub async fn update_source(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSourceRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/UpdateSource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "UpdateSource",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a source.
        pub async fn delete_source(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSourceRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/DeleteSource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "DeleteSource",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all the preference sets in a given project and location.
        pub async fn list_preference_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPreferenceSetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPreferenceSetsResponse>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/ListPreferenceSets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "ListPreferenceSets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a preference set.
        pub async fn get_preference_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPreferenceSetRequest>,
        ) -> std::result::Result<tonic::Response<super::PreferenceSet>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.migrationcenter.v1.MigrationCenter/GetPreferenceSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "GetPreferenceSet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new preference set in a given project and location.
        pub async fn create_preference_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePreferenceSetRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/CreatePreferenceSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "CreatePreferenceSet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a preference set.
        pub async fn update_preference_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePreferenceSetRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/UpdatePreferenceSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "UpdatePreferenceSet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a preference set.
        pub async fn delete_preference_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePreferenceSetRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/DeletePreferenceSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "DeletePreferenceSet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of regional settings.
        pub async fn get_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSettingsRequest>,
        ) -> std::result::Result<tonic::Response<super::Settings>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.migrationcenter.v1.MigrationCenter/GetSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "GetSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the regional-level project settings.
        pub async fn update_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSettingsRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/UpdateSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "UpdateSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a report configuration.
        pub async fn create_report_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReportConfigRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/CreateReportConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "CreateReportConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single ReportConfig.
        pub async fn get_report_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReportConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::ReportConfig>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.migrationcenter.v1.MigrationCenter/GetReportConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "GetReportConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists ReportConfigs in a given project and location.
        pub async fn list_report_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReportConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListReportConfigsResponse>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/ListReportConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "ListReportConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a ReportConfig.
        pub async fn delete_report_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReportConfigRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/DeleteReportConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "DeleteReportConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a report.
        pub async fn create_report(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReportRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/CreateReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "CreateReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Report.
        pub async fn get_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReportRequest>,
        ) -> std::result::Result<tonic::Response<super::Report>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.migrationcenter.v1.MigrationCenter/GetReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "GetReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Reports in a given ReportConfig.
        pub async fn list_reports(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReportsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListReportsResponse>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/ListReports",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "ListReports",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Report.
        pub async fn delete_report(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReportRequest>,
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
                "/google.cloud.migrationcenter.v1.MigrationCenter/DeleteReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.migrationcenter.v1.MigrationCenter",
                        "DeleteReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
