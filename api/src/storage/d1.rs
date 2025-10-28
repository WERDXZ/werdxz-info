use worker::*;
use serde::Deserialize;
use crate::models::post::{Post, PostListItem, Pagination};
use crate::models::project::Project;
use crate::models::tag::TagWithCount;

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
    pub search: Option<String>,
    pub sort_by: SortField,
    pub order: SortOrder,
}

impl Default for ListPostsParams {
    fn default() -> Self {
        Self {
            page: 1,
            limit: 10,
            tags: None,
            search: None,
            sort_by: SortField::PublishedAt,
            order: SortOrder::Desc,
        }
    }
}

/// Query type for determining which prepared statement to use
#[derive(Debug, Clone, Copy)]
enum QueryType {
    NoFilters,
    TagsOnly,
    SearchOnly,
    TagsAndSearch,
}

impl QueryType {
    fn from_params(params: &ListPostsParams) -> Self {
        match (params.tags.is_some(), params.search.is_some()) {
            (false, false) => Self::NoFilters,
            (true, false) => Self::TagsOnly,
            (false, true) => Self::SearchOnly,
            (true, true) => Self::TagsAndSearch,
        }
    }

    /// Get the count query for this filter combination
    fn count_query(&self) -> &'static str {
        match self {
            Self::NoFilters => {
                "SELECT COUNT(*) as count FROM posts p \
                 WHERE p.published_at <= datetime('now')"
            }
            Self::TagsOnly => {
                "SELECT COUNT(DISTINCT p.content_id) as count FROM posts p \
                 INNER JOIN post_tags pt ON p.content_id = pt.post_id \
                 INNER JOIN tags t ON pt.tag_id = t.id \
                 WHERE p.published_at <= datetime('now') \
                 AND t.name IN (SELECT json_each.value FROM json_each(?))"
            }
            Self::SearchOnly => {
                "SELECT COUNT(*) as count FROM posts p \
                 WHERE p.published_at <= datetime('now') \
                 AND (p.title LIKE '%' || ? || '%' OR p.summary LIKE '%' || ? || '%')"
            }
            Self::TagsAndSearch => {
                "SELECT COUNT(DISTINCT p.content_id) as count FROM posts p \
                 INNER JOIN post_tags pt ON p.content_id = pt.post_id \
                 INNER JOIN tags t ON pt.tag_id = t.id \
                 WHERE p.published_at <= datetime('now') \
                 AND t.name IN (SELECT json_each.value FROM json_each(?)) \
                 AND (p.title LIKE '%' || ? || '%' OR p.summary LIKE '%' || ? || '%')"
            }
        }
    }

    /// Get the select query for this filter combination
    fn select_query(&self, sort_field: SortField, sort_order: SortOrder) -> String {
        let base = "SELECT p.slug, p.title, p.summary, p.published_at, p.external_url, \
                    (SELECT json_group_array(t.name) FROM post_tags pt \
                     INNER JOIN tags t ON pt.tag_id = t.id \
                     WHERE pt.post_id = p.content_id) as tags \
                    FROM posts p";

        let order = format!(" ORDER BY p.{} {}", sort_field.to_sql(), sort_order.to_sql());

        match self {
            Self::NoFilters => {
                format!("{} WHERE p.published_at <= datetime('now'){} LIMIT ? OFFSET ?", base, order)
            }
            Self::TagsOnly => {
                format!(
                    "{} INNER JOIN post_tags pt ON p.content_id = pt.post_id \
                     INNER JOIN tags t ON pt.tag_id = t.id \
                     WHERE p.published_at <= datetime('now') \
                     AND t.name IN (SELECT json_each.value FROM json_each(?))\
                     {} LIMIT ? OFFSET ?",
                    base, order
                )
            }
            Self::SearchOnly => {
                format!(
                    "{} WHERE p.published_at <= datetime('now') \
                     AND (p.title LIKE '%' || ? || '%' OR p.summary LIKE '%' || ? || '%')\
                     {} LIMIT ? OFFSET ?",
                    base, order
                )
            }
            Self::TagsAndSearch => {
                format!(
                    "{} INNER JOIN post_tags pt ON p.content_id = pt.post_id \
                     INNER JOIN tags t ON pt.tag_id = t.id \
                     WHERE p.published_at <= datetime('now') \
                     AND t.name IN (SELECT json_each.value FROM json_each(?)) \
                     AND (p.title LIKE '%' || ? || '%' OR p.summary LIKE '%' || ? || '%')\
                     {} LIMIT ? OFFSET ?",
                    base, order
                )
            }
        }
    }
}

/// List posts with pagination and filtering
pub async fn list_posts(db: &D1Database, params: &ListPostsParams) -> Result<(Vec<PostListItem>, Pagination)> {
    let offset = (params.page - 1) * params.limit;
    let query_type = QueryType::from_params(params);

    // Execute count query based on filter combination
    let total = match query_type {
        QueryType::NoFilters => {
            let stmt = db.prepare(query_type.count_query());
            let result = stmt.first::<CountResult>(None).await?;
            result.map(|c| c.count).unwrap_or(0)
        }
        QueryType::TagsOnly => {
            let tags_json = serde_json::to_string(params.tags.as_ref().unwrap())
                .map_err(|_| Error::RustError("Failed to serialize tags".to_string()))?;
            let stmt = db.prepare(query_type.count_query())
                .bind(&[tags_json.into()])?;
            let result = stmt.first::<CountResult>(None).await?;
            result.map(|c| c.count).unwrap_or(0)
        }
        QueryType::SearchOnly => {
            let search = params.search.as_ref().unwrap();
            let stmt = db.prepare(query_type.count_query())
                .bind(&[search.clone().into(), search.clone().into()])?;
            let result = stmt.first::<CountResult>(None).await?;
            result.map(|c| c.count).unwrap_or(0)
        }
        QueryType::TagsAndSearch => {
            let tags_json = serde_json::to_string(params.tags.as_ref().unwrap())
                .map_err(|_| Error::RustError("Failed to serialize tags".to_string()))?;
            let search = params.search.as_ref().unwrap();
            let stmt = db.prepare(query_type.count_query())
                .bind(&[tags_json.into(), search.clone().into(), search.clone().into()])?;
            let result = stmt.first::<CountResult>(None).await?;
            result.map(|c| c.count).unwrap_or(0)
        }
    };

    // Execute select query based on filter combination
    let posts: Vec<PostListItem> = match query_type {
        QueryType::NoFilters => {
            let stmt = db.prepare(query_type.select_query(params.sort_by, params.order))
                .bind(&[params.limit.into(), offset.into()])?;
            let results = stmt.all().await?;
            results.results()?
        }
        QueryType::TagsOnly => {
            let tags_json = serde_json::to_string(params.tags.as_ref().unwrap())
                .map_err(|_| Error::RustError("Failed to serialize tags".to_string()))?;
            let stmt = db.prepare(query_type.select_query(params.sort_by, params.order))
                .bind(&[tags_json.into(), params.limit.into(), offset.into()])?;
            let results = stmt.all().await?;
            results.results()?
        }
        QueryType::SearchOnly => {
            let search = params.search.as_ref().unwrap();
            let stmt = db.prepare(query_type.select_query(params.sort_by, params.order))
                .bind(&[search.clone().into(), search.clone().into(), params.limit.into(), offset.into()])?;
            let results = stmt.all().await?;
            results.results()?
        }
        QueryType::TagsAndSearch => {
            let tags_json = serde_json::to_string(params.tags.as_ref().unwrap())
                .map_err(|_| Error::RustError("Failed to serialize tags".to_string()))?;
            let search = params.search.as_ref().unwrap();
            let stmt = db.prepare(query_type.select_query(params.sort_by, params.order))
                .bind(&[tags_json.into(), search.clone().into(), search.clone().into(), params.limit.into(), offset.into()])?;
            let results = stmt.all().await?;
            results.results()?
        }
    };

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

/// SQL query for fetching all tags with usage counts
const GET_ALL_TAGS_QUERY: &str =
    "SELECT t.name as tag, COUNT(pt.post_id) as count \
     FROM tags t \
     INNER JOIN post_tags pt ON t.id = pt.tag_id \
     INNER JOIN posts p ON pt.post_id = p.content_id \
     WHERE p.published_at <= datetime('now') \
     GROUP BY t.id, t.name \
     ORDER BY t.name ASC";

/// Get all tags with their usage counts (only includes tags from published posts)
pub async fn get_all_tags(db: &D1Database) -> Result<Vec<TagWithCount>> {
    let stmt = db.prepare(GET_ALL_TAGS_QUERY);
    let results = stmt.all().await?;
    let tags: Vec<TagWithCount> = results.results()?;
    Ok(tags)
}

// ============================================================================
// Project Queries
// ============================================================================

/// SQL query for fetching all projects
const GET_ALL_PROJECTS_QUERY: &str =
    "SELECT p.id, p.slug, p.name, p.description, p.stage, p.open_to_contributors, \
     p.readme_url, p.created_at, p.updated_at, \
     (SELECT json_group_array(t.name) FROM project_tags pt \
      INNER JOIN tags t ON pt.tag_id = t.id \
      WHERE pt.project_id = p.id) as tags, \
     (SELECT json_group_array(json_object('label', pu.label, 'url', pu.url)) \
      FROM project_urls pu WHERE pu.project_id = p.id) as urls \
     FROM projects p \
     ORDER BY p.updated_at DESC";

/// Get all projects
pub async fn get_all_projects(db: &D1Database) -> Result<Vec<Project>> {
    let stmt = db.prepare(GET_ALL_PROJECTS_QUERY);
    let results = stmt.all().await?;
    let projects: Vec<Project> = results.results()?;
    Ok(projects)
}

/// SQL query for fetching a single project by slug
const GET_PROJECT_BY_SLUG_QUERY: &str =
    "SELECT p.id, p.slug, p.name, p.description, p.stage, p.open_to_contributors, \
     p.readme_url, p.created_at, p.updated_at, \
     (SELECT json_group_array(t.name) FROM project_tags pt \
      INNER JOIN tags t ON pt.tag_id = t.id \
      WHERE pt.project_id = p.id) as tags, \
     (SELECT json_group_array(json_object('label', pu.label, 'url', pu.url)) \
      FROM project_urls pu WHERE pu.project_id = p.id) as urls \
     FROM projects p \
     WHERE p.slug = ? LIMIT 1";

/// Get a single project by slug
pub async fn get_project_by_slug(db: &D1Database, slug: &str) -> Result<Option<Project>> {
    let stmt = db.prepare(GET_PROJECT_BY_SLUG_QUERY)
        .bind(&[slug.into()])?;

    let result = stmt.first::<Project>(None).await?;
    Ok(result)
}
