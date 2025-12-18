use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AuthorError {
    #[error("Invalid ORCID format")]
    InvalidOrcid,
    #[error("Missing author name")]
    MissingName,
    #[error("Affiliation parsing error")]
    AffiliationParsingError,
    #[error("Unexpected author error: {0}")]
    UnexpectedError(String),
}
