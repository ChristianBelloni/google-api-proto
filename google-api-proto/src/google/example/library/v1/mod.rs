/// A single book in the library.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Book {
    /// The resource name of the book.
    /// Book names have the form `shelves/{shelf_id}/books/{book_id}`.
    /// The name is ignored when creating a book.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the book author.
    #[prost(string, tag = "2")]
    pub author: ::prost::alloc::string::String,
    /// The title of the book.
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// Value indicating whether the book has been read.
    #[prost(bool, tag = "4")]
    pub read: bool,
}
/// A Shelf contains a collection of books with a theme.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shelf {
    /// The resource name of the shelf.
    /// Shelf names have the form `shelves/{shelf_id}`.
    /// The name is ignored when creating a shelf.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The theme of the shelf
    #[prost(string, tag = "2")]
    pub theme: ::prost::alloc::string::String,
}
/// Request message for LibraryService.CreateShelf.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShelfRequest {
    /// The shelf to create.
    #[prost(message, optional, tag = "1")]
    pub shelf: ::core::option::Option<Shelf>,
}
/// Request message for LibraryService.GetShelf.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShelfRequest {
    /// The name of the shelf to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for LibraryService.ListShelves.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShelvesRequest {
    /// Requested page size. Server may return fewer shelves than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    /// Typically, this is the value of
    /// \[ListShelvesResponse.next_page_token][google.example.library.v1.ListShelvesResponse.next_page_token\]
    /// returned from the previous call to `ListShelves` method.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for LibraryService.ListShelves.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShelvesResponse {
    /// The list of shelves.
    #[prost(message, repeated, tag = "1")]
    pub shelves: ::prost::alloc::vec::Vec<Shelf>,
    /// A token to retrieve next page of results.
    /// Pass this value in the
    /// \[ListShelvesRequest.page_token][google.example.library.v1.ListShelvesRequest.page_token\]
    /// field in the subsequent call to `ListShelves` method to retrieve the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for LibraryService.DeleteShelf.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShelfRequest {
    /// The name of the shelf to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Describes the shelf being removed (other_shelf_name) and updated
/// (name) in this merge.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeShelvesRequest {
    /// The name of the shelf we're adding books to.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the shelf we're removing books from and deleting.
    #[prost(string, tag = "2")]
    pub other_shelf: ::prost::alloc::string::String,
}
/// Request message for LibraryService.CreateBook.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBookRequest {
    /// The name of the shelf in which the book is created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The book to create.
    #[prost(message, optional, tag = "2")]
    pub book: ::core::option::Option<Book>,
}
/// Request message for LibraryService.GetBook.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBookRequest {
    /// The name of the book to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for LibraryService.ListBooks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBooksRequest {
    /// The name of the shelf whose books we'd like to list.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer books than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    /// Typically, this is the value of
    /// \[ListBooksResponse.next_page_token][google.example.library.v1.ListBooksResponse.next_page_token\].
    /// returned from the previous call to `ListBooks` method.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for LibraryService.ListBooks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBooksResponse {
    /// The list of books.
    #[prost(message, repeated, tag = "1")]
    pub books: ::prost::alloc::vec::Vec<Book>,
    /// A token to retrieve next page of results.
    /// Pass this value in the
    /// \[ListBooksRequest.page_token][google.example.library.v1.ListBooksRequest.page_token\]
    /// field in the subsequent call to `ListBooks` method to retrieve the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for LibraryService.UpdateBook.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBookRequest {
    /// The name of the book to update.
    #[prost(message, optional, tag = "1")]
    pub book: ::core::option::Option<Book>,
    /// Required. Mask of fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for LibraryService.DeleteBook.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBookRequest {
    /// The name of the book to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Describes what book to move (name) and what shelf we're moving it
/// to (other_shelf_name).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveBookRequest {
    /// The name of the book to move.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the destination shelf.
    #[prost(string, tag = "2")]
    pub other_shelf_name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod library_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// This API represents a simple digital library. It lets you manage Shelf
    /// resources and Book resources in the library. It defines the following
    /// resource model:
    ///
    /// - The API has a collection of [Shelf][google.example.library.v1.Shelf]
    ///   resources, named `shelves/*`
    ///
    /// - Each Shelf has a collection of [Book][google.example.library.v1.Book]
    ///   resources, named `shelves/*/books/*`
    #[derive(Debug, Clone)]
    pub struct LibraryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LibraryServiceClient<T>
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
        ) -> LibraryServiceClient<InterceptedService<T, F>>
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
            LibraryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a shelf, and returns the new Shelf.
        pub async fn create_shelf(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateShelfRequest>,
        ) -> std::result::Result<tonic::Response<super::Shelf>, tonic::Status> {
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
                "/google.example.library.v1.LibraryService/CreateShelf",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.example.library.v1.LibraryService",
                        "CreateShelf",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a shelf. Returns NOT_FOUND if the shelf does not exist.
        pub async fn get_shelf(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShelfRequest>,
        ) -> std::result::Result<tonic::Response<super::Shelf>, tonic::Status> {
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
                "/google.example.library.v1.LibraryService/GetShelf",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.example.library.v1.LibraryService",
                        "GetShelf",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists shelves. The order is unspecified but deterministic. Newly created
        /// shelves will not necessarily be added to the end of this list.
        pub async fn list_shelves(
            &mut self,
            request: impl tonic::IntoRequest<super::ListShelvesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListShelvesResponse>,
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
                "/google.example.library.v1.LibraryService/ListShelves",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.example.library.v1.LibraryService",
                        "ListShelves",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a shelf. Returns NOT_FOUND if the shelf does not exist.
        pub async fn delete_shelf(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteShelfRequest>,
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
                "/google.example.library.v1.LibraryService/DeleteShelf",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.example.library.v1.LibraryService",
                        "DeleteShelf",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Merges two shelves by adding all books from the shelf named
        /// `other_shelf_name` to shelf `name`, and deletes
        /// `other_shelf_name`. Returns the updated shelf.
        /// The book ids of the moved books may not be the same as the original books.
        ///
        /// Returns NOT_FOUND if either shelf does not exist.
        /// This call is a no-op if the specified shelves are the same.
        pub async fn merge_shelves(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeShelvesRequest>,
        ) -> std::result::Result<tonic::Response<super::Shelf>, tonic::Status> {
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
                "/google.example.library.v1.LibraryService/MergeShelves",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.example.library.v1.LibraryService",
                        "MergeShelves",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a book, and returns the new Book.
        pub async fn create_book(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBookRequest>,
        ) -> std::result::Result<tonic::Response<super::Book>, tonic::Status> {
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
                "/google.example.library.v1.LibraryService/CreateBook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.example.library.v1.LibraryService",
                        "CreateBook",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a book. Returns NOT_FOUND if the book does not exist.
        pub async fn get_book(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBookRequest>,
        ) -> std::result::Result<tonic::Response<super::Book>, tonic::Status> {
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
                "/google.example.library.v1.LibraryService/GetBook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.example.library.v1.LibraryService",
                        "GetBook",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists books in a shelf. The order is unspecified but deterministic. Newly
        /// created books will not necessarily be added to the end of this list.
        /// Returns NOT_FOUND if the shelf does not exist.
        pub async fn list_books(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBooksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBooksResponse>,
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
                "/google.example.library.v1.LibraryService/ListBooks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.example.library.v1.LibraryService",
                        "ListBooks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a book. Returns NOT_FOUND if the book does not exist.
        pub async fn delete_book(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBookRequest>,
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
                "/google.example.library.v1.LibraryService/DeleteBook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.example.library.v1.LibraryService",
                        "DeleteBook",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a book. Returns INVALID_ARGUMENT if the name of the book
        /// is non-empty and does not equal the existing name.
        pub async fn update_book(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBookRequest>,
        ) -> std::result::Result<tonic::Response<super::Book>, tonic::Status> {
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
                "/google.example.library.v1.LibraryService/UpdateBook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.example.library.v1.LibraryService",
                        "UpdateBook",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Moves a book to another shelf, and returns the new book. The book
        /// id of the new book may not be the same as the original book.
        pub async fn move_book(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveBookRequest>,
        ) -> std::result::Result<tonic::Response<super::Book>, tonic::Status> {
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
                "/google.example.library.v1.LibraryService/MoveBook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.example.library.v1.LibraryService",
                        "MoveBook",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
