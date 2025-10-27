use utoipa::OpenApi;
use crate::models::blog::{Blog, BlogListItem, BlogsResponse, Pagination};
use crate::models::project::{Project as ProjectModel, ProjectUrl, ProjectsResponse};
use crate::models::resume::{Resume, Personal, Experience, Education, Project, Extracurricular};
use crate::models::tag::TagWithCount;
use crate::routes::meta::{HealthResponse, ServiceStatus, ApiInfoResponse, ApiEndpoints};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "werdxz API",
        version = env!("CARGO_PKG_VERSION"),
        description = "REST API for werdxz.info ecosystem - blog posts, resume data, and more",
        contact(
            name = "WERDXZ",
            email = "lwerdxzl@hotmail.com"
        )
    ),
    servers(
        (url = "https://api.werdxz.info", description = "Production"),
        (url = "http://localhost:8787", description = "Local development")
    ),
    paths(
        crate::routes::meta::handle_root,
        crate::routes::meta::handle_health,
        crate::routes::blogs::handle_list_blogs,
        crate::routes::blogs::handle_get_blog,
        crate::routes::blogs::handle_get_tags,
        crate::routes::projects::handle_list_projects,
        crate::routes::projects::handle_get_project,
        crate::routes::resume::handle_get_resume,
    ),
    components(
        schemas(
            Blog,
            BlogListItem,
            BlogsResponse,
            Pagination,
            TagWithCount,
            ProjectModel,
            ProjectUrl,
            ProjectsResponse,
            Resume,
            Personal,
            Experience,
            Education,
            Project,
            Extracurricular,
            HealthResponse,
            ServiceStatus,
            ApiInfoResponse,
            ApiEndpoints,
        )
    ),
    tags(
        (name = "meta", description = "API metadata and health endpoints"),
        (name = "blogs", description = "Blog post management"),
        (name = "projects", description = "Open-source projects and documentation"),
        (name = "resume", description = "Resume data and filtering")
    )
)]
pub struct ApiDoc;

pub fn get_openapi_spec() -> String {
    ApiDoc::openapi().to_pretty_json().unwrap_or_else(|_| {
        r#"{"error": "Failed to generate OpenAPI specification"}"#.to_string()
    })
}
