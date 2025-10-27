// Blog storage abstraction combining D1 + R2
use worker::*;
use crate::models::blog::{Blog, BlogsResponse};
use crate::storage::d1::{ListBlogsParams, list_blogs as d1_list_blogs, get_blog_by_slug as d1_get_blog_by_slug};
use crate::storage::r2::get_content;

/// Get full blog with content from R2
pub async fn get_full_blog(db: &D1Database, bucket: &Bucket, slug: &str) -> Result<Option<Blog>> {
    // Get metadata from D1
    let mut blog = match d1_get_blog_by_slug(db, slug).await? {
        Some(b) => b,
        None => return Ok(None),
    };

    // Get content from R2
    let content = get_content(bucket, &blog.content_id).await?;
    blog.content = content;

    Ok(Some(blog))
}

/// List blogs with pagination
pub async fn list_blogs_with_pagination(
    db: &D1Database,
    params: &ListBlogsParams,
) -> Result<BlogsResponse> {
    // Tag filtering now handled at SQL level in d1_list_blogs
    let (blogs, pagination) = d1_list_blogs(db, params).await?;

    // TODO: Add read_time_minutes column to D1 and calculate during publishing
    // For now, read time is None in listings (would need to fetch full content)

    Ok(BlogsResponse { blogs, pagination })
}
