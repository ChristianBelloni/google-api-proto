/// Schema of a table. This schema is a subset of
/// google.cloud.bigquery.v2.TableSchema containing information necessary to
/// generate valid message to write to BigQuery.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableSchema {
    /// Describes the fields in a table.
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<TableFieldSchema>,
}
/// TableFieldSchema defines a single field/column within a table schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableFieldSchema {
    /// Required. The field name. The name must contain only letters (a-z, A-Z),
    /// numbers (0-9), or underscores (_), and must start with a letter or
    /// underscore. The maximum length is 128 characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The field data type.
    #[prost(enumeration = "table_field_schema::Type", tag = "2")]
    pub r#type: i32,
    /// Optional. The field mode. The default value is NULLABLE.
    #[prost(enumeration = "table_field_schema::Mode", tag = "3")]
    pub mode: i32,
    /// Optional. Describes the nested schema fields if the type property is set to STRUCT.
    #[prost(message, repeated, tag = "4")]
    pub fields: ::prost::alloc::vec::Vec<TableFieldSchema>,
    /// Optional. The field description. The maximum length is 1,024 characters.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Maximum length of values of this field for STRINGS or BYTES.
    ///
    /// If max_length is not specified, no maximum length constraint is imposed
    /// on this field.
    ///
    /// If type = "STRING", then max_length represents the maximum UTF-8
    /// length of strings in this field.
    ///
    /// If type = "BYTES", then max_length represents the maximum number of
    /// bytes in this field.
    ///
    /// It is invalid to set this field if type is not "STRING" or "BYTES".
    #[prost(int64, tag = "7")]
    pub max_length: i64,
    /// Optional. Precision (maximum number of total digits in base 10) and scale
    /// (maximum number of digits in the fractional part in base 10) constraints
    /// for values of this field for NUMERIC or BIGNUMERIC.
    ///
    /// It is invalid to set precision or scale if type is not "NUMERIC" or
    /// "BIGNUMERIC".
    ///
    /// If precision and scale are not specified, no value range constraint is
    /// imposed on this field insofar as values are permitted by the type.
    ///
    /// Values of this NUMERIC or BIGNUMERIC field must be in this range when:
    ///
    /// * Precision (P) and scale (S) are specified:
    ///    [-10^(P-S) + 10^(-S), 10^(P-S) - 10^(-S)]
    /// * Precision (P) is specified but not scale (and thus scale is
    ///    interpreted to be equal to zero):
    ///    [-10^P + 1, 10^P - 1].
    ///
    /// Acceptable values for precision and scale if both are specified:
    ///
    /// * If type = "NUMERIC":
    ///    1 <= precision - scale <= 29 and 0 <= scale <= 9.
    /// * If type = "BIGNUMERIC":
    ///    1 <= precision - scale <= 38 and 0 <= scale <= 38.
    ///
    /// Acceptable values for precision if only precision is specified but not
    /// scale (and thus scale is interpreted to be equal to zero):
    ///
    /// * If type = "NUMERIC": 1 <= precision <= 29.
    /// * If type = "BIGNUMERIC": 1 <= precision <= 38.
    ///
    /// If scale is specified but not precision, then it is invalid.
    #[prost(int64, tag = "8")]
    pub precision: i64,
    /// Optional. See documentation for precision.
    #[prost(int64, tag = "9")]
    pub scale: i64,
}
/// Nested message and enum types in `TableFieldSchema`.
pub mod table_field_schema {
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
        /// Illegal value
        Unspecified = 0,
        /// 64K, UTF8
        String = 1,
        /// 64-bit signed
        Int64 = 2,
        /// 64-bit IEEE floating point
        Double = 3,
        /// Aggregate type
        Struct = 4,
        /// 64K, Binary
        Bytes = 5,
        /// 2-valued
        Bool = 6,
        /// 64-bit signed usec since UTC epoch
        Timestamp = 7,
        /// Civil date - Year, Month, Day
        Date = 8,
        /// Civil time - Hour, Minute, Second, Microseconds
        Time = 9,
        /// Combination of civil date and civil time
        Datetime = 10,
        /// Geography object
        Geography = 11,
        /// Numeric value
        Numeric = 12,
        /// BigNumeric value
        Bignumeric = 13,
        /// Interval
        Interval = 14,
        /// JSON, String
        Json = 15,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::String => "STRING",
                Type::Int64 => "INT64",
                Type::Double => "DOUBLE",
                Type::Struct => "STRUCT",
                Type::Bytes => "BYTES",
                Type::Bool => "BOOL",
                Type::Timestamp => "TIMESTAMP",
                Type::Date => "DATE",
                Type::Time => "TIME",
                Type::Datetime => "DATETIME",
                Type::Geography => "GEOGRAPHY",
                Type::Numeric => "NUMERIC",
                Type::Bignumeric => "BIGNUMERIC",
                Type::Interval => "INTERVAL",
                Type::Json => "JSON",
            }
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
    pub enum Mode {
        /// Illegal value
        Unspecified = 0,
        Nullable = 1,
        Required = 2,
        Repeated = 3,
    }
    impl Mode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mode::Unspecified => "MODE_UNSPECIFIED",
                Mode::Nullable => "NULLABLE",
                Mode::Required => "REQUIRED",
                Mode::Repeated => "REPEATED",
            }
        }
    }
}
/// Avro schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvroSchema {
    /// Json serialized schema, as described at
    /// <https://avro.apache.org/docs/1.8.1/spec.html.>
    #[prost(string, tag = "1")]
    pub schema: ::prost::alloc::string::String,
}
/// Avro rows.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvroRows {
    /// Binary serialized rows in a block.
    #[prost(bytes = "bytes", tag = "1")]
    pub serialized_binary_rows: ::prost::bytes::Bytes,
    /// \[Deprecated\] The count of rows in the returning block.
    /// Please use the format-independent ReadRowsResponse.row_count instead.
    #[deprecated]
    #[prost(int64, tag = "2")]
    pub row_count: i64,
}
/// Contains options specific to Avro Serialization.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvroSerializationOptions {
    /// Enable displayName attribute in Avro schema.
    ///
    /// The Avro specification requires field names to be alphanumeric.  By
    /// default, in cases when column names do not conform to these requirements
    /// (e.g. non-ascii unicode codepoints) and Avro is requested as an output
    /// format, the CreateReadSession call will fail.
    ///
    /// Setting this field to true, populates avro field names with a placeholder
    /// value and populates a "displayName" attribute for every avro field with the
    /// original column name.
    #[prost(bool, tag = "1")]
    pub enable_display_name_attribute: bool,
}
/// ProtoSchema describes the schema of the serialized protocol buffer data rows.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoSchema {
    /// Descriptor for input message.  The provided descriptor must be self
    /// contained, such that data rows sent can be fully decoded using only the
    /// single descriptor.  For data rows that are compositions of multiple
    /// independent messages, this means the descriptor may need to be transformed
    /// to only use nested types:
    /// <https://developers.google.com/protocol-buffers/docs/proto#nested>
    ///
    /// For additional information for how proto types and values map onto BigQuery
    /// see: <https://cloud.google.com/bigquery/docs/write-api#data_type_conversions>
    #[prost(message, optional, tag = "1")]
    pub proto_descriptor: ::core::option::Option<::prost_types::DescriptorProto>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoRows {
    /// A sequence of rows serialized as a Protocol Buffer.
    ///
    /// See <https://developers.google.com/protocol-buffers/docs/overview> for more
    /// information on deserializing this field.
    #[prost(bytes = "bytes", repeated, tag = "1")]
    pub serialized_rows: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
}
/// Arrow schema as specified in
/// <https://arrow.apache.org/docs/python/api/datatypes.html>
/// and serialized to bytes using IPC:
/// <https://arrow.apache.org/docs/format/Columnar.html#serialization-and-interprocess-communication-ipc>
///
/// See code samples on how this message can be deserialized.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrowSchema {
    /// IPC serialized Arrow schema.
    #[prost(bytes = "bytes", tag = "1")]
    pub serialized_schema: ::prost::bytes::Bytes,
}
/// Arrow RecordBatch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrowRecordBatch {
    /// IPC-serialized Arrow RecordBatch.
    #[prost(bytes = "bytes", tag = "1")]
    pub serialized_record_batch: ::prost::bytes::Bytes,
    /// \[Deprecated\] The count of rows in `serialized_record_batch`.
    /// Please use the format-independent ReadRowsResponse.row_count instead.
    #[deprecated]
    #[prost(int64, tag = "2")]
    pub row_count: i64,
}
/// Contains options specific to Arrow Serialization.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrowSerializationOptions {
    /// The compression codec to use for Arrow buffers in serialized record
    /// batches.
    #[prost(enumeration = "arrow_serialization_options::CompressionCodec", tag = "2")]
    pub buffer_compression: i32,
}
/// Nested message and enum types in `ArrowSerializationOptions`.
pub mod arrow_serialization_options {
    /// Compression codec's supported by Arrow.
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
    pub enum CompressionCodec {
        /// If unspecified no compression will be used.
        CompressionUnspecified = 0,
        /// LZ4 Frame (<https://github.com/lz4/lz4/blob/dev/doc/lz4_Frame_format.md>)
        Lz4Frame = 1,
        /// Zstandard compression.
        Zstd = 2,
    }
    impl CompressionCodec {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CompressionCodec::CompressionUnspecified => "COMPRESSION_UNSPECIFIED",
                CompressionCodec::Lz4Frame => "LZ4_FRAME",
                CompressionCodec::Zstd => "ZSTD",
            }
        }
    }
}
/// Information about the ReadSession.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadSession {
    /// Output only. Unique identifier for the session, in the form
    /// `projects/{project_id}/locations/{location}/sessions/{session_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time at which the session becomes invalid. After this time, subsequent
    /// requests to read this Session will return errors. The expire_time is
    /// automatically assigned and currently cannot be specified or updated.
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. Data format of the output data. DATA_FORMAT_UNSPECIFIED not supported.
    #[prost(enumeration = "DataFormat", tag = "3")]
    pub data_format: i32,
    /// Immutable. Table that this ReadSession is reading from, in the form
    /// `projects/{project_id}/datasets/{dataset_id}/tables/{table_id}`
    #[prost(string, tag = "6")]
    pub table: ::prost::alloc::string::String,
    /// Optional. Any modifiers which are applied when reading from the specified table.
    #[prost(message, optional, tag = "7")]
    pub table_modifiers: ::core::option::Option<read_session::TableModifiers>,
    /// Optional. Read options for this session (e.g. column selection, filters).
    #[prost(message, optional, tag = "8")]
    pub read_options: ::core::option::Option<read_session::TableReadOptions>,
    /// Output only. A list of streams created with the session.
    ///
    /// At least one stream is created with the session. In the future, larger
    /// request_stream_count values *may* result in this list being unpopulated,
    /// in that case, the user will need to use a List method to get the streams
    /// instead, which is not yet available.
    #[prost(message, repeated, tag = "10")]
    pub streams: ::prost::alloc::vec::Vec<ReadStream>,
    /// Output only. An estimate on the number of bytes this session will scan when
    /// all streams are completely consumed. This estimate is based on
    /// metadata from the table which might be incomplete or stale.
    #[prost(int64, tag = "12")]
    pub estimated_total_bytes_scanned: i64,
    /// Optional. ID set by client to annotate a session identity.  This does not need
    /// to be strictly unique, but instead the same ID should be used to group
    /// logically connected sessions (e.g. All using the same ID for all sessions
    /// needed to complete a Spark SQL query is reasonable).
    ///
    /// Maximum length is 256 bytes.
    #[prost(string, tag = "13")]
    pub trace_id: ::prost::alloc::string::String,
    /// The schema for the read. If read_options.selected_fields is set, the
    /// schema may be different from the table schema as it will only contain
    /// the selected fields.
    #[prost(oneof = "read_session::Schema", tags = "4, 5")]
    pub schema: ::core::option::Option<read_session::Schema>,
}
/// Nested message and enum types in `ReadSession`.
pub mod read_session {
    /// Additional attributes when reading a table.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableModifiers {
        /// The snapshot time of the table. If not set, interpreted as now.
        #[prost(message, optional, tag = "1")]
        pub snapshot_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Options dictating how we read a table.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableReadOptions {
        /// Optional. The names of the fields in the table to be returned. If no
        /// field names are specified, then all fields in the table are returned.
        ///
        /// Nested fields -- the child elements of a STRUCT field -- can be selected
        /// individually using their fully-qualified names, and will be returned as
        /// record fields containing only the selected nested fields. If a STRUCT
        /// field is specified in the selected fields list, all of the child elements
        /// will be returned.
        ///
        /// As an example, consider a table with the following schema:
        ///
        ///    {
        ///        "name": "struct_field",
        ///        "type": "RECORD",
        ///        "mode": "NULLABLE",
        ///        "fields": [
        ///            {
        ///                "name": "string_field1",
        ///                "type": "STRING",
        /// .              "mode": "NULLABLE"
        ///            },
        ///            {
        ///                "name": "string_field2",
        ///                "type": "STRING",
        ///                "mode": "NULLABLE"
        ///            }
        ///        ]
        ///    }
        ///
        /// Specifying "struct_field" in the selected fields list will result in a
        /// read session schema with the following logical structure:
        ///
        ///    struct_field {
        ///        string_field1
        ///        string_field2
        ///    }
        ///
        /// Specifying "struct_field.string_field1" in the selected fields list will
        /// result in a read session schema with the following logical structure:
        ///
        ///    struct_field {
        ///        string_field1
        ///    }
        ///
        /// The order of the fields in the read session schema is derived from the
        /// table schema and does not correspond to the order in which the fields are
        /// specified in this list.
        #[prost(string, repeated, tag = "1")]
        pub selected_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// SQL text filtering statement, similar to a WHERE clause in a query.
        /// Aggregates are not supported.
        ///
        /// Examples: "int_field > 5"
        ///            "date_field = CAST('2014-9-27' as DATE)"
        ///            "nullable_field is not NULL"
        ///            "st_equals(geo_field, st_geofromtext("POINT(2, 2)"))"
        ///            "numeric_field BETWEEN 1.0 AND 5.0"
        ///
        /// Restricted to a maximum length for 1 MB.
        #[prost(string, tag = "2")]
        pub row_restriction: ::prost::alloc::string::String,
        #[prost(
            oneof = "table_read_options::OutputFormatSerializationOptions",
            tags = "3, 4"
        )]
        pub output_format_serialization_options: ::core::option::Option<
            table_read_options::OutputFormatSerializationOptions,
        >,
    }
    /// Nested message and enum types in `TableReadOptions`.
    pub mod table_read_options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OutputFormatSerializationOptions {
            /// Optional. Options specific to the Apache Arrow output format.
            #[prost(message, tag = "3")]
            ArrowSerializationOptions(super::super::ArrowSerializationOptions),
            /// Optional. Options specific to the Apache Avro output format
            #[prost(message, tag = "4")]
            AvroSerializationOptions(super::super::AvroSerializationOptions),
        }
    }
    /// The schema for the read. If read_options.selected_fields is set, the
    /// schema may be different from the table schema as it will only contain
    /// the selected fields.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schema {
        /// Output only. Avro schema.
        #[prost(message, tag = "4")]
        AvroSchema(super::AvroSchema),
        /// Output only. Arrow schema.
        #[prost(message, tag = "5")]
        ArrowSchema(super::ArrowSchema),
    }
}
/// Information about a single stream that gets data out of the storage system.
/// Most of the information about `ReadStream` instances is aggregated, making
/// `ReadStream` lightweight.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadStream {
    /// Output only. Name of the stream, in the form
    /// `projects/{project_id}/locations/{location}/sessions/{session_id}/streams/{stream_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Information about a single stream that gets data inside the storage system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteStream {
    /// Output only. Name of the stream, in the form
    /// `projects/{project}/datasets/{dataset}/tables/{table}/streams/{stream}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Type of the stream.
    #[prost(enumeration = "write_stream::Type", tag = "2")]
    pub r#type: i32,
    /// Output only. Create time of the stream. For the _default stream, this is the
    /// creation_time of the table.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Commit time of the stream.
    /// If a stream is of `COMMITTED` type, then it will have a commit_time same as
    /// `create_time`. If the stream is of `PENDING` type, empty commit_time
    /// means it is not committed.
    #[prost(message, optional, tag = "4")]
    pub commit_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The schema of the destination table. It is only returned in
    /// `CreateWriteStream` response. Caller should generate data that's
    /// compatible with this schema to send in initial `AppendRowsRequest`.
    /// The table schema could go out of date during the life time of the stream.
    #[prost(message, optional, tag = "5")]
    pub table_schema: ::core::option::Option<TableSchema>,
    /// Immutable. Mode of the stream.
    #[prost(enumeration = "write_stream::WriteMode", tag = "7")]
    pub write_mode: i32,
    /// Immutable. The geographic location where the stream's dataset resides. See
    /// <https://cloud.google.com/bigquery/docs/locations> for supported
    /// locations.
    #[prost(string, tag = "8")]
    pub location: ::prost::alloc::string::String,
}
/// Nested message and enum types in `WriteStream`.
pub mod write_stream {
    /// Type enum of the stream.
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
        /// Unknown type.
        Unspecified = 0,
        /// Data will commit automatically and appear as soon as the write is
        /// acknowledged.
        Committed = 1,
        /// Data is invisible until the stream is committed.
        Pending = 2,
        /// Data is only visible up to the offset to which it was flushed.
        Buffered = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Committed => "COMMITTED",
                Type::Pending => "PENDING",
                Type::Buffered => "BUFFERED",
            }
        }
    }
    /// Mode enum of the stream.
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
    pub enum WriteMode {
        /// Unknown type.
        Unspecified = 0,
        /// Insert new records into the table.
        /// It is the default value if customers do not specify it.
        Insert = 1,
    }
    impl WriteMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WriteMode::Unspecified => "WRITE_MODE_UNSPECIFIED",
                WriteMode::Insert => "INSERT",
            }
        }
    }
}
/// Data format for input or output data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataFormat {
    /// Data format is unspecified.
    Unspecified = 0,
    /// Avro is a standard open source row based file format.
    /// See <https://avro.apache.org/> for more details.
    Avro = 1,
    /// Arrow is a standard open source column-based message format.
    /// See <https://arrow.apache.org/> for more details.
    Arrow = 2,
}
impl DataFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataFormat::Unspecified => "DATA_FORMAT_UNSPECIFIED",
            DataFormat::Avro => "AVRO",
            DataFormat::Arrow => "ARROW",
        }
    }
}
/// WriteStreamView is a view enum that controls what details about a write
/// stream should be returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WriteStreamView {
    /// The default / unset value.
    Unspecified = 0,
    /// The BASIC projection returns basic metadata about a write stream.  The
    /// basic view does not include schema information.  This is the default view
    /// returned by GetWriteStream.
    Basic = 1,
    /// The FULL projection returns all available write stream metadata, including
    /// the schema.  CreateWriteStream returns the full projection of write stream
    /// metadata.
    Full = 2,
}
impl WriteStreamView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WriteStreamView::Unspecified => "WRITE_STREAM_VIEW_UNSPECIFIED",
            WriteStreamView::Basic => "BASIC",
            WriteStreamView::Full => "FULL",
        }
    }
}
/// Request message for `CreateReadSession`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReadSessionRequest {
    /// Required. The request project that owns the session, in the form of
    /// `projects/{project_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Session to be created.
    #[prost(message, optional, tag = "2")]
    pub read_session: ::core::option::Option<ReadSession>,
    /// Max initial number of streams. If unset or zero, the server will
    /// provide a value of streams so as to produce reasonable throughput. Must be
    /// non-negative. The number of streams may be lower than the requested number,
    /// depending on the amount parallelism that is reasonable for the table.
    /// There is a default system max limit of 1,000.
    ///
    /// This must be greater than or equal to preferred_min_stream_count.
    /// Typically, clients should either leave this unset to let the system to
    /// determine an upper bound OR set this a size for the maximum "units of work"
    /// it can gracefully handle.
    #[prost(int32, tag = "3")]
    pub max_stream_count: i32,
    /// The minimum preferred stream count. This parameter can be used to inform
    /// the service that there is a desired lower bound on the number of streams.
    /// This is typically a target parallelism of the client (e.g. a Spark
    /// cluster with N-workers would set this to a low multiple of N to ensure
    /// good cluster utilization).
    ///
    /// The system will make a best effort to provide at least this number of
    /// streams, but in some cases might provide less.
    #[prost(int32, tag = "4")]
    pub preferred_min_stream_count: i32,
}
/// Request message for `ReadRows`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRowsRequest {
    /// Required. Stream to read rows from.
    #[prost(string, tag = "1")]
    pub read_stream: ::prost::alloc::string::String,
    /// The offset requested must be less than the last row read from Read.
    /// Requesting a larger offset is undefined. If not specified, start reading
    /// from offset zero.
    #[prost(int64, tag = "2")]
    pub offset: i64,
}
/// Information on if the current connection is being throttled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThrottleState {
    /// How much this connection is being throttled. Zero means no throttling,
    /// 100 means fully throttled.
    #[prost(int32, tag = "1")]
    pub throttle_percent: i32,
}
/// Estimated stream statistics for a given read Stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamStats {
    /// Represents the progress of the current stream.
    #[prost(message, optional, tag = "2")]
    pub progress: ::core::option::Option<stream_stats::Progress>,
}
/// Nested message and enum types in `StreamStats`.
pub mod stream_stats {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Progress {
        /// The fraction of rows assigned to the stream that have been processed by
        /// the server so far, not including the rows in the current response
        /// message.
        ///
        /// This value, along with `at_response_end`, can be used to interpolate
        /// the progress made as the rows in the message are being processed using
        /// the following formula: `at_response_start + (at_response_end -
        /// at_response_start) * rows_processed_from_response / rows_in_response`.
        ///
        /// Note that if a filter is provided, the `at_response_end` value of the
        /// previous response may not necessarily be equal to the
        /// `at_response_start` value of the current response.
        #[prost(double, tag = "1")]
        pub at_response_start: f64,
        /// Similar to `at_response_start`, except that this value includes the
        /// rows in the current response.
        #[prost(double, tag = "2")]
        pub at_response_end: f64,
    }
}
/// Response from calling `ReadRows` may include row data, progress and
/// throttling information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRowsResponse {
    /// Number of serialized rows in the rows block.
    #[prost(int64, tag = "6")]
    pub row_count: i64,
    /// Statistics for the stream.
    #[prost(message, optional, tag = "2")]
    pub stats: ::core::option::Option<StreamStats>,
    /// Throttling state. If unset, the latest response still describes
    /// the current throttling status.
    #[prost(message, optional, tag = "5")]
    pub throttle_state: ::core::option::Option<ThrottleState>,
    /// Row data is returned in format specified during session creation.
    #[prost(oneof = "read_rows_response::Rows", tags = "3, 4")]
    pub rows: ::core::option::Option<read_rows_response::Rows>,
    /// The schema for the read. If read_options.selected_fields is set, the
    /// schema may be different from the table schema as it will only contain
    /// the selected fields. This schema is equivalent to the one returned by
    /// CreateSession. This field is only populated in the first ReadRowsResponse
    /// RPC.
    #[prost(oneof = "read_rows_response::Schema", tags = "7, 8")]
    pub schema: ::core::option::Option<read_rows_response::Schema>,
}
/// Nested message and enum types in `ReadRowsResponse`.
pub mod read_rows_response {
    /// Row data is returned in format specified during session creation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rows {
        /// Serialized row data in AVRO format.
        #[prost(message, tag = "3")]
        AvroRows(super::AvroRows),
        /// Serialized row data in Arrow RecordBatch format.
        #[prost(message, tag = "4")]
        ArrowRecordBatch(super::ArrowRecordBatch),
    }
    /// The schema for the read. If read_options.selected_fields is set, the
    /// schema may be different from the table schema as it will only contain
    /// the selected fields. This schema is equivalent to the one returned by
    /// CreateSession. This field is only populated in the first ReadRowsResponse
    /// RPC.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schema {
        /// Output only. Avro schema.
        #[prost(message, tag = "7")]
        AvroSchema(super::AvroSchema),
        /// Output only. Arrow schema.
        #[prost(message, tag = "8")]
        ArrowSchema(super::ArrowSchema),
    }
}
/// Request message for `SplitReadStream`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitReadStreamRequest {
    /// Required. Name of the stream to split.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A value in the range (0.0, 1.0) that specifies the fractional point at
    /// which the original stream should be split. The actual split point is
    /// evaluated on pre-filtered rows, so if a filter is provided, then there is
    /// no guarantee that the division of the rows between the new child streams
    /// will be proportional to this fractional value. Additionally, because the
    /// server-side unit for assigning data is collections of rows, this fraction
    /// will always map to a data storage boundary on the server side.
    #[prost(double, tag = "2")]
    pub fraction: f64,
}
/// Response message for `SplitReadStream`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitReadStreamResponse {
    /// Primary stream, which contains the beginning portion of
    /// |original_stream|. An empty value indicates that the original stream can no
    /// longer be split.
    #[prost(message, optional, tag = "1")]
    pub primary_stream: ::core::option::Option<ReadStream>,
    /// Remainder stream, which contains the tail of |original_stream|. An empty
    /// value indicates that the original stream can no longer be split.
    #[prost(message, optional, tag = "2")]
    pub remainder_stream: ::core::option::Option<ReadStream>,
}
/// Request message for `CreateWriteStream`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWriteStreamRequest {
    /// Required. Reference to the table to which the stream belongs, in the format
    /// of `projects/{project}/datasets/{dataset}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Stream to be created.
    #[prost(message, optional, tag = "2")]
    pub write_stream: ::core::option::Option<WriteStream>,
}
/// Request message for `AppendRows`.
///
/// Due to the nature of AppendRows being a bidirectional streaming RPC, certain
/// parts of the AppendRowsRequest need only be specified for the first request
/// sent each time the gRPC network connection is opened/reopened.
///
/// The size of a single AppendRowsRequest must be less than 10 MB in size.
/// Requests larger than this return an error, typically `INVALID_ARGUMENT`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendRowsRequest {
    /// Required. The write_stream identifies the target of the append operation, and only
    /// needs to be specified as part of the first request on the gRPC connection.
    /// If provided for subsequent requests, it must match the value of the first
    /// request.
    ///
    /// For explicitly created write streams, the format is:
    ///
    /// * `projects/{project}/datasets/{dataset}/tables/{table}/streams/{id}`
    ///
    /// For the special default stream, the format is:
    ///
    /// * `projects/{project}/datasets/{dataset}/tables/{table}/streams/_default`.
    #[prost(string, tag = "1")]
    pub write_stream: ::prost::alloc::string::String,
    /// If present, the write is only performed if the next append offset is same
    /// as the provided value. If not present, the write is performed at the
    /// current end of stream. Specifying a value for this field is not allowed
    /// when calling AppendRows for the '_default' stream.
    #[prost(message, optional, tag = "2")]
    pub offset: ::core::option::Option<i64>,
    /// Id set by client to annotate its identity. Only initial request setting is
    /// respected.
    #[prost(string, tag = "6")]
    pub trace_id: ::prost::alloc::string::String,
    /// A map to indicate how to interpret missing value for some fields. Missing
    /// values are fields present in user schema but missing in rows. The key is
    /// the field name. The value is the interpretation of missing values for the
    /// field.
    ///
    /// For example, a map {'foo': NULL_VALUE, 'bar': DEFAULT_VALUE} means all
    /// missing values in field foo are interpreted as NULL, all missing values in
    /// field bar are interpreted as the default value of field bar in table
    /// schema.
    ///
    /// If a field is not in this map and has missing values, the missing values
    /// in this field are interpreted as NULL.
    ///
    /// This field only applies to the current request, it won't affect other
    /// requests on the connection.
    ///
    /// Currently, field name can only be top-level column name, can't be a struct
    /// field path like 'foo.bar'.
    #[prost(
        btree_map = "string, enumeration(append_rows_request::MissingValueInterpretation)",
        tag = "7"
    )]
    pub missing_value_interpretations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        i32,
    >,
    /// Input rows. The `writer_schema` field must be specified at the initial
    /// request and currently, it will be ignored if specified in following
    /// requests. Following requests must have data in the same format as the
    /// initial request.
    #[prost(oneof = "append_rows_request::Rows", tags = "4")]
    pub rows: ::core::option::Option<append_rows_request::Rows>,
}
/// Nested message and enum types in `AppendRowsRequest`.
pub mod append_rows_request {
    /// ProtoData contains the data rows and schema when constructing append
    /// requests.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoData {
        /// Proto schema used to serialize the data.  This value only needs to be
        /// provided as part of the first request on a gRPC network connection,
        /// and will be ignored for subsequent requests on the connection.
        #[prost(message, optional, tag = "1")]
        pub writer_schema: ::core::option::Option<super::ProtoSchema>,
        /// Serialized row data in protobuf message format.
        /// Currently, the backend expects the serialized rows to adhere to
        /// proto2 semantics when appending rows, particularly with respect to
        /// how default values are encoded.
        #[prost(message, optional, tag = "2")]
        pub rows: ::core::option::Option<super::ProtoRows>,
    }
    /// An enum to indicate how to interpret missing values. Missing values are
    /// fields present in user schema but missing in rows. A missing value can
    /// represent a NULL or a column default value defined in BigQuery table
    /// schema.
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
    pub enum MissingValueInterpretation {
        /// Invalid missing value interpretation. Requests with this value will be
        /// rejected.
        Unspecified = 0,
        /// Missing value is interpreted as NULL.
        NullValue = 1,
        /// Missing value is interpreted as column default value if declared in the
        /// table schema, NULL otherwise.
        DefaultValue = 2,
    }
    impl MissingValueInterpretation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MissingValueInterpretation::Unspecified => {
                    "MISSING_VALUE_INTERPRETATION_UNSPECIFIED"
                }
                MissingValueInterpretation::NullValue => "NULL_VALUE",
                MissingValueInterpretation::DefaultValue => "DEFAULT_VALUE",
            }
        }
    }
    /// Input rows. The `writer_schema` field must be specified at the initial
    /// request and currently, it will be ignored if specified in following
    /// requests. Following requests must have data in the same format as the
    /// initial request.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rows {
        /// Rows in proto format.
        #[prost(message, tag = "4")]
        ProtoRows(ProtoData),
    }
}
/// Response message for `AppendRows`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendRowsResponse {
    /// If backend detects a schema update, pass it to user so that user can
    /// use it to input new type of message. It will be empty when no schema
    /// updates have occurred.
    #[prost(message, optional, tag = "3")]
    pub updated_schema: ::core::option::Option<TableSchema>,
    /// If a request failed due to corrupted rows, no rows in the batch will be
    /// appended. The API will return row level error info, so that the caller can
    /// remove the bad rows and retry the request.
    #[prost(message, repeated, tag = "4")]
    pub row_errors: ::prost::alloc::vec::Vec<RowError>,
    /// The target of the append operation. Matches the write_stream in the
    /// corresponding request.
    #[prost(string, tag = "5")]
    pub write_stream: ::prost::alloc::string::String,
    #[prost(oneof = "append_rows_response::Response", tags = "1, 2")]
    pub response: ::core::option::Option<append_rows_response::Response>,
}
/// Nested message and enum types in `AppendRowsResponse`.
pub mod append_rows_response {
    /// AppendResult is returned for successful append requests.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppendResult {
        /// The row offset at which the last append occurred. The offset will not be
        /// set if appending using default streams.
        #[prost(message, optional, tag = "1")]
        pub offset: ::core::option::Option<i64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// Result if the append is successful.
        #[prost(message, tag = "1")]
        AppendResult(AppendResult),
        /// Error returned when problems were encountered.  If present,
        /// it indicates rows were not accepted into the system.
        /// Users can retry or continue with other append requests within the
        /// same connection.
        ///
        /// Additional information about error signalling:
        ///
        /// ALREADY_EXISTS: Happens when an append specified an offset, and the
        /// backend already has received data at this offset.  Typically encountered
        /// in retry scenarios, and can be ignored.
        ///
        /// OUT_OF_RANGE: Returned when the specified offset in the stream is beyond
        /// the current end of the stream.
        ///
        /// INVALID_ARGUMENT: Indicates a malformed request or data.
        ///
        /// ABORTED: Request processing is aborted because of prior failures.  The
        /// request can be retried if previous failure is addressed.
        ///
        /// INTERNAL: Indicates server side error(s) that can be retried.
        #[prost(message, tag = "2")]
        Error(super::super::super::super::super::rpc::Status),
    }
}
/// Request message for `GetWriteStreamRequest`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWriteStreamRequest {
    /// Required. Name of the stream to get, in the form of
    /// `projects/{project}/datasets/{dataset}/tables/{table}/streams/{stream}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates whether to get full or partial view of the WriteStream. If
    /// not set, view returned will be basic.
    #[prost(enumeration = "WriteStreamView", tag = "3")]
    pub view: i32,
}
/// Request message for `BatchCommitWriteStreams`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCommitWriteStreamsRequest {
    /// Required. Parent table that all the streams should belong to, in the form of
    /// `projects/{project}/datasets/{dataset}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The group of streams that will be committed atomically.
    #[prost(string, repeated, tag = "2")]
    pub write_streams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response message for `BatchCommitWriteStreams`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCommitWriteStreamsResponse {
    /// The time at which streams were committed in microseconds granularity.
    /// This field will only exist when there are no stream errors.
    /// **Note** if this field is not set, it means the commit was not successful.
    #[prost(message, optional, tag = "1")]
    pub commit_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Stream level error if commit failed. Only streams with error will be in
    /// the list.
    /// If empty, there is no error and all streams are committed successfully.
    /// If non empty, certain streams have errors and ZERO stream is committed due
    /// to atomicity guarantee.
    #[prost(message, repeated, tag = "2")]
    pub stream_errors: ::prost::alloc::vec::Vec<StorageError>,
}
/// Request message for invoking `FinalizeWriteStream`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeWriteStreamRequest {
    /// Required. Name of the stream to finalize, in the form of
    /// `projects/{project}/datasets/{dataset}/tables/{table}/streams/{stream}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for `FinalizeWriteStream`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeWriteStreamResponse {
    /// Number of rows in the finalized stream.
    #[prost(int64, tag = "1")]
    pub row_count: i64,
}
/// Request message for `FlushRows`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlushRowsRequest {
    /// Required. The stream that is the target of the flush operation.
    #[prost(string, tag = "1")]
    pub write_stream: ::prost::alloc::string::String,
    /// Ending offset of the flush operation. Rows before this offset(including
    /// this offset) will be flushed.
    #[prost(message, optional, tag = "2")]
    pub offset: ::core::option::Option<i64>,
}
/// Respond message for `FlushRows`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlushRowsResponse {
    /// The rows before this offset (including this offset) are flushed.
    #[prost(int64, tag = "1")]
    pub offset: i64,
}
/// Structured custom BigQuery Storage error message. The error can be attached
/// as error details in the returned rpc Status. In particular, the use of error
/// codes allows more structured error handling, and reduces the need to evaluate
/// unstructured error text strings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageError {
    /// BigQuery Storage specific error code.
    #[prost(enumeration = "storage_error::StorageErrorCode", tag = "1")]
    pub code: i32,
    /// Name of the failed entity.
    #[prost(string, tag = "2")]
    pub entity: ::prost::alloc::string::String,
    /// Message that describes the error.
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `StorageError`.
pub mod storage_error {
    /// Error code for `StorageError`.
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
    pub enum StorageErrorCode {
        /// Default error.
        Unspecified = 0,
        /// Table is not found in the system.
        TableNotFound = 1,
        /// Stream is already committed.
        StreamAlreadyCommitted = 2,
        /// Stream is not found.
        StreamNotFound = 3,
        /// Invalid Stream type.
        /// For example, you try to commit a stream that is not pending.
        InvalidStreamType = 4,
        /// Invalid Stream state.
        /// For example, you try to commit a stream that is not finalized or is
        /// garbaged.
        InvalidStreamState = 5,
        /// Stream is finalized.
        StreamFinalized = 6,
        /// There is a schema mismatch and it is caused by user schema has extra
        /// field than bigquery schema.
        SchemaMismatchExtraFields = 7,
        /// Offset already exists.
        OffsetAlreadyExists = 8,
        /// Offset out of range.
        OffsetOutOfRange = 9,
    }
    impl StorageErrorCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StorageErrorCode::Unspecified => "STORAGE_ERROR_CODE_UNSPECIFIED",
                StorageErrorCode::TableNotFound => "TABLE_NOT_FOUND",
                StorageErrorCode::StreamAlreadyCommitted => "STREAM_ALREADY_COMMITTED",
                StorageErrorCode::StreamNotFound => "STREAM_NOT_FOUND",
                StorageErrorCode::InvalidStreamType => "INVALID_STREAM_TYPE",
                StorageErrorCode::InvalidStreamState => "INVALID_STREAM_STATE",
                StorageErrorCode::StreamFinalized => "STREAM_FINALIZED",
                StorageErrorCode::SchemaMismatchExtraFields => {
                    "SCHEMA_MISMATCH_EXTRA_FIELDS"
                }
                StorageErrorCode::OffsetAlreadyExists => "OFFSET_ALREADY_EXISTS",
                StorageErrorCode::OffsetOutOfRange => "OFFSET_OUT_OF_RANGE",
            }
        }
    }
}
/// The message that presents row level error info in a request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowError {
    /// Index of the malformed row in the request.
    #[prost(int64, tag = "1")]
    pub index: i64,
    /// Structured error reason for a row error.
    #[prost(enumeration = "row_error::RowErrorCode", tag = "2")]
    pub code: i32,
    /// Description of the issue encountered when processing the row.
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RowError`.
pub mod row_error {
    /// Error code for `RowError`.
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
    pub enum RowErrorCode {
        /// Default error.
        Unspecified = 0,
        /// One or more fields in the row has errors.
        FieldsError = 1,
    }
    impl RowErrorCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RowErrorCode::Unspecified => "ROW_ERROR_CODE_UNSPECIFIED",
                RowErrorCode::FieldsError => "FIELDS_ERROR",
            }
        }
    }
}
/// Generated client implementations.
pub mod big_query_read_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// BigQuery Read API.
    ///
    /// The Read API can be used to read data from BigQuery.
    #[derive(Debug, Clone)]
    pub struct BigQueryReadClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BigQueryReadClient<T>
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
        ) -> BigQueryReadClient<InterceptedService<T, F>>
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
            BigQueryReadClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new read session. A read session divides the contents of a
        /// BigQuery table into one or more streams, which can then be used to read
        /// data from the table. The read session also specifies properties of the
        /// data to be read, such as a list of columns or a push-down filter describing
        /// the rows to be returned.
        ///
        /// A particular row can be read by at most one stream. When the caller has
        /// reached the end of each stream in the session, then all the data in the
        /// table has been read.
        ///
        /// Data is assigned to each stream such that roughly the same number of
        /// rows can be read from each stream. Because the server-side unit for
        /// assigning data is collections of rows, the API does not guarantee that
        /// each stream will return the same number or rows. Additionally, the
        /// limits are enforced based on the number of pre-filtered rows, so some
        /// filters can lead to lopsided assignments.
        ///
        /// Read sessions automatically expire 6 hours after they are created and do
        /// not require manual clean-up by the caller.
        pub async fn create_read_session(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReadSessionRequest>,
        ) -> Result<tonic::Response<super::ReadSession>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1.BigQueryRead/CreateReadSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Reads rows from the stream in the format prescribed by the ReadSession.
        /// Each response contains one or more table rows, up to a maximum of 100 MiB
        /// per response; read requests which attempt to read individual rows larger
        /// than 100 MiB will fail.
        ///
        /// Each request also returns a set of stream statistics reflecting the current
        /// state of the stream.
        pub async fn read_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRowsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ReadRowsResponse>>,
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
                "/google.cloud.bigquery.storage.v1.BigQueryRead/ReadRows",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Splits a given `ReadStream` into two `ReadStream` objects. These
        /// `ReadStream` objects are referred to as the primary and the residual
        /// streams of the split. The original `ReadStream` can still be read from in
        /// the same manner as before. Both of the returned `ReadStream` objects can
        /// also be read from, and the rows returned by both child streams will be
        /// the same as the rows read from the original stream.
        ///
        /// Moreover, the two child streams will be allocated back-to-back in the
        /// original `ReadStream`. Concretely, it is guaranteed that for streams
        /// original, primary, and residual, that original[0-j] = primary[0-j] and
        /// original[j-n] = residual[0-m] once the streams have been read to
        /// completion.
        pub async fn split_read_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::SplitReadStreamRequest>,
        ) -> Result<tonic::Response<super::SplitReadStreamResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1.BigQueryRead/SplitReadStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod big_query_write_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// BigQuery Write API.
    ///
    /// The Write API can be used to write data to BigQuery.
    ///
    /// For supplementary information about the Write API, see:
    /// https://cloud.google.com/bigquery/docs/write-api
    #[derive(Debug, Clone)]
    pub struct BigQueryWriteClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BigQueryWriteClient<T>
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
        ) -> BigQueryWriteClient<InterceptedService<T, F>>
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
            BigQueryWriteClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a write stream to the given table.
        /// Additionally, every table has a special stream named '_default'
        /// to which data can be written. This stream doesn't need to be created using
        /// CreateWriteStream. It is a stream that can be used simultaneously by any
        /// number of clients. Data written to this stream is considered committed as
        /// soon as an acknowledgement is received.
        pub async fn create_write_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWriteStreamRequest>,
        ) -> Result<tonic::Response<super::WriteStream>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1.BigQueryWrite/CreateWriteStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Appends data to the given stream.
        ///
        /// If `offset` is specified, the `offset` is checked against the end of
        /// stream. The server returns `OUT_OF_RANGE` in `AppendRowsResponse` if an
        /// attempt is made to append to an offset beyond the current end of the stream
        /// or `ALREADY_EXISTS` if user provides an `offset` that has already been
        /// written to. User can retry with adjusted offset within the same RPC
        /// connection. If `offset` is not specified, append happens at the end of the
        /// stream.
        ///
        /// The response contains an optional offset at which the append
        /// happened.  No offset information will be returned for appends to a
        /// default stream.
        ///
        /// Responses are received in the same order in which requests are sent.
        /// There will be one response for each successful inserted request.  Responses
        /// may optionally embed error information if the originating AppendRequest was
        /// not successfully processed.
        ///
        /// The specifics of when successfully appended data is made visible to the
        /// table are governed by the type of stream:
        ///
        /// * For COMMITTED streams (which includes the default stream), data is
        /// visible immediately upon successful append.
        ///
        /// * For BUFFERED streams, data is made visible via a subsequent `FlushRows`
        /// rpc which advances a cursor to a newer offset in the stream.
        ///
        /// * For PENDING streams, data is not made visible until the stream itself is
        /// finalized (via the `FinalizeWriteStream` rpc), and the stream is explicitly
        /// committed via the `BatchCommitWriteStreams` rpc.
        pub async fn append_rows(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::AppendRowsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::AppendRowsResponse>>,
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
                "/google.cloud.bigquery.storage.v1.BigQueryWrite/AppendRows",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /// Gets information about a write stream.
        pub async fn get_write_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWriteStreamRequest>,
        ) -> Result<tonic::Response<super::WriteStream>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1.BigQueryWrite/GetWriteStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Finalize a write stream so that no new data can be appended to the
        /// stream. Finalize is not supported on the '_default' stream.
        pub async fn finalize_write_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::FinalizeWriteStreamRequest>,
        ) -> Result<tonic::Response<super::FinalizeWriteStreamResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1.BigQueryWrite/FinalizeWriteStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Atomically commits a group of `PENDING` streams that belong to the same
        /// `parent` table.
        ///
        /// Streams must be finalized before commit and cannot be committed multiple
        /// times. Once a stream is committed, data in the stream becomes available
        /// for read operations.
        pub async fn batch_commit_write_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCommitWriteStreamsRequest>,
        ) -> Result<
            tonic::Response<super::BatchCommitWriteStreamsResponse>,
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
                "/google.cloud.bigquery.storage.v1.BigQueryWrite/BatchCommitWriteStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Flushes rows to a BUFFERED stream.
        ///
        /// If users are appending rows to BUFFERED stream, flush operation is
        /// required in order for the rows to become available for reading. A
        /// Flush operation flushes up to any previously flushed offset in a BUFFERED
        /// stream, to the offset specified in the request.
        ///
        /// Flush is not supported on the _default stream, since it is not BUFFERED.
        pub async fn flush_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::FlushRowsRequest>,
        ) -> Result<tonic::Response<super::FlushRowsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1.BigQueryWrite/FlushRows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
