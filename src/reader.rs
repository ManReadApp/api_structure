use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize)]
pub struct MangaReaderResponse {
    pub manga_id: String,
    pub titles: HashMap<String, Vec<String>>,
    pub kind: String,
    pub description: Option<String>,
    pub chapters: Vec<ReaderChapter>,
    pub favorite: bool,
    /// manga_id
    pub open_chapter: String,
    pub progress: f64,
}

impl MangaReaderResponse {
    pub fn no_chapters(&self) -> bool {
        self.chapters.is_empty()
    }
    pub fn missing_chapters(&self) -> Vec<f64> {
        let ch = self.chapters.iter().map(|v| v.chapter).collect::<Vec<_>>();
        let max = max_f64(&ch);

        let ch = ch
            .into_iter()
            .map(|v| v.to_string())
            .collect::<HashSet<_>>();
        let mut missing = vec![];
        if let Some(v) = max {
            for num in 1..v.floor() as u32 {
                let num = (num as f64).to_string();
                if !ch.contains(&num) {
                    missing.push(num);
                }
            }
        }
        missing.into_iter().map(|v| v.parse().unwrap()).collect()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReaderChapter {
    pub chapter_id: String,
    pub titles: Vec<String>,
    pub chapter: f64,
    pub sources: Vec<String>,
    pub release_date: Option<String>,
    ///Version, versionchapter
    pub versions: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct MangaReaderRequest {
    pub manga_id: String,
    pub chapter_id: Option<String>,
}

fn max_f64(items: &Vec<f64>) -> Option<f64> {
    let mut max = None;
    for item in items {
        if let Some(max) = &mut max {
            if item > max {
                *max = *item;
            }
        }else {
            max = Some(*item)
        }
    }
    max
}