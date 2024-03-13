/// A field of an index.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexField {
    /// The path of the field. Must match the field path specification described
    /// by [google.firestore.v1beta1.Document.fields][fields].
    /// Special field path `__name__` may be used by itself or at the end of a
    /// path. `__type__` may be used only at the end of path.
    #[prost(string, tag = "1")]
    pub field_path: ::prost::alloc::string::String,
    /// The field's mode.
    #[prost(enumeration = "index_field::Mode", tag = "2")]
    pub mode: i32,
}
/// Nested message and enum types in `IndexField`.
pub mod index_field {
    /// The mode determines how a field is indexed.
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
    pub enum Mode {
        /// The mode is unspecified.
        Unspecified = 0,
        /// The field's values are indexed so as to support sequencing in
        /// ascending order and also query by <, >, <=, >=, and =.
        Ascending = 2,
        /// The field's values are indexed so as to support sequencing in
        /// descending order and also query by <, >, <=, >=, and =.
        Descending = 3,
        /// The field's array values are indexed so as to support membership using
        /// ARRAY_CONTAINS queries.
        ArrayContains = 4,
    }
    impl Mode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mode::Unspecified => "MODE_UNSPECIFIED",
                Mode::Ascending => "ASCENDING",
                Mode::Descending => "DESCENDING",
                Mode::ArrayContains => "ARRAY_CONTAINS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "ASCENDING" => Some(Self::Ascending),
                "DESCENDING" => Some(Self::Descending),
                "ARRAY_CONTAINS" => Some(Self::ArrayContains),
                _ => None,
            }
        }
    }
}
/// An index definition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Index {
    /// The resource name of the index.
    /// Output only.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The collection ID to which this index applies. Required.
    #[prost(string, tag = "2")]
    pub collection_id: ::prost::alloc::string::String,
    /// The fields to index.
    #[prost(message, repeated, tag = "3")]
    pub fields: ::prost::alloc::vec::Vec<IndexField>,
    /// The state of the index.
    /// Output only.
    #[prost(enumeration = "index::State", tag = "6")]
    pub state: i32,
}
/// Nested message and enum types in `Index`.
pub mod index {
    /// The state of an index. During index creation, an index will be in the
    /// `CREATING` state. If the index is created successfully, it will transition
    /// to the `READY` state. If the index is not able to be created, it will
    /// transition to the `ERROR` state.
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
        /// The state is unspecified.
        Unspecified = 0,
        /// The index is being created.
        /// There is an active long-running operation for the index.
        /// The index is updated when writing a document.
        /// Some index data may exist.
        Creating = 3,
        /// The index is ready to be used.
        /// The index is updated when writing a document.
        /// The index is fully populated from all stored documents it applies to.
        Ready = 2,
        /// The index was being created, but something went wrong.
        /// There is no active long-running operation for the index,
        /// and the most recently finished long-running operation failed.
        /// The index is not updated when writing a document.
        /// Some index data may exist.
        Error = 5,
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
                State::Ready => "READY",
                State::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// The metadata message for [google.cloud.location.Location.metadata][google.cloud.location.Location.metadata].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {}
/// Metadata for index operations. This metadata populates
/// the metadata field of [google.longrunning.Operation][google.longrunning.Operation].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexOperationMetadata {
    /// The time that work began on the operation.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation ended, either successfully or otherwise. Unset if
    /// the operation is still active.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The index resource that this operation is acting on. For example:
    /// `projects/{project_id}/databases/{database_id}/indexes/{index_id}`
    #[prost(string, tag = "3")]
    pub index: ::prost::alloc::string::String,
    /// The type of index operation.
    #[prost(enumeration = "index_operation_metadata::OperationType", tag = "4")]
    pub operation_type: i32,
    /// True if the \[google.longrunning.Operation\] was cancelled. If the
    /// cancellation is in progress, cancelled will be true but
    /// [google.longrunning.Operation.done][google.longrunning.Operation.done] will be false.
    #[prost(bool, tag = "5")]
    pub cancelled: bool,
    /// Progress of the existing operation, measured in number of documents.
    #[prost(message, optional, tag = "6")]
    pub document_progress: ::core::option::Option<Progress>,
}
/// Nested message and enum types in `IndexOperationMetadata`.
pub mod index_operation_metadata {
    /// The type of index operation.
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
    pub enum OperationType {
        /// Unspecified. Never set by server.
        Unspecified = 0,
        /// The operation is creating the index. Initiated by a `CreateIndex` call.
        CreatingIndex = 1,
    }
    impl OperationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OperationType::Unspecified => "OPERATION_TYPE_UNSPECIFIED",
                OperationType::CreatingIndex => "CREATING_INDEX",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING_INDEX" => Some(Self::CreatingIndex),
                _ => None,
            }
        }
    }
}
/// Measures the progress of a particular metric.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Progress {
    /// An estimate of how much work has been completed. Note that this may be
    /// greater than `work_estimated`.
    #[prost(int64, tag = "1")]
    pub work_completed: i64,
    /// An estimate of how much work needs to be performed. Zero if the
    /// work estimate is unavailable. May change as work progresses.
    #[prost(int64, tag = "2")]
    pub work_estimated: i64,
}
/// The request for [FirestoreAdmin.CreateIndex][google.firestore.admin.v1beta1.FirestoreAdmin.CreateIndex].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIndexRequest {
    /// The name of the database this index will apply to. For example:
    /// `projects/{project_id}/databases/{database_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The index to create. The name and state fields are output only and will be
    /// ignored. Certain single field indexes cannot be created or deleted.
    #[prost(message, optional, tag = "2")]
    pub index: ::core::option::Option<Index>,
}
/// The request for [FirestoreAdmin.GetIndex][google.firestore.admin.v1beta1.FirestoreAdmin.GetIndex].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIndexRequest {
    /// The name of the index. For example:
    /// `projects/{project_id}/databases/{database_id}/indexes/{index_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for [FirestoreAdmin.ListIndexes][google.firestore.admin.v1beta1.FirestoreAdmin.ListIndexes].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIndexesRequest {
    /// The database name. For example:
    /// `projects/{project_id}/databases/{database_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard List page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard List page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The request for [FirestoreAdmin.DeleteIndex][google.firestore.admin.v1beta1.FirestoreAdmin.DeleteIndex].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIndexRequest {
    /// The index name. For example:
    /// `projects/{project_id}/databases/{database_id}/indexes/{index_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The response for [FirestoreAdmin.ListIndexes][google.firestore.admin.v1beta1.FirestoreAdmin.ListIndexes].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIndexesResponse {
    /// The indexes.
    #[prost(message, repeated, tag = "1")]
    pub indexes: ::prost::alloc::vec::Vec<Index>,
    /// The standard List next-page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for [FirestoreAdmin.ExportDocuments][google.firestore.admin.v1beta1.FirestoreAdmin.ExportDocuments].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDocumentsRequest {
    /// Database to export. Should be of the form:
    /// `projects/{project_id}/databases/{database_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Which collection ids to export. Unspecified means all collections.
    #[prost(string, repeated, tag = "3")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The output URI. Currently only supports Google Cloud Storage URIs of the
    /// form: `gs://BUCKET_NAME\[/NAMESPACE_PATH\]`, where `BUCKET_NAME` is the name
    /// of the Google Cloud Storage bucket and `NAMESPACE_PATH` is an optional
    /// Google Cloud Storage namespace path. When
    /// choosing a name, be sure to consider Google Cloud Storage naming
    /// guidelines: <https://cloud.google.com/storage/docs/naming.>
    /// If the URI is a bucket (without a namespace path), a prefix will be
    /// generated based on the start time.
    #[prost(string, tag = "4")]
    pub output_uri_prefix: ::prost::alloc::string::String,
}
/// The request for [FirestoreAdmin.ImportDocuments][google.firestore.admin.v1beta1.FirestoreAdmin.ImportDocuments].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsRequest {
    /// Database to import into. Should be of the form:
    /// `projects/{project_id}/databases/{database_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Which collection ids to import. Unspecified means all collections included
    /// in the import.
    #[prost(string, repeated, tag = "3")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Location of the exported files.
    /// This must match the output_uri_prefix of an ExportDocumentsResponse from
    /// an export that has completed successfully.
    /// See:
    /// [google.firestore.admin.v1beta1.ExportDocumentsResponse.output_uri_prefix][google.firestore.admin.v1beta1.ExportDocumentsResponse.output_uri_prefix].
    #[prost(string, tag = "4")]
    pub input_uri_prefix: ::prost::alloc::string::String,
}
/// Returned in the [google.longrunning.Operation][google.longrunning.Operation] response field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDocumentsResponse {
    /// Location of the output files. This can be used to begin an import
    /// into Cloud Firestore (this project or another project) after the operation
    /// completes successfully.
    #[prost(string, tag = "1")]
    pub output_uri_prefix: ::prost::alloc::string::String,
}
/// Metadata for ExportDocuments operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDocumentsMetadata {
    /// The time that work began on the operation.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation ended, either successfully or otherwise. Unset if
    /// the operation is still active.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The state of the export operation.
    #[prost(enumeration = "OperationState", tag = "3")]
    pub operation_state: i32,
    /// An estimate of the number of documents processed.
    #[prost(message, optional, tag = "4")]
    pub progress_documents: ::core::option::Option<Progress>,
    /// An estimate of the number of bytes processed.
    #[prost(message, optional, tag = "5")]
    pub progress_bytes: ::core::option::Option<Progress>,
    /// Which collection ids are being exported.
    #[prost(string, repeated, tag = "6")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Where the entities are being exported to.
    #[prost(string, tag = "7")]
    pub output_uri_prefix: ::prost::alloc::string::String,
}
/// Metadata for ImportDocuments operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsMetadata {
    /// The time that work began on the operation.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation ended, either successfully or otherwise. Unset if
    /// the operation is still active.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The state of the import operation.
    #[prost(enumeration = "OperationState", tag = "3")]
    pub operation_state: i32,
    /// An estimate of the number of documents processed.
    #[prost(message, optional, tag = "4")]
    pub progress_documents: ::core::option::Option<Progress>,
    /// An estimate of the number of bytes processed.
    #[prost(message, optional, tag = "5")]
    pub progress_bytes: ::core::option::Option<Progress>,
    /// Which collection ids are being imported.
    #[prost(string, repeated, tag = "6")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The location of the documents being imported.
    #[prost(string, tag = "7")]
    pub input_uri_prefix: ::prost::alloc::string::String,
}
/// The various possible states for an ongoing Operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationState {
    /// Unspecified.
    StateUnspecified = 0,
    /// Request is being prepared for processing.
    Initializing = 1,
    /// Request is actively being processed.
    Processing = 2,
    /// Request is in the process of being cancelled after user called
    /// google.longrunning.Operations.CancelOperation on the operation.
    Cancelling = 3,
    /// Request has been processed and is in its finalization stage.
    Finalizing = 4,
    /// Request has completed successfully.
    Successful = 5,
    /// Request has finished being processed, but encountered an error.
    Failed = 6,
    /// Request has finished being cancelled after user called
    /// google.longrunning.Operations.CancelOperation.
    Cancelled = 7,
}
impl OperationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationState::StateUnspecified => "STATE_UNSPECIFIED",
            OperationState::Initializing => "INITIALIZING",
            OperationState::Processing => "PROCESSING",
            OperationState::Cancelling => "CANCELLING",
            OperationState::Finalizing => "FINALIZING",
            OperationState::Successful => "SUCCESSFUL",
            OperationState::Failed => "FAILED",
            OperationState::Cancelled => "CANCELLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNSPECIFIED" => Some(Self::StateUnspecified),
            "INITIALIZING" => Some(Self::Initializing),
            "PROCESSING" => Some(Self::Processing),
            "CANCELLING" => Some(Self::Cancelling),
            "FINALIZING" => Some(Self::Finalizing),
            "SUCCESSFUL" => Some(Self::Successful),
            "FAILED" => Some(Self::Failed),
            "CANCELLED" => Some(Self::Cancelled),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod firestore_admin_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Cloud Firestore Admin API.
    ///
    /// This API provides several administrative services for Cloud Firestore.
    ///
    /// # Concepts
    ///
    /// Project, Database, Namespace, Collection, and Document are used as defined in
    /// the Google Cloud Firestore API.
    ///
    /// Operation: An Operation represents work being performed in the background.
    ///
    ///
    /// # Services
    ///
    /// ## Index
    ///
    /// The index service manages Cloud Firestore indexes.
    ///
    /// Index creation is performed asynchronously.
    /// An Operation resource is created for each such asynchronous operation.
    /// The state of the operation (including any errors encountered)
    /// may be queried via the Operation resource.
    ///
    /// ## Metadata
    ///
    /// Provides metadata and statistical information about data in Cloud Firestore.
    /// The data provided as part of this API may be stale.
    ///
    /// ## Operation
    ///
    /// The Operations collection provides a record of actions performed for the
    /// specified Project (including any Operations in progress). Operations are not
    /// created directly but through calls on other collections or resources.
    ///
    /// An Operation that is not yet done may be cancelled. The request to cancel is
    /// asynchronous and the Operation may continue to run for some time after the
    /// request to cancel is made.
    ///
    /// An Operation that is done may be deleted so that it is no longer listed as
    /// part of the Operation collection.
    ///
    /// Operations are created by service `FirestoreAdmin`, but are accessed via
    /// service `google.longrunning.Operations`.
    #[derive(Debug, Clone)]
    pub struct FirestoreAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FirestoreAdminClient<T>
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
        ) -> FirestoreAdminClient<InterceptedService<T, F>>
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
            FirestoreAdminClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates the specified index.
        /// A newly created index's initial state is `CREATING`. On completion of the
        /// returned [google.longrunning.Operation][google.longrunning.Operation], the state will be `READY`.
        /// If the index already exists, the call will return an `ALREADY_EXISTS`
        /// status.
        ///
        /// During creation, the process could result in an error, in which case the
        /// index will move to the `ERROR` state. The process can be recovered by
        /// fixing the data that caused the error, removing the index with
        /// [delete][google.firestore.admin.v1beta1.FirestoreAdmin.DeleteIndex], then re-creating the index with
        /// [create][google.firestore.admin.v1beta1.FirestoreAdmin.CreateIndex].
        ///
        /// Indexes with a single field cannot be created.
        pub async fn create_index(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateIndexRequest>,
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/CreateIndex",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.firestore.admin.v1beta1.FirestoreAdmin",
                        "CreateIndex",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the indexes that match the specified filters.
        pub async fn list_indexes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIndexesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListIndexesResponse>,
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/ListIndexes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.firestore.admin.v1beta1.FirestoreAdmin",
                        "ListIndexes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets an index.
        pub async fn get_index(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIndexRequest>,
        ) -> std::result::Result<tonic::Response<super::Index>, tonic::Status> {
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/GetIndex",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.firestore.admin.v1beta1.FirestoreAdmin",
                        "GetIndex",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an index.
        pub async fn delete_index(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIndexRequest>,
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/DeleteIndex",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.firestore.admin.v1beta1.FirestoreAdmin",
                        "DeleteIndex",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Exports a copy of all or a subset of documents from Google Cloud Firestore
        /// to another storage system, such as Google Cloud Storage. Recent updates to
        /// documents may not be reflected in the export. The export occurs in the
        /// background and its progress can be monitored and managed via the
        /// Operation resource that is created. The output of an export may only be
        /// used once the associated operation is done. If an export operation is
        /// cancelled before completion it may leave partial data behind in Google
        /// Cloud Storage.
        pub async fn export_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportDocumentsRequest>,
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/ExportDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.firestore.admin.v1beta1.FirestoreAdmin",
                        "ExportDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Imports documents into Google Cloud Firestore. Existing documents with the
        /// same name are overwritten. The import occurs in the background and its
        /// progress can be monitored and managed via the Operation resource that is
        /// created. If an ImportDocuments operation is cancelled, it is possible
        /// that a subset of the data has already been imported to Cloud Firestore.
        pub async fn import_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportDocumentsRequest>,
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/ImportDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.firestore.admin.v1beta1.FirestoreAdmin",
                        "ImportDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
