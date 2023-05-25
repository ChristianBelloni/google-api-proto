/// Message describing a MC Source of type Guest OS Scan.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestOsScan {
    /// reference to the corresponding Guest OS Scan in MC Source.
    #[prost(string, tag = "1")]
    pub core_source: ::prost::alloc::string::String,
}
/// Message describing a MC Source of type VSphere Scan.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VSphereScan {
    /// reference to the corresponding VSphere Scan in MC Source.
    #[prost(string, tag = "1")]
    pub core_source: ::prost::alloc::string::String,
}
/// Message describing Collector object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collector {
    /// name of resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Create time stamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update time stamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// User specified name of the Collector.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// User specified description of the Collector.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// Service Account email used to ingest data to this Collector.
    #[prost(string, tag = "7")]
    pub service_account: ::prost::alloc::string::String,
    /// Output only. Store cloud storage bucket name (which is a guid) created with
    /// this Collector.
    #[prost(string, tag = "8")]
    pub bucket: ::prost::alloc::string::String,
    /// User specified expected asset count.
    #[prost(int64, tag = "9")]
    pub expected_asset_count: i64,
    /// Output only. State of the Collector.
    #[prost(enumeration = "collector::State", tag = "10")]
    pub state: i32,
    /// Output only. Client version.
    #[prost(string, tag = "11")]
    pub client_version: ::prost::alloc::string::String,
    /// Output only. Reference to MC Source Guest Os Scan.
    #[prost(message, optional, tag = "12")]
    pub guest_os_scan: ::core::option::Option<GuestOsScan>,
    /// Output only. Reference to MC Source vsphere_scan.
    #[prost(message, optional, tag = "13")]
    pub vsphere_scan: ::core::option::Option<VSphereScan>,
    /// How many days to collect data.
    #[prost(int32, tag = "14")]
    pub collection_days: i32,
    /// Uri for EULA (End User License Agreement) from customer.
    #[prost(string, tag = "15")]
    pub eula_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Collector`.
pub mod collector {
    /// -- Using suggestion from API Linter Analyzer for nesting enum --
    /// -- <https://linter.aip.dev/216/nesting> --
    /// State of a Collector (server_side).
    /// States are used for internal purposes and named to keep
    /// convention of legacy product:
    /// <https://cloud.google.com/migrate/stratozone/docs/about-stratoprobe.>
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
        /// Collector state is not recognized.
        Unspecified = 0,
        /// Collector started to create, but hasn't been completed MC source creation
        /// and db object creation.
        Initializing = 1,
        /// Collector has been created, MC source creation and db object creation
        /// completed.
        ReadyToUse = 2,
        /// Collector client has been registered with client.
        Registered = 3,
        /// Collector client is actively scanning.
        Active = 4,
        /// Collector is not actively scanning.
        Paused = 5,
        /// Collector is starting background job for deletion.
        Deleting = 6,
        /// Collector completed all tasks for deletion.
        Decommissioned = 7,
        /// Collector is in error state.
        Error = 8,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Initializing => "STATE_INITIALIZING",
                State::ReadyToUse => "STATE_READY_TO_USE",
                State::Registered => "STATE_REGISTERED",
                State::Active => "STATE_ACTIVE",
                State::Paused => "STATE_PAUSED",
                State::Deleting => "STATE_DELETING",
                State::Decommissioned => "STATE_DECOMMISSIONED",
                State::Error => "STATE_ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATE_INITIALIZING" => Some(Self::Initializing),
                "STATE_READY_TO_USE" => Some(Self::ReadyToUse),
                "STATE_REGISTERED" => Some(Self::Registered),
                "STATE_ACTIVE" => Some(Self::Active),
                "STATE_PAUSED" => Some(Self::Paused),
                "STATE_DELETING" => Some(Self::Deleting),
                "STATE_DECOMMISSIONED" => Some(Self::Decommissioned),
                "STATE_ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// Message describing an Annotation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Annotation {
    /// name of resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Create time stamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update time stamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Type of an annotation.
    #[prost(enumeration = "annotation::Type", tag = "5")]
    pub r#type: i32,
}
/// Nested message and enum types in `Annotation`.
pub mod annotation {
    /// Types for project level setting.
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
        /// Unknown type
        Unspecified = 0,
        /// Indicates that this project has opted into StratoZone export.
        LegacyExportConsent = 1,
        /// Indicates that this project is created by Qwiklab.
        Qwiklab = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::LegacyExportConsent => "TYPE_LEGACY_EXPORT_CONSENT",
                Type::Qwiklab => "TYPE_QWIKLAB",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "TYPE_LEGACY_EXPORT_CONSENT" => Some(Self::LegacyExportConsent),
                "TYPE_QWIKLAB" => Some(Self::Qwiklab),
                _ => None,
            }
        }
    }
}
/// Message for creating an AnnotationS.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAnnotationRequest {
    /// Required. Name of the parent (project+location).
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "2")]
    pub annotation: ::core::option::Option<Annotation>,
    /// Optional. An optional request ID to identify requests.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for getting a specific Annotation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAnnotationRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a Collector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCollectorRequest {
    /// Required. Name of the parent (project+location).
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object.
    #[prost(string, tag = "2")]
    pub collector_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub collector: ::core::option::Option<Collector>,
    /// Optional. An optional request ID to identify requests.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for requesting list of Collectors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCollectorsRequest {
    /// Required. Parent value for ListCollectorsRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Collectors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCollectorsResponse {
    /// The list of Collectors.
    #[prost(message, repeated, tag = "1")]
    pub collectors: ::prost::alloc::vec::Vec<Collector>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a specific Collector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectorRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for deleting a Collector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollectorRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating a Collector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectorRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Collector resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag = "2")]
    pub collector: ::core::option::Option<Collector>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for resuming a Collector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeCollectorRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for registering a Collector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterCollectorRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for pausing a Collector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseCollectorRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
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
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod rapid_migration_assessment_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Rapid Migration Assessment service
    #[derive(Debug, Clone)]
    pub struct RapidMigrationAssessmentClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RapidMigrationAssessmentClient<T>
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
        ) -> RapidMigrationAssessmentClient<InterceptedService<T, F>>
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
            RapidMigrationAssessmentClient::new(
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
        /// Create a Collector to manage the on-prem appliance which collects
        /// information about Customer assets.
        pub async fn create_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCollectorRequest>,
        ) -> Result<
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
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/CreateCollector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an Annotation
        pub async fn create_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAnnotationRequest>,
        ) -> Result<
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
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/CreateAnnotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Annotation.
        pub async fn get_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAnnotationRequest>,
        ) -> Result<tonic::Response<super::Annotation>, tonic::Status> {
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
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/GetAnnotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Collectors in a given project and location.
        pub async fn list_collectors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCollectorsRequest>,
        ) -> Result<tonic::Response<super::ListCollectorsResponse>, tonic::Status> {
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
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/ListCollectors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Collector.
        pub async fn get_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCollectorRequest>,
        ) -> Result<tonic::Response<super::Collector>, tonic::Status> {
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
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/GetCollector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Collector.
        pub async fn update_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCollectorRequest>,
        ) -> Result<
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
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/UpdateCollector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Collector - changes state of collector to "Deleting".
        pub async fn delete_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCollectorRequest>,
        ) -> Result<
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
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/DeleteCollector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Resumes the given collector.
        pub async fn resume_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeCollectorRequest>,
        ) -> Result<
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
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/ResumeCollector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Registers the given collector.
        pub async fn register_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterCollectorRequest>,
        ) -> Result<
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
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/RegisterCollector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Pauses the given collector.
        pub async fn pause_collector(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseCollectorRequest>,
        ) -> Result<
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
                "/google.cloud.rapidmigrationassessment.v1.RapidMigrationAssessment/PauseCollector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
