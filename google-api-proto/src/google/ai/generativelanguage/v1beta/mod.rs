/// The base structured datatype containing multi-part content of a message.
///
/// A `Content` includes a `role` field designating the producer of the `Content`
/// and a `parts` field containing multi-part data that contains the content of
/// the message turn.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    /// Ordered `Parts` that constitute a single message. Parts may have different
    /// MIME types.
    #[prost(message, repeated, tag = "1")]
    pub parts: ::prost::alloc::vec::Vec<Part>,
    /// Optional. The producer of the content. Must be either 'user' or 'model'.
    ///
    /// Useful to set for multi-turn conversations, otherwise can be left blank
    /// or unset.
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
}
/// A datatype containing media that is part of a multi-part `Content` message.
///
/// A `Part` consists of data which has an associated datatype. A `Part` can only
/// contain one of the accepted types in `Part.data`.
///
/// A `Part` must have a fixed IANA MIME type identifying the type and subtype
/// of the media if the `inline_data` field is filled with raw bytes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Part {
    #[prost(oneof = "part::Data", tags = "2, 3, 4, 5")]
    pub data: ::core::option::Option<part::Data>,
}
/// Nested message and enum types in `Part`.
pub mod part {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// Inline text.
        #[prost(string, tag = "2")]
        Text(::prost::alloc::string::String),
        /// Inline media bytes.
        #[prost(message, tag = "3")]
        InlineData(super::Blob),
        /// A predicted `FunctionCall` returned from the model that contains
        /// a string representing the `FunctionDeclaration.name` with the
        /// arguments and their values.
        #[prost(message, tag = "4")]
        FunctionCall(super::FunctionCall),
        /// The result output of a `FunctionCall` that contains a string
        /// representing the `FunctionDeclaration.name` and a structured JSON
        /// object containing any output from the function is used as context to
        /// the model.
        #[prost(message, tag = "5")]
        FunctionResponse(super::FunctionResponse),
    }
}
/// Raw media bytes.
///
/// Text should not be sent as raw bytes, use the 'text' field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blob {
    /// The IANA standard MIME type of the source data.
    /// Accepted types include: "image/png", "image/jpeg", "image/heic",
    /// "image/heif", "image/webp".
    #[prost(string, tag = "1")]
    pub mime_type: ::prost::alloc::string::String,
    /// Raw bytes for media formats.
    #[prost(bytes = "bytes", tag = "2")]
    pub data: ::prost::bytes::Bytes,
}
/// Tool details that the model may use to generate response.
///
/// A `Tool` is a piece of code that enables the system to interact with
/// external systems to perform an action, or set of actions, outside of
/// knowledge and scope of the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tool {
    /// Optional. A list of `FunctionDeclarations` available to the model that can
    /// be used for function calling.
    ///
    /// The model or system does not execute the function. Instead the defined
    /// function may be returned as a [FunctionCall][content.part.function_call]
    /// with arguments to the client side for execution. The model may decide to
    /// call a subset of these functions by populating
    /// [FunctionCall][content.part.function_call] in the response. The next
    /// conversation turn may contain a
    /// [FunctionResponse][content.part.function_response]
    /// with the \[conent.role\] "function" generation context for the next model
    /// turn.
    #[prost(message, repeated, tag = "1")]
    pub function_declarations: ::prost::alloc::vec::Vec<FunctionDeclaration>,
}
/// Structured representation of a function declaration as defined by the
/// [OpenAPI 3.03 specification](<https://spec.openapis.org/oas/v3.0.3>). Included
/// in this declaration are the function name and parameters. This
/// FunctionDeclaration is a representation of a block of code that can be used
/// as a `Tool` by the model and executed by the client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionDeclaration {
    /// Required. The name of the function.
    /// Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum
    /// length of 63.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. A brief description of the function.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Describes the parameters to this function. Reflects the Open
    /// API 3.03 Parameter Object string Key: the name of the parameter. Parameter
    /// names are case sensitive. Schema Value: the Schema defining the type used
    /// for the parameter.
    #[prost(message, optional, tag = "3")]
    pub parameters: ::core::option::Option<Schema>,
}
/// A predicted `FunctionCall` returned from the model that contains
/// a string representing the `FunctionDeclaration.name` with the
/// arguments and their values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionCall {
    /// Required. The name of the function to call.
    /// Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum
    /// length of 63.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The function parameters and values in JSON object format.
    #[prost(message, optional, tag = "2")]
    pub args: ::core::option::Option<::prost_types::Struct>,
}
/// The result output from a `FunctionCall` that contains a string
/// representing the `FunctionDeclaration.name` and a structured JSON
/// object containing any output from the function is used as context to
/// the model. This should contain the result of a`FunctionCall` made
/// based on model prediction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionResponse {
    /// Required. The name of the function to call.
    /// Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum
    /// length of 63.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The function response in JSON object format.
    #[prost(message, optional, tag = "2")]
    pub response: ::core::option::Option<::prost_types::Struct>,
}
/// The `Schema` object allows the definition of input and output data types.
/// These types can be objects, but also primitives and arrays.
/// Represents a select subset of an [OpenAPI 3.0 schema
/// object](<https://spec.openapis.org/oas/v3.0.3#schema>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// Optional. Data type.
    #[prost(enumeration = "Type", tag = "1")]
    pub r#type: i32,
    /// Optional. The format of the data. This is used obnly for primative
    /// datatypes. Supported formats:
    ///   for NUMBER type: float, double
    ///   for INTEGER type: int32, int64
    #[prost(string, tag = "2")]
    pub format: ::prost::alloc::string::String,
    /// Optional. A brief description of the parameter. This could contain examples
    /// of use. Parameter description may be formatted as Markdown.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Indicates if the value may be null.
    #[prost(bool, tag = "4")]
    pub nullable: bool,
    /// Optional. Possible values of the element of Type.STRING with enum format.
    /// For example we can define an Enum Direction as :
    /// {type:STRING, format:enum, enum:\["EAST", NORTH", "SOUTH", "WEST"\]}
    #[prost(string, repeated, tag = "5")]
    pub r#enum: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Schema of the elements of Type.ARRAY.
    #[prost(message, optional, boxed, tag = "6")]
    pub items: ::core::option::Option<::prost::alloc::boxed::Box<Schema>>,
    /// Optional. Properties of Type.OBJECT.
    #[prost(btree_map = "string, message", tag = "7")]
    pub properties: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        Schema,
    >,
    /// Optional. Required properties of Type.OBJECT.
    #[prost(string, repeated, tag = "8")]
    pub required: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Passage included inline with a grounding configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroundingPassage {
    /// Identifier for the passage for attributing this passage in grounded
    /// answers.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Content of the passage.
    #[prost(message, optional, tag = "2")]
    pub content: ::core::option::Option<Content>,
}
/// A repeated list of passages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroundingPassages {
    /// List of passages.
    #[prost(message, repeated, tag = "1")]
    pub passages: ::prost::alloc::vec::Vec<GroundingPassage>,
}
/// Type contains the list of OpenAPI data types as defined by
/// <https://spec.openapis.org/oas/v3.0.3#data-types>
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    /// Not specified, should not be used.
    Unspecified = 0,
    /// String type.
    String = 1,
    /// Number type.
    Number = 2,
    /// Integer type.
    Integer = 3,
    /// Boolean type.
    Boolean = 4,
    /// Array type.
    Array = 5,
    /// Object type.
    Object = 6,
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
            Type::Number => "NUMBER",
            Type::Integer => "INTEGER",
            Type::Boolean => "BOOLEAN",
            Type::Array => "ARRAY",
            Type::Object => "OBJECT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "STRING" => Some(Self::String),
            "NUMBER" => Some(Self::Number),
            "INTEGER" => Some(Self::Integer),
            "BOOLEAN" => Some(Self::Boolean),
            "ARRAY" => Some(Self::Array),
            "OBJECT" => Some(Self::Object),
            _ => None,
        }
    }
}
/// A collection of source attributions for a piece of content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CitationMetadata {
    /// Citations to sources for a specific response.
    #[prost(message, repeated, tag = "1")]
    pub citation_sources: ::prost::alloc::vec::Vec<CitationSource>,
}
/// A citation to a source for a portion of a specific response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CitationSource {
    /// Optional. Start of segment of the response that is attributed to this
    /// source.
    ///
    /// Index indicates the start of the segment, measured in bytes.
    #[prost(int32, optional, tag = "1")]
    pub start_index: ::core::option::Option<i32>,
    /// Optional. End of the attributed segment, exclusive.
    #[prost(int32, optional, tag = "2")]
    pub end_index: ::core::option::Option<i32>,
    /// Optional. URI that is attributed as a source for a portion of the text.
    #[prost(string, optional, tag = "3")]
    pub uri: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. License for the GitHub project that is attributed as a source for
    /// segment.
    ///
    /// License info is required for code citations.
    #[prost(string, optional, tag = "4")]
    pub license: ::core::option::Option<::prost::alloc::string::String>,
}
/// A `Corpus` is a collection of `Document`s.
/// A project can create up to 5 corpora.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Corpus {
    /// Immutable. Identifier. The `Corpus` resource name. The ID (name excluding
    /// the "corpora/" prefix) can contain up to 40 characters that are lowercase
    /// alphanumeric or dashes
    /// (-). The ID cannot start or end with a dash. If the name is empty on
    /// create, a unique name will be derived from `display_name` along with a 12
    /// character random suffix.
    /// Example: `corpora/my-awesome-corpora-123a456b789c`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The human-readable display name for the `Corpus`. The display
    /// name must be no more than 128 characters in length, including spaces.
    /// Example: "Docs on Semantic Retriever"
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The Timestamp of when the `Corpus` was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The Timestamp of when the `Corpus` was last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A `Document` is a collection of `Chunk`s.
/// A `Corpus` can have a maximum of 10,000 `Document`s.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// Immutable. Identifier. The `Document` resource name. The ID (name excluding
    /// the "corpora/*/documents/" prefix) can contain up to 40 characters that are
    /// lowercase alphanumeric or dashes (-). The ID cannot start or end with a
    /// dash. If the name is empty on create, a unique name will be derived from
    /// `display_name` along with a 12 character random suffix.
    /// Example: `corpora/{corpus_id}/documents/my-awesome-doc-123a456b789c`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The human-readable display name for the `Document`. The display
    /// name must be no more than 512 characters in length, including spaces.
    /// Example: "Semantic Retriever Documentation"
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. User provided custom metadata stored as key-value pairs used for
    /// querying. A `Document` can have a maximum of 20 `CustomMetadata`.
    #[prost(message, repeated, tag = "3")]
    pub custom_metadata: ::prost::alloc::vec::Vec<CustomMetadata>,
    /// Output only. The Timestamp of when the `Document` was last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The Timestamp of when the `Document` was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// User provided string values assigned to a single metadata key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringList {
    /// The string values of the metadata to store.
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// User provided metadata stored as key-value pairs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomMetadata {
    /// Required. The key of the metadata to store.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(oneof = "custom_metadata::Value", tags = "2, 6, 7")]
    pub value: ::core::option::Option<custom_metadata::Value>,
}
/// Nested message and enum types in `CustomMetadata`.
pub mod custom_metadata {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// The string value of the metadata to store.
        #[prost(string, tag = "2")]
        StringValue(::prost::alloc::string::String),
        /// The StringList value of the metadata to store.
        #[prost(message, tag = "6")]
        StringListValue(super::StringList),
        /// The numeric value of the metadata to store.
        #[prost(float, tag = "7")]
        NumericValue(f32),
    }
}
/// User provided filter to limit retrieval based on `Chunk` or `Document` level
/// metadata values.
/// Example (genre = drama OR genre = action):
///    key = "document.custom_metadata.genre"
///    conditions = [{string_value = "drama", operation = EQUAL},
///                  {string_value = "action", operation = EQUAL}]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataFilter {
    /// Required. The key of the metadata to filter on.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Required. The `Condition`s for the given key that will trigger this filter.
    /// Multiple `Condition`s are joined by logical ORs.
    #[prost(message, repeated, tag = "2")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
}
/// Filter condition applicable to a single key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    /// Required. Operator applied to the given key-value pair to trigger the
    /// condition.
    #[prost(enumeration = "condition::Operator", tag = "5")]
    pub operation: i32,
    /// The value type must be consistent with the value type defined in the field
    /// for the corresponding key. If the value types are not consistent, the
    /// result will be an empty set. When the `CustomMetadata` has a `StringList`
    /// value type, the filtering condition should use `string_value` paired with
    /// an INCLUDES/EXCLUDES operation, otherwise the result will also be an empty
    /// set.
    #[prost(oneof = "condition::Value", tags = "1, 6")]
    pub value: ::core::option::Option<condition::Value>,
}
/// Nested message and enum types in `Condition`.
pub mod condition {
    /// Defines the valid operators that can be applied to a key-value pair.
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
    pub enum Operator {
        /// The default value. This value is unused.
        Unspecified = 0,
        /// Supported by numeric.
        Less = 1,
        /// Supported by numeric.
        LessEqual = 2,
        /// Supported by numeric & string.
        Equal = 3,
        /// Supported by numeric.
        GreaterEqual = 4,
        /// Supported by numeric.
        Greater = 5,
        /// Supported by numeric & string.
        NotEqual = 6,
        /// Supported by string only when `CustomMetadata` value type for the given
        /// key has a `string_list_value`.
        Includes = 7,
        /// Supported by string only when `CustomMetadata` value type for the given
        /// key has a `string_list_value`.
        Excludes = 8,
    }
    impl Operator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operator::Unspecified => "OPERATOR_UNSPECIFIED",
                Operator::Less => "LESS",
                Operator::LessEqual => "LESS_EQUAL",
                Operator::Equal => "EQUAL",
                Operator::GreaterEqual => "GREATER_EQUAL",
                Operator::Greater => "GREATER",
                Operator::NotEqual => "NOT_EQUAL",
                Operator::Includes => "INCLUDES",
                Operator::Excludes => "EXCLUDES",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATOR_UNSPECIFIED" => Some(Self::Unspecified),
                "LESS" => Some(Self::Less),
                "LESS_EQUAL" => Some(Self::LessEqual),
                "EQUAL" => Some(Self::Equal),
                "GREATER_EQUAL" => Some(Self::GreaterEqual),
                "GREATER" => Some(Self::Greater),
                "NOT_EQUAL" => Some(Self::NotEqual),
                "INCLUDES" => Some(Self::Includes),
                "EXCLUDES" => Some(Self::Excludes),
                _ => None,
            }
        }
    }
    /// The value type must be consistent with the value type defined in the field
    /// for the corresponding key. If the value types are not consistent, the
    /// result will be an empty set. When the `CustomMetadata` has a `StringList`
    /// value type, the filtering condition should use `string_value` paired with
    /// an INCLUDES/EXCLUDES operation, otherwise the result will also be an empty
    /// set.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// The string value to filter the metadata on.
        #[prost(string, tag = "1")]
        StringValue(::prost::alloc::string::String),
        /// The numeric value to filter the metadata on.
        #[prost(float, tag = "6")]
        NumericValue(f32),
    }
}
/// A `Chunk` is a subpart of a `Document` that is treated as an independent unit
/// for the purposes of vector representation and storage.
/// A `Corpus` can have a maximum of 1 million `Chunk`s.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chunk {
    /// Immutable. Identifier. The `Chunk` resource name. The ID (name excluding
    /// the "corpora/*/documents/*/chunks/" prefix) can contain up to 40 characters
    /// that are lowercase alphanumeric or dashes (-). The ID cannot start or end
    /// with a dash. If the name is empty on create, a random 12-character unique
    /// ID will be generated.
    /// Example: `corpora/{corpus_id}/documents/{document_id}/chunks/123a456b789c`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The content for the `Chunk`, such as the text string.
    /// The maximum number of tokens per chunk is 2043.
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ChunkData>,
    /// Optional. User provided custom metadata stored as key-value pairs.
    /// The maximum number of `CustomMetadata` per chunk is 20.
    #[prost(message, repeated, tag = "3")]
    pub custom_metadata: ::prost::alloc::vec::Vec<CustomMetadata>,
    /// Output only. The Timestamp of when the `Chunk` was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The Timestamp of when the `Chunk` was last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Current state of the `Chunk`.
    #[prost(enumeration = "chunk::State", tag = "6")]
    pub state: i32,
}
/// Nested message and enum types in `Chunk`.
pub mod chunk {
    /// States for the lifecycle of a `Chunk`.
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
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// `Chunk` is being processed (embedding and vector storage).
        PendingProcessing = 1,
        /// `Chunk` is processed and available for querying.
        Active = 2,
        /// `Chunk` failed processing.
        Failed = 10,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::PendingProcessing => "STATE_PENDING_PROCESSING",
                State::Active => "STATE_ACTIVE",
                State::Failed => "STATE_FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATE_PENDING_PROCESSING" => Some(Self::PendingProcessing),
                "STATE_ACTIVE" => Some(Self::Active),
                "STATE_FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// Extracted data that represents the `Chunk` content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkData {
    #[prost(oneof = "chunk_data::Data", tags = "1")]
    pub data: ::core::option::Option<chunk_data::Data>,
}
/// Nested message and enum types in `ChunkData`.
pub mod chunk_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// The `Chunk` content as a string.
        /// The maximum number of tokens per chunk is 2043.
        #[prost(string, tag = "1")]
        StringValue(::prost::alloc::string::String),
    }
}
/// Content filtering metadata associated with processing a single request.
///
/// ContentFilter contains a reason and an optional supporting string. The reason
/// may be unspecified.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentFilter {
    /// The reason content was blocked during request processing.
    #[prost(enumeration = "content_filter::BlockedReason", tag = "1")]
    pub reason: i32,
    /// A string that describes the filtering behavior in more detail.
    #[prost(string, optional, tag = "2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ContentFilter`.
pub mod content_filter {
    /// A list of reasons why content may have been blocked.
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
    pub enum BlockedReason {
        /// A blocked reason was not specified.
        Unspecified = 0,
        /// Content was blocked by safety settings.
        Safety = 1,
        /// Content was blocked, but the reason is uncategorized.
        Other = 2,
    }
    impl BlockedReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BlockedReason::Unspecified => "BLOCKED_REASON_UNSPECIFIED",
                BlockedReason::Safety => "SAFETY",
                BlockedReason::Other => "OTHER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BLOCKED_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "SAFETY" => Some(Self::Safety),
                "OTHER" => Some(Self::Other),
                _ => None,
            }
        }
    }
}
/// Safety feedback for an entire request.
///
/// This field is populated if content in the input and/or response is blocked
/// due to safety settings. SafetyFeedback may not exist for every HarmCategory.
/// Each SafetyFeedback will return the safety settings used by the request as
/// well as the lowest HarmProbability that should be allowed in order to return
/// a result.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafetyFeedback {
    /// Safety rating evaluated from content.
    #[prost(message, optional, tag = "1")]
    pub rating: ::core::option::Option<SafetyRating>,
    /// Safety settings applied to the request.
    #[prost(message, optional, tag = "2")]
    pub setting: ::core::option::Option<SafetySetting>,
}
/// Safety rating for a piece of content.
///
/// The safety rating contains the category of harm and the
/// harm probability level in that category for a piece of content.
/// Content is classified for safety across a number of
/// harm categories and the probability of the harm classification is included
/// here.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafetyRating {
    /// Required. The category for this rating.
    #[prost(enumeration = "HarmCategory", tag = "3")]
    pub category: i32,
    /// Required. The probability of harm for this content.
    #[prost(enumeration = "safety_rating::HarmProbability", tag = "4")]
    pub probability: i32,
    /// Was this content blocked because of this rating?
    #[prost(bool, tag = "5")]
    pub blocked: bool,
}
/// Nested message and enum types in `SafetyRating`.
pub mod safety_rating {
    /// The probability that a piece of content is harmful.
    ///
    /// The classification system gives the probability of the content being
    /// unsafe. This does not indicate the severity of harm for a piece of content.
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
    pub enum HarmProbability {
        /// Probability is unspecified.
        Unspecified = 0,
        /// Content has a negligible chance of being unsafe.
        Negligible = 1,
        /// Content has a low chance of being unsafe.
        Low = 2,
        /// Content has a medium chance of being unsafe.
        Medium = 3,
        /// Content has a high chance of being unsafe.
        High = 4,
    }
    impl HarmProbability {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HarmProbability::Unspecified => "HARM_PROBABILITY_UNSPECIFIED",
                HarmProbability::Negligible => "NEGLIGIBLE",
                HarmProbability::Low => "LOW",
                HarmProbability::Medium => "MEDIUM",
                HarmProbability::High => "HIGH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HARM_PROBABILITY_UNSPECIFIED" => Some(Self::Unspecified),
                "NEGLIGIBLE" => Some(Self::Negligible),
                "LOW" => Some(Self::Low),
                "MEDIUM" => Some(Self::Medium),
                "HIGH" => Some(Self::High),
                _ => None,
            }
        }
    }
}
/// Safety setting, affecting the safety-blocking behavior.
///
/// Passing a safety setting for a category changes the allowed proability that
/// content is blocked.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafetySetting {
    /// Required. The category for this setting.
    #[prost(enumeration = "HarmCategory", tag = "3")]
    pub category: i32,
    /// Required. Controls the probability threshold at which harm is blocked.
    #[prost(enumeration = "safety_setting::HarmBlockThreshold", tag = "4")]
    pub threshold: i32,
}
/// Nested message and enum types in `SafetySetting`.
pub mod safety_setting {
    /// Block at and beyond a specified harm probability.
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
    pub enum HarmBlockThreshold {
        /// Threshold is unspecified.
        Unspecified = 0,
        /// Content with NEGLIGIBLE will be allowed.
        BlockLowAndAbove = 1,
        /// Content with NEGLIGIBLE and LOW will be allowed.
        BlockMediumAndAbove = 2,
        /// Content with NEGLIGIBLE, LOW, and MEDIUM will be allowed.
        BlockOnlyHigh = 3,
        /// All content will be allowed.
        BlockNone = 4,
    }
    impl HarmBlockThreshold {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HarmBlockThreshold::Unspecified => "HARM_BLOCK_THRESHOLD_UNSPECIFIED",
                HarmBlockThreshold::BlockLowAndAbove => "BLOCK_LOW_AND_ABOVE",
                HarmBlockThreshold::BlockMediumAndAbove => "BLOCK_MEDIUM_AND_ABOVE",
                HarmBlockThreshold::BlockOnlyHigh => "BLOCK_ONLY_HIGH",
                HarmBlockThreshold::BlockNone => "BLOCK_NONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HARM_BLOCK_THRESHOLD_UNSPECIFIED" => Some(Self::Unspecified),
                "BLOCK_LOW_AND_ABOVE" => Some(Self::BlockLowAndAbove),
                "BLOCK_MEDIUM_AND_ABOVE" => Some(Self::BlockMediumAndAbove),
                "BLOCK_ONLY_HIGH" => Some(Self::BlockOnlyHigh),
                "BLOCK_NONE" => Some(Self::BlockNone),
                _ => None,
            }
        }
    }
}
/// The category of a rating.
///
/// These categories cover various kinds of harms that developers
/// may wish to adjust.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HarmCategory {
    /// Category is unspecified.
    Unspecified = 0,
    /// Negative or harmful comments targeting identity and/or protected attribute.
    Derogatory = 1,
    /// Content that is rude, disrepspectful, or profane.
    Toxicity = 2,
    /// Describes scenarios depictng violence against an individual or group, or
    /// general descriptions of gore.
    Violence = 3,
    /// Contains references to sexual acts or other lewd content.
    Sexual = 4,
    /// Promotes unchecked medical advice.
    Medical = 5,
    /// Dangerous content that promotes, facilitates, or encourages harmful acts.
    Dangerous = 6,
    /// Harasment content.
    Harassment = 7,
    /// Hate speech and content.
    HateSpeech = 8,
    /// Sexually explicit content.
    SexuallyExplicit = 9,
    /// Dangerous content.
    DangerousContent = 10,
}
impl HarmCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HarmCategory::Unspecified => "HARM_CATEGORY_UNSPECIFIED",
            HarmCategory::Derogatory => "HARM_CATEGORY_DEROGATORY",
            HarmCategory::Toxicity => "HARM_CATEGORY_TOXICITY",
            HarmCategory::Violence => "HARM_CATEGORY_VIOLENCE",
            HarmCategory::Sexual => "HARM_CATEGORY_SEXUAL",
            HarmCategory::Medical => "HARM_CATEGORY_MEDICAL",
            HarmCategory::Dangerous => "HARM_CATEGORY_DANGEROUS",
            HarmCategory::Harassment => "HARM_CATEGORY_HARASSMENT",
            HarmCategory::HateSpeech => "HARM_CATEGORY_HATE_SPEECH",
            HarmCategory::SexuallyExplicit => "HARM_CATEGORY_SEXUALLY_EXPLICIT",
            HarmCategory::DangerousContent => "HARM_CATEGORY_DANGEROUS_CONTENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HARM_CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
            "HARM_CATEGORY_DEROGATORY" => Some(Self::Derogatory),
            "HARM_CATEGORY_TOXICITY" => Some(Self::Toxicity),
            "HARM_CATEGORY_VIOLENCE" => Some(Self::Violence),
            "HARM_CATEGORY_SEXUAL" => Some(Self::Sexual),
            "HARM_CATEGORY_MEDICAL" => Some(Self::Medical),
            "HARM_CATEGORY_DANGEROUS" => Some(Self::Dangerous),
            "HARM_CATEGORY_HARASSMENT" => Some(Self::Harassment),
            "HARM_CATEGORY_HATE_SPEECH" => Some(Self::HateSpeech),
            "HARM_CATEGORY_SEXUALLY_EXPLICIT" => Some(Self::SexuallyExplicit),
            "HARM_CATEGORY_DANGEROUS_CONTENT" => Some(Self::DangerousContent),
            _ => None,
        }
    }
}
/// Request to generate a completion from the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateContentRequest {
    /// Required. The name of the `Model` to use for generating the completion.
    ///
    /// Format: `name=models/{model}`.
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The content of the current conversation with the model.
    ///
    /// For single-turn queries, this is a single instance. For multi-turn queries,
    /// this is a repeated field that contains conversation history + latest
    /// request.
    #[prost(message, repeated, tag = "2")]
    pub contents: ::prost::alloc::vec::Vec<Content>,
    /// Optional. A list of `Tools` the model may use to generate the next
    /// response.
    ///
    /// A `Tool` is a piece of code that enables the system to interact with
    /// external systems to perform an action, or set of actions, outside of
    /// knowledge and scope of the model. The only supported tool is currently
    /// `Function`.
    #[prost(message, repeated, tag = "5")]
    pub tools: ::prost::alloc::vec::Vec<Tool>,
    /// Optional. A list of unique `SafetySetting` instances for blocking unsafe
    /// content.
    ///
    /// This will be enforced on the `GenerateContentRequest.contents` and
    /// `GenerateContentResponse.candidates`. There should not be more than one
    /// setting for each `SafetyCategory` type. The API will block any contents and
    /// responses that fail to meet the thresholds set by these settings. This list
    /// overrides the default settings for each `SafetyCategory` specified in the
    /// safety_settings. If there is no `SafetySetting` for a given
    /// `SafetyCategory` provided in the list, the API will use the default safety
    /// setting for that category. Harm categories HARM_CATEGORY_HATE_SPEECH,
    /// HARM_CATEGORY_SEXUALLY_EXPLICIT, HARM_CATEGORY_DANGEROUS_CONTENT,
    /// HARM_CATEGORY_HARASSMENT are supported.
    #[prost(message, repeated, tag = "3")]
    pub safety_settings: ::prost::alloc::vec::Vec<SafetySetting>,
    /// Optional. Configuration options for model generation and outputs.
    #[prost(message, optional, tag = "4")]
    pub generation_config: ::core::option::Option<GenerationConfig>,
}
/// Configuration options for model generation and outputs. Not all parameters
/// may be configurable for every model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationConfig {
    /// Optional. Number of generated responses to return.
    ///
    /// This value must be between \[1, 8\], inclusive. If unset, this will default
    /// to 1.
    #[prost(int32, optional, tag = "1")]
    pub candidate_count: ::core::option::Option<i32>,
    /// Optional. The set of character sequences (up to 5) that will stop output
    /// generation. If specified, the API will stop at the first appearance of a
    /// stop sequence. The stop sequence will not be included as part of the
    /// response.
    #[prost(string, repeated, tag = "2")]
    pub stop_sequences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The maximum number of tokens to include in a candidate.
    ///
    /// If unset, this will default to output_token_limit specified in the `Model`
    /// specification.
    #[prost(int32, optional, tag = "4")]
    pub max_output_tokens: ::core::option::Option<i32>,
    /// Optional. Controls the randomness of the output.
    /// Note: The default value varies by model, see the `Model.temperature`
    /// attribute of the `Model` returned the `getModel` function.
    ///
    /// Values can range from \[0.0,1.0\],
    /// inclusive. A value closer to 1.0 will produce responses that are more
    /// varied and creative, while a value closer to 0.0 will typically result in
    /// more straightforward responses from the model.
    #[prost(float, optional, tag = "5")]
    pub temperature: ::core::option::Option<f32>,
    /// Optional. The maximum cumulative probability of tokens to consider when
    /// sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Tokens are sorted based on their assigned probabilities so that only the
    /// most likely tokens are considered. Top-k sampling directly limits the
    /// maximum number of tokens to consider, while Nucleus sampling limits number
    /// of tokens based on the cumulative probability.
    ///
    /// Note: The default value varies by model, see the `Model.top_p`
    /// attribute of the `Model` returned the `getModel` function.
    #[prost(float, optional, tag = "6")]
    pub top_p: ::core::option::Option<f32>,
    /// Optional. The maximum number of tokens to consider when sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Top-k sampling considers the set of `top_k` most probable tokens.
    /// Defaults to 40.
    ///
    /// Note: The default value varies by model, see the `Model.top_k`
    /// attribute of the `Model` returned the `getModel` function.
    #[prost(int32, optional, tag = "7")]
    pub top_k: ::core::option::Option<i32>,
}
/// Configuration for retrieving grounding content from a `Corpus` or
/// `Document` created using the Semantic Retriever API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SemanticRetrieverConfig {
    /// Required. Name of the resource for retrieval, e.g. corpora/123 or
    /// corpora/123/documents/abc.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// Required. Query to use for similarity matching `Chunk`s in the given
    /// resource.
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<Content>,
    /// Optional. Filters for selecting `Document`s and/or `Chunk`s from the
    /// resource.
    #[prost(message, repeated, tag = "3")]
    pub metadata_filters: ::prost::alloc::vec::Vec<MetadataFilter>,
    /// Optional. Maximum number of relevant `Chunk`s to retrieve.
    #[prost(int32, optional, tag = "4")]
    pub max_chunks_count: ::core::option::Option<i32>,
    /// Optional. Minimum relevance score for retrieved relevant `Chunk`s.
    #[prost(float, optional, tag = "5")]
    pub minimum_relevance_score: ::core::option::Option<f32>,
}
/// Response from the model supporting multiple candidates.
///
/// Note on safety ratings and content filtering. They are reported for both
/// prompt in `GenerateContentResponse.prompt_feedback` and for each candidate
/// in `finish_reason` and in `safety_ratings`. The API contract is that:
///   - either all requested candidates are returned or no candidates at all
///   - no candidates are returned only if there was something wrong with the
///     prompt (see `prompt_feedback`)
///   - feedback on each candidate is reported on `finish_reason` and
///     `safety_ratings`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateContentResponse {
    /// Candidate responses from the model.
    #[prost(message, repeated, tag = "1")]
    pub candidates: ::prost::alloc::vec::Vec<Candidate>,
    /// Returns the prompt's feedback related to the content filters.
    #[prost(message, optional, tag = "2")]
    pub prompt_feedback: ::core::option::Option<
        generate_content_response::PromptFeedback,
    >,
}
/// Nested message and enum types in `GenerateContentResponse`.
pub mod generate_content_response {
    /// A set of the feedback metadata the prompt specified in
    /// `GenerateContentRequest.content`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PromptFeedback {
        /// Optional. If set, the prompt was blocked and no candidates are returned.
        /// Rephrase your prompt.
        #[prost(enumeration = "prompt_feedback::BlockReason", tag = "1")]
        pub block_reason: i32,
        /// Ratings for safety of the prompt.
        /// There is at most one rating per category.
        #[prost(message, repeated, tag = "2")]
        pub safety_ratings: ::prost::alloc::vec::Vec<super::SafetyRating>,
    }
    /// Nested message and enum types in `PromptFeedback`.
    pub mod prompt_feedback {
        /// Specifies what was the reason why prompt was blocked.
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
        pub enum BlockReason {
            /// Default value. This value is unused.
            Unspecified = 0,
            /// Prompt was blocked due to safety reasons. You can inspect
            /// `safety_ratings` to understand which safety category blocked it.
            Safety = 1,
            /// Prompt was blocked due to unknown reaasons.
            Other = 2,
        }
        impl BlockReason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    BlockReason::Unspecified => "BLOCK_REASON_UNSPECIFIED",
                    BlockReason::Safety => "SAFETY",
                    BlockReason::Other => "OTHER",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "BLOCK_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                    "SAFETY" => Some(Self::Safety),
                    "OTHER" => Some(Self::Other),
                    _ => None,
                }
            }
        }
    }
}
/// A response candidate generated from the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candidate {
    /// Output only. Index of the candidate in the list of candidates.
    #[prost(int32, optional, tag = "3")]
    pub index: ::core::option::Option<i32>,
    /// Output only. Generated content returned from the model.
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Content>,
    /// Optional. Output only. The reason why the model stopped generating tokens.
    ///
    /// If empty, the model has not stopped generating the tokens.
    #[prost(enumeration = "candidate::FinishReason", tag = "2")]
    pub finish_reason: i32,
    /// List of ratings for the safety of a response candidate.
    ///
    /// There is at most one rating per category.
    #[prost(message, repeated, tag = "5")]
    pub safety_ratings: ::prost::alloc::vec::Vec<SafetyRating>,
    /// Output only. Citation information for model-generated candidate.
    ///
    /// This field may be populated with recitation information for any text
    /// included in the `content`. These are passages that are "recited" from
    /// copyrighted material in the foundational LLM's training data.
    #[prost(message, optional, tag = "6")]
    pub citation_metadata: ::core::option::Option<CitationMetadata>,
    /// Output only. Token count for this candidate.
    #[prost(int32, tag = "7")]
    pub token_count: i32,
    /// Output only. Attribution information for sources that contributed to a
    /// grounded answer.
    ///
    /// This field is populated for `GenerateAnswer` calls.
    #[prost(message, repeated, tag = "8")]
    pub grounding_attributions: ::prost::alloc::vec::Vec<GroundingAttribution>,
}
/// Nested message and enum types in `Candidate`.
pub mod candidate {
    /// Defines the reason why the model stopped generating tokens.
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
    pub enum FinishReason {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Natural stop point of the model or provided stop sequence.
        Stop = 1,
        /// The maximum number of tokens as specified in the request was reached.
        MaxTokens = 2,
        /// The candidate content was flagged for safety reasons.
        Safety = 3,
        /// The candidate content was flagged for recitation reasons.
        Recitation = 4,
        /// Unknown reason.
        Other = 5,
    }
    impl FinishReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FinishReason::Unspecified => "FINISH_REASON_UNSPECIFIED",
                FinishReason::Stop => "STOP",
                FinishReason::MaxTokens => "MAX_TOKENS",
                FinishReason::Safety => "SAFETY",
                FinishReason::Recitation => "RECITATION",
                FinishReason::Other => "OTHER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FINISH_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "STOP" => Some(Self::Stop),
                "MAX_TOKENS" => Some(Self::MaxTokens),
                "SAFETY" => Some(Self::Safety),
                "RECITATION" => Some(Self::Recitation),
                "OTHER" => Some(Self::Other),
                _ => None,
            }
        }
    }
}
/// Identifier for the source contributing to this attribution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributionSourceId {
    #[prost(oneof = "attribution_source_id::Source", tags = "1, 2")]
    pub source: ::core::option::Option<attribution_source_id::Source>,
}
/// Nested message and enum types in `AttributionSourceId`.
pub mod attribution_source_id {
    /// Identifier for a part within a `GroundingPassage`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroundingPassageId {
        /// Output only. ID of the passage matching the `GenerateAnswerRequest`'s
        /// `GroundingPassage.id`.
        #[prost(string, tag = "1")]
        pub passage_id: ::prost::alloc::string::String,
        /// Output only. Index of the part within the `GenerateAnswerRequest`'s
        /// `GroundingPassage.content`.
        #[prost(int32, tag = "2")]
        pub part_index: i32,
    }
    /// Identifier for a `Chunk` retrieved via Semantic Retriever specified in the
    /// `GenerateAnswerRequest` using `SemanticRetrieverConfig`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SemanticRetrieverChunk {
        /// Output only. Name of the source matching the request's
        /// `SemanticRetrieverConfig.source`. Example: `corpora/123` or
        /// `corpora/123/documents/abc`
        #[prost(string, tag = "1")]
        pub source: ::prost::alloc::string::String,
        /// Output only. Name of the `Chunk` containing the attributed text.
        /// Example: `corpora/123/documents/abc/chunks/xyz`
        #[prost(string, tag = "2")]
        pub chunk: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Identifier for an inline passage.
        #[prost(message, tag = "1")]
        GroundingPassage(GroundingPassageId),
        /// Identifier for a `Chunk` fetched via Semantic Retriever.
        #[prost(message, tag = "2")]
        SemanticRetrieverChunk(SemanticRetrieverChunk),
    }
}
/// Attribution for a source that contributed to an answer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroundingAttribution {
    /// Output only. Identifier for the source contributing to this attribution.
    #[prost(message, optional, tag = "3")]
    pub source_id: ::core::option::Option<AttributionSourceId>,
    /// Grounding source content that makes up this attribution.
    #[prost(message, optional, tag = "2")]
    pub content: ::core::option::Option<Content>,
}
/// Request to generate a grounded answer from the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAnswerRequest {
    /// Required. The name of the `Model` to use for generating the grounded
    /// response.
    ///
    /// Format: `model=models/{model}`.
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The content of the current conversation with the model. For
    /// single-turn queries, this is a single question to answer. For multi-turn
    /// queries, this is a repeated field that contains conversation history and
    /// the last `Content` in the list containing the question.
    #[prost(message, repeated, tag = "2")]
    pub contents: ::prost::alloc::vec::Vec<Content>,
    /// Required. Style in which answers should be returned.
    #[prost(enumeration = "generate_answer_request::AnswerStyle", tag = "5")]
    pub answer_style: i32,
    /// Optional. A list of unique `SafetySetting` instances for blocking unsafe
    /// content.
    ///
    /// This will be enforced on the `GenerateAnswerRequest.contents` and
    /// `GenerateAnswerResponse.candidate`. There should not be more than one
    /// setting for each `SafetyCategory` type. The API will block any contents and
    /// responses that fail to meet the thresholds set by these settings. This list
    /// overrides the default settings for each `SafetyCategory` specified in the
    /// safety_settings. If there is no `SafetySetting` for a given
    /// `SafetyCategory` provided in the list, the API will use the default safety
    /// setting for that category.
    #[prost(message, repeated, tag = "3")]
    pub safety_settings: ::prost::alloc::vec::Vec<SafetySetting>,
    /// Optional. Controls the randomness of the output.
    ///
    /// Values can range from \[0.0,1.0\], inclusive. A value closer to 1.0 will
    /// produce responses that are more varied and creative, while a value closer
    /// to 0.0 will typically result in more straightforward responses from the
    /// model. A low temperature (~0.2) is usually recommended for
    /// Attributed-Question-Answering use cases.
    #[prost(float, optional, tag = "4")]
    pub temperature: ::core::option::Option<f32>,
    /// The sources in which to ground the answer.
    #[prost(oneof = "generate_answer_request::GroundingSource", tags = "6, 7")]
    pub grounding_source: ::core::option::Option<
        generate_answer_request::GroundingSource,
    >,
}
/// Nested message and enum types in `GenerateAnswerRequest`.
pub mod generate_answer_request {
    /// Style for grounded answers.
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
    pub enum AnswerStyle {
        /// Unspecified answer style.
        Unspecified = 0,
        /// Succint but abstract style.
        Abstractive = 1,
        /// Very brief and extractive style.
        Extractive = 2,
        /// Verbose style including extra details. The response may be formatted as a
        /// sentence, paragraph, multiple paragraphs, or bullet points, etc.
        Verbose = 3,
    }
    impl AnswerStyle {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AnswerStyle::Unspecified => "ANSWER_STYLE_UNSPECIFIED",
                AnswerStyle::Abstractive => "ABSTRACTIVE",
                AnswerStyle::Extractive => "EXTRACTIVE",
                AnswerStyle::Verbose => "VERBOSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ANSWER_STYLE_UNSPECIFIED" => Some(Self::Unspecified),
                "ABSTRACTIVE" => Some(Self::Abstractive),
                "EXTRACTIVE" => Some(Self::Extractive),
                "VERBOSE" => Some(Self::Verbose),
                _ => None,
            }
        }
    }
    /// The sources in which to ground the answer.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GroundingSource {
        /// Passages provided inline with the request.
        #[prost(message, tag = "6")]
        InlinePassages(super::GroundingPassages),
        /// Content retrieved from resources created via the Semantic Retriever
        /// API.
        #[prost(message, tag = "7")]
        SemanticRetriever(super::SemanticRetrieverConfig),
    }
}
/// Response from the model for a grounded answer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAnswerResponse {
    /// Candidate answer from the model.
    ///
    /// Note: The model *always* attempts to provide a grounded answer, even when
    /// the answer is unlikely to be answerable from the given passages.
    /// In that case, a low-quality or ungrounded answer may be provided, along
    /// with a low `answerable_probability`.
    #[prost(message, optional, tag = "1")]
    pub answer: ::core::option::Option<Candidate>,
    /// Output only. The model's estimate of the probability that its answer is
    /// correct and grounded in the input passages.
    ///
    /// A low answerable_probability indicates that the answer might not be
    /// grounded in the sources.
    ///
    /// When `answerable_probability` is low, some clients may wish to:
    ///
    /// * Display a message to the effect of "We couldn’t answer that question" to
    /// the user.
    /// * Fall back to a general-purpose LLM that answers the question from world
    /// knowledge. The threshold and nature of such fallbacks will depend on
    /// individual clients’ use cases. 0.5 is a good starting threshold.
    #[prost(float, optional, tag = "2")]
    pub answerable_probability: ::core::option::Option<f32>,
}
/// Request containing the `Content` for the model to embed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedContentRequest {
    /// Required. The model's resource name. This serves as an ID for the Model to
    /// use.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The content to embed. Only the `parts.text` fields will be
    /// counted.
    #[prost(message, optional, tag = "2")]
    pub content: ::core::option::Option<Content>,
    /// Optional. Optional task type for which the embeddings will be used. Can
    /// only be set for `models/embedding-001`.
    #[prost(enumeration = "TaskType", optional, tag = "3")]
    pub task_type: ::core::option::Option<i32>,
    /// Optional. An optional title for the text. Only applicable when TaskType is
    /// `RETRIEVAL_DOCUMENT`.
    #[prost(string, optional, tag = "4")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
}
/// A list of floats representing an embedding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentEmbedding {
    /// The embedding values.
    #[prost(float, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<f32>,
}
/// The response to an `EmbedContentRequest`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedContentResponse {
    /// Output only. The embedding generated from the input content.
    #[prost(message, optional, tag = "1")]
    pub embedding: ::core::option::Option<ContentEmbedding>,
}
/// Batch request to get embeddings from the model for a list of prompts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchEmbedContentsRequest {
    /// Required. The model's resource name. This serves as an ID for the Model to
    /// use.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. Embed requests for the batch. The model in each of these requests
    /// must match the model specified `BatchEmbedContentsRequest.model`.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<EmbedContentRequest>,
}
/// The response to a `BatchEmbedContentsRequest`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchEmbedContentsResponse {
    /// Output only. The embeddings for each request, in the same order as provided
    /// in the batch request.
    #[prost(message, repeated, tag = "1")]
    pub embeddings: ::prost::alloc::vec::Vec<ContentEmbedding>,
}
/// Counts the number of tokens in the `prompt` sent to a model.
///
/// Models may tokenize text differently, so each model may return a different
/// `token_count`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountTokensRequest {
    /// Required. The model's resource name. This serves as an ID for the Model to
    /// use.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The input given to the model as a prompt.
    #[prost(message, repeated, tag = "2")]
    pub contents: ::prost::alloc::vec::Vec<Content>,
}
/// A response from `CountTokens`.
///
/// It returns the model's `token_count` for the `prompt`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountTokensResponse {
    /// The number of tokens that the `model` tokenizes the `prompt` into.
    ///
    /// Always non-negative.
    #[prost(int32, tag = "1")]
    pub total_tokens: i32,
}
/// Type of task for which the embedding will be used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskType {
    /// Unset value, which will default to one of the other enum values.
    Unspecified = 0,
    /// Specifies the given text is a query in a search/retrieval setting.
    RetrievalQuery = 1,
    /// Specifies the given text is a document from the corpus being searched.
    RetrievalDocument = 2,
    /// Specifies the given text will be used for STS.
    SemanticSimilarity = 3,
    /// Specifies that the given text will be classified.
    Classification = 4,
    /// Specifies that the embeddings will be used for clustering.
    Clustering = 5,
}
impl TaskType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TaskType::Unspecified => "TASK_TYPE_UNSPECIFIED",
            TaskType::RetrievalQuery => "RETRIEVAL_QUERY",
            TaskType::RetrievalDocument => "RETRIEVAL_DOCUMENT",
            TaskType::SemanticSimilarity => "SEMANTIC_SIMILARITY",
            TaskType::Classification => "CLASSIFICATION",
            TaskType::Clustering => "CLUSTERING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TASK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "RETRIEVAL_QUERY" => Some(Self::RetrievalQuery),
            "RETRIEVAL_DOCUMENT" => Some(Self::RetrievalDocument),
            "SEMANTIC_SIMILARITY" => Some(Self::SemanticSimilarity),
            "CLASSIFICATION" => Some(Self::Classification),
            "CLUSTERING" => Some(Self::Clustering),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod generative_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// API for using Large Models that generate multimodal content and have
    /// additional capabilities beyond text generation.
    #[derive(Debug, Clone)]
    pub struct GenerativeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GenerativeServiceClient<T>
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
        ) -> GenerativeServiceClient<InterceptedService<T, F>>
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
            GenerativeServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Generates a response from the model given an input
        /// `GenerateContentRequest`.
        pub async fn generate_content(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateContentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateContentResponse>,
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
                "/google.ai.generativelanguage.v1beta.GenerativeService/GenerateContent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.GenerativeService",
                        "GenerateContent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates a grounded answer from the model given an input
        /// `GenerateAnswerRequest`.
        pub async fn generate_answer(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateAnswerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateAnswerResponse>,
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
                "/google.ai.generativelanguage.v1beta.GenerativeService/GenerateAnswer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.GenerativeService",
                        "GenerateAnswer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates a streamed response from the model given an input
        /// `GenerateContentRequest`.
        pub async fn stream_generate_content(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateContentRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::GenerateContentResponse>>,
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
                "/google.ai.generativelanguage.v1beta.GenerativeService/StreamGenerateContent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.GenerativeService",
                        "StreamGenerateContent",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Generates an embedding from the model given an input `Content`.
        pub async fn embed_content(
            &mut self,
            request: impl tonic::IntoRequest<super::EmbedContentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EmbedContentResponse>,
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
                "/google.ai.generativelanguage.v1beta.GenerativeService/EmbedContent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.GenerativeService",
                        "EmbedContent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates multiple embeddings from the model given input text in a
        /// synchronous call.
        pub async fn batch_embed_contents(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchEmbedContentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchEmbedContentsResponse>,
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
                "/google.ai.generativelanguage.v1beta.GenerativeService/BatchEmbedContents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.GenerativeService",
                        "BatchEmbedContents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Runs a model's tokenizer on input content and returns the token count.
        pub async fn count_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::CountTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CountTokensResponse>,
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
                "/google.ai.generativelanguage.v1beta.GenerativeService/CountTokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.GenerativeService",
                        "CountTokens",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Information about a Generative Language Model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// Required. The resource name of the `Model`.
    ///
    /// Format: `models/{model}` with a `{model}` naming convention of:
    ///
    /// * "{base_model_id}-{version}"
    ///
    /// Examples:
    ///
    /// * `models/chat-bison-001`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the base model, pass this to the generation request.
    ///
    /// Examples:
    ///
    /// * `chat-bison`
    #[prost(string, tag = "2")]
    pub base_model_id: ::prost::alloc::string::String,
    /// Required. The version number of the model.
    ///
    /// This represents the major version
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// The human-readable name of the model. E.g. "Chat Bison".
    ///
    /// The name can be up to 128 characters long and can consist of any UTF-8
    /// characters.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// A short description of the model.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Maximum number of input tokens allowed for this model.
    #[prost(int32, tag = "6")]
    pub input_token_limit: i32,
    /// Maximum number of output tokens available for this model.
    #[prost(int32, tag = "7")]
    pub output_token_limit: i32,
    /// The model's supported generation methods.
    ///
    /// The method names are defined as Pascal case
    /// strings, such as `generateMessage` which correspond to API methods.
    #[prost(string, repeated, tag = "8")]
    pub supported_generation_methods: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Controls the randomness of the output.
    ///
    /// Values can range over `\[0.0,1.0\]`, inclusive. A value closer to `1.0` will
    /// produce responses that are more varied, while a value closer to `0.0` will
    /// typically result in less surprising responses from the model.
    /// This value specifies default to be used by the backend while making the
    /// call to the model.
    #[prost(float, optional, tag = "9")]
    pub temperature: ::core::option::Option<f32>,
    /// For Nucleus sampling.
    ///
    /// Nucleus sampling considers the smallest set of tokens whose probability
    /// sum is at least `top_p`.
    /// This value specifies default to be used by the backend while making the
    /// call to the model.
    #[prost(float, optional, tag = "10")]
    pub top_p: ::core::option::Option<f32>,
    /// For Top-k sampling.
    ///
    /// Top-k sampling considers the set of `top_k` most probable tokens.
    /// This value specifies default to be used by the backend while making the
    /// call to the model.
    #[prost(int32, optional, tag = "11")]
    pub top_k: ::core::option::Option<i32>,
}
/// Request to generate a message response from the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateMessageRequest {
    /// Required. The name of the model to use.
    ///
    /// Format: `name=models/{model}`.
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The structured textual input given to the model as a prompt.
    ///
    /// Given a
    /// prompt, the model will return what it predicts is the next message in the
    /// discussion.
    #[prost(message, optional, tag = "2")]
    pub prompt: ::core::option::Option<MessagePrompt>,
    /// Optional. Controls the randomness of the output.
    ///
    /// Values can range over `\[0.0,1.0\]`,
    /// inclusive. A value closer to `1.0` will produce responses that are more
    /// varied, while a value closer to `0.0` will typically result in
    /// less surprising responses from the model.
    #[prost(float, optional, tag = "3")]
    pub temperature: ::core::option::Option<f32>,
    /// Optional. The number of generated response messages to return.
    ///
    /// This value must be between
    /// `\[1, 8\]`, inclusive. If unset, this will default to `1`.
    #[prost(int32, optional, tag = "4")]
    pub candidate_count: ::core::option::Option<i32>,
    /// Optional. The maximum cumulative probability of tokens to consider when
    /// sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Nucleus sampling considers the smallest set of tokens whose probability
    /// sum is at least `top_p`.
    #[prost(float, optional, tag = "5")]
    pub top_p: ::core::option::Option<f32>,
    /// Optional. The maximum number of tokens to consider when sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Top-k sampling considers the set of `top_k` most probable tokens.
    #[prost(int32, optional, tag = "6")]
    pub top_k: ::core::option::Option<i32>,
}
/// The response from the model.
///
/// This includes candidate messages and
/// conversation history in the form of chronologically-ordered messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateMessageResponse {
    /// Candidate response messages from the model.
    #[prost(message, repeated, tag = "1")]
    pub candidates: ::prost::alloc::vec::Vec<Message>,
    /// The conversation history used by the model.
    #[prost(message, repeated, tag = "2")]
    pub messages: ::prost::alloc::vec::Vec<Message>,
    /// A set of content filtering metadata for the prompt and response
    /// text.
    ///
    /// This indicates which `SafetyCategory`(s) blocked a
    /// candidate from this response, the lowest `HarmProbability`
    /// that triggered a block, and the HarmThreshold setting for that category.
    #[prost(message, repeated, tag = "3")]
    pub filters: ::prost::alloc::vec::Vec<ContentFilter>,
}
/// The base unit of structured text.
///
/// A `Message` includes an `author` and the `content` of
/// the `Message`.
///
/// The `author` is used to tag messages when they are fed to the
/// model as text.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// Optional. The author of this Message.
    ///
    /// This serves as a key for tagging
    /// the content of this Message when it is fed to the model as text.
    ///
    /// The author can be any alphanumeric string.
    #[prost(string, tag = "1")]
    pub author: ::prost::alloc::string::String,
    /// Required. The text content of the structured `Message`.
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    /// Output only. Citation information for model-generated `content` in this
    /// `Message`.
    ///
    /// If this `Message` was generated as output from the model, this field may be
    /// populated with attribution information for any text included in the
    /// `content`. This field is used only on output.
    #[prost(message, optional, tag = "3")]
    pub citation_metadata: ::core::option::Option<CitationMetadata>,
}
/// All of the structured input text passed to the model as a prompt.
///
/// A `MessagePrompt` contains a structured set of fields that provide context
/// for the conversation, examples of user input/model output message pairs that
/// prime the model to respond in different ways, and the conversation history
/// or list of messages representing the alternating turns of the conversation
/// between the user and the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePrompt {
    /// Optional. Text that should be provided to the model first to ground the
    /// response.
    ///
    /// If not empty, this `context` will be given to the model first before the
    /// `examples` and `messages`. When using a `context` be sure to provide it
    /// with every request to maintain continuity.
    ///
    /// This field can be a description of your prompt to the model to help provide
    /// context and guide the responses. Examples: "Translate the phrase from
    /// English to French." or "Given a statement, classify the sentiment as happy,
    /// sad or neutral."
    ///
    /// Anything included in this field will take precedence over message history
    /// if the total input size exceeds the model's `input_token_limit` and the
    /// input request is truncated.
    #[prost(string, tag = "1")]
    pub context: ::prost::alloc::string::String,
    /// Optional. Examples of what the model should generate.
    ///
    /// This includes both user input and the response that the model should
    /// emulate.
    ///
    /// These `examples` are treated identically to conversation messages except
    /// that they take precedence over the history in `messages`:
    /// If the total input size exceeds the model's `input_token_limit` the input
    /// will be truncated. Items will be dropped from `messages` before `examples`.
    #[prost(message, repeated, tag = "2")]
    pub examples: ::prost::alloc::vec::Vec<Example>,
    /// Required. A snapshot of the recent conversation history sorted
    /// chronologically.
    ///
    /// Turns alternate between two authors.
    ///
    /// If the total input size exceeds the model's `input_token_limit` the input
    /// will be truncated: The oldest items will be dropped from `messages`.
    #[prost(message, repeated, tag = "3")]
    pub messages: ::prost::alloc::vec::Vec<Message>,
}
/// An input/output example used to instruct the Model.
///
/// It demonstrates how the model should respond or format its response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Example {
    /// Required. An example of an input `Message` from the user.
    #[prost(message, optional, tag = "1")]
    pub input: ::core::option::Option<Message>,
    /// Required. An example of what the model should output given the input.
    #[prost(message, optional, tag = "2")]
    pub output: ::core::option::Option<Message>,
}
/// Counts the number of tokens in the `prompt` sent to a model.
///
/// Models may tokenize text differently, so each model may return a different
/// `token_count`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountMessageTokensRequest {
    /// Required. The model's resource name. This serves as an ID for the Model to
    /// use.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The prompt, whose token count is to be returned.
    #[prost(message, optional, tag = "2")]
    pub prompt: ::core::option::Option<MessagePrompt>,
}
/// A response from `CountMessageTokens`.
///
/// It returns the model's `token_count` for the `prompt`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountMessageTokensResponse {
    /// The number of tokens that the `model` tokenizes the `prompt` into.
    ///
    /// Always non-negative.
    #[prost(int32, tag = "1")]
    pub token_count: i32,
}
/// Generated client implementations.
pub mod discuss_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// An API for using Generative Language Models (GLMs) in dialog applications.
    ///
    /// Also known as large language models (LLMs), this API provides models that
    /// are trained for multi-turn dialog.
    #[derive(Debug, Clone)]
    pub struct DiscussServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DiscussServiceClient<T>
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
        ) -> DiscussServiceClient<InterceptedService<T, F>>
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
            DiscussServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Generates a response from the model given an input `MessagePrompt`.
        pub async fn generate_message(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateMessageResponse>,
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
                "/google.ai.generativelanguage.v1beta.DiscussService/GenerateMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.DiscussService",
                        "GenerateMessage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Runs a model's tokenizer on a string and returns the token count.
        pub async fn count_message_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::CountMessageTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CountMessageTokensResponse>,
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
                "/google.ai.generativelanguage.v1beta.DiscussService/CountMessageTokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.DiscussService",
                        "CountMessageTokens",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Permission resource grants user, group or the rest of the world access to the
/// PaLM API resource (e.g. a tuned model, corpus).
///
/// A role is a collection of permitted operations that allows users to perform
/// specific actions on PaLM API resources. To make them available to users,
/// groups, or service accounts, you assign roles. When you assign a role, you
/// grant permissions that the role contains.
///
/// There are three concentric roles. Each role is a superset of the previous
/// role's permitted operations:
///
/// - reader can use the resource (e.g. tuned model, corpus) for inference
/// - writer has reader's permissions and additionally can edit and share
/// - owner has writer's permissions and additionally can delete
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permission {
    /// Output only. Identifier. The permission name. A unique name will be
    /// generated on create. Examples:
    ///      tunedModels/{tuned_model}/permissions/{permission}
    ///      corpora/{corpus}/permissions/{permission}
    /// Output only.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Immutable. The type of the grantee.
    #[prost(enumeration = "permission::GranteeType", optional, tag = "2")]
    pub grantee_type: ::core::option::Option<i32>,
    /// Optional. Immutable. The email address of the user of group which this
    /// permission refers. Field is not set when permission's grantee type is
    /// EVERYONE.
    #[prost(string, optional, tag = "3")]
    pub email_address: ::core::option::Option<::prost::alloc::string::String>,
    /// Required. The role granted by this permission.
    #[prost(enumeration = "permission::Role", optional, tag = "4")]
    pub role: ::core::option::Option<i32>,
}
/// Nested message and enum types in `Permission`.
pub mod permission {
    /// Defines types of the grantee of this permission.
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
    pub enum GranteeType {
        /// The default value. This value is unused.
        Unspecified = 0,
        /// Represents a user. When set, you must provide email_address for the user.
        User = 1,
        /// Represents a group. When set, you must provide email_address for the
        /// group.
        Group = 2,
        /// Represents access to everyone. No extra information is required.
        Everyone = 3,
    }
    impl GranteeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GranteeType::Unspecified => "GRANTEE_TYPE_UNSPECIFIED",
                GranteeType::User => "USER",
                GranteeType::Group => "GROUP",
                GranteeType::Everyone => "EVERYONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GRANTEE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "USER" => Some(Self::User),
                "GROUP" => Some(Self::Group),
                "EVERYONE" => Some(Self::Everyone),
                _ => None,
            }
        }
    }
    /// Defines the role granted by this permission.
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
    pub enum Role {
        /// The default value. This value is unused.
        Unspecified = 0,
        /// Owner can use, update, share and delete the resource.
        Owner = 1,
        /// Writer can use, update and share the resource.
        Writer = 2,
        /// Reader can use the resource.
        Reader = 3,
    }
    impl Role {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Role::Unspecified => "ROLE_UNSPECIFIED",
                Role::Owner => "OWNER",
                Role::Writer => "WRITER",
                Role::Reader => "READER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROLE_UNSPECIFIED" => Some(Self::Unspecified),
                "OWNER" => Some(Self::Owner),
                "WRITER" => Some(Self::Writer),
                "READER" => Some(Self::Reader),
                _ => None,
            }
        }
    }
}
/// Request to create a `Permission`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePermissionRequest {
    /// Required. The parent resource of the `Permission`.
    /// Formats:
    ///     `tunedModels/{tuned_model}`
    ///     `corpora/{corpus}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The permission to create.
    #[prost(message, optional, tag = "2")]
    pub permission: ::core::option::Option<Permission>,
}
/// Request for getting information about a specific `Permission`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPermissionRequest {
    /// Required. The resource name of the permission.
    ///
    /// Formats:
    ///     `tunedModels/{tuned_model}/permissions/{permission}`
    ///     `corpora/{corpus}/permissions/{permission}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for listing permissions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPermissionsRequest {
    /// Required. The parent resource of the permissions.
    /// Formats:
    ///     `tunedModels/{tuned_model}`
    ///     `corpora/{corpus}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `Permission`s to return (per page).
    /// The service may return fewer permissions.
    ///
    /// If unspecified, at most 10 permissions will be returned.
    /// This method returns at most 1000 permissions per page, even if you pass
    /// larger page_size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListPermissions` call.
    ///
    /// Provide the `page_token` returned by one request as an argument to the
    /// next request to retrieve the next page.
    ///
    /// When paginating, all other parameters provided to `ListPermissions`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response from `ListPermissions` containing a paginated list of
/// permissions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPermissionsResponse {
    /// Returned permissions.
    #[prost(message, repeated, tag = "1")]
    pub permissions: ::prost::alloc::vec::Vec<Permission>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    ///
    /// If this field is omitted, there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to update the `Permission`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePermissionRequest {
    /// Required. The permission to update.
    ///
    /// The permission's `name` field is used to identify the permission to update.
    #[prost(message, optional, tag = "1")]
    pub permission: ::core::option::Option<Permission>,
    /// Required. The list of fields to update. Accepted ones:
    ///   - role (`Permission.role` field)
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to delete the `Permission`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePermissionRequest {
    /// Required. The resource name of the permission.
    /// Formats:
    ///     `tunedModels/{tuned_model}/permissions/{permission}`
    ///     `corpora/{corpus}/permissions/{permission}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to transfer the ownership of the tuned model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferOwnershipRequest {
    /// Required. The resource name of the tuned model to transfer ownership.
    ///
    /// Format: `tunedModels/my-model-id`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The email address of the user to whom the tuned model is being
    /// transferred to.
    #[prost(string, tag = "2")]
    pub email_address: ::prost::alloc::string::String,
}
/// Response from `TransferOwnership`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferOwnershipResponse {}
/// Generated client implementations.
pub mod permission_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for managing permissions to PaLM API resources.
    #[derive(Debug, Clone)]
    pub struct PermissionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PermissionServiceClient<T>
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
        ) -> PermissionServiceClient<InterceptedService<T, F>>
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
            PermissionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Create a permission to a specific resource.
        pub async fn create_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePermissionRequest>,
        ) -> std::result::Result<tonic::Response<super::Permission>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1beta.PermissionService/CreatePermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.PermissionService",
                        "CreatePermission",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets information about a specific Permission.
        pub async fn get_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPermissionRequest>,
        ) -> std::result::Result<tonic::Response<super::Permission>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1beta.PermissionService/GetPermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.PermissionService",
                        "GetPermission",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists permissions for the specific resource.
        pub async fn list_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPermissionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPermissionsResponse>,
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
                "/google.ai.generativelanguage.v1beta.PermissionService/ListPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.PermissionService",
                        "ListPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the permission.
        pub async fn update_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePermissionRequest>,
        ) -> std::result::Result<tonic::Response<super::Permission>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1beta.PermissionService/UpdatePermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.PermissionService",
                        "UpdatePermission",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the permission.
        pub async fn delete_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePermissionRequest>,
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
                "/google.ai.generativelanguage.v1beta.PermissionService/DeletePermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.PermissionService",
                        "DeletePermission",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Transfers ownership of the tuned model.
        /// This is the only way to change ownership of the tuned model.
        /// The current owner will be downgraded to writer role.
        pub async fn transfer_ownership(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferOwnershipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransferOwnershipResponse>,
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
                "/google.ai.generativelanguage.v1beta.PermissionService/TransferOwnership",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.PermissionService",
                        "TransferOwnership",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// A fine-tuned model created using ModelService.CreateTunedModel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TunedModel {
    /// Output only. The tuned model name. A unique name will be generated on
    /// create. Example: `tunedModels/az2mb0bpw6i` If display_name is set on
    /// create, the id portion of the name will be set by concatenating the words
    /// of the display_name with hyphens and adding a random portion for
    /// uniqueness. Example:
    ///      display_name = "Sentence Translator"
    ///      name = "tunedModels/sentence-translator-u3b7m"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The name to display for this model in user interfaces.
    /// The display name must be up to 40 characters including spaces.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. A short description of this model.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Controls the randomness of the output.
    ///
    /// Values can range over `\[0.0,1.0\]`, inclusive. A value closer to `1.0` will
    /// produce responses that are more varied, while a value closer to `0.0` will
    /// typically result in less surprising responses from the model.
    ///
    /// This value specifies default to be the one used by the base model while
    /// creating the model.
    #[prost(float, optional, tag = "11")]
    pub temperature: ::core::option::Option<f32>,
    /// Optional. For Nucleus sampling.
    ///
    /// Nucleus sampling considers the smallest set of tokens whose probability
    /// sum is at least `top_p`.
    ///
    /// This value specifies default to be the one used by the base model while
    /// creating the model.
    #[prost(float, optional, tag = "12")]
    pub top_p: ::core::option::Option<f32>,
    /// Optional. For Top-k sampling.
    ///
    /// Top-k sampling considers the set of `top_k` most probable tokens.
    /// This value specifies default to be used by the backend while making the
    /// call to the model.
    ///
    /// This value specifies default to be the one used by the base model while
    /// creating the model.
    #[prost(int32, optional, tag = "13")]
    pub top_k: ::core::option::Option<i32>,
    /// Output only. The state of the tuned model.
    #[prost(enumeration = "tuned_model::State", tag = "7")]
    pub state: i32,
    /// Output only. The timestamp when this model was created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when this model was updated.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The tuning task that creates the tuned model.
    #[prost(message, optional, tag = "10")]
    pub tuning_task: ::core::option::Option<TuningTask>,
    /// The model used as the starting point for tuning.
    #[prost(oneof = "tuned_model::SourceModel", tags = "3, 4")]
    pub source_model: ::core::option::Option<tuned_model::SourceModel>,
}
/// Nested message and enum types in `TunedModel`.
pub mod tuned_model {
    /// The state of the tuned model.
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
        /// The default value. This value is unused.
        Unspecified = 0,
        /// The model is being created.
        Creating = 1,
        /// The model is ready to be used.
        Active = 2,
        /// The model failed to be created.
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
                State::Creating => "CREATING",
                State::Active => "ACTIVE",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// The model used as the starting point for tuning.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceModel {
        /// Optional. TunedModel to use as the starting point for training the new
        /// model.
        #[prost(message, tag = "3")]
        TunedModelSource(super::TunedModelSource),
        /// Immutable. The name of the `Model` to tune.
        /// Example: `models/text-bison-001`
        #[prost(string, tag = "4")]
        BaseModel(::prost::alloc::string::String),
    }
}
/// Tuned model as a source for training a new model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TunedModelSource {
    /// Immutable. The name of the `TunedModel` to use as the starting point for
    /// training the new model.
    /// Example: `tunedModels/my-tuned-model`
    #[prost(string, tag = "1")]
    pub tuned_model: ::prost::alloc::string::String,
    /// Output only. The name of the base `Model` this `TunedModel` was tuned from.
    /// Example: `models/text-bison-001`
    #[prost(string, tag = "2")]
    pub base_model: ::prost::alloc::string::String,
}
/// Tuning tasks that create tuned models.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TuningTask {
    /// Output only. The timestamp when tuning this model started.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when tuning this model completed.
    #[prost(message, optional, tag = "2")]
    pub complete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Metrics collected during tuning.
    #[prost(message, repeated, tag = "3")]
    pub snapshots: ::prost::alloc::vec::Vec<TuningSnapshot>,
    /// Required. Input only. Immutable. The model training data.
    #[prost(message, optional, tag = "4")]
    pub training_data: ::core::option::Option<Dataset>,
    /// Immutable. Hyperparameters controlling the tuning process. If not provided,
    /// default values will be used.
    #[prost(message, optional, tag = "5")]
    pub hyperparameters: ::core::option::Option<Hyperparameters>,
}
/// Hyperparameters controlling the tuning process.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hyperparameters {
    /// Immutable. The number of training epochs. An epoch is one pass through the
    /// training data. If not set, a default of 10 will be used.
    #[prost(int32, optional, tag = "14")]
    pub epoch_count: ::core::option::Option<i32>,
    /// Immutable. The batch size hyperparameter for tuning.
    /// If not set, a default of 16 or 64 will be used based on the number of
    /// training examples.
    #[prost(int32, optional, tag = "15")]
    pub batch_size: ::core::option::Option<i32>,
    /// Immutable. The learning rate hyperparameter for tuning.
    /// If not set, a default of 0.0002 or 0.002 will be calculated based on the
    /// number of training examples.
    #[prost(float, optional, tag = "16")]
    pub learning_rate: ::core::option::Option<f32>,
}
/// Dataset for training or validation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataset {
    /// Inline data or a reference to the data.
    #[prost(oneof = "dataset::Dataset", tags = "1")]
    pub dataset: ::core::option::Option<dataset::Dataset>,
}
/// Nested message and enum types in `Dataset`.
pub mod dataset {
    /// Inline data or a reference to the data.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Dataset {
        /// Optional. Inline examples.
        #[prost(message, tag = "1")]
        Examples(super::TuningExamples),
    }
}
/// A set of tuning examples. Can be training or validatation data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TuningExamples {
    /// Required. The examples. Example input can be for text or discuss, but all
    /// examples in a set must be of the same type.
    #[prost(message, repeated, tag = "1")]
    pub examples: ::prost::alloc::vec::Vec<TuningExample>,
}
/// A single example for tuning.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TuningExample {
    /// Required. The expected model output.
    #[prost(string, tag = "3")]
    pub output: ::prost::alloc::string::String,
    /// The input to the model for this example.
    #[prost(oneof = "tuning_example::ModelInput", tags = "1")]
    pub model_input: ::core::option::Option<tuning_example::ModelInput>,
}
/// Nested message and enum types in `TuningExample`.
pub mod tuning_example {
    /// The input to the model for this example.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ModelInput {
        /// Optional. Text model input.
        #[prost(string, tag = "1")]
        TextInput(::prost::alloc::string::String),
    }
}
/// Record for a single tuning step.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TuningSnapshot {
    /// Output only. The tuning step.
    #[prost(int32, tag = "1")]
    pub step: i32,
    /// Output only. The epoch this step was part of.
    #[prost(int32, tag = "2")]
    pub epoch: i32,
    /// Output only. The mean loss of the training examples for this step.
    #[prost(float, tag = "3")]
    pub mean_loss: f32,
    /// Output only. The timestamp when this metric was computed.
    #[prost(message, optional, tag = "4")]
    pub compute_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request for getting information about a specific Model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelRequest {
    /// Required. The resource name of the model.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for listing all Models.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsRequest {
    /// The maximum number of `Models` to return (per page).
    ///
    /// The service may return fewer models.
    /// If unspecified, at most 50 models will be returned per page.
    /// This method returns at most 1000 models per page, even if you pass a larger
    /// page_size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListModels` call.
    ///
    /// Provide the `page_token` returned by one request as an argument to the next
    /// request to retrieve the next page.
    ///
    /// When paginating, all other parameters provided to `ListModels` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response from `ListModel` containing a paginated list of Models.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsResponse {
    /// The returned Models.
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    ///
    /// If this field is omitted, there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for getting information about a specific Model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTunedModelRequest {
    /// Required. The resource name of the model.
    ///
    /// Format: `tunedModels/my-model-id`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for listing TunedModels.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTunedModelsRequest {
    /// Optional. The maximum number of `TunedModels` to return (per page).
    /// The service may return fewer tuned models.
    ///
    /// If unspecified, at most 10 tuned models will be returned.
    /// This method returns at most 1000 models per page, even if you pass a larger
    /// page_size.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListTunedModels` call.
    ///
    /// Provide the `page_token` returned by one request as an argument to the next
    /// request to retrieve the next page.
    ///
    /// When paginating, all other parameters provided to `ListTunedModels`
    /// must match the call that provided the page token.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A filter is a full text search over the tuned model's description
    /// and display name. By default, results will not include tuned models shared
    /// with everyone.
    ///
    /// Additional operators:
    ///    - owner:me
    ///    - writers:me
    ///    - readers:me
    ///    - readers:everyone
    ///
    /// Examples:
    ///    "owner:me" returns all tuned models to which caller has owner role
    ///    "readers:me" returns all tuned models to which caller has reader role
    ///    "readers:everyone" returns all tuned models that are shared with everyone
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
}
/// Response from `ListTunedModels` containing a paginated list of Models.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTunedModelsResponse {
    /// The returned Models.
    #[prost(message, repeated, tag = "1")]
    pub tuned_models: ::prost::alloc::vec::Vec<TunedModel>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    ///
    /// If this field is omitted, there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to create a TunedModel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTunedModelRequest {
    /// Optional. The unique id for the tuned model if specified.
    /// This value should be up to 40 characters, the first character must be a
    /// letter, the last could be a letter or a number. The id must match the
    /// regular expression: [a-z](\[a-z0-9-\]{0,38}\[a-z0-9\])?.
    #[prost(string, optional, tag = "1")]
    pub tuned_model_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Required. The tuned model to create.
    #[prost(message, optional, tag = "2")]
    pub tuned_model: ::core::option::Option<TunedModel>,
}
/// Metadata about the state and progress of creating a tuned model returned from
/// the long-running operation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTunedModelMetadata {
    /// Name of the tuned model associated with the tuning operation.
    #[prost(string, tag = "5")]
    pub tuned_model: ::prost::alloc::string::String,
    /// The total number of tuning steps.
    #[prost(int32, tag = "1")]
    pub total_steps: i32,
    /// The number of steps completed.
    #[prost(int32, tag = "2")]
    pub completed_steps: i32,
    /// The completed percentage for the tuning operation.
    #[prost(float, tag = "3")]
    pub completed_percent: f32,
    /// Metrics collected during tuning.
    #[prost(message, repeated, tag = "4")]
    pub snapshots: ::prost::alloc::vec::Vec<TuningSnapshot>,
}
/// Request to update a TunedModel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTunedModelRequest {
    /// Required. The tuned model to update.
    #[prost(message, optional, tag = "1")]
    pub tuned_model: ::core::option::Option<TunedModel>,
    /// Required. The list of fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to delete a TunedModel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTunedModelRequest {
    /// Required. The resource name of the model.
    /// Format: `tunedModels/my-model-id`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod model_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for getting metadata information about Generative Models.
    #[derive(Debug, Clone)]
    pub struct ModelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ModelServiceClient<T>
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
        ) -> ModelServiceClient<InterceptedService<T, F>>
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
            ModelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets information about a specific Model.
        pub async fn get_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelRequest>,
        ) -> std::result::Result<tonic::Response<super::Model>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1beta.ModelService/GetModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.ModelService",
                        "GetModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists models available through the API.
        pub async fn list_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListModelsResponse>,
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
                "/google.ai.generativelanguage.v1beta.ModelService/ListModels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.ModelService",
                        "ListModels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets information about a specific TunedModel.
        pub async fn get_tuned_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTunedModelRequest>,
        ) -> std::result::Result<tonic::Response<super::TunedModel>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1beta.ModelService/GetTunedModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.ModelService",
                        "GetTunedModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists tuned models owned by the user.
        pub async fn list_tuned_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTunedModelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTunedModelsResponse>,
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
                "/google.ai.generativelanguage.v1beta.ModelService/ListTunedModels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.ModelService",
                        "ListTunedModels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a tuned model.
        /// Intermediate tuning progress (if any) is accessed through the
        /// [google.longrunning.Operations] service.
        ///
        /// Status and results can be accessed through the Operations service.
        /// Example:
        ///   GET /v1/tunedModels/az2mb0bpw6i/operations/000-111-222
        pub async fn create_tuned_model(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTunedModelRequest>,
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
                "/google.ai.generativelanguage.v1beta.ModelService/CreateTunedModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.ModelService",
                        "CreateTunedModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a tuned model.
        pub async fn update_tuned_model(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTunedModelRequest>,
        ) -> std::result::Result<tonic::Response<super::TunedModel>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1beta.ModelService/UpdateTunedModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.ModelService",
                        "UpdateTunedModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a tuned model.
        pub async fn delete_tuned_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTunedModelRequest>,
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
                "/google.ai.generativelanguage.v1beta.ModelService/DeleteTunedModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.ModelService",
                        "DeleteTunedModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request to create a `Corpus`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCorpusRequest {
    /// Required. The `Corpus` to create.
    #[prost(message, optional, tag = "1")]
    pub corpus: ::core::option::Option<Corpus>,
}
/// Request for getting information about a specific `Corpus`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCorpusRequest {
    /// Required. The name of the `Corpus`.
    /// Example: `corpora/my-corpus-123`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to update a `Corpus`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCorpusRequest {
    /// Required. The `Corpus` to update.
    #[prost(message, optional, tag = "1")]
    pub corpus: ::core::option::Option<Corpus>,
    /// Required. The list of fields to update.
    /// Currently, this only supports updating `display_name`.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to delete a `Corpus`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCorpusRequest {
    /// Required. The resource name of the `Corpus`.
    /// Example: `corpora/my-corpus-123`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If set to true, any `Document`s and objects related to this
    /// `Corpus` will also be deleted.
    ///
    /// If false (the default), a `FAILED_PRECONDITION` error will be returned if
    /// `Corpus` contains any `Document`s.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request for listing `Corpora`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCorporaRequest {
    /// Optional. The maximum number of `Corpora` to return (per page).
    /// The service may return fewer `Corpora`.
    ///
    /// If unspecified, at most 10 `Corpora` will be returned.
    /// The maximum size limit is 20 `Corpora` per page.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListCorpora` call.
    ///
    /// Provide the `next_page_token` returned in the response as an argument to
    /// the next request to retrieve the next page.
    ///
    /// When paginating, all other parameters provided to `ListCorpora`
    /// must match the call that provided the page token.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response from `ListCorpora` containing a paginated list of `Corpora`.
/// The results are sorted by ascending `corpus.create_time`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCorporaResponse {
    /// The returned corpora.
    #[prost(message, repeated, tag = "1")]
    pub corpora: ::prost::alloc::vec::Vec<Corpus>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for querying a `Corpus`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCorpusRequest {
    /// Required. The name of the `Corpus` to query.
    /// Example: `corpora/my-corpus-123`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Query string to perform semantic search.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// Optional. Filter for `Chunk` and `Document` metadata. Each `MetadataFilter`
    /// object should correspond to a unique key. Multiple `MetadataFilter` objects
    /// are joined by logical "AND"s.
    ///
    /// Example query at document level:
    /// (year >= 2020 OR year < 2010) AND (genre = drama OR genre = action)
    ///
    /// `MetadataFilter` object list:
    ///   metadata_filters = [
    ///   {key = "document.custom_metadata.year"
    ///    conditions = [{int_value = 2020, operation = GREATER_EQUAL},
    ///                  {int_value = 2010, operation = LESS}]},
    ///   {key = "document.custom_metadata.year"
    ///    conditions = [{int_value = 2020, operation = GREATER_EQUAL},
    ///                  {int_value = 2010, operation = LESS}]},
    ///   {key = "document.custom_metadata.genre"
    ///    conditions = [{string_value = "drama", operation = EQUAL},
    ///                  {string_value = "action", operation = EQUAL}]}]
    ///
    /// Example query at chunk level for a numeric range of values:
    /// (year > 2015 AND year <= 2020)
    ///
    /// `MetadataFilter` object list:
    ///   metadata_filters = [
    ///   {key = "chunk.custom_metadata.year"
    ///    conditions = \[{int_value = 2015, operation = GREATER}\]},
    ///   {key = "chunk.custom_metadata.year"
    ///    conditions = \[{int_value = 2020, operation = LESS_EQUAL}\]}]
    ///
    /// Note: "AND"s for the same key are only supported for numeric values. String
    /// values only support "OR"s for the same key.
    #[prost(message, repeated, tag = "3")]
    pub metadata_filters: ::prost::alloc::vec::Vec<MetadataFilter>,
    /// Optional. The maximum number of `Chunk`s to return.
    /// The service may return fewer `Chunk`s.
    ///
    /// If unspecified, at most 10 `Chunk`s will be returned.
    /// The maximum specified result count is 100.
    #[prost(int32, tag = "4")]
    pub results_count: i32,
}
/// Response from `QueryCorpus` containing a list of relevant chunks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCorpusResponse {
    /// The relevant chunks.
    #[prost(message, repeated, tag = "1")]
    pub relevant_chunks: ::prost::alloc::vec::Vec<RelevantChunk>,
}
/// The information for a chunk relevant to a query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelevantChunk {
    /// `Chunk` relevance to the query.
    #[prost(float, tag = "1")]
    pub chunk_relevance_score: f32,
    /// `Chunk` associated with the query.
    #[prost(message, optional, tag = "2")]
    pub chunk: ::core::option::Option<Chunk>,
}
/// Request to create a `Document`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentRequest {
    /// Required. The name of the `Corpus` where this `Document` will be created.
    /// Example: `corpora/my-corpus-123`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The `Document` to create.
    #[prost(message, optional, tag = "2")]
    pub document: ::core::option::Option<Document>,
}
/// Request for getting information about a specific `Document`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentRequest {
    /// Required. The name of the `Document` to retrieve.
    /// Example: `corpora/my-corpus-123/documents/the-doc-abc`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to update a `Document`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentRequest {
    /// Required. The `Document` to update.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// Required. The list of fields to update.
    /// Currently, this only supports updating `display_name` and
    /// `custom_metadata`.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to delete a `Document`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentRequest {
    /// Required. The resource name of the `Document` to delete.
    /// Example: `corpora/my-corpus-123/documents/the-doc-abc`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If set to true, any `Chunk`s and objects related to this
    /// `Document` will also be deleted.
    ///
    /// If false (the default), a `FAILED_PRECONDITION` error will be returned if
    /// `Document` contains any `Chunk`s.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request for listing `Document`s.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentsRequest {
    /// Required. The name of the `Corpus` containing `Document`s.
    /// Example: `corpora/my-corpus-123`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `Document`s to return (per page).
    /// The service may return fewer `Document`s.
    ///
    /// If unspecified, at most 10 `Document`s will be returned.
    /// The maximum size limit is 20 `Document`s per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListDocuments` call.
    ///
    /// Provide the `next_page_token` returned in the response as an argument to
    /// the next request to retrieve the next page.
    ///
    /// When paginating, all other parameters provided to `ListDocuments`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response from `ListDocuments` containing a paginated list of `Document`s.
/// The `Document`s are sorted by ascending `document.create_time`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentsResponse {
    /// The returned `Document`s.
    #[prost(message, repeated, tag = "1")]
    pub documents: ::prost::alloc::vec::Vec<Document>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for querying a `Document`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDocumentRequest {
    /// Required. The name of the `Document` to query.
    /// Example: `corpora/my-corpus-123/documents/the-doc-abc`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Query string to perform semantic search.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// Optional. The maximum number of `Chunk`s to return.
    /// The service may return fewer `Chunk`s.
    ///
    /// If unspecified, at most 10 `Chunk`s will be returned.
    /// The maximum specified result count is 100.
    #[prost(int32, tag = "3")]
    pub results_count: i32,
    /// Optional. Filter for `Chunk` metadata. Each `MetadataFilter` object should
    /// correspond to a unique key. Multiple `MetadataFilter` objects are joined by
    /// logical "AND"s.
    ///
    /// Note: `Document`-level filtering is not supported for this request because
    /// a `Document` name is already specified.
    ///
    /// Example query:
    /// (year >= 2020 OR year < 2010) AND (genre = drama OR genre = action)
    ///
    /// `MetadataFilter` object list:
    ///   metadata_filters = [
    ///   {key = "chunk.custom_metadata.year"
    ///    conditions = [{int_value = 2020, operation = GREATER_EQUAL},
    ///                  {int_value = 2010, operation = LESS}},
    ///   {key = "chunk.custom_metadata.genre"
    ///    conditions = [{string_value = "drama", operation = EQUAL},
    ///                  {string_value = "action", operation = EQUAL}}]
    ///
    /// Example query for a numeric range of values:
    /// (year > 2015 AND year <= 2020)
    ///
    /// `MetadataFilter` object list:
    ///   metadata_filters = [
    ///   {key = "chunk.custom_metadata.year"
    ///    conditions = \[{int_value = 2015, operation = GREATER}\]},
    ///   {key = "chunk.custom_metadata.year"
    ///    conditions = \[{int_value = 2020, operation = LESS_EQUAL}\]}]
    ///
    /// Note: "AND"s for the same key are only supported for numeric values. String
    /// values only support "OR"s for the same key.
    #[prost(message, repeated, tag = "4")]
    pub metadata_filters: ::prost::alloc::vec::Vec<MetadataFilter>,
}
/// Response from `QueryDocument` containing a list of relevant chunks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDocumentResponse {
    /// The returned relevant chunks.
    #[prost(message, repeated, tag = "1")]
    pub relevant_chunks: ::prost::alloc::vec::Vec<RelevantChunk>,
}
/// Request to create a `Chunk`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChunkRequest {
    /// Required. The name of the `Document` where this `Chunk` will be created.
    /// Example: `corpora/my-corpus-123/documents/the-doc-abc`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The `Chunk` to create.
    #[prost(message, optional, tag = "2")]
    pub chunk: ::core::option::Option<Chunk>,
}
/// Request to batch create `Chunk`s.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateChunksRequest {
    /// Optional. The name of the `Document` where this batch of `Chunk`s will be
    /// created. The parent field in every `CreateChunkRequest` must match this
    /// value. Example: `corpora/my-corpus-123/documents/the-doc-abc`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request messages specifying the `Chunk`s to create.
    /// A maximum of 100 `Chunk`s can be created in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<CreateChunkRequest>,
}
/// Response from `BatchCreateChunks` containing a list of created `Chunk`s.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateChunksResponse {
    /// `Chunk`s created.
    #[prost(message, repeated, tag = "1")]
    pub chunks: ::prost::alloc::vec::Vec<Chunk>,
}
/// Request for getting information about a specific `Chunk`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChunkRequest {
    /// Required. The name of the `Chunk` to retrieve.
    /// Example: `corpora/my-corpus-123/documents/the-doc-abc/chunks/some-chunk`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to update a `Chunk`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChunkRequest {
    /// Required. The `Chunk` to update.
    #[prost(message, optional, tag = "1")]
    pub chunk: ::core::option::Option<Chunk>,
    /// Required. The list of fields to update.
    /// Currently, this only supports updating `custom_metadata` and `data`.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to batch update `Chunk`s.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateChunksRequest {
    /// Optional. The name of the `Document` containing the `Chunk`s to update.
    /// The parent field in every `UpdateChunkRequest` must match this value.
    /// Example: `corpora/my-corpus-123/documents/the-doc-abc`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request messages specifying the `Chunk`s to update.
    /// A maximum of 100 `Chunk`s can be updated in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<UpdateChunkRequest>,
}
/// Response from `BatchUpdateChunks` containing a list of updated `Chunk`s.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateChunksResponse {
    /// `Chunk`s updated.
    #[prost(message, repeated, tag = "1")]
    pub chunks: ::prost::alloc::vec::Vec<Chunk>,
}
/// Request to delete a `Chunk`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteChunkRequest {
    /// Required. The resource name of the `Chunk` to delete.
    /// Example: `corpora/my-corpus-123/documents/the-doc-abc/chunks/some-chunk`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to batch delete `Chunk`s.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteChunksRequest {
    /// Optional. The name of the `Document` containing the `Chunk`s to delete.
    /// The parent field in every `DeleteChunkRequest` must match this value.
    /// Example: `corpora/my-corpus-123/documents/the-doc-abc`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request messages specifying the `Chunk`s to delete.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<DeleteChunkRequest>,
}
/// Request for listing `Chunk`s.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChunksRequest {
    /// Required. The name of the `Document` containing `Chunk`s.
    /// Example: `corpora/my-corpus-123/documents/the-doc-abc`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `Chunk`s to return (per page).
    /// The service may return fewer `Chunk`s.
    ///
    /// If unspecified, at most 10 `Chunk`s will be returned.
    /// The maximum size limit is 100 `Chunk`s per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListChunks` call.
    ///
    /// Provide the `next_page_token` returned in the response as an argument to
    /// the next request to retrieve the next page.
    ///
    /// When paginating, all other parameters provided to `ListChunks`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response from `ListChunks` containing a paginated list of `Chunk`s.
/// The `Chunk`s are sorted by ascending `chunk.create_time`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChunksResponse {
    /// The returned `Chunk`s.
    #[prost(message, repeated, tag = "1")]
    pub chunks: ::prost::alloc::vec::Vec<Chunk>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod retriever_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// An API for semantic search over a corpus of user uploaded content.
    #[derive(Debug, Clone)]
    pub struct RetrieverServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RetrieverServiceClient<T>
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
        ) -> RetrieverServiceClient<InterceptedService<T, F>>
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
            RetrieverServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates an empty `Corpus`.
        pub async fn create_corpus(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCorpusRequest>,
        ) -> std::result::Result<tonic::Response<super::Corpus>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/CreateCorpus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "CreateCorpus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets information about a specific `Corpus`.
        pub async fn get_corpus(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCorpusRequest>,
        ) -> std::result::Result<tonic::Response<super::Corpus>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/GetCorpus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "GetCorpus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a `Corpus`.
        pub async fn update_corpus(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCorpusRequest>,
        ) -> std::result::Result<tonic::Response<super::Corpus>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/UpdateCorpus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "UpdateCorpus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a `Corpus`.
        pub async fn delete_corpus(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCorpusRequest>,
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/DeleteCorpus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "DeleteCorpus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all `Corpora` owned by the user.
        pub async fn list_corpora(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCorporaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCorporaResponse>,
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/ListCorpora",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "ListCorpora",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Performs semantic search over a `Corpus`.
        pub async fn query_corpus(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCorpusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCorpusResponse>,
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/QueryCorpus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "QueryCorpus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an empty `Document`.
        pub async fn create_document(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDocumentRequest>,
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/CreateDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "CreateDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets information about a specific `Document`.
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/GetDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "GetDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a `Document`.
        pub async fn update_document(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDocumentRequest>,
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/UpdateDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "UpdateDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a `Document`.
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/DeleteDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "DeleteDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all `Document`s in a `Corpus`.
        pub async fn list_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDocumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDocumentsResponse>,
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/ListDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "ListDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Performs semantic search over a `Document`.
        pub async fn query_document(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDocumentResponse>,
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/QueryDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "QueryDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a `Chunk`.
        pub async fn create_chunk(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateChunkRequest>,
        ) -> std::result::Result<tonic::Response<super::Chunk>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/CreateChunk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "CreateChunk",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Batch create `Chunk`s.
        pub async fn batch_create_chunks(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateChunksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateChunksResponse>,
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/BatchCreateChunks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "BatchCreateChunks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets information about a specific `Chunk`.
        pub async fn get_chunk(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChunkRequest>,
        ) -> std::result::Result<tonic::Response<super::Chunk>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/GetChunk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "GetChunk",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a `Chunk`.
        pub async fn update_chunk(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateChunkRequest>,
        ) -> std::result::Result<tonic::Response<super::Chunk>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/UpdateChunk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "UpdateChunk",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Batch update `Chunk`s.
        pub async fn batch_update_chunks(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateChunksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchUpdateChunksResponse>,
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/BatchUpdateChunks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "BatchUpdateChunks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a `Chunk`.
        pub async fn delete_chunk(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteChunkRequest>,
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/DeleteChunk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "DeleteChunk",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Batch delete `Chunk`s.
        pub async fn batch_delete_chunks(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteChunksRequest>,
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/BatchDeleteChunks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "BatchDeleteChunks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all `Chunk`s in a `Document`.
        pub async fn list_chunks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChunksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListChunksResponse>,
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
                "/google.ai.generativelanguage.v1beta.RetrieverService/ListChunks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.RetrieverService",
                        "ListChunks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request to generate a text completion response from the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateTextRequest {
    /// Required. The name of the `Model` or `TunedModel` to use for generating the
    /// completion.
    /// Examples:
    ///   models/text-bison-001
    ///   tunedModels/sentence-translator-u3b7m
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The free-form input text given to the model as a prompt.
    ///
    /// Given a prompt, the model will generate a TextCompletion response it
    /// predicts as the completion of the input text.
    #[prost(message, optional, tag = "2")]
    pub prompt: ::core::option::Option<TextPrompt>,
    /// Optional. Controls the randomness of the output.
    /// Note: The default value varies by model, see the `Model.temperature`
    /// attribute of the `Model` returned the `getModel` function.
    ///
    /// Values can range from \[0.0,1.0\],
    /// inclusive. A value closer to 1.0 will produce responses that are more
    /// varied and creative, while a value closer to 0.0 will typically result in
    /// more straightforward responses from the model.
    #[prost(float, optional, tag = "3")]
    pub temperature: ::core::option::Option<f32>,
    /// Optional. Number of generated responses to return.
    ///
    /// This value must be between \[1, 8\], inclusive. If unset, this will default
    /// to 1.
    #[prost(int32, optional, tag = "4")]
    pub candidate_count: ::core::option::Option<i32>,
    /// Optional. The maximum number of tokens to include in a candidate.
    ///
    /// If unset, this will default to output_token_limit specified in the `Model`
    /// specification.
    #[prost(int32, optional, tag = "5")]
    pub max_output_tokens: ::core::option::Option<i32>,
    /// Optional. The maximum cumulative probability of tokens to consider when
    /// sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Tokens are sorted based on their assigned probabilities so that only the
    /// most likely tokens are considered. Top-k sampling directly limits the
    /// maximum number of tokens to consider, while Nucleus sampling limits number
    /// of tokens based on the cumulative probability.
    ///
    /// Note: The default value varies by model, see the `Model.top_p`
    /// attribute of the `Model` returned the `getModel` function.
    #[prost(float, optional, tag = "6")]
    pub top_p: ::core::option::Option<f32>,
    /// Optional. The maximum number of tokens to consider when sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Top-k sampling considers the set of `top_k` most probable tokens.
    /// Defaults to 40.
    ///
    /// Note: The default value varies by model, see the `Model.top_k`
    /// attribute of the `Model` returned the `getModel` function.
    #[prost(int32, optional, tag = "7")]
    pub top_k: ::core::option::Option<i32>,
    /// Optional. A list of unique `SafetySetting` instances for blocking unsafe
    /// content.
    ///
    /// that will be enforced on the `GenerateTextRequest.prompt` and
    /// `GenerateTextResponse.candidates`. There should not be more than one
    /// setting for each `SafetyCategory` type. The API will block any prompts and
    /// responses that fail to meet the thresholds set by these settings. This list
    /// overrides the default settings for each `SafetyCategory` specified in the
    /// safety_settings. If there is no `SafetySetting` for a given
    /// `SafetyCategory` provided in the list, the API will use the default safety
    /// setting for that category. Harm categories HARM_CATEGORY_DEROGATORY,
    /// HARM_CATEGORY_TOXICITY, HARM_CATEGORY_VIOLENCE, HARM_CATEGORY_SEXUAL,
    /// HARM_CATEGORY_MEDICAL, HARM_CATEGORY_DANGEROUS are supported in text
    /// service.
    #[prost(message, repeated, tag = "8")]
    pub safety_settings: ::prost::alloc::vec::Vec<SafetySetting>,
    /// The set of character sequences (up to 5) that will stop output generation.
    /// If specified, the API will stop at the first appearance of a stop
    /// sequence. The stop sequence will not be included as part of the response.
    #[prost(string, repeated, tag = "9")]
    pub stop_sequences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The response from the model, including candidate completions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateTextResponse {
    /// Candidate responses from the model.
    #[prost(message, repeated, tag = "1")]
    pub candidates: ::prost::alloc::vec::Vec<TextCompletion>,
    /// A set of content filtering metadata for the prompt and response
    /// text.
    ///
    /// This indicates which `SafetyCategory`(s) blocked a
    /// candidate from this response, the lowest `HarmProbability`
    /// that triggered a block, and the HarmThreshold setting for that category.
    /// This indicates the smallest change to the `SafetySettings` that would be
    /// necessary to unblock at least 1 response.
    ///
    /// The blocking is configured by the `SafetySettings` in the request (or the
    /// default `SafetySettings` of the API).
    #[prost(message, repeated, tag = "3")]
    pub filters: ::prost::alloc::vec::Vec<ContentFilter>,
    /// Returns any safety feedback related to content filtering.
    #[prost(message, repeated, tag = "4")]
    pub safety_feedback: ::prost::alloc::vec::Vec<SafetyFeedback>,
}
/// Text given to the model as a prompt.
///
/// The Model will use this TextPrompt to Generate a text completion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextPrompt {
    /// Required. The prompt text.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
/// Output text returned from a model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextCompletion {
    /// Output only. The generated text returned from the model.
    #[prost(string, tag = "1")]
    pub output: ::prost::alloc::string::String,
    /// Ratings for the safety of a response.
    ///
    /// There is at most one rating per category.
    #[prost(message, repeated, tag = "2")]
    pub safety_ratings: ::prost::alloc::vec::Vec<SafetyRating>,
    /// Output only. Citation information for model-generated `output` in this
    /// `TextCompletion`.
    ///
    /// This field may be populated with attribution information for any text
    /// included in the `output`.
    #[prost(message, optional, tag = "3")]
    pub citation_metadata: ::core::option::Option<CitationMetadata>,
}
/// Request to get a text embedding from the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedTextRequest {
    /// Required. The model name to use with the format model=models/{model}.
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Optional. The free-form input text that the model will turn into an
    /// embedding.
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
/// The response to a EmbedTextRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedTextResponse {
    /// Output only. The embedding generated from the input text.
    #[prost(message, optional, tag = "1")]
    pub embedding: ::core::option::Option<Embedding>,
}
/// Batch request to get a text embedding from the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchEmbedTextRequest {
    /// Required. The name of the `Model` to use for generating the embedding.
    /// Examples:
    ///   models/embedding-gecko-001
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Optional. The free-form input texts that the model will turn into an
    /// embedding. The current limit is 100 texts, over which an error will be
    /// thrown.
    #[prost(string, repeated, tag = "2")]
    pub texts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Embed requests for the batch. Only one of `texts` or `requests`
    /// can be set.
    #[prost(message, repeated, tag = "3")]
    pub requests: ::prost::alloc::vec::Vec<EmbedTextRequest>,
}
/// The response to a EmbedTextRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchEmbedTextResponse {
    /// Output only. The embeddings generated from the input text.
    #[prost(message, repeated, tag = "1")]
    pub embeddings: ::prost::alloc::vec::Vec<Embedding>,
}
/// A list of floats representing the embedding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Embedding {
    /// The embedding values.
    #[prost(float, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<f32>,
}
/// Counts the number of tokens in the `prompt` sent to a model.
///
/// Models may tokenize text differently, so each model may return a different
/// `token_count`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountTextTokensRequest {
    /// Required. The model's resource name. This serves as an ID for the Model to
    /// use.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The free-form input text given to the model as a prompt.
    #[prost(message, optional, tag = "2")]
    pub prompt: ::core::option::Option<TextPrompt>,
}
/// A response from `CountTextTokens`.
///
/// It returns the model's `token_count` for the `prompt`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountTextTokensResponse {
    /// The number of tokens that the `model` tokenizes the `prompt` into.
    ///
    /// Always non-negative.
    #[prost(int32, tag = "1")]
    pub token_count: i32,
}
/// Generated client implementations.
pub mod text_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// API for using Generative Language Models (GLMs) trained to generate text.
    ///
    /// Also known as Large Language Models (LLM)s, these generate text given an
    /// input prompt from the user.
    #[derive(Debug, Clone)]
    pub struct TextServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TextServiceClient<T>
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
        ) -> TextServiceClient<InterceptedService<T, F>>
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
            TextServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Generates a response from the model given an input message.
        pub async fn generate_text(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateTextResponse>,
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
                "/google.ai.generativelanguage.v1beta.TextService/GenerateText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.TextService",
                        "GenerateText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates an embedding from the model given an input message.
        pub async fn embed_text(
            &mut self,
            request: impl tonic::IntoRequest<super::EmbedTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EmbedTextResponse>,
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
                "/google.ai.generativelanguage.v1beta.TextService/EmbedText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.TextService",
                        "EmbedText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates multiple embeddings from the model given input text in a
        /// synchronous call.
        pub async fn batch_embed_text(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchEmbedTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchEmbedTextResponse>,
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
                "/google.ai.generativelanguage.v1beta.TextService/BatchEmbedText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.TextService",
                        "BatchEmbedText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Runs a model's tokenizer on a text and returns the token count.
        pub async fn count_text_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::CountTextTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CountTextTokensResponse>,
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
                "/google.ai.generativelanguage.v1beta.TextService/CountTextTokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta.TextService",
                        "CountTextTokens",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
