/// Represents a purchase made by a customer on Cloud Marketplace.
/// Creating an order makes sure that both the Google backend systems
/// as well as external service provider's systems (if needed) allow use of
/// purchased products and ensures the appropriate billing events occur.
///
/// An Order can be made against one Product with multiple add-ons (optional) or
/// one Quote which might reference multiple products.
///
/// Customers typically choose a price plan for each Product purchased when
/// they create an order and can change their plan later, if the product allows.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    /// Output only. The resource name of the order.
    /// Has the form
    /// `billingAccounts/{billing_account}/orders/{order}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The user-specified name of the order.
    #[prost(string, tag = "10")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The items being purchased.
    #[prost(message, repeated, tag = "6")]
    pub line_items: ::prost::alloc::vec::Vec<LineItem>,
    /// Output only. Line items that were cancelled.
    #[prost(message, repeated, tag = "7")]
    pub cancelled_line_items: ::prost::alloc::vec::Vec<LineItem>,
    /// Output only. The creation timestamp.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update timestamp.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The weak etag of the order.
    #[prost(string, tag = "11")]
    pub etag: ::prost::alloc::string::String,
}
/// A single item within an order.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineItem {
    /// Output only. Line item ID.
    #[prost(string, tag = "1")]
    pub line_item_id: ::prost::alloc::string::String,
    /// Output only. Current state and information of this item. It tells what,
    /// e.g. which offer, is currently effective.
    #[prost(message, optional, tag = "2")]
    pub line_item_info: ::core::option::Option<LineItemInfo>,
    /// Output only. A change made on the item which is pending and not yet
    /// effective. Absence of this field indicates the line item is not undergoing
    /// a change.
    #[prost(message, optional, tag = "3")]
    pub pending_change: ::core::option::Option<LineItemChange>,
    /// Output only. Changes made on the item that are not pending anymore which
    /// might be because they already took effect, were reverted by the customer,
    /// or were rejected by the partner. No more operations are allowed on these
    /// changes.
    #[prost(message, repeated, tag = "4")]
    pub change_history: ::prost::alloc::vec::Vec<LineItemChange>,
}
/// A change made on a line item.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineItemChange {
    /// Output only. Change ID.
    /// All changes made within one order update operation have the same change_id.
    #[prost(string, tag = "1")]
    pub change_id: ::prost::alloc::string::String,
    /// Required. Type of the change to make.
    #[prost(enumeration = "LineItemChangeType", tag = "2")]
    pub change_type: i32,
    /// Output only. Line item info before the change.
    #[prost(message, optional, tag = "3")]
    pub old_line_item_info: ::core::option::Option<LineItemInfo>,
    /// Line item info after the change.
    #[prost(message, optional, tag = "4")]
    pub new_line_item_info: ::core::option::Option<LineItemInfo>,
    /// Output only. State of the change.
    #[prost(enumeration = "LineItemChangeState", tag = "5")]
    pub change_state: i32,
    /// Output only. Provider-supplied message explaining the LineItemChange's
    /// state. Mainly used to communicate progress and ETA for provisioning in the
    /// case of `PENDING_APPROVAL`, and to explain why the change request was
    /// denied or canceled in the case of `REJECTED` and `CANCELED` states.
    #[prost(string, tag = "6")]
    pub state_reason: ::prost::alloc::string::String,
    /// Output only. Predefined enum types for why this line item change is in
    /// current state. For example, a line item change's state could be
    /// `LINE_ITEM_CHANGE_STATE_COMPLETED` because of end-of-term expiration,
    /// immediate cancellation initiated by the user, or system-initiated
    /// cancellation.
    #[prost(enumeration = "LineItemChangeStateReasonType", tag = "10")]
    pub change_state_reason_type: i32,
    /// Output only. A time at which the change became or will become (in case of
    /// pending change) effective.
    #[prost(message, optional, tag = "7")]
    pub change_effective_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when change was initiated.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when change was updated, e.g. approved/rejected by
    /// partners or cancelled by the user.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Line item information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineItemInfo {
    /// Optional. The name of the offer can have either of these formats:
    /// 'billingAccounts/{billing_account}/offers/{offer}',
    /// or 'services/{service}/standardOffers/{offer}'.
    #[prost(string, tag = "13")]
    pub offer: ::prost::alloc::string::String,
    /// Optional. User-provided parameters.
    #[prost(message, repeated, tag = "9")]
    pub parameters: ::prost::alloc::vec::Vec<Parameter>,
    /// Output only. Information about the subscription created, if applicable.
    #[prost(message, optional, tag = "10")]
    pub subscription: ::core::option::Option<Subscription>,
}
/// User-provided Parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Parameter {
    /// Name of the parameter.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Value of parameter.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<parameter::Value>,
}
/// Nested message and enum types in `Parameter`.
pub mod parameter {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Value {
        /// The kind of value.
        #[prost(oneof = "value::Kind", tags = "3, 4, 5")]
        pub kind: ::core::option::Option<value::Kind>,
    }
    /// Nested message and enum types in `Value`.
    pub mod value {
        /// The kind of value.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            /// Represents an int64 value.
            #[prost(int64, tag = "3")]
            Int64Value(i64),
            /// Represents a string value.
            #[prost(string, tag = "4")]
            StringValue(::prost::alloc::string::String),
            /// Represents a double value.
            #[prost(double, tag = "5")]
            DoubleValue(f64),
        }
    }
}
/// Subscription information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscription {
    /// The timestamp when the subscription begins, if applicable.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The timestamp when the subscription ends, if applicable.
    #[prost(message, optional, tag = "1")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Whether auto renewal is enabled by user choice on current subscription.
    /// This field indicates order/subscription status after pending plan change is
    /// cancelled or rejected.
    #[prost(bool, tag = "2")]
    pub auto_renewal_enabled: bool,
}
/// Type of a line item change.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LineItemChangeType {
    /// Sentinel value. Do not use.
    Unspecified = 0,
    /// The change is to create a new line item.
    Create = 1,
    /// The change is to update an existing line item.
    Update = 2,
    /// The change is to cancel an existing line item.
    Cancel = 3,
    /// The change is to revert a cancellation.
    RevertCancellation = 4,
}
impl LineItemChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LineItemChangeType::Unspecified => "LINE_ITEM_CHANGE_TYPE_UNSPECIFIED",
            LineItemChangeType::Create => "LINE_ITEM_CHANGE_TYPE_CREATE",
            LineItemChangeType::Update => "LINE_ITEM_CHANGE_TYPE_UPDATE",
            LineItemChangeType::Cancel => "LINE_ITEM_CHANGE_TYPE_CANCEL",
            LineItemChangeType::RevertCancellation => {
                "LINE_ITEM_CHANGE_TYPE_REVERT_CANCELLATION"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LINE_ITEM_CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "LINE_ITEM_CHANGE_TYPE_CREATE" => Some(Self::Create),
            "LINE_ITEM_CHANGE_TYPE_UPDATE" => Some(Self::Update),
            "LINE_ITEM_CHANGE_TYPE_CANCEL" => Some(Self::Cancel),
            "LINE_ITEM_CHANGE_TYPE_REVERT_CANCELLATION" => Some(Self::RevertCancellation),
            _ => None,
        }
    }
}
/// State of a change.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LineItemChangeState {
    /// Sentinel value. Do not use.
    Unspecified = 0,
    /// Change is in this state when a change is initiated and waiting for
    /// partner approval.
    PendingApproval = 1,
    /// Change is in this state after it's approved by the partner or auto-approved
    /// but before it takes effect. The change can be overwritten
    /// or cancelled depending on the new line item info property (pending Private
    /// Offer change cannot be cancelled and can only be overwritten by another
    /// Private Offer).
    Approved = 2,
    /// Change is in this state after it's been activated.
    Completed = 3,
    /// Change is in this state if it was rejected by the partner.
    Rejected = 4,
    /// Change is in this state if it was abandoned by the user.
    Abandoned = 5,
    /// Change is in this state if it's currently being provisioned downstream. The
    /// change can't be overwritten or cancelled when it's in this state.
    Activating = 6,
}
impl LineItemChangeState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LineItemChangeState::Unspecified => "LINE_ITEM_CHANGE_STATE_UNSPECIFIED",
            LineItemChangeState::PendingApproval => {
                "LINE_ITEM_CHANGE_STATE_PENDING_APPROVAL"
            }
            LineItemChangeState::Approved => "LINE_ITEM_CHANGE_STATE_APPROVED",
            LineItemChangeState::Completed => "LINE_ITEM_CHANGE_STATE_COMPLETED",
            LineItemChangeState::Rejected => "LINE_ITEM_CHANGE_STATE_REJECTED",
            LineItemChangeState::Abandoned => "LINE_ITEM_CHANGE_STATE_ABANDONED",
            LineItemChangeState::Activating => "LINE_ITEM_CHANGE_STATE_ACTIVATING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LINE_ITEM_CHANGE_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "LINE_ITEM_CHANGE_STATE_PENDING_APPROVAL" => Some(Self::PendingApproval),
            "LINE_ITEM_CHANGE_STATE_APPROVED" => Some(Self::Approved),
            "LINE_ITEM_CHANGE_STATE_COMPLETED" => Some(Self::Completed),
            "LINE_ITEM_CHANGE_STATE_REJECTED" => Some(Self::Rejected),
            "LINE_ITEM_CHANGE_STATE_ABANDONED" => Some(Self::Abandoned),
            "LINE_ITEM_CHANGE_STATE_ACTIVATING" => Some(Self::Activating),
            _ => None,
        }
    }
}
/// Predefined types for line item change state reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LineItemChangeStateReasonType {
    /// Default value, indicating there's no predefined type for change state
    /// reason.
    Unspecified = 0,
    /// Change is in current state due to term expiration.
    Expired = 1,
    /// Change is in current state due to user-initiated cancellation.
    UserCancelled = 2,
    /// Change is in current state due to system-initiated cancellation.
    SystemCancelled = 3,
}
impl LineItemChangeStateReasonType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LineItemChangeStateReasonType::Unspecified => {
                "LINE_ITEM_CHANGE_STATE_REASON_TYPE_UNSPECIFIED"
            }
            LineItemChangeStateReasonType::Expired => {
                "LINE_ITEM_CHANGE_STATE_REASON_TYPE_EXPIRED"
            }
            LineItemChangeStateReasonType::UserCancelled => {
                "LINE_ITEM_CHANGE_STATE_REASON_TYPE_USER_CANCELLED"
            }
            LineItemChangeStateReasonType::SystemCancelled => {
                "LINE_ITEM_CHANGE_STATE_REASON_TYPE_SYSTEM_CANCELLED"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LINE_ITEM_CHANGE_STATE_REASON_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "LINE_ITEM_CHANGE_STATE_REASON_TYPE_EXPIRED" => Some(Self::Expired),
            "LINE_ITEM_CHANGE_STATE_REASON_TYPE_USER_CANCELLED" => {
                Some(Self::UserCancelled)
            }
            "LINE_ITEM_CHANGE_STATE_REASON_TYPE_SYSTEM_CANCELLED" => {
                Some(Self::SystemCancelled)
            }
            _ => None,
        }
    }
}
/// Request message for
/// \[ConsumerProcurementService.PlaceOrder][google.cloud.commerce.consumer.procurement.v1alpha1.ConsumerProcurementService.PlaceOrder\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaceOrderRequest {
    /// Required. The resource name of the parent resource.
    /// This field has the form  `billingAccounts/{billing-account-id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The user-specified name of the order being placed.
    #[prost(string, tag = "6")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Places order for offer. Required when an offer-based order is
    /// being placed.
    #[prost(message, repeated, tag = "10")]
    pub line_item_info: ::prost::alloc::vec::Vec<LineItemInfo>,
    /// Optional. A unique identifier for this request.
    /// The server will ignore subsequent requests that provide a duplicate request
    /// ID for at least 120 minutes after the first request.
    ///
    /// The request ID must be a valid
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier#Format>).
    #[prost(string, tag = "7")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message stored in the metadata field of the Operation returned by
/// \[ConsumerProcurementService.PlaceOrder][google.cloud.commerce.consumer.procurement.v1alpha1.ConsumerProcurementService.PlaceOrder\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaceOrderMetadata {}
/// Request message for
/// \[ConsumerProcurementService.GetOrder][google.cloud.commerce.consumer.procurement.v1alpha1.ConsumerProcurementService.GetOrder\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderRequest {
    /// Required. The name of the order to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[ConsumerProcurementService.ListOrders][google.cloud.commerce.consumer.procurement.v1alpha1.ConsumerProcurementService.ListOrders\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrdersRequest {
    /// Required. The parent resource to query for orders.
    /// This field has the form `billingAccounts/{billing-account-id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of entries requested.
    /// The default page size is 25 and the maximum page size is 200.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The token for fetching the next page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter that you can use to limit the list request.
    ///
    /// A query string that can match a selected set of attributes
    /// with string values. For example, `display_name=abc`.
    /// Supported query attributes are
    ///
    /// * `display_name`
    ///
    /// If the query contains special characters other than letters,
    /// underscore, or digits, the phrase must be quoted with double quotes. For
    /// example, `display_name="foo:bar"`, where the display name needs to be
    /// quoted because it contains special character colon.
    ///
    /// Queries can be combined with `OR`, and `NOT` to form more complex queries.
    /// You can also group them to force a desired evaluation order.
    /// For example, `display_name=abc OR display_name=def`.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for
/// \[ConsumerProcurementService.ListOrders][google.cloud.commerce.consumer.procurement.v1alpha1.ConsumerProcurementService.ListOrders\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrdersResponse {
    /// The list of orders in this response.
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    /// The token for fetching the next page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod consumer_procurement_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// ConsumerProcurementService allows customers to make purchases of products
    /// served by the Cloud Commerce platform.
    ///
    ///
    /// When purchases are made, the
    /// [ConsumerProcurementService][google.cloud.commerce.consumer.procurement.v1alpha1.ConsumerProcurementService]
    /// programs the appropriate backends, including both Google's own
    /// infrastructure, as well as third-party systems, and to enable billing setup
    /// for charging for the procured item.
    ///
    #[derive(Debug, Clone)]
    pub struct ConsumerProcurementServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConsumerProcurementServiceClient<T>
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
        ) -> ConsumerProcurementServiceClient<InterceptedService<T, F>>
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
            ConsumerProcurementServiceClient::new(
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
        /// Creates a new
        /// [Order][google.cloud.commerce.consumer.procurement.v1alpha1.Order].
        ///
        /// This API only supports GCP spend-based committed use
        /// discounts specified by GCP documentation.
        ///
        /// The returned long-running operation is in-progress until the backend
        /// completes the creation of the resource. Once completed, the order is
        /// in
        /// [OrderState.ORDER_STATE_ACTIVE][google.cloud.commerce.consumer.procurement.v1alpha1.OrderState.ORDER_STATE_ACTIVE].
        /// In case of failure, the order resource will be removed.
        pub async fn place_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PlaceOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::super::super::longrunning::Operation,
            >,
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
                "/google.cloud.commerce.consumer.procurement.v1alpha1.ConsumerProcurementService/PlaceOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.commerce.consumer.procurement.v1alpha1.ConsumerProcurementService",
                        "PlaceOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested
        /// [Order][google.cloud.commerce.consumer.procurement.v1alpha1.Order]
        /// resource.
        pub async fn get_order(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderRequest>,
        ) -> std::result::Result<tonic::Response<super::Order>, tonic::Status> {
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
                "/google.cloud.commerce.consumer.procurement.v1alpha1.ConsumerProcurementService/GetOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.commerce.consumer.procurement.v1alpha1.ConsumerProcurementService",
                        "GetOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists [Order][google.cloud.commerce.consumer.procurement.v1alpha1.Order]
        /// resources that the user has access to, within the scope of the parent
        /// resource.
        pub async fn list_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrdersResponse>,
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
                "/google.cloud.commerce.consumer.procurement.v1alpha1.ConsumerProcurementService/ListOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.commerce.consumer.procurement.v1alpha1.ConsumerProcurementService",
                        "ListOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
