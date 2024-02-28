/// USPS representation of a US address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UspsAddress {
    /// First address line.
    #[prost(string, tag = "1")]
    pub first_address_line: ::prost::alloc::string::String,
    /// Firm name.
    #[prost(string, tag = "2")]
    pub firm: ::prost::alloc::string::String,
    /// Second address line.
    #[prost(string, tag = "3")]
    pub second_address_line: ::prost::alloc::string::String,
    /// Puerto Rican urbanization name.
    #[prost(string, tag = "4")]
    pub urbanization: ::prost::alloc::string::String,
    /// City + state + postal code.
    #[prost(string, tag = "5")]
    pub city_state_zip_address_line: ::prost::alloc::string::String,
    /// City name.
    #[prost(string, tag = "6")]
    pub city: ::prost::alloc::string::String,
    /// 2 letter state code.
    #[prost(string, tag = "7")]
    pub state: ::prost::alloc::string::String,
    /// Postal code e.g. 10009.
    #[prost(string, tag = "8")]
    pub zip_code: ::prost::alloc::string::String,
    /// 4-digit postal code extension e.g. 5023.
    #[prost(string, tag = "9")]
    pub zip_code_extension: ::prost::alloc::string::String,
}
/// The USPS data for the address. `uspsData` is not guaranteed to be fully
/// populated for every US or PR address sent to the Address Validation API. It's
/// recommended to integrate the backup address fields in the response if you
/// utilize uspsData as the primary part of the response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UspsData {
    /// USPS standardized address.
    #[prost(message, optional, tag = "1")]
    pub standardized_address: ::core::option::Option<UspsAddress>,
    /// 2 digit delivery point code
    #[prost(string, tag = "2")]
    pub delivery_point_code: ::prost::alloc::string::String,
    /// The delivery point check digit. This number is added to the end of the
    /// delivery_point_barcode for mechanically scanned mail. Adding all the
    /// digits of the delivery_point_barcode, delivery_point_check_digit, postal
    /// code, and ZIP+4 together should yield a number divisible by 10.
    #[prost(string, tag = "3")]
    pub delivery_point_check_digit: ::prost::alloc::string::String,
    /// The possible values for DPV confirmation. Returns a single character or
    /// returns no value.
    ///
    /// * `N`: Primary and any secondary number information failed to
    /// DPV confirm.
    /// * `D`: Address was DPV confirmed for the primary number only, and the
    /// secondary number information was missing.
    /// * `S`: Address was DPV confirmed for the primary number only, and the
    /// secondary number information was present but not confirmed.
    /// * `Y`: Address was DPV confirmed for primary and any secondary numbers.
    /// * Empty: If the response does not contain a `dpv_confirmation` value, the
    /// address was not submitted for DPV confirmation.
    #[prost(string, tag = "4")]
    pub dpv_confirmation: ::prost::alloc::string::String,
    /// The footnotes from delivery point validation.
    /// Multiple footnotes may be strung together in the same string.
    ///
    /// * `AA`: Input address matched to the ZIP+4 file
    /// * `A1`: Input address was not matched to the ZIP+4 file
    /// * `BB`: Matched to DPV (all components)
    /// * `CC`: Secondary number not matched and not required
    /// * `C1`: Secondary number not matched but required
    /// * `N1`: High-rise address missing secondary number
    /// * `M1`: Primary number missing
    /// * `M3`: Primary number invalid
    /// * `P1`: Input address PO, RR or HC box number missing
    /// * `P3`: Input address PO, RR, or HC Box number invalid
    /// * `F1`: Input address matched to a military address
    /// * `G1`: Input address matched to a general delivery address
    /// * `U1`: Input address matched to a unique ZIP code
    /// * `PB`: Input address matched to PBSA record
    /// * `RR`: DPV confirmed address with PMB information
    /// * `R1`: DPV confirmed address without PMB information
    /// * `R7`: Carrier Route R777 or R779 record
    /// * `IA`: Informed Address identified
    /// * `TA`: Primary number matched by dropping a trailing alpha
    #[prost(string, tag = "5")]
    pub dpv_footnote: ::prost::alloc::string::String,
    /// Indicates if the address is a CMRA (Commercial Mail Receiving Agency)--a
    /// private business receiving mail for clients. Returns a single character.
    ///
    /// * `Y`: The address is a CMRA
    /// * `N`: The address is not a CMRA
    #[prost(string, tag = "6")]
    pub dpv_cmra: ::prost::alloc::string::String,
    /// Is this place vacant?
    /// Returns a single character.
    ///
    /// * `Y`: The address is vacant
    /// * `N`: The address is not vacant
    #[prost(string, tag = "7")]
    pub dpv_vacant: ::prost::alloc::string::String,
    /// Is this a no stat address or an active address?
    /// No stat addresses are ones which are not continuously occupied or addresses
    /// that the USPS does not service. Returns a single character.
    ///
    /// * `Y`: The address is not active
    /// * `N`: The address is active
    #[prost(string, tag = "8")]
    pub dpv_no_stat: ::prost::alloc::string::String,
    /// Indicates the NoStat type. Returns a reason code as int.
    ///
    /// * `1`: IDA (Internal Drop Address) – Addresses that do not receive mail
    /// directly from the USPS but are delivered to a drop address that services
    /// them.
    /// * `2`: CDS - Addresses that have not yet become deliverable. For example, a
    /// new subdivision where lots and primary numbers have been determined, but no
    /// structure exists yet for occupancy.
    /// * `3`: Collision - Addresses that do not actually DPV confirm.
    /// * `4`: CMZ (College, Military and Other Types) - ZIP + 4 records USPS has
    /// incorporated into the data.
    /// * `5`: Regular - Indicates addresses not receiving delivery and the
    /// addresses are not counted as possible deliveries.
    /// * `6`: Secondary Required - The address requires secondary information.
    #[prost(int32, tag = "29")]
    pub dpv_no_stat_reason_code: i32,
    /// Flag indicates mail is delivered to a single receptable at a site.
    /// Returns a single character.
    ///
    /// * `Y`: The mail is delivered to a single receptable at a site.
    /// * `N`: The mail is not delivered to a single receptable at a site.
    #[prost(string, tag = "30")]
    pub dpv_drop: ::prost::alloc::string::String,
    /// Indicates that mail is not delivered to the street address.
    /// Returns a single character.
    ///
    /// * `Y`: The mail is not delivered to the street address.
    /// * `N`: The mail is delivered to the street address.
    #[prost(string, tag = "31")]
    pub dpv_throwback: ::prost::alloc::string::String,
    /// Flag indicates mail delivery is not performed every day of the week.
    /// Returns a single character.
    ///
    /// * `Y`: The mail delivery is not performed every day of the week.
    /// * `N`: No indication the mail delivery is not performed every day of the
    /// week.
    #[prost(string, tag = "32")]
    pub dpv_non_delivery_days: ::prost::alloc::string::String,
    /// Integer identifying non-delivery days. It can be interrogated using bit
    /// flags:
    /// 0x40 – Sunday is a non-delivery day
    /// 0x20 – Monday is a non-delivery day
    /// 0x10 – Tuesday is a non-delivery day
    /// 0x08 – Wednesday is a non-delivery day
    /// 0x04 – Thursday is a non-delivery day
    /// 0x02 – Friday is a non-delivery day
    /// 0x01 – Saturday is a non-delivery day
    #[prost(int32, tag = "33")]
    pub dpv_non_delivery_days_values: i32,
    /// Flag indicates door is accessible, but package will not be left due to
    /// security concerns.
    /// Returns a single character.
    ///
    /// * `Y`: The package will not be left due to security concerns.
    /// * `N`: No indication the package will not be left due to security concerns.
    #[prost(string, tag = "34")]
    pub dpv_no_secure_location: ::prost::alloc::string::String,
    /// Indicates the address was matched to PBSA record.
    /// Returns a single character.
    ///
    /// * `Y`: The address was matched to PBSA record.
    /// * `N`: The address was not matched to PBSA record.
    #[prost(string, tag = "35")]
    pub dpv_pbsa: ::prost::alloc::string::String,
    /// Flag indicates addresses where USPS cannot knock on a door to deliver mail.
    /// Returns a single character.
    ///
    /// * `Y`: The door is not accessible.
    /// * `N`: No indication the door is not accessible.
    #[prost(string, tag = "36")]
    pub dpv_door_not_accessible: ::prost::alloc::string::String,
    /// Indicates that more than one DPV return code is valid for the address.
    /// Returns a single character.
    ///
    /// * `Y`: Address was DPV confirmed for primary and any secondary numbers.
    /// * `N`: Primary and any secondary number information failed to
    /// DPV confirm.
    /// * `S`: Address was DPV confirmed for the primary number only, and the
    /// secondary number information was present by not confirmed,  or a single
    /// trailing alpha on a primary number was dropped to make a DPV match and
    /// secondary information required.
    /// * `D`: Address was DPV confirmed for the primary number only, and the
    /// secondary number information was missing.
    /// * `R`: Address confirmed but assigned to phantom route R777 and R779 and
    /// USPS delivery is not provided.
    #[prost(string, tag = "37")]
    pub dpv_enhanced_delivery_code: ::prost::alloc::string::String,
    /// The carrier route code.
    /// A four character code consisting of a one letter prefix and a three digit
    /// route designator.
    ///
    /// Prefixes:
    ///
    /// * `C`: Carrier route (or city route)
    /// * `R`: Rural route
    /// * `H`: Highway Contract Route
    /// * `B`: Post Office Box Section
    /// * `G`: General delivery unit
    #[prost(string, tag = "9")]
    pub carrier_route: ::prost::alloc::string::String,
    /// Carrier route rate sort indicator.
    #[prost(string, tag = "10")]
    pub carrier_route_indicator: ::prost::alloc::string::String,
    /// The delivery address is matchable, but the EWS file indicates that an exact
    /// match will be available soon.
    #[prost(bool, tag = "11")]
    pub ews_no_match: bool,
    /// Main post office city.
    #[prost(string, tag = "12")]
    pub post_office_city: ::prost::alloc::string::String,
    /// Main post office state.
    #[prost(string, tag = "13")]
    pub post_office_state: ::prost::alloc::string::String,
    /// Abbreviated city.
    #[prost(string, tag = "14")]
    pub abbreviated_city: ::prost::alloc::string::String,
    /// FIPS county code.
    #[prost(string, tag = "15")]
    pub fips_county_code: ::prost::alloc::string::String,
    /// County name.
    #[prost(string, tag = "16")]
    pub county: ::prost::alloc::string::String,
    /// Enhanced Line of Travel (eLOT) number.
    #[prost(string, tag = "17")]
    pub elot_number: ::prost::alloc::string::String,
    /// eLOT Ascending/Descending Flag (A/D).
    #[prost(string, tag = "18")]
    pub elot_flag: ::prost::alloc::string::String,
    /// LACSLink return code.
    #[prost(string, tag = "19")]
    pub lacs_link_return_code: ::prost::alloc::string::String,
    /// LACSLink indicator.
    #[prost(string, tag = "20")]
    pub lacs_link_indicator: ::prost::alloc::string::String,
    /// PO Box only postal code.
    #[prost(bool, tag = "21")]
    pub po_box_only_postal_code: bool,
    /// Footnotes from matching a street or highrise record to suite information.
    /// If business name match is found, the secondary number is returned.
    ///
    /// * `A`: SuiteLink record match, business address improved.
    /// * `00`: No match, business address is not improved.
    #[prost(string, tag = "22")]
    pub suitelink_footnote: ::prost::alloc::string::String,
    /// PMB (Private Mail Box) unit designator.
    #[prost(string, tag = "23")]
    pub pmb_designator: ::prost::alloc::string::String,
    /// PMB (Private Mail Box) number;
    #[prost(string, tag = "24")]
    pub pmb_number: ::prost::alloc::string::String,
    /// Type of the address record that matches the input address.
    ///
    /// * `F`: FIRM. This is a match to a Firm Record, which is the finest level of
    /// match available for an address.
    /// * `G`: GENERAL DELIVERY. This is a match to a General Delivery record.
    /// * `H`: BUILDING / APARTMENT. This is a match to a Building or Apartment
    /// record.
    /// * `P`: POST OFFICE BOX. This is a match to a Post Office Box.
    /// * `R`: RURAL ROUTE or HIGHWAY CONTRACT: This is a match to either a Rural
    /// Route or a Highway Contract record, both of which may have associated Box
    /// Number ranges.
    /// * `S`: STREET RECORD: This is a match to a Street record containing a valid
    /// primary number range.
    #[prost(string, tag = "25")]
    pub address_record_type: ::prost::alloc::string::String,
    /// Indicator that a default address was found, but more specific addresses
    /// exists.
    #[prost(bool, tag = "26")]
    pub default_address: bool,
    /// Error message for USPS data retrieval. This is populated when USPS
    /// processing is suspended because of the detection of artificially created
    /// addresses.
    ///
    /// The USPS data fields might not be populated when this error is present.
    #[prost(string, tag = "27")]
    pub error_message: ::prost::alloc::string::String,
    /// Indicator that the request has been CASS processed.
    #[prost(bool, tag = "28")]
    pub cass_processed: bool,
}
/// Contains information about the place the input was geocoded to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Geocode {
    /// The geocoded location of the input.
    ///
    /// Using place IDs is preferred over using addresses,
    /// latitude/longitude coordinates, or plus codes. Using coordinates when
    /// routing or calculating driving directions will always result in the point
    /// being snapped to the road nearest to those coordinates. This may not be a
    /// road that will quickly or safely lead to the destination and may not be
    /// near an access point to the property. Additionally, when a location is
    /// reverse geocoded, there is no guarantee that the returned address will
    /// match the original.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The plus code corresponding to the `location`.
    #[prost(message, optional, tag = "2")]
    pub plus_code: ::core::option::Option<PlusCode>,
    /// The bounds of the geocoded place.
    #[prost(message, optional, tag = "4")]
    pub bounds: ::core::option::Option<super::super::super::geo::r#type::Viewport>,
    /// The size of the geocoded place, in meters. This is another measure of the
    /// coarseness of the geocoded location, but in physical size rather than in
    /// semantic meaning.
    #[prost(float, tag = "5")]
    pub feature_size_meters: f32,
    /// The PlaceID of the place this input geocodes to.
    ///
    /// For more information about Place IDs see
    /// [here](<https://developers.google.com/maps/documentation/places/web-service/place-id>).
    #[prost(string, tag = "6")]
    pub place_id: ::prost::alloc::string::String,
    /// The type(s) of place that the input geocoded to. For example,
    /// `\['locality', 'political'\]`. The full list of types can be found
    /// [here](<https://developers.google.com/maps/documentation/geocoding/requests-geocoding#Types>).
    #[prost(string, repeated, tag = "7")]
    pub place_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Plus code (<http://plus.codes>) is a location reference with two formats:
/// global code defining a 14mx14m (1/8000th of a degree) or smaller rectangle,
/// and compound code, replacing the prefix with a reference location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlusCode {
    /// Place's global (full) code, such as "9FWM33GV+HQ", representing an
    /// 1/8000 by 1/8000 degree area (~14 by 14 meters).
    #[prost(string, tag = "1")]
    pub global_code: ::prost::alloc::string::String,
    /// Place's compound code, such as "33GV+HQ, Ramberg, Norway", containing
    /// the suffix of the global code and replacing the prefix with a formatted
    /// name of a reference entity.
    #[prost(string, tag = "2")]
    pub compound_code: ::prost::alloc::string::String,
}
/// Details of the post-processed address. Post-processing includes
/// correcting misspelled parts of the address, replacing incorrect parts, and
/// inferring missing parts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    /// The post-processed address, formatted as a single-line address following
    /// the address formatting rules of the region where the address is located.
    #[prost(string, tag = "2")]
    pub formatted_address: ::prost::alloc::string::String,
    /// The post-processed address represented as a postal address.
    #[prost(message, optional, tag = "3")]
    pub postal_address: ::core::option::Option<
        super::super::super::r#type::PostalAddress,
    >,
    /// Unordered list. The individual address components of the formatted and
    /// corrected address, along with validation information. This provides
    /// information on the validation status of the individual components.
    ///
    /// Address components are not ordered in a particular way. Do not make any
    /// assumptions on the ordering of the address components in the list.
    #[prost(message, repeated, tag = "4")]
    pub address_components: ::prost::alloc::vec::Vec<AddressComponent>,
    /// The types of components that were expected to be present in a correctly
    /// formatted mailing address but were not found in the input AND could
    /// not be inferred. Components of this type are not present in
    /// `formatted_address`, `postal_address`, or `address_components`. An
    /// example might be `\['street_number', 'route'\]` for an input like
    /// "Boulder, Colorado, 80301, USA". The list of possible types can be found
    /// [here](<https://developers.google.com/maps/documentation/geocoding/requests-geocoding#Types>).
    #[prost(string, repeated, tag = "5")]
    pub missing_component_types: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// The types of the components that are present in the `address_components`
    /// but could not be confirmed to be correct. This field is provided for the
    /// sake of convenience: its contents are equivalent to iterating through the
    /// `address_components` to find the types of all the components where the
    /// [confirmation_level][google.maps.addressvalidation.v1.AddressComponent.confirmation_level]
    /// is not
    /// [CONFIRMED][google.maps.addressvalidation.v1.AddressComponent.ConfirmationLevel.CONFIRMED]
    /// or the
    /// [inferred][google.maps.addressvalidation.v1.AddressComponent.inferred]
    /// flag is not set to `true`. The list of possible types can be found
    /// [here](<https://developers.google.com/maps/documentation/geocoding/requests-geocoding#Types>).
    #[prost(string, repeated, tag = "6")]
    pub unconfirmed_component_types: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Any tokens in the input that could not be resolved. This might be an
    /// input that was not recognized as a valid part of an address (for example
    /// in an input like "123235253253 Main St, San Francisco, CA, 94105", the
    /// unresolved tokens may look like `\["123235253253"\]` since that does not
    /// look like a valid street number.
    #[prost(string, repeated, tag = "7")]
    pub unresolved_tokens: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents an address component, such as a street, city, or state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressComponent {
    /// The name for this component.
    #[prost(message, optional, tag = "1")]
    pub component_name: ::core::option::Option<ComponentName>,
    /// The type of the address component. See
    /// [Table 2: Additional types returned by the Places
    /// service](<https://developers.google.com/places/web-service/supported_types#table2>)
    /// for a list of possible types.
    #[prost(string, tag = "2")]
    pub component_type: ::prost::alloc::string::String,
    /// Indicates the level of certainty that we have that the component
    /// is correct.
    #[prost(enumeration = "address_component::ConfirmationLevel", tag = "3")]
    pub confirmation_level: i32,
    /// Indicates that the component was not part of the input, but we
    /// inferred it for the address location and believe it should be provided
    /// for a complete address.
    #[prost(bool, tag = "4")]
    pub inferred: bool,
    /// Indicates a correction to a misspelling in the component name.  The API
    /// does not always flag changes from one spelling variant to another, such as
    /// when changing "centre" to "center". It also does not always flag common
    /// misspellings, such as when changing "Amphitheater Pkwy" to "Amphitheatre
    /// Pkwy".
    #[prost(bool, tag = "5")]
    pub spell_corrected: bool,
    /// Indicates the name of the component was replaced with a completely
    /// different one, for example a wrong postal code being replaced with one that
    /// is correct for the address. This is not a cosmetic change, the input
    /// component has been changed to a different one.
    #[prost(bool, tag = "6")]
    pub replaced: bool,
    /// Indicates an address component that is not expected to be present in a
    /// postal address for the given region. We have retained it only because it
    /// was part of the input.
    #[prost(bool, tag = "7")]
    pub unexpected: bool,
}
/// Nested message and enum types in `AddressComponent`.
pub mod address_component {
    /// The different possible values for confirmation levels.
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
    pub enum ConfirmationLevel {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// We were able to verify that this component exists and makes sense in the
        /// context of the rest of the address.
        Confirmed = 1,
        /// This component could not be confirmed, but it is plausible that it
        /// exists. For example, a street number within a known valid range of
        /// numbers on a street where specific house numbers are not known.
        UnconfirmedButPlausible = 2,
        /// This component was not confirmed and is likely to be wrong. For
        /// example, a neighborhood that does not fit the rest of the address.
        UnconfirmedAndSuspicious = 3,
    }
    impl ConfirmationLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConfirmationLevel::Unspecified => "CONFIRMATION_LEVEL_UNSPECIFIED",
                ConfirmationLevel::Confirmed => "CONFIRMED",
                ConfirmationLevel::UnconfirmedButPlausible => "UNCONFIRMED_BUT_PLAUSIBLE",
                ConfirmationLevel::UnconfirmedAndSuspicious => {
                    "UNCONFIRMED_AND_SUSPICIOUS"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONFIRMATION_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
                "CONFIRMED" => Some(Self::Confirmed),
                "UNCONFIRMED_BUT_PLAUSIBLE" => Some(Self::UnconfirmedButPlausible),
                "UNCONFIRMED_AND_SUSPICIOUS" => Some(Self::UnconfirmedAndSuspicious),
                _ => None,
            }
        }
    }
}
/// A wrapper for the name of the component.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentName {
    /// The name text. For example, "5th Avenue" for a street name or "1253" for a
    /// street number.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// The BCP-47 language code. This will not be present if the component name is
    /// not associated with a language, such as a street number.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// The metadata for the address. `metadata` is not guaranteed to be fully
/// populated for every address sent to the Address Validation API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressMetadata {
    /// Indicates that this is the address of a business.
    /// If unset, indicates that the value is unknown.
    #[prost(bool, optional, tag = "2")]
    pub business: ::core::option::Option<bool>,
    /// Indicates that the address of a PO box.
    /// If unset, indicates that the value is unknown.
    #[prost(bool, optional, tag = "3")]
    pub po_box: ::core::option::Option<bool>,
    /// Indicates that this is the address of a residence.
    /// If unset, indicates that the value is unknown.
    #[prost(bool, optional, tag = "6")]
    pub residential: ::core::option::Option<bool>,
}
/// The request for validating an address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateAddressRequest {
    /// Required. The address being validated. Unformatted addresses should be
    /// submitted via [`address_lines`][google.type.PostalAddress.address_lines].
    ///
    /// The total length of the fields in this input must not exceed 280
    /// characters.
    ///
    /// Supported regions can be found
    /// [here](<https://developers.google.com/maps/documentation/address-validation/coverage>).
    ///
    /// The [language_code][google.type.PostalAddress.language_code] value in the
    /// input address is reserved for future uses and is ignored today. The
    /// validated address result will be populated based on the preferred language
    /// for the given address, as identified by the system.
    ///
    /// The Address Validation API ignores the values in
    /// [recipients][google.type.PostalAddress.recipients] and
    /// [organization][google.type.PostalAddress.organization]. Any values in those
    /// fields will be discarded and not returned. Please do not set them.
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<super::super::super::r#type::PostalAddress>,
    /// This field must be empty for the first address validation request. If
    /// more requests are necessary to fully validate a single address (for
    /// example if the changes the user makes after the initial validation need to
    /// be re-validated), then each followup request must populate this field with
    /// the
    /// [response_id][google.maps.addressvalidation.v1.ValidateAddressResponse.response_id]
    /// from the very first response in the validation sequence.
    #[prost(string, tag = "2")]
    pub previous_response_id: ::prost::alloc::string::String,
    /// Enables USPS CASS compatible mode. This affects _only_ the
    /// \[google.maps.addressvalidation.v1.ValidationResult.usps_data\] field of
    /// \[google.maps.addressvalidation.v1.ValidationResult\]. Note: for USPS CASS
    /// enabled requests for addresses in Puerto Rico, a
    /// \[google.type.PostalAddress.region_code\] of the `address` must be provided
    /// as "PR", or an \[google.type.PostalAddress.administrative_area\] of the
    /// `address` must be provided as "Puerto Rico" (case-insensitive) or "PR".
    ///
    /// It's recommended to use a componentized `address`, or alternatively specify
    /// at least two \[google.type.PostalAddress.address_lines\] where the first line
    /// contains the street number and name and the second line contains the city,
    /// state, and zip code.
    #[prost(bool, tag = "3")]
    pub enable_usps_cass: bool,
    /// Optional. A string which identifies an Autocomplete session for billing
    /// purposes. Must be a URL and filename safe base64 string with at most 36
    /// ASCII characters in length. Otherwise an INVALID_ARGUMENT error is
    /// returned.
    ///
    /// The session begins when the user starts typing a query, and concludes when
    /// they select a place and a call to Place Details or Address Validation is
    /// made. Each session can have multiple autocomplete queries, followed by one
    /// Place Details or Address Validation request. The credentials used for each
    /// request within a session must belong to the same Google Cloud Console
    /// project. Once a session has concluded, the token is no longer valid; your
    /// app must generate a fresh token for each session. If the `session_token`
    /// parameter is omitted, or if you reuse a session token, the session is
    /// charged as if no session token was provided (each request is billed
    /// separately).
    ///
    /// Note: Address Validation can only be used in sessions with the
    /// Autocomplete (New) API, not the old Autocomplete API. See
    /// <https://developers.google.com/maps/documentation/places/web-service/session-pricing>
    /// for more details.
    #[prost(string, tag = "5")]
    pub session_token: ::prost::alloc::string::String,
}
/// The response to an address validation request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateAddressResponse {
    /// The result of the address validation.
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<ValidationResult>,
    /// The UUID that identifies this response. If the address needs to be
    /// re-validated, this UUID *must* accompany the new request.
    #[prost(string, tag = "2")]
    pub response_id: ::prost::alloc::string::String,
}
/// The request for sending validation feedback.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvideValidationFeedbackRequest {
    /// Required. The outcome of the sequence of validation attempts.
    ///
    /// If this field is set to `VALIDATION_CONCLUSION_UNSPECIFIED`, an
    /// `INVALID_ARGUMENT` error will be returned.
    #[prost(
        enumeration = "provide_validation_feedback_request::ValidationConclusion",
        tag = "1"
    )]
    pub conclusion: i32,
    /// Required. The ID of the response that this feedback is for. This should be
    /// the
    /// [response_id][google.maps.addressvalidation.v1.ValidateAddressRequest.response_id]
    /// from the first response in a series of address validation attempts.
    #[prost(string, tag = "2")]
    pub response_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ProvideValidationFeedbackRequest`.
pub mod provide_validation_feedback_request {
    /// The possible final outcomes of the sequence of address validation requests
    /// needed to validate an address.
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
    pub enum ValidationConclusion {
        /// This value is unused.
        /// If the `ProvideValidationFeedbackRequest.conclusion` field is set to
        /// `VALIDATION_CONCLUSION_UNSPECIFIED`, an `INVALID_ARGUMENT` error will be
        /// returned.
        Unspecified = 0,
        /// The version of the address returned by the Address Validation API was
        /// used for the transaction.
        ValidatedVersionUsed = 1,
        /// The version of the address provided by the user was used for the
        /// transaction
        UserVersionUsed = 2,
        /// A version of the address that was entered after the last validation
        /// attempt but that was not re-validated was used for the transaction.
        UnvalidatedVersionUsed = 3,
        /// The transaction was abandoned and the address was not used.
        Unused = 4,
    }
    impl ValidationConclusion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValidationConclusion::Unspecified => "VALIDATION_CONCLUSION_UNSPECIFIED",
                ValidationConclusion::ValidatedVersionUsed => "VALIDATED_VERSION_USED",
                ValidationConclusion::UserVersionUsed => "USER_VERSION_USED",
                ValidationConclusion::UnvalidatedVersionUsed => {
                    "UNVALIDATED_VERSION_USED"
                }
                ValidationConclusion::Unused => "UNUSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VALIDATION_CONCLUSION_UNSPECIFIED" => Some(Self::Unspecified),
                "VALIDATED_VERSION_USED" => Some(Self::ValidatedVersionUsed),
                "USER_VERSION_USED" => Some(Self::UserVersionUsed),
                "UNVALIDATED_VERSION_USED" => Some(Self::UnvalidatedVersionUsed),
                "UNUSED" => Some(Self::Unused),
                _ => None,
            }
        }
    }
}
/// The response for validation feedback.
///
/// The response is empty if the feedback is sent successfully.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvideValidationFeedbackResponse {}
/// The result of validating an address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationResult {
    /// Overall verdict flags
    #[prost(message, optional, tag = "1")]
    pub verdict: ::core::option::Option<Verdict>,
    /// Information about the address itself as opposed to the geocode.
    #[prost(message, optional, tag = "2")]
    pub address: ::core::option::Option<Address>,
    /// Information about the location and place that the address geocoded to.
    #[prost(message, optional, tag = "3")]
    pub geocode: ::core::option::Option<Geocode>,
    /// Other information relevant to deliverability. `metadata` is not guaranteed
    /// to be fully populated for every address sent to the Address Validation API.
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<AddressMetadata>,
    /// Extra deliverability flags provided by USPS. Only provided in region `US`
    /// and `PR`.
    #[prost(message, optional, tag = "5")]
    pub usps_data: ::core::option::Option<UspsData>,
}
/// High level overview of the address validation result and geocode.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Verdict {
    /// The granularity of the **input** address. This is the result of parsing the
    /// input address and does not give any validation signals. For validation
    /// signals, refer to `validation_granularity` below.
    ///
    /// For example, if the input address includes a specific apartment number,
    /// then the `input_granularity` here will be `SUB_PREMISE`. If we cannot match
    /// the apartment number in the databases or the apartment number is invalid,
    /// the `validation_granularity` will likely be `PREMISE` or below.
    #[prost(enumeration = "verdict::Granularity", tag = "1")]
    pub input_granularity: i32,
    /// The granularity level that the API can fully **validate** the address to.
    /// For example, an `validation_granularity` of `PREMISE` indicates all address
    /// components at the level of `PREMISE` or more coarse can be validated.
    ///
    /// Per address component validation result can be found in
    /// \[google.maps.addressvalidation.v1.Address.address_components\].
    #[prost(enumeration = "verdict::Granularity", tag = "2")]
    pub validation_granularity: i32,
    /// Information about the granularity of the
    /// [`geocode`][google.maps.addressvalidation.v1.ValidationResult.geocode].
    /// This can be understood as the semantic meaning of how coarse or fine the
    /// geocoded location is.
    ///
    /// This can differ from the `validation_granularity` above occasionally. For
    /// example, our database might record the existence of an apartment number but
    /// do not have a precise location for the apartment within a big apartment
    /// complex. In that case, the `validation_granularity` will be `SUB_PREMISE`
    /// but the `geocode_granularity` will be `PREMISE`.
    #[prost(enumeration = "verdict::Granularity", tag = "3")]
    pub geocode_granularity: i32,
    /// The address is considered complete if there are no unresolved tokens, no
    /// unexpected or missing address components. See
    /// [`missing_component_types`][google.maps.addressvalidation.v1.Address.missing_component_types],
    /// [`unresolved_tokens`][google.maps.addressvalidation.v1.Address.unresolved_tokens]
    /// or
    /// [`unexpected`][google.maps.addressvalidation.v1.AddressComponent.unexpected]
    /// fields for more details.
    #[prost(bool, tag = "4")]
    pub address_complete: bool,
    /// At least one address component cannot be categorized or validated, see
    /// \[google.maps.addressvalidation.v1.Address.address_components\] for
    /// details.
    #[prost(bool, tag = "5")]
    pub has_unconfirmed_components: bool,
    /// At least one address component was inferred (added) that wasn't in the
    /// input, see
    /// \[google.maps.addressvalidation.v1.Address.address_components\] for
    /// details.
    #[prost(bool, tag = "6")]
    pub has_inferred_components: bool,
    /// At least one address component was replaced, see
    /// \[google.maps.addressvalidation.v1.Address.address_components\] for
    /// details.
    #[prost(bool, tag = "7")]
    pub has_replaced_components: bool,
}
/// Nested message and enum types in `Verdict`.
pub mod verdict {
    /// The various granularities that an address or a geocode can have.
    /// When used to indicate granularity for an *address*, these values indicate
    /// with how fine a granularity the address identifies a mailing destination.
    /// For example, an address such as "123 Main Street, Redwood City, CA, 94061"
    /// identifies a `PREMISE` while something like "Redwood City, CA, 94061"
    /// identifies a `LOCALITY`. However, if we are unable to find a geocode for
    /// "123 Main Street" in Redwood City, the geocode returned might be of
    /// `LOCALITY` granularity even though the address is more granular.
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
    pub enum Granularity {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Below-building level result, such as an apartment.
        SubPremise = 1,
        /// Building-level result.
        Premise = 2,
        /// A geocode that approximates the building-level location of the address.
        PremiseProximity = 3,
        /// The address or geocode indicates a block. Only used in regions which
        /// have block-level addressing, such as Japan.
        Block = 4,
        /// The geocode or address is granular to route, such as a street, road, or
        /// highway.
        Route = 5,
        /// All other granularities, which are bucketed together since they are not
        /// deliverable.
        Other = 6,
    }
    impl Granularity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Granularity::Unspecified => "GRANULARITY_UNSPECIFIED",
                Granularity::SubPremise => "SUB_PREMISE",
                Granularity::Premise => "PREMISE",
                Granularity::PremiseProximity => "PREMISE_PROXIMITY",
                Granularity::Block => "BLOCK",
                Granularity::Route => "ROUTE",
                Granularity::Other => "OTHER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GRANULARITY_UNSPECIFIED" => Some(Self::Unspecified),
                "SUB_PREMISE" => Some(Self::SubPremise),
                "PREMISE" => Some(Self::Premise),
                "PREMISE_PROXIMITY" => Some(Self::PremiseProximity),
                "BLOCK" => Some(Self::Block),
                "ROUTE" => Some(Self::Route),
                "OTHER" => Some(Self::Other),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod address_validation_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The service for validating addresses.
    #[derive(Debug, Clone)]
    pub struct AddressValidationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AddressValidationClient<T>
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
        ) -> AddressValidationClient<InterceptedService<T, F>>
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
            AddressValidationClient::new(InterceptedService::new(inner, interceptor))
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
        /// Validates an address.
        pub async fn validate_address(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ValidateAddressResponse>,
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
                "/google.maps.addressvalidation.v1.AddressValidation/ValidateAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.addressvalidation.v1.AddressValidation",
                        "ValidateAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Feedback about the outcome of the sequence of validation attempts. This
        /// should be the last call made after a sequence of validation calls for the
        /// same address, and should be called once the transaction is concluded. This
        /// should only be sent once for the sequence of `ValidateAddress` requests
        /// needed to validate an address fully.
        pub async fn provide_validation_feedback(
            &mut self,
            request: impl tonic::IntoRequest<super::ProvideValidationFeedbackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProvideValidationFeedbackResponse>,
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
                "/google.maps.addressvalidation.v1.AddressValidation/ProvideValidationFeedback",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.addressvalidation.v1.AddressValidation",
                        "ProvideValidationFeedback",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
