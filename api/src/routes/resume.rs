use worker::*;
use crate::models::resume::Resume;
use crate::errors::ApiError;
use crate::models::tag::Tag;

/// Get resume data with optional filtering
#[utoipa::path(
    get,
    path = "/v1/resume",
    tag = "resume",
    params(
        ("sections" = Option<String>, Query, description = "Filter sections (comma-separated): personal, experience, education, projects, extracurricular"),
        ("tags" = Option<String>, Query, description = "Filter by technology tags (comma-separated)"),
        ("format" = Option<String>, Query, description = "Output format: full or minimal"),
        ("limit" = Option<usize>, Query, description = "Limit items per section"),
    ),
    responses(
        (status = 200, description = "Resume data", body = crate::models::resume::Resume),
        (status = 404, description = "Resume not found")
    )
)]
pub async fn handle_get_resume(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Get KV namespace binding
    let kv = ctx.env.kv("RESUME_KV")?;

    // Fetch resume from KV
    let resume_data = match kv.get("resume").text().await? {
        Some(data) => data,
        None => {
            let error = ApiError::not_found("Resume");
            return error.to_response(404);
        }
    };

    // Parse resume JSON
    let mut resume: Resume = match serde_json::from_str(&resume_data) {
        Ok(r) => r,
        Err(_e) => {
            // Log detailed error server-side (when error logging is fully implemented)
            // console_error!("Failed to parse resume from KV: {:?}", e);
            let error = ApiError::internal_error("Unable to load resume data");
            return error.to_response(500);
        }
    };

    // Parse query parameters and apply filters
    let url = req.url()?;
    apply_filters(&mut resume, &url);

    // Add cache headers
    let mut response = Response::from_json(&resume)?;
    let headers = response.headers_mut();
    headers.set("Cache-Control", "public, max-age=3600")?; // Cache for 1 hour

    Ok(response)
}

/// Valid section names for validation
const VALID_SECTIONS: &[&str] = &["personal", "experience", "education", "projects", "extracurricular"];

fn apply_filters(resume: &mut Resume, url: &Url) {
    let query_pairs = url.query_pairs();

    for (key, value) in query_pairs {
        match key.as_ref() {
            "sections" => {
                let sections: Vec<String> = value
                    .split(',')
                    .map(|s| s.trim().to_lowercase())
                    .filter(|s| VALID_SECTIONS.contains(&s.as_str())) // Validate against allowlist
                    .collect();
                resume.filter_sections(&sections);
            }
            "tags" => {
                let tags = Tag::parse_many(&value);
                if !tags.is_empty() {
                    let tag_strings: Vec<String> = tags.iter().map(|t| t.to_string()).collect();
                    resume.filter_by_tags(&tag_strings);
                }
            }
            "format" if value == "minimal" => {
                resume.make_minimal();
            }
            "limit" => {
                if let Ok(limit) = value.parse::<usize>() {
                    // Cap limit at reasonable maximum
                    resume.limit_items(limit.min(100));
                }
            }
            _ => {}
        }
    }
}
