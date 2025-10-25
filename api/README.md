# werdxz API

REST API for werdxz.info built with [workers-rs](https://github.com/cloudflare/workers-rs) on Cloudflare Workers.

## Quick Start

```bash
# Development
cd api
wrangler dev

# Run integration tests
hurl --test --variable base_url=http://localhost:8787 tests/api.hurl

# Build
cargo build --release --target wasm32-unknown-unknown
```

## Architecture

- **D1 Database**: Blog post metadata (title, slug, tags, published dates)
- **R2 Bucket**: Markdown content storage for blog posts
- **KV Namespace**: Resume JSON data

### Endpoints

- `GET /` - API info and endpoint list
- `GET /v1/health` - Health check with dependency status
- `GET /v1/posts` - List blog posts (paginated, filterable by tags)
- `GET /v1/posts/:slug` - Get full post with markdown content
- `GET /v1/resume` - Get resume data (filterable by sections/tags)
- `GET /openapi.json` - OpenAPI 3.0 specification
- `GET /docs` - Interactive API documentation

## Configuration

### Environment Variables

- `ALLOWED_ORIGINS` (compile-time) - Additional CORS origins for development
  ```bash
  ALLOWED_ORIGINS="http://localhost:3000" cargo build
  ```

### Bindings (wrangler.toml)

- `DB` - D1 database binding
- `CONTENT_BUCKET` - R2 bucket for blog content
- `RESUME_KV` - KV namespace for resume data

## Security

- **Input validation**: All user inputs validated (slugs, tags, pagination)
- **CORS**: Restricted to `*.werdxz.info` domains
- **Rate limiting**: Handled by Cloudflare infrastructure
- **SQL injection prevention**: Parameterized queries with enum-based sort fields

## Development

```bash
# Check code
cargo check

# Run tests
cargo test

# Lint
cargo clippy

# Format
cargo fmt
```

## Deployment

```bash
# Deploy to production
wrangler deploy

# View logs
wrangler tail
```

## Database Migrations

```bash
# Run migrations locally
wrangler d1 execute DB --local --file migrations/0001_init.sql

# Run migrations in production
wrangler d1 execute DB --file migrations/0001_init.sql
```

## License

MIT
