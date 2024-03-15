/// The group information for methods in the Merchant API. The quota is shared
/// between all methods in the group. Even if none of the methods within the
/// group have usage the information for the group is returned.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaGroup {
    /// Identifier. The resource name of the quota group.
    /// Format: accounts/{account}/quotas/{group}
    /// Note: There is no guarantee on the format of {group}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The current quota usage, meaning the number of calls already
    /// made on a given day to the methods in the group. The daily quota limits
    /// reset at at 12:00 PM midday UTC.
    #[prost(int64, tag = "2")]
    pub quota_usage: i64,
    /// Output only. The maximum number of calls allowed per day for the group.
    #[prost(int64, tag = "3")]
    pub quota_limit: i64,
    /// Output only. The maximum number of calls allowed per minute for the group.
    #[prost(int64, tag = "5")]
    pub quota_minute_limit: i64,
    /// Output only. List of all methods group quota applies to.
    #[prost(message, repeated, tag = "4")]
    pub method_details: ::prost::alloc::vec::Vec<MethodDetails>,
}
/// The method details per method in the Merchant API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodDetails {
    /// Output only. The name of the method for example `products.list`.
    #[prost(string, tag = "1")]
    pub method: ::prost::alloc::string::String,
    /// Output only. The API version that the method belongs to.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// Output only. The sub-API that the method belongs to.
    #[prost(string, tag = "3")]
    pub subapi: ::prost::alloc::string::String,
    /// Output only. The path for the method such as
    /// `products/v1/productInputs.insert`
    #[prost(string, tag = "4")]
    pub path: ::prost::alloc::string::String,
}
/// Request message for the ListQuotaGroups method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQuotaGroupsRequest {
    /// Required. The merchant account who owns the collection of method quotas
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of quotas to return in the response, used
    /// for paging. Defaults to 500; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Token (if provided) to retrieve the subsequent page. All other
    /// parameters must match the original call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the ListMethodGroups method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQuotaGroupsResponse {
    /// The methods, current quota usage and limits per each group. The quota is
    /// shared between all methods in the group. The groups are sorted in
    /// descending order based on
    /// [quotaUsage][google.shopping.merchant.quota.v1main.QuotaGroup.quota_usage].
    #[prost(message, repeated, tag = "1")]
    pub quota_groups: ::prost::alloc::vec::Vec<QuotaGroup>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod quota_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to get method call quota information per Merchant API method.
    #[derive(Debug, Clone)]
    pub struct QuotaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> QuotaServiceClient<T>
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
        ) -> QuotaServiceClient<InterceptedService<T, F>>
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
            QuotaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists the daily call quota and usage per group for your Merchant
        /// Center account.
        pub async fn list_quota_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQuotaGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQuotaGroupsResponse>,
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
                "/google.shopping.merchant.quota.v1beta.QuotaService/ListQuotaGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.shopping.merchant.quota.v1beta.QuotaService",
                        "ListQuotaGroups",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
