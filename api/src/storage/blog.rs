// Blog storage abstraction combining D1 + R2
use worker::*;
use crate::models::post::{Post, PostsResponse};
use crate::storage::d1::{ListPostsParams, list_posts as d1_list_posts, get_post_by_slug as d1_get_post_by_slug};
use crate::storage::r2::get_content;

/// CDN base URL for blog assets
const CDN_BASE_URL: &str = "https://cloud.werdxz.info";

/// Rewrite relative image URLs in markdown content to absolute CDN URLs
///
/// Transforms patterns like:
/// - `![alt](./image.png)` → `![alt](https://cloud.werdxz.info/posts/{slug}/image.png)`
/// - `![alt](image.png)` → `![alt](https://cloud.werdxz.info/posts/{slug}/image.png)`
///
/// Leaves absolute URLs (http://, https://) unchanged.
fn rewrite_image_urls(content: &str, slug: &str) -> String {
    use regex::Regex;

    // Match markdown image syntax: ![alt](url)
    // Capture groups: 1=alt text, 2=url
    let re = Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap();

    re.replace_all(content, |caps: &regex::Captures| {
        let alt = &caps[1];
        let url = &caps[2];

        // Skip absolute URLs
        if url.starts_with("http://") || url.starts_with("https://") {
            return caps[0].to_string();
        }

        // Remove leading ./ if present
        let clean_path = url.strip_prefix("./").unwrap_or(url);

        // Build CDN URL
        let cdn_url = format!("{}/posts/{}/{}", CDN_BASE_URL, slug, clean_path);

        format!("![{}]({})", alt, cdn_url)
    }).to_string()
}

/// Get full post with content from R2
pub async fn get_full_post(db: &D1Database, bucket: &Bucket, slug: &str) -> Result<Option<Post>> {
    // Get metadata from D1
    let mut post = match d1_get_post_by_slug(db, slug).await? {
        Some(p) => p,
        None => return Ok(None),
    };

    // Get content from R2
    let content = get_content(bucket, &post.content_id).await?;

    // Rewrite relative image URLs to CDN URLs
    post.content = content.map(|c| rewrite_image_urls(&c, slug));

    Ok(Some(post))
}

/// List posts with pagination
pub async fn list_posts_with_pagination(
    db: &D1Database,
    params: &ListPostsParams,
) -> Result<PostsResponse> {
    // Tag filtering now handled at SQL level in d1_list_posts
    let (posts, pagination) = d1_list_posts(db, params).await?;

    // TODO: Add read_time_minutes column to D1 and calculate during publishing
    // For now, read time is None in listings (would need to fetch full content)

    Ok(PostsResponse { posts, pagination })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_relative_image_with_dot_slash() {
        let content = "Some text\n![Raytraced Output](./image.png)\nMore text";
        let result = rewrite_image_urls(content, "raytracer");
        assert_eq!(
            result,
            "Some text\n![Raytraced Output](https://cloud.werdxz.info/posts/raytracer/image.png)\nMore text"
        );
    }

    #[test]
    fn test_rewrite_relative_image_without_dot_slash() {
        let content = "![screenshot](screenshot.png)";
        let result = rewrite_image_urls(content, "my-post");
        assert_eq!(
            result,
            "![screenshot](https://cloud.werdxz.info/posts/my-post/screenshot.png)"
        );
    }

    #[test]
    fn test_preserve_absolute_urls() {
        let content = "![logo](https://example.com/logo.png)";
        let result = rewrite_image_urls(content, "test");
        assert_eq!(result, "![logo](https://example.com/logo.png)");

        let content_http = "![logo](http://example.com/logo.png)";
        let result_http = rewrite_image_urls(content_http, "test");
        assert_eq!(result_http, "![logo](http://example.com/logo.png)");
    }

    #[test]
    fn test_rewrite_multiple_images() {
        let content = "![a](./a.png) text ![b](b.jpg) more ![c](https://cdn.com/c.gif)";
        let result = rewrite_image_urls(content, "slug");
        assert_eq!(
            result,
            "![a](https://cloud.werdxz.info/posts/slug/a.png) text ![b](https://cloud.werdxz.info/posts/slug/b.jpg) more ![c](https://cdn.com/c.gif)"
        );
    }

    #[test]
    fn test_rewrite_empty_alt_text() {
        let content = "![](./diagram.svg)";
        let result = rewrite_image_urls(content, "post");
        assert_eq!(
            result,
            "![](https://cloud.werdxz.info/posts/post/diagram.svg)"
        );
    }

    #[test]
    fn test_no_images_unchanged() {
        let content = "Just some text with [a link](https://example.com)";
        let result = rewrite_image_urls(content, "post");
        assert_eq!(result, content);
    }
}
