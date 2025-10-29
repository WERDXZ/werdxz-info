# Design Document: Portfolio Showcase

## Architecture Overview

Portfolio site is a Leptos SSR application on Cloudflare Pages. Data is stored in the same KV namespace as the API (different binding name), and featured blog posts are fetched from the existing API server.

## Key Design Decisions

### 1. KV Storage Pattern: Index-Based

**Structure**:
```
portfolio:featured_projects → ["werdxz-info", "rust-api", ...]
portfolio:project:{id} → {title, description, technologies, ...}
portfolio:featured_experience → ["meta-2024", "google-2023", ...]
portfolio:experience:{id} → {company, role, period, ...}
portfolio:featured_posts → ["slug1", "slug2", ...]
portfolio:contact:{timestamp} → {name, email, message, timestamp}
```

**Why index-based?**
- KV list operations cost 10x more than reads ($5.00 vs $0.50 per 100K)
- Can fetch items in parallel: read index once, then parallel fetch each item
- Easy to update individual items without rewriting entire arrays
- Index array controls ordering independently

**Cost comparison for 6 projects**:
- Index pattern: 7 reads = $0.0000035
- List operations: 1 list = $0.00005 (14x more expensive)

### 2. Data Sources: Hybrid KV + API

**Projects & Experience → KV**:
- Portfolio-specific curated content
- Different messaging for business vs developer audiences
- No HTTP latency (direct KV bindings)

**Blog Posts → API**:
- Reuse existing blog infrastructure (D1, R2, CLI)
- Same content, just filtered by featured flag
- Single source of truth for post metadata
- Managed via existing `cargo xtask post` commands

**Featured posts flow**:
1. Portfolio stores list of slugs in KV: `portfolio:featured_posts → ["slug1", "slug2"]`
2. Server function reads slugs from KV
3. Server function fetches each post from API: `GET /v1/posts/{slug}`
4. Returns combined array to frontend

### 3. KV Namespace: Shared with API

**Decision**: Use same KV namespace as API, with `portfolio:*` prefix

**Existing API uses**: `api:*` prefix for caching
**Portfolio uses**: `portfolio:*` prefix for data

**Why shared?**
- Saves KV namespace quota (limited per account)
- One binding to manage
- Clear ownership via prefix convention
- Already proven pattern (API uses prefixes for different cache types)

**Binding configuration**:
```toml
# API (api/wrangler.toml)
[[kv_namespaces]]
binding = "KV"
id = "abc123..."

# Portfolio (portfolio/wrangler.toml)
[[kv_namespaces]]
binding = "PORTFOLIO"
id = "abc123..."  # Same namespace ID, different binding name
```

### 4. Frontend: Single Page Application

**One page with sections**:
- Hero
- Featured Projects (3-6 cards)
- Experience (2-4 items)
- Featured Writing (0-3 posts, optional)
- About/Skills
- Contact Form

**Why single-page?**
- Simpler (no router, no navigation state)
- Better performance (one server render)
- Better UX for recruiters (scroll vs click-wait-load)
- Industry standard for portfolios
- All content in one SEO-friendly HTML document

### 5. Contact Form: Store in KV, Manual Review

**v1**: Contact submissions → KV → manual CLI review

**Why not email immediately?**
- No external service dependencies
- No secrets to manage
- Good enough for low volume (dozens/month)
- Demonstrates KV write operations
- Can add email in v2 if needed

**Storage pattern**:
```
portfolio:contact:{unix_timestamp} → {
  name: string,
  email: string,
  message: string,
  timestamp: number
}
```

Using timestamp as key makes entries naturally ordered and unique.

### 6. Server Functions for All Data

All data fetching uses Leptos server functions:
- `get_featured_projects()` → reads from KV
- `get_featured_experience()` → reads from KV
- `get_featured_posts()` → reads KV slugs, fetches from API
- `submit_contact_form()` → writes to KV

**Benefits**:
- Type-safe (shared Rust types)
- SSR support (no loading spinners)
- Works without JS (progressive enhancement)
- Centralized error handling

**Example parallel fetching**:
```rust
#[server(GetFeaturedProjects)]
pub async fn get_featured_projects() -> Result<Vec<Project>, ServerFnError> {
    let kv = env::kv("PORTFOLIO")?;

    // Get index
    let ids: Vec<String> = kv.get("portfolio:featured_projects").json().await?;

    // Fetch all in parallel
    let futures = ids.iter().map(|id| {
        kv.get(&format!("portfolio:project:{}", id)).json()
    });

    Ok(futures::future::try_join_all(futures).await?)
}
```

## Technology Choices

### Leptos SSR
- Rust ecosystem (matches API, CLI)
- Type-safe server functions
- Small bundle size (~100KB)
- First-class SSR support
- Official Cloudflare Workers template

### Cloudflare Pages
- Edge SSR (low latency)
- Native KV bindings (no HTTP overhead)
- Zero-config deployment (git push)
- Free tier (100K requests/day)

### Cloudflare KV
- Edge-available (low latency)
- Simple key-value API
- Cost-effective with index pattern
- Native Pages integration
- Eventually consistent is fine for curated content

## Data Schemas

### Project
```typescript
{
  title: string
  description: string              // Full narrative
  technologies: string[]
  image_url: string
  redirect_url?: string           // Optional card click target
  links: Array<{                  // Action buttons
    label: string
    url: string
  }>
}
```

### Experience
```typescript
{
  company: string
  role: string
  period: string                  // "Summer 2024" (text format)
  location?: string
  description: string             // Impact narrative
  technologies: string[]
  redirect_url?: string
}
```

### Blog Post (from API)
```typescript
{
  slug: string
  title: string
  summary: string
  published_at: string
  tags: string[]
  featured: boolean
}
```

### Contact Submission
```typescript
{
  name: string
  email: string
  message: string
  timestamp: number
}
```

## Performance

### SSR at Edge
- Server-render at edge location near user
- No loading spinners, content-ready HTML
- Target: TTFB < 200ms, LCP < 2s

### Parallel KV Reads
Use `futures::future::try_join_all` to fetch multiple items concurrently:
- 6 projects fetched in parallel: ~100ms total (not 600ms sequential)

### Lazy Image Loading
Below-fold images use `loading="lazy"` attribute

## Error Handling

### Server Functions
Return `Result<T, ServerFnError>` with proper error messages

### Frontend Fallbacks
```rust
<Suspense fallback=|| view! { <p>"Loading..."</p> }>
    <ErrorBoundary fallback=|_| view! {
        <p>"Failed to load projects"</p>
    }>
        <ProjectsSection />
    </ErrorBoundary>
</Suspense>
```

### Graceful Degradation
If API is down, hide Featured Writing section rather than show error

## Security

### Input Validation
- Server-side validation in server functions
- Max lengths for text fields
- Email format validation
- URL format validation

### Spam Prevention (v1)
- Cloudflare automatic rate limiting
- Manual review via CLI
- Simple input validation

### No Secrets Required
- KV access via Pages bindings (no tokens in code)
- API server is public
- No email credentials (v1)

## Testing

### Local Development
```bash
wrangler pages dev portfolio/dist --kv PORTFOLIO
```

### CLI Testing
```bash
# Add test data
cargo xtask portfolio project add test-1 --title "Test Project" ...

# Verify in KV
wrangler kv:key get "portfolio:featured_projects" --namespace-id=...

# View contact submissions
cargo xtask portfolio contact list
```

### Manual E2E
1. Submit contact form → verify success
2. Check CLI: `cargo xtask portfolio contact list`
3. Verify submission appears

## Deployment

### Build
```bash
cd portfolio
trunk build --release
wrangler pages deploy dist
```

### Environment
**Development** (`.dev.vars`):
```
API_BASE_URL=http://localhost:60232
```

**Production** (Cloudflare Dashboard):
```
API_BASE_URL=https://api.werdxz.info
KV binding: PORTFOLIO → [namespace-id]
```

### Domain
Configure `portfolio.werdxz.info` in Cloudflare Pages dashboard

## Future Enhancements

### v2
- Email notifications on contact form submit
- Dark mode (respect prefers-color-scheme)
- Downloadable resume PDF
- View analytics per project

### v3
- Admin web UI (manage content via browser)
- Rich text editor for descriptions
- Direct R2 image upload
- Real-time analytics dashboard
