use crate::RequestImpl;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MangaCoverRequest {
    pub manga_id: String,
    pub file_ext: String,
}

impl RequestImpl for MangaCoverRequest {
    const ROUTE: &'static str = "cover";
    const AUTH: bool = true;
}
