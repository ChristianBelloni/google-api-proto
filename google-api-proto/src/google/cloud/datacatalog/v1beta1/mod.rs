/// Tags are used to attach custom metadata to Data Catalog resources. Tags
/// conform to the specifications within their tag template.
///
/// See [Data Catalog
/// IAM](<https://cloud.google.com/data-catalog/docs/concepts/iam>) for information
/// on the permissions needed to create or view tags.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    /// The resource name of the tag in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}/tags/{tag_id}
    ///
    /// where `tag_id` is a system-generated identifier.
    /// Note that this Tag may not actually be stored in the location in this name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The resource name of the tag template that this tag uses.
    /// Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}
    ///
    /// This field cannot be modified after creation.
    #[prost(string, tag = "2")]
    pub template: ::prost::alloc::string::String,
    /// Output only. The display name of the tag template.
    #[prost(string, tag = "5")]
    pub template_display_name: ::prost::alloc::string::String,
    /// Required. This maps the ID of a tag field to the value of and additional
    /// information about that field. Valid field IDs are defined by the tag's
    /// template. A tag must have at least 1 field and at most 500 fields.
    #[prost(btree_map = "string, message", tag = "3")]
    pub fields: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        TagField,
    >,
    /// The scope within the parent resource that this tag is attached to. If not
    /// provided, the tag is attached to the parent resource itself.
    /// Deleting the scope from the parent resource will delete all tags attached
    /// to that scope. These fields cannot be updated after creation.
    #[prost(oneof = "tag::Scope", tags = "4")]
    pub scope: ::core::option::Option<tag::Scope>,
}
/// Nested message and enum types in `Tag`.
pub mod tag {
    /// The scope within the parent resource that this tag is attached to. If not
    /// provided, the tag is attached to the parent resource itself.
    /// Deleting the scope from the parent resource will delete all tags attached
    /// to that scope. These fields cannot be updated after creation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scope {
        /// Resources like Entry can have schemas associated with them. This scope
        /// allows users to attach tags to an individual column based on that schema.
        ///
        /// For attaching a tag to a nested column, use `.` to separate the column
        /// names. Example:
        ///
        /// * `outer_column.inner_column`
        #[prost(string, tag = "4")]
        Column(::prost::alloc::string::String),
    }
}
/// Contains the value and supporting information for a field within
/// a \[Tag][google.cloud.datacatalog.v1beta1.Tag\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagField {
    /// Output only. The display name of this field.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The order of this field with respect to other fields in this
    /// tag. It can be set in
    /// \[Tag][google.cloud.datacatalog.v1beta1.TagTemplateField.order\]. For
    /// example, a higher value can indicate a more important field. The value can
    /// be negative. Multiple fields can have the same order, and field orders
    /// within a tag do not have to be sequential.
    #[prost(int32, tag = "7")]
    pub order: i32,
    /// Required. The value of this field.
    #[prost(oneof = "tag_field::Kind", tags = "2, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<tag_field::Kind>,
}
/// Nested message and enum types in `TagField`.
pub mod tag_field {
    /// Holds an enum value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnumValue {
        /// The display name of the enum value.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
    }
    /// Required. The value of this field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Holds the value for a tag field with double type.
        #[prost(double, tag = "2")]
        DoubleValue(f64),
        /// Holds the value for a tag field with string type.
        #[prost(string, tag = "3")]
        StringValue(::prost::alloc::string::String),
        /// Holds the value for a tag field with boolean type.
        #[prost(bool, tag = "4")]
        BoolValue(bool),
        /// Holds the value for a tag field with timestamp type.
        #[prost(message, tag = "5")]
        TimestampValue(::prost_types::Timestamp),
        /// Holds the value for a tag field with enum type. This value must be
        /// one of the allowed values in the definition of this enum.
        #[prost(message, tag = "6")]
        EnumValue(EnumValue),
    }
}
/// A tag template defines a tag, which can have one or more typed fields.
/// The template is used to create and attach the tag to Google Cloud resources.
/// [Tag template
/// roles](<https://cloud.google.com/iam/docs/understanding-roles#data-catalog-roles>)
/// provide permissions to create, edit, and use the template. See, for example,
/// the [TagTemplate
/// User](<https://cloud.google.com/data-catalog/docs/how-to/template-user>) role,
/// which includes permission to use the tag template to tag resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagTemplate {
    /// The resource name of the tag template in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}
    ///
    /// Note that this TagTemplate and its child resources may not actually be
    /// stored in the location in this name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The display name for this template. Defaults to an empty string.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Map of tag template field IDs to the settings for the field.
    /// This map is an exhaustive list of the allowed fields. This map must contain
    /// at least one field and at most 500 fields.
    ///
    /// The keys to this map are tag template field IDs. Field IDs can contain
    /// letters (both uppercase and lowercase), numbers (0-9) and underscores (_).
    /// Field IDs must be at least 1 character long and at most
    /// 64 characters long. Field IDs must start with a letter or underscore.
    #[prost(btree_map = "string, message", tag = "3")]
    pub fields: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        TagTemplateField,
    >,
}
/// The template for an individual field within a tag template.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagTemplateField {
    /// Output only. The resource name of the tag template field in URL format.
    /// Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template}/fields/{field}
    ///
    /// Note that this TagTemplateField may not actually be stored in the location
    /// in this name.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// The display name for this field. Defaults to an empty string.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The type of value this tag field can contain.
    #[prost(message, optional, tag = "2")]
    pub r#type: ::core::option::Option<FieldType>,
    /// Whether this is a required field. Defaults to false.
    #[prost(bool, tag = "3")]
    pub is_required: bool,
    /// The description for this field. Defaults to an empty string.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// The order of this field with respect to other fields in this tag
    /// template.  A higher value indicates a more important field. The value can
    /// be negative. Multiple fields can have the same order, and field orders
    /// within a tag do not have to be sequential.
    #[prost(int32, tag = "5")]
    pub order: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldType {
    /// Required.
    #[prost(oneof = "field_type::TypeDecl", tags = "1, 2")]
    pub type_decl: ::core::option::Option<field_type::TypeDecl>,
}
/// Nested message and enum types in `FieldType`.
pub mod field_type {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnumType {
        #[prost(message, repeated, tag = "1")]
        pub allowed_values: ::prost::alloc::vec::Vec<enum_type::EnumValue>,
    }
    /// Nested message and enum types in `EnumType`.
    pub mod enum_type {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct EnumValue {
            /// Required. The display name of the enum value. Must not be an empty
            /// string.
            #[prost(string, tag = "1")]
            pub display_name: ::prost::alloc::string::String,
        }
    }
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
    pub enum PrimitiveType {
        /// This is the default invalid value for a type.
        Unspecified = 0,
        /// A double precision number.
        Double = 1,
        /// An UTF-8 string.
        String = 2,
        /// A boolean value.
        Bool = 3,
        /// A timestamp.
        Timestamp = 4,
    }
    impl PrimitiveType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PrimitiveType::Unspecified => "PRIMITIVE_TYPE_UNSPECIFIED",
                PrimitiveType::Double => "DOUBLE",
                PrimitiveType::String => "STRING",
                PrimitiveType::Bool => "BOOL",
                PrimitiveType::Timestamp => "TIMESTAMP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PRIMITIVE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DOUBLE" => Some(Self::Double),
                "STRING" => Some(Self::String),
                "BOOL" => Some(Self::Bool),
                "TIMESTAMP" => Some(Self::Timestamp),
                _ => None,
            }
        }
    }
    /// Required.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeDecl {
        /// Represents primitive types - string, bool etc.
        #[prost(enumeration = "PrimitiveType", tag = "1")]
        PrimitiveType(i32),
        /// Represents an enum type.
        #[prost(message, tag = "2")]
        EnumType(EnumType),
    }
}
/// This enum describes all the possible systems that Data Catalog integrates
/// with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IntegratedSystem {
    /// Default unknown system.
    Unspecified = 0,
    /// BigQuery.
    Bigquery = 1,
    /// Cloud Pub/Sub.
    CloudPubsub = 2,
}
impl IntegratedSystem {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IntegratedSystem::Unspecified => "INTEGRATED_SYSTEM_UNSPECIFIED",
            IntegratedSystem::Bigquery => "BIGQUERY",
            IntegratedSystem::CloudPubsub => "CLOUD_PUBSUB",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INTEGRATED_SYSTEM_UNSPECIFIED" => Some(Self::Unspecified),
            "BIGQUERY" => Some(Self::Bigquery),
            "CLOUD_PUBSUB" => Some(Self::CloudPubsub),
            _ => None,
        }
    }
}
/// This enum describes all the systems that manage
/// Taxonomy and PolicyTag resources in DataCatalog.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ManagingSystem {
    /// Default value
    Unspecified = 0,
    /// Dataplex.
    Dataplex = 1,
    /// Other
    Other = 2,
}
impl ManagingSystem {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ManagingSystem::Unspecified => "MANAGING_SYSTEM_UNSPECIFIED",
            ManagingSystem::Dataplex => "MANAGING_SYSTEM_DATAPLEX",
            ManagingSystem::Other => "MANAGING_SYSTEM_OTHER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MANAGING_SYSTEM_UNSPECIFIED" => Some(Self::Unspecified),
            "MANAGING_SYSTEM_DATAPLEX" => Some(Self::Dataplex),
            "MANAGING_SYSTEM_OTHER" => Some(Self::Other),
            _ => None,
        }
    }
}
/// Timestamps about this resource according to a particular system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemTimestamps {
    /// The creation time of the resource within the given system.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The last-modified time of the resource within the given system.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The expiration time of the resource within the given system.
    /// Currently only apllicable to BigQuery resources.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Describes a Cloud Storage fileset entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsFilesetSpec {
    /// Required. Patterns to identify a set of files in Google Cloud Storage.
    /// See [Cloud Storage
    /// documentation](<https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames>)
    /// for more information. Note that bucket wildcards are currently not
    /// supported.
    ///
    /// Examples of valid file_patterns:
    ///
    ///   * `gs://bucket_name/dir/*`: matches all files within `bucket_name/dir`
    ///                               directory.
    ///   * `gs://bucket_name/dir/**`: matches all files in `bucket_name/dir`
    ///                                spanning all subdirectories.
    ///   * `gs://bucket_name/file*`: matches files prefixed by `file` in
    ///                               `bucket_name`
    ///   * `gs://bucket_name/??.txt`: matches files with two characters followed by
    ///                                `.txt` in `bucket_name`
    ///   * `gs://bucket_name/\[aeiou\].txt`: matches files that contain a single
    ///                                     vowel character followed by `.txt` in
    ///                                     `bucket_name`
    ///   * `gs://bucket_name/\[a-m\].txt`: matches files that contain `a`, `b`, ...
    ///                                   or `m` followed by `.txt` in `bucket_name`
    ///   * `gs://bucket_name/a/*/b`: matches all files in `bucket_name` that match
    ///                               `a/*/b` pattern, such as `a/c/b`, `a/d/b`
    ///   * `gs://another_bucket/a.txt`: matches `gs://another_bucket/a.txt`
    ///
    /// You can combine wildcards to provide more powerful matches, for example:
    ///
    ///   * `gs://bucket_name/\[a-m\]??.j*g`
    #[prost(string, repeated, tag = "1")]
    pub file_patterns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Sample files contained in this fileset, not all files
    /// contained in this fileset are represented here.
    #[prost(message, repeated, tag = "2")]
    pub sample_gcs_file_specs: ::prost::alloc::vec::Vec<GcsFileSpec>,
}
/// Specifications of a single file in Cloud Storage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsFileSpec {
    /// Required. The full file path. Example: `gs://bucket_name/a/b.txt`.
    #[prost(string, tag = "1")]
    pub file_path: ::prost::alloc::string::String,
    /// Output only. Timestamps about the Cloud Storage file.
    #[prost(message, optional, tag = "2")]
    pub gcs_timestamps: ::core::option::Option<SystemTimestamps>,
    /// Output only. The size of the file, in bytes.
    #[prost(int64, tag = "4")]
    pub size_bytes: i64,
}
/// Represents a schema (e.g. BigQuery, GoogleSQL, Avro schema).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// Required. Schema of columns. A maximum of 10,000 columns and sub-columns
    /// can be specified.
    #[prost(message, repeated, tag = "2")]
    pub columns: ::prost::alloc::vec::Vec<ColumnSchema>,
}
/// Representation of a column within a schema. Columns could be nested inside
/// other columns.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnSchema {
    /// Required. Name of the column.
    #[prost(string, tag = "6")]
    pub column: ::prost::alloc::string::String,
    /// Required. Type of the column.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Optional. Description of the column. Default value is an empty string.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Optional. A column's mode indicates whether the values in this column are
    /// required, nullable, etc. Only `NULLABLE`, `REQUIRED` and `REPEATED` are
    /// supported. Default mode is `NULLABLE`.
    #[prost(string, tag = "3")]
    pub mode: ::prost::alloc::string::String,
    /// Optional. Schema of sub-columns. A column can have zero or more
    /// sub-columns.
    #[prost(message, repeated, tag = "7")]
    pub subcolumns: ::prost::alloc::vec::Vec<ColumnSchema>,
}
/// A result that appears in the response of a search request. Each result
/// captures details of one entry that matches the search.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogResult {
    /// Type of the search result. This field can be used to determine which Get
    /// method to call to fetch the full resource.
    #[prost(enumeration = "SearchResultType", tag = "1")]
    pub search_result_type: i32,
    /// Sub-type of the search result. This is a dot-delimited description of the
    /// resource's full type, and is the same as the value callers would provide in
    /// the "type" search facet.  Examples: `entry.table`, `entry.dataStream`,
    /// `tagTemplate`.
    #[prost(string, tag = "2")]
    pub search_result_subtype: ::prost::alloc::string::String,
    /// The relative resource name of the resource in URL format.
    /// Examples:
    ///
    ///   * `projects/{project_id}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}`
    ///   * `projects/{project_id}/tagTemplates/{tag_template_id}`
    #[prost(string, tag = "3")]
    pub relative_resource_name: ::prost::alloc::string::String,
    /// The full name of the cloud resource the entry belongs to. See:
    /// <https://cloud.google.com/apis/design/resource_names#full_resource_name.>
    /// Example:
    ///
    ///   * `//bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId`
    #[prost(string, tag = "4")]
    pub linked_resource: ::prost::alloc::string::String,
    /// Last-modified timestamp of the entry from the managing system.
    #[prost(message, optional, tag = "7")]
    pub modify_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The different types of resources that can be returned in search.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SearchResultType {
    /// Default unknown type.
    Unspecified = 0,
    /// An \[Entry][google.cloud.datacatalog.v1beta1.Entry\].
    Entry = 1,
    /// A \[TagTemplate][google.cloud.datacatalog.v1beta1.TagTemplate\].
    TagTemplate = 2,
    /// An \[EntryGroup][google.cloud.datacatalog.v1beta1.EntryGroup\].
    EntryGroup = 3,
}
impl SearchResultType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SearchResultType::Unspecified => "SEARCH_RESULT_TYPE_UNSPECIFIED",
            SearchResultType::Entry => "ENTRY",
            SearchResultType::TagTemplate => "TAG_TEMPLATE",
            SearchResultType::EntryGroup => "ENTRY_GROUP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SEARCH_RESULT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ENTRY" => Some(Self::Entry),
            "TAG_TEMPLATE" => Some(Self::TagTemplate),
            "ENTRY_GROUP" => Some(Self::EntryGroup),
            _ => None,
        }
    }
}
/// Describes a BigQuery table.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryTableSpec {
    /// Output only. The table source type.
    #[prost(enumeration = "TableSourceType", tag = "1")]
    pub table_source_type: i32,
    /// Output only.
    #[prost(oneof = "big_query_table_spec::TypeSpec", tags = "2, 3")]
    pub type_spec: ::core::option::Option<big_query_table_spec::TypeSpec>,
}
/// Nested message and enum types in `BigQueryTableSpec`.
pub mod big_query_table_spec {
    /// Output only.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeSpec {
        /// Table view specification. This field should only be populated if
        /// `table_source_type` is `BIGQUERY_VIEW`.
        #[prost(message, tag = "2")]
        ViewSpec(super::ViewSpec),
        /// Spec of a BigQuery table. This field should only be populated if
        /// `table_source_type` is `BIGQUERY_TABLE`.
        #[prost(message, tag = "3")]
        TableSpec(super::TableSpec),
    }
}
/// Table view specification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewSpec {
    /// Output only. The query that defines the table view.
    #[prost(string, tag = "1")]
    pub view_query: ::prost::alloc::string::String,
}
/// Normal BigQuery table spec.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableSpec {
    /// Output only. If the table is a dated shard, i.e., with name pattern
    /// `\[prefix\]YYYYMMDD`, `grouped_entry` is the Data Catalog resource name of
    /// the date sharded grouped entry, for example,
    /// `projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}`.
    /// Otherwise, `grouped_entry` is empty.
    #[prost(string, tag = "1")]
    pub grouped_entry: ::prost::alloc::string::String,
}
/// Spec for a group of BigQuery tables with name pattern `\[prefix\]YYYYMMDD`.
/// Context:
/// <https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryDateShardedSpec {
    /// Output only. The Data Catalog resource name of the dataset entry the
    /// current table belongs to, for example,
    /// `projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}`.
    #[prost(string, tag = "1")]
    pub dataset: ::prost::alloc::string::String,
    /// Output only. The table name prefix of the shards. The name of any given
    /// shard is
    /// `\[table_prefix\]YYYYMMDD`, for example, for shard `MyTable20180101`, the
    /// `table_prefix` is `MyTable`.
    #[prost(string, tag = "2")]
    pub table_prefix: ::prost::alloc::string::String,
    /// Output only. Total number of shards.
    #[prost(int64, tag = "3")]
    pub shard_count: i64,
}
/// Table source type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TableSourceType {
    /// Default unknown type.
    Unspecified = 0,
    /// Table view.
    BigqueryView = 2,
    /// BigQuery native table.
    BigqueryTable = 5,
    /// BigQuery materialized view.
    BigqueryMaterializedView = 7,
}
impl TableSourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TableSourceType::Unspecified => "TABLE_SOURCE_TYPE_UNSPECIFIED",
            TableSourceType::BigqueryView => "BIGQUERY_VIEW",
            TableSourceType::BigqueryTable => "BIGQUERY_TABLE",
            TableSourceType::BigqueryMaterializedView => "BIGQUERY_MATERIALIZED_VIEW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TABLE_SOURCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "BIGQUERY_VIEW" => Some(Self::BigqueryView),
            "BIGQUERY_TABLE" => Some(Self::BigqueryTable),
            "BIGQUERY_MATERIALIZED_VIEW" => Some(Self::BigqueryMaterializedView),
            _ => None,
        }
    }
}
/// Detailed counts on the entry's usage.
/// Caveats:
///    - Only BigQuery tables have usage stats
///    - The usage stats only include BigQuery query jobs
///    - The usage stats might be underestimated, e.g. wildcard table references
///      are not yet counted in usage computation
///      <https://cloud.google.com/bigquery/docs/querying-wildcard-tables>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsageStats {
    /// The number of times that the underlying entry was successfully used.
    #[prost(float, tag = "1")]
    pub total_completions: f32,
    /// The number of times that the underlying entry was attempted to be used
    /// but failed.
    #[prost(float, tag = "2")]
    pub total_failures: f32,
    /// The number of times that the underlying entry was attempted to be used
    /// but was cancelled by the user.
    #[prost(float, tag = "3")]
    pub total_cancellations: f32,
    /// Total time spent (in milliseconds) during uses the resulted in completions.
    #[prost(float, tag = "4")]
    pub total_execution_time_for_completions_millis: f32,
}
/// The set of all usage signals that we store in Data Catalog.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsageSignal {
    /// The timestamp of the end of the usage statistics duration.
    #[prost(message, optional, tag = "1")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Usage statistics over each of the pre-defined time ranges, supported
    /// strings for time ranges are {"24H", "7D", "30D"}.
    #[prost(btree_map = "string, message", tag = "2")]
    pub usage_within_time_range: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        UsageStats,
    >,
}
/// Request message for
/// \[SearchCatalog][google.cloud.datacatalog.v1beta1.DataCatalog.SearchCatalog\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogRequest {
    /// Required. The scope of this search request. A `scope` that has empty
    /// `include_org_ids`, `include_project_ids` AND false
    /// `include_gcp_public_datasets` is considered invalid. Data Catalog will
    /// return an error in such a case.
    #[prost(message, optional, tag = "6")]
    pub scope: ::core::option::Option<search_catalog_request::Scope>,
    /// Optional. The query string in search query syntax. An empty query string
    /// will result in all data assets (in the specified scope) that the user has
    /// access to. Query strings can be simple as "x" or more qualified as:
    ///
    /// * name:x
    /// * column:x
    /// * description:y
    ///
    /// Note: Query tokens need to have a minimum of 3 characters for substring
    /// matching to work correctly. See [Data Catalog Search
    /// Syntax](<https://cloud.google.com/data-catalog/docs/how-to/search-reference>)
    /// for more information.
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// Number of results in the search page. If <=0 then defaults to 10. Max limit
    /// for page_size is 1000. Throws an invalid argument for page_size > 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token returned in an earlier
    /// \[SearchCatalogResponse.next_page_token][google.cloud.datacatalog.v1beta1.SearchCatalogResponse.next_page_token\],
    /// which indicates that this is a continuation of a prior
    /// \[SearchCatalogRequest][google.cloud.datacatalog.v1beta1.DataCatalog.SearchCatalog\]
    /// call, and that the system should return the next page of data. If empty,
    /// the first page is returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Specifies the ordering of results, currently supported case-sensitive
    /// choices are:
    ///
    ///    * `relevance`, only supports descending
    ///    * `last_modified_timestamp \[asc|desc\]`, defaults to descending if not
    ///      specified
    ///    * `default` that can only be descending
    ///
    /// If not specified, defaults to `relevance` descending.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SearchCatalogRequest`.
pub mod search_catalog_request {
    /// The criteria that select the subspace used for query matching.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Scope {
        /// The list of organization IDs to search within. To find your organization
        /// ID, follow instructions in
        /// <https://cloud.google.com/resource-manager/docs/creating-managing-organization.>
        #[prost(string, repeated, tag = "2")]
        pub include_org_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The list of project IDs to search within. To learn more about the
        /// distinction between project names/IDs/numbers, go to
        /// <https://cloud.google.com/docs/overview/#projects.>
        #[prost(string, repeated, tag = "3")]
        pub include_project_ids: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// If `true`, include Google Cloud public datasets in the
        /// search results. Info on Google Cloud public datasets is available at
        /// <https://cloud.google.com/public-datasets/.> By default, Google Cloud
        /// public datasets are excluded.
        #[prost(bool, tag = "7")]
        pub include_gcp_public_datasets: bool,
        /// Optional. The list of locations to search within.
        /// 1. If empty, search will be performed in all locations;
        /// 2. If any of the locations are NOT in the valid locations list, error
        /// will be returned;
        /// 3. Otherwise, search only the given locations for matching results.
        /// Typical usage is to leave this field empty. When a location is
        /// unreachable as returned in the `SearchCatalogResponse.unreachable` field,
        /// users can repeat the search request with this parameter set to get
        /// additional information on the error.
        ///
        /// Valid locations:
        ///   * asia-east1
        ///   * asia-east2
        ///   * asia-northeast1
        ///   * asia-northeast2
        ///   * asia-northeast3
        ///   * asia-south1
        ///   * asia-southeast1
        ///   * australia-southeast1
        ///   * eu
        ///   * europe-north1
        ///   * europe-west1
        ///   * europe-west2
        ///   * europe-west3
        ///   * europe-west4
        ///   * europe-west6
        ///   * global
        ///   * northamerica-northeast1
        ///   * southamerica-east1
        ///   * us
        ///   * us-central1
        ///   * us-east1
        ///   * us-east4
        ///   * us-west1
        ///   * us-west2
        #[prost(string, repeated, tag = "16")]
        pub restricted_locations: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
}
/// Response message for
/// \[SearchCatalog][google.cloud.datacatalog.v1beta1.DataCatalog.SearchCatalog\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogResponse {
    /// Search results.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<SearchCatalogResult>,
    /// The approximate total number of entries matched by the query.
    #[prost(int32, tag = "2")]
    pub total_size: i32,
    /// The token that can be used to retrieve the next page of results.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable locations. Search result does not include data from those
    /// locations. Users can get additional information on the error by repeating
    /// the search request with a more restrictive parameter -- setting the value
    /// for `SearchDataCatalogRequest.scope.restricted_locations`.
    #[prost(string, repeated, tag = "6")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// \[CreateEntryGroup][google.cloud.datacatalog.v1beta1.DataCatalog.CreateEntryGroup\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntryGroupRequest {
    /// Required. The name of the project this entry group is in. Example:
    ///
    /// * projects/{project_id}/locations/{location}
    ///
    /// Note that this EntryGroup and its child resources may not actually be
    /// stored in the location in this name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The id of the entry group to create.
    /// The id must begin with a letter or underscore, contain only English
    /// letters, numbers and underscores, and be at most 64 characters.
    #[prost(string, tag = "3")]
    pub entry_group_id: ::prost::alloc::string::String,
    /// The entry group to create. Defaults to an empty entry group.
    #[prost(message, optional, tag = "2")]
    pub entry_group: ::core::option::Option<EntryGroup>,
}
/// Request message for
/// \[UpdateEntryGroup][google.cloud.datacatalog.v1beta1.DataCatalog.UpdateEntryGroup\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntryGroupRequest {
    /// Required. The updated entry group. "name" field must be set.
    #[prost(message, optional, tag = "1")]
    pub entry_group: ::core::option::Option<EntryGroup>,
    /// Names of fields whose values to overwrite on an entry group.
    ///
    /// If this parameter is absent or empty, all modifiable fields
    /// are overwritten. If such fields are non-required and omitted in the
    /// request body, their values are emptied.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// \[GetEntryGroup][google.cloud.datacatalog.v1beta1.DataCatalog.GetEntryGroup\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntryGroupRequest {
    /// Required. The name of the entry group. For example,
    /// `projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The fields to return. If not set or empty, all fields are returned.
    #[prost(message, optional, tag = "2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// \[DeleteEntryGroup][google.cloud.datacatalog.v1beta1.DataCatalog.DeleteEntryGroup\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntryGroupRequest {
    /// Required. The name of the entry group. For example,
    /// `projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If true, deletes all entries in the entry group.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for
/// \[ListEntryGroups][google.cloud.datacatalog.v1beta1.DataCatalog.ListEntryGroups\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntryGroupsRequest {
    /// Required. The name of the location that contains the entry groups, which
    /// can be provided in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return. Default is 10. Max limit
    /// is 1000. Throws an invalid argument for `page_size > 1000`.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Token that specifies which page is requested. If empty, the first
    /// page is returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[ListEntryGroups][google.cloud.datacatalog.v1beta1.DataCatalog.ListEntryGroups\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntryGroupsResponse {
    /// EntryGroup details.
    #[prost(message, repeated, tag = "1")]
    pub entry_groups: ::prost::alloc::vec::Vec<EntryGroup>,
    /// Token to retrieve the next page of results. It is set to empty if no items
    /// remain in results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// \[CreateEntry][google.cloud.datacatalog.v1beta1.DataCatalog.CreateEntry\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntryRequest {
    /// Required. The name of the entry group this entry is in. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}
    ///
    /// Note that this Entry and its child resources may not actually be stored in
    /// the location in this name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The id of the entry to create.
    #[prost(string, tag = "3")]
    pub entry_id: ::prost::alloc::string::String,
    /// Required. The entry to create.
    #[prost(message, optional, tag = "2")]
    pub entry: ::core::option::Option<Entry>,
}
/// Request message for
/// \[UpdateEntry][google.cloud.datacatalog.v1beta1.DataCatalog.UpdateEntry\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntryRequest {
    /// Required. The updated entry. The "name" field must be set.
    #[prost(message, optional, tag = "1")]
    pub entry: ::core::option::Option<Entry>,
    /// Names of fields whose values to overwrite on an entry.
    ///
    /// If this parameter is absent or empty, all modifiable fields
    /// are overwritten. If such fields are non-required and omitted in the
    /// request body, their values are emptied.
    ///
    /// The following fields are modifiable:
    ///
    /// * For entries with type `DATA_STREAM`:
    ///     * `schema`
    /// * For entries with type `FILESET`:
    ///     * `schema`
    ///     * `display_name`
    ///     * `description`
    ///     * `gcs_fileset_spec`
    ///     * `gcs_fileset_spec.file_patterns`
    /// * For entries with `user_specified_type`:
    ///     * `schema`
    ///     * `display_name`
    ///     * `description`
    ///     * `user_specified_type`
    ///     * `user_specified_system`
    ///     * `linked_resource`
    ///     * `source_system_timestamps`
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// \[DeleteEntry][google.cloud.datacatalog.v1beta1.DataCatalog.DeleteEntry\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntryRequest {
    /// Required. The name of the entry. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[GetEntry][google.cloud.datacatalog.v1beta1.DataCatalog.GetEntry\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntryRequest {
    /// Required. The name of the entry. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[LookupEntry][google.cloud.datacatalog.v1beta1.DataCatalog.LookupEntry\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEntryRequest {
    /// Required. Represents either the Google Cloud Platform resource or SQL name
    /// for a Google Cloud Platform resource.
    #[prost(oneof = "lookup_entry_request::TargetName", tags = "1, 3")]
    pub target_name: ::core::option::Option<lookup_entry_request::TargetName>,
}
/// Nested message and enum types in `LookupEntryRequest`.
pub mod lookup_entry_request {
    /// Required. Represents either the Google Cloud Platform resource or SQL name
    /// for a Google Cloud Platform resource.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetName {
        /// The full name of the Google Cloud Platform resource the Data Catalog
        /// entry represents. See:
        /// <https://cloud.google.com/apis/design/resource_names#full_resource_name.>
        /// Full names are case-sensitive.
        ///
        /// Examples:
        ///
        ///   * //bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId
        ///   * //pubsub.googleapis.com/projects/projectId/topics/topicId
        #[prost(string, tag = "1")]
        LinkedResource(::prost::alloc::string::String),
        /// The SQL name of the entry. SQL names are case-sensitive.
        ///
        /// Examples:
        ///
        ///    * `pubsub.project_id.topic_id`
        ///    * ``pubsub.project_id.`topic.id.with.dots` ``
        ///    * `bigquery.table.project_id.dataset_id.table_id`
        ///    * `bigquery.dataset.project_id.dataset_id`
        ///    * `datacatalog.entry.project_id.location_id.entry_group_id.entry_id`
        ///
        /// `*_id`s should satisfy the standard SQL rules for identifiers.
        /// <https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical.>
        #[prost(string, tag = "3")]
        SqlResource(::prost::alloc::string::String),
    }
}
/// Entry Metadata.
/// A Data Catalog Entry resource represents another resource in Google
/// Cloud Platform (such as a BigQuery dataset or a Pub/Sub topic), or
/// outside of Google Cloud Platform. Clients can use the `linked_resource` field
/// in the Entry resource to refer to the original resource ID of the source
/// system.
///
/// An Entry resource contains resource details, such as its schema. An Entry can
/// also be used to attach flexible metadata, such as a
/// \[Tag][google.cloud.datacatalog.v1beta1.Tag\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entry {
    /// Output only. The Data Catalog resource name of the entry in URL format.
    /// Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}
    ///
    /// Note that this Entry and its child resources may not actually be stored in
    /// the location in this name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The resource this metadata entry refers to.
    ///
    /// For Google Cloud Platform resources, `linked_resource` is the [full name of
    /// the
    /// resource](<https://cloud.google.com/apis/design/resource_names#full_resource_name>).
    /// For example, the `linked_resource` for a table resource from BigQuery is:
    ///
    /// * //bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId
    ///
    /// Output only when Entry is of type in the EntryType enum. For entries with
    /// user_specified_type, this field is optional and defaults to an empty
    /// string.
    #[prost(string, tag = "9")]
    pub linked_resource: ::prost::alloc::string::String,
    /// Display information such as title and description. A short name to identify
    /// the entry, for example, "Analytics Data - Jan 2011". Default value is an
    /// empty string.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Entry description, which can consist of several sentences or paragraphs
    /// that describe entry contents. Default value is an empty string.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Schema of the entry. An entry might not have any schema attached to it.
    #[prost(message, optional, tag = "5")]
    pub schema: ::core::option::Option<Schema>,
    /// Output only. Timestamps about the underlying resource, not about this Data
    /// Catalog entry. Output only when Entry is of type in the EntryType enum. For
    /// entries with user_specified_type, this field is optional and defaults to an
    /// empty timestamp.
    #[prost(message, optional, tag = "7")]
    pub source_system_timestamps: ::core::option::Option<SystemTimestamps>,
    /// Output only. Statistics on the usage level of the resource.
    #[prost(message, optional, tag = "13")]
    pub usage_signal: ::core::option::Option<UsageSignal>,
    /// Required. Entry type.
    #[prost(oneof = "entry::EntryType", tags = "2, 16")]
    pub entry_type: ::core::option::Option<entry::EntryType>,
    /// The source system of the entry.
    #[prost(oneof = "entry::System", tags = "17, 18")]
    pub system: ::core::option::Option<entry::System>,
    /// Type specification information.
    #[prost(oneof = "entry::TypeSpec", tags = "6, 12, 15")]
    pub type_spec: ::core::option::Option<entry::TypeSpec>,
}
/// Nested message and enum types in `Entry`.
pub mod entry {
    /// Required. Entry type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EntryType {
        /// The type of the entry.
        /// Only used for Entries with types in the EntryType enum.
        #[prost(enumeration = "super::EntryType", tag = "2")]
        Type(i32),
        /// Entry type if it does not fit any of the input-allowed values listed in
        /// `EntryType` enum above. When creating an entry, users should check the
        /// enum values first, if nothing matches the entry to be created, then
        /// provide a custom value, for example "my_special_type".
        /// `user_specified_type` strings must begin with a letter or underscore and
        /// can only contain letters, numbers, and underscores; are case insensitive;
        /// must be at least 1 character and at most 64 characters long.
        ///
        /// Currently, only FILESET enum value is allowed. All other entries created
        /// through Data Catalog must use `user_specified_type`.
        #[prost(string, tag = "16")]
        UserSpecifiedType(::prost::alloc::string::String),
    }
    /// The source system of the entry.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum System {
        /// Output only. This field indicates the entry's source system that Data
        /// Catalog integrates with, such as BigQuery or Pub/Sub.
        #[prost(enumeration = "super::IntegratedSystem", tag = "17")]
        IntegratedSystem(i32),
        /// This field indicates the entry's source system that Data Catalog does not
        /// integrate with. `user_specified_system` strings must begin with a letter
        /// or underscore and can only contain letters, numbers, and underscores; are
        /// case insensitive; must be at least 1 character and at most 64 characters
        /// long.
        #[prost(string, tag = "18")]
        UserSpecifiedSystem(::prost::alloc::string::String),
    }
    /// Type specification information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeSpec {
        /// Specification that applies to a Cloud Storage fileset. This is only valid
        /// on entries of type FILESET.
        #[prost(message, tag = "6")]
        GcsFilesetSpec(super::GcsFilesetSpec),
        /// Specification that applies to a BigQuery table. This is only valid on
        /// entries of type `TABLE`.
        #[prost(message, tag = "12")]
        BigqueryTableSpec(super::BigQueryTableSpec),
        /// Specification for a group of BigQuery tables with name pattern
        /// `\[prefix\]YYYYMMDD`. Context:
        /// <https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding.>
        #[prost(message, tag = "15")]
        BigqueryDateShardedSpec(super::BigQueryDateShardedSpec),
    }
}
/// EntryGroup Metadata.
/// An EntryGroup resource represents a logical grouping of zero or more
/// Data Catalog \[Entry][google.cloud.datacatalog.v1beta1.Entry\] resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntryGroup {
    /// The resource name of the entry group in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}
    ///
    /// Note that this EntryGroup and its child resources may not actually be
    /// stored in the location in this name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A short name to identify the entry group, for example,
    /// "analytics data - jan 2011". Default value is an empty string.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Entry group description, which can consist of several sentences or
    /// paragraphs that describe entry group contents. Default value is an empty
    /// string.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Timestamps about this EntryGroup. Default value is empty
    /// timestamps.
    #[prost(message, optional, tag = "4")]
    pub data_catalog_timestamps: ::core::option::Option<SystemTimestamps>,
}
/// Request message for
/// \[CreateTagTemplate][google.cloud.datacatalog.v1beta1.DataCatalog.CreateTagTemplate\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagTemplateRequest {
    /// Required. The name of the project and the template location
    /// \[region\](<https://cloud.google.com/data-catalog/docs/concepts/regions.>
    ///
    /// Example:
    ///
    /// * projects/{project_id}/locations/us-central1
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The id of the tag template to create.
    #[prost(string, tag = "3")]
    pub tag_template_id: ::prost::alloc::string::String,
    /// Required. The tag template to create.
    #[prost(message, optional, tag = "2")]
    pub tag_template: ::core::option::Option<TagTemplate>,
}
/// Request message for
/// \[GetTagTemplate][google.cloud.datacatalog.v1beta1.DataCatalog.GetTagTemplate\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTagTemplateRequest {
    /// Required. The name of the tag template. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[UpdateTagTemplate][google.cloud.datacatalog.v1beta1.DataCatalog.UpdateTagTemplate\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagTemplateRequest {
    /// Required. The template to update. The "name" field must be set.
    #[prost(message, optional, tag = "1")]
    pub tag_template: ::core::option::Option<TagTemplate>,
    /// Names of fields whose values to overwrite on a tag template. Currently,
    /// only `display_name` can be overwritten.
    ///
    /// In general, if this parameter is absent or empty, all modifiable fields
    /// are overwritten. If such fields are non-required and omitted in the
    /// request body, their values are emptied.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// \[DeleteTagTemplate][google.cloud.datacatalog.v1beta1.DataCatalog.DeleteTagTemplate\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagTemplateRequest {
    /// Required. The name of the tag template to delete. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Currently, this field must always be set to `true`.
    /// This confirms the deletion of any possible tags using this template.
    /// `force = false` will be supported in the future.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for
/// \[CreateTag][google.cloud.datacatalog.v1beta1.DataCatalog.CreateTag\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagRequest {
    /// Required. The name of the resource to attach this tag to. Tags can be
    /// attached to Entries. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}
    ///
    /// Note that this Tag and its child resources may not actually be stored in
    /// the location in this name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The tag to create.
    #[prost(message, optional, tag = "2")]
    pub tag: ::core::option::Option<Tag>,
}
/// Request message for
/// \[UpdateTag][google.cloud.datacatalog.v1beta1.DataCatalog.UpdateTag\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagRequest {
    /// Required. The updated tag. The "name" field must be set.
    #[prost(message, optional, tag = "1")]
    pub tag: ::core::option::Option<Tag>,
    /// Note: Currently, this parameter can only take `"fields"` as value.
    ///
    /// Names of fields whose values to overwrite on a tag. Currently, a tag has
    /// the only modifiable field with the name `fields`.
    ///
    /// In general, if this parameter is absent or empty, all modifiable fields
    /// are overwritten. If such fields are non-required and omitted in the
    /// request body, their values are emptied.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// \[DeleteTag][google.cloud.datacatalog.v1beta1.DataCatalog.DeleteTag\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagRequest {
    /// Required. The name of the tag to delete. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}/tags/{tag_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[CreateTagTemplateField][google.cloud.datacatalog.v1beta1.DataCatalog.CreateTagTemplateField\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagTemplateFieldRequest {
    /// Required. The name of the project and the template location
    /// \[region\](<https://cloud.google.com/data-catalog/docs/concepts/regions>).
    ///
    /// Example:
    ///
    /// * projects/{project_id}/locations/us-central1/tagTemplates/{tag_template_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the tag template field to create.
    /// Field ids can contain letters (both uppercase and lowercase), numbers
    /// (0-9), underscores (_) and dashes (-). Field IDs must be at least 1
    /// character long and at most 128 characters long. Field IDs must also be
    /// unique within their template.
    #[prost(string, tag = "2")]
    pub tag_template_field_id: ::prost::alloc::string::String,
    /// Required. The tag template field to create.
    #[prost(message, optional, tag = "3")]
    pub tag_template_field: ::core::option::Option<TagTemplateField>,
}
/// Request message for
/// \[UpdateTagTemplateField][google.cloud.datacatalog.v1beta1.DataCatalog.UpdateTagTemplateField\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagTemplateFieldRequest {
    /// Required. The name of the tag template field. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}/fields/{tag_template_field_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The template to update.
    #[prost(message, optional, tag = "2")]
    pub tag_template_field: ::core::option::Option<TagTemplateField>,
    /// Optional. Names of fields whose values to overwrite on an individual field
    /// of a tag template. The following fields are modifiable:
    ///
    ///    * `display_name`
    ///    * `type.enum_type`
    ///    * `is_required`
    ///
    /// If this parameter is absent or empty, all modifiable fields
    /// are overwritten. If such fields are non-required and omitted in the request
    /// body, their values are emptied with one exception: when updating an enum
    /// type, the provided values are merged with the existing values. Therefore,
    /// enum values can only be added, existing enum values cannot be deleted or
    /// renamed.
    ///
    /// Additionally, updating a template field from optional to required is
    /// *not* allowed.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// \[RenameTagTemplateField][google.cloud.datacatalog.v1beta1.DataCatalog.RenameTagTemplateField\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTagTemplateFieldRequest {
    /// Required. The name of the tag template. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}/fields/{tag_template_field_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new ID of this tag template field. For example,
    /// `my_new_field`.
    #[prost(string, tag = "2")]
    pub new_tag_template_field_id: ::prost::alloc::string::String,
}
/// Request message for
/// \[RenameTagTemplateFieldEnumValue][google.cloud.datacatalog.v1.DataCatalog.RenameTagTemplateFieldEnumValue\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTagTemplateFieldEnumValueRequest {
    /// Required. The name of the enum field value. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}/fields/{tag_template_field_id}/enumValues/{enum_value_display_name}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new display name of the enum value. For example,
    /// `my_new_enum_value`.
    #[prost(string, tag = "2")]
    pub new_enum_value_display_name: ::prost::alloc::string::String,
}
/// Request message for
/// \[DeleteTagTemplateField][google.cloud.datacatalog.v1beta1.DataCatalog.DeleteTagTemplateField\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagTemplateFieldRequest {
    /// Required. The name of the tag template field to delete. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}/fields/{tag_template_field_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Currently, this field must always be set to `true`.
    /// This confirms the deletion of this field from any tags using this field.
    /// `force = false` will be supported in the future.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for
/// \[ListTags][google.cloud.datacatalog.v1beta1.DataCatalog.ListTags\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsRequest {
    /// Required. The name of the Data Catalog resource to list the tags of. The
    /// resource could be an \[Entry][google.cloud.datacatalog.v1beta1.Entry\] or an
    /// \[EntryGroup][google.cloud.datacatalog.v1beta1.EntryGroup\].
    ///
    /// Examples:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of tags to return. Default is 10. Max limit is 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token that specifies which page is requested. If empty, the first page is
    /// returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[ListTags][google.cloud.datacatalog.v1beta1.DataCatalog.ListTags\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsResponse {
    /// \[Tag][google.cloud.datacatalog.v1beta1.Tag\] details.
    #[prost(message, repeated, tag = "1")]
    pub tags: ::prost::alloc::vec::Vec<Tag>,
    /// Token to retrieve the next page of results. It is set to empty if no items
    /// remain in results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// \[ListEntries][google.cloud.datacatalog.v1beta1.DataCatalog.ListEntries\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntriesRequest {
    /// Required. The name of the entry group that contains the entries, which can
    /// be provided in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. Default is 10. Max limit is 1000.
    /// Throws an invalid argument for `page_size > 1000`.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token that specifies which page is requested. If empty, the first page is
    /// returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The fields to return for each Entry. If not set or empty, all
    /// fields are returned.
    /// For example, setting read_mask to contain only one path "name" will cause
    /// ListEntries to return a list of Entries with only "name" field.
    #[prost(message, optional, tag = "4")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for
/// \[ListEntries][google.cloud.datacatalog.v1beta1.DataCatalog.ListEntries\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntriesResponse {
    /// Entry details.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<Entry>,
    /// Token to retrieve the next page of results. It is set to empty if no items
    /// remain in results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Entry resources in Data Catalog can be of different types e.g. a BigQuery
/// Table entry is of type `TABLE`. This enum describes all the possible types
/// Data Catalog contains.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EntryType {
    /// Default unknown type.
    Unspecified = 0,
    /// Output only. The type of entry that has a GoogleSQL schema, including
    /// logical views.
    Table = 2,
    /// Output only. The type of models.
    /// <https://cloud.google.com/bigquery-ml/docs/bigqueryml-intro>
    Model = 5,
    /// Output only. An entry type which is used for streaming entries. Example:
    /// Pub/Sub topic.
    DataStream = 3,
    /// An entry type which is a set of files or objects. Example:
    /// Cloud Storage fileset.
    Fileset = 4,
}
impl EntryType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EntryType::Unspecified => "ENTRY_TYPE_UNSPECIFIED",
            EntryType::Table => "TABLE",
            EntryType::Model => "MODEL",
            EntryType::DataStream => "DATA_STREAM",
            EntryType::Fileset => "FILESET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENTRY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TABLE" => Some(Self::Table),
            "MODEL" => Some(Self::Model),
            "DATA_STREAM" => Some(Self::DataStream),
            "FILESET" => Some(Self::Fileset),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod data_catalog_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Data Catalog API service allows clients to discover, understand, and manage
    /// their data.
    #[derive(Debug, Clone)]
    pub struct DataCatalogClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataCatalogClient<T>
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
        ) -> DataCatalogClient<InterceptedService<T, F>>
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
            DataCatalogClient::new(InterceptedService::new(inner, interceptor))
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
        /// Searches Data Catalog for multiple resources like entries, tags that
        /// match a query.
        ///
        /// This is a custom method
        /// (https://cloud.google.com/apis/design/custom_methods) and does not return
        /// the complete resource, only the resource identifier and high level
        /// fields. Clients can subsequently call `Get` methods.
        ///
        /// Note that Data Catalog search queries do not guarantee full recall. Query
        /// results that match your query may not be returned, even in subsequent
        /// result pages. Also note that results returned (and not returned) can vary
        /// across repeated search queries.
        ///
        /// See [Data Catalog Search
        /// Syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference)
        /// for more information.
        pub async fn search_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchCatalogRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchCatalogResponse>,
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/SearchCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "SearchCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// A maximum of 10,000 entry groups may be created per organization across all
        /// locations.
        ///
        /// Users should enable the Data Catalog API in the project identified by
        /// the `parent` parameter (see [Data Catalog Resource Project]
        /// (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for
        /// more information).
        pub async fn create_entry_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntryGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::EntryGroup>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.DataCatalog/CreateEntryGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "CreateEntryGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an EntryGroup. The user should enable the Data Catalog API in the
        /// project identified by the `entry_group.name` parameter (see [Data Catalog
        /// Resource Project]
        /// (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for
        /// more information).
        pub async fn update_entry_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEntryGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::EntryGroup>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.DataCatalog/UpdateEntryGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "UpdateEntryGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets an EntryGroup.
        pub async fn get_entry_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntryGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::EntryGroup>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.DataCatalog/GetEntryGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "GetEntryGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an EntryGroup. Only entry groups that do not contain entries can be
        /// deleted. Users should enable the Data Catalog API in the project
        /// identified by the `name` parameter (see [Data Catalog Resource Project]
        /// (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for
        /// more information).
        pub async fn delete_entry_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntryGroupRequest>,
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/DeleteEntryGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "DeleteEntryGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists entry groups.
        pub async fn list_entry_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntryGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEntryGroupsResponse>,
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/ListEntryGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "ListEntryGroups",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an entry. Only entries of 'FILESET' type or user-specified type can
        /// be created.
        ///
        /// Users should enable the Data Catalog API in the project identified by
        /// the `parent` parameter (see [Data Catalog Resource Project]
        /// (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for
        /// more information).
        ///
        /// A maximum of 100,000 entries may be created per entry group.
        pub async fn create_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntryRequest>,
        ) -> std::result::Result<tonic::Response<super::Entry>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.DataCatalog/CreateEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "CreateEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing entry.
        /// Users should enable the Data Catalog API in the project identified by
        /// the `entry.name` parameter (see [Data Catalog Resource Project]
        /// (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for
        /// more information).
        pub async fn update_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEntryRequest>,
        ) -> std::result::Result<tonic::Response<super::Entry>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.DataCatalog/UpdateEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "UpdateEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an existing entry. Only entries created through
        /// [CreateEntry][google.cloud.datacatalog.v1beta1.DataCatalog.CreateEntry]
        /// method can be deleted.
        /// Users should enable the Data Catalog API in the project identified by
        /// the `name` parameter (see [Data Catalog Resource Project]
        /// (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for
        /// more information).
        pub async fn delete_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntryRequest>,
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/DeleteEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "DeleteEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets an entry.
        pub async fn get_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntryRequest>,
        ) -> std::result::Result<tonic::Response<super::Entry>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.DataCatalog/GetEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "GetEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get an entry by target resource name. This method allows clients to use
        /// the resource name from the source Google Cloud Platform service to get the
        /// Data Catalog Entry.
        pub async fn lookup_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupEntryRequest>,
        ) -> std::result::Result<tonic::Response<super::Entry>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.DataCatalog/LookupEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "LookupEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists entries.
        pub async fn list_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEntriesResponse>,
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/ListEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "ListEntries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a tag template. The user should enable the Data Catalog API in
        /// the project identified by the `parent` parameter (see [Data Catalog
        /// Resource
        /// Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project)
        /// for more information).
        pub async fn create_tag_template(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagTemplateRequest>,
        ) -> std::result::Result<tonic::Response<super::TagTemplate>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.DataCatalog/CreateTagTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "CreateTagTemplate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a tag template.
        pub async fn get_tag_template(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTagTemplateRequest>,
        ) -> std::result::Result<tonic::Response<super::TagTemplate>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.DataCatalog/GetTagTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "GetTagTemplate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a tag template. This method cannot be used to update the fields of
        /// a template. The tag template fields are represented as separate resources
        /// and should be updated using their own create/update/delete methods.
        /// Users should enable the Data Catalog API in the project identified by
        /// the `tag_template.name` parameter (see [Data Catalog Resource Project]
        /// (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for
        /// more information).
        pub async fn update_tag_template(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTagTemplateRequest>,
        ) -> std::result::Result<tonic::Response<super::TagTemplate>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.DataCatalog/UpdateTagTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "UpdateTagTemplate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a tag template and all tags using the template.
        /// Users should enable the Data Catalog API in the project identified by
        /// the `name` parameter (see [Data Catalog Resource Project]
        /// (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for
        /// more information).
        pub async fn delete_tag_template(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTagTemplateRequest>,
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/DeleteTagTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "DeleteTagTemplate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a field in a tag template. The user should enable the Data Catalog
        /// API in the project identified by the `parent` parameter (see
        /// [Data Catalog Resource
        /// Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project)
        /// for more information).
        pub async fn create_tag_template_field(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagTemplateFieldRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TagTemplateField>,
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/CreateTagTemplateField",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "CreateTagTemplateField",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a field in a tag template. This method cannot be used to update the
        /// field type. Users should enable the Data Catalog API in the project
        /// identified by the `name` parameter (see [Data Catalog Resource Project]
        /// (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for
        /// more information).
        pub async fn update_tag_template_field(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTagTemplateFieldRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TagTemplateField>,
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/UpdateTagTemplateField",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "UpdateTagTemplateField",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Renames a field in a tag template. The user should enable the Data Catalog
        /// API in the project identified by the `name` parameter (see [Data Catalog
        /// Resource
        /// Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project)
        /// for more information).
        pub async fn rename_tag_template_field(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameTagTemplateFieldRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TagTemplateField>,
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/RenameTagTemplateField",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "RenameTagTemplateField",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Renames an enum value in a tag template. The enum values have to be unique
        /// within one enum field. Thus, an enum value cannot be renamed with a name
        /// used in any other enum value within the same enum field.
        pub async fn rename_tag_template_field_enum_value(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RenameTagTemplateFieldEnumValueRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::TagTemplateField>,
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/RenameTagTemplateFieldEnumValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "RenameTagTemplateFieldEnumValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a field in a tag template and all uses of that field.
        /// Users should enable the Data Catalog API in the project identified by
        /// the `name` parameter (see [Data Catalog Resource Project]
        /// (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for
        /// more information).
        pub async fn delete_tag_template_field(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTagTemplateFieldRequest>,
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/DeleteTagTemplateField",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "DeleteTagTemplateField",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a tag on an [Entry][google.cloud.datacatalog.v1beta1.Entry].
        /// Note: The project identified by the `parent` parameter for the
        /// [tag](https://cloud.google.com/data-catalog/docs/reference/rest/v1beta1/projects.locations.entryGroups.entries.tags/create#path-parameters)
        /// and the
        /// [tag
        /// template](https://cloud.google.com/data-catalog/docs/reference/rest/v1beta1/projects.locations.tagTemplates/create#path-parameters)
        /// used to create the tag must be from the same organization.
        pub async fn create_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagRequest>,
        ) -> std::result::Result<tonic::Response<super::Tag>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.DataCatalog/CreateTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "CreateTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing tag.
        pub async fn update_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTagRequest>,
        ) -> std::result::Result<tonic::Response<super::Tag>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.DataCatalog/UpdateTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "UpdateTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a tag.
        pub async fn delete_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTagRequest>,
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/DeleteTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "DeleteTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists tags assigned to an [Entry][google.cloud.datacatalog.v1beta1.Entry].
        /// The [columns][google.cloud.datacatalog.v1beta1.Tag.column] in the response
        /// are lowercased.
        pub async fn list_tags(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTagsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTagsResponse>,
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/ListTags",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "ListTags",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the access control policy for a resource. Replaces any existing
        /// policy.
        /// Supported resources are:
        ///   - Tag templates.
        ///   - Entries.
        ///   - Entry groups.
        /// Note, this method cannot be used to manage policies for BigQuery, Pub/Sub
        /// and any external Google Cloud Platform resources synced to Data Catalog.
        ///
        /// Callers must have following Google IAM permission
        ///   - `datacatalog.tagTemplates.setIamPolicy` to set policies on tag
        ///     templates.
        ///   - `datacatalog.entries.setIamPolicy` to set policies on entries.
        ///   - `datacatalog.entryGroups.setIamPolicy` to set policies on entry groups.
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the access control policy for a resource. A `NOT_FOUND` error
        /// is returned if the resource does not exist. An empty policy is returned
        /// if the resource exists but does not have a policy set on it.
        ///
        /// Supported resources are:
        ///   - Tag templates.
        ///   - Entries.
        ///   - Entry groups.
        /// Note, this method cannot be used to manage policies for BigQuery, Pub/Sub
        /// and any external Google Cloud Platform resources synced to Data Catalog.
        ///
        /// Callers must have following Google IAM permission
        ///   - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag
        ///     templates.
        ///   - `datacatalog.entries.getIamPolicy` to get policies on entries.
        ///   - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups.
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the caller's permissions on a resource.
        /// If the resource does not exist, an empty set of permissions is returned
        /// (We don't return a `NOT_FOUND` error).
        ///
        /// Supported resources are:
        ///   - Tag templates.
        ///   - Entries.
        ///   - Entry groups.
        /// Note, this method cannot be used to manage policies for BigQuery, Pub/Sub
        /// and any external Google Cloud Platform resources synced to Data Catalog.
        ///
        /// A caller is not required to have Google IAM permission to make this
        /// request.
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
                "/google.cloud.datacatalog.v1beta1.DataCatalog/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.DataCatalog",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// A taxonomy is a collection of policy tags that classify data along a common
/// axis. For instance a data *sensitivity* taxonomy could contain policy tags
/// denoting PII such as age, zipcode, and SSN. A data *origin* taxonomy could
/// contain policy tags to distinguish user data, employee data, partner data,
/// public data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Taxonomy {
    /// Output only. Resource name of this taxonomy, whose format is:
    /// "projects/{project_number}/locations/{location_id}/taxonomies/{id}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. User defined name of this taxonomy. It must: contain only unicode
    /// letters, numbers, underscores, dashes and spaces; not start or end with
    /// spaces; and be at most 200 bytes long when encoded in UTF-8.
    ///
    /// The taxonomy display name must be unique within an organization.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Description of this taxonomy. It must: contain only unicode
    /// characters, tabs, newlines, carriage returns and page breaks; and be at
    /// most 2000 bytes long when encoded in UTF-8. If not set, defaults to an
    /// empty description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Number of policy tags contained in this taxonomy.
    #[prost(int32, tag = "4")]
    pub policy_tag_count: i32,
    /// Output only. Timestamps about this taxonomy. Only create_time and
    /// update_time are used.
    #[prost(message, optional, tag = "5")]
    pub taxonomy_timestamps: ::core::option::Option<SystemTimestamps>,
    /// Optional. A list of policy types that are activated for this taxonomy. If
    /// not set, defaults to an empty list.
    #[prost(enumeration = "taxonomy::PolicyType", repeated, packed = "false", tag = "6")]
    pub activated_policy_types: ::prost::alloc::vec::Vec<i32>,
    /// Output only. Identity of the service which owns the Taxonomy. This field is
    /// only populated when the taxonomy is created by a Google Cloud service.
    /// Currently only 'DATAPLEX' is supported.
    #[prost(message, optional, tag = "7")]
    pub service: ::core::option::Option<taxonomy::Service>,
}
/// Nested message and enum types in `Taxonomy`.
pub mod taxonomy {
    /// The source system of the Taxonomy.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Service {
        /// The Google Cloud service name.
        #[prost(enumeration = "super::ManagingSystem", tag = "1")]
        pub name: i32,
        /// The service agent for the service.
        #[prost(string, tag = "2")]
        pub identity: ::prost::alloc::string::String,
    }
    /// Defines policy types where policy tag can be used for.
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
    pub enum PolicyType {
        /// Unspecified policy type.
        Unspecified = 0,
        /// Fine grained access control policy, which enables access control on
        /// tagged resources.
        FineGrainedAccessControl = 1,
    }
    impl PolicyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PolicyType::Unspecified => "POLICY_TYPE_UNSPECIFIED",
                PolicyType::FineGrainedAccessControl => "FINE_GRAINED_ACCESS_CONTROL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "POLICY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "FINE_GRAINED_ACCESS_CONTROL" => Some(Self::FineGrainedAccessControl),
                _ => None,
            }
        }
    }
}
/// Denotes one policy tag in a taxonomy (e.g. ssn). Policy Tags can be defined
/// in a hierarchy. For example, consider the following hierarchy:
/// Geolocation -&gt; (LatLong, City, ZipCode). PolicyTag "Geolocation"
/// contains three child policy tags: "LatLong", "City", and "ZipCode".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTag {
    /// Output only. Resource name of this policy tag, whose format is:
    /// "projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{id}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. User defined name of this policy tag. It must: be unique within
    /// the parent taxonomy; contain only unicode letters, numbers, underscores,
    /// dashes and spaces; not start or end with spaces; and be at most 200 bytes
    /// long when encoded in UTF-8.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of this policy tag. It must: contain only unicode characters,
    /// tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes
    /// long when encoded in UTF-8. If not set, defaults to an empty description.
    /// If not set, defaults to an empty description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Resource name of this policy tag's parent policy tag (e.g. for the
    /// "LatLong" policy tag in the example above, this field contains the
    /// resource name of the "Geolocation" policy tag). If empty, it means this
    /// policy tag is a top level policy tag (e.g. this field is empty for the
    /// "Geolocation" policy tag in the example above). If not set, defaults to an
    /// empty string.
    #[prost(string, tag = "4")]
    pub parent_policy_tag: ::prost::alloc::string::String,
    /// Output only. Resource names of child policy tags of this policy tag.
    #[prost(string, repeated, tag = "5")]
    pub child_policy_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// \[CreateTaxonomy][google.cloud.datacatalog.v1beta1.PolicyTagManager.CreateTaxonomy\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaxonomyRequest {
    /// Required. Resource name of the project that the taxonomy will belong to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The taxonomy to be created.
    #[prost(message, optional, tag = "2")]
    pub taxonomy: ::core::option::Option<Taxonomy>,
}
/// Request message for
/// \[DeleteTaxonomy][google.cloud.datacatalog.v1beta1.PolicyTagManager.DeleteTaxonomy\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTaxonomyRequest {
    /// Required. Resource name of the taxonomy to be deleted. All policy tags in
    /// this taxonomy will also be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[UpdateTaxonomy][google.cloud.datacatalog.v1beta1.PolicyTagManager.UpdateTaxonomy\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaxonomyRequest {
    /// The taxonomy to update. Only description, display_name, and activated
    /// policy types can be updated.
    #[prost(message, optional, tag = "1")]
    pub taxonomy: ::core::option::Option<Taxonomy>,
    /// The update mask applies to the resource. For the `FieldMask` definition,
    /// see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    /// If not set, defaults to all of the fields that are allowed to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// \[ListTaxonomies][google.cloud.datacatalog.v1beta1.PolicyTagManager.ListTaxonomies\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTaxonomiesRequest {
    /// Required. Resource name of the project to list the taxonomies of.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. Must be a value between 1 and 1000.
    /// If not set, defaults to 50.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request, if any. If
    /// not set, defaults to an empty string.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Supported field for filter is 'service' and value is 'dataplex'.
    /// Eg: service=dataplex.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for
/// \[ListTaxonomies][google.cloud.datacatalog.v1beta1.PolicyTagManager.ListTaxonomies\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTaxonomiesResponse {
    /// Taxonomies that the project contains.
    #[prost(message, repeated, tag = "1")]
    pub taxonomies: ::prost::alloc::vec::Vec<Taxonomy>,
    /// Token used to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// \[GetTaxonomy][google.cloud.datacatalog.v1beta1.PolicyTagManager.GetTaxonomy\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaxonomyRequest {
    /// Required. Resource name of the requested taxonomy.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[CreatePolicyTag][google.cloud.datacatalog.v1beta1.PolicyTagManager.CreatePolicyTag\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicyTagRequest {
    /// Required. Resource name of the taxonomy that the policy tag will belong to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The policy tag to be created.
    #[prost(message, optional, tag = "2")]
    pub policy_tag: ::core::option::Option<PolicyTag>,
}
/// Request message for
/// \[DeletePolicyTag][google.cloud.datacatalog.v1beta1.PolicyTagManager.DeletePolicyTag\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePolicyTagRequest {
    /// Required. Resource name of the policy tag to be deleted. All of its
    /// descendant policy tags will also be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[UpdatePolicyTag][google.cloud.datacatalog.v1beta1.PolicyTagManager.UpdatePolicyTag\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePolicyTagRequest {
    /// The policy tag to update. Only the description, display_name, and
    /// parent_policy_tag fields can be updated.
    #[prost(message, optional, tag = "1")]
    pub policy_tag: ::core::option::Option<PolicyTag>,
    /// The update mask applies to the resource. Only display_name, description and
    /// parent_policy_tag can be updated and thus can be listed in the mask. If
    /// update_mask is not provided, all allowed fields (i.e. display_name,
    /// description and parent) will be updated. For more information including the
    /// `FieldMask` definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    /// If not set, defaults to all of the fields that are allowed to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// \[ListPolicyTags][google.cloud.datacatalog.v1beta1.PolicyTagManager.ListPolicyTags\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyTagsRequest {
    /// Required. Resource name of the taxonomy to list the policy tags of.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. Must be a value between 1 and 1000.
    /// If not set, defaults to 50.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any. If
    /// not set, defaults to an empty string.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[ListPolicyTags][google.cloud.datacatalog.v1beta1.PolicyTagManager.ListPolicyTags\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyTagsResponse {
    /// The policy tags that are in the requested taxonomy.
    #[prost(message, repeated, tag = "1")]
    pub policy_tags: ::prost::alloc::vec::Vec<PolicyTag>,
    /// Token used to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// \[GetPolicyTag][google.cloud.datacatalog.v1beta1.PolicyTagManager.GetPolicyTag\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyTagRequest {
    /// Required. Resource name of the requested policy tag.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod policy_tag_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The policy tag manager API service allows clients to manage their taxonomies
    /// and policy tags.
    #[derive(Debug, Clone)]
    pub struct PolicyTagManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PolicyTagManagerClient<T>
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
        ) -> PolicyTagManagerClient<InterceptedService<T, F>>
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
            PolicyTagManagerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a taxonomy in the specified project.
        pub async fn create_taxonomy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTaxonomyRequest>,
        ) -> std::result::Result<tonic::Response<super::Taxonomy>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.PolicyTagManager/CreateTaxonomy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManager",
                        "CreateTaxonomy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a taxonomy. This operation will also delete all
        /// policy tags in this taxonomy along with their associated policies.
        pub async fn delete_taxonomy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTaxonomyRequest>,
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
                "/google.cloud.datacatalog.v1beta1.PolicyTagManager/DeleteTaxonomy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManager",
                        "DeleteTaxonomy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a taxonomy.
        pub async fn update_taxonomy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTaxonomyRequest>,
        ) -> std::result::Result<tonic::Response<super::Taxonomy>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.PolicyTagManager/UpdateTaxonomy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManager",
                        "UpdateTaxonomy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all taxonomies in a project in a particular location that the caller
        /// has permission to view.
        pub async fn list_taxonomies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTaxonomiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTaxonomiesResponse>,
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
                "/google.cloud.datacatalog.v1beta1.PolicyTagManager/ListTaxonomies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManager",
                        "ListTaxonomies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a taxonomy.
        pub async fn get_taxonomy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTaxonomyRequest>,
        ) -> std::result::Result<tonic::Response<super::Taxonomy>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.PolicyTagManager/GetTaxonomy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManager",
                        "GetTaxonomy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a policy tag in the specified taxonomy.
        pub async fn create_policy_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePolicyTagRequest>,
        ) -> std::result::Result<tonic::Response<super::PolicyTag>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.PolicyTagManager/CreatePolicyTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManager",
                        "CreatePolicyTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a policy tag. Also deletes all of its descendant policy tags.
        pub async fn delete_policy_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePolicyTagRequest>,
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
                "/google.cloud.datacatalog.v1beta1.PolicyTagManager/DeletePolicyTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManager",
                        "DeletePolicyTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a policy tag.
        pub async fn update_policy_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePolicyTagRequest>,
        ) -> std::result::Result<tonic::Response<super::PolicyTag>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.PolicyTagManager/UpdatePolicyTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManager",
                        "UpdatePolicyTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all policy tags in a taxonomy.
        pub async fn list_policy_tags(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPolicyTagsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPolicyTagsResponse>,
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
                "/google.cloud.datacatalog.v1beta1.PolicyTagManager/ListPolicyTags",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManager",
                        "ListPolicyTags",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a policy tag.
        pub async fn get_policy_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPolicyTagRequest>,
        ) -> std::result::Result<tonic::Response<super::PolicyTag>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1beta1.PolicyTagManager/GetPolicyTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManager",
                        "GetPolicyTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the IAM policy for a taxonomy or a policy tag.
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
                "/google.cloud.datacatalog.v1beta1.PolicyTagManager/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManager",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the IAM policy for a taxonomy or a policy tag.
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
                "/google.cloud.datacatalog.v1beta1.PolicyTagManager/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManager",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the permissions that a caller has on the specified taxonomy or
        /// policy tag.
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
                "/google.cloud.datacatalog.v1beta1.PolicyTagManager/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManager",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Message capturing a taxonomy and its policy tag hierarchy as a nested proto.
/// Used for taxonomy import/export and mutation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedTaxonomy {
    /// Required. Display name of the taxonomy. Max 200 bytes when encoded in
    /// UTF-8.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of the serialized taxonomy. The length of the
    /// description is limited to 2000 bytes when encoded in UTF-8. If not set,
    /// defaults to an empty description.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Top level policy tags associated with the taxonomy if any.
    #[prost(message, repeated, tag = "3")]
    pub policy_tags: ::prost::alloc::vec::Vec<SerializedPolicyTag>,
    /// A list of policy types that are activated for a taxonomy.
    #[prost(enumeration = "taxonomy::PolicyType", repeated, tag = "4")]
    pub activated_policy_types: ::prost::alloc::vec::Vec<i32>,
}
/// Message representing one policy tag when exported as a nested proto.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedPolicyTag {
    /// Resource name of the policy tag.
    ///
    /// This field will be ignored when calling ImportTaxonomies.
    #[prost(string, tag = "1")]
    pub policy_tag: ::prost::alloc::string::String,
    /// Required. Display name of the policy tag. Max 200 bytes when encoded in
    /// UTF-8.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of the serialized policy tag. The length of the
    /// description is limited to 2000 bytes when encoded in UTF-8. If not set,
    /// defaults to an empty description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Children of the policy tag if any.
    #[prost(message, repeated, tag = "4")]
    pub child_policy_tags: ::prost::alloc::vec::Vec<SerializedPolicyTag>,
}
/// Request message for
/// \[ImportTaxonomies][google.cloud.datacatalog.v1beta1.PolicyTagManagerSerialization.ImportTaxonomies\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportTaxonomiesRequest {
    /// Required. Resource name of project that the imported taxonomies will belong
    /// to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Source taxonomies to be imported.
    #[prost(oneof = "import_taxonomies_request::Source", tags = "2")]
    pub source: ::core::option::Option<import_taxonomies_request::Source>,
}
/// Nested message and enum types in `ImportTaxonomiesRequest`.
pub mod import_taxonomies_request {
    /// Source taxonomies to be imported.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Inline source used for taxonomies to be imported.
        #[prost(message, tag = "2")]
        InlineSource(super::InlineSource),
    }
}
/// Inline source used for taxonomies import.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlineSource {
    /// Required. Taxonomies to be imported.
    #[prost(message, repeated, tag = "1")]
    pub taxonomies: ::prost::alloc::vec::Vec<SerializedTaxonomy>,
}
/// Response message for
/// \[ImportTaxonomies][google.cloud.datacatalog.v1beta1.PolicyTagManagerSerialization.ImportTaxonomies\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportTaxonomiesResponse {
    /// Taxonomies that were imported.
    #[prost(message, repeated, tag = "1")]
    pub taxonomies: ::prost::alloc::vec::Vec<Taxonomy>,
}
/// Request message for
/// \[ExportTaxonomies][google.cloud.datacatalog.v1beta1.PolicyTagManagerSerialization.ExportTaxonomies\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTaxonomiesRequest {
    /// Required. Resource name of the project that taxonomies to be exported
    /// will share.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Resource names of the taxonomies to be exported.
    #[prost(string, repeated, tag = "2")]
    pub taxonomies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Taxonomies export destination.
    #[prost(oneof = "export_taxonomies_request::Destination", tags = "3")]
    pub destination: ::core::option::Option<export_taxonomies_request::Destination>,
}
/// Nested message and enum types in `ExportTaxonomiesRequest`.
pub mod export_taxonomies_request {
    /// Required. Taxonomies export destination.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Export taxonomies as serialized taxonomies.
        #[prost(bool, tag = "3")]
        SerializedTaxonomies(bool),
    }
}
/// Response message for
/// \[ExportTaxonomies][google.cloud.datacatalog.v1beta1.PolicyTagManagerSerialization.ExportTaxonomies\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTaxonomiesResponse {
    /// List of taxonomies and policy tags in a tree structure.
    #[prost(message, repeated, tag = "1")]
    pub taxonomies: ::prost::alloc::vec::Vec<SerializedTaxonomy>,
}
/// Generated client implementations.
pub mod policy_tag_manager_serialization_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Policy tag manager serialization API service allows clients to manipulate
    /// their taxonomies and policy tags data with serialized format.
    #[derive(Debug, Clone)]
    pub struct PolicyTagManagerSerializationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PolicyTagManagerSerializationClient<T>
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
        ) -> PolicyTagManagerSerializationClient<InterceptedService<T, F>>
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
            PolicyTagManagerSerializationClient::new(
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
        /// Imports all taxonomies and their policy tags to a project as new
        /// taxonomies.
        ///
        /// This method provides a bulk taxonomy / policy tag creation using nested
        /// proto structure.
        pub async fn import_taxonomies(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportTaxonomiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImportTaxonomiesResponse>,
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
                "/google.cloud.datacatalog.v1beta1.PolicyTagManagerSerialization/ImportTaxonomies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManagerSerialization",
                        "ImportTaxonomies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Exports all taxonomies and their policy tags in a project.
        ///
        /// This method generates SerializedTaxonomy protos with nested policy tags
        /// that can be used as an input for future ImportTaxonomies calls.
        pub async fn export_taxonomies(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportTaxonomiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportTaxonomiesResponse>,
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
                "/google.cloud.datacatalog.v1beta1.PolicyTagManagerSerialization/ExportTaxonomies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1beta1.PolicyTagManagerSerialization",
                        "ExportTaxonomies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
