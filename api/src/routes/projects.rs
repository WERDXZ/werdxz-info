use worker::*;
use crate::storage::d1::{get_all_projects, get_project_by_slug};
use crate::errors::ApiError;
use crate::models::project::ProjectsResponse;

/// List all projects
#[utoipa::path(
    get,
    path = "/v1/projects",
    tag = "projects",
    responses(
        (status = 200, description = "List of all projects", body = crate::models::project::ProjectsResponse)
    )
)]
pub async fn handle_list_projects(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Get D1 database binding
    let db = ctx.env.d1("DB")?;

    // Query all projects
    match get_all_projects(&db).await {
        Ok(projects) => {
            let response = ProjectsResponse { projects };
            Response::from_json(&response)
        }
        Err(e) => {
            console_error!("Failed to list projects: {:?}", e);
            let error = ApiError::internal_error("Unable to load projects");
            error.to_response(500)
        }
    }
}

/// Get a single project by slug
#[utoipa::path(
    get,
    path = "/v1/projects/{slug}",
    tag = "projects",
    params(
        ("slug" = String, Path, description = "URL slug of the project")
    ),
    responses(
        (status = 200, description = "Project details", body = crate::models::project::Project),
        (status = 400, description = "Invalid slug format"),
        (status = 404, description = "Project not found")
    )
)]
pub async fn handle_get_project(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let slug = ctx.param("slug").ok_or_else(|| Error::RustError("Missing slug".to_string()))?;

    // Validate slug format (security: prevent path traversal, SQL injection attempts)
    if !is_valid_slug(slug) {
        let error = ApiError::bad_request("Invalid project slug format");
        return error.to_response(400);
    }

    // Get D1 database binding
    let db = ctx.env.d1("DB")?;

    // Get project
    match get_project_by_slug(&db, slug).await {
        Ok(Some(project)) => Response::from_json(&project),
        Ok(None) => {
            let error = ApiError::not_found("Project");
            error.to_response(404)
        }
        Err(e) => {
            // Log detailed error server-side
            console_error!("Failed to get project '{}': {:?}", slug, e);
            let error = ApiError::internal_error("Unable to load project");
            error.to_response(500)
        }
    }
}

/// Validate slug format for security
/// Allows: alphanumeric, hyphens, underscores
/// Max length: 100 characters
fn is_valid_slug(slug: &str) -> bool {
    !slug.is_empty()
        && slug.len() <= 100
        && slug.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_slug() {
        // Valid slugs
        assert!(is_valid_slug("werdxz-info"));
        assert!(is_valid_slug("example-cli-tool"));
        assert!(is_valid_slug("project_2024"));
        assert!(is_valid_slug("a"));

        // Invalid slugs
        assert!(!is_valid_slug("")); // Empty
        assert!(!is_valid_slug("../../etc/passwd")); // Path traversal
        assert!(!is_valid_slug("slug with spaces")); // Spaces
        assert!(!is_valid_slug("slug/with/slashes")); // Slashes
        assert!(!is_valid_slug("slug.with.dots")); // Dots
        assert!(!is_valid_slug("'; DROP TABLE projects; --")); // SQL injection attempt
        assert!(!is_valid_slug(&"a".repeat(101))); // Too long (> 100 chars)
    }
}
