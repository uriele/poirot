// ...existing code...
use serde::Deserialize;
use crate::domain::{errors::QueryError,provider::SearchResult};
use crate::services::provider::constants::arxiv_text;
use std::ops::{Deref,DerefMut};

// Use to extract Entries from Feed
#[derive(Debug,Default,Clone, Deserialize)]
pub struct ArxivResult {
    // We only care about <entry>...</entry>
    #[serde(rename = "entry", default)]
    pub entries: Vec<AtomEntry>,
}

impl SearchResult for ArxivResult{}

pub fn parse_atom_entries(xml: &str) -> Result<ArxivResult, QueryError> {
        quick_xml::de::from_str(xml).map_err(|e| QueryError::UnexpectedError(e.to_string()))
}



impl Deref for ArxivResult{
    type Target= Vec<AtomEntry>;
    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl DerefMut for ArxivResult{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}




fn normalize_abs_id(id_url: &str) -> Result<(String,Option<String>), QueryError> {
    let tail = id_url
        .trim()
        .strip_prefix(arxiv_text::ARXIV_ABS_HTTP)
        .or_else(|| id_url.trim().strip_prefix(arxiv_text::ARXIV_ABS_HTTPS))
        .ok_or(QueryError::UnexpectedError("Invalid arXiv ID URL".to_string()))?
        .trim();


    let regex_remove_version = fancy_regex::Regex::new(r"^(?P<id>.+?)(?P<version>v\d+)?$")
        .map_err(|e| QueryError::UnexpectedError(e.to_string()))?;
    let caps = regex_remove_version.captures(tail)
        .map_err(|e| QueryError::UnexpectedError(e.to_string()))?
        .ok_or_else(|| QueryError::UnexpectedError("Failed to capture arXiv ID".to_string()))?;
    let id = caps.name("id")
        .ok_or_else(|| QueryError::UnexpectedError("Missing id group".to_string()))?.as_str().to_string(); 
    let version = caps.name("version")
        .map(|m| m.as_str().to_string());
    

    Ok((id,version))
}
fn deserialize_abs_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;

    let (id,_) = normalize_abs_id(&raw)
        .map_err(serde::de::Error::custom)?;
    Ok(id)
}

// This is the part I am interested in
#[derive(Debug, Deserialize,Clone)]
pub struct AtomEntry {
    #[serde(rename = "id", deserialize_with = "deserialize_abs_id")]
    id: String,

    #[serde(rename = "title")]
    title: Option<String>,

    #[serde(rename = "summary")]
    summary: Option<String>,

    #[serde(rename = "published")]
    published: Option<String>,

    #[serde(rename = "updated")]
    updated: Option<String>,

    #[serde(rename = "author", default)]
    authors: Vec<AtomAuthor>,

    // <category term="cs.ET" .../>
    #[serde(rename = "category", default)]
    categories: Vec<AtomCategory>,
    #[serde(rename = "comment", alias = "arxiv:comment")]
    comment: Option<String>,

    #[serde(rename = "journal_ref", alias = "arxiv:journal_ref")]
    journal_ref: Option<String>,

    #[serde(rename = "doi", alias = "arxiv:doi")]
    doi: Option<String>,

    #[serde(rename = "primary_category", alias = "arxiv:primary_category")]
    primary_category: Option<AtomCategory>,
}

#[derive(Debug, Deserialize,Clone)]
struct AtomAuthor {
    #[serde(rename = "name")]
    name: Option<String>,
}

#[derive(Debug, Deserialize,Clone)]
struct AtomCategory {
    // XML attributes are addressed with '@'
    #[serde(rename = "@term")]
    term: String,
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_normalize_abs_id(){
        let id_url="http://arxiv.org/abs/1234.5678v2";
        let (id,version)=normalize_abs_id(id_url).unwrap();
        assert_eq!(id,"1234.5678".to_string());
        assert_eq!(version,Some("v2".to_string()));

        let id_url2="https://arxiv.org/abs/9876.5432";
        let (id2,version2)=normalize_abs_id(id_url2).unwrap();
        assert_eq!(id2,"9876.5432".to_string());
        assert_eq!(version2,None);
    }

    #[test]
    fn test_parse_atom_entries(){
        let sample_xml=r#"
        <feed xmlns="http://www.w3.org/2005/Atom" xmlns:arxiv="http://arxiv.org/schemas/atom">
            <entry>
                <id>http://arxiv.org/abs/1234.5678v1</id>
                <updated>2023-10-01T00:00:00Z</updated>
                <published>2023-09-30T00:00:00Z</published>
                <title>Sample Title</title>
                <summary>Sample abstract text.</summary>
                <author>
                    <name>First Author</name>
                </author>
                <author>
                    <name>Second Author</name>
                </author>
                <category term="cs.AI"/>
                <category term="stat.ML"/>
                <arxiv:comment>10 pages, 5 figures</arxiv:comment>
                <arxiv:journal_ref>Journal of Testing, 2023</arxiv:journal_ref>
                <arxiv:doi>10.1000/testdoi</arxiv:doi>
                <arxiv:primary_category term="cs.AI"/>
            </entry>
        </feed>
        "#;

        let entries=parse_atom_entries(sample_xml).unwrap();

        println!("{:#?}",entries);
        assert_eq!(entries.len(),1);
        let entry=&entries[0];
        assert_eq!(entry.id,"1234.5678".to_string());
        assert_eq!(entry.title.as_ref().unwrap().to_string(),"Sample Title".to_string());
        assert_eq!(entry.summary.as_ref().unwrap().to_string(),"Sample abstract text.".to_string());
        assert_eq!(entry.authors.len(),2);
        assert_eq!(entry.authors[0].name.as_ref().unwrap().to_string(),"First Author".to_string());
        assert_eq!(entry.categories.len(),2);
        assert_eq!(entry.categories[0].term,"cs.AI".to_string());
        assert_eq!(entry.comment.as_ref().unwrap().to_string(),"10 pages, 5 figures".to_string());
    }
}