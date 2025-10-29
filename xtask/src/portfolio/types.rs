use serde::{Deserialize, Serialize};

/// Featured project for portfolio showcase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    pub links: Vec<ProjectLink>,
}

/// Link associated with a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectLink {
    pub label: String,
    pub url: String,
}

/// Work experience entry for portfolio showcase
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Contact form submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactSubmission {
    pub name: String,
    pub email: String,
    pub message: String,
    pub timestamp: u64,
}

impl Project {
    pub fn new(
        title: String,
        description: String,
        technologies: Vec<String>,
        image_url: String,
        redirect_url: Option<String>,
        links: Vec<ProjectLink>,
    ) -> Self {
        Self {
            title,
            description,
            technologies,
            image_url,
            redirect_url,
            links,
        }
    }
}

impl ProjectLink {
    pub fn new(label: String, url: String) -> Self {
        Self { label, url }
    }
}

impl Experience {
    pub fn new(
        company: String,
        role: String,
        period: String,
        location: Option<String>,
        description: String,
        technologies: Vec<String>,
        redirect_url: Option<String>,
    ) -> Self {
        Self {
            company,
            role,
            period,
            location,
            description,
            technologies,
            redirect_url,
        }
    }
}
