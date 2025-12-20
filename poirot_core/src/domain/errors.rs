use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum QueryError {
    #[error("Invalid Query formatting: {0}")]
    WrongFormatting(String),
    #[error("Unexpected Query error: {0}")]
    UnexpectedError(String),
}
