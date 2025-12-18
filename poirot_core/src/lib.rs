pub mod database;
pub use database::schema::{HNSW_INDEX, SCHEMA};
pub mod domain;
pub mod services;
pub mod utils;
pub use domain::{Affiliation, Author, AuthorError, Name, Orcid};
