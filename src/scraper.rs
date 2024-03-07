pub enum ExternalSearchRequest {
    Simple(SimpleSearch),
    String((String, u32))
}

pub struct ValidSearch {
    pub sorts: Vec<String>,
    pub tags: Vec<String>,
    pub status: Vec<String>,
}

impl ValidSearch {
    pub fn anilist() -> Self {
        Self {
            sorts: vec![
                "popularity".to_string(),
                "score".to_string(),
                "trending".to_string(),
                "created".to_string(),
                "updated".to_string(),
            ],
            tags: vec![],
            status: vec![
                "releasing".to_string(),
                "finished".to_string(),
                "hiatus".to_string(),
                "cancelled".to_string(),
                "upcoming".to_string(),
            ],
        }
    }
}

pub struct SimpleSearch {
    pub search: Option<String>,
    pub sort: Option<String>,
    pub desc: bool,
    pub status: Option<String>,
    pub tags: Vec<String>,
    pub page: u32,
}

impl SimpleSearch {
    pub fn validate(&self, vs: &ValidSearch) -> bool {
        if let Some(v) = &self.sort {
            if !vs.sorts.contains(v) {
                return false;
            }
        }
        if let Some(v) = &self.status {
            if !vs.status.contains(v) {
                return false;
            }
        }
        for tag in &self.tags {
            if !vs.tags.contains(tag) {
                //TODO:
                //return false;
            }
        }
        true
    }
}