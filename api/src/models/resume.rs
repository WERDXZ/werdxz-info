use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Resume {
    #[serde(skip_serializing_if = "Option::is_none", rename = "$schema")]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal: Option<Personal>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub experience: Vec<Experience>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub education: Vec<Education>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub projects: Vec<Project>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extracurricular: Vec<Extracurricular>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Personal {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub website: String,
    pub github: String,
    pub linkedin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Experience {
    pub title: String,
    pub organization: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub location: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bullets: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Education {
    pub institution: String,
    pub degree: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub minors: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub location: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub gpa: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Project {
    pub title: String,
    pub date: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "liveUrl")]
    pub live_url: Option<String>,
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bullets: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default)]
    pub featured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Extracurricular {
    pub title: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub organization: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    pub dates: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub achievements: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl Resume {
    /// Filter by sections
    pub fn filter_sections(&mut self, sections: &[String]) {
        let section_set: std::collections::HashSet<_> = sections.iter().collect();

        if !section_set.contains(&"personal".to_string()) {
            self.personal = None;
        }
        if !section_set.contains(&"experience".to_string()) {
            self.experience.clear();
        }
        if !section_set.contains(&"education".to_string()) {
            self.education.clear();
        }
        if !section_set.contains(&"projects".to_string()) {
            self.projects.clear();
        }
        if !section_set.contains(&"extracurricular".to_string()) {
            self.extracurricular.clear();
        }
    }

    /// Filter by tags
    pub fn filter_by_tags(&mut self, tags: &[String]) {
        self.experience.retain(|item|
            item.tags.iter().any(|t| tags.contains(t))
        );
        self.projects.retain(|item|
            item.tags.iter().any(|t| tags.contains(t))
        );
        self.extracurricular.retain(|item|
            item.tags.iter().any(|t| tags.contains(t))
        );
    }

    /// Limit items per section
    pub fn limit_items(&mut self, limit: usize) {
        self.experience.truncate(limit);
        self.education.truncate(limit);
        self.projects.truncate(limit);
        self.extracurricular.truncate(limit);
    }

    /// Convert to minimal format (remove detailed fields)
    pub fn make_minimal(&mut self) {
        for exp in &mut self.experience {
            exp.description.clear();
            exp.bullets.clear();
        }
        for edu in &mut self.education {
            edu.minors.clear();
            edu.location.clear();
        }
        for proj in &mut self.projects {
            proj.bullets.clear();
        }
        for extra in &mut self.extracurricular {
            extra.description.clear();
            extra.achievements.clear();
        }
    }
}
