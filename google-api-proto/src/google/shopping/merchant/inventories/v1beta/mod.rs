// This file is @generated by prost-build.
/// Local inventory information for the product. Represents in-store information
/// for a specific product at the store specified by
/// [`storeCode`][google.shopping.merchant.inventories.v1beta.LocalInventory.store_code].
/// For a list of all accepted attribute values, see the [local product inventory
/// data specification](<https://support.google.com/merchants/answer/3061342>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalInventory {
    /// Output only. The name of the `LocalInventory` resource.
    /// Format:
    /// `accounts/{account}/products/{product}/localInventories/{store_code}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The account that owns the product. This field will be ignored
    /// if set by the client.
    #[prost(int64, tag = "2")]
    pub account: i64,
    /// Required. Immutable. Store code (the store ID from your Business Profile)
    /// of the physical store the product is sold in. See the [Local product
    /// inventory data
    /// specification](<https://support.google.com/merchants/answer/3061342>) for
    /// more information.
    #[prost(string, tag = "3")]
    pub store_code: ::prost::alloc::string::String,
    /// Price of the product at this store.
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<super::super::super::r#type::Price>,
    /// Sale price of the product at this store. Mandatory if
    /// [`salePriceEffectiveDate`][google.shopping.merchant.inventories.v1beta.LocalInventory.sale_price_effective_date]
    /// is defined.
    #[prost(message, optional, tag = "5")]
    pub sale_price: ::core::option::Option<super::super::super::r#type::Price>,
    /// The `TimePeriod` of the
    /// sale at this store.
    #[prost(message, optional, tag = "6")]
    pub sale_price_effective_date: ::core::option::Option<
        super::super::super::super::r#type::Interval,
    >,
    /// Availability of the product at this store.
    /// For accepted attribute values, see the [local product inventory data
    /// specification](<https://support.google.com/merchants/answer/3061342>)
    #[prost(string, optional, tag = "7")]
    pub availability: ::core::option::Option<::prost::alloc::string::String>,
    /// Quantity of the product available at this store. Must be greater than or
    /// equal to zero.
    #[prost(int64, optional, tag = "8")]
    pub quantity: ::core::option::Option<i64>,
    /// Supported pickup method for this product. Unless the value is `"not
    /// supported"`, this field must be submitted together with
    /// `pickupSla`.
    /// For accepted attribute values, see the [local product inventory data
    /// specification](<https://support.google.com/merchants/answer/3061342>)
    #[prost(string, optional, tag = "9")]
    pub pickup_method: ::core::option::Option<::prost::alloc::string::String>,
    /// Relative time period from the order date for an order for this product,
    /// from this store, to be ready for pickup. Must be submitted with
    /// `pickupMethod`.
    /// For accepted attribute values, see the [local product inventory data
    /// specification](<https://support.google.com/merchants/answer/3061342>)
    #[prost(string, optional, tag = "10")]
    pub pickup_sla: ::core::option::Option<::prost::alloc::string::String>,
    /// Location of the product inside the store. Maximum length is 20 bytes.
    #[prost(string, optional, tag = "11")]
    pub instore_product_location: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of custom (merchant-provided) attributes. You can also use
    /// `CustomAttribute` to submit any attribute of the data specification in its
    /// generic form.
    #[prost(message, repeated, tag = "12")]
    pub custom_attributes: ::prost::alloc::vec::Vec<
        super::super::super::r#type::CustomAttribute,
    >,
}
/// Request message for the `ListLocalInventories` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocalInventoriesRequest {
    /// Required. The `name` of the parent product to list local inventories for.
    /// Format:
    /// `accounts/{account}/products/{product}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of `LocalInventory` resources for the given
    /// product to return. The service returns fewer than this value if the number
    /// of inventories for the given product is less that than the `pageSize`. The
    /// default value is 25000. The maximum value is 25000; If a value higher than
    /// the maximum is specified, then the `pageSize` will default to the maximum
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListLocalInventories` call.
    /// Provide the page token to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListLocalInventories`
    /// must match the call that provided the page token. The token returned as
    /// [nextPageToken][google.shopping.merchant.inventories.v1beta.ListLocalInventoriesResponse.next_page_token]
    /// in the response to the previous request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the `ListLocalInventories` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocalInventoriesResponse {
    /// The `LocalInventory` resources for the given product from the specified
    /// account.
    #[prost(message, repeated, tag = "1")]
    pub local_inventories: ::prost::alloc::vec::Vec<LocalInventory>,
    /// A token, which can be sent as `pageToken` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the `InsertLocalInventory` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertLocalInventoryRequest {
    /// Required. The account and product where this inventory will be inserted.
    /// Format: `accounts/{account}/products/{product}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Local inventory information of the product. If the product
    /// already has a `LocalInventory` resource for the same `storeCode`, full
    /// replacement of the `LocalInventory` resource is performed.
    #[prost(message, optional, tag = "2")]
    pub local_inventory: ::core::option::Option<LocalInventory>,
}
/// Request message for the `DeleteLocalInventory` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLocalInventoryRequest {
    /// Required. The name of the local inventory for the given product to delete.
    /// Format:
    /// `accounts/{account}/products/{product}/localInventories/{store_code}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod local_inventory_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage local inventory for products
    #[derive(Debug, Clone)]
    pub struct LocalInventoryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LocalInventoryServiceClient<T>
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
        ) -> LocalInventoryServiceClient<InterceptedService<T, F>>
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
            LocalInventoryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists the `LocalInventory` resources for the given product in your merchant
        /// account. The response might contain fewer items than specified by
        /// `pageSize`. If `pageToken` was returned in previous request, it can be used
        /// to obtain additional results.
        ///
        /// `LocalInventory` resources are listed per product for a given account.
        pub async fn list_local_inventories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLocalInventoriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLocalInventoriesResponse>,
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
                "/google.shopping.merchant.inventories.v1beta.LocalInventoryService/ListLocalInventories",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.inventories.v1beta.LocalInventoryService",
                        "ListLocalInventories",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Inserts a `LocalInventory` resource to a product in your merchant
        /// account.
        ///
        /// Replaces the full `LocalInventory` resource if an entry with the same
        /// [`storeCode`][google.shopping.merchant.inventories.v1beta.LocalInventory.store_code]
        /// already exists for the product.
        ///
        /// It might take up to 30 minutes for the new or updated `LocalInventory`
        /// resource to appear in products.
        pub async fn insert_local_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertLocalInventoryRequest>,
        ) -> std::result::Result<tonic::Response<super::LocalInventory>, tonic::Status> {
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
                "/google.shopping.merchant.inventories.v1beta.LocalInventoryService/InsertLocalInventory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.inventories.v1beta.LocalInventoryService",
                        "InsertLocalInventory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified `LocalInventory` from the given product in your
        /// merchant account. It might take a up to an hour for the
        /// `LocalInventory` to be deleted from the specific product.
        /// Once you have received a successful delete response, wait for that
        /// period before attempting a delete again.
        pub async fn delete_local_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLocalInventoryRequest>,
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
                "/google.shopping.merchant.inventories.v1beta.LocalInventoryService/DeleteLocalInventory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.inventories.v1beta.LocalInventoryService",
                        "DeleteLocalInventory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Regional inventory information for the product. Represents specific
/// information like price and availability for a given product in a specific
/// [`region`][google.shopping.merchant.inventories.v1beta.RegionalInventory.region].
/// For a list of all accepted attribute values, see the [regional product
/// inventory data
/// specification](<https://support.google.com/merchants/answer/9698880>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionalInventory {
    /// Output only. The name of the `RegionalInventory` resource.
    /// Format:
    /// `{regional_inventory.name=accounts/{account}/products/{product}/regionalInventories/{region}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The account that owns the product. This field will be ignored
    /// if set by the client.
    #[prost(int64, tag = "2")]
    pub account: i64,
    /// Required. Immutable. ID of the region for this
    /// `RegionalInventory` resource. See the [Regional availability and
    /// pricing](<https://support.google.com/merchants/answer/9698880>) for more
    /// details.
    #[prost(string, tag = "3")]
    pub region: ::prost::alloc::string::String,
    /// Price of the product in this region.
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<super::super::super::r#type::Price>,
    /// Sale price of the product in this region. Mandatory if
    /// [`salePriceEffectiveDate`][google.shopping.merchant.inventories.v1beta.RegionalInventory.sale_price_effective_date]
    /// is defined.
    #[prost(message, optional, tag = "5")]
    pub sale_price: ::core::option::Option<super::super::super::r#type::Price>,
    /// The `TimePeriod` of the
    /// sale price in this region.
    #[prost(message, optional, tag = "6")]
    pub sale_price_effective_date: ::core::option::Option<
        super::super::super::super::r#type::Interval,
    >,
    /// Availability of the product in this region.
    /// For accepted attribute values, see the [regional product inventory data
    /// specification](<https://support.google.com/merchants/answer/3061342>)
    #[prost(string, optional, tag = "7")]
    pub availability: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of custom (merchant-provided) attributes. You can also use
    /// `CustomAttribute` to submit any attribute of the data specification in its
    /// generic form.
    #[prost(message, repeated, tag = "8")]
    pub custom_attributes: ::prost::alloc::vec::Vec<
        super::super::super::r#type::CustomAttribute,
    >,
}
/// Request message for the `ListRegionalInventories` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRegionalInventoriesRequest {
    /// Required. The `name` of the parent product to list `RegionalInventory`
    /// resources for. Format: `accounts/{account}/products/{product}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of `RegionalInventory` resources for the given product
    /// to return. The service returns fewer than this value if the number of
    /// inventories for the given product is less that than the `pageSize`. The
    /// default value is 25000. The maximum value is 100000; If a value higher than
    /// the maximum is specified, then the `pageSize` will default to the maximum.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListRegionalInventories` call.
    /// Provide the page token to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListRegionalInventories`
    /// must match the call that provided the page token. The token returned as
    /// [nextPageToken][google.shopping.merchant.inventories.v1beta.ListRegionalInventoriesResponse.next_page_token]
    /// in the response to the previous request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the `ListRegionalInventories` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRegionalInventoriesResponse {
    /// The `RegionalInventory` resources for the given product from the specified
    /// account.
    #[prost(message, repeated, tag = "1")]
    pub regional_inventories: ::prost::alloc::vec::Vec<RegionalInventory>,
    /// A token, which can be sent as `pageToken` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the `InsertRegionalInventory` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertRegionalInventoryRequest {
    /// Required. The account and product where this inventory will be inserted.
    /// Format: `accounts/{account}/products/{product}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Regional inventory information to add to the product. If the
    /// product already has a `RegionalInventory` resource for the same `region`,
    /// full replacement of the `RegionalInventory` resource is performed.
    #[prost(message, optional, tag = "2")]
    pub regional_inventory: ::core::option::Option<RegionalInventory>,
}
/// Request message for the `DeleteRegionalInventory` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRegionalInventoryRequest {
    /// Required. The name of the `RegionalInventory` resource to delete.
    /// Format:
    /// `accounts/{account}/products/{product}/regionalInventories/{region}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod regional_inventory_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage regional inventory for products. There is also separate
    /// `regions` resource and API to manage regions definitions.
    #[derive(Debug, Clone)]
    pub struct RegionalInventoryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RegionalInventoryServiceClient<T>
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
        ) -> RegionalInventoryServiceClient<InterceptedService<T, F>>
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
            RegionalInventoryServiceClient::new(
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
        /// Lists the `RegionalInventory` resources for the given product in your
        /// merchant account. The response might contain fewer items than specified by
        /// `pageSize`.  If `pageToken` was returned in previous request, it can be
        /// used to obtain additional results.
        ///
        /// `RegionalInventory` resources are listed per product for a given account.
        pub async fn list_regional_inventories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRegionalInventoriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRegionalInventoriesResponse>,
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
                "/google.shopping.merchant.inventories.v1beta.RegionalInventoryService/ListRegionalInventories",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.inventories.v1beta.RegionalInventoryService",
                        "ListRegionalInventories",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Inserts a `RegionalInventory` to a given product in your
        /// merchant account.
        ///
        /// Replaces the full `RegionalInventory` resource if an entry with the same
        /// [`region`][google.shopping.merchant.inventories.v1beta.RegionalInventory.region]
        /// already exists for the product.
        ///
        /// It might take up to 30 minutes for the new or updated `RegionalInventory`
        /// resource to appear in products.
        pub async fn insert_regional_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::InsertRegionalInventoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegionalInventory>,
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
                "/google.shopping.merchant.inventories.v1beta.RegionalInventoryService/InsertRegionalInventory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.inventories.v1beta.RegionalInventoryService",
                        "InsertRegionalInventory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified `RegionalInventory` resource from the given product
        /// in your merchant account.  It might take up to an hour for the
        /// `RegionalInventory` to be deleted from the specific product.
        /// Once you have received a successful delete response, wait for that
        /// period before attempting a delete again.
        pub async fn delete_regional_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRegionalInventoryRequest>,
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
                "/google.shopping.merchant.inventories.v1beta.RegionalInventoryService/DeleteRegionalInventory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.inventories.v1beta.RegionalInventoryService",
                        "DeleteRegionalInventory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
