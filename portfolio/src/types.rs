use serde::{Deserialize, Serialize};

/// Portfolio mode/persona
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Industry,
    Academia,
}

impl Mode {
    /// Convert from URL path segment
    pub fn from_path(path: &str) -> Self {
        match path {
            "industry" => Mode::Industry,
            "academia" => Mode::Academia,
            _ => Mode::Industry,
        }
    }

    /// Convert to URL path segment
    pub fn to_path(&self) -> &'static str {
        match self {
            Mode::Industry => "industry",
            Mode::Academia => "academia",
        }
    }

    /// Get KV namespace tag for this mode
    pub fn to_tag(&self) -> &'static str {
        match self {
            Mode::Industry => "industry",
            Mode::Academia => "academia",
        }
    }

    /// Get hero subtitle for this mode
    pub fn subtitle(&self) -> &'static str {
        match self {
            Mode::Industry => "Software Engineer",
            Mode::Academia => "Researcher & Educator",
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Industry
    }
}

/// Link for projects (e.g., "Website", "GitHub", "API")
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectLink {
    pub label: String,
    pub url: String,
}

/// Featured project for portfolio display
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    pub links: Vec<ProjectLink>,
}

/// Work experience for portfolio display
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Experience {
    pub company: String,
    pub role: String,
    pub period: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub description: String,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

/// Blog post from API (featured posts)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub published_at: String,
    pub tags: Vec<String>,
}

/// Hero content from KV (mode-specific)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeroContent {
    pub subtitle: String,
}

/// About section content from KV (mode-specific)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AboutContent {
    pub paragraphs: Vec<String>,
}
