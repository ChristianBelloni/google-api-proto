/// Catalog is the container of databases.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Catalog {
    /// Output only. The resource name.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation time of the catalog.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last modification time of the catalog.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The deletion time of the catalog. Only set after the catalog
    /// is deleted.
    #[prost(message, optional, tag = "4")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when this catalog is considered expired. Only set
    /// after the catalog is deleted.
    #[prost(message, optional, tag = "5")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Database is the container of tables.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
    /// Output only. The resource name.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation time of the database.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last modification time of the database.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The deletion time of the database. Only set after the database
    /// is deleted.
    #[prost(message, optional, tag = "4")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when this database is considered expired. Only set
    /// after the database is deleted.
    #[prost(message, optional, tag = "5")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The database type.
    #[prost(enumeration = "database::Type", tag = "6")]
    pub r#type: i32,
    /// Options specified for the database type.
    #[prost(oneof = "database::Options", tags = "7")]
    pub options: ::core::option::Option<database::Options>,
}
/// Nested message and enum types in `Database`.
pub mod database {
    /// The database type.
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
        /// The type is not specified.
        Unspecified = 0,
        /// Represents a database storing tables compatible with Hive Metastore
        /// tables.
        Hive = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Hive => "HIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "HIVE" => Some(Self::Hive),
                _ => None,
            }
        }
    }
    /// Options specified for the database type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Options {
        /// Options of a Hive database.
        #[prost(message, tag = "7")]
        HiveOptions(super::HiveDatabaseOptions),
    }
}
/// Represents a table.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Table {
    /// Output only. The resource name.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}/tables/{table_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation time of the table.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last modification time of the table.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The deletion time of the table. Only set after the table is
    /// deleted.
    #[prost(message, optional, tag = "4")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when this table is considered expired. Only set after
    /// the table is deleted.
    #[prost(message, optional, tag = "5")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The table type.
    #[prost(enumeration = "table::Type", tag = "6")]
    pub r#type: i32,
    /// The checksum of a table object computed by the server based on the value of
    /// other fields. It may be sent on update requests to ensure the client has an
    /// up-to-date value before proceeding. It is only checked for update table
    /// operations.
    #[prost(string, tag = "8")]
    pub etag: ::prost::alloc::string::String,
    /// Options specified for the table type.
    #[prost(oneof = "table::Options", tags = "7")]
    pub options: ::core::option::Option<table::Options>,
}
/// Nested message and enum types in `Table`.
pub mod table {
    /// The table type.
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
        /// The type is not specified.
        Unspecified = 0,
        /// Represents a table compatible with Hive Metastore tables.
        Hive = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Hive => "HIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "HIVE" => Some(Self::Hive),
                _ => None,
            }
        }
    }
    /// Options specified for the table type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Options {
        /// Options of a Hive table.
        #[prost(message, tag = "7")]
        HiveOptions(super::HiveTableOptions),
    }
}
/// Request message for the CreateCatalog method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCatalogRequest {
    /// Required. The parent resource where this catalog will be created.
    /// Format: projects/{project_id_or_number}/locations/{location_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The catalog to create.
    /// The `name` field does not need to be provided.
    #[prost(message, optional, tag = "2")]
    pub catalog: ::core::option::Option<Catalog>,
    /// Required. The ID to use for the catalog, which will become the final
    /// component of the catalog's resource name.
    #[prost(string, tag = "3")]
    pub catalog_id: ::prost::alloc::string::String,
}
/// Request message for the DeleteCatalog method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCatalogRequest {
    /// Required. The name of the catalog to delete.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the GetCatalog method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCatalogRequest {
    /// Required. The name of the catalog to retrieve.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the ListCatalogs method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogsRequest {
    /// Required. The parent, which owns this collection of catalogs.
    /// Format: projects/{project_id_or_number}/locations/{location_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of catalogs to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 50 catalogs will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListCatalogs` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCatalogs` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the ListCatalogs method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogsResponse {
    /// The catalogs from the specified project.
    #[prost(message, repeated, tag = "1")]
    pub catalogs: ::prost::alloc::vec::Vec<Catalog>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the CreateDatabase method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatabaseRequest {
    /// Required. The parent resource where this database will be created.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The database to create.
    /// The `name` field does not need to be provided.
    #[prost(message, optional, tag = "2")]
    pub database: ::core::option::Option<Database>,
    /// Required. The ID to use for the database, which will become the final
    /// component of the database's resource name.
    #[prost(string, tag = "3")]
    pub database_id: ::prost::alloc::string::String,
}
/// Request message for the DeleteDatabase method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatabaseRequest {
    /// Required. The name of the database to delete.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the UpdateDatabase method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatabaseRequest {
    /// Required. The database to update.
    ///
    /// The database's `name` field is used to identify the database to update.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}
    #[prost(message, optional, tag = "1")]
    pub database: ::core::option::Option<Database>,
    /// The list of fields to update.
    ///
    /// For the `FieldMask` definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    /// If not set, defaults to all of the fields that are allowed to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for the GetDatabase method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabaseRequest {
    /// Required. The name of the database to retrieve.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the ListDatabases method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatabasesRequest {
    /// Required. The parent, which owns this collection of databases.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of databases to return. The service may return fewer
    /// than this value.
    /// If unspecified, at most 50 databases will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListDatabases` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListDatabases` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the ListDatabases method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatabasesResponse {
    /// The databases from the specified catalog.
    #[prost(message, repeated, tag = "1")]
    pub databases: ::prost::alloc::vec::Vec<Database>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the CreateTable method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableRequest {
    /// Required. The parent resource where this table will be created.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The table to create. The `name` field does not need to be
    /// provided for the table creation.
    #[prost(message, optional, tag = "2")]
    pub table: ::core::option::Option<Table>,
    /// Required. The ID to use for the table, which will become the final
    /// component of the table's resource name.
    #[prost(string, tag = "3")]
    pub table_id: ::prost::alloc::string::String,
}
/// Request message for the DeleteTable method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTableRequest {
    /// Required. The name of the table to delete.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}/tables/{table_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the UpdateTable method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTableRequest {
    /// Required. The table to update.
    ///
    /// The table's `name` field is used to identify the table to update.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}/tables/{table_id}
    #[prost(message, optional, tag = "1")]
    pub table: ::core::option::Option<Table>,
    /// The list of fields to update.
    ///
    /// For the `FieldMask` definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    /// If not set, defaults to all of the fields that are allowed to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for the RenameTable method in MetastoreService
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTableRequest {
    /// Required. The table's `name` field is used to identify the table to rename.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}/tables/{table_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new `name` for the specified table, must be in the same
    /// database. Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}/tables/{table_id}
    #[prost(string, tag = "2")]
    pub new_name: ::prost::alloc::string::String,
}
/// Request message for the GetTable method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTableRequest {
    /// Required. The name of the table to retrieve.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}/tables/{table_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the ListTables method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTablesRequest {
    /// Required. The parent, which owns this collection of tables.
    /// Format:
    /// projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of tables to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 50 tables will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListTables` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListTables` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The view for the returned tables.
    #[prost(enumeration = "TableView", tag = "4")]
    pub view: i32,
}
/// Response message for the ListTables method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTablesResponse {
    /// The tables from the specified database.
    #[prost(message, repeated, tag = "1")]
    pub tables: ::prost::alloc::vec::Vec<Table>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Options of a Hive database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HiveDatabaseOptions {
    /// Cloud Storage folder URI where the database data is stored, starting with
    /// "gs://".
    #[prost(string, tag = "1")]
    pub location_uri: ::prost::alloc::string::String,
    /// Stores user supplied Hive database parameters.
    #[prost(btree_map = "string, string", tag = "2")]
    pub parameters: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Options of a Hive table.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HiveTableOptions {
    /// Stores user supplied Hive table parameters.
    #[prost(btree_map = "string, string", tag = "1")]
    pub parameters: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Hive table type. For example, MANAGED_TABLE, EXTERNAL_TABLE.
    #[prost(string, tag = "2")]
    pub table_type: ::prost::alloc::string::String,
    /// Stores physical storage information of the data.
    #[prost(message, optional, tag = "3")]
    pub storage_descriptor: ::core::option::Option<
        hive_table_options::StorageDescriptor,
    >,
}
/// Nested message and enum types in `HiveTableOptions`.
pub mod hive_table_options {
    /// Serializer and deserializer information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SerDeInfo {
        /// The fully qualified Java class name of the serialization library.
        #[prost(string, tag = "1")]
        pub serialization_lib: ::prost::alloc::string::String,
    }
    /// Stores physical storage information of the data.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StorageDescriptor {
        /// Cloud Storage folder URI where the table data is stored, starting with
        /// "gs://".
        #[prost(string, tag = "1")]
        pub location_uri: ::prost::alloc::string::String,
        /// The fully qualified Java class name of the input format.
        #[prost(string, tag = "2")]
        pub input_format: ::prost::alloc::string::String,
        /// The fully qualified Java class name of the output format.
        #[prost(string, tag = "3")]
        pub output_format: ::prost::alloc::string::String,
        /// Serializer and deserializer information.
        #[prost(message, optional, tag = "4")]
        pub serde_info: ::core::option::Option<SerDeInfo>,
    }
}
/// View on Table. Represents which fields will be populated for calls that
/// return Table objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TableView {
    /// Default value. The API will default to the BASIC view.
    Unspecified = 0,
    /// Include only table names.
    /// This is the default value.
    Basic = 1,
    /// Include everything.
    Full = 2,
}
impl TableView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TableView::Unspecified => "TABLE_VIEW_UNSPECIFIED",
            TableView::Basic => "BASIC",
            TableView::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TABLE_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "BASIC" => Some(Self::Basic),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod metastore_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// BigLake Metastore is a serverless, highly available, multi-tenant runtime
    /// metastore for Google Cloud Data Analytics products.
    ///
    /// The BigLake Metastore API defines the following resource model:
    ///
    /// * A collection of Google Cloud projects: `/projects/*`
    /// * Each project has a collection of available locations: `/locations/*`
    /// * Each location has a collection of catalogs: `/catalogs/*`
    /// * Each catalog has a collection of databases: `/databases/*`
    /// * Each database has a collection of tables: `/tables/*`
    #[derive(Debug, Clone)]
    pub struct MetastoreServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MetastoreServiceClient<T>
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
        ) -> MetastoreServiceClient<InterceptedService<T, F>>
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
            MetastoreServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new catalog.
        pub async fn create_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCatalogRequest>,
        ) -> std::result::Result<tonic::Response<super::Catalog>, tonic::Status> {
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/CreateCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "CreateCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an existing catalog specified by the catalog ID.
        pub async fn delete_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCatalogRequest>,
        ) -> std::result::Result<tonic::Response<super::Catalog>, tonic::Status> {
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/DeleteCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "DeleteCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the catalog specified by the resource name.
        pub async fn get_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCatalogRequest>,
        ) -> std::result::Result<tonic::Response<super::Catalog>, tonic::Status> {
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/GetCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "GetCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List all catalogs in a specified project.
        pub async fn list_catalogs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCatalogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCatalogsResponse>,
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/ListCatalogs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "ListCatalogs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new database.
        pub async fn create_database(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatabaseRequest>,
        ) -> std::result::Result<tonic::Response<super::Database>, tonic::Status> {
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/CreateDatabase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "CreateDatabase",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an existing database specified by the database ID.
        pub async fn delete_database(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatabaseRequest>,
        ) -> std::result::Result<tonic::Response<super::Database>, tonic::Status> {
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/DeleteDatabase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "DeleteDatabase",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing database specified by the database ID.
        pub async fn update_database(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatabaseRequest>,
        ) -> std::result::Result<tonic::Response<super::Database>, tonic::Status> {
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/UpdateDatabase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "UpdateDatabase",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the database specified by the resource name.
        pub async fn get_database(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatabaseRequest>,
        ) -> std::result::Result<tonic::Response<super::Database>, tonic::Status> {
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/GetDatabase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "GetDatabase",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List all databases in a specified catalog.
        pub async fn list_databases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatabasesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDatabasesResponse>,
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/ListDatabases",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "ListDatabases",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new table.
        pub async fn create_table(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTableRequest>,
        ) -> std::result::Result<tonic::Response<super::Table>, tonic::Status> {
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/CreateTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "CreateTable",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an existing table specified by the table ID.
        pub async fn delete_table(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTableRequest>,
        ) -> std::result::Result<tonic::Response<super::Table>, tonic::Status> {
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/DeleteTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "DeleteTable",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing table specified by the table ID.
        pub async fn update_table(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTableRequest>,
        ) -> std::result::Result<tonic::Response<super::Table>, tonic::Status> {
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/UpdateTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "UpdateTable",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Renames an existing table specified by the table ID.
        pub async fn rename_table(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameTableRequest>,
        ) -> std::result::Result<tonic::Response<super::Table>, tonic::Status> {
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/RenameTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "RenameTable",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the table specified by the resource name.
        pub async fn get_table(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTableRequest>,
        ) -> std::result::Result<tonic::Response<super::Table>, tonic::Status> {
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/GetTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "GetTable",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List all tables in a specified database.
        pub async fn list_tables(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTablesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTablesResponse>,
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
                "/google.cloud.bigquery.biglake.v1.MetastoreService/ListTables",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.biglake.v1.MetastoreService",
                        "ListTables",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
