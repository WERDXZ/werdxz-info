use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;

/// Deserialize tags from JSON array string or empty array
fn deserialize_tags<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(json_str) if !json_str.is_empty() && json_str != "null" => {
            serde_json::from_str(&json_str).map_err(serde::de::Error::custom)
        }
        _ => Ok(Vec::new()),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Post {
    pub content_id: String,
    pub slug: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,  // Fetched from R2, not in DB
    pub published_at: String,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty", deserialize_with = "deserialize_tags")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostListItem {
    pub slug: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub summary: String,
    pub published_at: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty", deserialize_with = "deserialize_tags")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_time_minutes: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostsResponse {
    pub posts: Vec<PostListItem>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Pagination {
    pub page: u32,
    pub limit: u32,
    pub total: u32,
    pub has_next: bool,
}
