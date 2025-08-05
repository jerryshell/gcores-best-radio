use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseRoot {
    pub data: Option<Vec<Datum>>,
    // pub included: Option<Vec<Included>>,
    // pub meta: Option<ResponseRootMeta>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Datum {
    pub id: String,
    #[serde(rename = "type")]
    pub datum_type: String,
    pub attributes: DatumAttributes,
    // pub relationships: Option<Relationships>,
    // pub meta: Option<DatumMeta>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DatumAttributes {
    pub title: serde_json::Value,
    pub desc: serde_json::Value,
    pub excerpt: serde_json::Value,
    pub is_published: serde_json::Value,
    pub thumb: serde_json::Value,
    pub app_cover: serde_json::Value,
    pub cover: serde_json::Value,
    pub comments_count: serde_json::Value,
    pub likes_count: serde_json::Value,
    pub bookmarks_count: serde_json::Value,
    pub is_verified: serde_json::Value,
    pub published_at: serde_json::Value,
    pub option_is_official: serde_json::Value,
    pub option_is_focus_showcase: serde_json::Value,
    pub duration: serde_json::Value,
    pub is_free: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DatumMeta {
    pub vote_flag: Option<serde_json::Value>,
    pub vote_id: Option<serde_json::Value>,
    pub bookmark_id: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Relationships {
    pub user: Option<Category>,
    pub category: Option<Category>,
    pub comments: Option<Relationship>,
    pub similarities: Option<Relationship>,
    pub media: Option<Relationship>,
    pub djs: Option<Djs>,
    pub entries: Option<Relationship>,
    pub entities: Option<Relationship>,
    pub tags: Option<Relationship>,
    pub catalog_tags: Option<Relationship>,
    pub portfolios: Option<Relationship>,
    pub audit_draft: Option<Relationship>,
    pub collections: Option<Relationship>,
    pub operational_events: Option<Relationship>,
    pub latest_collection: Option<Relationship>,
    pub albums: Option<Relationship>,
    pub latest_album: Option<Relationship>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationship {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub data: Option<Dat>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dat {
    #[serde(rename = "type")]
    pub dat_type: Option<DataType>,
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
    Categories,
    Users,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Djs {
    pub data: Option<Vec<Dat>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Included {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub included_type: Option<DataType>,
    pub attributes: Option<IncludedAttributes>,
    pub relationships: Option<HashMap<String, Relationship>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IncludedAttributes {
    pub nickname: Option<String>,
    pub thumb: Option<String>,
    pub location: Option<String>,
    pub is_fresh: Option<bool>,
    pub intro: Option<String>,
    pub sex: Option<i64>,
    pub followers_count: Option<i64>,
    pub followees_count: Option<i64>,
    pub created_at: Option<String>,
    pub psn_id: Option<String>,
    pub live_id: Option<String>,
    pub nintendo_friendcode: Option<String>,
    pub steam_id: Option<String>,
    pub is_deleted: Option<bool>,
    pub has_membership: Option<bool>,
    pub is_founder_member: Option<bool>,
    pub is_treated: Option<bool>,
    pub disable_image_download: Option<bool>,
    pub medals_count: Option<i64>,
    pub certified_info: Option<serde_json::Value>,
    pub is_gcores_official: Option<bool>,
    pub is_pro: Option<bool>,
    pub using_grpg_avatar: Option<bool>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub logo: Option<String>,
    pub background: Option<String>,
    pub updated_at: Option<String>,
    pub subscriptions_count: Option<i64>,
    pub scope: Option<String>,
    pub display_order: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResponseRootMeta {
    pub record_count: Option<i64>,
}
