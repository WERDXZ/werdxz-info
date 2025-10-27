# Tasks: Implement Projects Feature

## Phase 1: API Backend & Database

### 1.1 Create D1 Database Schema
- [ ] Create `projects` table with core metadata fields
- [ ] Create normalized `tags` table (shared with blog if possible)
- [ ] Create `project_tags` junction table
- [ ] Create `project_urls` table for flexible link management
- [ ] Add indexes for performance (project_id lookups)
- [ ] Write migration SQL file
- [ ] Test schema locally with wrangler

**Validation**: Run `npx wrangler d1 execute` to verify tables created

### 1.2 Implement Projects API Endpoints
- [ ] Add `/v1/projects` GET endpoint (list projects with pagination)
- [ ] Add `/v1/projects/{slug}` GET endpoint (single project detail)
- [ ] Add D1 query functions in `storage/d1.rs`
- [ ] Add Rust types for Project, ProjectUrl, Tag
- [ ] Add OpenAPI documentation for new endpoints
- [ ] Handle errors (404 for missing projects, 500 for DB errors)

**Validation**: Test endpoints locally with `xh GET http://localhost:8787/v1/projects`

**Dependencies**: 1.1 must be complete

### 1.3 Add Projects to OpenAPI Spec
- [ ] Update `api/openapi.yaml` with projects endpoints
- [ ] Add schema definitions for Project response types
- [ ] Update API documentation route to serve updated spec

**Validation**: Visit `/openapi.json` and verify projects endpoints listed

**Dependencies**: 1.2 must be complete

## Phase 2: CLI Management (xtask)

### 2.1 Add Project Management Commands
- [ ] Add `cargo xtask project add` command
- [ ] Add `cargo xtask project list` command
- [ ] Add `cargo xtask project update` command
- [ ] Add `cargo xtask project delete` command
- [ ] Support `--remote` flag for production D1 database
- [ ] Add input validation (slug format, required fields, URL validation)

**Validation**: Run `cargo xtask project add --name "Test" --slug "test"` locally and remotely

**Dependencies**: 1.1 must be complete

### 2.2 Add URL and Tag Management
- [ ] Support `--urls` flag with JSON array or multiple `--url label=https://...` args
- [ ] Support `--tags` flag with comma-separated values
- [ ] Normalize tags to lowercase
- [ ] Insert/update URLs in `project_urls` table
- [ ] Insert tags in normalized `tags` and `project_tags` tables

**Validation**: Add project with multiple URLs and tags, verify in D1

**Dependencies**: 2.1 must be complete

## Phase 3: Frontend (Deno Fresh)

### 3.1 Scaffold Deno Fresh Application
- [ ] Run `deno run -A -r https://fresh.deno.dev` to create `projects/` directory
- [ ] Remove demo routes and components
- [ ] Configure deployment for Cloudflare Pages
- [ ] Set up import map for shared utilities
- [ ] Add `.gitignore` for Deno build artifacts

**Validation**: Run `deno task start` and see Fresh welcome page

### 3.2 Implement Projects Listing Page
- [ ] Create `/routes/index.tsx` for projects list
- [ ] Fetch data from `/v1/projects` API endpoint
- [ ] Display project cards in grid layout (2 cols desktop, 1 col mobile)
- [ ] Show project name, description, stage badge, contributor badge
- [ ] Show tags as clickable filters (optional MVP feature)
- [ ] Add links to GitHub, demo, docs from project_urls
- [ ] Style with pure CSS using shared variables from cloud bucket

**Validation**: Visit `http://localhost:8000` and see project cards

**Dependencies**: 1.2 must be complete

### 3.3 Implement Project Detail Page
- [ ] Create `/routes/[slug].tsx` for project details
- [ ] Fetch project from `/v1/projects/{slug}` endpoint
- [ ] Fetch external README from `readme_url` field
- [ ] Render markdown README using `marked` library (same as blog)
- [ ] Display all project metadata (stage, tags, URLs, contributor status)
- [ ] Style markdown content with blog-like CSS
- [ ] Handle 404 for missing projects

**Validation**: Visit `/test-project` and see rendered README

**Dependencies**: 3.2 must be complete

### 3.4 Add Shared Styling
- [ ] Import shared CSS variables from `https://cloud.werdxz.info/shared/styles/variables.css`
- [ ] Create consistent header/footer matching blog and resume
- [ ] Add responsive breakpoints for mobile/tablet/desktop
- [ ] Style badges for project stage and contributor status
- [ ] Ensure accessibility (ARIA labels, keyboard navigation)

**Validation**: Test on mobile/desktop, verify design consistency with blog

**Dependencies**: 3.2, 3.3 must be complete

## Phase 4: Deployment & Integration

### 4.1 Deploy API Changes
- [ ] Run D1 migrations on production database
- [ ] Deploy updated API to `api.werdxz.info`
- [ ] Verify `/v1/projects` endpoints work in production
- [ ] Test with `xh GET https://api.werdxz.info/v1/projects`

**Validation**: Production API returns projects successfully

**Dependencies**: 1.1, 1.2, 1.3 must be complete

### 4.2 Deploy Fresh Frontend
- [ ] Configure Cloudflare Pages project for `projects/` directory
- [ ] Set up `projects.werdxz.info` subdomain
- [ ] Deploy Fresh app to Cloudflare Pages
- [ ] Test production site loads correctly
- [ ] Verify API integration works in production

**Validation**: Visit `https://projects.werdxz.info` and see projects

**Dependencies**: 3.2, 3.3, 3.4, 4.1 must be complete

### 4.3 Update Main Site Links
- [ ] Add "Projects" link to `www/public/index.html`
- [ ] Update sitemap.xml with `projects.werdxz.info`
- [ ] Test navigation flow from home → projects

**Validation**: Click "Projects" link from homepage

**Dependencies**: 4.2 must be complete

## Phase 5: Documentation & Testing

### 5.1 Add Documentation
- [ ] Update README with projects feature description
- [ ] Document xtask project commands usage
- [ ] Add example project JSON structure
- [ ] Document API endpoints in main README

**Validation**: New developer can add a project following docs

### 5.2 Add Initial Projects
- [ ] Add 2-3 real projects via `cargo xtask project add`
- [ ] Verify READMEs render correctly
- [ ] Test all links (GitHub, demos, docs)
- [ ] Verify tags work for filtering (if implemented)

**Validation**: Projects page shows real content, not placeholder

**Dependencies**: All previous phases complete

## Notes
- Phases can be partially parallelized (e.g., 2.1 can start after 1.1, doesn't need 1.2)
- Frontend work (Phase 3) can start in parallel with xtask (Phase 2) after Phase 1 complete
- Each task should be tested before moving to dependent tasks
