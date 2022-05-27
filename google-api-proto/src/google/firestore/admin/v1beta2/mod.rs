/// Cloud Firestore indexes enable simple and complex queries against
/// documents in a database.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Index {
    /// Output only. A server defined name for this index.
    /// The form of this name for composite indexes will be:
    /// `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{composite_index_id}`
    /// For single field indexes, this field will be empty.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Indexes with a collection query scope specified allow queries
    /// against a collection that is the child of a specific document, specified at
    /// query time, and that has the same collection id.
    ///
    /// Indexes with a collection group query scope specified allow queries against
    /// all collections descended from a specific document, specified at query
    /// time, and that have the same collection id as this index.
    #[prost(enumeration="index::QueryScope", tag="2")]
    pub query_scope: i32,
    /// The fields supported by this index.
    ///
    /// For composite indexes, this is always 2 or more fields.
    /// The last field entry is always for the field path `__name__`. If, on
    /// creation, `__name__` was not specified as the last field, it will be added
    /// automatically with the same direction as that of the last field defined. If
    /// the final field in a composite index is not directional, the `__name__`
    /// will be ordered ASCENDING (unless explicitly specified).
    ///
    /// For single field indexes, this will always be exactly one entry with a
    /// field path equal to the field path of the associated field.
    #[prost(message, repeated, tag="3")]
    pub fields: ::prost::alloc::vec::Vec<index::IndexField>,
    /// Output only. The serving state of the index.
    #[prost(enumeration="index::State", tag="4")]
    pub state: i32,
}
/// Nested message and enum types in `Index`.
pub mod index {
    /// A field in an index.
    /// The field_path describes which field is indexed, the value_mode describes
    /// how the field value is indexed.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IndexField {
        /// Can be __name__.
        /// For single field indexes, this must match the name of the field or may
        /// be omitted.
        #[prost(string, tag="1")]
        pub field_path: ::prost::alloc::string::String,
        /// How the field value is indexed.
        #[prost(oneof="index_field::ValueMode", tags="2, 3")]
        pub value_mode: ::core::option::Option<index_field::ValueMode>,
    }
    /// Nested message and enum types in `IndexField`.
    pub mod index_field {
        /// The supported orderings.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Order {
            /// The ordering is unspecified. Not a valid option.
            Unspecified = 0,
            /// The field is ordered by ascending field value.
            Ascending = 1,
            /// The field is ordered by descending field value.
            Descending = 2,
        }
        /// The supported array value configurations.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ArrayConfig {
            /// The index does not support additional array queries.
            Unspecified = 0,
            /// The index supports array containment queries.
            Contains = 1,
        }
        /// How the field value is indexed.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ValueMode {
            /// Indicates that this field supports ordering by the specified order or
            /// comparing using =, <, <=, >, >=.
            #[prost(enumeration="Order", tag="2")]
            Order(i32),
            /// Indicates that this field supports operations on `array_value`s.
            #[prost(enumeration="ArrayConfig", tag="3")]
            ArrayConfig(i32),
        }
    }
    /// Query Scope defines the scope at which a query is run. This is specified on
    /// a StructuredQuery's `from` field.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QueryScope {
        /// The query scope is unspecified. Not a valid option.
        Unspecified = 0,
        /// Indexes with a collection query scope specified allow queries
        /// against a collection that is the child of a specific document, specified
        /// at query time, and that has the collection id specified by the index.
        Collection = 1,
        /// Indexes with a collection group query scope specified allow queries
        /// against all collections that has the collection id specified by the
        /// index.
        CollectionGroup = 2,
    }
    /// The state of an index. During index creation, an index will be in the
    /// `CREATING` state. If the index is created successfully, it will transition
    /// to the `READY` state. If the index creation encounters a problem, the index
    /// will transition to the `NEEDS_REPAIR` state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state is unspecified.
        Unspecified = 0,
        /// The index is being created.
        /// There is an active long-running operation for the index.
        /// The index is updated when writing a document.
        /// Some index data may exist.
        Creating = 1,
        /// The index is ready to be used.
        /// The index is updated when writing a document.
        /// The index is fully populated from all stored documents it applies to.
        Ready = 2,
        /// The index was being created, but something went wrong.
        /// There is no active long-running operation for the index,
        /// and the most recently finished long-running operation failed.
        /// The index is not updated when writing a document.
        /// Some index data may exist.
        /// Use the google.longrunning.Operations API to determine why the operation
        /// that last attempted to create this index failed, then re-create the
        /// index.
        NeedsRepair = 3,
    }
}
/// Metadata for \[google.longrunning.Operation][google.longrunning.Operation\] results from
/// \[FirestoreAdmin.CreateIndex][google.firestore.admin.v1beta2.FirestoreAdmin.CreateIndex\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexOperationMetadata {
    /// The time this operation started.
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time this operation completed. Will be unset if operation still in
    /// progress.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The index resource that this operation is acting on. For example:
    /// `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{index_id}`
    #[prost(string, tag="3")]
    pub index: ::prost::alloc::string::String,
    /// The state of the operation.
    #[prost(enumeration="OperationState", tag="4")]
    pub state: i32,
    /// The progress, in documents, of this operation.
    #[prost(message, optional, tag="5")]
    pub progress_documents: ::core::option::Option<Progress>,
    /// The progress, in bytes, of this operation.
    #[prost(message, optional, tag="6")]
    pub progress_bytes: ::core::option::Option<Progress>,
}
/// Metadata for \[google.longrunning.Operation][google.longrunning.Operation\] results from
/// \[FirestoreAdmin.UpdateField][google.firestore.admin.v1beta2.FirestoreAdmin.UpdateField\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldOperationMetadata {
    /// The time this operation started.
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time this operation completed. Will be unset if operation still in
    /// progress.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The field resource that this operation is acting on. For example:
    /// `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}`
    #[prost(string, tag="3")]
    pub field: ::prost::alloc::string::String,
    /// A list of \[IndexConfigDelta][google.firestore.admin.v1beta2.FieldOperationMetadata.IndexConfigDelta\], which describe the intent of this
    /// operation.
    #[prost(message, repeated, tag="4")]
    pub index_config_deltas: ::prost::alloc::vec::Vec<field_operation_metadata::IndexConfigDelta>,
    /// The state of the operation.
    #[prost(enumeration="OperationState", tag="5")]
    pub state: i32,
    /// The progress, in documents, of this operation.
    #[prost(message, optional, tag="6")]
    pub document_progress: ::core::option::Option<Progress>,
    /// The progress, in bytes, of this operation.
    #[prost(message, optional, tag="7")]
    pub bytes_progress: ::core::option::Option<Progress>,
}
/// Nested message and enum types in `FieldOperationMetadata`.
pub mod field_operation_metadata {
    /// Information about an index configuration change.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IndexConfigDelta {
        /// Specifies how the index is changing.
        #[prost(enumeration="index_config_delta::ChangeType", tag="1")]
        pub change_type: i32,
        /// The index being changed.
        #[prost(message, optional, tag="2")]
        pub index: ::core::option::Option<super::Index>,
    }
    /// Nested message and enum types in `IndexConfigDelta`.
    pub mod index_config_delta {
        /// Specifies how the index is changing.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ChangeType {
            /// The type of change is not specified or known.
            Unspecified = 0,
            /// The single field index is being added.
            Add = 1,
            /// The single field index is being removed.
            Remove = 2,
        }
    }
}
/// Metadata for \[google.longrunning.Operation][google.longrunning.Operation\] results from
/// \[FirestoreAdmin.ExportDocuments][google.firestore.admin.v1beta2.FirestoreAdmin.ExportDocuments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDocumentsMetadata {
    /// The time this operation started.
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time this operation completed. Will be unset if operation still in
    /// progress.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The state of the export operation.
    #[prost(enumeration="OperationState", tag="3")]
    pub operation_state: i32,
    /// The progress, in documents, of this operation.
    #[prost(message, optional, tag="4")]
    pub progress_documents: ::core::option::Option<Progress>,
    /// The progress, in bytes, of this operation.
    #[prost(message, optional, tag="5")]
    pub progress_bytes: ::core::option::Option<Progress>,
    /// Which collection ids are being exported.
    #[prost(string, repeated, tag="6")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Where the entities are being exported to.
    #[prost(string, tag="7")]
    pub output_uri_prefix: ::prost::alloc::string::String,
}
/// Metadata for \[google.longrunning.Operation][google.longrunning.Operation\] results from
/// \[FirestoreAdmin.ImportDocuments][google.firestore.admin.v1beta2.FirestoreAdmin.ImportDocuments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsMetadata {
    /// The time this operation started.
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time this operation completed. Will be unset if operation still in
    /// progress.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The state of the import operation.
    #[prost(enumeration="OperationState", tag="3")]
    pub operation_state: i32,
    /// The progress, in documents, of this operation.
    #[prost(message, optional, tag="4")]
    pub progress_documents: ::core::option::Option<Progress>,
    /// The progress, in bytes, of this operation.
    #[prost(message, optional, tag="5")]
    pub progress_bytes: ::core::option::Option<Progress>,
    /// Which collection ids are being imported.
    #[prost(string, repeated, tag="6")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The location of the documents being imported.
    #[prost(string, tag="7")]
    pub input_uri_prefix: ::prost::alloc::string::String,
}
/// Returned in the \[google.longrunning.Operation][google.longrunning.Operation\] response field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDocumentsResponse {
    /// Location of the output files. This can be used to begin an import
    /// into Cloud Firestore (this project or another project) after the operation
    /// completes successfully.
    #[prost(string, tag="1")]
    pub output_uri_prefix: ::prost::alloc::string::String,
}
/// Describes the progress of the operation.
/// Unit of work is generic and must be interpreted based on where \[Progress][google.firestore.admin.v1beta2.Progress\]
/// is used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Progress {
    /// The amount of work estimated.
    #[prost(int64, tag="1")]
    pub estimated_work: i64,
    /// The amount of work completed.
    #[prost(int64, tag="2")]
    pub completed_work: i64,
}
/// Describes the state of the operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationState {
    /// Unspecified.
    Unspecified = 0,
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
/// Represents a single field in the database.
///
/// Fields are grouped by their "Collection Group", which represent all
/// collections in the database with the same id.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    /// A field name of the form
    /// `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}`
    ///
    /// A field path may be a simple field name, e.g. `address` or a path to fields
    /// within map_value , e.g. `address.city`,
    /// or a special field path. The only valid special field is `*`, which
    /// represents any field.
    ///
    /// Field paths may be quoted using ` (backtick). The only character that needs
    /// to be escaped within a quoted field path is the backtick character itself,
    /// escaped using a backslash. Special characters in field paths that
    /// must be quoted include: `*`, `.`,
    /// ``` (backtick), `[`, `]`, as well as any ascii symbolic characters.
    ///
    /// Examples:
    /// (Note: Comments here are written in markdown syntax, so there is an
    ///  additional layer of backticks to represent a code block)
    /// `\`address.city\`` represents a field named `address.city`, not the map key
    /// `city` in the field `address`.
    /// `\`*\`` represents a field named `*`, not any field.
    ///
    /// A special `Field` contains the default indexing settings for all fields.
    /// This field's resource name is:
    /// `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*`
    /// Indexes defined on this `Field` will be applied to all fields which do not
    /// have their own `Field` index configuration.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The index configuration for this field. If unset, field indexing will
    /// revert to the configuration defined by the `ancestor_field`. To
    /// explicitly remove all indexes for this field, specify an index config
    /// with an empty list of indexes.
    #[prost(message, optional, tag="2")]
    pub index_config: ::core::option::Option<field::IndexConfig>,
}
/// Nested message and enum types in `Field`.
pub mod field {
    /// The index configuration for this field.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IndexConfig {
        /// The indexes supported for this field.
        #[prost(message, repeated, tag="1")]
        pub indexes: ::prost::alloc::vec::Vec<super::Index>,
        /// Output only. When true, the `Field`'s index configuration is set from the
        /// configuration specified by the `ancestor_field`.
        /// When false, the `Field`'s index configuration is defined explicitly.
        #[prost(bool, tag="2")]
        pub uses_ancestor_config: bool,
        /// Output only. Specifies the resource name of the `Field` from which this field's
        /// index configuration is set (when `uses_ancestor_config` is true),
        /// or from which it *would* be set if this field had no index configuration
        /// (when `uses_ancestor_config` is false).
        #[prost(string, tag="3")]
        pub ancestor_field: ::prost::alloc::string::String,
        /// Output only
        /// When true, the `Field`'s index configuration is in the process of being
        /// reverted. Once complete, the index config will transition to the same
        /// state as the field specified by `ancestor_field`, at which point
        /// `uses_ancestor_config` will be `true` and `reverting` will be `false`.
        #[prost(bool, tag="4")]
        pub reverting: bool,
    }
}
/// The request for \[FirestoreAdmin.CreateIndex][google.firestore.admin.v1beta2.FirestoreAdmin.CreateIndex\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIndexRequest {
    /// A parent name of the form
    /// `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The composite index to create.
    #[prost(message, optional, tag="2")]
    pub index: ::core::option::Option<Index>,
}
/// The request for \[FirestoreAdmin.ListIndexes][google.firestore.admin.v1beta2.FirestoreAdmin.ListIndexes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIndexesRequest {
    /// A parent name of the form
    /// `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The filter to apply to list results.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// The number of results to return.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// A page token, returned from a previous call to
    /// \[FirestoreAdmin.ListIndexes][google.firestore.admin.v1beta2.FirestoreAdmin.ListIndexes\], that may be used to get the next
    /// page of results.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for \[FirestoreAdmin.ListIndexes][google.firestore.admin.v1beta2.FirestoreAdmin.ListIndexes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIndexesResponse {
    /// The requested indexes.
    #[prost(message, repeated, tag="1")]
    pub indexes: ::prost::alloc::vec::Vec<Index>,
    /// A page token that may be used to request another page of results. If blank,
    /// this is the last page.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for \[FirestoreAdmin.GetIndex][google.firestore.admin.v1beta2.FirestoreAdmin.GetIndex\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIndexRequest {
    /// A name of the form
    /// `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{index_id}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for \[FirestoreAdmin.DeleteIndex][google.firestore.admin.v1beta2.FirestoreAdmin.DeleteIndex\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIndexRequest {
    /// A name of the form
    /// `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{index_id}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for \[FirestoreAdmin.UpdateField][google.firestore.admin.v1beta2.FirestoreAdmin.UpdateField\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFieldRequest {
    /// The field to be updated.
    #[prost(message, optional, tag="1")]
    pub field: ::core::option::Option<Field>,
    /// A mask, relative to the field. If specified, only configuration specified
    /// by this field_mask will be updated in the field.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request for \[FirestoreAdmin.GetField][google.firestore.admin.v1beta2.FirestoreAdmin.GetField\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFieldRequest {
    /// A name of the form
    /// `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_id}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for \[FirestoreAdmin.ListFields][google.firestore.admin.v1beta2.FirestoreAdmin.ListFields\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFieldsRequest {
    /// A parent name of the form
    /// `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The filter to apply to list results. Currently,
    /// \[FirestoreAdmin.ListFields][google.firestore.admin.v1beta2.FirestoreAdmin.ListFields\] only supports listing fields
    /// that have been explicitly overridden. To issue this query, call
    /// \[FirestoreAdmin.ListFields][google.firestore.admin.v1beta2.FirestoreAdmin.ListFields\] with the filter set to
    /// `indexConfig.usesAncestorConfig:false`.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// The number of results to return.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// A page token, returned from a previous call to
    /// \[FirestoreAdmin.ListFields][google.firestore.admin.v1beta2.FirestoreAdmin.ListFields\], that may be used to get the next
    /// page of results.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for \[FirestoreAdmin.ListFields][google.firestore.admin.v1beta2.FirestoreAdmin.ListFields\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFieldsResponse {
    /// The requested fields.
    #[prost(message, repeated, tag="1")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
    /// A page token that may be used to request another page of results. If blank,
    /// this is the last page.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for \[FirestoreAdmin.ExportDocuments][google.firestore.admin.v1beta2.FirestoreAdmin.ExportDocuments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDocumentsRequest {
    /// Database to export. Should be of the form:
    /// `projects/{project_id}/databases/{database_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Which collection ids to export. Unspecified means all collections.
    #[prost(string, repeated, tag="2")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The output URI. Currently only supports Google Cloud Storage URIs of the
    /// form: `gs://BUCKET_NAME\[/NAMESPACE_PATH\]`, where `BUCKET_NAME` is the name
    /// of the Google Cloud Storage bucket and `NAMESPACE_PATH` is an optional
    /// Google Cloud Storage namespace path. When
    /// choosing a name, be sure to consider Google Cloud Storage naming
    /// guidelines: <https://cloud.google.com/storage/docs/naming.>
    /// If the URI is a bucket (without a namespace path), a prefix will be
    /// generated based on the start time.
    #[prost(string, tag="3")]
    pub output_uri_prefix: ::prost::alloc::string::String,
}
/// The request for \[FirestoreAdmin.ImportDocuments][google.firestore.admin.v1beta2.FirestoreAdmin.ImportDocuments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsRequest {
    /// Database to import into. Should be of the form:
    /// `projects/{project_id}/databases/{database_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Which collection ids to import. Unspecified means all collections included
    /// in the import.
    #[prost(string, repeated, tag="2")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Location of the exported files.
    /// This must match the output_uri_prefix of an ExportDocumentsResponse from
    /// an export that has completed successfully.
    /// See:
    /// \[google.firestore.admin.v1beta2.ExportDocumentsResponse.output_uri_prefix][google.firestore.admin.v1beta2.ExportDocumentsResponse.output_uri_prefix\].
    #[prost(string, tag="3")]
    pub input_uri_prefix: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod firestore_admin_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
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
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Creates a composite index. This returns a [google.longrunning.Operation][google.longrunning.Operation]
        /// which may be used to track the status of the creation. The metadata for
        /// the operation will be the type [IndexOperationMetadata][google.firestore.admin.v1beta2.IndexOperationMetadata].
        pub async fn create_index(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateIndexRequest>,
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
                "/google.firestore.admin.v1beta2.FirestoreAdmin/CreateIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists composite indexes.
        pub async fn list_indexes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIndexesRequest>,
        ) -> Result<tonic::Response<super::ListIndexesResponse>, tonic::Status> {
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
                "/google.firestore.admin.v1beta2.FirestoreAdmin/ListIndexes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a composite index.
        pub async fn get_index(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIndexRequest>,
        ) -> Result<tonic::Response<super::Index>, tonic::Status> {
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
                "/google.firestore.admin.v1beta2.FirestoreAdmin/GetIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a composite index.
        pub async fn delete_index(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIndexRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.firestore.admin.v1beta2.FirestoreAdmin/DeleteIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the metadata and configuration for a Field.
        pub async fn get_field(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFieldRequest>,
        ) -> Result<tonic::Response<super::Field>, tonic::Status> {
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
                "/google.firestore.admin.v1beta2.FirestoreAdmin/GetField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a field configuration. Currently, field updates apply only to
        /// single field index configuration. However, calls to
        /// [FirestoreAdmin.UpdateField][google.firestore.admin.v1beta2.FirestoreAdmin.UpdateField] should provide a field mask to avoid
        /// changing any configuration that the caller isn't aware of. The field mask
        /// should be specified as: `{ paths: "index_config" }`.
        ///
        /// This call returns a [google.longrunning.Operation][google.longrunning.Operation] which may be used to
        /// track the status of the field update. The metadata for
        /// the operation will be the type [FieldOperationMetadata][google.firestore.admin.v1beta2.FieldOperationMetadata].
        ///
        /// To configure the default field settings for the database, use
        /// the special `Field` with resource name:
        /// `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*`.
        pub async fn update_field(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFieldRequest>,
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
                "/google.firestore.admin.v1beta2.FirestoreAdmin/UpdateField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the field configuration and metadata for this database.
        ///
        /// Currently, [FirestoreAdmin.ListFields][google.firestore.admin.v1beta2.FirestoreAdmin.ListFields] only supports listing fields
        /// that have been explicitly overridden. To issue this query, call
        /// [FirestoreAdmin.ListFields][google.firestore.admin.v1beta2.FirestoreAdmin.ListFields] with the filter set to
        /// `indexConfig.usesAncestorConfig:false`.
        pub async fn list_fields(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFieldsRequest>,
        ) -> Result<tonic::Response<super::ListFieldsResponse>, tonic::Status> {
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
                "/google.firestore.admin.v1beta2.FirestoreAdmin/ListFields",
            );
            self.inner.unary(request.into_request(), path, codec).await
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
                "/google.firestore.admin.v1beta2.FirestoreAdmin/ExportDocuments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Imports documents into Google Cloud Firestore. Existing documents with the
        /// same name are overwritten. The import occurs in the background and its
        /// progress can be monitored and managed via the Operation resource that is
        /// created. If an ImportDocuments operation is cancelled, it is possible
        /// that a subset of the data has already been imported to Cloud Firestore.
        pub async fn import_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportDocumentsRequest>,
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
                "/google.firestore.admin.v1beta2.FirestoreAdmin/ImportDocuments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
