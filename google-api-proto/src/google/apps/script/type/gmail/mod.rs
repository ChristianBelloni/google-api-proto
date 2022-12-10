/// Properties customizing the appearance and execution of a Gmail add-on.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GmailAddOnManifest {
    /// Defines an endpoint that will be executed in contexts that don't
    /// match a declared contextual trigger. Any cards generated by this function
    /// will always be available to the user, but may be eclipsed by contextual
    /// content when this add-on declares more targeted triggers.
    ///
    /// If present, this overrides the configuration from
    /// `addOns.common.homepageTrigger`.
    #[prost(message, optional, tag = "14")]
    pub homepage_trigger: ::core::option::Option<super::HomepageExtensionPoint>,
    /// Defines the set of conditions that trigger the add-on.
    #[prost(message, repeated, tag = "3")]
    pub contextual_triggers: ::prost::alloc::vec::Vec<ContextualTrigger>,
    /// Defines set of [universal
    /// actions](/gmail/add-ons/how-tos/universal-actions) for the add-on. The user
    /// triggers universal actions from the add-on toolbar menu.
    #[prost(message, repeated, tag = "4")]
    pub universal_actions: ::prost::alloc::vec::Vec<UniversalAction>,
    /// Defines the compose time trigger for a compose time add-on. This is the
    /// trigger that causes an add-on to take action when the user is composing an
    /// email.
    /// All compose time addons are required to have the
    /// gmail.addons.current.action.compose scope even though it might not edit the
    /// draft.
    #[prost(message, optional, tag = "12")]
    pub compose_trigger: ::core::option::Option<ComposeTrigger>,
    /// The name of an endpoint that verifies that the add-on has
    /// all the required third-party authorizations, by probing the third-party
    /// APIs. If the probe fails, the function should throw an exception to
    /// initiate the authorization flow. This function is called before each
    /// invocation of the add-on, in order to ensure a smooth user experience.
    #[prost(string, tag = "7")]
    pub authorization_check_function: ::prost::alloc::string::String,
}
/// An action that is always available in the add-on toolbar menu regardless of
/// message context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniversalAction {
    /// Required. User-visible text describing the action, for example, "Add a new
    /// contact."
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// The type of the action determines the behavior of Gmail when the user
    /// invokes the action.
    #[prost(oneof = "universal_action::ActionType", tags = "2, 3")]
    pub action_type: ::core::option::Option<universal_action::ActionType>,
}
/// Nested message and enum types in `UniversalAction`.
pub mod universal_action {
    /// The type of the action determines the behavior of Gmail when the user
    /// invokes the action.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ActionType {
        /// A link that is opened by Gmail when the user triggers the action.
        #[prost(string, tag = "2")]
        OpenLink(::prost::alloc::string::String),
        /// An endpoint that is called when the user triggers the
        /// action. See the [universal actions
        /// guide](/gmail/add-ons/how-tos/universal-actions) for details.
        #[prost(string, tag = "3")]
        RunFunction(::prost::alloc::string::String),
    }
}
/// A trigger that activates when user is composing an email.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComposeTrigger {
    /// Defines the set of actions for compose time add-on. These are actions
    /// that user can trigger on a compose time addon.
    #[prost(message, repeated, tag = "5")]
    pub actions: ::prost::alloc::vec::Vec<super::MenuItemExtensionPoint>,
    /// Define the level of data access when a compose time addon is triggered.
    #[prost(enumeration = "compose_trigger::DraftAccess", tag = "4")]
    pub draft_access: i32,
}
/// Nested message and enum types in `ComposeTrigger`.
pub mod compose_trigger {
    /// An enum defining the level of data access this compose trigger requires.
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
    pub enum DraftAccess {
        /// Default value when nothing is set for DraftAccess.
        Unspecified = 0,
        /// NONE means compose trigger won't be able to access any data of the draft
        /// when a compose addon is triggered.
        None = 1,
        /// METADATA gives compose trigger the permission to access the metadata of
        /// the draft when a compose addon is triggered. This includes the audience
        /// list (To/cc list) of a draft message.
        Metadata = 2,
    }
    impl DraftAccess {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DraftAccess::Unspecified => "UNSPECIFIED",
                DraftAccess::None => "NONE",
                DraftAccess::Metadata => "METADATA",
            }
        }
    }
}
/// Defines a trigger that fires when the open email meets a specific criteria.
/// When the trigger fires, it executes a specific endpoint, usually
/// in order to create new cards and update the UI.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextualTrigger {
    /// Required. The name of the endpoint to call when a message matches the
    /// trigger.
    #[prost(string, tag = "4")]
    pub on_trigger_function: ::prost::alloc::string::String,
    /// The type of trigger determines the conditions Gmail uses to show the
    /// add-on.
    #[prost(oneof = "contextual_trigger::Trigger", tags = "1")]
    pub trigger: ::core::option::Option<contextual_trigger::Trigger>,
}
/// Nested message and enum types in `ContextualTrigger`.
pub mod contextual_trigger {
    /// The type of trigger determines the conditions Gmail uses to show the
    /// add-on.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trigger {
        /// UnconditionalTriggers are executed when any mail message is opened.
        #[prost(message, tag = "1")]
        Unconditional(super::UnconditionalTrigger),
    }
}
/// A trigger that fires when any email message is opened.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnconditionalTrigger {}
