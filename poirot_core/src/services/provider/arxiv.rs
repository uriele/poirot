use tokio::task::JoinSet; // for concurrent page fetching
use async_trait::async_trait;
use crate::services::provider::arxiv_text;
use crate::domain::{provider::{
    SearchQuery, 
    LiteratureProvider,
    },
};

use std::ops::{Deref,DerefMut};
use super::arxivquery_ast::{parse,ast,interpret};
use super::atomfeed::ArxivResult;
use crate::domain::errors::QueryError;
use crate::domain::library_items::LibraryItem;
use std::{hash::Hash, sync::{Arc,RwLock}};
use governor::DefaultDirectRateLimiter;
use governor::state::NotKeyed;
use governor::{Quota, RateLimiter,
    state::InMemoryState,
    clock::DefaultClock,
};
use crate::domain::builders::PoirotBuilder;
use poirot_macro::PoirotBuilder;
use nonzero_ext::{ nonzero};
use std::error::Error; 

/// ArxivProvider struct implementing LiteratureProvider trait
/// This struct provides methods to interact with the arXiv API
/// for searching and fetching literature items. Unlike RESTful APIs,
/// arXiv uses a query-based approach for accessing its resources.
/// The implementation details would include constructing appropriate
/// query URLs, handling responses, and parsing the data into the
/// defined domain models. 
/// 
/// Another option would be to implement an OAI-PMH client for arXiv, 
/// to harvest metadata records in a standardized way and save them locally.
/// If no abstract is saved in the local database, but an embedding is stores in the local 
/// vector database, we could just fetch the abstract from arXiv on demand.
/// 
#[derive(Debug,Clone)]
pub struct ArxivProvider {
    // Configuration fields if needed
    pub rate_limiter: Arc<DefaultDirectRateLimiter>,
    pub client: reqwest::Client,
}


fn create_rate_limiter() -> Result<RateLimiter<NotKeyed,InMemoryState,DefaultClock>, QueryError> {
    let quota = Quota::per_second(nonzero!(2u32));
    

    Ok(RateLimiter::direct_with_clock(quota, DefaultClock::default()))
}

impl ArxivProvider {
    pub fn new() -> Result<Self, QueryError> {
        let rate_limiter = Arc::new(create_rate_limiter()?);
        let client = reqwest::Client::builder()
            .build()
            .map_err(|e| QueryError::UnexpectedError(e.to_string()))?;
        Ok(ArxivProvider { rate_limiter,client })
    }
}   

// Necessary due to the weird format of Arxiv responses
const _MIN_PER_PAGE: u32 = 10;
const _MAX_PER_PAGE: u32 = 300;



#[derive(Default,Debug, Clone)]
pub enum SortOrder {
    #[default]
    Ascending,
    Descending
}


#[derive(Default,Debug,Clone)]
pub enum SortBy{
    #[default]
    Relevance,
    LastUpdatedDate,
    SubmittedDate,
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct QueryBody(ast::Expr);

impl Default for QueryBody{
    fn default() -> Self {
        QueryBody(ast::Expr::default())
    }
}


impl QueryBody{
    pub fn parse(s: String) -> Result<Self,QueryError>{
        let expr=parse::parse_expr(&s).map_err(
            |e| QueryError::WrongFormatting(format!("Failed to parse arXiv query body: {}",e.to_string()))
        )?;
        Ok(QueryBody(expr))
    }
}

impl ToString for QueryBody{
    fn to_string(&self) -> String {
        interpret::to_arxiv_string(&self.0)
    }
}


#[derive(Debug,Clone,Hash,PartialEq,Eq)]
pub struct PerPage(pub u32);

impl Default for PerPage{
    fn default() -> Self {
        PerPage(_MAX_PER_PAGE)
    }
}

impl Deref for PerPage{
    type Target= u32;
    fn deref(&self)-> &Self::Target {
        &self.0
    }
}

impl DerefMut for PerPage{
    fn deref_mut(&mut self)-> &mut Self::Target {
        &mut self.0 
    }
}

impl From<u32> for PerPage{
    fn from(value: u32) -> Self {
        PerPage(value)
    }
}

impl From<PerPage> for u32{
    fn from(value: PerPage) -> Self {
        value.0
    }
    
}

#[derive(Debug,Clone,PoirotBuilder)]
pub struct ArxivQuery{
    #[builder_mandatory]
    #[builder_no_method]
    pub query: QueryBody,
    pub from_date: Option<u64>,
    pub to_date: Option<u64>,
    #[builder_no_method]
    pub per_page: PerPage,
    pub from_page: Option<u32>,
    pub to_page: Option<u32>,
    pub sort_order: SortOrder,
    pub sort_by: SortBy,
}


impl Default for ArxivQuery{
    fn default() -> Self {
        ArxivQuery{
            query: QueryBody::default(), //should return QueryBody(Expr::Empty)
            from_date: None,
            to_date: None,
            per_page: _MAX_PER_PAGE.into(),
            // single page by default
            from_page: None,
            to_page: None,
            //
            sort_order: SortOrder::default(),
            sort_by: SortBy::default(),
        }
    }
}



fn tonow() -> u64 {
    let now = time::OffsetDateTime::now_utc();

    // Format: YYYYMMDDHHmm (hours + minutes)
    let year = now.year() as u64;
    let month = u8::from(now.month()) as u64;
    let day = now.day() as u64;
    let hour = now.hour() as u64;
    let minute = now.minute() as u64;

    format!{"{:04}{:02}{:02}{:02}{:02}", year, month, day, hour, minute}.parse().unwrap()
}

const FROMBEGINNING: u64 = 190001010000;

fn sort_by_to_string(sort_by: &SortBy) -> String {
    match sort_by {
        SortBy::Relevance => arxiv_text::RELEVANCE.to_string(),
        SortBy::LastUpdatedDate => arxiv_text::LAST_UPDATED_DATE.to_string(),
        SortBy::SubmittedDate =>  arxiv_text::SUBMITTED_DATE.to_string(),
    }
}

impl ArxivQuery{
    pub fn write_query(&self) -> Result<String,QueryError>{
        let body= self.query.to_string();
        let from_date = self.from_date;
        let to_date = self.to_date;
        // find if body has authors



        if let (Some(from_date), Some(to_date)) = (from_date, to_date) {
            if from_date > to_date {
                return Err(QueryError::WrongFormatting("from_date cannot be greater than to_date".to_string()));
            }
        }

        let time_range_str = match (from_date, to_date) {
            (None,None) => String::new(),
            _  => {
                format!("+{}[{}]", 
                    arxiv_text::SUBMITTED_DATE,
                    format!("{}+TO+{}",from_date.unwrap_or(FROMBEGINNING),to_date.unwrap_or(tonow())))
            }
        };


        let query_body= body.to_string();

        // if it's not empty, proceed
        (!query_body.is_empty())
            .then_some(())
            .ok_or(QueryError::WrongFormatting("Arxiv query body cannot be empty".to_string()))?;
            
        Ok(format!("{query_body}{time_range_str}"))
            
        
    }
}



impl SearchQuery for ArxivQuery{}

impl ArxivQueryBuilder {
    pub fn with_per_page(mut self, max: u32) -> Self {
        match max {
            _MIN_PER_PAGE.._MAX_PER_PAGE => {
                self.per_page = Some(max.into());
            },
            _MAX_PER_PAGE..=std::u32::MAX => {
                self.per_page = Some(_MAX_PER_PAGE.into());
            },
            _ => {
                self.per_page = Some(_MIN_PER_PAGE.into());
            }

        }
        self
    }
    pub fn with_query(mut self, query: String) -> Self {
        let parsed_query = QueryBody::parse(query).ok();
        self.query = parsed_query;
        self
    }
}
impl SearchQuery for Arc<RwLock<ArxivQuery>> {}


#[async_trait]
impl LiteratureProvider for ArxivProvider {
    type SearchQueryType = ArxivQuery;
    type SearchResultType = ArxivResult;
    async fn search(&self, query_read:Self::SearchQueryType) -> Result<Self::SearchResultType,Box<dyn Error+ Send + Sync>>{
        // Implementation for searching arXiv

        
        let arxiv_query = Arc::new(query_read.write_query()?);
        let per_page = *query_read.per_page;
        
        let sort_by = sort_by_to_string(&query_read.sort_by);

        let url_base=Arc::new(format!("{}?{}{}&{}{}&{}{}",arxiv_text::BASEURL,
            arxiv_text::SEARCH_QUERY, arxiv_query.as_str(),
            arxiv_text::SORT_BY, sort_by,
            arxiv_text::MAX_RESULTS, &query_read.per_page.to_string()));
                
        // I can clone u32 directly since it is already on the stack

        fn  pagination(from_page: Option<u32>, to_page: Option<u32>) -> Result<impl Iterator<Item = u32>,QueryError>{
            match (from_page, to_page) {
                (None,Some(to)) => {
                    Ok(0..to)
                },
                (Some(from),None) => {
                    Ok(from..from)
                },
                (Some(from), Some(to)) => {
                    if to < from {
                        return Err(QueryError::WrongFormatting("to_page cannot be less than from_page".to_string()));
                    }
                    Ok(from..to)
                },
                _ => Ok(0..0),
            }
        }

        
        let mut set = JoinSet::new();
        // Do all pages at once, but rate-limited
        for pag in pagination(query_read.from_page, query_read.to_page)? {
            
            let provider=self.clone();
            let url_base_clone=url_base.clone();
            set.spawn(async move {
                let start=per_page * pag;
                println!("Executing arXiv Query (page {} start {})", pag+1,start);
                provider.rate_limiter.until_ready().await;
                let url = format!("{}&start={start}",url_base_clone);
                println!("ArXiv Query URL: {}", url);

                let response= provider.client
                    .get(url)
                    .send()
                    .await;
                response

            });
        }


        set.join_all().await;

        Ok(ArxivResult::default())
    }
   
   async fn fetch_by_source_id(&self, _source_id: &str) -> Result<Box<dyn LibraryItem>, Box<dyn Error+ Send + Sync>>{
         // Implementation for fetching a literature item by source ID from arXiv
        Err(Box::new(QueryError::UnexpectedError("Not implemented".to_string())))
    }
}

#[allow(dead_code)]
impl ArxivProvider {
    // Additional helper methods specific to ArxivProvider can be added here
    async fn acquire_permit(&self) {
        self.rate_limiter.until_ready().await;
    }
}   

#[cfg(test)]
mod test{
    use super::*;

    use tokio::time::Instant;
    use tokio::task::JoinSet;
    #[test]
    fn test_build_arxiv_query(){
        
        let query = ArxivQuery::builder()
            .with_query(r#""quantum computing" && au:"John H. Doe""#.to_string())
            .with_per_page(150)
            .with_to_page(Some(1))
            .build()
            .unwrap();  
        
        let arc_query = Arc::new(RwLock::new(query));
        let arxiv_query_str = arc_query.read().unwrap().write_query().unwrap();

        println!("arXiv Query String: {}", arxiv_query_str);
        //assert!(arxiv_query_str.starts_with("http://export.arxiv.org/api/query?"));
        assert!(arxiv_query_str.contains(r#"au:"John%20H.%20Doe""#));
        assert!(arxiv_query_str.contains(r#""quantum%20computing""#));

        //assert_eq!(arxiv_query_str, r#"http://export.arxiv.org/api/query?search_query=all:"quantum%20computing"+AND+au:"John%20H.%20Doe"&sortBy=relevance&start=0&max_results=100"#);
        assert_eq!(arxiv_query_str, r#"all:"quantum%20computing"+AND+au:"John%20H.%20Doe""#);

    }


    #[tokio::test]
    async fn test_body(){
        let provider = ArxivProvider::new().unwrap();
        let query = ArxivQuery::builder()
            .with_query(r#""quantum computing" && au:"John H. Doe""#.to_string())
            .with_per_page(150)
            .with_from_page(Some(1))
            .build()
            .unwrap();
        
        

        let response = provider.search(query).await;

        assert!(response.is_ok());

    }

    #[tokio::test]
    async fn concurrent_searches_are_rate_limited() {
        let provider = ArxivProvider::new().unwrap();

        // build a single shared, read‑only query
        let query = ArxivQuery::builder()
            .with_query(r#""quantum computing" && au:"John H. Doe""#.to_string())
            .with_per_page(150)
            .with_from_page(Some(0))
            .with_to_page(Some(5))
            .build()
            .unwrap();
        
        let query =query;

        let start = Instant::now();

        // spawn 4 concurrent searched (should take less than 2s)
        
        let mut set = JoinSet::new();
        for i in 0..4 {
            let provider = provider.clone();
            let q = query.clone();
            set.spawn(async move {
                println!("Starting search #{}", i + 1);
                let _ = provider.search(q).await;
            });
        }

        set.join_all().await;

        let elapsed = start.elapsed();
        println!("Elapsed for 4 concurrent searches: {:?}", elapsed);

        assert!(
            elapsed.as_secs_f32() < 2.0,
            "rate limiter did not delay concurrent searches enough: {:?}",
            elapsed
        );


        // spawn 5 concurrent searches (should take more than 2s)
        
        let provider = ArxivProvider::new().unwrap();
        let mut set = JoinSet::new();
        for i in 0..5 {
            let provider = provider.clone();
            let q = query.clone();
            set.spawn(async move {
                println!("Starting search #{}", i + 1);
                let _ = provider.search(q).await;
            });
        }

        set.join_all().await;

        let elapsed = start.elapsed();
        println!("Elapsed for 5 concurrent searches: {:?}", elapsed);

        // With quota 4 per 2 seconds, the 5th must wait for the next window
        assert!(
            elapsed.as_secs_f32() >= 2.0,
            "rate limiter did not delay concurrent searches enough: {:?}",
            elapsed
        );


        

        // spawn 15 concurrent searches (should take more than 2s)
        
        let provider = ArxivProvider::new().unwrap();
        let mut set = JoinSet::new();
        for i in 0..15 {
            let provider = provider.clone();
            let q = query.clone();
            set.spawn(async move {
                println!("Starting search #{}", i + 1);
                let _ = provider.search(q).await;
            });
        }

        set.join_all().await;
        let elapsed = start.elapsed();
        println!("Elapsed for 5 concurrent searches: {:?}", elapsed);

        // With quota 4 per 2 seconds, the 5th must wait for the next window
        assert!(
            elapsed.as_secs_f32() >= 6.0,
            "rate limiter did not delay concurrent searches enough: {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn limiter_15_permits() {
        let limiter = Arc::new(create_rate_limiter().unwrap());

        let start = Instant::now();


        
        let mut set = JoinSet::new();
        
        for i in 0..15 {
            let limiter = limiter.clone();
            set.spawn(async move {
                limiter.until_ready().await;
                println!("Acquired permit #{}", i + 1);
            });
        }
        
        set.join_all().await;

        let elapsed = start.elapsed();
        println!("Elapsed for 15 permits (no HTTP): {:?}", elapsed);

        assert!(
            elapsed.as_secs_f32() >= 6.0 && elapsed.as_secs_f32() < 12.0,
            "unexpected limiter time: {:?}",
            elapsed
        );
    }
}