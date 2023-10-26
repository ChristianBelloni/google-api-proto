/// An individual entry in a log.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogEntry {
    /// Required. The resource name of the log to which this log entry belongs:
    ///
    ///      "projects/\[PROJECT_ID\]/logs/\[LOG_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/logs/\[LOG_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/logs/\[LOG_ID\]"
    ///      "folders/\[FOLDER_ID\]/logs/\[LOG_ID\]"
    ///
    /// A project number may be used in place of PROJECT_ID. The project number is
    /// translated to its corresponding PROJECT_ID internally and the `log_name`
    /// field will contain PROJECT_ID in queries and exports.
    ///
    /// `\[LOG_ID\]` must be URL-encoded within `log_name`. Example:
    /// `"organizations/1234567890/logs/cloudresourcemanager.googleapis.com%2Factivity"`.
    ///
    /// `\[LOG_ID\]` must be less than 512 characters long and can only include the
    /// following characters: upper and lower case alphanumeric characters,
    /// forward-slash, underscore, hyphen, and period.
    ///
    /// For backward compatibility, if `log_name` begins with a forward-slash, such
    /// as `/projects/...`, then the log entry is ingested as usual, but the
    /// forward-slash is removed. Listing the log entry will not show the leading
    /// slash and filtering for a log name with a leading slash will never return
    /// any results.
    #[prost(string, tag = "12")]
    pub log_name: ::prost::alloc::string::String,
    /// Required. The monitored resource that produced this log entry.
    ///
    /// Example: a log entry that reports a database error would be associated with
    /// the monitored resource designating the particular database that reported
    /// the error.
    #[prost(message, optional, tag = "8")]
    pub resource: ::core::option::Option<super::super::api::MonitoredResource>,
    /// Optional. The time the event described by the log entry occurred. This time
    /// is used to compute the log entry's age and to enforce the logs retention
    /// period. If this field is omitted in a new log entry, then Logging assigns
    /// it the current time. Timestamps have nanosecond accuracy, but trailing
    /// zeros in the fractional seconds might be omitted when the timestamp is
    /// displayed.
    ///
    /// Incoming log entries must have timestamps that don't exceed the
    /// [logs retention
    /// period](<https://cloud.google.com/logging/quotas#logs_retention_periods>) in
    /// the past, and that don't exceed 24 hours in the future. Log entries outside
    /// those time boundaries aren't ingested by Logging.
    #[prost(message, optional, tag = "9")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the log entry was received by Logging.
    #[prost(message, optional, tag = "24")]
    pub receive_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The severity of the log entry. The default value is
    /// `LogSeverity.DEFAULT`.
    #[prost(enumeration = "super::r#type::LogSeverity", tag = "10")]
    pub severity: i32,
    /// Optional. A unique identifier for the log entry. If you provide a value,
    /// then Logging considers other log entries in the same project, with the same
    /// `timestamp`, and with the same `insert_id` to be duplicates which are
    /// removed in a single query result. However, there are no guarantees of
    /// de-duplication in the export of logs.
    ///
    /// If the `insert_id` is omitted when writing a log entry, the Logging API
    /// assigns its own unique identifier in this field.
    ///
    /// In queries, the `insert_id` is also used to order log entries that have
    /// the same `log_name` and `timestamp` values.
    #[prost(string, tag = "4")]
    pub insert_id: ::prost::alloc::string::String,
    /// Optional. Information about the HTTP request associated with this log
    /// entry, if applicable.
    #[prost(message, optional, tag = "7")]
    pub http_request: ::core::option::Option<super::r#type::HttpRequest>,
    /// Optional. A map of key, value pairs that provides additional information
    /// about the log entry. The labels can be user-defined or system-defined.
    ///
    /// User-defined labels are arbitrary key, value pairs that you can use to
    /// classify logs.
    ///
    /// System-defined labels are defined by GCP services for platform logs.
    /// They have two components - a service namespace component and the
    /// attribute name. For example: `compute.googleapis.com/resource_name`.
    ///
    /// Cloud Logging truncates label keys that exceed 512 B and label
    /// values that exceed 64 KB upon their associated log entry being
    /// written. The truncation is indicated by an ellipsis at the
    /// end of the character string.
    #[prost(btree_map = "string, string", tag = "11")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. Information about an operation associated with the log entry, if
    /// applicable.
    #[prost(message, optional, tag = "15")]
    pub operation: ::core::option::Option<LogEntryOperation>,
    /// Optional. The REST resource name of the trace being written to
    /// [Cloud Trace](<https://cloud.google.com/trace>) in
    /// association with this log entry. For example, if your trace data is stored
    /// in the Cloud project "my-trace-project" and if the service that is creating
    /// the log entry receives a trace header that includes the trace ID "12345",
    /// then the service should use "projects/my-tracing-project/traces/12345".
    ///
    /// The `trace` field provides the link between logs and traces. By using
    /// this field, you can navigate from a log entry to a trace.
    #[prost(string, tag = "22")]
    pub trace: ::prost::alloc::string::String,
    /// Optional. The ID of the [Cloud Trace](<https://cloud.google.com/trace>) span
    /// associated with the current operation in which the log is being written.
    /// For example, if a span has the REST resource name of
    /// "projects/some-project/traces/some-trace/spans/some-span-id", then the
    /// `span_id` field is "some-span-id".
    ///
    /// A
    /// [Span](<https://cloud.google.com/trace/docs/reference/v2/rest/v2/projects.traces/batchWrite#Span>)
    /// represents a single operation within a trace. Whereas a trace may involve
    /// multiple different microservices running on multiple different machines,
    /// a span generally corresponds to a single logical operation being performed
    /// in a single instance of a microservice on one specific machine. Spans
    /// are the nodes within the tree that is a trace.
    ///
    /// Applications that are [instrumented for
    /// tracing](<https://cloud.google.com/trace/docs/setup>) will generally assign a
    /// new, unique span ID on each incoming request. It is also common to create
    /// and record additional spans corresponding to internal processing elements
    /// as well as issuing requests to dependencies.
    ///
    /// The span ID is expected to be a 16-character, hexadecimal encoding of an
    /// 8-byte array and should not be zero. It should be unique within the trace
    /// and should, ideally, be generated in a manner that is uniformly random.
    ///
    /// Example values:
    ///
    ///    - `000000000000004a`
    ///    - `7a2190356c3fc94b`
    ///    - `0000f00300090021`
    ///    - `d39223e101960076`
    #[prost(string, tag = "27")]
    pub span_id: ::prost::alloc::string::String,
    /// Optional. The sampling decision of the trace associated with the log entry.
    ///
    /// True means that the trace resource name in the `trace` field was sampled
    /// for storage in a trace backend. False means that the trace was not sampled
    /// for storage when this log entry was written, or the sampling decision was
    /// unknown at the time. A non-sampled `trace` value is still useful as a
    /// request correlation identifier. The default is False.
    #[prost(bool, tag = "30")]
    pub trace_sampled: bool,
    /// Optional. Source code location information associated with the log entry,
    /// if any.
    #[prost(message, optional, tag = "23")]
    pub source_location: ::core::option::Option<LogEntrySourceLocation>,
    /// Optional. Information indicating this LogEntry is part of a sequence of
    /// multiple log entries split from a single LogEntry.
    #[prost(message, optional, tag = "35")]
    pub split: ::core::option::Option<LogSplit>,
    /// The log entry payload, which can be one of multiple types.
    #[prost(oneof = "log_entry::Payload", tags = "2, 3, 6")]
    pub payload: ::core::option::Option<log_entry::Payload>,
}
/// Nested message and enum types in `LogEntry`.
pub mod log_entry {
    /// The log entry payload, which can be one of multiple types.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// The log entry payload, represented as a protocol buffer. Some Google
        /// Cloud Platform services use this field for their log entry payloads.
        ///
        /// The following protocol buffer types are supported; user-defined types
        /// are not supported:
        ///
        ///    "type.googleapis.com/google.cloud.audit.AuditLog"
        ///    "type.googleapis.com/google.appengine.logging.v1.RequestLog"
        #[prost(message, tag = "2")]
        ProtoPayload(::prost_types::Any),
        /// The log entry payload, represented as a Unicode string (UTF-8).
        #[prost(string, tag = "3")]
        TextPayload(::prost::alloc::string::String),
        /// The log entry payload, represented as a structure that is
        /// expressed as a JSON object.
        #[prost(message, tag = "6")]
        JsonPayload(::prost_types::Struct),
    }
}
/// Additional information about a potentially long-running operation with which
/// a log entry is associated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogEntryOperation {
    /// Optional. An arbitrary operation identifier. Log entries with the same
    /// identifier are assumed to be part of the same operation.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Optional. An arbitrary producer identifier. The combination of `id` and
    /// `producer` must be globally unique. Examples for `producer`:
    /// `"MyDivision.MyBigCompany.com"`, `"github.com/MyProject/MyApplication"`.
    #[prost(string, tag = "2")]
    pub producer: ::prost::alloc::string::String,
    /// Optional. Set this to True if this is the first log entry in the operation.
    #[prost(bool, tag = "3")]
    pub first: bool,
    /// Optional. Set this to True if this is the last log entry in the operation.
    #[prost(bool, tag = "4")]
    pub last: bool,
}
/// Additional information about the source code location that produced the log
/// entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogEntrySourceLocation {
    /// Optional. Source file name. Depending on the runtime environment, this
    /// might be a simple name or a fully-qualified name.
    #[prost(string, tag = "1")]
    pub file: ::prost::alloc::string::String,
    /// Optional. Line within the source file. 1-based; 0 indicates no line number
    /// available.
    #[prost(int64, tag = "2")]
    pub line: i64,
    /// Optional. Human-readable name of the function or method being invoked, with
    /// optional context such as the class or package name. This information may be
    /// used in contexts such as the logs viewer, where a file and line number are
    /// less meaningful. The format can vary by language. For example:
    /// `qual.if.ied.Class.method` (Java), `dir/package.func` (Go), `function`
    /// (Python).
    #[prost(string, tag = "3")]
    pub function: ::prost::alloc::string::String,
}
/// Additional information used to correlate multiple log entries. Used when a
/// single LogEntry would exceed the Google Cloud Logging size limit and is
/// split across multiple log entries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogSplit {
    /// A globally unique identifier for all log entries in a sequence of split log
    /// entries. All log entries with the same |LogSplit.uid| are assumed to be
    /// part of the same sequence of split log entries.
    #[prost(string, tag = "1")]
    pub uid: ::prost::alloc::string::String,
    /// The index of this LogEntry in the sequence of split log entries. Log
    /// entries are given |index| values 0, 1, ..., n-1 for a sequence of n log
    /// entries.
    #[prost(int32, tag = "2")]
    pub index: i32,
    /// The total number of log entries that the original LogEntry was split into.
    #[prost(int32, tag = "3")]
    pub total_splits: i32,
}
/// The parameters to DeleteLog.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLogRequest {
    /// Required. The resource name of the log to delete:
    ///
    /// * `projects/\[PROJECT_ID\]/logs/\[LOG_ID\]`
    /// * `organizations/\[ORGANIZATION_ID\]/logs/\[LOG_ID\]`
    /// * `billingAccounts/\[BILLING_ACCOUNT_ID\]/logs/\[LOG_ID\]`
    /// * `folders/\[FOLDER_ID\]/logs/\[LOG_ID\]`
    ///
    /// `\[LOG_ID\]` must be URL-encoded. For example,
    /// `"projects/my-project-id/logs/syslog"`,
    /// `"organizations/123/logs/cloudaudit.googleapis.com%2Factivity"`.
    ///
    /// For more information about log names, see
    /// [LogEntry][google.logging.v2.LogEntry].
    #[prost(string, tag = "1")]
    pub log_name: ::prost::alloc::string::String,
}
/// The parameters to WriteLogEntries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteLogEntriesRequest {
    /// Optional. A default log resource name that is assigned to all log entries
    /// in `entries` that do not specify a value for `log_name`:
    ///
    /// * `projects/\[PROJECT_ID\]/logs/\[LOG_ID\]`
    /// * `organizations/\[ORGANIZATION_ID\]/logs/\[LOG_ID\]`
    /// * `billingAccounts/\[BILLING_ACCOUNT_ID\]/logs/\[LOG_ID\]`
    /// * `folders/\[FOLDER_ID\]/logs/\[LOG_ID\]`
    ///
    /// `\[LOG_ID\]` must be URL-encoded. For example:
    ///
    ///      "projects/my-project-id/logs/syslog"
    ///      "organizations/123/logs/cloudaudit.googleapis.com%2Factivity"
    ///
    /// The permission `logging.logEntries.create` is needed on each project,
    /// organization, billing account, or folder that is receiving new log
    /// entries, whether the resource is specified in `logName` or in an
    /// individual log entry.
    #[prost(string, tag = "1")]
    pub log_name: ::prost::alloc::string::String,
    /// Optional. A default monitored resource object that is assigned to all log
    /// entries in `entries` that do not specify a value for `resource`. Example:
    ///
    ///      { "type": "gce_instance",
    ///        "labels": {
    ///          "zone": "us-central1-a", "instance_id": "00000000000000000000" }}
    ///
    /// See [LogEntry][google.logging.v2.LogEntry].
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<super::super::api::MonitoredResource>,
    /// Optional. Default labels that are added to the `labels` field of all log
    /// entries in `entries`. If a log entry already has a label with the same key
    /// as a label in this parameter, then the log entry's label is not changed.
    /// See [LogEntry][google.logging.v2.LogEntry].
    #[prost(btree_map = "string, string", tag = "3")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. The log entries to send to Logging. The order of log
    /// entries in this list does not matter. Values supplied in this method's
    /// `log_name`, `resource`, and `labels` fields are copied into those log
    /// entries in this list that do not include values for their corresponding
    /// fields. For more information, see the
    /// [LogEntry][google.logging.v2.LogEntry] type.
    ///
    /// If the `timestamp` or `insert_id` fields are missing in log entries, then
    /// this method supplies the current time or a unique identifier, respectively.
    /// The supplied values are chosen so that, among the log entries that did not
    /// supply their own values, the entries earlier in the list will sort before
    /// the entries later in the list. See the `entries.list` method.
    ///
    /// Log entries with timestamps that are more than the
    /// [logs retention period](<https://cloud.google.com/logging/quotas>) in
    /// the past or more than 24 hours in the future will not be available when
    /// calling `entries.list`. However, those log entries can still be [exported
    /// with
    /// LogSinks](<https://cloud.google.com/logging/docs/api/tasks/exporting-logs>).
    ///
    /// To improve throughput and to avoid exceeding the
    /// [quota limit](<https://cloud.google.com/logging/quotas>) for calls to
    /// `entries.write`, you should try to include several log entries in this
    /// list, rather than calling this method for each individual log entry.
    #[prost(message, repeated, tag = "4")]
    pub entries: ::prost::alloc::vec::Vec<LogEntry>,
    /// Optional. Whether a batch's valid entries should be written even if some
    /// other entry failed due to a permanent error such as INVALID_ARGUMENT or
    /// PERMISSION_DENIED. If any entry failed, then the response status is the
    /// response status of one of the failed entries. The response will include
    /// error details in `WriteLogEntriesPartialErrors.log_entry_errors` keyed by
    /// the entries' zero-based index in the `entries`. Failed requests for which
    /// no entries are written will not include per-entry errors.
    #[prost(bool, tag = "5")]
    pub partial_success: bool,
    /// Optional. If true, the request should expect normal response, but the
    /// entries won't be persisted nor exported. Useful for checking whether the
    /// logging API endpoints are working properly before sending valuable data.
    #[prost(bool, tag = "6")]
    pub dry_run: bool,
}
/// Result returned from WriteLogEntries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteLogEntriesResponse {}
/// Error details for WriteLogEntries with partial success.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteLogEntriesPartialErrors {
    /// When `WriteLogEntriesRequest.partial_success` is true, records the error
    /// status for entries that were not written due to a permanent error, keyed
    /// by the entry's zero-based index in `WriteLogEntriesRequest.entries`.
    ///
    /// Failed requests for which no entries are written will not include
    /// per-entry errors.
    #[prost(btree_map = "int32, message", tag = "1")]
    pub log_entry_errors: ::prost::alloc::collections::BTreeMap<
        i32,
        super::super::rpc::Status,
    >,
}
/// The parameters to `ListLogEntries`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLogEntriesRequest {
    /// Required. Names of one or more parent resources from which to
    /// retrieve log entries:
    ///
    /// *  `projects/\[PROJECT_ID\]`
    /// *  `organizations/\[ORGANIZATION_ID\]`
    /// *  `billingAccounts/\[BILLING_ACCOUNT_ID\]`
    /// *  `folders/\[FOLDER_ID\]`
    ///
    /// May alternatively be one or more views:
    ///
    ///   * `projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]`
    ///   * `organizations/\[ORGANIZATION_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]`
    ///   * `billingAccounts/\[BILLING_ACCOUNT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]`
    ///   * `folders/\[FOLDER_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]`
    ///
    /// Projects listed in the `project_ids` field are added to this list.
    /// A maximum of 100 resources may be specified in a single request.
    #[prost(string, repeated, tag = "8")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Only log entries that match the filter are returned.  An empty
    /// filter matches all log entries in the resources listed in `resource_names`.
    /// Referencing a parent resource that is not listed in `resource_names` will
    /// cause the filter to return no results. The maximum length of a filter is
    /// 20,000 characters.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. How the results should be sorted.  Presently, the only permitted
    /// values are `"timestamp asc"` (default) and `"timestamp desc"`. The first
    /// option returns entries in order of increasing values of
    /// `LogEntry.timestamp` (oldest first), and the second option returns entries
    /// in order of decreasing timestamps (newest first).  Entries with equal
    /// timestamps are returned in order of their `insert_id` values.
    #[prost(string, tag = "3")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return from this request.
    /// Default is 50. If the value is negative or exceeds 1000, the request is
    /// rejected. The presence of `next_page_token` in the response indicates that
    /// more results might be available.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. If present, then retrieve the next batch of results from the
    /// preceding call to this method.  `page_token` must be the value of
    /// `next_page_token` from the previous response.  The values of other method
    /// parameters should be identical to those in the previous call.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// Result returned from `ListLogEntries`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLogEntriesResponse {
    /// A list of log entries.  If `entries` is empty, `nextPageToken` may still be
    /// returned, indicating that more entries may exist.  See `nextPageToken` for
    /// more information.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<LogEntry>,
    /// If there might be more results than those appearing in this response, then
    /// `nextPageToken` is included.  To get the next set of results, call this
    /// method again using the value of `nextPageToken` as `pageToken`.
    ///
    /// If a value for `next_page_token` appears and the `entries` field is empty,
    /// it means that the search found no log entries so far but it did not have
    /// time to search all the possible log entries.  Retry the method with this
    /// value for `page_token` to continue the search.  Alternatively, consider
    /// speeding up the search by changing your filter to specify a single log name
    /// or resource type, or to narrow the time range of the search.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The parameters to ListMonitoredResourceDescriptors
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMonitoredResourceDescriptorsRequest {
    /// Optional. The maximum number of results to return from this request.
    /// Non-positive values are ignored.  The presence of `nextPageToken` in the
    /// response indicates that more results might be available.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// Optional. If present, then retrieve the next batch of results from the
    /// preceding call to this method.  `pageToken` must be the value of
    /// `nextPageToken` from the previous response.  The values of other method
    /// parameters should be identical to those in the previous call.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Result returned from ListMonitoredResourceDescriptors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMonitoredResourceDescriptorsResponse {
    /// A list of resource descriptors.
    #[prost(message, repeated, tag = "1")]
    pub resource_descriptors: ::prost::alloc::vec::Vec<
        super::super::api::MonitoredResourceDescriptor,
    >,
    /// If there might be more results than those appearing in this response, then
    /// `nextPageToken` is included.  To get the next set of results, call this
    /// method again using the value of `nextPageToken` as `pageToken`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The parameters to ListLogs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLogsRequest {
    /// Required. The resource name to list logs for:
    ///
    /// *  `projects/\[PROJECT_ID\]`
    /// *  `organizations/\[ORGANIZATION_ID\]`
    /// *  `billingAccounts/\[BILLING_ACCOUNT_ID\]`
    /// *  `folders/\[FOLDER_ID\]`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. List of resource names to list logs for:
    ///
    ///   * `projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]`
    ///   * `organizations/\[ORGANIZATION_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]`
    ///   * `billingAccounts/\[BILLING_ACCOUNT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]`
    ///   * `folders/\[FOLDER_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]`
    ///
    /// To support legacy queries, it could also be:
    ///
    /// *  `projects/\[PROJECT_ID\]`
    /// *  `organizations/\[ORGANIZATION_ID\]`
    /// *  `billingAccounts/\[BILLING_ACCOUNT_ID\]`
    /// *  `folders/\[FOLDER_ID\]`
    ///
    /// The resource name in the `parent` field is added to this list.
    #[prost(string, repeated, tag = "8")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The maximum number of results to return from this request.
    /// Non-positive values are ignored.  The presence of `nextPageToken` in the
    /// response indicates that more results might be available.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. If present, then retrieve the next batch of results from the
    /// preceding call to this method.  `pageToken` must be the value of
    /// `nextPageToken` from the previous response.  The values of other method
    /// parameters should be identical to those in the previous call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Result returned from ListLogs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLogsResponse {
    /// A list of log names. For example,
    /// `"projects/my-project/logs/syslog"` or
    /// `"organizations/123/logs/cloudresourcemanager.googleapis.com%2Factivity"`.
    #[prost(string, repeated, tag = "3")]
    pub log_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If there might be more results than those appearing in this response, then
    /// `nextPageToken` is included.  To get the next set of results, call this
    /// method again using the value of `nextPageToken` as `pageToken`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The parameters to `TailLogEntries`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailLogEntriesRequest {
    /// Required. Name of a parent resource from which to retrieve log entries:
    ///
    /// *  `projects/\[PROJECT_ID\]`
    /// *  `organizations/\[ORGANIZATION_ID\]`
    /// *  `billingAccounts/\[BILLING_ACCOUNT_ID\]`
    /// *  `folders/\[FOLDER_ID\]`
    ///
    /// May alternatively be one or more views:
    ///
    ///   * `projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]`
    ///   * `organizations/\[ORGANIZATION_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]`
    ///   * `billingAccounts/\[BILLING_ACCOUNT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]`
    ///   * `folders/\[FOLDER_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]`
    #[prost(string, repeated, tag = "1")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Only log entries that match the filter are returned.  An empty
    /// filter matches all log entries in the resources listed in `resource_names`.
    /// Referencing a parent resource that is not listed in `resource_names` will
    /// cause the filter to return no results. The maximum length of a filter is
    /// 20,000 characters.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The amount of time to buffer log entries at the server before
    /// being returned to prevent out of order results due to late arriving log
    /// entries. Valid values are between 0-60000 milliseconds. Defaults to 2000
    /// milliseconds.
    #[prost(message, optional, tag = "3")]
    pub buffer_window: ::core::option::Option<::prost_types::Duration>,
}
/// Result returned from `TailLogEntries`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailLogEntriesResponse {
    /// A list of log entries. Each response in the stream will order entries with
    /// increasing values of `LogEntry.timestamp`. Ordering is not guaranteed
    /// between separate responses.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<LogEntry>,
    /// If entries that otherwise would have been included in the session were not
    /// sent back to the client, counts of relevant entries omitted from the
    /// session with the reason that they were not included. There will be at most
    /// one of each reason per response. The counts represent the number of
    /// suppressed entries since the last streamed response.
    #[prost(message, repeated, tag = "2")]
    pub suppression_info: ::prost::alloc::vec::Vec<
        tail_log_entries_response::SuppressionInfo,
    >,
}
/// Nested message and enum types in `TailLogEntriesResponse`.
pub mod tail_log_entries_response {
    /// Information about entries that were omitted from the session.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SuppressionInfo {
        /// The reason that entries were omitted from the session.
        #[prost(enumeration = "suppression_info::Reason", tag = "1")]
        pub reason: i32,
        /// A lower bound on the count of entries omitted due to `reason`.
        #[prost(int32, tag = "2")]
        pub suppressed_count: i32,
    }
    /// Nested message and enum types in `SuppressionInfo`.
    pub mod suppression_info {
        /// An indicator of why entries were omitted.
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
        pub enum Reason {
            /// Unexpected default.
            Unspecified = 0,
            /// Indicates suppression occurred due to relevant entries being
            /// received in excess of rate limits. For quotas and limits, see
            /// [Logging API quotas and
            /// limits](<https://cloud.google.com/logging/quotas#api-limits>).
            RateLimit = 1,
            /// Indicates suppression occurred due to the client not consuming
            /// responses quickly enough.
            NotConsumed = 2,
        }
        impl Reason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Reason::Unspecified => "REASON_UNSPECIFIED",
                    Reason::RateLimit => "RATE_LIMIT",
                    Reason::NotConsumed => "NOT_CONSUMED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "REASON_UNSPECIFIED" => Some(Self::Unspecified),
                    "RATE_LIMIT" => Some(Self::RateLimit),
                    "NOT_CONSUMED" => Some(Self::NotConsumed),
                    _ => None,
                }
            }
        }
    }
}
/// Generated client implementations.
pub mod logging_service_v2_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for ingesting and querying logs.
    #[derive(Debug, Clone)]
    pub struct LoggingServiceV2Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LoggingServiceV2Client<T>
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
        ) -> LoggingServiceV2Client<InterceptedService<T, F>>
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
            LoggingServiceV2Client::new(InterceptedService::new(inner, interceptor))
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
        /// Deletes all the log entries in a log for the _Default Log Bucket. The log
        /// reappears if it receives new entries. Log entries written shortly before
        /// the delete operation might not be deleted. Entries received after the
        /// delete operation with a timestamp before the operation will be deleted.
        pub async fn delete_log(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLogRequest>,
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
                "/google.logging.v2.LoggingServiceV2/DeleteLog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.LoggingServiceV2", "DeleteLog"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Writes log entries to Logging. This API method is the
        /// only way to send log entries to Logging. This method
        /// is used, directly or indirectly, by the Logging agent
        /// (fluentd) and all logging libraries configured to use Logging.
        /// A single request may contain log entries for a maximum of 1000
        /// different resources (projects, organizations, billing accounts or
        /// folders)
        pub async fn write_log_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteLogEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WriteLogEntriesResponse>,
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
                "/google.logging.v2.LoggingServiceV2/WriteLogEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.LoggingServiceV2",
                        "WriteLogEntries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists log entries.  Use this method to retrieve log entries that originated
        /// from a project/folder/organization/billing account.  For ways to export log
        /// entries, see [Exporting
        /// Logs](https://cloud.google.com/logging/docs/export).
        pub async fn list_log_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLogEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLogEntriesResponse>,
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
                "/google.logging.v2.LoggingServiceV2/ListLogEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.LoggingServiceV2",
                        "ListLogEntries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the descriptors for monitored resource types used by Logging.
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
                "/google.logging.v2.LoggingServiceV2/ListMonitoredResourceDescriptors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.LoggingServiceV2",
                        "ListMonitoredResourceDescriptors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the logs in projects, organizations, folders, or billing accounts.
        /// Only logs that have entries are listed.
        pub async fn list_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLogsResponse>,
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
                "/google.logging.v2.LoggingServiceV2/ListLogs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.LoggingServiceV2", "ListLogs"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Streaming read of log entries as they are ingested. Until the stream is
        /// terminated, it will continue reading logs.
        pub async fn tail_log_entries(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::TailLogEntriesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::TailLogEntriesResponse>>,
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
                "/google.logging.v2.LoggingServiceV2/TailLogEntries",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.LoggingServiceV2",
                        "TailLogEntries",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Describes a logs-based metric. The value of the metric is the number of log
/// entries that match a logs filter in a given time interval.
///
/// Logs-based metrics can also be used to extract values from logs and create a
/// distribution of the values. The distribution records the statistics of the
/// extracted values along with an optional histogram of the values as specified
/// by the bucket options.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogMetric {
    /// Required. The client-assigned metric identifier.
    /// Examples: `"error_count"`, `"nginx/requests"`.
    ///
    /// Metric identifiers are limited to 100 characters and can include only the
    /// following characters: `A-Z`, `a-z`, `0-9`, and the special characters
    /// `_-.,+!*',()%/`. The forward-slash character (`/`) denotes a hierarchy of
    /// name pieces, and it cannot be the first character of the name.
    ///
    /// This field is the `\[METRIC_ID\]` part of a metric resource name in the
    /// format "projects/\[PROJECT_ID\]/metrics/\[METRIC_ID\]". Example: If the
    /// resource name of a metric is
    /// `"projects/my-project/metrics/nginx%2Frequests"`, this field's value is
    /// `"nginx/requests"`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A description of this metric, which is used in documentation.
    /// The maximum length of the description is 8000 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. An [advanced logs
    /// filter](<https://cloud.google.com/logging/docs/view/advanced_filters>) which
    /// is used to match log entries. Example:
    ///
    ///      "resource.type=gae_app AND severity>=ERROR"
    ///
    /// The maximum length of the filter is 20000 characters.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The resource name of the Log Bucket that owns the Log Metric.
    /// Only Log Buckets in projects are supported. The bucket has to be in the
    /// same project as the metric.
    ///
    /// For example:
    ///
    ///    `projects/my-project/locations/global/buckets/my-bucket`
    ///
    /// If empty, then the Log Metric is considered a non-Bucket Log Metric.
    #[prost(string, tag = "13")]
    pub bucket_name: ::prost::alloc::string::String,
    /// Optional. If set to True, then this metric is disabled and it does not
    /// generate any points.
    #[prost(bool, tag = "12")]
    pub disabled: bool,
    /// Optional. The metric descriptor associated with the logs-based metric.
    /// If unspecified, it uses a default metric descriptor with a DELTA metric
    /// kind, INT64 value type, with no labels and a unit of "1". Such a metric
    /// counts the number of log entries matching the `filter` expression.
    ///
    /// The `name`, `type`, and `description` fields in the `metric_descriptor`
    /// are output only, and is constructed using the `name` and `description`
    /// field in the LogMetric.
    ///
    /// To create a logs-based metric that records a distribution of log values, a
    /// DELTA metric kind with a DISTRIBUTION value type must be used along with
    /// a `value_extractor` expression in the LogMetric.
    ///
    /// Each label in the metric descriptor must have a matching label
    /// name as the key and an extractor expression as the value in the
    /// `label_extractors` map.
    ///
    /// The `metric_kind` and `value_type` fields in the `metric_descriptor` cannot
    /// be updated once initially configured. New labels can be added in the
    /// `metric_descriptor`, but existing labels cannot be modified except for
    /// their description.
    #[prost(message, optional, tag = "5")]
    pub metric_descriptor: ::core::option::Option<super::super::api::MetricDescriptor>,
    /// Optional. A `value_extractor` is required when using a distribution
    /// logs-based metric to extract the values to record from a log entry.
    /// Two functions are supported for value extraction: `EXTRACT(field)` or
    /// `REGEXP_EXTRACT(field, regex)`. The arguments are:
    ///
    ///    1. field: The name of the log entry field from which the value is to be
    ///       extracted.
    ///    2. regex: A regular expression using the Google RE2 syntax
    ///       (<https://github.com/google/re2/wiki/Syntax>) with a single capture
    ///       group to extract data from the specified log entry field. The value
    ///       of the field is converted to a string before applying the regex.
    ///       It is an error to specify a regex that does not include exactly one
    ///       capture group.
    ///
    /// The result of the extraction must be convertible to a double type, as the
    /// distribution always records double values. If either the extraction or
    /// the conversion to double fails, then those values are not recorded in the
    /// distribution.
    ///
    /// Example: `REGEXP_EXTRACT(jsonPayload.request, ".*quantity=(\d+).*")`
    #[prost(string, tag = "6")]
    pub value_extractor: ::prost::alloc::string::String,
    /// Optional. A map from a label key string to an extractor expression which is
    /// used to extract data from a log entry field and assign as the label value.
    /// Each label key specified in the LabelDescriptor must have an associated
    /// extractor expression in this map. The syntax of the extractor expression
    /// is the same as for the `value_extractor` field.
    ///
    /// The extracted value is converted to the type defined in the label
    /// descriptor. If either the extraction or the type conversion fails,
    /// the label will have a default value. The default value for a string
    /// label is an empty string, for an integer label its 0, and for a boolean
    /// label its `false`.
    ///
    /// Note that there are upper bounds on the maximum number of labels and the
    /// number of active time series that are allowed in a project.
    #[prost(btree_map = "string, string", tag = "7")]
    pub label_extractors: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. The `bucket_options` are required when the logs-based metric is
    /// using a DISTRIBUTION value type and it describes the bucket boundaries
    /// used to create a histogram of the extracted values.
    #[prost(message, optional, tag = "8")]
    pub bucket_options: ::core::option::Option<
        super::super::api::distribution::BucketOptions,
    >,
    /// Output only. The creation timestamp of the metric.
    ///
    /// This field may not be present for older metrics.
    #[prost(message, optional, tag = "9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update timestamp of the metric.
    ///
    /// This field may not be present for older metrics.
    #[prost(message, optional, tag = "10")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Deprecated. The API version that created or updated this metric.
    /// The v2 format is used by default and cannot be changed.
    #[deprecated]
    #[prost(enumeration = "log_metric::ApiVersion", tag = "4")]
    pub version: i32,
}
/// Nested message and enum types in `LogMetric`.
pub mod log_metric {
    /// Logging API version.
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
    pub enum ApiVersion {
        /// Logging API v2.
        V2 = 0,
        /// Logging API v1.
        V1 = 1,
    }
    impl ApiVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ApiVersion::V2 => "V2",
                ApiVersion::V1 => "V1",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "V2" => Some(Self::V2),
                "V1" => Some(Self::V1),
                _ => None,
            }
        }
    }
}
/// The parameters to ListLogMetrics.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLogMetricsRequest {
    /// Required. The name of the project containing the metrics:
    ///
    ///      "projects/\[PROJECT_ID\]"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. If present, then retrieve the next batch of results from the
    /// preceding call to this method. `pageToken` must be the value of
    /// `nextPageToken` from the previous response. The values of other method
    /// parameters should be identical to those in the previous call.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return from this request.
    /// Non-positive values are ignored. The presence of `nextPageToken` in the
    /// response indicates that more results might be available.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Result returned from ListLogMetrics.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLogMetricsResponse {
    /// A list of logs-based metrics.
    #[prost(message, repeated, tag = "1")]
    pub metrics: ::prost::alloc::vec::Vec<LogMetric>,
    /// If there might be more results than appear in this response, then
    /// `nextPageToken` is included. To get the next set of results, call this
    /// method again using the value of `nextPageToken` as `pageToken`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The parameters to GetLogMetric.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogMetricRequest {
    /// Required. The resource name of the desired metric:
    ///
    ///      "projects/\[PROJECT_ID\]/metrics/\[METRIC_ID\]"
    #[prost(string, tag = "1")]
    pub metric_name: ::prost::alloc::string::String,
}
/// The parameters to CreateLogMetric.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLogMetricRequest {
    /// Required. The resource name of the project in which to create the metric:
    ///
    ///      "projects/\[PROJECT_ID\]"
    ///
    /// The new metric must be provided in the request.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The new logs-based metric, which must not have an identifier that
    /// already exists.
    #[prost(message, optional, tag = "2")]
    pub metric: ::core::option::Option<LogMetric>,
}
/// The parameters to UpdateLogMetric.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLogMetricRequest {
    /// Required. The resource name of the metric to update:
    ///
    ///      "projects/\[PROJECT_ID\]/metrics/\[METRIC_ID\]"
    ///
    /// The updated metric must be provided in the request and it's
    /// `name` field must be the same as `\[METRIC_ID\]` If the metric
    /// does not exist in `\[PROJECT_ID\]`, then a new metric is created.
    #[prost(string, tag = "1")]
    pub metric_name: ::prost::alloc::string::String,
    /// Required. The updated metric.
    #[prost(message, optional, tag = "2")]
    pub metric: ::core::option::Option<LogMetric>,
}
/// The parameters to DeleteLogMetric.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLogMetricRequest {
    /// Required. The resource name of the metric to delete:
    ///
    ///      "projects/\[PROJECT_ID\]/metrics/\[METRIC_ID\]"
    #[prost(string, tag = "1")]
    pub metric_name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod metrics_service_v2_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for configuring logs-based metrics.
    #[derive(Debug, Clone)]
    pub struct MetricsServiceV2Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MetricsServiceV2Client<T>
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
        ) -> MetricsServiceV2Client<InterceptedService<T, F>>
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
            MetricsServiceV2Client::new(InterceptedService::new(inner, interceptor))
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
        /// Lists logs-based metrics.
        pub async fn list_log_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLogMetricsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLogMetricsResponse>,
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
                "/google.logging.v2.MetricsServiceV2/ListLogMetrics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.MetricsServiceV2",
                        "ListLogMetrics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a logs-based metric.
        pub async fn get_log_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLogMetricRequest>,
        ) -> std::result::Result<tonic::Response<super::LogMetric>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.MetricsServiceV2/GetLogMetric",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.MetricsServiceV2", "GetLogMetric"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a logs-based metric.
        pub async fn create_log_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLogMetricRequest>,
        ) -> std::result::Result<tonic::Response<super::LogMetric>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.MetricsServiceV2/CreateLogMetric",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.MetricsServiceV2",
                        "CreateLogMetric",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates or updates a logs-based metric.
        pub async fn update_log_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLogMetricRequest>,
        ) -> std::result::Result<tonic::Response<super::LogMetric>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.MetricsServiceV2/UpdateLogMetric",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.MetricsServiceV2",
                        "UpdateLogMetric",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a logs-based metric.
        pub async fn delete_log_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLogMetricRequest>,
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
                "/google.logging.v2.MetricsServiceV2/DeleteLogMetric",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.MetricsServiceV2",
                        "DeleteLogMetric",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Configuration for an indexed field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexConfig {
    /// Required. The LogEntry field path to index.
    ///
    /// Note that some paths are automatically indexed, and other paths are not
    /// eligible for indexing. See [indexing documentation](
    /// <https://cloud.google.com/logging/docs/view/advanced-queries#indexed-fields>)
    /// for details.
    ///
    /// For example: `jsonPayload.request.status`
    #[prost(string, tag = "1")]
    pub field_path: ::prost::alloc::string::String,
    /// Required. The type of data in this index.
    #[prost(enumeration = "IndexType", tag = "2")]
    pub r#type: i32,
    /// Output only. The timestamp when the index was last modified.
    ///
    /// This is used to return the timestamp, and will be ignored if supplied
    /// during update.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Describes a repository in which log entries are stored.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogBucket {
    /// Output only. The resource name of the bucket.
    ///
    /// For example:
    ///
    ///    `projects/my-project/locations/global/buckets/my-bucket`
    ///
    /// For a list of supported locations, see [Supported
    /// Regions](<https://cloud.google.com/logging/docs/region-support>)
    ///
    /// For the location of `global` it is unspecified where log entries are
    /// actually stored.
    ///
    /// After a bucket has been created, the location cannot be changed.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Describes this bucket.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The creation timestamp of the bucket. This is not set for any
    /// of the default buckets.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update timestamp of the bucket.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Logs will be retained by default for this amount of time, after which they
    /// will automatically be deleted. The minimum retention period is 1 day. If
    /// this value is set to zero at bucket creation time, the default time of 30
    /// days will be used.
    #[prost(int32, tag = "11")]
    pub retention_days: i32,
    /// Whether the bucket is locked.
    ///
    /// The retention period on a locked bucket cannot be changed. Locked buckets
    /// may only be deleted if they are empty.
    #[prost(bool, tag = "9")]
    pub locked: bool,
    /// Output only. The bucket lifecycle state.
    #[prost(enumeration = "LifecycleState", tag = "12")]
    pub lifecycle_state: i32,
    /// Whether log analytics is enabled for this bucket.
    ///
    /// Once enabled, log analytics features cannot be disabled.
    #[prost(bool, tag = "14")]
    pub analytics_enabled: bool,
    /// Log entry field paths that are denied access in this bucket.
    ///
    /// The following fields and their children are eligible: `textPayload`,
    /// `jsonPayload`, `protoPayload`, `httpRequest`, `labels`, `sourceLocation`.
    ///
    /// Restricting a repeated field will restrict all values. Adding a parent will
    /// block all child fields. (e.g. `foo.bar` will block `foo.bar.baz`)
    #[prost(string, repeated, tag = "15")]
    pub restricted_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of indexed fields and related configuration data.
    #[prost(message, repeated, tag = "17")]
    pub index_configs: ::prost::alloc::vec::Vec<IndexConfig>,
    /// The CMEK settings of the log bucket. If present, new log entries written to
    /// this log bucket are encrypted using the CMEK key provided in this
    /// configuration. If a log bucket has CMEK settings, the CMEK settings cannot
    /// be disabled later by updating the log bucket. Changing the KMS key is
    /// allowed.
    #[prost(message, optional, tag = "19")]
    pub cmek_settings: ::core::option::Option<CmekSettings>,
}
/// Describes a view over log entries in a bucket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogView {
    /// The resource name of the view.
    ///
    /// For example:
    ///
    ///    `projects/my-project/locations/global/buckets/my-bucket/views/my-view`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Describes this view.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The creation timestamp of the view.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update timestamp of the view.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Filter that restricts which log entries in a bucket are visible in this
    /// view.
    ///
    /// Filters are restricted to be a logical AND of ==/!= of any of the
    /// following:
    ///
    ///    - originating project/folder/organization/billing account.
    ///    - resource type
    ///    - log id
    ///
    /// For example:
    ///
    ///    SOURCE("projects/myproject") AND resource.type = "gce_instance"
    ///                                 AND LOG_ID("stdout")
    #[prost(string, tag = "7")]
    pub filter: ::prost::alloc::string::String,
}
/// Describes a sink used to export log entries to one of the following
/// destinations in any project: a Cloud Storage bucket, a BigQuery dataset, a
/// Pub/Sub topic or a Cloud Logging log bucket. A logs filter controls which log
/// entries are exported. The sink must be created within a project,
/// organization, billing account, or folder.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogSink {
    /// Required. The client-assigned sink identifier, unique within the project.
    ///
    /// For example: `"my-syslog-errors-to-pubsub"`. Sink identifiers are limited
    /// to 100 characters and can include only the following characters: upper and
    /// lower-case alphanumeric characters, underscores, hyphens, and periods.
    /// First character has to be alphanumeric.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The export destination:
    ///
    ///      "storage.googleapis.com/\[GCS_BUCKET\]"
    ///      "bigquery.googleapis.com/projects/\[PROJECT_ID\]/datasets/\[DATASET\]"
    ///      "pubsub.googleapis.com/projects/\[PROJECT_ID\]/topics/\[TOPIC_ID\]"
    ///
    /// The sink's `writer_identity`, set when the sink is created, must have
    /// permission to write to the destination or else the log entries are not
    /// exported. For more information, see
    /// [Exporting Logs with
    /// Sinks](<https://cloud.google.com/logging/docs/api/tasks/exporting-logs>).
    #[prost(string, tag = "3")]
    pub destination: ::prost::alloc::string::String,
    /// Optional. An [advanced logs
    /// filter](<https://cloud.google.com/logging/docs/view/advanced-queries>). The
    /// only exported log entries are those that are in the resource owning the
    /// sink and that match the filter.
    ///
    /// For example:
    ///
    ///    `logName="projects/\[PROJECT_ID\]/logs/\[LOG_ID\]" AND severity>=ERROR`
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. A description of this sink.
    ///
    /// The maximum length of the description is 8000 characters.
    #[prost(string, tag = "18")]
    pub description: ::prost::alloc::string::String,
    /// Optional. If set to true, then this sink is disabled and it does not export
    /// any log entries.
    #[prost(bool, tag = "19")]
    pub disabled: bool,
    /// Optional. Log entries that match any of these exclusion filters will not be
    /// exported.
    ///
    /// If a log entry is matched by both `filter` and one of `exclusion_filters`
    /// it will not be exported.
    #[prost(message, repeated, tag = "16")]
    pub exclusions: ::prost::alloc::vec::Vec<LogExclusion>,
    /// Deprecated. This field is unused.
    #[deprecated]
    #[prost(enumeration = "log_sink::VersionFormat", tag = "6")]
    pub output_version_format: i32,
    /// Output only. An IAM identity&mdash;a service account or group&mdash;under
    /// which Cloud Logging writes the exported log entries to the sink's
    /// destination. This field is either set by specifying
    /// `custom_writer_identity` or set automatically by
    /// [sinks.create][google.logging.v2.ConfigServiceV2.CreateSink] and
    /// [sinks.update][google.logging.v2.ConfigServiceV2.UpdateSink] based on the
    /// value of `unique_writer_identity` in those methods.
    ///
    /// Until you grant this identity write-access to the destination, log entry
    /// exports from this sink will fail. For more information, see [Granting
    /// Access for a
    /// Resource](<https://cloud.google.com/iam/docs/granting-roles-to-service-accounts#granting_access_to_a_service_account_for_a_resource>).
    /// Consult the destination service's documentation to determine the
    /// appropriate IAM roles to assign to the identity.
    ///
    /// Sinks that have a destination that is a log bucket in the same project as
    /// the sink cannot have a writer_identity and no additional permissions are
    /// required.
    #[prost(string, tag = "8")]
    pub writer_identity: ::prost::alloc::string::String,
    /// Optional. This field applies only to sinks owned by organizations and
    /// folders. If the field is false, the default, only the logs owned by the
    /// sink's parent resource are available for export. If the field is true, then
    /// log entries from all the projects, folders, and billing accounts contained
    /// in the sink's parent resource are also available for export. Whether a
    /// particular log entry from the children is exported depends on the sink's
    /// filter expression.
    ///
    /// For example, if this field is true, then the filter
    /// `resource.type=gce_instance` would export all Compute Engine VM instance
    /// log entries from all projects in the sink's parent.
    ///
    /// To only export entries from certain child projects, filter on the project
    /// part of the log name:
    ///
    ///    logName:("projects/test-project1/" OR "projects/test-project2/") AND
    ///    resource.type=gce_instance
    #[prost(bool, tag = "9")]
    pub include_children: bool,
    /// Output only. The creation timestamp of the sink.
    ///
    /// This field may not be present for older sinks.
    #[prost(message, optional, tag = "13")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update timestamp of the sink.
    ///
    /// This field may not be present for older sinks.
    #[prost(message, optional, tag = "14")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Destination dependent options.
    #[prost(oneof = "log_sink::Options", tags = "12")]
    pub options: ::core::option::Option<log_sink::Options>,
}
/// Nested message and enum types in `LogSink`.
pub mod log_sink {
    /// Deprecated. This is unused.
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
    pub enum VersionFormat {
        /// An unspecified format version that will default to V2.
        Unspecified = 0,
        /// `LogEntry` version 2 format.
        V2 = 1,
        /// `LogEntry` version 1 format.
        V1 = 2,
    }
    impl VersionFormat {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VersionFormat::Unspecified => "VERSION_FORMAT_UNSPECIFIED",
                VersionFormat::V2 => "V2",
                VersionFormat::V1 => "V1",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VERSION_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
                "V2" => Some(Self::V2),
                "V1" => Some(Self::V1),
                _ => None,
            }
        }
    }
    /// Destination dependent options.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Options {
        /// Optional. Options that affect sinks exporting data to BigQuery.
        #[prost(message, tag = "12")]
        BigqueryOptions(super::BigQueryOptions),
    }
}
/// Describes a BigQuery dataset that was created by a link.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryDataset {
    /// Output only. The full resource name of the BigQuery dataset. The DATASET_ID
    /// will match the ID of the link, so the link must match the naming
    /// restrictions of BigQuery datasets (alphanumeric characters and underscores
    /// only).
    ///
    /// The dataset will have a resource path of
    ///    "bigquery.googleapis.com/projects/\[PROJECT_ID\]/datasets/\[DATASET_ID\]"
    #[prost(string, tag = "1")]
    pub dataset_id: ::prost::alloc::string::String,
}
/// Describes a link connected to an analytics enabled bucket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Link {
    /// The resource name of the link. The name can have up to 100 characters.
    /// A valid link id (at the end of the link name) must only have alphanumeric
    /// characters and underscores within it.
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/links/\[LINK_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/links/\[LINK_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/links/\[LINK_ID\]"
    ///      "folders/\[FOLDER_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/links/\[LINK_ID\]"
    ///
    /// For example:
    ///
    ///    `projects/my-project/locations/global/buckets/my-bucket/links/my_link
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Describes this link.
    ///
    /// The maximum length of the description is 8000 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The creation timestamp of the link.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The resource lifecycle state.
    #[prost(enumeration = "LifecycleState", tag = "4")]
    pub lifecycle_state: i32,
    /// The information of a BigQuery Dataset. When a link is created, a BigQuery
    /// dataset is created along with it, in the same project as the LogBucket it's
    /// linked to. This dataset will also have BigQuery Views corresponding to the
    /// LogViews in the bucket.
    #[prost(message, optional, tag = "5")]
    pub bigquery_dataset: ::core::option::Option<BigQueryDataset>,
}
/// Options that change functionality of a sink exporting data to BigQuery.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryOptions {
    /// Optional. Whether to use [BigQuery's partition
    /// tables](<https://cloud.google.com/bigquery/docs/partitioned-tables>). By
    /// default, Cloud Logging creates dated tables based on the log entries'
    /// timestamps, e.g. syslog_20170523. With partitioned tables the date suffix
    /// is no longer present and [special query
    /// syntax](<https://cloud.google.com/bigquery/docs/querying-partitioned-tables>)
    /// has to be used instead. In both cases, tables are sharded based on UTC
    /// timezone.
    #[prost(bool, tag = "1")]
    pub use_partitioned_tables: bool,
    /// Output only. True if new timestamp column based partitioning is in use,
    /// false if legacy ingestion-time partitioning is in use.
    ///
    /// All new sinks will have this field set true and will use timestamp column
    /// based partitioning. If use_partitioned_tables is false, this value has no
    /// meaning and will be false. Legacy sinks using partitioned tables will have
    /// this field set to false.
    #[prost(bool, tag = "3")]
    pub uses_timestamp_column_partitioning: bool,
}
/// The parameters to `ListBuckets`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBucketsRequest {
    /// Required. The parent resource whose buckets are to be listed:
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/locations/\[LOCATION_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/locations/\[LOCATION_ID\]"
    ///      "folders/\[FOLDER_ID\]/locations/\[LOCATION_ID\]"
    ///
    /// Note: The locations portion of the resource must be specified, but
    /// supplying the character `-` in place of \[LOCATION_ID\] will return all
    /// buckets.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. If present, then retrieve the next batch of results from the
    /// preceding call to this method. `pageToken` must be the value of
    /// `nextPageToken` from the previous response. The values of other method
    /// parameters should be identical to those in the previous call.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return from this request.
    /// Non-positive values are ignored. The presence of `nextPageToken` in the
    /// response indicates that more results might be available.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// The response from ListBuckets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBucketsResponse {
    /// A list of buckets.
    #[prost(message, repeated, tag = "1")]
    pub buckets: ::prost::alloc::vec::Vec<LogBucket>,
    /// If there might be more results than appear in this response, then
    /// `nextPageToken` is included. To get the next set of results, call the same
    /// method again using the value of `nextPageToken` as `pageToken`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The parameters to `CreateBucket`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBucketRequest {
    /// Required. The resource in which to create the log bucket:
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/locations/global"`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A client-assigned identifier such as `"my-bucket"`. Identifiers
    /// are limited to 100 characters and can include only letters, digits,
    /// underscores, hyphens, and periods.
    #[prost(string, tag = "2")]
    pub bucket_id: ::prost::alloc::string::String,
    /// Required. The new bucket. The region specified in the new bucket must be
    /// compliant with any Location Restriction Org Policy. The name field in the
    /// bucket is ignored.
    #[prost(message, optional, tag = "3")]
    pub bucket: ::core::option::Option<LogBucket>,
}
/// The parameters to `UpdateBucket`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBucketRequest {
    /// Required. The full resource name of the bucket to update.
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "folders/\[FOLDER_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/locations/global/buckets/my-bucket"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The updated bucket.
    #[prost(message, optional, tag = "2")]
    pub bucket: ::core::option::Option<LogBucket>,
    /// Required. Field mask that specifies the fields in `bucket` that need an
    /// update. A bucket field will be overwritten if, and only if, it is in the
    /// update mask. `name` and output only fields cannot be updated.
    ///
    /// For a detailed `FieldMask` definition, see:
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    ///
    /// For example: `updateMask=retention_days`
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The parameters to `GetBucket`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBucketRequest {
    /// Required. The resource name of the bucket:
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "folders/\[FOLDER_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/locations/global/buckets/my-bucket"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The parameters to `DeleteBucket`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBucketRequest {
    /// Required. The full resource name of the bucket to delete.
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "folders/\[FOLDER_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/locations/global/buckets/my-bucket"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The parameters to `UndeleteBucket`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteBucketRequest {
    /// Required. The full resource name of the bucket to undelete.
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "folders/\[FOLDER_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/locations/global/buckets/my-bucket"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The parameters to `ListViews`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViewsRequest {
    /// Required. The bucket whose views are to be listed:
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. If present, then retrieve the next batch of results from the
    /// preceding call to this method. `pageToken` must be the value of
    /// `nextPageToken` from the previous response. The values of other method
    /// parameters should be identical to those in the previous call.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return from this request.
    ///
    /// Non-positive values are ignored. The presence of `nextPageToken` in the
    /// response indicates that more results might be available.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// The response from ListViews.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViewsResponse {
    /// A list of views.
    #[prost(message, repeated, tag = "1")]
    pub views: ::prost::alloc::vec::Vec<LogView>,
    /// If there might be more results than appear in this response, then
    /// `nextPageToken` is included. To get the next set of results, call the same
    /// method again using the value of `nextPageToken` as `pageToken`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The parameters to `CreateView`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateViewRequest {
    /// Required. The bucket in which to create the view
    ///
    ///      `"projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"`
    ///
    /// For example:
    ///
    ///    `"projects/my-project/locations/global/buckets/my-bucket"`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A client-assigned identifier such as `"my-view"`. Identifiers are
    /// limited to 100 characters and can include only letters, digits,
    /// underscores, hyphens, and periods.
    #[prost(string, tag = "2")]
    pub view_id: ::prost::alloc::string::String,
    /// Required. The new view.
    #[prost(message, optional, tag = "3")]
    pub view: ::core::option::Option<LogView>,
}
/// The parameters to `UpdateView`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateViewRequest {
    /// Required. The full resource name of the view to update
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/locations/global/buckets/my-bucket/views/my-view"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The updated view.
    #[prost(message, optional, tag = "2")]
    pub view: ::core::option::Option<LogView>,
    /// Optional. Field mask that specifies the fields in `view` that need
    /// an update. A field will be overwritten if, and only if, it is
    /// in the update mask. `name` and output only fields cannot be updated.
    ///
    /// For a detailed `FieldMask` definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    ///
    /// For example: `updateMask=filter`
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The parameters to `GetView`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetViewRequest {
    /// Required. The resource name of the policy:
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/locations/global/buckets/my-bucket/views/my-view"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The parameters to `DeleteView`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteViewRequest {
    /// Required. The full resource name of the view to delete:
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/views/\[VIEW_ID\]"
    ///
    /// For example:
    ///
    ///     `"projects/my-project/locations/global/buckets/my-bucket/views/my-view"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The parameters to `ListSinks`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSinksRequest {
    /// Required. The parent resource whose sinks are to be listed:
    ///
    ///      "projects/\[PROJECT_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]"
    ///      "folders/\[FOLDER_ID\]"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. If present, then retrieve the next batch of results from the
    /// preceding call to this method. `pageToken` must be the value of
    /// `nextPageToken` from the previous response. The values of other method
    /// parameters should be identical to those in the previous call.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return from this request.
    /// Non-positive values are ignored. The presence of `nextPageToken` in the
    /// response indicates that more results might be available.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Result returned from `ListSinks`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSinksResponse {
    /// A list of sinks.
    #[prost(message, repeated, tag = "1")]
    pub sinks: ::prost::alloc::vec::Vec<LogSink>,
    /// If there might be more results than appear in this response, then
    /// `nextPageToken` is included. To get the next set of results, call the same
    /// method again using the value of `nextPageToken` as `pageToken`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The parameters to `GetSink`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSinkRequest {
    /// Required. The resource name of the sink:
    ///
    ///      "projects/\[PROJECT_ID\]/sinks/\[SINK_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/sinks/\[SINK_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/sinks/\[SINK_ID\]"
    ///      "folders/\[FOLDER_ID\]/sinks/\[SINK_ID\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/sinks/my-sink"`
    #[prost(string, tag = "1")]
    pub sink_name: ::prost::alloc::string::String,
}
/// The parameters to `CreateSink`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSinkRequest {
    /// Required. The resource in which to create the sink:
    ///
    ///      "projects/\[PROJECT_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]"
    ///      "folders/\[FOLDER_ID\]"
    ///
    /// For examples:
    ///
    ///    `"projects/my-project"`
    ///    `"organizations/123456789"`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The new sink, whose `name` parameter is a sink identifier that
    /// is not already in use.
    #[prost(message, optional, tag = "2")]
    pub sink: ::core::option::Option<LogSink>,
    /// Optional. Determines the kind of IAM identity returned as `writer_identity`
    /// in the new sink. If this value is omitted or set to false, and if the
    /// sink's parent is a project, then the value returned as `writer_identity` is
    /// the same group or service account used by Cloud Logging before the addition
    /// of writer identities to this API. The sink's destination must be in the
    /// same project as the sink itself.
    ///
    /// If this field is set to true, or if the sink is owned by a non-project
    /// resource such as an organization, then the value of `writer_identity` will
    /// be a unique service account used only for exports from the new sink. For
    /// more information, see `writer_identity` in
    /// [LogSink][google.logging.v2.LogSink].
    #[prost(bool, tag = "3")]
    pub unique_writer_identity: bool,
}
/// The parameters to `UpdateSink`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSinkRequest {
    /// Required. The full resource name of the sink to update, including the
    /// parent resource and the sink identifier:
    ///
    ///      "projects/\[PROJECT_ID\]/sinks/\[SINK_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/sinks/\[SINK_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/sinks/\[SINK_ID\]"
    ///      "folders/\[FOLDER_ID\]/sinks/\[SINK_ID\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/sinks/my-sink"`
    #[prost(string, tag = "1")]
    pub sink_name: ::prost::alloc::string::String,
    /// Required. The updated sink, whose name is the same identifier that appears
    /// as part of `sink_name`.
    #[prost(message, optional, tag = "2")]
    pub sink: ::core::option::Option<LogSink>,
    /// Optional. See [sinks.create][google.logging.v2.ConfigServiceV2.CreateSink]
    /// for a description of this field. When updating a sink, the effect of this
    /// field on the value of `writer_identity` in the updated sink depends on both
    /// the old and new values of this field:
    ///
    /// +   If the old and new values of this field are both false or both true,
    ///      then there is no change to the sink's `writer_identity`.
    /// +   If the old value is false and the new value is true, then
    ///      `writer_identity` is changed to a unique service account.
    /// +   It is an error if the old value is true and the new value is
    ///      set to false or defaulted to false.
    #[prost(bool, tag = "3")]
    pub unique_writer_identity: bool,
    /// Optional. Field mask that specifies the fields in `sink` that need
    /// an update. A sink field will be overwritten if, and only if, it is
    /// in the update mask. `name` and output only fields cannot be updated.
    ///
    /// An empty `updateMask` is temporarily treated as using the following mask
    /// for backwards compatibility purposes:
    ///
    ///    `destination,filter,includeChildren`
    ///
    /// At some point in the future, behavior will be removed and specifying an
    /// empty `updateMask` will be an error.
    ///
    /// For a detailed `FieldMask` definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.FieldMask>
    ///
    /// For example: `updateMask=filter`
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The parameters to `DeleteSink`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSinkRequest {
    /// Required. The full resource name of the sink to delete, including the
    /// parent resource and the sink identifier:
    ///
    ///      "projects/\[PROJECT_ID\]/sinks/\[SINK_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/sinks/\[SINK_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/sinks/\[SINK_ID\]"
    ///      "folders/\[FOLDER_ID\]/sinks/\[SINK_ID\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/sinks/my-sink"`
    #[prost(string, tag = "1")]
    pub sink_name: ::prost::alloc::string::String,
}
/// The parameters to CreateLink.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLinkRequest {
    /// Required. The full resource name of the bucket to create a link for.
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    ///      "folders/\[FOLDER_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The new link.
    #[prost(message, optional, tag = "2")]
    pub link: ::core::option::Option<Link>,
    /// Required. The ID to use for the link. The link_id can have up to 100
    /// characters. A valid link_id must only have alphanumeric characters and
    /// underscores within it.
    #[prost(string, tag = "3")]
    pub link_id: ::prost::alloc::string::String,
}
/// The parameters to DeleteLink.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLinkRequest {
    /// Required. The full resource name of the link to delete.
    ///
    ///   "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/links/\[LINK_ID\]"
    ///    "organizations/\[ORGANIZATION_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/links/\[LINK_ID\]"
    ///    "billingAccounts/\[BILLING_ACCOUNT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/links/\[LINK_ID\]"
    ///    "folders/\[FOLDER_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/links/\[LINK_ID\]"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The parameters to ListLinks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLinksRequest {
    /// Required. The parent resource whose links are to be listed:
    ///
    ///    "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/links/"
    ///    "organizations/\[ORGANIZATION_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/"
    ///    "billingAccounts/\[BILLING_ACCOUNT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/"
    ///    "folders/\[FOLDER_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. If present, then retrieve the next batch of results from the
    /// preceding call to this method. `pageToken` must be the value of
    /// `nextPageToken` from the previous response.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return from this request.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// The response from ListLinks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLinksResponse {
    /// A list of links.
    #[prost(message, repeated, tag = "1")]
    pub links: ::prost::alloc::vec::Vec<Link>,
    /// If there might be more results than those appearing in this response, then
    /// `nextPageToken` is included. To get the next set of results, call the same
    /// method again using the value of `nextPageToken` as `pageToken`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The parameters to GetLink.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLinkRequest {
    /// Required. The resource name of the link:
    ///
    ///    "projects/\[PROJECT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/links/\[LINK_ID\]"
    ///    "organizations/\[ORGANIZATION_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/links/\[LINK_ID\]"
    ///    "billingAccounts/\[BILLING_ACCOUNT_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/links/\[LINK_ID\]"
    ///    "folders/\[FOLDER_ID\]/locations/\[LOCATION_ID\]/buckets/\[BUCKET_ID\]/links/\[LINK_ID\]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Specifies a set of log entries that are filtered out by a sink. If
/// your Google Cloud resource receives a large volume of log entries, you can
/// use exclusions to reduce your chargeable logs. Note that exclusions on
/// organization-level and folder-level sinks don't apply to child resources.
/// Note also that you cannot modify the _Required sink or exclude logs from it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogExclusion {
    /// Required. A client-assigned identifier, such as
    /// `"load-balancer-exclusion"`. Identifiers are limited to 100 characters and
    /// can include only letters, digits, underscores, hyphens, and periods. First
    /// character has to be alphanumeric.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A description of this exclusion.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. An [advanced logs
    /// filter](<https://cloud.google.com/logging/docs/view/advanced-queries>) that
    /// matches the log entries to be excluded. By using the [sample
    /// function](<https://cloud.google.com/logging/docs/view/advanced-queries#sample>),
    /// you can exclude less than 100% of the matching log entries.
    ///
    /// For example, the following query matches 99% of low-severity log entries
    /// from Google Cloud Storage buckets:
    ///
    ///    `resource.type=gcs_bucket severity<ERROR sample(insertId, 0.99)`
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. If set to True, then this exclusion is disabled and it does not
    /// exclude any log entries. You can [update an
    /// exclusion][google.logging.v2.ConfigServiceV2.UpdateExclusion] to change the
    /// value of this field.
    #[prost(bool, tag = "4")]
    pub disabled: bool,
    /// Output only. The creation timestamp of the exclusion.
    ///
    /// This field may not be present for older exclusions.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update timestamp of the exclusion.
    ///
    /// This field may not be present for older exclusions.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The parameters to `ListExclusions`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExclusionsRequest {
    /// Required. The parent resource whose exclusions are to be listed.
    ///
    ///      "projects/\[PROJECT_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]"
    ///      "folders/\[FOLDER_ID\]"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. If present, then retrieve the next batch of results from the
    /// preceding call to this method. `pageToken` must be the value of
    /// `nextPageToken` from the previous response. The values of other method
    /// parameters should be identical to those in the previous call.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return from this request.
    /// Non-positive values are ignored. The presence of `nextPageToken` in the
    /// response indicates that more results might be available.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Result returned from `ListExclusions`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExclusionsResponse {
    /// A list of exclusions.
    #[prost(message, repeated, tag = "1")]
    pub exclusions: ::prost::alloc::vec::Vec<LogExclusion>,
    /// If there might be more results than appear in this response, then
    /// `nextPageToken` is included. To get the next set of results, call the same
    /// method again using the value of `nextPageToken` as `pageToken`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The parameters to `GetExclusion`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExclusionRequest {
    /// Required. The resource name of an existing exclusion:
    ///
    ///      "projects/\[PROJECT_ID\]/exclusions/\[EXCLUSION_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/exclusions/\[EXCLUSION_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/exclusions/\[EXCLUSION_ID\]"
    ///      "folders/\[FOLDER_ID\]/exclusions/\[EXCLUSION_ID\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/exclusions/my-exclusion"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The parameters to `CreateExclusion`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateExclusionRequest {
    /// Required. The parent resource in which to create the exclusion:
    ///
    ///      "projects/\[PROJECT_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]"
    ///      "folders/\[FOLDER_ID\]"
    ///
    /// For examples:
    ///
    ///    `"projects/my-logging-project"`
    ///    `"organizations/123456789"`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The new exclusion, whose `name` parameter is an exclusion name
    /// that is not already used in the parent resource.
    #[prost(message, optional, tag = "2")]
    pub exclusion: ::core::option::Option<LogExclusion>,
}
/// The parameters to `UpdateExclusion`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateExclusionRequest {
    /// Required. The resource name of the exclusion to update:
    ///
    ///      "projects/\[PROJECT_ID\]/exclusions/\[EXCLUSION_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/exclusions/\[EXCLUSION_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/exclusions/\[EXCLUSION_ID\]"
    ///      "folders/\[FOLDER_ID\]/exclusions/\[EXCLUSION_ID\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/exclusions/my-exclusion"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. New values for the existing exclusion. Only the fields specified
    /// in `update_mask` are relevant.
    #[prost(message, optional, tag = "2")]
    pub exclusion: ::core::option::Option<LogExclusion>,
    /// Required. A non-empty list of fields to change in the existing exclusion.
    /// New values for the fields are taken from the corresponding fields in the
    /// [LogExclusion][google.logging.v2.LogExclusion] included in this request.
    /// Fields not mentioned in `update_mask` are not changed and are ignored in
    /// the request.
    ///
    /// For example, to change the filter and description of an exclusion,
    /// specify an `update_mask` of `"filter,description"`.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The parameters to `DeleteExclusion`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteExclusionRequest {
    /// Required. The resource name of an existing exclusion to delete:
    ///
    ///      "projects/\[PROJECT_ID\]/exclusions/\[EXCLUSION_ID\]"
    ///      "organizations/\[ORGANIZATION_ID\]/exclusions/\[EXCLUSION_ID\]"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/exclusions/\[EXCLUSION_ID\]"
    ///      "folders/\[FOLDER_ID\]/exclusions/\[EXCLUSION_ID\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/exclusions/my-exclusion"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The parameters to
/// [GetCmekSettings][google.logging.v2.ConfigServiceV2.GetCmekSettings].
///
/// See [Enabling CMEK for Log
/// Router](<https://cloud.google.com/logging/docs/routing/managed-encryption>) for
/// more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCmekSettingsRequest {
    /// Required. The resource for which to retrieve CMEK settings.
    ///
    ///      "projects/\[PROJECT_ID\]/cmekSettings"
    ///      "organizations/\[ORGANIZATION_ID\]/cmekSettings"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/cmekSettings"
    ///      "folders/\[FOLDER_ID\]/cmekSettings"
    ///
    /// For example:
    ///
    ///    `"organizations/12345/cmekSettings"`
    ///
    /// Note: CMEK for the Log Router can be configured for Google Cloud projects,
    /// folders, organizations and billing accounts. Once configured for an
    /// organization, it applies to all projects and folders in the Google Cloud
    /// organization.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The parameters to
/// [UpdateCmekSettings][google.logging.v2.ConfigServiceV2.UpdateCmekSettings].
///
/// See [Enabling CMEK for Log
/// Router](<https://cloud.google.com/logging/docs/routing/managed-encryption>) for
/// more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCmekSettingsRequest {
    /// Required. The resource name for the CMEK settings to update.
    ///
    ///      "projects/\[PROJECT_ID\]/cmekSettings"
    ///      "organizations/\[ORGANIZATION_ID\]/cmekSettings"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/cmekSettings"
    ///      "folders/\[FOLDER_ID\]/cmekSettings"
    ///
    /// For example:
    ///
    ///    `"organizations/12345/cmekSettings"`
    ///
    /// Note: CMEK for the Log Router can currently only be configured for Google
    /// Cloud organizations. Once configured, it applies to all projects and
    /// folders in the Google Cloud organization.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The CMEK settings to update.
    ///
    /// See [Enabling CMEK for Log
    /// Router](<https://cloud.google.com/logging/docs/routing/managed-encryption>)
    /// for more information.
    #[prost(message, optional, tag = "2")]
    pub cmek_settings: ::core::option::Option<CmekSettings>,
    /// Optional. Field mask identifying which fields from `cmek_settings` should
    /// be updated. A field will be overwritten if and only if it is in the update
    /// mask. Output only fields cannot be updated.
    ///
    /// See [FieldMask][google.protobuf.FieldMask] for more information.
    ///
    /// For example: `"updateMask=kmsKeyName"`
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Describes the customer-managed encryption key (CMEK) settings associated with
/// a project, folder, organization, billing account, or flexible resource.
///
/// Note: CMEK for the Log Router can currently only be configured for Google
/// Cloud organizations. Once configured, it applies to all projects and folders
/// in the Google Cloud organization.
///
/// See [Enabling CMEK for Log
/// Router](<https://cloud.google.com/logging/docs/routing/managed-encryption>) for
/// more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CmekSettings {
    /// Output only. The resource name of the CMEK settings.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The resource name for the configured Cloud KMS key.
    ///
    /// KMS key name format:
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION\]/keyRings/\[KEYRING\]/cryptoKeys/\[KEY\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key"`
    ///
    ///
    ///
    /// To enable CMEK for the Log Router, set this field to a valid
    /// `kms_key_name` for which the associated service account has the required
    /// cloudkms.cryptoKeyEncrypterDecrypter roles assigned for the key.
    ///
    /// The Cloud KMS key used by the Log Router can be updated by changing the
    /// `kms_key_name` to a new valid key name or disabled by setting the key name
    /// to an empty string. Encryption operations that are in progress will be
    /// completed with the key that was in use when they started. Decryption
    /// operations will be completed using the key that was used at the time of
    /// encryption unless access to that key has been revoked.
    ///
    /// To disable CMEK for the Log Router, set this field to an empty string.
    ///
    /// See [Enabling CMEK for Log
    /// Router](<https://cloud.google.com/logging/docs/routing/managed-encryption>)
    /// for more information.
    #[prost(string, tag = "2")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// The CryptoKeyVersion resource name for the configured Cloud KMS key.
    ///
    /// KMS key name format:
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION\]/keyRings/\[KEYRING\]/cryptoKeys/\[KEY\]/cryptoKeyVersions/\[VERSION\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1"`
    ///
    /// This is a read-only field used to convey the specific configured
    /// CryptoKeyVersion of `kms_key` that has been configured. It will be
    /// populated in cases where the CMEK settings are bound to a single key
    /// version.
    ///
    /// If this field is populated, the `kms_key` is tied to a specific
    /// CryptoKeyVersion.
    #[prost(string, tag = "4")]
    pub kms_key_version_name: ::prost::alloc::string::String,
    /// Output only. The service account that will be used by the Log Router to
    /// access your Cloud KMS key.
    ///
    /// Before enabling CMEK for Log Router, you must first assign the
    /// cloudkms.cryptoKeyEncrypterDecrypter role to the service account that
    /// the Log Router will use to access your Cloud KMS key. Use
    /// [GetCmekSettings][google.logging.v2.ConfigServiceV2.GetCmekSettings] to
    /// obtain the service account ID.
    ///
    /// See [Enabling CMEK for Log
    /// Router](<https://cloud.google.com/logging/docs/routing/managed-encryption>)
    /// for more information.
    #[prost(string, tag = "3")]
    pub service_account_id: ::prost::alloc::string::String,
}
/// The parameters to
/// [GetSettings][google.logging.v2.ConfigServiceV2.GetSettings].
///
/// See [Enabling CMEK for Log
/// Router](<https://cloud.google.com/logging/docs/routing/managed-encryption>) for
/// more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingsRequest {
    /// Required. The resource for which to retrieve settings.
    ///
    ///      "projects/\[PROJECT_ID\]/settings"
    ///      "organizations/\[ORGANIZATION_ID\]/settings"
    ///      "billingAccounts/\[BILLING_ACCOUNT_ID\]/settings"
    ///      "folders/\[FOLDER_ID\]/settings"
    ///
    /// For example:
    ///
    ///    `"organizations/12345/settings"`
    ///
    /// Note: Settings for the Log Router can be get for Google Cloud projects,
    /// folders, organizations and billing accounts. Currently it can only be
    /// configured for organizations. Once configured for an organization, it
    /// applies to all projects and folders in the Google Cloud organization.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The parameters to
/// [UpdateSettings][google.logging.v2.ConfigServiceV2.UpdateSettings].
///
/// See [Enabling CMEK for Log
/// Router](<https://cloud.google.com/logging/docs/routing/managed-encryption>) for
/// more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingsRequest {
    /// Required. The resource name for the settings to update.
    ///
    ///      "organizations/\[ORGANIZATION_ID\]/settings"
    ///
    /// For example:
    ///
    ///    `"organizations/12345/settings"`
    ///
    /// Note: Settings for the Log Router can currently only be configured for
    /// Google Cloud organizations. Once configured, it applies to all projects and
    /// folders in the Google Cloud organization.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The settings to update.
    ///
    /// See [Enabling CMEK for Log
    /// Router](<https://cloud.google.com/logging/docs/routing/managed-encryption>)
    /// for more information.
    #[prost(message, optional, tag = "2")]
    pub settings: ::core::option::Option<Settings>,
    /// Optional. Field mask identifying which fields from `settings` should
    /// be updated. A field will be overwritten if and only if it is in the update
    /// mask. Output only fields cannot be updated.
    ///
    /// See [FieldMask][google.protobuf.FieldMask] for more information.
    ///
    /// For example: `"updateMask=kmsKeyName"`
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Describes the settings associated with a project, folder, organization,
/// billing account, or flexible resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Settings {
    /// Output only. The resource name of the settings.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The resource name for the configured Cloud KMS key.
    ///
    /// KMS key name format:
    ///
    ///      "projects/\[PROJECT_ID\]/locations/\[LOCATION\]/keyRings/\[KEYRING\]/cryptoKeys/\[KEY\]"
    ///
    /// For example:
    ///
    ///    `"projects/my-project/locations/us-central1/keyRings/my-ring/cryptoKeys/my-key"`
    ///
    ///
    ///
    /// To enable CMEK for the Log Router, set this field to a valid
    /// `kms_key_name` for which the associated service account has the required
    /// `roles/cloudkms.cryptoKeyEncrypterDecrypter` role assigned for the key.
    ///
    /// The Cloud KMS key used by the Log Router can be updated by changing the
    /// `kms_key_name` to a new valid key name. Encryption operations that are in
    /// progress will be completed with the key that was in use when they started.
    /// Decryption operations will be completed using the key that was used at the
    /// time of encryption unless access to that key has been revoked.
    ///
    /// To disable CMEK for the Log Router, set this field to an empty string.
    ///
    /// See [Enabling CMEK for Log
    /// Router](<https://cloud.google.com/logging/docs/routing/managed-encryption>)
    /// for more information.
    #[prost(string, tag = "2")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// Output only. The service account that will be used by the Log Router to
    /// access your Cloud KMS key.
    ///
    /// Before enabling CMEK for Log Router, you must first assign the role
    /// `roles/cloudkms.cryptoKeyEncrypterDecrypter` to the service account that
    /// the Log Router will use to access your Cloud KMS key. Use
    /// [GetSettings][google.logging.v2.ConfigServiceV2.GetSettings] to
    /// obtain the service account ID.
    ///
    /// See [Enabling CMEK for Log
    /// Router](<https://cloud.google.com/logging/docs/routing/managed-encryption>)
    /// for more information.
    #[prost(string, tag = "3")]
    pub kms_service_account_id: ::prost::alloc::string::String,
    /// Optional. The Cloud region that will be used for _Default and _Required log
    /// buckets for newly created projects and folders. For example `europe-west1`.
    /// This setting does not affect the location of custom log buckets.
    #[prost(string, tag = "4")]
    pub storage_location: ::prost::alloc::string::String,
    /// Optional. If set to true, the _Default sink in newly created projects and
    /// folders will created in a disabled state. This can be used to automatically
    /// disable log ingestion if there is already an aggregated sink configured in
    /// the hierarchy. The _Default sink can be re-enabled manually if needed.
    #[prost(bool, tag = "5")]
    pub disable_default_sink: bool,
}
/// The parameters to CopyLogEntries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyLogEntriesRequest {
    /// Required. Log bucket from which to copy log entries.
    ///
    /// For example:
    ///
    ///    `"projects/my-project/locations/global/buckets/my-source-bucket"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A filter specifying which log entries to copy. The filter must be
    /// no more than 20k characters. An empty filter matches all log entries.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// Required. Destination to which to copy log entries.
    #[prost(string, tag = "4")]
    pub destination: ::prost::alloc::string::String,
}
/// Metadata for CopyLogEntries long running operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyLogEntriesMetadata {
    /// The create time of an operation.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time of an operation.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// State of an operation.
    #[prost(enumeration = "OperationState", tag = "3")]
    pub state: i32,
    /// Identifies whether the user has requested cancellation of the operation.
    #[prost(bool, tag = "4")]
    pub cancellation_requested: bool,
    /// CopyLogEntries RPC request.
    #[prost(message, optional, tag = "5")]
    pub request: ::core::option::Option<CopyLogEntriesRequest>,
    /// Estimated progress of the operation (0 - 100%).
    #[prost(int32, tag = "6")]
    pub progress: i32,
    /// The IAM identity of a service account that must be granted access to the
    /// destination.
    ///
    /// If the service account is not granted permission to the destination within
    /// an hour, the operation will be cancelled.
    ///
    /// For example: `"serviceAccount:foo@bar.com"`
    #[prost(string, tag = "7")]
    pub writer_identity: ::prost::alloc::string::String,
}
/// Response type for CopyLogEntries long running operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyLogEntriesResponse {
    /// Number of log entries copied.
    #[prost(int64, tag = "1")]
    pub log_entries_copied_count: i64,
}
/// Metadata for LongRunningUpdateBucket Operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketMetadata {
    /// The create time of an operation.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time of an operation.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// State of an operation.
    #[prost(enumeration = "OperationState", tag = "3")]
    pub state: i32,
    #[prost(oneof = "bucket_metadata::Request", tags = "4, 5")]
    pub request: ::core::option::Option<bucket_metadata::Request>,
}
/// Nested message and enum types in `BucketMetadata`.
pub mod bucket_metadata {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// LongRunningCreateBucket RPC request.
        #[prost(message, tag = "4")]
        CreateBucketRequest(super::CreateBucketRequest),
        /// LongRunningUpdateBucket RPC request.
        #[prost(message, tag = "5")]
        UpdateBucketRequest(super::UpdateBucketRequest),
    }
}
/// Metadata for long running Link operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkMetadata {
    /// The start time of an operation.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time of an operation.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// State of an operation.
    #[prost(enumeration = "OperationState", tag = "3")]
    pub state: i32,
    #[prost(oneof = "link_metadata::Request", tags = "4, 5")]
    pub request: ::core::option::Option<link_metadata::Request>,
}
/// Nested message and enum types in `LinkMetadata`.
pub mod link_metadata {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// CreateLink RPC request.
        #[prost(message, tag = "4")]
        CreateLinkRequest(super::CreateLinkRequest),
        /// DeleteLink RPC request.
        #[prost(message, tag = "5")]
        DeleteLinkRequest(super::DeleteLinkRequest),
    }
}
/// Cloud Logging specific location metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// Indicates whether or not Log Analytics features are supported in the given
    /// location.
    #[prost(bool, tag = "1")]
    pub log_analytics_enabled: bool,
}
/// List of different operation states.
/// High level state of the operation. This is used to report the job's
/// current state to the user. Once a long running operation is created,
/// the current state of the operation can be queried even before the
/// operation is finished and the final result is available.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationState {
    /// Should not be used.
    Unspecified = 0,
    /// The operation is scheduled.
    Scheduled = 1,
    /// Waiting for necessary permissions.
    WaitingForPermissions = 2,
    /// The operation is running.
    Running = 3,
    /// The operation was completed successfully.
    Succeeded = 4,
    /// The operation failed.
    Failed = 5,
    /// The operation was cancelled by the user.
    Cancelled = 6,
}
impl OperationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationState::Unspecified => "OPERATION_STATE_UNSPECIFIED",
            OperationState::Scheduled => "OPERATION_STATE_SCHEDULED",
            OperationState::WaitingForPermissions => {
                "OPERATION_STATE_WAITING_FOR_PERMISSIONS"
            }
            OperationState::Running => "OPERATION_STATE_RUNNING",
            OperationState::Succeeded => "OPERATION_STATE_SUCCEEDED",
            OperationState::Failed => "OPERATION_STATE_FAILED",
            OperationState::Cancelled => "OPERATION_STATE_CANCELLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_STATE_SCHEDULED" => Some(Self::Scheduled),
            "OPERATION_STATE_WAITING_FOR_PERMISSIONS" => {
                Some(Self::WaitingForPermissions)
            }
            "OPERATION_STATE_RUNNING" => Some(Self::Running),
            "OPERATION_STATE_SUCCEEDED" => Some(Self::Succeeded),
            "OPERATION_STATE_FAILED" => Some(Self::Failed),
            "OPERATION_STATE_CANCELLED" => Some(Self::Cancelled),
            _ => None,
        }
    }
}
/// LogBucket lifecycle states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LifecycleState {
    /// Unspecified state. This is only used/useful for distinguishing unset
    /// values.
    Unspecified = 0,
    /// The normal and active state.
    Active = 1,
    /// The resource has been marked for deletion by the user. For some resources
    /// (e.g. buckets), this can be reversed by an un-delete operation.
    DeleteRequested = 2,
    /// The resource has been marked for an update by the user. It will remain in
    /// this state until the update is complete.
    Updating = 3,
    /// The resource has been marked for creation by the user. It will remain in
    /// this state until the creation is complete.
    Creating = 4,
    /// The resource is in an INTERNAL error state.
    Failed = 5,
}
impl LifecycleState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LifecycleState::Unspecified => "LIFECYCLE_STATE_UNSPECIFIED",
            LifecycleState::Active => "ACTIVE",
            LifecycleState::DeleteRequested => "DELETE_REQUESTED",
            LifecycleState::Updating => "UPDATING",
            LifecycleState::Creating => "CREATING",
            LifecycleState::Failed => "FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LIFECYCLE_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "ACTIVE" => Some(Self::Active),
            "DELETE_REQUESTED" => Some(Self::DeleteRequested),
            "UPDATING" => Some(Self::Updating),
            "CREATING" => Some(Self::Creating),
            "FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
/// IndexType is used for custom indexing. It describes the type of an indexed
/// field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IndexType {
    /// The index's type is unspecified.
    Unspecified = 0,
    /// The index is a string-type index.
    String = 1,
    /// The index is a integer-type index.
    Integer = 2,
}
impl IndexType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IndexType::Unspecified => "INDEX_TYPE_UNSPECIFIED",
            IndexType::String => "INDEX_TYPE_STRING",
            IndexType::Integer => "INDEX_TYPE_INTEGER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INDEX_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "INDEX_TYPE_STRING" => Some(Self::String),
            "INDEX_TYPE_INTEGER" => Some(Self::Integer),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod config_service_v2_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for configuring sinks used to route log entries.
    #[derive(Debug, Clone)]
    pub struct ConfigServiceV2Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConfigServiceV2Client<T>
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
        ) -> ConfigServiceV2Client<InterceptedService<T, F>>
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
            ConfigServiceV2Client::new(InterceptedService::new(inner, interceptor))
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
        /// Lists log buckets.
        pub async fn list_buckets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBucketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBucketsResponse>,
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
                "/google.logging.v2.ConfigServiceV2/ListBuckets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "ListBuckets"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a log bucket.
        pub async fn get_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBucketRequest>,
        ) -> std::result::Result<tonic::Response<super::LogBucket>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/GetBucket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "GetBucket"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a log bucket asynchronously that can be used to store log entries.
        ///
        /// After a bucket has been created, the bucket's location cannot be changed.
        pub async fn create_bucket_async(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBucketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.logging.v2.ConfigServiceV2/CreateBucketAsync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.ConfigServiceV2",
                        "CreateBucketAsync",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a log bucket asynchronously.
        ///
        /// If the bucket has a `lifecycle_state` of `DELETE_REQUESTED`, then
        /// `FAILED_PRECONDITION` will be returned.
        ///
        /// After a bucket has been created, the bucket's location cannot be changed.
        pub async fn update_bucket_async(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBucketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.logging.v2.ConfigServiceV2/UpdateBucketAsync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.ConfigServiceV2",
                        "UpdateBucketAsync",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a log bucket that can be used to store log entries. After a bucket
        /// has been created, the bucket's location cannot be changed.
        pub async fn create_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBucketRequest>,
        ) -> std::result::Result<tonic::Response<super::LogBucket>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/CreateBucket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "CreateBucket"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a log bucket.
        ///
        /// If the bucket has a `lifecycle_state` of `DELETE_REQUESTED`, then
        /// `FAILED_PRECONDITION` will be returned.
        ///
        /// After a bucket has been created, the bucket's location cannot be changed.
        pub async fn update_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBucketRequest>,
        ) -> std::result::Result<tonic::Response<super::LogBucket>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/UpdateBucket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "UpdateBucket"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a log bucket.
        ///
        /// Changes the bucket's `lifecycle_state` to the `DELETE_REQUESTED` state.
        /// After 7 days, the bucket will be purged and all log entries in the bucket
        /// will be permanently deleted.
        pub async fn delete_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBucketRequest>,
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
                "/google.logging.v2.ConfigServiceV2/DeleteBucket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "DeleteBucket"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Undeletes a log bucket. A bucket that has been deleted can be undeleted
        /// within the grace period of 7 days.
        pub async fn undelete_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteBucketRequest>,
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
                "/google.logging.v2.ConfigServiceV2/UndeleteBucket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.ConfigServiceV2",
                        "UndeleteBucket",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists views on a log bucket.
        pub async fn list_views(
            &mut self,
            request: impl tonic::IntoRequest<super::ListViewsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListViewsResponse>,
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
                "/google.logging.v2.ConfigServiceV2/ListViews",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "ListViews"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a view on a log bucket..
        pub async fn get_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetViewRequest>,
        ) -> std::result::Result<tonic::Response<super::LogView>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/GetView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.logging.v2.ConfigServiceV2", "GetView"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates a view over log entries in a log bucket. A bucket may contain a
        /// maximum of 30 views.
        pub async fn create_view(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateViewRequest>,
        ) -> std::result::Result<tonic::Response<super::LogView>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/CreateView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "CreateView"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a view on a log bucket. This method replaces the following fields
        /// in the existing view with values from the new view: `filter`.
        /// If an `UNAVAILABLE` error is returned, this indicates that system is not in
        /// a state where it can update the view. If this occurs, please try again in a
        /// few minutes.
        pub async fn update_view(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateViewRequest>,
        ) -> std::result::Result<tonic::Response<super::LogView>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/UpdateView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "UpdateView"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a view on a log bucket.
        /// If an `UNAVAILABLE` error is returned, this indicates that system is not in
        /// a state where it can delete the view. If this occurs, please try again in a
        /// few minutes.
        pub async fn delete_view(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteViewRequest>,
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
                "/google.logging.v2.ConfigServiceV2/DeleteView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "DeleteView"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists sinks.
        pub async fn list_sinks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSinksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSinksResponse>,
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
                "/google.logging.v2.ConfigServiceV2/ListSinks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "ListSinks"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a sink.
        pub async fn get_sink(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSinkRequest>,
        ) -> std::result::Result<tonic::Response<super::LogSink>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/GetSink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.logging.v2.ConfigServiceV2", "GetSink"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates a sink that exports specified log entries to a destination. The
        /// export of newly-ingested log entries begins immediately, unless the sink's
        /// `writer_identity` is not permitted to write to the destination. A sink can
        /// export log entries only from the resource owning the sink.
        pub async fn create_sink(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSinkRequest>,
        ) -> std::result::Result<tonic::Response<super::LogSink>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/CreateSink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "CreateSink"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a sink. This method replaces the following fields in the existing
        /// sink with values from the new sink: `destination`, and `filter`.
        ///
        /// The updated sink might also have a new `writer_identity`; see the
        /// `unique_writer_identity` field.
        pub async fn update_sink(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSinkRequest>,
        ) -> std::result::Result<tonic::Response<super::LogSink>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/UpdateSink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "UpdateSink"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a sink. If the sink has a unique `writer_identity`, then that
        /// service account is also deleted.
        pub async fn delete_sink(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSinkRequest>,
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
                "/google.logging.v2.ConfigServiceV2/DeleteSink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "DeleteSink"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Asynchronously creates a linked dataset in BigQuery which makes it possible
        /// to use BigQuery to read the logs stored in the log bucket. A log bucket may
        /// currently only contain one link.
        pub async fn create_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.logging.v2.ConfigServiceV2/CreateLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "CreateLink"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a link. This will also delete the corresponding BigQuery linked
        /// dataset.
        pub async fn delete_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.logging.v2.ConfigServiceV2/DeleteLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "DeleteLink"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists links.
        pub async fn list_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLinksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLinksResponse>,
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
                "/google.logging.v2.ConfigServiceV2/ListLinks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "ListLinks"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a link.
        pub async fn get_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLinkRequest>,
        ) -> std::result::Result<tonic::Response<super::Link>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/GetLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.logging.v2.ConfigServiceV2", "GetLink"));
            self.inner.unary(req, path, codec).await
        }
        /// Lists all the exclusions on the _Default sink in a parent resource.
        pub async fn list_exclusions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExclusionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExclusionsResponse>,
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
                "/google.logging.v2.ConfigServiceV2/ListExclusions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.ConfigServiceV2",
                        "ListExclusions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the description of an exclusion in the _Default sink.
        pub async fn get_exclusion(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExclusionRequest>,
        ) -> std::result::Result<tonic::Response<super::LogExclusion>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/GetExclusion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "GetExclusion"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new exclusion in the _Default sink in a specified parent
        /// resource. Only log entries belonging to that resource can be excluded. You
        /// can have up to 10 exclusions in a resource.
        pub async fn create_exclusion(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExclusionRequest>,
        ) -> std::result::Result<tonic::Response<super::LogExclusion>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/CreateExclusion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.ConfigServiceV2",
                        "CreateExclusion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Changes one or more properties of an existing exclusion in the _Default
        /// sink.
        pub async fn update_exclusion(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateExclusionRequest>,
        ) -> std::result::Result<tonic::Response<super::LogExclusion>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/UpdateExclusion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.ConfigServiceV2",
                        "UpdateExclusion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an exclusion in the _Default sink.
        pub async fn delete_exclusion(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteExclusionRequest>,
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
                "/google.logging.v2.ConfigServiceV2/DeleteExclusion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.ConfigServiceV2",
                        "DeleteExclusion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the Logging CMEK settings for the given resource.
        ///
        /// Note: CMEK for the Log Router can be configured for Google Cloud projects,
        /// folders, organizations and billing accounts. Once configured for an
        /// organization, it applies to all projects and folders in the Google Cloud
        /// organization.
        ///
        /// See [Enabling CMEK for Log
        /// Router](https://cloud.google.com/logging/docs/routing/managed-encryption)
        /// for more information.
        pub async fn get_cmek_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCmekSettingsRequest>,
        ) -> std::result::Result<tonic::Response<super::CmekSettings>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/GetCmekSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.ConfigServiceV2",
                        "GetCmekSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the Log Router CMEK settings for the given resource.
        ///
        /// Note: CMEK for the Log Router can currently only be configured for Google
        /// Cloud organizations. Once configured, it applies to all projects and
        /// folders in the Google Cloud organization.
        ///
        /// [UpdateCmekSettings][google.logging.v2.ConfigServiceV2.UpdateCmekSettings]
        /// will fail if 1) `kms_key_name` is invalid, or 2) the associated service
        /// account does not have the required
        /// `roles/cloudkms.cryptoKeyEncrypterDecrypter` role assigned for the key, or
        /// 3) access to the key is disabled.
        ///
        /// See [Enabling CMEK for Log
        /// Router](https://cloud.google.com/logging/docs/routing/managed-encryption)
        /// for more information.
        pub async fn update_cmek_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCmekSettingsRequest>,
        ) -> std::result::Result<tonic::Response<super::CmekSettings>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.logging.v2.ConfigServiceV2/UpdateCmekSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.ConfigServiceV2",
                        "UpdateCmekSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the Log Router settings for the given resource.
        ///
        /// Note: Settings for the Log Router can be get for Google Cloud projects,
        /// folders, organizations and billing accounts. Currently it can only be
        /// configured for organizations. Once configured for an organization, it
        /// applies to all projects and folders in the Google Cloud organization.
        ///
        /// See [Enabling CMEK for Log
        /// Router](https://cloud.google.com/logging/docs/routing/managed-encryption)
        /// for more information.
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
                "/google.logging.v2.ConfigServiceV2/GetSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.logging.v2.ConfigServiceV2", "GetSettings"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the Log Router settings for the given resource.
        ///
        /// Note: Settings for the Log Router can currently only be configured for
        /// Google Cloud organizations. Once configured, it applies to all projects and
        /// folders in the Google Cloud organization.
        ///
        /// [UpdateSettings][google.logging.v2.ConfigServiceV2.UpdateSettings]
        /// will fail if 1) `kms_key_name` is invalid, or 2) the associated service
        /// account does not have the required
        /// `roles/cloudkms.cryptoKeyEncrypterDecrypter` role assigned for the key, or
        /// 3) access to the key is disabled. 4) `location_id` is not supported by
        /// Logging. 5) `location_id` violate OrgPolicy.
        ///
        /// See [Enabling CMEK for Log
        /// Router](https://cloud.google.com/logging/docs/routing/managed-encryption)
        /// for more information.
        pub async fn update_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSettingsRequest>,
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
                "/google.logging.v2.ConfigServiceV2/UpdateSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.ConfigServiceV2",
                        "UpdateSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Copies a set of log entries from a log bucket to a Cloud Storage bucket.
        pub async fn copy_log_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::CopyLogEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.logging.v2.ConfigServiceV2/CopyLogEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.logging.v2.ConfigServiceV2",
                        "CopyLogEntries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
