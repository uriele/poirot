pub mod database;
pub use database::schema::{SCHEMA, HNSW_INDEX};
pub mod domain;
pub mod services;
pub mod utils;
pub use domain::{Name,Orcid,Author, AuthorError, Affiliation};
