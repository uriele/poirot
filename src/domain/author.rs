use thiserror::Error;
use crate::domain::affiliation::Affiliation;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub first: String,
    pub middle: Option<String>,
    pub last: String,
}

impl Name {
    pub fn builder() -> NameBuilder {
        NameBuilder::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Orcid(String);

impl Orcid{
    fn parse(orcid_str: &str) -> Result<Self, AuthorError> {
        // Simple validation logic for ORCID
        let parts: Vec<&str> = orcid_str.split('-').collect();
        if parts.len() != 4 || !parts.iter().all(|part| part.len() == 4 && part.chars().all(|c| c.is_digit(10))) {
            return Err(AuthorError::InvalidOrcid);
        }
        Ok(Orcid(orcid_str.to_string()))
    }
}


#[derive(Debug, Default)]
pub struct NameBuilder {
    first: Option<String>,
    middle: Option<String>,
    last: Option<String>,
}

impl NameBuilder {
    pub fn first(mut self, first: impl Into<String>) -> Self {
        self.first = match first.into().as_str() {
            "" => None,
            s => Some(s.to_string()),
        };
        self
    }

    pub fn middle(mut self, middle: impl Into<String>) -> Self {
        self.middle = match middle.into().as_str() {
            "" => None,
            s => Some(s.to_string()),
        };
        self
    }

    pub fn last(mut self, last: impl Into<String>) -> Self {
        self.last = match last.into().as_str() {
            "" => None,
            s => Some(s.to_string()),
        };
        self
    }

    pub fn build(self) -> Result<Name, AuthorError> {
        let first = self.first.ok_or(AuthorError::MissingName)?;
        let last = self.last.ok_or(AuthorError::MissingName)?;
        let middle = self.middle;
        Ok(Name {
            first,
            middle,
            last,
        })
    }
}


#[derive(Debug, Clone)]
pub struct Author {
    pub name: Name,
    pub orcid: Option<Orcid>,
    pub affiliation: Option<Affiliation>,
    pub tags: Vec<String>,
}

impl Author {
    pub fn builder() -> AuthorBuilder {
        AuthorBuilder::default()
    }
}


#[derive(Debug, Default)]
pub struct AuthorBuilder {
    name: Option<Name>,
    orcid: Option<Orcid>,
    affiliation: Option<Affiliation>,
    tags: Vec<String>,
}


impl AuthorBuilder {
    pub fn name(mut self, name: Name) -> Result<Self, AuthorError> {
        self.name = Some(name);
        Ok(self)
    }


    pub fn name_from_str(mut self, name_str: &str) -> Result<Self, AuthorError> {
        let name_parts: Vec<&str> = name_str.split_whitespace().collect();
        if name_parts.is_empty() {
            return Err(AuthorError::MissingName);
        }
        let name: Name = match name_parts.len() {
            2 => Name::builder().first(name_parts[0]).last(name_parts[1]).build()?,
            3 => Name::builder().first(name_parts[0]).middle(name_parts[1]).last(name_parts[2]).build()?,
            _ => return Err(AuthorError::MissingName),
        };

        self = self.name(name)?;
        Ok(self)
    }


    pub fn orcid(mut self, orcid: Orcid) -> Result<Self, AuthorError> {
        self.orcid = Some(orcid);
        Ok(self)
    }

    pub fn orcid_from_str(mut self, orcid_str: &str) -> Result<Self, AuthorError> {
        let orcid = Orcid::parse(orcid_str)?;
        self = self.orcid(orcid)?;
        Ok(self)
    }



    pub fn affiliation(mut self, affiliation: Affiliation) -> Result<Self, AuthorError> {
        self.affiliation = Some(affiliation);
        Ok(self)
    }

    pub fn affiliation_from_str(mut self, affil_str: &str) -> Result<Self, AuthorError> {
        let affiliation = Affiliation::parse(affil_str);
        self = self.affiliation(affiliation)?;
        Ok(self)
    }   

    pub fn tags(mut self, tags: Vec<String>) -> Result<Self, AuthorError> {
        self.tags = tags;
        Ok(self)
    }

    pub fn build(self) -> Result<Author, AuthorError> {
        let name = self.name.ok_or(AuthorError::MissingName)?;
        let orcid =self.orcid;
        let affiliation = self.affiliation;
        let tags = self.tags;
        
        Ok(
            Author {
            name,
            orcid,
            affiliation,
            tags,
        })
    }
}





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







#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_author_builder() {
        let name = Name::builder()
            .first("Jane")
            .last("Smith")
            .build()
            .unwrap();
        let orcid = Orcid::parse("0000-0001-2345-6789").unwrap();
        let affiliation = Affiliation::parse("University X; Department Y; 123 Street; Country Z");
        let author = Author::builder()
            .name(name).expect("Name is required")
            .orcid(orcid).expect("ORCID parsing failed")
            .affiliation(affiliation).expect("Affiliation parsing failed")
            .tags(vec!["Physics".to_string(), "Astronomy".to_string()]).expect("Tags parsing failed")       
            .build()
            .unwrap();

        assert_eq!(author.name.first, "Jane");
        assert_eq!(author.name.last, "Smith");
        assert_eq!(author.orcid.unwrap().0, "0000-0001-2345-6789");
        assert_eq!(author.affiliation.unwrap().institution.unwrap(), "University X");
        assert_eq!(author.tags, vec!["Physics", "Astronomy"]);
    }


    #[test]
    fn test_orcid_parse_valid() {
        let orcid_str = "0000-0002-1825-0097";
        let orcid = Orcid::parse(orcid_str).unwrap();
        assert_eq!(orcid.0, orcid_str);
    }

    #[test]
    fn test_orcid_parse_invalid() {
        let orcid_str = "0000-0002-1825-009X";
        let result = Orcid::parse(orcid_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_name_builder() {
        let name = Name::builder()
            .first("John")
            .middle("H.")
            .last("Doe")
            .build()
            .unwrap();
        assert_eq!(
            name,
            Name {
                first: "John".to_string(),
                middle: Some("H.".to_string()),
                last: "Doe".to_string(),
            }
        );
    }

    #[test]
    fn test_name_builder_missing_first() {
        let result = Name::builder().last("Doe").build();
        assert_eq!(result, Err(AuthorError::MissingName));
    }

    #[test]
    fn test_name_builder_missing_last() {
        let result = Name::builder().first("John").build();
        assert_eq!(result, Err(AuthorError::MissingName));
    }
}

