/// A RequestHeader contains fields common to all Delivery RPC requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliveryRequestHeader {
    /// The BCP-47 language code, such as en-US or sr-Latn. For more information,
    /// see <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.> If none
    /// is specified, the response may be in any language, with a preference for
    /// English if such a name exists. Field value example: `en-US`.
    #[prost(string, tag = "1")]
    pub language_code: ::prost::alloc::string::String,
    /// Required. CLDR region code of the region where the request originates.
    /// Field value example: `US`.
    #[prost(string, tag = "2")]
    pub region_code: ::prost::alloc::string::String,
    /// Version of the calling SDK, if applicable.
    /// The version format is "major.minor.patch", example: `1.1.2`.
    #[prost(string, tag = "3")]
    pub sdk_version: ::prost::alloc::string::String,
    /// Version of the operating system on which the calling SDK is running.
    /// Field value examples: `4.4.1`, `12.1`.
    #[prost(string, tag = "4")]
    pub os_version: ::prost::alloc::string::String,
    /// Model of the device on which the calling SDK is running.
    /// Field value examples: `iPhone12,1`, `SM-G920F`.
    #[prost(string, tag = "5")]
    pub device_model: ::prost::alloc::string::String,
    /// The type of SDK sending the request.
    #[prost(enumeration = "delivery_request_header::SdkType", tag = "6")]
    pub sdk_type: i32,
    /// Version of the MapSDK which the calling SDK depends on, if applicable.
    /// The version format is "major.minor.patch", example: `5.2.1`.
    #[prost(string, tag = "7")]
    pub maps_sdk_version: ::prost::alloc::string::String,
    /// Version of the NavSDK which the calling SDK depends on, if applicable.
    /// The version format is "major.minor.patch", example: `2.1.0`.
    #[prost(string, tag = "8")]
    pub nav_sdk_version: ::prost::alloc::string::String,
    /// Platform of the calling SDK.
    #[prost(enumeration = "delivery_request_header::Platform", tag = "9")]
    pub platform: i32,
    /// Manufacturer of the Android device from the calling SDK, only applicable
    /// for the Android SDKs.
    /// Field value example: `Samsung`.
    #[prost(string, tag = "10")]
    pub manufacturer: ::prost::alloc::string::String,
    /// Android API level of the calling SDK, only applicable for the Android SDKs.
    /// Field value example: `23`.
    #[prost(int32, tag = "11")]
    pub android_api_level: i32,
}
/// Nested message and enum types in `DeliveryRequestHeader`.
pub mod delivery_request_header {
    /// Possible types of SDK.
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
    pub enum SdkType {
        /// The default value. This value is used if the `sdk_type` is omitted.
        Unspecified = 0,
        /// The calling SDK is Consumer.
        Consumer = 1,
        /// The calling SDK is Driver.
        Driver = 2,
        /// The calling SDK is JavaScript.
        Javascript = 3,
    }
    impl SdkType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SdkType::Unspecified => "SDK_TYPE_UNSPECIFIED",
                SdkType::Consumer => "CONSUMER",
                SdkType::Driver => "DRIVER",
                SdkType::Javascript => "JAVASCRIPT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SDK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CONSUMER" => Some(Self::Consumer),
                "DRIVER" => Some(Self::Driver),
                "JAVASCRIPT" => Some(Self::Javascript),
                _ => None,
            }
        }
    }
    /// The platform of the calling SDK.
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
        /// The default value. This value is used if the platform is omitted.
        Unspecified = 0,
        /// The request is coming from Android.
        Android = 1,
        /// The request is coming from iOS.
        Ios = 2,
        /// The request is coming from the web.
        Web = 3,
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
                Platform::Web => "WEB",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PLATFORM_UNSPECIFIED" => Some(Self::Unspecified),
                "ANDROID" => Some(Self::Android),
                "IOS" => Some(Self::Ios),
                "WEB" => Some(Self::Web),
                _ => None,
            }
        }
    }
}
/// Describes a vehicle attribute as a key-value pair. The "key:value" string
/// length cannot exceed 256 characters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliveryVehicleAttribute {
    /// The attribute's key.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The attribute's value.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// The location, speed, and heading of a vehicle at a point in time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliveryVehicleLocation {
    /// The location of the vehicle.
    /// When it is sent to Fleet Engine, the vehicle's location is a GPS location.
    /// When you receive it in a response, the vehicle's location can be either a
    /// GPS location, a supplemental location, or some other estimated location.
    /// The source is specified in `location_sensor`.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<
        super::super::super::super::google::r#type::LatLng,
    >,
    /// Deprecated: Use `latlng_accuracy` instead.
    #[deprecated]
    #[prost(message, optional, tag = "8")]
    pub horizontal_accuracy: ::core::option::Option<f64>,
    /// Accuracy of `location` in meters as a radius.
    #[prost(message, optional, tag = "22")]
    pub latlng_accuracy: ::core::option::Option<f64>,
    /// Direction the vehicle is moving in degrees.  0 represents North.
    /// The valid range is [0,360).
    #[prost(message, optional, tag = "2")]
    pub heading: ::core::option::Option<i32>,
    /// Deprecated: Use `heading_accuracy` instead.
    #[deprecated]
    #[prost(message, optional, tag = "10")]
    pub bearing_accuracy: ::core::option::Option<f64>,
    /// Accuracy of `heading` in degrees.
    #[prost(message, optional, tag = "23")]
    pub heading_accuracy: ::core::option::Option<f64>,
    /// Altitude in meters above WGS84.
    #[prost(message, optional, tag = "5")]
    pub altitude: ::core::option::Option<f64>,
    /// Deprecated: Use `altitude_accuracy` instead.
    #[deprecated]
    #[prost(message, optional, tag = "9")]
    pub vertical_accuracy: ::core::option::Option<f64>,
    /// Accuracy of `altitude` in meters.
    #[prost(message, optional, tag = "24")]
    pub altitude_accuracy: ::core::option::Option<f64>,
    /// Speed of the vehicle in kilometers per hour.
    /// Deprecated: Use `speed` instead.
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub speed_kmph: ::core::option::Option<i32>,
    /// Speed of the vehicle in meters/second
    #[prost(message, optional, tag = "6")]
    pub speed: ::core::option::Option<f64>,
    /// Accuracy of `speed` in meters/second.
    #[prost(message, optional, tag = "7")]
    pub speed_accuracy: ::core::option::Option<f64>,
    /// The time when `location` was reported by the sensor according to the
    /// sensor's clock.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the server received the location information.
    #[prost(message, optional, tag = "13")]
    pub server_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Provider of location data (for example, `GPS`).
    #[prost(enumeration = "DeliveryVehicleLocationSensor", tag = "11")]
    pub location_sensor: i32,
    /// Whether `location` is snapped to a road.
    #[prost(message, optional, tag = "27")]
    pub is_road_snapped: ::core::option::Option<bool>,
    /// Input only. Indicates whether the GPS sensor is enabled on the mobile
    /// device.
    #[prost(message, optional, tag = "12")]
    pub is_gps_sensor_enabled: ::core::option::Option<bool>,
    /// Input only. Time (in seconds) since this location was first sent to the
    /// server. This will be zero for the first update. If the time is unknown (for
    /// example, when the app restarts), this value resets to zero.
    #[prost(message, optional, tag = "14")]
    pub time_since_update: ::core::option::Option<i32>,
    /// Input only. Deprecated: Other signals are now used to determine if a
    /// location is stale.
    #[deprecated]
    #[prost(message, optional, tag = "15")]
    pub num_stale_updates: ::core::option::Option<i32>,
    /// Raw vehicle location (unprocessed by road-snapper).
    #[prost(message, optional, tag = "16")]
    pub raw_location: ::core::option::Option<
        super::super::super::super::google::r#type::LatLng,
    >,
    /// Timestamp associated with the raw location.
    #[prost(message, optional, tag = "17")]
    pub raw_location_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Source of the raw location.
    #[prost(enumeration = "DeliveryVehicleLocationSensor", tag = "28")]
    pub raw_location_sensor: i32,
    /// Accuracy of `raw_location` as a radius, in meters.
    #[prost(message, optional, tag = "25")]
    pub raw_location_accuracy: ::core::option::Option<f64>,
    /// Supplemental location provided by the integrating app.
    #[prost(message, optional, tag = "18")]
    pub supplemental_location: ::core::option::Option<
        super::super::super::super::google::r#type::LatLng,
    >,
    /// Timestamp associated with the supplemental location.
    #[prost(message, optional, tag = "19")]
    pub supplemental_location_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Source of the supplemental location.
    #[prost(enumeration = "DeliveryVehicleLocationSensor", tag = "20")]
    pub supplemental_location_sensor: i32,
    /// Accuracy of `supplemental_location` as a radius, in meters.
    #[prost(message, optional, tag = "21")]
    pub supplemental_location_accuracy: ::core::option::Option<f64>,
    /// Deprecated: Use `is_road_snapped` instead.
    #[deprecated]
    #[prost(bool, tag = "26")]
    pub road_snapped: bool,
}
/// A time range.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeWindow {
    /// Required. The start time of the time window (inclusive).
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The end time of the time window (inclusive).
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Describes a task attribute as a key-value pair. The "key:value" string length
/// cannot exceed 256 characters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskAttribute {
    /// The attribute's key. Keys may not contain the colon character (:).
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The attribute's value, can be in string, bool, or double type. If none are
    /// set the TaskAttribute string_value will be stored as the empty string "".
    #[prost(oneof = "task_attribute::TaskAttributeValue", tags = "2, 3, 4")]
    pub task_attribute_value: ::core::option::Option<task_attribute::TaskAttributeValue>,
}
/// Nested message and enum types in `TaskAttribute`.
pub mod task_attribute {
    /// The attribute's value, can be in string, bool, or double type. If none are
    /// set the TaskAttribute string_value will be stored as the empty string "".
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TaskAttributeValue {
        /// String typed attribute value.
        #[prost(string, tag = "2")]
        StringValue(::prost::alloc::string::String),
        /// Boolean typed attribute value.
        #[prost(bool, tag = "3")]
        BoolValue(bool),
        /// Double typed attribute value.
        #[prost(double, tag = "4")]
        NumberValue(f64),
    }
}
/// The sensor or methodology used to determine the location.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeliveryVehicleLocationSensor {
    /// The sensor is unspecified or unknown.
    UnknownSensor = 0,
    /// GPS or Assisted GPS.
    Gps = 1,
    /// Assisted GPS, cell tower ID, or WiFi access point.
    Network = 2,
    /// Cell tower ID or WiFi access point.
    Passive = 3,
    /// A location determined by the mobile device to be the most likely
    /// road position.
    RoadSnappedLocationProvider = 4,
    /// A customer-supplied location from an independent source.  Typically, this
    /// value is used for a location provided from sources other than the mobile
    /// device running Driver SDK.  If the original source is described by one of
    /// the other enum values, use that value. Locations marked
    /// CUSTOMER_SUPPLIED_LOCATION are typically provided via a DeliveryVehicle's
    /// `last_location.supplemental_location_sensor`.
    CustomerSuppliedLocation = 5,
    /// A location calculated by Fleet Engine based on the signals available to it.
    /// Output only. This value will be rejected if it is received in a request.
    FleetEngineLocation = 6,
    /// Android's Fused Location Provider.
    FusedLocationProvider = 100,
    /// The location provider on Apple operating systems.
    CoreLocation = 200,
}
impl DeliveryVehicleLocationSensor {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeliveryVehicleLocationSensor::UnknownSensor => "UNKNOWN_SENSOR",
            DeliveryVehicleLocationSensor::Gps => "GPS",
            DeliveryVehicleLocationSensor::Network => "NETWORK",
            DeliveryVehicleLocationSensor::Passive => "PASSIVE",
            DeliveryVehicleLocationSensor::RoadSnappedLocationProvider => {
                "ROAD_SNAPPED_LOCATION_PROVIDER"
            }
            DeliveryVehicleLocationSensor::CustomerSuppliedLocation => {
                "CUSTOMER_SUPPLIED_LOCATION"
            }
            DeliveryVehicleLocationSensor::FleetEngineLocation => "FLEET_ENGINE_LOCATION",
            DeliveryVehicleLocationSensor::FusedLocationProvider => {
                "FUSED_LOCATION_PROVIDER"
            }
            DeliveryVehicleLocationSensor::CoreLocation => "CORE_LOCATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_SENSOR" => Some(Self::UnknownSensor),
            "GPS" => Some(Self::Gps),
            "NETWORK" => Some(Self::Network),
            "PASSIVE" => Some(Self::Passive),
            "ROAD_SNAPPED_LOCATION_PROVIDER" => Some(Self::RoadSnappedLocationProvider),
            "CUSTOMER_SUPPLIED_LOCATION" => Some(Self::CustomerSuppliedLocation),
            "FLEET_ENGINE_LOCATION" => Some(Self::FleetEngineLocation),
            "FUSED_LOCATION_PROVIDER" => Some(Self::FusedLocationProvider),
            "CORE_LOCATION" => Some(Self::CoreLocation),
            _ => None,
        }
    }
}
/// The vehicle's navigation status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeliveryVehicleNavigationStatus {
    /// Unspecified navigation status.
    UnknownNavigationStatus = 0,
    /// The Driver app's navigation is in `FREE_NAV` mode.
    NoGuidance = 1,
    /// Turn-by-turn navigation is available and the Driver app navigation has
    /// entered `GUIDED_NAV` mode.
    EnrouteToDestination = 2,
    /// The vehicle has gone off the suggested route.
    OffRoute = 3,
    /// The vehicle is within approximately 50m of the destination.
    ArrivedAtDestination = 4,
}
impl DeliveryVehicleNavigationStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeliveryVehicleNavigationStatus::UnknownNavigationStatus => {
                "UNKNOWN_NAVIGATION_STATUS"
            }
            DeliveryVehicleNavigationStatus::NoGuidance => "NO_GUIDANCE",
            DeliveryVehicleNavigationStatus::EnrouteToDestination => {
                "ENROUTE_TO_DESTINATION"
            }
            DeliveryVehicleNavigationStatus::OffRoute => "OFF_ROUTE",
            DeliveryVehicleNavigationStatus::ArrivedAtDestination => {
                "ARRIVED_AT_DESTINATION"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_NAVIGATION_STATUS" => Some(Self::UnknownNavigationStatus),
            "NO_GUIDANCE" => Some(Self::NoGuidance),
            "ENROUTE_TO_DESTINATION" => Some(Self::EnrouteToDestination),
            "OFF_ROUTE" => Some(Self::OffRoute),
            "ARRIVED_AT_DESTINATION" => Some(Self::ArrivedAtDestination),
            _ => None,
        }
    }
}
/// The `DeliveryVehicle` message. A delivery vehicle transports shipments from a
/// depot to a delivery location, and from a pickup location to the depot. In
/// some cases, delivery vehicles also transport shipments directly from the
/// pickup location to the delivery location.
///
/// Note: gRPC and REST APIs use different field naming conventions. For example,
/// the `DeliveryVehicle.current_route_segment` field in the gRPC API and the
/// `DeliveryVehicle.currentRouteSegment` field in the REST API refer to the same
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliveryVehicle {
    /// The unique name of this Delivery Vehicle.
    /// The format is `providers/{provider}/deliveryVehicles/{vehicle}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The last reported location of the Delivery Vehicle.
    #[prost(message, optional, tag = "2")]
    pub last_location: ::core::option::Option<DeliveryVehicleLocation>,
    /// The Delivery Vehicle's navigation status.
    #[prost(enumeration = "DeliveryVehicleNavigationStatus", tag = "3")]
    pub navigation_status: i32,
    /// The encoded polyline specifying the route that the navigation recommends
    /// taking to the next waypoint. Your driver app updates this when a
    /// stop is reached or passed, and when the navigation reroutes. These
    /// `LatLng`s are returned in
    /// `Task.journey_sharing_info.remaining_vehicle_journey_segments\[0\].path`
    /// (gRPC) or `Task.journeySharingInfo.remainingVehicleJourneySegments\[0\].path`
    /// (REST) for all active Tasks assigned to the Vehicle.
    ///
    /// There are a few cases where this field might not be used to populate
    /// `Task.journey_sharing_info.remaining_vehicle_journey_segments\[0\].path`
    /// (gRPC) or `Task.journeySharingInfo.remainingVehicleJourneySegments\[0\].path`
    /// (REST):
    ///
    /// * The endpoint of the `current_route_segment` does not match
    /// `DeliveryVehicle.remaining_vehicle_journey_segments\[0\].stop` (gRPC) or
    /// `DeliveryVehicle.remainingVehicleJourneySegments\[0\].stop` (REST).
    ///
    /// * The driver app has not updated its location recently, so the last
    /// updated value for this field might be stale.
    ///
    /// * The driver app has recently updated its location, but the
    /// `current_route_segment` is stale, and points to a previous vehicle stop.
    ///
    /// In these cases, Fleet Engine populates this field with a route from the
    /// most recently passed VehicleStop to the upcoming VehicleStop to ensure that
    /// the consumer of this field has the best available information on the
    /// current path of the Delivery Vehicle.
    #[prost(bytes = "bytes", tag = "4")]
    pub current_route_segment: ::prost::bytes::Bytes,
    /// The location where the `current_route_segment` ends. This is not currently
    /// populated by the driver app, but you can supply it on
    /// `UpdateDeliveryVehicle` calls. It is either the `LatLng` from the upcoming
    /// vehicle stop, or the last `LatLng` of the `current_route_segment`. Fleet
    /// Engine will then do its best to interpolate to an actual `VehicleStop`.
    ///
    /// This field is ignored in `UpdateDeliveryVehicle` calls if the
    /// `current_route_segment` field is empty.
    #[prost(message, optional, tag = "5")]
    pub current_route_segment_end_point: ::core::option::Option<
        super::super::super::super::google::r#type::LatLng,
    >,
    /// The remaining driving distance for the `current_route_segment`.
    /// The Driver app typically provides this field, but there are some
    /// circumstances in which Fleet Engine will override the value sent by the
    /// app. For more information, see
    /// \[DeliveryVehicle.current_route_segment][maps.fleetengine.delivery.v1.DeliveryVehicle.current_route_segment\].
    /// This field is returned in
    /// `Task.remaining_vehicle_journey_segments\[0\].driving_distance_meters` (gRPC)
    /// or `Task.remainingVehicleJourneySegments\[0\].drivingDistanceMeters` (REST)
    /// for all active `Task`s assigned to the Delivery Vehicle.
    ///
    /// Fleet Engine ignores this field in `UpdateDeliveryVehicleRequest` if the
    /// `current_route_segment` field is empty.
    #[prost(message, optional, tag = "6")]
    pub remaining_distance_meters: ::core::option::Option<i32>,
    /// The remaining driving time for the `current_route_segment`.
    /// The Driver app typically provides this field, but there are some
    /// circumstances in which Fleet Engine will override the value sent by the
    /// app.  For more information, see
    /// \[DeliveryVehicle.current_route_segment][maps.fleetengine.delivery.v1.DeliveryVehicle.current_route_segment\].
    /// This field is returned in
    /// `Task.remaining_vehicle_journey_segments\[0\].driving_duration` (gRPC) or
    /// `Task.remainingVehicleJourneySegments\[0\].drivingDuration` (REST) for all
    /// active tasks assigned to the Delivery Vehicle.
    ///
    /// Fleet Engine ignores this field in `UpdateDeliveryVehicleRequest` if the
    /// `current_route_segment` field is empty.
    #[prost(message, optional, tag = "7")]
    pub remaining_duration: ::core::option::Option<::prost_types::Duration>,
    /// The journey segments assigned to this Delivery Vehicle, starting from the
    /// Vehicle's most recently reported location. This field won't be populated
    /// in the response of `ListDeliveryVehicles`.
    #[prost(message, repeated, tag = "8")]
    pub remaining_vehicle_journey_segments: ::prost::alloc::vec::Vec<
        VehicleJourneySegment,
    >,
    /// A list of custom Delivery Vehicle attributes. A Delivery Vehicle can have
    /// at most 50 attributes, and each attribute must have a unique key.
    #[prost(message, repeated, tag = "9")]
    pub attributes: ::prost::alloc::vec::Vec<DeliveryVehicleAttribute>,
}
/// A location with any additional identifiers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationInfo {
    /// The location's coordinates.
    #[prost(message, optional, tag = "1")]
    pub point: ::core::option::Option<
        super::super::super::super::google::r#type::LatLng,
    >,
}
/// Represents a Vehicle’s travel segment - from its previous stop to the
/// current stop. If it is the first active stop, then it is from the
/// Vehicle’s current location to this stop.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleJourneySegment {
    /// Specifies the stop location, along with the `Task`s associated with
    /// the stop. Some fields of the VehicleStop might not be present if this
    /// journey segment is part of `JourneySharingInfo`.
    #[prost(message, optional, tag = "1")]
    pub stop: ::core::option::Option<VehicleStop>,
    /// Output only. The travel distance from the previous stop to this stop.
    /// If the current stop is the first stop in the list of journey
    /// segments, then the starting point is the vehicle's location recorded
    /// at the time that this stop was added to the list. This field might not be
    /// present if this journey segment is part of `JourneySharingInfo`.
    #[prost(message, optional, tag = "2")]
    pub driving_distance_meters: ::core::option::Option<i32>,
    /// Output only. The travel time from the previous stop to this stop.
    /// If the current stop is the first stop in the list of journey
    /// segments, then the starting point is the Vehicle's location recorded
    /// at the time that this stop was added to the list.
    ///
    /// If this field is defined in the path
    /// `Task.remaining_vehicle_journey_segments\[0\].driving_duration` (gRPC) or
    /// `Task.remainingVehicleJourneySegments\[0\].drivingDuration` (REST),
    /// then it may be populated with the value from
    /// `DeliveryVehicle.remaining_duration` (gRPC) or
    /// `DeliveryVehicle.remainingDuration` (REST).
    /// This provides the remaining driving duration from the driver app's latest
    /// known location rather than the driving time from the previous stop.
    #[prost(message, optional, tag = "3")]
    pub driving_duration: ::core::option::Option<::prost_types::Duration>,
    /// Output only. The path from the previous stop to this stop. If the current
    /// stop is the first stop in the list of journey segments, then this is the
    /// path from the vehicle's current location to this stop at the time that the
    /// stop was added to the list. This field might not be present if this journey
    /// segment is part of `JourneySharingInfo`.
    ///
    /// If this field is defined in the path
    /// `Task.journey_sharing_info.remaining_vehicle_journey_segments\[0\].path`
    /// (gRPC) or `Task.journeySharingInfo.remainingVehicleJourneySegments\[0\].path`
    /// (REST), then it may be populated with the `LatLng`s decoded from
    /// `DeliveryVehicle.current_route_segment` (gRPC) or
    /// `DeliveryVehicle.currentRouteSegment` (REST). This provides the driving
    /// path from the driver app's latest known location rather than the path from
    /// the previous stop.
    #[prost(message, repeated, tag = "5")]
    pub path: ::prost::alloc::vec::Vec<
        super::super::super::super::google::r#type::LatLng,
    >,
}
/// Describes a point where a Vehicle stops to perform one or more `Task`s.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleStop {
    /// Required. The location of the stop. Note that the locations in the `Task`s
    /// might not exactly match this location, but will be within a short distance
    /// of it. This field won't be populated in the response of either a `GetTask`,
    /// or a `SearchTasks` call.
    #[prost(message, optional, tag = "1")]
    pub planned_location: ::core::option::Option<LocationInfo>,
    /// The list of `Task`s to be performed at this stop. This field won't be
    /// populated in the response of either a `GetTask` or `SearchTasks` call.
    #[prost(message, repeated, tag = "2")]
    pub tasks: ::prost::alloc::vec::Vec<vehicle_stop::TaskInfo>,
    /// The state of the `VehicleStop`. This field won't be populated in the
    /// response of either a `GetTask`, or a `SearchTasks` call.
    #[prost(enumeration = "vehicle_stop::State", tag = "3")]
    pub state: i32,
}
/// Nested message and enum types in `VehicleStop`.
pub mod vehicle_stop {
    /// Additional information about the Task performed at this stop.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TaskInfo {
        /// The Task ID. This field won't be populated in the response of either a
        /// `GetTask`, or a `SearchTasks` call. Task IDs are subject to the following
        /// restrictions:
        ///
        /// * Must be a valid Unicode string.
        /// * Limited to a maximum length of 64 characters.
        /// * Normalized according to [Unicode Normalization Form C]
        /// (<http://www.unicode.org/reports/tr15/>).
        /// * May not contain any of the following ASCII characters: '/', ':', '?',
        /// ',', or '#'.
        #[prost(string, tag = "1")]
        pub task_id: ::prost::alloc::string::String,
        /// Output only. The time required to perform the Task.
        #[prost(message, optional, tag = "2")]
        pub task_duration: ::core::option::Option<::prost_types::Duration>,
        /// Output only. The time window during which the task should be completed.
        /// This is only set in the response to `GetDeliveryVehicle`.
        #[prost(message, optional, tag = "3")]
        pub target_time_window: ::core::option::Option<super::TimeWindow>,
    }
    /// The current state of a `VehicleStop`.
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
        /// Unknown.
        Unspecified = 0,
        /// Created, but not actively routing.
        New = 1,
        /// Assigned and actively routing.
        Enroute = 2,
        /// Arrived at stop. Assumes that when the Vehicle is routing to the next
        /// stop, that all previous stops have been completed.
        Arrived = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::New => "NEW",
                State::Enroute => "ENROUTE",
                State::Arrived => "ARRIVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "NEW" => Some(Self::New),
                "ENROUTE" => Some(Self::Enroute),
                "ARRIVED" => Some(Self::Arrived),
                _ => None,
            }
        }
    }
}
/// A Task in the Delivery API represents a single action to track. In general,
/// there is a distinction between shipment-related Tasks and break Tasks. A
/// shipment can have multiple Tasks associated with it. For example, there could
/// be one Task for the pickup, and one for the drop-off or transfer. Also,
/// different Tasks for a given shipment can be handled by different vehicles.
/// For example, one vehicle could handle the pickup, driving the shipment to the
/// hub, while another vehicle drives the same shipment from the hub to the
/// drop-off location.
///
/// Note: gRPC and REST APIs use different field naming conventions. For example,
/// the `Task.journey_sharing_info` field in the gRPC API and the
/// `DeliveryVehicle.journeySharingInfo` field in the REST API refer to the same
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// Must be in the format `providers/{provider}/tasks/{task}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. Defines the type of the Task. For example, a break or
    /// shipment.
    #[prost(enumeration = "task::Type", tag = "2")]
    pub r#type: i32,
    /// Required. The current execution state of the Task.
    #[prost(enumeration = "task::State", tag = "3")]
    pub state: i32,
    /// The outcome of the Task.
    #[prost(enumeration = "task::TaskOutcome", tag = "9")]
    pub task_outcome: i32,
    /// The timestamp that indicates when the `Task`'s outcome was set by the
    /// provider.
    #[prost(message, optional, tag = "10")]
    pub task_outcome_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The location where the `Task`'s outcome was set. This value is updated as
    /// part of `UpdateTask`. If this value isn't explicitly updated by the
    /// provider, then Fleet Engine populates it by default with the last known
    /// vehicle location (the *raw* location).
    #[prost(message, optional, tag = "11")]
    pub task_outcome_location: ::core::option::Option<LocationInfo>,
    /// Indicates where the value of the `task_outcome_location` came from.
    #[prost(enumeration = "task::TaskOutcomeLocationSource", tag = "12")]
    pub task_outcome_location_source: i32,
    /// Immutable. This field facilitates the storing of an ID so you can avoid
    /// using a complicated mapping. You cannot set `tracking_id` for Tasks of type
    /// `UNAVAILABLE` and `SCHEDULED_STOP`. These IDs are subject to the
    /// following restrictions:
    ///
    /// * Must be a valid Unicode string.
    /// * Limited to a maximum length of 64 characters.
    /// * Normalized according to [Unicode Normalization Form C]
    /// (<http://www.unicode.org/reports/tr15/>).
    /// * May not contain any of the following ASCII characters: '/', ':', '?',
    /// ',', or '#'.
    #[prost(string, tag = "4")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Output only. The ID of the vehicle that is executing this Task. Delivery
    /// Vehicle IDs are subject to the following restrictions:
    ///
    /// * Must be a valid Unicode string.
    /// * Limited to a maximum length of 64 characters.
    /// * Normalized according to [Unicode Normalization Form C]
    /// (<http://www.unicode.org/reports/tr15/>).
    /// * May not contain any of the following ASCII characters: '/', ':', '?',
    /// ',', or '#'.
    #[prost(string, tag = "5")]
    pub delivery_vehicle_id: ::prost::alloc::string::String,
    /// Immutable. The location where the Task will be completed.
    /// Optional for `UNAVAILABLE` Tasks, but required for all other Tasks.
    #[prost(message, optional, tag = "6")]
    pub planned_location: ::core::option::Option<LocationInfo>,
    /// Required. Immutable. The time needed to execute a Task at this location.
    #[prost(message, optional, tag = "7")]
    pub task_duration: ::core::option::Option<::prost_types::Duration>,
    /// The time window during which the task should be completed.
    #[prost(message, optional, tag = "14")]
    pub target_time_window: ::core::option::Option<TimeWindow>,
    /// Output only. Journey sharing-specific fields. Not populated when state is
    /// `CLOSED`.
    #[prost(message, optional, tag = "8")]
    pub journey_sharing_info: ::core::option::Option<task::JourneySharingInfo>,
    /// The configuration for task tracking that specifies which data elements are
    /// visible to the end users under what circumstances.
    #[prost(message, optional, tag = "13")]
    pub task_tracking_view_config: ::core::option::Option<TaskTrackingViewConfig>,
    /// A list of custom Task attributes. Each attribute must have a unique key.
    #[prost(message, repeated, tag = "15")]
    pub attributes: ::prost::alloc::vec::Vec<TaskAttribute>,
}
/// Nested message and enum types in `Task`.
pub mod task {
    /// Journey sharing specific fields.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct JourneySharingInfo {
        /// Tracking information for the stops that the assigned vehicle will make
        /// before it completes this Task. Note that this list can contain stops
        /// from other tasks.
        ///
        /// The first segment,
        /// `Task.journey_sharing_info.remaining_vehicle_journey_segments\[0\]` (gRPC)
        /// or `Task.journeySharingInfo.remainingVehicleJourneySegments\[0\]` (REST),
        /// contains route information from the driver's last known location to the
        /// upcoming `VehicleStop`. Current route information usually comes from the
        /// driver app, except for some cases noted in the documentation for
        /// \[DeliveryVehicle.current_route_segment][maps.fleetengine.delivery.v1.DeliveryVehicle.current_route_segment\].
        /// The other segments in
        /// `Task.journey_sharing_info.remaining_vehicle_journey_segments` (gRPC) or
        /// `Task.journeySharingInfo.remainingVehicleJourneySegments` (REST) are
        /// populated by Fleet Engine. They provide route information between the
        /// remaining `VehicleStops`.
        #[prost(message, repeated, tag = "1")]
        pub remaining_vehicle_journey_segments: ::prost::alloc::vec::Vec<
            super::VehicleJourneySegment,
        >,
        /// Indicates the vehicle's last reported location of the assigned vehicle.
        #[prost(message, optional, tag = "2")]
        pub last_location: ::core::option::Option<super::DeliveryVehicleLocation>,
        /// Indicates whether the vehicle's lastLocation can be snapped to
        /// the `current_route_segment`. This value is False if either
        /// `last_location` or `current_route_segment` don't exist. This value is
        /// computed by Fleet Engine. Updates from clients are ignored.
        #[prost(bool, tag = "3")]
        pub last_location_snappable: bool,
    }
    /// The type of Task.
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
    pub enum Type {
        /// Default, the Task type is unknown.
        Unspecified = 0,
        /// A pickup Task is the action taken for picking up a shipment from a
        /// customer. Depot or feeder vehicle pickups should use the `SCHEDULED_STOP`
        /// type.
        Pickup = 1,
        /// A delivery Task is the action taken for delivering a shipment to an end
        /// customer. Depot or feeder vehicle dropoffs should use the
        /// `SCHEDULED_STOP` type.
        Delivery = 2,
        /// A scheduled stop Task is used for planning purposes. For example, it
        /// could represent picking up or dropping off shipments from feeder vehicles
        /// or depots. It shouldn't be used for any shipments that are picked up or
        /// dropped off from an end customer.
        ScheduledStop = 3,
        /// A Task that means the Vehicle is not available for service. For example,
        /// this can happen when the driver takes a break, or when the vehicle
        /// is being refueled.
        Unavailable = 4,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Pickup => "PICKUP",
                Type::Delivery => "DELIVERY",
                Type::ScheduledStop => "SCHEDULED_STOP",
                Type::Unavailable => "UNAVAILABLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PICKUP" => Some(Self::Pickup),
                "DELIVERY" => Some(Self::Delivery),
                "SCHEDULED_STOP" => Some(Self::ScheduledStop),
                "UNAVAILABLE" => Some(Self::Unavailable),
                _ => None,
            }
        }
    }
    /// The state of a Task. This indicates the Tasks's progress.
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
        /// Default. Used for an unspecified or unrecognized Task state.
        Unspecified = 0,
        /// Either the Task has not yet been assigned to a delivery vehicle, or the
        /// delivery vehicle has not yet passed the `Task`'s assigned vehicle stop.
        Open = 1,
        /// When the vehicle passes the vehicle stop for this Task.
        Closed = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Open => "OPEN",
                State::Closed => "CLOSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "OPEN" => Some(Self::Open),
                "CLOSED" => Some(Self::Closed),
                _ => None,
            }
        }
    }
    /// The outcome of attempting to execute a Task. When `TaskState` is closed,
    /// `TaskOutcome` indicates whether it was completed successfully.
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
    pub enum TaskOutcome {
        /// The Task outcome before its value is set.
        Unspecified = 0,
        /// The Task completed successfully.
        Succeeded = 1,
        /// Either the Task couldn't be completed, or it was cancelled.
        Failed = 2,
    }
    impl TaskOutcome {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TaskOutcome::Unspecified => "TASK_OUTCOME_UNSPECIFIED",
                TaskOutcome::Succeeded => "SUCCEEDED",
                TaskOutcome::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TASK_OUTCOME_UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// The identity of the source that populated the `task_outcome_location`.
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
    pub enum TaskOutcomeLocationSource {
        /// The task outcome before it is set.
        Unspecified = 0,
        /// The provider-specified the `task_outcome_location`.
        Provider = 2,
        /// The provider didn't specify the `task_outcome_location`, so Fleet Engine
        /// used the last known vehicle location.
        LastVehicleLocation = 3,
    }
    impl TaskOutcomeLocationSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TaskOutcomeLocationSource::Unspecified => {
                    "TASK_OUTCOME_LOCATION_SOURCE_UNSPECIFIED"
                }
                TaskOutcomeLocationSource::Provider => "PROVIDER",
                TaskOutcomeLocationSource::LastVehicleLocation => "LAST_VEHICLE_LOCATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TASK_OUTCOME_LOCATION_SOURCE_UNSPECIFIED" => Some(Self::Unspecified),
                "PROVIDER" => Some(Self::Provider),
                "LAST_VEHICLE_LOCATION" => Some(Self::LastVehicleLocation),
                _ => None,
            }
        }
    }
}
/// The configuration message that defines when a data element of a Task should
/// be visible to the end users.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskTrackingViewConfig {
    /// The field that specifies when route polyline points can be visible. If this
    /// field is not specified, the project level default visibility configuration
    /// for this data will be used.
    #[prost(message, optional, tag = "1")]
    pub route_polyline_points_visibility: ::core::option::Option<
        task_tracking_view_config::VisibilityOption,
    >,
    /// The field that specifies when estimated arrival time can be visible. If
    /// this field is not specified, the project level default visibility
    /// configuration for this data will be used.
    #[prost(message, optional, tag = "2")]
    pub estimated_arrival_time_visibility: ::core::option::Option<
        task_tracking_view_config::VisibilityOption,
    >,
    /// The field that specifies when estimated task completion time can be
    /// visible. If this field is not specified, the project level default
    /// visibility configuration for this data will be used.
    #[prost(message, optional, tag = "3")]
    pub estimated_task_completion_time_visibility: ::core::option::Option<
        task_tracking_view_config::VisibilityOption,
    >,
    /// The field that specifies when remaining driving distance can be visible. If
    /// this field is not specified, the project level default visibility
    /// configuration for this data will be used.
    #[prost(message, optional, tag = "4")]
    pub remaining_driving_distance_visibility: ::core::option::Option<
        task_tracking_view_config::VisibilityOption,
    >,
    /// The field that specifies when remaining stop count can be visible. If this
    /// field is not specified, the project level default visibility configuration
    /// for this data will be used.
    #[prost(message, optional, tag = "5")]
    pub remaining_stop_count_visibility: ::core::option::Option<
        task_tracking_view_config::VisibilityOption,
    >,
    /// The field that specifies when vehicle location can be visible. If this
    /// field is not specified, the project level default visibility configuration
    /// for this data will be used.
    #[prost(message, optional, tag = "6")]
    pub vehicle_location_visibility: ::core::option::Option<
        task_tracking_view_config::VisibilityOption,
    >,
}
/// Nested message and enum types in `TaskTrackingViewConfig`.
pub mod task_tracking_view_config {
    /// The option message that defines when a data element should be visible to
    /// the end users.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VisibilityOption {
        #[prost(oneof = "visibility_option::VisibilityOption", tags = "1, 2, 3, 4, 5")]
        pub visibility_option: ::core::option::Option<
            visibility_option::VisibilityOption,
        >,
    }
    /// Nested message and enum types in `VisibilityOption`.
    pub mod visibility_option {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum VisibilityOption {
            /// This data element is visible to the end users if the remaining stop
            /// count <= remaining_stop_count_threshold.
            #[prost(int32, tag = "1")]
            RemainingStopCountThreshold(i32),
            /// This data element is visible to the end users if the ETA to the stop
            /// <= duration_until_estimated_arrival_time_threshold.
            #[prost(message, tag = "2")]
            DurationUntilEstimatedArrivalTimeThreshold(::prost_types::Duration),
            /// This data element is visible to the end users if the remaining
            /// driving distance in meters <=
            /// remaining_driving_distance_meters_threshold.
            #[prost(int32, tag = "3")]
            RemainingDrivingDistanceMetersThreshold(i32),
            /// If set to true, this data element is always visible to the end users
            /// with no thresholds. This field cannot be set to false.
            #[prost(bool, tag = "4")]
            Always(bool),
            /// If set to true, this data element is always hidden from the end users
            /// with no thresholds. This field cannot be set to false.
            #[prost(bool, tag = "5")]
            Never(bool),
        }
    }
}
/// The `TaskTrackingInfo` message. The message contains task tracking
/// information which will be used for display. If a tracking ID is associated
/// with multiple Tasks, Fleet Engine uses a heuristic to decide which Task's
/// TaskTrackingInfo to select.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskTrackingInfo {
    /// Must be in the format `providers/{provider}/taskTrackingInfo/{tracking}`,
    /// where `tracking` represents the tracking ID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The tracking ID of a Task.
    /// * Must be a valid Unicode string.
    /// * Limited to a maximum length of 64 characters.
    /// * Normalized according to [Unicode Normalization Form C]
    /// (<http://www.unicode.org/reports/tr15/>).
    /// * May not contain any of the following ASCII characters: '/', ':', '?',
    /// ',', or '#'.
    #[prost(string, tag = "2")]
    pub tracking_id: ::prost::alloc::string::String,
    /// The vehicle's last location.
    #[prost(message, optional, tag = "3")]
    pub vehicle_location: ::core::option::Option<DeliveryVehicleLocation>,
    /// A list of points which when connected forms a polyline of the vehicle's
    /// expected route to the location of this task.
    #[prost(message, repeated, tag = "4")]
    pub route_polyline_points: ::prost::alloc::vec::Vec<
        super::super::super::super::google::r#type::LatLng,
    >,
    /// Indicates the number of stops the vehicle remaining until the task stop is
    /// reached, including the task stop. For example, if the vehicle's next stop
    /// is the task stop, the value will be 1.
    #[prost(message, optional, tag = "5")]
    pub remaining_stop_count: ::core::option::Option<i32>,
    /// The total remaining distance in meters to the `VehicleStop` of interest.
    #[prost(message, optional, tag = "6")]
    pub remaining_driving_distance_meters: ::core::option::Option<i32>,
    /// The timestamp that indicates the estimated arrival time to the stop
    /// location.
    #[prost(message, optional, tag = "7")]
    pub estimated_arrival_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The timestamp that indicates the estimated completion time of a Task.
    #[prost(message, optional, tag = "8")]
    pub estimated_task_completion_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The current execution state of the Task.
    #[prost(enumeration = "task::State", tag = "11")]
    pub state: i32,
    /// The outcome of attempting to execute a Task.
    #[prost(enumeration = "task::TaskOutcome", tag = "9")]
    pub task_outcome: i32,
    /// The timestamp that indicates when the Task's outcome was set by the
    /// provider.
    #[prost(message, optional, tag = "12")]
    pub task_outcome_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. The location where the Task will be completed.
    #[prost(message, optional, tag = "10")]
    pub planned_location: ::core::option::Option<LocationInfo>,
    /// The time window during which the task should be completed.
    #[prost(message, optional, tag = "13")]
    pub target_time_window: ::core::option::Option<TimeWindow>,
    /// The custom attributes set on the task.
    #[prost(message, repeated, tag = "14")]
    pub attributes: ::prost::alloc::vec::Vec<TaskAttribute>,
}
/// The `CreateDeliveryVehicle` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDeliveryVehicleRequest {
    /// Optional. The standard Delivery API request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<DeliveryRequestHeader>,
    /// Required. Must be in the format `providers/{provider}`. The provider must
    /// be the Google Cloud Project ID. For example, `sample-cloud-project`.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Delivery Vehicle ID must be unique and subject to the
    /// following restrictions:
    ///
    /// * Must be a valid Unicode string.
    /// * Limited to a maximum length of 64 characters.
    /// * Normalized according to [Unicode Normalization Form C]
    /// (<http://www.unicode.org/reports/tr15/>).
    /// * May not contain any of the following ASCII characters: '/', ':', '?',
    /// ',', or '#'.
    #[prost(string, tag = "4")]
    pub delivery_vehicle_id: ::prost::alloc::string::String,
    /// Required. The `DeliveryVehicle` entity to create. When creating a new
    /// delivery vehicle, you may set the following optional fields:
    ///
    /// * last_location
    /// * attributes
    ///
    /// Note: The DeliveryVehicle's `name` field is ignored. All other
    /// DeliveryVehicle fields must not be set; otherwise, an error is returned.
    #[prost(message, optional, tag = "5")]
    pub delivery_vehicle: ::core::option::Option<DeliveryVehicle>,
}
/// The `GetDeliveryVehicle` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeliveryVehicleRequest {
    /// Optional. The standard Delivery API request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<DeliveryRequestHeader>,
    /// Required. Must be in the format
    /// `providers/{provider}/deliveryVehicles/{delivery_vehicle}`.
    /// The `provider` must be the Google Cloud Project ID. For example,
    /// `sample-cloud-project`.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// The `ListDeliveryVehicles` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeliveryVehiclesRequest {
    /// Optional. The standard Delivery API request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<DeliveryRequestHeader>,
    /// Required. Must be in the format `providers/{provider}`.
    /// The `provider` must be the Google Cloud Project ID.
    /// For example, `sample-cloud-project`.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of vehicles to return. The service may return
    /// fewer than this number. If you don't specify this number, then the server
    /// determines the number of results to return.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListDeliveryVehicles`
    /// call. You must provide this in order to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListDeliveryVehicles`
    /// must match the call that provided the page token.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A filter query to apply when listing delivery vehicles. See
    /// <http://aip.dev/160> for examples of the filter syntax. If you don't specify
    /// a value, or if you specify an empty string for the filter, then all
    /// delivery vehicles are returned.
    ///
    /// Note that the only queries supported for `ListDeliveryVehicles` are
    /// on vehicle attributes (for example, `attributes.<key> = <value>` or
    /// `attributes.<key1> = <value1> AND attributes.<key2> = <value2>`). Also, all
    /// attributes are stored as strings, so the only supported comparisons against
    /// attributes are string comparisons. In order to compare against number or
    /// boolean values, the values must be explicitly quoted to be treated as
    /// strings (for example, `attributes.<key> = "10"` or
    /// `attributes.<key> = "true"`).
    ///
    /// The maximum number of restrictions allowed in a filter query is 50. A
    /// restriction is a part of the query of the form
    /// `attribute.<KEY> <COMPARATOR> <VALUE>`, for example `attributes.foo = bar`
    /// is 1 restriction.
    #[prost(string, tag = "6")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. A filter that limits the vehicles returned to those whose last
    /// known location was in the rectangular area defined by the viewport.
    #[prost(message, optional, tag = "7")]
    pub viewport: ::core::option::Option<
        super::super::super::super::google::geo::r#type::Viewport,
    >,
}
/// The `ListDeliveryVehicles` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeliveryVehiclesResponse {
    /// The set of delivery vehicles that meet the requested filtering criteria.
    /// When no filter is specified, the request returns all delivery vehicles. A
    /// successful response can also be empty. An empty response indicates that no
    /// delivery vehicles were found meeting the requested filter criteria.
    #[prost(message, repeated, tag = "1")]
    pub delivery_vehicles: ::prost::alloc::vec::Vec<DeliveryVehicle>,
    /// You can pass this token in the `ListDeliveryVehiclesRequest` to continue to
    /// list results. When all of the results are returned, this field won't be in
    /// the response, or it will be an empty string.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of delivery vehicles that match the request criteria,
    /// across all pages.
    #[prost(int64, tag = "3")]
    pub total_size: i64,
}
/// The `UpdateDeliveryVehicle` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDeliveryVehicleRequest {
    /// Optional. The standard Delivery API request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<DeliveryRequestHeader>,
    /// Required. The `DeliveryVehicle` entity update to apply.
    /// Note: You cannot update the name of the `DeliveryVehicle`.
    #[prost(message, optional, tag = "3")]
    pub delivery_vehicle: ::core::option::Option<DeliveryVehicle>,
    /// Required. A field mask that indicates which `DeliveryVehicle` fields to
    /// update. Note that the update_mask must contain at least one field.
    ///
    /// This is a comma-separated list of fully qualified names of fields. Example:
    /// `"remaining_vehicle_journey_segments"`.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The `BatchCreateTask` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateTasksRequest {
    /// Optional. The standard Delivery API request header.
    /// Note: If you set this field, then the header field in the
    /// `CreateTaskRequest` messages must either be empty, or it must match this
    /// field.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<DeliveryRequestHeader>,
    /// Required. The parent resource shared by all tasks. This value must be in
    /// the format `providers/{provider}`. The `provider` must be the Google Cloud
    /// Project ID. For example, `sample-cloud-project`. The parent field in the
    /// `CreateTaskRequest` messages must either  be empty, or it must match this
    /// field.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request message that specifies the resources to create.
    /// Note: You can create a maximum of 500 tasks in a batch.
    #[prost(message, repeated, tag = "4")]
    pub requests: ::prost::alloc::vec::Vec<CreateTaskRequest>,
}
/// The `BatchCreateTask` response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateTasksResponse {
    /// The created Tasks.
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
}
/// The `CreateTask` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaskRequest {
    /// Optional. The standard Delivery API request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<DeliveryRequestHeader>,
    /// Required. Must be in the format `providers/{provider}`. The `provider` must
    /// be the Google Cloud Project ID. For example, `sample-cloud-project`.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Task ID must be unique, but it should be not a shipment
    /// tracking ID. To store a shipment tracking ID, use the `tracking_id` field.
    /// Note that multiple tasks can have the same `tracking_id`. Task IDs are
    /// subject to the following restrictions:
    ///
    /// * Must be a valid Unicode string.
    /// * Limited to a maximum length of 64 characters.
    /// * Normalized according to [Unicode Normalization Form C]
    /// (<http://www.unicode.org/reports/tr15/>).
    /// * May not contain any of the following ASCII characters: '/', ':', '?',
    /// ',', or '#'.
    #[prost(string, tag = "5")]
    pub task_id: ::prost::alloc::string::String,
    /// Required. The Task entity to create.
    /// When creating a Task, the following fields are required:
    ///
    /// * `type`
    /// * `state` (must be set to `OPEN`)
    /// * `tracking_id` (must not be set for `UNAVAILABLE` or `SCHEDULED_STOP`
    /// tasks, but required for all other task types)
    /// * `planned_location` (optional for `UNAVAILABLE` tasks)
    /// * `task_duration`
    ///
    /// Note: The Task's `name` field is ignored. All other Task fields must not be
    /// set; otherwise, an error is returned.
    #[prost(message, optional, tag = "4")]
    pub task: ::core::option::Option<Task>,
}
/// The `GetTask` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskRequest {
    /// Optional. The standard Delivery API request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<DeliveryRequestHeader>,
    /// Required. Must be in the format `providers/{provider}/tasks/{task}`. The
    /// `provider` must be the Google Cloud Project ID. For example,
    /// `sample-cloud-project`.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// The `SearchTasks` request message that contains the `tracking_id`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTasksRequest {
    /// Optional. The standard Delivery API request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<DeliveryRequestHeader>,
    /// Required. Must be in the format `providers/{provider}`.
    /// The provider must be the Google Cloud Project ID. For example,
    /// `sample-cloud-project`.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The identifier of the set of related Tasks being requested.
    /// Tracking IDs are subject to the following restrictions:
    ///
    /// * Must be a valid Unicode string.
    /// * Limited to a maximum length of 64 characters.
    /// * Normalized according to [Unicode Normalization Form C]
    /// (<http://www.unicode.org/reports/tr15/>).
    /// * May not contain any of the following ASCII characters: '/', ':', '?',
    /// ',', or '#'.
    #[prost(string, tag = "4")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Optional. The maximum number of Tasks to return. The service may return
    /// fewer than this value. If you don't specify this value, then the server
    /// determines the number of results to return.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `SearchTasks` call. You
    /// must provide this value to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `SearchTasks` must match
    /// the call that provided the page token.
    #[prost(string, tag = "6")]
    pub page_token: ::prost::alloc::string::String,
}
/// The `SearchTasks` response. It contains the set of Tasks that meet the search
/// criteria in the `SearchTasksRequest`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTasksResponse {
    /// The set of Tasks for the requested `tracking_id`. A successful response can
    /// also be empty. An empty response indicates that no Tasks are associated
    /// with the supplied `tracking_id`.
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
    /// Pass this token in the `SearchTasksRequest` to continue to
    /// list results. If all results have been returned, then this field is either
    /// an empty string, or it doesn't appear in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The `UpdateTask` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaskRequest {
    /// Optional. The standard Delivery API request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<DeliveryRequestHeader>,
    /// Required. The Task associated with the update.
    /// The following fields are maintained by Fleet Engine. Do not update
    /// them using `Task.update`.
    ///
    ///    * `last_location`.
    ///    * `last_location_snappable`.
    ///    * `name`.
    ///    * `remaining_vehicle_journey_segments`.
    ///    * `task_outcome_location_source`.
    ///
    /// Note: You cannot change the value of `task_outcome` once you set it.
    ///
    /// If the Task has been assigned to a delivery vehicle, then don't set the
    /// Task state to CLOSED using `Task.update`. Instead, remove the `VehicleStop`
    /// that contains the Task from the delivery vehicle, which automatically sets
    /// the Task state to CLOSED.
    #[prost(message, optional, tag = "3")]
    pub task: ::core::option::Option<Task>,
    /// Required. The field mask that indicates which Task fields to update.
    /// Note: The `update_mask` must contain at least one field.
    ///
    /// This is a comma-separated list of fully qualified names of fields. Example:
    /// `"task_outcome,task_outcome_time,task_outcome_location"`.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The `ListTasks` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksRequest {
    /// Optional. The standard Delivery API request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<DeliveryRequestHeader>,
    /// Required. Must be in the format `providers/{provider}`.
    /// The `provider` must be the Google Cloud Project ID. For example,
    /// `sample-cloud-project`.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of Tasks to return. The service may return
    /// fewer than this value. If you don't specify this value, then the server
    /// determines the number of results to return.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. A page token received from a previous `ListTasks` call.
    /// You can provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListTasks` must match
    /// the call that provided the page token.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A filter query to apply when listing Tasks. See
    /// <http://aip.dev/160> for examples of filter syntax. If you don't specify a
    /// value, or if you filter on an empty string, then all Tasks are returned.
    /// For information about the Task properties that you can filter on, see [Task
    /// list](/maps/documentation/transportation-logistics/last-mile-fleet-solution/fleet-performance/fleet-engine/deliveries_api#list_tasks).
    #[prost(string, tag = "6")]
    pub filter: ::prost::alloc::string::String,
}
/// The `ListTasks` response that contains the set of Tasks that meet the filter
/// criteria in the `ListTasksRequest`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksResponse {
    /// The set of Tasks that meet the requested filtering criteria. When no filter
    /// is specified, the request returns all tasks. A successful response can also
    /// be empty. An empty response indicates that no Tasks were found meeting the
    /// requested filter criteria.
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
    /// Pass this token in the `ListTasksRequest` to continue to list results.
    /// If all results have been returned, then this field is either an empty
    /// string, or it doesn't appear in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of Tasks that match the request criteria, across all
    /// pages.
    #[prost(int64, tag = "3")]
    pub total_size: i64,
}
/// The `GetTaskTrackingInfoRequest` request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskTrackingInfoRequest {
    /// Optional. The standard Delivery API request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<DeliveryRequestHeader>,
    /// Required. Must be in the format
    /// `providers/{provider}/taskTrackingInfo/{tracking_id}`. The `provider`
    /// must be the Google Cloud Project ID, and the `tracking_id` must be the
    /// tracking ID associated with the task. An example name can be
    /// `providers/sample-cloud-project/taskTrackingInfo/sample-tracking-id`.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod delivery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Last Mile Delivery service.
    #[derive(Debug, Clone)]
    pub struct DeliveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DeliveryServiceClient<T>
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
        ) -> DeliveryServiceClient<InterceptedService<T, F>>
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
            DeliveryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates and returns a new `DeliveryVehicle`.
        pub async fn create_delivery_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDeliveryVehicleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeliveryVehicle>,
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
                "/maps.fleetengine.delivery.v1.DeliveryService/CreateDeliveryVehicle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "maps.fleetengine.delivery.v1.DeliveryService",
                        "CreateDeliveryVehicle",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the specified `DeliveryVehicle` instance.
        pub async fn get_delivery_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeliveryVehicleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeliveryVehicle>,
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
                "/maps.fleetengine.delivery.v1.DeliveryService/GetDeliveryVehicle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "maps.fleetengine.delivery.v1.DeliveryService",
                        "GetDeliveryVehicle",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Writes updated `DeliveryVehicle` data to Fleet Engine, and assigns
        /// `Tasks` to the `DeliveryVehicle`. You cannot update the name of the
        /// `DeliveryVehicle`. You *can* update `remaining_vehicle_journey_segments`
        /// though, but it must contain all of the `VehicleJourneySegment`s currently
        /// on the `DeliveryVehicle`. The `task_id`s are retrieved from
        /// `remaining_vehicle_journey_segments`, and their corresponding `Tasks` are
        /// assigned to the `DeliveryVehicle` if they have not yet been assigned.
        pub async fn update_delivery_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDeliveryVehicleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeliveryVehicle>,
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
                "/maps.fleetengine.delivery.v1.DeliveryService/UpdateDeliveryVehicle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "maps.fleetengine.delivery.v1.DeliveryService",
                        "UpdateDeliveryVehicle",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates and returns a batch of new `Task` objects.
        pub async fn batch_create_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateTasksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateTasksResponse>,
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
                "/maps.fleetengine.delivery.v1.DeliveryService/BatchCreateTasks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "maps.fleetengine.delivery.v1.DeliveryService",
                        "BatchCreateTasks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates and returns a new `Task` object.
        pub async fn create_task(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTaskRequest>,
        ) -> std::result::Result<tonic::Response<super::Task>, tonic::Status> {
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
                "/maps.fleetengine.delivery.v1.DeliveryService/CreateTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "maps.fleetengine.delivery.v1.DeliveryService",
                        "CreateTask",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets information about a `Task`.
        pub async fn get_task(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTaskRequest>,
        ) -> std::result::Result<tonic::Response<super::Task>, tonic::Status> {
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
                "/maps.fleetengine.delivery.v1.DeliveryService/GetTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "maps.fleetengine.delivery.v1.DeliveryService",
                        "GetTask",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets all `Task`s with a particular `tracking_id`.
        pub async fn search_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchTasksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchTasksResponse>,
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
                "/maps.fleetengine.delivery.v1.DeliveryService/SearchTasks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "maps.fleetengine.delivery.v1.DeliveryService",
                        "SearchTasks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates `Task` data.
        pub async fn update_task(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTaskRequest>,
        ) -> std::result::Result<tonic::Response<super::Task>, tonic::Status> {
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
                "/maps.fleetengine.delivery.v1.DeliveryService/UpdateTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "maps.fleetengine.delivery.v1.DeliveryService",
                        "UpdateTask",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets all `Task`s that meet the specified filtering criteria.
        pub async fn list_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTasksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTasksResponse>,
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
                "/maps.fleetengine.delivery.v1.DeliveryService/ListTasks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "maps.fleetengine.delivery.v1.DeliveryService",
                        "ListTasks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the specified `TaskTrackingInfo` instance.
        pub async fn get_task_tracking_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTaskTrackingInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TaskTrackingInfo>,
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
                "/maps.fleetengine.delivery.v1.DeliveryService/GetTaskTrackingInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "maps.fleetengine.delivery.v1.DeliveryService",
                        "GetTaskTrackingInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets all `DeliveryVehicle`s that meet the specified filtering criteria.
        pub async fn list_delivery_vehicles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeliveryVehiclesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDeliveryVehiclesResponse>,
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
                "/maps.fleetengine.delivery.v1.DeliveryService/ListDeliveryVehicles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "maps.fleetengine.delivery.v1.DeliveryService",
                        "ListDeliveryVehicles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
