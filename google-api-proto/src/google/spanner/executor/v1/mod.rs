/// Request to executor service that start a new Spanner action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpannerAsyncActionRequest {
    /// Action id to uniquely identify this action request.
    #[prost(int32, tag = "1")]
    pub action_id: i32,
    /// The actual SpannerAction to perform.
    #[prost(message, optional, tag = "2")]
    pub action: ::core::option::Option<SpannerAction>,
}
/// Response from executor service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpannerAsyncActionResponse {
    /// Action id corresponds to the request.
    #[prost(int32, tag = "1")]
    pub action_id: i32,
    /// If action results are split into multiple responses, only the last response
    /// can and should contain status.
    #[prost(message, optional, tag = "2")]
    pub outcome: ::core::option::Option<SpannerActionOutcome>,
}
/// SpannerAction defines a primitive action that can be performed against
/// Spanner, such as begin or commit a transaction, or perform a read or
/// mutation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpannerAction {
    /// Database against which to perform action.
    /// In a context where a series of actions take place, an action may omit
    /// database path if it applies to the same database as the previous action.
    #[prost(string, tag = "1")]
    pub database_path: ::prost::alloc::string::String,
    /// Action represents a spanner action kind, there will only be one action kind
    /// per SpannerAction.
    #[prost(
        oneof = "spanner_action::Action",
        tags = "10, 11, 20, 21, 22, 23, 24, 25, 27, 30, 40, 41, 42, 43, 44, 50"
    )]
    pub action: ::core::option::Option<spanner_action::Action>,
}
/// Nested message and enum types in `SpannerAction`.
pub mod spanner_action {
    /// Action represents a spanner action kind, there will only be one action kind
    /// per SpannerAction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// Action to start a transaction.
        #[prost(message, tag = "10")]
        Start(super::StartTransactionAction),
        /// Action to finish a transaction.
        #[prost(message, tag = "11")]
        Finish(super::FinishTransactionAction),
        /// Action to do a normal read.
        #[prost(message, tag = "20")]
        Read(super::ReadAction),
        /// Action to do a query.
        #[prost(message, tag = "21")]
        Query(super::QueryAction),
        /// Action to buffer a mutation.
        #[prost(message, tag = "22")]
        Mutation(super::MutationAction),
        /// Action to a DML.
        #[prost(message, tag = "23")]
        Dml(super::DmlAction),
        /// Action to a batch DML.
        #[prost(message, tag = "24")]
        BatchDml(super::BatchDmlAction),
        /// Action to write a mutation.
        #[prost(message, tag = "25")]
        Write(super::WriteMutationsAction),
        /// Action to a partitioned update.
        #[prost(message, tag = "27")]
        PartitionedUpdate(super::PartitionedUpdateAction),
        /// Action that contains any administrative operation, like database,
        /// instance manipulation.
        #[prost(message, tag = "30")]
        Admin(super::AdminAction),
        /// Action to start a batch transaction.
        #[prost(message, tag = "40")]
        StartBatchTxn(super::StartBatchTransactionAction),
        /// Action to close a batch transaction.
        #[prost(message, tag = "41")]
        CloseBatchTxn(super::CloseBatchTransactionAction),
        /// Action to generate database partitions for batch read.
        #[prost(message, tag = "42")]
        GenerateDbPartitionsRead(super::GenerateDbPartitionsForReadAction),
        /// Action to generate database partitions for batch query.
        #[prost(message, tag = "43")]
        GenerateDbPartitionsQuery(super::GenerateDbPartitionsForQueryAction),
        /// Action to execute batch actions on generated partitions.
        #[prost(message, tag = "44")]
        ExecutePartition(super::ExecutePartitionAction),
        /// Action to execute change stream query.
        #[prost(message, tag = "50")]
        ExecuteChangeStreamQuery(super::ExecuteChangeStreamQuery),
    }
}
/// A single read request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAction {
    /// The table to read at.
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    /// The index to read at if it's an index read.
    #[prost(string, optional, tag = "2")]
    pub index: ::core::option::Option<::prost::alloc::string::String>,
    /// List of columns must begin with the key columns used for the read.
    #[prost(string, repeated, tag = "3")]
    pub column: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Keys for performing this read.
    #[prost(message, optional, tag = "4")]
    pub keys: ::core::option::Option<KeySet>,
    /// Limit on number of rows to read. If set, must be positive.
    #[prost(int32, tag = "5")]
    pub limit: i32,
}
/// A SQL query request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAction {
    /// The SQL string.
    #[prost(string, tag = "1")]
    pub sql: ::prost::alloc::string::String,
    /// Parameters for the SQL string.
    #[prost(message, repeated, tag = "2")]
    pub params: ::prost::alloc::vec::Vec<query_action::Parameter>,
}
/// Nested message and enum types in `QueryAction`.
pub mod query_action {
    /// Parameter that bind to placeholders in the SQL string
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Parameter {
        /// Name of the parameter (with no leading @).
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Type of the parameter.
        #[prost(message, optional, tag = "2")]
        pub r#type: ::core::option::Option<super::super::super::v1::Type>,
        /// Value of the parameter.
        #[prost(message, optional, tag = "3")]
        pub value: ::core::option::Option<super::Value>,
    }
}
/// A single DML statement.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmlAction {
    /// DML statement.
    #[prost(message, optional, tag = "1")]
    pub update: ::core::option::Option<QueryAction>,
    /// Whether to autocommit the transaction after executing the DML statement,
    /// if the Executor supports autocommit.
    #[prost(bool, optional, tag = "2")]
    pub autocommit_if_supported: ::core::option::Option<bool>,
}
/// Batch of DML statements invoked using batched execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDmlAction {
    /// DML statements.
    #[prost(message, repeated, tag = "1")]
    pub updates: ::prost::alloc::vec::Vec<QueryAction>,
}
/// Value represents a single value that can be read or written to/from
/// Spanner.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// Type of array element. Only set if value is an array.
    #[prost(message, optional, tag = "12")]
    pub array_type: ::core::option::Option<super::super::v1::Type>,
    /// Exactly one of the following fields will be present.
    #[prost(oneof = "value::ValueType", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub value_type: ::core::option::Option<value::ValueType>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// Exactly one of the following fields will be present.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ValueType {
        /// If is_null is set, then this value is null.
        #[prost(bool, tag = "1")]
        IsNull(bool),
        /// Int type value. It's used for all integer number types, like int32 and
        /// int64.
        #[prost(int64, tag = "2")]
        IntValue(i64),
        /// Bool type value.
        #[prost(bool, tag = "3")]
        BoolValue(bool),
        /// Double type value. It's used for all float point types, like float and
        /// double.
        #[prost(double, tag = "4")]
        DoubleValue(f64),
        /// Bytes type value, stored in CORD. It's also used for PROTO type value.
        #[prost(bytes, tag = "5")]
        BytesValue(::prost::bytes::Bytes),
        /// String type value, stored in CORD.
        #[prost(string, tag = "6")]
        StringValue(::prost::alloc::string::String),
        /// Struct type value. It contains a ValueList representing the values in
        /// this struct.
        #[prost(message, tag = "7")]
        StructValue(super::ValueList),
        /// Timestamp type value.
        #[prost(message, tag = "8")]
        TimestampValue(::prost_types::Timestamp),
        /// Date type value. Date is specified as a number of days since Unix epoch.
        #[prost(int32, tag = "9")]
        DateDaysValue(i32),
        /// If set, holds the sentinel value for the transaction CommitTimestamp.
        #[prost(bool, tag = "10")]
        IsCommitTimestamp(bool),
        /// Array type value. The underlying Valuelist should have values that have
        /// the same type.
        #[prost(message, tag = "11")]
        ArrayValue(super::ValueList),
    }
}
/// KeyRange represents a range of rows in a table or index.
///
/// A range has a start key and an end key. These keys can be open or
/// closed, indicating if the range includes rows with that key.
///
/// Keys are represented by "ValueList", where the ith value in the list
/// corresponds to the ith component of the table or index primary key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyRange {
    /// "start" and "limit" must have the same number of key parts,
    /// though they may name only a prefix of the table or index key.
    /// The start key of this KeyRange.
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<ValueList>,
    /// The end key of this KeyRange.
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<ValueList>,
    /// "start" and "limit" type for this KeyRange.
    #[prost(enumeration = "key_range::Type", optional, tag = "3")]
    pub r#type: ::core::option::Option<i32>,
}
/// Nested message and enum types in `KeyRange`.
pub mod key_range {
    /// Type controls whether "start" and "limit" are open or closed. By default,
    /// "start" is closed, and "limit" is open.
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
        /// "TYPE_UNSPECIFIED" is equivalent to "CLOSED_OPEN".
        Unspecified = 0,
        /// \[start,limit\]
        ClosedClosed = 1,
        /// [start,limit)
        ClosedOpen = 2,
        /// (start,limit]
        OpenClosed = 3,
        /// (start,limit)
        OpenOpen = 4,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::ClosedClosed => "CLOSED_CLOSED",
                Type::ClosedOpen => "CLOSED_OPEN",
                Type::OpenClosed => "OPEN_CLOSED",
                Type::OpenOpen => "OPEN_OPEN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOSED_CLOSED" => Some(Self::ClosedClosed),
                "CLOSED_OPEN" => Some(Self::ClosedOpen),
                "OPEN_CLOSED" => Some(Self::OpenClosed),
                "OPEN_OPEN" => Some(Self::OpenOpen),
                _ => None,
            }
        }
    }
}
/// KeySet defines a collection of Spanner keys and/or key ranges. All
/// the keys are expected to be in the same table. The keys need not be
/// sorted in any particular way.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeySet {
    /// A list of specific keys. Entries in "keys" should have exactly as
    /// many elements as there are columns in the primary or index key
    /// with which this "KeySet" is used.
    #[prost(message, repeated, tag = "1")]
    pub point: ::prost::alloc::vec::Vec<ValueList>,
    /// A list of key ranges.
    #[prost(message, repeated, tag = "2")]
    pub range: ::prost::alloc::vec::Vec<KeyRange>,
    /// For convenience "all" can be set to "true" to indicate that this
    /// "KeySet" matches all keys in the table or index. Note that any keys
    /// specified in "keys" or "ranges" are only yielded once.
    #[prost(bool, tag = "3")]
    pub all: bool,
}
/// List of values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueList {
    /// Values contained in this ValueList.
    #[prost(message, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<Value>,
}
/// A single mutation request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutationAction {
    /// Mods that contained in this mutation.
    #[prost(message, repeated, tag = "1")]
    pub r#mod: ::prost::alloc::vec::Vec<mutation_action::Mod>,
}
/// Nested message and enum types in `MutationAction`.
pub mod mutation_action {
    /// Arguments to Insert, InsertOrUpdate, and Replace operations.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InsertArgs {
        /// The names of the columns to be written.
        #[prost(string, repeated, tag = "1")]
        pub column: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Type information for the "values" entries below.
        #[prost(message, repeated, tag = "2")]
        pub r#type: ::prost::alloc::vec::Vec<super::super::super::v1::Type>,
        /// The values to be written.
        #[prost(message, repeated, tag = "3")]
        pub values: ::prost::alloc::vec::Vec<super::ValueList>,
    }
    /// Arguments to Update.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateArgs {
        /// The columns to be updated. Identical to InsertArgs.column.
        #[prost(string, repeated, tag = "1")]
        pub column: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Type information for "values". Identical to InsertArgs.type.
        #[prost(message, repeated, tag = "2")]
        pub r#type: ::prost::alloc::vec::Vec<super::super::super::v1::Type>,
        /// The values to be updated. Identical to InsertArgs.values.
        #[prost(message, repeated, tag = "3")]
        pub values: ::prost::alloc::vec::Vec<super::ValueList>,
    }
    /// Mod represents the write action that will be perform to a table. Each mod
    /// will specify exactly one action, from insert, update, insert_or_update,
    /// replace and delete.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Mod {
        /// The table to write.
        #[prost(string, tag = "1")]
        pub table: ::prost::alloc::string::String,
        /// Exactly one of the remaining elements may be present.
        /// Insert new rows into "table".
        #[prost(message, optional, tag = "2")]
        pub insert: ::core::option::Option<InsertArgs>,
        /// Update columns stored in existing rows of "table".
        #[prost(message, optional, tag = "3")]
        pub update: ::core::option::Option<UpdateArgs>,
        /// Insert or update existing rows of "table".
        #[prost(message, optional, tag = "4")]
        pub insert_or_update: ::core::option::Option<InsertArgs>,
        /// Replace existing rows of "table".
        #[prost(message, optional, tag = "5")]
        pub replace: ::core::option::Option<InsertArgs>,
        /// Delete rows from "table".
        #[prost(message, optional, tag = "6")]
        pub delete_keys: ::core::option::Option<super::KeySet>,
    }
}
/// WriteMutationAction defines an action of flushing the mutation so they
/// are visible to subsequent operations in the transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteMutationsAction {
    /// The mutation to write.
    #[prost(message, optional, tag = "1")]
    pub mutation: ::core::option::Option<MutationAction>,
}
/// PartitionedUpdateAction defines an action to execute a partitioned DML
/// which runs different partitions in parallel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionedUpdateAction {
    /// Options for partitioned update.
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<
        partitioned_update_action::ExecutePartitionedUpdateOptions,
    >,
    /// Partitioned dml query.
    #[prost(message, optional, tag = "2")]
    pub update: ::core::option::Option<QueryAction>,
}
/// Nested message and enum types in `PartitionedUpdateAction`.
pub mod partitioned_update_action {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecutePartitionedUpdateOptions {
        /// RPC Priority
        #[prost(
            enumeration = "super::super::super::v1::request_options::Priority",
            optional,
            tag = "1"
        )]
        pub rpc_priority: ::core::option::Option<i32>,
        /// Transaction tag
        #[prost(string, optional, tag = "2")]
        pub tag: ::core::option::Option<::prost::alloc::string::String>,
    }
}
/// StartTransactionAction defines an action of initializing a transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartTransactionAction {
    /// Concurrency is for read-only transactions and must be omitted for
    /// read-write transactions.
    #[prost(message, optional, tag = "1")]
    pub concurrency: ::core::option::Option<Concurrency>,
    /// Metadata about tables and columns that will be involved in this
    /// transaction. It is to convert values of key parts correctly.
    #[prost(message, repeated, tag = "2")]
    pub table: ::prost::alloc::vec::Vec<TableMetadata>,
    /// Transaction_seed contains workid and op pair for this transaction, used for
    /// testing.
    #[prost(string, tag = "3")]
    pub transaction_seed: ::prost::alloc::string::String,
    /// Execution options (e.g., whether transaction is opaque, optimistic).
    #[prost(message, optional, tag = "4")]
    pub execution_options: ::core::option::Option<TransactionExecutionOptions>,
}
/// Concurrency for read-only transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Concurrency {
    /// True if exact_timestamp_micros is set, and the chosen timestamp is that of
    /// a snapshot epoch.
    #[prost(bool, tag = "7")]
    pub snapshot_epoch_read: bool,
    /// If set, this is a snapshot epoch read constrained to read only the
    /// specified log scope root table, and its children. Will not be set for full
    /// database epochs.
    #[prost(string, tag = "8")]
    pub snapshot_epoch_root_table: ::prost::alloc::string::String,
    /// Set only when batch is true.
    #[prost(int64, tag = "9")]
    pub batch_read_timestamp_micros: i64,
    /// Concurrency mode set for read-only transactions, exactly one mode below
    /// should be set.
    #[prost(oneof = "concurrency::ConcurrencyMode", tags = "1, 2, 3, 4, 5, 6")]
    pub concurrency_mode: ::core::option::Option<concurrency::ConcurrencyMode>,
}
/// Nested message and enum types in `Concurrency`.
pub mod concurrency {
    /// Concurrency mode set for read-only transactions, exactly one mode below
    /// should be set.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConcurrencyMode {
        /// Indicates a read at a consistent timestamp that is specified relative to
        /// now. That is, if the caller has specified an exact staleness of s
        /// seconds, we will read at now - s.
        #[prost(double, tag = "1")]
        StalenessSeconds(f64),
        /// Indicates a boundedly stale read that reads at a timestamp >= T.
        #[prost(int64, tag = "2")]
        MinReadTimestampMicros(i64),
        /// Indicates a boundedly stale read that is at most N seconds stale.
        #[prost(double, tag = "3")]
        MaxStalenessSeconds(f64),
        /// Indicates a read at a consistent timestamp.
        #[prost(int64, tag = "4")]
        ExactTimestampMicros(i64),
        /// Indicates a strong read, must only be set to true, or unset.
        #[prost(bool, tag = "5")]
        Strong(bool),
        /// Indicates a batch read, must only be set to true, or unset.
        #[prost(bool, tag = "6")]
        Batch(bool),
    }
}
/// TableMetadata contains metadata of a single table.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableMetadata {
    /// Table name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Columns, in the same order as in the schema.
    #[prost(message, repeated, tag = "2")]
    pub column: ::prost::alloc::vec::Vec<ColumnMetadata>,
    /// Keys, in order. Column name is currently not populated.
    #[prost(message, repeated, tag = "3")]
    pub key_column: ::prost::alloc::vec::Vec<ColumnMetadata>,
}
/// ColumnMetadata represents metadata of a single column.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnMetadata {
    /// Column name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Column type.
    #[prost(message, optional, tag = "2")]
    pub r#type: ::core::option::Option<super::super::v1::Type>,
}
/// Options for executing the transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionExecutionOptions {
    /// Whether optimistic concurrency should be used to execute this transaction.
    #[prost(bool, tag = "1")]
    pub optimistic: bool,
}
/// FinishTransactionAction defines an action of finishing a transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishTransactionAction {
    /// Defines how exactly the transaction should be completed, e.g. with
    /// commit or abortion.
    #[prost(enumeration = "finish_transaction_action::Mode", tag = "1")]
    pub mode: i32,
}
/// Nested message and enum types in `FinishTransactionAction`.
pub mod finish_transaction_action {
    /// Mode indicates how the transaction should be finished.
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
        /// "MODE_UNSPECIFIED" is equivalent to "COMMIT".
        Unspecified = 0,
        /// Commit the transaction.
        Commit = 1,
        /// Drop the transaction without committing it.
        Abandon = 2,
    }
    impl Mode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mode::Unspecified => "MODE_UNSPECIFIED",
                Mode::Commit => "COMMIT",
                Mode::Abandon => "ABANDON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "COMMIT" => Some(Self::Commit),
                "ABANDON" => Some(Self::Abandon),
                _ => None,
            }
        }
    }
}
/// AdminAction defines all the cloud spanner admin actions, including
/// instance/database admin ops, backup ops and operation actions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminAction {
    /// Exactly one of the actions below will be performed in AdminAction.
    #[prost(
        oneof = "admin_action::Action",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 27, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 28"
    )]
    pub action: ::core::option::Option<admin_action::Action>,
}
/// Nested message and enum types in `AdminAction`.
pub mod admin_action {
    /// Exactly one of the actions below will be performed in AdminAction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// Action that creates a user instance config.
        #[prost(message, tag = "1")]
        CreateUserInstanceConfig(super::CreateUserInstanceConfigAction),
        /// Action that updates a user instance config.
        #[prost(message, tag = "2")]
        UpdateUserInstanceConfig(super::UpdateUserInstanceConfigAction),
        /// Action that deletes a user instance config.
        #[prost(message, tag = "3")]
        DeleteUserInstanceConfig(super::DeleteUserInstanceConfigAction),
        /// Action that gets a user instance config.
        #[prost(message, tag = "4")]
        GetCloudInstanceConfig(super::GetCloudInstanceConfigAction),
        /// Action that lists user instance configs.
        #[prost(message, tag = "5")]
        ListInstanceConfigs(super::ListCloudInstanceConfigsAction),
        /// Action that creates a Cloud Spanner instance.
        #[prost(message, tag = "6")]
        CreateCloudInstance(super::CreateCloudInstanceAction),
        /// Action that updates a Cloud Spanner instance.
        #[prost(message, tag = "7")]
        UpdateCloudInstance(super::UpdateCloudInstanceAction),
        /// Action that deletes a Cloud Spanner instance.
        #[prost(message, tag = "8")]
        DeleteCloudInstance(super::DeleteCloudInstanceAction),
        /// Action that lists Cloud Spanner instances.
        #[prost(message, tag = "9")]
        ListCloudInstances(super::ListCloudInstancesAction),
        /// Action that retrieves a Cloud Spanner instance.
        #[prost(message, tag = "10")]
        GetCloudInstance(super::GetCloudInstanceAction),
        /// Action that creates a Cloud Spanner database.
        #[prost(message, tag = "11")]
        CreateCloudDatabase(super::CreateCloudDatabaseAction),
        /// Action that updates the schema of a Cloud Spanner database.
        #[prost(message, tag = "12")]
        UpdateCloudDatabaseDdl(super::UpdateCloudDatabaseDdlAction),
        /// Action that updates the schema of a Cloud Spanner database.
        #[prost(message, tag = "27")]
        UpdateCloudDatabase(super::UpdateCloudDatabaseAction),
        /// Action that drops a Cloud Spanner database.
        #[prost(message, tag = "13")]
        DropCloudDatabase(super::DropCloudDatabaseAction),
        /// Action that lists Cloud Spanner databases.
        #[prost(message, tag = "14")]
        ListCloudDatabases(super::ListCloudDatabasesAction),
        /// Action that lists Cloud Spanner database operations.
        #[prost(message, tag = "15")]
        ListCloudDatabaseOperations(super::ListCloudDatabaseOperationsAction),
        /// Action that restores a Cloud Spanner database from a backup.
        #[prost(message, tag = "16")]
        RestoreCloudDatabase(super::RestoreCloudDatabaseAction),
        /// Action that gets a Cloud Spanner database.
        #[prost(message, tag = "17")]
        GetCloudDatabase(super::GetCloudDatabaseAction),
        /// Action that creates a Cloud Spanner database backup.
        #[prost(message, tag = "18")]
        CreateCloudBackup(super::CreateCloudBackupAction),
        /// Action that copies a Cloud Spanner database backup.
        #[prost(message, tag = "19")]
        CopyCloudBackup(super::CopyCloudBackupAction),
        /// Action that gets a Cloud Spanner database backup.
        #[prost(message, tag = "20")]
        GetCloudBackup(super::GetCloudBackupAction),
        /// Action that updates a Cloud Spanner database backup.
        #[prost(message, tag = "21")]
        UpdateCloudBackup(super::UpdateCloudBackupAction),
        /// Action that deletes a Cloud Spanner database backup.
        #[prost(message, tag = "22")]
        DeleteCloudBackup(super::DeleteCloudBackupAction),
        /// Action that lists Cloud Spanner database backups.
        #[prost(message, tag = "23")]
        ListCloudBackups(super::ListCloudBackupsAction),
        /// Action that lists Cloud Spanner database backup operations.
        #[prost(message, tag = "24")]
        ListCloudBackupOperations(super::ListCloudBackupOperationsAction),
        /// Action that gets an operation.
        #[prost(message, tag = "25")]
        GetOperation(super::GetOperationAction),
        /// Action that cancels an operation.
        #[prost(message, tag = "26")]
        CancelOperation(super::CancelOperationAction),
        /// Action that reconfigures a Cloud Spanner database.
        #[prost(message, tag = "28")]
        ReconfigureCloudDatabase(super::ReconfigureCloudDatabaseAction),
    }
}
/// Action that creates a user instance config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserInstanceConfigAction {
    /// User instance config ID (not path), e.g. "custom-config".
    #[prost(string, tag = "1")]
    pub user_config_id: ::prost::alloc::string::String,
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Base config ID, e.g. "test-config".
    #[prost(string, tag = "3")]
    pub base_config_id: ::prost::alloc::string::String,
    /// Replicas that should be included in the user config.
    #[prost(message, repeated, tag = "4")]
    pub replicas: ::prost::alloc::vec::Vec<
        super::super::admin::instance::v1::ReplicaInfo,
    >,
}
/// Action that updates a user instance config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserInstanceConfigAction {
    /// User instance config ID (not path), e.g. "custom-config".
    #[prost(string, tag = "1")]
    pub user_config_id: ::prost::alloc::string::String,
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// The descriptive name for this instance config as it appears in UIs.
    #[prost(string, optional, tag = "3")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    /// labels.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Action that gets a user instance config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCloudInstanceConfigAction {
    /// Instance config ID (not path), e.g. "custom-config".
    #[prost(string, tag = "1")]
    pub instance_config_id: ::prost::alloc::string::String,
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
/// Action that deletes a user instance configs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserInstanceConfigAction {
    /// User instance config ID (not path), e.g. "custom-config".
    #[prost(string, tag = "1")]
    pub user_config_id: ::prost::alloc::string::String,
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
/// Action that lists user instance configs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCloudInstanceConfigsAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Number of instance configs to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, optional, tag = "2")]
    pub page_size: ::core::option::Option<i32>,
    /// If non-empty, "page_token" should contain a next_page_token
    /// from a previous ListInstanceConfigsResponse to the same "parent".
    #[prost(string, optional, tag = "3")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Action that creates a Cloud Spanner instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCloudInstanceAction {
    /// Cloud instance ID (not path), e.g. "test-instance".
    #[prost(string, tag = "1")]
    pub instance_id: ::prost::alloc::string::String,
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Instance config ID, e.g. "test-config".
    #[prost(string, tag = "3")]
    pub instance_config_id: ::prost::alloc::string::String,
    /// Number of nodes (processing_units should not be set or set to 0 if used).
    #[prost(int32, optional, tag = "4")]
    pub node_count: ::core::option::Option<i32>,
    /// Number of processing units (node_count should be set to 0 if used).
    #[prost(int32, optional, tag = "6")]
    pub processing_units: ::core::option::Option<i32>,
    /// The autoscaling config for this instance. If non-empty, an autoscaling
    /// instance will be created (processing_units and node_count should be set to
    /// 0 if used).
    #[prost(message, optional, tag = "7")]
    pub autoscaling_config: ::core::option::Option<
        super::super::admin::instance::v1::AutoscalingConfig,
    >,
    /// labels.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Action that updates a Cloud Spanner instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCloudInstanceAction {
    /// Cloud instance ID (not path), e.g. "test-instance".
    #[prost(string, tag = "1")]
    pub instance_id: ::prost::alloc::string::String,
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// The descriptive name for this instance as it appears in UIs.
    /// Must be unique per project and between 4 and 30 characters in length.
    #[prost(string, optional, tag = "3")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The number of nodes allocated to this instance. At most one of either
    /// node_count or processing_units should be present in the message.
    #[prost(int32, optional, tag = "4")]
    pub node_count: ::core::option::Option<i32>,
    /// The number of processing units allocated to this instance. At most one of
    /// processing_units or node_count should be present in the message.
    #[prost(int32, optional, tag = "5")]
    pub processing_units: ::core::option::Option<i32>,
    /// The autoscaling config for this instance. If non-empty, this instance is
    /// using autoscaling (processing_units and node_count should be set to
    /// 0 if used).
    #[prost(message, optional, tag = "7")]
    pub autoscaling_config: ::core::option::Option<
        super::super::admin::instance::v1::AutoscalingConfig,
    >,
    /// labels.
    #[prost(btree_map = "string, string", tag = "6")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Action that deletes a Cloud Spanner instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCloudInstanceAction {
    /// Cloud instance ID (not path), e.g. "test-instance".
    #[prost(string, tag = "1")]
    pub instance_id: ::prost::alloc::string::String,
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
/// Action that creates a Cloud Spanner database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCloudDatabaseAction {
    /// Cloud instance ID (not path), e.g. "test-instance".
    #[prost(string, tag = "1")]
    pub instance_id: ::prost::alloc::string::String,
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud database ID (not full path), e.g. "db0".
    #[prost(string, tag = "3")]
    pub database_id: ::prost::alloc::string::String,
    /// SDL statements to apply to the new database.
    #[prost(string, repeated, tag = "4")]
    pub sdl_statement: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The KMS key used to encrypt the database to be created if the database
    /// should be CMEK protected.
    #[prost(message, optional, tag = "5")]
    pub encryption_config: ::core::option::Option<
        super::super::admin::database::v1::EncryptionConfig,
    >,
    /// Optional SQL dialect (GOOGLESQL or POSTGRESQL).  Default: GOOGLESQL.
    #[prost(string, optional, tag = "6")]
    pub dialect: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "bytes", optional, tag = "7")]
    pub proto_descriptors: ::core::option::Option<::prost::bytes::Bytes>,
}
/// Action that updates the schema of a Cloud Spanner database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCloudDatabaseDdlAction {
    /// Cloud instance ID (not path), e.g. "test-instance".
    #[prost(string, tag = "1")]
    pub instance_id: ::prost::alloc::string::String,
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud database ID (not full path), e.g. "db0".
    #[prost(string, tag = "3")]
    pub database_id: ::prost::alloc::string::String,
    /// SDL statements to apply to the database.
    #[prost(string, repeated, tag = "4")]
    pub sdl_statement: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Op ID can be used to track progress of the update. If set, it must be
    /// unique per database. If not set, Cloud Spanner will generate operation ID
    /// automatically.
    #[prost(string, tag = "5")]
    pub operation_id: ::prost::alloc::string::String,
    #[prost(bytes = "bytes", optional, tag = "6")]
    pub proto_descriptors: ::core::option::Option<::prost::bytes::Bytes>,
}
/// Action that updates a Cloud Spanner database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCloudDatabaseAction {
    /// Cloud instance ID (not path), e.g. "test-instance".
    #[prost(string, tag = "1")]
    pub instance_id: ::prost::alloc::string::String,
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud database name (not full path), e.g. "db0".
    #[prost(string, tag = "3")]
    pub database_name: ::prost::alloc::string::String,
    /// Updated value of enable_drop_protection, this is the only field that has
    /// supported to be updated.
    #[prost(bool, tag = "4")]
    pub enable_drop_protection: bool,
}
/// Action that drops a Cloud Spanner database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropCloudDatabaseAction {
    /// Cloud instance ID (not path), e.g. "test-instance".
    #[prost(string, tag = "1")]
    pub instance_id: ::prost::alloc::string::String,
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud database ID (not full path), e.g. "db0".
    #[prost(string, tag = "3")]
    pub database_id: ::prost::alloc::string::String,
}
/// Action that reconfigures a Cloud Spanner database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconfigureCloudDatabaseAction {
    /// The fully qualified uri of the database to be reconfigured.
    #[prost(string, optional, tag = "1")]
    pub database_uri: ::core::option::Option<::prost::alloc::string::String>,
    /// The locations of the serving regions, e.g. "asia-south1".
    #[prost(string, repeated, tag = "2")]
    pub serving_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Action that lists Cloud Spanner databases.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCloudDatabasesAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud instance ID (not path) to list databases from, e.g. "test-instance".
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Number of databases to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If non-empty, "page_token" should contain a next_page_token
    /// from a previous ListDatabasesResponse to the same "parent"
    /// and with the same "filter".
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Action that lists Cloud Spanner databases.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCloudInstancesAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// A filter expression that filters what operations are returned in the
    /// response.
    /// The expression must specify the field name, a comparison operator,
    /// and the value that you want to use for filtering.
    /// Refer spanner_instance_admin.proto.ListInstancesRequest for
    /// detail.
    #[prost(string, optional, tag = "2")]
    pub filter: ::core::option::Option<::prost::alloc::string::String>,
    /// Number of instances to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, optional, tag = "3")]
    pub page_size: ::core::option::Option<i32>,
    /// If non-empty, "page_token" should contain a next_page_token
    /// from a previous ListInstancesResponse to the same "parent"
    /// and with the same "filter".
    #[prost(string, optional, tag = "4")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Action that retrieves a Cloud Spanner instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCloudInstanceAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud instance ID (not path) to retrieve the instance from,
    /// e.g. "test-instance".
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
}
/// Action that lists Cloud Spanner database operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCloudDatabaseOperationsAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud instance ID (not path) to list database operations from,
    /// e.g. "test-instance".
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// A filter expression that filters what operations are returned in the
    /// response.
    /// The expression must specify the field name, a comparison operator,
    /// and the value that you want to use for filtering.
    /// Refer spanner_database_admin.proto.ListDatabaseOperationsRequest for
    /// detail.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// Number of databases to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// If non-empty, "page_token" should contain a next_page_token
    /// from a previous ListDatabaseOperationsResponse to the same "parent"
    /// and with the same "filter".
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// Action that restores a Cloud Spanner database from a backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreCloudDatabaseAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud instance ID (not path) containing the backup, e.g. "backup-instance".
    #[prost(string, tag = "2")]
    pub backup_instance_id: ::prost::alloc::string::String,
    /// The id of the backup from which to restore, e.g. "test-backup".
    #[prost(string, tag = "3")]
    pub backup_id: ::prost::alloc::string::String,
    /// Cloud instance ID (not path) containing the database, e.g.
    /// "database-instance".
    #[prost(string, tag = "4")]
    pub database_instance_id: ::prost::alloc::string::String,
    /// The id of the database to create and restore to, e.g. "db0". Note that this
    /// database must not already exist.
    #[prost(string, tag = "5")]
    pub database_id: ::prost::alloc::string::String,
}
/// Action that gets a Cloud Spanner database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCloudDatabaseAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud instance ID (not path), e.g. "test-instance".
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// The id of the database to get, e.g. "db0".
    #[prost(string, tag = "3")]
    pub database_id: ::prost::alloc::string::String,
}
/// Action that creates a Cloud Spanner database backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCloudBackupAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud instance ID (not path), e.g. "test-instance".
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// The id of the backup to be created, e.g. "test-backup".
    #[prost(string, tag = "3")]
    pub backup_id: ::prost::alloc::string::String,
    /// The id of the database from which this backup was
    /// created, e.g. "db0". Note that this needs to be in the
    /// same instance as the backup.
    #[prost(string, tag = "4")]
    pub database_id: ::prost::alloc::string::String,
    /// Output only. The expiration time of the backup, which must be at least 6
    /// hours and at most 366 days from the time the request is received.
    #[prost(message, optional, tag = "5")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The version time of the backup, which must be within the time range of
    /// \[earliest_version_time, NOW\], where earliest_version_time is retrieved by
    /// cloud spanner frontend API (See details: go/cs-pitr-lite-design).
    #[prost(message, optional, tag = "6")]
    pub version_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Action that copies a Cloud Spanner database backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyCloudBackupAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud instance ID (not path), e.g. "test-instance".
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// The id of the backup to be created, e.g. "test-backup".
    #[prost(string, tag = "3")]
    pub backup_id: ::prost::alloc::string::String,
    /// The fully qualified uri of the source backup from which this
    /// backup was copied. eg.
    /// "projects/<project_id>/instances/<instance_id>/backups/<backup_id>".
    #[prost(string, tag = "4")]
    pub source_backup: ::prost::alloc::string::String,
    /// Output only. The expiration time of the backup, which must be at least 6
    /// hours and at most 366 days from the time the request is received.
    #[prost(message, optional, tag = "5")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Action that gets a Cloud Spanner database backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCloudBackupAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud instance ID (not path), e.g. "test-instance".
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// The id of the backup to get, e.g. "test-backup".
    #[prost(string, tag = "3")]
    pub backup_id: ::prost::alloc::string::String,
}
/// Action that updates a Cloud Spanner database backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCloudBackupAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud instance ID (not path), e.g. "test-instance".
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// The id of the backup to update, e.g. "test-backup".
    #[prost(string, tag = "3")]
    pub backup_id: ::prost::alloc::string::String,
    /// Output only. Updated value of expire_time, this is the only field
    /// that supported to be updated.
    #[prost(message, optional, tag = "4")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Action that deletes a Cloud Spanner database backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCloudBackupAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud instance ID (not path), e.g. "test-instance".
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// The id of the backup to delete, e.g. "test-backup".
    #[prost(string, tag = "3")]
    pub backup_id: ::prost::alloc::string::String,
}
/// Action that lists Cloud Spanner database backups.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCloudBackupsAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud instance ID (not path) to list backups from, e.g. "test-instance".
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// A filter expression that filters backups listed in the response.
    /// The expression must specify the field name, a comparison operator,
    /// and the value that you want to use for filtering.
    /// Refer backup.proto.ListBackupsRequest for detail.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// Number of backups to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// If non-empty, "page_token" should contain a next_page_token
    /// from a previous ListBackupsResponse to the same "parent"
    /// and with the same "filter".
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// Action that lists Cloud Spanner database backup operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCloudBackupOperationsAction {
    /// Cloud project ID, e.g. "spanner-cloud-systest".
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Cloud instance ID (not path) to list backup operations from,
    /// e.g. "test-instance".
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// A filter expression that filters what operations are returned in the
    /// response.
    /// The expression must specify the field name, a comparison operator,
    /// and the value that you want to use for filtering.
    /// Refer backup.proto.ListBackupOperationsRequest for detail.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// Number of backups to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// If non-empty, "page_token" should contain a next_page_token
    /// from a previous ListBackupOperationsResponse to the same "parent"
    /// and with the same "filter".
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// Action that gets an operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationAction {
    /// The name of the operation resource.
    #[prost(string, tag = "1")]
    pub operation: ::prost::alloc::string::String,
}
/// Action that cancels an operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOperationAction {
    /// The name of the operation resource to be cancelled.
    #[prost(string, tag = "1")]
    pub operation: ::prost::alloc::string::String,
}
/// Starts a batch read-only transaction in executor. Successful outcomes of this
/// action will contain batch_txn_id--the identificator that can be used to start
/// the same transaction in other Executors to parallelize partition processing.
///
/// Example of a batch read flow:
/// 1. Start batch transaction with a timestamp (StartBatchTransactionAction)
/// 2. Generate database partitions for a read or query
/// (GenerateDbPartitionsForReadAction/GenerateDbPartitionsForQueryAction)
/// 3. Call ExecutePartitionAction for some or all partitions, process rows
/// 4. Clean up the transaction (CloseBatchTransactionAction).
///
/// More sophisticated example, with parallel processing:
/// 1. Start batch transaction with a timestamp (StartBatchTransactionAction),
/// note the returned BatchTransactionId
/// 2. Generate database partitions for a read or query
/// (GenerateDbPartitionsForReadAction/GenerateDbPartitionsForQueryAction)
/// 3. Distribute the partitions over a pool of workers, along with the
/// transaction ID.
///
/// In each worker:
/// 4-1. StartBatchTransactionAction with the given transaction ID
/// 4-2. ExecutePartitionAction for each partition it got, process read results
/// 4-3. Close (not cleanup) the transaction (CloseBatchTransactionAction).
///
/// When all workers are done:
/// 5. Cleanup the transaction (CloseBatchTransactionAction). This can be done
/// either by the last worker to finish the job, or by the main Executor that
/// initialized this transaction in the first place. It is also possible to clean
/// it up with a brand new Executor -- just execute StartBatchTransactionAction
/// with the ID, then clean it up right away.
///
/// Cleaning up is optional, but recommended.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartBatchTransactionAction {
    /// Database role to assume while performing this action. Setting the
    /// database_role will enforce additional role-based access checks on this
    /// action.
    #[prost(string, tag = "3")]
    pub cloud_database_role: ::prost::alloc::string::String,
    /// To start a new transaction, specify an exact timestamp. Alternatively, an
    /// existing batch transaction ID can be used. Either one of two must be
    /// set.
    #[prost(oneof = "start_batch_transaction_action::Param", tags = "1, 2")]
    pub param: ::core::option::Option<start_batch_transaction_action::Param>,
}
/// Nested message and enum types in `StartBatchTransactionAction`.
pub mod start_batch_transaction_action {
    /// To start a new transaction, specify an exact timestamp. Alternatively, an
    /// existing batch transaction ID can be used. Either one of two must be
    /// set.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Param {
        /// The exact timestamp to start the batch transaction.
        #[prost(message, tag = "1")]
        BatchTxnTime(::prost_types::Timestamp),
        /// ID of a batch read-only transaction. It can be used to start the same
        /// batch transaction on multiple executors and parallelize partition
        /// processing.
        #[prost(bytes, tag = "2")]
        Tid(::prost::bytes::Bytes),
    }
}
/// Closes or cleans up the currently opened batch read-only transaction.
///
/// Once a transaction is closed, the Executor can be disposed of or used to
/// start start another transaction. Closing a batch transaction in one Executor
/// doesn't affect the transaction's state in other Executors that also read from
/// it.
///
/// When a transaction is cleaned up, it becomes globally invalid. Cleaning up is
/// optional, but recommended.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseBatchTransactionAction {
    /// Indicates whether the transaction needs to be cleaned up.
    #[prost(bool, tag = "1")]
    pub cleanup: bool,
}
/// Generate database partitions for the given read. Successful outcomes will
/// contain database partitions in the db_partition field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDbPartitionsForReadAction {
    /// Read to generate partitions for.
    #[prost(message, optional, tag = "1")]
    pub read: ::core::option::Option<ReadAction>,
    /// Metadata related to the tables involved in the read.
    #[prost(message, repeated, tag = "2")]
    pub table: ::prost::alloc::vec::Vec<TableMetadata>,
    /// Desired size of data in each partition. Spanner doesn't guarantee to
    /// respect this value.
    #[prost(int64, optional, tag = "3")]
    pub desired_bytes_per_partition: ::core::option::Option<i64>,
    /// If set, the desired max number of partitions. Spanner doesn't guarantee to
    /// respect this value.
    #[prost(int64, optional, tag = "4")]
    pub max_partition_count: ::core::option::Option<i64>,
}
/// Generate database partitions for the given query. Successful outcomes will
/// contain database partitions in the db_partition field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDbPartitionsForQueryAction {
    /// Query to generate partitions for.
    #[prost(message, optional, tag = "1")]
    pub query: ::core::option::Option<QueryAction>,
    /// Desired size of data in each partition. Spanner doesn't guarantee to
    /// respect this value.
    #[prost(int64, optional, tag = "2")]
    pub desired_bytes_per_partition: ::core::option::Option<i64>,
}
/// Identifies a database partition generated for a particular read or query. To
/// read rows from the partition, use ExecutePartitionAction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchPartition {
    /// Serialized Partition instance.
    #[prost(bytes = "bytes", tag = "1")]
    pub partition: ::prost::bytes::Bytes,
    /// The partition token decrypted from partition.
    #[prost(bytes = "bytes", tag = "2")]
    pub partition_token: ::prost::bytes::Bytes,
    /// Table name is set iff the partition was generated for a read (as opposed to
    /// a query).
    #[prost(string, optional, tag = "3")]
    pub table: ::core::option::Option<::prost::alloc::string::String>,
    /// Index name if the partition was generated for an index read.
    #[prost(string, optional, tag = "4")]
    pub index: ::core::option::Option<::prost::alloc::string::String>,
}
/// Performs a read or query for the given partitions. This action must be
/// executed in the context of the same transaction that was used to generate
/// given partitions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutePartitionAction {
    /// Batch partition to execute on.
    #[prost(message, optional, tag = "1")]
    pub partition: ::core::option::Option<BatchPartition>,
}
/// Execute a change stream TVF query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteChangeStreamQuery {
    /// Name for this change stream.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Specifies that records with commit_timestamp greater than or equal to
    /// start_time should be returned.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Specifies that records with commit_timestamp less than or equal to
    /// end_time should be returned.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Specifies which change stream partition to query, based on the content of
    /// child partitions records.
    #[prost(string, optional, tag = "4")]
    pub partition_token: ::core::option::Option<::prost::alloc::string::String>,
    /// Read options for this change stream query.
    #[prost(string, repeated, tag = "5")]
    pub read_options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Determines how frequently a heartbeat ChangeRecord will be returned in case
    /// there are no transactions committed in this partition, in milliseconds.
    #[prost(int32, optional, tag = "6")]
    pub heartbeat_milliseconds: ::core::option::Option<i32>,
    /// Deadline for this change stream query, in seconds.
    #[prost(int64, optional, tag = "7")]
    pub deadline_seconds: ::core::option::Option<i64>,
    /// Database role to assume while performing this action. This should only be
    /// set for cloud requests. Setting the database role will enforce additional
    /// role-based access checks on this action.
    #[prost(string, optional, tag = "8")]
    pub cloud_database_role: ::core::option::Option<::prost::alloc::string::String>,
}
/// SpannerActionOutcome defines a result of execution of a single SpannerAction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpannerActionOutcome {
    /// If an outcome is split into multiple parts, status will be set only in the
    /// last part.
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Transaction timestamp. It must be set for successful committed actions.
    #[prost(message, optional, tag = "2")]
    pub commit_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Result of a ReadAction. This field must be set for ReadActions even if
    /// no rows were read.
    #[prost(message, optional, tag = "3")]
    pub read_result: ::core::option::Option<ReadResult>,
    /// Result of a Query. This field must be set for Queries even if no rows were
    /// read.
    #[prost(message, optional, tag = "4")]
    pub query_result: ::core::option::Option<QueryResult>,
    /// This bit indicates that Spanner has restarted the current transaction. It
    /// means that the client should replay all the reads and writes.
    /// Setting it to true is only valid in the context of a read-write
    /// transaction, as an outcome of a committing FinishTransactionAction.
    #[prost(bool, optional, tag = "5")]
    pub transaction_restarted: ::core::option::Option<bool>,
    /// In successful StartBatchTransactionAction outcomes, this contains the ID of
    /// the transaction.
    #[prost(bytes = "bytes", optional, tag = "6")]
    pub batch_txn_id: ::core::option::Option<::prost::bytes::Bytes>,
    /// Generated database partitions (result of a
    /// GenetageDbPartitionsForReadAction/GenerateDbPartitionsForQueryAction).
    #[prost(message, repeated, tag = "7")]
    pub db_partition: ::prost::alloc::vec::Vec<BatchPartition>,
    /// Result of admin related actions.
    #[prost(message, optional, tag = "8")]
    pub admin_result: ::core::option::Option<AdminResult>,
    /// Stores rows modified by query in single DML or batch DML action.
    /// In case of batch DML action, stores 0 as row count of errored DML query.
    #[prost(int64, repeated, tag = "9")]
    pub dml_rows_modified: ::prost::alloc::vec::Vec<i64>,
    /// Change stream records returned by a change stream query.
    #[prost(message, repeated, tag = "10")]
    pub change_stream_records: ::prost::alloc::vec::Vec<ChangeStreamRecord>,
}
/// AdminResult contains admin action results, for database/backup/operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminResult {
    /// Results of cloud backup related actions.
    #[prost(message, optional, tag = "1")]
    pub backup_response: ::core::option::Option<CloudBackupResponse>,
    /// Results of operation related actions.
    #[prost(message, optional, tag = "2")]
    pub operation_response: ::core::option::Option<OperationResponse>,
    /// Results of database related actions.
    #[prost(message, optional, tag = "3")]
    pub database_response: ::core::option::Option<CloudDatabaseResponse>,
    /// Results of instance related actions.
    #[prost(message, optional, tag = "4")]
    pub instance_response: ::core::option::Option<CloudInstanceResponse>,
    /// Results of instance config related actions.
    #[prost(message, optional, tag = "5")]
    pub instance_config_response: ::core::option::Option<CloudInstanceConfigResponse>,
}
/// CloudBackupResponse contains results returned by cloud backup related
/// actions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudBackupResponse {
    /// List of backups returned by ListCloudBackupsAction.
    #[prost(message, repeated, tag = "1")]
    pub listed_backups: ::prost::alloc::vec::Vec<
        super::super::admin::database::v1::Backup,
    >,
    /// List of operations returned by ListCloudBackupOperationsAction.
    #[prost(message, repeated, tag = "2")]
    pub listed_backup_operations: ::prost::alloc::vec::Vec<
        super::super::super::longrunning::Operation,
    >,
    /// "next_page_token" can be sent in a subsequent list action
    /// to fetch more of the matching data.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Backup returned by GetCloudBackupAction/UpdateCloudBackupAction.
    #[prost(message, optional, tag = "4")]
    pub backup: ::core::option::Option<super::super::admin::database::v1::Backup>,
}
/// OperationResponse contains results returned by operation related actions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationResponse {
    /// List of operations returned by ListOperationsAction.
    #[prost(message, repeated, tag = "1")]
    pub listed_operations: ::prost::alloc::vec::Vec<
        super::super::super::longrunning::Operation,
    >,
    /// "next_page_token" can be sent in a subsequent list action
    /// to fetch more of the matching data.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Operation returned by GetOperationAction.
    #[prost(message, optional, tag = "3")]
    pub operation: ::core::option::Option<super::super::super::longrunning::Operation>,
}
/// CloudInstanceResponse contains results returned by cloud instance related
/// actions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudInstanceResponse {
    /// List of instances returned by ListCloudInstancesAction.
    #[prost(message, repeated, tag = "1")]
    pub listed_instances: ::prost::alloc::vec::Vec<
        super::super::admin::instance::v1::Instance,
    >,
    /// "next_page_token" can be sent in a subsequent list action
    /// to fetch more of the matching data.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Instance returned by GetCloudInstanceAction
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<super::super::admin::instance::v1::Instance>,
}
/// CloudInstanceConfigResponse contains results returned by cloud instance
/// config related actions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudInstanceConfigResponse {
    /// List of instance configs returned by ListCloudInstanceConfigsAction.
    #[prost(message, repeated, tag = "1")]
    pub listed_instance_configs: ::prost::alloc::vec::Vec<
        super::super::admin::instance::v1::InstanceConfig,
    >,
    /// "next_page_token" can be sent in a subsequent list action
    /// to fetch more of the matching data.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Instance config returned by GetCloudInstanceConfigAction.
    #[prost(message, optional, tag = "3")]
    pub instance_config: ::core::option::Option<
        super::super::admin::instance::v1::InstanceConfig,
    >,
}
/// CloudDatabaseResponse contains results returned by cloud database related
/// actions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudDatabaseResponse {
    /// List of databases returned by ListCloudDatabasesAction.
    #[prost(message, repeated, tag = "1")]
    pub listed_databases: ::prost::alloc::vec::Vec<
        super::super::admin::database::v1::Database,
    >,
    /// List of operations returned by ListCloudDatabaseOperationsAction.
    #[prost(message, repeated, tag = "2")]
    pub listed_database_operations: ::prost::alloc::vec::Vec<
        super::super::super::longrunning::Operation,
    >,
    /// "next_page_token" can be sent in a subsequent list action
    /// to fetch more of the matching data.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Database returned by GetCloudDatabaseAction
    #[prost(message, optional, tag = "4")]
    pub database: ::core::option::Option<super::super::admin::database::v1::Database>,
}
/// ReadResult contains rows read.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResult {
    /// Table name.
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    /// Index name, if read from an index.
    #[prost(string, optional, tag = "2")]
    pub index: ::core::option::Option<::prost::alloc::string::String>,
    /// Request index (multiread only).
    #[prost(int32, optional, tag = "3")]
    pub request_index: ::core::option::Option<i32>,
    /// Rows read. Each row is a struct with multiple fields, one for each column
    /// in read result. All rows have the same type.
    #[prost(message, repeated, tag = "4")]
    pub row: ::prost::alloc::vec::Vec<ValueList>,
    /// The type of rows read. It must be set if at least one row was read.
    #[prost(message, optional, tag = "5")]
    pub row_type: ::core::option::Option<super::super::v1::StructType>,
}
/// QueryResult contains result of a Query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResult {
    /// Rows read. Each row is a struct with multiple fields, one for each column
    /// in read result. All rows have the same type.
    #[prost(message, repeated, tag = "1")]
    pub row: ::prost::alloc::vec::Vec<ValueList>,
    /// The type of rows read. It must be set if at least one row was read.
    #[prost(message, optional, tag = "2")]
    pub row_type: ::core::option::Option<super::super::v1::StructType>,
}
/// Raw ChangeStream records.
/// Encodes one of: DataChangeRecord, HeartbeatRecord, ChildPartitionsRecord
/// returned from the ChangeStream API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeStreamRecord {
    /// Record represents one type of the change stream record.
    #[prost(oneof = "change_stream_record::Record", tags = "1, 2, 3")]
    pub record: ::core::option::Option<change_stream_record::Record>,
}
/// Nested message and enum types in `ChangeStreamRecord`.
pub mod change_stream_record {
    /// Record represents one type of the change stream record.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Record {
        /// Data change record.
        #[prost(message, tag = "1")]
        DataChange(super::DataChangeRecord),
        /// Child partitions record.
        #[prost(message, tag = "2")]
        ChildPartition(super::ChildPartitionsRecord),
        /// Heartbeat record.
        #[prost(message, tag = "3")]
        Heartbeat(super::HeartbeatRecord),
    }
}
/// ChangeStream data change record.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataChangeRecord {
    /// The timestamp in which the change was committed.
    #[prost(message, optional, tag = "1")]
    pub commit_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The sequence number for the record within the transaction.
    #[prost(string, tag = "2")]
    pub record_sequence: ::prost::alloc::string::String,
    /// A globally unique string that represents the transaction in which the
    /// change was committed.
    #[prost(string, tag = "3")]
    pub transaction_id: ::prost::alloc::string::String,
    /// Indicates whether this is the last record for a transaction in the current
    /// partition.
    #[prost(bool, tag = "4")]
    pub is_last_record: bool,
    /// Name of the table affected by the change.
    #[prost(string, tag = "5")]
    pub table: ::prost::alloc::string::String,
    /// Column types defined in the schema.
    #[prost(message, repeated, tag = "6")]
    pub column_types: ::prost::alloc::vec::Vec<data_change_record::ColumnType>,
    /// Changes made in the transaction.
    #[prost(message, repeated, tag = "7")]
    pub mods: ::prost::alloc::vec::Vec<data_change_record::Mod>,
    /// Describes the type of change. One of INSERT, UPDATE or DELETE.
    #[prost(string, tag = "8")]
    pub mod_type: ::prost::alloc::string::String,
    /// One of value capture type: NEW_VALUES, OLD_VALUES, OLD_AND_NEW_VALUES.
    #[prost(string, tag = "9")]
    pub value_capture_type: ::prost::alloc::string::String,
    /// Number of records in transactions.
    #[prost(int64, tag = "10")]
    pub record_count: i64,
    /// Number of partitions in transactions.
    #[prost(int64, tag = "11")]
    pub partition_count: i64,
    /// Transaction tag info.
    #[prost(string, tag = "12")]
    pub transaction_tag: ::prost::alloc::string::String,
    /// Whether the transaction is a system transactionn.
    #[prost(bool, tag = "13")]
    pub is_system_transaction: bool,
}
/// Nested message and enum types in `DataChangeRecord`.
pub mod data_change_record {
    /// Column types.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ColumnType {
        /// Column name.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Column type in JSON.
        #[prost(string, tag = "2")]
        pub r#type: ::prost::alloc::string::String,
        /// Whether the column is a primary key column.
        #[prost(bool, tag = "3")]
        pub is_primary_key: bool,
        /// The position of the column as defined in the schema.
        #[prost(int64, tag = "4")]
        pub ordinal_position: i64,
    }
    /// Describes the changes that were made.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Mod {
        /// The primary key values in JSON.
        #[prost(string, tag = "1")]
        pub keys: ::prost::alloc::string::String,
        /// The new values of the changed columns in JSON. Only contain the non-key
        /// columns.
        #[prost(string, tag = "2")]
        pub new_values: ::prost::alloc::string::String,
        /// The old values of the changed columns in JSON. Only contain the non-key
        /// columns.
        #[prost(string, tag = "3")]
        pub old_values: ::prost::alloc::string::String,
    }
}
/// ChangeStream child partition record.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChildPartitionsRecord {
    /// Data change records returned from child partitions in this child partitions
    /// record will have a commit timestamp greater than or equal to start_time.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A monotonically increasing sequence number that can be used to define the
    /// ordering of the child partitions record when there are multiple child
    /// partitions records returned with the same start_time in a particular
    /// partition.
    #[prost(string, tag = "2")]
    pub record_sequence: ::prost::alloc::string::String,
    /// A set of child partitions and their associated information.
    #[prost(message, repeated, tag = "3")]
    pub child_partitions: ::prost::alloc::vec::Vec<
        child_partitions_record::ChildPartition,
    >,
}
/// Nested message and enum types in `ChildPartitionsRecord`.
pub mod child_partitions_record {
    /// A single child partition.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChildPartition {
        /// Partition token string used to identify the child partition in queries.
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// Parent partition tokens of this child partition.
        #[prost(string, repeated, tag = "2")]
        pub parent_partition_tokens: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
}
/// ChangeStream heartbeat record.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatRecord {
    /// Timestamp for this heartbeat check.
    #[prost(message, optional, tag = "1")]
    pub heartbeat_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod spanner_executor_proxy_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service that executes SpannerActions asynchronously.
    #[derive(Debug, Clone)]
    pub struct SpannerExecutorProxyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SpannerExecutorProxyClient<T>
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
        ) -> SpannerExecutorProxyClient<InterceptedService<T, F>>
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
            SpannerExecutorProxyClient::new(InterceptedService::new(inner, interceptor))
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
        /// ExecuteActionAsync is a streaming call that starts executing a new Spanner
        /// action.
        ///
        /// For each request, the server will reply with one or more responses, but
        /// only the last response will contain status in the outcome.
        ///
        /// Responses can be matched to requests by action_id. It is allowed to have
        /// multiple actions in flight--in that case, actions are be executed in
        /// parallel.
        pub async fn execute_action_async(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::SpannerAsyncActionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SpannerAsyncActionResponse>>,
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
                "/google.spanner.executor.v1.SpannerExecutorProxy/ExecuteActionAsync",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.spanner.executor.v1.SpannerExecutorProxy",
                        "ExecuteActionAsync",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
