use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseRoot {
    pub data: Option<Vec<Datum>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Datum {
    pub id: String,
    #[serde(rename = "type")]
    pub datum_type: String,
    pub attributes: DatumAttributes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DatumAttributes {
    pub title: serde_json::Value,
    pub desc: serde_json::Value,
    pub excerpt: serde_json::Value,
    pub thumb: serde_json::Value,
    pub cover: serde_json::Value,
    pub comments_count: serde_json::Value,
    pub likes_count: serde_json::Value,
    pub bookmarks_count: serde_json::Value,
    pub published_at: serde_json::Value,
    pub duration: serde_json::Value,
}
