#[cfg(any(feature = "google-actions-sdk-v2-interactionmodel-prompt",))]
pub mod prompt;

#[cfg(any(feature = "google-actions-sdk-v2-interactionmodel-type",))]
pub mod r#type;

/// Entity sets describe the pre-defined set of entities that the values of
/// built-in intent parameters can come from. Entity sets can be referenced from
/// entity_set in built-in intent parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntitySet {
    /// Required. The list of entities this entity set supports.
    #[prost(message, repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<entity_set::Entity>,
}
/// Nested message and enum types in `EntitySet`.
pub mod entity_set {
    /// An entity a built-in intent parameter value can come from.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entity {
        /// Required. The ID of the entity.
        /// For a list of built-in-intent parameters and their supported entities,
        /// see
        /// <https://developers.google.com/assistant/conversational/build/built-in-intents>
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
    }
}
/// Defines a handler to be executed after an event. Examples of events are
/// intent and condition based events in a scene.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventHandler {
    /// Name of the webhook handler to call.
    #[prost(string, tag = "1")]
    pub webhook_handler: ::prost::alloc::string::String,
    /// Prompts can either be inlined or referenced by name.
    #[prost(oneof = "event_handler::Prompt", tags = "2, 3")]
    pub prompt: ::core::option::Option<event_handler::Prompt>,
}
/// Nested message and enum types in `EventHandler`.
pub mod event_handler {
    /// Prompts can either be inlined or referenced by name.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Prompt {
        /// Inlined static prompt. Can contain references to string resources in
        /// bundles.
        #[prost(message, tag = "2")]
        StaticPrompt(super::prompt::StaticPrompt),
        /// Name of the static prompt to invoke.
        #[prost(string, tag = "3")]
        StaticPromptName(::prost::alloc::string::String),
    }
}
/// Registers events that trigger as the result of a true condition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionalEvent {
    /// Required. Filter condition for this event to trigger. If condition is evaluated to
    /// true then the associated `handler` will be triggered.
    /// The following variable references are supported:
    ///   `$session` - To reference data in session storage.
    ///   `$user` - To reference data in user storage.
    /// The following boolean operators are supported (with examples):
    ///   `&&` - `session.params.counter > 0 && session.params.counter < 100`
    ///   `||` - `session.params.foo == "John" || session.params.counter == "Adam"`
    ///   `!`  - `!(session.params.counter == 5)`
    /// The following comparisons are supported:
    ///   `==`, `!=`, `<`, `>`, `<=`, `>=`
    /// The following list and string operators are supported (with examples):
    ///   `in`        - "Watermelon" in `session.params.fruitList`
    ///   `size`      - `size(session.params.fruitList) > 2`
    ///   `substring` - `session.params.fullName.contains("John")`
    #[prost(string, tag = "1")]
    pub condition: ::prost::alloc::string::String,
    /// Optional. Destination scene which the conversation should jump to when the associated
    /// condition is evaluated to true. The state of the current scene is destroyed
    /// on the transition.
    #[prost(string, tag = "2")]
    pub transition_to_scene: ::prost::alloc::string::String,
    /// Optional. Event handler which is triggered when the associated condition is evaluated
    /// to `true`. Should execute before transitioning to the destination scene.
    /// Useful to generate Prompts in response to events.
    #[prost(message, optional, tag = "3")]
    pub handler: ::core::option::Option<EventHandler>,
}
/// Intents map open-ended user input to structured objects. Spoken
/// phrases are matched to intents with Google's Natural Language Understanding
/// (NLU). Intent matches can trigger events in your conversation design to
/// progress the user's conversation.
/// The intent name is specified in the name of the file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Intent {
    /// The list of parameters within the training phrases. All parameters must be
    /// defined here to be used in the training phrase.
    #[prost(message, repeated, tag = "1")]
    pub parameters: ::prost::alloc::vec::Vec<intent::IntentParameter>,
    /// Training phrases allow Google’s NLU to automatically match intents with
    /// user input. The more unique phrases that are provided, the better chance
    /// this intent will be matched.
    /// The following is the format of training phrase part which are annotated.
    /// Note that `auto` field is optional and the default behavior when `auto` is
    /// not specified is equivalent to `auto=false`.
    /// `($<paramName> '<sample text>' auto=<true or false>)`
    /// `auto = true` means the part was auto annotated by NLU.
    /// `auto = false` means the part was annotated by the user. This is the
    ///     default when auto is not specified.
    /// Example:
    /// "Book a flight from ($source 'San Francisco' auto=false) to ($dest
    /// 'Vancouver')"
    #[prost(string, repeated, tag = "2")]
    pub training_phrases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Intent`.
pub mod intent {
    /// Definition of a parameter which can be used inside training phrases.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntentParameter {
        /// Required. Unique name of the intent parameter. Can be used in conditions and
        /// responses to reference intent parameters extracted by NLU with
        /// $intent.params.\[name\].resolved
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The type of the intent parameter.
        #[prost(oneof = "intent_parameter::ParameterType", tags = "2, 3")]
        pub parameter_type: ::core::option::Option<intent_parameter::ParameterType>,
    }
    /// Nested message and enum types in `IntentParameter`.
    pub mod intent_parameter {
        /// Entity set references for an intent parameter.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct EntitySetReferences {
            /// Required. Entity set references for an intent parameter.
            #[prost(message, repeated, tag = "1")]
            pub entity_set_references:
                ::prost::alloc::vec::Vec<entity_set_references::EntitySetReference>,
        }
        /// Nested message and enum types in `EntitySetReferences`.
        pub mod entity_set_references {
            /// A reference to the set of allowed entities for this intent parameter.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct EntitySetReference {
                /// Required. Identifies the specific collection of entities to be considered for a
                /// given parameter. The corresponding entity set definition should be
                /// present in the custom/entitySets/ directory.
                #[prost(string, tag = "1")]
                pub entity_set: ::prost::alloc::string::String,
            }
        }
        /// The type of the intent parameter.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ParameterType {
            /// Optional. Declares the data type of this parameter.
            /// This should not be set for built-in intents.
            #[prost(message, tag = "2")]
            Type(super::super::r#type::ClassReference),
            /// Optional. References to the sets of allowed entities for this intent parameter.
            /// Only valid for parameters of a built-in intent. These
            /// references point to entity sets in the 'custom/entitySets' directory.
            #[prost(message, tag = "3")]
            EntitySetReferences(EntitySetReferences),
        }
    }
}
/// Configuration for a slot. Slots are single units of data that can be filled
/// through natural language (ie. intent parameters), session parameters, and
/// other sources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Slot {
    /// Required. Name of the slot.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Declares the data type of this slot.
    #[prost(message, optional, tag = "2")]
    pub r#type: ::core::option::Option<r#type::ClassReference>,
    /// Optional. Indicates whether the slot is required to be filled before
    /// advancing. Required slots that are not filled will trigger a customizable
    /// prompt to the user.
    #[prost(bool, tag = "3")]
    pub required: bool,
    /// Optional. Registers Prompts for different stages of slot filling.
    #[prost(message, optional, tag = "4")]
    pub prompt_settings: ::core::option::Option<slot::PromptSettings>,
    /// Optional. Commit behavior associated with the slot.
    #[prost(message, optional, tag = "5")]
    pub commit_behavior: ::core::option::Option<slot::CommitBehavior>,
    /// Optional. Additional configuration associated with the slot which is
    /// used for filling the slot. The format of the config is specific to the
    /// type of the slot. Resource references to user or session parameter can be
    /// added to this config. This config is needed for filling slots related to
    /// transactions and user engagement.
    ///
    /// Example:
    ///  For a slot of type actions.type.CompletePurchaseValue, the following
    ///  config proposes a digital good order with a reference to a client defined
    ///  session parameter `userSelectedSkuId`:
    ///
    ///    {
    ///      "@type": "type.googleapis.com/
    ///                  google.actions.transactions.v3.CompletePurchaseValueSpec",
    ///      "skuId": {
    ///        "skuType": "SKU_TYPE_IN_APP",
    ///        "id": "$session.params.userSelectedSkuId",
    ///        "packageName": "com.example.company"
    ///      }
    ///    }
    #[prost(message, optional, tag = "6")]
    pub config: ::core::option::Option<::prost_types::Value>,
    /// Optional. Configuration to populate a default value for this slot.
    #[prost(message, optional, tag = "7")]
    pub default_value: ::core::option::Option<slot::DefaultValue>,
}
/// Nested message and enum types in `Slot`.
pub mod slot {
    /// A single place where slot prompts are defined.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PromptSettings {
        /// Prompt for the slot value itself. Example: "What size did you want?"
        #[prost(message, optional, tag = "1")]
        pub initial_prompt: ::core::option::Option<super::EventHandler>,
        /// Prompt to give when the user's input does not match the expected
        /// value type for the slot for the first time. Example: "Sorry, I
        /// didn't get that."
        #[prost(message, optional, tag = "2")]
        pub no_match_prompt1: ::core::option::Option<super::EventHandler>,
        /// Prompt to give when the user's input does not match the expected
        /// value type for the slot for the second time. Example: "Sorry, I
        /// didn't get that."
        #[prost(message, optional, tag = "3")]
        pub no_match_prompt2: ::core::option::Option<super::EventHandler>,
        /// Prompt to give when the user's input does not match the expected
        /// value type for the slot for the last time. Example: "Sorry, I
        /// didn't get that."
        #[prost(message, optional, tag = "4")]
        pub no_match_final_prompt: ::core::option::Option<super::EventHandler>,
        /// Prompt to give when the user does not provide an input for the first
        /// time. Example: "Sorry, I didn't get that."
        #[prost(message, optional, tag = "5")]
        pub no_input_prompt1: ::core::option::Option<super::EventHandler>,
        /// Prompt to give when the user does not provide an input for the second
        /// time. Example: "Sorry, I didn't get that."
        #[prost(message, optional, tag = "6")]
        pub no_input_prompt2: ::core::option::Option<super::EventHandler>,
        /// Prompt to give when the user does not provide an input for the last
        /// time. Example: "Sorry, I didn't get that."
        #[prost(message, optional, tag = "7")]
        pub no_input_final_prompt: ::core::option::Option<super::EventHandler>,
    }
    /// Message describing the commit behavior associated with the slot after it
    /// has been successfully filled.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CommitBehavior {
        /// The session parameter to write the slot value after it is filled. Note
        /// that nested paths are not currently supported. "$$" is used to write the
        /// slot value to a session parameter with same name as the slot.
        /// Eg: write_session_param = "fruit" corresponds to "$session.params.fruit".
        /// write_session_param = "ticket" corresponds to "$session.params.ticket".
        #[prost(string, tag = "1")]
        pub write_session_param: ::prost::alloc::string::String,
    }
    /// Configuration to populate a default value for this slot.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DefaultValue {
        /// Optional. The session parameter to be used to initialize the slot value, if it has
        /// a non-empty value. The type of the value must match the type of the slot.
        /// Note that nested paths are not currently supported.
        /// Eg: `session_param = "fruit"` corresponds to `$session.params.fruit`.
        /// `session_param = "ticket"` corresponds to `$session.params.ticket`.
        #[prost(string, tag = "1")]
        pub session_param: ::prost::alloc::string::String,
        /// Optional. Constant default value for the slot. This will only be used if a value
        /// for this slot was not populated through the `session_param`. The
        /// type for this value must match the type of the slot.
        #[prost(message, optional, tag = "2")]
        pub constant: ::core::option::Option<::prost_types::Value>,
    }
}
/// Registers Events which trigger as the result of an intent match.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentEvent {
    /// Required. Intent triggering the event.
    #[prost(string, tag = "1")]
    pub intent: ::prost::alloc::string::String,
    /// Optional. Destination scene which the conversation should jump to. The state of the
    /// current scene is destroyed on the transition.
    #[prost(string, tag = "2")]
    pub transition_to_scene: ::prost::alloc::string::String,
    /// Optional. Event handler which is triggered when the intent is matched. Should execute
    /// before transitioning to the destination scene. Useful to generate prompts
    /// in response to events.
    #[prost(message, optional, tag = "3")]
    pub handler: ::core::option::Option<EventHandler>,
}
/// Defines a global intent handler. Global intent events are scoped to the
/// entire Actions project and may be overridden by intent handlers in a scene.
/// Intent names must be unique within an Actions project.
///
/// Global intents can be matched anytime during a session, allowing users to
/// access common flows like  "get help" or "go back home." They can also be
/// used to deep link users into specific flows when they invoke an Action.
///
/// Note, the intent name is specified in the name of the file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalIntentEvent {
    /// Optional. Destination scene which the conversation should jump to. The state of the
    /// current scene is destroyed on the transition.
    #[prost(string, tag = "1")]
    pub transition_to_scene: ::prost::alloc::string::String,
    /// Optional. Event handler which is triggered when the intent is matched. Should execute
    /// before transitioning to the destination scene. Useful to generate Prompts
    /// in response to events.
    #[prost(message, optional, tag = "2")]
    pub handler: ::core::option::Option<EventHandler>,
}
/// Scene is the basic unit of control flow when designing a conversation. They
/// can be chained together with other scenes, generate prompts for the end user,
/// and define slots.
/// The scene name is specified in the name of the file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scene {
    /// Handler to invoke when transitioning into this scene.
    #[prost(message, optional, tag = "1")]
    pub on_enter: ::core::option::Option<EventHandler>,
    /// The list of events that trigger based on intents. These events can
    /// be triggered at any time after the on_load Handler has been called.
    /// Important - these events define the set of intents which are scoped to
    /// this scene and will take precedence over any globally defined events that
    /// have the same intents or their triggering phrases. Intent names must be
    /// unique within a scene.
    #[prost(message, repeated, tag = "2")]
    pub intent_events: ::prost::alloc::vec::Vec<IntentEvent>,
    /// The list of events to trigger based on conditional statements. These are
    /// evaluated after the form has been filled or immediately after on_load if
    /// this scene does not have a form (evaluation is only done once). Only the
    /// first matching event will be triggered.
    #[prost(message, repeated, tag = "3")]
    pub conditional_events: ::prost::alloc::vec::Vec<ConditionalEvent>,
    /// Ordered list of slots. Each slot defines the type of data
    /// that it will resolve and configuration to customize the experience of this
    /// resolution (e.g. prompts).
    #[prost(message, repeated, tag = "4")]
    pub slots: ::prost::alloc::vec::Vec<Slot>,
    /// Handler called when there is a change in state of a slot not
    /// caused by updates within another Handler. This allows slots to be
    /// invalidated, the scene invalidated or other changes to scene state.
    #[prost(message, optional, tag = "5")]
    pub on_slot_updated: ::core::option::Option<EventHandler>,
}
