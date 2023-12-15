/// [Developer Preview](<https://developers.google.com/workspace/preview>).
/// Virtual place where conferences are held. Only one active conference can be
/// held in one space at any given time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Space {
    /// Immutable. Resource name of the space.
    /// Format: `spaces/{space}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. URI used to join meeting, such as
    /// `<https://meet.google.com/abc-mnop-xyz`.>
    #[prost(string, tag = "2")]
    pub meeting_uri: ::prost::alloc::string::String,
    /// Output only. Type friendly code to join the meeting. Format:
    /// `\[a-z\]+-\[a-z\]+-\[a-z\]+` such as `abc-mnop-xyz`. The maximum length is 128
    /// characters. Can ONLY be used as alias of the space ID to get the space.
    #[prost(string, tag = "3")]
    pub meeting_code: ::prost::alloc::string::String,
    /// Configuration pertaining to the meeting space.
    #[prost(message, optional, tag = "5")]
    pub config: ::core::option::Option<SpaceConfig>,
    /// Active conference if it exists.
    #[prost(message, optional, tag = "6")]
    pub active_conference: ::core::option::Option<ActiveConference>,
}
/// Active conference.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveConference {
    /// Output only. Reference to 'ConferenceRecord' resource.
    /// Format: `conferenceRecords/{conference_record}` where `{conference_record}`
    /// is a unique id for each instance of a call within a space.
    #[prost(string, tag = "1")]
    pub conference_record: ::prost::alloc::string::String,
}
/// The configuration pertaining to a meeting space.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpaceConfig {
    /// Access type of the meeting space that determines who can join without
    /// knocking. Default: The user's default access settings.  Controlled by the
    /// user's admin for enterprise users or RESTRICTED.
    #[prost(enumeration = "space_config::AccessType", tag = "1")]
    pub access_type: i32,
    /// Defines the entry points that can be used to join meetings hosted in this
    /// meeting space.
    /// Default: EntryPointAccess.ALL
    #[prost(enumeration = "space_config::EntryPointAccess", tag = "2")]
    pub entry_point_access: i32,
}
/// Nested message and enum types in `SpaceConfig`.
pub mod space_config {
    /// Possible access types for a meeting space.
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
    pub enum AccessType {
        /// Default value specified by the user's organization.
        /// Note: This is never returned, as the configured access type is
        /// returned instead.
        Unspecified = 0,
        /// Anyone with the join information (for example, the URL or phone access
        /// information) can join without knocking.
        Open = 1,
        /// Members of the host's organization, invited external users, and dial-in
        /// users can join without knocking. Everyone else must knock.
        Trusted = 2,
        /// Only invitees can join without knocking. Everyone else must knock.
        Restricted = 3,
    }
    impl AccessType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccessType::Unspecified => "ACCESS_TYPE_UNSPECIFIED",
                AccessType::Open => "OPEN",
                AccessType::Trusted => "TRUSTED",
                AccessType::Restricted => "RESTRICTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACCESS_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "OPEN" => Some(Self::Open),
                "TRUSTED" => Some(Self::Trusted),
                "RESTRICTED" => Some(Self::Restricted),
                _ => None,
            }
        }
    }
    /// Entry points that can be used to join a meeting.  Example:
    /// `meet.google.com`, the Embed SDK Web, or a mobile application.
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
    pub enum EntryPointAccess {
        /// Unused.
        Unspecified = 0,
        /// All entry points are allowed.
        All = 1,
        /// Only entry points owned by the Google Cloud project that created the
        /// space can be used to join meetings in this space.  Apps can use the Embed
        /// SDK Web or mobile Meet SDKs to create owned entry points.
        CreatorAppOnly = 2,
    }
    impl EntryPointAccess {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EntryPointAccess::Unspecified => "ENTRY_POINT_ACCESS_UNSPECIFIED",
                EntryPointAccess::All => "ALL",
                EntryPointAccess::CreatorAppOnly => "CREATOR_APP_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENTRY_POINT_ACCESS_UNSPECIFIED" => Some(Self::Unspecified),
                "ALL" => Some(Self::All),
                "CREATOR_APP_ONLY" => Some(Self::CreatorAppOnly),
                _ => None,
            }
        }
    }
}
/// [Developer Preview](<https://developers.google.com/workspace/preview>).
/// Single instance of a meeting held in a space.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConferenceRecord {
    /// Identifier. Resource name of the conference record.
    /// Format: `conferenceRecords/{conference_record}` where `{conference_record}`
    /// is a unique id for each instance of a call within a space.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Timestamp when the conference started, always set.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the conference ended.
    /// Set for past conferences. Unset if the conference is ongoing.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server enforced expire time for when this conference record
    /// resource is deleted. The resource is deleted 30 days after the conference
    /// ends.
    #[prost(message, optional, tag = "4")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The space where the conference was held.
    #[prost(string, tag = "5")]
    pub space: ::prost::alloc::string::String,
}
/// [Developer Preview](<https://developers.google.com/workspace/preview>).
/// User who attended or is attending a conference.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Participant {
    /// Output only. Resource name of the participant.
    /// Format: `conferenceRecords/{conference_record}/participants/{participant}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time when the participant joined the meeting for the first
    /// time.
    #[prost(message, optional, tag = "7")]
    pub earliest_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the participant left the meeting for the last time.
    /// This can be null if it is an active meeting.
    #[prost(message, optional, tag = "8")]
    pub latest_end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "participant::User", tags = "4, 5, 6")]
    pub user: ::core::option::Option<participant::User>,
}
/// Nested message and enum types in `Participant`.
pub mod participant {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum User {
        /// Signed-in user.
        #[prost(message, tag = "4")]
        SignedinUser(super::SignedinUser),
        /// Anonymous user.
        #[prost(message, tag = "5")]
        AnonymousUser(super::AnonymousUser),
        /// User who calls in from their phone.
        #[prost(message, tag = "6")]
        PhoneUser(super::PhoneUser),
    }
}
/// [Developer Preview](<https://developers.google.com/workspace/preview>).
/// Refers to each unique join/leave session when a user joins a conference from
/// a device. Note that any time a user joins the conference a new unique ID is
/// assigned. That means if a user joins a space multiple times from the same
/// device, they're assigned different IDs, and are also be treated as different
/// participant sessions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantSession {
    /// Identifier. Session id.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Timestamp when the user session started.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the user session ended. Unset if the user
    /// session hasn’t ended.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A signed-in user can be:
/// a) An individual joining from a personal computer, mobile device, or through
/// companion mode.
/// b) A robot account used by conference room devices.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedinUser {
    /// Output only. Unique ID for the user. Interoperable with Admin SDK API and
    /// People API. Format: `users/{user}`
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
    /// Output only. For a personal device, it's the user's first and last name.
    /// For a robot account, it's the admin specified device name. For example,
    /// "Altostrat Room".
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
}
/// User who joins anonymously (meaning not signed into a Google Account).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnonymousUser {
    /// Output only. User provided name when they join a conference anonymously.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
}
/// User dialing in from a phone where the user's identity is unknown because
/// they haven't signed in with a Google Account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoneUser {
    /// Output only. Partially redacted user's phone number when they call in.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
}
/// [Developer Preview](<https://developers.google.com/workspace/preview>).
/// Metadata about a recording created during a conference.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Recording {
    /// Output only. Resource name of the recording.
    /// Format: `conferenceRecords/{conference_record}/recordings/{recording}`
    /// where `{recording}` is a 1:1 mapping to each unique recording session
    /// during the conference.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Current state.
    #[prost(enumeration = "recording::State", tag = "3")]
    pub state: i32,
    /// Output only. Timestamp when the recording started.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the recording ended.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "recording::Destination", tags = "6")]
    pub destination: ::core::option::Option<recording::Destination>,
}
/// Nested message and enum types in `Recording`.
pub mod recording {
    /// Current state of the recording session.
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
        /// Default, never used.
        Unspecified = 0,
        /// An active recording session has started.
        Started = 1,
        /// This recording session has ended, but the recording file hasn't been
        /// generated yet.
        Ended = 2,
        /// Recording file is generated and ready to download.
        FileGenerated = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Started => "STARTED",
                State::Ended => "ENDED",
                State::FileGenerated => "FILE_GENERATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STARTED" => Some(Self::Started),
                "ENDED" => Some(Self::Ended),
                "FILE_GENERATED" => Some(Self::FileGenerated),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Output only. Recording is saved to Google Drive as an mp4 file. The
        /// `drive_destination` includes the Drive `fileId` that can be used to
        /// download the file using the `files.get` method of the Drive API.
        #[prost(message, tag = "6")]
        DriveDestination(super::DriveDestination),
    }
}
/// Export location where a recording file is saved in Google Drive.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriveDestination {
    /// Output only. The `fileId` for the underlying MP4 file. For example,
    /// "1kuceFZohVoCh6FulBHxwy6I15Ogpc4hP". Use `$ GET
    /// <https://www.googleapis.com/drive/v3/files/{$fileId}?alt=media`> to download
    /// the blob. For more information, see
    /// <https://developers.google.com/drive/api/v3/reference/files/get.>
    #[prost(string, tag = "1")]
    pub file: ::prost::alloc::string::String,
    /// Output only. Link used to play back the recording file in the browser. For
    /// example, `<https://drive.google.com/file/d/{$fileId}/view`.>
    #[prost(string, tag = "2")]
    pub export_uri: ::prost::alloc::string::String,
}
/// [Developer Preview](<https://developers.google.com/workspace/preview>).
/// Metadata for a transcript generated from a conference. It refers to the ASR
/// (Automatic Speech Recognition) result of user's speech during the conference.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transcript {
    /// Output only. Resource name of the transcript.
    /// Format: `conferenceRecords/{conference_record}/transcripts/{transcript}`,
    /// where `{transcript}` is a 1:1 mapping to each unique transcription session
    /// of the conference.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Current state.
    #[prost(enumeration = "transcript::State", tag = "3")]
    pub state: i32,
    /// Output only. Timestamp when the transcript started.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the transcript stopped.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "transcript::Destination", tags = "6")]
    pub destination: ::core::option::Option<transcript::Destination>,
}
/// Nested message and enum types in `Transcript`.
pub mod transcript {
    /// Current state of the transcript session.
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
        /// Default, never used.
        Unspecified = 0,
        /// An active transcript session has started.
        Started = 1,
        /// This transcript session has ended, but the transcript file hasn't been
        /// generated yet.
        Ended = 2,
        /// Transcript file is generated and ready to download.
        FileGenerated = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Started => "STARTED",
                State::Ended => "ENDED",
                State::FileGenerated => "FILE_GENERATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STARTED" => Some(Self::Started),
                "ENDED" => Some(Self::Ended),
                "FILE_GENERATED" => Some(Self::FileGenerated),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Output only. Where the Google Docs transcript is saved.
        #[prost(message, tag = "6")]
        DocsDestination(super::DocsDestination),
    }
}
/// Google Docs location where the transcript file is saved.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocsDestination {
    /// Output only. The document ID for the underlying Google Docs transcript
    /// file. For example, "1kuceFZohVoCh6FulBHxwy6I15Ogpc4hP". Use the
    /// `documents.get` method of the Google Docs API
    /// (<https://developers.google.com/docs/api/reference/rest/v1/documents/get>) to
    /// fetch the content.
    #[prost(string, tag = "1")]
    pub document: ::prost::alloc::string::String,
    /// Output only. URI for the Google Docs transcript file. Use
    /// `<https://docs.google.com/document/d/{$DocumentId}/view`> to browse the
    /// transcript in the browser.
    #[prost(string, tag = "2")]
    pub export_uri: ::prost::alloc::string::String,
}
/// [Developer Preview](<https://developers.google.com/workspace/preview>).
/// Single entry for one user’s speech during a transcript session.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranscriptEntry {
    /// Output only. Resource name of the entry. Format:
    /// "conferenceRecords/{conference_record}/transcripts/{transcript}/entries/{entry}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Refer to the participant who speaks.
    #[prost(string, tag = "2")]
    pub participant: ::prost::alloc::string::String,
    /// Output only. The transcribed text of the participant's voice, at maximum
    /// 10K words. Note that the limit is subject to change.
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    /// Output only. Language of spoken text, such as "en-US".
    /// IETF BCP 47 syntax (<https://tools.ietf.org/html/bcp47>)
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
    /// Output only. Timestamp when the transcript entry started.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the transcript entry ended.
    #[prost(message, optional, tag = "6")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request to create a space.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSpaceRequest {
    /// Space to be created. As of May 2023, the input space can be empty. Later on
    /// the input space can be non-empty when space configuration is introduced.
    #[prost(message, optional, tag = "1")]
    pub space: ::core::option::Option<Space>,
}
/// Request to get a space.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSpaceRequest {
    /// Required. Resource name of the space.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to update a space.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSpaceRequest {
    /// Required. Space to be updated.
    #[prost(message, optional, tag = "1")]
    pub space: ::core::option::Option<Space>,
    /// Optional. Field mask used to specify the fields to be updated in the space.
    /// If update_mask isn't provided, it defaults to '*' and updates all
    /// fields provided in the request, including deleting fields not set in the
    /// request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to end an ongoing conference of a space.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndActiveConferenceRequest {
    /// Required. Resource name of the space.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to get a conference record.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConferenceRecordRequest {
    /// Required. Resource name of the conference.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to fetch list of conference records per user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConferenceRecordsRequest {
    /// Optional. Maximum number of conference records to return. The service might
    /// return fewer than this value. If unspecified, at most 25 conference records
    /// are returned. The maximum value is 100; values above 100 are coerced to
    /// 100. Maximum might change in the future.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// Optional. Page token returned from previous List Call.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. User specified filtering condition in EBNF format. The following
    /// are the filterable fields:
    ///
    /// * `space.meeting_code`
    /// * `space.name`
    /// * `start_time`
    /// * `end_time`
    ///
    /// For example, `space.meeting_code = "abc-mnop-xyz"`.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
}
/// Response of ListConferenceRecords method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConferenceRecordsResponse {
    /// List of conferences in one page.
    #[prost(message, repeated, tag = "1")]
    pub conference_records: ::prost::alloc::vec::Vec<ConferenceRecord>,
    /// Token to be circulated back for further List call if current List does NOT
    /// include all the Conferences. Unset if all conferences have been returned.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to get a Participant.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParticipantRequest {
    /// Required. Resource name of the participant.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to fetch list of participant per conference.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListParticipantsRequest {
    /// Required. Format: `conferenceRecords/{conference_record}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of participants to return. The service might return fewer
    /// than this value.
    /// If unspecified, at most 100 participants are returned.
    /// The maximum value is 250; values above 250 are coerced to 250.
    /// Maximum might change in the future.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token returned from previous List Call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. User specified filtering condition in EBNF format. The following
    /// are the filterable fields:
    ///
    /// * `earliest_start_time`
    /// * `latest_end_time`
    ///
    /// For example, `latest_end_time IS NULL` returns active participants in
    /// the conference.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response of ListParticipants method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListParticipantsResponse {
    /// List of participants in one page.
    #[prost(message, repeated, tag = "1")]
    pub participants: ::prost::alloc::vec::Vec<Participant>,
    /// Token to be circulated back for further List call if current List doesn't
    /// include all the participants. Unset if all participants are returned.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total, exact number of `participants`. By default, this field isn't
    /// included in the response. Set the field mask in
    /// [SystemParameterContext](<https://cloud.google.com/apis/docs/system-parameters>)
    /// to receive this field in the response.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request to get a participant session.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParticipantSessionRequest {
    /// Required. Resource name of the participant.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to fetch list of participant sessions per conference record per
/// participant.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListParticipantSessionsRequest {
    /// Required. Format:
    /// `conferenceRecords/{conference_record}/participants/{participant}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of participant sessions to return. The service
    /// might return fewer than this value. If unspecified, at most 100
    /// participants are returned. The maximum value is 250; values above 250 are
    /// coerced to 250. Maximum might change in the future.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Page token returned from previous List Call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. User specified filtering condition in EBNF format. The following
    /// are the filterable fields:
    ///
    /// * `start_time`
    /// * `end_time`
    ///
    /// For example, `end_time IS NULL` returns active participant sessions in
    /// the conference record.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response of ListParticipants method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListParticipantSessionsResponse {
    /// List of participants in one page.
    #[prost(message, repeated, tag = "1")]
    pub participant_sessions: ::prost::alloc::vec::Vec<ParticipantSession>,
    /// Token to be circulated back for further List call if current List doesn't
    /// include all the participants. Unset if all participants are returned.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetRecording method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecordingRequest {
    /// Required. Resource name of the recording.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for ListRecordings method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRecordingsRequest {
    /// Required. Format: `conferenceRecords/{conference_record}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of recordings to return. The service might return fewer
    /// than this value.
    /// If unspecified, at most 10 recordings are returned.
    /// The maximum value is 100; values above 100 are coerced to 100.
    /// Maximum might change in the future.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token returned from previous List Call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for ListRecordings method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRecordingsResponse {
    /// List of recordings in one page.
    #[prost(message, repeated, tag = "1")]
    pub recordings: ::prost::alloc::vec::Vec<Recording>,
    /// Token to be circulated back for further List call if current List doesn't
    /// include all the recordings. Unset if all recordings are returned.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for GetTranscript method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTranscriptRequest {
    /// Required. Resource name of the transcript.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for ListTranscripts method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTranscriptsRequest {
    /// Required. Format: `conferenceRecords/{conference_record}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of transcripts to return. The service might return fewer
    /// than this value.
    /// If unspecified, at most 10 transcripts are returned.
    /// The maximum value is 100; values above 100 are coerced to 100.
    /// Maximum might change in the future.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token returned from previous List Call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for ListTranscripts method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTranscriptsResponse {
    /// List of transcripts in one page.
    #[prost(message, repeated, tag = "1")]
    pub transcripts: ::prost::alloc::vec::Vec<Transcript>,
    /// Token to be circulated back for further List call if current List doesn't
    /// include all the transcripts. Unset if all transcripts are returned.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for GetTranscriptEntry method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTranscriptEntryRequest {
    /// Required. Resource name of the `TranscriptEntry`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for ListTranscriptEntries method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTranscriptEntriesRequest {
    /// Required. Format:
    /// `conferenceRecords/{conference_record}/transcripts/{transcript}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of entries to return. The service might return fewer than
    /// this value.
    /// If unspecified, at most 10 entries are returned.
    /// The maximum value is 100; values above 100 are coerced to 100.
    /// Maximum might change in the future.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token returned from previous List Call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for ListTranscriptEntries method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTranscriptEntriesResponse {
    /// List of TranscriptEntries in one page.
    #[prost(message, repeated, tag = "1")]
    pub transcript_entries: ::prost::alloc::vec::Vec<TranscriptEntry>,
    /// Token to be circulated back for further List call if current List doesn't
    /// include all the transcript entries. Unset if all entries are returned.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod spaces_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// REST API for services dealing with spaces.
    #[derive(Debug, Clone)]
    pub struct SpacesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SpacesServiceClient<T>
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
        ) -> SpacesServiceClient<InterceptedService<T, F>>
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
            SpacesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Creates a space.
        pub async fn create_space(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSpaceRequest>,
        ) -> std::result::Result<tonic::Response<super::Space>, tonic::Status> {
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
                "/google.apps.meet.v2beta.SpacesService/CreateSpace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.SpacesService",
                        "CreateSpace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Gets a space by `space_id` or `meeting_code`.
        pub async fn get_space(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSpaceRequest>,
        ) -> std::result::Result<tonic::Response<super::Space>, tonic::Status> {
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
                "/google.apps.meet.v2beta.SpacesService/GetSpace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.apps.meet.v2beta.SpacesService", "GetSpace"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Updates a space.
        pub async fn update_space(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSpaceRequest>,
        ) -> std::result::Result<tonic::Response<super::Space>, tonic::Status> {
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
                "/google.apps.meet.v2beta.SpacesService/UpdateSpace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.SpacesService",
                        "UpdateSpace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Ends an active conference (if there is one).
        pub async fn end_active_conference(
            &mut self,
            request: impl tonic::IntoRequest<super::EndActiveConferenceRequest>,
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
                "/google.apps.meet.v2beta.SpacesService/EndActiveConference",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.SpacesService",
                        "EndActiveConference",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod conference_records_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// REST API for services dealing with conference records.
    #[derive(Debug, Clone)]
    pub struct ConferenceRecordsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConferenceRecordsServiceClient<T>
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
        ) -> ConferenceRecordsServiceClient<InterceptedService<T, F>>
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
            ConferenceRecordsServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Gets a conference record by conference ID.
        pub async fn get_conference_record(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConferenceRecordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConferenceRecord>,
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
                "/google.apps.meet.v2beta.ConferenceRecordsService/GetConferenceRecord",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.ConferenceRecordsService",
                        "GetConferenceRecord",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Lists the conference records by start time and in descending order.
        pub async fn list_conference_records(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConferenceRecordsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConferenceRecordsResponse>,
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
                "/google.apps.meet.v2beta.ConferenceRecordsService/ListConferenceRecords",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.ConferenceRecordsService",
                        "ListConferenceRecords",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Gets a participant by participant ID.
        pub async fn get_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetParticipantRequest>,
        ) -> std::result::Result<tonic::Response<super::Participant>, tonic::Status> {
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
                "/google.apps.meet.v2beta.ConferenceRecordsService/GetParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.ConferenceRecordsService",
                        "GetParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Lists the participants in a conference record, by default ordered by join
        /// time and in descending order. This API supports `fields` as standard
        /// parameters like every other API. However, when the `fields` request
        /// parameter is omitted, this API defaults to `'participants/*,
        /// next_page_token'`.
        pub async fn list_participants(
            &mut self,
            request: impl tonic::IntoRequest<super::ListParticipantsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListParticipantsResponse>,
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
                "/google.apps.meet.v2beta.ConferenceRecordsService/ListParticipants",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.ConferenceRecordsService",
                        "ListParticipants",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Gets a participant session by participant session ID.
        pub async fn get_participant_session(
            &mut self,
            request: impl tonic::IntoRequest<super::GetParticipantSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ParticipantSession>,
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
                "/google.apps.meet.v2beta.ConferenceRecordsService/GetParticipantSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.ConferenceRecordsService",
                        "GetParticipantSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Lists the participant sessions of a participant in a conference record, by
        /// default ordered by join time and in descending order. This API supports
        /// `fields` as standard parameters like every other API. However, when the
        /// `fields` request parameter is omitted this API defaults to
        /// `'participantsessions/*, next_page_token'`.
        pub async fn list_participant_sessions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListParticipantSessionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListParticipantSessionsResponse>,
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
                "/google.apps.meet.v2beta.ConferenceRecordsService/ListParticipantSessions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.ConferenceRecordsService",
                        "ListParticipantSessions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Gets a recording by recording ID.
        pub async fn get_recording(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRecordingRequest>,
        ) -> std::result::Result<tonic::Response<super::Recording>, tonic::Status> {
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
                "/google.apps.meet.v2beta.ConferenceRecordsService/GetRecording",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.ConferenceRecordsService",
                        "GetRecording",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Lists the recording resources from the conference record.
        pub async fn list_recordings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRecordingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRecordingsResponse>,
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
                "/google.apps.meet.v2beta.ConferenceRecordsService/ListRecordings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.ConferenceRecordsService",
                        "ListRecordings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Gets a transcript by transcript ID.
        pub async fn get_transcript(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTranscriptRequest>,
        ) -> std::result::Result<tonic::Response<super::Transcript>, tonic::Status> {
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
                "/google.apps.meet.v2beta.ConferenceRecordsService/GetTranscript",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.ConferenceRecordsService",
                        "GetTranscript",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Lists the set of transcripts from the conference record.
        pub async fn list_transcripts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTranscriptsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTranscriptsResponse>,
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
                "/google.apps.meet.v2beta.ConferenceRecordsService/ListTranscripts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.ConferenceRecordsService",
                        "ListTranscripts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Gets a `TranscriptEntry` resource by entry ID.
        ///
        /// Note: The transcript entries returned by the Google Meet API might not
        /// match the transcription found in the Google Docs transcript file. This can
        /// occur when the Google Docs transcript file is modified after generation.
        pub async fn get_transcript_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTranscriptEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TranscriptEntry>,
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
                "/google.apps.meet.v2beta.ConferenceRecordsService/GetTranscriptEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.ConferenceRecordsService",
                        "GetTranscriptEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// [Developer Preview](https://developers.google.com/workspace/preview).
        /// Lists the structured transcript entries per transcript. By default, ordered
        /// by start time and in ascending order.
        ///
        /// Note: The transcript entries returned by the Google Meet API might not
        /// match the transcription found in the Google Docs transcript file. This can
        /// occur when the Google Docs transcript file is modified after generation.
        pub async fn list_transcript_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTranscriptEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTranscriptEntriesResponse>,
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
                "/google.apps.meet.v2beta.ConferenceRecordsService/ListTranscriptEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.meet.v2beta.ConferenceRecordsService",
                        "ListTranscriptEntries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
