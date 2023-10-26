/// The price represented as a number and currency.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    /// The price represented as a number in micros (1 million micros is an
    /// equivalent to one's currency standard unit, for example, 1 USD = 1000000
    /// micros).
    /// This field can also be set as infinity by setting to -1.
    /// This field only support -1 and positive value.
    #[prost(int64, optional, tag = "1")]
    pub amount_micros: ::core::option::Option<i64>,
    /// The currency of the price using three-letter acronyms according to [ISO
    /// 4217](<http://en.wikipedia.org/wiki/ISO_4217>).
    #[prost(string, optional, tag = "2")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
}
/// A message that represents custom attributes. Exactly one of `value` or
/// `group_values` must not be empty.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAttribute {
    /// The name of the attribute.
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The value of the attribute. If `value` is not empty, `group_values` must be
    /// empty.
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// Subattributes within this attribute group.  If
    /// `group_values` is not empty, `value` must be empty.
    #[prost(message, repeated, tag = "3")]
    pub group_values: ::prost::alloc::vec::Vec<CustomAttribute>,
}
/// Destinations available for a product.
///
/// Destinations are used in Merchant Center to allow you to control where the
/// products from your data feed should be displayed.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Destination {}
/// Nested message and enum types in `Destination`.
pub mod destination {
    /// Destination values.
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
    pub enum DestinationEnum {
        /// Not specified.
        Unspecified = 0,
        /// [Shopping ads](<https://support.google.com/google-ads/answer/2454022>).
        ShoppingAds = 1,
        /// [Display ads](<https://support.google.com/merchants/answer/6069387>).
        DisplayAds = 2,
        /// [Local inventory
        /// ads](<https://support.google.com/merchants/answer/3057972>).
        LocalInventoryAds = 3,
        /// [Free listings](<https://support.google.com/merchants/answer/9199328>).
        FreeListings = 4,
        /// [Free local product
        /// listings](<https://support.google.com/merchants/answer/9825611>).
        FreeLocalListings = 5,
        /// [YouTube Shopping](<https://support.google.com/merchants/answer/12362804>).
        YoutubeShopping = 6,
    }
    impl DestinationEnum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DestinationEnum::Unspecified => "DESTINATION_ENUM_UNSPECIFIED",
                DestinationEnum::ShoppingAds => "SHOPPING_ADS",
                DestinationEnum::DisplayAds => "DISPLAY_ADS",
                DestinationEnum::LocalInventoryAds => "LOCAL_INVENTORY_ADS",
                DestinationEnum::FreeListings => "FREE_LISTINGS",
                DestinationEnum::FreeLocalListings => "FREE_LOCAL_LISTINGS",
                DestinationEnum::YoutubeShopping => "YOUTUBE_SHOPPING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DESTINATION_ENUM_UNSPECIFIED" => Some(Self::Unspecified),
                "SHOPPING_ADS" => Some(Self::ShoppingAds),
                "DISPLAY_ADS" => Some(Self::DisplayAds),
                "LOCAL_INVENTORY_ADS" => Some(Self::LocalInventoryAds),
                "FREE_LISTINGS" => Some(Self::FreeListings),
                "FREE_LOCAL_LISTINGS" => Some(Self::FreeLocalListings),
                "YOUTUBE_SHOPPING" => Some(Self::YoutubeShopping),
                _ => None,
            }
        }
    }
}
/// Reporting contexts that your account and product issues apply to.
///
/// Reporting contexts are groups of surfaces and formats for product results on
/// Google. They can represent the entire destination (for example, [Shopping
/// ads](<https://support.google.com/merchants/answer/6149970>)) or a subset of
/// formats within a destination (for example, [Discovery
/// ads](<https://support.google.com/merchants/answer/13389785>)).
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportingContext {}
/// Nested message and enum types in `ReportingContext`.
pub mod reporting_context {
    /// Reporting context values.
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
    pub enum ReportingContextEnum {
        /// Not specified.
        Unspecified = 0,
        /// [Shopping ads](<https://support.google.com/merchants/answer/6149970>).
        ShoppingAds = 1,
        /// [Discovery and Demand Gen
        /// ads](<https://support.google.com/merchants/answer/13389785>).
        DiscoveryAds = 2,
        /// [Video ads](<https://support.google.com/google-ads/answer/6340491>).
        VideoAds = 3,
        /// [Display ads](<https://support.google.com/merchants/answer/6069387>).
        DisplayAds = 4,
        /// [Local inventory
        /// ads](<https://support.google.com/merchants/answer/3271956>).
        LocalInventoryAds = 5,
        /// [Vehicle inventory
        /// ads](<https://support.google.com/merchants/answer/11544533>).
        VehicleInventoryAds = 6,
        /// [Free product
        /// listings](<https://support.google.com/merchants/answer/9199328>).
        FreeListings = 7,
        /// [Free local product
        /// listings](<https://support.google.com/merchants/answer/9825611>).
        FreeLocalListings = 8,
        /// [Free local vehicle
        /// listings](<https://support.google.com/merchants/answer/11544533>).
        FreeLocalVehicleListings = 9,
        /// [YouTube
        /// Shopping](<https://support.google.com/merchants/answer/13478370>).
        YoutubeShopping = 10,
        /// [Cloud retail](<https://cloud.google.com/solutions/retail>).
        CloudRetail = 11,
        /// [Local cloud retail](<https://cloud.google.com/solutions/retail>).
        LocalCloudRetail = 12,
    }
    impl ReportingContextEnum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReportingContextEnum::Unspecified => "REPORTING_CONTEXT_ENUM_UNSPECIFIED",
                ReportingContextEnum::ShoppingAds => "SHOPPING_ADS",
                ReportingContextEnum::DiscoveryAds => "DISCOVERY_ADS",
                ReportingContextEnum::VideoAds => "VIDEO_ADS",
                ReportingContextEnum::DisplayAds => "DISPLAY_ADS",
                ReportingContextEnum::LocalInventoryAds => "LOCAL_INVENTORY_ADS",
                ReportingContextEnum::VehicleInventoryAds => "VEHICLE_INVENTORY_ADS",
                ReportingContextEnum::FreeListings => "FREE_LISTINGS",
                ReportingContextEnum::FreeLocalListings => "FREE_LOCAL_LISTINGS",
                ReportingContextEnum::FreeLocalVehicleListings => {
                    "FREE_LOCAL_VEHICLE_LISTINGS"
                }
                ReportingContextEnum::YoutubeShopping => "YOUTUBE_SHOPPING",
                ReportingContextEnum::CloudRetail => "CLOUD_RETAIL",
                ReportingContextEnum::LocalCloudRetail => "LOCAL_CLOUD_RETAIL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REPORTING_CONTEXT_ENUM_UNSPECIFIED" => Some(Self::Unspecified),
                "SHOPPING_ADS" => Some(Self::ShoppingAds),
                "DISCOVERY_ADS" => Some(Self::DiscoveryAds),
                "VIDEO_ADS" => Some(Self::VideoAds),
                "DISPLAY_ADS" => Some(Self::DisplayAds),
                "LOCAL_INVENTORY_ADS" => Some(Self::LocalInventoryAds),
                "VEHICLE_INVENTORY_ADS" => Some(Self::VehicleInventoryAds),
                "FREE_LISTINGS" => Some(Self::FreeListings),
                "FREE_LOCAL_LISTINGS" => Some(Self::FreeLocalListings),
                "FREE_LOCAL_VEHICLE_LISTINGS" => Some(Self::FreeLocalVehicleListings),
                "YOUTUBE_SHOPPING" => Some(Self::YoutubeShopping),
                "CLOUD_RETAIL" => Some(Self::CloudRetail),
                "LOCAL_CLOUD_RETAIL" => Some(Self::LocalCloudRetail),
                _ => None,
            }
        }
    }
}
/// [Channel](<https://support.google.com/merchants/answer/7361332>) of a product.
///
/// Channel is used to distinguish between online and local products.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {}
/// Nested message and enum types in `Channel`.
pub mod channel {
    /// Channel values.
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
    pub enum ChannelEnum {
        /// Not specified.
        Unspecified = 0,
        /// Online product.
        Online = 1,
        /// Local product.
        Local = 2,
    }
    impl ChannelEnum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ChannelEnum::Unspecified => "CHANNEL_ENUM_UNSPECIFIED",
                ChannelEnum::Online => "ONLINE",
                ChannelEnum::Local => "LOCAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CHANNEL_ENUM_UNSPECIFIED" => Some(Self::Unspecified),
                "ONLINE" => Some(Self::Online),
                "LOCAL" => Some(Self::Local),
                _ => None,
            }
        }
    }
}
