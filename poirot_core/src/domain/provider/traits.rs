use async_trait::async_trait;
use crate::domain::provider::{SearchQuery, SearchResult};

use crate::domain::authors::{Author,AuthorGraph};
use std::error::Error; 
use crate::domain::library_items::LibraryItem;
#[async_trait]
pub trait LiteratureProvider {
    type SearchQueryType: SearchQuery;
    type SearchResultType: SearchResult;
    async fn search(&self, query: Self::SearchQueryType) -> Result<Self::SearchResultType,Box<dyn Error + Send + Sync>>;

    async fn author_graph(&self, _author: &Author) -> Result<AuthorGraph, Box<dyn Error + Send + Sync>>{
        Ok(AuthorGraph::default())
    }
    async fn fetch_by_source_id(&self, source_id: &str) -> Result<Box<dyn LibraryItem>, Box<dyn Error + Send + Sync>>;
}