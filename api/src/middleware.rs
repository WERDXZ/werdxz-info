use worker::*;

/// Generate a unique request ID
pub fn generate_request_id() -> String {
    // Use a simple counter-based ID for now
    // In production, Workers provides request IDs automatically
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("req_{:x}", id)
}

/// Add CORS headers to response
pub fn add_cors_headers(response: Response, origin: Option<&str>) -> Result<Response> {
    let headers = response.headers();

    // Check if origin is from allowed domains
    let allowed_origin = match origin {
        Some(o) if is_allowed_origin(o) => o,
        _ => {
            // Reject unrecognized origins by not setting CORS headers
            // Browser will block the request
            return Ok(response);
        }
    };

    headers.set("Access-Control-Allow-Origin", allowed_origin)?;
    headers.set("Access-Control-Allow-Methods", "GET, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type, X-Request-ID")?;
    headers.set("Access-Control-Max-Age", "86400")?;

    Ok(response)
}

/// Check if origin is allowed
/// Default: *.werdxz.info and werdxz.info (HTTPS only)
/// Opt-in via compile-time env: ALLOWED_ORIGINS (comma-separated)
fn is_allowed_origin(origin: &str) -> bool {
    // Always allow werdxz.info domains (HTTPS only for security)
    if origin.ends_with(".werdxz.info") || origin == "https://werdxz.info" {
        return true;
    }

    // Check compile-time ALLOWED_ORIGINS environment variable
    // Set via: ALLOWED_ORIGINS="http://localhost:3000,http://127.0.0.1:3000" cargo build
    // Or in wrangler.toml: [env.dev.build] command = "ALLOWED_ORIGINS=... worker-build"
    if let Some(allowed_origins) = option_env!("ALLOWED_ORIGINS") {
        let origins: Vec<&str> = allowed_origins.split(',').map(|s| s.trim()).collect();
        if origins.contains(&origin) {
            return true;
        }
    }

    false
}

/// Handle OPTIONS preflight requests
pub fn handle_options() -> Result<Response> {
    let mut response = Response::empty()?;
    response = add_cors_headers(response, None)?;
    Ok(response.with_status(204))
}

/// Add request ID header to response
pub fn add_request_id_header(mut response: Response, request_id: &str) -> Result<Response> {
    response.headers_mut().set("X-Request-ID", request_id)?;
    Ok(response)
}
