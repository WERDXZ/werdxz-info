# Proposal: Implement API Backend

## Why

The werdxz.info ecosystem needs a centralized API backend to serve dynamic content across multiple frontend applications. Currently, static content like resume data is stored in cloud buckets, but there's no system for managing blog posts, filtering resume data, or providing structured API access to content.

This API will:
- Enable the planned blog section with structured post storage and retrieval
- Provide resume data filtering capabilities via query parameters
- Establish a foundation for future features (portfolio, analytics, etc.)
- Maintain the project's multi-stack philosophy by using workers-rs (Rust on Cloudflare Workers)

## What Changes

### New Capability: API Server Infrastructure
Create a workers-rs API at `api.werdxz.info` with:
- REST endpoints following OpenAPI 3.0 specification
- D1 database for metadata storage
- R2 bucket for file content storage
- CORS configuration for frontend access
- Read-only public access (admin via wrangler CLI wrapper)

### New Capability: Blog Posts API
Endpoints for managing blog posts:
- `GET /posts` - List posts with pagination, filtering, sorting
- `GET /posts/:slug` - Get single post with full content
- Metadata in D1 (title, slug, date, tags, summary, status)
- Full content stored in R2 bucket

### New Capability: Resume Filtering API
Extend resume.json access with filtering:
- `GET /resume` - Full resume data (current behavior)
- `GET /resume?sections=experience,education` - Filter sections
- `GET /resume?tags=rust,backend` - Filter by technology tags
- `GET /resume?format=minimal` - Different output formats

### Infrastructure
- Cloudflare Workers deployment at `api.werdxz.info`
- D1 database: `werdxz-api-db`
- R2 bucket: `werdxz-content` (blog posts)
- KV namespace: `werdxz-resume` (resume data)
- wrangler.toml configuration
- OpenAPI spec generation

### Admin Tooling
- Local wrangler wrapper CLI for content management
- Commands: `create-post`, `update-post`, `delete-post`, `publish-post`, `update-resume`
- Direct D1/R2/KV access via wrangler bindings (no HTTP auth needed initially)

## Dependencies

- Requires Cloudflare Workers account (already configured)
- Requires D1 database creation
- Requires R2 bucket creation
- Requires KV namespace creation
- Frontend apps will need CORS origins configured

## Out of Scope

- Authentication/authorization for public API (read-only for v1)
- Portfolio projects API (future iteration)
- Analytics/view counting (future iteration)
- Comment system (future iteration)
- Full CMS web interface (CLI-first approach)
