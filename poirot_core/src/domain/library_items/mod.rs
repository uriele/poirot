pub mod doi;
pub mod uri;
use crate::domain::authors::Author;
use crate::domain::builders::PoirotBuilder;
use crate::domain::errors::QueryError;
pub use doi::Doi;
use poirot_macro::{LibraryItem, PoirotBuilder};
use std::collections::HashSet;
pub use uri::Uri;

#[derive(Debug, Clone)]
pub enum LibraryItemType {
    Article(Article),
    Book(Book),
    Thesis(Thesis),
    Software(Software),
    Dataset(Dataset),
    LectureNote(LectureNote),
    Presentation(Presentation),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title(pub String);

impl ToString for Title {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Title {
    pub fn parse(title_str: &str) -> Result<Self, QueryError> {
        match title_str.is_empty() {
            true => {
                return Err(QueryError::WrongFormatting(
                    "Title cannot be empty".to_string(),
                ));
            }
            false => (),
        }

        let title_str = title_str
            .trim()
            .split_whitespace()
            .filter(|s| !s.is_empty()) // map does not affect empty string better to filter first
            .map(|s| s.to_lowercase())
            .collect::<Vec<String>>()
            .join(" ");

        Ok(Title(title_str.to_string()))
    }
}

pub trait LibraryItem {
    fn has_pages(&self) -> bool {
        false
    }
    fn has_doi(&self) -> bool {
        false
    }
    fn has_website(&self) -> bool {
        false
    }
}

#[derive(Debug, Default, Clone, Hash)]
pub struct Pages(pub Option<u32>, pub Option<u32>);

impl Pages {
    pub fn parse(start: Option<u32>, end: Option<u32>) -> Self {
        Pages(start, end)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none() && self.1.is_none()
    }
}

impl PartialEq for Pages {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Pages {}

#[derive(Debug, Clone, LibraryItem, PoirotBuilder)]
pub struct Article {
    #[builder_mandatory]
    pub title: Title,
    pub authors: HashSet<Author>,
    #[library_item(publication_year)]
    pub publication_year: Option<u32>,
    pub abstract_text: Option<String>,
    #[library_item(pages)]
    pub pages: Pages,
    #[library_item(doi)]
    pub doi: Option<Doi>,
    #[builder_mandatory]
    pub source_id: String,
}

#[derive(Debug, Clone, LibraryItem, PoirotBuilder)]
pub struct Book {
    #[builder_mandatory]
    pub title: Title,
    pub authors: HashSet<Author>,
    #[library_item(publication_year)]
    pub publication_year: Option<u32>,
    #[library_item(pages)]
    pub pages: Pages,
    #[library_item(doi)]
    pub doi: Option<Doi>,
    #[builder_mandatory]
    pub source_id: String,
}

#[derive(Debug, Clone, LibraryItem, PoirotBuilder)]
pub struct Thesis {
    #[builder_mandatory]
    pub title: Title,
    pub authors: HashSet<Author>,
    #[library_item(publication_year)]
    pub publication_year: Option<u32>,
    #[library_item(pages)]
    pub pages: Pages,
    #[library_item(doi)]
    pub doi: Option<Doi>,
    #[builder_mandatory]
    pub source_id: String,
}

#[derive(Debug, Clone, LibraryItem, PoirotBuilder)]
pub struct Software {
    #[builder_mandatory]
    pub title: Title,
    pub authors: HashSet<Author>,
    #[library_item(publication_year)]
    pub publication_year: Option<u32>,
    pub abstract_text: Option<String>,
    #[library_item(website)]
    pub website: Option<Uri>,
    #[builder_mandatory]
    pub source_id: String,
}

#[derive(Debug, Clone, LibraryItem, PoirotBuilder)]
pub struct Dataset {
    #[builder_mandatory]
    pub title: Title,
    pub authors: HashSet<Author>,
    #[library_item(publication_year)]
    pub publication_year: Option<u32>,
    pub abstract_text: Option<String>,
    #[library_item(doi)]
    pub doi: Option<Doi>,
    #[library_item(website)]
    pub website: Option<Uri>,
    #[builder_mandatory]
    pub source_id: String,
}

#[derive(Debug, Clone, LibraryItem, PoirotBuilder)]
pub struct LectureNote {
    #[builder_mandatory]
    pub title: Title,
    pub authors: HashSet<Author>,
    #[library_item(publication_year)]
    pub publication_year: Option<u32>,
    #[library_item(doi)]
    pub doi: Option<Doi>,
    #[builder_mandatory]
    pub source_id: String,
}

#[derive(Debug, Clone, LibraryItem, PoirotBuilder)]
pub struct Presentation {
    #[builder_mandatory]
    pub title: Title,
    pub authors: HashSet<Author>,
    #[library_item(publication_year)]
    pub publication_year: Option<u32>,
    #[library_item(doi)]
    pub doi: Option<Doi>,
    #[builder_mandatory]
    pub source_id: String,
}

#[derive(Debug, Clone, LibraryItem, PoirotBuilder)]
pub struct Patent {
    #[builder_mandatory]
    pub title: Title,
    pub authors: HashSet<Author>,
    #[library_item(publication_year)]
    pub publication_year: Option<u32>,
    pub abstract_text: Option<String>,
    #[library_item(doi)]
    pub doi: Option<Doi>,
    #[builder_mandatory]
    pub source_id: String,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_title_parser() {
        let title = "My   First bOOk";
        let correct_title = "my first book".to_string();
        let title_obj = Title::parse(title);
        assert!(title_obj.is_ok());
        let title_obj = title_obj.unwrap();
        assert_eq!(title_obj.to_string(), correct_title)
    }

    #[test]
    fn test_title_parser_empty() {
        let title = "";
        let title_obj = Title::parse(title);
        assert!(title_obj.is_err());
    }

    #[test]
    fn test_article_builder_success() {
        let title = Title::parse("An article").unwrap();
        let article = Article::builder()
            .with_title(title)
            .with_source_id("source_123".to_string())
            .build();
        assert!(article.is_ok());
        let article = article.unwrap();
        assert_eq!(article.source_id, "source_123".to_string());

        // test adding optional parameters

        let authors = vec![
            Author::builder()
                .name_from_str("Alice B. Example")
                .and_then(|q| q.build())
                .unwrap(),
        ]
        .into_iter()
        .collect();

        let doi = Doi::parse("10.1000/xyz123");

        let article = Article::builder()
            .with_title(Title::parse("Another article").unwrap())
            .with_source_id("source_789".to_string())
            .with_authors(authors)
            .with_publication_year(Some(2023))
            .with_pages(Pages::parse(Some(1), Some(10)))
            .with_abstract_text(Some("This is an abstract".to_string()))
            .with_doi(doi.ok())
            .build();

        assert!(article.is_ok());
        let article = article.unwrap();
        assert_eq!(article.source_id, "source_789".to_string());
        assert_eq!(article.publication_year, Some(2023));
        assert_eq!(article.pages, Pages::parse(Some(1), Some(10)));
        assert_eq!(
            article.doi.unwrap().to_string(),
            "10.1000/xyz123".to_string()
        );
        assert_eq!(article.authors.len(), 1);
        assert_eq!(
            article.authors.iter().next().unwrap().to_string(),
            "Alice B. Example".to_string()
        );
        assert_eq!(
            article.abstract_text.unwrap(),
            "This is an abstract".to_string()
        );
    }
    #[test]
    fn test_article_builder_missing_mandatory() {
        let title = Title::parse("An article").unwrap();
        let article = Article::builder().with_title(title).build();
        assert!(article.is_err());
    }
    #[test]
    fn test_book_builder_success() {
        let title = Title::parse("A book").unwrap();
        let book = Book::builder()
            .with_title(title)
            .with_source_id("source_456".to_string())
            .build();
        assert!(book.is_ok());
        let book = book.unwrap();
        assert_eq!(book.source_id, "source_456".to_string());
    }
}
