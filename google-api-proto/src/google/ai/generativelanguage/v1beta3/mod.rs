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
            _ => None,
        }
    }
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
                "/google.ai.generativelanguage.v1beta3.DiscussService/GenerateMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.DiscussService",
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
                "/google.ai.generativelanguage.v1beta3.DiscussService/CountMessageTokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.DiscussService",
                        "CountMessageTokens",
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
/// Permission resource grants user, group or the rest of the world access to the
/// PaLM API resource (e.g. a tuned model, file).
///
/// A role is a collection of permitted operations that allows users to perform
/// specific actions on PaLM API resources. To make them available to users,
/// groups, or service accounts, you assign roles. When you assign a role, you
/// grant permissions that the role contains.
///
/// There are three concentric roles. Each role is a superset of the previous
/// role's permitted operations:
///   - reader can use the resource (e.g. tuned model) for inference
///   - writer has reader's permissions and additionally can edit and share
///   - owner has writer's permissions and additionally can delete
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permission {
    /// Output only. The permission name. A unique name will be generated on
    /// create. Example: tunedModels/{tuned_model}permssions/{permission} Output
    /// only.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The type of the grantee.
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
    /// Format: tunedModels/{tuned_model}
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
    /// Format: `tunedModels/{tuned_model}permissions/{permission}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for listing permissions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPermissionsRequest {
    /// Required. The parent resource of the permissions.
    /// Format: tunedModels/{tuned_model}
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
    /// Format: `tunedModels/{tuned_model}/permissions/{permission}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to transfer the ownership of the tuned model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferOwnershipRequest {
    /// Required. The resource name of the tuned model to transfer ownership .
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
                "/google.ai.generativelanguage.v1beta3.PermissionService/CreatePermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.PermissionService",
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
                "/google.ai.generativelanguage.v1beta3.PermissionService/GetPermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.PermissionService",
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
                "/google.ai.generativelanguage.v1beta3.PermissionService/ListPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.PermissionService",
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
                "/google.ai.generativelanguage.v1beta3.PermissionService/UpdatePermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.PermissionService",
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
                "/google.ai.generativelanguage.v1beta3.PermissionService/DeletePermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.PermissionService",
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
                "/google.ai.generativelanguage.v1beta3.PermissionService/TransferOwnership",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.PermissionService",
                        "TransferOwnership",
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
    /// A list of unique `SafetySetting` instances for blocking unsafe content.
    ///
    /// that will be enforced on the `GenerateTextRequest.prompt` and
    /// `GenerateTextResponse.candidates`. There should not be more than one
    /// setting for each `SafetyCategory` type. The API will block any prompts and
    /// responses that fail to meet the thresholds set by these settings. This list
    /// overrides the default settings for each `SafetyCategory` specified in the
    /// safety_settings. If there is no `SafetySetting` for a given
    /// `SafetyCategory` provided in the list, the API will use the default safety
    /// setting for that category.
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
    /// Required. The free-form input text that the model will turn into an
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
    /// Required. The free-form input texts that the model will turn into an
    /// embedding.  The current limit is 100 texts, over which an error will be
    /// thrown.
    #[prost(string, repeated, tag = "2")]
    pub texts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
                "/google.ai.generativelanguage.v1beta3.TextService/GenerateText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.TextService",
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
                "/google.ai.generativelanguage.v1beta3.TextService/EmbedText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.TextService",
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
                "/google.ai.generativelanguage.v1beta3.TextService/BatchEmbedText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.TextService",
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
                "/google.ai.generativelanguage.v1beta3.TextService/CountTextTokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.TextService",
                        "CountTextTokens",
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
                "/google.ai.generativelanguage.v1beta3.ModelService/GetModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.ModelService",
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
                "/google.ai.generativelanguage.v1beta3.ModelService/ListModels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.ModelService",
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
                "/google.ai.generativelanguage.v1beta3.ModelService/GetTunedModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.ModelService",
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
                "/google.ai.generativelanguage.v1beta3.ModelService/ListTunedModels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.ModelService",
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
                "/google.ai.generativelanguage.v1beta3.ModelService/CreateTunedModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.ModelService",
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
                "/google.ai.generativelanguage.v1beta3.ModelService/UpdateTunedModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.ModelService",
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
                "/google.ai.generativelanguage.v1beta3.ModelService/DeleteTunedModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1beta3.ModelService",
                        "DeleteTunedModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
