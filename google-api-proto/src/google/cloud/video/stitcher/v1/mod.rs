/// Slate object
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Slate {
    /// Output only. The name of the slate, in the form of
    /// `projects/{project_number}/locations/{location}/slates/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The URI to fetch the source content for the slate. This URI must return an
    /// MP4 video with at least one audio track.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// gam_slate has all the GAM-related attributes of slates.
    #[prost(message, optional, tag = "3")]
    pub gam_slate: ::core::option::Option<slate::GamSlate>,
}
/// Nested message and enum types in `Slate`.
pub mod slate {
    /// GamSlate object has Google Ad Manager (GAM) related properties for the
    /// slate.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GamSlate {
        /// Required. Ad Manager network code to associate with the live config.
        #[prost(string, tag = "1")]
        pub network_code: ::prost::alloc::string::String,
        /// Output only. The identifier generated for the slate by GAM.
        #[prost(int64, tag = "2")]
        pub gam_slate_id: i64,
    }
}
/// Container for a live session's ad tag detail.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveAdTagDetail {
    /// The resource name in the form of
    /// `projects/{project}/locations/{location}/liveSessions/{live_session}/liveAdTagDetails/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of ad requests.
    #[prost(message, repeated, tag = "2")]
    pub ad_requests: ::prost::alloc::vec::Vec<AdRequest>,
}
/// Information related to the details for one ad tag. This resource is only
/// available for VOD sessions that do not implement Google Ad Manager ad
/// insertion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VodAdTagDetail {
    /// The name of the ad tag detail for the specified VOD session, in the form of
    /// `projects/{project}/locations/{location}/vodSessions/{vod_session_id}/vodAdTagDetails/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of ad requests for one ad tag.
    #[prost(message, repeated, tag = "2")]
    pub ad_requests: ::prost::alloc::vec::Vec<AdRequest>,
}
/// Details of an ad request to an ad server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdRequest {
    /// The ad tag URI processed with integrated macros.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// The request metadata used to make the ad request.
    #[prost(message, optional, tag = "2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    /// The response metadata received from the ad request.
    #[prost(message, optional, tag = "3")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
}
/// Metadata for an ad request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMetadata {
    /// The HTTP headers of the ad request.
    #[prost(message, optional, tag = "1")]
    pub headers: ::core::option::Option<::prost_types::Struct>,
}
/// Metadata for the response of an ad request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMetadata {
    /// Error message received when making the ad request.
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
    /// Headers from the response.
    #[prost(message, optional, tag = "2")]
    pub headers: ::core::option::Option<::prost_types::Struct>,
    /// Status code for the response.
    #[prost(string, tag = "3")]
    pub status_code: ::prost::alloc::string::String,
    /// Size in bytes of the response.
    #[prost(int32, tag = "4")]
    pub size_bytes: i32,
    /// Total time elapsed for the response.
    #[prost(message, optional, tag = "5")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// The body of the response.
    #[prost(string, tag = "6")]
    pub body: ::prost::alloc::string::String,
}
/// Detailed information related to the interstitial of a VOD session. This
/// resource is only available for VOD sessions that do not implement Google Ad
/// Manager ad insertion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VodStitchDetail {
    /// The name of the stitch detail in the specified VOD session, in the form of
    /// `projects/{project}/locations/{location}/vodSessions/{vod_session_id}/vodStitchDetails/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of ad processing details for the fetched ad playlist.
    #[prost(message, repeated, tag = "3")]
    pub ad_stitch_details: ::prost::alloc::vec::Vec<AdStitchDetail>,
}
/// Metadata for a stitched ad.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdStitchDetail {
    /// Required. The ad break ID of the processed ad.
    #[prost(string, tag = "1")]
    pub ad_break_id: ::prost::alloc::string::String,
    /// Required. The ad ID of the processed ad.
    #[prost(string, tag = "2")]
    pub ad_id: ::prost::alloc::string::String,
    /// Required. The time offset of the processed ad.
    #[prost(message, optional, tag = "3")]
    pub ad_time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Optional. Indicates the reason why the ad has been skipped.
    #[prost(string, tag = "4")]
    pub skip_reason: ::prost::alloc::string::String,
    /// Optional. The metadata of the chosen media file for the ad.
    #[prost(btree_map = "string, message", tag = "5")]
    pub media: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
}
/// Describes an event and a trigger URI.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Describes the event that occurred.
    #[prost(enumeration = "event::EventType", tag = "1")]
    pub r#type: i32,
    /// The URI to trigger for this event.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// The ID of the event.
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    /// The offset in seconds if the event type is `PROGRESS`.
    #[prost(message, optional, tag = "4")]
    pub offset: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    /// Describes the event that occurred.
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
    pub enum EventType {
        /// The event type is unspecified.
        Unspecified = 0,
        /// First frame of creative ad viewed.
        CreativeView = 1,
        /// Creative ad started.
        Start = 2,
        /// Start of an ad break.
        BreakStart = 3,
        /// End of an ad break.
        BreakEnd = 4,
        /// Impression.
        Impression = 5,
        /// First quartile progress.
        FirstQuartile = 6,
        /// Midpoint progress.
        Midpoint = 7,
        /// Third quartile progress.
        ThirdQuartile = 8,
        /// Ad progress completed.
        Complete = 9,
        /// Specific progress event with an offset.
        Progress = 10,
        /// Player muted.
        Mute = 11,
        /// Player unmuted.
        Unmute = 12,
        /// Player paused.
        Pause = 13,
        /// Click event.
        Click = 14,
        /// Click-through event.
        ClickThrough = 15,
        /// Player rewinding.
        Rewind = 16,
        /// Player resumed.
        Resume = 17,
        /// Error event.
        Error = 18,
        /// Ad expanded to a larger size.
        Expand = 21,
        /// Ad collapsed to a smaller size.
        Collapse = 22,
        /// Non-linear ad closed.
        Close = 24,
        /// Linear ad closed.
        CloseLinear = 25,
        /// Ad skipped.
        Skip = 26,
        /// Accept invitation event.
        AcceptInvitation = 27,
    }
    impl EventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventType::Unspecified => "EVENT_TYPE_UNSPECIFIED",
                EventType::CreativeView => "CREATIVE_VIEW",
                EventType::Start => "START",
                EventType::BreakStart => "BREAK_START",
                EventType::BreakEnd => "BREAK_END",
                EventType::Impression => "IMPRESSION",
                EventType::FirstQuartile => "FIRST_QUARTILE",
                EventType::Midpoint => "MIDPOINT",
                EventType::ThirdQuartile => "THIRD_QUARTILE",
                EventType::Complete => "COMPLETE",
                EventType::Progress => "PROGRESS",
                EventType::Mute => "MUTE",
                EventType::Unmute => "UNMUTE",
                EventType::Pause => "PAUSE",
                EventType::Click => "CLICK",
                EventType::ClickThrough => "CLICK_THROUGH",
                EventType::Rewind => "REWIND",
                EventType::Resume => "RESUME",
                EventType::Error => "ERROR",
                EventType::Expand => "EXPAND",
                EventType::Collapse => "COLLAPSE",
                EventType::Close => "CLOSE",
                EventType::CloseLinear => "CLOSE_LINEAR",
                EventType::Skip => "SKIP",
                EventType::AcceptInvitation => "ACCEPT_INVITATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATIVE_VIEW" => Some(Self::CreativeView),
                "START" => Some(Self::Start),
                "BREAK_START" => Some(Self::BreakStart),
                "BREAK_END" => Some(Self::BreakEnd),
                "IMPRESSION" => Some(Self::Impression),
                "FIRST_QUARTILE" => Some(Self::FirstQuartile),
                "MIDPOINT" => Some(Self::Midpoint),
                "THIRD_QUARTILE" => Some(Self::ThirdQuartile),
                "COMPLETE" => Some(Self::Complete),
                "PROGRESS" => Some(Self::Progress),
                "MUTE" => Some(Self::Mute),
                "UNMUTE" => Some(Self::Unmute),
                "PAUSE" => Some(Self::Pause),
                "CLICK" => Some(Self::Click),
                "CLICK_THROUGH" => Some(Self::ClickThrough),
                "REWIND" => Some(Self::Rewind),
                "RESUME" => Some(Self::Resume),
                "ERROR" => Some(Self::Error),
                "EXPAND" => Some(Self::Expand),
                "COLLAPSE" => Some(Self::Collapse),
                "CLOSE" => Some(Self::Close),
                "CLOSE_LINEAR" => Some(Self::CloseLinear),
                "SKIP" => Some(Self::Skip),
                "ACCEPT_INVITATION" => Some(Self::AcceptInvitation),
                _ => None,
            }
        }
    }
}
/// Indicates a time in which a list of events should be triggered
/// during media playback.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressEvent {
    /// The time when the following tracking events occurs. The time is in
    /// seconds relative to the start of the VOD asset.
    #[prost(message, optional, tag = "1")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// The list of progress tracking events for the ad break. These can be of
    /// the following IAB types: `BREAK_START`, `BREAK_END`, `IMPRESSION`,
    /// `CREATIVE_VIEW`, `START`, `FIRST_QUARTILE`, `MIDPOINT`, `THIRD_QUARTILE`,
    /// `COMPLETE`, `PROGRESS`.
    #[prost(message, repeated, tag = "2")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
/// Metadata for companion ads.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompanionAds {
    /// Indicates how many of the companions should be displayed with the ad.
    #[prost(enumeration = "companion_ads::DisplayRequirement", tag = "1")]
    pub display_requirement: i32,
    /// List of companion ads.
    #[prost(message, repeated, tag = "2")]
    pub companions: ::prost::alloc::vec::Vec<Companion>,
}
/// Nested message and enum types in `CompanionAds`.
pub mod companion_ads {
    /// Indicates how many of the companions should be displayed with the ad.
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
    pub enum DisplayRequirement {
        /// Required companions are not specified. The default is ALL.
        Unspecified = 0,
        /// All companions are required to be displayed.
        All = 1,
        /// At least one of companions needs to be displayed.
        Any = 2,
        /// All companions are optional for display.
        None = 3,
    }
    impl DisplayRequirement {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DisplayRequirement::Unspecified => "DISPLAY_REQUIREMENT_UNSPECIFIED",
                DisplayRequirement::All => "ALL",
                DisplayRequirement::Any => "ANY",
                DisplayRequirement::None => "NONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DISPLAY_REQUIREMENT_UNSPECIFIED" => Some(Self::Unspecified),
                "ALL" => Some(Self::All),
                "ANY" => Some(Self::Any),
                "NONE" => Some(Self::None),
                _ => None,
            }
        }
    }
}
/// Metadata for a companion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Companion {
    /// The API necessary to communicate with the creative if available.
    #[prost(string, tag = "1")]
    pub api_framework: ::prost::alloc::string::String,
    /// The pixel height of the placement slot for the intended creative.
    #[prost(int32, tag = "2")]
    pub height_px: i32,
    /// The pixel width of the placement slot for the intended creative.
    #[prost(int32, tag = "3")]
    pub width_px: i32,
    /// The pixel height of the creative.
    #[prost(int32, tag = "4")]
    pub asset_height_px: i32,
    /// The maximum pixel height of the creative in its expanded state.
    #[prost(int32, tag = "5")]
    pub expanded_height_px: i32,
    /// The pixel width of the creative.
    #[prost(int32, tag = "6")]
    pub asset_width_px: i32,
    /// The maximum pixel width of the creative in its expanded state.
    #[prost(int32, tag = "7")]
    pub expanded_width_px: i32,
    /// The ID used to identify the desired placement on a publisher's page.
    /// Values to be used should be discussed between publishers and
    /// advertisers.
    #[prost(string, tag = "8")]
    pub ad_slot_id: ::prost::alloc::string::String,
    /// The list of tracking events for the companion.
    #[prost(message, repeated, tag = "9")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    /// Ad resource associated with the companion ad.
    #[prost(oneof = "companion::AdResource", tags = "10, 11, 12")]
    pub ad_resource: ::core::option::Option<companion::AdResource>,
}
/// Nested message and enum types in `Companion`.
pub mod companion {
    /// Ad resource associated with the companion ad.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AdResource {
        /// The IFrame ad resource associated with the companion ad.
        #[prost(message, tag = "10")]
        IframeAdResource(super::IframeAdResource),
        /// The static ad resource associated with the companion ad.
        #[prost(message, tag = "11")]
        StaticAdResource(super::StaticAdResource),
        /// The HTML ad resource associated with the companion ad.
        #[prost(message, tag = "12")]
        HtmlAdResource(super::HtmlAdResource),
    }
}
/// Metadata for an HTML ad resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HtmlAdResource {
    /// The HTML to display for the ad resource.
    #[prost(string, tag = "1")]
    pub html_source: ::prost::alloc::string::String,
}
/// Metadata for an IFrame ad resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IframeAdResource {
    /// URI source for an IFrame to display for the ad resource.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// Metadata for a static ad resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticAdResource {
    /// URI to the static file for the ad resource.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// Describes the MIME type of the ad resource.
    #[prost(string, tag = "2")]
    pub creative_type: ::prost::alloc::string::String,
}
/// Metadata for used to register live configs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveConfig {
    /// Output only. The resource name of the live config, in the form of
    /// `projects/{project}/locations/{location}/liveConfigs/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Source URI for the live stream manifest.
    #[prost(string, tag = "2")]
    pub source_uri: ::prost::alloc::string::String,
    /// The default ad tag associated with this live stream config.
    #[prost(string, tag = "3")]
    pub ad_tag_uri: ::prost::alloc::string::String,
    /// Additional metadata used to register a live stream with Google Ad Manager
    /// (GAM)
    #[prost(message, optional, tag = "4")]
    pub gam_live_config: ::core::option::Option<GamLiveConfig>,
    /// Output only. State of the live config.
    #[prost(enumeration = "live_config::State", tag = "5")]
    pub state: i32,
    /// Required. Determines how the ads are tracked. If
    /// \[gam_live_config][google.cloud.video.stitcher.v1.LiveConfig.gam_live_config\]
    /// is set, the value must be `CLIENT` because the IMA SDK handles ad tracking.
    #[prost(enumeration = "AdTracking", tag = "6")]
    pub ad_tracking: i32,
    /// This must refer to a slate in the same
    /// project. If Google Ad Manager (GAM) is used for ads, this string sets the
    /// value of `slateCreativeId` in
    /// <https://developers.google.com/ad-manager/api/reference/v202211/LiveStreamEventService.LiveStreamEvent#slateCreativeId>
    #[prost(string, tag = "7")]
    pub default_slate: ::prost::alloc::string::String,
    /// Defines the stitcher behavior in case an ad does not align exactly with
    /// the ad break boundaries. If not specified, the default is `CUT_CURRENT`.
    #[prost(enumeration = "live_config::StitchingPolicy", tag = "8")]
    pub stitching_policy: i32,
    /// The configuration for prefetching ads.
    #[prost(message, optional, tag = "10")]
    pub prefetch_config: ::core::option::Option<PrefetchConfig>,
}
/// Nested message and enum types in `LiveConfig`.
pub mod live_config {
    /// State of the live config.
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
        /// State is not specified.
        Unspecified = 0,
        /// Live config is being created.
        Creating = 1,
        /// Live config is ready for use.
        Ready = 2,
        /// Live config is queued up for deletion.
        Deleting = 3,
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
                State::Ready => "READY",
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
    /// Defines the ad stitching behavior in case the ad duration does not align
    /// exactly with the ad break boundaries. If not specified, the default is
    /// `CUT_CURRENT`.
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
    pub enum StitchingPolicy {
        /// Stitching policy is not specified.
        Unspecified = 0,
        /// Cuts an ad short and returns to content in the middle of the ad.
        CutCurrent = 1,
        /// Finishes stitching the current ad before returning to content.
        CompleteAd = 2,
    }
    impl StitchingPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StitchingPolicy::Unspecified => "STITCHING_POLICY_UNSPECIFIED",
                StitchingPolicy::CutCurrent => "CUT_CURRENT",
                StitchingPolicy::CompleteAd => "COMPLETE_AD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STITCHING_POLICY_UNSPECIFIED" => Some(Self::Unspecified),
                "CUT_CURRENT" => Some(Self::CutCurrent),
                "COMPLETE_AD" => Some(Self::CompleteAd),
                _ => None,
            }
        }
    }
}
/// The configuration for prefetch ads.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrefetchConfig {
    /// Required. Indicates whether the option to prefetch ad requests is enabled.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// The duration in seconds of the part of the break to be prefetched.
    /// This field is only relevant if prefetch is enabled.
    /// You should set this duration to as long as possible to increase the
    /// benefits of prefetching, but not longer than the shortest ad break
    /// expected. For example, for a live event with 30s and 60s ad breaks, the
    /// initial duration should be set to 30s.
    #[prost(message, optional, tag = "2")]
    pub initial_ad_request_duration: ::core::option::Option<::prost_types::Duration>,
}
/// Metadata used to register a live stream with Google Ad Manager (GAM)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GamLiveConfig {
    /// Required. Ad Manager network code to associate with the live config.
    #[prost(string, tag = "1")]
    pub network_code: ::prost::alloc::string::String,
    /// Output only. The asset key identifier generated for the live config.
    #[prost(string, tag = "2")]
    pub asset_key: ::prost::alloc::string::String,
    /// Output only. The custom asset key identifier generated for the live config.
    #[prost(string, tag = "3")]
    pub custom_asset_key: ::prost::alloc::string::String,
}
/// Determines the ad tracking policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AdTracking {
    /// The ad tracking policy is not specified.
    Unspecified = 0,
    /// Client-side ad tracking is specified. The client player is expected to
    /// trigger playback and activity events itself.
    Client = 1,
    /// The Video Stitcher API will trigger playback events on behalf of
    /// the client player.
    Server = 2,
}
impl AdTracking {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AdTracking::Unspecified => "AD_TRACKING_UNSPECIFIED",
            AdTracking::Client => "CLIENT",
            AdTracking::Server => "SERVER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AD_TRACKING_UNSPECIFIED" => Some(Self::Unspecified),
            "CLIENT" => Some(Self::Client),
            "SERVER" => Some(Self::Server),
            _ => None,
        }
    }
}
/// Metadata for a VOD session. The session expires 4 hours after its creation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VodSession {
    /// Output only. The name of the VOD session, in the form of
    /// `projects/{project_number}/locations/{location}/vodSessions/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Metadata of what was stitched into the content.
    #[prost(message, optional, tag = "2")]
    pub interstitials: ::core::option::Option<Interstitials>,
    /// Output only. The playback URI of the stitched content.
    #[prost(string, tag = "4")]
    pub play_uri: ::prost::alloc::string::String,
    /// Required. URI of the media to stitch.
    #[prost(string, tag = "5")]
    pub source_uri: ::prost::alloc::string::String,
    /// Required. Ad tag URI.
    #[prost(string, tag = "6")]
    pub ad_tag_uri: ::prost::alloc::string::String,
    /// Key value pairs for ad tag macro replacement. If the
    /// specified ad tag URI has macros, this field provides the mapping
    /// to the value that will replace the macro in the ad tag URI.
    /// Macros are designated by square brackets.
    /// For example:
    ///
    ///    Ad tag URI: `"<https://doubleclick.google.com/ad/1?geo_id=\[geoId\]"`>
    ///
    ///    Ad tag macro map: `{"geoId": "123"}`
    ///
    ///    Fully qualified ad tag:
    ///    `"`<https://doubleclick.google.com/ad/1?geo_id=123"`>
    #[prost(btree_map = "string, string", tag = "7")]
    pub ad_tag_macro_map: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Additional options that affect the output of the manifest.
    #[prost(message, optional, tag = "9")]
    pub manifest_options: ::core::option::Option<ManifestOptions>,
    /// Output only. The generated ID of the VodSession's source media.
    #[prost(string, tag = "10")]
    pub asset_id: ::prost::alloc::string::String,
    /// Required. Determines how the ad should be tracked.
    #[prost(enumeration = "AdTracking", tag = "11")]
    pub ad_tracking: i32,
    /// This field should be set with appropriate values if GAM is being used for
    /// ads.
    #[prost(message, optional, tag = "13")]
    pub gam_settings: ::core::option::Option<vod_session::GamSettings>,
}
/// Nested message and enum types in `VodSession`.
pub mod vod_session {
    /// Defines fields related to Google Ad Manager (GAM). This should be set if
    /// GAM is being used for ads.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GamSettings {
        /// Required. Ad Manager network code.
        #[prost(string, tag = "1")]
        pub network_code: ::prost::alloc::string::String,
        /// Required. The stream ID generated by Ad Manager.
        #[prost(string, tag = "2")]
        pub stream_id: ::prost::alloc::string::String,
    }
}
/// Describes what was stitched into a VOD session's manifest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interstitials {
    /// List of ad breaks ordered by time.
    #[prost(message, repeated, tag = "1")]
    pub ad_breaks: ::prost::alloc::vec::Vec<VodSessionAdBreak>,
    /// Information related to the content of the VOD session.
    #[prost(message, optional, tag = "2")]
    pub session_content: ::core::option::Option<VodSessionContent>,
}
/// Metadata for an inserted ad in a VOD session.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VodSessionAd {
    /// Duration in seconds of the ad.
    #[prost(message, optional, tag = "1")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Metadata of companion ads associated with the ad.
    #[prost(message, optional, tag = "2")]
    pub companion_ads: ::core::option::Option<CompanionAds>,
    /// The list of progress tracking events for the ad break. These can be of
    /// the following IAB types: `MUTE`, `UNMUTE`, `PAUSE`, `CLICK`,
    /// `CLICK_THROUGH`, `REWIND`, `RESUME`, `ERROR`, `FULLSCREEN`,
    /// `EXIT_FULLSCREEN`, `EXPAND`, `COLLAPSE`, `ACCEPT_INVITATION_LINEAR`,
    /// `CLOSE_LINEAR`, `SKIP`.
    #[prost(message, repeated, tag = "3")]
    pub activity_events: ::prost::alloc::vec::Vec<Event>,
}
/// Metadata for the entire stitched content in a VOD session.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VodSessionContent {
    /// The total duration in seconds of the content including the ads stitched
    /// in.
    #[prost(message, optional, tag = "1")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
/// Metadata for an inserted ad break.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VodSessionAdBreak {
    /// List of events that are expected to be triggered, ordered by time.
    #[prost(message, repeated, tag = "1")]
    pub progress_events: ::prost::alloc::vec::Vec<ProgressEvent>,
    /// Ordered list of ads stitched into the ad break.
    #[prost(message, repeated, tag = "2")]
    pub ads: ::prost::alloc::vec::Vec<VodSessionAd>,
    /// Ad break end time in seconds relative to the start of the VOD asset.
    #[prost(message, optional, tag = "3")]
    pub end_time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Ad break start time in seconds relative to the start of the VOD asset.
    #[prost(message, optional, tag = "4")]
    pub start_time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Metadata for a live session. The session expires 5 minutes after the client
/// stops fetching the session's playlists.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveSession {
    /// Output only. The name of the live session, in the form of
    /// `projects/{project}/locations/{location}/liveSessions/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The URI to play the live session's ad-stitched stream.
    #[prost(string, tag = "2")]
    pub play_uri: ::prost::alloc::string::String,
    /// Key value pairs for ad tag macro replacement. If the
    /// specified ad tag URI has macros, this field provides the mapping
    /// to the value that will replace the macro in the ad tag URI.
    /// Macros are designated by square brackets.
    ///
    /// For example:
    ///
    ///    Ad tag URI: "<https://doubleclick.google.com/ad/1?geo_id=\[geoId\]">
    ///
    ///    Ad tag macros: `{"geoId": "123"}`
    ///
    ///    Fully qualified ad tag:
    ///    `"<https://doubleclick.google.com/ad/1?geo_id=123"`>
    #[prost(btree_map = "string, string", tag = "6")]
    pub ad_tag_macros: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Additional options that affect the output of the manifest.
    #[prost(message, optional, tag = "10")]
    pub manifest_options: ::core::option::Option<ManifestOptions>,
    /// This field should be set with appropriate values if GAM is being used for
    /// ads.
    #[prost(message, optional, tag = "15")]
    pub gam_settings: ::core::option::Option<live_session::GamSettings>,
    /// Required. The resource name of the live config for this session, in the
    /// form of `projects/{project}/locations/{location}/liveConfigs/{id}`.
    #[prost(string, tag = "16")]
    pub live_config: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LiveSession`.
pub mod live_session {
    /// Defines fields related to Google Ad Manager (GAM). This should be set if
    /// GAM is being used for ads.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GamSettings {
        /// Required. The stream ID generated by Ad Manager.
        #[prost(string, tag = "1")]
        pub stream_id: ::prost::alloc::string::String,
    }
}
/// Options for manifest generation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManifestOptions {
    /// If specified, the output manifest will only return renditions matching the
    /// specified filters.
    #[prost(message, repeated, tag = "1")]
    pub include_renditions: ::prost::alloc::vec::Vec<RenditionFilter>,
    /// If specified, the output manifest will orders the video and muxed
    /// renditions by bitrate according to the ordering policy.
    #[prost(enumeration = "manifest_options::OrderPolicy", tag = "2")]
    pub bitrate_order: i32,
}
/// Nested message and enum types in `ManifestOptions`.
pub mod manifest_options {
    /// Defines the ordering policy during manifest generation.
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
    pub enum OrderPolicy {
        /// Ordering policy is not specified.
        Unspecified = 0,
        /// Order by ascending.
        Ascending = 1,
        /// Order by descending.
        Descending = 2,
    }
    impl OrderPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OrderPolicy::Unspecified => "ORDER_POLICY_UNSPECIFIED",
                OrderPolicy::Ascending => "ASCENDING",
                OrderPolicy::Descending => "DESCENDING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ORDER_POLICY_UNSPECIFIED" => Some(Self::Unspecified),
                "ASCENDING" => Some(Self::Ascending),
                "DESCENDING" => Some(Self::Descending),
                _ => None,
            }
        }
    }
}
/// Filters for a video or muxed redition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenditionFilter {
    /// Bitrate in bits per second for the rendition. If set, only renditions with
    /// the exact bitrate will match.
    #[prost(int32, tag = "1")]
    pub bitrate_bps: i32,
    /// Codecs for the rendition. If set, only renditions with the exact value
    /// will match.
    #[prost(string, tag = "2")]
    pub codecs: ::prost::alloc::string::String,
}
/// Configuration for a CDN key. Used by the Video Stitcher
/// to sign URIs for fetching video manifests and signing
/// media segments for playback.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdnKey {
    /// The resource name of the CDN key, in the form of
    /// `projects/{project}/locations/{location}/cdnKeys/{id}`.
    /// The name is ignored when creating a CDN key.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The hostname this key applies to.
    #[prost(string, tag = "4")]
    pub hostname: ::prost::alloc::string::String,
    /// Configuration associated with the CDN key.
    #[prost(oneof = "cdn_key::CdnKeyConfig", tags = "5, 6, 8")]
    pub cdn_key_config: ::core::option::Option<cdn_key::CdnKeyConfig>,
}
/// Nested message and enum types in `CdnKey`.
pub mod cdn_key {
    /// Configuration associated with the CDN key.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CdnKeyConfig {
        /// The configuration for a Google Cloud CDN key.
        #[prost(message, tag = "5")]
        GoogleCdnKey(super::GoogleCdnKey),
        /// The configuration for an Akamai CDN key.
        #[prost(message, tag = "6")]
        AkamaiCdnKey(super::AkamaiCdnKey),
        /// The configuration for a Media CDN key.
        #[prost(message, tag = "8")]
        MediaCdnKey(super::MediaCdnKey),
    }
}
/// Configuration for a Google Cloud CDN key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleCdnKey {
    /// Input only. Secret for this Google Cloud CDN key.
    #[prost(bytes = "bytes", tag = "1")]
    pub private_key: ::prost::bytes::Bytes,
    /// The public name of the Google Cloud CDN key.
    #[prost(string, tag = "2")]
    pub key_name: ::prost::alloc::string::String,
}
/// Configuration for an Akamai CDN key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AkamaiCdnKey {
    /// Input only. Token key for the Akamai CDN edge configuration.
    #[prost(bytes = "bytes", tag = "1")]
    pub token_key: ::prost::bytes::Bytes,
}
/// Configuration for a Media CDN key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaCdnKey {
    /// Input only. 64-byte ed25519 private key for this Media CDN key.
    #[prost(bytes = "bytes", tag = "1")]
    pub private_key: ::prost::bytes::Bytes,
    /// The keyset name of the Media CDN key.
    #[prost(string, tag = "2")]
    pub key_name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.createCdnKey.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCdnKeyRequest {
    /// Required. The project in which the CDN key should be created, in the form
    /// of `projects/{project_number}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The CDN key resource to create.
    #[prost(message, optional, tag = "2")]
    pub cdn_key: ::core::option::Option<CdnKey>,
    /// Required. The ID to use for the CDN key, which will become the final
    /// component of the CDN key's resource name.
    ///
    /// This value should conform to RFC-1034, which restricts to
    /// lower-case letters, numbers, and hyphen, with the first character a
    /// letter, the last a letter or a number, and a 63 character maximum.
    #[prost(string, tag = "3")]
    pub cdn_key_id: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.listCdnKeys.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCdnKeysRequest {
    /// Required. The project that contains the list of CDN keys, in the form of
    /// `projects/{project_number}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for VideoStitcher.ListCdnKeys.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCdnKeysResponse {
    /// List of CDN keys.
    #[prost(message, repeated, tag = "1")]
    pub cdn_keys: ::prost::alloc::vec::Vec<CdnKey>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for VideoStitcherService.getCdnKey.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCdnKeyRequest {
    /// Required. The name of the CDN key to be retrieved, in the form of
    /// `projects/{project}/locations/{location}/cdnKeys/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.deleteCdnKey.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCdnKeyRequest {
    /// Required. The name of the CDN key to be deleted, in the form of
    /// `projects/{project_number}/locations/{location}/cdnKeys/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.updateCdnKey.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCdnKeyRequest {
    /// Required. The CDN key resource which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub cdn_key: ::core::option::Option<CdnKey>,
    /// Required. The update mask applies to the resource.
    /// For the `FieldMask` definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for VideoStitcherService.createVodSession
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVodSessionRequest {
    /// Required. The project and location in which the VOD session should be
    /// created, in the form of `projects/{project_number}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Parameters for creating a session.
    #[prost(message, optional, tag = "2")]
    pub vod_session: ::core::option::Option<VodSession>,
}
/// Request message for VideoStitcherService.getVodSession
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVodSessionRequest {
    /// Required. The name of the VOD session to be retrieved, in the form of
    /// `projects/{project_number}/locations/{location}/vodSessions/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.listVodStitchDetails.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVodStitchDetailsRequest {
    /// Required. The VOD session where the stitch details belong to, in the form
    /// of `projects/{project}/locations/{location}/vodSessions/{id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for VideoStitcherService.listVodStitchDetails.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVodStitchDetailsResponse {
    /// A List of stitch Details.
    #[prost(message, repeated, tag = "1")]
    pub vod_stitch_details: ::prost::alloc::vec::Vec<VodStitchDetail>,
    /// The pagination token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.getVodStitchDetail.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVodStitchDetailRequest {
    /// Required. The name of the stitch detail in the specified VOD session, in
    /// the form of
    /// `projects/{project}/locations/{location}/vodSessions/{vod_session_id}/vodStitchDetails/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.listVodAdTagDetails.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVodAdTagDetailsRequest {
    /// Required. The VOD session which the ad tag details belong to, in the form
    /// of `projects/{project}/locations/{location}/vodSessions/{vod_session_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for VideoStitcherService.listVodAdTagDetails.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVodAdTagDetailsResponse {
    /// A List of ad tag details.
    #[prost(message, repeated, tag = "1")]
    pub vod_ad_tag_details: ::prost::alloc::vec::Vec<VodAdTagDetail>,
    /// The pagination token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.getVodAdTagDetail
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVodAdTagDetailRequest {
    /// Required. The name of the ad tag detail for the specified VOD session, in
    /// the form of
    /// `projects/{project}/locations/{location}/vodSessions/{vod_session_id}/vodAdTagDetails/{vod_ad_tag_detail}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.listLiveAdTagDetails.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLiveAdTagDetailsRequest {
    /// Required. The resource parent in the form of
    /// `projects/{project}/locations/{location}/liveSessions/{live_session}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The pagination token returned from a previous List request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for VideoStitcherService.listLiveAdTagDetails.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLiveAdTagDetailsResponse {
    /// A list of live session ad tag details.
    #[prost(message, repeated, tag = "1")]
    pub live_ad_tag_details: ::prost::alloc::vec::Vec<LiveAdTagDetail>,
    /// The pagination token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.getLiveAdTagDetail
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLiveAdTagDetailRequest {
    /// Required. The resource name in the form of
    /// `projects/{project}/locations/{location}/liveSessions/{live_session}/liveAdTagDetails/{live_ad_tag_detail}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.createSlate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSlateRequest {
    /// Required. The project in which the slate should be created, in the form of
    /// `projects/{project_number}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The unique identifier for the slate.
    /// This value should conform to RFC-1034, which restricts to
    /// lower-case letters, numbers, and hyphen, with the first character a
    /// letter, the last a letter or a number, and a 63 character maximum.
    #[prost(string, tag = "2")]
    pub slate_id: ::prost::alloc::string::String,
    /// Required. The slate to create.
    #[prost(message, optional, tag = "3")]
    pub slate: ::core::option::Option<Slate>,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported `(00000000-0000-0000-0000-000000000000)`.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.getSlate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSlateRequest {
    /// Required. The name of the slate to be retrieved, of the slate, in the form
    /// of `projects/{project_number}/locations/{location}/slates/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.listSlates.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSlatesRequest {
    /// Required. The project to list slates, in the form of
    /// `projects/{project_number}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for VideoStitcherService.listSlates.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSlatesResponse {
    /// The list of slates
    #[prost(message, repeated, tag = "1")]
    pub slates: ::prost::alloc::vec::Vec<Slate>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for VideoStitcherService.updateSlate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSlateRequest {
    /// Required. The resource with updated fields.
    #[prost(message, optional, tag = "1")]
    pub slate: ::core::option::Option<Slate>,
    /// Required. The update mask which specifies fields which should be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for VideoStitcherService.deleteSlate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSlateRequest {
    /// Required. The name of the slate to be deleted, in the form of
    /// `projects/{project_number}/locations/{location}/slates/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.createLiveSession.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLiveSessionRequest {
    /// Required. The project and location in which the live session should be
    /// created, in the form of `projects/{project_number}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Parameters for creating a live session.
    #[prost(message, optional, tag = "2")]
    pub live_session: ::core::option::Option<LiveSession>,
}
/// Request message for VideoStitcherService.getSession.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLiveSessionRequest {
    /// Required. The name of the live session, in the form of
    /// `projects/{project_number}/locations/{location}/liveSessions/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.createLiveConfig
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLiveConfigRequest {
    /// Required. The project in which the live config should be created, in
    /// the form of `projects/{project_number}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The unique identifier ID to use for the live config.
    #[prost(string, tag = "2")]
    pub live_config_id: ::prost::alloc::string::String,
    /// Required. The live config resource to create.
    #[prost(message, optional, tag = "3")]
    pub live_config: ::core::option::Option<LiveConfig>,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported `(00000000-0000-0000-0000-000000000000)`.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.listLiveConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLiveConfigsRequest {
    /// Required. The project that contains the list of live configs, in the
    /// form of `projects/{project_number}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to apply to list results (see
    /// \[Filtering\](<https://google.aip.dev/160>)).
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specifies the ordering of results following
    /// [Cloud API
    /// syntax](<https://cloud.google.com/apis/design/design_patterns#sorting_order>).
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for VideoStitcher.ListLiveConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLiveConfigsResponse {
    /// List of live configs.
    #[prost(message, repeated, tag = "1")]
    pub live_configs: ::prost::alloc::vec::Vec<LiveConfig>,
    /// The pagination token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for VideoStitcherService.getLiveConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLiveConfigRequest {
    /// Required. The name of the live config to be retrieved, in the form
    /// of
    /// `projects/{project_number}/locations/{location}/liveConfigs/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for VideoStitcherService.deleteLiveConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLiveConfigRequest {
    /// Required. The name of the live config to be deleted, in the form of
    /// `projects/{project_number}/locations/{location}/liveConfigs/{id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod video_stitcher_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Video-On-Demand content stitching API allows you to insert ads
    /// into (VoD) video on demand files. You will be able to render custom
    /// scrubber bars with highlighted ads, enforce ad policies, allow
    /// seamless playback and tracking on native players and monetize
    /// content with any standard VMAP compliant ad server.
    #[derive(Debug, Clone)]
    pub struct VideoStitcherServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VideoStitcherServiceClient<T>
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
        ) -> VideoStitcherServiceClient<InterceptedService<T, F>>
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
            VideoStitcherServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new CDN key.
        pub async fn create_cdn_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCdnKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/CreateCdnKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "CreateCdnKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all CDN keys in the specified project and location.
        pub async fn list_cdn_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCdnKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCdnKeysResponse>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/ListCdnKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "ListCdnKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the specified CDN key.
        pub async fn get_cdn_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCdnKeyRequest>,
        ) -> std::result::Result<tonic::Response<super::CdnKey>, tonic::Status> {
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetCdnKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "GetCdnKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified CDN key.
        pub async fn delete_cdn_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCdnKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/DeleteCdnKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "DeleteCdnKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified CDN key. Only update fields specified
        /// in the call method body.
        pub async fn update_cdn_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCdnKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/UpdateCdnKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "UpdateCdnKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a client side playback VOD session and returns the full
        /// tracking and playback metadata of the session.
        pub async fn create_vod_session(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVodSessionRequest>,
        ) -> std::result::Result<tonic::Response<super::VodSession>, tonic::Status> {
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/CreateVodSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "CreateVodSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the full tracking, playback metadata, and relevant ad-ops
        /// logs for the specified VOD session.
        pub async fn get_vod_session(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVodSessionRequest>,
        ) -> std::result::Result<tonic::Response<super::VodSession>, tonic::Status> {
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetVodSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "GetVodSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a list of detailed stitching information of the specified VOD
        /// session.
        pub async fn list_vod_stitch_details(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVodStitchDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVodStitchDetailsResponse>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/ListVodStitchDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "ListVodStitchDetails",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the specified stitching information for the specified VOD session.
        pub async fn get_vod_stitch_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVodStitchDetailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VodStitchDetail>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetVodStitchDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "GetVodStitchDetail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Return the list of ad tag details for the specified VOD session.
        pub async fn list_vod_ad_tag_details(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVodAdTagDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVodAdTagDetailsResponse>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/ListVodAdTagDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "ListVodAdTagDetails",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the specified ad tag detail for the specified VOD session.
        pub async fn get_vod_ad_tag_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVodAdTagDetailRequest>,
        ) -> std::result::Result<tonic::Response<super::VodAdTagDetail>, tonic::Status> {
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetVodAdTagDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "GetVodAdTagDetail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Return the list of ad tag details for the specified live session.
        pub async fn list_live_ad_tag_details(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLiveAdTagDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLiveAdTagDetailsResponse>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/ListLiveAdTagDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "ListLiveAdTagDetails",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the specified ad tag detail for the specified live session.
        pub async fn get_live_ad_tag_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLiveAdTagDetailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LiveAdTagDetail>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetLiveAdTagDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "GetLiveAdTagDetail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a slate.
        pub async fn create_slate(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSlateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/CreateSlate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "CreateSlate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all slates in the specified project and location.
        pub async fn list_slates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSlatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSlatesResponse>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/ListSlates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "ListSlates",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the specified slate.
        pub async fn get_slate(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSlateRequest>,
        ) -> std::result::Result<tonic::Response<super::Slate>, tonic::Status> {
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetSlate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "GetSlate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified slate.
        pub async fn update_slate(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSlateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/UpdateSlate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "UpdateSlate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified slate.
        pub async fn delete_slate(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSlateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/DeleteSlate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "DeleteSlate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new live session.
        pub async fn create_live_session(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLiveSessionRequest>,
        ) -> std::result::Result<tonic::Response<super::LiveSession>, tonic::Status> {
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/CreateLiveSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "CreateLiveSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the details for the specified live session.
        pub async fn get_live_session(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLiveSessionRequest>,
        ) -> std::result::Result<tonic::Response<super::LiveSession>, tonic::Status> {
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetLiveSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "GetLiveSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Registers the live config with the provided unique ID in
        /// the specified region.
        pub async fn create_live_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLiveConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/CreateLiveConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "CreateLiveConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all live configs managed by the Video Stitcher that
        /// belong to the specified project and region.
        pub async fn list_live_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLiveConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLiveConfigsResponse>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/ListLiveConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "ListLiveConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the specified live config managed by the Video
        /// Stitcher service.
        pub async fn get_live_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLiveConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::LiveConfig>, tonic::Status> {
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/GetLiveConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "GetLiveConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified live config.
        pub async fn delete_live_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLiveConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.stitcher.v1.VideoStitcherService/DeleteLiveConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.video.stitcher.v1.VideoStitcherService",
                        "DeleteLiveConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
