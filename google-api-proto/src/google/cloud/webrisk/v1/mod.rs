/// Describes an API diff request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeThreatListDiffRequest {
    /// Required. The threat list to update. Only a single ThreatType should be
    /// specified per request. If you want to handle multiple ThreatTypes, you must
    /// make one request per ThreatType.
    #[prost(enumeration = "ThreatType", tag = "1")]
    pub threat_type: i32,
    /// The current version token of the client for the requested list (the
    /// client version that was received from the last successful diff).
    /// If the client does not have a version token (this is the first time calling
    /// ComputeThreatListDiff), this may be left empty and a full database
    /// snapshot will be returned.
    #[prost(bytes = "bytes", tag = "2")]
    pub version_token: ::prost::bytes::Bytes,
    /// Required. The constraints associated with this request.
    #[prost(message, optional, tag = "3")]
    pub constraints: ::core::option::Option<
        compute_threat_list_diff_request::Constraints,
    >,
}
/// Nested message and enum types in `ComputeThreatListDiffRequest`.
pub mod compute_threat_list_diff_request {
    /// The constraints for this diff.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Constraints {
        /// The maximum size in number of entries. The diff will not contain more
        /// entries than this value.  This should be a power of 2 between 2**10 and
        /// 2**20.  If zero, no diff size limit is set.
        #[prost(int32, tag = "1")]
        pub max_diff_entries: i32,
        /// Sets the maximum number of entries that the client is willing to have
        /// in the local database. This should be a power of 2 between 2**10 and
        /// 2**20. If zero, no database size limit is set.
        #[prost(int32, tag = "2")]
        pub max_database_entries: i32,
        /// The compression types supported by the client.
        #[prost(enumeration = "super::CompressionType", repeated, tag = "3")]
        pub supported_compressions: ::prost::alloc::vec::Vec<i32>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeThreatListDiffResponse {
    /// The type of response. This may indicate that an action must be taken by the
    /// client when the response is received.
    #[prost(enumeration = "compute_threat_list_diff_response::ResponseType", tag = "4")]
    pub response_type: i32,
    /// A set of entries to add to a local threat type's list.
    #[prost(message, optional, tag = "5")]
    pub additions: ::core::option::Option<ThreatEntryAdditions>,
    /// A set of entries to remove from a local threat type's list.
    /// This field may be empty.
    #[prost(message, optional, tag = "6")]
    pub removals: ::core::option::Option<ThreatEntryRemovals>,
    /// The new opaque client version token. This should be retained by the client
    /// and passed into the next call of ComputeThreatListDiff as 'version_token'.
    /// A separate version token should be stored and used for each threatList.
    #[prost(bytes = "bytes", tag = "7")]
    pub new_version_token: ::prost::bytes::Bytes,
    /// The expected SHA256 hash of the client state; that is, of the sorted list
    /// of all hashes present in the database after applying the provided diff.
    /// If the client state doesn't match the expected state, the client must
    /// discard this diff and retry later.
    #[prost(message, optional, tag = "8")]
    pub checksum: ::core::option::Option<compute_threat_list_diff_response::Checksum>,
    /// The soonest the client should wait before issuing any diff
    /// request. Querying sooner is unlikely to produce a meaningful diff.
    /// Waiting longer is acceptable considering the use case.
    /// If this field is not set clients may update as soon as they want.
    #[prost(message, optional, tag = "2")]
    pub recommended_next_diff: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `ComputeThreatListDiffResponse`.
pub mod compute_threat_list_diff_response {
    /// The expected state of a client's local database.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Checksum {
        /// The SHA256 hash of the client state; that is, of the sorted list of all
        /// hashes present in the database.
        #[prost(bytes = "bytes", tag = "1")]
        pub sha256: ::prost::bytes::Bytes,
    }
    /// The type of response sent to the client.
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
    pub enum ResponseType {
        /// Unknown.
        Unspecified = 0,
        /// Partial updates are applied to the client's existing local database.
        Diff = 1,
        /// Full updates resets the client's entire local database. This means
        /// that either the client had no state, was seriously out-of-date,
        /// or the client is believed to be corrupt.
        Reset = 2,
    }
    impl ResponseType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResponseType::Unspecified => "RESPONSE_TYPE_UNSPECIFIED",
                ResponseType::Diff => "DIFF",
                ResponseType::Reset => "RESET",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESPONSE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DIFF" => Some(Self::Diff),
                "RESET" => Some(Self::Reset),
                _ => None,
            }
        }
    }
}
/// Request to check URI entries against threatLists.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchUrisRequest {
    /// Required. The URI to be checked for matches.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// Required. The ThreatLists to search in. Multiple ThreatLists may be
    /// specified.
    #[prost(enumeration = "ThreatType", repeated, packed = "false", tag = "2")]
    pub threat_types: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchUrisResponse {
    /// The threat list matches. This might be empty if the URI is on no list.
    #[prost(message, optional, tag = "1")]
    pub threat: ::core::option::Option<search_uris_response::ThreatUri>,
}
/// Nested message and enum types in `SearchUrisResponse`.
pub mod search_uris_response {
    /// Contains threat information on a matching uri.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThreatUri {
        /// The ThreatList this threat belongs to.
        #[prost(enumeration = "super::ThreatType", repeated, tag = "1")]
        pub threat_types: ::prost::alloc::vec::Vec<i32>,
        /// The cache lifetime for the returned match. Clients must not cache this
        /// response past this timestamp to avoid false positives.
        #[prost(message, optional, tag = "2")]
        pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    }
}
/// Request to return full hashes matched by the provided hash prefixes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchHashesRequest {
    /// A hash prefix, consisting of the most significant 4-32 bytes of a SHA256
    /// hash. For JSON requests, this field is base64-encoded.
    /// Note that if this parameter is provided by a URI, it must be encoded using
    /// the web safe base64 variant (RFC 4648).
    #[prost(bytes = "bytes", tag = "1")]
    pub hash_prefix: ::prost::bytes::Bytes,
    /// Required. The ThreatLists to search in. Multiple ThreatLists may be
    /// specified.
    #[prost(enumeration = "ThreatType", repeated, packed = "false", tag = "2")]
    pub threat_types: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchHashesResponse {
    /// The full hashes that matched the requested prefixes.
    /// The hash will be populated in the key.
    #[prost(message, repeated, tag = "1")]
    pub threats: ::prost::alloc::vec::Vec<search_hashes_response::ThreatHash>,
    /// For requested entities that did not match the threat list, how long to
    /// cache the response until.
    #[prost(message, optional, tag = "2")]
    pub negative_expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `SearchHashesResponse`.
pub mod search_hashes_response {
    /// Contains threat information on a matching hash.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThreatHash {
        /// The ThreatList this threat belongs to.
        /// This must contain at least one entry.
        #[prost(enumeration = "super::ThreatType", repeated, tag = "1")]
        pub threat_types: ::prost::alloc::vec::Vec<i32>,
        /// A 32 byte SHA256 hash. This field is in binary format. For JSON
        /// requests, hashes are base64-encoded.
        #[prost(bytes = "bytes", tag = "2")]
        pub hash: ::prost::bytes::Bytes,
        /// The cache lifetime for the returned match. Clients must not cache this
        /// response past this timestamp to avoid false positives.
        #[prost(message, optional, tag = "3")]
        pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    }
}
/// Contains the set of entries to add to a local database.
/// May contain a combination of compressed and raw data in a single response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreatEntryAdditions {
    /// The raw SHA256-formatted entries.
    /// Repeated to allow returning sets of hashes with different prefix sizes.
    #[prost(message, repeated, tag = "1")]
    pub raw_hashes: ::prost::alloc::vec::Vec<RawHashes>,
    /// The encoded 4-byte prefixes of SHA256-formatted entries, using a
    /// Golomb-Rice encoding. The hashes are converted to uint32, sorted in
    /// ascending order, then delta encoded and stored as encoded_data.
    #[prost(message, optional, tag = "2")]
    pub rice_hashes: ::core::option::Option<RiceDeltaEncoding>,
}
/// Contains the set of entries to remove from a local database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreatEntryRemovals {
    /// The raw removal indices for a local list.
    #[prost(message, optional, tag = "1")]
    pub raw_indices: ::core::option::Option<RawIndices>,
    /// The encoded local, lexicographically-sorted list indices, using a
    /// Golomb-Rice encoding. Used for sending compressed removal indices. The
    /// removal indices (uint32) are sorted in ascending order, then delta encoded
    /// and stored as encoded_data.
    #[prost(message, optional, tag = "2")]
    pub rice_indices: ::core::option::Option<RiceDeltaEncoding>,
}
/// A set of raw indices to remove from a local list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawIndices {
    /// The indices to remove from a lexicographically-sorted local list.
    #[prost(int32, repeated, tag = "1")]
    pub indices: ::prost::alloc::vec::Vec<i32>,
}
/// The uncompressed threat entries in hash format.
/// Hashes can be anywhere from 4 to 32 bytes in size. A large majority are 4
/// bytes, but some hashes are lengthened if they collide with the hash of a
/// popular URI.
///
/// Used for sending ThreatEntryAdditons to clients that do not support
/// compression, or when sending non-4-byte hashes to clients that do support
/// compression.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawHashes {
    /// The number of bytes for each prefix encoded below.  This field can be
    /// anywhere from 4 (shortest prefix) to 32 (full SHA256 hash).
    /// In practice this is almost always 4, except in exceptional circumstances.
    #[prost(int32, tag = "1")]
    pub prefix_size: i32,
    /// The hashes, in binary format, concatenated into one long string. Hashes are
    /// sorted in lexicographic order. For JSON API users, hashes are
    /// base64-encoded.
    #[prost(bytes = "bytes", tag = "2")]
    pub raw_hashes: ::prost::bytes::Bytes,
}
/// The Rice-Golomb encoded data. Used for sending compressed 4-byte hashes or
/// compressed removal indices.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RiceDeltaEncoding {
    /// The offset of the first entry in the encoded data, or, if only a single
    /// integer was encoded, that single integer's value. If the field is empty or
    /// missing, assume zero.
    #[prost(int64, tag = "1")]
    pub first_value: i64,
    /// The Golomb-Rice parameter, which is a number between 2 and 28. This field
    /// is missing (that is, zero) if `num_entries` is zero.
    #[prost(int32, tag = "2")]
    pub rice_parameter: i32,
    /// The number of entries that are delta encoded in the encoded data. If only a
    /// single integer was encoded, this will be zero and the single value will be
    /// stored in `first_value`.
    #[prost(int32, tag = "3")]
    pub entry_count: i32,
    /// The encoded deltas that are encoded using the Golomb-Rice coder.
    #[prost(bytes = "bytes", tag = "4")]
    pub encoded_data: ::prost::bytes::Bytes,
}
/// Wraps a URI that might be displaying malicious content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Submission {
    /// Required. The URI that is being reported for malicious content to be
    /// analyzed.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// Output only. ThreatTypes found to be associated with the submitted URI
    /// after reviewing it. This might be empty if the URI was not added to any
    /// list.
    #[prost(enumeration = "ThreatType", repeated, packed = "false", tag = "2")]
    pub threat_types: ::prost::alloc::vec::Vec<i32>,
}
/// Context about the submission including the type of abuse found on the URI and
/// supporting details.
/// option (google.api.message_visibility).restriction = "TRUSTED_TESTER";
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreatInfo {
    /// The type of abuse.
    #[prost(enumeration = "threat_info::AbuseType", tag = "1")]
    pub abuse_type: i32,
    /// Confidence that the URI is unsafe.
    #[prost(message, optional, tag = "2")]
    pub threat_confidence: ::core::option::Option<threat_info::Confidence>,
    /// Context about why the URI is unsafe.
    #[prost(message, optional, tag = "3")]
    pub threat_justification: ::core::option::Option<threat_info::ThreatJustification>,
}
/// Nested message and enum types in `ThreatInfo`.
pub mod threat_info {
    /// Confidence that a URI is unsafe.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Confidence {
        #[prost(oneof = "confidence::Value", tags = "1, 2")]
        pub value: ::core::option::Option<confidence::Value>,
    }
    /// Nested message and enum types in `Confidence`.
    pub mod confidence {
        /// Enum representation of confidence.
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
        pub enum ConfidenceLevel {
            /// Default.
            Unspecified = 0,
            /// Less than 60% confidence that the URI is unsafe.
            Low = 1,
            /// Between 60% and 80% confidence that the URI is unsafe.
            Medium = 2,
            /// Greater than 80% confidence that the URI is unsafe.
            High = 3,
        }
        impl ConfidenceLevel {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ConfidenceLevel::Unspecified => "CONFIDENCE_LEVEL_UNSPECIFIED",
                    ConfidenceLevel::Low => "LOW",
                    ConfidenceLevel::Medium => "MEDIUM",
                    ConfidenceLevel::High => "HIGH",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "CONFIDENCE_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
                    "LOW" => Some(Self::Low),
                    "MEDIUM" => Some(Self::Medium),
                    "HIGH" => Some(Self::High),
                    _ => None,
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            /// A decimal representation of confidence in the range of 0
            /// to 1 where 0 indicates no confidence and 1 indicates
            /// complete confidence.
            #[prost(float, tag = "1")]
            Score(f32),
            /// Enum representation of confidence.
            #[prost(enumeration = "ConfidenceLevel", tag = "2")]
            Level(i32),
        }
    }
    /// Context about why the URI is unsafe.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThreatJustification {
        /// Labels associated with this URI that explain how it was classified.
        #[prost(
            enumeration = "threat_justification::JustificationLabel",
            repeated,
            tag = "1"
        )]
        pub labels: ::prost::alloc::vec::Vec<i32>,
        /// Free-form context on why this URI is unsafe.
        #[prost(string, repeated, tag = "2")]
        pub comments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Nested message and enum types in `ThreatJustification`.
    pub mod threat_justification {
        /// Labels that explain how the URI was classified.
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
        pub enum JustificationLabel {
            /// Default.
            Unspecified = 0,
            /// The submitter manually verified that the submission is unsafe.
            ManualVerification = 1,
            /// The submitter received the submission from an end user.
            UserReport = 2,
            /// The submitter received the submission from an automated system.
            AutomatedReport = 3,
        }
        impl JustificationLabel {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    JustificationLabel::Unspecified => "JUSTIFICATION_LABEL_UNSPECIFIED",
                    JustificationLabel::ManualVerification => "MANUAL_VERIFICATION",
                    JustificationLabel::UserReport => "USER_REPORT",
                    JustificationLabel::AutomatedReport => "AUTOMATED_REPORT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "JUSTIFICATION_LABEL_UNSPECIFIED" => Some(Self::Unspecified),
                    "MANUAL_VERIFICATION" => Some(Self::ManualVerification),
                    "USER_REPORT" => Some(Self::UserReport),
                    "AUTOMATED_REPORT" => Some(Self::AutomatedReport),
                    _ => None,
                }
            }
        }
    }
    /// The abuse type found on the URI.
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
    pub enum AbuseType {
        /// Default.
        Unspecified = 0,
        /// The URI contains malware.
        Malware = 1,
        /// The URI contains social engineering.
        SocialEngineering = 2,
        /// The URI contains unwanted software.
        UnwantedSoftware = 3,
    }
    impl AbuseType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AbuseType::Unspecified => "ABUSE_TYPE_UNSPECIFIED",
                AbuseType::Malware => "MALWARE",
                AbuseType::SocialEngineering => "SOCIAL_ENGINEERING",
                AbuseType::UnwantedSoftware => "UNWANTED_SOFTWARE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ABUSE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "MALWARE" => Some(Self::Malware),
                "SOCIAL_ENGINEERING" => Some(Self::SocialEngineering),
                "UNWANTED_SOFTWARE" => Some(Self::UnwantedSoftware),
                _ => None,
            }
        }
    }
}
/// Details about how the threat was discovered.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreatDiscovery {
    /// Platform on which the threat was discovered.
    #[prost(enumeration = "threat_discovery::Platform", tag = "1")]
    pub platform: i32,
    /// CLDR region code of the countries/regions the URI poses a threat ordered
    /// from most impact to least impact. Example: "US" for United States.
    #[prost(string, repeated, tag = "2")]
    pub region_codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ThreatDiscovery`.
pub mod threat_discovery {
    /// Platform types.
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
    pub enum Platform {
        /// Default.
        Unspecified = 0,
        /// General Android platform.
        Android = 1,
        /// General iOS platform.
        Ios = 2,
        /// General macOS platform.
        Macos = 3,
        /// General Windows platform.
        Windows = 4,
    }
    impl Platform {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Platform::Unspecified => "PLATFORM_UNSPECIFIED",
                Platform::Android => "ANDROID",
                Platform::Ios => "IOS",
                Platform::Macos => "MACOS",
                Platform::Windows => "WINDOWS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PLATFORM_UNSPECIFIED" => Some(Self::Unspecified),
                "ANDROID" => Some(Self::Android),
                "IOS" => Some(Self::Ios),
                "MACOS" => Some(Self::Macos),
                "WINDOWS" => Some(Self::Windows),
                _ => None,
            }
        }
    }
}
/// Request to send a potentially phishy URI to WebRisk.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSubmissionRequest {
    /// Required. The name of the project that is making the submission. This
    /// string is in the format "projects/{project_number}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The submission that contains the content of the phishing report.
    #[prost(message, optional, tag = "2")]
    pub submission: ::core::option::Option<Submission>,
}
/// Request to send a potentially malicious URI to WebRisk.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitUriRequest {
    /// Required. The name of the project that is making the submission. This
    /// string is in the format "projects/{project_number}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The submission that contains the URI to be scanned.
    #[prost(message, optional, tag = "2")]
    pub submission: ::core::option::Option<Submission>,
    /// Provides additional information about the submission.
    #[prost(message, optional, tag = "3")]
    pub threat_info: ::core::option::Option<ThreatInfo>,
    /// Provides additional information about how the submission was discovered.
    #[prost(message, optional, tag = "4")]
    pub threat_discovery: ::core::option::Option<ThreatDiscovery>,
}
/// Metadata for the Submit URI long-running operation.
/// option (google.api.message_visibility).restriction = "TRUSTED_TESTER";
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitUriMetadata {
    /// The state of the operation.
    #[prost(enumeration = "submit_uri_metadata::State", tag = "1")]
    pub state: i32,
    /// Creation time of the operation.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Latest update time of the operation.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `SubmitUriMetadata`.
pub mod submit_uri_metadata {
    /// Enum that represents the state of the long-running operation.
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
        /// Default unspecified state.
        Unspecified = 0,
        /// The operation is currently running.
        Running = 1,
        /// The operation finished with a success status.
        Succeeded = 2,
        /// The operation was cancelled.
        Cancelled = 3,
        /// The operation finished with a failure status.
        Failed = 4,
        /// The operation was closed with no action taken.
        Closed = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Running => "RUNNING",
                State::Succeeded => "SUCCEEDED",
                State::Cancelled => "CANCELLED",
                State::Failed => "FAILED",
                State::Closed => "CLOSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "CANCELLED" => Some(Self::Cancelled),
                "FAILED" => Some(Self::Failed),
                "CLOSED" => Some(Self::Closed),
                _ => None,
            }
        }
    }
}
/// The type of threat. This maps directly to the threat list a threat may
/// belong to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ThreatType {
    /// No entries should match this threat type. This threat type is unused.
    Unspecified = 0,
    /// Malware targeting any platform.
    Malware = 1,
    /// Social engineering targeting any platform.
    SocialEngineering = 2,
    /// Unwanted software targeting any platform.
    UnwantedSoftware = 3,
    /// A list of extended coverage social engineering URIs targeting any
    /// platform.
    SocialEngineeringExtendedCoverage = 4,
}
impl ThreatType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ThreatType::Unspecified => "THREAT_TYPE_UNSPECIFIED",
            ThreatType::Malware => "MALWARE",
            ThreatType::SocialEngineering => "SOCIAL_ENGINEERING",
            ThreatType::UnwantedSoftware => "UNWANTED_SOFTWARE",
            ThreatType::SocialEngineeringExtendedCoverage => {
                "SOCIAL_ENGINEERING_EXTENDED_COVERAGE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "THREAT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "MALWARE" => Some(Self::Malware),
            "SOCIAL_ENGINEERING" => Some(Self::SocialEngineering),
            "UNWANTED_SOFTWARE" => Some(Self::UnwantedSoftware),
            "SOCIAL_ENGINEERING_EXTENDED_COVERAGE" => {
                Some(Self::SocialEngineeringExtendedCoverage)
            }
            _ => None,
        }
    }
}
/// The ways in which threat entry sets can be compressed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompressionType {
    /// Unknown.
    Unspecified = 0,
    /// Raw, uncompressed data.
    Raw = 1,
    /// Rice-Golomb encoded data.
    Rice = 2,
}
impl CompressionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CompressionType::Unspecified => "COMPRESSION_TYPE_UNSPECIFIED",
            CompressionType::Raw => "RAW",
            CompressionType::Rice => "RICE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMPRESSION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "RAW" => Some(Self::Raw),
            "RICE" => Some(Self::Rice),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod web_risk_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Web Risk API defines an interface to detect malicious URLs on your
    /// website and in client applications.
    #[derive(Debug, Clone)]
    pub struct WebRiskServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> WebRiskServiceClient<T>
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
        ) -> WebRiskServiceClient<InterceptedService<T, F>>
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
            WebRiskServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets the most recent threat list diffs. These diffs should be applied to
        /// a local database of hashes to keep it up-to-date. If the local database is
        /// empty or excessively out-of-date, a complete snapshot of the database will
        /// be returned. This Method only updates a single ThreatList at a time. To
        /// update multiple ThreatList databases, this method needs to be called once
        /// for each list.
        pub async fn compute_threat_list_diff(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeThreatListDiffRequest>,
        ) -> Result<
            tonic::Response<super::ComputeThreatListDiffResponse>,
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
                "/google.cloud.webrisk.v1.WebRiskService/ComputeThreatListDiff",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// This method is used to check whether a URI is on a given threatList.
        /// Multiple threatLists may be searched in a single query.
        /// The response will list all requested threatLists the URI was found to
        /// match. If the URI is not found on any of the requested ThreatList an
        /// empty response will be returned.
        pub async fn search_uris(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchUrisRequest>,
        ) -> Result<tonic::Response<super::SearchUrisResponse>, tonic::Status> {
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
                "/google.cloud.webrisk.v1.WebRiskService/SearchUris",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the full hashes that match the requested hash prefix.
        /// This is used after a hash prefix is looked up in a threatList
        /// and there is a match. The client side threatList only holds partial hashes
        /// so the client must query this method to determine if there is a full
        /// hash match of a threat.
        pub async fn search_hashes(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchHashesRequest>,
        ) -> Result<tonic::Response<super::SearchHashesResponse>, tonic::Status> {
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
                "/google.cloud.webrisk.v1.WebRiskService/SearchHashes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a Submission of a URI suspected of containing phishing content to
        /// be reviewed. If the result verifies the existence of malicious phishing
        /// content, the site will be added to the [Google's Social Engineering
        /// lists](https://support.google.com/webmasters/answer/6350487/) in order to
        /// protect users that could get exposed to this threat in the future. Only
        /// allowlisted projects can use this method during Early Access. Please reach
        /// out to Sales or your customer engineer to obtain access.
        pub async fn create_submission(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSubmissionRequest>,
        ) -> Result<tonic::Response<super::Submission>, tonic::Status> {
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
                "/google.cloud.webrisk.v1.WebRiskService/CreateSubmission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Submits a URI suspected of containing malicious content to be reviewed.
        /// Returns a google.longrunning.Operation which, once the review is complete,
        /// is updated with its result. You can use the [Pub/Sub API]
        /// (https://cloud.google.com/pubsub) to receive notifications for the returned
        /// Operation. If the result verifies the existence of malicious content, the
        /// site will be added to the [Google's Social Engineering lists]
        /// (https://support.google.com/webmasters/answer/6350487/) in order to
        /// protect users that could get exposed to this threat in the future. Only
        /// allowlisted projects can use this method during Early Access. Please reach
        /// out to Sales or your customer engineer to obtain access.
        pub async fn submit_uri(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitUriRequest>,
        ) -> Result<
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
                "/google.cloud.webrisk.v1.WebRiskService/SubmitUri",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
