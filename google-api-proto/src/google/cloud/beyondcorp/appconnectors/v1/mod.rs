/// ResourceInfo represents the information/status of an app connector resource.
/// Such as:
/// - remote_agent
///    - container
///      - runtime
///      - appgateway
///        - appconnector
///          - appconnection
///            - tunnel
///        - logagent
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceInfo {
    /// Required. Unique Id for the resource.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Overall health status. Overall status is derived based on the status of
    /// each sub level resources.
    #[prost(enumeration = "HealthStatus", tag = "2")]
    pub status: i32,
    /// Specific details for the resource. This is for internal use only.
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<::prost_types::Any>,
    /// The timestamp to collect the info. It is suggested to be set by
    /// the topmost level resource only.
    #[prost(message, optional, tag = "4")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// List of Info for the sub level resources.
    #[prost(message, repeated, tag = "5")]
    pub sub: ::prost::alloc::vec::Vec<ResourceInfo>,
}
/// HealthStatus represents the health status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HealthStatus {
    /// Health status is unknown: not initialized or failed to retrieve.
    Unspecified = 0,
    /// The resource is healthy.
    Healthy = 1,
    /// The resource is unhealthy.
    Unhealthy = 2,
    /// The resource is unresponsive.
    Unresponsive = 3,
    /// Some sub-resources are UNHEALTHY.
    Degraded = 4,
}
impl HealthStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HealthStatus::Unspecified => "HEALTH_STATUS_UNSPECIFIED",
            HealthStatus::Healthy => "HEALTHY",
            HealthStatus::Unhealthy => "UNHEALTHY",
            HealthStatus::Unresponsive => "UNRESPONSIVE",
            HealthStatus::Degraded => "DEGRADED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HEALTH_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "HEALTHY" => Some(Self::Healthy),
            "UNHEALTHY" => Some(Self::Unhealthy),
            "UNRESPONSIVE" => Some(Self::Unresponsive),
            "DEGRADED" => Some(Self::Degraded),
            _ => None,
        }
    }
}
/// AppConnectorInstanceConfig defines the instance config of a AppConnector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppConnectorInstanceConfig {
    /// Required. A monotonically increasing number generated and maintained
    /// by the API provider. Every time a config changes in the backend, the
    /// sequenceNumber should be bumped up to reflect the change.
    #[prost(int64, tag = "1")]
    pub sequence_number: i64,
    /// The SLM instance agent configuration.
    #[prost(message, optional, tag = "2")]
    pub instance_config: ::core::option::Option<::prost_types::Any>,
    /// NotificationConfig defines the notification mechanism that the remote
    /// instance should subscribe to in order to receive notification.
    #[prost(message, optional, tag = "3")]
    pub notification_config: ::core::option::Option<NotificationConfig>,
    /// ImageConfig defines the GCR images to run for the remote agent's control
    /// plane.
    #[prost(message, optional, tag = "4")]
    pub image_config: ::core::option::Option<ImageConfig>,
}
/// NotificationConfig defines the mechanisms to notify instance agent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationConfig {
    #[prost(oneof = "notification_config::Config", tags = "1")]
    pub config: ::core::option::Option<notification_config::Config>,
}
/// Nested message and enum types in `NotificationConfig`.
pub mod notification_config {
    /// The configuration for Pub/Sub messaging for the AppConnector.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CloudPubSubNotificationConfig {
        /// The Pub/Sub subscription the AppConnector uses to receive notifications.
        #[prost(string, tag = "1")]
        pub pubsub_subscription: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        /// Cloud Pub/Sub Configuration to receive notifications.
        #[prost(message, tag = "1")]
        PubsubNotification(CloudPubSubNotificationConfig),
    }
}
/// ImageConfig defines the control plane images to run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageConfig {
    /// The initial image the remote agent will attempt to run for the control
    /// plane.
    #[prost(string, tag = "1")]
    pub target_image: ::prost::alloc::string::String,
    /// The stable image that the remote agent will fallback to if the target image
    /// fails.
    #[prost(string, tag = "2")]
    pub stable_image: ::prost::alloc::string::String,
}
/// Request message for BeyondCorp.ListAppConnectors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppConnectorsRequest {
    /// Required. The resource name of the AppConnector location using the form:
    /// `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return.
    /// If not specified, a default value of 50 will be used by the service.
    /// Regardless of the page_size value, the response may include a partial list
    /// and a caller should only rely on response's
    /// [next_page_token][BeyondCorp.ListAppConnectorsResponse.next_page_token] to
    /// determine if there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous
    /// ListAppConnectorsRequest, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A filter specifying constraints of a list operation.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specifies the ordering of results. See
    /// [Sorting
    /// order](<https://cloud.google.com/apis/design/design_patterns#sorting_order>)
    /// for more information.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for BeyondCorp.ListAppConnectors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppConnectorsResponse {
    /// A list of BeyondCorp AppConnectors in the project.
    #[prost(message, repeated, tag = "1")]
    pub app_connectors: ::prost::alloc::vec::Vec<AppConnector>,
    /// A token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// A list of locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for BeyondCorp.GetAppConnector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAppConnectorRequest {
    /// Required. BeyondCorp AppConnector name using the form:
    /// `projects/{project_id}/locations/{location_id}/appConnectors/{app_connector_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BeyondCorp.CreateAppConnector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAppConnectorRequest {
    /// Required. The resource project name of the AppConnector location using the
    /// form: `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. User-settable AppConnector resource ID.
    ///
    ///   * Must start with a letter.
    ///   * Must contain between 4-63 characters from `/[a-z][0-9]-/`.
    ///   * Must end with a number or a letter.
    #[prost(string, tag = "2")]
    pub app_connector_id: ::prost::alloc::string::String,
    /// Required. A BeyondCorp AppConnector resource.
    #[prost(message, optional, tag = "3")]
    pub app_connector: ::core::option::Option<AppConnector>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Request message for BeyondCorp.UpdateAppConnector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAppConnectorRequest {
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. The elements of the repeated paths field may only include these
    /// fields from \[BeyondCorp.AppConnector\]:
    /// * `labels`
    /// * `display_name`
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. AppConnector message with updated fields. Only supported fields
    /// specified in update_mask are updated.
    #[prost(message, optional, tag = "2")]
    pub app_connector: ::core::option::Option<AppConnector>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for BeyondCorp.DeleteAppConnector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAppConnectorRequest {
    /// Required. BeyondCorp AppConnector name using the form:
    /// `projects/{project_id}/locations/{location_id}/appConnectors/{app_connector_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Request report the connector status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportStatusRequest {
    /// Required. BeyondCorp Connector name using the form:
    /// `projects/{project_id}/locations/{location_id}/connectors/{connector}`
    #[prost(string, tag = "1")]
    pub app_connector: ::prost::alloc::string::String,
    /// Required. Resource info of the connector.
    #[prost(message, optional, tag = "2")]
    pub resource_info: ::core::option::Option<ResourceInfo>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A BeyondCorp connector resource that represents an application facing
/// component deployed proximal to and with direct access to the application
/// instances. It is used to establish connectivity between the remote enterprise
/// environment and GCP. It initiates connections to the applications and can
/// proxy the data from users over the connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppConnector {
    /// Required. Unique resource name of the AppConnector.
    /// The name is ignored when creating a AppConnector.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Timestamp when the resource was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the resource was last modified.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Resource labels to represent user provided metadata.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. An arbitrary user-provided name for the AppConnector. Cannot
    /// exceed 64 characters.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. A unique identifier for the instance generated by the
    /// system.
    #[prost(string, tag = "6")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The current state of the AppConnector.
    #[prost(enumeration = "app_connector::State", tag = "7")]
    pub state: i32,
    /// Required. Principal information about the Identity of the AppConnector.
    #[prost(message, optional, tag = "8")]
    pub principal_info: ::core::option::Option<app_connector::PrincipalInfo>,
    /// Optional. Resource info of the connector.
    #[prost(message, optional, tag = "11")]
    pub resource_info: ::core::option::Option<ResourceInfo>,
}
/// Nested message and enum types in `AppConnector`.
pub mod app_connector {
    /// PrincipalInfo represents an Identity oneof.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PrincipalInfo {
        #[prost(oneof = "principal_info::Type", tags = "1")]
        pub r#type: ::core::option::Option<principal_info::Type>,
    }
    /// Nested message and enum types in `PrincipalInfo`.
    pub mod principal_info {
        /// ServiceAccount represents a GCP service account.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServiceAccount {
            /// Email address of the service account.
            #[prost(string, tag = "1")]
            pub email: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// A GCP service account.
            #[prost(message, tag = "1")]
            ServiceAccount(ServiceAccount),
        }
    }
    /// Represents the different states of a AppConnector.
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
        /// Default value. This value is unused.
        Unspecified = 0,
        /// AppConnector is being created.
        Creating = 1,
        /// AppConnector has been created.
        Created = 2,
        /// AppConnector's configuration is being updated.
        Updating = 3,
        /// AppConnector is being deleted.
        Deleting = 4,
        /// AppConnector is down and may be restored in the future.
        /// This happens when CCFE sends ProjectState = OFF.
        Down = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Created => "CREATED",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Down => "DOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "CREATED" => Some(Self::Created),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "DOWN" => Some(Self::Down),
                _ => None,
            }
        }
    }
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppConnectorOperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod app_connectors_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// API Overview:
    ///
    /// The `beyondcorp.googleapis.com` service implements the Google Cloud
    /// BeyondCorp API.
    ///
    /// Data Model:
    ///
    /// The AppConnectorsService exposes the following resource:
    ///
    /// * AppConnectors, named as follows:
    ///   `projects/{project_id}/locations/{location_id}/appConnectors/{app_connector_id}`.
    ///
    /// The AppConnectorsService provides methods to manage
    /// (create/read/update/delete) BeyondCorp AppConnectors.
    #[derive(Debug, Clone)]
    pub struct AppConnectorsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AppConnectorsServiceClient<T>
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
        ) -> AppConnectorsServiceClient<InterceptedService<T, F>>
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
            AppConnectorsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists AppConnectors in a given project and location.
        pub async fn list_app_connectors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAppConnectorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAppConnectorsResponse>,
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
                "/google.cloud.beyondcorp.appconnectors.v1.AppConnectorsService/ListAppConnectors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.beyondcorp.appconnectors.v1.AppConnectorsService",
                        "ListAppConnectors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single AppConnector.
        pub async fn get_app_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAppConnectorRequest>,
        ) -> std::result::Result<tonic::Response<super::AppConnector>, tonic::Status> {
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
                "/google.cloud.beyondcorp.appconnectors.v1.AppConnectorsService/GetAppConnector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.beyondcorp.appconnectors.v1.AppConnectorsService",
                        "GetAppConnector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new AppConnector in a given project and location.
        pub async fn create_app_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAppConnectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.appconnectors.v1.AppConnectorsService/CreateAppConnector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.beyondcorp.appconnectors.v1.AppConnectorsService",
                        "CreateAppConnector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single AppConnector.
        pub async fn update_app_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAppConnectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.appconnectors.v1.AppConnectorsService/UpdateAppConnector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.beyondcorp.appconnectors.v1.AppConnectorsService",
                        "UpdateAppConnector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single AppConnector.
        pub async fn delete_app_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAppConnectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.appconnectors.v1.AppConnectorsService/DeleteAppConnector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.beyondcorp.appconnectors.v1.AppConnectorsService",
                        "DeleteAppConnector",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Report status for a given connector.
        pub async fn report_status(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.appconnectors.v1.AppConnectorsService/ReportStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.beyondcorp.appconnectors.v1.AppConnectorsService",
                        "ReportStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
