#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Affiliation {
    pub institution: Option<String>,
    pub department: Option<String>,
    pub address: Option<String>,
    pub country: Option<String>,
}

impl Affiliation {
    pub fn parse(affil_str: &str) -> Self {
        // Simple parsing logic, can be improved with more sophisticated parsing
        let parts: Vec<&str> = affil_str.split(';').map(|s| s.trim()).collect();
        let institution = parts.get(0)
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty());
        let department = parts.get(1)
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty());
        let address = parts.get(2)
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty());
        let country = parts.get(3)
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty());
        Affiliation {
            institution,
            department,
            address,
            country,
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affiliation_parse() {
        let affil_str = "University of Example; Department of Testing; 123 Test St; Testland";
        let affil = Affiliation::parse(affil_str);
        assert_eq!(affil.institution.unwrap(), "University of Example");
        assert_eq!(affil.department.unwrap(), "Department of Testing");
        assert_eq!(affil.address.unwrap(), "123 Test St");
        assert_eq!(affil.country.unwrap(), "Testland");

        let affil_str_partial = "Institute of Samples; ;456 Sample Rd";
        let affil_partial = Affiliation::parse(affil_str_partial);

        println!("{:?}", affil_partial);
        assert_eq!(affil_partial.institution.unwrap(), "Institute of Samples");
        assert!(affil_partial.department.is_none());
        assert_eq!(affil_partial.address.unwrap(), "456 Sample Rd");
        assert!(affil_partial.country.is_none());
    }
}