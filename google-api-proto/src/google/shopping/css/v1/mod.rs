/// The request message for the `ListChildAccounts` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChildAccountsRequest {
    /// Required. The parent account. Must be a CSS group or domain.
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// If set, only the MC accounts with the given label ID will be returned.
    #[prost(int64, optional, tag = "2")]
    pub label_id: ::core::option::Option<i64>,
    /// If set, only the MC accounts with the given name (case sensitive) will be
    /// returned.
    #[prost(string, optional, tag = "3")]
    pub full_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. The maximum number of accounts to return. The service may return
    /// fewer than this value. If unspecified, at most 50 accounts will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListChildAccounts` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListChildAccounts` must
    /// match the call that provided the page token.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the `ListChildAccounts` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChildAccountsResponse {
    /// The CSS/MC accounts returned for the specified CSS parent account.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<Account>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for the `GetAccount` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountRequest {
    /// Required. The name of the managed CSS/MC account.
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Only required when retrieving MC account information.
    /// The CSS domain that is the parent resource of the MC account.
    /// Format: accounts/{account}
    #[prost(string, optional, tag = "2")]
    pub parent: ::core::option::Option<::prost::alloc::string::String>,
}
/// The request message for the `UpdateLabels` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccountLabelsRequest {
    /// Required. The label resource name.
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The list of label IDs to overwrite the existing account label IDs.
    /// If the list is empty, all currently assigned label IDs will be deleted.
    #[prost(int64, repeated, tag = "2")]
    pub label_ids: ::prost::alloc::vec::Vec<i64>,
    /// Optional. Only required when updating MC account labels.
    /// The CSS domain that is the parent resource of the MC account.
    /// Format: accounts/{account}
    #[prost(string, optional, tag = "3")]
    pub parent: ::core::option::Option<::prost::alloc::string::String>,
}
/// Information about CSS/MC account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// The label resource name.
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Immutable. The CSS/MC account's full name.
    #[prost(string, tag = "2")]
    pub full_name: ::prost::alloc::string::String,
    /// The CSS/MC account's short display name.
    #[prost(string, optional, tag = "3")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Immutable. The CSS/MC account's homepage.
    #[prost(string, optional, tag = "4")]
    pub homepage_uri: ::core::option::Option<::prost::alloc::string::String>,
    /// The CSS/MC account's parent resource. CSS group for CSS domains; CSS
    /// domain for MC accounts. Returned only if the user has access to the
    /// parent account.
    #[prost(string, optional, tag = "5")]
    pub parent: ::core::option::Option<::prost::alloc::string::String>,
    /// Manually created label IDs assigned to the CSS/MC account by a CSS parent
    /// account.
    #[prost(int64, repeated, tag = "6")]
    pub label_ids: ::prost::alloc::vec::Vec<i64>,
    /// Automatically created label IDs assigned to the MC account by
    /// CSS Center.
    #[prost(int64, repeated, tag = "7")]
    pub automatic_label_ids: ::prost::alloc::vec::Vec<i64>,
    /// Output only. The type of this account.
    #[prost(enumeration = "account::AccountType", tag = "8")]
    pub account_type: i32,
}
/// Nested message and enum types in `Account`.
pub mod account {
    /// The account type.
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
        /// Unknown account type.
        Unspecified = 0,
        /// CSS group account.
        CssGroup = 1,
        /// CSS domain account.
        CssDomain = 2,
        /// MC Primary CSS MCA account.
        McPrimaryCssMca = 3,
        /// MC CSS MCA account.
        McCssMca = 4,
        /// MC Marketplace MCA account.
        McMarketplaceMca = 5,
        /// MC Other MCA account.
        McOtherMca = 6,
        /// MC Standalone account.
        McStandalone = 7,
        /// MC MCA sub-account.
        McMcaSubaccount = 8,
    }
    impl AccountType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccountType::Unspecified => "ACCOUNT_TYPE_UNSPECIFIED",
                AccountType::CssGroup => "CSS_GROUP",
                AccountType::CssDomain => "CSS_DOMAIN",
                AccountType::McPrimaryCssMca => "MC_PRIMARY_CSS_MCA",
                AccountType::McCssMca => "MC_CSS_MCA",
                AccountType::McMarketplaceMca => "MC_MARKETPLACE_MCA",
                AccountType::McOtherMca => "MC_OTHER_MCA",
                AccountType::McStandalone => "MC_STANDALONE",
                AccountType::McMcaSubaccount => "MC_MCA_SUBACCOUNT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACCOUNT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CSS_GROUP" => Some(Self::CssGroup),
                "CSS_DOMAIN" => Some(Self::CssDomain),
                "MC_PRIMARY_CSS_MCA" => Some(Self::McPrimaryCssMca),
                "MC_CSS_MCA" => Some(Self::McCssMca),
                "MC_MARKETPLACE_MCA" => Some(Self::McMarketplaceMca),
                "MC_OTHER_MCA" => Some(Self::McOtherMca),
                "MC_STANDALONE" => Some(Self::McStandalone),
                "MC_MCA_SUBACCOUNT" => Some(Self::McMcaSubaccount),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod accounts_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing CSS/MC account information.
    #[derive(Debug, Clone)]
    pub struct AccountsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AccountsServiceClient<T>
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
        ) -> AccountsServiceClient<InterceptedService<T, F>>
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
            AccountsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all the accounts under the specified CSS account ID, and
        /// optionally filters by label ID and account name.
        pub async fn list_child_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChildAccountsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListChildAccountsResponse>,
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
                "/google.shopping.css.v1.AccountsService/ListChildAccounts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.css.v1.AccountsService",
                        "ListChildAccounts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a single CSS/MC account by ID.
        pub async fn get_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountRequest>,
        ) -> std::result::Result<tonic::Response<super::Account>, tonic::Status> {
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
                "/google.shopping.css.v1.AccountsService/GetAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.css.v1.AccountsService",
                        "GetAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates labels assigned to CSS/MC accounts by a CSS domain.
        pub async fn update_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAccountLabelsRequest>,
        ) -> std::result::Result<tonic::Response<super::Account>, tonic::Status> {
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
                "/google.shopping.css.v1.AccountsService/UpdateLabels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.css.v1.AccountsService",
                        "UpdateLabels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Attributes for CSS Product.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attributes {
    /// URL directly linking to your the Product Detail Page of the CSS.
    #[prost(string, optional, tag = "1")]
    pub cpp_link: ::core::option::Option<::prost::alloc::string::String>,
    /// URL for the mobile-optimized version of the Product Detail Page of the CSS.
    #[prost(string, optional, tag = "2")]
    pub cpp_mobile_link: ::core::option::Option<::prost::alloc::string::String>,
    /// Allows advertisers to override the item URL when the product is shown
    /// within the context of Product Ads.
    #[prost(string, optional, tag = "42")]
    pub cpp_ads_redirect: ::core::option::Option<::prost::alloc::string::String>,
    /// Low Price of the aggregate offer.
    #[prost(message, optional, tag = "3")]
    pub low_price: ::core::option::Option<super::super::r#type::Price>,
    /// High Price of the aggregate offer.
    #[prost(message, optional, tag = "4")]
    pub high_price: ::core::option::Option<super::super::r#type::Price>,
    /// The number of aggregate offers.
    #[prost(int64, optional, tag = "5")]
    pub number_of_offers: ::core::option::Option<i64>,
    /// Condition of the headline offer.
    #[prost(string, optional, tag = "6")]
    pub headline_offer_condition: ::core::option::Option<::prost::alloc::string::String>,
    /// Headline Price of the aggregate offer.
    #[prost(message, optional, tag = "7")]
    pub headline_offer_price: ::core::option::Option<super::super::r#type::Price>,
    /// Link to the headline offer.
    #[prost(string, optional, tag = "8")]
    pub headline_offer_link: ::core::option::Option<::prost::alloc::string::String>,
    /// Mobile Link to the headline offer.
    #[prost(string, optional, tag = "9")]
    pub headline_offer_mobile_link: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    /// Headline Price of the aggregate offer.
    #[prost(message, optional, tag = "41")]
    pub headline_offer_shipping_price: ::core::option::Option<
        super::super::r#type::Price,
    >,
    /// Title of the item.
    #[prost(string, optional, tag = "10")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    /// URL of an image of the item.
    #[prost(string, optional, tag = "11")]
    pub image_link: ::core::option::Option<::prost::alloc::string::String>,
    /// Additional URL of images of the item.
    #[prost(string, repeated, tag = "12")]
    pub additional_image_links: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Description of the item.
    #[prost(string, optional, tag = "13")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Product Related Attributes.\[14-36\]
    /// Brand of the item.
    #[prost(string, optional, tag = "14")]
    pub brand: ::core::option::Option<::prost::alloc::string::String>,
    /// Manufacturer Part Number
    /// ([MPN](<https://support.google.com/merchants/answer/188494#mpn>)) of the
    /// item.
    #[prost(string, optional, tag = "15")]
    pub mpn: ::core::option::Option<::prost::alloc::string::String>,
    /// Global Trade Item Number
    /// ([GTIN](<https://support.google.com/merchants/answer/188494#gtin>)) of the
    /// item.
    #[prost(string, optional, tag = "16")]
    pub gtin: ::core::option::Option<::prost::alloc::string::String>,
    /// Categories of the item (formatted as in [products data
    /// specification](<https://support.google.com/merchants/answer/6324406>)).
    #[prost(string, repeated, tag = "36")]
    pub product_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Google's category of the item (see [Google product
    /// taxonomy](<https://support.google.com/merchants/answer/1705911>)). When
    /// querying products, this field will contain the user provided value. There
    /// is currently no way to get back the auto assigned google product
    /// categories through the API.
    #[prost(string, optional, tag = "17")]
    pub google_product_category: ::core::option::Option<::prost::alloc::string::String>,
    /// Set to true if the item is targeted towards adults.
    #[prost(bool, optional, tag = "18")]
    pub adult: ::core::option::Option<bool>,
    /// The number of identical products in a merchant-defined multipack.
    #[prost(int64, optional, tag = "19")]
    pub multipack: ::core::option::Option<i64>,
    /// Whether the item is a merchant-defined bundle. A bundle is a custom
    /// grouping of different products sold by a merchant for a single price.
    #[prost(bool, optional, tag = "20")]
    pub is_bundle: ::core::option::Option<bool>,
    /// Target age group of the item.
    #[prost(string, optional, tag = "21")]
    pub age_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Color of the item.
    #[prost(string, optional, tag = "22")]
    pub color: ::core::option::Option<::prost::alloc::string::String>,
    /// Target gender of the item.
    #[prost(string, optional, tag = "23")]
    pub gender: ::core::option::Option<::prost::alloc::string::String>,
    /// The material of which the item is made.
    #[prost(string, optional, tag = "24")]
    pub material: ::core::option::Option<::prost::alloc::string::String>,
    /// The item's pattern (e.g. polka dots).
    #[prost(string, optional, tag = "25")]
    pub pattern: ::core::option::Option<::prost::alloc::string::String>,
    /// Size of the item. Only one value is allowed. For variants with different
    /// sizes, insert a separate product for each size with the same
    /// `itemGroupId` value (see
    /// [<https://support.google.com/merchants/answer/6324492](size> definition)).
    #[prost(string, optional, tag = "26")]
    pub size: ::core::option::Option<::prost::alloc::string::String>,
    /// System in which the size is specified. Recommended for apparel items.
    #[prost(string, optional, tag = "27")]
    pub size_system: ::core::option::Option<::prost::alloc::string::String>,
    /// The cut of the item. It can be used to represent combined size types for
    /// apparel items. Maximum two of size types can be provided (see
    /// [<https://support.google.com/merchants/answer/6324497](size> type)).
    #[prost(string, repeated, tag = "28")]
    pub size_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Shared identifier for all variants of the same product.
    #[prost(string, optional, tag = "29")]
    pub item_group_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Technical specification or additional product details.
    #[prost(message, repeated, tag = "30")]
    pub product_details: ::prost::alloc::vec::Vec<ProductDetail>,
    /// The weight of the product in the units provided. The value must be
    /// between 0 (exclusive) and 2000 (inclusive).
    #[prost(message, optional, tag = "31")]
    pub product_weight: ::core::option::Option<ProductWeight>,
    /// The length of the product in the units provided. The value must be
    /// between 0 (exclusive) and 3000 (inclusive).
    #[prost(message, optional, tag = "32")]
    pub product_length: ::core::option::Option<ProductDimension>,
    /// The width of the product in the units provided. The value must be between
    /// 0 (exclusive) and 3000 (inclusive).
    #[prost(message, optional, tag = "33")]
    pub product_width: ::core::option::Option<ProductDimension>,
    /// The height of the product in the units provided. The value must be
    /// between
    /// 0 (exclusive) and 3000 (inclusive).
    #[prost(message, optional, tag = "34")]
    pub product_height: ::core::option::Option<ProductDimension>,
    /// Bullet points describing the most relevant highlights of a product.
    #[prost(string, repeated, tag = "35")]
    pub product_highlights: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of certificates claimed by the CSS for the given product.
    #[prost(message, repeated, tag = "39")]
    pub certifications: ::prost::alloc::vec::Vec<Certification>,
    /// Date on which the item should expire, as specified upon insertion, in
    /// [ISO
    /// 8601](<http://en.wikipedia.org/wiki/ISO_8601>) format. The actual
    /// expiration date in Google Shopping is exposed in `productstatuses` as
    /// [googleExpirationDate](<https://support.google.com/merchants/answer/6324499>)
    /// and might be earlier if `expirationDate` is too far in the future.
    /// Note: It may take 2+ days from the expiration date for the item to
    /// actually get deleted.
    #[prost(message, optional, tag = "40")]
    pub expiration_date: ::core::option::Option<::prost_types::Timestamp>,
    /// The list of destinations to include for this target (corresponds to
    /// checked check boxes in Merchant Center). Default destinations are always
    /// included unless provided in `excludedDestinations`.
    #[prost(string, repeated, tag = "43")]
    pub included_destinations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of destinations to exclude for this target (corresponds to
    /// unchecked check boxes in Merchant Center).
    #[prost(string, repeated, tag = "44")]
    pub excluded_destinations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Publication of this item will be temporarily paused.
    #[prost(string, optional, tag = "45")]
    pub pause: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom label 0 for custom grouping of items in a Shopping campaign.
    #[prost(string, optional, tag = "46")]
    pub custom_label_0: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom label 1 for custom grouping of items in a Shopping campaign.
    #[prost(string, optional, tag = "47")]
    pub custom_label_1: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom label 2 for custom grouping of items in a Shopping campaign.
    #[prost(string, optional, tag = "48")]
    pub custom_label_2: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom label 3 for custom grouping of items in a Shopping campaign.
    #[prost(string, optional, tag = "49")]
    pub custom_label_3: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom label 4 for custom grouping of items in a Shopping campaign.
    #[prost(string, optional, tag = "50")]
    pub custom_label_4: ::core::option::Option<::prost::alloc::string::String>,
}
/// The certification for the product.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Certification {
    /// Name of the certification.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Name of the certification body.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// A unique code to identify the certification.
    #[prost(string, tag = "3")]
    pub code: ::prost::alloc::string::String,
}
/// The product details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductDetail {
    /// The section header used to group a set of product details.
    #[prost(string, tag = "1")]
    pub section_name: ::prost::alloc::string::String,
    /// The name of the product detail.
    #[prost(string, tag = "2")]
    pub attribute_name: ::prost::alloc::string::String,
    /// The value of the product detail.
    #[prost(string, tag = "3")]
    pub attribute_value: ::prost::alloc::string::String,
}
/// The dimension of the product.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductDimension {
    /// Required. The dimension value represented as a number. The value can have a
    /// maximum precision of four decimal places.
    #[prost(double, tag = "1")]
    pub value: f64,
    /// Required. The dimension units.
    /// Acceptable values are:
    ///    * "`in`"
    ///    * "`cm`"
    #[prost(string, tag = "2")]
    pub unit: ::prost::alloc::string::String,
}
/// The weight of the product.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductWeight {
    /// Required. The weight represented as a number. The weight can have a maximum
    /// precision of four decimal places.
    #[prost(double, tag = "1")]
    pub value: f64,
    /// Required. The weight unit.
    /// Acceptable values are:
    ///    * "`g`"
    ///    * "`kg`"
    ///    * "`oz`"
    ///    * "`lb`"
    #[prost(string, tag = "2")]
    pub unit: ::prost::alloc::string::String,
}
/// The status of the Css Product, data validation issues, that is,
/// information about the Css Product computed asynchronously.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CssProductStatus {
    /// The intended destinations for the product.
    #[prost(message, repeated, tag = "3")]
    pub destination_statuses: ::prost::alloc::vec::Vec<
        css_product_status::DestinationStatus,
    >,
    /// A list of all issues associated with the product.
    #[prost(message, repeated, tag = "4")]
    pub item_level_issues: ::prost::alloc::vec::Vec<css_product_status::ItemLevelIssue>,
    /// Date on which the item has been created, in [ISO
    /// 8601](<http://en.wikipedia.org/wiki/ISO_8601>) format.
    #[prost(message, optional, tag = "5")]
    pub creation_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Date on which the item has been last updated, in [ISO
    /// 8601](<http://en.wikipedia.org/wiki/ISO_8601>) format.
    #[prost(message, optional, tag = "6")]
    pub last_update_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Date on which the item expires in Google Shopping, in [ISO
    /// 8601](<http://en.wikipedia.org/wiki/ISO_8601>) format.
    #[prost(message, optional, tag = "7")]
    pub google_expiration_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `CssProductStatus`.
pub mod css_product_status {
    /// The destination status of the product status.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DestinationStatus {
        /// The name of the destination
        #[prost(string, tag = "1")]
        pub destination: ::prost::alloc::string::String,
        /// List of country codes (ISO 3166-1 alpha-2) where the aggregate offer is
        /// approved.
        #[prost(string, repeated, tag = "2")]
        pub approved_countries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// List of country codes (ISO 3166-1 alpha-2) where the aggregate offer is
        /// pending approval.
        #[prost(string, repeated, tag = "3")]
        pub pending_countries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// List of country codes (ISO 3166-1 alpha-2) where the aggregate offer is
        /// disapproved.
        #[prost(string, repeated, tag = "4")]
        pub disapproved_countries: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    /// The ItemLevelIssue of the product status.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ItemLevelIssue {
        /// The error code of the issue.
        #[prost(string, tag = "1")]
        pub code: ::prost::alloc::string::String,
        /// How this issue affects serving of the aggregate offer.
        #[prost(string, tag = "2")]
        pub servability: ::prost::alloc::string::String,
        /// Whether the issue can be resolved by the merchant.
        #[prost(string, tag = "3")]
        pub resolution: ::prost::alloc::string::String,
        /// The attribute's name, if the issue is caused by a single attribute.
        #[prost(string, tag = "4")]
        pub attribute: ::prost::alloc::string::String,
        /// The destination the issue applies to.
        #[prost(string, tag = "5")]
        pub destination: ::prost::alloc::string::String,
        /// A short issue description in English.
        #[prost(string, tag = "6")]
        pub description: ::prost::alloc::string::String,
        /// A detailed issue description in English.
        #[prost(string, tag = "7")]
        pub detail: ::prost::alloc::string::String,
        /// The URL of a web page to help with resolving this issue.
        #[prost(string, tag = "8")]
        pub documentation: ::prost::alloc::string::String,
        /// List of country codes (ISO 3166-1 alpha-2) where issue applies to the
        /// aggregate offer.
        #[prost(string, repeated, tag = "9")]
        pub applicable_countries: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
}
/// Label assigned by CSS domain or CSS group to one of its sub-accounts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLabel {
    /// The resource name of the label.
    /// Format: accounts/{account}/labels/{label}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The ID of the label.
    #[prost(int64, tag = "2")]
    pub label_id: i64,
    /// Output only. The ID of account this label belongs to.
    #[prost(int64, tag = "3")]
    pub account_id: i64,
    /// The display name of this label.
    #[prost(string, optional, tag = "4")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The description of this label.
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The type of this label.
    #[prost(enumeration = "account_label::LabelType", tag = "6")]
    pub label_type: i32,
}
/// Nested message and enum types in `AccountLabel`.
pub mod account_label {
    /// The label type.
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
    pub enum LabelType {
        /// Unknown label type.
        Unspecified = 0,
        /// Indicates that the label was created manually.
        Manual = 1,
        /// Indicates that the label was created automatically by CSS Center.
        Automatic = 2,
    }
    impl LabelType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LabelType::Unspecified => "LABEL_TYPE_UNSPECIFIED",
                LabelType::Manual => "MANUAL",
                LabelType::Automatic => "AUTOMATIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LABEL_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "MANUAL" => Some(Self::Manual),
                "AUTOMATIC" => Some(Self::Automatic),
                _ => None,
            }
        }
    }
}
/// Request message for the `ListAccountLabels` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountLabelsRequest {
    /// Required. The parent account.
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of labels to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 50 labels will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAccountLabels` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListAccountLabels` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the `ListAccountLabels` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountLabelsResponse {
    /// The labels from the specified account.
    #[prost(message, repeated, tag = "1")]
    pub account_labels: ::prost::alloc::vec::Vec<AccountLabel>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the 'CreateAccountLanel' method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountLabelRequest {
    /// Required. The parent account.
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The label to create.
    #[prost(message, optional, tag = "2")]
    pub account_label: ::core::option::Option<AccountLabel>,
}
/// Request message for the `UpdateAccountLabel` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccountLabelRequest {
    /// Required. The updated label. All fields must be provided.
    #[prost(message, optional, tag = "1")]
    pub account_label: ::core::option::Option<AccountLabel>,
}
/// Request message for the 'DeleteAccountLabel' method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAccountLabelRequest {
    /// Required. The name of the label to delete.
    /// Format:  accounts/{account}/labels/{label}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod account_labels_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages Merchant Center and CSS accounts labels.
    #[derive(Debug, Clone)]
    pub struct AccountLabelsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AccountLabelsServiceClient<T>
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
        ) -> AccountLabelsServiceClient<InterceptedService<T, F>>
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
            AccountLabelsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists the labels assigned to an account.
        pub async fn list_account_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccountLabelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAccountLabelsResponse>,
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
                "/google.shopping.css.v1.AccountLabelsService/ListAccountLabels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.css.v1.AccountLabelsService",
                        "ListAccountLabels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new label, not assigned to any account.
        pub async fn create_account_label(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAccountLabelRequest>,
        ) -> std::result::Result<tonic::Response<super::AccountLabel>, tonic::Status> {
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
                "/google.shopping.css.v1.AccountLabelsService/CreateAccountLabel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.css.v1.AccountLabelsService",
                        "CreateAccountLabel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a label.
        pub async fn update_account_label(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAccountLabelRequest>,
        ) -> std::result::Result<tonic::Response<super::AccountLabel>, tonic::Status> {
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
                "/google.shopping.css.v1.AccountLabelsService/UpdateAccountLabel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.css.v1.AccountLabelsService",
                        "UpdateAccountLabel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a label and removes it from all accounts to which it was assigned.
        pub async fn delete_account_label(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAccountLabelRequest>,
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
                "/google.shopping.css.v1.AccountLabelsService/DeleteAccountLabel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.css.v1.AccountLabelsService",
                        "DeleteAccountLabel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The request message for the `GetCssProduct` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCssProductRequest {
    /// Required. The name of the CSS product to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The processed CSS Product(a.k.a Aggregate Offer internally).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CssProduct {
    /// The name of the CSS Product.
    /// Format:
    /// `"accounts/{account}/cssProducts/{css_product}"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Your unique raw identifier for the product.
    #[prost(string, tag = "2")]
    pub raw_provided_id: ::prost::alloc::string::String,
    /// Output only. The two-letter [ISO
    /// 639-1](<http://en.wikipedia.org/wiki/ISO_639-1>) language code for the
    /// product.
    #[prost(string, tag = "3")]
    pub content_language: ::prost::alloc::string::String,
    /// Output only. The feed label for the product.
    #[prost(string, tag = "4")]
    pub feed_label: ::prost::alloc::string::String,
    /// Output only. A list of product attributes.
    #[prost(message, optional, tag = "5")]
    pub attributes: ::core::option::Option<Attributes>,
    /// Output only. A list of custom (CSS-provided) attributes. It can also be
    /// used to submit any attribute of the feed specification in its generic form
    /// (for example,
    /// `{ "name": "size type", "value": "regular" }`).
    /// This is useful for submitting attributes not explicitly exposed by the
    /// API, such as additional attributes used for Buy on Google.
    #[prost(message, repeated, tag = "6")]
    pub custom_attributes: ::prost::alloc::vec::Vec<
        super::super::r#type::CustomAttribute,
    >,
    /// Output only. The status of a product, data validation issues, that is,
    /// information about a product computed asynchronously.
    #[prost(message, optional, tag = "8")]
    pub css_product_status: ::core::option::Option<CssProductStatus>,
}
/// Request message for the ListCssProducts method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCssProductsRequest {
    /// Required. The account/domain to list processed CSS Products for.
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of CSS Products to return. The service may return
    /// fewer than this value.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000. If
    /// unspecified, the maximum number of CSS products will be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListCssProducts` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCssProducts`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the ListCssProducts method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCssProductsResponse {
    /// The processed CSS products from the specified account. These are your
    /// processed CSS products after applying rules and supplemental feeds.
    #[prost(message, repeated, tag = "1")]
    pub css_products: ::prost::alloc::vec::Vec<CssProduct>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod css_products_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for doing get and list on Css Products(a.k.a Aggregate Offers
    /// internally).
    #[derive(Debug, Clone)]
    pub struct CssProductsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CssProductsServiceClient<T>
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
        ) -> CssProductsServiceClient<InterceptedService<T, F>>
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
            CssProductsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Retrieves the processed CSS Product from your CSS Center account. After
        /// inserting, updating, or deleting a product input, it may take several
        /// minutes before the updated final product can be retrieved.
        pub async fn get_css_product(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCssProductRequest>,
        ) -> std::result::Result<tonic::Response<super::CssProduct>, tonic::Status> {
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
                "/google.shopping.css.v1.CssProductsService/GetCssProduct",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.css.v1.CssProductsService",
                        "GetCssProduct",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the processed CSS Products in your CSS Center account. The response
        /// might contain fewer items than specified by pageSize. Rely on pageToken to
        /// determine if there are more items to be requested.
        ///
        /// After inserting, updating, or deleting a CSS product input, it may
        /// take several minutes before the updated processed CSS product can be
        /// retrieved.
        pub async fn list_css_products(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCssProductsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCssProductsResponse>,
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
                "/google.shopping.css.v1.CssProductsService/ListCssProducts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.css.v1.CssProductsService",
                        "ListCssProducts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// This resource represents input data you submit for a CSS Product, not
/// the processed CSS Product that you see in CSS Center, in Shopping Ads, or
/// across Google surfaces.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CssProductInput {
    /// The name of the CSS Product input.
    /// Format:
    /// `accounts/{account}/cssProductInputs/{css_product_input}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The name of the processed CSS Product.
    /// Format:
    /// `accounts/{account}/cssProducts/{css_product}`
    /// "
    #[prost(string, tag = "2")]
    pub final_name: ::prost::alloc::string::String,
    /// Required. Your unique identifier for the CSS Product. This is the same for
    /// the CSS Product input and processed CSS Product. We only allow ids with
    /// alphanumerics, underscores and dashes. See the [products feed
    /// specification](<https://support.google.com/merchants/answer/188494#id>) for
    /// details.
    #[prost(string, tag = "3")]
    pub raw_provided_id: ::prost::alloc::string::String,
    /// Required. The two-letter [ISO
    /// 639-1](<http://en.wikipedia.org/wiki/ISO_639-1>) language code for the CSS
    /// Product.
    #[prost(string, tag = "4")]
    pub content_language: ::prost::alloc::string::String,
    /// Required. The [feed
    /// label](<https://developers.google.com/shopping-content/guides/products/feed-labels>)
    /// for the CSS Product.
    /// Feed Label is synonymous to "target country" and hence should always be a
    /// valid region code. For example: 'DE' for Germany, 'FR' for France.
    #[prost(string, tag = "5")]
    pub feed_label: ::prost::alloc::string::String,
    /// Represents the existing version (freshness) of the CSS Product, which
    /// can be used to preserve the right order when multiple updates are done at
    /// the same time.
    ///
    /// This field must not be set to the future time.
    ///
    /// If set, the update is prevented if a newer version of the item already
    /// exists in our system (that is the last update time of the existing
    /// CSS products is later than the freshness time set in the update). If
    /// the update happens, the last update time is then set to this freshness
    /// time.
    ///
    /// If not set, the update will not be prevented and the last update time will
    /// default to when this request was received by the CSS API.
    ///
    /// If the operation is prevented, the aborted exception will be
    /// thrown.
    #[prost(message, optional, tag = "6")]
    pub freshness_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A list of CSS Product attributes.
    #[prost(message, optional, tag = "7")]
    pub attributes: ::core::option::Option<Attributes>,
    /// A list of custom (CSS-provided) attributes. It can also be used for
    /// submitting any attribute of the feed specification in its generic
    /// form (for example:
    /// `{ "name": "size type", "value": "regular" }`).
    /// This is useful for submitting attributes not explicitly exposed by the
    /// API, such as additional attributes used for Buy on Google.
    #[prost(message, repeated, tag = "8")]
    pub custom_attributes: ::prost::alloc::vec::Vec<
        super::super::r#type::CustomAttribute,
    >,
}
/// Request message for the InsertCssProductInput method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertCssProductInputRequest {
    /// Required. The account where this CSS Product will be inserted.
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The CSS Product Input to insert.
    #[prost(message, optional, tag = "2")]
    pub css_product_input: ::core::option::Option<CssProductInput>,
    /// Required. The primary or supplemental feed id. If CSS Product already
    /// exists and feed id provided is different, then the CSS Product will be
    /// moved to a new feed. Note: For now, CSSs do not need to provide feed ids as
    /// we create feeds on the fly. We do not have supplemental feed support for
    /// CSS Products yet.
    #[prost(int64, tag = "3")]
    pub feed_id: i64,
}
/// Request message for the DeleteCssProductInput method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCssProductInputRequest {
    /// Required. The name of the CSS product input resource to delete.
    /// Format: accounts/{account}/cssProductInputs/{css_product_input}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The Content API Supplemental Feed ID.
    /// The field must not be set if the action applies to a primary feed.
    /// If the field is set, then product action applies to a supplemental feed
    /// instead of primary Content API feed.
    #[prost(int64, optional, tag = "2")]
    pub supplemental_feed_id: ::core::option::Option<i64>,
}
/// Generated client implementations.
pub mod css_product_inputs_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to use CssProductInput resource.
    /// This service helps to insert/update/delete CSS Products.
    #[derive(Debug, Clone)]
    pub struct CssProductInputsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CssProductInputsServiceClient<T>
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
        ) -> CssProductInputsServiceClient<InterceptedService<T, F>>
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
            CssProductInputsServiceClient::new(
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
        /// Uploads a CssProductInput to your CSS Center account. If an
        /// input with the same contentLanguage, identity, feedLabel and feedId already
        /// exists, this method replaces that entry.
        ///
        /// After inserting, updating, or deleting a CSS Product input, it may
        /// take several minutes before the processed CSS Product can be retrieved.
        pub async fn insert_css_product_input(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertCssProductInputRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CssProductInput>,
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
                "/google.shopping.css.v1.CssProductInputsService/InsertCssProductInput",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.css.v1.CssProductInputsService",
                        "InsertCssProductInput",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a CSS Product input from your CSS Center account.
        ///
        /// After a delete it may take several minutes until the input is no longer
        /// available.
        pub async fn delete_css_product_input(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCssProductInputRequest>,
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
                "/google.shopping.css.v1.CssProductInputsService/DeleteCssProductInput",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.css.v1.CssProductInputsService",
                        "DeleteCssProductInput",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
