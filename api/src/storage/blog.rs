// Blog storage abstraction combining D1 + R2
use worker::*;
use crate::models::post::{Post, PostsResponse};
use crate::storage::d1::{ListPostsParams, list_posts as d1_list_posts, get_post_by_slug as d1_get_post_by_slug};
use crate::storage::r2::get_content;

/// Get full post with content from R2
pub async fn get_full_post(db: &D1Database, bucket: &Bucket, slug: &str) -> Result<Option<Post>> {
    // Get metadata from D1
    let mut post = match d1_get_post_by_slug(db, slug).await? {
        Some(p) => p,
        None => return Ok(None),
    };

    // Get content from R2
    let content = get_content(bucket, &post.content_id).await?;
    post.content = content;

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
