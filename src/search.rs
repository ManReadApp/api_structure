use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::error::ApiErr;
use crate::RequestImpl;
use crate::ApiErrorType;

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchResponse {
    pub manga_id: String,
    pub titles: HashMap<String, Vec<String>>,
    pub tags: Vec<String>,
    pub status: Status,
    pub ext: String,
    pub number: u32
}
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Status {
    Dropped,
    Hiatus,
    Ongoing,
    Completed,
    Upcoming
}

impl TryFrom<u64> for Status {
    type Error = ApiErr;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Dropped),
            1 => Ok(Self::Hiatus),
            2 => Ok(Self::Ongoing),
            3 => Ok(Self::Completed),
            4 => Ok(Self::Upcoming),
            _ => Err(ApiErr {
                message: Some("Couldnt find manga status".to_string()),
                cause: None,
                err_type: ApiErrorType::InternalError,
            })
        }
    }
}

impl From<Status> for u64 {
    fn from(value: Status) -> Self {
        match value {
            Status::Dropped => 0,
            Status::Hiatus => 1,
            Status::Ongoing => 2,
            Status::Completed => 3,
            Status::Upcoming => 4,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchRequest {
    pub order: Order,
    pub desc: bool,
    pub limit: u32,
    pub page: u32,
    pub query: ItemOrArray,
}

impl RequestImpl for SearchRequest {
    const ROUTE: &'static str = "search";
    const AUTH: bool = true;
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub enum Order {
    Id,
    Alphabetical,
    Updated,
    LastRead,
    Popularity,
}

/// can contain item or array
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ItemOrArray {
    Item(Item),
    Array(Array),
}

/// array joined with and or or
#[derive(Serialize, Deserialize, Debug)]
pub struct Array {
    pub or: bool,
    pub items: Vec<ItemOrArray>,
}

/// item include or exclude
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub not: bool,
    pub data: ItemData,
}

/// field and value
#[derive(Serialize, Deserialize, Debug)]
pub struct ItemData {
    pub name: String,
    pub value: ItemValue,
}

impl ItemData {
    pub fn enum_(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            value: ItemValue::None
        }
    }
}

/// define the type it should be parsed to
pub enum ItemKind {
    Bool,
    Int,
    String,
    CmpFloat,
    CmpInt,
    Float,
}

impl ItemKind {
    pub fn parse(&self, s: &str) -> Result<ItemValue, String> {
        Ok(match self {
            ItemKind::Bool => ItemValue::Bool(s.parse().map_err(|_| "")?),
            ItemKind::Int => ItemValue::Int(s.parse().map_err(|_| "")?),
            ItemKind::Float => ItemValue::Float(s.parse().map_err(|_| "")?),
            ItemKind::String => ItemValue::String(s.to_string()),
            ItemKind::CmpFloat => {
                let (a, b, c) = parse(s)?;
                ItemValue::CmpFloat {
                    eq: a,
                    bigger: b,
                    value: c,
                }
            }
            ItemKind::CmpInt => {
                let (a, b, c) = parse(s)?;
                ItemValue::CmpInt {
                    eq: a,
                    bigger: b,
                    value: c,
                }
            }
        })
    }
}

/// enum with different values
#[derive(Serialize, Deserialize, Debug)]
pub enum ItemValue {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    CmpFloat { eq: bool, bigger: bool, value: f32 },
    CmpInt { eq: bool, bigger: bool, value: i64 },
}

pub struct Field {
    pub name: String,
    pub abbr: Vec<String>,
    pub kind: ItemKind,
}

fn parse<T: FromStr>(s: &str) -> Result<(bool, bool, T), String> {
    let (str, b, s) = if let Some(v) = s.strip_prefix('>') {
        (v, true, false)
    } else if let Some(v) = s.strip_prefix('<') {
        (v, false, true)
    } else {
        (s, false, false)
    };
    let (eq, num) = if let Some(v) = str.strip_prefix('=') {
        (true, v)
    } else {
        (false, str)
    };
    Ok((
        eq,
        b == s || b,
        num.parse::<T>()
            .map_err(|_| format!("Failed to parse: {}", num))?,
    ))
}
