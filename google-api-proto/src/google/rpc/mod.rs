#[cfg(any(feature = "google-rpc-context"))]
pub mod context;
/// The `Status` type defines a logical error model that is suitable for
/// different programming environments, including REST APIs and RPC APIs. It is
/// used by \[gRPC\](<https://github.com/grpc>). Each `Status` message contains
/// three pieces of data: error code, error message, and error details.
///
/// You can find out more about this error model and how to work with it in the
/// [API Design Guide](<https://cloud.google.com/apis/design/errors>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// The status code, which should be an enum value of \[google.rpc.Code][google.rpc.Code\].
    #[prost(int32, tag="1")]
    pub code: i32,
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// \[google.rpc.Status.details][google.rpc.Status.details\] field, or localized by the client.
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    /// A list of messages that carry the error details.  There is a common set of
    /// message types for APIs to use.
    #[prost(message, repeated, tag="3")]
    pub details: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// The canonical error codes for gRPC APIs.
///
///
/// Sometimes multiple error codes may apply.  Services should return
/// the most specific error code that applies.  For example, prefer
/// `OUT_OF_RANGE` over `FAILED_PRECONDITION` if both codes apply.
/// Similarly prefer `NOT_FOUND` or `ALREADY_EXISTS` over `FAILED_PRECONDITION`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Code {
    /// Not an error; returned on success
    ///
    /// HTTP Mapping: 200 OK
    Ok = 0,
    /// The operation was cancelled, typically by the caller.
    ///
    /// HTTP Mapping: 499 Client Closed Request
    Cancelled = 1,
    /// Unknown error.  For example, this error may be returned when
    /// a `Status` value received from another address space belongs to
    /// an error space that is not known in this address space.  Also
    /// errors raised by APIs that do not return enough error information
    /// may be converted to this error.
    ///
    /// HTTP Mapping: 500 Internal Server Error
    Unknown = 2,
    /// The client specified an invalid argument.  Note that this differs
    /// from `FAILED_PRECONDITION`.  `INVALID_ARGUMENT` indicates arguments
    /// that are problematic regardless of the state of the system
    /// (e.g., a malformed file name).
    ///
    /// HTTP Mapping: 400 Bad Request
    InvalidArgument = 3,
    /// The deadline expired before the operation could complete. For operations
    /// that change the state of the system, this error may be returned
    /// even if the operation has completed successfully.  For example, a
    /// successful response from a server could have been delayed long
    /// enough for the deadline to expire.
    ///
    /// HTTP Mapping: 504 Gateway Timeout
    DeadlineExceeded = 4,
    /// Some requested entity (e.g., file or directory) was not found.
    ///
    /// Note to server developers: if a request is denied for an entire class
    /// of users, such as gradual feature rollout or undocumented whitelist,
    /// `NOT_FOUND` may be used. If a request is denied for some users within
    /// a class of users, such as user-based access control, `PERMISSION_DENIED`
    /// must be used.
    ///
    /// HTTP Mapping: 404 Not Found
    NotFound = 5,
    /// The entity that a client attempted to create (e.g., file or directory)
    /// already exists.
    ///
    /// HTTP Mapping: 409 Conflict
    AlreadyExists = 6,
    /// The caller does not have permission to execute the specified
    /// operation. `PERMISSION_DENIED` must not be used for rejections
    /// caused by exhausting some resource (use `RESOURCE_EXHAUSTED`
    /// instead for those errors). `PERMISSION_DENIED` must not be
    /// used if the caller can not be identified (use `UNAUTHENTICATED`
    /// instead for those errors). This error code does not imply the
    /// request is valid or the requested entity exists or satisfies
    /// other pre-conditions.
    ///
    /// HTTP Mapping: 403 Forbidden
    PermissionDenied = 7,
    /// The request does not have valid authentication credentials for the
    /// operation.
    ///
    /// HTTP Mapping: 401 Unauthorized
    Unauthenticated = 16,
    /// Some resource has been exhausted, perhaps a per-user quota, or
    /// perhaps the entire file system is out of space.
    ///
    /// HTTP Mapping: 429 Too Many Requests
    ResourceExhausted = 8,
    /// The operation was rejected because the system is not in a state
    /// required for the operation's execution.  For example, the directory
    /// to be deleted is non-empty, an rmdir operation is applied to
    /// a non-directory, etc.
    ///
    /// Service implementors can use the following guidelines to decide
    /// between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`:
    ///   (a) Use `UNAVAILABLE` if the client can retry just the failing call.
    ///   (b) Use `ABORTED` if the client should retry at a higher level
    ///       (e.g., when a client-specified test-and-set fails, indicating the
    ///       client should restart a read-modify-write sequence).
    ///   (c) Use `FAILED_PRECONDITION` if the client should not retry until
    ///       the system state has been explicitly fixed.  E.g., if an "rmdir"
    ///       fails because the directory is non-empty, `FAILED_PRECONDITION`
    ///       should be returned since the client should not retry unless
    ///       the files are deleted from the directory.
    ///
    /// HTTP Mapping: 400 Bad Request
    FailedPrecondition = 9,
    /// The operation was aborted, typically due to a concurrency issue such as
    /// a sequencer check failure or transaction abort.
    ///
    /// See the guidelines above for deciding between `FAILED_PRECONDITION`,
    /// `ABORTED`, and `UNAVAILABLE`.
    ///
    /// HTTP Mapping: 409 Conflict
    Aborted = 10,
    /// The operation was attempted past the valid range.  E.g., seeking or
    /// reading past end-of-file.
    ///
    /// Unlike `INVALID_ARGUMENT`, this error indicates a problem that may
    /// be fixed if the system state changes. For example, a 32-bit file
    /// system will generate `INVALID_ARGUMENT` if asked to read at an
    /// offset that is not in the range \[0,2^32-1\], but it will generate
    /// `OUT_OF_RANGE` if asked to read from an offset past the current
    /// file size.
    ///
    /// There is a fair bit of overlap between `FAILED_PRECONDITION` and
    /// `OUT_OF_RANGE`.  We recommend using `OUT_OF_RANGE` (the more specific
    /// error) when it applies so that callers who are iterating through
    /// a space can easily look for an `OUT_OF_RANGE` error to detect when
    /// they are done.
    ///
    /// HTTP Mapping: 400 Bad Request
    OutOfRange = 11,
    /// The operation is not implemented or is not supported/enabled in this
    /// service.
    ///
    /// HTTP Mapping: 501 Not Implemented
    Unimplemented = 12,
    /// Internal errors.  This means that some invariants expected by the
    /// underlying system have been broken.  This error code is reserved
    /// for serious errors.
    ///
    /// HTTP Mapping: 500 Internal Server Error
    Internal = 13,
    /// The service is currently unavailable.  This is most likely a
    /// transient condition, which can be corrected by retrying with
    /// a backoff. Note that it is not always safe to retry
    /// non-idempotent operations.
    ///
    /// See the guidelines above for deciding between `FAILED_PRECONDITION`,
    /// `ABORTED`, and `UNAVAILABLE`.
    ///
    /// HTTP Mapping: 503 Service Unavailable
    Unavailable = 14,
    /// Unrecoverable data loss or corruption.
    ///
    /// HTTP Mapping: 500 Internal Server Error
    DataLoss = 15,
}
impl Code {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Code::Ok => "OK",
            Code::Cancelled => "CANCELLED",
            Code::Unknown => "UNKNOWN",
            Code::InvalidArgument => "INVALID_ARGUMENT",
            Code::DeadlineExceeded => "DEADLINE_EXCEEDED",
            Code::NotFound => "NOT_FOUND",
            Code::AlreadyExists => "ALREADY_EXISTS",
            Code::PermissionDenied => "PERMISSION_DENIED",
            Code::Unauthenticated => "UNAUTHENTICATED",
            Code::ResourceExhausted => "RESOURCE_EXHAUSTED",
            Code::FailedPrecondition => "FAILED_PRECONDITION",
            Code::Aborted => "ABORTED",
            Code::OutOfRange => "OUT_OF_RANGE",
            Code::Unimplemented => "UNIMPLEMENTED",
            Code::Internal => "INTERNAL",
            Code::Unavailable => "UNAVAILABLE",
            Code::DataLoss => "DATA_LOSS",
        }
    }
}
/// Describes when the clients can retry a failed request. Clients could ignore
/// the recommendation here or retry when this information is missing from error
/// responses.
///
/// It's always recommended that clients should use exponential backoff when
/// retrying.
///
/// Clients should wait until `retry_delay` amount of time has passed since
/// receiving the error response before retrying.  If retrying requests also
/// fail, clients should use an exponential backoff scheme to gradually increase
/// the delay between retries based on `retry_delay`, until either a maximum
/// number of retries have been reached or a maximum retry delay cap has been
/// reached.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryInfo {
    /// Clients should wait at least this long between retrying the same request.
    #[prost(message, optional, tag="1")]
    pub retry_delay: ::core::option::Option<::prost_types::Duration>,
}
/// Describes additional debugging info.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugInfo {
    /// The stack trace entries indicating where the error occurred.
    #[prost(string, repeated, tag="1")]
    pub stack_entries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Additional debugging information provided by the server.
    #[prost(string, tag="2")]
    pub detail: ::prost::alloc::string::String,
}
/// Describes how a quota check failed.
///
/// For example if a daily limit was exceeded for the calling project,
/// a service could respond with a QuotaFailure detail containing the project
/// id and the description of the quota limit that was exceeded.  If the
/// calling project hasn't enabled the service in the developer console, then
/// a service could respond with the project id and set `service_disabled`
/// to true.
///
/// Also see RetryInfo and Help types for other details about handling a
/// quota failure.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaFailure {
    /// Describes all quota violations.
    #[prost(message, repeated, tag="1")]
    pub violations: ::prost::alloc::vec::Vec<quota_failure::Violation>,
}
/// Nested message and enum types in `QuotaFailure`.
pub mod quota_failure {
    /// A message type used to describe a single quota violation.  For example, a
    /// daily quota or a custom quota that was exceeded.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Violation {
        /// The subject on which the quota check failed.
        /// For example, "clientip:<ip address of client>" or "project:<Google
        /// developer project id>".
        #[prost(string, tag="1")]
        pub subject: ::prost::alloc::string::String,
        /// A description of how the quota check failed. Clients can use this
        /// description to find more about the quota configuration in the service's
        /// public documentation, or find the relevant quota limit to adjust through
        /// developer console.
        ///
        /// For example: "Service disabled" or "Daily Limit for read operations
        /// exceeded".
        #[prost(string, tag="2")]
        pub description: ::prost::alloc::string::String,
    }
}
/// Describes the cause of the error with structured details.
///
/// Example of an error when contacting the "pubsub.googleapis.com" API when it
/// is not enabled:
///
///      { "reason": "API_DISABLED"
///        "domain": "googleapis.com"
///        "metadata": {
///          "resource": "projects/123",
///          "service": "pubsub.googleapis.com"
///        }
///      }
///
/// This response indicates that the pubsub.googleapis.com API is not enabled.
///
/// Example of an error that is returned when attempting to create a Spanner
/// instance in a region that is out of stock:
///
///      { "reason": "STOCKOUT"
///        "domain": "spanner.googleapis.com",
///        "metadata": {
///          "availableRegions": "us-central1,us-east2"
///        }
///      }
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorInfo {
    /// The reason of the error. This is a constant value that identifies the
    /// proximate cause of the error. Error reasons are unique within a particular
    /// domain of errors. This should be at most 63 characters and match
    /// /\[A-Z0-9_\]+/.
    #[prost(string, tag="1")]
    pub reason: ::prost::alloc::string::String,
    /// The logical grouping to which the "reason" belongs. The error domain
    /// is typically the registered service name of the tool or product that
    /// generates the error. Example: "pubsub.googleapis.com". If the error is
    /// generated by some common infrastructure, the error domain must be a
    /// globally unique value that identifies the infrastructure. For Google API
    /// infrastructure, the error domain is "googleapis.com".
    #[prost(string, tag="2")]
    pub domain: ::prost::alloc::string::String,
    /// Additional structured details about this error.
    ///
    /// Keys should match /\[a-zA-Z0-9-_\]/ and be limited to 64 characters in
    /// length. When identifying the current value of an exceeded limit, the units
    /// should be contained in the key, not the value.  For example, rather than
    /// {"instanceLimit": "100/request"}, should be returned as,
    /// {"instanceLimitPerRequest": "100"}, if the client exceeds the number of
    /// instances that can be created in a single (batch) request.
    #[prost(btree_map="string, string", tag="3")]
    pub metadata: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Describes what preconditions have failed.
///
/// For example, if an RPC failed because it required the Terms of Service to be
/// acknowledged, it could list the terms of service violation in the
/// PreconditionFailure message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreconditionFailure {
    /// Describes all precondition violations.
    #[prost(message, repeated, tag="1")]
    pub violations: ::prost::alloc::vec::Vec<precondition_failure::Violation>,
}
/// Nested message and enum types in `PreconditionFailure`.
pub mod precondition_failure {
    /// A message type used to describe a single precondition failure.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Violation {
        /// The type of PreconditionFailure. We recommend using a service-specific
        /// enum type to define the supported precondition violation subjects. For
        /// example, "TOS" for "Terms of Service violation".
        #[prost(string, tag="1")]
        pub r#type: ::prost::alloc::string::String,
        /// The subject, relative to the type, that failed.
        /// For example, "google.com/cloud" relative to the "TOS" type would indicate
        /// which terms of service is being referenced.
        #[prost(string, tag="2")]
        pub subject: ::prost::alloc::string::String,
        /// A description of how the precondition failed. Developers can use this
        /// description to understand how to fix the failure.
        ///
        /// For example: "Terms of service not accepted".
        #[prost(string, tag="3")]
        pub description: ::prost::alloc::string::String,
    }
}
/// Describes violations in a client request. This error type focuses on the
/// syntactic aspects of the request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BadRequest {
    /// Describes all violations in a client request.
    #[prost(message, repeated, tag="1")]
    pub field_violations: ::prost::alloc::vec::Vec<bad_request::FieldViolation>,
}
/// Nested message and enum types in `BadRequest`.
pub mod bad_request {
    /// A message type used to describe a single bad request field.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FieldViolation {
        /// A path leading to a field in the request body. The value will be a
        /// sequence of dot-separated identifiers that identify a protocol buffer
        /// field. E.g., "field_violations.field" would identify this field.
        #[prost(string, tag="1")]
        pub field: ::prost::alloc::string::String,
        /// A description of why the request element is bad.
        #[prost(string, tag="2")]
        pub description: ::prost::alloc::string::String,
    }
}
/// Contains metadata about the request that clients can attach when filing a bug
/// or providing other forms of feedback.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestInfo {
    /// An opaque string that should only be interpreted by the service generating
    /// it. For example, it can be used to identify requests in the service's logs.
    #[prost(string, tag="1")]
    pub request_id: ::prost::alloc::string::String,
    /// Any data that was used to serve this request. For example, an encrypted
    /// stack trace that can be sent back to the service provider for debugging.
    #[prost(string, tag="2")]
    pub serving_data: ::prost::alloc::string::String,
}
/// Describes the resource that is being accessed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceInfo {
    /// A name for the type of resource being accessed, e.g. "sql table",
    /// "cloud storage bucket", "file", "Google calendar"; or the type URL
    /// of the resource: e.g. "type.googleapis.com/google.pubsub.v1.Topic".
    #[prost(string, tag="1")]
    pub resource_type: ::prost::alloc::string::String,
    /// The name of the resource being accessed.  For example, a shared calendar
    /// name: "example.com_4fghdhgsrgh@group.calendar.google.com", if the current
    /// error is \[google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED\].
    #[prost(string, tag="2")]
    pub resource_name: ::prost::alloc::string::String,
    /// The owner of the resource (optional).
    /// For example, "user:<owner email>" or "project:<Google developer project
    /// id>".
    #[prost(string, tag="3")]
    pub owner: ::prost::alloc::string::String,
    /// Describes what error is encountered when accessing this resource.
    /// For example, updating a cloud project may require the `writer` permission
    /// on the developer console project.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
}
/// Provides links to documentation or for performing an out of band action.
///
/// For example, if a quota check failed with an error indicating the calling
/// project hasn't enabled the accessed service, this can contain a URL pointing
/// directly to the right place in the developer console to flip the bit.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Help {
    /// URL(s) pointing to additional information on handling the current error.
    #[prost(message, repeated, tag="1")]
    pub links: ::prost::alloc::vec::Vec<help::Link>,
}
/// Nested message and enum types in `Help`.
pub mod help {
    /// Describes a URL link.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Link {
        /// Describes what the link offers.
        #[prost(string, tag="1")]
        pub description: ::prost::alloc::string::String,
        /// The URL of the link.
        #[prost(string, tag="2")]
        pub url: ::prost::alloc::string::String,
    }
}
/// Provides a localized error message that is safe to return to the user
/// which can be attached to an RPC error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalizedMessage {
    /// The locale used following the specification defined at
    /// <http://www.rfc-editor.org/rfc/bcp/bcp47.txt.>
    /// Examples are: "en-US", "fr-CH", "es-MX"
    #[prost(string, tag="1")]
    pub locale: ::prost::alloc::string::String,
    /// The localized error message in the above locale.
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
