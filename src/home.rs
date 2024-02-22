use serde::{Deserialize, Serialize};
use crate::RequestImpl;
use crate::search::SearchResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct HomeResponse {
    pub trending: Vec<SearchResponse>,
    pub newest: Vec<SearchResponse>,
    pub latest_updates: Vec<SearchResponse>,
    pub favorites: Vec<SearchResponse>,
    pub reading: Vec<SearchResponse>,
}

impl RequestImpl for HomeResponse {
    const ROUTE: &'static str = "home";
    const AUTH: bool = true;
}