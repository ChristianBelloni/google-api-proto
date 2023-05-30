/// Message representing a period of time between two timestamps.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampRange {
    /// Begin of the period (inclusive).
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// End of the period (exclusive).
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A resource that represents a location with full geographic information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// The type of a location, which corresponds to the address lines field of
    /// \[google.type.PostalAddress][google.type.PostalAddress\]. For example,
    /// "Downtown, Atlanta, GA, USA" has a type of
    /// \[LocationType.NEIGHBORHOOD][google.cloud.talent.v4.Location.LocationType.NEIGHBORHOOD\],
    /// and "Kansas City, KS, USA" has a type of
    /// \[LocationType.LOCALITY][google.cloud.talent.v4.Location.LocationType.LOCALITY\].
    #[prost(enumeration = "location::LocationType", tag = "1")]
    pub location_type: i32,
    /// Postal address of the location that includes human readable information,
    /// such as postal delivery and payments addresses. Given a postal address,
    /// a postal service can deliver items to a premises, P.O. Box, or other
    /// delivery location.
    #[prost(message, optional, tag = "2")]
    pub postal_address: ::core::option::Option<
        super::super::super::r#type::PostalAddress,
    >,
    /// An object representing a latitude/longitude pair.
    #[prost(message, optional, tag = "3")]
    pub lat_lng: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// Radius in miles of the job location. This value is derived from the
    /// location bounding box in which a circle with the specified radius
    /// centered from \[google.type.LatLng][google.type.LatLng\] covers the area
    /// associated with the job location. For example, currently, "Mountain View,
    /// CA, USA" has a radius of 6.17 miles.
    #[prost(double, tag = "4")]
    pub radius_miles: f64,
}
/// Nested message and enum types in `Location`.
pub mod location {
    /// An enum which represents the type of a location.
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
    pub enum LocationType {
        /// Default value if the type isn't specified.
        Unspecified = 0,
        /// A country level location.
        Country = 1,
        /// A state or equivalent level location.
        AdministrativeArea = 2,
        /// A county or equivalent level location.
        SubAdministrativeArea = 3,
        /// A city or equivalent level location.
        Locality = 4,
        /// A postal code level location.
        PostalCode = 5,
        /// A sublocality is a subdivision of a locality, for example a city borough,
        /// ward, or arrondissement. Sublocalities are usually recognized by a local
        /// political authority. For example, Manhattan and Brooklyn are recognized
        /// as boroughs by the City of New York, and are therefore modeled as
        /// sublocalities.
        SubLocality = 6,
        /// A district or equivalent level location.
        SubLocality1 = 7,
        /// A smaller district or equivalent level display.
        SubLocality2 = 8,
        /// A neighborhood level location.
        Neighborhood = 9,
        /// A street address level location.
        StreetAddress = 10,
    }
    impl LocationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LocationType::Unspecified => "LOCATION_TYPE_UNSPECIFIED",
                LocationType::Country => "COUNTRY",
                LocationType::AdministrativeArea => "ADMINISTRATIVE_AREA",
                LocationType::SubAdministrativeArea => "SUB_ADMINISTRATIVE_AREA",
                LocationType::Locality => "LOCALITY",
                LocationType::PostalCode => "POSTAL_CODE",
                LocationType::SubLocality => "SUB_LOCALITY",
                LocationType::SubLocality1 => "SUB_LOCALITY_1",
                LocationType::SubLocality2 => "SUB_LOCALITY_2",
                LocationType::Neighborhood => "NEIGHBORHOOD",
                LocationType::StreetAddress => "STREET_ADDRESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOCATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "COUNTRY" => Some(Self::Country),
                "ADMINISTRATIVE_AREA" => Some(Self::AdministrativeArea),
                "SUB_ADMINISTRATIVE_AREA" => Some(Self::SubAdministrativeArea),
                "LOCALITY" => Some(Self::Locality),
                "POSTAL_CODE" => Some(Self::PostalCode),
                "SUB_LOCALITY" => Some(Self::SubLocality),
                "SUB_LOCALITY_1" => Some(Self::SubLocality1),
                "SUB_LOCALITY_2" => Some(Self::SubLocality2),
                "NEIGHBORHOOD" => Some(Self::Neighborhood),
                "STREET_ADDRESS" => Some(Self::StreetAddress),
                _ => None,
            }
        }
    }
}
/// Meta information related to the job searcher or entity
/// conducting the job search. This information is used to improve the
/// performance of the service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMetadata {
    /// Required if
    /// \[allow_missing_ids][google.cloud.talent.v4.RequestMetadata.allow_missing_ids\]
    /// is unset or `false`.
    ///
    /// The client-defined scope or source of the service call, which typically
    /// is the domain on
    /// which the service has been implemented and is currently being run.
    ///
    /// For example, if the service is being run by client <em>Foo, Inc.</em>, on
    /// job board www.foo.com and career site www.bar.com, then this field is
    /// set to "foo.com" for use on the job board, and "bar.com" for use on the
    /// career site.
    ///
    /// Note that any improvements to the model for a particular tenant site rely
    /// on this field being set correctly to a unique domain.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    /// Required if
    /// \[allow_missing_ids][google.cloud.talent.v4.RequestMetadata.allow_missing_ids\]
    /// is unset or `false`.
    ///
    /// A unique session identification string. A session is defined as the
    /// duration of an end user's interaction with the service over a certain
    /// period.
    /// Obfuscate this field for privacy concerns before
    /// providing it to the service.
    ///
    /// Note that any improvements to the model for a particular tenant site rely
    /// on this field being set correctly to a unique session ID.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "2")]
    pub session_id: ::prost::alloc::string::String,
    /// Required if
    /// \[allow_missing_ids][google.cloud.talent.v4.RequestMetadata.allow_missing_ids\]
    /// is unset or `false`.
    ///
    /// A unique user identification string, as determined by the client.
    /// To have the strongest positive impact on search quality
    /// make sure the client-level is unique.
    /// Obfuscate this field for privacy concerns before
    /// providing it to the service.
    ///
    /// Note that any improvements to the model for a particular tenant site rely
    /// on this field being set correctly to a unique user ID.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    /// Only set when any of
    /// \[domain][google.cloud.talent.v4.RequestMetadata.domain\],
    /// \[session_id][google.cloud.talent.v4.RequestMetadata.session_id\] and
    /// \[user_id][google.cloud.talent.v4.RequestMetadata.user_id\] isn't available
    /// for some reason. It is highly recommended not to set this field and provide
    /// accurate \[domain][google.cloud.talent.v4.RequestMetadata.domain\],
    /// \[session_id][google.cloud.talent.v4.RequestMetadata.session_id\] and
    /// \[user_id][google.cloud.talent.v4.RequestMetadata.user_id\] for the best
    /// service experience.
    #[prost(bool, tag = "4")]
    pub allow_missing_ids: bool,
    /// The type of device used by the job seeker at the time of the call to the
    /// service.
    #[prost(message, optional, tag = "5")]
    pub device_info: ::core::option::Option<DeviceInfo>,
}
/// Additional information returned to client, such as debugging information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMetadata {
    /// A unique id associated with this call.
    /// This id is logged for tracking purposes.
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
}
/// Device information collected from the job seeker, candidate, or
/// other entity conducting the job search. Providing this information improves
/// the quality of the search results across devices.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfo {
    /// Type of the device.
    #[prost(enumeration = "device_info::DeviceType", tag = "1")]
    pub device_type: i32,
    /// A device-specific ID. The ID must be a unique identifier that
    /// distinguishes the device from other devices.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DeviceInfo`.
pub mod device_info {
    /// An enumeration describing an API access portal and exposure mechanism.
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
    pub enum DeviceType {
        /// The device type isn't specified.
        Unspecified = 0,
        /// A desktop web browser, such as, Chrome, Firefox, Safari, or Internet
        /// Explorer)
        Web = 1,
        /// A mobile device web browser, such as a phone or tablet with a Chrome
        /// browser.
        MobileWeb = 2,
        /// An Android device native application.
        Android = 3,
        /// An iOS device native application.
        Ios = 4,
        /// A bot, as opposed to a device operated by human beings, such as a web
        /// crawler.
        Bot = 5,
        /// Other devices types.
        Other = 6,
    }
    impl DeviceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DeviceType::Unspecified => "DEVICE_TYPE_UNSPECIFIED",
                DeviceType::Web => "WEB",
                DeviceType::MobileWeb => "MOBILE_WEB",
                DeviceType::Android => "ANDROID",
                DeviceType::Ios => "IOS",
                DeviceType::Bot => "BOT",
                DeviceType::Other => "OTHER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEVICE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "WEB" => Some(Self::Web),
                "MOBILE_WEB" => Some(Self::MobileWeb),
                "ANDROID" => Some(Self::Android),
                "IOS" => Some(Self::Ios),
                "BOT" => Some(Self::Bot),
                "OTHER" => Some(Self::Other),
                _ => None,
            }
        }
    }
}
/// Custom attribute values that are either filterable or non-filterable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAttribute {
    /// Exactly one of
    /// \[string_values][google.cloud.talent.v4.CustomAttribute.string_values\] or
    /// \[long_values][google.cloud.talent.v4.CustomAttribute.long_values\] must be
    /// specified.
    ///
    /// This field is used to perform a string match (`CASE_SENSITIVE_MATCH` or
    /// `CASE_INSENSITIVE_MATCH`) search.
    /// For filterable `string_value`s, a maximum total number of 200 values
    /// is allowed, with each `string_value` has a byte size of no more than
    /// 500B. For unfilterable `string_values`, the maximum total byte size of
    /// unfilterable `string_values` is 50KB.
    ///
    /// Empty string isn't allowed.
    #[prost(string, repeated, tag = "1")]
    pub string_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Exactly one of
    /// \[string_values][google.cloud.talent.v4.CustomAttribute.string_values\] or
    /// \[long_values][google.cloud.talent.v4.CustomAttribute.long_values\] must be
    /// specified.
    ///
    /// This field is used to perform number range search.
    /// (`EQ`, `GT`, `GE`, `LE`, `LT`) over filterable `long_value`.
    ///
    /// Currently at most 1
    /// \[long_values][google.cloud.talent.v4.CustomAttribute.long_values\] is
    /// supported.
    #[prost(int64, repeated, tag = "2")]
    pub long_values: ::prost::alloc::vec::Vec<i64>,
    /// If the `filterable` flag is true, the custom field values may be used for
    /// custom attribute filters
    /// \[JobQuery.custom_attribute_filter][google.cloud.talent.v4.JobQuery.custom_attribute_filter\].
    /// If false, these values may not be used for custom attribute filters.
    ///
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub filterable: bool,
    /// If the `keyword_searchable` flag is true, the keywords in custom fields are
    /// searchable by keyword match.
    /// If false, the values are not searchable by keyword match.
    ///
    /// Default is false.
    #[prost(bool, tag = "4")]
    pub keyword_searchable: bool,
}
/// Spell check result.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpellingCorrection {
    /// Indicates if the query was corrected by the spell checker.
    #[prost(bool, tag = "1")]
    pub corrected: bool,
    /// Correction output consisting of the corrected keyword string.
    #[prost(string, tag = "2")]
    pub corrected_text: ::prost::alloc::string::String,
    /// Corrected output with html tags to highlight the corrected words.
    /// Corrected words are called out with the "<b><i>...</i></b>" html tags.
    ///
    /// For example, the user input query is "software enginear", where the second
    /// word, "enginear," is incorrect. It should be "engineer". When spelling
    /// correction is enabled, this value is
    /// "software <b><i>engineer</i></b>".
    #[prost(string, tag = "3")]
    pub corrected_html: ::prost::alloc::string::String,
}
/// Job compensation details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompensationInfo {
    /// Job compensation information.
    ///
    /// At most one entry can be of type
    /// \[CompensationInfo.CompensationType.BASE][google.cloud.talent.v4.CompensationInfo.CompensationType.BASE\],
    /// which is referred as **base compensation entry** for the job.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<compensation_info::CompensationEntry>,
    /// Output only. Annualized base compensation range. Computed as base
    /// compensation entry's
    /// \[CompensationEntry.amount][google.cloud.talent.v4.CompensationInfo.CompensationEntry.amount\]
    /// times
    /// \[CompensationEntry.expected_units_per_year][google.cloud.talent.v4.CompensationInfo.CompensationEntry.expected_units_per_year\].
    ///
    /// See
    /// \[CompensationEntry][google.cloud.talent.v4.CompensationInfo.CompensationEntry\]
    /// for explanation on compensation annualization.
    #[prost(message, optional, tag = "2")]
    pub annualized_base_compensation_range: ::core::option::Option<
        compensation_info::CompensationRange,
    >,
    /// Output only. Annualized total compensation range. Computed as all
    /// compensation entries'
    /// \[CompensationEntry.amount][google.cloud.talent.v4.CompensationInfo.CompensationEntry.amount\]
    /// times
    /// \[CompensationEntry.expected_units_per_year][google.cloud.talent.v4.CompensationInfo.CompensationEntry.expected_units_per_year\].
    ///
    /// See
    /// \[CompensationEntry][google.cloud.talent.v4.CompensationInfo.CompensationEntry\]
    /// for explanation on compensation annualization.
    #[prost(message, optional, tag = "3")]
    pub annualized_total_compensation_range: ::core::option::Option<
        compensation_info::CompensationRange,
    >,
}
/// Nested message and enum types in `CompensationInfo`.
pub mod compensation_info {
    /// A compensation entry that represents one component of compensation, such
    /// as base pay, bonus, or other compensation type.
    ///
    /// Annualization: One compensation entry can be annualized if
    /// - it contains valid
    /// \[amount][google.cloud.talent.v4.CompensationInfo.CompensationEntry.amount\]
    /// or
    /// \[range][google.cloud.talent.v4.CompensationInfo.CompensationEntry.range\].
    /// - and its
    /// \[expected_units_per_year][google.cloud.talent.v4.CompensationInfo.CompensationEntry.expected_units_per_year\]
    /// is set or can be derived. Its annualized range is determined as
    /// (\[amount][google.cloud.talent.v4.CompensationInfo.CompensationEntry.amount\]
    /// or
    /// \[range][google.cloud.talent.v4.CompensationInfo.CompensationEntry.range\])
    /// times
    /// \[expected_units_per_year][google.cloud.talent.v4.CompensationInfo.CompensationEntry.expected_units_per_year\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompensationEntry {
        /// Compensation type.
        ///
        /// Default is
        /// \[CompensationType.COMPENSATION_TYPE_UNSPECIFIED][google.cloud.talent.v4.CompensationInfo.CompensationType.COMPENSATION_TYPE_UNSPECIFIED\].
        #[prost(enumeration = "CompensationType", tag = "1")]
        pub r#type: i32,
        /// Frequency of the specified amount.
        ///
        /// Default is
        /// \[CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED][google.cloud.talent.v4.CompensationInfo.CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED\].
        #[prost(enumeration = "CompensationUnit", tag = "2")]
        pub unit: i32,
        /// Compensation description.  For example, could
        /// indicate equity terms or provide additional context to an estimated
        /// bonus.
        #[prost(string, tag = "5")]
        pub description: ::prost::alloc::string::String,
        /// Expected number of units paid each year. If not specified, when
        /// \[Job.employment_types][google.cloud.talent.v4.Job.employment_types\] is
        /// FULLTIME, a default value is inferred based on
        /// \[unit][google.cloud.talent.v4.CompensationInfo.CompensationEntry.unit\].
        /// Default values:
        /// - HOURLY: 2080
        /// - DAILY: 260
        /// - WEEKLY: 52
        /// - MONTHLY: 12
        /// - ANNUAL: 1
        #[prost(message, optional, tag = "6")]
        pub expected_units_per_year: ::core::option::Option<f64>,
        /// Compensation amount. It could be a fixed amount or a floating range.
        #[prost(oneof = "compensation_entry::CompensationAmount", tags = "3, 4")]
        pub compensation_amount: ::core::option::Option<
            compensation_entry::CompensationAmount,
        >,
    }
    /// Nested message and enum types in `CompensationEntry`.
    pub mod compensation_entry {
        /// Compensation amount. It could be a fixed amount or a floating range.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum CompensationAmount {
            /// Compensation amount.
            #[prost(message, tag = "3")]
            Amount(super::super::super::super::super::r#type::Money),
            /// Compensation range.
            #[prost(message, tag = "4")]
            Range(super::CompensationRange),
        }
    }
    /// Compensation range.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompensationRange {
        /// The maximum amount of compensation. If left empty, the value is set
        /// to a maximal compensation value and the currency code is set to
        /// match the [currency code]\[google.type.Money.currency_code\] of
        /// min_compensation.
        #[prost(message, optional, tag = "2")]
        pub max_compensation: ::core::option::Option<
            super::super::super::super::r#type::Money,
        >,
        /// The minimum amount of compensation. If left empty, the value is set
        /// to zero and the currency code is set to match the
        /// [currency code]\[google.type.Money.currency_code\] of max_compensation.
        #[prost(message, optional, tag = "1")]
        pub min_compensation: ::core::option::Option<
            super::super::super::super::r#type::Money,
        >,
    }
    /// The type of compensation.
    ///
    /// For compensation amounts specified in non-monetary amounts,
    /// describe the compensation scheme in the
    /// \[CompensationEntry.description][google.cloud.talent.v4.CompensationInfo.CompensationEntry.description\].
    ///
    /// For example, tipping format is described in
    /// \[CompensationEntry.description][google.cloud.talent.v4.CompensationInfo.CompensationEntry.description\]
    /// (for example, "expect 15-20% tips based on customer bill.") and an estimate
    /// of the tips provided in
    /// \[CompensationEntry.amount][google.cloud.talent.v4.CompensationInfo.CompensationEntry.amount\]
    /// or
    /// \[CompensationEntry.range][google.cloud.talent.v4.CompensationInfo.CompensationEntry.range\]
    /// ($10 per hour).
    ///
    /// For example, equity is described in
    /// \[CompensationEntry.description][google.cloud.talent.v4.CompensationInfo.CompensationEntry.description\]
    /// (for example, "1% - 2% equity vesting over 4 years, 1 year cliff") and
    /// value estimated in
    /// \[CompensationEntry.amount][google.cloud.talent.v4.CompensationInfo.CompensationEntry.amount\]
    /// or
    /// \[CompensationEntry.range][google.cloud.talent.v4.CompensationInfo.CompensationEntry.range\].
    /// If no value estimate is possible, units are
    /// \[CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED][google.cloud.talent.v4.CompensationInfo.CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED\]
    /// and then further clarified in
    /// \[CompensationEntry.description][google.cloud.talent.v4.CompensationInfo.CompensationEntry.description\]
    /// field.
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
    pub enum CompensationType {
        /// Default value.
        Unspecified = 0,
        /// Base compensation: Refers to the fixed amount of money paid to an
        /// employee by an employer in return for work performed. Base compensation
        /// does not include benefits, bonuses or any other potential compensation
        /// from an employer.
        Base = 1,
        /// Bonus.
        Bonus = 2,
        /// Signing bonus.
        SigningBonus = 3,
        /// Equity.
        Equity = 4,
        /// Profit sharing.
        ProfitSharing = 5,
        /// Commission.
        Commissions = 6,
        /// Tips.
        Tips = 7,
        /// Other compensation type.
        OtherCompensationType = 8,
    }
    impl CompensationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CompensationType::Unspecified => "COMPENSATION_TYPE_UNSPECIFIED",
                CompensationType::Base => "BASE",
                CompensationType::Bonus => "BONUS",
                CompensationType::SigningBonus => "SIGNING_BONUS",
                CompensationType::Equity => "EQUITY",
                CompensationType::ProfitSharing => "PROFIT_SHARING",
                CompensationType::Commissions => "COMMISSIONS",
                CompensationType::Tips => "TIPS",
                CompensationType::OtherCompensationType => "OTHER_COMPENSATION_TYPE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPENSATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BASE" => Some(Self::Base),
                "BONUS" => Some(Self::Bonus),
                "SIGNING_BONUS" => Some(Self::SigningBonus),
                "EQUITY" => Some(Self::Equity),
                "PROFIT_SHARING" => Some(Self::ProfitSharing),
                "COMMISSIONS" => Some(Self::Commissions),
                "TIPS" => Some(Self::Tips),
                "OTHER_COMPENSATION_TYPE" => Some(Self::OtherCompensationType),
                _ => None,
            }
        }
    }
    /// Pay frequency.
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
    pub enum CompensationUnit {
        /// Default value.
        Unspecified = 0,
        /// Hourly.
        Hourly = 1,
        /// Daily.
        Daily = 2,
        /// Weekly
        Weekly = 3,
        /// Monthly.
        Monthly = 4,
        /// Yearly.
        Yearly = 5,
        /// One time.
        OneTime = 6,
        /// Other compensation units.
        OtherCompensationUnit = 7,
    }
    impl CompensationUnit {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CompensationUnit::Unspecified => "COMPENSATION_UNIT_UNSPECIFIED",
                CompensationUnit::Hourly => "HOURLY",
                CompensationUnit::Daily => "DAILY",
                CompensationUnit::Weekly => "WEEKLY",
                CompensationUnit::Monthly => "MONTHLY",
                CompensationUnit::Yearly => "YEARLY",
                CompensationUnit::OneTime => "ONE_TIME",
                CompensationUnit::OtherCompensationUnit => "OTHER_COMPENSATION_UNIT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPENSATION_UNIT_UNSPECIFIED" => Some(Self::Unspecified),
                "HOURLY" => Some(Self::Hourly),
                "DAILY" => Some(Self::Daily),
                "WEEKLY" => Some(Self::Weekly),
                "MONTHLY" => Some(Self::Monthly),
                "YEARLY" => Some(Self::Yearly),
                "ONE_TIME" => Some(Self::OneTime),
                "OTHER_COMPENSATION_UNIT" => Some(Self::OtherCompensationUnit),
                _ => None,
            }
        }
    }
}
/// Metadata used for long running operations returned by CTS batch APIs.
/// It's used to replace
/// \[google.longrunning.Operation.metadata][google.longrunning.Operation.metadata\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOperationMetadata {
    /// The state of a long running operation.
    #[prost(enumeration = "batch_operation_metadata::State", tag = "1")]
    pub state: i32,
    /// More detailed information about operation state.
    #[prost(string, tag = "2")]
    pub state_description: ::prost::alloc::string::String,
    /// Count of successful item(s) inside an operation.
    #[prost(int32, tag = "3")]
    pub success_count: i32,
    /// Count of failed item(s) inside an operation.
    #[prost(int32, tag = "4")]
    pub failure_count: i32,
    /// Count of total item(s) inside an operation.
    #[prost(int32, tag = "5")]
    pub total_count: i32,
    /// The time when the batch operation is created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the batch operation status is updated. The metadata and the
    /// \[update_time][google.cloud.talent.v4.BatchOperationMetadata.update_time\] is
    /// refreshed every minute otherwise cached data is returned.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the batch operation is finished and
    /// \[google.longrunning.Operation.done][google.longrunning.Operation.done\] is
    /// set to `true`.
    #[prost(message, optional, tag = "8")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `BatchOperationMetadata`.
pub mod batch_operation_metadata {
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
        /// Default value.
        Unspecified = 0,
        /// The batch operation is being prepared for processing.
        Initializing = 1,
        /// The batch operation is actively being processed.
        Processing = 2,
        /// The batch operation is processed, and at least one item has been
        /// successfully processed.
        Succeeded = 3,
        /// The batch operation is done and no item has been successfully processed.
        Failed = 4,
        /// The batch operation is in the process of cancelling after
        /// \[google.longrunning.Operations.CancelOperation][google.longrunning.Operations.CancelOperation\]
        /// is called.
        Cancelling = 5,
        /// The batch operation is done after
        /// \[google.longrunning.Operations.CancelOperation][google.longrunning.Operations.CancelOperation\]
        /// is called. Any items processed before cancelling are returned in the
        /// response.
        Cancelled = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Initializing => "INITIALIZING",
                State::Processing => "PROCESSING",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::Cancelling => "CANCELLING",
                State::Cancelled => "CANCELLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "INITIALIZING" => Some(Self::Initializing),
                "PROCESSING" => Some(Self::Processing),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "CANCELLING" => Some(Self::Cancelling),
                "CANCELLED" => Some(Self::Cancelled),
                _ => None,
            }
        }
    }
}
/// An enum that represents the size of the company.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompanySize {
    /// Default value if the size isn't specified.
    Unspecified = 0,
    /// The company has less than 50 employees.
    Mini = 1,
    /// The company has between 50 and 99 employees.
    Small = 2,
    /// The company has between 100 and 499 employees.
    Smedium = 3,
    /// The company has between 500 and 999 employees.
    Medium = 4,
    /// The company has between 1,000 and 4,999 employees.
    Big = 5,
    /// The company has between 5,000 and 9,999 employees.
    Bigger = 6,
    /// The company has 10,000 or more employees.
    Giant = 7,
}
impl CompanySize {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CompanySize::Unspecified => "COMPANY_SIZE_UNSPECIFIED",
            CompanySize::Mini => "MINI",
            CompanySize::Small => "SMALL",
            CompanySize::Smedium => "SMEDIUM",
            CompanySize::Medium => "MEDIUM",
            CompanySize::Big => "BIG",
            CompanySize::Bigger => "BIGGER",
            CompanySize::Giant => "GIANT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMPANY_SIZE_UNSPECIFIED" => Some(Self::Unspecified),
            "MINI" => Some(Self::Mini),
            "SMALL" => Some(Self::Small),
            "SMEDIUM" => Some(Self::Smedium),
            "MEDIUM" => Some(Self::Medium),
            "BIG" => Some(Self::Big),
            "BIGGER" => Some(Self::Bigger),
            "GIANT" => Some(Self::Giant),
            _ => None,
        }
    }
}
/// An enum that represents employee benefits included with the job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobBenefit {
    /// Default value if the type isn't specified.
    Unspecified = 0,
    /// The job includes access to programs that support child care, such
    /// as daycare.
    ChildCare = 1,
    /// The job includes dental services covered by a dental
    /// insurance plan.
    Dental = 2,
    /// The job offers specific benefits to domestic partners.
    DomesticPartner = 3,
    /// The job allows for a flexible work schedule.
    FlexibleHours = 4,
    /// The job includes health services covered by a medical insurance plan.
    Medical = 5,
    /// The job includes a life insurance plan provided by the employer or
    /// available for purchase by the employee.
    LifeInsurance = 6,
    /// The job allows for a leave of absence to a parent to care for a newborn
    /// child.
    ParentalLeave = 7,
    /// The job includes a workplace retirement plan provided by the
    /// employer or available for purchase by the employee.
    RetirementPlan = 8,
    /// The job allows for paid time off due to illness.
    SickDays = 9,
    /// The job includes paid time off for vacation.
    Vacation = 10,
    /// The job includes vision services covered by a vision
    /// insurance plan.
    Vision = 11,
}
impl JobBenefit {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JobBenefit::Unspecified => "JOB_BENEFIT_UNSPECIFIED",
            JobBenefit::ChildCare => "CHILD_CARE",
            JobBenefit::Dental => "DENTAL",
            JobBenefit::DomesticPartner => "DOMESTIC_PARTNER",
            JobBenefit::FlexibleHours => "FLEXIBLE_HOURS",
            JobBenefit::Medical => "MEDICAL",
            JobBenefit::LifeInsurance => "LIFE_INSURANCE",
            JobBenefit::ParentalLeave => "PARENTAL_LEAVE",
            JobBenefit::RetirementPlan => "RETIREMENT_PLAN",
            JobBenefit::SickDays => "SICK_DAYS",
            JobBenefit::Vacation => "VACATION",
            JobBenefit::Vision => "VISION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "JOB_BENEFIT_UNSPECIFIED" => Some(Self::Unspecified),
            "CHILD_CARE" => Some(Self::ChildCare),
            "DENTAL" => Some(Self::Dental),
            "DOMESTIC_PARTNER" => Some(Self::DomesticPartner),
            "FLEXIBLE_HOURS" => Some(Self::FlexibleHours),
            "MEDICAL" => Some(Self::Medical),
            "LIFE_INSURANCE" => Some(Self::LifeInsurance),
            "PARENTAL_LEAVE" => Some(Self::ParentalLeave),
            "RETIREMENT_PLAN" => Some(Self::RetirementPlan),
            "SICK_DAYS" => Some(Self::SickDays),
            "VACATION" => Some(Self::Vacation),
            "VISION" => Some(Self::Vision),
            _ => None,
        }
    }
}
/// Educational degree level defined in International Standard Classification
/// of Education (ISCED).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DegreeType {
    /// Default value. Represents no degree, or early childhood education.
    /// Maps to ISCED code 0.
    /// Ex) Kindergarten
    Unspecified = 0,
    /// Primary education which is typically the first stage of compulsory
    /// education. ISCED code 1.
    /// Ex) Elementary school
    PrimaryEducation = 1,
    /// Lower secondary education; First stage of secondary education building on
    /// primary education, typically with a more subject-oriented curriculum.
    /// ISCED code 2.
    /// Ex) Middle school
    LowerSecondaryEducation = 2,
    /// Middle education; Second/final stage of secondary education preparing for
    /// tertiary education and/or providing skills relevant to employment.
    /// Usually with an increased range of subject options and streams. ISCED
    /// code 3.
    /// Ex) High school
    UpperSecondaryEducation = 3,
    /// Adult Remedial Education; Programmes providing learning experiences that
    /// build on secondary education and prepare for labour market entry and/or
    /// tertiary education. The content is broader than secondary but not as
    /// complex as tertiary education. ISCED code 4.
    AdultRemedialEducation = 4,
    /// Associate's or equivalent; Short first tertiary programmes that are
    /// typically practically-based, occupationally-specific and prepare for
    /// labour market entry. These programmes may also provide a pathway to other
    /// tertiary programmes. ISCED code 5.
    AssociatesOrEquivalent = 5,
    /// Bachelor's or equivalent; Programmes designed to provide intermediate
    /// academic and/or professional knowledge, skills and competencies leading
    /// to a first tertiary degree or equivalent qualification. ISCED code 6.
    BachelorsOrEquivalent = 6,
    /// Master's or equivalent; Programmes designed to provide advanced academic
    /// and/or professional knowledge, skills and competencies leading to a
    /// second tertiary degree or equivalent qualification. ISCED code 7.
    MastersOrEquivalent = 7,
    /// Doctoral or equivalent; Programmes designed primarily to lead to an
    /// advanced research qualification, usually concluding with the submission
    /// and defense of a substantive dissertation of publishable quality based on
    /// original research. ISCED code 8.
    DoctoralOrEquivalent = 8,
}
impl DegreeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DegreeType::Unspecified => "DEGREE_TYPE_UNSPECIFIED",
            DegreeType::PrimaryEducation => "PRIMARY_EDUCATION",
            DegreeType::LowerSecondaryEducation => "LOWER_SECONDARY_EDUCATION",
            DegreeType::UpperSecondaryEducation => "UPPER_SECONDARY_EDUCATION",
            DegreeType::AdultRemedialEducation => "ADULT_REMEDIAL_EDUCATION",
            DegreeType::AssociatesOrEquivalent => "ASSOCIATES_OR_EQUIVALENT",
            DegreeType::BachelorsOrEquivalent => "BACHELORS_OR_EQUIVALENT",
            DegreeType::MastersOrEquivalent => "MASTERS_OR_EQUIVALENT",
            DegreeType::DoctoralOrEquivalent => "DOCTORAL_OR_EQUIVALENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEGREE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PRIMARY_EDUCATION" => Some(Self::PrimaryEducation),
            "LOWER_SECONDARY_EDUCATION" => Some(Self::LowerSecondaryEducation),
            "UPPER_SECONDARY_EDUCATION" => Some(Self::UpperSecondaryEducation),
            "ADULT_REMEDIAL_EDUCATION" => Some(Self::AdultRemedialEducation),
            "ASSOCIATES_OR_EQUIVALENT" => Some(Self::AssociatesOrEquivalent),
            "BACHELORS_OR_EQUIVALENT" => Some(Self::BachelorsOrEquivalent),
            "MASTERS_OR_EQUIVALENT" => Some(Self::MastersOrEquivalent),
            "DOCTORAL_OR_EQUIVALENT" => Some(Self::DoctoralOrEquivalent),
            _ => None,
        }
    }
}
/// An enum that represents the employment type of a job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EmploymentType {
    /// The default value if the employment type isn't specified.
    Unspecified = 0,
    /// The job requires working a number of hours that constitute full
    /// time employment, typically 40 or more hours per week.
    FullTime = 1,
    /// The job entails working fewer hours than a full time job,
    /// typically less than 40 hours a week.
    PartTime = 2,
    /// The job is offered as a contracted, as opposed to a salaried employee,
    /// position.
    Contractor = 3,
    /// The job is offered as a contracted position with the understanding
    /// that it's converted into a full-time position at the end of the
    /// contract. Jobs of this type are also returned by a search for
    /// \[EmploymentType.CONTRACTOR][google.cloud.talent.v4.EmploymentType.CONTRACTOR\]
    /// jobs.
    ContractToHire = 4,
    /// The job is offered as a temporary employment opportunity, usually
    /// a short-term engagement.
    Temporary = 5,
    /// The job is a fixed-term opportunity for students or entry-level job
    /// seekers to obtain on-the-job training, typically offered as a summer
    /// position.
    Intern = 6,
    /// The is an opportunity for an individual to volunteer, where there's no
    /// expectation of compensation for the provided services.
    Volunteer = 7,
    /// The job requires an employee to work on an as-needed basis with a
    /// flexible schedule.
    PerDiem = 8,
    /// The job involves employing people in remote areas and flying them
    /// temporarily to the work site instead of relocating employees and their
    /// families permanently.
    FlyInFlyOut = 9,
    /// The job does not fit any of the other listed types.
    OtherEmploymentType = 10,
}
impl EmploymentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EmploymentType::Unspecified => "EMPLOYMENT_TYPE_UNSPECIFIED",
            EmploymentType::FullTime => "FULL_TIME",
            EmploymentType::PartTime => "PART_TIME",
            EmploymentType::Contractor => "CONTRACTOR",
            EmploymentType::ContractToHire => "CONTRACT_TO_HIRE",
            EmploymentType::Temporary => "TEMPORARY",
            EmploymentType::Intern => "INTERN",
            EmploymentType::Volunteer => "VOLUNTEER",
            EmploymentType::PerDiem => "PER_DIEM",
            EmploymentType::FlyInFlyOut => "FLY_IN_FLY_OUT",
            EmploymentType::OtherEmploymentType => "OTHER_EMPLOYMENT_TYPE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EMPLOYMENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "FULL_TIME" => Some(Self::FullTime),
            "PART_TIME" => Some(Self::PartTime),
            "CONTRACTOR" => Some(Self::Contractor),
            "CONTRACT_TO_HIRE" => Some(Self::ContractToHire),
            "TEMPORARY" => Some(Self::Temporary),
            "INTERN" => Some(Self::Intern),
            "VOLUNTEER" => Some(Self::Volunteer),
            "PER_DIEM" => Some(Self::PerDiem),
            "FLY_IN_FLY_OUT" => Some(Self::FlyInFlyOut),
            "OTHER_EMPLOYMENT_TYPE" => Some(Self::OtherEmploymentType),
            _ => None,
        }
    }
}
/// An enum that represents the required experience level required for the job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobLevel {
    /// The default value if the level isn't specified.
    Unspecified = 0,
    /// Entry-level individual contributors, typically with less than 2 years of
    /// experience in a similar role. Includes interns.
    EntryLevel = 1,
    /// Experienced individual contributors, typically with 2+ years of
    /// experience in a similar role.
    Experienced = 2,
    /// Entry- to mid-level managers responsible for managing a team of people.
    Manager = 3,
    /// Senior-level managers responsible for managing teams of managers.
    Director = 4,
    /// Executive-level managers and above, including C-level positions.
    Executive = 5,
}
impl JobLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JobLevel::Unspecified => "JOB_LEVEL_UNSPECIFIED",
            JobLevel::EntryLevel => "ENTRY_LEVEL",
            JobLevel::Experienced => "EXPERIENCED",
            JobLevel::Manager => "MANAGER",
            JobLevel::Director => "DIRECTOR",
            JobLevel::Executive => "EXECUTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "JOB_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "ENTRY_LEVEL" => Some(Self::EntryLevel),
            "EXPERIENCED" => Some(Self::Experienced),
            "MANAGER" => Some(Self::Manager),
            "DIRECTOR" => Some(Self::Director),
            "EXECUTIVE" => Some(Self::Executive),
            _ => None,
        }
    }
}
/// An enum that represents the categorization or primary focus of specific
/// role. This value is different than the "industry" associated with a role,
/// which is related to the categorization of the company listing the job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobCategory {
    /// The default value if the category isn't specified.
    Unspecified = 0,
    /// An accounting and finance job, such as an Accountant.
    AccountingAndFinance = 1,
    /// An administrative and office job, such as an Administrative Assistant.
    AdministrativeAndOffice = 2,
    /// An advertising and marketing job, such as Marketing Manager.
    AdvertisingAndMarketing = 3,
    /// An animal care job, such as Veterinarian.
    AnimalCare = 4,
    /// An art, fashion, or design job, such as Designer.
    ArtFashionAndDesign = 5,
    /// A business operations job, such as Business Operations Manager.
    BusinessOperations = 6,
    /// A cleaning and facilities job, such as Custodial Staff.
    CleaningAndFacilities = 7,
    /// A computer and IT job, such as Systems Administrator.
    ComputerAndIt = 8,
    /// A construction job, such as General Laborer.
    Construction = 9,
    /// A customer service job, such s Cashier.
    CustomerService = 10,
    /// An education job, such as School Teacher.
    Education = 11,
    /// An entertainment and travel job, such as Flight Attendant.
    EntertainmentAndTravel = 12,
    /// A farming or outdoor job, such as Park Ranger.
    FarmingAndOutdoors = 13,
    /// A healthcare job, such as Registered Nurse.
    Healthcare = 14,
    /// A human resources job, such as Human Resources Director.
    HumanResources = 15,
    /// An installation, maintenance, or repair job, such as Electrician.
    InstallationMaintenanceAndRepair = 16,
    /// A legal job, such as Law Clerk.
    Legal = 17,
    /// A management job, often used in conjunction with another category,
    /// such as Store Manager.
    Management = 18,
    /// A manufacturing or warehouse job, such as Assembly Technician.
    ManufacturingAndWarehouse = 19,
    /// A media, communications, or writing job, such as Media Relations.
    MediaCommunicationsAndWriting = 20,
    /// An oil, gas or mining job, such as Offshore Driller.
    OilGasAndMining = 21,
    /// A personal care and services job, such as Hair Stylist.
    PersonalCareAndServices = 22,
    /// A protective services job, such as Security Guard.
    ProtectiveServices = 23,
    /// A real estate job, such as Buyer's Agent.
    RealEstate = 24,
    /// A restaurant and hospitality job, such as Restaurant Server.
    RestaurantAndHospitality = 25,
    /// A sales and/or retail job, such Sales Associate.
    SalesAndRetail = 26,
    /// A science and engineering job, such as Lab Technician.
    ScienceAndEngineering = 27,
    /// A social services or non-profit job, such as Case Worker.
    SocialServicesAndNonProfit = 28,
    /// A sports, fitness, or recreation job, such as Personal Trainer.
    SportsFitnessAndRecreation = 29,
    /// A transportation or logistics job, such as Truck Driver.
    TransportationAndLogistics = 30,
}
impl JobCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JobCategory::Unspecified => "JOB_CATEGORY_UNSPECIFIED",
            JobCategory::AccountingAndFinance => "ACCOUNTING_AND_FINANCE",
            JobCategory::AdministrativeAndOffice => "ADMINISTRATIVE_AND_OFFICE",
            JobCategory::AdvertisingAndMarketing => "ADVERTISING_AND_MARKETING",
            JobCategory::AnimalCare => "ANIMAL_CARE",
            JobCategory::ArtFashionAndDesign => "ART_FASHION_AND_DESIGN",
            JobCategory::BusinessOperations => "BUSINESS_OPERATIONS",
            JobCategory::CleaningAndFacilities => "CLEANING_AND_FACILITIES",
            JobCategory::ComputerAndIt => "COMPUTER_AND_IT",
            JobCategory::Construction => "CONSTRUCTION",
            JobCategory::CustomerService => "CUSTOMER_SERVICE",
            JobCategory::Education => "EDUCATION",
            JobCategory::EntertainmentAndTravel => "ENTERTAINMENT_AND_TRAVEL",
            JobCategory::FarmingAndOutdoors => "FARMING_AND_OUTDOORS",
            JobCategory::Healthcare => "HEALTHCARE",
            JobCategory::HumanResources => "HUMAN_RESOURCES",
            JobCategory::InstallationMaintenanceAndRepair => {
                "INSTALLATION_MAINTENANCE_AND_REPAIR"
            }
            JobCategory::Legal => "LEGAL",
            JobCategory::Management => "MANAGEMENT",
            JobCategory::ManufacturingAndWarehouse => "MANUFACTURING_AND_WAREHOUSE",
            JobCategory::MediaCommunicationsAndWriting => {
                "MEDIA_COMMUNICATIONS_AND_WRITING"
            }
            JobCategory::OilGasAndMining => "OIL_GAS_AND_MINING",
            JobCategory::PersonalCareAndServices => "PERSONAL_CARE_AND_SERVICES",
            JobCategory::ProtectiveServices => "PROTECTIVE_SERVICES",
            JobCategory::RealEstate => "REAL_ESTATE",
            JobCategory::RestaurantAndHospitality => "RESTAURANT_AND_HOSPITALITY",
            JobCategory::SalesAndRetail => "SALES_AND_RETAIL",
            JobCategory::ScienceAndEngineering => "SCIENCE_AND_ENGINEERING",
            JobCategory::SocialServicesAndNonProfit => "SOCIAL_SERVICES_AND_NON_PROFIT",
            JobCategory::SportsFitnessAndRecreation => "SPORTS_FITNESS_AND_RECREATION",
            JobCategory::TransportationAndLogistics => "TRANSPORTATION_AND_LOGISTICS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "JOB_CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
            "ACCOUNTING_AND_FINANCE" => Some(Self::AccountingAndFinance),
            "ADMINISTRATIVE_AND_OFFICE" => Some(Self::AdministrativeAndOffice),
            "ADVERTISING_AND_MARKETING" => Some(Self::AdvertisingAndMarketing),
            "ANIMAL_CARE" => Some(Self::AnimalCare),
            "ART_FASHION_AND_DESIGN" => Some(Self::ArtFashionAndDesign),
            "BUSINESS_OPERATIONS" => Some(Self::BusinessOperations),
            "CLEANING_AND_FACILITIES" => Some(Self::CleaningAndFacilities),
            "COMPUTER_AND_IT" => Some(Self::ComputerAndIt),
            "CONSTRUCTION" => Some(Self::Construction),
            "CUSTOMER_SERVICE" => Some(Self::CustomerService),
            "EDUCATION" => Some(Self::Education),
            "ENTERTAINMENT_AND_TRAVEL" => Some(Self::EntertainmentAndTravel),
            "FARMING_AND_OUTDOORS" => Some(Self::FarmingAndOutdoors),
            "HEALTHCARE" => Some(Self::Healthcare),
            "HUMAN_RESOURCES" => Some(Self::HumanResources),
            "INSTALLATION_MAINTENANCE_AND_REPAIR" => {
                Some(Self::InstallationMaintenanceAndRepair)
            }
            "LEGAL" => Some(Self::Legal),
            "MANAGEMENT" => Some(Self::Management),
            "MANUFACTURING_AND_WAREHOUSE" => Some(Self::ManufacturingAndWarehouse),
            "MEDIA_COMMUNICATIONS_AND_WRITING" => {
                Some(Self::MediaCommunicationsAndWriting)
            }
            "OIL_GAS_AND_MINING" => Some(Self::OilGasAndMining),
            "PERSONAL_CARE_AND_SERVICES" => Some(Self::PersonalCareAndServices),
            "PROTECTIVE_SERVICES" => Some(Self::ProtectiveServices),
            "REAL_ESTATE" => Some(Self::RealEstate),
            "RESTAURANT_AND_HOSPITALITY" => Some(Self::RestaurantAndHospitality),
            "SALES_AND_RETAIL" => Some(Self::SalesAndRetail),
            "SCIENCE_AND_ENGINEERING" => Some(Self::ScienceAndEngineering),
            "SOCIAL_SERVICES_AND_NON_PROFIT" => Some(Self::SocialServicesAndNonProfit),
            "SPORTS_FITNESS_AND_RECREATION" => Some(Self::SportsFitnessAndRecreation),
            "TRANSPORTATION_AND_LOGISTICS" => Some(Self::TransportationAndLogistics),
            _ => None,
        }
    }
}
/// An enum that represents the job posting region. In most cases, job postings
/// don't need to specify a region. If a region is given, jobs are
/// eligible for searches in the specified region.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PostingRegion {
    /// If the region is unspecified, the job is only returned if it
    /// matches the \[LocationFilter][google.cloud.talent.v4.LocationFilter\].
    Unspecified = 0,
    /// In addition to exact location matching, job posting is returned when the
    /// \[LocationFilter][google.cloud.talent.v4.LocationFilter\] in the search query
    /// is in the same administrative area as the returned job posting. For
    /// example, if a `ADMINISTRATIVE_AREA` job is posted in "CA, USA", it's
    /// returned if \[LocationFilter][google.cloud.talent.v4.LocationFilter\] has
    /// "Mountain View".
    ///
    /// Administrative area refers to top-level administrative subdivision of this
    /// country. For example, US state, IT region, UK constituent nation and
    /// JP prefecture.
    AdministrativeArea = 1,
    /// In addition to exact location matching, job is returned when
    /// \[LocationFilter][google.cloud.talent.v4.LocationFilter\] in search query is
    /// in the same country as this job. For example, if a `NATION_WIDE` job is
    /// posted in "USA", it's returned if
    /// \[LocationFilter][google.cloud.talent.v4.LocationFilter\] has 'Mountain
    /// View'.
    Nation = 2,
    /// Job allows employees to work remotely (telecommute).
    /// If locations are provided with this value, the job is
    /// considered as having a location, but telecommuting is allowed.
    Telecommute = 3,
}
impl PostingRegion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PostingRegion::Unspecified => "POSTING_REGION_UNSPECIFIED",
            PostingRegion::AdministrativeArea => "ADMINISTRATIVE_AREA",
            PostingRegion::Nation => "NATION",
            PostingRegion::Telecommute => "TELECOMMUTE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "POSTING_REGION_UNSPECIFIED" => Some(Self::Unspecified),
            "ADMINISTRATIVE_AREA" => Some(Self::AdministrativeArea),
            "NATION" => Some(Self::Nation),
            "TELECOMMUTE" => Some(Self::Telecommute),
            _ => None,
        }
    }
}
/// Deprecated. All resources are only visible to the owner.
///
/// An enum that represents who has view access to the resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Visibility {
    /// Default value.
    Unspecified = 0,
    /// The resource is only visible to the GCP account who owns it.
    AccountOnly = 1,
    /// The resource is visible to the owner and may be visible to other
    /// applications and processes at Google.
    SharedWithGoogle = 2,
    /// The resource is visible to the owner and may be visible to all other API
    /// clients.
    SharedWithPublic = 3,
}
impl Visibility {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Visibility::Unspecified => "VISIBILITY_UNSPECIFIED",
            Visibility::AccountOnly => "ACCOUNT_ONLY",
            Visibility::SharedWithGoogle => "SHARED_WITH_GOOGLE",
            Visibility::SharedWithPublic => "SHARED_WITH_PUBLIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VISIBILITY_UNSPECIFIED" => Some(Self::Unspecified),
            "ACCOUNT_ONLY" => Some(Self::AccountOnly),
            "SHARED_WITH_GOOGLE" => Some(Self::SharedWithGoogle),
            "SHARED_WITH_PUBLIC" => Some(Self::SharedWithPublic),
            _ => None,
        }
    }
}
/// Option for HTML content sanitization on user input fields, for example, job
/// description. By setting this option, user can determine whether and how
/// sanitization is performed on these fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HtmlSanitization {
    /// Default value.
    Unspecified = 0,
    /// Disables sanitization on HTML input.
    Disabled = 1,
    /// Sanitizes HTML input, only accepts bold, italic, ordered list, and
    /// unordered list markup tags.
    SimpleFormattingOnly = 2,
}
impl HtmlSanitization {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HtmlSanitization::Unspecified => "HTML_SANITIZATION_UNSPECIFIED",
            HtmlSanitization::Disabled => "HTML_SANITIZATION_DISABLED",
            HtmlSanitization::SimpleFormattingOnly => "SIMPLE_FORMATTING_ONLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HTML_SANITIZATION_UNSPECIFIED" => Some(Self::Unspecified),
            "HTML_SANITIZATION_DISABLED" => Some(Self::Disabled),
            "SIMPLE_FORMATTING_ONLY" => Some(Self::SimpleFormattingOnly),
            _ => None,
        }
    }
}
/// Method for commute. Walking, biking and wheelchair accessible transit is
/// still in the Preview stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommuteMethod {
    /// Commute method isn't specified.
    Unspecified = 0,
    /// Commute time is calculated based on driving time.
    Driving = 1,
    /// Commute time is calculated based on public transit including bus, metro,
    /// subway, and so on.
    Transit = 2,
    /// Commute time is calculated based on walking time.
    Walking = 3,
    /// Commute time is calculated based on biking time.
    Cycling = 4,
    /// Commute time is calculated based on public transit that is wheelchair
    /// accessible.
    TransitAccessible = 5,
}
impl CommuteMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CommuteMethod::Unspecified => "COMMUTE_METHOD_UNSPECIFIED",
            CommuteMethod::Driving => "DRIVING",
            CommuteMethod::Transit => "TRANSIT",
            CommuteMethod::Walking => "WALKING",
            CommuteMethod::Cycling => "CYCLING",
            CommuteMethod::TransitAccessible => "TRANSIT_ACCESSIBLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMMUTE_METHOD_UNSPECIFIED" => Some(Self::Unspecified),
            "DRIVING" => Some(Self::Driving),
            "TRANSIT" => Some(Self::Transit),
            "WALKING" => Some(Self::Walking),
            "CYCLING" => Some(Self::Cycling),
            "TRANSIT_ACCESSIBLE" => Some(Self::TransitAccessible),
            _ => None,
        }
    }
}
/// A Company resource represents a company in the service. A company is the
/// entity that owns job postings, that is, the hiring entity responsible for
/// employing applicants for the job position.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Company {
    /// Required during company update.
    ///
    /// The resource name for a company. This is generated by the service when a
    /// company is created.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}", for
    /// example, "projects/foo/tenants/bar/companies/baz".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name of the company, for example, "Google LLC".
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Client side company identifier, used to uniquely identify the
    /// company.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "3")]
    pub external_id: ::prost::alloc::string::String,
    /// The employer's company size.
    #[prost(enumeration = "CompanySize", tag = "4")]
    pub size: i32,
    /// The street address of the company's main headquarters, which may be
    /// different from the job location. The service attempts
    /// to geolocate the provided address, and populates a more specific
    /// location wherever possible in
    /// \[DerivedInfo.headquarters_location][google.cloud.talent.v4.Company.DerivedInfo.headquarters_location\].
    #[prost(string, tag = "5")]
    pub headquarters_address: ::prost::alloc::string::String,
    /// Set to true if it is the hiring agency that post jobs for other
    /// employers.
    ///
    /// Defaults to false if not provided.
    #[prost(bool, tag = "6")]
    pub hiring_agency: bool,
    /// Equal Employment Opportunity legal disclaimer text to be
    /// associated with all jobs, and typically to be displayed in all
    /// roles.
    ///
    /// The maximum number of allowed characters is 500.
    #[prost(string, tag = "7")]
    pub eeo_text: ::prost::alloc::string::String,
    /// The URI representing the company's primary web site or home page,
    /// for example, "<https://www.google.com".>
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "8")]
    pub website_uri: ::prost::alloc::string::String,
    /// The URI to employer's career site or careers page on the employer's web
    /// site, for example, "<https://careers.google.com".>
    #[prost(string, tag = "9")]
    pub career_site_uri: ::prost::alloc::string::String,
    /// A URI that hosts the employer's company logo.
    #[prost(string, tag = "10")]
    pub image_uri: ::prost::alloc::string::String,
    /// This field is deprecated. Please set the searchability of the custom
    /// attribute in the
    /// \[Job.custom_attributes][google.cloud.talent.v4.Job.custom_attributes\] going
    /// forward.
    ///
    /// A list of keys of filterable
    /// \[Job.custom_attributes][google.cloud.talent.v4.Job.custom_attributes\],
    /// whose corresponding `string_values` are used in keyword searches. Jobs with
    /// `string_values` under these specified field keys are returned if any
    /// of the values match the search keyword. Custom field values with
    /// parenthesis, brackets and special symbols are not searchable as-is,
    /// and those keyword queries must be surrounded by quotes.
    #[deprecated]
    #[prost(string, repeated, tag = "11")]
    pub keyword_searchable_job_custom_attributes: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Output only. Derived details about the company.
    #[prost(message, optional, tag = "12")]
    pub derived_info: ::core::option::Option<company::DerivedInfo>,
    /// Output only. Indicates whether a company is flagged to be suspended from
    /// public availability by the service when job content appears suspicious,
    /// abusive, or spammy.
    #[prost(bool, tag = "13")]
    pub suspended: bool,
}
/// Nested message and enum types in `Company`.
pub mod company {
    /// Derived details about the company.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DerivedInfo {
        /// A structured headquarters location of the company, resolved from
        /// \[Company.headquarters_address][google.cloud.talent.v4.Company.headquarters_address\]
        /// if provided.
        #[prost(message, optional, tag = "1")]
        pub headquarters_location: ::core::option::Option<super::Location>,
    }
}
/// The Request of the CreateCompany method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCompanyRequest {
    /// Required. Resource name of the tenant under which the company is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The company to be created.
    #[prost(message, optional, tag = "2")]
    pub company: ::core::option::Option<Company>,
}
/// Request for getting a company by name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCompanyRequest {
    /// Required. The resource name of the company to be retrieved.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}", for
    /// example, "projects/api-test-project/tenants/foo/companies/bar".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for updating a specified company.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCompanyRequest {
    /// Required. The company resource to replace the current resource in the
    /// system.
    #[prost(message, optional, tag = "1")]
    pub company: ::core::option::Option<Company>,
    /// Strongly recommended for the best service experience.
    ///
    /// If \[update_mask][google.cloud.talent.v4.UpdateCompanyRequest.update_mask\]
    /// is provided, only the specified fields in
    /// \[company][google.cloud.talent.v4.UpdateCompanyRequest.company\] are updated.
    /// Otherwise all the fields are updated.
    ///
    /// A field mask to specify the company fields to be updated. Only
    /// top level fields of \[Company][google.cloud.talent.v4.Company\] are
    /// supported.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to delete a company.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCompanyRequest {
    /// Required. The resource name of the company to be deleted.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}", for
    /// example, "projects/foo/tenants/bar/companies/baz".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// List companies for which the client has ACL visibility.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCompaniesRequest {
    /// Required. Resource name of the tenant under which the company is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The starting indicator from which to return results.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of companies to be returned, at most 100.
    /// Default is 100 if a non-positive number is provided.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Set to true if the companies requested must have open jobs.
    ///
    /// Defaults to false.
    ///
    /// If true, at most
    /// \[page_size][google.cloud.talent.v4.ListCompaniesRequest.page_size\] of
    /// companies are fetched, among which only those with open jobs are returned.
    #[prost(bool, tag = "4")]
    pub require_open_jobs: bool,
}
/// The List companies response object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCompaniesResponse {
    /// Companies for the current client.
    #[prost(message, repeated, tag = "1")]
    pub companies: ::prost::alloc::vec::Vec<Company>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
/// Generated client implementations.
pub mod company_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service that handles company management, including CRUD and enumeration.
    #[derive(Debug, Clone)]
    pub struct CompanyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CompanyServiceClient<T>
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
        ) -> CompanyServiceClient<InterceptedService<T, F>>
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
            CompanyServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new company entity.
        pub async fn create_company(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCompanyRequest>,
        ) -> Result<tonic::Response<super::Company>, tonic::Status> {
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
                "/google.cloud.talent.v4.CompanyService/CreateCompany",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves specified company.
        pub async fn get_company(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCompanyRequest>,
        ) -> Result<tonic::Response<super::Company>, tonic::Status> {
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
                "/google.cloud.talent.v4.CompanyService/GetCompany",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates specified company.
        pub async fn update_company(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCompanyRequest>,
        ) -> Result<tonic::Response<super::Company>, tonic::Status> {
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
                "/google.cloud.talent.v4.CompanyService/UpdateCompany",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes specified company.
        /// Prerequisite: The company has no jobs associated with it.
        pub async fn delete_company(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCompanyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.talent.v4.CompanyService/DeleteCompany",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all companies associated with the project.
        pub async fn list_companies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCompaniesRequest>,
        ) -> Result<tonic::Response<super::ListCompaniesResponse>, tonic::Status> {
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
                "/google.cloud.talent.v4.CompanyService/ListCompanies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A Tenant resource represents a tenant in the service. A tenant is a group or
/// entity that shares common access with specific privileges for resources like
/// jobs. Customer may create multiple tenants to provide data isolation for
/// different groups.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tenant {
    /// Required during tenant update.
    ///
    /// The resource name for a tenant. This is generated by the service when a
    /// tenant is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Client side tenant identifier, used to uniquely identify the
    /// tenant.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "2")]
    pub external_id: ::prost::alloc::string::String,
}
/// The Request of the CreateTenant method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTenantRequest {
    /// Required. Resource name of the project under which the tenant is created.
    ///
    /// The format is "projects/{project_id}", for example,
    /// "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The tenant to be created.
    #[prost(message, optional, tag = "2")]
    pub tenant: ::core::option::Option<Tenant>,
}
/// Request for getting a tenant by name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTenantRequest {
    /// Required. The resource name of the tenant to be retrieved.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for updating a specified tenant.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTenantRequest {
    /// Required. The tenant resource to replace the current resource in the
    /// system.
    #[prost(message, optional, tag = "1")]
    pub tenant: ::core::option::Option<Tenant>,
    /// Strongly recommended for the best service experience.
    ///
    /// If \[update_mask][google.cloud.talent.v4.UpdateTenantRequest.update_mask\] is
    /// provided, only the specified fields in
    /// \[tenant][google.cloud.talent.v4.UpdateTenantRequest.tenant\] are updated.
    /// Otherwise all the fields are updated.
    ///
    /// A field mask to specify the tenant fields to be updated. Only
    /// top level fields of \[Tenant][google.cloud.talent.v4.Tenant\] are supported.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to delete a tenant.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTenantRequest {
    /// Required. The resource name of the tenant to be deleted.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// List tenants for which the client has ACL visibility.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTenantsRequest {
    /// Required. Resource name of the project under which the tenant is created.
    ///
    /// The format is "projects/{project_id}", for example,
    /// "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The starting indicator from which to return results.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of tenants to be returned, at most 100.
    /// Default is 100 if a non-positive number is provided.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// The List tenants response object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTenantsResponse {
    /// Tenants for the current client.
    #[prost(message, repeated, tag = "1")]
    pub tenants: ::prost::alloc::vec::Vec<Tenant>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
/// Generated client implementations.
pub mod tenant_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service that handles tenant management, including CRUD and enumeration.
    #[derive(Debug, Clone)]
    pub struct TenantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TenantServiceClient<T>
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
        ) -> TenantServiceClient<InterceptedService<T, F>>
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
            TenantServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new tenant entity.
        pub async fn create_tenant(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTenantRequest>,
        ) -> Result<tonic::Response<super::Tenant>, tonic::Status> {
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
                "/google.cloud.talent.v4.TenantService/CreateTenant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves specified tenant.
        pub async fn get_tenant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTenantRequest>,
        ) -> Result<tonic::Response<super::Tenant>, tonic::Status> {
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
                "/google.cloud.talent.v4.TenantService/GetTenant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates specified tenant.
        pub async fn update_tenant(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTenantRequest>,
        ) -> Result<tonic::Response<super::Tenant>, tonic::Status> {
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
                "/google.cloud.talent.v4.TenantService/UpdateTenant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes specified tenant.
        pub async fn delete_tenant(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTenantRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.talent.v4.TenantService/DeleteTenant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all tenants associated with the project.
        pub async fn list_tenants(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTenantsRequest>,
        ) -> Result<tonic::Response<super::ListTenantsResponse>, tonic::Status> {
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
                "/google.cloud.talent.v4.TenantService/ListTenants",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// An event issued when an end user interacts with the application that
/// implements Cloud Talent Solution. Providing this information improves the
/// quality of results for the API clients, enabling the
/// service to perform optimally. The number of events sent must be consistent
/// with other calls, such as job searches, issued to the service by the client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientEvent {
    /// Strongly recommended for the best service experience.
    ///
    /// A unique ID generated in the API responses. It can be found in
    /// \[ResponseMetadata.request_id][google.cloud.talent.v4.ResponseMetadata.request_id\].
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    /// Required. A unique identifier, generated by the client application.
    #[prost(string, tag = "2")]
    pub event_id: ::prost::alloc::string::String,
    /// Required. The timestamp of the event.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Notes about the event provided by recruiters or other users, for example,
    /// feedback on why a job was bookmarked.
    #[prost(string, tag = "9")]
    pub event_notes: ::prost::alloc::string::String,
    /// Required.
    ///
    /// The detail information of a specific event type.
    #[prost(oneof = "client_event::Event", tags = "5")]
    pub event: ::core::option::Option<client_event::Event>,
}
/// Nested message and enum types in `ClientEvent`.
pub mod client_event {
    /// Required.
    ///
    /// The detail information of a specific event type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// An event issued when a job seeker interacts with the application that
        /// implements Cloud Talent Solution.
        #[prost(message, tag = "5")]
        JobEvent(super::JobEvent),
    }
}
/// An event issued when a job seeker interacts with the application that
/// implements Cloud Talent Solution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobEvent {
    /// Required. The type of the event (see
    /// \[JobEventType][google.cloud.talent.v4.JobEvent.JobEventType\]).
    #[prost(enumeration = "job_event::JobEventType", tag = "1")]
    pub r#type: i32,
    /// Required. The [job name(s)]\[google.cloud.talent.v4.Job.name\] associated
    /// with this event. For example, if this is an
    /// \[impression][google.cloud.talent.v4.JobEvent.JobEventType.IMPRESSION\]
    /// event, this field contains the identifiers of all jobs shown to the job
    /// seeker. If this was a
    /// \[view][google.cloud.talent.v4.JobEvent.JobEventType.VIEW\] event, this field
    /// contains the identifier of the viewed job.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}", for
    /// example, "projects/foo/tenants/bar/jobs/baz".
    #[prost(string, repeated, tag = "2")]
    pub jobs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `JobEvent`.
pub mod job_event {
    /// An enumeration of an event attributed to the behavior of the end user,
    /// such as a job seeker.
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
    pub enum JobEventType {
        /// The event is unspecified by other provided values.
        Unspecified = 0,
        /// The job seeker or other entity interacting with the service has
        /// had a job rendered in their view, such as in a list of search results in
        /// a compressed or clipped format. This event is typically associated with
        /// the viewing of a jobs list on a single page by a job seeker.
        Impression = 1,
        /// The job seeker, or other entity interacting with the service, has
        /// viewed the details of a job, including the full description. This
        /// event doesn't apply to the viewing a snippet of a job appearing as a
        /// part of the job search results. Viewing a snippet is associated with an
        /// \[impression][google.cloud.talent.v4.JobEvent.JobEventType.IMPRESSION\]).
        View = 2,
        /// The job seeker or other entity interacting with the service
        /// performed an action to view a job and was redirected to a different
        /// website for job.
        ViewRedirect = 3,
        /// The job seeker or other entity interacting with the service
        /// began the process or demonstrated the intention of applying for a job.
        ApplicationStart = 4,
        /// The job seeker or other entity interacting with the service
        /// submitted an application for a job.
        ApplicationFinish = 5,
        /// The job seeker or other entity interacting with the service
        /// submitted an application for a job with a single click without
        /// entering information. If a job seeker performs this action, send only
        /// this event to the service. Do not also send
        /// \[JobEventType.APPLICATION_START][google.cloud.talent.v4.JobEvent.JobEventType.APPLICATION_START\]
        /// or
        /// \[JobEventType.APPLICATION_FINISH][google.cloud.talent.v4.JobEvent.JobEventType.APPLICATION_FINISH\]
        /// events.
        ApplicationQuickSubmission = 6,
        /// The job seeker or other entity interacting with the service
        /// performed an action to apply to a job and was redirected to a different
        /// website to complete the application.
        ApplicationRedirect = 7,
        /// The job seeker or other entity interacting with the service began the
        /// process or demonstrated the intention of applying for a job from the
        /// search results page without viewing the details of the job posting.
        /// If sending this event, JobEventType.VIEW event shouldn't be sent.
        ApplicationStartFromSearch = 8,
        /// The job seeker, or other entity interacting with the service, performs an
        /// action with a single click from the search results page to apply to a job
        /// (without viewing the details of the job posting), and is redirected
        /// to a different website to complete the application. If a candidate
        /// performs this action, send only this event to the service. Do not also
        /// send
        /// \[JobEventType.APPLICATION_START][google.cloud.talent.v4.JobEvent.JobEventType.APPLICATION_START\],
        /// \[JobEventType.APPLICATION_FINISH][google.cloud.talent.v4.JobEvent.JobEventType.APPLICATION_FINISH\]
        /// or \[JobEventType.VIEW][google.cloud.talent.v4.JobEvent.JobEventType.VIEW\]
        /// events.
        ApplicationRedirectFromSearch = 9,
        /// This event should be used when a company submits an application
        /// on behalf of a job seeker. This event is intended for use by staffing
        /// agencies attempting to place candidates.
        ApplicationCompanySubmit = 10,
        /// The job seeker or other entity interacting with the service demonstrated
        /// an interest in a job by bookmarking or saving it.
        Bookmark = 11,
        /// The job seeker or other entity interacting with the service was
        /// sent a notification, such as an email alert or device notification,
        /// containing one or more jobs listings generated by the service.
        Notification = 12,
        /// The job seeker or other entity interacting with the service was
        /// employed by the hiring entity (employer). Send this event
        /// only if the job seeker was hired through an application that was
        /// initiated by a search conducted through the Cloud Talent Solution
        /// service.
        Hired = 13,
        /// A recruiter or staffing agency submitted an application on behalf of the
        /// candidate after interacting with the service to identify a suitable job
        /// posting.
        SentCv = 14,
        /// The entity interacting with the service (for example, the job seeker),
        /// was granted an initial interview by the hiring entity (employer). This
        /// event should only be sent if the job seeker was granted an interview as
        /// part of an application that was initiated by a search conducted through /
        /// recommendation provided by the Cloud Talent Solution service.
        InterviewGranted = 15,
    }
    impl JobEventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JobEventType::Unspecified => "JOB_EVENT_TYPE_UNSPECIFIED",
                JobEventType::Impression => "IMPRESSION",
                JobEventType::View => "VIEW",
                JobEventType::ViewRedirect => "VIEW_REDIRECT",
                JobEventType::ApplicationStart => "APPLICATION_START",
                JobEventType::ApplicationFinish => "APPLICATION_FINISH",
                JobEventType::ApplicationQuickSubmission => {
                    "APPLICATION_QUICK_SUBMISSION"
                }
                JobEventType::ApplicationRedirect => "APPLICATION_REDIRECT",
                JobEventType::ApplicationStartFromSearch => {
                    "APPLICATION_START_FROM_SEARCH"
                }
                JobEventType::ApplicationRedirectFromSearch => {
                    "APPLICATION_REDIRECT_FROM_SEARCH"
                }
                JobEventType::ApplicationCompanySubmit => "APPLICATION_COMPANY_SUBMIT",
                JobEventType::Bookmark => "BOOKMARK",
                JobEventType::Notification => "NOTIFICATION",
                JobEventType::Hired => "HIRED",
                JobEventType::SentCv => "SENT_CV",
                JobEventType::InterviewGranted => "INTERVIEW_GRANTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "JOB_EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "IMPRESSION" => Some(Self::Impression),
                "VIEW" => Some(Self::View),
                "VIEW_REDIRECT" => Some(Self::ViewRedirect),
                "APPLICATION_START" => Some(Self::ApplicationStart),
                "APPLICATION_FINISH" => Some(Self::ApplicationFinish),
                "APPLICATION_QUICK_SUBMISSION" => Some(Self::ApplicationQuickSubmission),
                "APPLICATION_REDIRECT" => Some(Self::ApplicationRedirect),
                "APPLICATION_START_FROM_SEARCH" => Some(Self::ApplicationStartFromSearch),
                "APPLICATION_REDIRECT_FROM_SEARCH" => {
                    Some(Self::ApplicationRedirectFromSearch)
                }
                "APPLICATION_COMPANY_SUBMIT" => Some(Self::ApplicationCompanySubmit),
                "BOOKMARK" => Some(Self::Bookmark),
                "NOTIFICATION" => Some(Self::Notification),
                "HIRED" => Some(Self::Hired),
                "SENT_CV" => Some(Self::SentCv),
                "INTERVIEW_GRANTED" => Some(Self::InterviewGranted),
                _ => None,
            }
        }
    }
}
/// The report event request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClientEventRequest {
    /// Required. Resource name of the tenant under which the event is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Events issued when end user interacts with customer's application
    /// that uses Cloud Talent Solution.
    #[prost(message, optional, tag = "2")]
    pub client_event: ::core::option::Option<ClientEvent>,
}
/// Generated client implementations.
pub mod event_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service handles client event report.
    #[derive(Debug, Clone)]
    pub struct EventServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EventServiceClient<T>
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
        ) -> EventServiceClient<InterceptedService<T, F>>
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
            EventServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Report events issued when end user interacts with customer's application
        /// that uses Cloud Talent Solution. You may inspect the created events in
        /// [self service
        /// tools](https://console.cloud.google.com/talent-solution/overview).
        /// [Learn
        /// more](https://cloud.google.com/talent-solution/docs/management-tools)
        /// about self service tools.
        pub async fn create_client_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClientEventRequest>,
        ) -> Result<tonic::Response<super::ClientEvent>, tonic::Status> {
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
                "/google.cloud.talent.v4.EventService/CreateClientEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The query required to perform a search query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobQuery {
    /// The query string that matches against the job title, description, and
    /// location fields.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// The language code of \[query][google.cloud.talent.v4.JobQuery.query\]. For
    /// example, "en-US". This field helps to better interpret the query.
    ///
    /// If a value isn't specified, the query language code is automatically
    /// detected, which may not be accurate.
    ///
    /// Language code should be in BCP-47 format, such as "en-US" or "sr-Latn".
    /// For more information, see
    /// [Tags for Identifying Languages](<https://tools.ietf.org/html/bcp47>).
    #[prost(string, tag = "14")]
    pub query_language_code: ::prost::alloc::string::String,
    /// This filter specifies the company entities to search against.
    ///
    /// If a value isn't specified, jobs are searched for against all
    /// companies.
    ///
    /// If multiple values are specified, jobs are searched against the
    /// companies specified.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}". For
    /// example, "projects/foo/tenants/bar/companies/baz".
    ///
    /// At most 20 company filters are allowed.
    #[prost(string, repeated, tag = "2")]
    pub companies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The location filter specifies geo-regions containing the jobs to
    /// search against. See \[LocationFilter][google.cloud.talent.v4.LocationFilter\]
    /// for more information.
    ///
    /// If a location value isn't specified, jobs fitting the other search
    /// criteria are retrieved regardless of where they're located.
    ///
    /// If multiple values are specified, jobs are retrieved from any of the
    /// specified locations. If different values are specified for the
    /// \[LocationFilter.distance_in_miles][google.cloud.talent.v4.LocationFilter.distance_in_miles\]
    /// parameter, the maximum provided distance is used for all locations.
    ///
    /// At most 5 location filters are allowed.
    #[prost(message, repeated, tag = "3")]
    pub location_filters: ::prost::alloc::vec::Vec<LocationFilter>,
    /// The category filter specifies the categories of jobs to search against.
    /// See \[JobCategory][google.cloud.talent.v4.JobCategory\] for more information.
    ///
    /// If a value isn't specified, jobs from any category are searched against.
    ///
    /// If multiple values are specified, jobs from any of the specified
    /// categories are searched against.
    #[prost(enumeration = "JobCategory", repeated, tag = "4")]
    pub job_categories: ::prost::alloc::vec::Vec<i32>,
    /// Allows filtering jobs by commute time with different travel methods (for
    ///   example, driving or public transit).
    ///
    /// Note: This only works when you specify a
    /// \[CommuteMethod][google.cloud.talent.v4.CommuteMethod\]. In this case,
    /// \[location_filters][google.cloud.talent.v4.JobQuery.location_filters\] is
    /// ignored.
    ///
    ///   Currently we don't support sorting by commute time.
    #[prost(message, optional, tag = "5")]
    pub commute_filter: ::core::option::Option<CommuteFilter>,
    /// This filter specifies the company
    /// \[Company.display_name][google.cloud.talent.v4.Company.display_name\] of the
    /// jobs to search against. The company name must match the value exactly.
    ///
    /// Alternatively, the value being searched for can be wrapped in different
    /// match operators.
    /// `SUBSTRING_MATCH(\[value\])`
    /// The company name must contain a case insensitive substring match of the
    /// value. Using this function may increase latency.
    ///
    /// Sample Value: `SUBSTRING_MATCH(google)`
    ///
    /// `MULTI_WORD_TOKEN_MATCH(\[value\])`
    /// The value will be treated as a multi word token and the company name must
    /// contain a case insensitive match of the value. Using this function may
    /// increase latency.
    ///
    /// Sample Value: `MULTI_WORD_TOKEN_MATCH(google)`
    ///
    /// If a value isn't specified, jobs within the search results are
    /// associated with any company.
    ///
    /// If multiple values are specified, jobs within the search results may be
    /// associated with any of the specified companies.
    ///
    /// At most 20 company display name filters are allowed.
    #[prost(string, repeated, tag = "6")]
    pub company_display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// This search filter is applied only to
    /// \[Job.compensation_info][google.cloud.talent.v4.Job.compensation_info\]. For
    /// example, if the filter is specified as "Hourly job with per-hour
    /// compensation > $15", only jobs meeting these criteria are searched. If a
    /// filter isn't defined, all open jobs are searched.
    #[prost(message, optional, tag = "7")]
    pub compensation_filter: ::core::option::Option<CompensationFilter>,
    /// This filter specifies a structured syntax to match against the
    /// \[Job.custom_attributes][google.cloud.talent.v4.Job.custom_attributes\]
    /// marked as `filterable`.
    ///
    /// The syntax for this expression is a subset of SQL syntax.
    ///
    /// Supported operators are: `=`, `!=`, `<`, `<=`, `>`, and `>=` where the
    /// left of the operator is a custom field key and the right of the operator
    /// is a number or a quoted string. You must escape backslash (\\) and
    /// quote (\") characters.
    ///
    /// Supported functions are `LOWER(\[field_name\])` to
    /// perform a case insensitive match and `EMPTY(\[field_name\])` to filter on the
    /// existence of a key.
    ///
    /// Boolean expressions (AND/OR/NOT) are supported up to 3 levels of
    /// nesting (for example, "((A AND B AND C) OR NOT D) AND E"), a maximum of 100
    /// comparisons or functions are allowed in the expression. The expression
    /// must be < 10000 bytes in length.
    ///
    /// Sample Query:
    /// `(LOWER(driving_license)="class \"a\"" OR EMPTY(driving_license)) AND
    /// driving_years > 10`
    #[prost(string, tag = "8")]
    pub custom_attribute_filter: ::prost::alloc::string::String,
    /// This flag controls the spell-check feature. If false, the
    /// service attempts to correct a misspelled query,
    /// for example, "enginee" is corrected to "engineer".
    ///
    /// Defaults to false: a spell check is performed.
    #[prost(bool, tag = "9")]
    pub disable_spell_check: bool,
    /// The employment type filter specifies the employment type of jobs to
    /// search against, such as
    /// \[EmploymentType.FULL_TIME][google.cloud.talent.v4.EmploymentType.FULL_TIME\].
    ///
    /// If a value isn't specified, jobs in the search results includes any
    /// employment type.
    ///
    /// If multiple values are specified, jobs in the search results include
    /// any of the specified employment types.
    #[prost(enumeration = "EmploymentType", repeated, tag = "10")]
    pub employment_types: ::prost::alloc::vec::Vec<i32>,
    /// This filter specifies the locale of jobs to search against,
    /// for example, "en-US".
    ///
    /// If a value isn't specified, the search results can contain jobs in any
    /// locale.
    ///
    ///
    /// Language codes should be in BCP-47 format, such as "en-US" or "sr-Latn".
    /// For more information, see
    /// [Tags for Identifying Languages](<https://tools.ietf.org/html/bcp47>).
    ///
    /// At most 10 language code filters are allowed.
    #[prost(string, repeated, tag = "11")]
    pub language_codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Jobs published within a range specified by this filter are searched
    /// against.
    #[prost(message, optional, tag = "12")]
    pub publish_time_range: ::core::option::Option<TimestampRange>,
    /// This filter specifies a list of job names to be excluded during search.
    ///
    /// At most 400 excluded job names are allowed.
    #[prost(string, repeated, tag = "13")]
    pub excluded_jobs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Geographic region of the search.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationFilter {
    /// The address name, such as "Mountain View" or "Bay Area".
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// CLDR region code of the country/region. This field may be used in two ways:
    ///
    /// 1) If telecommute preference is not set, this field is used address
    /// ambiguity of the user-input address. For example, "Liverpool" may refer to
    /// "Liverpool, NY, US" or "Liverpool, UK". This region code biases the
    /// address resolution toward a specific country or territory. If this field is
    /// not set, address resolution is biased toward the United States by default.
    ///
    /// 2) If telecommute preference is set to TELECOMMUTE_ALLOWED, the
    /// telecommute location filter will be limited to the region specified in this
    /// field. If this field is not set, the telecommute job locations will not be
    ///
    /// See
    /// <https://unicode-org.github.io/cldr-staging/charts/latest/supplemental/territory_information.html>
    /// for details. Example: "CH" for Switzerland.
    #[prost(string, tag = "2")]
    pub region_code: ::prost::alloc::string::String,
    /// The latitude and longitude of the geographic center to search from. This
    /// field is ignored if `address` is provided.
    #[prost(message, optional, tag = "3")]
    pub lat_lng: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The distance_in_miles is applied when the location being searched for is
    /// identified as a city or smaller. This field is ignored if the location
    /// being searched for is a state or larger.
    #[prost(double, tag = "4")]
    pub distance_in_miles: f64,
    /// Allows the client to return jobs without a
    /// set location, specifically, telecommuting jobs (telecommuting is considered
    /// by the service as a special location).
    /// \[Job.posting_region][google.cloud.talent.v4.Job.posting_region\] indicates
    /// if a job permits telecommuting. If this field is set to
    /// \[TelecommutePreference.TELECOMMUTE_ALLOWED][google.cloud.talent.v4.LocationFilter.TelecommutePreference.TELECOMMUTE_ALLOWED\],
    /// telecommuting jobs are searched, and
    /// \[address][google.cloud.talent.v4.LocationFilter.address\] and
    /// \[lat_lng][google.cloud.talent.v4.LocationFilter.lat_lng\] are ignored. If
    /// not set or set to
    /// \[TelecommutePreference.TELECOMMUTE_EXCLUDED][google.cloud.talent.v4.LocationFilter.TelecommutePreference.TELECOMMUTE_EXCLUDED\],
    /// the telecommute status of the jobs is ignored. Jobs that have
    /// \[PostingRegion.TELECOMMUTE][google.cloud.talent.v4.PostingRegion.TELECOMMUTE\]
    /// and have additional \[Job.addresses][google.cloud.talent.v4.Job.addresses\]
    /// may still be matched based on other location filters using
    /// \[address][google.cloud.talent.v4.LocationFilter.address\] or \[latlng][\].
    ///
    /// This filter can be used by itself to search exclusively for telecommuting
    /// jobs, or it can be combined with another location
    /// filter to search for a combination of job locations,
    /// such as "Mountain View" or "telecommuting" jobs. However, when used in
    /// combination with other location filters, telecommuting jobs can be
    /// treated as less relevant than other jobs in the search response.
    ///
    /// This field is only used for job search requests.
    #[prost(enumeration = "location_filter::TelecommutePreference", tag = "5")]
    pub telecommute_preference: i32,
}
/// Nested message and enum types in `LocationFilter`.
pub mod location_filter {
    /// Specify whether to include telecommute jobs.
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
    pub enum TelecommutePreference {
        /// Default value if the telecommute preference isn't specified.
        Unspecified = 0,
        /// Deprecated: Ignore telecommute status of jobs. Use
        /// TELECOMMUTE_JOBS_EXCLUDED if want to exclude telecommute jobs.
        TelecommuteExcluded = 1,
        /// Allow telecommute jobs.
        TelecommuteAllowed = 2,
        /// Exclude telecommute jobs.
        TelecommuteJobsExcluded = 3,
    }
    impl TelecommutePreference {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TelecommutePreference::Unspecified => {
                    "TELECOMMUTE_PREFERENCE_UNSPECIFIED"
                }
                TelecommutePreference::TelecommuteExcluded => "TELECOMMUTE_EXCLUDED",
                TelecommutePreference::TelecommuteAllowed => "TELECOMMUTE_ALLOWED",
                TelecommutePreference::TelecommuteJobsExcluded => {
                    "TELECOMMUTE_JOBS_EXCLUDED"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TELECOMMUTE_PREFERENCE_UNSPECIFIED" => Some(Self::Unspecified),
                "TELECOMMUTE_EXCLUDED" => Some(Self::TelecommuteExcluded),
                "TELECOMMUTE_ALLOWED" => Some(Self::TelecommuteAllowed),
                "TELECOMMUTE_JOBS_EXCLUDED" => Some(Self::TelecommuteJobsExcluded),
                _ => None,
            }
        }
    }
}
/// Filter on job compensation type and amount.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompensationFilter {
    /// Required. Type of filter.
    #[prost(enumeration = "compensation_filter::FilterType", tag = "1")]
    pub r#type: i32,
    /// Required. Specify desired `base compensation entry's`
    /// \[CompensationInfo.CompensationUnit][google.cloud.talent.v4.CompensationInfo.CompensationUnit\].
    #[prost(
        enumeration = "compensation_info::CompensationUnit",
        repeated,
        packed = "false",
        tag = "2"
    )]
    pub units: ::prost::alloc::vec::Vec<i32>,
    /// Compensation range.
    #[prost(message, optional, tag = "3")]
    pub range: ::core::option::Option<compensation_info::CompensationRange>,
    /// If set to true, jobs with unspecified compensation range fields are
    /// included.
    #[prost(bool, tag = "4")]
    pub include_jobs_with_unspecified_compensation_range: bool,
}
/// Nested message and enum types in `CompensationFilter`.
pub mod compensation_filter {
    /// Specify the type of filtering.
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
    pub enum FilterType {
        /// Filter type unspecified. Position holder, INVALID, should never be used.
        Unspecified = 0,
        /// Filter by `base compensation entry's` unit. A job is a match if and
        /// only if the job contains a base CompensationEntry and the base
        /// CompensationEntry's unit matches provided
        /// \[units][google.cloud.talent.v4.CompensationFilter.units\]. Populate one or
        /// more \[units][google.cloud.talent.v4.CompensationFilter.units\].
        ///
        /// See
        /// \[CompensationInfo.CompensationEntry][google.cloud.talent.v4.CompensationInfo.CompensationEntry\]
        /// for definition of base compensation entry.
        UnitOnly = 1,
        /// Filter by `base compensation entry's` unit and amount / range. A job
        /// is a match if and only if the job contains a base CompensationEntry, and
        /// the base entry's unit matches provided
        /// \[CompensationUnit][google.cloud.talent.v4.CompensationInfo.CompensationUnit\]
        /// and amount or range overlaps with provided
        /// \[CompensationRange][google.cloud.talent.v4.CompensationInfo.CompensationRange\].
        ///
        /// See
        /// \[CompensationInfo.CompensationEntry][google.cloud.talent.v4.CompensationInfo.CompensationEntry\]
        /// for definition of base compensation entry.
        ///
        /// Set exactly one \[units][google.cloud.talent.v4.CompensationFilter.units\]
        /// and populate \[range][google.cloud.talent.v4.CompensationFilter.range\].
        UnitAndAmount = 2,
        /// Filter by annualized base compensation amount and `base compensation
        /// entry's` unit. Populate
        /// \[range][google.cloud.talent.v4.CompensationFilter.range\] and zero or more
        /// \[units][google.cloud.talent.v4.CompensationFilter.units\].
        AnnualizedBaseAmount = 3,
        /// Filter by annualized total compensation amount and `base compensation
        /// entry's` unit . Populate
        /// \[range][google.cloud.talent.v4.CompensationFilter.range\] and zero or more
        /// \[units][google.cloud.talent.v4.CompensationFilter.units\].
        AnnualizedTotalAmount = 4,
    }
    impl FilterType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FilterType::Unspecified => "FILTER_TYPE_UNSPECIFIED",
                FilterType::UnitOnly => "UNIT_ONLY",
                FilterType::UnitAndAmount => "UNIT_AND_AMOUNT",
                FilterType::AnnualizedBaseAmount => "ANNUALIZED_BASE_AMOUNT",
                FilterType::AnnualizedTotalAmount => "ANNUALIZED_TOTAL_AMOUNT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FILTER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "UNIT_ONLY" => Some(Self::UnitOnly),
                "UNIT_AND_AMOUNT" => Some(Self::UnitAndAmount),
                "ANNUALIZED_BASE_AMOUNT" => Some(Self::AnnualizedBaseAmount),
                "ANNUALIZED_TOTAL_AMOUNT" => Some(Self::AnnualizedTotalAmount),
                _ => None,
            }
        }
    }
}
/// Parameters needed for commute search.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommuteFilter {
    /// Required. The method of transportation to calculate the commute time for.
    #[prost(enumeration = "CommuteMethod", tag = "1")]
    pub commute_method: i32,
    /// Required. The latitude and longitude of the location to calculate the
    /// commute time from.
    #[prost(message, optional, tag = "2")]
    pub start_coordinates: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// Required. The maximum travel time in seconds. The maximum allowed value is
    /// `3600s` (one hour). Format is `123s`.
    #[prost(message, optional, tag = "3")]
    pub travel_duration: ::core::option::Option<::prost_types::Duration>,
    /// If `true`, jobs without street level addresses may also be returned.
    /// For city level addresses, the city center is used. For state and coarser
    /// level addresses, text matching is used.
    /// If this field is set to `false` or isn't specified, only jobs that include
    /// street level addresses will be returned by commute search.
    #[prost(bool, tag = "4")]
    pub allow_imprecise_addresses: bool,
    /// Traffic factor to take into account while searching by commute.
    #[prost(oneof = "commute_filter::TrafficOption", tags = "5, 6")]
    pub traffic_option: ::core::option::Option<commute_filter::TrafficOption>,
}
/// Nested message and enum types in `CommuteFilter`.
pub mod commute_filter {
    /// The traffic density to use when calculating commute time.
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
    pub enum RoadTraffic {
        /// Road traffic situation isn't specified.
        Unspecified = 0,
        /// Optimal commute time without considering any traffic impact.
        TrafficFree = 1,
        /// Commute time calculation takes in account the peak traffic impact.
        BusyHour = 2,
    }
    impl RoadTraffic {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RoadTraffic::Unspecified => "ROAD_TRAFFIC_UNSPECIFIED",
                RoadTraffic::TrafficFree => "TRAFFIC_FREE",
                RoadTraffic::BusyHour => "BUSY_HOUR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROAD_TRAFFIC_UNSPECIFIED" => Some(Self::Unspecified),
                "TRAFFIC_FREE" => Some(Self::TrafficFree),
                "BUSY_HOUR" => Some(Self::BusyHour),
                _ => None,
            }
        }
    }
    /// Traffic factor to take into account while searching by commute.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TrafficOption {
        /// Specifies the traffic density to use when calculating commute time.
        #[prost(enumeration = "RoadTraffic", tag = "5")]
        RoadTraffic(i32),
        /// The departure time used to calculate traffic impact, represented as
        /// \[google.type.TimeOfDay][google.type.TimeOfDay\] in local time zone.
        ///
        /// Currently traffic model is restricted to hour level resolution.
        #[prost(message, tag = "6")]
        DepartureTime(super::super::super::super::r#type::TimeOfDay),
    }
}
/// The histogram request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramQuery {
    /// An expression specifies a histogram request against matching jobs for
    /// searches.
    ///
    /// See
    /// \[SearchJobsRequest.histogram_queries][google.cloud.talent.v4.SearchJobsRequest.histogram_queries\]
    /// for details about syntax.
    #[prost(string, tag = "1")]
    pub histogram_query: ::prost::alloc::string::String,
}
/// Histogram result that matches
/// \[HistogramQuery][google.cloud.talent.v4.HistogramQuery\] specified in
/// searches.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramQueryResult {
    /// Requested histogram expression.
    #[prost(string, tag = "1")]
    pub histogram_query: ::prost::alloc::string::String,
    /// A map from the values of the facet associated with distinct values to the
    /// number of matching entries with corresponding value.
    ///
    /// The key format is:
    ///
    /// * (for string histogram) string values stored in the field.
    /// * (for named numeric bucket) name specified in `bucket()` function, like
    ///    for `bucket(0, MAX, "non-negative")`, the key will be `non-negative`.
    /// * (for anonymous numeric bucket) range formatted as `<low>-<high>`, for
    ///    example, `0-1000`, `MIN-0`, and `0-MAX`.
    #[prost(btree_map = "string, int64", tag = "2")]
    pub histogram: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        i64,
    >,
}
/// A Job resource represents a job posting (also referred to as a "job listing"
/// or "job requisition"). A job belongs to a
/// \[Company][google.cloud.talent.v4.Company\], which is the hiring entity
/// responsible for the job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// Required during job update.
    ///
    /// The resource name for the job. This is generated by the service when a
    /// job is created.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}". For
    /// example, "projects/foo/tenants/bar/jobs/baz".
    ///
    /// Use of this field in job queries and API calls is preferred over the use of
    /// \[requisition_id][google.cloud.talent.v4.Job.requisition_id\] since this
    /// value is unique.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The resource name of the company listing the job.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}". For
    /// example, "projects/foo/tenants/bar/companies/baz".
    #[prost(string, tag = "2")]
    pub company: ::prost::alloc::string::String,
    /// Required. The requisition ID, also referred to as the posting ID, is
    /// assigned by the client to identify a job. This field is intended to be used
    /// by clients for client identification and tracking of postings. A job isn't
    /// allowed to be created if there is another job with the same
    /// \[company][google.cloud.talent.v4.Job.name\],
    /// \[language_code][google.cloud.talent.v4.Job.language_code\] and
    /// \[requisition_id][google.cloud.talent.v4.Job.requisition_id\].
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "3")]
    pub requisition_id: ::prost::alloc::string::String,
    /// Required. The title of the job, such as "Software Engineer"
    ///
    /// The maximum number of allowed characters is 500.
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    /// Required. The description of the job, which typically includes a
    /// multi-paragraph description of the company and related information.
    /// Separate fields are provided on the job object for
    /// \[responsibilities][google.cloud.talent.v4.Job.responsibilities\],
    /// \[qualifications][google.cloud.talent.v4.Job.qualifications\], and other job
    /// characteristics. Use of these separate job fields is recommended.
    ///
    /// This field accepts and sanitizes HTML input, and also accepts
    /// bold, italic, ordered list, and unordered list markup tags.
    ///
    /// The maximum number of allowed characters is 100,000.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Strongly recommended for the best service experience.
    ///
    /// Location(s) where the employer is looking to hire for this job posting.
    ///
    /// Specifying the full street address(es) of the hiring location enables
    /// better API results, especially job searches by commute time.
    ///
    /// At most 50 locations are allowed for best search performance. If a job has
    /// more locations, it is suggested to split it into multiple jobs with unique
    /// \[requisition_id][google.cloud.talent.v4.Job.requisition_id\]s (e.g. 'ReqA'
    /// becomes 'ReqA-1', 'ReqA-2', and so on.) as multiple jobs with the same
    /// \[company][google.cloud.talent.v4.Job.company\],
    /// \[language_code][google.cloud.talent.v4.Job.language_code\] and
    /// \[requisition_id][google.cloud.talent.v4.Job.requisition_id\] are not
    /// allowed. If the original
    /// \[requisition_id][google.cloud.talent.v4.Job.requisition_id\] must be
    /// preserved, a custom field should be used for storage. It is also suggested
    /// to group the locations that close to each other in the same job for better
    /// search experience.
    ///
    /// Jobs with multiple addresses must have their addresses with the same
    /// \[LocationType][\] to allow location filtering to work properly. (For
    /// example, a Job with addresses "1600 Amphitheatre Parkway, Mountain View,
    /// CA, USA" and "London, UK" may not have location filters applied correctly
    /// at search time since the first is a \[LocationType.STREET_ADDRESS][\] and the
    /// second is a \[LocationType.LOCALITY][\].) If a job needs to have multiple
    /// addresses, it is suggested to split it into multiple jobs with same
    /// LocationTypes.
    ///
    /// The maximum number of allowed characters is 500.
    #[prost(string, repeated, tag = "6")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Job application information.
    #[prost(message, optional, tag = "7")]
    pub application_info: ::core::option::Option<job::ApplicationInfo>,
    /// The benefits included with the job.
    #[prost(enumeration = "JobBenefit", repeated, tag = "8")]
    pub job_benefits: ::prost::alloc::vec::Vec<i32>,
    /// Job compensation information (a.k.a. "pay rate") i.e., the compensation
    /// that will paid to the employee.
    #[prost(message, optional, tag = "9")]
    pub compensation_info: ::core::option::Option<CompensationInfo>,
    /// A map of fields to hold both filterable and non-filterable custom job
    /// attributes that are not covered by the provided structured fields.
    ///
    /// The keys of the map are strings up to 64 bytes and must match the
    /// pattern: `\[a-zA-Z][a-zA-Z0-9_\]*`. For example, key0LikeThis or
    /// KEY_1_LIKE_THIS.
    ///
    /// At most 100 filterable and at most 100 unfilterable keys are supported.
    /// For filterable `string_values`, across all keys at most 200 values are
    /// allowed, with each string no more than 255 characters. For unfilterable
    /// `string_values`, the maximum total size of `string_values` across all keys
    /// is 50KB.
    #[prost(btree_map = "string, message", tag = "10")]
    pub custom_attributes: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        CustomAttribute,
    >,
    /// The desired education degrees for the job, such as Bachelors, Masters.
    #[prost(enumeration = "DegreeType", repeated, tag = "11")]
    pub degree_types: ::prost::alloc::vec::Vec<i32>,
    /// The department or functional area within the company with the open
    /// position.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "12")]
    pub department: ::prost::alloc::string::String,
    /// The employment type(s) of a job, for example,
    /// [full time]\[google.cloud.talent.v4.EmploymentType.FULL_TIME\] or
    /// [part time]\[google.cloud.talent.v4.EmploymentType.PART_TIME\].
    #[prost(enumeration = "EmploymentType", repeated, tag = "13")]
    pub employment_types: ::prost::alloc::vec::Vec<i32>,
    /// A description of bonus, commission, and other compensation
    /// incentives associated with the job not including salary or pay.
    ///
    /// The maximum number of allowed characters is 10,000.
    #[prost(string, tag = "14")]
    pub incentives: ::prost::alloc::string::String,
    /// The language of the posting. This field is distinct from
    /// any requirements for fluency that are associated with the job.
    ///
    /// Language codes must be in BCP-47 format, such as "en-US" or "sr-Latn".
    /// For more information, see
    /// [Tags for Identifying Languages](<https://tools.ietf.org/html/bcp47>){:
    /// class="external" target="_blank" }.
    ///
    /// If this field is unspecified and
    /// \[Job.description][google.cloud.talent.v4.Job.description\] is present,
    /// detected language code based on
    /// \[Job.description][google.cloud.talent.v4.Job.description\] is assigned,
    /// otherwise defaults to 'en_US'.
    #[prost(string, tag = "15")]
    pub language_code: ::prost::alloc::string::String,
    /// The experience level associated with the job, such as "Entry Level".
    #[prost(enumeration = "JobLevel", tag = "16")]
    pub job_level: i32,
    /// A promotion value of the job, as determined by the client.
    /// The value determines the sort order of the jobs returned when searching for
    /// jobs using the featured jobs search call, with higher promotional values
    /// being returned first and ties being resolved by relevance sort. Only the
    /// jobs with a promotionValue >0 are returned in a FEATURED_JOB_SEARCH.
    ///
    /// Default value is 0, and negative values are treated as 0.
    #[prost(int32, tag = "17")]
    pub promotion_value: i32,
    /// A description of the qualifications required to perform the
    /// job. The use of this field is recommended
    /// as an alternative to using the more general
    /// \[description][google.cloud.talent.v4.Job.description\] field.
    ///
    /// This field accepts and sanitizes HTML input, and also accepts
    /// bold, italic, ordered list, and unordered list markup tags.
    ///
    /// The maximum number of allowed characters is 10,000.
    #[prost(string, tag = "18")]
    pub qualifications: ::prost::alloc::string::String,
    /// A description of job responsibilities. The use of this field is
    /// recommended as an alternative to using the more general
    /// \[description][google.cloud.talent.v4.Job.description\] field.
    ///
    /// This field accepts and sanitizes HTML input, and also accepts
    /// bold, italic, ordered list, and unordered list markup tags.
    ///
    /// The maximum number of allowed characters is 10,000.
    #[prost(string, tag = "19")]
    pub responsibilities: ::prost::alloc::string::String,
    /// The job \[PostingRegion][google.cloud.talent.v4.PostingRegion\] (for example,
    /// state, country) throughout which the job is available. If this field is
    /// set, a \[LocationFilter][google.cloud.talent.v4.LocationFilter\] in a search
    /// query within the job region finds this job posting if an exact location
    /// match isn't specified. If this field is set to
    /// \[PostingRegion.NATION][google.cloud.talent.v4.PostingRegion.NATION\] or
    /// \[PostingRegion.ADMINISTRATIVE_AREA][google.cloud.talent.v4.PostingRegion.ADMINISTRATIVE_AREA\],
    /// setting job \[Job.addresses][google.cloud.talent.v4.Job.addresses\] to the
    /// same location level as this field is strongly recommended.
    #[prost(enumeration = "PostingRegion", tag = "20")]
    pub posting_region: i32,
    /// Deprecated. The job is only visible to the owner.
    ///
    /// The visibility of the job.
    ///
    /// Defaults to
    /// \[Visibility.ACCOUNT_ONLY][google.cloud.talent.v4.Visibility.ACCOUNT_ONLY\]
    /// if not specified.
    #[deprecated]
    #[prost(enumeration = "Visibility", tag = "21")]
    pub visibility: i32,
    /// The start timestamp of the job in UTC time zone. Typically this field
    /// is used for contracting engagements. Invalid timestamps are ignored.
    #[prost(message, optional, tag = "22")]
    pub job_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end timestamp of the job. Typically this field is used for contracting
    /// engagements. Invalid timestamps are ignored.
    #[prost(message, optional, tag = "23")]
    pub job_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The timestamp this job posting was most recently published. The default
    /// value is the time the request arrives at the server. Invalid timestamps are
    /// ignored.
    #[prost(message, optional, tag = "24")]
    pub posting_publish_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Strongly recommended for the best service experience.
    ///
    /// The expiration timestamp of the job. After this timestamp, the
    /// job is marked as expired, and it no longer appears in search results. The
    /// expired job can't be listed by the
    /// \[ListJobs][google.cloud.talent.v4.JobService.ListJobs\] API, but it can be
    /// retrieved with the \[GetJob][google.cloud.talent.v4.JobService.GetJob\] API
    /// or updated with the
    /// \[UpdateJob][google.cloud.talent.v4.JobService.UpdateJob\] API or deleted
    /// with the \[DeleteJob][google.cloud.talent.v4.JobService.DeleteJob\] API. An
    /// expired job can be updated and opened again by using a future expiration
    /// timestamp. Updating an expired job fails if there is another existing open
    /// job with same \[company][google.cloud.talent.v4.Job.company\],
    /// \[language_code][google.cloud.talent.v4.Job.language_code\] and
    /// \[requisition_id][google.cloud.talent.v4.Job.requisition_id\].
    ///
    /// The expired jobs are retained in our system for 90 days. However, the
    /// overall expired job count cannot exceed 3 times the maximum number of
    /// open jobs over previous 7 days. If this threshold is exceeded,
    /// expired jobs are cleaned out in order of earliest expire time.
    /// Expired jobs are no longer accessible after they are cleaned
    /// out.
    ///
    /// Invalid timestamps are ignored, and treated as expire time not provided.
    ///
    /// If the timestamp is before the instant request is made, the job
    /// is treated as expired immediately on creation. This kind of job can
    /// not be updated. And when creating a job with past timestamp, the
    /// \[posting_publish_time][google.cloud.talent.v4.Job.posting_publish_time\]
    /// must be set before
    /// \[posting_expire_time][google.cloud.talent.v4.Job.posting_expire_time\]. The
    /// purpose of this feature is to allow other objects, such as \[Application][\],
    /// to refer a job that didn't exist in the system prior to becoming expired.
    /// If you want to modify a job that was expired on creation, delete it and
    /// create a new one.
    ///
    /// If this value isn't provided at the time of job creation or is invalid,
    /// the job posting expires after 30 days from the job's creation time. For
    /// example, if the job was created on 2017/01/01 13:00AM UTC with an
    /// unspecified expiration date, the job expires after 2017/01/31 13:00AM UTC.
    ///
    /// If this value isn't provided on job update, it depends on the field masks
    /// set by
    /// \[UpdateJobRequest.update_mask][google.cloud.talent.v4.UpdateJobRequest.update_mask\].
    /// If the field masks include
    /// \[job_end_time][google.cloud.talent.v4.Job.job_end_time\], or the masks are
    /// empty meaning that every field is updated, the job posting expires after 30
    /// days from the job's last update time. Otherwise the expiration date isn't
    /// updated.
    #[prost(message, optional, tag = "25")]
    pub posting_expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when this job posting was created.
    #[prost(message, optional, tag = "26")]
    pub posting_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when this job posting was last updated.
    #[prost(message, optional, tag = "27")]
    pub posting_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Display name of the company listing the job.
    #[prost(string, tag = "28")]
    pub company_display_name: ::prost::alloc::string::String,
    /// Output only. Derived details about the job posting.
    #[prost(message, optional, tag = "29")]
    pub derived_info: ::core::option::Option<job::DerivedInfo>,
    /// Options for job processing.
    #[prost(message, optional, tag = "30")]
    pub processing_options: ::core::option::Option<job::ProcessingOptions>,
}
/// Nested message and enum types in `Job`.
pub mod job {
    /// Application related details of a job posting.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApplicationInfo {
        /// Use this field to specify email address(es) to which resumes or
        /// applications can be sent.
        ///
        /// The maximum number of allowed characters for each entry is 255.
        #[prost(string, repeated, tag = "1")]
        pub emails: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Use this field to provide instructions, such as "Mail your application
        /// to ...", that a candidate can follow to apply for the job.
        ///
        /// This field accepts and sanitizes HTML input, and also accepts
        /// bold, italic, ordered list, and unordered list markup tags.
        ///
        /// The maximum number of allowed characters is 3,000.
        #[prost(string, tag = "2")]
        pub instruction: ::prost::alloc::string::String,
        /// Use this URI field to direct an applicant to a website, for example to
        /// link to an online application form.
        ///
        /// The maximum number of allowed characters for each entry is 2,000.
        #[prost(string, repeated, tag = "3")]
        pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Derived details about the job posting.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DerivedInfo {
        /// Structured locations of the job, resolved from
        /// \[Job.addresses][google.cloud.talent.v4.Job.addresses\].
        ///
        /// \[locations][google.cloud.talent.v4.Job.DerivedInfo.locations\] are exactly
        /// matched to \[Job.addresses][google.cloud.talent.v4.Job.addresses\] in the
        /// same order.
        #[prost(message, repeated, tag = "1")]
        pub locations: ::prost::alloc::vec::Vec<super::Location>,
        /// Job categories derived from \[Job.title][google.cloud.talent.v4.Job.title\]
        /// and \[Job.description][google.cloud.talent.v4.Job.description\].
        #[prost(enumeration = "super::JobCategory", repeated, tag = "3")]
        pub job_categories: ::prost::alloc::vec::Vec<i32>,
    }
    /// Options for job processing.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProcessingOptions {
        /// If set to `true`, the service does not attempt to resolve a
        /// more precise address for the job.
        #[prost(bool, tag = "1")]
        pub disable_street_address_resolution: bool,
        /// Option for job HTML content sanitization. Applied fields are:
        ///
        /// * description
        /// * applicationInfo.instruction
        /// * incentives
        /// * qualifications
        /// * responsibilities
        ///
        /// HTML tags in these fields may be stripped if sanitiazation isn't
        /// disabled.
        ///
        /// Defaults to
        /// \[HtmlSanitization.SIMPLE_FORMATTING_ONLY][google.cloud.talent.v4.HtmlSanitization.SIMPLE_FORMATTING_ONLY\].
        #[prost(enumeration = "super::HtmlSanitization", tag = "2")]
        pub html_sanitization: i32,
    }
}
/// Create job request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateJobRequest {
    /// Required. The resource name of the tenant under which the job is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Job to be created.
    #[prost(message, optional, tag = "2")]
    pub job: ::core::option::Option<Job>,
}
/// Get job request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobRequest {
    /// Required. The resource name of the job to retrieve.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}". For
    /// example, "projects/foo/tenants/bar/jobs/baz".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Update job request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateJobRequest {
    /// Required. The Job to be updated.
    #[prost(message, optional, tag = "1")]
    pub job: ::core::option::Option<Job>,
    /// Strongly recommended for the best service experience.
    ///
    /// If \[update_mask][google.cloud.talent.v4.UpdateJobRequest.update_mask\] is
    /// provided, only the specified fields in
    /// \[job][google.cloud.talent.v4.UpdateJobRequest.job\] are updated. Otherwise
    /// all the fields are updated.
    ///
    /// A field mask to restrict the fields that are updated. Only
    /// top level fields of \[Job][google.cloud.talent.v4.Job\] are supported.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Delete job request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteJobRequest {
    /// Required. The resource name of the job to be deleted.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}". For
    /// example, "projects/foo/tenants/bar/jobs/baz".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// List jobs request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsRequest {
    /// Required. The resource name of the tenant under which the job is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The filter string specifies the jobs to be enumerated.
    ///
    /// Supported operator: =, AND
    ///
    /// The fields eligible for filtering are:
    ///
    /// * `companyName`
    /// * `requisitionId`
    /// * `status` Available values: OPEN, EXPIRED, ALL. Defaults to
    /// OPEN if no value is specified.
    ///
    /// At least one of `companyName` and `requisitionId` must present or an
    /// INVALID_ARGUMENT error is thrown.
    ///
    /// Sample Query:
    ///
    /// * companyName = "projects/foo/tenants/bar/companies/baz"
    /// * companyName = "projects/foo/tenants/bar/companies/baz" AND
    /// requisitionId = "req-1"
    /// * companyName = "projects/foo/tenants/bar/companies/baz" AND
    /// status = "EXPIRED"
    /// * requisitionId = "req-1"
    /// * requisitionId = "req-1" AND status = "EXPIRED"
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The starting point of a query result.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of jobs to be returned per page of results.
    ///
    /// If \[job_view][google.cloud.talent.v4.ListJobsRequest.job_view\] is set to
    /// \[JobView.JOB_VIEW_ID_ONLY][google.cloud.talent.v4.JobView.JOB_VIEW_ID_ONLY\],
    /// the maximum allowed page size is 1000. Otherwise, the maximum allowed page
    /// size is 100.
    ///
    /// Default is 100 if empty or a number < 1 is specified.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// The desired job attributes returned for jobs in the
    /// search response. Defaults to
    /// \[JobView.JOB_VIEW_FULL][google.cloud.talent.v4.JobView.JOB_VIEW_FULL\] if no
    /// value is specified.
    #[prost(enumeration = "JobView", tag = "5")]
    pub job_view: i32,
}
/// List jobs response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsResponse {
    /// The Jobs for a given company.
    ///
    /// The maximum number of items returned is based on the limit field
    /// provided in the request.
    #[prost(message, repeated, tag = "1")]
    pub jobs: ::prost::alloc::vec::Vec<Job>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
/// The Request body of the `SearchJobs` call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchJobsRequest {
    /// Required. The resource name of the tenant to search within.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Mode of a search.
    ///
    /// Defaults to
    /// \[SearchMode.JOB_SEARCH][google.cloud.talent.v4.SearchJobsRequest.SearchMode.JOB_SEARCH\].
    #[prost(enumeration = "search_jobs_request::SearchMode", tag = "2")]
    pub search_mode: i32,
    /// Required. The meta information collected about the job searcher, used to
    /// improve the search quality of the service. The identifiers (such as
    /// `user_id`) are provided by users, and must be unique and consistent.
    #[prost(message, optional, tag = "3")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    /// Query used to search against jobs, such as keyword, location filters, etc.
    #[prost(message, optional, tag = "4")]
    pub job_query: ::core::option::Option<JobQuery>,
    /// Controls whether to broaden the search when it produces sparse results.
    /// Broadened queries append results to the end of the matching results
    /// list.
    ///
    /// Defaults to false.
    #[prost(bool, tag = "5")]
    pub enable_broadening: bool,
    /// An expression specifies a histogram request against matching jobs.
    ///
    /// Expression syntax is an aggregation function call with histogram facets and
    /// other options.
    ///
    /// Available aggregation function calls are:
    /// * `count(string_histogram_facet)`: Count the number of matching entities,
    /// for each distinct attribute value.
    /// * `count(numeric_histogram_facet, list of buckets)`: Count the number of
    /// matching entities within each bucket.
    ///
    /// A maximum of 200 histogram buckets are supported.
    ///
    /// Data types:
    ///
    /// * Histogram facet: facet names with format `\[a-zA-Z][a-zA-Z0-9_\]+`.
    /// * String: string like "any string with backslash escape for quote(\")."
    /// * Number: whole number and floating point number like 10, -1 and -0.01.
    /// * List: list of elements with comma(,) separator surrounded by square
    /// brackets, for example, [1, 2, 3] and ["one", "two", "three"].
    ///
    /// Built-in constants:
    ///
    /// * MIN (minimum number similar to java Double.MIN_VALUE)
    /// * MAX (maximum number similar to java Double.MAX_VALUE)
    ///
    /// Built-in functions:
    ///
    /// * bucket(start, end[, label]): bucket built-in function creates a bucket
    /// with range of [start, end). Note that the end is exclusive, for example,
    /// bucket(1, MAX, "positive number") or bucket(1, 10).
    ///
    /// Job histogram facets:
    ///
    /// * company_display_name: histogram by
    /// \[Job.company_display_name][google.cloud.talent.v4.Job.company_display_name\].
    /// * employment_type: histogram by
    /// \[Job.employment_types][google.cloud.talent.v4.Job.employment_types\], for
    /// example,
    ///    "FULL_TIME", "PART_TIME".
    /// * company_size (DEPRECATED): histogram by
    /// \[CompanySize][google.cloud.talent.v4.CompanySize\], for example, "SMALL",
    /// "MEDIUM", "BIG".
    /// * publish_time_in_day: histogram by the
    /// \[Job.posting_publish_time][google.cloud.talent.v4.Job.posting_publish_time\]
    ///    in days.
    ///    Must specify list of numeric buckets in spec.
    /// * publish_time_in_month: histogram by the
    /// \[Job.posting_publish_time][google.cloud.talent.v4.Job.posting_publish_time\]
    ///    in months.
    ///    Must specify list of numeric buckets in spec.
    /// * publish_time_in_year: histogram by the
    /// \[Job.posting_publish_time][google.cloud.talent.v4.Job.posting_publish_time\]
    ///    in years.
    ///    Must specify list of numeric buckets in spec.
    /// * degree_types: histogram by the
    /// \[Job.degree_types][google.cloud.talent.v4.Job.degree_types\], for example,
    ///    "Bachelors", "Masters".
    /// * job_level: histogram by the
    /// \[Job.job_level][google.cloud.talent.v4.Job.job_level\], for example, "Entry
    ///    Level".
    /// * country: histogram by the country code of jobs, for example, "US", "FR".
    /// * admin1: histogram by the admin1 code of jobs, which is a global
    ///    placeholder referring to the state, province, or the particular term a
    ///    country uses to define the geographic structure below the country level,
    ///    for example, "CA", "IL".
    /// * city: histogram by a combination of the "city name, admin1 code". For
    ///    example,  "Mountain View, CA", "New York, NY".
    /// * admin1_country: histogram by a combination of the "admin1 code, country",
    ///    for example, "CA, US", "IL, US".
    /// * city_coordinate: histogram by the city center's GPS coordinates (latitude
    ///    and longitude), for example, 37.4038522,-122.0987765. Since the
    ///    coordinates of a city center can change, customers may need to refresh
    ///    them periodically.
    /// * locale: histogram by the
    /// \[Job.language_code][google.cloud.talent.v4.Job.language_code\], for example,
    /// "en-US",
    ///    "fr-FR".
    /// * language: histogram by the language subtag of the
    /// \[Job.language_code][google.cloud.talent.v4.Job.language_code\],
    ///    for example, "en", "fr".
    /// * category: histogram by the
    /// \[JobCategory][google.cloud.talent.v4.JobCategory\], for example,
    ///    "COMPUTER_AND_IT", "HEALTHCARE".
    /// * base_compensation_unit: histogram by the
    ///    \[CompensationInfo.CompensationUnit][google.cloud.talent.v4.CompensationInfo.CompensationUnit\]
    ///    of base salary, for example, "WEEKLY", "MONTHLY".
    /// * base_compensation: histogram by the base salary. Must specify list of
    ///    numeric buckets to group results by.
    /// * annualized_base_compensation: histogram by the base annualized salary.
    ///    Must specify list of numeric buckets to group results by.
    /// * annualized_total_compensation: histogram by the total annualized salary.
    ///    Must specify list of numeric buckets to group results by.
    /// * string_custom_attribute: histogram by string
    /// \[Job.custom_attributes][google.cloud.talent.v4.Job.custom_attributes\].
    ///    Values can be accessed via square bracket notations like
    ///    string_custom_attribute\["key1"\].
    /// * numeric_custom_attribute: histogram by numeric
    /// \[Job.custom_attributes][google.cloud.talent.v4.Job.custom_attributes\].
    ///    Values can be accessed via square bracket notations like
    ///    numeric_custom_attribute\["key1"\]. Must specify list of numeric buckets to
    ///    group results by.
    ///
    /// Example expressions:
    ///
    /// * `count(admin1)`
    /// * `count(base_compensation, [bucket(1000, 10000), bucket(10000, 100000),
    /// bucket(100000, MAX)])`
    /// * `count(string_custom_attribute\["some-string-custom-attribute"\])`
    /// * `count(numeric_custom_attribute\["some-numeric-custom-attribute"\],
    ///    [bucket(MIN, 0, "negative"), bucket(0, MAX, "non-negative")])`
    #[prost(message, repeated, tag = "7")]
    pub histogram_queries: ::prost::alloc::vec::Vec<HistogramQuery>,
    /// The desired job attributes returned for jobs in the search response.
    /// Defaults to
    /// \[JobView.JOB_VIEW_SMALL][google.cloud.talent.v4.JobView.JOB_VIEW_SMALL\] if
    /// no value is specified.
    #[prost(enumeration = "JobView", tag = "8")]
    pub job_view: i32,
    /// An integer that specifies the current offset (that is, starting result
    /// location, amongst the jobs deemed by the API as relevant) in search
    /// results. This field is only considered if
    /// \[page_token][google.cloud.talent.v4.SearchJobsRequest.page_token\] is unset.
    ///
    /// The maximum allowed value is 5000. Otherwise an error is thrown.
    ///
    /// For example, 0 means to  return results starting from the first matching
    /// job, and 10 means to return from the 11th job. This can be used for
    /// pagination, (for example, pageSize = 10 and offset = 10 means to return
    /// from the second page).
    #[prost(int32, tag = "9")]
    pub offset: i32,
    /// A limit on the number of jobs returned in the search results.
    /// Increasing this value above the default value of 10 can increase search
    /// response time. The value can be between 1 and 100.
    #[prost(int32, tag = "10")]
    pub max_page_size: i32,
    /// The token specifying the current offset within
    /// search results. See
    /// \[SearchJobsResponse.next_page_token][google.cloud.talent.v4.SearchJobsResponse.next_page_token\]
    /// for an explanation of how to obtain the next set of query results.
    #[prost(string, tag = "11")]
    pub page_token: ::prost::alloc::string::String,
    /// The criteria determining how search results are sorted. Default is
    /// `"relevance desc"`.
    ///
    /// Supported options are:
    ///
    /// * `"relevance desc"`: By relevance descending, as determined by the API
    ///    algorithms. Relevance thresholding of query results is only available
    ///    with this ordering.
    /// * `"posting_publish_time desc"`: By
    /// \[Job.posting_publish_time][google.cloud.talent.v4.Job.posting_publish_time\]
    ///    descending.
    /// * `"posting_update_time desc"`: By
    /// \[Job.posting_update_time][google.cloud.talent.v4.Job.posting_update_time\]
    ///    descending.
    /// * `"title"`: By \[Job.title][google.cloud.talent.v4.Job.title\] ascending.
    /// * `"title desc"`: By \[Job.title][google.cloud.talent.v4.Job.title\]
    /// descending.
    /// * `"annualized_base_compensation"`: By job's
    ///    \[CompensationInfo.annualized_base_compensation_range][google.cloud.talent.v4.CompensationInfo.annualized_base_compensation_range\]
    ///    ascending. Jobs whose annualized base compensation is unspecified are put
    ///    at the end of search results.
    /// * `"annualized_base_compensation desc"`: By job's
    ///    \[CompensationInfo.annualized_base_compensation_range][google.cloud.talent.v4.CompensationInfo.annualized_base_compensation_range\]
    ///    descending. Jobs whose annualized base compensation is unspecified are
    ///    put at the end of search results.
    /// * `"annualized_total_compensation"`: By job's
    ///    \[CompensationInfo.annualized_total_compensation_range][google.cloud.talent.v4.CompensationInfo.annualized_total_compensation_range\]
    ///    ascending. Jobs whose annualized base compensation is unspecified are put
    ///    at the end of search results.
    /// * `"annualized_total_compensation desc"`: By job's
    ///    \[CompensationInfo.annualized_total_compensation_range][google.cloud.talent.v4.CompensationInfo.annualized_total_compensation_range\]
    ///    descending. Jobs whose annualized base compensation is unspecified are
    ///    put at the end of search results.
    /// * `"custom_ranking desc"`: By the relevance score adjusted to the
    ///    \[SearchJobsRequest.CustomRankingInfo.ranking_expression][google.cloud.talent.v4.SearchJobsRequest.CustomRankingInfo.ranking_expression\]
    ///    with weight factor assigned by
    ///    \[SearchJobsRequest.CustomRankingInfo.importance_level][google.cloud.talent.v4.SearchJobsRequest.CustomRankingInfo.importance_level\]
    ///    in descending order.
    /// * Location sorting: Use the special syntax to order jobs by distance:<br>
    ///    `"distance_from('Hawaii')"`: Order by distance from Hawaii.<br>
    ///    `"distance_from(19.89, 155.5)"`: Order by distance from a coordinate.<br>
    ///    `"distance_from('Hawaii'), distance_from('Puerto Rico')"`: Order by
    ///    multiple locations. See details below.<br>
    ///    `"distance_from('Hawaii'), distance_from(19.89, 155.5)"`: Order by
    ///    multiple locations. See details below.<br>
    ///    The string can have a maximum of 256 characters. When multiple distance
    ///    centers are provided, a job that is close to any of the distance centers
    ///    would have a high rank. When a job has multiple locations, the job
    ///    location closest to one of the distance centers will be used. Jobs that
    ///    don't have locations will be ranked at the bottom. Distance is calculated
    ///    with a precision of 11.3 meters (37.4 feet). Diversification strategy is
    ///    still applied unless explicitly disabled in
    ///    \[diversification_level][google.cloud.talent.v4.SearchJobsRequest.diversification_level\].
    #[prost(string, tag = "12")]
    pub order_by: ::prost::alloc::string::String,
    /// Controls whether highly similar jobs are returned next to each other in
    /// the search results. Jobs are identified as highly similar based on
    /// their titles, job categories, and locations. Highly similar results are
    /// clustered so that only one representative job of the cluster is
    /// displayed to the job seeker higher up in the results, with the other jobs
    /// being displayed lower down in the results.
    ///
    /// Defaults to
    /// \[DiversificationLevel.SIMPLE][google.cloud.talent.v4.SearchJobsRequest.DiversificationLevel.SIMPLE\]
    /// if no value is specified.
    #[prost(enumeration = "search_jobs_request::DiversificationLevel", tag = "13")]
    pub diversification_level: i32,
    /// Controls over how job documents get ranked on top of existing relevance
    /// score (determined by API algorithm).
    #[prost(message, optional, tag = "14")]
    pub custom_ranking_info: ::core::option::Option<
        search_jobs_request::CustomRankingInfo,
    >,
    /// This field is deprecated. Please use
    /// \[SearchJobsRequest.keyword_match_mode][google.cloud.talent.v4.SearchJobsRequest.keyword_match_mode\]
    /// going forward.
    ///
    /// To migrate, disable_keyword_match set to false maps to
    /// \[KeywordMatchMode.KEYWORD_MATCH_ALL][google.cloud.talent.v4.SearchJobsRequest.KeywordMatchMode.KEYWORD_MATCH_ALL\],
    /// and disable_keyword_match set to true maps to
    /// \[KeywordMatchMode.KEYWORD_MATCH_DISABLED][google.cloud.talent.v4.SearchJobsRequest.KeywordMatchMode.KEYWORD_MATCH_DISABLED\].
    /// If
    /// \[SearchJobsRequest.keyword_match_mode][google.cloud.talent.v4.SearchJobsRequest.keyword_match_mode\]
    /// is set, this field is ignored.
    ///
    /// Controls whether to disable exact keyword match on
    /// \[Job.title][google.cloud.talent.v4.Job.title\],
    /// \[Job.description][google.cloud.talent.v4.Job.description\],
    /// \[Job.company_display_name][google.cloud.talent.v4.Job.company_display_name\],
    /// \[Job.addresses][google.cloud.talent.v4.Job.addresses\],
    /// \[Job.qualifications][google.cloud.talent.v4.Job.qualifications\]. When
    /// disable keyword match is turned off, a keyword match returns jobs that do
    /// not match given category filters when there are matching keywords. For
    /// example, for the query "program manager," a result is returned even if the
    /// job posting has the title "software developer," which doesn't fall into
    /// "program manager" ontology, but does have "program manager" appearing in
    /// its description.
    ///
    /// For queries like "cloud" that don't contain title or
    /// location specific ontology, jobs with "cloud" keyword matches are returned
    /// regardless of this flag's value.
    ///
    /// Use
    /// \[Company.keyword_searchable_job_custom_attributes][google.cloud.talent.v4.Company.keyword_searchable_job_custom_attributes\]
    /// if company-specific globally matched custom field/attribute string values
    /// are needed. Enabling keyword match improves recall of subsequent search
    /// requests.
    ///
    /// Defaults to false.
    #[deprecated]
    #[prost(bool, tag = "16")]
    pub disable_keyword_match: bool,
    /// Controls what keyword match options to use. If both keyword_match_mode and
    /// disable_keyword_match are set, keyword_match_mode will take precedence.
    ///
    /// Defaults to
    /// \[KeywordMatchMode.KEYWORD_MATCH_ALL][google.cloud.talent.v4.SearchJobsRequest.KeywordMatchMode.KEYWORD_MATCH_ALL\]
    /// if no value is specified.
    #[prost(enumeration = "search_jobs_request::KeywordMatchMode", tag = "18")]
    pub keyword_match_mode: i32,
}
/// Nested message and enum types in `SearchJobsRequest`.
pub mod search_jobs_request {
    /// Custom ranking information for
    /// \[SearchJobsRequest][google.cloud.talent.v4.SearchJobsRequest\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomRankingInfo {
        /// Required. Controls over how important the score of
        /// \[CustomRankingInfo.ranking_expression][google.cloud.talent.v4.SearchJobsRequest.CustomRankingInfo.ranking_expression\]
        /// gets applied to job's final ranking position.
        ///
        /// An error is thrown if not specified.
        #[prost(enumeration = "custom_ranking_info::ImportanceLevel", tag = "1")]
        pub importance_level: i32,
        /// Required. Controls over how job documents get ranked on top of existing
        /// relevance score (determined by API algorithm). A combination of the
        /// ranking expression and relevance score is used to determine job's final
        /// ranking position.
        ///
        /// The syntax for this expression is a subset of Google SQL syntax.
        ///
        /// Supported operators are: +, -, *, /, where the left and right side of
        /// the operator is either a numeric
        /// \[Job.custom_attributes][google.cloud.talent.v4.Job.custom_attributes\]
        /// key, integer/double value or an expression that can be evaluated to a
        /// number.
        ///
        /// Parenthesis are supported to adjust calculation precedence. The
        /// expression must be < 200 characters in length.
        ///
        /// The expression is considered invalid for a job if the expression
        /// references custom attributes that are not populated on the job or if the
        /// expression results in a divide by zero. If an expression is invalid for a
        /// job, that job is demoted to the end of the results.
        ///
        /// Sample ranking expression
        /// (year + 25) * 0.25 - (freshness / 0.5)
        #[prost(string, tag = "2")]
        pub ranking_expression: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `CustomRankingInfo`.
    pub mod custom_ranking_info {
        /// The importance level for
        /// \[CustomRankingInfo.ranking_expression][google.cloud.talent.v4.SearchJobsRequest.CustomRankingInfo.ranking_expression\].
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
        pub enum ImportanceLevel {
            /// Default value if the importance level isn't specified.
            Unspecified = 0,
            /// The given ranking expression is of None importance, existing relevance
            /// score (determined by API algorithm) dominates job's final ranking
            /// position.
            None = 1,
            /// The given ranking expression is of Low importance in terms of job's
            /// final ranking position compared to existing relevance
            /// score (determined by API algorithm).
            Low = 2,
            /// The given ranking expression is of Mild importance in terms of job's
            /// final ranking position compared to existing relevance
            /// score (determined by API algorithm).
            Mild = 3,
            /// The given ranking expression is of Medium importance in terms of job's
            /// final ranking position compared to existing relevance
            /// score (determined by API algorithm).
            Medium = 4,
            /// The given ranking expression is of High importance in terms of job's
            /// final ranking position compared to existing relevance
            /// score (determined by API algorithm).
            High = 5,
            /// The given ranking expression is of Extreme importance, and dominates
            /// job's final ranking position with existing relevance
            /// score (determined by API algorithm) ignored.
            Extreme = 6,
        }
        impl ImportanceLevel {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ImportanceLevel::Unspecified => "IMPORTANCE_LEVEL_UNSPECIFIED",
                    ImportanceLevel::None => "NONE",
                    ImportanceLevel::Low => "LOW",
                    ImportanceLevel::Mild => "MILD",
                    ImportanceLevel::Medium => "MEDIUM",
                    ImportanceLevel::High => "HIGH",
                    ImportanceLevel::Extreme => "EXTREME",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "IMPORTANCE_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
                    "NONE" => Some(Self::None),
                    "LOW" => Some(Self::Low),
                    "MILD" => Some(Self::Mild),
                    "MEDIUM" => Some(Self::Medium),
                    "HIGH" => Some(Self::High),
                    "EXTREME" => Some(Self::Extreme),
                    _ => None,
                }
            }
        }
    }
    /// A string-represented enumeration of the job search mode. The service
    /// operate differently for different modes of service.
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
    pub enum SearchMode {
        /// The mode of the search method isn't specified. The default search
        /// behavior is identical to JOB_SEARCH search behavior.
        Unspecified = 0,
        /// The job search matches against all jobs, and featured jobs
        /// (jobs with promotionValue > 0) are not specially handled.
        JobSearch = 1,
        /// The job search matches only against featured jobs (jobs with a
        /// promotionValue > 0). This method doesn't return any jobs having a
        /// promotionValue <= 0. The search results order is determined by the
        /// promotionValue (jobs with a higher promotionValue are returned higher up
        /// in the search results), with relevance being used as a tiebreaker.
        FeaturedJobSearch = 2,
    }
    impl SearchMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SearchMode::Unspecified => "SEARCH_MODE_UNSPECIFIED",
                SearchMode::JobSearch => "JOB_SEARCH",
                SearchMode::FeaturedJobSearch => "FEATURED_JOB_SEARCH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEARCH_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "JOB_SEARCH" => Some(Self::JobSearch),
                "FEATURED_JOB_SEARCH" => Some(Self::FeaturedJobSearch),
                _ => None,
            }
        }
    }
    /// Controls whether highly similar jobs are returned next to each other in
    /// the search results. Jobs are identified as highly similar based on
    /// their titles, job categories, and locations. Highly similar results are
    /// clustered so that only one representative job of the cluster is
    /// displayed to the job seeker higher up in the results, with the other jobs
    /// being displayed lower down in the results.
    ///
    /// If you are using pageToken to page through the result set,
    /// latency might be lower but we can't guarantee that all results are
    /// returned. If you are using page offset, latency might be higher but all
    /// results are returned.
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
    pub enum DiversificationLevel {
        /// The diversification level isn't specified.
        Unspecified = 0,
        /// Disables diversification. Jobs that would normally be pushed to the last
        /// page would not have their positions altered. This may result in highly
        /// similar jobs appearing in sequence in the search results.
        Disabled = 1,
        /// Default diversifying behavior. The result list is ordered so that
        /// highly similar results are pushed to the end of the last page of search
        /// results.
        Simple = 2,
        /// Only one job from the same company will be shown at once, other jobs
        /// under same company are pushed to the end of the last page of search
        /// result.
        OnePerCompany = 3,
        /// Similar to ONE_PER_COMPANY, but it allows at most two jobs in the
        /// same company to be shown at once, the other jobs under same company are
        /// pushed to the end of the last page of search result.
        TwoPerCompany = 4,
        /// Similar to ONE_PER_COMPANY, but it allows at most three jobs in the
        /// same company to be shown at once, the other jobs under same company are
        /// dropped.
        MaxThreePerCompany = 6,
        /// The result list is ordered such that somewhat similar results are pushed
        /// to the end of the last page of the search results. This option is
        /// recommended if SIMPLE diversification does not diversify enough.
        DiversifyByLooserSimilarity = 5,
    }
    impl DiversificationLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DiversificationLevel::Unspecified => "DIVERSIFICATION_LEVEL_UNSPECIFIED",
                DiversificationLevel::Disabled => "DISABLED",
                DiversificationLevel::Simple => "SIMPLE",
                DiversificationLevel::OnePerCompany => "ONE_PER_COMPANY",
                DiversificationLevel::TwoPerCompany => "TWO_PER_COMPANY",
                DiversificationLevel::MaxThreePerCompany => "MAX_THREE_PER_COMPANY",
                DiversificationLevel::DiversifyByLooserSimilarity => {
                    "DIVERSIFY_BY_LOOSER_SIMILARITY"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DIVERSIFICATION_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
                "DISABLED" => Some(Self::Disabled),
                "SIMPLE" => Some(Self::Simple),
                "ONE_PER_COMPANY" => Some(Self::OnePerCompany),
                "TWO_PER_COMPANY" => Some(Self::TwoPerCompany),
                "MAX_THREE_PER_COMPANY" => Some(Self::MaxThreePerCompany),
                "DIVERSIFY_BY_LOOSER_SIMILARITY" => {
                    Some(Self::DiversifyByLooserSimilarity)
                }
                _ => None,
            }
        }
    }
    /// Controls what keyword matching behavior the search has. When keyword
    /// matching is enabled, a keyword match returns jobs that may not match given
    /// category filters when there are matching keywords. For example, for the
    /// query "program manager" with KeywordMatchMode set to KEYWORD_MATCH_ALL, a
    /// job posting with the title "software developer," which doesn't fall into
    /// "program manager" ontology, and "program manager" appearing in its
    /// description will be surfaced.
    ///
    /// For queries like "cloud" that don't contain title or
    /// location specific ontology, jobs with "cloud" keyword matches are returned
    /// regardless of this enum's value.
    ///
    /// Use
    /// \[Company.keyword_searchable_job_custom_attributes][google.cloud.talent.v4.Company.keyword_searchable_job_custom_attributes\]
    /// if company-specific globally matched custom field/attribute string values
    /// are needed. Enabling keyword match improves recall of subsequent search
    /// requests.
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
    pub enum KeywordMatchMode {
        /// The keyword match option isn't specified. Defaults to
        /// \[KeywordMatchMode.KEYWORD_MATCH_ALL][google.cloud.talent.v4.SearchJobsRequest.KeywordMatchMode.KEYWORD_MATCH_ALL\]
        /// behavior.
        Unspecified = 0,
        /// Disables keyword matching.
        KeywordMatchDisabled = 1,
        /// Enable keyword matching over
        /// \[Job.title][google.cloud.talent.v4.Job.title\],
        /// \[Job.description][google.cloud.talent.v4.Job.description\],
        /// \[Job.company_display_name][google.cloud.talent.v4.Job.company_display_name\],
        /// \[Job.addresses][google.cloud.talent.v4.Job.addresses\],
        /// \[Job.qualifications][google.cloud.talent.v4.Job.qualifications\], and
        /// keyword searchable
        /// \[Job.custom_attributes][google.cloud.talent.v4.Job.custom_attributes\]
        /// fields.
        KeywordMatchAll = 2,
        /// Only enable keyword matching over
        /// \[Job.title][google.cloud.talent.v4.Job.title\].
        KeywordMatchTitleOnly = 3,
    }
    impl KeywordMatchMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeywordMatchMode::Unspecified => "KEYWORD_MATCH_MODE_UNSPECIFIED",
                KeywordMatchMode::KeywordMatchDisabled => "KEYWORD_MATCH_DISABLED",
                KeywordMatchMode::KeywordMatchAll => "KEYWORD_MATCH_ALL",
                KeywordMatchMode::KeywordMatchTitleOnly => "KEYWORD_MATCH_TITLE_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "KEYWORD_MATCH_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "KEYWORD_MATCH_DISABLED" => Some(Self::KeywordMatchDisabled),
                "KEYWORD_MATCH_ALL" => Some(Self::KeywordMatchAll),
                "KEYWORD_MATCH_TITLE_ONLY" => Some(Self::KeywordMatchTitleOnly),
                _ => None,
            }
        }
    }
}
/// Response for SearchJob method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchJobsResponse {
    /// The Job entities that match the specified
    /// \[SearchJobsRequest][google.cloud.talent.v4.SearchJobsRequest\].
    #[prost(message, repeated, tag = "1")]
    pub matching_jobs: ::prost::alloc::vec::Vec<search_jobs_response::MatchingJob>,
    /// The histogram results that match with specified
    /// \[SearchJobsRequest.histogram_queries][google.cloud.talent.v4.SearchJobsRequest.histogram_queries\].
    #[prost(message, repeated, tag = "2")]
    pub histogram_query_results: ::prost::alloc::vec::Vec<HistogramQueryResult>,
    /// The token that specifies the starting position of the next page of results.
    /// This field is empty if there are no more results.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The location filters that the service applied to the specified query. If
    /// any filters are lat-lng based, the
    /// \[Location.location_type][google.cloud.talent.v4.Location.location_type\] is
    /// \[Location.LocationType.LOCATION_TYPE_UNSPECIFIED][google.cloud.talent.v4.Location.LocationType.LOCATION_TYPE_UNSPECIFIED\].
    #[prost(message, repeated, tag = "4")]
    pub location_filters: ::prost::alloc::vec::Vec<Location>,
    /// Number of jobs that match the specified query.
    ///
    /// Note: This size is precise only if the total is less than 100,000.
    #[prost(int32, tag = "6")]
    pub total_size: i32,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    #[prost(message, optional, tag = "7")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
    /// If query broadening is enabled, we may append additional results from the
    /// broadened query. This number indicates how many of the jobs returned in the
    /// jobs field are from the broadened query. These results are always at the
    /// end of the jobs list. In particular, a value of 0, or if the field isn't
    /// set, all the jobs in the jobs list are from the original
    /// (without broadening) query. If this field is non-zero, subsequent requests
    /// with offset after this result set should contain all broadened results.
    #[prost(int32, tag = "8")]
    pub broadened_query_jobs_count: i32,
    /// The spell checking result, and correction.
    #[prost(message, optional, tag = "9")]
    pub spell_correction: ::core::option::Option<SpellingCorrection>,
}
/// Nested message and enum types in `SearchJobsResponse`.
pub mod search_jobs_response {
    /// Job entry with metadata inside
    /// \[SearchJobsResponse][google.cloud.talent.v4.SearchJobsResponse\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchingJob {
        /// Job resource that matches the specified
        /// \[SearchJobsRequest][google.cloud.talent.v4.SearchJobsRequest\].
        #[prost(message, optional, tag = "1")]
        pub job: ::core::option::Option<super::Job>,
        /// A summary of the job with core information that's displayed on the search
        /// results listing page.
        #[prost(string, tag = "2")]
        pub job_summary: ::prost::alloc::string::String,
        /// Contains snippets of text from the
        /// \[Job.title][google.cloud.talent.v4.Job.title\] field most closely matching
        /// a search query's keywords, if available. The matching query keywords are
        /// enclosed in HTML bold tags.
        #[prost(string, tag = "3")]
        pub job_title_snippet: ::prost::alloc::string::String,
        /// Contains snippets of text from the
        /// \[Job.description][google.cloud.talent.v4.Job.description\] and similar
        /// fields that most closely match a search query's keywords, if available.
        /// All HTML tags in the original fields are stripped when returned in this
        /// field, and matching query keywords are enclosed in HTML bold tags.
        #[prost(string, tag = "4")]
        pub search_text_snippet: ::prost::alloc::string::String,
        /// Commute information which is generated based on specified
        ///   \[CommuteFilter][google.cloud.talent.v4.CommuteFilter\].
        #[prost(message, optional, tag = "5")]
        pub commute_info: ::core::option::Option<CommuteInfo>,
    }
    /// Commute details related to this job.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CommuteInfo {
        /// Location used as the destination in the commute calculation.
        #[prost(message, optional, tag = "1")]
        pub job_location: ::core::option::Option<super::Location>,
        /// The number of seconds required to travel to the job location from the
        /// query location. A duration of 0 seconds indicates that the job isn't
        /// reachable within the requested duration, but was returned as part of an
        /// expanded query.
        #[prost(message, optional, tag = "2")]
        pub travel_duration: ::core::option::Option<::prost_types::Duration>,
    }
}
/// Request to create a batch of jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateJobsRequest {
    /// Required. The resource name of the tenant under which the job is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The jobs to be created.
    /// A maximum of 200 jobs can be created in a batch.
    #[prost(message, repeated, tag = "2")]
    pub jobs: ::prost::alloc::vec::Vec<Job>,
}
/// Request to update a batch of jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateJobsRequest {
    /// Required. The resource name of the tenant under which the job is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The jobs to be updated.
    /// A maximum of 200 jobs can be updated in a batch.
    #[prost(message, repeated, tag = "2")]
    pub jobs: ::prost::alloc::vec::Vec<Job>,
    /// Strongly recommended for the best service experience. Be aware that it will
    /// also increase latency when checking the status of a batch operation.
    ///
    /// If \[update_mask][google.cloud.talent.v4.BatchUpdateJobsRequest.update_mask\]
    /// is provided, only the specified fields in \[Job][google.cloud.talent.v4.Job\]
    /// are updated. Otherwise all the fields are updated.
    ///
    /// A field mask to restrict the fields that are updated. Only
    /// top level fields of \[Job][google.cloud.talent.v4.Job\] are supported.
    ///
    /// If \[update_mask][google.cloud.talent.v4.BatchUpdateJobsRequest.update_mask\]
    /// is provided, The \[Job][google.cloud.talent.v4.Job\] inside
    /// \[JobResult][JobOperationResult.JobResult\]
    /// will only contains fields that is updated, plus the Id of the Job.
    /// Otherwise,  \[Job][google.cloud.talent.v4.Job\] will include all fields,
    /// which can yield a very large response.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to delete a batch of jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteJobsRequest {
    /// Required. The resource name of the tenant under which the job is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenants/bar".
    ///
    /// The parent of all of the jobs specified in `names` must match this field.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The names of the jobs to delete.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}".
    /// For example, "projects/foo/tenants/bar/jobs/baz".
    ///
    /// A maximum of 200 jobs can be deleted in a batch.
    #[prost(string, repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Mutation result of a job from a batch operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobResult {
    /// Here \[Job][google.cloud.talent.v4.Job\] only contains basic information
    /// including \[name][google.cloud.talent.v4.Job.name\],
    /// \[company][google.cloud.talent.v4.Job.company\],
    /// \[language_code][google.cloud.talent.v4.Job.language_code\] and
    /// \[requisition_id][google.cloud.talent.v4.Job.requisition_id\], use getJob
    /// method to retrieve detailed information of the created/updated job.
    #[prost(message, optional, tag = "1")]
    pub job: ::core::option::Option<Job>,
    /// The status of the job processed. This field is populated if the
    /// processing of the \[job][google.cloud.talent.v4.JobResult.job\] fails.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
}
/// The result of
/// \[JobService.BatchCreateJobs][google.cloud.talent.v4.JobService.BatchCreateJobs\].
/// It's used to replace
/// \[google.longrunning.Operation.response][google.longrunning.Operation.response\]
/// in case of success.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateJobsResponse {
    /// List of job mutation results from a batch create operation. It can change
    /// until operation status is FINISHED, FAILED or CANCELLED.
    #[prost(message, repeated, tag = "1")]
    pub job_results: ::prost::alloc::vec::Vec<JobResult>,
}
/// The result of
/// \[JobService.BatchUpdateJobs][google.cloud.talent.v4.JobService.BatchUpdateJobs\].
/// It's used to replace
/// \[google.longrunning.Operation.response][google.longrunning.Operation.response\]
/// in case of success.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateJobsResponse {
    /// List of job mutation results from a batch update operation. It can change
    /// until operation status is FINISHED, FAILED or CANCELLED.
    #[prost(message, repeated, tag = "1")]
    pub job_results: ::prost::alloc::vec::Vec<JobResult>,
}
/// The result of
/// \[JobService.BatchDeleteJobs][google.cloud.talent.v4.JobService.BatchDeleteJobs\].
/// It's used to replace
/// \[google.longrunning.Operation.response][google.longrunning.Operation.response\]
/// in case of success.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteJobsResponse {
    /// List of job mutation results from a batch delete operation. It can change
    /// until operation status is FINISHED, FAILED or CANCELLED.
    #[prost(message, repeated, tag = "1")]
    pub job_results: ::prost::alloc::vec::Vec<JobResult>,
}
/// An enum that specifies the job attributes that are returned in the
/// \[MatchingJob.job][google.cloud.talent.v4.SearchJobsResponse.MatchingJob.job\]
/// or \[ListJobsResponse.jobs][google.cloud.talent.v4.ListJobsResponse.jobs\]
/// fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobView {
    /// Default value.
    Unspecified = 0,
    /// A ID only view of job, with following attributes:
    /// \[Job.name][google.cloud.talent.v4.Job.name\],
    /// \[Job.requisition_id][google.cloud.talent.v4.Job.requisition_id\],
    /// \[Job.language_code][google.cloud.talent.v4.Job.language_code\].
    IdOnly = 1,
    /// A minimal view of the job, with the following attributes:
    /// \[Job.name][google.cloud.talent.v4.Job.name\],
    /// \[Job.requisition_id][google.cloud.talent.v4.Job.requisition_id\],
    /// \[Job.title][google.cloud.talent.v4.Job.title\],
    /// \[Job.company][google.cloud.talent.v4.Job.company\],
    /// \[Job.DerivedInfo.locations][google.cloud.talent.v4.Job.DerivedInfo.locations\],
    /// \[Job.language_code][google.cloud.talent.v4.Job.language_code\].
    Minimal = 2,
    /// A small view of the job, with the following attributes in the search
    /// results: \[Job.name][google.cloud.talent.v4.Job.name\],
    /// \[Job.requisition_id][google.cloud.talent.v4.Job.requisition_id\],
    /// \[Job.title][google.cloud.talent.v4.Job.title\],
    /// \[Job.company][google.cloud.talent.v4.Job.company\],
    /// \[Job.DerivedInfo.locations][google.cloud.talent.v4.Job.DerivedInfo.locations\],
    /// \[Job.visibility][google.cloud.talent.v4.Job.visibility\],
    /// \[Job.language_code][google.cloud.talent.v4.Job.language_code\],
    /// \[Job.description][google.cloud.talent.v4.Job.description\].
    Small = 3,
    /// All available attributes are included in the search results.
    Full = 4,
}
impl JobView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JobView::Unspecified => "JOB_VIEW_UNSPECIFIED",
            JobView::IdOnly => "JOB_VIEW_ID_ONLY",
            JobView::Minimal => "JOB_VIEW_MINIMAL",
            JobView::Small => "JOB_VIEW_SMALL",
            JobView::Full => "JOB_VIEW_FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "JOB_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "JOB_VIEW_ID_ONLY" => Some(Self::IdOnly),
            "JOB_VIEW_MINIMAL" => Some(Self::Minimal),
            "JOB_VIEW_SMALL" => Some(Self::Small),
            "JOB_VIEW_FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod job_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service handles job management, including job CRUD, enumeration and search.
    #[derive(Debug, Clone)]
    pub struct JobServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> JobServiceClient<T>
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
        ) -> JobServiceClient<InterceptedService<T, F>>
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
            JobServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new job.
        ///
        /// Typically, the job becomes searchable within 10 seconds, but it may take
        /// up to 5 minutes.
        pub async fn create_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
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
                "/google.cloud.talent.v4.JobService/CreateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Begins executing a batch create jobs operation.
        pub async fn batch_create_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateJobsRequest>,
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
                "/google.cloud.talent.v4.JobService/BatchCreateJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves the specified job, whose status is OPEN or recently EXPIRED
        /// within the last 90 days.
        pub async fn get_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
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
                "/google.cloud.talent.v4.JobService/GetJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates specified job.
        ///
        /// Typically, updated contents become visible in search results within 10
        /// seconds, but it may take up to 5 minutes.
        pub async fn update_job(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
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
                "/google.cloud.talent.v4.JobService/UpdateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Begins executing a batch update jobs operation.
        pub async fn batch_update_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateJobsRequest>,
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
                "/google.cloud.talent.v4.JobService/BatchUpdateJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the specified job.
        ///
        /// Typically, the job becomes unsearchable within 10 seconds, but it may take
        /// up to 5 minutes.
        pub async fn delete_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.talent.v4.JobService/DeleteJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Begins executing a batch delete jobs operation.
        pub async fn batch_delete_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteJobsRequest>,
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
                "/google.cloud.talent.v4.JobService/BatchDeleteJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists jobs by filter.
        pub async fn list_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobsRequest>,
        ) -> Result<tonic::Response<super::ListJobsResponse>, tonic::Status> {
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
                "/google.cloud.talent.v4.JobService/ListJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Searches for jobs using the provided
        /// [SearchJobsRequest][google.cloud.talent.v4.SearchJobsRequest].
        ///
        /// This call constrains the
        /// [visibility][google.cloud.talent.v4.Job.visibility] of jobs present in the
        /// database, and only returns jobs that the caller has permission to search
        /// against.
        pub async fn search_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchJobsRequest>,
        ) -> Result<tonic::Response<super::SearchJobsResponse>, tonic::Status> {
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
                "/google.cloud.talent.v4.JobService/SearchJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Searches for jobs using the provided
        /// [SearchJobsRequest][google.cloud.talent.v4.SearchJobsRequest].
        ///
        /// This API call is intended for the use case of targeting passive job
        /// seekers (for example, job seekers who have signed up to receive email
        /// alerts about potential job opportunities), it has different algorithmic
        /// adjustments that are designed to specifically target passive job seekers.
        ///
        /// This call constrains the
        /// [visibility][google.cloud.talent.v4.Job.visibility] of jobs present in the
        /// database, and only returns jobs the caller has permission to search
        /// against.
        pub async fn search_jobs_for_alert(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchJobsRequest>,
        ) -> Result<tonic::Response<super::SearchJobsResponse>, tonic::Status> {
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
                "/google.cloud.talent.v4.JobService/SearchJobsForAlert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Auto-complete parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteQueryRequest {
    /// Required. Resource name of tenant the completion is performed within.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub tenant: ::prost::alloc::string::String,
    /// Required. The query used to generate suggestions.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// The list of languages of the query. This is
    /// the BCP-47 language code, such as "en-US" or "sr-Latn".
    /// For more information, see
    /// [Tags for Identifying Languages](<https://tools.ietf.org/html/bcp47>).
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, repeated, tag = "3")]
    pub language_codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Completion result count.
    ///
    /// The maximum allowed page size is 10.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// If provided, restricts completion to specified company.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}", for
    /// example, "projects/foo/tenants/bar/companies/baz".
    #[prost(string, tag = "5")]
    pub company: ::prost::alloc::string::String,
    /// The scope of the completion. The defaults is
    /// \[CompletionScope.PUBLIC][google.cloud.talent.v4.CompleteQueryRequest.CompletionScope.PUBLIC\].
    #[prost(enumeration = "complete_query_request::CompletionScope", tag = "6")]
    pub scope: i32,
    /// The completion topic. The default is
    /// \[CompletionType.COMBINED][google.cloud.talent.v4.CompleteQueryRequest.CompletionType.COMBINED\].
    #[prost(enumeration = "complete_query_request::CompletionType", tag = "7")]
    pub r#type: i32,
}
/// Nested message and enum types in `CompleteQueryRequest`.
pub mod complete_query_request {
    /// Enum to specify the scope of completion.
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
    pub enum CompletionScope {
        /// Default value.
        Unspecified = 0,
        /// Suggestions are based only on the data provided by the client.
        Tenant = 1,
        /// Suggestions are based on all jobs data in the system that's visible to
        /// the client
        Public = 2,
    }
    impl CompletionScope {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CompletionScope::Unspecified => "COMPLETION_SCOPE_UNSPECIFIED",
                CompletionScope::Tenant => "TENANT",
                CompletionScope::Public => "PUBLIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPLETION_SCOPE_UNSPECIFIED" => Some(Self::Unspecified),
                "TENANT" => Some(Self::Tenant),
                "PUBLIC" => Some(Self::Public),
                _ => None,
            }
        }
    }
    /// Enum to specify auto-completion topics.
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
    pub enum CompletionType {
        /// Default value.
        Unspecified = 0,
        /// Suggest job titles for jobs autocomplete.
        ///
        /// For
        /// \[CompletionType.JOB_TITLE][google.cloud.talent.v4.CompleteQueryRequest.CompletionType.JOB_TITLE\]
        /// type, only open jobs with the same
        /// \[language_codes][google.cloud.talent.v4.CompleteQueryRequest.language_codes\]
        /// are returned.
        JobTitle = 1,
        /// Suggest company names for jobs autocomplete.
        ///
        /// For
        /// \[CompletionType.COMPANY_NAME][google.cloud.talent.v4.CompleteQueryRequest.CompletionType.COMPANY_NAME\]
        /// type, only companies having open jobs with the same
        /// \[language_codes][google.cloud.talent.v4.CompleteQueryRequest.language_codes\]
        /// are returned.
        CompanyName = 2,
        /// Suggest both job titles and company names for jobs autocomplete.
        ///
        /// For
        /// \[CompletionType.COMBINED][google.cloud.talent.v4.CompleteQueryRequest.CompletionType.COMBINED\]
        /// type, only open jobs with the same
        /// \[language_codes][google.cloud.talent.v4.CompleteQueryRequest.language_codes\]
        /// or companies having open jobs with the same
        /// \[language_codes][google.cloud.talent.v4.CompleteQueryRequest.language_codes\]
        /// are returned.
        Combined = 3,
    }
    impl CompletionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CompletionType::Unspecified => "COMPLETION_TYPE_UNSPECIFIED",
                CompletionType::JobTitle => "JOB_TITLE",
                CompletionType::CompanyName => "COMPANY_NAME",
                CompletionType::Combined => "COMBINED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPLETION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "JOB_TITLE" => Some(Self::JobTitle),
                "COMPANY_NAME" => Some(Self::CompanyName),
                "COMBINED" => Some(Self::Combined),
                _ => None,
            }
        }
    }
}
/// Response of auto-complete query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteQueryResponse {
    /// Results of the matching job/company candidates.
    #[prost(message, repeated, tag = "1")]
    pub completion_results: ::prost::alloc::vec::Vec<
        complete_query_response::CompletionResult,
    >,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
/// Nested message and enum types in `CompleteQueryResponse`.
pub mod complete_query_response {
    /// Resource that represents completion results.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompletionResult {
        /// The suggestion for the query.
        #[prost(string, tag = "1")]
        pub suggestion: ::prost::alloc::string::String,
        /// The completion topic.
        #[prost(
            enumeration = "super::complete_query_request::CompletionType",
            tag = "2"
        )]
        pub r#type: i32,
        /// The URI of the company image for
        /// \[COMPANY_NAME][google.cloud.talent.v4.CompleteQueryRequest.CompletionType.COMPANY_NAME\].
        #[prost(string, tag = "3")]
        pub image_uri: ::prost::alloc::string::String,
    }
}
/// Generated client implementations.
pub mod completion_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service handles auto completion.
    #[derive(Debug, Clone)]
    pub struct CompletionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CompletionClient<T>
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
        ) -> CompletionClient<InterceptedService<T, F>>
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
            CompletionClient::new(InterceptedService::new(inner, interceptor))
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
        /// Completes the specified prefix with keyword suggestions.
        /// Intended for use by a job search auto-complete search box.
        pub async fn complete_query(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteQueryRequest>,
        ) -> Result<tonic::Response<super::CompleteQueryResponse>, tonic::Status> {
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
                "/google.cloud.talent.v4.Completion/CompleteQuery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
