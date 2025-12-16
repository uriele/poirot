
use crate::domain::library_items::{
    LibraryItemType
};

//use time::{Date,};
use std::fmt::Debug;

pub trait SearchQuery: Sync+Send {}
pub trait SearchResult: Sync+Send {}