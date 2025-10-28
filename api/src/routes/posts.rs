use worker::*;
use crate::storage::d1::ListPostsParams;
use crate::storage::blog::{list_posts_with_pagination, get_full_post};
use crate::storage::d1::get_all_tags as d1_get_all_tags;
use crate::errors::ApiError;
use crate::models::tag::Tag;

/// List blog posts with pagination and filtering
#[utoipa::path(
    get,
    path = "/v1/posts",
    tag = "posts",
    params(
        ("page" = Option<u32>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<u32>, Query, description = "Items per page (default: 10, max: 50)"),
        ("tags" = Option<String>, Query, description = "Filter by tags (comma-separated)"),
        ("search" = Option<String>, Query, description = "Search in title and summary"),
        ("sort" = Option<String>, Query, description = "Sort field: published_at or title (default: published_at)"),
        ("order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of blog posts", body = crate::models::post::PostsResponse)
    )
)]
pub async fn handle_list_posts(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Get D1 database binding
    let db = ctx.env.d1("DB")?;

    // Parse query parameters
    let url = req.url()?;
    let params = parse_list_params(&url);

    // Query posts
    match list_posts_with_pagination(&db, &params).await {
        Ok(response) => Response::from_json(&response),
        Err(_e) => {
            // Log detailed error server-side (when error logging is fully implemented)
            // console_error!("Failed to list posts: {:?}", e);
            let error = ApiError::internal_error("Unable to load posts");
            error.to_response(500)
        }
    }
}

/// Get a single blog post by slug
#[utoipa::path(
    get,
    path = "/v1/posts/{slug}",
    tag = "posts",
    params(
        ("slug" = String, Path, description = "URL slug of the post")
    ),
    responses(
        (status = 200, description = "Full blog post with content", body = crate::models::post::Post),
        (status = 400, description = "Invalid slug format"),
        (status = 404, description = "Post not found")
    )
)]
pub async fn handle_get_post(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let slug = ctx.param("slug").ok_or_else(|| Error::RustError("Missing slug".to_string()))?;

    // Validate slug format (security: prevent path traversal, SQL injection attempts)
    if !is_valid_slug(slug) {
        let error = ApiError::bad_request("Invalid post slug format");
        return error.to_response(400);
    }

    // Get D1 database and R2 bucket bindings
    let db = ctx.env.d1("DB")?;
    let bucket = ctx.env.bucket("CONTENT_BUCKET")?;

    // Get post with content
    match get_full_post(&db, &bucket, slug).await {
        Ok(Some(post)) => Response::from_json(&post),
        Ok(None) => {
            let error = ApiError::not_found("Post");
            error.to_response(404)
        }
        Err(e) => {
            // Log detailed error server-side
            console_error!("Failed to get post '{}': {:?}", slug, e);
            let error = ApiError::internal_error("Unable to load post");
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

/// Parse query parameters for list endpoint
fn parse_list_params(url: &Url) -> ListPostsParams {
    let query_pairs = url.query_pairs();

    let mut params = ListPostsParams::default();

    for (key, value) in query_pairs {
        match key.as_ref() {
            "page" => {
                if let Ok(p) = value.parse::<u32>() {
                    params.page = p.max(1);
                }
            }
            "limit" => {
                if let Ok(l) = value.parse::<u32>() {
                    params.limit = l.clamp(1, 50);
                }
            }
            "tags" => {
                let tags = Tag::parse_many(&value);
                if !tags.is_empty() {
                    params.tags = Some(tags.iter().map(|t| t.to_string()).collect());
                }
            }
            "search" => {
                if !value.is_empty() {
                    params.search = Some(value.to_string());
                }
            }
            "sort" => {
                params.sort_by = crate::storage::d1::SortField::from_str(&value);
            }
            "order" => {
                params.order = crate::storage::d1::SortOrder::from_str(&value);
            }
            _ => {}
        }
    }

    params
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_slug() {
        // Valid slugs
        assert!(is_valid_slug("hello-world"));
        assert!(is_valid_slug("rust_programming"));
        assert!(is_valid_slug("post-2024"));
        assert!(is_valid_slug("a"));

        // Invalid slugs
        assert!(!is_valid_slug("")); // Empty
        assert!(!is_valid_slug("../../etc/passwd")); // Path traversal
        assert!(!is_valid_slug("slug with spaces")); // Spaces
        assert!(!is_valid_slug("slug/with/slashes")); // Slashes
        assert!(!is_valid_slug("slug.with.dots")); // Dots
        assert!(!is_valid_slug("'; DROP TABLE posts; --")); // SQL injection attempt
        assert!(!is_valid_slug(&"a".repeat(101))); // Too long (> 100 chars)
    }

    #[test]
    fn test_parse_list_params_defaults() {
        let url = Url::parse("http://example.com/posts").unwrap();
        let params = parse_list_params(&url);

        assert_eq!(params.page, 1);
        assert_eq!(params.limit, 10);
        assert!(params.tags.is_none());
    }

    #[test]
    fn test_parse_list_params_with_values() {
        let url = Url::parse("http://example.com/posts?page=2&limit=20&tags=rust,webdev").unwrap();
        let params = parse_list_params(&url);

        assert_eq!(params.page, 2);
        assert_eq!(params.limit, 20);
        assert_eq!(params.tags, Some(vec!["rust".to_string(), "webdev".to_string()]));
    }

    #[test]
    fn test_parse_list_params_filters_invalid_tags() {
        let url = Url::parse("http://example.com/posts?tags=rust,invalid tag,python,../../etc/passwd").unwrap();
        let params = parse_list_params(&url);

        // Only valid tags should be included
        assert_eq!(params.tags, Some(vec!["rust".to_string(), "python".to_string()]));
    }

    #[test]
    fn test_parse_list_params_limits() {
        // Page minimum
        let url = Url::parse("http://example.com/posts?page=0").unwrap();
        let params = parse_list_params(&url);
        assert_eq!(params.page, 1);

        // Limit minimum and maximum
        let url = Url::parse("http://example.com/posts?limit=0").unwrap();
        let params = parse_list_params(&url);
        assert_eq!(params.limit, 1);

        let url = Url::parse("http://example.com/posts?limit=999").unwrap();
        let params = parse_list_params(&url);
        assert_eq!(params.limit, 50); // Clamped to max
    }

    #[test]
    fn test_parse_list_params_invalid_values() {
        let url = Url::parse("http://example.com/posts?page=invalid&limit=bad").unwrap();
        let params = parse_list_params(&url);

        // Should use defaults when parsing fails
        assert_eq!(params.page, 1);
        assert_eq!(params.limit, 10);
    }
}

/// Get all available tags with usage counts
#[utoipa::path(
    get,
    path = "/v1/tags",
    tag = "posts",
    responses(
        (status = 200, description = "List of all tags with usage counts", body = Vec<crate::models::tag::TagWithCount>)
    )
)]
pub async fn handle_get_tags(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Get D1 database binding
    let db = ctx.env.d1("DB")?;

    // Query all tags with counts
    match d1_get_all_tags(&db).await {
        Ok(tags) => Response::from_json(&tags),
        Err(_e) => {
            // Log detailed error server-side (when error logging is fully implemented)
            // console_error!("Failed to list tags: {:?}", e);
            let error = ApiError::internal_error("Unable to load tags");
            error.to_response(500)
        }
    }
}
