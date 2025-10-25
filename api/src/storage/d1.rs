use worker::*;
use serde::Deserialize;
use crate::models::post::{Post, PostListItem, Pagination};

/// Count query result
#[derive(Deserialize)]
struct CountResult {
    count: u32,
}

/// Sort field for blog posts
#[derive(Debug, Clone, Copy)]
pub enum SortField {
    PublishedAt,
    Title,
}

impl SortField {
    pub fn from_str(s: &str) -> Self {
        match s {
            "title" => Self::Title,
            _ => Self::PublishedAt, // Default to published_at
        }
    }

    pub fn to_sql(self) -> &'static str {
        match self {
            Self::PublishedAt => "published_at",
            Self::Title => "title",
        }
    }
}

/// Sort order
#[derive(Debug, Clone, Copy)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl SortOrder {
    pub fn from_str(s: &str) -> Self {
        match s {
            "asc" => Self::Asc,
            _ => Self::Desc, // Default to desc
        }
    }

    pub fn to_sql(self) -> &'static str {
        match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        }
    }
}

/// Query parameters for listing posts
pub struct ListPostsParams {
    pub page: u32,
    pub limit: u32,
    pub tags: Option<Vec<String>>,
    pub sort_by: SortField,
    pub order: SortOrder,
}

impl Default for ListPostsParams {
    fn default() -> Self {
        Self {
            page: 1,
            limit: 10,
            tags: None,
            sort_by: SortField::PublishedAt,
            order: SortOrder::Desc,
        }
    }
}

/// Query builder for type-safe SQL construction
struct PostQuery {
    has_tag_filter: bool,
    sort_field: SortField,
    sort_order: SortOrder,
}

impl PostQuery {
    fn new(params: &ListPostsParams) -> Self {
        Self {
            has_tag_filter: params.tags.is_some(),
            sort_field: params.sort_by,
            sort_order: params.order,
        }
    }

    fn count_query(&self) -> &'static str {
        if self.has_tag_filter {
            "SELECT COUNT(DISTINCT p.content_id) as count FROM posts p \
             INNER JOIN post_tags pt ON p.content_id = pt.post_id \
             INNER JOIN tags t ON pt.tag_id = t.id \
             WHERE p.published_at <= datetime('now') AND t.name IN (SELECT json_each.value FROM json_each(?))"
        } else {
            "SELECT COUNT(*) as count FROM posts p WHERE p.published_at <= datetime('now')"
        }
    }

    fn select_query(&self) -> String {
        let base = "SELECT p.slug, p.title, p.summary, p.published_at, p.external_url, \
                    (SELECT json_group_array(t.name) FROM post_tags pt \
                     INNER JOIN tags t ON pt.tag_id = t.id \
                     WHERE pt.post_id = p.content_id) as tags \
                    FROM posts p";

        let where_clause = if self.has_tag_filter {
            " INNER JOIN post_tags pt ON p.content_id = pt.post_id \
              INNER JOIN tags t ON pt.tag_id = t.id \
              WHERE p.published_at <= datetime('now') AND t.name IN (SELECT json_each.value FROM json_each(?))"
        } else {
            " WHERE p.published_at <= datetime('now')"
        };

        let order_clause = format!(" ORDER BY p.{} {}", self.sort_field.to_sql(), self.sort_order.to_sql());

        format!("{}{}{} LIMIT ? OFFSET ?",
            base,
            where_clause,
            order_clause
        )
    }
}

/// List posts with pagination and filtering
pub async fn list_posts(db: &D1Database, params: &ListPostsParams) -> Result<(Vec<PostListItem>, Pagination)> {
    let offset = (params.page - 1) * params.limit;
    let query = PostQuery::new(params);

    // Count total matching posts
    let mut count_stmt = db.prepare(query.count_query());
    if let Some(ref tags) = params.tags {
        let tags_json = serde_json::to_string(tags)
            .map_err(|_| Error::RustError("Failed to serialize tags".to_string()))?;
        count_stmt = count_stmt.bind(&[tags_json.into()])?;
    }
    let count_result = count_stmt.first::<CountResult>(None).await?;
    let total = count_result.map(|c| c.count).unwrap_or(0);

    // Query posts with tags aggregated as JSON array
    let mut stmt = db.prepare(query.select_query());
    if let Some(ref tags) = params.tags {
        let tags_json = serde_json::to_string(tags)
            .map_err(|_| Error::RustError("Failed to serialize tags".to_string()))?;
        stmt = stmt.bind(&[tags_json.into(), params.limit.into(), offset.into()])?;
    } else {
        stmt = stmt.bind(&[params.limit.into(), offset.into()])?;
    }

    let results = stmt.all().await?;
    let posts: Vec<PostListItem> = results.results()?;

    // Calculate pagination
    let has_next = (params.page * params.limit) < total;
    let pagination = Pagination {
        page: params.page,
        limit: params.limit,
        total,
        has_next,
    };

    Ok((posts, pagination))
}

/// SQL query for fetching a single post by slug
const GET_POST_BY_SLUG_QUERY: &str =
    "SELECT p.content_id, p.slug, p.title, p.summary, p.published_at, p.updated_at, p.external_url, p.created_at, \
     (SELECT json_group_array(t.name) FROM post_tags pt \
      INNER JOIN tags t ON pt.tag_id = t.id \
      WHERE pt.post_id = p.content_id) as tags \
     FROM posts p \
     WHERE p.slug = ? LIMIT 1";

/// Get a single post by slug
pub async fn get_post_by_slug(db: &D1Database, slug: &str) -> Result<Option<Post>> {
    let stmt = db.prepare(GET_POST_BY_SLUG_QUERY)
        .bind(&[slug.into()])?;

    let result = stmt.first::<Post>(None).await?;
    Ok(result)
}
