use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::search::Status;

#[derive(Serialize, Deserialize)]
pub struct MangaInfoRequest {
    pub manga_id: String
}

#[derive(Serialize, Deserialize)]
pub struct MangaInfoResponse {
    pub manga_id: String,
    pub titles: HashMap<String, Vec<String>>,
    kind: String,
    pub description: Option<String>,
    pub tags: Vec<Tag>,
    pub status: Status,
    pub visibility: Visibility,
    pub uploader: String,
    pub artists: Vec<String>,
    pub authors: Vec<String>,
    pub cover: u32,
    pub cover_ext: String,
    pub chapters: Vec<Chapter>,
    pub sources: Vec<ExternalSite>,
    pub relations: Vec<(String, String)>,
    pub scraper: bool,
    pub favorite: bool,
    /// manga_id
    pub progress: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct ExternalSite {
    url: String,
    icon_uri: String
}

#[derive(Serialize, Deserialize)]
pub struct Chapter {
    pub titles: Vec<String>,
    pub chapter: f64,
    pub tags: Vec<Tag>,
    pub sources: Vec<String>,
    pub release_date: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    tag: String,
    pub description: Option<String>,
    pub sex: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Visibility {

}