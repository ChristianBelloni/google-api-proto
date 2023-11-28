/// Container for enum that determines if the described artifact is a resource
/// or a field, and if it is a field, when it segments search queries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAds360FieldCategoryEnum {}
/// Nested message and enum types in `SearchAds360FieldCategoryEnum`.
pub mod search_ads360_field_category_enum {
    /// The category of the artifact.
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
    pub enum SearchAds360FieldCategory {
        /// Unspecified
        Unspecified = 0,
        /// Unknown
        Unknown = 1,
        /// The described artifact is a resource.
        Resource = 2,
        /// The described artifact is a field and is an attribute of a resource.
        /// Including a resource attribute field in a query may segment the query if
        /// the resource to which it is attributed segments the resource found in
        /// the FROM clause.
        Attribute = 3,
        /// The described artifact is a field and always segments search queries.
        Segment = 5,
        /// The described artifact is a field and is a metric. It never segments
        /// search queries.
        Metric = 6,
    }
    impl SearchAds360FieldCategory {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SearchAds360FieldCategory::Unspecified => "UNSPECIFIED",
                SearchAds360FieldCategory::Unknown => "UNKNOWN",
                SearchAds360FieldCategory::Resource => "RESOURCE",
                SearchAds360FieldCategory::Attribute => "ATTRIBUTE",
                SearchAds360FieldCategory::Segment => "SEGMENT",
                SearchAds360FieldCategory::Metric => "METRIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "RESOURCE" => Some(Self::Resource),
                "ATTRIBUTE" => Some(Self::Attribute),
                "SEGMENT" => Some(Self::Segment),
                "METRIC" => Some(Self::Metric),
                _ => None,
            }
        }
    }
}
/// Container holding the various data types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAds360FieldDataTypeEnum {}
/// Nested message and enum types in `SearchAds360FieldDataTypeEnum`.
pub mod search_ads360_field_data_type_enum {
    /// These are the various types a SearchAds360Service artifact may take on.
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
    pub enum SearchAds360FieldDataType {
        /// Unspecified
        Unspecified = 0,
        /// Unknown
        Unknown = 1,
        /// Maps to google.protobuf.BoolValue
        ///
        /// Applicable operators:  =, !=
        Boolean = 2,
        /// Maps to google.protobuf.StringValue. It can be compared using the set of
        /// operators specific to dates however.
        ///
        /// Applicable operators:  =, <, >, <=, >=, BETWEEN, DURING, and IN
        Date = 3,
        /// Maps to google.protobuf.DoubleValue
        ///
        /// Applicable operators:  =, !=, <, >, IN, NOT IN
        Double = 4,
        /// Maps to an enum. It's specific definition can be found at type_url.
        ///
        /// Applicable operators:  =, !=, IN, NOT IN
        Enum = 5,
        /// Maps to google.protobuf.FloatValue
        ///
        /// Applicable operators:  =, !=, <, >, IN, NOT IN
        Float = 6,
        /// Maps to google.protobuf.Int32Value
        ///
        /// Applicable operators:  =, !=, <, >, <=, >=, BETWEEN, IN, NOT IN
        Int32 = 7,
        /// Maps to google.protobuf.Int64Value
        ///
        /// Applicable operators:  =, !=, <, >, <=, >=, BETWEEN, IN, NOT IN
        Int64 = 8,
        /// Maps to a protocol buffer message type. The data type's details can be
        /// found in type_url.
        ///
        /// No operators work with MESSAGE fields.
        Message = 9,
        /// Maps to google.protobuf.StringValue. Represents the resource name
        /// (unique id) of a resource or one of its foreign keys.
        ///
        /// No operators work with RESOURCE_NAME fields.
        ResourceName = 10,
        /// Maps to google.protobuf.StringValue.
        ///
        /// Applicable operators:  =, !=, LIKE, NOT LIKE, IN, NOT IN
        String = 11,
        /// Maps to google.protobuf.UInt64Value
        ///
        /// Applicable operators:  =, !=, <, >, <=, >=, BETWEEN, IN, NOT IN
        Uint64 = 12,
    }
    impl SearchAds360FieldDataType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SearchAds360FieldDataType::Unspecified => "UNSPECIFIED",
                SearchAds360FieldDataType::Unknown => "UNKNOWN",
                SearchAds360FieldDataType::Boolean => "BOOLEAN",
                SearchAds360FieldDataType::Date => "DATE",
                SearchAds360FieldDataType::Double => "DOUBLE",
                SearchAds360FieldDataType::Enum => "ENUM",
                SearchAds360FieldDataType::Float => "FLOAT",
                SearchAds360FieldDataType::Int32 => "INT32",
                SearchAds360FieldDataType::Int64 => "INT64",
                SearchAds360FieldDataType::Message => "MESSAGE",
                SearchAds360FieldDataType::ResourceName => "RESOURCE_NAME",
                SearchAds360FieldDataType::String => "STRING",
                SearchAds360FieldDataType::Uint64 => "UINT64",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "BOOLEAN" => Some(Self::Boolean),
                "DATE" => Some(Self::Date),
                "DOUBLE" => Some(Self::Double),
                "ENUM" => Some(Self::Enum),
                "FLOAT" => Some(Self::Float),
                "INT32" => Some(Self::Int32),
                "INT64" => Some(Self::Int64),
                "MESSAGE" => Some(Self::Message),
                "RESOURCE_NAME" => Some(Self::ResourceName),
                "STRING" => Some(Self::String),
                "UINT64" => Some(Self::Uint64),
                _ => None,
            }
        }
    }
}
/// Container for enum describing types of payable and free interactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionEventTypeEnum {}
/// Nested message and enum types in `InteractionEventTypeEnum`.
pub mod interaction_event_type_enum {
    /// Enum describing possible types of payable and free interactions.
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
    pub enum InteractionEventType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Click to site. In most cases, this interaction navigates to an external
        /// location, usually the advertiser's landing page. This is also the default
        /// InteractionEventType for click events.
        Click = 2,
        /// The user's expressed intent to engage with the ad in-place.
        Engagement = 3,
        /// User viewed a video ad.
        VideoView = 4,
        /// The default InteractionEventType for ad conversion events.
        /// This is used when an ad conversion row does NOT indicate
        /// that the free interactions (for example, the ad conversions)
        /// should be 'promoted' and reported as part of the core metrics.
        /// These are simply other (ad) conversions.
        None = 5,
    }
    impl InteractionEventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InteractionEventType::Unspecified => "UNSPECIFIED",
                InteractionEventType::Unknown => "UNKNOWN",
                InteractionEventType::Click => "CLICK",
                InteractionEventType::Engagement => "ENGAGEMENT",
                InteractionEventType::VideoView => "VIDEO_VIEW",
                InteractionEventType::None => "NONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "CLICK" => Some(Self::Click),
                "ENGAGEMENT" => Some(Self::Engagement),
                "VIDEO_VIEW" => Some(Self::VideoView),
                "NONE" => Some(Self::None),
                _ => None,
            }
        }
    }
}
/// The relative performance compared to other advertisers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QualityScoreBucketEnum {}
/// Nested message and enum types in `QualityScoreBucketEnum`.
pub mod quality_score_bucket_enum {
    /// Enum listing the possible quality score buckets.
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
    pub enum QualityScoreBucket {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Quality of the creative is below average.
        BelowAverage = 2,
        /// Quality of the creative is average.
        Average = 3,
        /// Quality of the creative is above average.
        AboveAverage = 4,
    }
    impl QualityScoreBucket {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                QualityScoreBucket::Unspecified => "UNSPECIFIED",
                QualityScoreBucket::Unknown => "UNKNOWN",
                QualityScoreBucket::BelowAverage => "BELOW_AVERAGE",
                QualityScoreBucket::Average => "AVERAGE",
                QualityScoreBucket::AboveAverage => "ABOVE_AVERAGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "BELOW_AVERAGE" => Some(Self::BelowAverage),
                "AVERAGE" => Some(Self::Average),
                "ABOVE_AVERAGE" => Some(Self::AboveAverage),
                _ => None,
            }
        }
    }
}
/// Container for enum describing the type of demographic age ranges.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgeRangeTypeEnum {}
/// Nested message and enum types in `AgeRangeTypeEnum`.
pub mod age_range_type_enum {
    /// The type of demographic age ranges (for example, between 18 and 24 years
    /// old).
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
    pub enum AgeRangeType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Between 18 and 24 years old.
        AgeRange1824 = 503001,
        /// Between 25 and 34 years old.
        AgeRange2534 = 503002,
        /// Between 35 and 44 years old.
        AgeRange3544 = 503003,
        /// Between 45 and 54 years old.
        AgeRange4554 = 503004,
        /// Between 55 and 64 years old.
        AgeRange5564 = 503005,
        /// 65 years old and beyond.
        AgeRange65Up = 503006,
        /// Undetermined age range.
        AgeRangeUndetermined = 503999,
    }
    impl AgeRangeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AgeRangeType::Unspecified => "UNSPECIFIED",
                AgeRangeType::Unknown => "UNKNOWN",
                AgeRangeType::AgeRange1824 => "AGE_RANGE_18_24",
                AgeRangeType::AgeRange2534 => "AGE_RANGE_25_34",
                AgeRangeType::AgeRange3544 => "AGE_RANGE_35_44",
                AgeRangeType::AgeRange4554 => "AGE_RANGE_45_54",
                AgeRangeType::AgeRange5564 => "AGE_RANGE_55_64",
                AgeRangeType::AgeRange65Up => "AGE_RANGE_65_UP",
                AgeRangeType::AgeRangeUndetermined => "AGE_RANGE_UNDETERMINED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "AGE_RANGE_18_24" => Some(Self::AgeRange1824),
                "AGE_RANGE_25_34" => Some(Self::AgeRange2534),
                "AGE_RANGE_35_44" => Some(Self::AgeRange3544),
                "AGE_RANGE_45_54" => Some(Self::AgeRange4554),
                "AGE_RANGE_55_64" => Some(Self::AgeRange5564),
                "AGE_RANGE_65_UP" => Some(Self::AgeRange65Up),
                "AGE_RANGE_UNDETERMINED" => Some(Self::AgeRangeUndetermined),
                _ => None,
            }
        }
    }
}
/// Container for enumeration of days of the week, for example, "Monday".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DayOfWeekEnum {}
/// Nested message and enum types in `DayOfWeekEnum`.
pub mod day_of_week_enum {
    /// Enumerates days of the week, for example, "Monday".
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
    pub enum DayOfWeek {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Monday.
        Monday = 2,
        /// Tuesday.
        Tuesday = 3,
        /// Wednesday.
        Wednesday = 4,
        /// Thursday.
        Thursday = 5,
        /// Friday.
        Friday = 6,
        /// Saturday.
        Saturday = 7,
        /// Sunday.
        Sunday = 8,
    }
    impl DayOfWeek {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DayOfWeek::Unspecified => "UNSPECIFIED",
                DayOfWeek::Unknown => "UNKNOWN",
                DayOfWeek::Monday => "MONDAY",
                DayOfWeek::Tuesday => "TUESDAY",
                DayOfWeek::Wednesday => "WEDNESDAY",
                DayOfWeek::Thursday => "THURSDAY",
                DayOfWeek::Friday => "FRIDAY",
                DayOfWeek::Saturday => "SATURDAY",
                DayOfWeek::Sunday => "SUNDAY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "MONDAY" => Some(Self::Monday),
                "TUESDAY" => Some(Self::Tuesday),
                "WEDNESDAY" => Some(Self::Wednesday),
                "THURSDAY" => Some(Self::Thursday),
                "FRIDAY" => Some(Self::Friday),
                "SATURDAY" => Some(Self::Saturday),
                "SUNDAY" => Some(Self::Sunday),
                _ => None,
            }
        }
    }
}
/// Container for enumeration of Google Ads devices available for targeting.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceEnum {}
/// Nested message and enum types in `DeviceEnum`.
pub mod device_enum {
    /// Enumerates Google Ads devices available for targeting.
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
    pub enum Device {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Mobile devices with full browsers.
        Mobile = 2,
        /// Tablets with full browsers.
        Tablet = 3,
        /// Computers.
        Desktop = 4,
        /// Smart TVs and game consoles.
        ConnectedTv = 6,
        /// Other device types.
        Other = 5,
    }
    impl Device {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Device::Unspecified => "UNSPECIFIED",
                Device::Unknown => "UNKNOWN",
                Device::Mobile => "MOBILE",
                Device::Tablet => "TABLET",
                Device::Desktop => "DESKTOP",
                Device::ConnectedTv => "CONNECTED_TV",
                Device::Other => "OTHER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "MOBILE" => Some(Self::Mobile),
                "TABLET" => Some(Self::Tablet),
                "DESKTOP" => Some(Self::Desktop),
                "CONNECTED_TV" => Some(Self::ConnectedTv),
                "OTHER" => Some(Self::Other),
                _ => None,
            }
        }
    }
}
/// Container for enum describing the type of demographic genders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenderTypeEnum {}
/// Nested message and enum types in `GenderTypeEnum`.
pub mod gender_type_enum {
    /// The type of demographic genders (for example, female).
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
    pub enum GenderType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Male.
        Male = 10,
        /// Female.
        Female = 11,
        /// Undetermined gender.
        Undetermined = 20,
    }
    impl GenderType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GenderType::Unspecified => "UNSPECIFIED",
                GenderType::Unknown => "UNKNOWN",
                GenderType::Male => "MALE",
                GenderType::Female => "FEMALE",
                GenderType::Undetermined => "UNDETERMINED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "MALE" => Some(Self::Male),
                "FEMALE" => Some(Self::Female),
                "UNDETERMINED" => Some(Self::Undetermined),
                _ => None,
            }
        }
    }
}
/// Message describing Keyword match types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordMatchTypeEnum {}
/// Nested message and enum types in `KeywordMatchTypeEnum`.
pub mod keyword_match_type_enum {
    /// Possible Keyword match types.
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
    pub enum KeywordMatchType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Exact match.
        Exact = 2,
        /// Phrase match.
        Phrase = 3,
        /// Broad match.
        Broad = 4,
    }
    impl KeywordMatchType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeywordMatchType::Unspecified => "UNSPECIFIED",
                KeywordMatchType::Unknown => "UNKNOWN",
                KeywordMatchType::Exact => "EXACT",
                KeywordMatchType::Phrase => "PHRASE",
                KeywordMatchType::Broad => "BROAD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "EXACT" => Some(Self::Exact),
                "PHRASE" => Some(Self::Phrase),
                "BROAD" => Some(Self::Broad),
                _ => None,
            }
        }
    }
}
/// Container for enum describing the type of the listing group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupTypeEnum {}
/// Nested message and enum types in `ListingGroupTypeEnum`.
pub mod listing_group_type_enum {
    /// The type of the listing group.
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
    pub enum ListingGroupType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Subdivision of products along some listing dimension. These nodes
        /// are not used by serving to target listing entries, but is purely
        /// to define the structure of the tree.
        Subdivision = 2,
        /// Listing group unit that defines a bid.
        Unit = 3,
    }
    impl ListingGroupType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupType::Unspecified => "UNSPECIFIED",
                ListingGroupType::Unknown => "UNKNOWN",
                ListingGroupType::Subdivision => "SUBDIVISION",
                ListingGroupType::Unit => "UNIT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "SUBDIVISION" => Some(Self::Subdivision),
                "UNIT" => Some(Self::Unit),
                _ => None,
            }
        }
    }
}
/// Container for enum describing unit of radius in location group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationGroupRadiusUnitsEnum {}
/// Nested message and enum types in `LocationGroupRadiusUnitsEnum`.
pub mod location_group_radius_units_enum {
    /// The unit of radius distance in location group (for example, MILES)
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
    pub enum LocationGroupRadiusUnits {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Meters
        Meters = 2,
        /// Miles
        Miles = 3,
        /// Milli Miles
        MilliMiles = 4,
    }
    impl LocationGroupRadiusUnits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LocationGroupRadiusUnits::Unspecified => "UNSPECIFIED",
                LocationGroupRadiusUnits::Unknown => "UNKNOWN",
                LocationGroupRadiusUnits::Meters => "METERS",
                LocationGroupRadiusUnits::Miles => "MILES",
                LocationGroupRadiusUnits::MilliMiles => "MILLI_MILES",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "METERS" => Some(Self::Meters),
                "MILES" => Some(Self::Miles),
                "MILLI_MILES" => Some(Self::MilliMiles),
                _ => None,
            }
        }
    }
}
/// Container for enumeration of quarter-hours.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MinuteOfHourEnum {}
/// Nested message and enum types in `MinuteOfHourEnum`.
pub mod minute_of_hour_enum {
    /// Enumerates of quarter-hours. For example, "FIFTEEN"
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
    pub enum MinuteOfHour {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Zero minutes past the hour.
        Zero = 2,
        /// Fifteen minutes past the hour.
        Fifteen = 3,
        /// Thirty minutes past the hour.
        Thirty = 4,
        /// Forty-five minutes past the hour.
        FortyFive = 5,
    }
    impl MinuteOfHour {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MinuteOfHour::Unspecified => "UNSPECIFIED",
                MinuteOfHour::Unknown => "UNKNOWN",
                MinuteOfHour::Zero => "ZERO",
                MinuteOfHour::Fifteen => "FIFTEEN",
                MinuteOfHour::Thirty => "THIRTY",
                MinuteOfHour::FortyFive => "FORTY_FIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ZERO" => Some(Self::Zero),
                "FIFTEEN" => Some(Self::Fifteen),
                "THIRTY" => Some(Self::Thirty),
                "FORTY_FIVE" => Some(Self::FortyFive),
                _ => None,
            }
        }
    }
}
/// Container for enum describing webpage condition operand in webpage criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebpageConditionOperandEnum {}
/// Nested message and enum types in `WebpageConditionOperandEnum`.
pub mod webpage_condition_operand_enum {
    /// The webpage condition operand in webpage criterion.
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
    pub enum WebpageConditionOperand {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Operand denoting a webpage URL targeting condition.
        Url = 2,
        /// Operand denoting a webpage category targeting condition.
        Category = 3,
        /// Operand denoting a webpage title targeting condition.
        PageTitle = 4,
        /// Operand denoting a webpage content targeting condition.
        PageContent = 5,
        /// Operand denoting a webpage custom label targeting condition.
        CustomLabel = 6,
    }
    impl WebpageConditionOperand {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WebpageConditionOperand::Unspecified => "UNSPECIFIED",
                WebpageConditionOperand::Unknown => "UNKNOWN",
                WebpageConditionOperand::Url => "URL",
                WebpageConditionOperand::Category => "CATEGORY",
                WebpageConditionOperand::PageTitle => "PAGE_TITLE",
                WebpageConditionOperand::PageContent => "PAGE_CONTENT",
                WebpageConditionOperand::CustomLabel => "CUSTOM_LABEL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "URL" => Some(Self::Url),
                "CATEGORY" => Some(Self::Category),
                "PAGE_TITLE" => Some(Self::PageTitle),
                "PAGE_CONTENT" => Some(Self::PageContent),
                "CUSTOM_LABEL" => Some(Self::CustomLabel),
                _ => None,
            }
        }
    }
}
/// Container for enum describing webpage condition operator in webpage
/// criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebpageConditionOperatorEnum {}
/// Nested message and enum types in `WebpageConditionOperatorEnum`.
pub mod webpage_condition_operator_enum {
    /// The webpage condition operator in webpage criterion.
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
    pub enum WebpageConditionOperator {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The argument web condition is equal to the compared web condition.
        Equals = 2,
        /// The argument web condition is part of the compared web condition.
        Contains = 3,
    }
    impl WebpageConditionOperator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WebpageConditionOperator::Unspecified => "UNSPECIFIED",
                WebpageConditionOperator::Unknown => "UNKNOWN",
                WebpageConditionOperator::Equals => "EQUALS",
                WebpageConditionOperator::Contains => "CONTAINS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "EQUALS" => Some(Self::Equals),
                "CONTAINS" => Some(Self::Contains),
                _ => None,
            }
        }
    }
}
/// Container for enumeration of Google Ads network types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdNetworkTypeEnum {}
/// Nested message and enum types in `AdNetworkTypeEnum`.
pub mod ad_network_type_enum {
    /// Enumerates Google Ads network types.
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
    pub enum AdNetworkType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Google search.
        Search = 2,
        /// Search partners.
        SearchPartners = 3,
        /// Display Network.
        Content = 4,
        /// YouTube Search.
        YoutubeSearch = 5,
        /// YouTube Videos
        YoutubeWatch = 6,
        /// Cross-network.
        Mixed = 7,
    }
    impl AdNetworkType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdNetworkType::Unspecified => "UNSPECIFIED",
                AdNetworkType::Unknown => "UNKNOWN",
                AdNetworkType::Search => "SEARCH",
                AdNetworkType::SearchPartners => "SEARCH_PARTNERS",
                AdNetworkType::Content => "CONTENT",
                AdNetworkType::YoutubeSearch => "YOUTUBE_SEARCH",
                AdNetworkType::YoutubeWatch => "YOUTUBE_WATCH",
                AdNetworkType::Mixed => "MIXED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "SEARCH" => Some(Self::Search),
                "SEARCH_PARTNERS" => Some(Self::SearchPartners),
                "CONTENT" => Some(Self::Content),
                "YOUTUBE_SEARCH" => Some(Self::YoutubeSearch),
                "YOUTUBE_WATCH" => Some(Self::YoutubeWatch),
                "MIXED" => Some(Self::Mixed),
                _ => None,
            }
        }
    }
}
/// Container for enum describing the category of conversions that are associated
/// with a ConversionAction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionCategoryEnum {}
/// Nested message and enum types in `ConversionActionCategoryEnum`.
pub mod conversion_action_category_enum {
    /// The category of conversions that are associated with a ConversionAction.
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
    pub enum ConversionActionCategory {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Default category.
        Default = 2,
        /// User visiting a page.
        PageView = 3,
        /// Purchase, sales, or "order placed" event.
        Purchase = 4,
        /// Signup user action.
        Signup = 5,
        /// Lead-generating action.
        Lead = 6,
        /// Software download action (as for an app).
        Download = 7,
        /// The addition of items to a shopping cart or bag on an advertiser site.
        AddToCart = 8,
        /// When someone enters the checkout flow on an advertiser site.
        BeginCheckout = 9,
        /// The start of a paid subscription for a product or service.
        SubscribePaid = 10,
        /// A call to indicate interest in an advertiser's offering.
        PhoneCallLead = 11,
        /// A lead conversion imported from an external source into Google Ads.
        ImportedLead = 12,
        /// A submission of a form on an advertiser site indicating business
        /// interest.
        SubmitLeadForm = 13,
        /// A booking of an appointment with an advertiser's business.
        BookAppointment = 14,
        /// A quote or price estimate request.
        RequestQuote = 15,
        /// A search for an advertiser's business location with intention to visit.
        GetDirections = 16,
        /// A click to an advertiser's partner's site.
        OutboundClick = 17,
        /// A call, SMS, email, chat or other type of contact to an advertiser.
        Contact = 18,
        /// A website engagement event such as long site time or a Google Analytics
        /// (GA) Smart Goal. Intended to be used for GA, Firebase, GA Gold goal
        /// imports.
        Engagement = 19,
        /// A visit to a physical store location.
        StoreVisit = 20,
        /// A sale occurring in a physical store.
        StoreSale = 21,
        /// A lead conversion imported from an external source into Google Ads,
        /// that has been further qualified by the advertiser (marketing/sales team).
        /// In the lead-to-sale journey, advertisers get leads, then act on them
        /// by reaching out to the consumer. If the consumer is interested and
        /// may end up buying their product, the advertiser marks such leads as
        /// "qualified leads".
        QualifiedLead = 22,
        /// A lead conversion imported from an external source into Google Ads, that
        /// has further completed a chosen stage as defined by the lead gen
        /// advertiser.
        ConvertedLead = 23,
    }
    impl ConversionActionCategory {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionActionCategory::Unspecified => "UNSPECIFIED",
                ConversionActionCategory::Unknown => "UNKNOWN",
                ConversionActionCategory::Default => "DEFAULT",
                ConversionActionCategory::PageView => "PAGE_VIEW",
                ConversionActionCategory::Purchase => "PURCHASE",
                ConversionActionCategory::Signup => "SIGNUP",
                ConversionActionCategory::Lead => "LEAD",
                ConversionActionCategory::Download => "DOWNLOAD",
                ConversionActionCategory::AddToCart => "ADD_TO_CART",
                ConversionActionCategory::BeginCheckout => "BEGIN_CHECKOUT",
                ConversionActionCategory::SubscribePaid => "SUBSCRIBE_PAID",
                ConversionActionCategory::PhoneCallLead => "PHONE_CALL_LEAD",
                ConversionActionCategory::ImportedLead => "IMPORTED_LEAD",
                ConversionActionCategory::SubmitLeadForm => "SUBMIT_LEAD_FORM",
                ConversionActionCategory::BookAppointment => "BOOK_APPOINTMENT",
                ConversionActionCategory::RequestQuote => "REQUEST_QUOTE",
                ConversionActionCategory::GetDirections => "GET_DIRECTIONS",
                ConversionActionCategory::OutboundClick => "OUTBOUND_CLICK",
                ConversionActionCategory::Contact => "CONTACT",
                ConversionActionCategory::Engagement => "ENGAGEMENT",
                ConversionActionCategory::StoreVisit => "STORE_VISIT",
                ConversionActionCategory::StoreSale => "STORE_SALE",
                ConversionActionCategory::QualifiedLead => "QUALIFIED_LEAD",
                ConversionActionCategory::ConvertedLead => "CONVERTED_LEAD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "DEFAULT" => Some(Self::Default),
                "PAGE_VIEW" => Some(Self::PageView),
                "PURCHASE" => Some(Self::Purchase),
                "SIGNUP" => Some(Self::Signup),
                "LEAD" => Some(Self::Lead),
                "DOWNLOAD" => Some(Self::Download),
                "ADD_TO_CART" => Some(Self::AddToCart),
                "BEGIN_CHECKOUT" => Some(Self::BeginCheckout),
                "SUBSCRIBE_PAID" => Some(Self::SubscribePaid),
                "PHONE_CALL_LEAD" => Some(Self::PhoneCallLead),
                "IMPORTED_LEAD" => Some(Self::ImportedLead),
                "SUBMIT_LEAD_FORM" => Some(Self::SubmitLeadForm),
                "BOOK_APPOINTMENT" => Some(Self::BookAppointment),
                "REQUEST_QUOTE" => Some(Self::RequestQuote),
                "GET_DIRECTIONS" => Some(Self::GetDirections),
                "OUTBOUND_CLICK" => Some(Self::OutboundClick),
                "CONTACT" => Some(Self::Contact),
                "ENGAGEMENT" => Some(Self::Engagement),
                "STORE_VISIT" => Some(Self::StoreVisit),
                "STORE_SALE" => Some(Self::StoreSale),
                "QUALIFIED_LEAD" => Some(Self::QualifiedLead),
                "CONVERTED_LEAD" => Some(Self::ConvertedLead),
                _ => None,
            }
        }
    }
}
/// Locality of a product offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductChannelEnum {}
/// Nested message and enum types in `ProductChannelEnum`.
pub mod product_channel_enum {
    /// Enum describing the locality of a product offer.
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
    pub enum ProductChannel {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The item is sold online.
        Online = 2,
        /// The item is sold in local stores.
        Local = 3,
    }
    impl ProductChannel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProductChannel::Unspecified => "UNSPECIFIED",
                ProductChannel::Unknown => "UNKNOWN",
                ProductChannel::Online => "ONLINE",
                ProductChannel::Local => "LOCAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ONLINE" => Some(Self::Online),
                "LOCAL" => Some(Self::Local),
                _ => None,
            }
        }
    }
}
/// Availability of a product offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductChannelExclusivityEnum {}
/// Nested message and enum types in `ProductChannelExclusivityEnum`.
pub mod product_channel_exclusivity_enum {
    /// Enum describing the availability of a product offer.
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
    pub enum ProductChannelExclusivity {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The item is sold through one channel only, either local stores or online
        /// as indicated by its ProductChannel.
        SingleChannel = 2,
        /// The item is matched to its online or local stores counterpart, indicating
        /// it is available for purchase in both ShoppingProductChannels.
        MultiChannel = 3,
    }
    impl ProductChannelExclusivity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProductChannelExclusivity::Unspecified => "UNSPECIFIED",
                ProductChannelExclusivity::Unknown => "UNKNOWN",
                ProductChannelExclusivity::SingleChannel => "SINGLE_CHANNEL",
                ProductChannelExclusivity::MultiChannel => "MULTI_CHANNEL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "SINGLE_CHANNEL" => Some(Self::SingleChannel),
                "MULTI_CHANNEL" => Some(Self::MultiChannel),
                _ => None,
            }
        }
    }
}
/// Condition of a product offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductConditionEnum {}
/// Nested message and enum types in `ProductConditionEnum`.
pub mod product_condition_enum {
    /// Enum describing the condition of a product offer.
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
    pub enum ProductCondition {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The product condition is old.
        Old = 2,
        /// The product condition is new.
        New = 3,
        /// The product condition is refurbished.
        Refurbished = 4,
        /// The product condition is used.
        Used = 5,
    }
    impl ProductCondition {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProductCondition::Unspecified => "UNSPECIFIED",
                ProductCondition::Unknown => "UNKNOWN",
                ProductCondition::Old => "OLD",
                ProductCondition::New => "NEW",
                ProductCondition::Refurbished => "REFURBISHED",
                ProductCondition::Used => "USED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "OLD" => Some(Self::Old),
                "NEW" => Some(Self::New),
                "REFURBISHED" => Some(Self::Refurbished),
                "USED" => Some(Self::Used),
                _ => None,
            }
        }
    }
}
/// Indicates summary row setting in request parameter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummaryRowSettingEnum {}
/// Nested message and enum types in `SummaryRowSettingEnum`.
pub mod summary_row_setting_enum {
    /// Enum describing return summary row settings.
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
    pub enum SummaryRowSetting {
        /// Not specified.
        Unspecified = 0,
        /// Represent unknown values of return summary row.
        Unknown = 1,
        /// Do not return summary row.
        NoSummaryRow = 2,
        /// Return summary row along with results. The summary row will be returned
        /// in the last batch alone (last batch will contain no results).
        SummaryRowWithResults = 3,
        /// Return summary row only and return no results.
        SummaryRowOnly = 4,
    }
    impl SummaryRowSetting {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SummaryRowSetting::Unspecified => "UNSPECIFIED",
                SummaryRowSetting::Unknown => "UNKNOWN",
                SummaryRowSetting::NoSummaryRow => "NO_SUMMARY_ROW",
                SummaryRowSetting::SummaryRowWithResults => "SUMMARY_ROW_WITH_RESULTS",
                SummaryRowSetting::SummaryRowOnly => "SUMMARY_ROW_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "NO_SUMMARY_ROW" => Some(Self::NoSummaryRow),
                "SUMMARY_ROW_WITH_RESULTS" => Some(Self::SummaryRowWithResults),
                "SUMMARY_ROW_ONLY" => Some(Self::SummaryRowOnly),
                _ => None,
            }
        }
    }
}
/// The dimensions that can be targeted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetingDimensionEnum {}
/// Nested message and enum types in `TargetingDimensionEnum`.
pub mod targeting_dimension_enum {
    /// Enum describing possible targeting dimensions.
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
    pub enum TargetingDimension {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Keyword criteria, for example, 'mars cruise'. KEYWORD may be used as a
        /// custom bid dimension. Keywords are always a targeting dimension, so may
        /// not be set as a target "ALL" dimension with TargetRestriction.
        Keyword = 2,
        /// Audience criteria, which include user list, user interest, custom
        /// affinity,  and custom in market.
        Audience = 3,
        /// Topic criteria for targeting categories of content, for example,
        /// 'category::Animals>Pets' Used for Display and Video targeting.
        Topic = 4,
        /// Criteria for targeting gender.
        Gender = 5,
        /// Criteria for targeting age ranges.
        AgeRange = 6,
        /// Placement criteria, which include websites like 'www.flowers4sale.com',
        /// as well as mobile applications, mobile app categories, YouTube videos,
        /// and YouTube channels.
        Placement = 7,
        /// Criteria for parental status targeting.
        ParentalStatus = 8,
        /// Criteria for income range targeting.
        IncomeRange = 9,
    }
    impl TargetingDimension {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TargetingDimension::Unspecified => "UNSPECIFIED",
                TargetingDimension::Unknown => "UNKNOWN",
                TargetingDimension::Keyword => "KEYWORD",
                TargetingDimension::Audience => "AUDIENCE",
                TargetingDimension::Topic => "TOPIC",
                TargetingDimension::Gender => "GENDER",
                TargetingDimension::AgeRange => "AGE_RANGE",
                TargetingDimension::Placement => "PLACEMENT",
                TargetingDimension::ParentalStatus => "PARENTAL_STATUS",
                TargetingDimension::IncomeRange => "INCOME_RANGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "KEYWORD" => Some(Self::Keyword),
                "AUDIENCE" => Some(Self::Audience),
                "TOPIC" => Some(Self::Topic),
                "GENDER" => Some(Self::Gender),
                "AGE_RANGE" => Some(Self::AgeRange),
                "PLACEMENT" => Some(Self::Placement),
                "PARENTAL_STATUS" => Some(Self::ParentalStatus),
                "INCOME_RANGE" => Some(Self::IncomeRange),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible ad rotation modes of ads within an
/// ad group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdRotationModeEnum {}
/// Nested message and enum types in `AdGroupAdRotationModeEnum`.
pub mod ad_group_ad_rotation_mode_enum {
    /// The possible ad rotation modes of an ad group.
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
    pub enum AdGroupAdRotationMode {
        /// The ad rotation mode has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// Optimize ad group ads based on clicks or conversions.
        Optimize = 2,
        /// Rotate evenly forever.
        RotateForever = 3,
    }
    impl AdGroupAdRotationMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupAdRotationMode::Unspecified => "UNSPECIFIED",
                AdGroupAdRotationMode::Unknown => "UNKNOWN",
                AdGroupAdRotationMode::Optimize => "OPTIMIZE",
                AdGroupAdRotationMode::RotateForever => "ROTATE_FOREVER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "OPTIMIZE" => Some(Self::Optimize),
                "ROTATE_FOREVER" => Some(Self::RotateForever),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible AdGroup engine statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupEngineStatusEnum {}
/// Nested message and enum types in `AdGroupEngineStatusEnum`.
pub mod ad_group_engine_status_enum {
    /// Status of the ad group engine.
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
    pub enum AdGroupEngineStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Deprecated. Do not use.
        AdGroupEligible = 2,
        /// No ads are running for this ad group, because the ad group's end date has
        /// passed.
        AdGroupExpired = 3,
        /// The ad group has been deleted.
        AdGroupRemoved = 4,
        /// No ads are running for this ad group because the associated ad group is
        /// still in draft form.
        AdGroupDraft = 5,
        /// The ad group has been paused.
        AdGroupPaused = 6,
        /// The ad group is active and currently serving ads.
        AdGroupServing = 7,
        /// The ad group has been submitted (Microsoft Bing Ads legacy status).
        AdGroupSubmitted = 8,
        /// No ads are running for this ad group, because the campaign has been
        /// paused.
        CampaignPaused = 9,
        /// No ads are running for this ad group, because the account has been
        /// paused.
        AccountPaused = 10,
    }
    impl AdGroupEngineStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupEngineStatus::Unspecified => "UNSPECIFIED",
                AdGroupEngineStatus::Unknown => "UNKNOWN",
                AdGroupEngineStatus::AdGroupEligible => "AD_GROUP_ELIGIBLE",
                AdGroupEngineStatus::AdGroupExpired => "AD_GROUP_EXPIRED",
                AdGroupEngineStatus::AdGroupRemoved => "AD_GROUP_REMOVED",
                AdGroupEngineStatus::AdGroupDraft => "AD_GROUP_DRAFT",
                AdGroupEngineStatus::AdGroupPaused => "AD_GROUP_PAUSED",
                AdGroupEngineStatus::AdGroupServing => "AD_GROUP_SERVING",
                AdGroupEngineStatus::AdGroupSubmitted => "AD_GROUP_SUBMITTED",
                AdGroupEngineStatus::CampaignPaused => "CAMPAIGN_PAUSED",
                AdGroupEngineStatus::AccountPaused => "ACCOUNT_PAUSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "AD_GROUP_ELIGIBLE" => Some(Self::AdGroupEligible),
                "AD_GROUP_EXPIRED" => Some(Self::AdGroupExpired),
                "AD_GROUP_REMOVED" => Some(Self::AdGroupRemoved),
                "AD_GROUP_DRAFT" => Some(Self::AdGroupDraft),
                "AD_GROUP_PAUSED" => Some(Self::AdGroupPaused),
                "AD_GROUP_SERVING" => Some(Self::AdGroupServing),
                "AD_GROUP_SUBMITTED" => Some(Self::AdGroupSubmitted),
                "CAMPAIGN_PAUSED" => Some(Self::CampaignPaused),
                "ACCOUNT_PAUSED" => Some(Self::AccountPaused),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible statuses of an ad group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupStatusEnum {}
/// Nested message and enum types in `AdGroupStatusEnum`.
pub mod ad_group_status_enum {
    /// The possible statuses of an ad group.
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
    pub enum AdGroupStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The ad group is enabled.
        Enabled = 2,
        /// The ad group is paused.
        Paused = 3,
        /// The ad group is removed.
        Removed = 4,
    }
    impl AdGroupStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupStatus::Unspecified => "UNSPECIFIED",
                AdGroupStatus::Unknown => "UNKNOWN",
                AdGroupStatus::Enabled => "ENABLED",
                AdGroupStatus::Paused => "PAUSED",
                AdGroupStatus::Removed => "REMOVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "PAUSED" => Some(Self::Paused),
                "REMOVED" => Some(Self::Removed),
                _ => None,
            }
        }
    }
}
/// Defines types of an ad group, specific to a particular campaign channel
/// type. This type drives validations that restrict which entities can be
/// added to the ad group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupTypeEnum {}
/// Nested message and enum types in `AdGroupTypeEnum`.
pub mod ad_group_type_enum {
    /// Enum listing the possible types of an ad group.
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
    pub enum AdGroupType {
        /// The type has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The default ad group type for Search campaigns.
        SearchStandard = 2,
        /// The default ad group type for Display campaigns.
        DisplayStandard = 3,
        /// The ad group type for Shopping campaigns serving standard product ads.
        ShoppingProductAds = 4,
        /// The type for ad groups that are limited to serving Showcase or Merchant
        /// ads in Shopping results.
        ShoppingShowcaseAds = 5,
        /// The default ad group type for Hotel campaigns.
        HotelAds = 6,
        /// The type for ad groups in Smart Shopping campaigns.
        ShoppingSmartAds = 7,
        /// Short unskippable in-stream video ads.
        VideoBumper = 8,
        /// TrueView (skippable) in-stream video ads.
        VideoTrueViewInStream = 9,
        /// TrueView in-display video ads.
        VideoTrueViewInDisplay = 10,
        /// Unskippable in-stream video ads.
        VideoNonSkippableInStream = 11,
        /// Outstream video ads.
        VideoOutstream = 12,
        /// Ad group type for Dynamic Search Ads ad groups.
        SearchDynamicAds = 13,
        /// The type for ad groups in Shopping Comparison Listing campaigns.
        ShoppingComparisonListingAds = 14,
        /// The ad group type for Promoted Hotel ad groups.
        PromotedHotelAds = 15,
        /// Video responsive ad groups.
        VideoResponsive = 16,
        /// Video efficient reach ad groups.
        VideoEfficientReach = 17,
        /// Ad group type for Smart campaigns.
        SmartCampaignAds = 18,
        /// Ad group type for Travel campaigns.
        TravelAds = 19,
    }
    impl AdGroupType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupType::Unspecified => "UNSPECIFIED",
                AdGroupType::Unknown => "UNKNOWN",
                AdGroupType::SearchStandard => "SEARCH_STANDARD",
                AdGroupType::DisplayStandard => "DISPLAY_STANDARD",
                AdGroupType::ShoppingProductAds => "SHOPPING_PRODUCT_ADS",
                AdGroupType::ShoppingShowcaseAds => "SHOPPING_SHOWCASE_ADS",
                AdGroupType::HotelAds => "HOTEL_ADS",
                AdGroupType::ShoppingSmartAds => "SHOPPING_SMART_ADS",
                AdGroupType::VideoBumper => "VIDEO_BUMPER",
                AdGroupType::VideoTrueViewInStream => "VIDEO_TRUE_VIEW_IN_STREAM",
                AdGroupType::VideoTrueViewInDisplay => "VIDEO_TRUE_VIEW_IN_DISPLAY",
                AdGroupType::VideoNonSkippableInStream => "VIDEO_NON_SKIPPABLE_IN_STREAM",
                AdGroupType::VideoOutstream => "VIDEO_OUTSTREAM",
                AdGroupType::SearchDynamicAds => "SEARCH_DYNAMIC_ADS",
                AdGroupType::ShoppingComparisonListingAds => {
                    "SHOPPING_COMPARISON_LISTING_ADS"
                }
                AdGroupType::PromotedHotelAds => "PROMOTED_HOTEL_ADS",
                AdGroupType::VideoResponsive => "VIDEO_RESPONSIVE",
                AdGroupType::VideoEfficientReach => "VIDEO_EFFICIENT_REACH",
                AdGroupType::SmartCampaignAds => "SMART_CAMPAIGN_ADS",
                AdGroupType::TravelAds => "TRAVEL_ADS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "SEARCH_STANDARD" => Some(Self::SearchStandard),
                "DISPLAY_STANDARD" => Some(Self::DisplayStandard),
                "SHOPPING_PRODUCT_ADS" => Some(Self::ShoppingProductAds),
                "SHOPPING_SHOWCASE_ADS" => Some(Self::ShoppingShowcaseAds),
                "HOTEL_ADS" => Some(Self::HotelAds),
                "SHOPPING_SMART_ADS" => Some(Self::ShoppingSmartAds),
                "VIDEO_BUMPER" => Some(Self::VideoBumper),
                "VIDEO_TRUE_VIEW_IN_STREAM" => Some(Self::VideoTrueViewInStream),
                "VIDEO_TRUE_VIEW_IN_DISPLAY" => Some(Self::VideoTrueViewInDisplay),
                "VIDEO_NON_SKIPPABLE_IN_STREAM" => Some(Self::VideoNonSkippableInStream),
                "VIDEO_OUTSTREAM" => Some(Self::VideoOutstream),
                "SEARCH_DYNAMIC_ADS" => Some(Self::SearchDynamicAds),
                "SHOPPING_COMPARISON_LISTING_ADS" => {
                    Some(Self::ShoppingComparisonListingAds)
                }
                "PROMOTED_HOTEL_ADS" => Some(Self::PromotedHotelAds),
                "VIDEO_RESPONSIVE" => Some(Self::VideoResponsive),
                "VIDEO_EFFICIENT_REACH" => Some(Self::VideoEfficientReach),
                "SMART_CAMPAIGN_ADS" => Some(Self::SmartCampaignAds),
                "TRAVEL_ADS" => Some(Self::TravelAds),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible AdGroupAd engine statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdEngineStatusEnum {}
/// Nested message and enum types in `AdGroupAdEngineStatusEnum`.
pub mod ad_group_ad_engine_status_enum {
    /// Enumerates AdGroupAd engine statuses.
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
    pub enum AdGroupAdEngineStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Deprecated. Do not use.
        AdGroupAdEligible = 2,
        /// Baidu: Creative was not approved.
        AdGroupAdInappropriateForCampaign = 3,
        /// Baidu: Mobile URL in process to be reviewed.
        AdGroupAdMobileUrlUnderReview = 4,
        /// Baidu: Creative is invalid on mobile device but valid on desktop.
        AdGroupAdPartiallyInvalid = 5,
        /// Baidu: Creative is ready for activation.
        AdGroupAdToBeActivated = 6,
        /// Baidu: Creative not reviewed.
        AdGroupAdNotReviewed = 7,
        /// Deprecated. Do not use. Previously used by Gemini
        AdGroupAdOnHold = 8,
        /// Creative has been paused.
        AdGroupAdPaused = 9,
        /// Creative has been removed.
        AdGroupAdRemoved = 10,
        /// Creative is pending review.
        AdGroupAdPendingReview = 11,
        /// Creative is under review.
        AdGroupAdUnderReview = 12,
        /// Creative has been approved.
        AdGroupAdApproved = 13,
        /// Creative has been disapproved.
        AdGroupAdDisapproved = 14,
        /// Creative is serving.
        AdGroupAdServing = 15,
        /// Creative has been paused because the account is paused.
        AdGroupAdAccountPaused = 16,
        /// Creative has been paused because the campaign is paused.
        AdGroupAdCampaignPaused = 17,
        /// Creative has been paused because the ad group is paused.
        AdGroupAdAdGroupPaused = 18,
    }
    impl AdGroupAdEngineStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupAdEngineStatus::Unspecified => "UNSPECIFIED",
                AdGroupAdEngineStatus::Unknown => "UNKNOWN",
                AdGroupAdEngineStatus::AdGroupAdEligible => "AD_GROUP_AD_ELIGIBLE",
                AdGroupAdEngineStatus::AdGroupAdInappropriateForCampaign => {
                    "AD_GROUP_AD_INAPPROPRIATE_FOR_CAMPAIGN"
                }
                AdGroupAdEngineStatus::AdGroupAdMobileUrlUnderReview => {
                    "AD_GROUP_AD_MOBILE_URL_UNDER_REVIEW"
                }
                AdGroupAdEngineStatus::AdGroupAdPartiallyInvalid => {
                    "AD_GROUP_AD_PARTIALLY_INVALID"
                }
                AdGroupAdEngineStatus::AdGroupAdToBeActivated => {
                    "AD_GROUP_AD_TO_BE_ACTIVATED"
                }
                AdGroupAdEngineStatus::AdGroupAdNotReviewed => "AD_GROUP_AD_NOT_REVIEWED",
                AdGroupAdEngineStatus::AdGroupAdOnHold => "AD_GROUP_AD_ON_HOLD",
                AdGroupAdEngineStatus::AdGroupAdPaused => "AD_GROUP_AD_PAUSED",
                AdGroupAdEngineStatus::AdGroupAdRemoved => "AD_GROUP_AD_REMOVED",
                AdGroupAdEngineStatus::AdGroupAdPendingReview => {
                    "AD_GROUP_AD_PENDING_REVIEW"
                }
                AdGroupAdEngineStatus::AdGroupAdUnderReview => "AD_GROUP_AD_UNDER_REVIEW",
                AdGroupAdEngineStatus::AdGroupAdApproved => "AD_GROUP_AD_APPROVED",
                AdGroupAdEngineStatus::AdGroupAdDisapproved => "AD_GROUP_AD_DISAPPROVED",
                AdGroupAdEngineStatus::AdGroupAdServing => "AD_GROUP_AD_SERVING",
                AdGroupAdEngineStatus::AdGroupAdAccountPaused => {
                    "AD_GROUP_AD_ACCOUNT_PAUSED"
                }
                AdGroupAdEngineStatus::AdGroupAdCampaignPaused => {
                    "AD_GROUP_AD_CAMPAIGN_PAUSED"
                }
                AdGroupAdEngineStatus::AdGroupAdAdGroupPaused => {
                    "AD_GROUP_AD_AD_GROUP_PAUSED"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "AD_GROUP_AD_ELIGIBLE" => Some(Self::AdGroupAdEligible),
                "AD_GROUP_AD_INAPPROPRIATE_FOR_CAMPAIGN" => {
                    Some(Self::AdGroupAdInappropriateForCampaign)
                }
                "AD_GROUP_AD_MOBILE_URL_UNDER_REVIEW" => {
                    Some(Self::AdGroupAdMobileUrlUnderReview)
                }
                "AD_GROUP_AD_PARTIALLY_INVALID" => Some(Self::AdGroupAdPartiallyInvalid),
                "AD_GROUP_AD_TO_BE_ACTIVATED" => Some(Self::AdGroupAdToBeActivated),
                "AD_GROUP_AD_NOT_REVIEWED" => Some(Self::AdGroupAdNotReviewed),
                "AD_GROUP_AD_ON_HOLD" => Some(Self::AdGroupAdOnHold),
                "AD_GROUP_AD_PAUSED" => Some(Self::AdGroupAdPaused),
                "AD_GROUP_AD_REMOVED" => Some(Self::AdGroupAdRemoved),
                "AD_GROUP_AD_PENDING_REVIEW" => Some(Self::AdGroupAdPendingReview),
                "AD_GROUP_AD_UNDER_REVIEW" => Some(Self::AdGroupAdUnderReview),
                "AD_GROUP_AD_APPROVED" => Some(Self::AdGroupAdApproved),
                "AD_GROUP_AD_DISAPPROVED" => Some(Self::AdGroupAdDisapproved),
                "AD_GROUP_AD_SERVING" => Some(Self::AdGroupAdServing),
                "AD_GROUP_AD_ACCOUNT_PAUSED" => Some(Self::AdGroupAdAccountPaused),
                "AD_GROUP_AD_CAMPAIGN_PAUSED" => Some(Self::AdGroupAdCampaignPaused),
                "AD_GROUP_AD_AD_GROUP_PAUSED" => Some(Self::AdGroupAdAdGroupPaused),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible statuses of an AdGroupAd.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdStatusEnum {}
/// Nested message and enum types in `AdGroupAdStatusEnum`.
pub mod ad_group_ad_status_enum {
    /// The possible statuses of an AdGroupAd.
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
    pub enum AdGroupAdStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The ad group ad is enabled.
        Enabled = 2,
        /// The ad group ad is paused.
        Paused = 3,
        /// The ad group ad is removed.
        Removed = 4,
    }
    impl AdGroupAdStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupAdStatus::Unspecified => "UNSPECIFIED",
                AdGroupAdStatus::Unknown => "UNKNOWN",
                AdGroupAdStatus::Enabled => "ENABLED",
                AdGroupAdStatus::Paused => "PAUSED",
                AdGroupAdStatus::Removed => "REMOVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "PAUSED" => Some(Self::Paused),
                "REMOVED" => Some(Self::Removed),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible types of an ad.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdTypeEnum {}
/// Nested message and enum types in `AdTypeEnum`.
pub mod ad_type_enum {
    /// The possible types of an ad.
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
    pub enum AdType {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The ad is a text ad.
        TextAd = 2,
        /// The ad is an expanded text ad.
        ExpandedTextAd = 3,
        /// The ad is a call only ad.
        CallOnlyAd = 6,
        /// The ad is an expanded dynamic search ad.
        ExpandedDynamicSearchAd = 7,
        /// The ad is a hotel ad.
        HotelAd = 8,
        /// The ad is a Smart Shopping ad.
        ShoppingSmartAd = 9,
        /// The ad is a standard Shopping ad.
        ShoppingProductAd = 10,
        /// The ad is a video ad.
        VideoAd = 12,
        /// This ad is a Gmail ad.
        GmailAd = 13,
        /// This ad is an Image ad.
        ImageAd = 14,
        /// The ad is a responsive search ad.
        ResponsiveSearchAd = 15,
        /// The ad is a legacy responsive display ad.
        LegacyResponsiveDisplayAd = 16,
        /// The ad is an app ad.
        AppAd = 17,
        /// The ad is a legacy app install ad.
        LegacyAppInstallAd = 18,
        /// The ad is a responsive display ad.
        ResponsiveDisplayAd = 19,
        /// The ad is a local ad.
        LocalAd = 20,
        /// The ad is a display upload ad with the HTML5_UPLOAD_AD product type.
        Html5UploadAd = 21,
        /// The ad is a display upload ad with one of the DYNAMIC_HTML5_* product
        /// types.
        DynamicHtml5Ad = 22,
        /// The ad is an app engagement ad.
        AppEngagementAd = 23,
        /// The ad is a Shopping Comparison Listing ad.
        ShoppingComparisonListingAd = 24,
        /// Video bumper ad.
        VideoBumperAd = 25,
        /// Video non-skippable in-stream ad.
        VideoNonSkippableInStreamAd = 26,
        /// Video outstream ad.
        VideoOutstreamAd = 27,
        /// Video TrueView in-display ad.
        VideoTrueviewDiscoveryAd = 28,
        /// Video TrueView in-stream ad.
        VideoTrueviewInStreamAd = 29,
        /// Video responsive ad.
        VideoResponsiveAd = 30,
        /// Smart campaign ad.
        SmartCampaignAd = 31,
        /// Universal app pre-registration ad.
        AppPreRegistrationAd = 33,
        /// Discovery multi asset ad.
        DiscoveryMultiAssetAd = 35,
        /// Discovery carousel ad.
        DiscoveryCarouselAd = 36,
        /// Travel ad.
        TravelAd = 37,
        /// Discovery video responsive ad.
        DiscoveryVideoResponsiveAd = 38,
    }
    impl AdType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdType::Unspecified => "UNSPECIFIED",
                AdType::Unknown => "UNKNOWN",
                AdType::TextAd => "TEXT_AD",
                AdType::ExpandedTextAd => "EXPANDED_TEXT_AD",
                AdType::CallOnlyAd => "CALL_ONLY_AD",
                AdType::ExpandedDynamicSearchAd => "EXPANDED_DYNAMIC_SEARCH_AD",
                AdType::HotelAd => "HOTEL_AD",
                AdType::ShoppingSmartAd => "SHOPPING_SMART_AD",
                AdType::ShoppingProductAd => "SHOPPING_PRODUCT_AD",
                AdType::VideoAd => "VIDEO_AD",
                AdType::GmailAd => "GMAIL_AD",
                AdType::ImageAd => "IMAGE_AD",
                AdType::ResponsiveSearchAd => "RESPONSIVE_SEARCH_AD",
                AdType::LegacyResponsiveDisplayAd => "LEGACY_RESPONSIVE_DISPLAY_AD",
                AdType::AppAd => "APP_AD",
                AdType::LegacyAppInstallAd => "LEGACY_APP_INSTALL_AD",
                AdType::ResponsiveDisplayAd => "RESPONSIVE_DISPLAY_AD",
                AdType::LocalAd => "LOCAL_AD",
                AdType::Html5UploadAd => "HTML5_UPLOAD_AD",
                AdType::DynamicHtml5Ad => "DYNAMIC_HTML5_AD",
                AdType::AppEngagementAd => "APP_ENGAGEMENT_AD",
                AdType::ShoppingComparisonListingAd => "SHOPPING_COMPARISON_LISTING_AD",
                AdType::VideoBumperAd => "VIDEO_BUMPER_AD",
                AdType::VideoNonSkippableInStreamAd => "VIDEO_NON_SKIPPABLE_IN_STREAM_AD",
                AdType::VideoOutstreamAd => "VIDEO_OUTSTREAM_AD",
                AdType::VideoTrueviewDiscoveryAd => "VIDEO_TRUEVIEW_DISCOVERY_AD",
                AdType::VideoTrueviewInStreamAd => "VIDEO_TRUEVIEW_IN_STREAM_AD",
                AdType::VideoResponsiveAd => "VIDEO_RESPONSIVE_AD",
                AdType::SmartCampaignAd => "SMART_CAMPAIGN_AD",
                AdType::AppPreRegistrationAd => "APP_PRE_REGISTRATION_AD",
                AdType::DiscoveryMultiAssetAd => "DISCOVERY_MULTI_ASSET_AD",
                AdType::DiscoveryCarouselAd => "DISCOVERY_CAROUSEL_AD",
                AdType::TravelAd => "TRAVEL_AD",
                AdType::DiscoveryVideoResponsiveAd => "DISCOVERY_VIDEO_RESPONSIVE_AD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "TEXT_AD" => Some(Self::TextAd),
                "EXPANDED_TEXT_AD" => Some(Self::ExpandedTextAd),
                "CALL_ONLY_AD" => Some(Self::CallOnlyAd),
                "EXPANDED_DYNAMIC_SEARCH_AD" => Some(Self::ExpandedDynamicSearchAd),
                "HOTEL_AD" => Some(Self::HotelAd),
                "SHOPPING_SMART_AD" => Some(Self::ShoppingSmartAd),
                "SHOPPING_PRODUCT_AD" => Some(Self::ShoppingProductAd),
                "VIDEO_AD" => Some(Self::VideoAd),
                "GMAIL_AD" => Some(Self::GmailAd),
                "IMAGE_AD" => Some(Self::ImageAd),
                "RESPONSIVE_SEARCH_AD" => Some(Self::ResponsiveSearchAd),
                "LEGACY_RESPONSIVE_DISPLAY_AD" => Some(Self::LegacyResponsiveDisplayAd),
                "APP_AD" => Some(Self::AppAd),
                "LEGACY_APP_INSTALL_AD" => Some(Self::LegacyAppInstallAd),
                "RESPONSIVE_DISPLAY_AD" => Some(Self::ResponsiveDisplayAd),
                "LOCAL_AD" => Some(Self::LocalAd),
                "HTML5_UPLOAD_AD" => Some(Self::Html5UploadAd),
                "DYNAMIC_HTML5_AD" => Some(Self::DynamicHtml5Ad),
                "APP_ENGAGEMENT_AD" => Some(Self::AppEngagementAd),
                "SHOPPING_COMPARISON_LISTING_AD" => {
                    Some(Self::ShoppingComparisonListingAd)
                }
                "VIDEO_BUMPER_AD" => Some(Self::VideoBumperAd),
                "VIDEO_NON_SKIPPABLE_IN_STREAM_AD" => {
                    Some(Self::VideoNonSkippableInStreamAd)
                }
                "VIDEO_OUTSTREAM_AD" => Some(Self::VideoOutstreamAd),
                "VIDEO_TRUEVIEW_DISCOVERY_AD" => Some(Self::VideoTrueviewDiscoveryAd),
                "VIDEO_TRUEVIEW_IN_STREAM_AD" => Some(Self::VideoTrueviewInStreamAd),
                "VIDEO_RESPONSIVE_AD" => Some(Self::VideoResponsiveAd),
                "SMART_CAMPAIGN_AD" => Some(Self::SmartCampaignAd),
                "APP_PRE_REGISTRATION_AD" => Some(Self::AppPreRegistrationAd),
                "DISCOVERY_MULTI_ASSET_AD" => Some(Self::DiscoveryMultiAssetAd),
                "DISCOVERY_CAROUSEL_AD" => Some(Self::DiscoveryCarouselAd),
                "TRAVEL_AD" => Some(Self::TravelAd),
                "DISCOVERY_VIDEO_RESPONSIVE_AD" => Some(Self::DiscoveryVideoResponsiveAd),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible statuses of an asset link.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetLinkStatusEnum {}
/// Nested message and enum types in `AssetLinkStatusEnum`.
pub mod asset_link_status_enum {
    /// Enum describing statuses of an asset link.
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
    pub enum AssetLinkStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Asset link is enabled.
        Enabled = 2,
        /// Asset link has been removed.
        Removed = 3,
        /// Asset link is paused.
        Paused = 4,
    }
    impl AssetLinkStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetLinkStatus::Unspecified => "UNSPECIFIED",
                AssetLinkStatus::Unknown => "UNKNOWN",
                AssetLinkStatus::Enabled => "ENABLED",
                AssetLinkStatus::Removed => "REMOVED",
                AssetLinkStatus::Paused => "PAUSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "REMOVED" => Some(Self::Removed),
                "PAUSED" => Some(Self::Paused),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible statuses of the linkage between asset
/// set and its container.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSetLinkStatusEnum {}
/// Nested message and enum types in `AssetSetLinkStatusEnum`.
pub mod asset_set_link_status_enum {
    /// The possible statuses of the linkage between asset set and its container.
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
    pub enum AssetSetLinkStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        /// This is a response-only value.
        Unknown = 1,
        /// The linkage between asset set and its container is enabled.
        Enabled = 2,
        /// The linkage between asset set and its container is removed.
        Removed = 3,
    }
    impl AssetSetLinkStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetSetLinkStatus::Unspecified => "UNSPECIFIED",
                AssetSetLinkStatus::Unknown => "UNKNOWN",
                AssetSetLinkStatus::Enabled => "ENABLED",
                AssetSetLinkStatus::Removed => "REMOVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "REMOVED" => Some(Self::Removed),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible AdGroupCriterion engine statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionEngineStatusEnum {}
/// Nested message and enum types in `AdGroupCriterionEngineStatusEnum`.
pub mod ad_group_criterion_engine_status_enum {
    /// Enumerates AdGroupCriterion engine statuses.
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
    pub enum AdGroupCriterionEngineStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Deprecated. Do not use.
        AdGroupCriterionEligible = 2,
        /// Baidu: Bid or quality too low to be displayed.
        AdGroupCriterionInappropriateForCampaign = 3,
        /// Baidu: Bid or quality too low for mobile, but eligible to display for
        /// desktop.
        AdGroupCriterionInvalidMobileSearch = 4,
        /// Baidu: Bid or quality too low for desktop, but eligible to display for
        /// mobile.
        AdGroupCriterionInvalidPcSearch = 5,
        /// Baidu: Bid or quality too low to be displayed.
        AdGroupCriterionInvalidSearch = 6,
        /// Baidu: Paused by Baidu due to low search volume.
        AdGroupCriterionLowSearchVolume = 7,
        /// Baidu: Mobile URL in process to be reviewed.
        AdGroupCriterionMobileUrlUnderReview = 8,
        /// Baidu: The landing page for one device is invalid, while the landing
        /// page for the other device is valid.
        AdGroupCriterionPartiallyInvalid = 9,
        /// Baidu: Keyword has been created and paused by Baidu account management,
        /// and is now ready for you to activate it.
        AdGroupCriterionToBeActivated = 10,
        /// Baidu: In process to be reviewed by Baidu. Gemini: Criterion under
        /// review.
        AdGroupCriterionUnderReview = 11,
        /// Baidu: Criterion to be reviewed.
        AdGroupCriterionNotReviewed = 12,
        /// Deprecated. Do not use. Previously used by Gemini
        AdGroupCriterionOnHold = 13,
        /// Y!J : Criterion pending review
        AdGroupCriterionPendingReview = 14,
        /// Criterion has been paused.
        AdGroupCriterionPaused = 15,
        /// Criterion has been removed.
        AdGroupCriterionRemoved = 16,
        /// Criterion has been approved.
        AdGroupCriterionApproved = 17,
        /// Criterion has been disapproved.
        AdGroupCriterionDisapproved = 18,
        /// Criterion is active and serving.
        AdGroupCriterionServing = 19,
        /// Criterion has been paused since the account is paused.
        AdGroupCriterionAccountPaused = 20,
    }
    impl AdGroupCriterionEngineStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupCriterionEngineStatus::Unspecified => "UNSPECIFIED",
                AdGroupCriterionEngineStatus::Unknown => "UNKNOWN",
                AdGroupCriterionEngineStatus::AdGroupCriterionEligible => {
                    "AD_GROUP_CRITERION_ELIGIBLE"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionInappropriateForCampaign => {
                    "AD_GROUP_CRITERION_INAPPROPRIATE_FOR_CAMPAIGN"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionInvalidMobileSearch => {
                    "AD_GROUP_CRITERION_INVALID_MOBILE_SEARCH"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionInvalidPcSearch => {
                    "AD_GROUP_CRITERION_INVALID_PC_SEARCH"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionInvalidSearch => {
                    "AD_GROUP_CRITERION_INVALID_SEARCH"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionLowSearchVolume => {
                    "AD_GROUP_CRITERION_LOW_SEARCH_VOLUME"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionMobileUrlUnderReview => {
                    "AD_GROUP_CRITERION_MOBILE_URL_UNDER_REVIEW"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionPartiallyInvalid => {
                    "AD_GROUP_CRITERION_PARTIALLY_INVALID"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionToBeActivated => {
                    "AD_GROUP_CRITERION_TO_BE_ACTIVATED"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionUnderReview => {
                    "AD_GROUP_CRITERION_UNDER_REVIEW"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionNotReviewed => {
                    "AD_GROUP_CRITERION_NOT_REVIEWED"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionOnHold => {
                    "AD_GROUP_CRITERION_ON_HOLD"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionPendingReview => {
                    "AD_GROUP_CRITERION_PENDING_REVIEW"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionPaused => {
                    "AD_GROUP_CRITERION_PAUSED"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionRemoved => {
                    "AD_GROUP_CRITERION_REMOVED"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionApproved => {
                    "AD_GROUP_CRITERION_APPROVED"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionDisapproved => {
                    "AD_GROUP_CRITERION_DISAPPROVED"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionServing => {
                    "AD_GROUP_CRITERION_SERVING"
                }
                AdGroupCriterionEngineStatus::AdGroupCriterionAccountPaused => {
                    "AD_GROUP_CRITERION_ACCOUNT_PAUSED"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "AD_GROUP_CRITERION_ELIGIBLE" => Some(Self::AdGroupCriterionEligible),
                "AD_GROUP_CRITERION_INAPPROPRIATE_FOR_CAMPAIGN" => {
                    Some(Self::AdGroupCriterionInappropriateForCampaign)
                }
                "AD_GROUP_CRITERION_INVALID_MOBILE_SEARCH" => {
                    Some(Self::AdGroupCriterionInvalidMobileSearch)
                }
                "AD_GROUP_CRITERION_INVALID_PC_SEARCH" => {
                    Some(Self::AdGroupCriterionInvalidPcSearch)
                }
                "AD_GROUP_CRITERION_INVALID_SEARCH" => {
                    Some(Self::AdGroupCriterionInvalidSearch)
                }
                "AD_GROUP_CRITERION_LOW_SEARCH_VOLUME" => {
                    Some(Self::AdGroupCriterionLowSearchVolume)
                }
                "AD_GROUP_CRITERION_MOBILE_URL_UNDER_REVIEW" => {
                    Some(Self::AdGroupCriterionMobileUrlUnderReview)
                }
                "AD_GROUP_CRITERION_PARTIALLY_INVALID" => {
                    Some(Self::AdGroupCriterionPartiallyInvalid)
                }
                "AD_GROUP_CRITERION_TO_BE_ACTIVATED" => {
                    Some(Self::AdGroupCriterionToBeActivated)
                }
                "AD_GROUP_CRITERION_UNDER_REVIEW" => {
                    Some(Self::AdGroupCriterionUnderReview)
                }
                "AD_GROUP_CRITERION_NOT_REVIEWED" => {
                    Some(Self::AdGroupCriterionNotReviewed)
                }
                "AD_GROUP_CRITERION_ON_HOLD" => Some(Self::AdGroupCriterionOnHold),
                "AD_GROUP_CRITERION_PENDING_REVIEW" => {
                    Some(Self::AdGroupCriterionPendingReview)
                }
                "AD_GROUP_CRITERION_PAUSED" => Some(Self::AdGroupCriterionPaused),
                "AD_GROUP_CRITERION_REMOVED" => Some(Self::AdGroupCriterionRemoved),
                "AD_GROUP_CRITERION_APPROVED" => Some(Self::AdGroupCriterionApproved),
                "AD_GROUP_CRITERION_DISAPPROVED" => {
                    Some(Self::AdGroupCriterionDisapproved)
                }
                "AD_GROUP_CRITERION_SERVING" => Some(Self::AdGroupCriterionServing),
                "AD_GROUP_CRITERION_ACCOUNT_PAUSED" => {
                    Some(Self::AdGroupCriterionAccountPaused)
                }
                _ => None,
            }
        }
    }
}
/// Message describing AdGroupCriterion statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionStatusEnum {}
/// Nested message and enum types in `AdGroupCriterionStatusEnum`.
pub mod ad_group_criterion_status_enum {
    /// The possible statuses of an AdGroupCriterion.
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
    pub enum AdGroupCriterionStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The ad group criterion is enabled.
        Enabled = 2,
        /// The ad group criterion is paused.
        Paused = 3,
        /// The ad group criterion is removed.
        Removed = 4,
    }
    impl AdGroupCriterionStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupCriterionStatus::Unspecified => "UNSPECIFIED",
                AdGroupCriterionStatus::Unknown => "UNKNOWN",
                AdGroupCriterionStatus::Enabled => "ENABLED",
                AdGroupCriterionStatus::Paused => "PAUSED",
                AdGroupCriterionStatus::Removed => "REMOVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "PAUSED" => Some(Self::Paused),
                "REMOVED" => Some(Self::Removed),
                _ => None,
            }
        }
    }
}
/// The possible types of a criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionTypeEnum {}
/// Nested message and enum types in `CriterionTypeEnum`.
pub mod criterion_type_enum {
    /// Enum describing possible criterion types.
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
    pub enum CriterionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Keyword, for example, 'mars cruise'.
        Keyword = 2,
        /// Placement, also known as Website, for example, 'www.flowers4sale.com'
        Placement = 3,
        /// Mobile application categories to target.
        MobileAppCategory = 4,
        /// Mobile applications to target.
        MobileApplication = 5,
        /// Devices to target.
        Device = 6,
        /// Locations to target.
        Location = 7,
        /// Listing groups to target.
        ListingGroup = 8,
        /// Ad Schedule.
        AdSchedule = 9,
        /// Age range.
        AgeRange = 10,
        /// Gender.
        Gender = 11,
        /// Income Range.
        IncomeRange = 12,
        /// Parental status.
        ParentalStatus = 13,
        /// YouTube Video.
        YoutubeVideo = 14,
        /// YouTube Channel.
        YoutubeChannel = 15,
        /// User list.
        UserList = 16,
        /// Proximity.
        Proximity = 17,
        /// A topic target on the display network (for example, "Pets & Animals").
        Topic = 18,
        /// Listing scope to target.
        ListingScope = 19,
        /// Language.
        Language = 20,
        /// IpBlock.
        IpBlock = 21,
        /// Content Label for category exclusion.
        ContentLabel = 22,
        /// Carrier.
        Carrier = 23,
        /// A category the user is interested in.
        UserInterest = 24,
        /// Webpage criterion for dynamic search ads.
        Webpage = 25,
        /// Operating system version.
        OperatingSystemVersion = 26,
        /// App payment model.
        AppPaymentModel = 27,
        /// Mobile device.
        MobileDevice = 28,
        /// Custom affinity.
        CustomAffinity = 29,
        /// Custom intent.
        CustomIntent = 30,
        /// Location group.
        LocationGroup = 31,
        /// Custom audience
        CustomAudience = 32,
        /// Combined audience
        CombinedAudience = 33,
        /// Smart Campaign keyword theme
        KeywordTheme = 34,
        /// Audience
        Audience = 35,
        /// Local Services Ads Service ID.
        LocalServiceId = 37,
        /// Brand
        Brand = 39,
        /// Brand List
        BrandList = 40,
    }
    impl CriterionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CriterionType::Unspecified => "UNSPECIFIED",
                CriterionType::Unknown => "UNKNOWN",
                CriterionType::Keyword => "KEYWORD",
                CriterionType::Placement => "PLACEMENT",
                CriterionType::MobileAppCategory => "MOBILE_APP_CATEGORY",
                CriterionType::MobileApplication => "MOBILE_APPLICATION",
                CriterionType::Device => "DEVICE",
                CriterionType::Location => "LOCATION",
                CriterionType::ListingGroup => "LISTING_GROUP",
                CriterionType::AdSchedule => "AD_SCHEDULE",
                CriterionType::AgeRange => "AGE_RANGE",
                CriterionType::Gender => "GENDER",
                CriterionType::IncomeRange => "INCOME_RANGE",
                CriterionType::ParentalStatus => "PARENTAL_STATUS",
                CriterionType::YoutubeVideo => "YOUTUBE_VIDEO",
                CriterionType::YoutubeChannel => "YOUTUBE_CHANNEL",
                CriterionType::UserList => "USER_LIST",
                CriterionType::Proximity => "PROXIMITY",
                CriterionType::Topic => "TOPIC",
                CriterionType::ListingScope => "LISTING_SCOPE",
                CriterionType::Language => "LANGUAGE",
                CriterionType::IpBlock => "IP_BLOCK",
                CriterionType::ContentLabel => "CONTENT_LABEL",
                CriterionType::Carrier => "CARRIER",
                CriterionType::UserInterest => "USER_INTEREST",
                CriterionType::Webpage => "WEBPAGE",
                CriterionType::OperatingSystemVersion => "OPERATING_SYSTEM_VERSION",
                CriterionType::AppPaymentModel => "APP_PAYMENT_MODEL",
                CriterionType::MobileDevice => "MOBILE_DEVICE",
                CriterionType::CustomAffinity => "CUSTOM_AFFINITY",
                CriterionType::CustomIntent => "CUSTOM_INTENT",
                CriterionType::LocationGroup => "LOCATION_GROUP",
                CriterionType::CustomAudience => "CUSTOM_AUDIENCE",
                CriterionType::CombinedAudience => "COMBINED_AUDIENCE",
                CriterionType::KeywordTheme => "KEYWORD_THEME",
                CriterionType::Audience => "AUDIENCE",
                CriterionType::LocalServiceId => "LOCAL_SERVICE_ID",
                CriterionType::Brand => "BRAND",
                CriterionType::BrandList => "BRAND_LIST",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "KEYWORD" => Some(Self::Keyword),
                "PLACEMENT" => Some(Self::Placement),
                "MOBILE_APP_CATEGORY" => Some(Self::MobileAppCategory),
                "MOBILE_APPLICATION" => Some(Self::MobileApplication),
                "DEVICE" => Some(Self::Device),
                "LOCATION" => Some(Self::Location),
                "LISTING_GROUP" => Some(Self::ListingGroup),
                "AD_SCHEDULE" => Some(Self::AdSchedule),
                "AGE_RANGE" => Some(Self::AgeRange),
                "GENDER" => Some(Self::Gender),
                "INCOME_RANGE" => Some(Self::IncomeRange),
                "PARENTAL_STATUS" => Some(Self::ParentalStatus),
                "YOUTUBE_VIDEO" => Some(Self::YoutubeVideo),
                "YOUTUBE_CHANNEL" => Some(Self::YoutubeChannel),
                "USER_LIST" => Some(Self::UserList),
                "PROXIMITY" => Some(Self::Proximity),
                "TOPIC" => Some(Self::Topic),
                "LISTING_SCOPE" => Some(Self::ListingScope),
                "LANGUAGE" => Some(Self::Language),
                "IP_BLOCK" => Some(Self::IpBlock),
                "CONTENT_LABEL" => Some(Self::ContentLabel),
                "CARRIER" => Some(Self::Carrier),
                "USER_INTEREST" => Some(Self::UserInterest),
                "WEBPAGE" => Some(Self::Webpage),
                "OPERATING_SYSTEM_VERSION" => Some(Self::OperatingSystemVersion),
                "APP_PAYMENT_MODEL" => Some(Self::AppPaymentModel),
                "MOBILE_DEVICE" => Some(Self::MobileDevice),
                "CUSTOM_AFFINITY" => Some(Self::CustomAffinity),
                "CUSTOM_INTENT" => Some(Self::CustomIntent),
                "LOCATION_GROUP" => Some(Self::LocationGroup),
                "CUSTOM_AUDIENCE" => Some(Self::CustomAudience),
                "COMBINED_AUDIENCE" => Some(Self::CombinedAudience),
                "KEYWORD_THEME" => Some(Self::KeywordTheme),
                "AUDIENCE" => Some(Self::Audience),
                "LOCAL_SERVICE_ID" => Some(Self::LocalServiceId),
                "BRAND" => Some(Self::Brand),
                "BRAND_LIST" => Some(Self::BrandList),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible data types for call conversion
/// reporting state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallConversionReportingStateEnum {}
/// Nested message and enum types in `CallConversionReportingStateEnum`.
pub mod call_conversion_reporting_state_enum {
    /// Possible data types for a call conversion action state.
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
    pub enum CallConversionReportingState {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Call conversion action is disabled.
        Disabled = 2,
        /// Call conversion action will use call conversion type set at the
        /// account level.
        UseAccountLevelCallConversionAction = 3,
        /// Call conversion action will use call conversion type set at the resource
        /// (call only ads/call extensions) level.
        UseResourceLevelCallConversionAction = 4,
    }
    impl CallConversionReportingState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CallConversionReportingState::Unspecified => "UNSPECIFIED",
                CallConversionReportingState::Unknown => "UNKNOWN",
                CallConversionReportingState::Disabled => "DISABLED",
                CallConversionReportingState::UseAccountLevelCallConversionAction => {
                    "USE_ACCOUNT_LEVEL_CALL_CONVERSION_ACTION"
                }
                CallConversionReportingState::UseResourceLevelCallConversionAction => {
                    "USE_RESOURCE_LEVEL_CALL_CONVERSION_ACTION"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "DISABLED" => Some(Self::Disabled),
                "USE_ACCOUNT_LEVEL_CALL_CONVERSION_ACTION" => {
                    Some(Self::UseAccountLevelCallConversionAction)
                }
                "USE_RESOURCE_LEVEL_CALL_CONVERSION_ACTION" => {
                    Some(Self::UseResourceLevelCallConversionAction)
                }
                _ => None,
            }
        }
    }
}
/// Container for enum describing the call to action types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallToActionTypeEnum {}
/// Nested message and enum types in `CallToActionTypeEnum`.
pub mod call_to_action_type_enum {
    /// Enum describing possible types of call to action.
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
    pub enum CallToActionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The call to action type is learn more.
        LearnMore = 2,
        /// The call to action type is get quote.
        GetQuote = 3,
        /// The call to action type is apply now.
        ApplyNow = 4,
        /// The call to action type is sign up.
        SignUp = 5,
        /// The call to action type is contact us.
        ContactUs = 6,
        /// The call to action type is subscribe.
        Subscribe = 7,
        /// The call to action type is download.
        Download = 8,
        /// The call to action type is book now.
        BookNow = 9,
        /// The call to action type is shop now.
        ShopNow = 10,
        /// The call to action type is buy now.
        BuyNow = 11,
        /// The call to action type is donate now.
        DonateNow = 12,
        /// The call to action type is order now.
        OrderNow = 13,
        /// The call to action type is play now.
        PlayNow = 14,
        /// The call to action type is see more.
        SeeMore = 15,
        /// The call to action type is start now.
        StartNow = 16,
        /// The call to action type is visit site.
        VisitSite = 17,
        /// The call to action type is watch now.
        WatchNow = 18,
    }
    impl CallToActionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CallToActionType::Unspecified => "UNSPECIFIED",
                CallToActionType::Unknown => "UNKNOWN",
                CallToActionType::LearnMore => "LEARN_MORE",
                CallToActionType::GetQuote => "GET_QUOTE",
                CallToActionType::ApplyNow => "APPLY_NOW",
                CallToActionType::SignUp => "SIGN_UP",
                CallToActionType::ContactUs => "CONTACT_US",
                CallToActionType::Subscribe => "SUBSCRIBE",
                CallToActionType::Download => "DOWNLOAD",
                CallToActionType::BookNow => "BOOK_NOW",
                CallToActionType::ShopNow => "SHOP_NOW",
                CallToActionType::BuyNow => "BUY_NOW",
                CallToActionType::DonateNow => "DONATE_NOW",
                CallToActionType::OrderNow => "ORDER_NOW",
                CallToActionType::PlayNow => "PLAY_NOW",
                CallToActionType::SeeMore => "SEE_MORE",
                CallToActionType::StartNow => "START_NOW",
                CallToActionType::VisitSite => "VISIT_SITE",
                CallToActionType::WatchNow => "WATCH_NOW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "LEARN_MORE" => Some(Self::LearnMore),
                "GET_QUOTE" => Some(Self::GetQuote),
                "APPLY_NOW" => Some(Self::ApplyNow),
                "SIGN_UP" => Some(Self::SignUp),
                "CONTACT_US" => Some(Self::ContactUs),
                "SUBSCRIBE" => Some(Self::Subscribe),
                "DOWNLOAD" => Some(Self::Download),
                "BOOK_NOW" => Some(Self::BookNow),
                "SHOP_NOW" => Some(Self::ShopNow),
                "BUY_NOW" => Some(Self::BuyNow),
                "DONATE_NOW" => Some(Self::DonateNow),
                "ORDER_NOW" => Some(Self::OrderNow),
                "PLAY_NOW" => Some(Self::PlayNow),
                "SEE_MORE" => Some(Self::SeeMore),
                "START_NOW" => Some(Self::StartNow),
                "VISIT_SITE" => Some(Self::VisitSite),
                "WATCH_NOW" => Some(Self::WatchNow),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible types of a location ownership.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationOwnershipTypeEnum {}
/// Nested message and enum types in `LocationOwnershipTypeEnum`.
pub mod location_ownership_type_enum {
    /// Possible types of a location ownership.
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
    pub enum LocationOwnershipType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Business Owner of location(legacy location extension - LE).
        BusinessOwner = 2,
        /// Affiliate location(Third party location extension - ALE).
        Affiliate = 3,
    }
    impl LocationOwnershipType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LocationOwnershipType::Unspecified => "UNSPECIFIED",
                LocationOwnershipType::Unknown => "UNKNOWN",
                LocationOwnershipType::BusinessOwner => "BUSINESS_OWNER",
                LocationOwnershipType::Affiliate => "AFFILIATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "BUSINESS_OWNER" => Some(Self::BusinessOwner),
                "AFFILIATE" => Some(Self::Affiliate),
                _ => None,
            }
        }
    }
}
/// Container for enum describing the mime types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MimeTypeEnum {}
/// Nested message and enum types in `MimeTypeEnum`.
pub mod mime_type_enum {
    /// The mime type
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
    pub enum MimeType {
        /// The mime type has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// MIME type of image/jpeg.
        ImageJpeg = 2,
        /// MIME type of image/gif.
        ImageGif = 3,
        /// MIME type of image/png.
        ImagePng = 4,
        /// MIME type of application/x-shockwave-flash.
        Flash = 5,
        /// MIME type of text/html.
        TextHtml = 6,
        /// MIME type of application/pdf.
        Pdf = 7,
        /// MIME type of application/msword.
        Msword = 8,
        /// MIME type of application/vnd.ms-excel.
        Msexcel = 9,
        /// MIME type of application/rtf.
        Rtf = 10,
        /// MIME type of audio/wav.
        AudioWav = 11,
        /// MIME type of audio/mp3.
        AudioMp3 = 12,
        /// MIME type of application/x-html5-ad-zip.
        Html5AdZip = 13,
    }
    impl MimeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MimeType::Unspecified => "UNSPECIFIED",
                MimeType::Unknown => "UNKNOWN",
                MimeType::ImageJpeg => "IMAGE_JPEG",
                MimeType::ImageGif => "IMAGE_GIF",
                MimeType::ImagePng => "IMAGE_PNG",
                MimeType::Flash => "FLASH",
                MimeType::TextHtml => "TEXT_HTML",
                MimeType::Pdf => "PDF",
                MimeType::Msword => "MSWORD",
                MimeType::Msexcel => "MSEXCEL",
                MimeType::Rtf => "RTF",
                MimeType::AudioWav => "AUDIO_WAV",
                MimeType::AudioMp3 => "AUDIO_MP3",
                MimeType::Html5AdZip => "HTML5_AD_ZIP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "IMAGE_JPEG" => Some(Self::ImageJpeg),
                "IMAGE_GIF" => Some(Self::ImageGif),
                "IMAGE_PNG" => Some(Self::ImagePng),
                "FLASH" => Some(Self::Flash),
                "TEXT_HTML" => Some(Self::TextHtml),
                "PDF" => Some(Self::Pdf),
                "MSWORD" => Some(Self::Msword),
                "MSEXCEL" => Some(Self::Msexcel),
                "RTF" => Some(Self::Rtf),
                "AUDIO_WAV" => Some(Self::AudioWav),
                "AUDIO_MP3" => Some(Self::AudioMp3),
                "HTML5_AD_ZIP" => Some(Self::Html5AdZip),
                _ => None,
            }
        }
    }
}
/// Container for enum describing different types of mobile app vendors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileAppVendorEnum {}
/// Nested message and enum types in `MobileAppVendorEnum`.
pub mod mobile_app_vendor_enum {
    /// The type of mobile app vendor
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
    pub enum MobileAppVendor {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Mobile app vendor for Apple app store.
        AppleAppStore = 2,
        /// Mobile app vendor for Google app store.
        GoogleAppStore = 3,
    }
    impl MobileAppVendor {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MobileAppVendor::Unspecified => "UNSPECIFIED",
                MobileAppVendor::Unknown => "UNKNOWN",
                MobileAppVendor::AppleAppStore => "APPLE_APP_STORE",
                MobileAppVendor::GoogleAppStore => "GOOGLE_APP_STORE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "APPLE_APP_STORE" => Some(Self::AppleAppStore),
                "GOOGLE_APP_STORE" => Some(Self::GoogleAppStore),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible Asset engine statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetEngineStatusEnum {}
/// Nested message and enum types in `AssetEngineStatusEnum`.
pub mod asset_engine_status_enum {
    /// Next ID = 11
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
    pub enum AssetEngineStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The asset is active.
        Serving = 2,
        /// The asset is active limited.
        ServingLimited = 3,
        /// The asset is disapproved (not eligible).
        Disapproved = 4,
        /// The asset is inactive (pending).
        Disabled = 5,
        /// The asset has been removed.
        Removed = 6,
    }
    impl AssetEngineStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetEngineStatus::Unspecified => "UNSPECIFIED",
                AssetEngineStatus::Unknown => "UNKNOWN",
                AssetEngineStatus::Serving => "SERVING",
                AssetEngineStatus::ServingLimited => "SERVING_LIMITED",
                AssetEngineStatus::Disapproved => "DISAPPROVED",
                AssetEngineStatus::Disabled => "DISABLED",
                AssetEngineStatus::Removed => "REMOVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "SERVING" => Some(Self::Serving),
                "SERVING_LIMITED" => Some(Self::ServingLimited),
                "DISAPPROVED" => Some(Self::Disapproved),
                "DISABLED" => Some(Self::Disabled),
                "REMOVED" => Some(Self::Removed),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible statuses of an asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetStatusEnum {}
/// Nested message and enum types in `AssetStatusEnum`.
pub mod asset_status_enum {
    /// The possible statuses of an asset.
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
    pub enum AssetStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        /// This is a response-only value.
        Unknown = 1,
        /// The asset is enabled.
        Enabled = 2,
        /// The asset is removed.
        Removed = 3,
        /// The asset is archived.
        Archived = 4,
        /// The asset is system generated pending user review.
        PendingSystemGenerated = 5,
    }
    impl AssetStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetStatus::Unspecified => "UNSPECIFIED",
                AssetStatus::Unknown => "UNKNOWN",
                AssetStatus::Enabled => "ENABLED",
                AssetStatus::Removed => "REMOVED",
                AssetStatus::Archived => "ARCHIVED",
                AssetStatus::PendingSystemGenerated => "PENDING_SYSTEM_GENERATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "REMOVED" => Some(Self::Removed),
                "ARCHIVED" => Some(Self::Archived),
                "PENDING_SYSTEM_GENERATED" => Some(Self::PendingSystemGenerated),
                _ => None,
            }
        }
    }
}
/// Container for enum describing the types of asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetTypeEnum {}
/// Nested message and enum types in `AssetTypeEnum`.
pub mod asset_type_enum {
    /// Enum describing possible types of asset.
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
    pub enum AssetType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// YouTube video asset.
        YoutubeVideo = 2,
        /// Media bundle asset.
        MediaBundle = 3,
        /// Image asset.
        Image = 4,
        /// Text asset.
        Text = 5,
        /// Lead form asset.
        LeadForm = 6,
        /// Book on Google asset.
        BookOnGoogle = 7,
        /// Promotion asset.
        Promotion = 8,
        /// Callout asset.
        Callout = 9,
        /// Structured Snippet asset.
        StructuredSnippet = 10,
        /// Sitelink asset.
        Sitelink = 11,
        /// Page Feed asset.
        PageFeed = 12,
        /// Dynamic Education asset.
        DynamicEducation = 13,
        /// Mobile app asset.
        MobileApp = 14,
        /// Hotel callout asset.
        HotelCallout = 15,
        /// Call asset.
        Call = 16,
        /// Price asset.
        Price = 17,
        /// Call to action asset.
        CallToAction = 18,
        /// Dynamic real estate asset.
        DynamicRealEstate = 19,
        /// Dynamic custom asset.
        DynamicCustom = 20,
        /// Dynamic hotels and rentals asset.
        DynamicHotelsAndRentals = 21,
        /// Dynamic flights asset.
        DynamicFlights = 22,
        /// Discovery Carousel Card asset.
        DiscoveryCarouselCard = 23,
        /// Dynamic travel asset.
        DynamicTravel = 24,
        /// Dynamic local asset.
        DynamicLocal = 25,
        /// Dynamic jobs asset.
        DynamicJobs = 26,
        /// Location asset.
        Location = 27,
        /// Hotel property asset.
        HotelProperty = 28,
    }
    impl AssetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetType::Unspecified => "UNSPECIFIED",
                AssetType::Unknown => "UNKNOWN",
                AssetType::YoutubeVideo => "YOUTUBE_VIDEO",
                AssetType::MediaBundle => "MEDIA_BUNDLE",
                AssetType::Image => "IMAGE",
                AssetType::Text => "TEXT",
                AssetType::LeadForm => "LEAD_FORM",
                AssetType::BookOnGoogle => "BOOK_ON_GOOGLE",
                AssetType::Promotion => "PROMOTION",
                AssetType::Callout => "CALLOUT",
                AssetType::StructuredSnippet => "STRUCTURED_SNIPPET",
                AssetType::Sitelink => "SITELINK",
                AssetType::PageFeed => "PAGE_FEED",
                AssetType::DynamicEducation => "DYNAMIC_EDUCATION",
                AssetType::MobileApp => "MOBILE_APP",
                AssetType::HotelCallout => "HOTEL_CALLOUT",
                AssetType::Call => "CALL",
                AssetType::Price => "PRICE",
                AssetType::CallToAction => "CALL_TO_ACTION",
                AssetType::DynamicRealEstate => "DYNAMIC_REAL_ESTATE",
                AssetType::DynamicCustom => "DYNAMIC_CUSTOM",
                AssetType::DynamicHotelsAndRentals => "DYNAMIC_HOTELS_AND_RENTALS",
                AssetType::DynamicFlights => "DYNAMIC_FLIGHTS",
                AssetType::DiscoveryCarouselCard => "DISCOVERY_CAROUSEL_CARD",
                AssetType::DynamicTravel => "DYNAMIC_TRAVEL",
                AssetType::DynamicLocal => "DYNAMIC_LOCAL",
                AssetType::DynamicJobs => "DYNAMIC_JOBS",
                AssetType::Location => "LOCATION",
                AssetType::HotelProperty => "HOTEL_PROPERTY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "YOUTUBE_VIDEO" => Some(Self::YoutubeVideo),
                "MEDIA_BUNDLE" => Some(Self::MediaBundle),
                "IMAGE" => Some(Self::Image),
                "TEXT" => Some(Self::Text),
                "LEAD_FORM" => Some(Self::LeadForm),
                "BOOK_ON_GOOGLE" => Some(Self::BookOnGoogle),
                "PROMOTION" => Some(Self::Promotion),
                "CALLOUT" => Some(Self::Callout),
                "STRUCTURED_SNIPPET" => Some(Self::StructuredSnippet),
                "SITELINK" => Some(Self::Sitelink),
                "PAGE_FEED" => Some(Self::PageFeed),
                "DYNAMIC_EDUCATION" => Some(Self::DynamicEducation),
                "MOBILE_APP" => Some(Self::MobileApp),
                "HOTEL_CALLOUT" => Some(Self::HotelCallout),
                "CALL" => Some(Self::Call),
                "PRICE" => Some(Self::Price),
                "CALL_TO_ACTION" => Some(Self::CallToAction),
                "DYNAMIC_REAL_ESTATE" => Some(Self::DynamicRealEstate),
                "DYNAMIC_CUSTOM" => Some(Self::DynamicCustom),
                "DYNAMIC_HOTELS_AND_RENTALS" => Some(Self::DynamicHotelsAndRentals),
                "DYNAMIC_FLIGHTS" => Some(Self::DynamicFlights),
                "DISCOVERY_CAROUSEL_CARD" => Some(Self::DiscoveryCarouselCard),
                "DYNAMIC_TRAVEL" => Some(Self::DynamicTravel),
                "DYNAMIC_LOCAL" => Some(Self::DynamicLocal),
                "DYNAMIC_JOBS" => Some(Self::DynamicJobs),
                "LOCATION" => Some(Self::Location),
                "HOTEL_PROPERTY" => Some(Self::HotelProperty),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible ad strengths.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdStrengthEnum {}
/// Nested message and enum types in `AdStrengthEnum`.
pub mod ad_strength_enum {
    /// Enum listing the possible ad strengths.
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
    pub enum AdStrength {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The ad strength is currently pending.
        Pending = 2,
        /// No ads could be generated.
        NoAds = 3,
        /// Poor strength.
        Poor = 4,
        /// Average strength.
        Average = 5,
        /// Good strength.
        Good = 6,
        /// Excellent strength.
        Excellent = 7,
    }
    impl AdStrength {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdStrength::Unspecified => "UNSPECIFIED",
                AdStrength::Unknown => "UNKNOWN",
                AdStrength::Pending => "PENDING",
                AdStrength::NoAds => "NO_ADS",
                AdStrength::Poor => "POOR",
                AdStrength::Average => "AVERAGE",
                AdStrength::Good => "GOOD",
                AdStrength::Excellent => "EXCELLENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "PENDING" => Some(Self::Pending),
                "NO_ADS" => Some(Self::NoAds),
                "POOR" => Some(Self::Poor),
                "AVERAGE" => Some(Self::Average),
                "GOOD" => Some(Self::Good),
                "EXCELLENT" => Some(Self::Excellent),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible statuses of an asset group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroupStatusEnum {}
/// Nested message and enum types in `AssetGroupStatusEnum`.
pub mod asset_group_status_enum {
    /// The possible statuses of an asset group.
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
    pub enum AssetGroupStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        Unknown = 1,
        /// The asset group is enabled.
        Enabled = 2,
        /// The asset group is paused.
        Paused = 3,
        /// The asset group is removed.
        Removed = 4,
    }
    impl AssetGroupStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetGroupStatus::Unspecified => "UNSPECIFIED",
                AssetGroupStatus::Unknown => "UNKNOWN",
                AssetGroupStatus::Enabled => "ENABLED",
                AssetGroupStatus::Paused => "PAUSED",
                AssetGroupStatus::Removed => "REMOVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "PAUSED" => Some(Self::Paused),
                "REMOVED" => Some(Self::Removed),
                _ => None,
            }
        }
    }
}
/// Container for enum describing the possible placements of an asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetFieldTypeEnum {}
/// Nested message and enum types in `AssetFieldTypeEnum`.
pub mod asset_field_type_enum {
    /// Enum describing the possible placements of an asset.
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
    pub enum AssetFieldType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The asset is linked for use as a headline.
        Headline = 2,
        /// The asset is linked for use as a description.
        Description = 3,
        /// The asset is linked for use as mandatory ad text.
        MandatoryAdText = 4,
        /// The asset is linked for use as a marketing image.
        MarketingImage = 5,
        /// The asset is linked for use as a media bundle.
        MediaBundle = 6,
        /// The asset is linked for use as a YouTube video.
        YoutubeVideo = 7,
        /// The asset is linked to indicate that a hotels campaign is "Book on
        /// Google" enabled.
        BookOnGoogle = 8,
        /// The asset is linked for use as a Lead Form extension.
        LeadForm = 9,
        /// The asset is linked for use as a Promotion extension.
        Promotion = 10,
        /// The asset is linked for use as a Callout extension.
        Callout = 11,
        /// The asset is linked for use as a Structured Snippet extension.
        StructuredSnippet = 12,
        /// The asset is linked for use as a Sitelink.
        Sitelink = 13,
        /// The asset is linked for use as a Mobile App extension.
        MobileApp = 14,
        /// The asset is linked for use as a Hotel Callout extension.
        HotelCallout = 15,
        /// The asset is linked for use as a Call extension.
        Call = 16,
        /// The asset is linked for use as a Price extension.
        Price = 24,
        /// The asset is linked for use as a long headline.
        LongHeadline = 17,
        /// The asset is linked for use as a business name.
        BusinessName = 18,
        /// The asset is linked for use as a square marketing image.
        SquareMarketingImage = 19,
        /// The asset is linked for use as a portrait marketing image.
        PortraitMarketingImage = 20,
        /// The asset is linked for use as a logo.
        Logo = 21,
        /// The asset is linked for use as a landscape logo.
        LandscapeLogo = 22,
        /// The asset is linked for use as a non YouTube logo.
        Video = 23,
        /// The asset is linked for use to select a call-to-action.
        CallToActionSelection = 25,
        /// The asset is linked for use to select an ad image.
        AdImage = 26,
        /// The asset is linked for use as a business logo.
        BusinessLogo = 27,
        /// The asset is linked for use as a hotel property in a Performance Max for
        /// travel goals campaign.
        HotelProperty = 28,
    }
    impl AssetFieldType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetFieldType::Unspecified => "UNSPECIFIED",
                AssetFieldType::Unknown => "UNKNOWN",
                AssetFieldType::Headline => "HEADLINE",
                AssetFieldType::Description => "DESCRIPTION",
                AssetFieldType::MandatoryAdText => "MANDATORY_AD_TEXT",
                AssetFieldType::MarketingImage => "MARKETING_IMAGE",
                AssetFieldType::MediaBundle => "MEDIA_BUNDLE",
                AssetFieldType::YoutubeVideo => "YOUTUBE_VIDEO",
                AssetFieldType::BookOnGoogle => "BOOK_ON_GOOGLE",
                AssetFieldType::LeadForm => "LEAD_FORM",
                AssetFieldType::Promotion => "PROMOTION",
                AssetFieldType::Callout => "CALLOUT",
                AssetFieldType::StructuredSnippet => "STRUCTURED_SNIPPET",
                AssetFieldType::Sitelink => "SITELINK",
                AssetFieldType::MobileApp => "MOBILE_APP",
                AssetFieldType::HotelCallout => "HOTEL_CALLOUT",
                AssetFieldType::Call => "CALL",
                AssetFieldType::Price => "PRICE",
                AssetFieldType::LongHeadline => "LONG_HEADLINE",
                AssetFieldType::BusinessName => "BUSINESS_NAME",
                AssetFieldType::SquareMarketingImage => "SQUARE_MARKETING_IMAGE",
                AssetFieldType::PortraitMarketingImage => "PORTRAIT_MARKETING_IMAGE",
                AssetFieldType::Logo => "LOGO",
                AssetFieldType::LandscapeLogo => "LANDSCAPE_LOGO",
                AssetFieldType::Video => "VIDEO",
                AssetFieldType::CallToActionSelection => "CALL_TO_ACTION_SELECTION",
                AssetFieldType::AdImage => "AD_IMAGE",
                AssetFieldType::BusinessLogo => "BUSINESS_LOGO",
                AssetFieldType::HotelProperty => "HOTEL_PROPERTY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "HEADLINE" => Some(Self::Headline),
                "DESCRIPTION" => Some(Self::Description),
                "MANDATORY_AD_TEXT" => Some(Self::MandatoryAdText),
                "MARKETING_IMAGE" => Some(Self::MarketingImage),
                "MEDIA_BUNDLE" => Some(Self::MediaBundle),
                "YOUTUBE_VIDEO" => Some(Self::YoutubeVideo),
                "BOOK_ON_GOOGLE" => Some(Self::BookOnGoogle),
                "LEAD_FORM" => Some(Self::LeadForm),
                "PROMOTION" => Some(Self::Promotion),
                "CALLOUT" => Some(Self::Callout),
                "STRUCTURED_SNIPPET" => Some(Self::StructuredSnippet),
                "SITELINK" => Some(Self::Sitelink),
                "MOBILE_APP" => Some(Self::MobileApp),
                "HOTEL_CALLOUT" => Some(Self::HotelCallout),
                "CALL" => Some(Self::Call),
                "PRICE" => Some(Self::Price),
                "LONG_HEADLINE" => Some(Self::LongHeadline),
                "BUSINESS_NAME" => Some(Self::BusinessName),
                "SQUARE_MARKETING_IMAGE" => Some(Self::SquareMarketingImage),
                "PORTRAIT_MARKETING_IMAGE" => Some(Self::PortraitMarketingImage),
                "LOGO" => Some(Self::Logo),
                "LANDSCAPE_LOGO" => Some(Self::LandscapeLogo),
                "VIDEO" => Some(Self::Video),
                "CALL_TO_ACTION_SELECTION" => Some(Self::CallToActionSelection),
                "AD_IMAGE" => Some(Self::AdImage),
                "BUSINESS_LOGO" => Some(Self::BusinessLogo),
                "HOTEL_PROPERTY" => Some(Self::HotelProperty),
                _ => None,
            }
        }
    }
}
/// Container for enum describing the levels of bidding category used in
/// ListingGroupFilterDimension.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterBiddingCategoryLevelEnum {}
/// Nested message and enum types in `ListingGroupFilterBiddingCategoryLevelEnum`.
pub mod listing_group_filter_bidding_category_level_enum {
    /// The level of the listing group filter bidding category.
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
    pub enum ListingGroupFilterBiddingCategoryLevel {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Level 1.
        Level1 = 2,
        /// Level 2.
        Level2 = 3,
        /// Level 3.
        Level3 = 4,
        /// Level 4.
        Level4 = 5,
        /// Level 5.
        Level5 = 6,
    }
    impl ListingGroupFilterBiddingCategoryLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterBiddingCategoryLevel::Unspecified => "UNSPECIFIED",
                ListingGroupFilterBiddingCategoryLevel::Unknown => "UNKNOWN",
                ListingGroupFilterBiddingCategoryLevel::Level1 => "LEVEL1",
                ListingGroupFilterBiddingCategoryLevel::Level2 => "LEVEL2",
                ListingGroupFilterBiddingCategoryLevel::Level3 => "LEVEL3",
                ListingGroupFilterBiddingCategoryLevel::Level4 => "LEVEL4",
                ListingGroupFilterBiddingCategoryLevel::Level5 => "LEVEL5",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "LEVEL1" => Some(Self::Level1),
                "LEVEL2" => Some(Self::Level2),
                "LEVEL3" => Some(Self::Level3),
                "LEVEL4" => Some(Self::Level4),
                "LEVEL5" => Some(Self::Level5),
                _ => None,
            }
        }
    }
}
/// Container for enum describing the indexes of custom attribute used in
/// ListingGroupFilterDimension.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterCustomAttributeIndexEnum {}
/// Nested message and enum types in `ListingGroupFilterCustomAttributeIndexEnum`.
pub mod listing_group_filter_custom_attribute_index_enum {
    /// The index of customer attributes.
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
    pub enum ListingGroupFilterCustomAttributeIndex {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// First listing group filter custom attribute.
        Index0 = 2,
        /// Second listing group filter custom attribute.
        Index1 = 3,
        /// Third listing group filter custom attribute.
        Index2 = 4,
        /// Fourth listing group filter custom attribute.
        Index3 = 5,
        /// Fifth listing group filter custom attribute.
        Index4 = 6,
    }
    impl ListingGroupFilterCustomAttributeIndex {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterCustomAttributeIndex::Unspecified => "UNSPECIFIED",
                ListingGroupFilterCustomAttributeIndex::Unknown => "UNKNOWN",
                ListingGroupFilterCustomAttributeIndex::Index0 => "INDEX0",
                ListingGroupFilterCustomAttributeIndex::Index1 => "INDEX1",
                ListingGroupFilterCustomAttributeIndex::Index2 => "INDEX2",
                ListingGroupFilterCustomAttributeIndex::Index3 => "INDEX3",
                ListingGroupFilterCustomAttributeIndex::Index4 => "INDEX4",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "INDEX0" => Some(Self::Index0),
                "INDEX1" => Some(Self::Index1),
                "INDEX2" => Some(Self::Index2),
                "INDEX3" => Some(Self::Index3),
                "INDEX4" => Some(Self::Index4),
                _ => None,
            }
        }
    }
}
/// Locality of a product offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterProductChannelEnum {}
/// Nested message and enum types in `ListingGroupFilterProductChannelEnum`.
pub mod listing_group_filter_product_channel_enum {
    /// Enum describing the locality of a product offer.
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
    pub enum ListingGroupFilterProductChannel {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The item is sold online.
        Online = 2,
        /// The item is sold in local stores.
        Local = 3,
    }
    impl ListingGroupFilterProductChannel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterProductChannel::Unspecified => "UNSPECIFIED",
                ListingGroupFilterProductChannel::Unknown => "UNKNOWN",
                ListingGroupFilterProductChannel::Online => "ONLINE",
                ListingGroupFilterProductChannel::Local => "LOCAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ONLINE" => Some(Self::Online),
                "LOCAL" => Some(Self::Local),
                _ => None,
            }
        }
    }
}
/// Condition of a product offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterProductConditionEnum {}
/// Nested message and enum types in `ListingGroupFilterProductConditionEnum`.
pub mod listing_group_filter_product_condition_enum {
    /// Enum describing the condition of a product offer.
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
    pub enum ListingGroupFilterProductCondition {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The product condition is new.
        New = 2,
        /// The product condition is refurbished.
        Refurbished = 3,
        /// The product condition is used.
        Used = 4,
    }
    impl ListingGroupFilterProductCondition {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterProductCondition::Unspecified => "UNSPECIFIED",
                ListingGroupFilterProductCondition::Unknown => "UNKNOWN",
                ListingGroupFilterProductCondition::New => "NEW",
                ListingGroupFilterProductCondition::Refurbished => "REFURBISHED",
                ListingGroupFilterProductCondition::Used => "USED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "NEW" => Some(Self::New),
                "REFURBISHED" => Some(Self::Refurbished),
                "USED" => Some(Self::Used),
                _ => None,
            }
        }
    }
}
/// Level of the type of a product offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterProductTypeLevelEnum {}
/// Nested message and enum types in `ListingGroupFilterProductTypeLevelEnum`.
pub mod listing_group_filter_product_type_level_enum {
    /// Enum describing the level of the type of a product offer.
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
    pub enum ListingGroupFilterProductTypeLevel {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Level 1.
        Level1 = 2,
        /// Level 2.
        Level2 = 3,
        /// Level 3.
        Level3 = 4,
        /// Level 4.
        Level4 = 5,
        /// Level 5.
        Level5 = 6,
    }
    impl ListingGroupFilterProductTypeLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterProductTypeLevel::Unspecified => "UNSPECIFIED",
                ListingGroupFilterProductTypeLevel::Unknown => "UNKNOWN",
                ListingGroupFilterProductTypeLevel::Level1 => "LEVEL1",
                ListingGroupFilterProductTypeLevel::Level2 => "LEVEL2",
                ListingGroupFilterProductTypeLevel::Level3 => "LEVEL3",
                ListingGroupFilterProductTypeLevel::Level4 => "LEVEL4",
                ListingGroupFilterProductTypeLevel::Level5 => "LEVEL5",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "LEVEL1" => Some(Self::Level1),
                "LEVEL2" => Some(Self::Level2),
                "LEVEL3" => Some(Self::Level3),
                "LEVEL4" => Some(Self::Level4),
                "LEVEL5" => Some(Self::Level5),
                _ => None,
            }
        }
    }
}
/// Container for enum describing the type of the listing group filter node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterTypeEnum {}
/// Nested message and enum types in `ListingGroupFilterTypeEnum`.
pub mod listing_group_filter_type_enum {
    /// The type of the listing group filter.
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
    pub enum ListingGroupFilterType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Subdivision of products along some listing dimensions.
        Subdivision = 2,
        /// An included listing group filter leaf node.
        UnitIncluded = 3,
        /// An excluded listing group filter leaf node.
        UnitExcluded = 4,
    }
    impl ListingGroupFilterType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterType::Unspecified => "UNSPECIFIED",
                ListingGroupFilterType::Unknown => "UNKNOWN",
                ListingGroupFilterType::Subdivision => "SUBDIVISION",
                ListingGroupFilterType::UnitIncluded => "UNIT_INCLUDED",
                ListingGroupFilterType::UnitExcluded => "UNIT_EXCLUDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "SUBDIVISION" => Some(Self::Subdivision),
                "UNIT_INCLUDED" => Some(Self::UnitIncluded),
                "UNIT_EXCLUDED" => Some(Self::UnitExcluded),
                _ => None,
            }
        }
    }
}
/// Container for enum describing the type of the vertical a listing group filter
/// tree represents.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterVerticalEnum {}
/// Nested message and enum types in `ListingGroupFilterVerticalEnum`.
pub mod listing_group_filter_vertical_enum {
    /// The type of the listing group filter vertical.
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
    pub enum ListingGroupFilterVertical {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Represents the shopping vertical.
        Shopping = 2,
    }
    impl ListingGroupFilterVertical {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterVertical::Unspecified => "UNSPECIFIED",
                ListingGroupFilterVertical::Unknown => "UNKNOWN",
                ListingGroupFilterVertical::Shopping => "SHOPPING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "SHOPPING" => Some(Self::Shopping),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible asset field types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServedAssetFieldTypeEnum {}
/// Nested message and enum types in `ServedAssetFieldTypeEnum`.
pub mod served_asset_field_type_enum {
    /// The possible asset field types.
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
    pub enum ServedAssetFieldType {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The asset is used in headline 1.
        Headline1 = 2,
        /// The asset is used in headline 2.
        Headline2 = 3,
        /// The asset is used in headline 3.
        Headline3 = 4,
        /// The asset is used in description 1.
        Description1 = 5,
        /// The asset is used in description 2.
        Description2 = 6,
        /// The asset was used in a headline. Use this only if there is only one
        /// headline in the ad. Otherwise, use the HEADLINE_1, HEADLINE_2 or
        /// HEADLINE_3 enums
        Headline = 7,
        /// The asset was used as a headline in portrait image.
        HeadlineInPortrait = 8,
        /// The asset was used in a long headline (used in MultiAssetResponsiveAd).
        LongHeadline = 9,
        /// The asset was used in a description. Use this only if there is only one
        /// description in the ad. Otherwise, use the DESCRIPTION_1 or DESCRIPTION_@
        /// enums
        Description = 10,
        /// The asset was used as description in portrait image.
        DescriptionInPortrait = 11,
        /// The asset was used as business name in portrait image.
        BusinessNameInPortrait = 12,
        /// The asset was used as business name.
        BusinessName = 13,
        /// The asset was used as a marketing image.
        MarketingImage = 14,
        /// The asset was used as a marketing image in portrait image.
        MarketingImageInPortrait = 15,
        /// The asset was used as a square marketing image.
        SquareMarketingImage = 16,
        /// The asset was used as a portrait marketing image.
        PortraitMarketingImage = 17,
        /// The asset was used as a logo.
        Logo = 18,
        /// The asset was used as a landscape logo.
        LandscapeLogo = 19,
        /// The asset was used as a call-to-action.
        CallToAction = 20,
        /// The asset was used as a YouTube video.
        YouTubeVideo = 21,
        /// This asset is used as a sitelink.
        Sitelink = 22,
        /// This asset is used as a call.
        Call = 23,
        /// This asset is used as a mobile app.
        MobileApp = 24,
        /// This asset is used as a callout.
        Callout = 25,
        /// This asset is used as a structured snippet.
        StructuredSnippet = 26,
        /// This asset is used as a price.
        Price = 27,
        /// This asset is used as a promotion.
        Promotion = 28,
        /// This asset is used as an image.
        AdImage = 29,
        /// The asset is used as a lead form.
        LeadForm = 30,
        /// The asset is used as a business logo.
        BusinessLogo = 31,
    }
    impl ServedAssetFieldType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ServedAssetFieldType::Unspecified => "UNSPECIFIED",
                ServedAssetFieldType::Unknown => "UNKNOWN",
                ServedAssetFieldType::Headline1 => "HEADLINE_1",
                ServedAssetFieldType::Headline2 => "HEADLINE_2",
                ServedAssetFieldType::Headline3 => "HEADLINE_3",
                ServedAssetFieldType::Description1 => "DESCRIPTION_1",
                ServedAssetFieldType::Description2 => "DESCRIPTION_2",
                ServedAssetFieldType::Headline => "HEADLINE",
                ServedAssetFieldType::HeadlineInPortrait => "HEADLINE_IN_PORTRAIT",
                ServedAssetFieldType::LongHeadline => "LONG_HEADLINE",
                ServedAssetFieldType::Description => "DESCRIPTION",
                ServedAssetFieldType::DescriptionInPortrait => "DESCRIPTION_IN_PORTRAIT",
                ServedAssetFieldType::BusinessNameInPortrait => {
                    "BUSINESS_NAME_IN_PORTRAIT"
                }
                ServedAssetFieldType::BusinessName => "BUSINESS_NAME",
                ServedAssetFieldType::MarketingImage => "MARKETING_IMAGE",
                ServedAssetFieldType::MarketingImageInPortrait => {
                    "MARKETING_IMAGE_IN_PORTRAIT"
                }
                ServedAssetFieldType::SquareMarketingImage => "SQUARE_MARKETING_IMAGE",
                ServedAssetFieldType::PortraitMarketingImage => {
                    "PORTRAIT_MARKETING_IMAGE"
                }
                ServedAssetFieldType::Logo => "LOGO",
                ServedAssetFieldType::LandscapeLogo => "LANDSCAPE_LOGO",
                ServedAssetFieldType::CallToAction => "CALL_TO_ACTION",
                ServedAssetFieldType::YouTubeVideo => "YOU_TUBE_VIDEO",
                ServedAssetFieldType::Sitelink => "SITELINK",
                ServedAssetFieldType::Call => "CALL",
                ServedAssetFieldType::MobileApp => "MOBILE_APP",
                ServedAssetFieldType::Callout => "CALLOUT",
                ServedAssetFieldType::StructuredSnippet => "STRUCTURED_SNIPPET",
                ServedAssetFieldType::Price => "PRICE",
                ServedAssetFieldType::Promotion => "PROMOTION",
                ServedAssetFieldType::AdImage => "AD_IMAGE",
                ServedAssetFieldType::LeadForm => "LEAD_FORM",
                ServedAssetFieldType::BusinessLogo => "BUSINESS_LOGO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "HEADLINE_1" => Some(Self::Headline1),
                "HEADLINE_2" => Some(Self::Headline2),
                "HEADLINE_3" => Some(Self::Headline3),
                "DESCRIPTION_1" => Some(Self::Description1),
                "DESCRIPTION_2" => Some(Self::Description2),
                "HEADLINE" => Some(Self::Headline),
                "HEADLINE_IN_PORTRAIT" => Some(Self::HeadlineInPortrait),
                "LONG_HEADLINE" => Some(Self::LongHeadline),
                "DESCRIPTION" => Some(Self::Description),
                "DESCRIPTION_IN_PORTRAIT" => Some(Self::DescriptionInPortrait),
                "BUSINESS_NAME_IN_PORTRAIT" => Some(Self::BusinessNameInPortrait),
                "BUSINESS_NAME" => Some(Self::BusinessName),
                "MARKETING_IMAGE" => Some(Self::MarketingImage),
                "MARKETING_IMAGE_IN_PORTRAIT" => Some(Self::MarketingImageInPortrait),
                "SQUARE_MARKETING_IMAGE" => Some(Self::SquareMarketingImage),
                "PORTRAIT_MARKETING_IMAGE" => Some(Self::PortraitMarketingImage),
                "LOGO" => Some(Self::Logo),
                "LANDSCAPE_LOGO" => Some(Self::LandscapeLogo),
                "CALL_TO_ACTION" => Some(Self::CallToAction),
                "YOU_TUBE_VIDEO" => Some(Self::YouTubeVideo),
                "SITELINK" => Some(Self::Sitelink),
                "CALL" => Some(Self::Call),
                "MOBILE_APP" => Some(Self::MobileApp),
                "CALLOUT" => Some(Self::Callout),
                "STRUCTURED_SNIPPET" => Some(Self::StructuredSnippet),
                "PRICE" => Some(Self::Price),
                "PROMOTION" => Some(Self::Promotion),
                "AD_IMAGE" => Some(Self::AdImage),
                "LEAD_FORM" => Some(Self::LeadForm),
                "BUSINESS_LOGO" => Some(Self::BusinessLogo),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible statuses of an asset set asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSetAssetStatusEnum {}
/// Nested message and enum types in `AssetSetAssetStatusEnum`.
pub mod asset_set_asset_status_enum {
    /// The possible statuses of an asset set asset.
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
    pub enum AssetSetAssetStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        /// This is a response-only value.
        Unknown = 1,
        /// The asset set asset is enabled.
        Enabled = 2,
        /// The asset set asset is removed.
        Removed = 3,
    }
    impl AssetSetAssetStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetSetAssetStatus::Unspecified => "UNSPECIFIED",
                AssetSetAssetStatus::Unknown => "UNKNOWN",
                AssetSetAssetStatus::Enabled => "ENABLED",
                AssetSetAssetStatus::Removed => "REMOVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "REMOVED" => Some(Self::Removed),
                _ => None,
            }
        }
    }
}
/// Container for enum describing where on the first search results page the
/// automated bidding system should target impressions for the
/// TargetImpressionShare bidding strategy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetImpressionShareLocationEnum {}
/// Nested message and enum types in `TargetImpressionShareLocationEnum`.
pub mod target_impression_share_location_enum {
    /// Enum describing possible goals.
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
    pub enum TargetImpressionShareLocation {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Any location on the web page.
        AnywhereOnPage = 2,
        /// Top box of ads.
        TopOfPage = 3,
        /// Top slot in the top box of ads.
        AbsoluteTopOfPage = 4,
    }
    impl TargetImpressionShareLocation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TargetImpressionShareLocation::Unspecified => "UNSPECIFIED",
                TargetImpressionShareLocation::Unknown => "UNKNOWN",
                TargetImpressionShareLocation::AnywhereOnPage => "ANYWHERE_ON_PAGE",
                TargetImpressionShareLocation::TopOfPage => "TOP_OF_PAGE",
                TargetImpressionShareLocation::AbsoluteTopOfPage => {
                    "ABSOLUTE_TOP_OF_PAGE"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ANYWHERE_ON_PAGE" => Some(Self::AnywhereOnPage),
                "TOP_OF_PAGE" => Some(Self::TopOfPage),
                "ABSOLUTE_TOP_OF_PAGE" => Some(Self::AbsoluteTopOfPage),
                _ => None,
            }
        }
    }
}
/// Message describing BiddingStrategy statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategyStatusEnum {}
/// Nested message and enum types in `BiddingStrategyStatusEnum`.
pub mod bidding_strategy_status_enum {
    /// The possible statuses of a BiddingStrategy.
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
    pub enum BiddingStrategyStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The bidding strategy is enabled.
        Enabled = 2,
        /// The bidding strategy is removed.
        Removed = 4,
    }
    impl BiddingStrategyStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BiddingStrategyStatus::Unspecified => "UNSPECIFIED",
                BiddingStrategyStatus::Unknown => "UNKNOWN",
                BiddingStrategyStatus::Enabled => "ENABLED",
                BiddingStrategyStatus::Removed => "REMOVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "REMOVED" => Some(Self::Removed),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible bidding strategy types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategyTypeEnum {}
/// Nested message and enum types in `BiddingStrategyTypeEnum`.
pub mod bidding_strategy_type_enum {
    /// Enum describing possible bidding strategy types.
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
    pub enum BiddingStrategyType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Commission is an automatic bidding strategy in which the advertiser pays
        /// a certain portion of the conversion value.
        Commission = 16,
        /// Enhanced CPC is a bidding strategy that raises bids for clicks
        /// that seem more likely to lead to a conversion and lowers
        /// them for clicks where they seem less likely.
        EnhancedCpc = 2,
        /// Used for return value only. Indicates that a campaign does not have a
        /// bidding strategy. This prevents the campaign from serving. For example,
        /// a campaign may be attached to a manager bidding strategy and the serving
        /// account is subsequently unlinked from the manager account. In this case
        /// the campaign will automatically be detached from the now inaccessible
        /// manager bidding strategy and transition to the INVALID bidding strategy
        /// type.
        Invalid = 17,
        /// Manual bidding strategy that allows advertiser to set the bid per
        /// advertiser-specified action.
        ManualCpa = 18,
        /// Manual click based bidding where user pays per click.
        ManualCpc = 3,
        /// Manual impression based bidding
        /// where user pays per thousand impressions.
        ManualCpm = 4,
        /// A bidding strategy that pays a configurable amount per video view.
        ManualCpv = 13,
        /// A bidding strategy that automatically maximizes number of conversions
        /// given a daily budget.
        MaximizeConversions = 10,
        /// An automated bidding strategy that automatically sets bids to maximize
        /// revenue while spending your budget.
        MaximizeConversionValue = 11,
        /// Page-One Promoted bidding scheme, which sets max cpc bids to
        /// target impressions on page one or page one promoted slots on google.com.
        /// This enum value is deprecated.
        PageOnePromoted = 5,
        /// Percent Cpc is bidding strategy where bids are a fraction of the
        /// advertised price for some good or service.
        PercentCpc = 12,
        /// Target CPA is an automated bid strategy that sets bids
        /// to help get as many conversions as possible
        /// at the target cost-per-acquisition (CPA) you set.
        TargetCpa = 6,
        /// Target CPM is an automated bid strategy that sets bids to help get
        /// as many impressions as possible at the target cost per one thousand
        /// impressions (CPM) you set.
        TargetCpm = 14,
        /// An automated bidding strategy that sets bids so that a certain percentage
        /// of search ads are shown at the top of the first page (or other targeted
        /// location).
        TargetImpressionShare = 15,
        /// Target Outrank Share is an automated bidding strategy that sets bids
        /// based on the target fraction of auctions where the advertiser
        /// should outrank a specific competitor.
        /// This enum value is deprecated.
        TargetOutrankShare = 7,
        /// Target ROAS is an automated bidding strategy
        /// that helps you maximize revenue while averaging
        /// a specific target Return On Average Spend (ROAS).
        TargetRoas = 8,
        /// Target Spend is an automated bid strategy that sets your bids
        /// to help get as many clicks as possible within your budget.
        TargetSpend = 9,
    }
    impl BiddingStrategyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BiddingStrategyType::Unspecified => "UNSPECIFIED",
                BiddingStrategyType::Unknown => "UNKNOWN",
                BiddingStrategyType::Commission => "COMMISSION",
                BiddingStrategyType::EnhancedCpc => "ENHANCED_CPC",
                BiddingStrategyType::Invalid => "INVALID",
                BiddingStrategyType::ManualCpa => "MANUAL_CPA",
                BiddingStrategyType::ManualCpc => "MANUAL_CPC",
                BiddingStrategyType::ManualCpm => "MANUAL_CPM",
                BiddingStrategyType::ManualCpv => "MANUAL_CPV",
                BiddingStrategyType::MaximizeConversions => "MAXIMIZE_CONVERSIONS",
                BiddingStrategyType::MaximizeConversionValue => {
                    "MAXIMIZE_CONVERSION_VALUE"
                }
                BiddingStrategyType::PageOnePromoted => "PAGE_ONE_PROMOTED",
                BiddingStrategyType::PercentCpc => "PERCENT_CPC",
                BiddingStrategyType::TargetCpa => "TARGET_CPA",
                BiddingStrategyType::TargetCpm => "TARGET_CPM",
                BiddingStrategyType::TargetImpressionShare => "TARGET_IMPRESSION_SHARE",
                BiddingStrategyType::TargetOutrankShare => "TARGET_OUTRANK_SHARE",
                BiddingStrategyType::TargetRoas => "TARGET_ROAS",
                BiddingStrategyType::TargetSpend => "TARGET_SPEND",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "COMMISSION" => Some(Self::Commission),
                "ENHANCED_CPC" => Some(Self::EnhancedCpc),
                "INVALID" => Some(Self::Invalid),
                "MANUAL_CPA" => Some(Self::ManualCpa),
                "MANUAL_CPC" => Some(Self::ManualCpc),
                "MANUAL_CPM" => Some(Self::ManualCpm),
                "MANUAL_CPV" => Some(Self::ManualCpv),
                "MAXIMIZE_CONVERSIONS" => Some(Self::MaximizeConversions),
                "MAXIMIZE_CONVERSION_VALUE" => Some(Self::MaximizeConversionValue),
                "PAGE_ONE_PROMOTED" => Some(Self::PageOnePromoted),
                "PERCENT_CPC" => Some(Self::PercentCpc),
                "TARGET_CPA" => Some(Self::TargetCpa),
                "TARGET_CPM" => Some(Self::TargetCpm),
                "TARGET_IMPRESSION_SHARE" => Some(Self::TargetImpressionShare),
                "TARGET_OUTRANK_SHARE" => Some(Self::TargetOutrankShare),
                "TARGET_ROAS" => Some(Self::TargetRoas),
                "TARGET_SPEND" => Some(Self::TargetSpend),
                _ => None,
            }
        }
    }
}
/// Possible ad serving statuses of a campaign.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdServingOptimizationStatusEnum {}
/// Nested message and enum types in `AdServingOptimizationStatusEnum`.
pub mod ad_serving_optimization_status_enum {
    /// Enum describing possible serving statuses.
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
    pub enum AdServingOptimizationStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// Ad serving is optimized based on CTR for the campaign.
        Optimize = 2,
        /// Ad serving is optimized based on CTR * Conversion for the campaign. If
        /// the campaign is not in the conversion optimizer bidding strategy, it will
        /// default to OPTIMIZED.
        ConversionOptimize = 3,
        /// Ads are rotated evenly for 90 days, then optimized for clicks.
        Rotate = 4,
        /// Show lower performing ads more evenly with higher performing ads, and do
        /// not optimize.
        RotateIndefinitely = 5,
        /// Ad serving optimization status is not available.
        Unavailable = 6,
    }
    impl AdServingOptimizationStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdServingOptimizationStatus::Unspecified => "UNSPECIFIED",
                AdServingOptimizationStatus::Unknown => "UNKNOWN",
                AdServingOptimizationStatus::Optimize => "OPTIMIZE",
                AdServingOptimizationStatus::ConversionOptimize => "CONVERSION_OPTIMIZE",
                AdServingOptimizationStatus::Rotate => "ROTATE",
                AdServingOptimizationStatus::RotateIndefinitely => "ROTATE_INDEFINITELY",
                AdServingOptimizationStatus::Unavailable => "UNAVAILABLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "OPTIMIZE" => Some(Self::Optimize),
                "CONVERSION_OPTIMIZE" => Some(Self::ConversionOptimize),
                "ROTATE" => Some(Self::Rotate),
                "ROTATE_INDEFINITELY" => Some(Self::RotateIndefinitely),
                "UNAVAILABLE" => Some(Self::Unavailable),
                _ => None,
            }
        }
    }
}
/// An immutable specialization of an Advertising Channel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvertisingChannelSubTypeEnum {}
/// Nested message and enum types in `AdvertisingChannelSubTypeEnum`.
pub mod advertising_channel_sub_type_enum {
    /// Enum describing the different channel subtypes.
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
    pub enum AdvertisingChannelSubType {
        /// Not specified.
        Unspecified = 0,
        /// Used as a return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Mobile app campaigns for Search.
        SearchMobileApp = 2,
        /// Mobile app campaigns for Display.
        DisplayMobileApp = 3,
        /// AdWords express campaigns for search.
        SearchExpress = 4,
        /// AdWords Express campaigns for display.
        DisplayExpress = 5,
        /// Smart Shopping campaigns.
        ShoppingSmartAds = 6,
        /// Gmail Ad campaigns.
        DisplayGmailAd = 7,
        /// Smart display campaigns. New campaigns of this sub type cannot be
        /// created.
        DisplaySmartCampaign = 8,
        /// Video Outstream campaigns.
        VideoOutstream = 9,
        /// Video TrueView for Action campaigns.
        VideoAction = 10,
        /// Video campaigns with non-skippable video ads.
        VideoNonSkippable = 11,
        /// App Campaign that lets you easily promote your Android or iOS app
        /// across Google's top properties including Search, Play, YouTube, and the
        /// Google Display Network.
        AppCampaign = 12,
        /// App Campaign for engagement, focused on driving re-engagement with the
        /// app across several of Google's top properties including Search, YouTube,
        /// and the Google Display Network.
        AppCampaignForEngagement = 13,
        /// Campaigns specialized for local advertising.
        LocalCampaign = 14,
        /// Shopping Comparison Listing campaigns.
        ShoppingComparisonListingAds = 15,
        /// Standard Smart campaigns.
        SmartCampaign = 16,
        /// Video campaigns with sequence video ads.
        VideoSequence = 17,
        /// App Campaign for pre registration, specialized for advertising mobile
        /// app pre-registration, that targets multiple advertising channels across
        /// Google Play, YouTube and Display Network.
        AppCampaignForPreRegistration = 18,
        /// Video reach campaign with Target Frequency bidding strategy.
        VideoReachTargetFrequency = 19,
        /// Travel Activities campaigns.
        TravelActivities = 20,
    }
    impl AdvertisingChannelSubType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdvertisingChannelSubType::Unspecified => "UNSPECIFIED",
                AdvertisingChannelSubType::Unknown => "UNKNOWN",
                AdvertisingChannelSubType::SearchMobileApp => "SEARCH_MOBILE_APP",
                AdvertisingChannelSubType::DisplayMobileApp => "DISPLAY_MOBILE_APP",
                AdvertisingChannelSubType::SearchExpress => "SEARCH_EXPRESS",
                AdvertisingChannelSubType::DisplayExpress => "DISPLAY_EXPRESS",
                AdvertisingChannelSubType::ShoppingSmartAds => "SHOPPING_SMART_ADS",
                AdvertisingChannelSubType::DisplayGmailAd => "DISPLAY_GMAIL_AD",
                AdvertisingChannelSubType::DisplaySmartCampaign => {
                    "DISPLAY_SMART_CAMPAIGN"
                }
                AdvertisingChannelSubType::VideoOutstream => "VIDEO_OUTSTREAM",
                AdvertisingChannelSubType::VideoAction => "VIDEO_ACTION",
                AdvertisingChannelSubType::VideoNonSkippable => "VIDEO_NON_SKIPPABLE",
                AdvertisingChannelSubType::AppCampaign => "APP_CAMPAIGN",
                AdvertisingChannelSubType::AppCampaignForEngagement => {
                    "APP_CAMPAIGN_FOR_ENGAGEMENT"
                }
                AdvertisingChannelSubType::LocalCampaign => "LOCAL_CAMPAIGN",
                AdvertisingChannelSubType::ShoppingComparisonListingAds => {
                    "SHOPPING_COMPARISON_LISTING_ADS"
                }
                AdvertisingChannelSubType::SmartCampaign => "SMART_CAMPAIGN",
                AdvertisingChannelSubType::VideoSequence => "VIDEO_SEQUENCE",
                AdvertisingChannelSubType::AppCampaignForPreRegistration => {
                    "APP_CAMPAIGN_FOR_PRE_REGISTRATION"
                }
                AdvertisingChannelSubType::VideoReachTargetFrequency => {
                    "VIDEO_REACH_TARGET_FREQUENCY"
                }
                AdvertisingChannelSubType::TravelActivities => "TRAVEL_ACTIVITIES",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "SEARCH_MOBILE_APP" => Some(Self::SearchMobileApp),
                "DISPLAY_MOBILE_APP" => Some(Self::DisplayMobileApp),
                "SEARCH_EXPRESS" => Some(Self::SearchExpress),
                "DISPLAY_EXPRESS" => Some(Self::DisplayExpress),
                "SHOPPING_SMART_ADS" => Some(Self::ShoppingSmartAds),
                "DISPLAY_GMAIL_AD" => Some(Self::DisplayGmailAd),
                "DISPLAY_SMART_CAMPAIGN" => Some(Self::DisplaySmartCampaign),
                "VIDEO_OUTSTREAM" => Some(Self::VideoOutstream),
                "VIDEO_ACTION" => Some(Self::VideoAction),
                "VIDEO_NON_SKIPPABLE" => Some(Self::VideoNonSkippable),
                "APP_CAMPAIGN" => Some(Self::AppCampaign),
                "APP_CAMPAIGN_FOR_ENGAGEMENT" => Some(Self::AppCampaignForEngagement),
                "LOCAL_CAMPAIGN" => Some(Self::LocalCampaign),
                "SHOPPING_COMPARISON_LISTING_ADS" => {
                    Some(Self::ShoppingComparisonListingAds)
                }
                "SMART_CAMPAIGN" => Some(Self::SmartCampaign),
                "VIDEO_SEQUENCE" => Some(Self::VideoSequence),
                "APP_CAMPAIGN_FOR_PRE_REGISTRATION" => {
                    Some(Self::AppCampaignForPreRegistration)
                }
                "VIDEO_REACH_TARGET_FREQUENCY" => Some(Self::VideoReachTargetFrequency),
                "TRAVEL_ACTIVITIES" => Some(Self::TravelActivities),
                _ => None,
            }
        }
    }
}
/// The channel type a campaign may target to serve on.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvertisingChannelTypeEnum {}
/// Nested message and enum types in `AdvertisingChannelTypeEnum`.
pub mod advertising_channel_type_enum {
    /// Enum describing the various advertising channel types.
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
    pub enum AdvertisingChannelType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Search Network. Includes display bundled, and Search+ campaigns.
        Search = 2,
        /// Google Display Network only.
        Display = 3,
        /// Shopping campaigns serve on the shopping property
        /// and on google.com search results.
        Shopping = 4,
        /// Hotel Ads campaigns.
        Hotel = 5,
        /// Video campaigns.
        Video = 6,
        /// App Campaigns, and App Campaigns for Engagement, that run
        /// across multiple channels.
        MultiChannel = 7,
        /// Local ads campaigns.
        Local = 8,
        /// Smart campaigns.
        Smart = 9,
        /// Performance Max campaigns.
        PerformanceMax = 10,
        /// Local services campaigns.
        LocalServices = 11,
        /// Discovery campaigns.
        Discovery = 12,
        /// Travel campaigns.
        Travel = 13,
    }
    impl AdvertisingChannelType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdvertisingChannelType::Unspecified => "UNSPECIFIED",
                AdvertisingChannelType::Unknown => "UNKNOWN",
                AdvertisingChannelType::Search => "SEARCH",
                AdvertisingChannelType::Display => "DISPLAY",
                AdvertisingChannelType::Shopping => "SHOPPING",
                AdvertisingChannelType::Hotel => "HOTEL",
                AdvertisingChannelType::Video => "VIDEO",
                AdvertisingChannelType::MultiChannel => "MULTI_CHANNEL",
                AdvertisingChannelType::Local => "LOCAL",
                AdvertisingChannelType::Smart => "SMART",
                AdvertisingChannelType::PerformanceMax => "PERFORMANCE_MAX",
                AdvertisingChannelType::LocalServices => "LOCAL_SERVICES",
                AdvertisingChannelType::Discovery => "DISCOVERY",
                AdvertisingChannelType::Travel => "TRAVEL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "SEARCH" => Some(Self::Search),
                "DISPLAY" => Some(Self::Display),
                "SHOPPING" => Some(Self::Shopping),
                "HOTEL" => Some(Self::Hotel),
                "VIDEO" => Some(Self::Video),
                "MULTI_CHANNEL" => Some(Self::MultiChannel),
                "LOCAL" => Some(Self::Local),
                "SMART" => Some(Self::Smart),
                "PERFORMANCE_MAX" => Some(Self::PerformanceMax),
                "LOCAL_SERVICES" => Some(Self::LocalServices),
                "DISCOVERY" => Some(Self::Discovery),
                "TRAVEL" => Some(Self::Travel),
                _ => None,
            }
        }
    }
}
/// Message describing BiddingStrategy system statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategySystemStatusEnum {}
/// Nested message and enum types in `BiddingStrategySystemStatusEnum`.
pub mod bidding_strategy_system_status_enum {
    /// The possible system statuses of a BiddingStrategy.
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
    pub enum BiddingStrategySystemStatus {
        /// Signals that an unexpected error occurred, for example, no bidding
        /// strategy type was found, or no status information was found.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The bid strategy is active, and AdWords cannot find any specific issues
        /// with the strategy.
        Enabled = 2,
        /// The bid strategy is learning because it has been recently created or
        /// recently reactivated.
        LearningNew = 3,
        /// The bid strategy is learning because of a recent setting change.
        LearningSettingChange = 4,
        /// The bid strategy is learning because of a recent budget change.
        LearningBudgetChange = 5,
        /// The bid strategy is learning because of recent change in number of
        /// campaigns, ad groups or keywords attached to it.
        LearningCompositionChange = 6,
        /// The bid strategy depends on conversion reporting and the customer
        /// recently modified conversion types that were relevant to the
        /// bid strategy.
        LearningConversionTypeChange = 7,
        /// The bid strategy depends on conversion reporting and the customer
        /// recently changed their conversion settings.
        LearningConversionSettingChange = 8,
        /// The bid strategy is limited by its bid ceiling.
        LimitedByCpcBidCeiling = 9,
        /// The bid strategy is limited by its bid floor.
        LimitedByCpcBidFloor = 10,
        /// The bid strategy is limited because there was not enough conversion
        /// traffic over the past weeks.
        LimitedByData = 11,
        /// A significant fraction of keywords in this bid strategy are limited by
        /// budget.
        LimitedByBudget = 12,
        /// The bid strategy cannot reach its target spend because its spend has
        /// been de-prioritized.
        LimitedByLowPrioritySpend = 13,
        /// A significant fraction of keywords in this bid strategy have a low
        /// Quality Score.
        LimitedByLowQuality = 14,
        /// The bid strategy cannot fully spend its budget because of narrow
        /// targeting.
        LimitedByInventory = 15,
        /// Missing conversion tracking (no pings present) and/or remarketing lists
        /// for SSC.
        MisconfiguredZeroEligibility = 16,
        /// The bid strategy depends on conversion reporting and the customer is
        /// lacking conversion types that might be reported against this strategy.
        MisconfiguredConversionTypes = 17,
        /// The bid strategy depends on conversion reporting and the customer's
        /// conversion settings are misconfigured.
        MisconfiguredConversionSettings = 18,
        /// There are campaigns outside the bid strategy that share budgets with
        /// campaigns included in the strategy.
        MisconfiguredSharedBudget = 19,
        /// The campaign has an invalid strategy type and is not serving.
        MisconfiguredStrategyType = 20,
        /// The bid strategy is not active. Either there are no active campaigns,
        /// ad groups or keywords attached to the bid strategy. Or there are no
        /// active budgets connected to the bid strategy.
        Paused = 21,
        /// This bid strategy currently does not support status reporting.
        Unavailable = 22,
        /// There were multiple LEARNING_* system statuses for this bid strategy
        /// during the time in question.
        MultipleLearning = 23,
        /// There were multiple LIMITED_* system statuses for this bid strategy
        /// during the time in question.
        MultipleLimited = 24,
        /// There were multiple MISCONFIGURED_* system statuses for this bid strategy
        /// during the time in question.
        MultipleMisconfigured = 25,
        /// There were multiple system statuses for this bid strategy during the
        /// time in question.
        Multiple = 26,
    }
    impl BiddingStrategySystemStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BiddingStrategySystemStatus::Unspecified => "UNSPECIFIED",
                BiddingStrategySystemStatus::Unknown => "UNKNOWN",
                BiddingStrategySystemStatus::Enabled => "ENABLED",
                BiddingStrategySystemStatus::LearningNew => "LEARNING_NEW",
                BiddingStrategySystemStatus::LearningSettingChange => {
                    "LEARNING_SETTING_CHANGE"
                }
                BiddingStrategySystemStatus::LearningBudgetChange => {
                    "LEARNING_BUDGET_CHANGE"
                }
                BiddingStrategySystemStatus::LearningCompositionChange => {
                    "LEARNING_COMPOSITION_CHANGE"
                }
                BiddingStrategySystemStatus::LearningConversionTypeChange => {
                    "LEARNING_CONVERSION_TYPE_CHANGE"
                }
                BiddingStrategySystemStatus::LearningConversionSettingChange => {
                    "LEARNING_CONVERSION_SETTING_CHANGE"
                }
                BiddingStrategySystemStatus::LimitedByCpcBidCeiling => {
                    "LIMITED_BY_CPC_BID_CEILING"
                }
                BiddingStrategySystemStatus::LimitedByCpcBidFloor => {
                    "LIMITED_BY_CPC_BID_FLOOR"
                }
                BiddingStrategySystemStatus::LimitedByData => "LIMITED_BY_DATA",
                BiddingStrategySystemStatus::LimitedByBudget => "LIMITED_BY_BUDGET",
                BiddingStrategySystemStatus::LimitedByLowPrioritySpend => {
                    "LIMITED_BY_LOW_PRIORITY_SPEND"
                }
                BiddingStrategySystemStatus::LimitedByLowQuality => {
                    "LIMITED_BY_LOW_QUALITY"
                }
                BiddingStrategySystemStatus::LimitedByInventory => "LIMITED_BY_INVENTORY",
                BiddingStrategySystemStatus::MisconfiguredZeroEligibility => {
                    "MISCONFIGURED_ZERO_ELIGIBILITY"
                }
                BiddingStrategySystemStatus::MisconfiguredConversionTypes => {
                    "MISCONFIGURED_CONVERSION_TYPES"
                }
                BiddingStrategySystemStatus::MisconfiguredConversionSettings => {
                    "MISCONFIGURED_CONVERSION_SETTINGS"
                }
                BiddingStrategySystemStatus::MisconfiguredSharedBudget => {
                    "MISCONFIGURED_SHARED_BUDGET"
                }
                BiddingStrategySystemStatus::MisconfiguredStrategyType => {
                    "MISCONFIGURED_STRATEGY_TYPE"
                }
                BiddingStrategySystemStatus::Paused => "PAUSED",
                BiddingStrategySystemStatus::Unavailable => "UNAVAILABLE",
                BiddingStrategySystemStatus::MultipleLearning => "MULTIPLE_LEARNING",
                BiddingStrategySystemStatus::MultipleLimited => "MULTIPLE_LIMITED",
                BiddingStrategySystemStatus::MultipleMisconfigured => {
                    "MULTIPLE_MISCONFIGURED"
                }
                BiddingStrategySystemStatus::Multiple => "MULTIPLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "LEARNING_NEW" => Some(Self::LearningNew),
                "LEARNING_SETTING_CHANGE" => Some(Self::LearningSettingChange),
                "LEARNING_BUDGET_CHANGE" => Some(Self::LearningBudgetChange),
                "LEARNING_COMPOSITION_CHANGE" => Some(Self::LearningCompositionChange),
                "LEARNING_CONVERSION_TYPE_CHANGE" => {
                    Some(Self::LearningConversionTypeChange)
                }
                "LEARNING_CONVERSION_SETTING_CHANGE" => {
                    Some(Self::LearningConversionSettingChange)
                }
                "LIMITED_BY_CPC_BID_CEILING" => Some(Self::LimitedByCpcBidCeiling),
                "LIMITED_BY_CPC_BID_FLOOR" => Some(Self::LimitedByCpcBidFloor),
                "LIMITED_BY_DATA" => Some(Self::LimitedByData),
                "LIMITED_BY_BUDGET" => Some(Self::LimitedByBudget),
                "LIMITED_BY_LOW_PRIORITY_SPEND" => Some(Self::LimitedByLowPrioritySpend),
                "LIMITED_BY_LOW_QUALITY" => Some(Self::LimitedByLowQuality),
                "LIMITED_BY_INVENTORY" => Some(Self::LimitedByInventory),
                "MISCONFIGURED_ZERO_ELIGIBILITY" => {
                    Some(Self::MisconfiguredZeroEligibility)
                }
                "MISCONFIGURED_CONVERSION_TYPES" => {
                    Some(Self::MisconfiguredConversionTypes)
                }
                "MISCONFIGURED_CONVERSION_SETTINGS" => {
                    Some(Self::MisconfiguredConversionSettings)
                }
                "MISCONFIGURED_SHARED_BUDGET" => Some(Self::MisconfiguredSharedBudget),
                "MISCONFIGURED_STRATEGY_TYPE" => Some(Self::MisconfiguredStrategyType),
                "PAUSED" => Some(Self::Paused),
                "UNAVAILABLE" => Some(Self::Unavailable),
                "MULTIPLE_LEARNING" => Some(Self::MultipleLearning),
                "MULTIPLE_LIMITED" => Some(Self::MultipleLimited),
                "MULTIPLE_MISCONFIGURED" => Some(Self::MultipleMisconfigured),
                "MULTIPLE" => Some(Self::Multiple),
                _ => None,
            }
        }
    }
}
/// Message describing Campaign serving statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignServingStatusEnum {}
/// Nested message and enum types in `CampaignServingStatusEnum`.
pub mod campaign_serving_status_enum {
    /// Possible serving statuses of a campaign.
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
    pub enum CampaignServingStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// Serving.
        Serving = 2,
        /// None.
        None = 3,
        /// Ended.
        Ended = 4,
        /// Pending.
        Pending = 5,
        /// Suspended.
        Suspended = 6,
    }
    impl CampaignServingStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CampaignServingStatus::Unspecified => "UNSPECIFIED",
                CampaignServingStatus::Unknown => "UNKNOWN",
                CampaignServingStatus::Serving => "SERVING",
                CampaignServingStatus::None => "NONE",
                CampaignServingStatus::Ended => "ENDED",
                CampaignServingStatus::Pending => "PENDING",
                CampaignServingStatus::Suspended => "SUSPENDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "SERVING" => Some(Self::Serving),
                "NONE" => Some(Self::None),
                "ENDED" => Some(Self::Ended),
                "PENDING" => Some(Self::Pending),
                "SUSPENDED" => Some(Self::Suspended),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible statuses of a campaign.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignStatusEnum {}
/// Nested message and enum types in `CampaignStatusEnum`.
pub mod campaign_status_enum {
    /// Possible statuses of a campaign.
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
    pub enum CampaignStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Campaign is active and can show ads.
        Enabled = 2,
        /// Campaign has been paused by the user.
        Paused = 3,
        /// Campaign has been removed.
        Removed = 4,
    }
    impl CampaignStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CampaignStatus::Unspecified => "UNSPECIFIED",
                CampaignStatus::Unknown => "UNKNOWN",
                CampaignStatus::Enabled => "ENABLED",
                CampaignStatus::Paused => "PAUSED",
                CampaignStatus::Removed => "REMOVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "PAUSED" => Some(Self::Paused),
                "REMOVED" => Some(Self::Removed),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible negative geo target types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NegativeGeoTargetTypeEnum {}
/// Nested message and enum types in `NegativeGeoTargetTypeEnum`.
pub mod negative_geo_target_type_enum {
    /// The possible negative geo target types.
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
    pub enum NegativeGeoTargetType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Specifies that a user is excluded from seeing the ad if they
        /// are in, or show interest in, advertiser's excluded locations.
        PresenceOrInterest = 4,
        /// Specifies that a user is excluded from seeing the ad if they
        /// are in advertiser's excluded locations.
        Presence = 5,
    }
    impl NegativeGeoTargetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NegativeGeoTargetType::Unspecified => "UNSPECIFIED",
                NegativeGeoTargetType::Unknown => "UNKNOWN",
                NegativeGeoTargetType::PresenceOrInterest => "PRESENCE_OR_INTEREST",
                NegativeGeoTargetType::Presence => "PRESENCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "PRESENCE_OR_INTEREST" => Some(Self::PresenceOrInterest),
                "PRESENCE" => Some(Self::Presence),
                _ => None,
            }
        }
    }
}
/// Container for enum describing the type of optimization goal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizationGoalTypeEnum {}
/// Nested message and enum types in `OptimizationGoalTypeEnum`.
pub mod optimization_goal_type_enum {
    /// The type of optimization goal
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
    pub enum OptimizationGoalType {
        /// Not specified.
        Unspecified = 0,
        /// Used as a return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Optimize for call clicks. Call click conversions are times people
        /// selected 'Call' to contact a store after viewing an ad.
        CallClicks = 2,
        /// Optimize for driving directions. Driving directions conversions are
        /// times people selected 'Get directions' to navigate to a store after
        /// viewing an ad.
        DrivingDirections = 3,
        /// Optimize for pre-registration. Pre-registration conversions are the
        /// number of pre-registration signups to receive a notification when the app
        /// is released.
        AppPreRegistration = 4,
    }
    impl OptimizationGoalType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OptimizationGoalType::Unspecified => "UNSPECIFIED",
                OptimizationGoalType::Unknown => "UNKNOWN",
                OptimizationGoalType::CallClicks => "CALL_CLICKS",
                OptimizationGoalType::DrivingDirections => "DRIVING_DIRECTIONS",
                OptimizationGoalType::AppPreRegistration => "APP_PRE_REGISTRATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "CALL_CLICKS" => Some(Self::CallClicks),
                "DRIVING_DIRECTIONS" => Some(Self::DrivingDirections),
                "APP_PRE_REGISTRATION" => Some(Self::AppPreRegistration),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible positive geo target types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositiveGeoTargetTypeEnum {}
/// Nested message and enum types in `PositiveGeoTargetTypeEnum`.
pub mod positive_geo_target_type_enum {
    /// The possible positive geo target types.
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
    pub enum PositiveGeoTargetType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Specifies that an ad is triggered if the user is in,
        /// or shows interest in, advertiser's targeted locations.
        PresenceOrInterest = 5,
        /// Specifies that an ad is triggered if the user
        /// searches for advertiser's targeted locations.
        /// This can only be used with Search and standard
        /// Shopping campaigns.
        SearchInterest = 6,
        /// Specifies that an ad is triggered if the user is in
        /// or regularly in advertiser's targeted locations.
        Presence = 7,
    }
    impl PositiveGeoTargetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PositiveGeoTargetType::Unspecified => "UNSPECIFIED",
                PositiveGeoTargetType::Unknown => "UNKNOWN",
                PositiveGeoTargetType::PresenceOrInterest => "PRESENCE_OR_INTEREST",
                PositiveGeoTargetType::SearchInterest => "SEARCH_INTEREST",
                PositiveGeoTargetType::Presence => "PRESENCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "PRESENCE_OR_INTEREST" => Some(Self::PresenceOrInterest),
                "SEARCH_INTEREST" => Some(Self::SearchInterest),
                "PRESENCE" => Some(Self::Presence),
                _ => None,
            }
        }
    }
}
/// Message describing Budget delivery methods. A delivery method determines the
/// rate at which the Budget is spent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetDeliveryMethodEnum {}
/// Nested message and enum types in `BudgetDeliveryMethodEnum`.
pub mod budget_delivery_method_enum {
    /// Possible delivery methods of a Budget.
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
    pub enum BudgetDeliveryMethod {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The budget server will throttle serving evenly across
        /// the entire time period.
        Standard = 2,
        /// The budget server will not throttle serving,
        /// and ads will serve as fast as possible.
        Accelerated = 3,
    }
    impl BudgetDeliveryMethod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BudgetDeliveryMethod::Unspecified => "UNSPECIFIED",
                BudgetDeliveryMethod::Unknown => "UNKNOWN",
                BudgetDeliveryMethod::Standard => "STANDARD",
                BudgetDeliveryMethod::Accelerated => "ACCELERATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "STANDARD" => Some(Self::Standard),
                "ACCELERATED" => Some(Self::Accelerated),
                _ => None,
            }
        }
    }
}
/// Message describing Budget period.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetPeriodEnum {}
/// Nested message and enum types in `BudgetPeriodEnum`.
pub mod budget_period_enum {
    /// Possible period of a Budget.
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
    pub enum BudgetPeriod {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Daily budget.
        Daily = 2,
        /// Fixed daily budget.
        FixedDaily = 4,
        /// Custom budget can be used with total_amount to specify lifetime budget
        /// limit.
        CustomPeriod = 5,
    }
    impl BudgetPeriod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BudgetPeriod::Unspecified => "UNSPECIFIED",
                BudgetPeriod::Unknown => "UNKNOWN",
                BudgetPeriod::Daily => "DAILY",
                BudgetPeriod::FixedDaily => "FIXED_DAILY",
                BudgetPeriod::CustomPeriod => "CUSTOM_PERIOD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "DAILY" => Some(Self::Daily),
                "FIXED_DAILY" => Some(Self::FixedDaily),
                "CUSTOM_PERIOD" => Some(Self::CustomPeriod),
                _ => None,
            }
        }
    }
}
/// Message describing CampaignCriterion statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignCriterionStatusEnum {}
/// Nested message and enum types in `CampaignCriterionStatusEnum`.
pub mod campaign_criterion_status_enum {
    /// The possible statuses of a CampaignCriterion.
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
    pub enum CampaignCriterionStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The campaign criterion is enabled.
        Enabled = 2,
        /// The campaign criterion is paused.
        Paused = 3,
        /// The campaign criterion is removed.
        Removed = 4,
    }
    impl CampaignCriterionStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CampaignCriterionStatus::Unspecified => "UNSPECIFIED",
                CampaignCriterionStatus::Unknown => "UNKNOWN",
                CampaignCriterionStatus::Enabled => "ENABLED",
                CampaignCriterionStatus::Paused => "PAUSED",
                CampaignCriterionStatus::Removed => "REMOVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "PAUSED" => Some(Self::Paused),
                "REMOVED" => Some(Self::Removed),
                _ => None,
            }
        }
    }
}
/// Container for enum describing conversion attribution type in SearchAds360
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributionTypeEnum {}
/// Nested message and enum types in `AttributionTypeEnum`.
pub mod attribution_type_enum {
    /// Conversion attribution type in SearchAds360
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
    pub enum AttributionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The conversion is attributed to a visit.
        Visit = 2,
        /// The conversion is attributed to a criterion and ad pair.
        CriterionAd = 3,
    }
    impl AttributionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AttributionType::Unspecified => "UNSPECIFIED",
                AttributionType::Unknown => "UNKNOWN",
                AttributionType::Visit => "VISIT",
                AttributionType::CriterionAd => "CRITERION_AD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "VISIT" => Some(Self::Visit),
                "CRITERION_AD" => Some(Self::CriterionAd),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible statuses of a conversion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionStatusEnum {}
/// Nested message and enum types in `ConversionStatusEnum`.
pub mod conversion_status_enum {
    /// Possible statuses of a conversion.
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
    pub enum ConversionStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The conversion is enabled.
        Enabled = 2,
        /// The conversion has been removed.
        Removed = 3,
    }
    impl ConversionStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionStatus::Unspecified => "UNSPECIFIED",
                ConversionStatus::Unknown => "UNKNOWN",
                ConversionStatus::Enabled => "ENABLED",
                ConversionStatus::Removed => "REMOVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "REMOVED" => Some(Self::Removed),
                _ => None,
            }
        }
    }
}
/// Container for enum representing the attribution model that describes how to
/// distribute credit for a particular conversion across potentially many prior
/// interactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributionModelEnum {}
/// Nested message and enum types in `AttributionModelEnum`.
pub mod attribution_model_enum {
    /// The attribution model that describes how to distribute credit for a
    /// particular conversion across potentially many prior interactions.
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
    pub enum AttributionModel {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Uses external attribution.
        External = 100,
        /// Attributes all credit for a conversion to its last click.
        GoogleAdsLastClick = 101,
        /// Attributes all credit for a conversion to its first click using Google
        /// Search attribution.
        GoogleSearchAttributionFirstClick = 102,
        /// Attributes credit for a conversion equally across all of its clicks using
        /// Google Search attribution.
        GoogleSearchAttributionLinear = 103,
        /// Attributes exponentially more credit for a conversion to its more recent
        /// clicks using Google Search attribution (half-life is 1 week).
        GoogleSearchAttributionTimeDecay = 104,
        /// Attributes 40% of the credit for a conversion to its first and last
        /// clicks. Remaining 20% is evenly distributed across all other clicks. This
        /// uses Google Search attribution.
        GoogleSearchAttributionPositionBased = 105,
        /// Flexible model that uses machine learning to determine the appropriate
        /// distribution of credit among clicks using Google Search attribution.
        GoogleSearchAttributionDataDriven = 106,
    }
    impl AttributionModel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AttributionModel::Unspecified => "UNSPECIFIED",
                AttributionModel::Unknown => "UNKNOWN",
                AttributionModel::External => "EXTERNAL",
                AttributionModel::GoogleAdsLastClick => "GOOGLE_ADS_LAST_CLICK",
                AttributionModel::GoogleSearchAttributionFirstClick => {
                    "GOOGLE_SEARCH_ATTRIBUTION_FIRST_CLICK"
                }
                AttributionModel::GoogleSearchAttributionLinear => {
                    "GOOGLE_SEARCH_ATTRIBUTION_LINEAR"
                }
                AttributionModel::GoogleSearchAttributionTimeDecay => {
                    "GOOGLE_SEARCH_ATTRIBUTION_TIME_DECAY"
                }
                AttributionModel::GoogleSearchAttributionPositionBased => {
                    "GOOGLE_SEARCH_ATTRIBUTION_POSITION_BASED"
                }
                AttributionModel::GoogleSearchAttributionDataDriven => {
                    "GOOGLE_SEARCH_ATTRIBUTION_DATA_DRIVEN"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "EXTERNAL" => Some(Self::External),
                "GOOGLE_ADS_LAST_CLICK" => Some(Self::GoogleAdsLastClick),
                "GOOGLE_SEARCH_ATTRIBUTION_FIRST_CLICK" => {
                    Some(Self::GoogleSearchAttributionFirstClick)
                }
                "GOOGLE_SEARCH_ATTRIBUTION_LINEAR" => {
                    Some(Self::GoogleSearchAttributionLinear)
                }
                "GOOGLE_SEARCH_ATTRIBUTION_TIME_DECAY" => {
                    Some(Self::GoogleSearchAttributionTimeDecay)
                }
                "GOOGLE_SEARCH_ATTRIBUTION_POSITION_BASED" => {
                    Some(Self::GoogleSearchAttributionPositionBased)
                }
                "GOOGLE_SEARCH_ATTRIBUTION_DATA_DRIVEN" => {
                    Some(Self::GoogleSearchAttributionDataDriven)
                }
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible statuses of a conversion action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionStatusEnum {}
/// Nested message and enum types in `ConversionActionStatusEnum`.
pub mod conversion_action_status_enum {
    /// Possible statuses of a conversion action.
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
    pub enum ConversionActionStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Conversions will be recorded.
        Enabled = 2,
        /// Conversions will not be recorded.
        Removed = 3,
        /// Conversions will not be recorded and the conversion action will not
        /// appear in the UI.
        Hidden = 4,
    }
    impl ConversionActionStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionActionStatus::Unspecified => "UNSPECIFIED",
                ConversionActionStatus::Unknown => "UNKNOWN",
                ConversionActionStatus::Enabled => "ENABLED",
                ConversionActionStatus::Removed => "REMOVED",
                ConversionActionStatus::Hidden => "HIDDEN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "REMOVED" => Some(Self::Removed),
                "HIDDEN" => Some(Self::Hidden),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible types of a conversion action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionTypeEnum {}
/// Nested message and enum types in `ConversionActionTypeEnum`.
pub mod conversion_action_type_enum {
    /// Possible types of a conversion action.
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
    pub enum ConversionActionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Conversions that occur when a user clicks on an ad's call extension.
        AdCall = 2,
        /// Conversions that occur when a user on a mobile device clicks a phone
        /// number.
        ClickToCall = 3,
        /// Conversions that occur when a user downloads a mobile app from the Google
        /// Play Store.
        GooglePlayDownload = 4,
        /// Conversions that occur when a user makes a purchase in an app through
        /// Android billing.
        GooglePlayInAppPurchase = 5,
        /// Call conversions that are tracked by the advertiser and uploaded.
        UploadCalls = 6,
        /// Conversions that are tracked by the advertiser and uploaded with
        /// attributed clicks.
        UploadClicks = 7,
        /// Conversions that occur on a webpage.
        Webpage = 8,
        /// Conversions that occur when a user calls a dynamically-generated phone
        /// number from an advertiser's website.
        WebsiteCall = 9,
        /// Store Sales conversion based on first-party or third-party merchant
        /// data uploads.
        /// Only customers on the allowlist can use store sales direct upload types.
        StoreSalesDirectUpload = 10,
        /// Store Sales conversion based on first-party or third-party merchant
        /// data uploads and/or from in-store purchases using cards from payment
        /// networks.
        /// Only customers on the allowlist can use store sales types.
        /// Read only.
        StoreSales = 11,
        /// Android app first open conversions tracked through Firebase.
        FirebaseAndroidFirstOpen = 12,
        /// Android app in app purchase conversions tracked through Firebase.
        FirebaseAndroidInAppPurchase = 13,
        /// Android app custom conversions tracked through Firebase.
        FirebaseAndroidCustom = 14,
        /// iOS app first open conversions tracked through Firebase.
        FirebaseIosFirstOpen = 15,
        /// iOS app in app purchase conversions tracked through Firebase.
        FirebaseIosInAppPurchase = 16,
        /// iOS app custom conversions tracked through Firebase.
        FirebaseIosCustom = 17,
        /// Android app first open conversions tracked through Third Party App
        /// Analytics.
        ThirdPartyAppAnalyticsAndroidFirstOpen = 18,
        /// Android app in app purchase conversions tracked through Third Party App
        /// Analytics.
        ThirdPartyAppAnalyticsAndroidInAppPurchase = 19,
        /// Android app custom conversions tracked through Third Party App Analytics.
        ThirdPartyAppAnalyticsAndroidCustom = 20,
        /// iOS app first open conversions tracked through Third Party App Analytics.
        ThirdPartyAppAnalyticsIosFirstOpen = 21,
        /// iOS app in app purchase conversions tracked through Third Party App
        /// Analytics.
        ThirdPartyAppAnalyticsIosInAppPurchase = 22,
        /// iOS app custom conversions tracked through Third Party App Analytics.
        ThirdPartyAppAnalyticsIosCustom = 23,
        /// Conversions that occur when a user pre-registers a mobile app from the
        /// Google Play Store. Read only.
        AndroidAppPreRegistration = 24,
        /// Conversions that track all Google Play downloads which aren't tracked
        /// by an app-specific type. Read only.
        AndroidInstallsAllOtherApps = 25,
        /// Floodlight activity that counts the number of times that users have
        /// visited a particular webpage after seeing or clicking on one of
        /// an advertiser's ads. Read only.
        FloodlightAction = 26,
        /// Floodlight activity that tracks the number of sales made or the number
        /// of items purchased. Can also capture the total value of each sale.
        /// Read only.
        FloodlightTransaction = 27,
        /// Conversions that track local actions from Google's products and
        /// services after interacting with an ad. Read only.
        GoogleHosted = 28,
        /// Conversions reported when a user submits a lead form. Read only.
        LeadFormSubmit = 29,
        /// Conversions that come from Salesforce. Read only.
        Salesforce = 30,
        /// Conversions imported from Search Ads 360 Floodlight data. Read only.
        SearchAds360 = 31,
        /// Call conversions that occur on Smart campaign Ads without call tracking
        /// setup, using Smart campaign custom criteria. Read only.
        SmartCampaignAdClicksToCall = 32,
        /// The user clicks on a call element within Google Maps. Smart campaign
        /// only. Read only.
        SmartCampaignMapClicksToCall = 33,
        /// The user requests directions to a business location within Google Maps.
        /// Smart campaign only. Read only.
        SmartCampaignMapDirections = 34,
        /// Call conversions that occur on Smart campaign Ads with call tracking
        /// setup, using Smart campaign custom criteria. Read only.
        SmartCampaignTrackedCalls = 35,
        /// Conversions that occur when a user visits an advertiser's retail store.
        /// Read only.
        StoreVisits = 36,
        /// Conversions created from website events (such as form submissions or page
        /// loads), that don't use individually coded event snippets. Read only.
        WebpageCodeless = 37,
        /// Conversions that come from linked Universal Analytics goals.
        UniversalAnalyticsGoal = 38,
        /// Conversions that come from linked Universal Analytics transactions.
        UniversalAnalyticsTransaction = 39,
        /// Conversions that come from linked Google Analytics 4 custom event
        /// conversions.
        GoogleAnalytics4Custom = 40,
        /// Conversions that come from linked Google Analytics 4 purchase
        /// conversions.
        GoogleAnalytics4Purchase = 41,
    }
    impl ConversionActionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionActionType::Unspecified => "UNSPECIFIED",
                ConversionActionType::Unknown => "UNKNOWN",
                ConversionActionType::AdCall => "AD_CALL",
                ConversionActionType::ClickToCall => "CLICK_TO_CALL",
                ConversionActionType::GooglePlayDownload => "GOOGLE_PLAY_DOWNLOAD",
                ConversionActionType::GooglePlayInAppPurchase => {
                    "GOOGLE_PLAY_IN_APP_PURCHASE"
                }
                ConversionActionType::UploadCalls => "UPLOAD_CALLS",
                ConversionActionType::UploadClicks => "UPLOAD_CLICKS",
                ConversionActionType::Webpage => "WEBPAGE",
                ConversionActionType::WebsiteCall => "WEBSITE_CALL",
                ConversionActionType::StoreSalesDirectUpload => {
                    "STORE_SALES_DIRECT_UPLOAD"
                }
                ConversionActionType::StoreSales => "STORE_SALES",
                ConversionActionType::FirebaseAndroidFirstOpen => {
                    "FIREBASE_ANDROID_FIRST_OPEN"
                }
                ConversionActionType::FirebaseAndroidInAppPurchase => {
                    "FIREBASE_ANDROID_IN_APP_PURCHASE"
                }
                ConversionActionType::FirebaseAndroidCustom => "FIREBASE_ANDROID_CUSTOM",
                ConversionActionType::FirebaseIosFirstOpen => "FIREBASE_IOS_FIRST_OPEN",
                ConversionActionType::FirebaseIosInAppPurchase => {
                    "FIREBASE_IOS_IN_APP_PURCHASE"
                }
                ConversionActionType::FirebaseIosCustom => "FIREBASE_IOS_CUSTOM",
                ConversionActionType::ThirdPartyAppAnalyticsAndroidFirstOpen => {
                    "THIRD_PARTY_APP_ANALYTICS_ANDROID_FIRST_OPEN"
                }
                ConversionActionType::ThirdPartyAppAnalyticsAndroidInAppPurchase => {
                    "THIRD_PARTY_APP_ANALYTICS_ANDROID_IN_APP_PURCHASE"
                }
                ConversionActionType::ThirdPartyAppAnalyticsAndroidCustom => {
                    "THIRD_PARTY_APP_ANALYTICS_ANDROID_CUSTOM"
                }
                ConversionActionType::ThirdPartyAppAnalyticsIosFirstOpen => {
                    "THIRD_PARTY_APP_ANALYTICS_IOS_FIRST_OPEN"
                }
                ConversionActionType::ThirdPartyAppAnalyticsIosInAppPurchase => {
                    "THIRD_PARTY_APP_ANALYTICS_IOS_IN_APP_PURCHASE"
                }
                ConversionActionType::ThirdPartyAppAnalyticsIosCustom => {
                    "THIRD_PARTY_APP_ANALYTICS_IOS_CUSTOM"
                }
                ConversionActionType::AndroidAppPreRegistration => {
                    "ANDROID_APP_PRE_REGISTRATION"
                }
                ConversionActionType::AndroidInstallsAllOtherApps => {
                    "ANDROID_INSTALLS_ALL_OTHER_APPS"
                }
                ConversionActionType::FloodlightAction => "FLOODLIGHT_ACTION",
                ConversionActionType::FloodlightTransaction => "FLOODLIGHT_TRANSACTION",
                ConversionActionType::GoogleHosted => "GOOGLE_HOSTED",
                ConversionActionType::LeadFormSubmit => "LEAD_FORM_SUBMIT",
                ConversionActionType::Salesforce => "SALESFORCE",
                ConversionActionType::SearchAds360 => "SEARCH_ADS_360",
                ConversionActionType::SmartCampaignAdClicksToCall => {
                    "SMART_CAMPAIGN_AD_CLICKS_TO_CALL"
                }
                ConversionActionType::SmartCampaignMapClicksToCall => {
                    "SMART_CAMPAIGN_MAP_CLICKS_TO_CALL"
                }
                ConversionActionType::SmartCampaignMapDirections => {
                    "SMART_CAMPAIGN_MAP_DIRECTIONS"
                }
                ConversionActionType::SmartCampaignTrackedCalls => {
                    "SMART_CAMPAIGN_TRACKED_CALLS"
                }
                ConversionActionType::StoreVisits => "STORE_VISITS",
                ConversionActionType::WebpageCodeless => "WEBPAGE_CODELESS",
                ConversionActionType::UniversalAnalyticsGoal => {
                    "UNIVERSAL_ANALYTICS_GOAL"
                }
                ConversionActionType::UniversalAnalyticsTransaction => {
                    "UNIVERSAL_ANALYTICS_TRANSACTION"
                }
                ConversionActionType::GoogleAnalytics4Custom => {
                    "GOOGLE_ANALYTICS_4_CUSTOM"
                }
                ConversionActionType::GoogleAnalytics4Purchase => {
                    "GOOGLE_ANALYTICS_4_PURCHASE"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "AD_CALL" => Some(Self::AdCall),
                "CLICK_TO_CALL" => Some(Self::ClickToCall),
                "GOOGLE_PLAY_DOWNLOAD" => Some(Self::GooglePlayDownload),
                "GOOGLE_PLAY_IN_APP_PURCHASE" => Some(Self::GooglePlayInAppPurchase),
                "UPLOAD_CALLS" => Some(Self::UploadCalls),
                "UPLOAD_CLICKS" => Some(Self::UploadClicks),
                "WEBPAGE" => Some(Self::Webpage),
                "WEBSITE_CALL" => Some(Self::WebsiteCall),
                "STORE_SALES_DIRECT_UPLOAD" => Some(Self::StoreSalesDirectUpload),
                "STORE_SALES" => Some(Self::StoreSales),
                "FIREBASE_ANDROID_FIRST_OPEN" => Some(Self::FirebaseAndroidFirstOpen),
                "FIREBASE_ANDROID_IN_APP_PURCHASE" => {
                    Some(Self::FirebaseAndroidInAppPurchase)
                }
                "FIREBASE_ANDROID_CUSTOM" => Some(Self::FirebaseAndroidCustom),
                "FIREBASE_IOS_FIRST_OPEN" => Some(Self::FirebaseIosFirstOpen),
                "FIREBASE_IOS_IN_APP_PURCHASE" => Some(Self::FirebaseIosInAppPurchase),
                "FIREBASE_IOS_CUSTOM" => Some(Self::FirebaseIosCustom),
                "THIRD_PARTY_APP_ANALYTICS_ANDROID_FIRST_OPEN" => {
                    Some(Self::ThirdPartyAppAnalyticsAndroidFirstOpen)
                }
                "THIRD_PARTY_APP_ANALYTICS_ANDROID_IN_APP_PURCHASE" => {
                    Some(Self::ThirdPartyAppAnalyticsAndroidInAppPurchase)
                }
                "THIRD_PARTY_APP_ANALYTICS_ANDROID_CUSTOM" => {
                    Some(Self::ThirdPartyAppAnalyticsAndroidCustom)
                }
                "THIRD_PARTY_APP_ANALYTICS_IOS_FIRST_OPEN" => {
                    Some(Self::ThirdPartyAppAnalyticsIosFirstOpen)
                }
                "THIRD_PARTY_APP_ANALYTICS_IOS_IN_APP_PURCHASE" => {
                    Some(Self::ThirdPartyAppAnalyticsIosInAppPurchase)
                }
                "THIRD_PARTY_APP_ANALYTICS_IOS_CUSTOM" => {
                    Some(Self::ThirdPartyAppAnalyticsIosCustom)
                }
                "ANDROID_APP_PRE_REGISTRATION" => Some(Self::AndroidAppPreRegistration),
                "ANDROID_INSTALLS_ALL_OTHER_APPS" => {
                    Some(Self::AndroidInstallsAllOtherApps)
                }
                "FLOODLIGHT_ACTION" => Some(Self::FloodlightAction),
                "FLOODLIGHT_TRANSACTION" => Some(Self::FloodlightTransaction),
                "GOOGLE_HOSTED" => Some(Self::GoogleHosted),
                "LEAD_FORM_SUBMIT" => Some(Self::LeadFormSubmit),
                "SALESFORCE" => Some(Self::Salesforce),
                "SEARCH_ADS_360" => Some(Self::SearchAds360),
                "SMART_CAMPAIGN_AD_CLICKS_TO_CALL" => {
                    Some(Self::SmartCampaignAdClicksToCall)
                }
                "SMART_CAMPAIGN_MAP_CLICKS_TO_CALL" => {
                    Some(Self::SmartCampaignMapClicksToCall)
                }
                "SMART_CAMPAIGN_MAP_DIRECTIONS" => Some(Self::SmartCampaignMapDirections),
                "SMART_CAMPAIGN_TRACKED_CALLS" => Some(Self::SmartCampaignTrackedCalls),
                "STORE_VISITS" => Some(Self::StoreVisits),
                "WEBPAGE_CODELESS" => Some(Self::WebpageCodeless),
                "UNIVERSAL_ANALYTICS_GOAL" => Some(Self::UniversalAnalyticsGoal),
                "UNIVERSAL_ANALYTICS_TRANSACTION" => {
                    Some(Self::UniversalAnalyticsTransaction)
                }
                "GOOGLE_ANALYTICS_4_CUSTOM" => Some(Self::GoogleAnalytics4Custom),
                "GOOGLE_ANALYTICS_4_PURCHASE" => Some(Self::GoogleAnalytics4Purchase),
                _ => None,
            }
        }
    }
}
/// Container for enum indicating data driven model status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataDrivenModelStatusEnum {}
/// Nested message and enum types in `DataDrivenModelStatusEnum`.
pub mod data_driven_model_status_enum {
    /// Enumerates data driven model statuses.
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
    pub enum DataDrivenModelStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The data driven model is available.
        Available = 2,
        /// The data driven model is stale. It hasn't been updated for at least 7
        /// days. It is still being used, but will become expired if it does not get
        /// updated for 30 days.
        Stale = 3,
        /// The data driven model expired. It hasn't been updated for at least 30
        /// days and cannot be used. Most commonly this is because there hasn't been
        /// the required number of events in a recent 30-day period.
        Expired = 4,
        /// The data driven model has never been generated. Most commonly this is
        /// because there has never been the required number of events in any 30-day
        /// period.
        NeverGenerated = 5,
    }
    impl DataDrivenModelStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DataDrivenModelStatus::Unspecified => "UNSPECIFIED",
                DataDrivenModelStatus::Unknown => "UNKNOWN",
                DataDrivenModelStatus::Available => "AVAILABLE",
                DataDrivenModelStatus::Stale => "STALE",
                DataDrivenModelStatus::Expired => "EXPIRED",
                DataDrivenModelStatus::NeverGenerated => "NEVER_GENERATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "AVAILABLE" => Some(Self::Available),
                "STALE" => Some(Self::Stale),
                "EXPIRED" => Some(Self::Expired),
                "NEVER_GENERATED" => Some(Self::NeverGenerated),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible statuses of an account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountStatusEnum {}
/// Nested message and enum types in `AccountStatusEnum`.
pub mod account_status_enum {
    /// Possible statuses of an account.
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
    pub enum AccountStatus {
        /// Default value.
        Unspecified = 0,
        /// Unknown value.
        Unknown = 1,
        /// Account is able to serve ads.
        Enabled = 2,
        /// Account is deactivated by the user.
        Paused = 3,
        /// Account is deactivated by an internal process.
        Suspended = 4,
        /// Account is irrevocably deactivated.
        Removed = 5,
        /// Account is still in the process of setup, not ENABLED yet.
        Draft = 6,
    }
    impl AccountStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccountStatus::Unspecified => "UNSPECIFIED",
                AccountStatus::Unknown => "UNKNOWN",
                AccountStatus::Enabled => "ENABLED",
                AccountStatus::Paused => "PAUSED",
                AccountStatus::Suspended => "SUSPENDED",
                AccountStatus::Removed => "REMOVED",
                AccountStatus::Draft => "DRAFT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "PAUSED" => Some(Self::Paused),
                "SUSPENDED" => Some(Self::Suspended),
                "REMOVED" => Some(Self::Removed),
                "DRAFT" => Some(Self::Draft),
                _ => None,
            }
        }
    }
}
/// Container for enum describing engine account types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountTypeEnum {}
/// Nested message and enum types in `AccountTypeEnum`.
pub mod account_type_enum {
    /// Possible engine account types.
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
    pub enum AccountType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Baidu account.
        Baidu = 2,
        /// Engine track account.
        EngineTrack = 3,
        /// Facebook account.
        Facebook = 4,
        /// Facebook account managed through gateway.
        FacebookGateway = 5,
        /// Google Ads account.
        GoogleAds = 6,
        /// Microsoft Advertising account.
        Microsoft = 7,
        /// Search Ads 360 manager account.
        SearchAds360 = 8,
        /// Yahoo Japan account.
        YahooJapan = 9,
    }
    impl AccountType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccountType::Unspecified => "UNSPECIFIED",
                AccountType::Unknown => "UNKNOWN",
                AccountType::Baidu => "BAIDU",
                AccountType::EngineTrack => "ENGINE_TRACK",
                AccountType::Facebook => "FACEBOOK",
                AccountType::FacebookGateway => "FACEBOOK_GATEWAY",
                AccountType::GoogleAds => "GOOGLE_ADS",
                AccountType::Microsoft => "MICROSOFT",
                AccountType::SearchAds360 => "SEARCH_ADS_360",
                AccountType::YahooJapan => "YAHOO_JAPAN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "BAIDU" => Some(Self::Baidu),
                "ENGINE_TRACK" => Some(Self::EngineTrack),
                "FACEBOOK" => Some(Self::Facebook),
                "FACEBOOK_GATEWAY" => Some(Self::FacebookGateway),
                "GOOGLE_ADS" => Some(Self::GoogleAds),
                "MICROSOFT" => Some(Self::Microsoft),
                "SEARCH_ADS_360" => Some(Self::SearchAds360),
                "YAHOO_JAPAN" => Some(Self::YahooJapan),
                _ => None,
            }
        }
    }
}
/// Container for enum representing the conversion tracking status of the
/// customer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionTrackingStatusEnum {}
/// Nested message and enum types in `ConversionTrackingStatusEnum`.
pub mod conversion_tracking_status_enum {
    /// Conversion Tracking status of the customer.
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
    pub enum ConversionTrackingStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Customer does not use any conversion tracking.
        NotConversionTracked = 2,
        /// The conversion actions are created and managed by this customer.
        ConversionTrackingManagedBySelf = 3,
        /// The conversion actions are created and managed by the manager specified
        /// in the request's `login-customer-id`.
        ConversionTrackingManagedByThisManager = 4,
        /// The conversion actions are created and managed by a manager different
        /// from the customer or manager specified in the request's
        /// `login-customer-id`.
        ConversionTrackingManagedByAnotherManager = 5,
    }
    impl ConversionTrackingStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionTrackingStatus::Unspecified => "UNSPECIFIED",
                ConversionTrackingStatus::Unknown => "UNKNOWN",
                ConversionTrackingStatus::NotConversionTracked => {
                    "NOT_CONVERSION_TRACKED"
                }
                ConversionTrackingStatus::ConversionTrackingManagedBySelf => {
                    "CONVERSION_TRACKING_MANAGED_BY_SELF"
                }
                ConversionTrackingStatus::ConversionTrackingManagedByThisManager => {
                    "CONVERSION_TRACKING_MANAGED_BY_THIS_MANAGER"
                }
                ConversionTrackingStatus::ConversionTrackingManagedByAnotherManager => {
                    "CONVERSION_TRACKING_MANAGED_BY_ANOTHER_MANAGER"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "NOT_CONVERSION_TRACKED" => Some(Self::NotConversionTracked),
                "CONVERSION_TRACKING_MANAGED_BY_SELF" => {
                    Some(Self::ConversionTrackingManagedBySelf)
                }
                "CONVERSION_TRACKING_MANAGED_BY_THIS_MANAGER" => {
                    Some(Self::ConversionTrackingManagedByThisManager)
                }
                "CONVERSION_TRACKING_MANAGED_BY_ANOTHER_MANAGER" => {
                    Some(Self::ConversionTrackingManagedByAnotherManager)
                }
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible statuses of a customer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerStatusEnum {}
/// Nested message and enum types in `CustomerStatusEnum`.
pub mod customer_status_enum {
    /// Possible statuses of a customer.
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
    pub enum CustomerStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Indicates an active account able to serve ads.
        Enabled = 2,
        /// Indicates a canceled account unable to serve ads.
        /// Can be reactivated by an admin user.
        Canceled = 3,
        /// Indicates a suspended account unable to serve ads.
        /// May only be activated by Google support.
        Suspended = 4,
        /// Indicates a closed account unable to serve ads.
        /// Test account will also have CLOSED status.
        /// Status is permanent and may not be reopened.
        Closed = 5,
    }
    impl CustomerStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomerStatus::Unspecified => "UNSPECIFIED",
                CustomerStatus::Unknown => "UNKNOWN",
                CustomerStatus::Enabled => "ENABLED",
                CustomerStatus::Canceled => "CANCELED",
                CustomerStatus::Suspended => "SUSPENDED",
                CustomerStatus::Closed => "CLOSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "CANCELED" => Some(Self::Canceled),
                "SUSPENDED" => Some(Self::Suspended),
                "CLOSED" => Some(Self::Closed),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible status of a manager and client link.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagerLinkStatusEnum {}
/// Nested message and enum types in `ManagerLinkStatusEnum`.
pub mod manager_link_status_enum {
    /// Possible statuses of a link.
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
    pub enum ManagerLinkStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Indicates current in-effect relationship
        Active = 2,
        /// Indicates terminated relationship
        Inactive = 3,
        /// Indicates relationship has been requested by manager, but the client
        /// hasn't accepted yet.
        Pending = 4,
        /// Relationship was requested by the manager, but the client has refused.
        Refused = 5,
        /// Indicates relationship has been requested by manager, but manager
        /// canceled it.
        Canceled = 6,
    }
    impl ManagerLinkStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ManagerLinkStatus::Unspecified => "UNSPECIFIED",
                ManagerLinkStatus::Unknown => "UNKNOWN",
                ManagerLinkStatus::Active => "ACTIVE",
                ManagerLinkStatus::Inactive => "INACTIVE",
                ManagerLinkStatus::Pending => "PENDING",
                ManagerLinkStatus::Refused => "REFUSED",
                ManagerLinkStatus::Canceled => "CANCELED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ACTIVE" => Some(Self::Active),
                "INACTIVE" => Some(Self::Inactive),
                "PENDING" => Some(Self::Pending),
                "REFUSED" => Some(Self::Refused),
                "CANCELED" => Some(Self::Canceled),
                _ => None,
            }
        }
    }
}
/// Container for describing the status of a geo target constant.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoTargetConstantStatusEnum {}
/// Nested message and enum types in `GeoTargetConstantStatusEnum`.
pub mod geo_target_constant_status_enum {
    /// The possible statuses of a geo target constant.
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
    pub enum GeoTargetConstantStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The geo target constant is valid.
        Enabled = 2,
        /// The geo target constant is obsolete and will be removed.
        RemovalPlanned = 3,
    }
    impl GeoTargetConstantStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GeoTargetConstantStatus::Unspecified => "UNSPECIFIED",
                GeoTargetConstantStatus::Unknown => "UNKNOWN",
                GeoTargetConstantStatus::Enabled => "ENABLED",
                GeoTargetConstantStatus::RemovalPlanned => "REMOVAL_PLANNED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "REMOVAL_PLANNED" => Some(Self::RemovalPlanned),
                _ => None,
            }
        }
    }
}
/// Container for enum describing possible status of a label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelStatusEnum {}
/// Nested message and enum types in `LabelStatusEnum`.
pub mod label_status_enum {
    /// Possible statuses of a label.
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
    pub enum LabelStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Label is enabled.
        Enabled = 2,
        /// Label is removed.
        Removed = 3,
    }
    impl LabelStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LabelStatus::Unspecified => "UNSPECIFIED",
                LabelStatus::Unknown => "UNKNOWN",
                LabelStatus::Enabled => "ENABLED",
                LabelStatus::Removed => "REMOVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ENABLED" => Some(Self::Enabled),
                "REMOVED" => Some(Self::Removed),
                _ => None,
            }
        }
    }
}
/// Level of a product bidding category.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductBiddingCategoryLevelEnum {}
/// Nested message and enum types in `ProductBiddingCategoryLevelEnum`.
pub mod product_bidding_category_level_enum {
    /// Enum describing the level of the product bidding category.
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
    pub enum ProductBiddingCategoryLevel {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Level 1.
        Level1 = 2,
        /// Level 2.
        Level2 = 3,
        /// Level 3.
        Level3 = 4,
        /// Level 4.
        Level4 = 5,
        /// Level 5.
        Level5 = 6,
    }
    impl ProductBiddingCategoryLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProductBiddingCategoryLevel::Unspecified => "UNSPECIFIED",
                ProductBiddingCategoryLevel::Unknown => "UNKNOWN",
                ProductBiddingCategoryLevel::Level1 => "LEVEL1",
                ProductBiddingCategoryLevel::Level2 => "LEVEL2",
                ProductBiddingCategoryLevel::Level3 => "LEVEL3",
                ProductBiddingCategoryLevel::Level4 => "LEVEL4",
                ProductBiddingCategoryLevel::Level5 => "LEVEL5",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "LEVEL1" => Some(Self::Level1),
                "LEVEL2" => Some(Self::Level2),
                "LEVEL3" => Some(Self::Level3),
                "LEVEL4" => Some(Self::Level4),
                "LEVEL5" => Some(Self::Level5),
                _ => None,
            }
        }
    }
}
/// Status of the product bidding category.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductBiddingCategoryStatusEnum {}
/// Nested message and enum types in `ProductBiddingCategoryStatusEnum`.
pub mod product_bidding_category_status_enum {
    /// Enum describing the status of the product bidding category.
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
    pub enum ProductBiddingCategoryStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The category is active and can be used for bidding.
        Active = 2,
        /// The category is obsolete. Used only for reporting purposes.
        Obsolete = 3,
    }
    impl ProductBiddingCategoryStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProductBiddingCategoryStatus::Unspecified => "UNSPECIFIED",
                ProductBiddingCategoryStatus::Unknown => "UNKNOWN",
                ProductBiddingCategoryStatus::Active => "ACTIVE",
                ProductBiddingCategoryStatus::Obsolete => "OBSOLETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "ACTIVE" => Some(Self::Active),
                "OBSOLETE" => Some(Self::Obsolete),
                _ => None,
            }
        }
    }
}
/// The user list types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListTypeEnum {}
/// Nested message and enum types in `UserListTypeEnum`.
pub mod user_list_type_enum {
    /// Enum containing possible user list types.
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
    pub enum UserListType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// UserList represented as a collection of conversion types.
        Remarketing = 2,
        /// UserList represented as a combination of other user lists/interests.
        Logical = 3,
        /// UserList created in the Google Ad Manager platform.
        ExternalRemarketing = 4,
        /// UserList associated with a rule.
        RuleBased = 5,
        /// UserList with users similar to users of another UserList.
        Similar = 6,
        /// UserList of first-party CRM data provided by advertiser in the form of
        /// emails or other formats.
        CrmBased = 7,
    }
    impl UserListType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListType::Unspecified => "UNSPECIFIED",
                UserListType::Unknown => "UNKNOWN",
                UserListType::Remarketing => "REMARKETING",
                UserListType::Logical => "LOGICAL",
                UserListType::ExternalRemarketing => "EXTERNAL_REMARKETING",
                UserListType::RuleBased => "RULE_BASED",
                UserListType::Similar => "SIMILAR",
                UserListType::CrmBased => "CRM_BASED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "REMARKETING" => Some(Self::Remarketing),
                "LOGICAL" => Some(Self::Logical),
                "EXTERNAL_REMARKETING" => Some(Self::ExternalRemarketing),
                "RULE_BASED" => Some(Self::RuleBased),
                "SIMILAR" => Some(Self::Similar),
                "CRM_BASED" => Some(Self::CrmBased),
                _ => None,
            }
        }
    }
}
/// The value type of custom columns.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomColumnValueTypeEnum {}
/// Nested message and enum types in `CustomColumnValueTypeEnum`.
pub mod custom_column_value_type_enum {
    /// Enum containing possible custom column value types.
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
    pub enum CustomColumnValueType {
        /// Not specified.
        Unspecified = 0,
        /// Unknown.
        Unknown = 1,
        /// The custom column value is a string.
        String = 2,
        /// The custom column value is an int64 number.
        Int64 = 3,
        /// The custom column value is a double number.
        Double = 4,
        /// The custom column value is a boolean.
        Boolean = 5,
    }
    impl CustomColumnValueType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomColumnValueType::Unspecified => "UNSPECIFIED",
                CustomColumnValueType::Unknown => "UNKNOWN",
                CustomColumnValueType::String => "STRING",
                CustomColumnValueType::Int64 => "INT64",
                CustomColumnValueType::Double => "DOUBLE",
                CustomColumnValueType::Boolean => "BOOLEAN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "STRING" => Some(Self::String),
                "INT64" => Some(Self::Int64),
                "DOUBLE" => Some(Self::Double),
                "BOOLEAN" => Some(Self::Boolean),
                _ => None,
            }
        }
    }
}
