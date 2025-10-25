use serde::{Deserialize, Serialize};
use worker::*;

/// API error response
/// Note: Request IDs are included in the X-Request-ID response header, not in the body
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

impl ApiError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::new("NOT_FOUND", format!("{} not found", resource.into()))
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new("INTERNAL_ERROR", message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new("BAD_REQUEST", message)
    }

    pub fn to_response(&self, status: u16) -> Result<Response> {
        let body = serde_json::json!({
            "error": self
        });
        Ok(Response::from_json(&body)?.with_status(status))
    }
}
