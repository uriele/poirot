#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uri(pub String);

const VALID_PREFIXES: [&str; 4] = ["http://", "https://","ftp://","ftps://"];


impl Uri {
    pub fn parse(s: &str) -> Result<Self, String> {
        // Simple validation for example purposes
        if VALID_PREFIXES.iter().any(|&prefix| s.starts_with(prefix)) {
            Ok(Uri(s.to_string()))
        } else {
            Err("Invalid URI format".to_string())
        }
    }
}

impl ToString for Uri {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::{Client};

    #[test]
    fn test_uri_parse_valid() {
        let valid_uris = vec![
            "http://example.com",
            "https://example.com/resource",
        ];

        for uri_str in valid_uris {
            let uri = Uri::parse(uri_str);
            assert!(uri.is_ok());
            assert_eq!(uri.unwrap().to_string(), uri_str.to_string());
        }
    }

    #[test]
    fn test_uri_parse_invalid() {
        let invalid_uris = vec![
            "example.com/resource",
            "://invalid.uri",
        ];

        for uri_str in invalid_uris {
            let uri = Uri::parse(uri_str);
            assert!(uri.is_err());
        }
   }
   #[test]
   fn test_reqwest_integration() {
       let uri_str = "https://www.rust-lang.org/";
       let uri = Uri::parse(uri_str).unwrap();
       let client= Client::builder().build().unwrap();
       let reqwest_url = client.get(uri.to_string()).build();
       assert!(reqwest_url.is_ok());
       assert_eq!(reqwest_url.unwrap().url().as_str(), uri_str);
   }
}
