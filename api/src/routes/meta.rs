use worker::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

/// Simple health check result for D1
#[derive(Deserialize)]
struct HealthCheck {
    #[allow(dead_code)]
    result: i32,
}

/// Service status in health response
#[derive(Serialize, ToSchema)]
#[serde(untagged)]
pub enum ServiceStatus {
    Ok(String),
    Error { status: String, error: String },
}

/// Health check response
#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub services: HashMap<String, ServiceStatus>,
}

/// API root response
#[derive(Serialize, ToSchema)]
pub struct ApiInfoResponse {
    pub service: String,
    pub version: String,
    pub api_version: String,
    pub endpoints: ApiEndpoints,
}

/// API endpoints listing
#[derive(Serialize, ToSchema)]
pub struct ApiEndpoints {
    pub docs: String,
    pub openapi: String,
    pub health: String,
    pub blogs: String,
    pub resume: String,
}

/// Health check endpoint with dependency checks
#[utoipa::path(
    get,
    path = "/v1/health",
    tag = "meta",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 503, description = "Service is unhealthy", body = HealthResponse)
    )
)]
pub async fn handle_health(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let mut services = HashMap::new();
    let mut all_healthy = true;

    // Check D1 database
    match check_d1(&ctx).await {
        Ok(_) => services.insert("d1".to_string(), ServiceStatus::Ok("ok".to_string())),
        Err(e) => {
            all_healthy = false;
            services.insert("d1".to_string(), ServiceStatus::Error {
                status: "error".to_string(),
                error: e.to_string(),
            })
        }
    };

    // Check R2 bucket
    match check_r2(&ctx).await {
        Ok(_) => services.insert("r2".to_string(), ServiceStatus::Ok("ok".to_string())),
        Err(e) => {
            all_healthy = false;
            services.insert("r2".to_string(), ServiceStatus::Error {
                status: "error".to_string(),
                error: e.to_string(),
            })
        }
    };

    // Check KV namespace
    match check_kv(&ctx).await {
        Ok(_) => services.insert("kv".to_string(), ServiceStatus::Ok("ok".to_string())),
        Err(e) => {
            all_healthy = false;
            services.insert("kv".to_string(), ServiceStatus::Error {
                status: "error".to_string(),
                error: e.to_string(),
            })
        }
    };

    let health = HealthResponse {
        status: if all_healthy { "healthy" } else { "unhealthy" }.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        services,
    };

    let status_code = if all_healthy { 200 } else { 503 };
    Ok(Response::from_json(&health)?.with_status(status_code))
}

/// Check D1 database connectivity
async fn check_d1(ctx: &RouteContext<()>) -> Result<()> {
    let db = ctx.env.d1("DB")?;
    // Simple query to check connectivity
    let _ = db.prepare("SELECT 1 as result").first::<HealthCheck>(None).await?;
    Ok(())
}

/// Check R2 bucket connectivity
async fn check_r2(ctx: &RouteContext<()>) -> Result<()> {
    let _bucket = ctx.env.bucket("CONTENT_BUCKET")?;
    // Just checking if we can get the binding is enough
    Ok(())
}

/// Check KV namespace connectivity
async fn check_kv(ctx: &RouteContext<()>) -> Result<()> {
    let _kv = ctx.env.kv("RESUME_KV")?;
    // Just checking if we can get the binding is enough
    Ok(())
}

/// API root endpoint with service information
#[utoipa::path(
    get,
    path = "/",
    tag = "meta",
    responses(
        (status = 200, description = "API service information", body = ApiInfoResponse)
    )
)]
pub async fn handle_root(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let info = ApiInfoResponse {
        service: "werdxz-api".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        api_version: "v1".to_string(),
        endpoints: ApiEndpoints {
            docs: "/docs".to_string(),
            openapi: "/openapi.json".to_string(),
            health: "/v1/health".to_string(),
            blogs: "/v1/blogs".to_string(),
            resume: "/v1/resume".to_string(),
        },
    };
    Response::from_json(&info)
}

pub async fn handle_openapi_spec(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let spec = crate::openapi::get_openapi_spec();
    Response::from_html(spec)
        .map(|mut r| {
            let _ = r.headers_mut().set("Content-Type", "application/json");
            r
        })
}
