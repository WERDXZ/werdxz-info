use serde::{Deserialize, Serialize};

/// Portfolio mode/persona
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    SoftwareEngineer,
    Fullstack,
    Rust,
    Student,
}

impl Mode {
    /// Convert from URL path segment
    pub fn from_path(path: &str) -> Self {
        match path {
            "fullstack" => Mode::Fullstack,
            "rust" => Mode::Rust,
            "student" => Mode::Student,
            _ => Mode::SoftwareEngineer,
        }
    }

    /// Convert to URL path segment
    pub fn to_path(&self) -> &'static str {
        match self {
            Mode::SoftwareEngineer => "",
            Mode::Fullstack => "fullstack",
            Mode::Rust => "rust",
            Mode::Student => "student",
        }
    }

    /// Get KV namespace tag for this mode
    pub fn to_tag(&self) -> &'static str {
        match self {
            Mode::SoftwareEngineer => "software-engineer",
            Mode::Fullstack => "fullstack",
            Mode::Rust => "rust",
            Mode::Student => "student",
        }
    }

    /// Get hero subtitle for this mode
    pub fn subtitle(&self) -> &'static str {
        match self {
            Mode::SoftwareEngineer => "Software Engineer",
            Mode::Fullstack => "Full Stack Developer",
            Mode::Rust => "Rust Developer",
            Mode::Student => "Student & Developer",
        }
    }

    /// Get hero description for this mode
    pub fn description(&self) -> &'static str {
        match self {
            Mode::SoftwareEngineer => {
                "Building scalable systems and developer tools with Rust, TypeScript, and modern cloud infrastructure."
            }
            Mode::Fullstack => {
                "Crafting end-to-end web applications with modern frameworks, APIs, and cloud deployment."
            }
            Mode::Rust => {
                "Systems programming and high-performance tools with Rust, WebAssembly, and native development."
            }
            Mode::Student => {
                "Learning and building projects across systems programming, web development, and cloud infrastructure."
            }
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::SoftwareEngineer
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
    pub technologies: Vec<String>,
    pub image_url: String,
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
    pub technologies: Vec<String>,
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
    pub description: String,
}

/// About section content from KV (mode-specific)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AboutContent {
    pub paragraphs: Vec<String>,
}
