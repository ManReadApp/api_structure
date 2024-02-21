use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ScrapeAccount {
    pub username: String,
    pub password: String
}

impl ScrapeAccount {
    pub fn new(user: String, pass: String) -> Self {
        Self {
            username: user,
            password: pass,
        }
    }
}