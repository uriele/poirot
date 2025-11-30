pub mod author;
pub mod affiliation;
pub mod types;
pub mod orcid;
pub use author::{Author, AuthorError,Name,Orcid};
pub use affiliation::Affiliation;