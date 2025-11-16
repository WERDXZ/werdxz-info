use worker::*;

mod errors;
mod logging;
mod middleware;
mod models;
mod openapi;
mod routes;
mod storage;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let start = Date::now().as_millis();

    // Generate request ID
    let request_id = middleware::generate_request_id();

    // Log incoming request
    let method = req.method().to_string();
    let path = req.path();
    let user_agent = req.headers().get("User-Agent").ok().flatten();
    logging::log_request(&request_id, &method, &path, user_agent.as_deref());

    // Get origin for CORS
    let origin = req.headers().get("Origin").ok().flatten();

    // Handle OPTIONS preflight
    if req.method() == Method::Options {
        let response = middleware::handle_options()?;
        let duration_ms = Date::now().as_millis() - start;
        logging::log_response(&request_id, 204, duration_ms);
        return Ok(response);
    }

    // Initialize router
    let response = Router::new()
        // Meta endpoints (unversioned)
        .get_async("/", |req, ctx| async move { routes::meta::handle_root(req, ctx).await })
        .get_async("/openapi.json", |req, ctx| async move { routes::meta::handle_openapi_spec(req, ctx).await })

        // v1 API endpoints
        .get_async("/v1/health", |req, ctx| async move { routes::meta::handle_health(req, ctx).await })
        .get_async("/v1/posts", |req, ctx| async move { routes::posts::handle_list_posts(req, ctx).await })
        .get_async("/v1/posts/:slug", |req, ctx| async move { routes::posts::handle_get_post(req, ctx).await })
        .get_async("/v1/tags", |req, ctx| async move { routes::posts::handle_get_tags(req, ctx).await })
        .get_async("/v1/projects", |req, ctx| async move { routes::projects::handle_list_projects(req, ctx).await })
        .get_async("/v1/projects/:slug", |req, ctx| async move { routes::projects::handle_get_project(req, ctx).await })
        .get_async("/v1/resume", |req, ctx| async move { routes::resume::handle_get_resume(req, ctx).await })

        .run(req, env)
        .await?;

    // Apply middleware to response
    let response = middleware::add_cors_headers(response, origin.as_deref())?;
    let response = middleware::add_request_id_header(response, &request_id)?;

    // Log response
    let duration_ms = Date::now().as_millis() - start;
    let status = response.status_code();
    logging::log_response(&request_id, status, duration_ms);

    Ok(response)
}
