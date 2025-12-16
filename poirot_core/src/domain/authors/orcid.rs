use crate::domain::authors::author_error::AuthorError;

#[derive(Debug, Clone, PartialEq, Eq,Hash)]
pub struct Orcid(pub String);

impl Orcid{
    pub fn parse(orcid_str: &str) -> Result<Self, AuthorError> {
        // Simple validation logic for ORCID
        let parts: Vec<&str> = orcid_str.split('-').collect();
        if parts.len() != 4 || !parts.iter().all(|part| part.len() == 4 && part.chars().all(|c| c.is_digit(10))) {
            return Err(AuthorError::InvalidOrcid);
        }
        Ok(Orcid(orcid_str.to_string()))
    }
}

