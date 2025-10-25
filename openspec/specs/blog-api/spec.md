# blog-api Specification

## Purpose
TBD - created by archiving change implement-api-backend. Update Purpose after archive.
## Requirements
### Requirement: List Blog Posts
The API MUST provide an endpoint to list blog posts with pagination, filtering, and sorting capabilities.

#### Scenario: List published posts with default pagination
**GIVEN** there are 25 published blog posts in the database
**WHEN** GET /posts is requested with no query parameters
**THEN** respond with 200 OK
**AND** return posts array with 10 items (default limit)
**AND** include pagination metadata: `{"page": 1, "limit": 10, "total": 25, "has_next": true}`
**AND** posts ordered by published_at descending

#### Scenario: Filter posts by tags
**GIVEN** posts tagged with "rust" and "web"
**WHEN** GET /posts?tags=rust is requested
**THEN** respond with 200 OK
**AND** return only posts containing "rust" tag (filtered via SQL JOIN with post_tags and tags tables)
**AND** include pagination metadata
**AND** pagination total reflects filtered count

#### Scenario: Filter posts by status
**GIVEN** posts in draft and published status
**WHEN** GET /posts?status=draft is requested
**THEN** respond with 200 OK
**AND** return only draft posts
**AND** default to published if status not specified

#### Scenario: Pagination with custom limit
**GIVEN** there are 50 published posts
**WHEN** GET /posts?page=2&limit=20 is requested
**THEN** respond with 200 OK
**AND** return posts 21-40
**AND** pagination shows page=2, limit=20, total=50

#### Scenario: Invalid pagination parameters
**GIVEN** a request with invalid pagination
**WHEN** GET /posts?limit=100 is requested (exceeds max of 50)
**THEN** respond with 400 Bad Request
**AND** error message indicates limit exceeds maximum

#### Scenario: Sort posts by title
**GIVEN** multiple published posts
**WHEN** GET /posts?sort=title&order=asc is requested
**THEN** respond with 200 OK
**AND** return posts sorted alphabetically by title ascending

### Requirement: Get Single Blog Post
The API MUST provide an endpoint to retrieve a single blog post by slug with full content.

#### Scenario: Retrieve post by valid slug
**GIVEN** a published post with slug "my-first-post" exists
**WHEN** GET /posts/my-first-post is requested
**THEN** respond with 200 OK
**AND** return post metadata (title, slug, summary, dates, tags)
**AND** return full markdown content from Durable Object
**AND** include read_time_minutes estimate

#### Scenario: Post not found
**GIVEN** no post with slug "non-existent" exists
**WHEN** GET /posts/non-existent is requested
**THEN** respond with 404 Not Found
**AND** error code "NOT_FOUND"
**AND** message "Post with slug 'non-existent' not found"

#### Scenario: Draft post access
**GIVEN** a draft post exists (not published)
**WHEN** GET /posts/{draft-slug} is requested
**THEN** respond with 404 Not Found (drafts not publicly accessible)

### Requirement: Post Metadata Schema
Blog post responses MUST include consistent metadata fields.

#### Scenario: Post list item structure
**GIVEN** any post in a list response
**WHEN** the response is formatted
**THEN** include required fields: slug, title, published_at
**AND** include optional fields: summary, tags, read_time_minutes
**AND** exclude full content from list responses

#### Scenario: Full post structure
**GIVEN** a single post detail response
**WHEN** the response is formatted
**THEN** include all fields from list response
**AND** include full markdown content
**AND** include updated_at timestamp if post was edited

### Requirement: D1 Schema for Posts
The D1 database MUST have a normalized schema with posts, tags, and post_tags tables.

#### Scenario: Posts table exists
**GIVEN** the API is deployed
**WHEN** querying the D1 database
**THEN** posts table exists with columns: content_id (PRIMARY KEY), slug (UNIQUE), title, summary, published_at, updated_at, external_url, created_at
**AND** slug column has UNIQUE constraint
**AND** index exists on published_at

#### Scenario: Tags table exists with normalized storage
**GIVEN** the API is deployed
**WHEN** querying the D1 database
**THEN** tags table exists with columns: id (PRIMARY KEY AUTOINCREMENT), name (UNIQUE)
**AND** each unique tag is stored once for reuse across posts
**AND** tag names are normalized to lowercase

#### Scenario: Post-Tags junction table exists
**GIVEN** the API is deployed
**WHEN** querying the D1 database
**THEN** post_tags junction table exists with columns: post_id (FK to posts.content_id), tag_id (FK to tags.id)
**AND** composite PRIMARY KEY on (post_id, tag_id)
**AND** FOREIGN KEY constraints with CASCADE DELETE
**AND** indexes exist on both post_id and tag_id for efficient lookups

### Requirement: Content Storage in R2
Blog post content MUST be stored in R2 bucket, referenced by content_id in D1.

#### Scenario: Store post content
**GIVEN** a new blog post is created via wrangler CLI
**WHEN** content is provided
**THEN** store markdown file in R2 bucket at `posts/{content_id}.md`
**AND** save content_id in D1 posts table
**AND** insert tags into tags table (if new) and create post_tags associations

#### Scenario: Retrieve post content
**GIVEN** a post exists with content_id reference
**WHEN** fetching the full post
**THEN** query D1 for metadata and tags via JOIN
**AND** retrieve content from R2 bucket using content_id
**AND** combine metadata + content in response

### Requirement: Read Time Calculation
Post responses MUST include estimated reading time in minutes.

#### Scenario: Calculate read time
**GIVEN** a blog post with markdown content
**WHEN** formatting the response
**THEN** calculate reading time based on word count (assume 200 words/minute)
**AND** include read_time_minutes field
**AND** round up to nearest minute (minimum 1 minute)

### Requirement: Tag Normalization
Post tags MUST be normalized to lowercase for consistent filtering.

#### Scenario: Tag storage normalization
**GIVEN** a post is created with tags ["Rust", "Web", "WASM"]
**WHEN** storing tags in the database
**THEN** normalize all tags to lowercase: ["rust", "web", "wasm"]

#### Scenario: Tag query normalization
**GIVEN** a request to filter by tags
**WHEN** GET /posts?tags=Rust,WASM is requested
**THEN** normalize query tags to lowercase
**AND** match against normalized database tags

