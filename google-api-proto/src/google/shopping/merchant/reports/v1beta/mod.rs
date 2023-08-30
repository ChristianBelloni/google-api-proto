/// Request message for the `ReportService.Search` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRequest {
    /// Required. Id of the account making the call. Must be a standalone account
    /// or an MCA subaccount. Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Query that defines a report to be retrieved.
    ///
    /// For details on how to construct your query, see the Query Language
    /// guide. For the full list of available tables and fields, see the Available
    /// fields.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// Number of `ReportRows` to retrieve in a single page. Defaults to the
    /// maximum of 1000. Values above 1000 are coerced to 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Token of the page to retrieve. If not specified, the first page of results
    /// is returned. In order to request the next page of results, the value
    /// obtained from `next_page_token` in the previous response should be used.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the `ReportService.Search` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResponse {
    /// Rows that matched the search query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<ReportRow>,
    /// Token which can be sent as `page_token` to retrieve the next page. If
    /// omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Result row returned from the search query.
///
/// Only the message corresponding to the queried table is populated in the
/// response. Within the populated message, only the fields requested explicitly
/// in the query are populated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportRow {
    /// Fields available for query in `product_performance_view` table.
    #[prost(message, optional, tag = "1")]
    pub product_performance_view: ::core::option::Option<ProductPerformanceView>,
    /// Fields available for query in `non_product_performance_view` table.
    #[prost(message, optional, tag = "7")]
    pub non_product_performance_view: ::core::option::Option<NonProductPerformanceView>,
    /// Fields available for query in `product_view` table.
    #[prost(message, optional, tag = "2")]
    pub product_view: ::core::option::Option<ProductView>,
    /// Fields available for query in `price_competitiveness_product_view` table.
    #[prost(message, optional, tag = "3")]
    pub price_competitiveness_product_view: ::core::option::Option<
        PriceCompetitivenessProductView,
    >,
    /// Fields available for query in `price_insights_product_view` table.
    #[prost(message, optional, tag = "4")]
    pub price_insights_product_view: ::core::option::Option<PriceInsightsProductView>,
    /// Fields available for query in `best_sellers_product_cluster_view` table.
    #[prost(message, optional, tag = "5")]
    pub best_sellers_product_cluster_view: ::core::option::Option<
        BestSellersProductClusterView,
    >,
    /// Fields available for query in `best_sellers_brand_view` table.
    #[prost(message, optional, tag = "6")]
    pub best_sellers_brand_view: ::core::option::Option<BestSellersBrandView>,
    /// Fields available for query in `competitive_visibility_competitor_view`
    /// table.
    #[prost(message, optional, tag = "8")]
    pub competitive_visibility_competitor_view: ::core::option::Option<
        CompetitiveVisibilityCompetitorView,
    >,
    /// Fields available for query in `competitive_visibility_top_merchant_view`
    /// table.
    #[prost(message, optional, tag = "9")]
    pub competitive_visibility_top_merchant_view: ::core::option::Option<
        CompetitiveVisibilityTopMerchantView,
    >,
    /// Fields available for query in `competitive_visibility_benchmark_view`
    /// table.
    #[prost(message, optional, tag = "10")]
    pub competitive_visibility_benchmark_view: ::core::option::Option<
        CompetitiveVisibilityBenchmarkView,
    >,
}
/// Fields available for query in `product_performance_view` table.
///
/// Product performance data for your account, including performance metrics (for
/// example, `clicks`) and dimensions according to which performance metrics are
/// segmented (for example, `offer_id`). Values of product dimensions, such as
/// `offer_id`, reflect the state of a product at the time of the impression.
///
/// Segment fields cannot be selected in queries without also selecting at least
/// one metric field.
///
/// Values are only set for fields requested explicitly in the request's search
/// query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductPerformanceView {
    /// Destination of the product to which metrics apply. Segment.
    ///
    /// Product performance data is not available for the LOCAL_INVENTORY_ADS
    /// destination.
    #[prost(
        enumeration = "super::super::super::r#type::Destination",
        optional,
        tag = "1"
    )]
    pub destination: ::core::option::Option<i32>,
    /// Date in the merchant timezone to which metrics apply. Segment.
    ///
    /// Condition on `date` is required in the `WHERE` clause.
    #[prost(message, optional, tag = "2")]
    pub date: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// First day of the week (Monday) of the metrics date in the merchant
    /// timezone. Segment.
    #[prost(message, optional, tag = "3")]
    pub week: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// Code of the country where the customer is located at the time of the event.
    /// Represented in the ISO 3166 format. Segment.
    ///
    /// If the customer country cannot be determined, a special 'ZZ' code is
    /// returned.
    #[prost(string, optional, tag = "4")]
    pub customer_country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Merchant Center account id of the seller of the offer. Segment.
    ///
    /// Available only for multi-client accounts.
    #[prost(int64, optional, tag = "29")]
    pub account_id: ::core::option::Option<i64>,
    /// Merchant Center account name of the seller of the offer. Segment.
    ///
    /// Available only for multi-client accounts.
    #[prost(string, optional, tag = "30")]
    pub account_display_name: ::core::option::Option<::prost::alloc::string::String>,
    /// [External account
    /// id](<https://support.google.com/merchants/answer/11537846?hl=en>) submitted
    /// in an offer feed by a multi-seller account to identify the seller of the
    /// offer. Segment.
    ///
    /// Available only for multi-client accounts. This field is non-empty only for
    /// auto-seller accounts.
    #[prost(string, optional, tag = "31")]
    pub external_account_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Merchant-provided id of the product. Segment.
    #[prost(string, optional, tag = "5")]
    pub offer_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Title of the product. Segment.
    #[prost(string, optional, tag = "6")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    /// Brand of the product. Segment.
    #[prost(string, optional, tag = "7")]
    pub brand: ::core::option::Option<::prost::alloc::string::String>,
    /// [Product category (1st
    /// level)](<https://developers.google.com/shopping-content/guides/reports/segmentation#category_and_product_type>)
    /// in Google's product taxonomy. Segment.
    #[prost(string, optional, tag = "8")]
    pub category_l1: ::core::option::Option<::prost::alloc::string::String>,
    /// [Product category (2nd
    /// level)](<https://developers.google.com/shopping-content/guides/reports/segmentation#category_and_product_type>)
    /// in Google's product taxonomy. Segment.
    #[prost(string, optional, tag = "9")]
    pub category_l2: ::core::option::Option<::prost::alloc::string::String>,
    /// [Product category (3rd
    /// level)](<https://developers.google.com/shopping-content/guides/reports/segmentation#category_and_product_type>)
    /// in Google's product taxonomy. Segment.
    #[prost(string, optional, tag = "10")]
    pub category_l3: ::core::option::Option<::prost::alloc::string::String>,
    /// [Product category (4th
    /// level)](<https://developers.google.com/shopping-content/guides/reports/segmentation#category_and_product_type>)
    /// in Google's product taxonomy. Segment.
    #[prost(string, optional, tag = "11")]
    pub category_l4: ::core::option::Option<::prost::alloc::string::String>,
    /// [Product category (5th
    /// level)](<https://developers.google.com/shopping-content/guides/reports/segmentation#category_and_product_type>)
    /// in Google's product taxonomy. Segment.
    #[prost(string, optional, tag = "12")]
    pub category_l5: ::core::option::Option<::prost::alloc::string::String>,
    /// [Product type (1st
    /// level)](<https://developers.google.com/shopping-content/guides/reports/segmentation#category_and_product_type>)
    /// in merchant's own product taxonomy. Segment.
    #[prost(string, optional, tag = "13")]
    pub product_type_l1: ::core::option::Option<::prost::alloc::string::String>,
    /// [Product type (2nd
    /// level)](<https://developers.google.com/shopping-content/guides/reports/segmentation#category_and_product_type>)
    /// in merchant's own product taxonomy. Segment.
    #[prost(string, optional, tag = "14")]
    pub product_type_l2: ::core::option::Option<::prost::alloc::string::String>,
    /// [Product type (3rd
    /// level)](<https://developers.google.com/shopping-content/guides/reports/segmentation#category_and_product_type>)
    /// in merchant's own product taxonomy. Segment.
    #[prost(string, optional, tag = "15")]
    pub product_type_l3: ::core::option::Option<::prost::alloc::string::String>,
    /// [Product type (4th
    /// level)](<https://developers.google.com/shopping-content/guides/reports/segmentation#category_and_product_type>)
    /// in merchant's own product taxonomy. Segment.
    #[prost(string, optional, tag = "16")]
    pub product_type_l4: ::core::option::Option<::prost::alloc::string::String>,
    /// [Product type (5th
    /// level)](<https://developers.google.com/shopping-content/guides/reports/segmentation#category_and_product_type>)
    /// in merchant's own product taxonomy. Segment.
    #[prost(string, optional, tag = "17")]
    pub product_type_l5: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom label 0 for custom grouping of products. Segment.
    #[prost(string, optional, tag = "18")]
    pub custom_label0: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom label 1 for custom grouping of products. Segment.
    #[prost(string, optional, tag = "19")]
    pub custom_label1: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom label 2 for custom grouping of products. Segment.
    #[prost(string, optional, tag = "20")]
    pub custom_label2: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom label 3 for custom grouping of products. Segment.
    #[prost(string, optional, tag = "21")]
    pub custom_label3: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom label 4 for custom grouping of products. Segment.
    #[prost(string, optional, tag = "22")]
    pub custom_label4: ::core::option::Option<::prost::alloc::string::String>,
    /// Number of clicks. Metric.
    #[prost(int64, optional, tag = "23")]
    pub clicks: ::core::option::Option<i64>,
    /// Number of times merchant's products are shown. Metric.
    #[prost(int64, optional, tag = "24")]
    pub impressions: ::core::option::Option<i64>,
    /// Click-through rate - the number of clicks merchant's products receive
    /// (clicks) divided by the number of times the products are shown
    /// (impressions). Metric.
    #[prost(double, optional, tag = "25")]
    pub click_through_rate: ::core::option::Option<f64>,
    /// Number of conversions attributed to the product, reported on the conversion
    /// date. Depending on the attribution model, a conversion might be distributed
    /// across multiple clicks, where each click gets its own credit assigned. This
    /// metric is a sum of all such credits. Metric.
    ///
    /// Available only for the `FREE` traffic source.
    #[prost(double, optional, tag = "26")]
    pub conversions: ::core::option::Option<f64>,
    /// Value of conversions attributed to the product, reported on the conversion
    /// date. Metric.
    ///
    /// Available only for the `FREE` traffic source.
    #[prost(message, optional, tag = "27")]
    pub conversion_value: ::core::option::Option<super::super::super::r#type::Price>,
    /// Number of conversions divided by the number of clicks, reported on the
    /// impression date. Metric.
    ///
    /// Available only for the `FREE` traffic source.
    #[prost(double, optional, tag = "28")]
    pub conversion_rate: ::core::option::Option<f64>,
}
/// Fields available for query in `product_view` table.
///
/// Products in the current inventory. Products in this table are the same as in
/// Products sub-API but not all product attributes from Products sub-API are
/// available for query in this table. In contrast to Products sub-API, this
/// table allows to filter the returned list of products by product attributes.
/// To retrieve a single product by `id` or list all products, Products sub-API
/// should be used.
///
/// Values are only set for fields requested explicitly in the request's search
/// query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductView {
    /// REST ID of the product, in the form of languageCode~feedLabel~offerId.
    /// Merchant API methods that operate on products take this as their `name`
    /// parameter.
    ///
    /// Required in the `SELECT` clause.
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// Language code of the product in BCP 47 format.
    #[prost(string, optional, tag = "2")]
    pub language_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Feed label of the product.
    #[prost(string, optional, tag = "3")]
    pub feed_label: ::core::option::Option<::prost::alloc::string::String>,
    /// Merchant-provided id of the product.
    #[prost(string, optional, tag = "4")]
    pub offer_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Title of the product.
    #[prost(string, optional, tag = "5")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    /// Brand of the product.
    #[prost(string, optional, tag = "6")]
    pub brand: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (1st level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "7")]
    pub category_l1: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (2nd level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "8")]
    pub category_l2: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (3rd level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "9")]
    pub category_l3: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (4th level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "10")]
    pub category_l4: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (5th level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "11")]
    pub category_l5: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (1st level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "12")]
    pub product_type_l1: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (2nd level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "13")]
    pub product_type_l2: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (3rd level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "14")]
    pub product_type_l3: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (4th level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "15")]
    pub product_type_l4: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (5th level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "16")]
    pub product_type_l5: ::core::option::Option<::prost::alloc::string::String>,
    /// Product price. Absent if the information about the price of the product is
    /// not available.
    #[prost(message, optional, tag = "17")]
    pub price: ::core::option::Option<super::super::super::r#type::Price>,
    /// \[Condition\](<https://support.google.com/merchants/answer/6324469>) of the
    /// product.
    #[prost(string, optional, tag = "18")]
    pub condition: ::core::option::Option<::prost::alloc::string::String>,
    /// \[Availability\](<https://support.google.com/merchants/answer/6324448>) of the
    /// product.
    #[prost(string, optional, tag = "19")]
    pub availability: ::core::option::Option<::prost::alloc::string::String>,
    /// Normalized [shipping
    /// label](<https://support.google.com/merchants/answer/6324504>) specified in
    /// the feed.
    #[prost(string, optional, tag = "20")]
    pub shipping_label: ::core::option::Option<::prost::alloc::string::String>,
    /// List of Global Trade Item Numbers (GTINs) of the product.
    #[prost(string, repeated, tag = "21")]
    pub gtin: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Item group id provided by the merchant for grouping variants together.
    #[prost(string, optional, tag = "22")]
    pub item_group_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Link to the processed image of the product, hosted on the Google
    /// infrastructure.
    #[prost(string, optional, tag = "23")]
    pub thumbnail_link: ::core::option::Option<::prost::alloc::string::String>,
    /// The time the merchant created the product in timestamp seconds.
    #[prost(message, optional, tag = "24")]
    pub creation_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Expiration date for the product, specified on insertion.
    #[prost(message, optional, tag = "25")]
    pub expiration_date: ::core::option::Option<
        super::super::super::super::r#type::Date,
    >,
    /// Aggregated destination status.
    #[prost(
        enumeration = "product_view::AggregatedDestinationStatus",
        optional,
        tag = "26"
    )]
    pub aggregated_destination_status: ::core::option::Option<i32>,
    /// List of item issues for the product.
    ///
    /// **This field cannot be used for sorting the results.**
    ///
    /// **Only selected attributes of this field (for example,
    /// `item_issues.severity.aggregated_severity`) can be used for filtering the
    /// results.**
    #[prost(message, repeated, tag = "27")]
    pub item_issues: ::prost::alloc::vec::Vec<product_view::ItemIssue>,
}
/// Nested message and enum types in `ProductView`.
pub mod product_view {
    /// Item issue associated with the product.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ItemIssue {
        /// Item issue type.
        #[prost(message, optional, tag = "1")]
        pub r#type: ::core::option::Option<item_issue::ItemIssueType>,
        /// Item issue severity.
        #[prost(message, optional, tag = "2")]
        pub severity: ::core::option::Option<item_issue::ItemIssueSeverity>,
        /// Item issue resolution.
        #[prost(enumeration = "item_issue::ItemIssueResolution", optional, tag = "3")]
        pub resolution: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `ItemIssue`.
    pub mod item_issue {
        /// Issue type.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ItemIssueType {
            /// Error code of the issue, equivalent to the `code` of [Product
            /// issues](<https://developers.google.com/shopping-content/guides/product-issues>).
            #[prost(string, optional, tag = "1")]
            pub code: ::core::option::Option<::prost::alloc::string::String>,
            /// Canonical attribute name for attribute-specific issues.
            #[prost(string, optional, tag = "2")]
            pub canonical_attribute: ::core::option::Option<
                ::prost::alloc::string::String,
            >,
        }
        /// How the issue affects the serving of the product.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ItemIssueSeverity {
            /// Issue severity per destination.
            #[prost(message, repeated, tag = "1")]
            pub severity_per_destination: ::prost::alloc::vec::Vec<
                item_issue_severity::IssueSeverityPerDestination,
            >,
            /// Aggregated severity of the issue for all destinations it affects.
            ///
            /// **This field can be used for filtering the results.**
            #[prost(
                enumeration = "item_issue_severity::AggregatedIssueSeverity",
                optional,
                tag = "2"
            )]
            pub aggregated_severity: ::core::option::Option<i32>,
        }
        /// Nested message and enum types in `ItemIssueSeverity`.
        pub mod item_issue_severity {
            /// Issue severity per destination.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct IssueSeverityPerDestination {
                /// Destination the issue applies to.
                #[prost(
                    enumeration = "super::super::super::super::super::super::r#type::Destination",
                    optional,
                    tag = "1"
                )]
                pub destination: ::core::option::Option<i32>,
                /// List of disapproved countries in the destination, represented in ISO
                /// 3166 format.
                #[prost(string, repeated, tag = "2")]
                pub disapproved_countries: ::prost::alloc::vec::Vec<
                    ::prost::alloc::string::String,
                >,
                /// List of demoted countries in the destination, represented in ISO 3166
                /// format.
                #[prost(string, repeated, tag = "3")]
                pub demoted_countries: ::prost::alloc::vec::Vec<
                    ::prost::alloc::string::String,
                >,
            }
            /// Issue severity aggregated for all destinations.
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
            pub enum AggregatedIssueSeverity {
                /// Not specified.
                Unspecified = 0,
                /// Issue disapproves the product in at least one destination.
                Disapproved = 1,
                /// Issue demotes the product in all destinations it affects.
                Demoted = 2,
                /// Issue resolution is `PENDING_PROCESSING`.
                Pending = 3,
            }
            impl AggregatedIssueSeverity {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        AggregatedIssueSeverity::Unspecified => {
                            "AGGREGATED_ISSUE_SEVERITY_UNSPECIFIED"
                        }
                        AggregatedIssueSeverity::Disapproved => "DISAPPROVED",
                        AggregatedIssueSeverity::Demoted => "DEMOTED",
                        AggregatedIssueSeverity::Pending => "PENDING",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "AGGREGATED_ISSUE_SEVERITY_UNSPECIFIED" => {
                            Some(Self::Unspecified)
                        }
                        "DISAPPROVED" => Some(Self::Disapproved),
                        "DEMOTED" => Some(Self::Demoted),
                        "PENDING" => Some(Self::Pending),
                        _ => None,
                    }
                }
            }
        }
        /// How to resolve the issue.
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
        pub enum ItemIssueResolution {
            /// Not specified.
            Unspecified = 0,
            /// The merchant has to fix the issue.
            MerchantAction = 1,
            /// The issue will be resolved automatically (for example, image crawl) or
            /// through a Google review. No merchant action is required now. Resolution
            /// might lead to another issue (for example, if crawl fails).
            PendingProcessing = 2,
        }
        impl ItemIssueResolution {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ItemIssueResolution::Unspecified => {
                        "ITEM_ISSUE_RESOLUTION_UNSPECIFIED"
                    }
                    ItemIssueResolution::MerchantAction => "MERCHANT_ACTION",
                    ItemIssueResolution::PendingProcessing => "PENDING_PROCESSING",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "ITEM_ISSUE_RESOLUTION_UNSPECIFIED" => Some(Self::Unspecified),
                    "MERCHANT_ACTION" => Some(Self::MerchantAction),
                    "PENDING_PROCESSING" => Some(Self::PendingProcessing),
                    _ => None,
                }
            }
        }
    }
    /// Status of the product aggregated for all destinations.
    ///
    /// Here's an example of how the aggregated destination status is computed:
    ///
    /// Free listings | Shopping Ads | Status
    /// --------------|--------------|------------------------------
    /// Approved      | Approved     | ELIGIBLE
    /// Approved      | Pending      | ELIGIBLE
    /// Approved      | Disapproved  | ELIGIBLE_LIMITED
    /// Pending       | Pending      | PENDING
    /// Disapproved   | Disapproved  | NOT_ELIGIBLE_OR_DISAPPROVED
    ///
    ///
    ///
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
    pub enum AggregatedDestinationStatus {
        /// Not specified.
        Unspecified = 0,
        /// Product is not eligible or is disapproved for all destinations.
        NotEligibleOrDisapproved = 1,
        /// Product's status is pending in all destinations.
        Pending = 2,
        /// Product is eligible for some (but not all) destinations.
        EligibleLimited = 3,
        /// Product is eligible for all destinations.
        Eligible = 4,
    }
    impl AggregatedDestinationStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AggregatedDestinationStatus::Unspecified => {
                    "AGGREGATED_DESTINATION_STATUS_UNSPECIFIED"
                }
                AggregatedDestinationStatus::NotEligibleOrDisapproved => {
                    "NOT_ELIGIBLE_OR_DISAPPROVED"
                }
                AggregatedDestinationStatus::Pending => "PENDING",
                AggregatedDestinationStatus::EligibleLimited => "ELIGIBLE_LIMITED",
                AggregatedDestinationStatus::Eligible => "ELIGIBLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "AGGREGATED_DESTINATION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "NOT_ELIGIBLE_OR_DISAPPROVED" => Some(Self::NotEligibleOrDisapproved),
                "PENDING" => Some(Self::Pending),
                "ELIGIBLE_LIMITED" => Some(Self::EligibleLimited),
                "ELIGIBLE" => Some(Self::Eligible),
                _ => None,
            }
        }
    }
}
/// Fields available for query in `price_competitiveness_product_view` table.
///
/// [Price competitiveness](<https://support.google.com/merchants/answer/9626903>)
/// report.
///
/// Values are only set for fields requested explicitly in the request's search
/// query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceCompetitivenessProductView {
    /// Country of the price benchmark. Represented in the ISO 3166 format.
    ///
    /// Required in the `SELECT` clause.
    #[prost(string, optional, tag = "1")]
    pub report_country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// REST ID of the product, in the form of languageCode~feedLabel~offerId. Can
    /// be used to join data with the `product_view` table.
    ///
    /// Required in the `SELECT` clause.
    #[prost(string, optional, tag = "2")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// Merchant-provided id of the product.
    #[prost(string, optional, tag = "3")]
    pub offer_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Title of the product.
    #[prost(string, optional, tag = "4")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    /// Brand of the product.
    #[prost(string, optional, tag = "5")]
    pub brand: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (1st level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "6")]
    pub category_l1: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (2nd level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "7")]
    pub category_l2: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (3rd level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "8")]
    pub category_l3: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (4th level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "9")]
    pub category_l4: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (5th level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "10")]
    pub category_l5: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (1st level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "11")]
    pub product_type_l1: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (2nd level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "12")]
    pub product_type_l2: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (3rd level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "13")]
    pub product_type_l3: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (4th level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "14")]
    pub product_type_l4: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (5th level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "15")]
    pub product_type_l5: ::core::option::Option<::prost::alloc::string::String>,
    /// Current price of the product.
    #[prost(message, optional, tag = "16")]
    pub price: ::core::option::Option<super::super::super::r#type::Price>,
    /// Latest available price benchmark for the product's catalog in the benchmark
    /// country.
    #[prost(message, optional, tag = "17")]
    pub benchmark_price: ::core::option::Option<super::super::super::r#type::Price>,
}
/// Fields available for query in `price_insights_product_view` table.
///
/// [Price insights](<https://support.google.com/merchants/answer/11916926>)
/// report.
///
/// Values are only set for fields requested explicitly in the request's search
/// query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceInsightsProductView {
    /// REST ID of the product, in the form of languageCode~feedLabel~offerId. Can
    /// be used to join data with the `product_view` table.
    ///
    /// Required in the `SELECT` clause.
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// Merchant-provided id of the product.
    #[prost(string, optional, tag = "2")]
    pub offer_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Title of the product.
    #[prost(string, optional, tag = "3")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    /// Brand of the product.
    #[prost(string, optional, tag = "4")]
    pub brand: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (1st level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "5")]
    pub category_l1: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (2nd level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "6")]
    pub category_l2: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (3rd level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "7")]
    pub category_l3: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (4th level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "8")]
    pub category_l4: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (5th level) in [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "9")]
    pub category_l5: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (1st level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "10")]
    pub product_type_l1: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (2nd level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "11")]
    pub product_type_l2: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (3rd level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "12")]
    pub product_type_l3: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (4th level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "13")]
    pub product_type_l4: ::core::option::Option<::prost::alloc::string::String>,
    /// Product type (5th level) in merchant's own [product
    /// taxonomy](<https://support.google.com/merchants/answer/6324406>).
    #[prost(string, optional, tag = "14")]
    pub product_type_l5: ::core::option::Option<::prost::alloc::string::String>,
    /// Current price of the product.
    #[prost(message, optional, tag = "15")]
    pub price: ::core::option::Option<super::super::super::r#type::Price>,
    /// Latest suggested price for the product.
    #[prost(message, optional, tag = "16")]
    pub suggested_price: ::core::option::Option<super::super::super::r#type::Price>,
    /// Predicted change in impressions as a fraction after introducing the
    /// suggested price compared to current active price. For example, 0.05 is a 5%
    /// predicted increase in impressions.
    #[prost(double, optional, tag = "17")]
    pub predicted_impressions_change_fraction: ::core::option::Option<f64>,
    /// Predicted change in clicks as a fraction after introducing the
    /// suggested price compared to current active price. For example, 0.05 is a 5%
    /// predicted increase in clicks.
    #[prost(double, optional, tag = "18")]
    pub predicted_clicks_change_fraction: ::core::option::Option<f64>,
    /// Predicted change in conversions as a fraction after introducing the
    /// suggested price compared to current active price. For example, 0.05 is a 5%
    /// predicted increase in conversions).
    #[prost(double, optional, tag = "19")]
    pub predicted_conversions_change_fraction: ::core::option::Option<f64>,
    /// Predicted change in gross profit as a fraction after introducing the
    /// suggested price compared to current active price. For example, 0.05 is a 5%
    /// predicted increase in gross profit.
    #[prost(double, optional, tag = "20")]
    pub predicted_gross_profit_change_fraction: ::core::option::Option<f64>,
    /// Predicted change in gross profit after introducing the suggested price
    /// for a month compared to current active price.
    #[prost(message, optional, tag = "21")]
    pub predicted_monthly_gross_profit_change: ::core::option::Option<
        super::super::super::r#type::Price,
    >,
}
/// Fields available for query in `best_sellers_product_cluster_view` table.
///
/// [Best sellers](<https://support.google.com/merchants/answer/9488679>) report
/// with top product clusters. A product cluster is a grouping for different
/// offers and variants that represent the same product, for example, Google
/// Pixel 7.
///
/// Values are only set for fields requested explicitly in the request's search
/// query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BestSellersProductClusterView {
    /// Report date. The value of this field can only be one of the following:
    ///
    /// *   The first day of the week (Monday) for weekly reports,
    /// *   The first day of the month for monthly reports.
    ///
    /// Required in the `SELECT` clause. If a `WHERE` condition on `report_date` is
    /// not specified in the query, the latest available weekly or monthly report
    /// is returned.
    #[prost(message, optional, tag = "1")]
    pub report_date: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// Granularity of the report. The ranking can be done over a week or a month
    /// timeframe.
    ///
    /// Required in the `SELECT` clause. Condition on `report_granularity` is
    /// required in the `WHERE` clause.
    #[prost(enumeration = "ReportGranularity", optional, tag = "2")]
    pub report_granularity: ::core::option::Option<i32>,
    /// Country where the ranking is calculated. Represented in the ISO 3166
    /// format.
    ///
    /// Required in the `SELECT` clause. Condition on `report_country_code` is
    /// required in the `WHERE` clause.
    #[prost(string, optional, tag = "3")]
    pub report_country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Google product category ID to calculate the ranking for, represented in
    /// [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    ///
    /// Required in the `SELECT` clause. If a `WHERE` condition on
    /// `report_category_id` is not specified in the query, rankings for all
    /// top-level categories are returned.
    #[prost(int64, optional, tag = "4")]
    pub report_category_id: ::core::option::Option<i64>,
    /// Google-assigned id of the product cluster.
    #[prost(string, optional, tag = "5")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// Title of the product cluster.
    #[prost(string, optional, tag = "6")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    /// Brand of the product cluster.
    #[prost(string, optional, tag = "7")]
    pub brand: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (1st level) of the product cluster, represented in
    /// [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "8")]
    pub category_l1: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (2nd level) of the product cluster, represented in
    /// [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "9")]
    pub category_l2: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (3rd level) of the product cluster, represented in
    /// [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "10")]
    pub category_l3: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (4th level) of the product cluster, represented in
    /// [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "11")]
    pub category_l4: ::core::option::Option<::prost::alloc::string::String>,
    /// Product category (5th level) of the product cluster, represented in
    /// [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    #[prost(string, optional, tag = "12")]
    pub category_l5: ::core::option::Option<::prost::alloc::string::String>,
    /// GTINs of example variants of the product cluster.
    #[prost(string, repeated, tag = "13")]
    pub variant_gtins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Whether the product cluster is `IN_STOCK` in your product feed in at least
    /// one of the countries, `OUT_OF_STOCK` in your product feed in all countries,
    /// or `NOT_IN_INVENTORY` at all.
    ///
    /// The field doesn't take the Best sellers report country filter into account.
    #[prost(
        enumeration = "best_sellers_product_cluster_view::InventoryStatus",
        optional,
        tag = "14"
    )]
    pub inventory_status: ::core::option::Option<i32>,
    /// Whether there is at least one product of the brand currently `IN_STOCK` in
    /// your product feed in at least one of the countries, all products are
    /// `OUT_OF_STOCK` in your product feed in all countries, or
    /// `NOT_IN_INVENTORY`.
    ///
    /// The field doesn't take the Best sellers report country filter into account.
    #[prost(
        enumeration = "best_sellers_product_cluster_view::InventoryStatus",
        optional,
        tag = "15"
    )]
    pub brand_inventory_status: ::core::option::Option<i32>,
    /// Popularity of the product cluster on Ads and organic surfaces, in the
    /// selected category and country, based on the estimated number of units sold.
    #[prost(int64, optional, tag = "16")]
    pub rank: ::core::option::Option<i64>,
    /// Popularity rank in the previous week or month.
    #[prost(int64, optional, tag = "17")]
    pub previous_rank: ::core::option::Option<i64>,
    /// Estimated demand in relation to the product cluster with the highest
    /// popularity rank in the same category and country.
    #[prost(enumeration = "RelativeDemand", optional, tag = "18")]
    pub relative_demand: ::core::option::Option<i32>,
    /// Estimated demand in relation to the product cluster with the highest
    /// popularity rank in the same category and country in the previous week or
    /// month.
    #[prost(enumeration = "RelativeDemand", optional, tag = "19")]
    pub previous_relative_demand: ::core::option::Option<i32>,
    /// Change in the estimated demand. Whether it rose, sank or remained flat.
    #[prost(enumeration = "RelativeDemandChangeType", optional, tag = "20")]
    pub relative_demand_change: ::core::option::Option<i32>,
}
/// Nested message and enum types in `BestSellersProductClusterView`.
pub mod best_sellers_product_cluster_view {
    /// Status of the product cluster or brand in your inventory.
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
    pub enum InventoryStatus {
        /// Not specified.
        Unspecified = 0,
        /// You have a product for this product cluster or brand in stock.
        InStock = 1,
        /// You have a product for this product cluster or brand in inventory but it
        /// is currently out of stock.
        OutOfStock = 2,
        /// You do not have a product for this product cluster or brand in inventory.
        NotInInventory = 3,
    }
    impl InventoryStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InventoryStatus::Unspecified => "INVENTORY_STATUS_UNSPECIFIED",
                InventoryStatus::InStock => "IN_STOCK",
                InventoryStatus::OutOfStock => "OUT_OF_STOCK",
                InventoryStatus::NotInInventory => "NOT_IN_INVENTORY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INVENTORY_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "IN_STOCK" => Some(Self::InStock),
                "OUT_OF_STOCK" => Some(Self::OutOfStock),
                "NOT_IN_INVENTORY" => Some(Self::NotInInventory),
                _ => None,
            }
        }
    }
}
/// Fields available for query in `best_sellers_brand_view` table.
///
/// [Best sellers](<https://support.google.com/merchants/answer/9488679>) report
/// with top brands.
///
/// Values are only set for fields requested explicitly in the request's search
/// query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BestSellersBrandView {
    /// Report date. The value of this field can only be one of the following:
    ///
    /// *   The first day of the week (Monday) for weekly reports,
    /// *   The first day of the month for monthly reports.
    ///
    /// Required in the `SELECT` clause. If a `WHERE` condition on `report_date` is
    /// not specified in the query, the latest available weekly or monthly report
    /// is returned.
    #[prost(message, optional, tag = "1")]
    pub report_date: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// Granularity of the report. The ranking can be done over a week or a month
    /// timeframe.
    ///
    /// Required in the `SELECT` clause. Condition on `report_granularity` is
    /// required in the `WHERE` clause.
    #[prost(enumeration = "ReportGranularity", optional, tag = "2")]
    pub report_granularity: ::core::option::Option<i32>,
    /// Country where the ranking is calculated. Represented in the ISO 3166
    /// format.
    ///
    /// Required in the `SELECT` clause. Condition on `report_country_code` is
    /// required in the `WHERE` clause.
    #[prost(string, optional, tag = "3")]
    pub report_country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Google product category ID to calculate the ranking for, represented in
    /// [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    ///
    /// Required in the `SELECT` clause. If a `WHERE` condition on
    /// `report_category_id` is not specified in the query, rankings for all
    /// top-level categories are returned.
    #[prost(int64, optional, tag = "4")]
    pub report_category_id: ::core::option::Option<i64>,
    /// Google-assigned id of the brand.
    #[prost(string, optional, tag = "5")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the brand.
    #[prost(string, optional, tag = "6")]
    pub brand: ::core::option::Option<::prost::alloc::string::String>,
    /// Popularity of the brand on Ads and organic surfaces, in the selected
    /// category and country, based on the estimated number of units sold.
    #[prost(int64, optional, tag = "7")]
    pub rank: ::core::option::Option<i64>,
    /// Popularity rank in the previous week or month.
    #[prost(int64, optional, tag = "8")]
    pub previous_rank: ::core::option::Option<i64>,
    /// Estimated demand in relation to the brand with the highest popularity rank
    /// in the same category and country.
    #[prost(enumeration = "RelativeDemand", optional, tag = "9")]
    pub relative_demand: ::core::option::Option<i32>,
    /// Estimated demand in relation to the brand with the highest popularity rank
    /// in the same category and country in the previous week or month.
    #[prost(enumeration = "RelativeDemand", optional, tag = "10")]
    pub previous_relative_demand: ::core::option::Option<i32>,
    /// Change in the estimated demand. Whether it rose, sank or remained flat.
    #[prost(enumeration = "RelativeDemandChangeType", optional, tag = "11")]
    pub relative_demand_change: ::core::option::Option<i32>,
}
/// Fields available for query in `non_product_performance_view` table.
///
/// Performance data on images and website links leading to your non-product
/// website pages. This includes performance metrics (for example, `clicks`)
/// and dimensions according to which performance metrics are segmented (for
/// example, `date`).
///
/// Segment fields cannot be selected in queries without also selecting at least
/// one metric field.
///
/// Values are only set for fields requested explicitly in the request's search
/// query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NonProductPerformanceView {
    /// Date in the merchant timezone to which metrics apply. Segment.
    ///
    /// Condition on `date` is required in the `WHERE` clause.
    #[prost(message, optional, tag = "1")]
    pub date: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// First day of the week (Monday) of the metrics date in the merchant
    /// timezone. Segment.
    #[prost(message, optional, tag = "2")]
    pub week: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// Number of clicks on images and website links leading to your non-product
    /// website pages. Metric.
    #[prost(int64, optional, tag = "3")]
    pub clicks: ::core::option::Option<i64>,
    /// Number of times images and website links leading to your non-product
    /// website pages were shown. Metric.
    #[prost(int64, optional, tag = "4")]
    pub impressions: ::core::option::Option<i64>,
    /// Click-through rate - the number of clicks (`clicks`) divided by the number
    /// of impressions (`impressions`) of images and website links leading to your
    /// non-product website pages. Metric.
    #[prost(double, optional, tag = "5")]
    pub click_through_rate: ::core::option::Option<f64>,
}
/// Fields available for query in `competitive_visibility_competitor_view` table.
///
/// [Competitive
/// visibility](<https://support.google.com/merchants/answer/11366442>) report with
/// businesses with similar visibility.
///
/// Values are only set for fields requested explicitly in the request's search
/// query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompetitiveVisibilityCompetitorView {
    /// Date of this row.
    ///
    /// A condition on `date` is required in the `WHERE` clause.
    #[prost(message, optional, tag = "1")]
    pub date: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// Domain of your competitor or your domain, if 'is_your_domain' is true.
    ///
    /// Required in the `SELECT` clause. Cannot be filtered on in the 'WHERE'
    /// clause.
    #[prost(string, optional, tag = "2")]
    pub domain: ::core::option::Option<::prost::alloc::string::String>,
    /// True if this row contains data for your domain.
    ///
    /// Cannot be filtered on in the 'WHERE' clause.
    #[prost(bool, optional, tag = "3")]
    pub is_your_domain: ::core::option::Option<bool>,
    /// Country where impressions appeared.
    ///
    /// Required in the `SELECT` clause. A condition on `report_country_code` is
    /// required in the `WHERE` clause.
    #[prost(string, optional, tag = "4")]
    pub report_country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Google product category ID to calculate the report for, represented in
    /// [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    ///
    /// Required in the `SELECT` clause. A condition on `report_category_id` is
    /// required in the `WHERE` clause.
    #[prost(int64, optional, tag = "5")]
    pub report_category_id: ::core::option::Option<i64>,
    /// Traffic source of impressions.
    ///
    /// Required in the `SELECT` clause.
    #[prost(enumeration = "TrafficSource", optional, tag = "6")]
    pub traffic_source: ::core::option::Option<i32>,
    /// Position of the domain in the similar businesses ranking for the selected
    /// keys (`date`, `report_category_id`, `report_country_code`,
    /// `traffic_source`) based on impressions. 1 is the highest.
    ///
    /// Cannot be filtered on in the 'WHERE' clause.
    #[prost(int64, optional, tag = "7")]
    pub rank: ::core::option::Option<i64>,
    /// [Ads / organic ratio]
    /// (<https://support.google.com/merchants/answer/11366442#zippy=%2Cads-free-ratio>)
    /// shows how often the domain receives impressions from Shopping ads compared
    /// to organic traffic. The number is rounded and bucketed.
    ///
    /// Cannot be filtered on in the 'WHERE' clause.
    #[prost(double, optional, tag = "8")]
    pub ads_organic_ratio: ::core::option::Option<f64>,
    /// [Page overlap rate]
    /// (<https://support.google.com/merchants/answer/11366442#zippy=%2Cpage-overlap-rate>)
    /// shows how frequently competing retailers’ offers are shown together with
    /// your offers on the same page.
    ///
    /// Cannot be filtered on in the 'WHERE' clause.
    #[prost(double, optional, tag = "9")]
    pub page_overlap_rate: ::core::option::Option<f64>,
    /// [Higher position rate]
    /// (<https://support.google.com/merchants/answer/11366442#zippy=%2Chigher-position-rate>)
    /// shows how often a competitor’s offer got placed in a higher position on the
    /// page than your offer.
    ///
    /// Cannot be filtered on in the 'WHERE' clause.
    #[prost(double, optional, tag = "10")]
    pub higher_position_rate: ::core::option::Option<f64>,
    /// [Relative visibility]
    /// (<https://support.google.com/merchants/answer/11366442#zippy=%2Crelative-visibility>)
    /// shows how often your competitors’ offers are shown compared to your offers.
    /// In other words, this is the number of displayed impressions of a competitor
    /// retailer divided by the number of your displayed impressions during a
    /// selected time range for a selected product category and country.
    ///
    /// Cannot be filtered on in the 'WHERE' clause.
    #[prost(double, optional, tag = "11")]
    pub relative_visibility: ::core::option::Option<f64>,
}
/// Fields available for query in `competitive_visibility_top_merchant_view`
/// table.
///
/// [Competitive
/// visibility](<https://support.google.com/merchants/answer/11366442>) report with
/// business with highest visibility.
///
/// Values are only set for fields requested explicitly in the request's search
/// query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompetitiveVisibilityTopMerchantView {
    /// Date of this row.
    ///
    /// Cannot be selected in the `SELECT` clause. A condition on `date` is
    /// required in the `WHERE` clause.
    #[prost(message, optional, tag = "1")]
    pub date: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// Domain of your competitor or your domain, if 'is_your_domain' is true.
    ///
    /// Required in the `SELECT` clause. Cannot be filtered on in the 'WHERE'
    /// clause.
    #[prost(string, optional, tag = "2")]
    pub domain: ::core::option::Option<::prost::alloc::string::String>,
    /// True if this row contains data for your domain.
    ///
    /// Cannot be filtered on in the 'WHERE' clause.
    #[prost(bool, optional, tag = "3")]
    pub is_your_domain: ::core::option::Option<bool>,
    /// Country where impressions appeared.
    ///
    /// Required in the `SELECT` clause. A condition on `report_country_code` is
    /// required in the `WHERE` clause.
    #[prost(string, optional, tag = "4")]
    pub report_country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Google product category ID to calculate the report for, represented in
    /// [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    ///
    /// Required in the `SELECT` clause. A condition on `report_category_id` is
    /// required in the `WHERE` clause.
    #[prost(int64, optional, tag = "5")]
    pub report_category_id: ::core::option::Option<i64>,
    /// Traffic source of impressions.
    ///
    /// Required in the `SELECT` clause.
    #[prost(enumeration = "TrafficSource", optional, tag = "6")]
    pub traffic_source: ::core::option::Option<i32>,
    /// Position of the domain in the top merchants ranking for the selected keys
    /// (`date`, `report_category_id`, `report_country_code`, `traffic_source`)
    /// based on impressions. 1 is the highest.
    ///
    /// Cannot be filtered on in the 'WHERE' clause.
    #[prost(int64, optional, tag = "7")]
    pub rank: ::core::option::Option<i64>,
    /// [Ads / organic ratio]
    /// (<https://support.google.com/merchants/answer/11366442#zippy=%2Cads-free-ratio>)
    /// shows how often the domain receives impressions from Shopping ads compared
    /// to organic traffic. The number is rounded and bucketed.
    ///
    /// Cannot be filtered on in the 'WHERE' clause.
    #[prost(double, optional, tag = "8")]
    pub ads_organic_ratio: ::core::option::Option<f64>,
    /// [Page overlap rate]
    /// (<https://support.google.com/merchants/answer/11366442#zippy=%2Cpage-overlap-rate>)
    /// shows how frequently competing retailers’ offers are shown together with
    /// your offers on the same page.
    ///
    /// Cannot be filtered on in the 'WHERE' clause.
    #[prost(double, optional, tag = "9")]
    pub page_overlap_rate: ::core::option::Option<f64>,
    /// [Higher position rate]
    /// (<https://support.google.com/merchants/answer/11366442#zippy=%2Chigher-position-rate>)
    /// shows how often a competitor’s offer got placed in a higher position on the
    /// page than your offer.
    ///
    /// Cannot be filtered on in the 'WHERE' clause.
    #[prost(double, optional, tag = "10")]
    pub higher_position_rate: ::core::option::Option<f64>,
}
/// Fields available for query in `competitive_visibility_benchmark_view` table.
///
/// [Competitive
/// visibility](<https://support.google.com/merchants/answer/11366442>) report with
/// the category benchmark.
///
/// Values are only set for fields requested explicitly in the request's search
/// query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompetitiveVisibilityBenchmarkView {
    /// Date of this row.
    ///
    /// Required in the `SELECT` clause. A condition on `date` is required in the
    /// `WHERE` clause.
    #[prost(message, optional, tag = "1")]
    pub date: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// Country where impressions appeared.
    ///
    /// Required in the `SELECT` clause. A condition on `report_country_code` is
    /// required in the `WHERE` clause.
    #[prost(string, optional, tag = "2")]
    pub report_country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Google product category ID to calculate the report for, represented in
    /// [Google's product
    /// taxonomy](<https://support.google.com/merchants/answer/6324436>).
    ///
    /// Required in the `SELECT` clause. A condition on `report_category_id` is
    /// required in the `WHERE` clause.
    #[prost(int64, optional, tag = "3")]
    pub report_category_id: ::core::option::Option<i64>,
    /// Traffic source of impressions.
    ///
    /// Required in the `SELECT` clause.
    #[prost(enumeration = "TrafficSource", optional, tag = "4")]
    pub traffic_source: ::core::option::Option<i32>,
    /// Change in visibility based on impressions for your domain with respect to
    /// the start of the selected time range (or first day with non-zero
    /// impressions).
    ///
    /// Cannot be filtered on in the 'WHERE' clause.
    #[prost(double, optional, tag = "5")]
    pub your_domain_visibility_trend: ::core::option::Option<f64>,
    /// Change in visibility based on impressions with respect to the start of the
    /// selected time range (or first day with non-zero impressions) for a
    /// combined set of merchants with highest visibility approximating the
    /// market.
    ///
    /// Cannot be filtered on in the 'WHERE' clause.
    #[prost(double, optional, tag = "6")]
    pub category_benchmark_visibility_trend: ::core::option::Option<f64>,
}
/// Granularity of the Best sellers report. Best sellers reports are computed
/// over a week and a month timeframe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReportGranularity {
    /// Not specified.
    Unspecified = 0,
    /// Report is computed over a week timeframe.
    Weekly = 1,
    /// Report is computed over a month timeframe.
    Monthly = 2,
}
impl ReportGranularity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReportGranularity::Unspecified => "REPORT_GRANULARITY_UNSPECIFIED",
            ReportGranularity::Weekly => "WEEKLY",
            ReportGranularity::Monthly => "MONTHLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REPORT_GRANULARITY_UNSPECIFIED" => Some(Self::Unspecified),
            "WEEKLY" => Some(Self::Weekly),
            "MONTHLY" => Some(Self::Monthly),
            _ => None,
        }
    }
}
/// Relative demand of a product cluster or brand in the Best sellers report.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RelativeDemand {
    /// Not specified.
    Unspecified = 0,
    /// Demand is 0-5% of the demand of the highest ranked product cluster or
    /// brand.
    VeryLow = 10,
    /// Demand is 6-10% of the demand of the highest ranked product cluster or
    /// brand.
    Low = 20,
    /// Demand is 11-20% of the demand of the highest ranked product cluster or
    /// brand.
    Medium = 30,
    /// Demand is 21-50% of the demand of the highest ranked product cluster or
    /// brand.
    High = 40,
    /// Demand is 51-100% of the demand of the highest ranked product cluster or
    /// brand.
    VeryHigh = 50,
}
impl RelativeDemand {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RelativeDemand::Unspecified => "RELATIVE_DEMAND_UNSPECIFIED",
            RelativeDemand::VeryLow => "VERY_LOW",
            RelativeDemand::Low => "LOW",
            RelativeDemand::Medium => "MEDIUM",
            RelativeDemand::High => "HIGH",
            RelativeDemand::VeryHigh => "VERY_HIGH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RELATIVE_DEMAND_UNSPECIFIED" => Some(Self::Unspecified),
            "VERY_LOW" => Some(Self::VeryLow),
            "LOW" => Some(Self::Low),
            "MEDIUM" => Some(Self::Medium),
            "HIGH" => Some(Self::High),
            "VERY_HIGH" => Some(Self::VeryHigh),
            _ => None,
        }
    }
}
/// Relative demand of a product cluster or brand in the Best sellers report
/// compared to the previous time period.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RelativeDemandChangeType {
    /// Not specified.
    Unspecified = 0,
    /// Relative demand is lower than the previous time period.
    Sinker = 1,
    /// Relative demand is equal to the previous time period.
    Flat = 2,
    /// Relative demand is higher than the previous time period.
    Riser = 3,
}
impl RelativeDemandChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RelativeDemandChangeType::Unspecified => {
                "RELATIVE_DEMAND_CHANGE_TYPE_UNSPECIFIED"
            }
            RelativeDemandChangeType::Sinker => "SINKER",
            RelativeDemandChangeType::Flat => "FLAT",
            RelativeDemandChangeType::Riser => "RISER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RELATIVE_DEMAND_CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SINKER" => Some(Self::Sinker),
            "FLAT" => Some(Self::Flat),
            "RISER" => Some(Self::Riser),
            _ => None,
        }
    }
}
/// Traffic source of impressions in the Competitive visibility report.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrafficSource {
    /// Not specified.
    Unspecified = 0,
    /// Organic traffic.
    Organic = 1,
    /// Traffic from ads.
    Ads = 2,
    /// Organic and ads traffic.
    All = 3,
}
impl TrafficSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrafficSource::Unspecified => "TRAFFIC_SOURCE_UNSPECIFIED",
            TrafficSource::Organic => "ORGANIC",
            TrafficSource::Ads => "ADS",
            TrafficSource::All => "ALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRAFFIC_SOURCE_UNSPECIFIED" => Some(Self::Unspecified),
            "ORGANIC" => Some(Self::Organic),
            "ADS" => Some(Self::Ads),
            "ALL" => Some(Self::All),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod report_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for retrieving reports and insights about your products, their
    /// performance, and their competitive environment on Google.
    #[derive(Debug, Clone)]
    pub struct ReportServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ReportServiceClient<T>
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
        ) -> ReportServiceClient<InterceptedService<T, F>>
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
            ReportServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Retrieves a report defined by a search query. The response might contain
        /// fewer rows than specified by `page_size`. Rely on `next_page_token` to
        /// determine if there are more rows to be requested.
        pub async fn search(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRequest>,
        ) -> std::result::Result<tonic::Response<super::SearchResponse>, tonic::Status> {
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
                "/google.shopping.merchant.reports.v1beta.ReportService/Search",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.reports.v1beta.ReportService",
                        "Search",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
