# Proposal: Build Portfolio Showcase

## Summary

Create a business-focused portfolio website at `portfolio.werdxz.info` to showcase 3-6 featured projects, 2-4 work experiences, and 0-3 featured blog posts to recruiters and hiring managers. Built with Leptos SSR on Cloudflare Pages, this single-page application will present curated content emphasizing business value, outcomes, and impact rather than technical details.

## Why

Current `projects.werdxz.info` serves developers with technical details, but there's no dedicated space for business audiences (recruiters, hiring managers) who need different messaging. This proposal creates a separate portfolio site optimized for converting professional opportunities, with curated content emphasizing business value and measurable impact.

## Motivation

### Problem
- Current `projects.werdxz.info` is developer-focused (technical details, GitHub links, contribution guides)
- No dedicated space to showcase work for **business audiences** (recruiters, hiring managers, potential employers)
- Need a curated, professional presentation of best work with business-focused messaging
- No way to highlight professional writing/thought leadership alongside projects

### Goals
1. **Primary**: Create a professional portfolio site that converts recruiters/hiring managers into interview opportunities
2. **Secondary**: Demonstrate Leptos SSR + server functions capability in production
3. **Tertiary**: Establish separate data layer for curated portfolio content (distinct from comprehensive projects list)
4. **Bonus**: Cross-promote blog content by featuring best technical articles

### Non-Goals (Out of Scope)
- Homepage (werdxz.info) redesign with HTMX - separate proposal
- Multi-page routing with project detail pages - keeping it single-page for simplicity and performance
- Real-time email sending - v1 stores contact form submissions in KV for manual review
- Search/filtering UI - not needed for small curated lists (3-6 items per section)
- Analytics dashboard - use Cloudflare Analytics instead

## Proposed Solution

### High-Level Architecture

```
┌─────────────────────────────────────────────────────┐
│ portfolio.werdxz.info (Leptos SSR)                  │
│                                                     │
│ Server Functions (run on Cloudflare Pages):        │
│  ├─ get_featured_projects() → KV                   │
│  ├─ get_featured_experience() → KV                 │
│  ├─ get_featured_posts() → api.werdxz.info/v1/posts│
│  └─ submit_contact_form() → KV                     │
│                                                     │
│ Sections (single page):                            │
│  ├─ Hero (name, title, CTA)                        │
│  ├─ Featured Projects (grid of 3-6 cards)          │
│  ├─ Experience (2-4 timeline items)                │
│  ├─ Featured Writing (0-3 blog posts) - optional   │
│  ├─ About/Skills                                   │
│  └─ Contact Form                                   │
└─────────────────────────────────────────────────────┘
           │                            │
           │ KV Binding                 │ HTTP
           ▼                            ▼
    ┌─────────────────┐      ┌────────────────────┐
    │ Cloudflare KV   │      │ api.werdxz.info    │
    │ (shared w/ API) │      │ /v1/blogs?featured │
    │                 │      └────────────────────┘
    │ Keys:           │
    │ portfolio:      │
    │   featured_*    │
    │   project:*     │
    │   experience:*  │
    │   contact:*     │
    └─────────────────┘
```

### Data Architecture

**KV Storage (portfolio:* prefix in shared namespace)**:
```
portfolio:featured_projects      → ["werdxz-info", "rust-api", ...]
portfolio:project:{id}           → { title, description, technologies, image_url, redirect_url?, links[] }

portfolio:featured_experience    → ["meta-2024", "google-2023", ...]
portfolio:experience:{id}        → { company, role, period, location?, description, technologies, redirect_url? }

portfolio:contact:{timestamp}    → { name, email, message, timestamp }
```

**API Server** (existing):
```
GET /v1/posts?featured=true&limit=3
→ Returns featured blog posts (managed via existing blog CLI)
```

### Key Design Decisions

#### 1. Single Page vs. Multi-Route
- **Chosen**: Single page with anchor-based section scrolling
- **Rationale**:
  - Simpler implementation and maintenance
  - Better initial load performance (one server render)
  - Easier navigation for recruiters (scroll vs. click-wait-load)
  - Follows Brittany Chiang inspiration site pattern

#### 2. KV Prefix vs. Separate Namespace
- **Chosen**: Shared KV namespace with `portfolio:*` prefix
- **Rationale**:
  - Cost optimization: avoids expensive list operations by using index keys
  - Future-proof for homepage featured content (can share namespace)
  - Simpler infrastructure (one KV namespace to manage)
  - Clear data ownership via prefix convention

#### 3. Data Sources: KV vs. API Server
- **Chosen**: Hybrid approach
  - **Projects & Experience**: KV (portfolio-specific curated content)
  - **Blog Posts**: API server (reuse existing blog infrastructure)
- **Rationale**:
  - Projects/experience have different copy for business vs. developer audiences
  - Blog posts are same content, just filtered by "featured" flag
  - No HTTP overhead for projects/experience (direct KV bindings)
  - Blog already has robust management via CLI and D1 storage

#### 4. Contact Form Handling
- **Chosen**: Store in KV, manual review via CLI (v1)
- **Rationale**:
  - Simpler implementation (no external email service dependencies)
  - Good enough for low-volume contact forms
  - Can add email notifications in v2 if needed
  - Demonstrates KV write operations

#### 5. Featured Posts Integration
- **Chosen**: Fetch from existing blog API with featured flag filter
- **Rationale**:
  - Reuses existing blog infrastructure (D1, R2, CLI)
  - No data duplication (single source of truth)
  - Featured flag managed via existing `cargo xtask blog` commands
  - Server function hides HTTP call complexity

## Affected Capabilities

### New Capabilities
1. **portfolio-frontend** - Leptos SSR single-page portfolio site
2. **portfolio-data-storage** - KV-based storage for featured projects, experience, and contact submissions
3. **portfolio-cli** - CLI commands for managing portfolio content (projects, experience)

### Modified Capabilities
- **blog-api** - Add `featured` flag support to existing blog posts (minor enhancement)

### Removed Capabilities
- None

## Success Criteria

### Must Have (v1)
- [ ] Portfolio site accessible at `portfolio.werdxz.info`
- [ ] Displays 3-6 featured projects with business-focused descriptions
- [ ] Displays 2-4 work experiences with impact narratives
- [ ] Displays 0-3 featured blog posts (fetched from API)
- [ ] Responsive design (mobile, tablet, desktop) using global design tokens
- [ ] Contact form stores submissions in KV
- [ ] CLI commands to manage projects (`add`, `list`, `remove`)
- [ ] CLI commands to manage experience (`add`, `list`, `remove`)
- [ ] CLI command to view contact submissions
- [ ] Server-rendered on first load (SEO-friendly, no loading spinners)
- [ ] All sections use Leptos server functions for data fetching

### Nice to Have (Future v2)
- Email notifications when contact form submitted
- View counter per project (analytics)
- Downloadable resume PDF link
- Dark mode support (respecting prefers-color-scheme)
- Animated scroll transitions between sections

### Out of Scope
- Real-time analytics dashboard
- Admin web UI (CLI-first approach)
- Multi-language support
- Project detail pages (all content on one page)

## Data Schemas

### Project
```typescript
{
  title: string                      // "Personal Website Monorepo"
  description: string                // Full narrative paragraph
  technologies: string[]             // ["Rust", "TypeScript", "Cloudflare"]
  image_url: string                  // "https://cloud.werdxz.info/portfolio/..."
  redirect_url?: string              // Optional click target for card
  links: Array<{                     // Action buttons
    label: string                    // "Website", "GitHub", "API"
    url: string
  }>
}
```

### Experience
```typescript
{
  company: string                    // "Meta"
  role: string                       // "Software Engineering Intern"
  period: string                     // "Summer 2024" (text format)
  location?: string                  // "Menlo Park, CA"
  description: string                // Impact narrative paragraph
  technologies: string[]             // ["Python", "React", "GraphQL"]
  redirect_url?: string              // Optional company/project link
}
```

### Blog Post (from API)
```typescript
{
  slug: string                       // "building-rust-api-cloudflare"
  title: string                      // "Building a Rust API..."
  summary: string                    // Brief description
  published_at: string               // ISO date
  tags: string[]                     // ["rust", "cloudflare"]
  featured: boolean                  // true for portfolio display
}
```

### Contact Submission
```typescript
{
  name: string
  email: string
  message: string
  timestamp: number                  // Unix timestamp
}
```

## Implementation Phases

### Phase 1: Core Frontend Structure (Days 1-2)
- Scaffold Leptos app with Cloudflare Pages configuration
- Single-page layout with sections (Hero, Projects, Experience, Writing, About, Contact)
- Static content rendering (Hero, About/Skills sections)
- Responsive CSS using global design tokens from cloud bucket
- Basic component structure (no server functions yet)

**Deliverable**: Static portfolio page with layout and styling

### Phase 2: Projects Section (Day 3)
- Define Project data schema in Rust types
- Implement `get_featured_projects()` server function with KV binding
- Create `ProjectCard` component with image, description, tech tags, links
- Grid layout for projects section
- CLI: `cargo xtask portfolio add-project`, `list-projects`, `remove-project`

**Deliverable**: Projects section with server-rendered content from KV

### Phase 3: Experience Section (Day 4)
- Define Experience data schema in Rust types
- Implement `get_featured_experience()` server function with KV binding
- Create `ExperienceItem` component (timeline-style or card-based)
- CLI: `cargo xtask portfolio add-experience`, `list-experience`, `remove-experience`

**Deliverable**: Experience section with server-rendered content from KV

### Phase 4: Featured Writing Section (Day 5)
- Add `featured` boolean field to blog posts schema in API
- Update `GET /v1/posts` to support `?featured=true` query param
- Implement `get_featured_posts()` server function calling API
- Create `BlogPostCard` component
- Optional section rendering (hide if no featured posts)
- CLI: Use existing `cargo xtask post update {slug} --featured` command

**Deliverable**: Featured writing section pulling from blog API

### Phase 5: Contact Form (Day 6)
- Create `ContactForm` component with client-side validation
- Implement `submit_contact_form()` server function writing to KV
- Success/error state handling
- CLI: `cargo xtask portfolio contacts` to view submissions

**Deliverable**: Working contact form with KV storage

### Phase 6: Deployment & Polish (Day 7)
- Cloudflare Pages deployment configuration
- KV namespace binding setup (production)
- Domain configuration for `portfolio.werdxz.info`
- Cross-site navigation links (to/from projects, blog, homepage)
- Final responsive testing (mobile, tablet, desktop)
- Performance optimization (image loading, Suspense boundaries)

**Deliverable**: Live production deployment at portfolio.werdxz.info

## CLI Commands Reference

```bash
# Projects
cargo xtask portfolio add-project <id> \
  --title "Project Title" \
  --description "Full narrative..." \
  --technologies rust,typescript,cloudflare \
  --image "https://cloud.werdxz.info/..." \
  --redirect "https://projects.werdxz.info/project" \
  --link "Website:https://example.com" \
  --link "GitHub:https://github.com/..."

cargo xtask portfolio list-projects
cargo xtask portfolio remove-project <id>

# Experience
cargo xtask portfolio add-experience <id> \
  --company "Company Name" \
  --role "Job Title" \
  --period "Summer 2024" \
  --location "City, State" \
  --description "Impact narrative..." \
  --technologies python,react,graphql \
  --redirect "https://company.com"

cargo xtask portfolio list-experience
cargo xtask portfolio remove-experience <id>

# Blog Posts (use existing blog CLI)
cargo xtask post update <slug> --featured  # Mark as featured
cargo xtask post update <slug> --no-featured  # Unmark

# Contacts
cargo xtask portfolio contacts  # List all contact form submissions
```

## Risks & Mitigation

### Risk: Leptos SSR complexity on Cloudflare Pages
- **Impact**: Medium - Potential build/deployment issues
- **Likelihood**: Low - Leptos has official Cloudflare Workers template
- **Mitigation**: Start with official template, incremental feature adds, thorough local testing with `wrangler dev`

### Risk: KV storage costs
- **Impact**: Low - Minimal cost with current traffic
- **Likelihood**: Low - Index-based pattern avoids expensive list operations
- **Mitigation**: Monitor usage via Cloudflare dashboard, consider caching if traffic spikes

### Risk: Maintaining two "projects" presentations
- **Impact**: Low - Content divergence over time
- **Likelihood**: Medium - Manual curation required
- **Mitigation**: Clear audience separation documentation, quarterly review process, CLI makes updates easy

### Risk: API dependency for blog posts
- **Impact**: Medium - Portfolio page shows error if API is down
- **Likelihood**: Low - API has 99.9% uptime
- **Mitigation**: Graceful error handling (hide section if fetch fails), consider KV caching layer in v2

## Open Questions

1. ~~Should we use shared KV or separate namespace?~~ → **Resolved**: Shared with `portfolio:*` prefix
2. ~~Email sending in v1?~~ → **Resolved**: v2 feature, store in KV for now
3. ~~Multi-page or single-page?~~ → **Resolved**: Single page with sections
4. ~~Should we include featured blog posts?~~ → **Resolved**: Yes, fetch from API
5. Should we implement view counters/analytics in v1? → **Propose**: Defer to v2, use Cloudflare Analytics
6. Should contact form have CAPTCHA/spam protection? → **Propose**: Defer to v2, manual review sufficient initially

## Timeline Estimate

- Phase 1 (Frontend scaffold): 2 days
- Phase 2 (Projects): 1 day
- Phase 3 (Experience): 1 day
- Phase 4 (Featured writing): 1 day
- Phase 5 (Contact form): 1 day
- Phase 6 (Deployment): 1 day

**Total**: 7 days

## Related Work

- **Depends on**: Existing blog API (`/v1/posts` endpoint)
- **Future**: Homepage redesign with HTMX (separate proposal)
- **Future**: Email notifications for contact form (v2 enhancement)
- **Future**: Analytics integration (v2 enhancement)
