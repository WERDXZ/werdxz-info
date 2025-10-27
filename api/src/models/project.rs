use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProjectUrl {
    pub label: String,
    pub url: String,
}

impl ProjectUrl {
    /// Deserialize URLs from JSON array string or empty array
    pub fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<Self>, D::Error>
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
}

/// Helper to deserialize tags from JSON array string
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

/// Helper to deserialize SQLite INTEGER (0 or 1) to bool
fn deserialize_bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value: i64 = Deserialize::deserialize(deserializer)?;
    Ok(value != 0)
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Project {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub stage: String,
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub open_to_contributors: bool,
    pub readme_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty", deserialize_with = "deserialize_tags")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", deserialize_with = "ProjectUrl::deserialize_vec")]
    pub urls: Vec<ProjectUrl>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProjectsResponse {
    pub projects: Vec<Project>,
}
