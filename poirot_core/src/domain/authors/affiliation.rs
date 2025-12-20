use crate::utils::normalize_and_filter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Affiliation {
    pub institution: Option<String>,
    pub department: Option<String>,
    pub address: Option<String>,
    pub country: Option<String>,
}

mod indexing {
    pub const INSTITUTION: usize = 0;
    pub const DEPARTMENT: usize = 1;
    pub const ADDRESS: usize = 2;
    pub const COUNTRY: usize = 3;
}


impl Affiliation {
    pub fn parse(affil_str: &str) -> Self {
        // Simple parsing logic, can be improved with more sophisticated parsing
        let parts: Vec<&str> = affil_str.split(';').map(|s| s.trim()).collect();
        let institution = parts.get(indexing::INSTITUTION).and_then(|s| normalize_and_filter(s));
        let department = parts.get(indexing::DEPARTMENT).and_then(|s| normalize_and_filter(s));
        let address = parts.get(indexing::ADDRESS).and_then(|s| normalize_and_filter(s));
        let country = parts.get(indexing::COUNTRY).and_then(|s| normalize_and_filter(s));
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
        assert_eq!(
            affil.institution.unwrap(),
            "University of Example".to_lowercase()
        );
        assert_eq!(
            affil.department.unwrap(),
            "Department of Testing".to_lowercase()
        );
        assert_eq!(affil.address.unwrap(), "123 Test St".to_lowercase());
        assert_eq!(affil.country.unwrap(), "Testland".to_lowercase());

        let affil_str_partial = "Institute of Samples; ;456 Sample Rd";
        let affil_partial = Affiliation::parse(affil_str_partial);

        println!("{:?}", affil_partial);
        assert_eq!(
            affil_partial.institution.unwrap(),
            "Institute of Samples".to_lowercase()
        );
        assert!(affil_partial.department.is_none());
        assert_eq!(
            affil_partial.address.unwrap(),
            "456 Sample Rd".to_lowercase()
        );
        assert!(affil_partial.country.is_none());
    }
}
