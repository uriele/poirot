use crate::domain::authors::author_error::AuthorError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Orcid(pub String);

impl Orcid {
    pub fn parse(orcid_str: &str) -> Result<Self, AuthorError> {
        // Simple validation logic for ORCID

        let regex_orcid = fancy_regex::Regex::new(r#"^(\d{4})(-?)\d{4}\2\d{4}\2\d{3}[\dX]$"#)
            .map_err(|e| AuthorError::UnexpectedError(e.to_string()))?;
        let _ = regex_orcid
            .captures(orcid_str)
            .map_err(|e| AuthorError::UnexpectedError(e.to_string()))?
            .ok_or_else(|| AuthorError::InvalidOrcid)?;

        Ok(Orcid(orcid_str.into()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_valid_orcid() {
        let vals = [
            "0000000000000000",
            "000000000000000X",
            "0000-0000-0000-0000",
            "0000-0000-0000-000X",
            "1234567898765432",
            "123456789876543X",
            "1234-5678-9876-5432",
            "1234-5678-9876-543X",
        ];

        for val in vals {
            println!("{} {:?}", val, Orcid::parse(val));
            assert!(Orcid::parse(val).is_ok());
        }
    }
    #[test]
    fn test_invalid_orcid() {
        let vals = [
            "00000000000000X",    // too short
            "000000000000000Y",   // wrong checksum letter
            "0000-00000000-0000", // wrong formatting
            "00000000-0000-000X", // wrong formatting
            "12345637898765432",  // too long
            "1234567898765X43",   // wrong checksup letter position
            "1234-5678-9876",     // too short
            "1234-5678-9876-54X", // too short
        ];

        for val in vals {
            println!("{} {:?}", val, Orcid::parse(val));
            assert!(Orcid::parse(val).is_err());
        }
    }
}
