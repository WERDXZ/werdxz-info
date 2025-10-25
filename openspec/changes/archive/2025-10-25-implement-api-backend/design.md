# Design: API Backend Architecture

## Overview

The API backend uses workers-rs (Rust on Cloudflare Workers) to provide a unified REST API for the werdxz.info ecosystem. It follows a hybrid storage approach: metadata in D1 for queryability, content in Durable Objects for performance.

## Architecture

### Technology Stack

**Runtime:** Cloudflare Workers via workers-rs
**Language:** Rust
**Framework:** worker crate with Router
**Storage:**
- D1 (SQLite) for metadata, relationships, queries
- R2 (Object Storage) for blog content storage (markdown files)
- KV (Workers KV) for resume data (small, frequently-read JSON)
**API Style:** REST with OpenAPI 3.0 specification
**Deployment:** api.werdxz.info subdomain

### Storage Strategy

**D1 Database (`werdxz-api-db`):**
```sql
-- Blog posts metadata
CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    summary TEXT,
    published_at TEXT,
    updated_at TEXT,
    status TEXT NOT NULL, -- draft, published, archived
    tags TEXT, -- JSON array
    content_id TEXT NOT NULL, -- Durable Object ID
    created_at TEXT NOT NULL
);

CREATE INDEX idx_posts_slug ON posts(slug);
CREATE INDEX idx_posts_status ON posts(status);
CREATE INDEX idx_posts_published_at ON posts(published_at);
```

**R2 Bucket (`werdxz-content`):**
- Stores markdown content for blog posts
- Key format: `posts/{content_id}.md`
- ID referenced from D1 metadata
- Simple, cost-effective object storage
- No egress fees

**KV Namespace (`werdxz-resume`):**
- Stores resume.json data
- Key: `resume` (contains full resume JSON)
- Automatic edge caching for fast global reads
- Optimized for small, frequently-accessed data
- Eventually consistent (fine for resume updates)

### API Versioning

**Strategy:** URL path versioning with `/v1` prefix

All API endpoints are versioned under `/v1/` to allow for future breaking changes:
- ✅ Clear versioning visible in URLs
- ✅ Clients pin to specific API version
- ✅ Can maintain multiple versions simultaneously (v1, v2, etc.)
- ✅ No ambiguity about which version is being used

**Unversioned endpoints** (meta/infrastructure):
- `/` - API root with service info
- `/openapi.json` - OpenAPI specification
- `/docs` - Interactive Swagger UI documentation

**Versioned endpoints** (v1):
- `/v1/health` - Health check
- `/v1/posts` - Blog posts list
- `/v1/posts/:slug` - Single post
- `/v1/resume` - Resume data

**Future evolution:**
- Breaking changes → bump to `/v2/...`
- Deprecation notices in v1 responses
- Maintain v1 for 6-12 months after v2 launch

### Routing Structure

```rust
// Main router
Router::new()
    // Meta endpoints (unversioned)
    .get("/", handle_root)
    .get("/openapi.json", handle_openapi_spec)

    // v1 API endpoints
    .get("/v1/health", handle_health)
    .get("/v1/posts", handle_list_posts)
    .get("/v1/posts/:slug", handle_get_post)
    .get("/v1/resume", handle_get_resume)
```

### API Endpoints

#### Blog Posts

**GET /v1/posts**
Query parameters:
- `page` (int, default: 1)
- `limit` (int, default: 10, max: 50)
- `status` (draft|published|archived, default: published)
- `tags` (comma-separated)
- `sort` (published_at|title, default: published_at)
- `order` (asc|desc, default: desc)

Response:
```json
{
  "posts": [
    {
      "slug": "my-first-post",
      "title": "My First Post",
      "summary": "Introduction to...",
      "published_at": "2025-01-15T10:00:00Z",
      "tags": ["rust", "web"],
      "read_time_minutes": 5
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 10,
    "total": 25,
    "has_next": true
  }
}
```

**GET /v1/posts/:slug**
Response:
```json
{
  "slug": "my-first-post",
  "title": "My First Post",
  "summary": "Introduction to...",
  "content": "# My First Post\n\n...",
  "published_at": "2025-01-15T10:00:00Z",
  "updated_at": "2025-01-16T14:30:00Z",
  "tags": ["rust", "web"],
  "read_time_minutes": 5
}
```

#### Resume

**GET /v1/resume**
Query parameters:
- `sections` (comma-separated: experience,education,projects,extracurricular)
- `tags` (filter items by technology tags)
- `format` (full|minimal, default: full)
- `limit` (limit items per section)

Fetches from KV, applies filters server-side, returns JSON.
Benefits: Edge caching, fast reads, simple updates via KV put.

### CORS Configuration

```rust
// Allow all werdxz.info subdomains
headers.set("Access-Control-Allow-Origin", origin)?;
headers.set("Access-Control-Allow-Methods", "GET, OPTIONS")?;
headers.set("Access-Control-Allow-Headers", "Content-Type")?;
headers.set("Access-Control-Max-Age", "86400")?;

// Whitelist: www, blog, portfolio, resume subdomains
```

### Error Handling

Standard error responses:
```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Post with slug 'invalid' not found",
    "request_id": "req_abc123"
  }
}
```

HTTP status codes:
- 200: Success
- 400: Bad request (invalid query params)
- 404: Resource not found
- 429: Rate limited
- 500: Internal server error

### Admin CLI: cargo xtask

Workspace-aware admin tooling via cargo xtask pattern with nested subcommands:

**Project Management:**
```bash
cargo xtask project list                      # Discover all projects (auto-detects package.json/Cargo.toml)
cargo xtask project deploy api --production   # Deploy any discovered project
cargo xtask project deploy www                # Works with both Rust and Node.js projects
```

**Blog Post Management:**
```bash
cargo xtask post publish post.md \
  --slug my-post \
  --title "My Post" \
  --summary "Summary text" \
  --tags rust,web

cargo xtask post list [--remote]             # List posts from D1
cargo xtask post delete my-post [--remote]   # Delete post metadata + note R2 cleanup
```

**Resume Management:**
```bash
cargo xtask resume update [--remote]          # Fetch from cloud.werdxz.info → upload to KV
```

**Database Operations:**
```bash
cargo xtask migrate [--remote]                # Apply D1 migrations
```

**Key Features:**
- **Workspace-Aware:** Finds workspace root automatically, works from any subdirectory
- **Project Discovery:** Scans workspace for projects with package.json or Cargo.toml
- **Generic Deployment:** Validates project exists before deploying to Cloudflare Workers
- **Nested Subcommands:** Clean organization (project, post, resume, migrate)
- **Direct wrangler Integration:** All operations use wrangler CLI for backend access

Implementation wraps wrangler commands:
```bash
# Post publishing internally does:
wrangler r2 object put cloud posts/{uuid}.md --file post.md
wrangler d1 execute werdxz-db --command "INSERT INTO posts..."

# Resume update internally does:
curl -s https://cloud.werdxz.info/resume/public/resume.json > /tmp/resume.json
wrangler kv key put resume --path /tmp/resume.json --namespace-id=... [--remote]

# Deployment internally does:
cd <project-dir> && npx wrangler deploy [--production]
```

### OpenAPI Specification

Auto-generated OpenAPI 3.0 spec served at `/openapi.json`:
- Endpoint documentation
- Request/response schemas
- Query parameter validation
- Example requests/responses

### Interactive API Documentation

**Swagger UI** at `/docs`:
- Custom-styled Swagger UI using shared CSS variables from `cloud.werdxz.info/shared/styles/variables.css`
- Interactive API exploration and testing
- Embedded using `include_str!` macro from `static/docs.html`
- Loads OpenAPI spec from `/openapi.json`
- Responsive design with dark mode support
- Consistent branding with design system (colors, spacing, typography, shadows)

## Trade-offs

### D1 + R2 vs Single Database

**Chosen:** Hybrid approach (D1 + R2)
**Alternative:** Store everything in D1

**Rationale:**
- D1 has row size limits (1MB), blog posts may exceed this
- R2 is cost-effective for blob storage (no egress fees!)
- D1 excellent for queries, indexes, relationships
- Separation of concerns: metadata vs content
- R2 consistent with existing architecture (resume.json in cloud bucket)

**Trade-off:**
- More complex (two storage systems)
- Two round-trips for full post retrieval
- BUT: Better performance, scalability, cost efficiency

### Read-Only Public API vs Full CRUD

**Chosen:** Read-only with admin CLI
**Alternative:** HTTP-based authentication for writes

**Rationale:**
- Simpler security model (no auth needed for v1)
- Admin operations via wrangler more secure
- Reduces attack surface
- Faster initial development

**Trade-off:**
- No web-based CMS initially
- Requires wrangler/CLI access for content management
- BUT: Can add HTTP auth later without breaking changes

### REST vs GraphQL

**Chosen:** REST with OpenAPI
**Alternative:** GraphQL API

**Rationale:**
- Simpler implementation
- Better caching (HTTP-level)
- OpenAPI spec provides discoverability
- workers-rs has better REST support

**Trade-off:**
- Less flexible queries
- Potential over-fetching
- BUT: Simpler, faster, easier to maintain

## Future Considerations

### Content Versioning
R2 can store version history using key prefixes:
```
posts/{content_id}.md → latest content
posts/{content_id}/v1.md → version 1
posts/{content_id}/v2.md → version 2
```

### Caching Strategy
- Cache-Control headers for static content
- Cloudflare CDN for global distribution
- Stale-while-revalidate for blog posts

### Analytics Integration
Future endpoint: `POST /analytics/view/:slug`
Track views, popular posts, etc.

### Search
Future: Full-text search via D1 FTS5 or external search service

## Implementation Notes

### Project Structure
```
# Rust workspace at root
Cargo.toml                  # Workspace manifest (edition 2024)
.cargo/
└── config.toml            # cargo xtask alias

api/
├── src/
│   ├── lib.rs              # Main worker entry point
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── posts.rs        # Blog posts endpoints
│   │   ├── resume.rs       # Resume filtering
│   │   └── meta.rs         # Health, OpenAPI, docs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── post.rs         # Post struct, validation
│   │   └── resume.rs       # Resume filtering logic
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── d1.rs           # D1 queries
│   │   ├── r2.rs           # R2 object storage
│   │   └── blog.rs         # Blog post queries
│   ├── openapi/
│   │   └── spec.rs         # OpenAPI spec generation
│   └── errors.rs           # Error types
├── static/
│   └── docs.html           # Custom Swagger UI page
├── migrations/
│   └── 0001_init.sql       # D1 schema
├── tests/
│   └── api.hurl           # Hurl test suite (17 tests)
├── wrangler.toml
└── Cargo.toml

xtask/                     # Admin CLI (cargo xtask pattern)
├── src/
│   └── main.rs            # Nested subcommands (project, post, resume, migrate)
└── Cargo.toml
```

### Dependencies

**API (api/Cargo.toml):**
```toml
[dependencies]
worker = { version = "0.6", features = ["d1"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde", "wasmbind"] }
utoipa = { version = "5", features = ["chrono"] }
```

**xtask (xtask/Cargo.toml):**
```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
anyhow = "1.0"
uuid = { version = "1.11", features = ["v4"] }
```
