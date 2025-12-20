pub mod authors;
pub mod builders;
pub mod constants;
pub mod errors;
pub mod library_items;
pub mod provider;
pub use crate::domain::library_items::{Doi, LibraryItemType, Pages, Title, Uri};
pub use authors::{Affiliation, Author, AuthorError, Name, Orcid};
pub use errors::QueryError;
