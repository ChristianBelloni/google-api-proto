/// Timestamps associated with this resource in a particular system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemTimestamps {
    /// Creation timestamp of the resource within the given system.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Timestamp of the last modification of the resource or its metadata within
    /// a given system.
    ///
    /// Note: Depending on the source system, not every modification updates this
    /// timestamp.
    /// For example, BigQuery timestamps every metadata modification but not data
    /// or permission changes.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Expiration timestamp of the resource within the given system.
    ///
    /// Currently only applicable to BigQuery resources.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Describes a Cloud Storage fileset entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsFilesetSpec {
    /// Required. Patterns to identify a set of files in Google Cloud Storage.
    ///
    /// For more information, see \[Wildcard Names\]
    /// (<https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames>).
    ///
    /// Note: Currently, bucket wildcards are not supported.
    ///
    /// Examples of valid `file_patterns`:
    ///
    ///   * `gs://bucket_name/dir/*`: matches all files in `bucket_name/dir`
    ///                               directory
    ///   * `gs://bucket_name/dir/**`: matches all files in `bucket_name/dir`
    ///                                and all subdirectories
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
    ///                               the `a/*/b` pattern, such as `a/c/b`, `a/d/b`
    ///   * `gs://another_bucket/a.txt`: matches `gs://another_bucket/a.txt`
    ///
    /// You can combine wildcards to match complex sets of files, for example:
    ///
    /// `gs://bucket_name/\[a-m\]??.j*g`
    #[prost(string, repeated, tag = "1")]
    pub file_patterns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Sample files contained in this fileset, not all files
    /// contained in this fileset are represented here.
    #[prost(message, repeated, tag = "2")]
    pub sample_gcs_file_specs: ::prost::alloc::vec::Vec<GcsFileSpec>,
}
/// Specification of a single file in Cloud Storage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsFileSpec {
    /// Required. Full file path. Example: `gs://bucket_name/a/b.txt`.
    #[prost(string, tag = "1")]
    pub file_path: ::prost::alloc::string::String,
    /// Output only. Creation, modification, and expiration timestamps of a Cloud
    /// Storage file.
    #[prost(message, optional, tag = "2")]
    pub gcs_timestamps: ::core::option::Option<SystemTimestamps>,
    /// Output only. File size in bytes.
    #[prost(int64, tag = "4")]
    pub size_bytes: i64,
}
/// Specification for the BigQuery connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryConnectionSpec {
    /// The type of the BigQuery connection.
    #[prost(enumeration = "big_query_connection_spec::ConnectionType", tag = "1")]
    pub connection_type: i32,
    /// True if there are credentials attached to the BigQuery connection; false
    /// otherwise.
    #[prost(bool, tag = "3")]
    pub has_credential: bool,
    #[prost(oneof = "big_query_connection_spec::ConnectionSpec", tags = "2")]
    pub connection_spec: ::core::option::Option<
        big_query_connection_spec::ConnectionSpec,
    >,
}
/// Nested message and enum types in `BigQueryConnectionSpec`.
pub mod big_query_connection_spec {
    /// The type of the BigQuery connection.
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
    pub enum ConnectionType {
        /// Unspecified type.
        Unspecified = 0,
        /// Cloud SQL connection.
        CloudSql = 1,
    }
    impl ConnectionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConnectionType::Unspecified => "CONNECTION_TYPE_UNSPECIFIED",
                ConnectionType::CloudSql => "CLOUD_SQL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONNECTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOUD_SQL" => Some(Self::CloudSql),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConnectionSpec {
        /// Specification for the BigQuery connection to a Cloud SQL instance.
        #[prost(message, tag = "2")]
        CloudSql(super::CloudSqlBigQueryConnectionSpec),
    }
}
/// Specification for the BigQuery connection to a Cloud SQL instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlBigQueryConnectionSpec {
    /// Cloud SQL instance ID in the format of `project:location:instance`.
    #[prost(string, tag = "1")]
    pub instance_id: ::prost::alloc::string::String,
    /// Database name.
    #[prost(string, tag = "2")]
    pub database: ::prost::alloc::string::String,
    /// Type of the Cloud SQL database.
    #[prost(
        enumeration = "cloud_sql_big_query_connection_spec::DatabaseType",
        tag = "3"
    )]
    pub r#type: i32,
}
/// Nested message and enum types in `CloudSqlBigQueryConnectionSpec`.
pub mod cloud_sql_big_query_connection_spec {
    /// Supported Cloud SQL database types.
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
    pub enum DatabaseType {
        /// Unspecified database type.
        Unspecified = 0,
        /// Cloud SQL for PostgreSQL.
        Postgres = 1,
        /// Cloud SQL for MySQL.
        Mysql = 2,
    }
    impl DatabaseType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DatabaseType::Unspecified => "DATABASE_TYPE_UNSPECIFIED",
                DatabaseType::Postgres => "POSTGRES",
                DatabaseType::Mysql => "MYSQL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATABASE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "POSTGRES" => Some(Self::Postgres),
                "MYSQL" => Some(Self::Mysql),
                _ => None,
            }
        }
    }
}
/// Fields specific for BigQuery routines.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryRoutineSpec {
    /// Paths of the imported libraries.
    #[prost(string, repeated, tag = "1")]
    pub imported_libraries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Entry metadata relevant only to the user and private to them.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonalDetails {
    /// True if the entry is starred by the user; false otherwise.
    #[prost(bool, tag = "1")]
    pub starred: bool,
    /// Set if the entry is starred; unset otherwise.
    #[prost(message, optional, tag = "2")]
    pub star_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// This enum lists all the systems that Data Catalog integrates with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IntegratedSystem {
    /// Default unknown system.
    Unspecified = 0,
    /// BigQuery.
    Bigquery = 1,
    /// Cloud Pub/Sub.
    CloudPubsub = 2,
    /// Dataproc Metastore.
    DataprocMetastore = 3,
    /// Dataplex.
    Dataplex = 4,
    /// Cloud Spanner
    CloudSpanner = 6,
    /// Cloud Bigtable
    CloudBigtable = 7,
    /// Cloud Sql
    CloudSql = 8,
    /// Looker
    Looker = 9,
    /// Vertex AI
    VertexAi = 10,
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
            IntegratedSystem::DataprocMetastore => "DATAPROC_METASTORE",
            IntegratedSystem::Dataplex => "DATAPLEX",
            IntegratedSystem::CloudSpanner => "CLOUD_SPANNER",
            IntegratedSystem::CloudBigtable => "CLOUD_BIGTABLE",
            IntegratedSystem::CloudSql => "CLOUD_SQL",
            IntegratedSystem::Looker => "LOOKER",
            IntegratedSystem::VertexAi => "VERTEX_AI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INTEGRATED_SYSTEM_UNSPECIFIED" => Some(Self::Unspecified),
            "BIGQUERY" => Some(Self::Bigquery),
            "CLOUD_PUBSUB" => Some(Self::CloudPubsub),
            "DATAPROC_METASTORE" => Some(Self::DataprocMetastore),
            "DATAPLEX" => Some(Self::Dataplex),
            "CLOUD_SPANNER" => Some(Self::CloudSpanner),
            "CLOUD_BIGTABLE" => Some(Self::CloudBigtable),
            "CLOUD_SQL" => Some(Self::CloudSql),
            "LOOKER" => Some(Self::Looker),
            "VERTEX_AI" => Some(Self::VertexAi),
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
/// Physical location of an entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSource {
    /// Service that physically stores the data.
    #[prost(enumeration = "data_source::Service", tag = "1")]
    pub service: i32,
    /// Full name of a resource as defined by the service. For example:
    ///
    /// `//bigquery.googleapis.com/projects/{PROJECT_ID}/locations/{LOCATION}/datasets/{DATASET_ID}/tables/{TABLE_ID}`
    #[prost(string, tag = "2")]
    pub resource: ::prost::alloc::string::String,
    /// Output only. Data Catalog entry name, if applicable.
    #[prost(string, tag = "3")]
    pub source_entry: ::prost::alloc::string::String,
    #[prost(oneof = "data_source::Properties", tags = "4")]
    pub properties: ::core::option::Option<data_source::Properties>,
}
/// Nested message and enum types in `DataSource`.
pub mod data_source {
    /// Name of a service that stores the data.
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
    pub enum Service {
        /// Default unknown service.
        Unspecified = 0,
        /// Google Cloud Storage service.
        CloudStorage = 1,
        /// BigQuery service.
        Bigquery = 2,
    }
    impl Service {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Service::Unspecified => "SERVICE_UNSPECIFIED",
                Service::CloudStorage => "CLOUD_STORAGE",
                Service::Bigquery => "BIGQUERY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SERVICE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOUD_STORAGE" => Some(Self::CloudStorage),
                "BIGQUERY" => Some(Self::Bigquery),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Properties {
        /// Detailed properties of the underlying storage.
        #[prost(message, tag = "4")]
        StorageProperties(super::StorageProperties),
    }
}
/// Details the properties of the underlying storage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageProperties {
    /// Patterns to identify a set of files for this fileset.
    ///
    /// Examples of a valid `file_pattern`:
    ///
    ///   * `gs://bucket_name/dir/*`: matches all files in the `bucket_name/dir`
    ///                               directory
    ///   * `gs://bucket_name/dir/**`: matches all files in the `bucket_name/dir`
    ///                                and all subdirectories recursively
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
    ///                               the `a/*/b` pattern, such as `a/c/b`, `a/d/b`
    ///   * `gs://another_bucket/a.txt`: matches `gs://another_bucket/a.txt`
    #[prost(string, repeated, tag = "1")]
    pub file_pattern: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// File type in MIME format, for example, `text/plain`.
    #[prost(string, tag = "2")]
    pub file_type: ::prost::alloc::string::String,
}
/// Native schema used by a resource represented as an entry. Used by query
/// engines for deserializing and parsing source data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalSchema {
    #[prost(oneof = "physical_schema::Schema", tags = "1, 2, 3, 4, 5, 6")]
    pub schema: ::core::option::Option<physical_schema::Schema>,
}
/// Nested message and enum types in `PhysicalSchema`.
pub mod physical_schema {
    /// Schema in Avro JSON format.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AvroSchema {
        /// JSON source of the Avro schema.
        #[prost(string, tag = "1")]
        pub text: ::prost::alloc::string::String,
    }
    /// Schema in Thrift format.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThriftSchema {
        /// Thrift IDL source of the schema.
        #[prost(string, tag = "1")]
        pub text: ::prost::alloc::string::String,
    }
    /// Schema in protocol buffer format.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtobufSchema {
        /// Protocol buffer source of the schema.
        #[prost(string, tag = "1")]
        pub text: ::prost::alloc::string::String,
    }
    /// Marks a Parquet-encoded data source.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ParquetSchema {}
    /// Marks an ORC-encoded data source.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OrcSchema {}
    /// Marks a CSV-encoded data source.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CsvSchema {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schema {
        /// Schema in Avro JSON format.
        #[prost(message, tag = "1")]
        Avro(AvroSchema),
        /// Schema in Thrift format.
        #[prost(message, tag = "2")]
        Thrift(ThriftSchema),
        /// Schema in protocol buffer format.
        #[prost(message, tag = "3")]
        Protobuf(ProtobufSchema),
        /// Marks a Parquet-encoded data source.
        #[prost(message, tag = "4")]
        Parquet(ParquetSchema),
        /// Marks an ORC-encoded data source.
        #[prost(message, tag = "5")]
        Orc(OrcSchema),
        /// Marks a CSV-encoded data source.
        #[prost(message, tag = "6")]
        Csv(CsvSchema),
    }
}
/// Common Dataplex fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataplexSpec {
    /// Fully qualified resource name of an asset in Dataplex, to which the
    /// underlying data source (Cloud Storage bucket or BigQuery dataset) of the
    /// entity is attached.
    #[prost(string, tag = "1")]
    pub asset: ::prost::alloc::string::String,
    /// Format of the data.
    #[prost(message, optional, tag = "2")]
    pub data_format: ::core::option::Option<PhysicalSchema>,
    /// Compression format of the data, e.g., zip, gzip etc.
    #[prost(string, tag = "3")]
    pub compression_format: ::prost::alloc::string::String,
    /// Project ID of the underlying Cloud Storage or BigQuery data. Note that
    /// this may not be the same project as the correspondingly Dataplex lake /
    /// zone / asset.
    #[prost(string, tag = "4")]
    pub project_id: ::prost::alloc::string::String,
}
/// Entry specyfication for a Dataplex fileset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataplexFilesetSpec {
    /// Common Dataplex fields.
    #[prost(message, optional, tag = "1")]
    pub dataplex_spec: ::core::option::Option<DataplexSpec>,
}
/// Entry specification for a Dataplex table.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataplexTableSpec {
    /// List of external tables registered by Dataplex in other systems based on
    /// the same underlying data.
    ///
    /// External tables allow to query this data in those systems.
    #[prost(message, repeated, tag = "1")]
    pub external_tables: ::prost::alloc::vec::Vec<DataplexExternalTable>,
    /// Common Dataplex fields.
    #[prost(message, optional, tag = "2")]
    pub dataplex_spec: ::core::option::Option<DataplexSpec>,
    /// Indicates if the table schema is managed by the user or not.
    #[prost(bool, tag = "3")]
    pub user_managed: bool,
}
/// External table registered by Dataplex.
/// Dataplex publishes data discovered from an asset into multiple other systems
/// (BigQuery, DPMS) in form of tables. We call them "external tables". External
/// tables are also synced into the Data Catalog.
/// This message contains pointers to
/// those external tables (fully qualified name, resource name et cetera) within
/// the Data Catalog.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataplexExternalTable {
    /// Service in which the external table is registered.
    #[prost(enumeration = "IntegratedSystem", tag = "1")]
    pub system: i32,
    /// Fully qualified name (FQN) of the external table.
    #[prost(string, tag = "28")]
    pub fully_qualified_name: ::prost::alloc::string::String,
    /// Google Cloud resource name of the external table.
    #[prost(string, tag = "3")]
    pub google_cloud_resource: ::prost::alloc::string::String,
    /// Name of the Data Catalog entry representing the external table.
    #[prost(string, tag = "4")]
    pub data_catalog_entry: ::prost::alloc::string::String,
}
/// Represents a schema, for example, a BigQuery, GoogleSQL, or Avro schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// The unified GoogleSQL-like schema of columns.
    ///
    /// The overall maximum number of columns and nested columns is 10,000.
    /// The maximum nested depth is 15 levels.
    #[prost(message, repeated, tag = "2")]
    pub columns: ::prost::alloc::vec::Vec<ColumnSchema>,
}
/// A column within a schema. Columns can be nested inside
/// other columns.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnSchema {
    /// Required. Name of the column.
    ///
    /// Must be a UTF-8 string without dots (.).
    /// The maximum size is 64 bytes.
    #[prost(string, tag = "6")]
    pub column: ::prost::alloc::string::String,
    /// Required. Type of the column.
    ///
    /// Must be a UTF-8 string with the maximum size of 128 bytes.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Optional. Description of the column. Default value is an empty string.
    ///
    /// The description must be a UTF-8 string with the maximum size of 2000
    /// bytes.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Optional. A column's mode indicates whether values in this column are
    /// required, nullable, or repeated.
    ///
    /// Only `NULLABLE`, `REQUIRED`, and `REPEATED` values are supported.
    /// Default mode is `NULLABLE`.
    #[prost(string, tag = "3")]
    pub mode: ::prost::alloc::string::String,
    /// Optional. Default value for the column.
    #[prost(string, tag = "8")]
    pub default_value: ::prost::alloc::string::String,
    /// Optional. Ordinal position
    #[prost(int32, tag = "9")]
    pub ordinal_position: i32,
    /// Optional. Most important inclusion of this column.
    #[prost(enumeration = "column_schema::IndexingType", tag = "10")]
    pub highest_indexing_type: i32,
    /// Optional. Schema of sub-columns. A column can have zero or more
    /// sub-columns.
    #[prost(message, repeated, tag = "7")]
    pub subcolumns: ::prost::alloc::vec::Vec<ColumnSchema>,
    /// Optional. Garbage collection policy for the column or column family.
    /// Applies to systems like Cloud Bigtable.
    #[prost(string, tag = "11")]
    pub gc_rule: ::prost::alloc::string::String,
    /// Information only applying for columns in Entries from a specific system.
    #[prost(oneof = "column_schema::SystemSpec", tags = "18")]
    pub system_spec: ::core::option::Option<column_schema::SystemSpec>,
}
/// Nested message and enum types in `ColumnSchema`.
pub mod column_schema {
    /// Column info specific to Looker System.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LookerColumnSpec {
        /// Looker specific column type of this column.
        #[prost(enumeration = "looker_column_spec::LookerColumnType", tag = "1")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `LookerColumnSpec`.
    pub mod looker_column_spec {
        /// Column type in Looker.
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
        pub enum LookerColumnType {
            /// Unspecified.
            Unspecified = 0,
            /// Dimension.
            Dimension = 1,
            /// Dimension group - parent for Dimension.
            DimensionGroup = 2,
            /// Filter.
            Filter = 3,
            /// Measure.
            Measure = 4,
            /// Parameter.
            Parameter = 5,
        }
        impl LookerColumnType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    LookerColumnType::Unspecified => "LOOKER_COLUMN_TYPE_UNSPECIFIED",
                    LookerColumnType::Dimension => "DIMENSION",
                    LookerColumnType::DimensionGroup => "DIMENSION_GROUP",
                    LookerColumnType::Filter => "FILTER",
                    LookerColumnType::Measure => "MEASURE",
                    LookerColumnType::Parameter => "PARAMETER",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "LOOKER_COLUMN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "DIMENSION" => Some(Self::Dimension),
                    "DIMENSION_GROUP" => Some(Self::DimensionGroup),
                    "FILTER" => Some(Self::Filter),
                    "MEASURE" => Some(Self::Measure),
                    "PARAMETER" => Some(Self::Parameter),
                    _ => None,
                }
            }
        }
    }
    /// Specifies inclusion of the column in an index
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
    pub enum IndexingType {
        /// Unspecified.
        Unspecified = 0,
        /// Column not a part of an index.
        None = 1,
        /// Column Part of non unique index.
        NonUnique = 2,
        /// Column part of unique index.
        Unique = 3,
        /// Column part of the primary key.
        PrimaryKey = 4,
    }
    impl IndexingType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IndexingType::Unspecified => "INDEXING_TYPE_UNSPECIFIED",
                IndexingType::None => "INDEXING_TYPE_NONE",
                IndexingType::NonUnique => "INDEXING_TYPE_NON_UNIQUE",
                IndexingType::Unique => "INDEXING_TYPE_UNIQUE",
                IndexingType::PrimaryKey => "INDEXING_TYPE_PRIMARY_KEY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INDEXING_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "INDEXING_TYPE_NONE" => Some(Self::None),
                "INDEXING_TYPE_NON_UNIQUE" => Some(Self::NonUnique),
                "INDEXING_TYPE_UNIQUE" => Some(Self::Unique),
                "INDEXING_TYPE_PRIMARY_KEY" => Some(Self::PrimaryKey),
                _ => None,
            }
        }
    }
    /// Information only applying for columns in Entries from a specific system.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SystemSpec {
        /// Looker specific column info of this column.
        #[prost(message, tag = "18")]
        LookerColumnSpec(LookerColumnSpec),
    }
}
/// Result in the response to a search request.
///
/// Each result captures details of one entry that matches the search.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogResult {
    /// Type of the search result.
    ///
    /// You can use this field to determine which get method to call to fetch the
    /// full resource.
    #[prost(enumeration = "SearchResultType", tag = "1")]
    pub search_result_type: i32,
    /// Sub-type of the search result.
    ///
    /// A dot-delimited full type of the resource. The same type you
    /// specify in the `type` search predicate.
    ///
    /// Examples: `entry.table`, `entry.dataStream`, `tagTemplate`.
    #[prost(string, tag = "2")]
    pub search_result_subtype: ::prost::alloc::string::String,
    /// The relative name of the resource in URL format.
    ///
    /// Examples:
    ///
    ///   * `projects/{PROJECT_ID}/locations/{LOCATION_ID}/entryGroups/{ENTRY_GROUP_ID}/entries/{ENTRY_ID}`
    ///   * `projects/{PROJECT_ID}/tagTemplates/{TAG_TEMPLATE_ID}`
    #[prost(string, tag = "3")]
    pub relative_resource_name: ::prost::alloc::string::String,
    /// The full name of the Google Cloud resource the entry belongs to.
    ///
    /// For more information, see \[Full Resource Name\]
    /// (/apis/design/resource_names#full_resource_name).
    ///
    /// Example:
    ///
    /// `//bigquery.googleapis.com/projects/PROJECT_ID/datasets/DATASET_ID/tables/TABLE_ID`
    #[prost(string, tag = "4")]
    pub linked_resource: ::prost::alloc::string::String,
    /// The last modification timestamp of the entry in the source system.
    #[prost(message, optional, tag = "7")]
    pub modify_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Fully qualified name (FQN) of the resource.
    ///
    /// FQNs take two forms:
    ///
    /// * For non-regionalized resources:
    ///
    ///    `{SYSTEM}:{PROJECT}.{PATH_TO_RESOURCE_SEPARATED_WITH_DOTS}`
    ///
    /// * For regionalized resources:
    ///
    ///    `{SYSTEM}:{PROJECT}.{LOCATION_ID}.{PATH_TO_RESOURCE_SEPARATED_WITH_DOTS}`
    ///
    /// Example for a DPMS table:
    ///
    /// `dataproc_metastore:PROJECT_ID.LOCATION_ID.INSTANCE_ID.DATABASE_ID.TABLE_ID`
    #[prost(string, tag = "10")]
    pub fully_qualified_name: ::prost::alloc::string::String,
    /// The display name of the result.
    #[prost(string, tag = "12")]
    pub display_name: ::prost::alloc::string::String,
    /// Entry description that can consist of several sentences or paragraphs that
    /// describe entry contents.
    #[prost(string, tag = "13")]
    pub description: ::prost::alloc::string::String,
    /// The source system of the entry. Applicable only when the
    /// `search_result_type` is `ENTRY`.
    #[prost(oneof = "search_catalog_result::System", tags = "8, 9")]
    pub system: ::core::option::Option<search_catalog_result::System>,
}
/// Nested message and enum types in `SearchCatalogResult`.
pub mod search_catalog_result {
    /// The source system of the entry. Applicable only when the
    /// `search_result_type` is `ENTRY`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum System {
        /// Output only. The source system that Data Catalog automatically integrates
        /// with, such as BigQuery, Cloud Pub/Sub, or Dataproc Metastore.
        #[prost(enumeration = "super::IntegratedSystem", tag = "8")]
        IntegratedSystem(i32),
        /// Custom source system that you can manually integrate Data Catalog with.
        #[prost(string, tag = "9")]
        UserSpecifiedSystem(::prost::alloc::string::String),
    }
}
/// The resource types that can be returned in search results.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SearchResultType {
    /// Default unknown type.
    Unspecified = 0,
    /// An [Entry][google.cloud.datacatalog.v1.Entry].
    Entry = 1,
    /// A [TagTemplate][google.cloud.datacatalog.v1.TagTemplate].
    TagTemplate = 2,
    /// An [EntryGroup][google.cloud.datacatalog.v1.EntryGroup].
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
        /// Table view specification. Populated only if
        /// the `table_source_type` is `BIGQUERY_VIEW`.
        #[prost(message, tag = "2")]
        ViewSpec(super::ViewSpec),
        /// Specification of a BigQuery table. Populated only if
        /// the `table_source_type` is `BIGQUERY_TABLE`.
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
/// Normal BigQuery table specification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableSpec {
    /// Output only. If the table is date-sharded, that is, it matches the
    /// `\[prefix\]YYYYMMDD` name pattern, this field is the Data Catalog resource
    /// name of the date-sharded grouped entry. For example:
    ///
    /// `projects/{PROJECT_ID}/locations/{LOCATION}/entrygroups/{ENTRY_GROUP_ID}/entries/{ENTRY_ID}`.
    ///
    /// Otherwise, `grouped_entry` is empty.
    #[prost(string, tag = "1")]
    pub grouped_entry: ::prost::alloc::string::String,
}
/// Specification for a group of BigQuery tables with the `\[prefix\]YYYYMMDD` name
/// pattern.
///
/// For more information, see \[Introduction to partitioned tables\]
/// (<https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryDateShardedSpec {
    /// Output only. The Data Catalog resource name of the dataset entry the
    /// current table belongs to. For example:
    ///
    /// `projects/{PROJECT_ID}/locations/{LOCATION}/entrygroups/{ENTRY_GROUP_ID}/entries/{ENTRY_ID}`.
    #[prost(string, tag = "1")]
    pub dataset: ::prost::alloc::string::String,
    /// Output only. The table name prefix of the shards.
    ///
    /// The name of any given shard is `\[table_prefix\]YYYYMMDD`.
    /// For example, for the `MyTable20180101` shard, the
    /// `table_prefix` is `MyTable`.
    #[prost(string, tag = "2")]
    pub table_prefix: ::prost::alloc::string::String,
    /// Output only. Total number of shards.
    #[prost(int64, tag = "3")]
    pub shard_count: i64,
    /// Output only. BigQuery resource name of the latest shard.
    #[prost(string, tag = "4")]
    pub latest_shard_resource: ::prost::alloc::string::String,
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
/// Tags contain custom metadata and are attached to Data Catalog resources. Tags
/// conform with the specification of their tag template.
///
/// See [Data Catalog
/// IAM](<https://cloud.google.com/data-catalog/docs/concepts/iam>) for information
/// on the permissions needed to create or view tags.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    /// The resource name of the tag in URL format where tag ID is a
    /// system-generated identifier.
    ///
    /// Note: The tag itself might not be stored in the location specified in its
    /// name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The resource name of the tag template this tag uses. Example:
    ///
    /// `projects/{PROJECT_ID}/locations/{LOCATION}/tagTemplates/{TAG_TEMPLATE_ID}`
    ///
    /// This field cannot be modified after creation.
    #[prost(string, tag = "2")]
    pub template: ::prost::alloc::string::String,
    /// Output only. The display name of the tag template.
    #[prost(string, tag = "5")]
    pub template_display_name: ::prost::alloc::string::String,
    /// Required. Maps the ID of a tag field to its value and additional
    /// information about that field.
    ///
    /// Tag template defines valid field IDs. A tag
    /// must have at least 1 field and at most 500 fields.
    #[prost(btree_map = "string, message", tag = "3")]
    pub fields: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        TagField,
    >,
    /// The scope within the parent resource that this tag is attached to. If not
    /// provided, the tag is attached to the parent resource itself.
    ///
    /// Deleting the scope from the parent resource deletes all tags attached
    /// to that scope.
    ///
    /// These fields cannot be updated after creation.
    #[prost(oneof = "tag::Scope", tags = "4")]
    pub scope: ::core::option::Option<tag::Scope>,
}
/// Nested message and enum types in `Tag`.
pub mod tag {
    /// The scope within the parent resource that this tag is attached to. If not
    /// provided, the tag is attached to the parent resource itself.
    ///
    /// Deleting the scope from the parent resource deletes all tags attached
    /// to that scope.
    ///
    /// These fields cannot be updated after creation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scope {
        /// Resources like entry can have schemas associated with them. This scope
        /// allows you to attach tags to an individual column based on that schema.
        ///
        /// To attach a tag to a nested column, separate column names with a dot
        /// (`.`). Example: `column.nested_column`.
        #[prost(string, tag = "4")]
        Column(::prost::alloc::string::String),
    }
}
/// Contains the value and additional information on a field within
/// a [Tag][google.cloud.datacatalog.v1.Tag].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagField {
    /// Output only. The display name of this field.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The order of this field with respect to other fields in this
    /// tag. Can be set by
    /// [Tag][google.cloud.datacatalog.v1.TagTemplateField.order].
    ///
    /// For example, a higher value can indicate a more important field.
    /// The value can be negative. Multiple fields can have the same order, and
    /// field orders within a tag don't have to be sequential.
    #[prost(int32, tag = "7")]
    pub order: i32,
    /// Required. The value of this field.
    #[prost(oneof = "tag_field::Kind", tags = "2, 3, 4, 5, 6, 8")]
    pub kind: ::core::option::Option<tag_field::Kind>,
}
/// Nested message and enum types in `TagField`.
pub mod tag_field {
    /// An enum value.
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
        /// The value of a tag field with a double type.
        #[prost(double, tag = "2")]
        DoubleValue(f64),
        /// The value of a tag field with a string type.
        ///
        /// The maximum length is 2000 UTF-8 characters.
        #[prost(string, tag = "3")]
        StringValue(::prost::alloc::string::String),
        /// The value of a tag field with a boolean type.
        #[prost(bool, tag = "4")]
        BoolValue(bool),
        /// The value of a tag field with a timestamp type.
        #[prost(message, tag = "5")]
        TimestampValue(::prost_types::Timestamp),
        /// The value of a tag field with an enum type.
        ///
        /// This value must be one of the allowed values listed in this enum.
        #[prost(message, tag = "6")]
        EnumValue(EnumValue),
        /// The value of a tag field with a rich text type.
        ///
        /// The maximum length is 10 MiB as this value holds HTML descriptions
        /// including encoded images. The maximum length of the text without images
        /// is 100 KiB.
        #[prost(string, tag = "8")]
        RichtextValue(::prost::alloc::string::String),
    }
}
/// A tag template defines a tag that can have one or more typed fields.
///
/// The template is used to create tags that are attached to Google Cloud
///   resources. \[Tag template roles\]
/// (<https://cloud.google.com/iam/docs/understanding-roles#data-catalog-roles>)
/// provide permissions to create, edit, and use the template. For example,
/// see the \[TagTemplate User\]
/// (<https://cloud.google.com/data-catalog/docs/how-to/template-user>) role
/// that includes a permission to use the tag template to tag resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagTemplate {
    /// The resource name of the tag template in URL format.
    ///
    /// Note: The tag template itself and its child resources might not be
    /// stored in the location specified in its name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Display name for this template. Defaults to an empty string.
    ///
    /// The name must contain only Unicode letters, numbers (0-9), underscores (_),
    /// dashes (-), spaces ( ), and can't start or end with spaces.
    /// The maximum length is 200 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Indicates whether tags created with this template are public. Public tags
    /// do not require tag template access to appear in
    /// [ListTags][google.cloud.datacatalog.v1.ListTags] API response.
    ///
    /// Additionally, you can search for a public tag by value with a
    /// simple search query in addition to using a ``tag:`` predicate.
    #[prost(bool, tag = "5")]
    pub is_publicly_readable: bool,
    /// Required. Map of tag template field IDs to the settings for the field.
    /// This map is an exhaustive list of the allowed fields. The map must contain
    /// at least one field and at most 500 fields.
    ///
    /// The keys to this map are tag template field IDs. The IDs have the
    /// following limitations:
    ///
    /// * Can contain uppercase and lowercase letters, numbers (0-9) and
    ///    underscores (_).
    /// * Must be at least 1 character and at most 64 characters long.
    /// * Must start with a letter or underscore.
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
    /// `projects/{PROJECT_ID}/locations/{LOCATION}/tagTemplates/{TAG_TEMPLATE}/fields/{FIELD}`
    ///
    /// Note: The tag template field itself might not be stored in the location
    /// specified in its name.
    ///
    /// The name must contain only letters (a-z, A-Z), numbers (0-9),
    /// or underscores (_), and must start with a letter or underscore.
    /// The maximum length is 64 characters.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// The display name for this field. Defaults to an empty string.
    ///
    /// The name must contain only Unicode letters, numbers (0-9), underscores (_),
    /// dashes (-), spaces ( ), and can't start or end with spaces.
    /// The maximum length is 200 characters.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The type of value this tag field can contain.
    #[prost(message, optional, tag = "2")]
    pub r#type: ::core::option::Option<FieldType>,
    /// If true, this field is required. Defaults to false.
    #[prost(bool, tag = "3")]
    pub is_required: bool,
    /// The description for this field. Defaults to an empty string.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// The order of this field with respect to other fields in this tag
    /// template.
    ///
    /// For example, a higher value can indicate a more important field.
    /// The value can be negative. Multiple fields can have the same order and
    /// field orders within a tag don't have to be sequential.
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
        /// The set of allowed values for this enum.
        ///
        /// This set must not be empty and can include up to 100 allowed values.
        /// The display names of the values in this set must not be empty and must
        /// be case-insensitively unique within this set.
        ///
        /// The order of items in this set is preserved. This field can be used to
        /// create, remove, and reorder enum values. To rename enum values, use the
        /// `RenameTagTemplateFieldEnumValue` method.
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
            ///
            /// The name must contain only Unicode letters, numbers (0-9), underscores
            /// (_), dashes (-), spaces ( ), and can't start or end with spaces. The
            /// maximum length is 200 characters.
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
        /// The default invalid value for a type.
        Unspecified = 0,
        /// A double precision number.
        Double = 1,
        /// An UTF-8 string.
        String = 2,
        /// A boolean value.
        Bool = 3,
        /// A timestamp.
        Timestamp = 4,
        /// A Richtext description.
        Richtext = 5,
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
                PrimitiveType::Richtext => "RICHTEXT",
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
                "RICHTEXT" => Some(Self::Richtext),
                _ => None,
            }
        }
    }
    /// Required.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeDecl {
        /// Primitive types, such as string, boolean, etc.
        #[prost(enumeration = "PrimitiveType", tag = "1")]
        PrimitiveType(i32),
        /// An enum type.
        #[prost(message, tag = "2")]
        EnumType(EnumType),
    }
}
/// Detailed statistics on the entry's usage.
///
/// Usage statistics have the following limitations:
///
/// - Only BigQuery tables have them.
/// - They only include BigQuery query jobs.
/// - They might be underestimated because wildcard table references
///    are not yet counted. For more information, see
///    \[Querying multiple tables using a wildcard table\]
///    (<https://cloud.google.com/bigquery/docs/querying-wildcard-tables>)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsageStats {
    /// The number of successful uses of the underlying entry.
    #[prost(float, tag = "1")]
    pub total_completions: f32,
    /// The number of failed attempts to use the underlying entry.
    #[prost(float, tag = "2")]
    pub total_failures: f32,
    /// The number of cancelled attempts to use the underlying entry.
    #[prost(float, tag = "3")]
    pub total_cancellations: f32,
    /// Total time spent only on successful uses, in milliseconds.
    #[prost(float, tag = "4")]
    pub total_execution_time_for_completions_millis: f32,
}
/// Common statistics on the entry's usage.
///
/// They can be set on any system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonUsageStats {
    /// View count in source system.
    #[prost(int64, optional, tag = "1")]
    pub view_count: ::core::option::Option<i64>,
}
/// The set of all usage signals that Data Catalog stores.
///
/// Note: Usually, these signals are updated daily. In rare cases, an update may
/// fail but will be performed again on the next day.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsageSignal {
    /// The end timestamp of the duration of usage statistics.
    #[prost(message, optional, tag = "1")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. BigQuery usage statistics over each of the predefined time
    /// ranges.
    ///
    /// Supported time ranges are `{"24H", "7D", "30D"}`.
    #[prost(btree_map = "string, message", tag = "2")]
    pub usage_within_time_range: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        UsageStats,
    >,
    /// Common usage statistics over each of the predefined time ranges.
    ///
    /// Supported time ranges are `{"24H", "7D", "30D", "Lifetime"}`.
    #[prost(btree_map = "string, message", tag = "3")]
    pub common_usage_within_time_range: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        CommonUsageStats,
    >,
    /// Favorite count in the source system.
    #[prost(int64, optional, tag = "4")]
    pub favorite_count: ::core::option::Option<i64>,
}
/// Request message for
/// [SearchCatalog][google.cloud.datacatalog.v1.DataCatalog.SearchCatalog].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogRequest {
    /// Required. The scope of this search request.
    ///
    /// The `scope` is invalid if `include_org_ids`, `include_project_ids` are
    /// empty AND `include_gcp_public_datasets` is set to `false`. In this case,
    /// the request returns an error.
    #[prost(message, optional, tag = "6")]
    pub scope: ::core::option::Option<search_catalog_request::Scope>,
    /// Optional. The query string with a minimum of 3 characters and specific
    /// syntax. For more information, see [Data Catalog search
    /// syntax](<https://cloud.google.com/data-catalog/docs/how-to/search-reference>).
    ///
    /// An empty query string returns all data assets (in the specified scope)
    /// that you have access to.
    ///
    /// A query string can be a simple `xyz` or qualified by predicates:
    ///
    /// * `name:x`
    /// * `column:y`
    /// * `description:z`
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// Upper bound on the number of results you can get in a single response.
    ///
    /// Can't be negative or 0, defaults to 10 in this case.
    /// The maximum number is 1000. If exceeded, throws an "invalid argument"
    /// exception.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token that, if specified, returns the next page of
    /// search results. If empty, returns the first page.
    ///
    /// This token is returned in the
    /// [SearchCatalogResponse.next_page_token][google.cloud.datacatalog.v1.SearchCatalogResponse.next_page_token]
    /// field of the response to a previous
    /// [SearchCatalogRequest][google.cloud.datacatalog.v1.DataCatalog.SearchCatalog]
    /// call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Specifies the order of results.
    ///
    /// Currently supported case-sensitive values are:
    ///
    /// * `relevance` that can only be descending
    /// * `last_modified_timestamp \[asc|desc\]` with descending (`desc`) as default
    /// * `default` that can only be descending
    ///
    /// Search queries don't guarantee full recall. Results that match your query
    /// might not be returned, even in subsequent result pages. Additionally,
    /// returned (and not returned) results can vary if you repeat search queries.
    /// If you are experiencing recall issues and you don't have to fetch the
    /// results in any specific order, consider setting this parameter to
    /// `default`.
    ///
    /// If this parameter is omitted, it defaults to the descending `relevance`.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. If set, use searchAll permission granted on organizations from
    /// `include_org_ids` and projects from `include_project_ids` instead of the
    /// fine grained per resource permissions when filtering the search results.
    /// The only allowed `order_by` criteria for admin_search mode is `default`.
    /// Using this flags guarantees a full recall of the search results.
    #[prost(bool, tag = "17")]
    pub admin_search: bool,
}
/// Nested message and enum types in `SearchCatalogRequest`.
pub mod search_catalog_request {
    /// The criteria that select the subspace used for query matching.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Scope {
        /// The list of organization IDs to search within.
        ///
        /// To find your organization ID, follow the steps from
        /// \[Creating and managing organizations\]
        /// (/resource-manager/docs/creating-managing-organization).
        #[prost(string, repeated, tag = "2")]
        pub include_org_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The list of project IDs to search within.
        ///
        /// For more information on the distinction between project names, IDs, and
        /// numbers, see [Projects](/docs/overview/#projects).
        #[prost(string, repeated, tag = "3")]
        pub include_project_ids: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// If `true`, include Google Cloud public datasets in
        /// search results. By default, they are excluded.
        ///
        /// See [Google Cloud Public Datasets](/public-datasets) for more
        /// information.
        #[prost(bool, tag = "7")]
        pub include_gcp_public_datasets: bool,
        /// Optional. The list of locations to search within. If empty, all locations
        /// are searched.
        ///
        /// Returns an error if any location in the list isn't one of the [Supported
        /// regions](<https://cloud.google.com/data-catalog/docs/concepts/regions#supported_regions>).
        ///
        /// If a location is unreachable, its name is returned in the
        /// `SearchCatalogResponse.unreachable` field. To get additional information
        /// on the error, repeat the search request and set the location name as the
        /// value of this parameter.
        #[prost(string, repeated, tag = "16")]
        pub restricted_locations: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Optional. If `true`, search only among starred entries.
        ///
        /// By default, all results are returned, starred or not.
        #[prost(bool, tag = "18")]
        pub starred_only: bool,
        /// Optional. This field is deprecated. The search mechanism for public and
        /// private tag templates is the same.
        #[deprecated]
        #[prost(bool, tag = "19")]
        pub include_public_tag_templates: bool,
    }
}
/// Response message for
/// [SearchCatalog][google.cloud.datacatalog.v1.DataCatalog.SearchCatalog].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogResponse {
    /// Search results.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<SearchCatalogResult>,
    /// The approximate total number of entries matched by the query.
    #[prost(int32, tag = "2")]
    pub total_size: i32,
    /// Pagination token that can be used in subsequent calls to retrieve the next
    /// page of results.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable locations. Search results don't include data from those
    /// locations.
    ///
    /// To get additional information on an error, repeat the search request and
    /// restrict it to specific locations by setting the
    /// `SearchCatalogRequest.scope.restricted_locations` parameter.
    #[prost(string, repeated, tag = "6")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [CreateEntryGroup][google.cloud.datacatalog.v1.DataCatalog.CreateEntryGroup].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntryGroupRequest {
    /// Required. The names of the project and location that the new entry group
    /// belongs to.
    ///
    /// Note: The entry group itself and its child resources might not be
    /// stored in the location specified in its name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the entry group to create.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), and must start with a letter or underscore.
    /// The maximum size is 64 bytes when encoded in UTF-8.
    #[prost(string, tag = "3")]
    pub entry_group_id: ::prost::alloc::string::String,
    /// The entry group to create. Defaults to empty.
    #[prost(message, optional, tag = "2")]
    pub entry_group: ::core::option::Option<EntryGroup>,
}
/// Request message for
/// [UpdateEntryGroup][google.cloud.datacatalog.v1.DataCatalog.UpdateEntryGroup].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntryGroupRequest {
    /// Required. Updates for the entry group. The `name` field must be set.
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
/// [GetEntryGroup][google.cloud.datacatalog.v1.DataCatalog.GetEntryGroup].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntryGroupRequest {
    /// Required. The name of the entry group to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The fields to return. If empty or omitted, all fields are returned.
    #[prost(message, optional, tag = "2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [DeleteEntryGroup][google.cloud.datacatalog.v1.DataCatalog.DeleteEntryGroup].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntryGroupRequest {
    /// Required. The name of the entry group to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If true, deletes all entries in the entry group.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for
/// [ListEntryGroups][google.cloud.datacatalog.v1.DataCatalog.ListEntryGroups].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntryGroupsRequest {
    /// Required. The name of the location that contains the entry groups to list.
    ///
    /// Can be provided as a URL.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return.
    ///
    /// Default is 10. Maximum limit is 1000.
    /// Throws an invalid argument if `page_size` is greater than 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token that specifies the next page to return.
    /// If empty, returns the first page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [ListEntryGroups][google.cloud.datacatalog.v1.DataCatalog.ListEntryGroups].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntryGroupsResponse {
    /// Entry group details.
    #[prost(message, repeated, tag = "1")]
    pub entry_groups: ::prost::alloc::vec::Vec<EntryGroup>,
    /// Pagination token to specify in the next call to retrieve the next page of
    /// results. Empty if there are no more items.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [CreateEntry][google.cloud.datacatalog.v1.DataCatalog.CreateEntry].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntryRequest {
    /// Required. The name of the entry group this entry belongs to.
    ///
    /// Note: The entry itself and its child resources might not be stored in
    /// the location specified in its name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the entry to create.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9),
    /// and underscores (_).
    /// The maximum size is 64 bytes when encoded in UTF-8.
    #[prost(string, tag = "3")]
    pub entry_id: ::prost::alloc::string::String,
    /// Required. The entry to create.
    #[prost(message, optional, tag = "2")]
    pub entry: ::core::option::Option<Entry>,
}
/// Request message for
/// [UpdateEntry][google.cloud.datacatalog.v1.DataCatalog.UpdateEntry].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntryRequest {
    /// Required. Updates for the entry. The `name` field must be set.
    #[prost(message, optional, tag = "1")]
    pub entry: ::core::option::Option<Entry>,
    /// Names of fields whose values to overwrite on an entry.
    ///
    /// If this parameter is absent or empty, all modifiable fields
    /// are overwritten. If such fields are non-required and omitted in the
    /// request body, their values are emptied.
    ///
    /// You can modify only the fields listed below.
    ///
    /// For entries with type `DATA_STREAM`:
    ///
    /// * `schema`
    ///
    /// For entries with type `FILESET`:
    ///
    /// * `schema`
    /// * `display_name`
    /// * `description`
    /// * `gcs_fileset_spec`
    /// * `gcs_fileset_spec.file_patterns`
    ///
    /// For entries with `user_specified_type`:
    ///
    /// * `schema`
    /// * `display_name`
    /// * `description`
    /// * `user_specified_type`
    /// * `user_specified_system`
    /// * `linked_resource`
    /// * `source_system_timestamps`
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [DeleteEntry][google.cloud.datacatalog.v1.DataCatalog.DeleteEntry].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntryRequest {
    /// Required. The name of the entry to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [GetEntry][google.cloud.datacatalog.v1.DataCatalog.GetEntry].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntryRequest {
    /// Required. The name of the entry to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [LookupEntry][google.cloud.datacatalog.v1.DataCatalog.LookupEntry].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEntryRequest {
    /// Project where the lookup should be performed. Required to lookup
    /// entry that is not a part of `DPMS` or `DATAPLEX` `integrated_system`
    /// using its `fully_qualified_name`. Ignored in other cases.
    #[prost(string, tag = "6")]
    pub project: ::prost::alloc::string::String,
    /// Location where the lookup should be performed. Required to lookup
    /// entry that is not a part of `DPMS` or `DATAPLEX` `integrated_system`
    /// using its `fully_qualified_name`. Ignored in other cases.
    #[prost(string, tag = "7")]
    pub location: ::prost::alloc::string::String,
    /// Required. A full name, SQL name, or a fully qualified name of a
    /// Google Cloud Platform resource.
    #[prost(oneof = "lookup_entry_request::TargetName", tags = "1, 3, 5")]
    pub target_name: ::core::option::Option<lookup_entry_request::TargetName>,
}
/// Nested message and enum types in `LookupEntryRequest`.
pub mod lookup_entry_request {
    /// Required. A full name, SQL name, or a fully qualified name of a
    /// Google Cloud Platform resource.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetName {
        /// The full name of the Google Cloud Platform resource the Data Catalog
        /// entry represents. For more information, see \[Full Resource Name\]
        /// (<https://cloud.google.com/apis/design/resource_names#full_resource_name>).
        ///
        /// Full names are case-sensitive. For example:
        ///
        ///   * `//bigquery.googleapis.com/projects/{PROJECT_ID}/datasets/{DATASET_ID}/tables/{TABLE_ID}`
        ///   * `//pubsub.googleapis.com/projects/{PROJECT_ID}/topics/{TOPIC_ID}`
        #[prost(string, tag = "1")]
        LinkedResource(::prost::alloc::string::String),
        /// The SQL name of the entry. SQL names are case-sensitive.
        ///
        /// Examples:
        ///
        /// * `pubsub.topic.{PROJECT_ID}.{TOPIC_ID}`
        /// * `pubsub.topic.{PROJECT_ID}.`\``{TOPIC.ID.SEPARATED.WITH.DOTS}`\`
        /// * `bigquery.table.{PROJECT_ID}.{DATASET_ID}.{TABLE_ID}`
        /// * `bigquery.dataset.{PROJECT_ID}.{DATASET_ID}`
        /// * `datacatalog.entry.{PROJECT_ID}.{LOCATION_ID}.{ENTRY_GROUP_ID}.{ENTRY_ID}`
        ///
        /// Identifiers (`*_ID`) should comply with the
        /// \[Lexical structure in Standard SQL\]
        /// (<https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical>).
        #[prost(string, tag = "3")]
        SqlResource(::prost::alloc::string::String),
        /// [Fully Qualified Name
        /// (FQN)](<https://cloud.google.com//data-catalog/docs/fully-qualified-names>)
        /// of the resource.
        ///
        /// FQNs take two forms:
        ///
        /// * For non-regionalized resources:
        ///
        ///    `{SYSTEM}:{PROJECT}.{PATH_TO_RESOURCE_SEPARATED_WITH_DOTS}`
        ///
        /// * For regionalized resources:
        ///
        ///    `{SYSTEM}:{PROJECT}.{LOCATION_ID}.{PATH_TO_RESOURCE_SEPARATED_WITH_DOTS}`
        ///
        /// Example for a DPMS table:
        ///
        /// `dataproc_metastore:{PROJECT_ID}.{LOCATION_ID}.{INSTANCE_ID}.{DATABASE_ID}.{TABLE_ID}`
        #[prost(string, tag = "5")]
        FullyQualifiedName(::prost::alloc::string::String),
    }
}
/// Entry metadata.
/// A Data Catalog entry represents another resource in Google
/// Cloud Platform (such as a BigQuery dataset or a Pub/Sub topic) or
/// outside of it. You can use the `linked_resource` field
/// in the entry resource to refer to the original resource ID of the source
/// system.
///
/// An entry resource contains resource details, for example, its schema.
/// Additionally, you can attach flexible metadata to an entry in the form of a
/// [Tag][google.cloud.datacatalog.v1.Tag].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entry {
    /// Output only. The resource name of an entry in URL format.
    ///
    /// Note: The entry itself and its child resources might not be
    /// stored in the location specified in its name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The resource this metadata entry refers to.
    ///
    /// For Google Cloud Platform resources, `linked_resource` is the
    /// \[Full Resource Name\]
    /// (<https://cloud.google.com/apis/design/resource_names#full_resource_name>).
    /// For example, the `linked_resource` for a table resource from BigQuery is:
    ///
    /// `//bigquery.googleapis.com/projects/{PROJECT_ID}/datasets/{DATASET_ID}/tables/{TABLE_ID}`
    ///
    /// Output only when the entry is one of the types in the `EntryType` enum.
    ///
    /// For entries with a `user_specified_type`, this field is optional and
    /// defaults to an empty string.
    ///
    /// The resource string must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), periods (.), colons (:), slashes (/), dashes (-),
    /// and hashes (#).
    /// The maximum size is 200 bytes when encoded in UTF-8.
    #[prost(string, tag = "9")]
    pub linked_resource: ::prost::alloc::string::String,
    /// [Fully Qualified Name
    /// (FQN)](<https://cloud.google.com//data-catalog/docs/fully-qualified-names>)
    /// of the resource. Set automatically for entries representing resources from
    /// synced systems. Settable only during creation, and read-only later. Can
    /// be used for search and lookup of the entries.
    ///
    #[prost(string, tag = "29")]
    pub fully_qualified_name: ::prost::alloc::string::String,
    /// Display name of an entry.
    ///
    /// The maximum size is 500 bytes when encoded in UTF-8.
    /// Default value is an empty string.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Entry description that can consist of several sentences or paragraphs
    /// that describe entry contents.
    ///
    /// The description must not contain Unicode non-characters as well as C0
    /// and C1 control codes except tabs (HT), new lines (LF), carriage returns
    /// (CR), and page breaks (FF).
    /// The maximum size is 2000 bytes when encoded in UTF-8.
    /// Default value is an empty string.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Business Context of the entry. Not supported for BigQuery datasets
    #[prost(message, optional, tag = "37")]
    pub business_context: ::core::option::Option<BusinessContext>,
    /// Schema of the entry. An entry might not have any schema attached to it.
    #[prost(message, optional, tag = "5")]
    pub schema: ::core::option::Option<Schema>,
    /// Timestamps from the underlying resource, not from the Data Catalog
    /// entry.
    ///
    /// Output only when the entry has a system listed in the `IntegratedSystem`
    /// enum. For entries with `user_specified_system`, this field is optional
    /// and defaults to an empty timestamp.
    #[prost(message, optional, tag = "7")]
    pub source_system_timestamps: ::core::option::Option<SystemTimestamps>,
    /// Resource usage statistics.
    #[prost(message, optional, tag = "13")]
    pub usage_signal: ::core::option::Option<UsageSignal>,
    /// Cloud labels attached to the entry.
    ///
    /// In Data Catalog, you can create and modify labels attached only to custom
    /// entries. Synced entries have unmodifiable labels that come from the source
    /// system.
    #[prost(btree_map = "string, string", tag = "14")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Physical location of the entry.
    #[prost(message, optional, tag = "20")]
    pub data_source: ::core::option::Option<DataSource>,
    /// Output only. Additional information related to the entry. Private to the
    /// current user.
    #[prost(message, optional, tag = "26")]
    pub personal_details: ::core::option::Option<PersonalDetails>,
    /// Required. Entry type.
    #[prost(oneof = "entry::EntryType", tags = "2, 16")]
    pub entry_type: ::core::option::Option<entry::EntryType>,
    /// The source system of the entry.
    #[prost(oneof = "entry::System", tags = "17, 18")]
    pub system: ::core::option::Option<entry::System>,
    /// System specification.
    /// Can be used as a complement for `spec`, when some metadata is relevant for
    /// all entries existing within given system
    #[prost(oneof = "entry::SystemSpec", tags = "39, 40, 41")]
    pub system_spec: ::core::option::Option<entry::SystemSpec>,
    /// Type specification.
    #[prost(oneof = "entry::TypeSpec", tags = "6, 12, 15")]
    pub type_spec: ::core::option::Option<entry::TypeSpec>,
    /// Type- and system-specific information. Specifications for types contain
    /// fields common to all entries of a given type, and sub-specifications with
    /// fields specific to a given source system.
    ///
    /// When extending the API with new types and systems, use this field instead
    /// of the legacy `type_spec`.
    #[prost(oneof = "entry::Spec", tags = "24, 27, 28, 32, 33, 42, 43")]
    pub spec: ::core::option::Option<entry::Spec>,
}
/// Nested message and enum types in `Entry`.
pub mod entry {
    /// Required. Entry type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EntryType {
        /// The type of the entry.
        ///
        /// For details, see [`EntryType`](#entrytype).
        #[prost(enumeration = "super::EntryType", tag = "2")]
        Type(i32),
        /// Custom entry type that doesn't match any of the values allowed for input
        /// and listed in the `EntryType` enum.
        ///
        /// When creating an entry, first check the type values in the enum.
        /// If there are no appropriate types for the new entry,
        /// provide a custom value, for example, `my_special_type`.
        ///
        /// The `user_specified_type` string has the following limitations:
        ///
        /// * Is case insensitive.
        /// * Must begin with a letter or underscore.
        /// * Can only contain letters, numbers, and underscores.
        /// * Must be at least 1 character and at most 64 characters long.
        #[prost(string, tag = "16")]
        UserSpecifiedType(::prost::alloc::string::String),
    }
    /// The source system of the entry.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum System {
        /// Output only. Indicates the entry's source system that Data Catalog
        /// integrates with, such as BigQuery, Pub/Sub, or Dataproc Metastore.
        #[prost(enumeration = "super::IntegratedSystem", tag = "17")]
        IntegratedSystem(i32),
        /// Indicates the entry's source system that Data Catalog doesn't
        /// automatically integrate with.
        ///
        /// The `user_specified_system` string has the following limitations:
        ///
        /// * Is case insensitive.
        /// * Must begin with a letter or underscore.
        /// * Can only contain letters, numbers, and underscores.
        /// * Must be at least 1 character and at most 64 characters long.
        #[prost(string, tag = "18")]
        UserSpecifiedSystem(::prost::alloc::string::String),
    }
    /// System specification.
    /// Can be used as a complement for `spec`, when some metadata is relevant for
    /// all entries existing within given system
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SystemSpec {
        /// Specification that applies to a relational database system. Only settable
        /// when `user_specified_system` is equal to `SQL_DATABASE`
        #[prost(message, tag = "39")]
        SqlDatabaseSystemSpec(super::SqlDatabaseSystemSpec),
        /// Specification that applies to Looker sysstem. Only settable when
        /// `user_specified_system` is equal to `LOOKER`
        #[prost(message, tag = "40")]
        LookerSystemSpec(super::LookerSystemSpec),
        /// Specification that applies to Cloud Bigtable system. Only settable when
        /// `integrated_system` is equal to `CLOUD_BIGTABLE`
        #[prost(message, tag = "41")]
        CloudBigtableSystemSpec(super::CloudBigtableSystemSpec),
    }
    /// Type specification.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeSpec {
        /// Specification that applies to a Cloud Storage fileset. Valid only
        /// for entries with the `FILESET` type.
        #[prost(message, tag = "6")]
        GcsFilesetSpec(super::GcsFilesetSpec),
        /// Output only. Specification that applies to a BigQuery table. Valid only
        /// for entries with the `TABLE` type.
        #[prost(message, tag = "12")]
        BigqueryTableSpec(super::BigQueryTableSpec),
        /// Output only. Specification for a group of BigQuery tables with
        /// the `\[prefix\]YYYYMMDD` name pattern.
        ///
        /// For more information, see \[Introduction to partitioned tables\]
        /// (<https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding>).
        #[prost(message, tag = "15")]
        BigqueryDateShardedSpec(super::BigQueryDateShardedSpec),
    }
    /// Type- and system-specific information. Specifications for types contain
    /// fields common to all entries of a given type, and sub-specifications with
    /// fields specific to a given source system.
    ///
    /// When extending the API with new types and systems, use this field instead
    /// of the legacy `type_spec`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Spec {
        /// Specification that applies to a table resource. Valid only
        /// for entries with the `TABLE` or `EXPLORE` type.
        #[prost(message, tag = "24")]
        DatabaseTableSpec(super::DatabaseTableSpec),
        /// Specification that applies to a data source connection. Valid only
        /// for entries with the `DATA_SOURCE_CONNECTION` type.
        #[prost(message, tag = "27")]
        DataSourceConnectionSpec(super::DataSourceConnectionSpec),
        /// Specification that applies to a user-defined function or procedure. Valid
        /// only for entries with the `ROUTINE` type.
        #[prost(message, tag = "28")]
        RoutineSpec(super::RoutineSpec),
        /// Specification that applies to a dataset.
        #[prost(message, tag = "32")]
        DatasetSpec(super::DatasetSpec),
        /// Specification that applies to a fileset resource. Valid only
        /// for entries with the `FILESET` type.
        #[prost(message, tag = "33")]
        FilesetSpec(super::FilesetSpec),
        /// Specification that applies to a Service resource.
        #[prost(message, tag = "42")]
        ServiceSpec(super::ServiceSpec),
        /// Model specification.
        #[prost(message, tag = "43")]
        ModelSpec(super::ModelSpec),
    }
}
/// Specification that applies to a table resource. Valid only
/// for entries with the `TABLE` type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseTableSpec {
    /// Type of this table.
    #[prost(enumeration = "database_table_spec::TableType", tag = "1")]
    pub r#type: i32,
    /// Output only. Fields specific to a Dataplex table and present only in the
    /// Dataplex table entries.
    #[prost(message, optional, tag = "2")]
    pub dataplex_table: ::core::option::Option<DataplexTableSpec>,
    /// Spec what aplies to tables that are actually views.
    /// Not set for "real" tables.
    #[prost(message, optional, tag = "3")]
    pub database_view_spec: ::core::option::Option<
        database_table_spec::DatabaseViewSpec,
    >,
}
/// Nested message and enum types in `DatabaseTableSpec`.
pub mod database_table_spec {
    /// Specification that applies to database view.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatabaseViewSpec {
        /// Type of this view.
        #[prost(enumeration = "database_view_spec::ViewType", tag = "1")]
        pub view_type: i32,
        /// Definition of the view.
        #[prost(oneof = "database_view_spec::SourceDefinition", tags = "2, 3")]
        pub source_definition: ::core::option::Option<
            database_view_spec::SourceDefinition,
        >,
    }
    /// Nested message and enum types in `DatabaseViewSpec`.
    pub mod database_view_spec {
        /// Concrete type of the view.
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
        pub enum ViewType {
            /// Default unknown view type.
            Unspecified = 0,
            /// Standard view.
            StandardView = 1,
            /// Materialized view.
            MaterializedView = 2,
        }
        impl ViewType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ViewType::Unspecified => "VIEW_TYPE_UNSPECIFIED",
                    ViewType::StandardView => "STANDARD_VIEW",
                    ViewType::MaterializedView => "MATERIALIZED_VIEW",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "VIEW_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "STANDARD_VIEW" => Some(Self::StandardView),
                    "MATERIALIZED_VIEW" => Some(Self::MaterializedView),
                    _ => None,
                }
            }
        }
        /// Definition of the view.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum SourceDefinition {
            /// Name of a singular table this view reflects one to one.
            #[prost(string, tag = "2")]
            BaseTable(::prost::alloc::string::String),
            /// SQL query used to generate this view.
            #[prost(string, tag = "3")]
            SqlQuery(::prost::alloc::string::String),
        }
    }
    /// Type of the table.
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
    pub enum TableType {
        /// Default unknown table type.
        Unspecified = 0,
        /// Native table.
        Native = 1,
        /// External table.
        External = 2,
    }
    impl TableType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TableType::Unspecified => "TABLE_TYPE_UNSPECIFIED",
                TableType::Native => "NATIVE",
                TableType::External => "EXTERNAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TABLE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "NATIVE" => Some(Self::Native),
                "EXTERNAL" => Some(Self::External),
                _ => None,
            }
        }
    }
}
/// Specification that applies to a fileset. Valid only for entries with the
/// 'FILESET' type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilesetSpec {
    /// Fields specific to a Dataplex fileset and present only in the Dataplex
    /// fileset entries.
    #[prost(message, optional, tag = "1")]
    pub dataplex_fileset: ::core::option::Option<DataplexFilesetSpec>,
}
/// Specification that applies to a data source connection. Valid only for
/// entries with the `DATA_SOURCE_CONNECTION` type.
/// Only one of internal specs can be set at the time, and cannot
/// be changed later.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSourceConnectionSpec {
    /// Output only. Fields specific to BigQuery connections.
    #[prost(message, optional, tag = "1")]
    pub bigquery_connection_spec: ::core::option::Option<BigQueryConnectionSpec>,
}
/// Specification that applies to a routine. Valid only for
/// entries with the `ROUTINE` type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutineSpec {
    /// The type of the routine.
    #[prost(enumeration = "routine_spec::RoutineType", tag = "1")]
    pub routine_type: i32,
    /// The language the routine is written in. The exact value depends on the
    /// source system. For BigQuery routines, possible values are:
    ///
    /// * `SQL`
    /// * `JAVASCRIPT`
    #[prost(string, tag = "2")]
    pub language: ::prost::alloc::string::String,
    /// Arguments of the routine.
    #[prost(message, repeated, tag = "3")]
    pub routine_arguments: ::prost::alloc::vec::Vec<routine_spec::Argument>,
    /// Return type of the argument. The exact value depends on the source system
    /// and the language.
    #[prost(string, tag = "4")]
    pub return_type: ::prost::alloc::string::String,
    /// The body of the routine.
    #[prost(string, tag = "5")]
    pub definition_body: ::prost::alloc::string::String,
    /// Contains fields specific to the source system.
    #[prost(oneof = "routine_spec::SystemSpec", tags = "6")]
    pub system_spec: ::core::option::Option<routine_spec::SystemSpec>,
}
/// Nested message and enum types in `RoutineSpec`.
pub mod routine_spec {
    /// Input or output argument of a function or stored procedure.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Argument {
        /// The name of the argument. A return argument of a function might not have
        /// a name.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Specifies whether the argument is input or output.
        #[prost(enumeration = "argument::Mode", tag = "2")]
        pub mode: i32,
        /// Type of the argument. The exact value depends on the source system and
        /// the language.
        #[prost(string, tag = "3")]
        pub r#type: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Argument`.
    pub mod argument {
        /// The input or output mode of the argument.
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
        pub enum Mode {
            /// Unspecified mode.
            Unspecified = 0,
            /// The argument is input-only.
            In = 1,
            /// The argument is output-only.
            Out = 2,
            /// The argument is both an input and an output.
            Inout = 3,
        }
        impl Mode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Mode::Unspecified => "MODE_UNSPECIFIED",
                    Mode::In => "IN",
                    Mode::Out => "OUT",
                    Mode::Inout => "INOUT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "MODE_UNSPECIFIED" => Some(Self::Unspecified),
                    "IN" => Some(Self::In),
                    "OUT" => Some(Self::Out),
                    "INOUT" => Some(Self::Inout),
                    _ => None,
                }
            }
        }
    }
    /// The fine-grained type of the routine.
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
    pub enum RoutineType {
        /// Unspecified type.
        Unspecified = 0,
        /// Non-builtin permanent scalar function.
        ScalarFunction = 1,
        /// Stored procedure.
        Procedure = 2,
    }
    impl RoutineType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RoutineType::Unspecified => "ROUTINE_TYPE_UNSPECIFIED",
                RoutineType::ScalarFunction => "SCALAR_FUNCTION",
                RoutineType::Procedure => "PROCEDURE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROUTINE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SCALAR_FUNCTION" => Some(Self::ScalarFunction),
                "PROCEDURE" => Some(Self::Procedure),
                _ => None,
            }
        }
    }
    /// Contains fields specific to the source system.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SystemSpec {
        /// Fields specific for BigQuery routines.
        #[prost(message, tag = "6")]
        BigqueryRoutineSpec(super::BigQueryRoutineSpec),
    }
}
/// Specification that applies to a dataset. Valid only for
/// entries with the `DATASET` type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetSpec {
    /// Fields specific to the source system.
    #[prost(oneof = "dataset_spec::SystemSpec", tags = "2")]
    pub system_spec: ::core::option::Option<dataset_spec::SystemSpec>,
}
/// Nested message and enum types in `DatasetSpec`.
pub mod dataset_spec {
    /// Fields specific to the source system.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SystemSpec {
        /// Vertex AI Dataset specific fields
        #[prost(message, tag = "2")]
        VertexDatasetSpec(super::VertexDatasetSpec),
    }
}
/// Specification that applies to
/// entries that are part `SQL_DATABASE` system
/// (user_specified_type)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlDatabaseSystemSpec {
    /// SQL Database Engine.
    /// enum SqlEngine {
    ///   UNDEFINED = 0;
    ///   MY_SQL = 1;
    ///   POSTGRE_SQL = 2;
    ///   SQL_SERVER = 3;
    /// }
    /// Engine of the enclosing database instance.
    #[prost(string, tag = "1")]
    pub sql_engine: ::prost::alloc::string::String,
    /// Version of the database engine.
    #[prost(string, tag = "2")]
    pub database_version: ::prost::alloc::string::String,
    /// Host of the SQL database
    /// enum InstanceHost {
    ///   UNDEFINED = 0;
    ///   SELF_HOSTED = 1;
    ///   CLOUD_SQL = 2;
    ///   AMAZON_RDS = 3;
    ///   AZURE_SQL = 4;
    /// }
    /// Host of the enclousing database instance.
    #[prost(string, tag = "3")]
    pub instance_host: ::prost::alloc::string::String,
}
/// Specification that applies to
/// entries that are part `LOOKER` system
/// (user_specified_type)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookerSystemSpec {
    /// ID of the parent Looker Instance. Empty if it does not exist.
    /// Example value: `someinstance.looker.com`
    #[prost(string, tag = "1")]
    pub parent_instance_id: ::prost::alloc::string::String,
    /// Name of the parent Looker Instance. Empty if it does not exist.
    #[prost(string, tag = "2")]
    pub parent_instance_display_name: ::prost::alloc::string::String,
    /// ID of the parent Model. Empty if it does not exist.
    #[prost(string, tag = "3")]
    pub parent_model_id: ::prost::alloc::string::String,
    /// Name of the parent Model. Empty if it does not exist.
    #[prost(string, tag = "4")]
    pub parent_model_display_name: ::prost::alloc::string::String,
    /// ID of the parent View. Empty if it does not exist.
    #[prost(string, tag = "5")]
    pub parent_view_id: ::prost::alloc::string::String,
    /// Name of the parent View. Empty if it does not exist.
    #[prost(string, tag = "6")]
    pub parent_view_display_name: ::prost::alloc::string::String,
}
/// Specification that applies to
/// all entries that are part of `CLOUD_BIGTABLE` system
/// (user_specified_type)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudBigtableSystemSpec {
    /// Display name of the Instance. This is user specified and different from
    /// the resource name.
    #[prost(string, tag = "1")]
    pub instance_display_name: ::prost::alloc::string::String,
}
/// Specification that applies to Instance
/// entries that are part of `CLOUD_BIGTABLE` system.
/// (user_specified_type)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudBigtableInstanceSpec {
    /// The list of clusters for the Instance.
    #[prost(message, repeated, tag = "1")]
    pub cloud_bigtable_cluster_specs: ::prost::alloc::vec::Vec<
        cloud_bigtable_instance_spec::CloudBigtableClusterSpec,
    >,
}
/// Nested message and enum types in `CloudBigtableInstanceSpec`.
pub mod cloud_bigtable_instance_spec {
    /// Spec that applies to clusters of an Instance of Cloud Bigtable.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CloudBigtableClusterSpec {
        /// Name of the cluster.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// Location of the cluster, typically a Cloud zone.
        #[prost(string, tag = "2")]
        pub location: ::prost::alloc::string::String,
        /// Type of the resource. For a cluster this would be "CLUSTER".
        #[prost(string, tag = "3")]
        pub r#type: ::prost::alloc::string::String,
        /// A link back to the parent resource, in this case Instance.
        #[prost(string, tag = "4")]
        pub linked_resource: ::prost::alloc::string::String,
    }
}
/// Specification that applies to a Service resource. Valid only
/// for entries with the `SERVICE` type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceSpec {
    /// System spec
    #[prost(oneof = "service_spec::SystemSpec", tags = "1")]
    pub system_spec: ::core::option::Option<service_spec::SystemSpec>,
}
/// Nested message and enum types in `ServiceSpec`.
pub mod service_spec {
    /// System spec
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SystemSpec {
        /// Specification that applies to Instance entries of `CLOUD_BIGTABLE`
        /// system.
        #[prost(message, tag = "1")]
        CloudBigtableInstanceSpec(super::CloudBigtableInstanceSpec),
    }
}
/// Detail description of the source information of a Vertex model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VertexModelSourceInfo {
    /// Type of the model source.
    #[prost(enumeration = "vertex_model_source_info::ModelSourceType", tag = "1")]
    pub source_type: i32,
    /// If this Model is copy of another Model. If true then
    /// [source_type][google.cloud.datacatalog.v1.VertexModelSourceInfo.source_type]
    /// pertains to the original.
    #[prost(bool, tag = "2")]
    pub copy: bool,
}
/// Nested message and enum types in `VertexModelSourceInfo`.
pub mod vertex_model_source_info {
    /// Source of the model.
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
    pub enum ModelSourceType {
        /// Should not be used.
        Unspecified = 0,
        /// The Model is uploaded by automl training pipeline.
        Automl = 1,
        /// The Model is uploaded by user or custom training pipeline.
        Custom = 2,
        /// The Model is registered and sync'ed from BigQuery ML.
        Bqml = 3,
        /// The Model is saved or tuned from Model Garden.
        ModelGarden = 4,
    }
    impl ModelSourceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ModelSourceType::Unspecified => "MODEL_SOURCE_TYPE_UNSPECIFIED",
                ModelSourceType::Automl => "AUTOML",
                ModelSourceType::Custom => "CUSTOM",
                ModelSourceType::Bqml => "BQML",
                ModelSourceType::ModelGarden => "MODEL_GARDEN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MODEL_SOURCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "AUTOML" => Some(Self::Automl),
                "CUSTOM" => Some(Self::Custom),
                "BQML" => Some(Self::Bqml),
                "MODEL_GARDEN" => Some(Self::ModelGarden),
                _ => None,
            }
        }
    }
}
/// Specification for vertex model resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VertexModelSpec {
    /// The version ID of the model.
    #[prost(string, tag = "1")]
    pub version_id: ::prost::alloc::string::String,
    /// User provided version aliases so that a model version can be referenced via
    /// alias
    #[prost(string, repeated, tag = "2")]
    pub version_aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The description of this version.
    #[prost(string, tag = "3")]
    pub version_description: ::prost::alloc::string::String,
    /// Source of a Vertex model.
    #[prost(message, optional, tag = "4")]
    pub vertex_model_source_info: ::core::option::Option<VertexModelSourceInfo>,
    /// URI of the Docker image to be used as the custom container for serving
    /// predictions.
    #[prost(string, tag = "5")]
    pub container_image_uri: ::prost::alloc::string::String,
}
/// Specification for vertex dataset resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VertexDatasetSpec {
    /// The number of DataItems in this Dataset. Only apply for non-structured
    /// Dataset.
    #[prost(int64, tag = "1")]
    pub data_item_count: i64,
    /// Type of the dataset.
    #[prost(enumeration = "vertex_dataset_spec::DataType", tag = "2")]
    pub data_type: i32,
}
/// Nested message and enum types in `VertexDatasetSpec`.
pub mod vertex_dataset_spec {
    /// Type of data stored in the dataset.
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
    pub enum DataType {
        /// Should not be used.
        Unspecified = 0,
        /// Structured data dataset.
        Table = 1,
        /// Image dataset which supports ImageClassification, ImageObjectDetection
        /// and ImageSegmentation problems.
        Image = 2,
        /// Document dataset which supports TextClassification, TextExtraction and
        /// TextSentiment problems.
        Text = 3,
        /// Video dataset which supports VideoClassification, VideoObjectTracking and
        /// VideoActionRecognition problems.
        Video = 4,
        /// Conversation dataset which supports conversation problems.
        Conversation = 5,
        /// TimeSeries dataset.
        TimeSeries = 6,
        /// Document dataset which supports DocumentAnnotation problems.
        Document = 7,
        /// TextToSpeech dataset which supports TextToSpeech problems.
        TextToSpeech = 8,
        /// Translation dataset which supports Translation problems.
        Translation = 9,
        /// Store Vision dataset which is used for HITL integration.
        StoreVision = 10,
        /// Enterprise Knowledge Graph dataset which is used for HITL labeling
        /// integration.
        EnterpriseKnowledgeGraph = 11,
        /// Text prompt dataset which supports Large Language Models.
        TextPrompt = 12,
    }
    impl DataType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DataType::Unspecified => "DATA_TYPE_UNSPECIFIED",
                DataType::Table => "TABLE",
                DataType::Image => "IMAGE",
                DataType::Text => "TEXT",
                DataType::Video => "VIDEO",
                DataType::Conversation => "CONVERSATION",
                DataType::TimeSeries => "TIME_SERIES",
                DataType::Document => "DOCUMENT",
                DataType::TextToSpeech => "TEXT_TO_SPEECH",
                DataType::Translation => "TRANSLATION",
                DataType::StoreVision => "STORE_VISION",
                DataType::EnterpriseKnowledgeGraph => "ENTERPRISE_KNOWLEDGE_GRAPH",
                DataType::TextPrompt => "TEXT_PROMPT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATA_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "TABLE" => Some(Self::Table),
                "IMAGE" => Some(Self::Image),
                "TEXT" => Some(Self::Text),
                "VIDEO" => Some(Self::Video),
                "CONVERSATION" => Some(Self::Conversation),
                "TIME_SERIES" => Some(Self::TimeSeries),
                "DOCUMENT" => Some(Self::Document),
                "TEXT_TO_SPEECH" => Some(Self::TextToSpeech),
                "TRANSLATION" => Some(Self::Translation),
                "STORE_VISION" => Some(Self::StoreVision),
                "ENTERPRISE_KNOWLEDGE_GRAPH" => Some(Self::EnterpriseKnowledgeGraph),
                "TEXT_PROMPT" => Some(Self::TextPrompt),
                _ => None,
            }
        }
    }
}
/// Specification that applies to a model. Valid only for
/// entries with the `MODEL` type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelSpec {
    /// System spec
    #[prost(oneof = "model_spec::SystemSpec", tags = "1")]
    pub system_spec: ::core::option::Option<model_spec::SystemSpec>,
}
/// Nested message and enum types in `ModelSpec`.
pub mod model_spec {
    /// System spec
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SystemSpec {
        /// Specification for vertex model resources.
        #[prost(message, tag = "1")]
        VertexModelSpec(super::VertexModelSpec),
    }
}
/// Business Context of the entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusinessContext {
    /// Entry overview fields for rich text descriptions of entries.
    #[prost(message, optional, tag = "1")]
    pub entry_overview: ::core::option::Option<EntryOverview>,
    /// Contact people for the entry.
    #[prost(message, optional, tag = "2")]
    pub contacts: ::core::option::Option<Contacts>,
}
/// Entry overview fields for rich text descriptions of entries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntryOverview {
    /// Entry overview with support for rich text.
    ///
    /// The overview must only contain Unicode characters, and should be
    /// formatted using HTML.
    /// The maximum length is 10 MiB as this value holds HTML descriptions
    /// including encoded images. The maximum length of the text without images
    /// is 100 KiB.
    #[prost(string, tag = "1")]
    pub overview: ::prost::alloc::string::String,
}
/// Contact people for the entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contacts {
    /// The list of contact people for the entry.
    #[prost(message, repeated, tag = "1")]
    pub people: ::prost::alloc::vec::Vec<contacts::Person>,
}
/// Nested message and enum types in `Contacts`.
pub mod contacts {
    /// A contact person for the entry.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Person {
        /// Designation of the person, for example, Data Steward.
        #[prost(string, tag = "1")]
        pub designation: ::prost::alloc::string::String,
        /// Email of the person in the format of `john.doe@xyz`,
        /// `<john.doe@xyz>`, or `John Doe<john.doe@xyz>`.
        #[prost(string, tag = "2")]
        pub email: ::prost::alloc::string::String,
    }
}
/// Entry group metadata.
///
/// An `EntryGroup` resource represents a logical grouping of zero or more
/// Data Catalog [Entry][google.cloud.datacatalog.v1.Entry] resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntryGroup {
    /// The resource name of the entry group in URL format.
    ///
    /// Note: The entry group itself and its child resources might not be
    /// stored in the location specified in its name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A short name to identify the entry group, for example,
    /// "analytics data - jan 2011". Default value is an empty string.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Entry group description. Can consist of several sentences or
    /// paragraphs that describe the entry group contents.
    /// Default value is an empty string.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Timestamps of the entry group. Default value is empty.
    #[prost(message, optional, tag = "4")]
    pub data_catalog_timestamps: ::core::option::Option<SystemTimestamps>,
}
/// Request message for
/// [CreateTagTemplate][google.cloud.datacatalog.v1.DataCatalog.CreateTagTemplate].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagTemplateRequest {
    /// Required. The name of the project and the template location
    /// [region](<https://cloud.google.com/data-catalog/docs/concepts/regions>).
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the tag template to create.
    ///
    /// The ID must contain only lowercase letters (a-z), numbers (0-9),
    /// or underscores (_), and must start with a letter or underscore.
    /// The maximum size is 64 bytes when encoded in UTF-8.
    #[prost(string, tag = "3")]
    pub tag_template_id: ::prost::alloc::string::String,
    /// Required. The tag template to create.
    #[prost(message, optional, tag = "2")]
    pub tag_template: ::core::option::Option<TagTemplate>,
}
/// Request message for
/// [GetTagTemplate][google.cloud.datacatalog.v1.DataCatalog.GetTagTemplate].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTagTemplateRequest {
    /// Required. The name of the tag template to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [UpdateTagTemplate][google.cloud.datacatalog.v1.DataCatalog.UpdateTagTemplate].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagTemplateRequest {
    /// Required. The template to update. The `name` field must be set.
    #[prost(message, optional, tag = "1")]
    pub tag_template: ::core::option::Option<TagTemplate>,
    /// Names of fields whose values to overwrite on a tag template. Currently,
    /// only `display_name` and `is_publicly_readable` can be overwritten.
    ///
    /// If this parameter is absent or empty, all modifiable fields
    /// are overwritten. If such fields are non-required and omitted in the
    /// request body, their values are emptied.
    ///
    /// Note: Updating the `is_publicly_readable` field may require up to 12
    /// hours to take effect in search results.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [DeleteTagTemplate][google.cloud.datacatalog.v1.DataCatalog.DeleteTagTemplate].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagTemplateRequest {
    /// Required. The name of the tag template to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. If true, deletes all tags that use this template.
    ///
    /// Currently, `true` is the only supported value.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for
/// [CreateTag][google.cloud.datacatalog.v1.DataCatalog.CreateTag].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagRequest {
    /// Required. The name of the resource to attach this tag to.
    ///
    /// Tags can be attached to entries or entry groups. An entry can have up to
    /// 1000 attached tags.
    ///
    /// Note: The tag and its child resources might not be stored in
    /// the location specified in its name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The tag to create.
    #[prost(message, optional, tag = "2")]
    pub tag: ::core::option::Option<Tag>,
}
/// Request message for
/// [UpdateTag][google.cloud.datacatalog.v1.DataCatalog.UpdateTag].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagRequest {
    /// Required. The updated tag. The "name" field must be set.
    #[prost(message, optional, tag = "1")]
    pub tag: ::core::option::Option<Tag>,
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
/// [DeleteTag][google.cloud.datacatalog.v1.DataCatalog.DeleteTag].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagRequest {
    /// Required. The name of the tag to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [CreateTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.CreateTagTemplateField].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagTemplateFieldRequest {
    /// Required. The name of the project and the template location
    /// [region](<https://cloud.google.com/data-catalog/docs/concepts/regions>).
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the tag template field to create.
    ///
    /// Note: Adding a required field to an existing template is *not* allowed.
    ///
    /// Field IDs can contain letters (both uppercase and lowercase), numbers
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
/// [UpdateTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.UpdateTagTemplateField].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagTemplateFieldRequest {
    /// Required. The name of the tag template field.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The template to update.
    #[prost(message, optional, tag = "2")]
    pub tag_template_field: ::core::option::Option<TagTemplateField>,
    /// Optional. Names of fields whose values to overwrite on an individual field
    /// of a tag template. The following fields are modifiable:
    ///
    /// * `display_name`
    /// * `type.enum_type`
    /// * `is_required`
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
/// [RenameTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.RenameTagTemplateField].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTagTemplateFieldRequest {
    /// Required. The name of the tag template field.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new ID of this tag template field. For example,
    /// `my_new_field`.
    #[prost(string, tag = "2")]
    pub new_tag_template_field_id: ::prost::alloc::string::String,
}
/// Request message for
/// [RenameTagTemplateFieldEnumValue][google.cloud.datacatalog.v1.DataCatalog.RenameTagTemplateFieldEnumValue].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTagTemplateFieldEnumValueRequest {
    /// Required. The name of the enum field value.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new display name of the enum value. For example,
    /// `my_new_enum_value`.
    #[prost(string, tag = "2")]
    pub new_enum_value_display_name: ::prost::alloc::string::String,
}
/// Request message for
/// [DeleteTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.DeleteTagTemplateField].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagTemplateFieldRequest {
    /// Required. The name of the tag template field to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. If true, deletes this field from any tags that use it.
    ///
    /// Currently, `true` is the only supported value.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for
/// [ListTags][google.cloud.datacatalog.v1.DataCatalog.ListTags].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsRequest {
    /// Required. The name of the Data Catalog resource to list the tags of.
    ///
    /// The resource can be an [Entry][google.cloud.datacatalog.v1.Entry]
    /// or an [EntryGroup][google.cloud.datacatalog.v1.EntryGroup]
    /// (without `/entries/{entries}` at the end).
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of tags to return. Default is 10. Maximum limit is 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Pagination token that specifies the next page to return. If empty, the
    /// first page is returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [ListTags][google.cloud.datacatalog.v1.DataCatalog.ListTags].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsResponse {
    /// [Tag][google.cloud.datacatalog.v1.Tag] details.
    #[prost(message, repeated, tag = "1")]
    pub tags: ::prost::alloc::vec::Vec<Tag>,
    /// Pagination token of the next results page. Empty if there are
    /// no more items in results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [ReconcileTags][google.cloud.datacatalog.v1.DataCatalog.ReconcileTags].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconcileTagsRequest {
    /// Required. Name of [Entry][google.cloud.datacatalog.v1.Entry] to be tagged.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The name of the tag template, which is used for reconciliation.
    #[prost(string, tag = "2")]
    pub tag_template: ::prost::alloc::string::String,
    /// If set to `true`, deletes entry tags related to a tag template
    /// not listed in the tags source from an entry. If set to `false`,
    /// unlisted tags are retained.
    #[prost(bool, tag = "3")]
    pub force_delete_missing: bool,
    /// A list of tags to apply to an entry. A tag can specify a
    /// tag template, which must be the template specified in the
    /// `ReconcileTagsRequest`.
    /// The sole entry and each of its columns must be mentioned at most once.
    #[prost(message, repeated, tag = "4")]
    pub tags: ::prost::alloc::vec::Vec<Tag>,
}
/// [Long-running operation][google.longrunning.Operation]
/// response message returned by
/// [ReconcileTags][google.cloud.datacatalog.v1.DataCatalog.ReconcileTags].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconcileTagsResponse {
    /// Number of tags created in the request.
    #[prost(int64, tag = "1")]
    pub created_tags_count: i64,
    /// Number of tags updated in the request.
    #[prost(int64, tag = "2")]
    pub updated_tags_count: i64,
    /// Number of tags deleted in the request.
    #[prost(int64, tag = "3")]
    pub deleted_tags_count: i64,
}
/// [Long-running operation][google.longrunning.Operation]
/// metadata message returned by the
/// [ReconcileTags][google.cloud.datacatalog.v1.DataCatalog.ReconcileTags].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconcileTagsMetadata {
    /// State of the reconciliation operation.
    #[prost(enumeration = "reconcile_tags_metadata::ReconciliationState", tag = "1")]
    pub state: i32,
    /// Maps the name of each tagged column (or empty string for a
    /// sole entry) to tagging operation [status][google.rpc.Status].
    #[prost(btree_map = "string, message", tag = "2")]
    pub errors: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        super::super::super::rpc::Status,
    >,
}
/// Nested message and enum types in `ReconcileTagsMetadata`.
pub mod reconcile_tags_metadata {
    /// Enum holding possible states of the reconciliation operation.
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
    pub enum ReconciliationState {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The reconciliation has been queued and awaits for execution.
        ReconciliationQueued = 1,
        /// The reconciliation is in progress.
        ReconciliationInProgress = 2,
        /// The reconciliation has been finished.
        ReconciliationDone = 3,
    }
    impl ReconciliationState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReconciliationState::Unspecified => "RECONCILIATION_STATE_UNSPECIFIED",
                ReconciliationState::ReconciliationQueued => "RECONCILIATION_QUEUED",
                ReconciliationState::ReconciliationInProgress => {
                    "RECONCILIATION_IN_PROGRESS"
                }
                ReconciliationState::ReconciliationDone => "RECONCILIATION_DONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RECONCILIATION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "RECONCILIATION_QUEUED" => Some(Self::ReconciliationQueued),
                "RECONCILIATION_IN_PROGRESS" => Some(Self::ReconciliationInProgress),
                "RECONCILIATION_DONE" => Some(Self::ReconciliationDone),
                _ => None,
            }
        }
    }
}
/// Request message for
/// [ListEntries][google.cloud.datacatalog.v1.DataCatalog.ListEntries].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntriesRequest {
    /// Required. The name of the entry group that contains the entries to list.
    ///
    /// Can be provided in URL format.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. Default is 10. Maximum limit is
    /// 1000. Throws an invalid argument if `page_size` is more than 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Pagination token that specifies the next page to return. If empty, the
    /// first page is returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The fields to return for each entry. If empty or omitted, all
    /// fields are returned.
    ///
    /// For example, to return a list of entries with only the `name` field,
    /// set `read_mask` to only one path with the `name` value.
    #[prost(message, optional, tag = "4")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for
/// [ListEntries][google.cloud.datacatalog.v1.DataCatalog.ListEntries].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntriesResponse {
    /// Entry details.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<Entry>,
    /// Pagination token of the next results page. Empty if there are no more items
    /// in results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [StarEntry][google.cloud.datacatalog.v1.DataCatalog.StarEntry].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StarEntryRequest {
    /// Required. The name of the entry to mark as starred.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for
/// [StarEntry][google.cloud.datacatalog.v1.DataCatalog.StarEntry].
/// Empty for now
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StarEntryResponse {}
/// Request message for
/// [UnstarEntry][google.cloud.datacatalog.v1.DataCatalog.UnstarEntry].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnstarEntryRequest {
    /// Required. The name of the entry to mark as **not** starred.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for
/// [UnstarEntry][google.cloud.datacatalog.v1.DataCatalog.UnstarEntry].
/// Empty for now
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnstarEntryResponse {}
/// Request message for
/// [ImportEntries][google.cloud.datacatalog.v1.DataCatalog.ImportEntries]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportEntriesRequest {
    /// Required. Target entry group for ingested entries.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. (Optional) Dataplex task job id, if specified will be used as
    /// part of ImportEntries LRO ID
    #[prost(string, tag = "3")]
    pub job_id: ::prost::alloc::string::String,
    /// Source of imported entries, e.g. dump stored in a Cloud Storage
    #[prost(oneof = "import_entries_request::Source", tags = "2")]
    pub source: ::core::option::Option<import_entries_request::Source>,
}
/// Nested message and enum types in `ImportEntriesRequest`.
pub mod import_entries_request {
    /// Source of imported entries, e.g. dump stored in a Cloud Storage
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Path to a Cloud Storage bucket that contains a dump ready for ingestion.
        #[prost(string, tag = "2")]
        GcsBucketPath(::prost::alloc::string::String),
    }
}
/// Response message for [long-running operation][google.longrunning.Operation]
/// returned by the
/// [ImportEntries][google.cloud.datacatalog.v1.DataCatalog.ImportEntries].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportEntriesResponse {
    /// Cumulative number of entries created and entries updated as a result of
    /// import operation.
    #[prost(int64, optional, tag = "5")]
    pub upserted_entries_count: ::core::option::Option<i64>,
    /// Number of entries deleted as a result of import operation.
    #[prost(int64, optional, tag = "6")]
    pub deleted_entries_count: ::core::option::Option<i64>,
}
/// Metadata message for [long-running operation][google.longrunning.Operation]
/// returned by the
/// [ImportEntries][google.cloud.datacatalog.v1.DataCatalog.ImportEntries].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportEntriesMetadata {
    /// State of the import operation.
    #[prost(enumeration = "import_entries_metadata::ImportState", tag = "1")]
    pub state: i32,
    /// Partial errors that are encountered during the ImportEntries operation.
    /// There is no guarantee that all the encountered errors are reported.
    /// However, if no errors are reported, it means that no errors were
    /// encountered.
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// Nested message and enum types in `ImportEntriesMetadata`.
pub mod import_entries_metadata {
    /// Enum holding possible states of the import operation.
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
    pub enum ImportState {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The dump with entries has been queued for import.
        ImportQueued = 1,
        /// The import of entries is in progress.
        ImportInProgress = 2,
        /// The import of entries has been finished.
        ImportDone = 3,
        /// The import of entries has been abandoned in favor of a newer request.
        ImportObsolete = 4,
    }
    impl ImportState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ImportState::Unspecified => "IMPORT_STATE_UNSPECIFIED",
                ImportState::ImportQueued => "IMPORT_QUEUED",
                ImportState::ImportInProgress => "IMPORT_IN_PROGRESS",
                ImportState::ImportDone => "IMPORT_DONE",
                ImportState::ImportObsolete => "IMPORT_OBSOLETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IMPORT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "IMPORT_QUEUED" => Some(Self::ImportQueued),
                "IMPORT_IN_PROGRESS" => Some(Self::ImportInProgress),
                "IMPORT_DONE" => Some(Self::ImportDone),
                "IMPORT_OBSOLETE" => Some(Self::ImportObsolete),
                _ => None,
            }
        }
    }
}
/// Request message for
/// [ModifyEntryOverview][google.cloud.datacatalog.v1.DataCatalog.ModifyEntryOverview].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyEntryOverviewRequest {
    /// Required. The full resource name of the entry.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new value for the Entry Overview.
    #[prost(message, optional, tag = "2")]
    pub entry_overview: ::core::option::Option<EntryOverview>,
}
/// Request message for
/// [ModifyEntryContacts][google.cloud.datacatalog.v1.DataCatalog.ModifyEntryContacts].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyEntryContactsRequest {
    /// Required. The full resource name of the entry.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new value for the Contacts.
    #[prost(message, optional, tag = "2")]
    pub contacts: ::core::option::Option<Contacts>,
}
/// Metadata automatically ingested from Google Cloud resources like BigQuery
/// tables or Pub/Sub topics always uses enum values from `EntryType` as the type
/// of entry.
///
/// Other sources of metadata like Hive or Oracle databases can identify the type
/// by either using one of the enum values from `EntryType` (for example,
/// `FILESET` for a Cloud Storage fileset) or specifying a custom value using
/// the [`Entry`](#resource:-entry) field `user_specified_type`. For more
/// information, see
/// [Surface files from Cloud Storage with fileset
/// entries](/data-catalog/docs/how-to/filesets) or [Create custom entries for
/// your data sources](/data-catalog/docs/how-to/custom-entries).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EntryType {
    /// Default unknown type.
    Unspecified = 0,
    /// The entry type that has a GoogleSQL schema, including
    /// logical views.
    Table = 2,
    /// The type of models.
    ///
    /// For more information, see [Supported models in BigQuery
    /// ML](/bigquery/docs/bqml-introduction#supported_models).
    Model = 5,
    /// An entry type for streaming entries. For example, a Pub/Sub topic.
    DataStream = 3,
    /// An entry type for a set of files or objects. For example, a
    /// Cloud Storage fileset.
    Fileset = 4,
    /// A group of servers that work together. For example, a Kafka cluster.
    Cluster = 6,
    /// A database.
    Database = 7,
    /// Connection to a data source. For example, a BigQuery
    /// connection.
    DataSourceConnection = 8,
    /// Routine, for example, a BigQuery routine.
    Routine = 9,
    /// A Dataplex lake.
    Lake = 10,
    /// A Dataplex zone.
    Zone = 11,
    /// A service, for example, a Dataproc Metastore service.
    Service = 14,
    /// Schema within a relational database.
    DatabaseSchema = 15,
    /// A Dashboard, for example from Looker.
    Dashboard = 16,
    /// A Looker Explore.
    ///
    /// For more information, see \[Looker Explore API\]
    /// (<https://developers.looker.com/api/explorer/4.0/methods/LookmlModel/lookml_model_explore>).
    Explore = 17,
    /// A Looker Look.
    ///
    /// For more information, see \[Looker Look API\]
    /// (<https://developers.looker.com/api/explorer/4.0/methods/Look>).
    Look = 18,
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
            EntryType::Cluster => "CLUSTER",
            EntryType::Database => "DATABASE",
            EntryType::DataSourceConnection => "DATA_SOURCE_CONNECTION",
            EntryType::Routine => "ROUTINE",
            EntryType::Lake => "LAKE",
            EntryType::Zone => "ZONE",
            EntryType::Service => "SERVICE",
            EntryType::DatabaseSchema => "DATABASE_SCHEMA",
            EntryType::Dashboard => "DASHBOARD",
            EntryType::Explore => "EXPLORE",
            EntryType::Look => "LOOK",
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
            "CLUSTER" => Some(Self::Cluster),
            "DATABASE" => Some(Self::Database),
            "DATA_SOURCE_CONNECTION" => Some(Self::DataSourceConnection),
            "ROUTINE" => Some(Self::Routine),
            "LAKE" => Some(Self::Lake),
            "ZONE" => Some(Self::Zone),
            "SERVICE" => Some(Self::Service),
            "DATABASE_SCHEMA" => Some(Self::DatabaseSchema),
            "DASHBOARD" => Some(Self::Dashboard),
            "EXPLORE" => Some(Self::Explore),
            "LOOK" => Some(Self::Look),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod data_catalog_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Data Catalog API service allows you to discover, understand, and manage
    /// your data.
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
        /// Searches Data Catalog for multiple resources like entries and tags that
        /// match a query.
        ///
        /// This is a [Custom Method]
        /// (https://cloud.google.com/apis/design/custom_methods) that doesn't return
        /// all information on a resource, only its ID and high level fields. To get
        /// more information, you can subsequently call specific get methods.
        ///
        /// Note: Data Catalog search queries don't guarantee full recall. Results
        /// that match your query might not be returned, even in subsequent
        /// result pages. Additionally, returned (and not returned) results can vary
        /// if you repeat search queries.
        ///
        /// For more information, see [Data Catalog search syntax]
        /// (https://cloud.google.com/data-catalog/docs/how-to/search-reference).
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
                "/google.cloud.datacatalog.v1.DataCatalog/SearchCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "SearchCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an entry group.
        ///
        /// An entry group contains logically related entries together with [Cloud
        /// Identity and Access Management](/data-catalog/docs/concepts/iam) policies.
        /// These policies specify users who can create, edit, and view entries
        /// within entry groups.
        ///
        /// Data Catalog automatically creates entry groups with names that start with
        /// the `@` symbol for the following resources:
        ///
        /// * BigQuery entries (`@bigquery`)
        /// * Pub/Sub topics (`@pubsub`)
        /// * Dataproc Metastore services (`@dataproc_metastore_{SERVICE_NAME_HASH}`)
        ///
        /// You can create your own entry groups for Cloud Storage fileset entries
        /// and custom entries together with the corresponding IAM policies.
        /// User-created entry groups can't contain the `@` symbol, it is reserved
        /// for automatically created groups.
        ///
        /// Entry groups, like entries, can be searched.
        ///
        /// A maximum of 10,000 entry groups may be created per organization across all
        /// locations.
        ///
        /// You must enable the Data Catalog API in the project identified by
        /// the `parent` parameter. For more information, see [Data Catalog resource
        /// project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
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
                "/google.cloud.datacatalog.v1.DataCatalog/CreateEntryGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "CreateEntryGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets an entry group.
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
                "/google.cloud.datacatalog.v1.DataCatalog/GetEntryGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "GetEntryGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an entry group.
        ///
        /// You must enable the Data Catalog API in the project identified by
        /// the `entry_group.name` parameter. For more information, see [Data Catalog
        /// resource
        /// project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
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
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateEntryGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "UpdateEntryGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an entry group.
        ///
        /// You must enable the Data Catalog API in the project
        /// identified by the `name` parameter. For more information, see [Data Catalog
        /// resource
        /// project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
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
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteEntryGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
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
                "/google.cloud.datacatalog.v1.DataCatalog/ListEntryGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "ListEntryGroups",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an entry.
        ///
        /// You can create entries only with 'FILESET', 'CLUSTER', 'DATA_STREAM',
        /// or custom types. Data Catalog automatically creates entries with other
        /// types during metadata ingestion from integrated systems.
        ///
        /// You must enable the Data Catalog API in the project identified by
        /// the `parent` parameter. For more information, see [Data Catalog resource
        /// project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
        ///
        /// An entry group can have a maximum of 100,000 entries.
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
                "/google.cloud.datacatalog.v1.DataCatalog/CreateEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "CreateEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing entry.
        ///
        /// You must enable the Data Catalog API in the project identified by
        /// the `entry.name` parameter. For more information, see [Data Catalog
        /// resource
        /// project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
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
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "UpdateEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an existing entry.
        ///
        /// You can delete only the entries created by the
        /// [CreateEntry][google.cloud.datacatalog.v1.DataCatalog.CreateEntry]
        /// method.
        ///
        /// You must enable the Data Catalog API in the project identified by
        /// the `name` parameter. For more information, see [Data Catalog
        /// resource
        /// project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
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
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
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
                "/google.cloud.datacatalog.v1.DataCatalog/GetEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "GetEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets an entry by its target resource name.
        ///
        /// The resource name comes from the source Google Cloud Platform service.
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
                "/google.cloud.datacatalog.v1.DataCatalog/LookupEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "LookupEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists entries.
        ///
        /// Note: Currently, this method can list only custom entries.
        /// To get a list of both custom and automatically created entries, use
        /// [SearchCatalog][google.cloud.datacatalog.v1.DataCatalog.SearchCatalog].
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
                "/google.cloud.datacatalog.v1.DataCatalog/ListEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "ListEntries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Modifies entry overview, part of the business context of an
        /// [Entry][google.cloud.datacatalog.v1.Entry].
        ///
        /// To call this method, you must have the `datacatalog.entries.updateOverview`
        /// IAM permission on the corresponding project.
        pub async fn modify_entry_overview(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyEntryOverviewRequest>,
        ) -> std::result::Result<tonic::Response<super::EntryOverview>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/ModifyEntryOverview",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "ModifyEntryOverview",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Modifies contacts, part of the business context of an
        /// [Entry][google.cloud.datacatalog.v1.Entry].
        ///
        /// To call this method, you must have the `datacatalog.entries.updateContacts`
        /// IAM permission on the corresponding project.
        pub async fn modify_entry_contacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyEntryContactsRequest>,
        ) -> std::result::Result<tonic::Response<super::Contacts>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/ModifyEntryContacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "ModifyEntryContacts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a tag template.
        ///
        /// You must enable the Data Catalog API in the project identified by the
        /// `parent` parameter.
        /// For more information, see [Data Catalog resource project]
        /// (https://cloud.google.com/data-catalog/docs/concepts/resource-project).
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
                "/google.cloud.datacatalog.v1.DataCatalog/CreateTagTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
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
                "/google.cloud.datacatalog.v1.DataCatalog/GetTagTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "GetTagTemplate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a tag template.
        ///
        /// You can't update template fields with this method. These fields are
        /// separate resources with their own create, update, and delete methods.
        ///
        /// You must enable the Data Catalog API in the project identified by
        /// the `tag_template.name` parameter. For more information, see [Data Catalog
        /// resource
        /// project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
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
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateTagTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "UpdateTagTemplate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a tag template and all tags that use it.
        ///
        /// You must enable the Data Catalog API in the project identified by
        /// the `name` parameter. For more information, see [Data Catalog resource
        /// project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
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
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteTagTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "DeleteTagTemplate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a field in a tag template.
        ///
        /// You must enable the Data Catalog API in the project identified by
        /// the `parent` parameter. For more information, see [Data Catalog resource
        /// project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
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
                "/google.cloud.datacatalog.v1.DataCatalog/CreateTagTemplateField",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "CreateTagTemplateField",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a field in a tag template.
        ///
        /// You can't update the field type with this method.
        ///
        /// You must enable the Data Catalog API in the project
        /// identified by the `name` parameter. For more information, see [Data Catalog
        /// resource
        /// project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
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
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateTagTemplateField",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "UpdateTagTemplateField",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Renames a field in a tag template.
        ///
        /// You must enable the Data Catalog API in the project identified by the
        /// `name` parameter. For more information, see [Data Catalog resource project]
        /// (https://cloud.google.com/data-catalog/docs/concepts/resource-project).
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
                "/google.cloud.datacatalog.v1.DataCatalog/RenameTagTemplateField",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "RenameTagTemplateField",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Renames an enum value in a tag template.
        ///
        /// Within a single enum field, enum values must be unique.
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
                "/google.cloud.datacatalog.v1.DataCatalog/RenameTagTemplateFieldEnumValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "RenameTagTemplateFieldEnumValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a field in a tag template and all uses of this field from the tags
        /// based on this template.
        ///
        /// You must enable the Data Catalog API in the project identified by
        /// the `name` parameter. For more information, see [Data Catalog resource
        /// project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).
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
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteTagTemplateField",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "DeleteTagTemplateField",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a tag and assigns it to:
        ///
        /// * An [Entry][google.cloud.datacatalog.v1.Entry] if the method name is
        ///   `projects.locations.entryGroups.entries.tags.create`.
        /// * Or [EntryGroup][google.cloud.datacatalog.v1.EntryGroup]if the method
        ///   name is `projects.locations.entryGroups.tags.create`.
        ///
        /// Note: The project identified by the `parent` parameter for the [tag]
        /// (https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries.tags/create#path-parameters)
        /// and the [tag template]
        /// (https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.tagTemplates/create#path-parameters)
        /// used to create the tag must be in the same organization.
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
                "/google.cloud.datacatalog.v1.DataCatalog/CreateTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
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
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
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
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "DeleteTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists tags assigned to an [Entry][google.cloud.datacatalog.v1.Entry].
        /// The [columns][google.cloud.datacatalog.v1.Tag.column] in the response are
        /// lowercased.
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
                "/google.cloud.datacatalog.v1.DataCatalog/ListTags",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "ListTags",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// `ReconcileTags` creates or updates a list of tags on the entry.
        /// If the
        /// [ReconcileTagsRequest.force_delete_missing][google.cloud.datacatalog.v1.ReconcileTagsRequest.force_delete_missing]
        /// parameter is set, the operation deletes tags not included in the input tag
        /// list.
        ///
        /// `ReconcileTags` returns a [long-running operation]
        /// [google.longrunning.Operation] resource that can be queried with
        /// [Operations.GetOperation][google.longrunning.Operations.GetOperation]
        /// to return [ReconcileTagsMetadata]
        /// [google.cloud.datacatalog.v1.ReconcileTagsMetadata] and
        /// a [ReconcileTagsResponse]
        /// [google.cloud.datacatalog.v1.ReconcileTagsResponse] message.
        pub async fn reconcile_tags(
            &mut self,
            request: impl tonic::IntoRequest<super::ReconcileTagsRequest>,
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
                "/google.cloud.datacatalog.v1.DataCatalog/ReconcileTags",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "ReconcileTags",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Marks an [Entry][google.cloud.datacatalog.v1.Entry] as starred by
        /// the current user. Starring information is private to each user.
        pub async fn star_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::StarEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StarEntryResponse>,
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
                "/google.cloud.datacatalog.v1.DataCatalog/StarEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "StarEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Marks an [Entry][google.cloud.datacatalog.v1.Entry] as NOT starred by
        /// the current user. Starring information is private to each user.
        pub async fn unstar_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::UnstarEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnstarEntryResponse>,
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
                "/google.cloud.datacatalog.v1.DataCatalog/UnstarEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "UnstarEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets an access control policy for a resource. Replaces any existing
        /// policy.
        ///
        /// Supported resources are:
        ///
        /// - Tag templates
        /// - Entry groups
        ///
        /// Note: This method sets policies only within Data Catalog and can't be
        /// used to manage policies in BigQuery, Pub/Sub, Dataproc Metastore, and any
        /// external Google Cloud Platform resources synced with the Data Catalog.
        ///
        /// To call this method, you must have the following Google IAM permissions:
        ///
        /// - `datacatalog.tagTemplates.setIamPolicy` to set policies on tag
        ///   templates.
        /// - `datacatalog.entryGroups.setIamPolicy` to set policies on entry groups.
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
                "/google.cloud.datacatalog.v1.DataCatalog/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the access control policy for a resource.
        ///
        /// May return:
        ///
        /// * A`NOT_FOUND` error if the resource doesn't exist or you don't have the
        ///   permission to view it.
        /// * An empty policy if the resource exists but doesn't have a set policy.
        ///
        /// Supported resources are:
        ///
        /// - Tag templates
        /// - Entry groups
        ///
        /// Note: This method doesn't get policies from Google Cloud Platform
        /// resources ingested into Data Catalog.
        ///
        /// To call this method, you must have the following Google IAM permissions:
        ///
        /// - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag
        ///   templates.
        /// - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups.
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
                "/google.cloud.datacatalog.v1.DataCatalog/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets your permissions on a resource.
        ///
        /// Returns an empty set of permissions if the resource doesn't exist.
        ///
        /// Supported resources are:
        ///
        /// - Tag templates
        /// - Entry groups
        ///
        /// Note: This method gets policies only within Data Catalog and can't be
        /// used to get policies from BigQuery, Pub/Sub, Dataproc Metastore, and any
        /// external Google Cloud Platform resources ingested into Data Catalog.
        ///
        /// No Google IAM permissions are required to call this method.
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
                "/google.cloud.datacatalog.v1.DataCatalog/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Imports entries from a source, such as data previously dumped into a
        /// Cloud Storage bucket, into Data Catalog. Import of entries
        /// is a sync operation that reconciles the state of the third-party system
        /// with the Data Catalog.
        ///
        /// `ImportEntries` accepts source data snapshots of a third-party system.
        /// Snapshot should be delivered as a .wire or base65-encoded .txt file
        /// containing a sequence of Protocol Buffer messages of
        /// [DumpItem][google.cloud.datacatalog.v1.DumpItem] type.
        ///
        /// `ImportEntries` returns a [long-running operation]
        /// [google.longrunning.Operation] resource that can be queried with
        /// [Operations.GetOperation][google.longrunning.Operations.GetOperation]
        /// to return
        /// [ImportEntriesMetadata][google.cloud.datacatalog.v1.ImportEntriesMetadata]
        /// and an
        /// [ImportEntriesResponse][google.cloud.datacatalog.v1.ImportEntriesResponse]
        /// message.
        pub async fn import_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportEntriesRequest>,
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
                "/google.cloud.datacatalog.v1.DataCatalog/ImportEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.DataCatalog",
                        "ImportEntries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Wrapper containing Entry and information about Tags
/// that should and should not be attached to it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaggedEntry {
    /// Optional. Tags that should be ingested into the Data Catalog.
    /// Caller should populate template name, column and fields.
    #[prost(message, repeated, tag = "2")]
    pub present_tags: ::prost::alloc::vec::Vec<Tag>,
    /// Optional. Tags that should be deleted from the Data Catalog.
    /// Caller should populate template name and column only.
    #[prost(message, repeated, tag = "3")]
    pub absent_tags: ::prost::alloc::vec::Vec<Tag>,
    /// Required. Entry to be ingested.
    #[prost(oneof = "tagged_entry::Entry", tags = "1")]
    pub entry: ::core::option::Option<tagged_entry::Entry>,
}
/// Nested message and enum types in `TaggedEntry`.
pub mod tagged_entry {
    /// Required. Entry to be ingested.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Entry {
        /// Non-encrypted Data Catalog v1 Entry.
        #[prost(message, tag = "1")]
        V1Entry(super::Entry),
    }
}
/// Wrapper for any item that can be contained in the dump.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DumpItem {
    #[prost(oneof = "dump_item::Item", tags = "1")]
    pub item: ::core::option::Option<dump_item::Item>,
}
/// Nested message and enum types in `DumpItem`.
pub mod dump_item {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        /// Entry and its tags.
        #[prost(message, tag = "1")]
        TaggedEntry(super::TaggedEntry),
    }
}
/// A taxonomy is a collection of hierarchical policy tags that classify data
/// along a common axis.
///
/// For example, a "data sensitivity" taxonomy might contain the following policy
/// tags:
///
/// ```
/// + PII
///    + Account number
///    + Age
///    + SSN
///    + Zipcode
/// + Financials
///    + Revenue
/// ```
///
/// A "data origin" taxonomy might contain the following policy tags:
///
/// ```
/// + User data
/// + Employee data
/// + Partner data
/// + Public data
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Taxonomy {
    /// Output only. Resource name of this taxonomy in URL format.
    ///
    /// Note: Policy tag manager generates unique taxonomy IDs.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. User-defined name of this taxonomy.
    ///
    /// The name can't start or end with spaces, must contain only Unicode letters,
    /// numbers, underscores, dashes, and spaces, and be at most 200 bytes long
    /// when encoded in UTF-8.
    ///
    /// The taxonomy display name must be unique within an organization.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Description of this taxonomy. If not set, defaults to empty.
    ///
    /// The description must contain only Unicode characters, tabs, newlines,
    /// carriage returns, and page breaks, and be at most 2000 bytes long when
    /// encoded in UTF-8.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Number of policy tags in this taxonomy.
    #[prost(int32, tag = "4")]
    pub policy_tag_count: i32,
    /// Output only. Creation and modification timestamps of this taxonomy.
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
    /// Defines policy types where the policy tags can be used for.
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
        /// Fine-grained access control policy that enables access control on
        /// tagged sub-resources.
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
/// Denotes one policy tag in a taxonomy, for example, SSN.
///
/// Policy tags can be defined in a hierarchy. For example:
///
/// ```
/// + Geolocation
///    + LatLong
///    + City
///    + ZipCode
/// ```
///
/// Where the "Geolocation" policy tag contains three children.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTag {
    /// Output only. Resource name of this policy tag in the URL format.
    ///
    /// The policy tag manager generates unique taxonomy IDs and policy tag IDs.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. User-defined name of this policy tag.
    ///
    /// The name can't start or end with spaces and must be unique within the
    /// parent taxonomy, contain only Unicode letters, numbers, underscores, dashes
    /// and spaces, and be at most 200 bytes long when encoded in UTF-8.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of this policy tag. If not set, defaults to empty.
    ///
    /// The description must contain only Unicode characters,
    /// tabs, newlines, carriage returns and page breaks, and be at most 2000 bytes
    /// long when encoded in UTF-8.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Resource name of this policy tag's parent policy tag. If empty, this is a
    /// top level tag. If not set, defaults to an empty string.
    ///
    /// For example, for the "LatLong" policy tag in the example above, this field
    /// contains the resource name of the "Geolocation" policy tag, and, for
    /// "Geolocation", this field is empty.
    #[prost(string, tag = "4")]
    pub parent_policy_tag: ::prost::alloc::string::String,
    /// Output only. Resource names of child policy tags of this policy tag.
    #[prost(string, repeated, tag = "5")]
    pub child_policy_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [CreateTaxonomy][google.cloud.datacatalog.v1.PolicyTagManager.CreateTaxonomy].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaxonomyRequest {
    /// Required. Resource name of the project that the taxonomy will belong to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The taxonomy to create.
    #[prost(message, optional, tag = "2")]
    pub taxonomy: ::core::option::Option<Taxonomy>,
}
/// Request message for
/// [DeleteTaxonomy][google.cloud.datacatalog.v1.PolicyTagManager.DeleteTaxonomy].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTaxonomyRequest {
    /// Required. Resource name of the taxonomy to delete.
    ///
    /// Note: All policy tags in this taxonomy are also deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [UpdateTaxonomy][google.cloud.datacatalog.v1.PolicyTagManager.UpdateTaxonomy].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaxonomyRequest {
    /// The taxonomy to update. You can update only its description, display name,
    /// and activated policy types.
    #[prost(message, optional, tag = "1")]
    pub taxonomy: ::core::option::Option<Taxonomy>,
    /// Specifies fields to update. If not set, defaults to all fields you can
    /// update.
    ///
    /// For more information, see \[FieldMask\]
    /// (<https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>).
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [ListTaxonomies][google.cloud.datacatalog.v1.PolicyTagManager.ListTaxonomies].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTaxonomiesRequest {
    /// Required. Resource name of the project to list the taxonomies of.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. Must be a value between 1 and 1000
    /// inclusively. If not set, defaults to 50.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The pagination token of the next results page. If not set,
    /// the first page is returned.
    ///
    /// The token is returned in the response to a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Supported field for filter is 'service' and value is 'dataplex'.
    /// Eg: service=dataplex.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for
/// [ListTaxonomies][google.cloud.datacatalog.v1.PolicyTagManager.ListTaxonomies].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTaxonomiesResponse {
    /// Taxonomies that the project contains.
    #[prost(message, repeated, tag = "1")]
    pub taxonomies: ::prost::alloc::vec::Vec<Taxonomy>,
    /// Pagination token of the next results page. Empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [GetTaxonomy][google.cloud.datacatalog.v1.PolicyTagManager.GetTaxonomy].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaxonomyRequest {
    /// Required. Resource name of the taxonomy to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [CreatePolicyTag][google.cloud.datacatalog.v1.PolicyTagManager.CreatePolicyTag].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicyTagRequest {
    /// Required. Resource name of the taxonomy that the policy tag will belong to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The policy tag to create.
    #[prost(message, optional, tag = "2")]
    pub policy_tag: ::core::option::Option<PolicyTag>,
}
/// Request message for
/// [DeletePolicyTag][google.cloud.datacatalog.v1.PolicyTagManager.DeletePolicyTag].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePolicyTagRequest {
    /// Required. Resource name of the policy tag to delete.
    ///
    /// Note: All of its descendant policy tags are also deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [UpdatePolicyTag][google.cloud.datacatalog.v1.PolicyTagManager.UpdatePolicyTag].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePolicyTagRequest {
    /// The policy tag to update. You can update only its description, display
    /// name, and parent policy tag fields.
    #[prost(message, optional, tag = "1")]
    pub policy_tag: ::core::option::Option<PolicyTag>,
    /// Specifies the fields to update.
    ///
    /// You can update only display name, description, and parent policy tag.
    /// If not set, defaults to all updatable fields.
    /// For more information, see \[FieldMask\]
    /// (<https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>).
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [ListPolicyTags][google.cloud.datacatalog.v1.PolicyTagManager.ListPolicyTags].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyTagsRequest {
    /// Required. Resource name of the taxonomy to list the policy tags of.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. Must be a value between 1 and 1000
    /// inclusively.
    /// If not set, defaults to 50.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The pagination token of the next results page. If not set, returns the
    /// first page.
    ///
    /// The token is returned in the response to a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [ListPolicyTags][google.cloud.datacatalog.v1.PolicyTagManager.ListPolicyTags].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyTagsResponse {
    /// The policy tags that belong to the taxonomy.
    #[prost(message, repeated, tag = "1")]
    pub policy_tags: ::prost::alloc::vec::Vec<PolicyTag>,
    /// Pagination token of the next results page. Empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [GetPolicyTag][google.cloud.datacatalog.v1.PolicyTagManager.GetPolicyTag].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyTagRequest {
    /// Required. Resource name of the policy tag.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod policy_tag_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Policy Tag Manager API service allows you to manage your policy tags and
    /// taxonomies.
    ///
    /// Policy tags are used to tag BigQuery columns and apply additional access
    /// control policies. A taxonomy is a hierarchical grouping of policy tags that
    /// classify data along a common axis.
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
        /// Creates a taxonomy in a specified project.
        ///
        /// The taxonomy is initially empty, that is, it doesn't contain policy tags.
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/CreateTaxonomy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManager",
                        "CreateTaxonomy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a taxonomy, including all policy tags in this
        /// taxonomy, their associated policies, and the policy tags references from
        /// BigQuery columns.
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/DeleteTaxonomy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManager",
                        "DeleteTaxonomy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a taxonomy, including its display name,
        /// description, and activated policy types.
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/UpdateTaxonomy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManager",
                        "UpdateTaxonomy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all taxonomies in a project in a particular location that you
        /// have a permission to view.
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/ListTaxonomies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManager",
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/GetTaxonomy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManager",
                        "GetTaxonomy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a policy tag in a taxonomy.
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/CreatePolicyTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManager",
                        "CreatePolicyTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a policy tag together with the following:
        ///
        /// * All of its descendant policy tags, if any
        /// * Policies associated with the policy tag and its descendants
        /// * References from BigQuery table schema of the policy tag and its
        ///   descendants
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/DeletePolicyTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManager",
                        "DeletePolicyTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a policy tag, including its display
        /// name, description, and parent policy tag.
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/UpdatePolicyTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManager",
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/ListPolicyTags",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManager",
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/GetPolicyTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManager",
                        "GetPolicyTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the IAM policy for a policy tag or a taxonomy.
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManager",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the IAM policy for a policy tag or a taxonomy.
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManager",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns your permissions on a specified policy tag or
        /// taxonomy.
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
                "/google.cloud.datacatalog.v1.PolicyTagManager/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManager",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// A nested protocol buffer that represents a taxonomy and the hierarchy of its
/// policy tags. Used for taxonomy replacement, import, and
/// export.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedTaxonomy {
    /// Required. Display name of the taxonomy. At most 200 bytes when encoded in
    /// UTF-8.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of the serialized taxonomy. At most 2000 bytes when
    /// encoded in UTF-8. If not set, defaults to an empty description.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Top level policy tags associated with the taxonomy, if any.
    #[prost(message, repeated, tag = "3")]
    pub policy_tags: ::prost::alloc::vec::Vec<SerializedPolicyTag>,
    /// A list of policy types that are activated per taxonomy.
    #[prost(enumeration = "taxonomy::PolicyType", repeated, tag = "4")]
    pub activated_policy_types: ::prost::alloc::vec::Vec<i32>,
}
/// A nested protocol buffer that represents a policy tag and all its
/// descendants.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedPolicyTag {
    /// Resource name of the policy tag.
    ///
    /// This field is ignored when calling `ImportTaxonomies`.
    #[prost(string, tag = "1")]
    pub policy_tag: ::prost::alloc::string::String,
    /// Required. Display name of the policy tag. At most 200 bytes when encoded
    /// in UTF-8.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of the serialized policy tag. At most
    /// 2000 bytes when encoded in UTF-8. If not set, defaults to an
    /// empty description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Children of the policy tag, if any.
    #[prost(message, repeated, tag = "4")]
    pub child_policy_tags: ::prost::alloc::vec::Vec<SerializedPolicyTag>,
}
/// Request message for
/// [ReplaceTaxonomy][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ReplaceTaxonomy].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceTaxonomyRequest {
    /// Required. Resource name of the taxonomy to update.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Taxonomy to update along with its child policy tags.
    #[prost(message, optional, tag = "2")]
    pub serialized_taxonomy: ::core::option::Option<SerializedTaxonomy>,
}
/// Request message for
/// [ImportTaxonomies][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ImportTaxonomies].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportTaxonomiesRequest {
    /// Required. Resource name of project that the imported taxonomies will belong
    /// to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Source taxonomies to import.
    #[prost(oneof = "import_taxonomies_request::Source", tags = "2, 3")]
    pub source: ::core::option::Option<import_taxonomies_request::Source>,
}
/// Nested message and enum types in `ImportTaxonomiesRequest`.
pub mod import_taxonomies_request {
    /// Source taxonomies to import.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Inline source taxonomy to import.
        #[prost(message, tag = "2")]
        InlineSource(super::InlineSource),
        /// Cross-regional source taxonomy to import.
        #[prost(message, tag = "3")]
        CrossRegionalSource(super::CrossRegionalSource),
    }
}
/// Inline source containing taxonomies to import.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlineSource {
    /// Required. Taxonomies to import.
    #[prost(message, repeated, tag = "1")]
    pub taxonomies: ::prost::alloc::vec::Vec<SerializedTaxonomy>,
}
/// Cross-regional source used to import an existing taxonomy into a different
/// region.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossRegionalSource {
    /// Required. The resource name of the source taxonomy to import.
    #[prost(string, tag = "1")]
    pub taxonomy: ::prost::alloc::string::String,
}
/// Response message for
/// [ImportTaxonomies][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ImportTaxonomies].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportTaxonomiesResponse {
    /// Imported taxonomies.
    #[prost(message, repeated, tag = "1")]
    pub taxonomies: ::prost::alloc::vec::Vec<Taxonomy>,
}
/// Request message for
/// [ExportTaxonomies][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ExportTaxonomies].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTaxonomiesRequest {
    /// Required. Resource name of the project that the exported taxonomies belong
    /// to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Resource names of the taxonomies to export.
    #[prost(string, repeated, tag = "2")]
    pub taxonomies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Export destination for taxonomies.
    #[prost(oneof = "export_taxonomies_request::Destination", tags = "3")]
    pub destination: ::core::option::Option<export_taxonomies_request::Destination>,
}
/// Nested message and enum types in `ExportTaxonomiesRequest`.
pub mod export_taxonomies_request {
    /// Required. Export destination for taxonomies.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Serialized export taxonomies that contain all the policy
        /// tags as nested protocol buffers.
        #[prost(bool, tag = "3")]
        SerializedTaxonomies(bool),
    }
}
/// Response message for
/// [ExportTaxonomies][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ExportTaxonomies].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTaxonomiesResponse {
    /// List of taxonomies and policy tags as nested protocol buffers.
    #[prost(message, repeated, tag = "1")]
    pub taxonomies: ::prost::alloc::vec::Vec<SerializedTaxonomy>,
}
/// Generated client implementations.
pub mod policy_tag_manager_serialization_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Policy Tag Manager Serialization API service allows you to manipulate
    /// your policy tags and taxonomies in a serialized format.
    ///
    /// Taxonomy is a hierarchical group of policy tags.
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
        /// Replaces (updates) a taxonomy and all its policy tags.
        ///
        /// The taxonomy and its entire hierarchy of policy tags must be
        /// represented literally by `SerializedTaxonomy` and the nested
        /// `SerializedPolicyTag` messages.
        ///
        /// This operation automatically does the following:
        ///
        /// - Deletes the existing policy tags that are missing from the
        ///   `SerializedPolicyTag`.
        /// - Creates policy tags that don't have resource names. They are considered
        ///   new.
        /// - Updates policy tags with valid resources names accordingly.
        pub async fn replace_taxonomy(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplaceTaxonomyRequest>,
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
                "/google.cloud.datacatalog.v1.PolicyTagManagerSerialization/ReplaceTaxonomy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManagerSerialization",
                        "ReplaceTaxonomy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates new taxonomies (including their policy tags) in a given project
        /// by importing from inlined or cross-regional sources.
        ///
        /// For a cross-regional source, new taxonomies are created by copying
        /// from a source in another region.
        ///
        /// For an inlined source, taxonomies and policy tags are created in bulk using
        /// nested protocol buffer structures.
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
                "/google.cloud.datacatalog.v1.PolicyTagManagerSerialization/ImportTaxonomies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManagerSerialization",
                        "ImportTaxonomies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Exports taxonomies in the requested type and returns them,
        /// including their policy tags. The requested taxonomies must belong to the
        /// same project.
        ///
        /// This method generates `SerializedTaxonomy` protocol buffers with nested
        /// policy tags that can be used as input for `ImportTaxonomies` calls.
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
                "/google.cloud.datacatalog.v1.PolicyTagManagerSerialization/ExportTaxonomies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.v1.PolicyTagManagerSerialization",
                        "ExportTaxonomies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
