use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AuthorError {
    #[error("Invalid ORCID format")]
    InvalidOrcid,
    #[error("Missing author name")]
    MissingName,
    #[error("Affiliation parsing error")]
    AffiliationParsingError,
    #[error("General author error: {0}")]
    General(String),
}


