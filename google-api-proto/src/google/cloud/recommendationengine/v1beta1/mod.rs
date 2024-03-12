/// FeatureMap represents extra features that customers want to include in the
/// recommendation model for catalogs/user events as categorical/numerical
/// features.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureMap {
    /// Categorical features that can take on one of a limited number of possible
    /// values. Some examples would be the brand/maker of a product, or country of
    /// a customer.
    ///
    /// Feature names and values must be UTF-8 encoded strings.
    ///
    /// For example: `{ "colors": {"value": \["yellow", "green"\]},
    ///                  "sizes": {"value":\["S", "M"\]}`
    #[prost(btree_map = "string, message", tag = "1")]
    pub categorical_features: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        feature_map::StringList,
    >,
    /// Numerical features. Some examples would be the height/weight of a product,
    /// or age of a customer.
    ///
    /// Feature names must be UTF-8 encoded strings.
    ///
    /// For example: `{ "lengths_cm": {"value":\[2.3, 15.4\]},
    ///                  "heights_cm": {"value":\[8.1, 6.4\]} }`
    #[prost(btree_map = "string, message", tag = "2")]
    pub numerical_features: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        feature_map::FloatList,
    >,
}
/// Nested message and enum types in `FeatureMap`.
pub mod feature_map {
    /// A list of string features.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringList {
        /// String feature value with a length limit of 128 bytes.
        #[prost(string, repeated, tag = "1")]
        pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// A list of float features.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FloatList {
        /// Float feature value.
        #[prost(float, repeated, tag = "1")]
        pub value: ::prost::alloc::vec::Vec<f32>,
    }
}
/// CatalogItem captures all metadata information of items to be recommended.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CatalogItem {
    /// Required. Catalog item identifier. UTF-8 encoded string with a length limit
    /// of 128 bytes.
    ///
    /// This id must be unique among all catalog items within the same catalog. It
    /// should also be used when logging user events in order for the user events
    /// to be joined with the Catalog.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Required. Catalog item categories. This field is repeated for supporting
    /// one catalog item belonging to several parallel category hierarchies.
    ///
    /// For example, if a shoes product belongs to both
    /// \["Shoes & Accessories" -> "Shoes"\] and
    /// \["Sports & Fitness" -> "Athletic Clothing" -> "Shoes"\], it could be
    /// represented as:
    ///
    ///       "categoryHierarchies": [
    ///         { "categories": \["Shoes & Accessories", "Shoes"\]},
    ///         { "categories": \["Sports & Fitness", "Athletic Clothing", "Shoes"\] }
    ///       ]
    #[prost(message, repeated, tag = "2")]
    pub category_hierarchies: ::prost::alloc::vec::Vec<catalog_item::CategoryHierarchy>,
    /// Required. Catalog item title. UTF-8 encoded string with a length limit of 1
    /// KiB.
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// Optional. Catalog item description. UTF-8 encoded string with a length
    /// limit of 5 KiB.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Highly encouraged. Extra catalog item attributes to be
    /// included in the recommendation model. For example, for retail products,
    /// this could include the store name, vendor, style, color, etc. These are
    /// very strong signals for recommendation model, thus we highly recommend
    /// providing the item attributes here.
    #[prost(message, optional, tag = "5")]
    pub item_attributes: ::core::option::Option<FeatureMap>,
    /// Optional. Language of the title/description/item_attributes. Use language
    /// tags defined by BCP 47. <https://www.rfc-editor.org/rfc/bcp/bcp47.txt.> Our
    /// supported language codes include 'en', 'es', 'fr', 'de', 'ar', 'fa', 'zh',
    /// 'ja', 'ko', 'sv', 'ro', 'nl'. For other languages, contact
    /// your Google account manager.
    #[prost(string, tag = "6")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. Filtering tags associated with the catalog item. Each tag should
    /// be a UTF-8 encoded string with a length limit of 1 KiB.
    ///
    /// This tag can be used for filtering recommendation results by passing the
    /// tag as part of the predict request filter.
    #[prost(string, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Variant group identifier for prediction results. UTF-8 encoded
    /// string with a length limit of 128 bytes.
    ///
    /// This field must be enabled before it can be used. [Learn
    /// more](/recommendations-ai/docs/catalog#item-group-id).
    #[prost(string, tag = "9")]
    pub item_group_id: ::prost::alloc::string::String,
    /// Extra catalog item metadata for different recommendation types.
    #[prost(oneof = "catalog_item::RecommendationType", tags = "10")]
    pub recommendation_type: ::core::option::Option<catalog_item::RecommendationType>,
}
/// Nested message and enum types in `CatalogItem`.
pub mod catalog_item {
    /// Category represents catalog item category hierarchy.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CategoryHierarchy {
        /// Required. Catalog item categories. Each category should be a UTF-8
        /// encoded string with a length limit of 2 KiB.
        ///
        /// Note that the order in the list denotes the specificity (from least to
        /// most specific).
        #[prost(string, repeated, tag = "1")]
        pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Extra catalog item metadata for different recommendation types.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RecommendationType {
        /// Optional. Metadata specific to retail products.
        #[prost(message, tag = "10")]
        ProductMetadata(super::ProductCatalogItem),
    }
}
/// ProductCatalogItem captures item metadata specific to retail products.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductCatalogItem {
    /// Optional. A map to pass the costs associated with the product.
    ///
    /// For example:
    /// {"manufacturing": 45.5} The profit of selling this item is computed like
    /// so:
    ///
    /// * If 'exactPrice' is provided, profit = displayPrice - sum(costs)
    /// * If 'priceRange' is provided, profit = minPrice - sum(costs)
    #[prost(btree_map = "string, float", tag = "3")]
    pub costs: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        f32,
    >,
    /// Optional. Only required if the price is set. Currency code for price/costs. Use
    /// three-character ISO-4217 code.
    #[prost(string, tag = "4")]
    pub currency_code: ::prost::alloc::string::String,
    /// Optional. Online stock state of the catalog item. Default is `IN_STOCK`.
    #[prost(enumeration = "product_catalog_item::StockState", tag = "5")]
    pub stock_state: i32,
    /// Optional. The available quantity of the item.
    #[prost(int64, tag = "6")]
    pub available_quantity: i64,
    /// Optional. Canonical URL directly linking to the item detail page with a
    /// length limit of 5 KiB..
    #[prost(string, tag = "7")]
    pub canonical_product_uri: ::prost::alloc::string::String,
    /// Optional. Product images for the catalog item.
    #[prost(message, repeated, tag = "8")]
    pub images: ::prost::alloc::vec::Vec<Image>,
    /// Product price. Only one of 'exactPrice'/'priceRange' can be provided.
    #[prost(oneof = "product_catalog_item::Price", tags = "1, 2")]
    pub price: ::core::option::Option<product_catalog_item::Price>,
}
/// Nested message and enum types in `ProductCatalogItem`.
pub mod product_catalog_item {
    /// Exact product price.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExactPrice {
        /// Optional. Display price of the product.
        #[prost(float, tag = "1")]
        pub display_price: f32,
        /// Optional. Price of the product without any discount. If zero, by default
        /// set to be the 'displayPrice'.
        #[prost(float, tag = "2")]
        pub original_price: f32,
    }
    /// Product price range when there are a range of prices for different
    /// variations of the same product.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PriceRange {
        /// Required. The minimum product price.
        #[prost(float, tag = "1")]
        pub min: f32,
        /// Required. The maximum product price.
        #[prost(float, tag = "2")]
        pub max: f32,
    }
    /// Item stock state. If this field is unspecified, the item is
    /// assumed to be in stock.
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
    pub enum StockState {
        /// Default item stock status. Should never be used.
        Unspecified = 0,
        /// Item out of stock.
        OutOfStock = 1,
        /// Item that is in pre-order state.
        Preorder = 2,
        /// Item that is back-ordered (i.e. temporarily out of stock).
        Backorder = 3,
    }
    impl StockState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StockState::Unspecified => "STOCK_STATE_UNSPECIFIED",
                StockState::OutOfStock => "OUT_OF_STOCK",
                StockState::Preorder => "PREORDER",
                StockState::Backorder => "BACKORDER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STOCK_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "OUT_OF_STOCK" => Some(Self::OutOfStock),
                "PREORDER" => Some(Self::Preorder),
                "BACKORDER" => Some(Self::Backorder),
                _ => None,
            }
        }
    }
    /// Product price. Only one of 'exactPrice'/'priceRange' can be provided.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Price {
        /// Optional. The exact product price.
        #[prost(message, tag = "1")]
        ExactPrice(ExactPrice),
        /// Optional. The product price range.
        #[prost(message, tag = "2")]
        PriceRange(PriceRange),
    }
}
/// Catalog item thumbnail/detail image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// Required. URL of the image with a length limit of 5 KiB.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// Optional. Height of the image in number of pixels.
    #[prost(int32, tag = "2")]
    pub height: i32,
    /// Optional. Width of the image in number of pixels.
    #[prost(int32, tag = "3")]
    pub width: i32,
}
/// UserEvent captures all metadata information recommendation engine needs to
/// know about how end users interact with customers' website.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEvent {
    /// Required. User event type. Allowed values are:
    ///
    /// * `add-to-cart` Products being added to cart.
    /// * `add-to-list` Items being added to a list (shopping list, favorites
    ///    etc).
    /// * `category-page-view` Special pages such as sale or promotion pages
    ///    viewed.
    /// * `checkout-start` User starting a checkout process.
    /// * `detail-page-view` Products detail page viewed.
    /// * `home-page-view` Homepage viewed.
    /// * `page-visit` Generic page visits not included in the event types above.
    /// * `purchase-complete` User finishing a purchase.
    /// * `refund` Purchased items being refunded or returned.
    /// * `remove-from-cart` Products being removed from cart.
    /// * `remove-from-list` Items being removed from a list.
    /// * `search` Product search.
    /// * `shopping-cart-page-view` User viewing a shopping cart.
    /// * `impression` List of items displayed. Used by Google Tag Manager.
    #[prost(string, tag = "1")]
    pub event_type: ::prost::alloc::string::String,
    /// Required. User information.
    #[prost(message, optional, tag = "2")]
    pub user_info: ::core::option::Option<UserInfo>,
    /// Optional. User event detailed information common across different
    /// recommendation types.
    #[prost(message, optional, tag = "3")]
    pub event_detail: ::core::option::Option<EventDetail>,
    /// Optional. Retail product specific user event metadata.
    ///
    /// This field is required for the following event types:
    ///
    /// * `add-to-cart`
    /// * `add-to-list`
    /// * `category-page-view`
    /// * `checkout-start`
    /// * `detail-page-view`
    /// * `purchase-complete`
    /// * `refund`
    /// * `remove-from-cart`
    /// * `remove-from-list`
    /// * `search`
    ///
    /// This field is optional for the following event types:
    ///
    /// * `page-visit`
    /// * `shopping-cart-page-view` - note that 'product_event_detail' should be
    ///    set for this unless the shopping cart is empty.
    ///
    /// This field is not allowed for the following event types:
    ///
    /// * `home-page-view`
    #[prost(message, optional, tag = "4")]
    pub product_event_detail: ::core::option::Option<ProductEventDetail>,
    /// Optional. Only required for ImportUserEvents method. Timestamp of user
    /// event created.
    #[prost(message, optional, tag = "5")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. This field should *not* be set when using JavaScript pixel
    /// or the Recommendations AI Tag. Defaults to `EVENT_SOURCE_UNSPECIFIED`.
    #[prost(enumeration = "user_event::EventSource", tag = "6")]
    pub event_source: i32,
}
/// Nested message and enum types in `UserEvent`.
pub mod user_event {
    /// User event source.
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
    pub enum EventSource {
        /// Unspecified event source.
        Unspecified = 0,
        /// The event is ingested via a javascript pixel or Recommendations AI Tag
        /// through automl datalayer or JS Macros.
        Automl = 1,
        /// The event is ingested via Recommendations AI Tag through Enhanced
        /// Ecommerce datalayer.
        Ecommerce = 2,
        /// The event is ingested via Import user events API.
        BatchUpload = 3,
    }
    impl EventSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventSource::Unspecified => "EVENT_SOURCE_UNSPECIFIED",
                EventSource::Automl => "AUTOML",
                EventSource::Ecommerce => "ECOMMERCE",
                EventSource::BatchUpload => "BATCH_UPLOAD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EVENT_SOURCE_UNSPECIFIED" => Some(Self::Unspecified),
                "AUTOML" => Some(Self::Automl),
                "ECOMMERCE" => Some(Self::Ecommerce),
                "BATCH_UPLOAD" => Some(Self::BatchUpload),
                _ => None,
            }
        }
    }
}
/// Information of end users.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// Required. A unique identifier for tracking visitors with a length limit of
    /// 128 bytes.
    ///
    /// For example, this could be implemented with a http cookie, which should be
    /// able to uniquely identify a visitor on a single device. This unique
    /// identifier should not change if the visitor log in/out of the website.
    /// Maximum length 128 bytes. Cannot be empty.
    #[prost(string, tag = "1")]
    pub visitor_id: ::prost::alloc::string::String,
    /// Optional. Unique identifier for logged-in user with a length limit of 128
    /// bytes. Required only for logged-in users.
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// Optional. IP address of the user. This could be either IPv4 (e.g. 104.133.9.80) or
    /// IPv6 (e.g. 2001:0db8:85a3:0000:0000:8a2e:0370:7334). This should *not* be
    /// set when using the javascript pixel or if `direct_user_request` is set.
    /// Used to extract location information for personalization.
    #[prost(string, tag = "3")]
    pub ip_address: ::prost::alloc::string::String,
    /// Optional. User agent as included in the HTTP header. UTF-8 encoded string
    /// with a length limit of 1 KiB.
    ///
    /// This should *not* be set when using the JavaScript pixel or if
    /// `directUserRequest` is set.
    #[prost(string, tag = "4")]
    pub user_agent: ::prost::alloc::string::String,
    /// Optional. Indicates if the request is made directly from the end user
    /// in which case the user_agent and ip_address fields can be populated
    /// from the HTTP request. This should *not* be set when using the javascript
    /// pixel. This flag should be set only if the API request is made directly
    /// from the end user such as a mobile app (and not if a gateway or a server is
    /// processing and pushing the user events).
    #[prost(bool, tag = "5")]
    pub direct_user_request: bool,
}
/// User event details shared by all recommendation types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDetail {
    /// Optional. Complete url (window.location.href) of the user's current page.
    /// When using the JavaScript pixel, this value is filled in automatically.
    /// Maximum length 5KB.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// Optional. The referrer url of the current page. When using
    /// the JavaScript pixel, this value is filled in automatically.
    #[prost(string, tag = "6")]
    pub referrer_uri: ::prost::alloc::string::String,
    /// Optional. A unique id of a web page view.
    /// This should be kept the same for all user events triggered from the same
    /// pageview. For example, an item detail page view could trigger multiple
    /// events as the user is browsing the page.
    /// The `pageViewId` property should be kept the same for all these events so
    /// that they can be grouped together properly. This `pageViewId` will be
    /// automatically generated if using the JavaScript pixel.
    #[prost(string, tag = "2")]
    pub page_view_id: ::prost::alloc::string::String,
    /// Optional. A list of identifiers for the independent experiment groups
    /// this user event belongs to. This is used to distinguish between user events
    /// associated with different experiment setups (e.g. using Recommendation
    /// Engine system, using different recommendation models).
    #[prost(string, repeated, tag = "3")]
    pub experiment_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Recommendation token included in the recommendation prediction
    /// response.
    ///
    /// This field enables accurate attribution of recommendation model
    /// performance.
    ///
    /// This token enables us to accurately attribute page view or purchase back to
    /// the event and the particular predict response containing this
    /// clicked/purchased item. If user clicks on product K in the recommendation
    /// results, pass the `PredictResponse.recommendationToken` property as a url
    /// parameter to product K's page. When recording events on product K's page,
    /// log the PredictResponse.recommendation_token to this field.
    ///
    /// Optional, but highly encouraged for user events that are the result of a
    /// recommendation prediction query.
    #[prost(string, tag = "4")]
    pub recommendation_token: ::prost::alloc::string::String,
    /// Optional. Extra user event features to include in the recommendation
    /// model.
    ///
    /// For product recommendation, an example of extra user information is
    /// traffic_channel, i.e. how user arrives at the site. Users can arrive
    /// at the site by coming to the site directly, or coming through Google
    /// search, and etc.
    #[prost(message, optional, tag = "5")]
    pub event_attributes: ::core::option::Option<FeatureMap>,
}
/// ProductEventDetail captures user event information specific to retail
/// products.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductEventDetail {
    /// Required for `search` events. Other event types should not set this field.
    /// The user's search query as UTF-8 encoded text with a length limit of 5 KiB.
    #[prost(string, tag = "1")]
    pub search_query: ::prost::alloc::string::String,
    /// Required for `category-page-view` events. Other event types should not set
    /// this field.
    /// The categories associated with a category page.
    /// Category pages include special pages such as sales or promotions. For
    /// instance, a special sale page may have the category hierarchy:
    /// categories : \["Sales", "2017 Black Friday Deals"\].
    #[prost(message, repeated, tag = "2")]
    pub page_categories: ::prost::alloc::vec::Vec<catalog_item::CategoryHierarchy>,
    /// The main product details related to the event.
    ///
    /// This field is required for the following event types:
    ///
    /// * `add-to-cart`
    /// * `add-to-list`
    /// * `checkout-start`
    /// * `detail-page-view`
    /// * `purchase-complete`
    /// * `refund`
    /// * `remove-from-cart`
    /// * `remove-from-list`
    ///
    /// This field is optional for the following event types:
    ///
    /// * `page-visit`
    /// * `shopping-cart-page-view` - note that 'product_details' should be set for
    ///    this unless the shopping cart is empty.
    ///
    /// This field is not allowed for the following event types:
    ///
    /// * `category-page-view`
    /// * `home-page-view`
    /// * `search`
    #[prost(message, repeated, tag = "3")]
    pub product_details: ::prost::alloc::vec::Vec<ProductDetail>,
    /// Required for `add-to-list` and `remove-from-list` events. The id or name of
    /// the list that the item is being added to or removed from. Other event types
    /// should not set this field.
    #[prost(string, tag = "4")]
    pub list_id: ::prost::alloc::string::String,
    /// Optional. The id or name of the associated shopping cart. This id is used
    /// to associate multiple items added or present in the cart before purchase.
    ///
    /// This can only be set for `add-to-cart`, `remove-from-cart`,
    /// `checkout-start`, `purchase-complete`, or `shopping-cart-page-view` events.
    #[prost(string, tag = "5")]
    pub cart_id: ::prost::alloc::string::String,
    /// Optional. A transaction represents the entire purchase transaction.
    /// Required for `purchase-complete` events. Optional for `checkout-start`
    /// events. Other event types should not set this field.
    #[prost(message, optional, tag = "6")]
    pub purchase_transaction: ::core::option::Option<PurchaseTransaction>,
}
/// A transaction represents the entire purchase transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchaseTransaction {
    /// Optional. The transaction ID with a length limit of 128 bytes.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Required. Total revenue or grand total associated with the transaction.
    /// This value include shipping, tax, or other adjustments to total revenue
    /// that you want to include as part of your revenue calculations. This field
    /// is not required if the event type is `refund`.
    #[prost(float, tag = "2")]
    pub revenue: f32,
    /// Optional. All the taxes associated with the transaction.
    #[prost(btree_map = "string, float", tag = "3")]
    pub taxes: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        f32,
    >,
    /// Optional. All the costs associated with the product. These can be
    /// manufacturing costs, shipping expenses not borne by the end user, or any
    /// other costs.
    ///
    /// Total product cost such that
    ///    profit = revenue - (sum(taxes) + sum(costs))
    /// If product_cost is not set, then
    ///    profit = revenue - tax - shipping - sum(CatalogItem.costs).
    ///
    /// If CatalogItem.cost is not specified for one of the items, CatalogItem.cost
    /// based profit *cannot* be calculated for this Transaction.
    #[prost(btree_map = "string, float", tag = "4")]
    pub costs: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        f32,
    >,
    /// Required. Currency code. Use three-character ISO-4217 code. This field
    /// is not required if the event type is `refund`.
    #[prost(string, tag = "6")]
    pub currency_code: ::prost::alloc::string::String,
}
/// Detailed product information associated with a user event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductDetail {
    /// Required. Catalog item ID. UTF-8 encoded string with a length limit of 128
    /// characters.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Optional. Currency code for price/costs. Use three-character ISO-4217
    /// code. Required only if originalPrice or displayPrice is set.
    #[prost(string, tag = "2")]
    pub currency_code: ::prost::alloc::string::String,
    /// Optional. Original price of the product. If provided, this will override
    /// the original price in Catalog for this product.
    #[prost(float, tag = "3")]
    pub original_price: f32,
    /// Optional. Display price of the product (e.g. discounted price). If
    /// provided, this will override the display price in Catalog for this product.
    #[prost(float, tag = "4")]
    pub display_price: f32,
    /// Optional. Item stock state. If provided, this overrides the stock state
    /// in Catalog for items in this event.
    #[prost(enumeration = "product_catalog_item::StockState", tag = "5")]
    pub stock_state: i32,
    /// Optional. Quantity of the product associated with the user event. For
    /// example, this field will be 2 if two products are added to the shopping
    /// cart for `add-to-cart` event. Required for `add-to-cart`, `add-to-list`,
    /// `remove-from-cart`, `checkout-start`, `purchase-complete`, `refund` event
    /// types.
    #[prost(int32, tag = "6")]
    pub quantity: i32,
    /// Optional. Quantity of the products in stock when a user event happens.
    /// Optional. If provided, this overrides the available quantity in Catalog for
    /// this event. and can only be set if `stock_status` is set to `IN_STOCK`.
    ///
    /// Note that if an item is out of stock, you must set the `stock_state` field
    /// to be `OUT_OF_STOCK`. Leaving this field unspecified / as zero is not
    /// sufficient to mark the item out of stock.
    #[prost(int32, tag = "7")]
    pub available_quantity: i32,
    /// Optional. Extra features associated with a product in the user event.
    #[prost(message, optional, tag = "8")]
    pub item_attributes: ::core::option::Option<FeatureMap>,
}
/// Google Cloud Storage location for input content.
/// format.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Required. Google Cloud Storage URIs to input files. URI can be up to
    /// 2000 characters long. URIs can match the full object path (for example,
    /// `gs://bucket/directory/object.json`) or a pattern matching one or more
    /// files, such as `gs://bucket/directory/*.json`. A request can
    /// contain at most 100 files, and each file can be up to 2 GB. See
    /// [Importing catalog information](/recommendations-ai/docs/upload-catalog)
    /// for the expected file format and setup instructions.
    #[prost(string, repeated, tag = "1")]
    pub input_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The inline source for the input config for ImportCatalogItems method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CatalogInlineSource {
    /// Optional. A list of catalog items to update/create. Recommended max of 10k
    /// items.
    #[prost(message, repeated, tag = "1")]
    pub catalog_items: ::prost::alloc::vec::Vec<CatalogItem>,
}
/// The inline source for the input config for ImportUserEvents method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEventInlineSource {
    /// Optional. A list of user events to import. Recommended max of 10k items.
    #[prost(message, repeated, tag = "1")]
    pub user_events: ::prost::alloc::vec::Vec<UserEvent>,
}
/// Configuration of destination for Import related errors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportErrorsConfig {
    /// Required. Errors destination.
    #[prost(oneof = "import_errors_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<import_errors_config::Destination>,
}
/// Nested message and enum types in `ImportErrorsConfig`.
pub mod import_errors_config {
    /// Required. Errors destination.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Google Cloud Storage path for import errors. This must be an empty,
        /// existing Cloud Storage bucket. Import errors will be written to a file in
        /// this bucket, one per line, as a JSON-encoded
        /// `google.rpc.Status` message.
        #[prost(string, tag = "1")]
        GcsPrefix(::prost::alloc::string::String),
    }
}
/// Request message for Import methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportCatalogItemsRequest {
    /// Required. `projects/1234/locations/global/catalogs/default_catalog`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Unique identifier provided by client, within the ancestor
    /// dataset scope. Ensures idempotency and used for request deduplication.
    /// Server-generated if unspecified. Up to 128 characters long. This is
    /// returned as google.longrunning.Operation.name in the response.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Required. The desired input location of the data.
    #[prost(message, optional, tag = "3")]
    pub input_config: ::core::option::Option<InputConfig>,
    /// Optional. The desired location of errors incurred during the Import.
    #[prost(message, optional, tag = "4")]
    pub errors_config: ::core::option::Option<ImportErrorsConfig>,
}
/// Request message for the ImportUserEvents request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsRequest {
    /// Required.
    /// `projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Unique identifier provided by client, within the ancestor
    /// dataset scope. Ensures idempotency for expensive long running operations.
    /// Server-generated if unspecified. Up to 128 characters long. This is
    /// returned as google.longrunning.Operation.name in the response. Note that
    /// this field must not be set if the desired input config is
    /// catalog_inline_source.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Required. The desired input location of the data.
    #[prost(message, optional, tag = "3")]
    pub input_config: ::core::option::Option<InputConfig>,
    /// Optional. The desired location of errors incurred during the Import.
    #[prost(message, optional, tag = "4")]
    pub errors_config: ::core::option::Option<ImportErrorsConfig>,
}
/// The input config source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputConfig {
    /// Required. The source of the input.
    #[prost(oneof = "input_config::Source", tags = "1, 2, 3")]
    pub source: ::core::option::Option<input_config::Source>,
}
/// Nested message and enum types in `InputConfig`.
pub mod input_config {
    /// Required. The source of the input.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Inline source for the input content for Catalog items.
        #[prost(message, tag = "1")]
        CatalogInlineSource(super::CatalogInlineSource),
        /// Google Cloud Storage location for the input content.
        #[prost(message, tag = "2")]
        GcsSource(super::GcsSource),
        /// The Inline source for the input content for UserEvents.
        #[prost(message, tag = "3")]
        UserEventInlineSource(super::UserEventInlineSource),
    }
}
/// Metadata related to the progress of the Import operation. This will be
/// returned by the google.longrunning.Operation.metadata field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportMetadata {
    /// Name of the operation.
    #[prost(string, tag = "5")]
    pub operation_name: ::prost::alloc::string::String,
    /// Id of the request / operation. This is parroting back the requestId that
    /// was passed in the request.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Operation create time.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Count of entries that were processed successfully.
    #[prost(int64, tag = "1")]
    pub success_count: i64,
    /// Count of entries that encountered errors while processing.
    #[prost(int64, tag = "2")]
    pub failure_count: i64,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Response of the ImportCatalogItemsRequest. If the long running
/// operation is done, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportCatalogItemsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors in the request if set.
    #[prost(message, optional, tag = "2")]
    pub errors_config: ::core::option::Option<ImportErrorsConfig>,
}
/// Response of the ImportUserEventsRequest. If the long running
/// operation was successful, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors if this field was set in
    /// the request.
    #[prost(message, optional, tag = "2")]
    pub errors_config: ::core::option::Option<ImportErrorsConfig>,
    /// Aggregated statistics of user event import status.
    #[prost(message, optional, tag = "3")]
    pub import_summary: ::core::option::Option<UserEventImportSummary>,
}
/// A summary of import result. The UserEventImportSummary summarizes
/// the import status for user events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEventImportSummary {
    /// Count of user events imported with complete existing catalog information.
    #[prost(int64, tag = "1")]
    pub joined_events_count: i64,
    /// Count of user events imported, but with catalog information not found
    /// in the imported catalog.
    #[prost(int64, tag = "2")]
    pub unjoined_events_count: i64,
}
/// Request message for CreateCatalogItem method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCatalogItemRequest {
    /// Required. The parent catalog resource name, such as
    /// `projects/*/locations/global/catalogs/default_catalog`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The catalog item to create.
    #[prost(message, optional, tag = "2")]
    pub catalog_item: ::core::option::Option<CatalogItem>,
}
/// Request message for GetCatalogItem method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCatalogItemRequest {
    /// Required. Full resource name of catalog item, such as
    /// `projects/*/locations/global/catalogs/default_catalog/catalogitems/some_catalog_item_id`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListCatalogItems method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogItemsRequest {
    /// Required. The parent catalog resource name, such as
    /// `projects/*/locations/global/catalogs/default_catalog`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of results to return per page. If zero, the
    /// service will choose a reasonable default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The previous ListCatalogItemsResponse.next_page_token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A filter to apply on the list results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for ListCatalogItems method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogItemsResponse {
    /// The catalog items.
    #[prost(message, repeated, tag = "1")]
    pub catalog_items: ::prost::alloc::vec::Vec<CatalogItem>,
    /// If empty, the list is complete. If nonempty, the token to pass to the next
    /// request's ListCatalogItemRequest.page_token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for UpdateCatalogItem method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCatalogItemRequest {
    /// Required. Full resource name of catalog item, such as
    /// `projects/*/locations/global/catalogs/default_catalog/catalogItems/some_catalog_item_id`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The catalog item to update/create. The 'catalog_item_id' field
    /// has to match that in the 'name'.
    #[prost(message, optional, tag = "2")]
    pub catalog_item: ::core::option::Option<CatalogItem>,
    /// Optional. Indicates which fields in the provided 'item' to update. If not
    /// set, will by default update all fields.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteCatalogItem method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCatalogItemRequest {
    /// Required. Full resource name of catalog item, such as
    /// `projects/*/locations/global/catalogs/default_catalog/catalogItems/some_catalog_item_id`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod catalog_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for ingesting catalog information of the customer's website.
    #[derive(Debug, Clone)]
    pub struct CatalogServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CatalogServiceClient<T>
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
        ) -> CatalogServiceClient<InterceptedService<T, F>>
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
            CatalogServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a catalog item.
        pub async fn create_catalog_item(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCatalogItemRequest>,
        ) -> std::result::Result<tonic::Response<super::CatalogItem>, tonic::Status> {
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
                "/google.cloud.recommendationengine.v1beta1.CatalogService/CreateCatalogItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.CatalogService",
                        "CreateCatalogItem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a specific catalog item.
        pub async fn get_catalog_item(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCatalogItemRequest>,
        ) -> std::result::Result<tonic::Response<super::CatalogItem>, tonic::Status> {
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
                "/google.cloud.recommendationengine.v1beta1.CatalogService/GetCatalogItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.CatalogService",
                        "GetCatalogItem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a list of catalog items.
        pub async fn list_catalog_items(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCatalogItemsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCatalogItemsResponse>,
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
                "/google.cloud.recommendationengine.v1beta1.CatalogService/ListCatalogItems",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.CatalogService",
                        "ListCatalogItems",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a catalog item. Partial updating is supported. Non-existing
        /// items will be created.
        pub async fn update_catalog_item(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCatalogItemRequest>,
        ) -> std::result::Result<tonic::Response<super::CatalogItem>, tonic::Status> {
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
                "/google.cloud.recommendationengine.v1beta1.CatalogService/UpdateCatalogItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.CatalogService",
                        "UpdateCatalogItem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a catalog item.
        pub async fn delete_catalog_item(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCatalogItemRequest>,
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
                "/google.cloud.recommendationengine.v1beta1.CatalogService/DeleteCatalogItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.CatalogService",
                        "DeleteCatalogItem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Bulk import of multiple catalog items. Request processing may be
        /// synchronous. No partial updating supported. Non-existing items will be
        /// created.
        ///
        /// Operation.response is of type ImportResponse. Note that it is
        /// possible for a subset of the items to be successfully updated.
        pub async fn import_catalog_items(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportCatalogItemsRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.recommendationengine.v1beta1.CatalogService/ImportCatalogItems",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.CatalogService",
                        "ImportCatalogItems",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Registered Api Key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictionApiKeyRegistration {
    /// The API key.
    #[prost(string, tag = "1")]
    pub api_key: ::prost::alloc::string::String,
}
/// Request message for the `CreatePredictionApiKeyRegistration` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePredictionApiKeyRegistrationRequest {
    /// Required. The parent resource path.
    /// `projects/*/locations/global/catalogs/default_catalog/eventStores/default_event_store`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The prediction API key registration.
    #[prost(message, optional, tag = "2")]
    pub prediction_api_key_registration: ::core::option::Option<
        PredictionApiKeyRegistration,
    >,
}
/// Request message for the `ListPredictionApiKeyRegistrations`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPredictionApiKeyRegistrationsRequest {
    /// Required. The parent placement resource name such as
    /// `projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of results to return per page. If unset, the
    /// service will choose a reasonable default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The previous `ListPredictionApiKeyRegistration.nextPageToken`.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the `ListPredictionApiKeyRegistrations`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPredictionApiKeyRegistrationsResponse {
    /// The list of registered API keys.
    #[prost(message, repeated, tag = "1")]
    pub prediction_api_key_registrations: ::prost::alloc::vec::Vec<
        PredictionApiKeyRegistration,
    >,
    /// If empty, the list is complete. If nonempty, pass the token to the next
    /// request's `ListPredictionApiKeysRegistrationsRequest.pageToken`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `DeletePredictionApiKeyRegistration` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePredictionApiKeyRegistrationRequest {
    /// Required. The API key to unregister including full resource path.
    /// `projects/*/locations/global/catalogs/default_catalog/eventStores/default_event_store/predictionApiKeyRegistrations/<YOUR_API_KEY>`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod prediction_api_key_registry_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for registering API keys for use with the `predict` method. If you
    /// use an API key to request predictions, you must first register the API key.
    /// Otherwise, your prediction request is rejected. If you use OAuth to
    /// authenticate your `predict` method call, you do not need to register an API
    /// key. You can register up to 20 API keys per project.
    #[derive(Debug, Clone)]
    pub struct PredictionApiKeyRegistryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PredictionApiKeyRegistryClient<T>
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
        ) -> PredictionApiKeyRegistryClient<InterceptedService<T, F>>
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
            PredictionApiKeyRegistryClient::new(
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
        /// Register an API key for use with predict method.
        pub async fn create_prediction_api_key_registration(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreatePredictionApiKeyRegistrationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::PredictionApiKeyRegistration>,
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
                "/google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry/CreatePredictionApiKeyRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry",
                        "CreatePredictionApiKeyRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List the registered apiKeys for use with predict method.
        pub async fn list_prediction_api_key_registrations(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListPredictionApiKeyRegistrationsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListPredictionApiKeyRegistrationsResponse>,
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
                "/google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry/ListPredictionApiKeyRegistrations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry",
                        "ListPredictionApiKeyRegistrations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Unregister an apiKey from using for predict method.
        pub async fn delete_prediction_api_key_registration(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DeletePredictionApiKeyRegistrationRequest,
            >,
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
                "/google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry/DeletePredictionApiKeyRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry",
                        "DeletePredictionApiKeyRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request message for Predict method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictRequest {
    /// Required. Full resource name of the format:
    /// `{name=projects/*/locations/global/catalogs/default_catalog/eventStores/default_event_store/placements/*}`
    /// The id of the recommendation engine placement. This id is used to identify
    /// the set of models that will be used to make the prediction.
    ///
    /// We currently support three placements with the following IDs by default:
    ///
    /// * `shopping_cart`: Predicts items frequently bought together with one or
    ///    more catalog items in the same shopping session. Commonly displayed after
    ///    `add-to-cart` events, on product detail pages, or on the shopping cart
    ///    page.
    ///
    /// * `home_page`: Predicts the next product that a user will most likely
    ///    engage with or purchase based on the shopping or viewing history of the
    ///    specified `userId` or `visitorId`. For example - Recommendations for you.
    ///
    /// * `product_detail`: Predicts the next product that a user will most likely
    ///    engage with or purchase. The prediction is based on the shopping or
    ///    viewing history of the specified `userId` or `visitorId` and its
    ///    relevance to a specified `CatalogItem`. Typically used on product detail
    ///    pages. For example - More items like this.
    ///
    /// * `recently_viewed_default`: Returns up to 75 items recently viewed by the
    ///    specified `userId` or `visitorId`, most recent ones first. Returns
    ///    nothing if neither of them has viewed any items yet. For example -
    ///    Recently viewed.
    ///
    /// The full list of available placements can be seen at
    /// <https://console.cloud.google.com/recommendation/datafeeds/default_catalog/dashboard>
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Context about the user, what they are looking at and what action
    /// they took to trigger the predict request. Note that this user event detail
    /// won't be ingested to userEvent logs. Thus, a separate userEvent write
    /// request is required for event logging.
    #[prost(message, optional, tag = "2")]
    pub user_event: ::core::option::Option<UserEvent>,
    /// Optional. Maximum number of results to return per page. Set this property
    /// to the number of prediction results required. If zero, the service will
    /// choose a reasonable default.
    #[prost(int32, tag = "7")]
    pub page_size: i32,
    /// Optional. The previous PredictResponse.next_page_token.
    #[prost(string, tag = "8")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter for restricting prediction results. Accepts values for
    /// tags and the `filterOutOfStockItems` flag.
    ///
    ///   * Tag expressions. Restricts predictions to items that match all of the
    ///     specified tags. Boolean operators `OR` and `NOT` are supported if the
    ///     expression is enclosed in parentheses, and must be separated from the
    ///     tag values by a space. `-"tagA"` is also supported and is equivalent to
    ///     `NOT "tagA"`. Tag values must be double quoted UTF-8 encoded strings
    ///     with a size limit of 1 KiB.
    ///
    ///   * filterOutOfStockItems. Restricts predictions to items that do not have a
    ///     stockState value of OUT_OF_STOCK.
    ///
    /// Examples:
    ///
    ///   * tag=("Red" OR "Blue") tag="New-Arrival" tag=(NOT "promotional")
    ///   * filterOutOfStockItems  tag=(-"promotional")
    ///   * filterOutOfStockItems
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Use dryRun mode for this prediction query. If set to true, a
    /// dummy model will be used that returns arbitrary catalog items.
    /// Note that the dryRun mode should only be used for testing the API, or if
    /// the model is not ready.
    #[prost(bool, tag = "4")]
    pub dry_run: bool,
    /// Optional. Additional domain specific parameters for the predictions.
    ///
    /// Allowed values:
    ///
    /// * `returnCatalogItem`: Boolean. If set to true, the associated catalogItem
    ///     object will be returned in the
    ///    `PredictResponse.PredictionResult.itemMetadata` object in the method
    ///     response.
    /// * `returnItemScore`: Boolean. If set to true, the prediction 'score'
    ///     corresponding to each returned item will be set in the `metadata`
    ///     field in the prediction response. The given 'score' indicates the
    ///     probability of an item being clicked/purchased given the user's context
    ///     and history.
    #[prost(btree_map = "string, message", tag = "6")]
    pub params: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
    /// Optional. The labels for the predict request.
    ///
    ///   * Label keys can contain lowercase letters, digits and hyphens, must start
    ///     with a letter, and must end with a letter or digit.
    ///   * Non-zero label values can contain lowercase letters, digits and hyphens,
    ///     must start with a letter, and must end with a letter or digit.
    ///   * No more than 64 labels can be associated with a given request.
    ///
    /// See <https://goo.gl/xmQnxf> for more information on and examples of labels.
    #[prost(btree_map = "string, string", tag = "9")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Response message for predict method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictResponse {
    /// A list of recommended items. The order represents the ranking (from the
    /// most relevant item to the least).
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<predict_response::PredictionResult>,
    /// A unique recommendation token. This should be included in the user event
    /// logs resulting from this recommendation, which enables accurate attribution
    /// of recommendation model performance.
    #[prost(string, tag = "2")]
    pub recommendation_token: ::prost::alloc::string::String,
    /// IDs of items in the request that were missing from the catalog.
    #[prost(string, repeated, tag = "3")]
    pub items_missing_in_catalog: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// True if the dryRun property was set in the request.
    #[prost(bool, tag = "4")]
    pub dry_run: bool,
    /// Additional domain specific prediction response metadata.
    #[prost(btree_map = "string, message", tag = "5")]
    pub metadata: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
    /// If empty, the list is complete. If nonempty, the token to pass to the next
    /// request's PredictRequest.page_token.
    #[prost(string, tag = "6")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PredictResponse`.
pub mod predict_response {
    /// PredictionResult represents the recommendation prediction results.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PredictionResult {
        /// ID of the recommended catalog item
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Additional item metadata / annotations.
        ///
        /// Possible values:
        ///
        /// * `catalogItem`: JSON representation of the catalogItem. Will be set if
        ///    `returnCatalogItem` is set to true in `PredictRequest.params`.
        /// * `score`: Prediction score in double value. Will be set if
        ///    `returnItemScore` is set to true in `PredictRequest.params`.
        #[prost(btree_map = "string, message", tag = "2")]
        pub item_metadata: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            ::prost_types::Value,
        >,
    }
}
/// Generated client implementations.
pub mod prediction_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for making recommendation prediction.
    #[derive(Debug, Clone)]
    pub struct PredictionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PredictionServiceClient<T>
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
        ) -> PredictionServiceClient<InterceptedService<T, F>>
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
            PredictionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Makes a recommendation prediction. If using API Key based authentication,
        /// the API Key must be registered using the
        /// [PredictionApiKeyRegistry][google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry]
        /// service. [Learn more](/recommendations-ai/docs/setting-up#register-key).
        pub async fn predict(
            &mut self,
            request: impl tonic::IntoRequest<super::PredictRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PredictResponse>,
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
                "/google.cloud.recommendationengine.v1beta1.PredictionService/Predict",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.PredictionService",
                        "Predict",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request message for PurgeUserEvents method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeUserEventsRequest {
    /// Required. The resource name of the event_store under which the events are
    /// created. The format is
    /// `projects/${projectId}/locations/global/catalogs/${catalogId}/eventStores/${eventStoreId}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The filter string to specify the events to be deleted. Empty
    /// string filter is not allowed. This filter can also be used with
    /// ListUserEvents API to list events that will be deleted. The eligible fields
    /// for filtering are:
    /// * eventType - UserEvent.eventType field of type string.
    /// * eventTime - in ISO 8601 "zulu" format.
    /// * visitorId - field of type string. Specifying this will delete all events
    /// associated with a visitor.
    /// * userId - field of type string. Specifying this will delete all events
    /// associated with a user.
    /// Example 1: Deleting all events in a time range.
    /// `eventTime > "2012-04-23T18:25:43.511Z" eventTime <
    /// "2012-04-23T18:30:43.511Z"`
    /// Example 2: Deleting specific eventType in time range.
    /// `eventTime > "2012-04-23T18:25:43.511Z" eventType = "detail-page-view"`
    /// Example 3: Deleting all events for a specific visitor
    /// `visitorId = visitor1024`
    /// The filtering fields are assumed to have an implicit AND.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The default value is false. Override this flag to true to
    /// actually perform the purge. If the field is not set to true, a sampling of
    /// events to be deleted will be returned.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Metadata related to the progress of the PurgeUserEvents operation.
/// This will be returned by the google.longrunning.Operation.metadata field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeUserEventsMetadata {
    /// The ID of the request / operation.
    #[prost(string, tag = "1")]
    pub operation_name: ::prost::alloc::string::String,
    /// Operation create time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Response of the PurgeUserEventsRequest. If the long running operation is
/// successfully done, then this message is returned by the
/// google.longrunning.Operations.response field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeUserEventsResponse {
    /// The total count of events purged as a result of the operation.
    #[prost(int64, tag = "1")]
    pub purged_events_count: i64,
    /// A sampling of events deleted (or will be deleted) depending on the `force`
    /// property in the request. Max of 500 items will be returned.
    #[prost(message, repeated, tag = "2")]
    pub user_events_sample: ::prost::alloc::vec::Vec<UserEvent>,
}
/// Request message for WriteUserEvent method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteUserEventRequest {
    /// Required. The parent eventStore resource name, such as
    /// `projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User event to write.
    #[prost(message, optional, tag = "2")]
    pub user_event: ::core::option::Option<UserEvent>,
}
/// Request message for CollectUserEvent method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectUserEventRequest {
    /// Required. The parent eventStore name, such as
    /// `projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. URL encoded UserEvent proto.
    #[prost(string, tag = "2")]
    pub user_event: ::prost::alloc::string::String,
    /// Optional. The url including cgi-parameters but excluding the hash fragment.
    /// The URL must be truncated to 1.5K bytes to conservatively be under the 2K
    /// bytes. This is often more useful than the referer url, because many
    /// browsers only send the domain for 3rd party requests.
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// Optional. The event timestamp in milliseconds. This prevents browser
    /// caching of otherwise identical get requests. The name is abbreviated to
    /// reduce the payload bytes.
    #[prost(int64, tag = "4")]
    pub ets: i64,
}
/// Request message for ListUserEvents method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserEventsRequest {
    /// Required. The parent eventStore resource name, such as
    /// `projects/*/locations/*/catalogs/default_catalog/eventStores/default_event_store`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of results to return per page. If zero, the
    /// service will choose a reasonable default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The previous ListUserEventsResponse.next_page_token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering expression to specify restrictions over
    /// returned events. This is a sequence of terms, where each term applies some
    /// kind of a restriction to the returned user events. Use this expression to
    /// restrict results to a specific time range, or filter events by eventType.
    ///     eg: eventTime > "2012-04-23T18:25:43.511Z" eventsMissingCatalogItems
    ///     eventTime<"2012-04-23T18:25:43.511Z" eventType=search
    ///
    ///    We expect only 3 types of fields:
    ///
    ///     * eventTime: this can be specified a maximum of 2 times, once with a
    ///       less than operator and once with a greater than operator. The
    ///       eventTime restrict should result in one contiguous valid eventTime
    ///       range.
    ///
    ///     * eventType: only 1 eventType restriction can be specified.
    ///
    ///     * eventsMissingCatalogItems: specififying this will restrict results
    ///       to events for which catalog items were not found in the catalog. The
    ///       default behavior is to return only those events for which catalog
    ///       items were found.
    ///
    ///    Some examples of valid filters expressions:
    ///
    ///    * Example 1: eventTime > "2012-04-23T18:25:43.511Z"
    ///              eventTime < "2012-04-23T18:30:43.511Z"
    ///    * Example 2: eventTime > "2012-04-23T18:25:43.511Z"
    ///              eventType = detail-page-view
    ///    * Example 3: eventsMissingCatalogItems
    ///              eventType = search eventTime < "2018-04-23T18:30:43.511Z"
    ///    * Example 4: eventTime > "2012-04-23T18:25:43.511Z"
    ///    * Example 5: eventType = search
    ///    * Example 6: eventsMissingCatalogItems
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for ListUserEvents method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserEventsResponse {
    /// The user events.
    #[prost(message, repeated, tag = "1")]
    pub user_events: ::prost::alloc::vec::Vec<UserEvent>,
    /// If empty, the list is complete. If nonempty, the token to pass to the next
    /// request's ListUserEvents.page_token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod user_event_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for ingesting end user actions on the customer website.
    #[derive(Debug, Clone)]
    pub struct UserEventServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UserEventServiceClient<T>
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
        ) -> UserEventServiceClient<InterceptedService<T, F>>
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
            UserEventServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Writes a single user event.
        pub async fn write_user_event(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteUserEventRequest>,
        ) -> std::result::Result<tonic::Response<super::UserEvent>, tonic::Status> {
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/WriteUserEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.UserEventService",
                        "WriteUserEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Writes a single user event from the browser. This uses a GET request to
        /// due to browser restriction of POST-ing to a 3rd party domain.
        ///
        /// This method is used only by the Recommendations AI JavaScript pixel.
        /// Users should not call this method directly.
        pub async fn collect_user_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CollectUserEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::api::HttpBody>,
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/CollectUserEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.UserEventService",
                        "CollectUserEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a list of user events within a time range, with potential filtering.
        pub async fn list_user_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserEventsResponse>,
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/ListUserEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.UserEventService",
                        "ListUserEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes permanently all user events specified by the filter provided.
        /// Depending on the number of events specified by the filter, this operation
        /// could take hours or days to complete. To test a filter, use the list
        /// command first.
        pub async fn purge_user_events(
            &mut self,
            request: impl tonic::IntoRequest<super::PurgeUserEventsRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/PurgeUserEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.UserEventService",
                        "PurgeUserEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Bulk import of User events. Request processing might be
        /// synchronous. Events that already exist are skipped.
        /// Use this method for backfilling historical user events.
        ///
        /// Operation.response is of type ImportResponse. Note that it is
        /// possible for a subset of the items to be successfully inserted.
        /// Operation.metadata is of type ImportMetadata.
        pub async fn import_user_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportUserEventsRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/ImportUserEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recommendationengine.v1beta1.UserEventService",
                        "ImportUserEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
