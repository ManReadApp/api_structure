use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
