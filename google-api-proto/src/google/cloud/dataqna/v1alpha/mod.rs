/// Describes string annotation from both semantic and formatting perspectives.
/// Example:
///
/// User Query:
///
///    top countries by population in Africa
///
///    0   4         14 17         28 31    37
///
/// Table Data:
///
/// + "country" - dimension
/// + "population" - metric
/// + "Africa" - value in the "continent" column
///
/// text_formatted = `"top countries by population in Africa"`
///
/// html_formatted =
///    `"top <b>countries</b> by <b>population</b> in <i>Africa</i>"`
///
/// ```
/// markups = [
///    {DIMENSION, 4, 12}, // 'countries'
///    {METRIC, 17, 26}, // 'population'
///    {FILTER, 31, 36}  // 'Africa'
/// ]
/// ```
///
/// Note that html formattings for 'DIMENSION' and 'METRIC' are the same, while
/// semantic markups are different.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotatedString {
    /// Text version of the string.
    #[prost(string, tag = "1")]
    pub text_formatted: ::prost::alloc::string::String,
    /// HTML version of the string annotation.
    #[prost(string, tag = "2")]
    pub html_formatted: ::prost::alloc::string::String,
    /// Semantic version of the string annotation.
    #[prost(message, repeated, tag = "3")]
    pub markups: ::prost::alloc::vec::Vec<annotated_string::SemanticMarkup>,
}
/// Nested message and enum types in `AnnotatedString`.
pub mod annotated_string {
    /// Semantic markup denotes a substring (by index and length) with markup
    /// information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SemanticMarkup {
        /// The semantic type of the markup substring.
        #[prost(enumeration = "SemanticMarkupType", tag = "1")]
        pub r#type: i32,
        /// Unicode character index of the query.
        #[prost(int32, tag = "2")]
        pub start_char_index: i32,
        /// The length (number of unicode characters) of the markup substring.
        #[prost(int32, tag = "3")]
        pub length: i32,
    }
    /// Semantic markup types.
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
    pub enum SemanticMarkupType {
        /// No markup type was specified.
        MarkupTypeUnspecified = 0,
        /// Markup for a substring denoting a metric.
        Metric = 1,
        /// Markup for a substring denoting a dimension.
        Dimension = 2,
        /// Markup for a substring denoting a filter.
        Filter = 3,
        /// Markup for an unused substring.
        Unused = 4,
        /// Markup for a substring containing blocked phrases.
        Blocked = 5,
        /// Markup for a substring that contains terms for row.
        Row = 6,
    }
    impl SemanticMarkupType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SemanticMarkupType::MarkupTypeUnspecified => "MARKUP_TYPE_UNSPECIFIED",
                SemanticMarkupType::Metric => "METRIC",
                SemanticMarkupType::Dimension => "DIMENSION",
                SemanticMarkupType::Filter => "FILTER",
                SemanticMarkupType::Unused => "UNUSED",
                SemanticMarkupType::Blocked => "BLOCKED",
                SemanticMarkupType::Row => "ROW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MARKUP_TYPE_UNSPECIFIED" => Some(Self::MarkupTypeUnspecified),
                "METRIC" => Some(Self::Metric),
                "DIMENSION" => Some(Self::Dimension),
                "FILTER" => Some(Self::Filter),
                "UNUSED" => Some(Self::Unused),
                "BLOCKED" => Some(Self::Blocked),
                "ROW" => Some(Self::Row),
                _ => None,
            }
        }
    }
}
/// Request for query suggestions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestQueriesRequest {
    /// Required. The parent of the suggestion query is the resource denoting the project and
    /// location.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The scopes to which this search is restricted. The only supported scope
    /// pattern is
    /// `//bigquery.googleapis.com/projects/{GCP-PROJECT-ID}/datasets/{DATASET-ID}/tables/{TABLE-ID}`.
    #[prost(string, repeated, tag = "2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// User query for which to generate suggestions. If the query is empty, zero
    /// state suggestions are returned. This allows UIs to display suggestions
    /// right away, helping the user to get a sense of what a query might look
    /// like.
    #[prost(string, tag = "3")]
    pub query: ::prost::alloc::string::String,
    /// The requested suggestion type. Multiple suggestion types can be
    /// requested, but there is no guarantee that the service will return
    /// suggestions for each type. Suggestions for a requested type might rank
    /// lower than suggestions for other types and the service may decide to cut
    /// these suggestions off.
    #[prost(enumeration = "SuggestionType", repeated, tag = "4")]
    pub suggestion_types: ::prost::alloc::vec::Vec<i32>,
}
/// A suggestion for a query with a ranking score.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Suggestion {
    /// Detailed information about the suggestion.
    #[prost(message, optional, tag = "1")]
    pub suggestion_info: ::core::option::Option<SuggestionInfo>,
    /// The score of the suggestion. This can be used to define ordering in UI.
    /// The score represents confidence in the suggestion where higher is better.
    /// All score values must be in the range [0, 1).
    #[prost(double, tag = "2")]
    pub ranking_score: f64,
    /// The type of the suggestion.
    #[prost(enumeration = "SuggestionType", tag = "3")]
    pub suggestion_type: i32,
}
/// Detailed information about the suggestion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestionInfo {
    /// Annotations for the suggestion. This provides information about which part
    /// of the suggestion corresponds to what semantic meaning (e.g. a metric).
    #[prost(message, optional, tag = "1")]
    pub annotated_suggestion: ::core::option::Option<AnnotatedString>,
    /// Matches between user query and the annotated string.
    #[prost(message, repeated, tag = "2")]
    pub query_matches: ::prost::alloc::vec::Vec<suggestion_info::MatchInfo>,
}
/// Nested message and enum types in `SuggestionInfo`.
pub mod suggestion_info {
    /// MatchInfo describes which part of suggestion matched with data in user
    /// typed query. This can be used to highlight matching parts in the UI. This
    /// is different from the annotations provided in annotated_suggestion. The
    /// annotated_suggestion provides information about the semantic meaning, while
    /// this provides information about how it relates to the input.
    ///
    /// Example:
    /// user query: `top products`
    ///
    /// ```
    /// annotated_suggestion {
    ///   text_formatted = "top product_group"
    ///   html_formatted = "top <b>product_group</b>"
    ///   markups {
    ///    {type: TEXT, start_char_index: 0, length: 3}
    ///    {type: DIMENSION, start_char_index: 4, length: 13}
    ///   }
    /// }
    ///
    /// query_matches {
    ///   { start_char_index: 0, length: 3 }
    ///   { start_char_index: 4, length: 7}
    /// }
    /// ```
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchInfo {
        /// Unicode character index of the string annotation.
        #[prost(int32, tag = "1")]
        pub start_char_index: i32,
        /// Count of unicode characters of this substring.
        #[prost(int32, tag = "2")]
        pub length: i32,
    }
}
/// Response to SuggestQueries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestQueriesResponse {
    /// A list of suggestions.
    #[prost(message, repeated, tag = "1")]
    pub suggestions: ::prost::alloc::vec::Vec<Suggestion>,
}
/// The type of suggestion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SuggestionType {
    /// No suggestiont type is specified.
    Unspecified = 0,
    /// Entity suggestion type. Suggestions are for single entities.
    Entity = 1,
    /// Template suggestion type. Suggestions are for full sentences.
    Template = 2,
}
impl SuggestionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SuggestionType::Unspecified => "SUGGESTION_TYPE_UNSPECIFIED",
            SuggestionType::Entity => "ENTITY",
            SuggestionType::Template => "TEMPLATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUGGESTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ENTITY" => Some(Self::Entity),
            "TEMPLATE" => Some(Self::Template),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod auto_suggestion_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// This stateless API provides automatic suggestions for natural language
    /// queries for the data sources in the provided project and location.
    ///
    /// The service provides a resourceless operation `suggestQueries` that can be
    /// called to get a list of suggestions for a given incomplete query and scope
    /// (or list of scopes) under which the query is to be interpreted.
    ///
    /// There are two types of suggestions, ENTITY for single entity suggestions
    /// and TEMPLATE for full sentences. By default, both types are returned.
    ///
    /// Example Request:
    /// ```
    /// GetSuggestions({
    ///   parent: "locations/us/projects/my-project"
    ///   scopes:
    ///   "//bigquery.googleapis.com/projects/my-project/datasets/my-dataset/tables/my-table"
    ///   query: "top it"
    /// })
    /// ```
    ///
    /// The service will retrieve information based on the given scope(s) and give
    /// suggestions based on that (e.g. "top item" for "top it" if "item" is a known
    /// dimension for the provided scope).
    /// ```
    /// suggestions {
    ///   suggestion_info {
    ///     annotated_suggestion {
    ///       text_formatted: "top item by sum of usd_revenue_net"
    ///       markups {
    ///         type: DIMENSION
    ///         start_char_index: 4
    ///         length: 4
    ///       }
    ///       markups {
    ///         type: METRIC
    ///         start_char_index: 19
    ///         length: 15
    ///       }
    ///     }
    ///     query_matches {
    ///       start_char_index: 0
    ///       length: 6
    ///     }
    ///   }
    ///   suggestion_type: TEMPLATE
    ///   ranking_score: 0.9
    /// }
    /// suggestions {
    ///   suggestion_info {
    ///     annotated_suggestion {
    ///       text_formatted: "item"
    ///       markups {
    ///         type: DIMENSION
    ///         start_char_index: 4
    ///         length: 2
    ///       }
    ///     }
    ///     query_matches {
    ///       start_char_index: 0
    ///       length: 6
    ///     }
    ///   }
    ///   suggestion_type: ENTITY
    ///   ranking_score: 0.8
    /// }
    /// ```
    #[derive(Debug, Clone)]
    pub struct AutoSuggestionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AutoSuggestionServiceClient<T>
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
        ) -> AutoSuggestionServiceClient<InterceptedService<T, F>>
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
            AutoSuggestionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets a list of suggestions based on a prefix string.
        /// AutoSuggestion tolerance should be less than 1 second.
        pub async fn suggest_queries(
            &mut self,
            request: impl tonic::IntoRequest<super::SuggestQueriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SuggestQueriesResponse>,
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
                "/google.cloud.dataqna.v1alpha.AutoSuggestionService/SuggestQueries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataqna.v1alpha.AutoSuggestionService",
                        "SuggestQueries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The question resource represents a natural language query, its settings,
/// understanding generated by the system, and answer retrieval status.
/// A question cannot be modified.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Question {
    /// Output only. Immutable. The unique identifier for the Question. The ID is server-generated.
    /// Example: `projects/foo/locations/bar/questions/123`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. Scopes to be used for the question. A scope defines the relevant data set
    /// scope. It can be a reference to a specific data source or a collection of
    /// data sources. Currently, support is limited to a single BigQuery table.
    /// There must be exactly one `scopes` element.
    ///
    /// Example:
    /// `//bigquery.googleapis.com/projects/test-project/datasets/foo/tables/bar`
    #[prost(string, repeated, tag = "2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Immutable. The query in natural language.
    #[prost(string, tag = "3")]
    pub query: ::prost::alloc::string::String,
    /// A list of annotations to use instead of the default annotation of a data
    /// source (set in the data source reference resource). There must not be
    /// more than one annotation with the same data source reference.
    #[prost(string, repeated, tag = "4")]
    pub data_source_annotations: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// An error field explaining why interpretation failed. This is only populated
    /// if the interpretation failed.
    ///
    /// Note: This is different from getting a status error on the request itself.
    /// This is not a client or server error and the Question resource is still
    /// persisted, but the service could not interpret the question. Clients should
    /// present the error to the user so the user can rephrase the question.
    #[prost(message, optional, tag = "5")]
    pub interpret_error: ::core::option::Option<InterpretError>,
    /// A list of interpretations for this question.
    #[prost(message, repeated, tag = "6")]
    pub interpretations: ::prost::alloc::vec::Vec<Interpretation>,
    /// Time when the question was created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The e-mail address of the user that created this question.
    #[prost(string, tag = "8")]
    pub user_email: ::prost::alloc::string::String,
    /// Input only. Immutable. Flags to request additional information for debugging purposes.
    #[prost(message, optional, tag = "9")]
    pub debug_flags: ::core::option::Option<DebugFlags>,
    /// Top level debug information.
    /// This will be stored as the type DebugInformation.
    /// Using Any so clients don't need to pull in anything
    /// inside the debug message.
    #[prost(message, optional, tag = "10")]
    pub debug_info: ::core::option::Option<::prost_types::Any>,
}
/// Details on the failure to interpret the question.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterpretError {
    /// Error message explaining why this question could not be interpreted.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// The code for the error category why the interpretation failed.
    #[prost(enumeration = "interpret_error::InterpretErrorCode", tag = "2")]
    pub code: i32,
    /// Details on interpretation failure.
    #[prost(message, optional, tag = "3")]
    pub details: ::core::option::Option<interpret_error::InterpretErrorDetails>,
}
/// Nested message and enum types in `InterpretError`.
pub mod interpret_error {
    /// Details on interpretation failure.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InterpretErrorDetails {
        /// Populated if parts of the query are unsupported.
        #[prost(message, optional, tag = "1")]
        pub unsupported_details: ::core::option::Option<InterpretUnsupportedDetails>,
        /// Populated if the query is incomplete.
        #[prost(message, optional, tag = "2")]
        pub incomplete_query_details: ::core::option::Option<
            InterpretIncompleteQueryDetails,
        >,
        /// Populated if the query was too ambiguous.
        #[prost(message, optional, tag = "3")]
        pub ambiguity_details: ::core::option::Option<InterpretAmbiguityDetails>,
    }
    /// Details about unsupported parts in a query.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InterpretUnsupportedDetails {
        /// Unsupported operators. For example: median.
        #[prost(string, repeated, tag = "1")]
        pub operators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Unsupported intents.
        #[prost(string, repeated, tag = "2")]
        pub intent: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Details about an incomplete query.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InterpretIncompleteQueryDetails {
        /// List of missing interpret entities.
        #[prost(enumeration = "super::InterpretEntity", repeated, tag = "1")]
        pub entities: ::prost::alloc::vec::Vec<i32>,
    }
    /// Details about a query that was too ambiguous. Currently, the message
    /// has no fields and its presence signals that there was ambiguity.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InterpretAmbiguityDetails {}
    /// The interpret error code provides an error category why the interpretation
    /// failed.
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
    pub enum InterpretErrorCode {
        /// No interpret error code was specified.
        Unspecified = 0,
        /// The query is not valid.
        InvalidQuery = 1,
        /// The interpreter failed to understand the question. For example, because
        /// it was too ambiguous.
        FailedToUnderstand = 2,
        /// The interpreter could understand the question, but was not able to arrive
        /// at an answer. For example, because a requested operation is not
        /// supported.
        FailedToAnswer = 3,
    }
    impl InterpretErrorCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InterpretErrorCode::Unspecified => "INTERPRET_ERROR_CODE_UNSPECIFIED",
                InterpretErrorCode::InvalidQuery => "INVALID_QUERY",
                InterpretErrorCode::FailedToUnderstand => "FAILED_TO_UNDERSTAND",
                InterpretErrorCode::FailedToAnswer => "FAILED_TO_ANSWER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INTERPRET_ERROR_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "INVALID_QUERY" => Some(Self::InvalidQuery),
                "FAILED_TO_UNDERSTAND" => Some(Self::FailedToUnderstand),
                "FAILED_TO_ANSWER" => Some(Self::FailedToAnswer),
                _ => None,
            }
        }
    }
}
/// Information about the backend status (such as BigQuery) of the execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionInfo {
    /// Status returned by the backend when the job was created.
    #[prost(message, optional, tag = "1")]
    pub job_creation_status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Status of the job execution.
    #[prost(enumeration = "execution_info::JobExecutionState", tag = "2")]
    pub job_execution_state: i32,
    /// Time when the execution was triggered.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// BigQuery job information.
    /// Future versions will have different backends. Hence, clients must make sure
    /// they can handle it when this field is not populated.
    #[prost(message, optional, tag = "4")]
    pub bigquery_job: ::core::option::Option<BigQueryJob>,
}
/// Nested message and enum types in `ExecutionInfo`.
pub mod execution_info {
    /// Enum of possible job execution statuses.
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
    pub enum JobExecutionState {
        /// No job execution was specified.
        Unspecified = 0,
        /// No job execution was requested, yet.
        NotExecuted = 1,
        /// The job is running.
        Running = 2,
        /// The job completed successfully.
        Succeeded = 3,
        /// The job completed unsuccessfully.
        Failed = 4,
    }
    impl JobExecutionState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JobExecutionState::Unspecified => "JOB_EXECUTION_STATE_UNSPECIFIED",
                JobExecutionState::NotExecuted => "NOT_EXECUTED",
                JobExecutionState::Running => "RUNNING",
                JobExecutionState::Succeeded => "SUCCEEDED",
                JobExecutionState::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "JOB_EXECUTION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "NOT_EXECUTED" => Some(Self::NotExecuted),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// BigQuery job information. This can be used to query the BigQuery API and
/// retrieve the current job's status (using
/// [jobs.get](<https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/get>)).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryJob {
    /// The job ID.
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
    /// The project ID of the job.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// The location where the job is running.
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
}
/// An interpretation of a natural language query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interpretation {
    /// List of data sources used in the current understanding.
    #[prost(string, repeated, tag = "1")]
    pub data_sources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The level of confidence that one of the interpretations is correct. This is
    /// a value in the range \[0, 1\] where a value of 0.5 or below is to be
    /// considered a low confidence.
    #[prost(double, tag = "2")]
    pub confidence: f64,
    /// A list of unused phrases. Clients should display a Did You Mean (DYM)
    ///   dialog if this is non-empty, even if this is the only interpretation.
    #[prost(string, repeated, tag = "3")]
    pub unused_phrases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Human readable representation of the query.
    #[prost(message, optional, tag = "4")]
    pub human_readable: ::core::option::Option<HumanReadable>,
    /// Information about the interpretation structure that helps to understand and
    /// visualize the response.
    #[prost(message, optional, tag = "5")]
    pub interpretation_structure: ::core::option::Option<InterpretationStructure>,
    /// Representation of the data query to be sent to the backend.
    #[prost(message, optional, tag = "6")]
    pub data_query: ::core::option::Option<DataQuery>,
    /// Information about the backend response. This is populated only if execution
    /// of an interpretation was requested.
    #[prost(message, optional, tag = "7")]
    pub execution_info: ::core::option::Option<ExecutionInfo>,
}
/// Representation of the data query for the backend.
/// This is provided for informational purposes only. Clients should not use
/// it to send it to the backend directly, but rather use the `execute` RPC
/// to trigger the execution. Using the `execute` RPC is needed in order to
/// track the state of a question and report on it correctly to the data
/// administrators.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataQuery {
    /// The generated SQL query to be sent to the backend.
    #[prost(string, tag = "1")]
    pub sql: ::prost::alloc::string::String,
}
/// Human readable interpretation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HumanReadable {
    /// Generated query explaining the interpretation.
    #[prost(message, optional, tag = "1")]
    pub generated_interpretation: ::core::option::Option<AnnotatedString>,
    /// Annotations on the original query.
    #[prost(message, optional, tag = "2")]
    pub original_question: ::core::option::Option<AnnotatedString>,
}
/// Information about the interpretation structure that helps to understand and
/// visualize the response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterpretationStructure {
    /// List of possible visualization types to apply for this interpretation. The
    /// order has no relevance.
    #[prost(
        enumeration = "interpretation_structure::VisualizationType",
        repeated,
        tag = "1"
    )]
    pub visualization_types: ::prost::alloc::vec::Vec<i32>,
    /// Information about the output columns, that is, the columns that will be
    /// returned by the backend.
    #[prost(message, repeated, tag = "2")]
    pub column_info: ::prost::alloc::vec::Vec<interpretation_structure::ColumnInfo>,
}
/// Nested message and enum types in `InterpretationStructure`.
pub mod interpretation_structure {
    /// Information about a column.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ColumnInfo {
        /// The alias of the output column as used by the backend. For example, the
        /// field name in the schema provided in the query response in BigQuery.
        #[prost(string, tag = "1")]
        pub output_alias: ::prost::alloc::string::String,
        /// Human readable name of the output column.
        #[prost(string, tag = "2")]
        pub display_name: ::prost::alloc::string::String,
    }
    /// Enumeration of visualzation types to use for query response data.
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
    pub enum VisualizationType {
        /// No visualization type was specified.
        Unspecified = 0,
        /// Show a table.
        Table = 1,
        /// Show a [bar
        /// chart](<https://developers.google.com/chart/interactive/docs/gallery/barchart>).
        BarChart = 2,
        /// Show a [column
        /// chart](<https://developers.google.com/chart/interactive/docs/gallery/columnchart>).
        ColumnChart = 3,
        /// Show a
        /// [timeline](<https://developers.google.com/chart/interactive/docs/gallery/timeline>).
        Timeline = 4,
        /// Show a [scatter
        /// plot](<https://developers.google.com/chart/interactive/docs/gallery/scatterchart>).
        ScatterPlot = 5,
        /// Show a [pie
        /// chart](<https://developers.google.com/chart/interactive/docs/gallery/piechart>).
        PieChart = 6,
        /// Show a [line
        /// chart](<https://developers.google.com/chart/interactive/docs/gallery/linechart>).
        LineChart = 7,
        /// Show an [area
        /// chart](<https://developers.google.com/chart/interactive/docs/gallery/areachart>).
        AreaChart = 8,
        /// Show a [combo
        /// chart](<https://developers.google.com/chart/interactive/docs/gallery/combochart>).
        ComboChart = 9,
        /// Show a
        /// [histogram](<https://developers.google.com/chart/interactive/docs/gallery/histogram>).
        Histogram = 10,
        /// This denotes queries when the user has not specified the particular type
        /// of chart and has mentioned only a generic chart name such as "Chart",
        /// "Plot", "Graph", etc. This will differentiate it from specific charting
        /// terms such as "Bar chart", "Pie chart", etc.
        GenericChart = 11,
        /// The user tried to specify a chart type, but the interpreter could not
        /// understand the type. The client should display a generic chart and may
        /// give a hint to the user that the requested type was not understood.
        ChartNotUnderstood = 12,
    }
    impl VisualizationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VisualizationType::Unspecified => "VISUALIZATION_TYPE_UNSPECIFIED",
                VisualizationType::Table => "TABLE",
                VisualizationType::BarChart => "BAR_CHART",
                VisualizationType::ColumnChart => "COLUMN_CHART",
                VisualizationType::Timeline => "TIMELINE",
                VisualizationType::ScatterPlot => "SCATTER_PLOT",
                VisualizationType::PieChart => "PIE_CHART",
                VisualizationType::LineChart => "LINE_CHART",
                VisualizationType::AreaChart => "AREA_CHART",
                VisualizationType::ComboChart => "COMBO_CHART",
                VisualizationType::Histogram => "HISTOGRAM",
                VisualizationType::GenericChart => "GENERIC_CHART",
                VisualizationType::ChartNotUnderstood => "CHART_NOT_UNDERSTOOD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VISUALIZATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "TABLE" => Some(Self::Table),
                "BAR_CHART" => Some(Self::BarChart),
                "COLUMN_CHART" => Some(Self::ColumnChart),
                "TIMELINE" => Some(Self::Timeline),
                "SCATTER_PLOT" => Some(Self::ScatterPlot),
                "PIE_CHART" => Some(Self::PieChart),
                "LINE_CHART" => Some(Self::LineChart),
                "AREA_CHART" => Some(Self::AreaChart),
                "COMBO_CHART" => Some(Self::ComboChart),
                "HISTOGRAM" => Some(Self::Histogram),
                "GENERIC_CHART" => Some(Self::GenericChart),
                "CHART_NOT_UNDERSTOOD" => Some(Self::ChartNotUnderstood),
                _ => None,
            }
        }
    }
}
/// Configuriation of debug flags.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugFlags {
    /// Whether to include the original VAQuery.
    #[prost(bool, tag = "1")]
    pub include_va_query: bool,
    /// Whether to include the original nested VAQuery.
    #[prost(bool, tag = "2")]
    pub include_nested_va_query: bool,
    /// Whether to include the original human interpretation strings generated
    /// by Analyza.
    #[prost(bool, tag = "3")]
    pub include_human_interpretation: bool,
    /// Whether to include the Aqua debug response.
    #[prost(bool, tag = "4")]
    pub include_aqua_debug_response: bool,
    /// The time in milliseconds from Unix epoch to be used
    /// to process the query. This is useful for testing
    /// the queries at different time period.
    /// If not set or time_override <= 0, then the current
    /// time is used.
    #[prost(int64, tag = "5")]
    pub time_override: i64,
    /// Set to true if request is initiated by an internal Google user.
    #[prost(bool, tag = "6")]
    pub is_internal_google_user: bool,
    /// Determines whether cache needs to be ignored. If set to
    /// true, cache won't be queried and updated.
    #[prost(bool, tag = "7")]
    pub ignore_cache: bool,
    /// Whether to include the request/response pair from the call to the
    /// EntityIndex for SearchEntities.
    #[prost(bool, tag = "8")]
    pub include_search_entities_rpc: bool,
    /// Whether to include the request/response pair from the call to the
    /// Annotations service for ListColumnAnnotations.
    #[prost(bool, tag = "9")]
    pub include_list_column_annotations_rpc: bool,
    /// Whether to include the entity list passed to Analyza.
    #[prost(bool, tag = "10")]
    pub include_virtual_analyst_entities: bool,
    /// Whether to include the table list.
    #[prost(bool, tag = "11")]
    pub include_table_list: bool,
    /// Whether to include the domain list.
    #[prost(bool, tag = "12")]
    pub include_domain_list: bool,
}
/// Query entities of an interpretation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InterpretEntity {
    /// No interpret entity was specified.
    Unspecified = 0,
    /// A dimenstion entity.
    Dimension = 1,
    /// A metric entity.
    Metric = 2,
}
impl InterpretEntity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InterpretEntity::Unspecified => "INTERPRET_ENTITY_UNSPECIFIED",
            InterpretEntity::Dimension => "DIMENSION",
            InterpretEntity::Metric => "METRIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INTERPRET_ENTITY_UNSPECIFIED" => Some(Self::Unspecified),
            "DIMENSION" => Some(Self::Dimension),
            "METRIC" => Some(Self::Metric),
            _ => None,
        }
    }
}
/// Feedback provided by a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserFeedback {
    /// Required. The unique identifier for the user feedback.
    /// User feedback is a singleton resource on a Question.
    /// Example: `projects/foo/locations/bar/questions/1234/userFeedback`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Free form user feedback, such as a text box.
    #[prost(string, tag = "2")]
    pub free_form_feedback: ::prost::alloc::string::String,
    /// The user feedback rating
    #[prost(enumeration = "user_feedback::UserFeedbackRating", tag = "3")]
    pub rating: i32,
}
/// Nested message and enum types in `UserFeedback`.
pub mod user_feedback {
    /// Enumeration of feedback ratings.
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
    pub enum UserFeedbackRating {
        /// No rating was specified.
        Unspecified = 0,
        /// The user provided positive feedback.
        Positive = 1,
        /// The user provided negative feedback.
        Negative = 2,
    }
    impl UserFeedbackRating {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserFeedbackRating::Unspecified => "USER_FEEDBACK_RATING_UNSPECIFIED",
                UserFeedbackRating::Positive => "POSITIVE",
                UserFeedbackRating::Negative => "NEGATIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "USER_FEEDBACK_RATING_UNSPECIFIED" => Some(Self::Unspecified),
                "POSITIVE" => Some(Self::Positive),
                "NEGATIVE" => Some(Self::Negative),
                _ => None,
            }
        }
    }
}
/// A request to get a previously created question.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQuestionRequest {
    /// Required. The unique identifier for the question.
    /// Example: `projects/foo/locations/bar/questions/1234`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The list of fields to be retrieved.
    #[prost(message, optional, tag = "2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to create a question resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateQuestionRequest {
    /// Required. The name of the project this data source reference belongs to.
    /// Example: `projects/foo/locations/bar`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The question to create.
    #[prost(message, optional, tag = "2")]
    pub question: ::core::option::Option<Question>,
}
/// Request to execute an interpretation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteQuestionRequest {
    /// Required. The unique identifier for the question.
    /// Example: `projects/foo/locations/bar/questions/1234`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Index of the interpretation to execute.
    #[prost(int32, tag = "2")]
    pub interpretation_index: i32,
}
/// Request to get user feedback.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserFeedbackRequest {
    /// Required. The unique identifier for the user feedback.
    /// User feedback is a singleton resource on a Question.
    /// Example: `projects/foo/locations/bar/questions/1234/userFeedback`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to updates user feedback.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserFeedbackRequest {
    /// Required. The user feedback to update. This can be called even if there is no
    /// user feedback so far.
    /// The feedback's name field is used to identify the user feedback (and the
    /// corresponding question) to update.
    #[prost(message, optional, tag = "1")]
    pub user_feedback: ::core::option::Option<UserFeedback>,
    /// The list of fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Generated client implementations.
pub mod question_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to interpret natural language queries.
    /// The service allows to create `Question` resources that are interpreted and
    /// are filled with one or more interpretations if the question could be
    /// interpreted. Once a `Question` resource is created and has at least one
    /// interpretation, an interpretation can be chosen for execution, which
    /// triggers a query to the backend (for BigQuery, it will create a job).
    /// Upon successful execution of that interpretation, backend specific
    /// information will be returned so that the client can retrieve the results
    /// from the backend.
    ///
    /// The `Question` resources are named `projects/*/locations/*/questions/*`.
    ///
    /// The `Question` resource has a singletion sub-resource `UserFeedback` named
    /// `projects/*/locations/*/questions/*/userFeedback`, which allows access to
    /// user feedback.
    #[derive(Debug, Clone)]
    pub struct QuestionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> QuestionServiceClient<T>
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
        ) -> QuestionServiceClient<InterceptedService<T, F>>
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
            QuestionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets a previously created question.
        pub async fn get_question(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQuestionRequest>,
        ) -> std::result::Result<tonic::Response<super::Question>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataqna.v1alpha.QuestionService/GetQuestion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataqna.v1alpha.QuestionService",
                        "GetQuestion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a question.
        pub async fn create_question(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateQuestionRequest>,
        ) -> std::result::Result<tonic::Response<super::Question>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataqna.v1alpha.QuestionService/CreateQuestion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataqna.v1alpha.QuestionService",
                        "CreateQuestion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Executes an interpretation.
        pub async fn execute_question(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteQuestionRequest>,
        ) -> std::result::Result<tonic::Response<super::Question>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataqna.v1alpha.QuestionService/ExecuteQuestion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataqna.v1alpha.QuestionService",
                        "ExecuteQuestion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets previously created user feedback.
        pub async fn get_user_feedback(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserFeedbackRequest>,
        ) -> std::result::Result<tonic::Response<super::UserFeedback>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataqna.v1alpha.QuestionService/GetUserFeedback",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataqna.v1alpha.QuestionService",
                        "GetUserFeedback",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates user feedback. This creates user feedback if there was none before
        /// (upsert).
        pub async fn update_user_feedback(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserFeedbackRequest>,
        ) -> std::result::Result<tonic::Response<super::UserFeedback>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataqna.v1alpha.QuestionService/UpdateUserFeedback",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.dataqna.v1alpha.QuestionService",
                        "UpdateUserFeedback",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
