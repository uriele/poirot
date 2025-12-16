use fancy_regex::Regex;
use core::error::Error;
use crate::domain::errors::QueryError;
const DOI_REGEX: &str = r"^10.(\d{4,9})/([-._;()/:A-Za-z0-9]+)$";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Doi{
    pub prefix: String,
    pub suffix: String,
}

impl Doi {
    pub fn parse(doi_str: &str) -> Result<Self, Box<dyn Error>> {
        let captured = Regex::new(DOI_REGEX)
            .map_err(|_| Box::new(QueryError::UnexpectedError("Failed to compile DOI regex".to_string())))?
            .captures(doi_str)?;

        if let Some(captures) = captured {
            Ok(Doi {
                prefix: captures.get(1).unwrap().as_str().to_string(),
                suffix: captures.get(2).unwrap().as_str().to_string(),
            })
        } else {
            Err(Box::new(QueryError::UnexpectedError(format!("Invalid DOI format: {}", doi_str))))
        }
    }

}


impl ToString for Doi {
    fn to_string(&self) -> String {
        format!("10.{}/{}", self.prefix, self.suffix)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn generate_valid_prefix() -> String {
        use rand::Rng;
        let mut rng = rand::rng();
        let len: usize = rng.random_range(4..=9);
        (0..len).map(|_| rng.random_range(0..10).to_string()).collect()
    }

    fn generate_valid_suffix() -> String {
        use rand::Rng;
        let mut rng = rand::rng();
        let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._;()/:"
            .chars()
            .collect();
        let len: usize = rng.random_range(1..=20);
        (0..len)
            .map(|_| chars[rng.random_range(0..chars.len())])
            .collect()
    }

    #[test]
    fn test_doi_to_string() {
        let doi = Doi {
            prefix: "1000".to_string(),
            suffix: "xyz123".to_string(),
        };
        assert_eq!(doi.to_string(), "10.1000/xyz123");
    }

    #[test]
    fn test_doi_parse_valid(){
        let prefixs = Vec::from_iter((0..10).map(|_| generate_valid_prefix()));
        let suffixs = Vec::from_iter((0..10).map(|_| generate_valid_suffix()));

        let doi_str = prefixs.iter().zip(suffixs.iter()).map(|(p,s)| format!("10.{}/{}",p,s)).collect::<Vec<String>>();

        for doi_s in doi_str{
            let doi = Doi::parse(&doi_s);
            assert!(doi.is_ok());
            assert_eq!(doi.unwrap().to_string(),doi_s);
        }

    }

    #[test]
    fn test_doi_parse_invalid(){
        let invalid_dois = vec![
            "10./suffix",
            "10.prefix/",
            "prefix/suffix",
            "10.1234/invalid char!",
            "10.1234",
            "10.1234/",
            "/suffix",
            "10.1234/suffix with spaces",
        ];

        for doi_s in invalid_dois{
            let doi = Doi::parse(doi_s);
            assert!(doi.is_err());
        }
    }
}

