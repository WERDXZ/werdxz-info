/// Structured logging utilities for Cloudflare Workers
///
/// Provides macros for structured logging with automatic request_id correlation
use worker::console_log;

/// Log levels
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

/// Log a structured message with request_id
pub fn log(level: LogLevel, request_id: &str, message: &str, context: Option<&str>) {
    let log_entry = if let Some(ctx) = context {
        format!(
            "[{}] request_id={} {} | {}",
            level.as_str(),
            request_id,
            message,
            ctx
        )
    } else {
        format!(
            "[{}] request_id={} {}",
            level.as_str(),
            request_id,
            message
        )
    };

    console_log!("{}", log_entry);
}

/// Log an incoming request
pub fn log_request(request_id: &str, method: &str, path: &str, user_agent: Option<&str>) {
    let ua = user_agent.unwrap_or("unknown");
    log(
        LogLevel::Info,
        request_id,
        "incoming request",
        Some(&format!("method={} path={} user_agent=\"{}\"", method, path, ua)),
    );
}

/// Log a completed request with timing
pub fn log_response(request_id: &str, status: u16, duration_ms: u64) {
    log(
        LogLevel::Info,
        request_id,
        "request completed",
        Some(&format!("status={} duration_ms={}", status, duration_ms)),
    );
}

/// Log an error
#[allow(dead_code)]
pub fn log_error(request_id: &str, error: &str, context: Option<&str>) {
    log(LogLevel::Error, request_id, error, context);
}

/// Log a warning
#[allow(dead_code)]
pub fn log_warn(request_id: &str, warning: &str, context: Option<&str>) {
    log(LogLevel::Warn, request_id, warning, context);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_as_str() {
        assert_eq!(LogLevel::Info.as_str(), "INFO");
        assert_eq!(LogLevel::Warn.as_str(), "WARN");
        assert_eq!(LogLevel::Error.as_str(), "ERROR");
    }
}
