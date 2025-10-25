use worker::*;

/// Retrieve markdown content from R2
///
/// Note: Admin operations (PUT, DELETE, LIST) should be done via wrangler CLI:
/// - wrangler r2 object put cloud posts/uuid.md --file post.md
/// - wrangler r2 object delete cloud posts/uuid.md
/// - wrangler r2 object list cloud --prefix posts/
pub async fn get_content(bucket: &Bucket, content_id: &str) -> Result<Option<String>> {
    let key = format!("posts/{}.md", content_id);

    match bucket.get(&key).execute().await? {
        Some(object) => {
            let content = object.body()
                .ok_or_else(|| Error::RustError("Failed to get object body".to_string()))?
                .text()
                .await?;
            Ok(Some(content))
        }
        None => Ok(None),
    }
}
