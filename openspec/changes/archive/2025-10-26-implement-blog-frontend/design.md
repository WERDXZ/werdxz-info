# Design: Blog Frontend Architecture

## Technology Choice: Qwik City

### Why Qwik City?

**Resumability over Hydration:**
- Traditional frameworks (React, Vue, Next.js) ship a full app bundle that re-executes on the client ("hydration")
- Qwik uses "resumability" - the client continues where SSR left off with zero/minimal JS
- For a blog (mostly static content), this means near-instant interactivity

**Performance Benefits:**
- O(1) loading time regardless of app complexity
- Code splitting at component-level (not route-level)
- Lazy-load only what the user interacts with
- Perfect for content-heavy blogs with syntax highlighting, images, etc.

**Edge-First Architecture:**
- Built for Cloudflare Pages / Workers
- SSG + ISR support out of the box
- Integrates well with existing Cloudflare infrastructure

**Alternatives Considered:**

| Framework | Pros | Cons | Decision |
|-----------|------|------|----------|
| **Astro** | Content-focused, excellent DX, partial hydration | Less edge-optimized, different mental model | ❌ Good but Qwik fits better |
| **Next.js** | Mature ecosystem, rich features | Heavy client bundle, React overhead | ❌ Too heavy for blog |
| **SolidStart** | Similar performance to Qwik | Smaller ecosystem, less edge support | ❌ Qwik more mature for edge |
| **Qwik City** | Resumability, edge-native, minimal JS | Newer ecosystem, learning curve | ✅ **Selected** |

## Architecture Decisions

### 1. Static Site Generation (SSG) Strategy

**Approach:**
- **Build-time SSG** for all published posts
- **Incremental Static Regeneration (ISR)** for new posts (rebuild on-demand)
- **Edge caching** via Cloudflare Pages CDN

**Rationale:**
- Blog posts are immutable once published (content rarely changes)
- SSG provides best performance and SEO
- ISR avoids full rebuild for every new post

**Implementation:**
```typescript
// In Qwik City route
export const onStaticGenerate: StaticGenerateHandler = async () => {
  const response = await fetch('https://api.werdxz.info/v1/posts?limit=1000');
  const { posts } = await response.json();

  return {
    params: posts.map(post => ({ slug: post.slug })),
  };
};
```

### 2. Routing Structure

**File-based routing in `blog/src/routes/`:**

```
blog/src/routes/
├── index.tsx              # Blog index (list all posts)
├── posts/
│   └── [slug]/
│       └── index.tsx      # Individual post page
├── tags/
│   └── [tag]/
│       └── index.tsx      # Posts filtered by tag
└── layout.tsx             # Shared layout with nav/footer
```

**Route behavior:**
- `/` - Blog index with pagination (10 posts per page)
- `/posts/{slug}` - Individual post with full content
- `/tags/{tag}` - Posts filtered by specific tag

**Data fetching:**
- Use Qwik's `routeLoader$()` for SSG/SSR data fetching
- Fetch from `api.werdxz.info/v1/posts` endpoints
- Cache responses at edge (Cloudflare CDN)

### 3. Content Rendering

**Markdown Rendering:**
- Use `markdown-it` or `remark` for parsing
- Syntax highlighting with `shiki` (VS Code's highlighter - smaller bundle than Prism)
- Sanitize HTML output (prevent XSS)

**Component Structure:**
```typescript
<Post>
  <PostHeader title={} date={} tags={} />
  <PostContent markdown={} />  # Render markdown to HTML
  <PostFooter readTime={} />
</Post>
```

**Code block handling:**
- Syntax highlighting with Shiki (supports 100+ languages)
- Line numbers optional
- Copy button for code blocks

### 4. Design System Integration

**CSS Variables:**
- Import design tokens from `https://cloud.werdxz.info/shared/styles/variables.css`
- Use same variables as homepage and resume for consistency

**Component styling:**
- Use CSS Modules or Qwik's scoped styles
- Minimal custom CSS (leverage design tokens)
- Mobile-first responsive design

**Typography:**
- Use `--font-family-sans` for UI elements
- Use `--font-family-mono` for code blocks
- Respect `--font-size-*` and `--spacing-*` variables

### 5. Deployment Strategy

**Cloudflare Pages:**
- Deploy to `blog.werdxz.info`
- Automatic deployments on git push (main branch)
- Preview deployments for PRs

**Build configuration:**
```toml
# wrangler.toml for Cloudflare Pages
name = "werdxz-blog"
compatibility_date = "2025-10-24"

[build]
command = "npm run build"
directory = "dist"
```

**Environment:**
- `VITE_API_URL=https://api.werdxz.info/v1` (production)
- `VITE_API_URL=http://localhost:60232/v1` (development)

### 6. Performance Optimizations

**Image Loading:**
- Use Cloudflare Images for post images (if added later)
- Lazy-load images below fold
- WebP with JPEG fallback

**Fonts:**
- Load fonts from design system (already in CSS variables)
- Use `font-display: swap` to prevent FOIT

**Bundle Size:**
- Qwik's automatic code splitting
- Tree-shaking unused markdown-it plugins
- Minify CSS and JS in production

**Caching Strategy:**
```
Static assets: Cache-Control: public, max-age=31536000, immutable
HTML pages: Cache-Control: public, max-age=3600, s-maxage=86400
API requests: Cache at edge, revalidate every 5 minutes
```

### 7. SEO Optimization

**Meta Tags:**
```typescript
export const head: DocumentHead = {
  title: post.title,
  meta: [
    { name: 'description', content: post.summary },
    { property: 'og:title', content: post.title },
    { property: 'og:description', content: post.summary },
    { property: 'og:type', content: 'article' },
    { property: 'article:published_time', content: post.published_at },
    { property: 'article:tag', content: post.tags.join(',') },
  ],
  links: [
    { rel: 'canonical', href: `https://blog.werdxz.info/posts/${post.slug}` },
  ],
};
```

**Structured Data:**
- JSON-LD for Article schema
- Include author, datePublished, headline, image

**Sitemap:**
- Generate `sitemap.xml` at build time
- Include all published posts
- Submit to Google Search Console

### 8. Development Workflow

**Local Development:**
```bash
cd blog/
npm install
npm run dev  # Starts dev server on http://localhost:5173
```

**Testing:**
- Use local API server (`http://localhost:60232`) or production API
- Environment variable switches between dev/prod API

**Deployment:**
```bash
npm run build              # Build for production
wrangler pages deploy dist # Deploy to Cloudflare Pages
```

## Trade-offs

### Qwik Learning Curve
**Trade-off:** Qwik is newer than Next.js/Astro, smaller ecosystem
**Mitigation:** Well-documented, active Discord community, worth the performance gains

### Static Generation Rebuild
**Trade-off:** New posts require full rebuild (not dynamic)
**Mitigation:** Use ISR or webhook from CLI to trigger builds

### Markdown Processing
**Trade-off:** Heavy markdown parsing can slow builds
**Mitigation:** Use faster parsers (remark), cache parsed HTML if needed

## Open Questions
None - architecture is well-defined given requirements.
