# Implementation Tasks

## Phase 1: Content Management (Revised: Shell Scripts Instead of Rust xtask)

### Task 1.1: Create Shell Script Infrastructure
- [x] Create `scripts/portfolio/` directory structure
- [x] Create `_common.sh` with shared utilities
- [x] Set up KV helper functions (put, get, list, delete)
- [x] Add mode validation helpers

**Files**: `scripts/portfolio/_common.sh`

### Task 1.2: Define JSON Schemas
- [x] Create `portfolio/schemas/` directory
- [x] Define `hero-content.json` schema
- [x] Define `about-content.json` schema
- [x] Define `project.json` schema
- [x] Define `experience.json` schema

**Files**: `portfolio/schemas/*.json`

### Task 1.3: Implement Hero Content Management
- [x] Create `hero.sh` script with get/set/list commands
- [x] Add JSON validation against schema
- [x] Support all 4 portfolio modes
- [x] Test against production KV

**Files**: `scripts/portfolio/hero.sh`

### Task 1.4: Implement About Content Management
- [x] Create `about.sh` script with get/set/list commands
- [x] Support multiple paragraphs per mode
- [x] Add JSON validation
- [x] Test against production KV

**Files**: `scripts/portfolio/about.sh`

### Task 1.5: Implement Project Management
- [x] Create `project.sh` script
- [x] Support get/set/list commands
- [x] Support featured list management per mode
- [x] Add JSON validation

**Files**: `scripts/portfolio/project.sh`

### Task 1.6: Implement Experience Management
- [x] Create `experience.sh` script
- [x] Support get/set/list commands
- [x] Support featured list management per mode
- [x] Add JSON validation

**Files**: `scripts/portfolio/experience.sh`

### Task 1.7: Document Shell Scripts
- [x] Create README.md with usage examples
- [x] Document all commands and options
- [x] Add JSON schema documentation

**Files**: `scripts/portfolio/README.md`

### Task 1.8: Existing xtask Portfolio Commands (Already Done)
- [x] `cargo xtask portfolio project` - Project management
- [x] `cargo xtask portfolio experience` - Experience management
- [x] `cargo xtask portfolio post` - Featured posts management
- [x] `cargo xtask portfolio contact` - Contact form viewer

**Note**: Shell scripts complement xtask for content that changes frequently (hero/about).

## Phase 2: Frontend Foundation (portfolio-frontend)

### Task 2.1: Initialize Leptos SSR Project
- [x] Create `portfolio/` directory
- [x] Initialize Leptos with Cloudflare Workers SSR
- [x] Configure `Cargo.toml` with leptos, worker, serde dependencies
- [x] Set up `wrangler.toml` with KV binding
- [x] Configure worker-build for SSR deployment

**Files**: `portfolio/Cargo.toml`, `portfolio/wrangler.toml`, `portfolio/src/lib.rs`

### Task 2.2: Define Multi-Mode Support
- [x] Create `Mode` enum (SoftwareEngineer, Fullstack, Rust, Student)
- [x] Implement URL path routing for modes
- [x] Add mode-specific KV key prefixes
- [x] Create HeroContent and AboutContent types for KV storage

**Files**: `portfolio/src/types.rs`

### Task 2.3: Implement Server Functions
- [x] Implement `get_featured_projects(mode)` server function
- [x] Implement `get_featured_experience(mode)` server function
- [x] Implement `get_featured_posts(mode)` server function with API calls
- [x] Implement `get_hero_content(mode)` for dynamic hero text
- [x] Implement `get_about_content(mode)` for dynamic about text
- [x] Add error handling and logging

**Files**: `portfolio/src/server_functions.rs`

### Task 2.4: Create Component Structure
- [x] Create `portfolio/src/components/` directory
- [x] Implement `Hero` component with KV-based content
- [x] Implement `AboutSection` component with KV-based paragraphs
- [x] Implement `ProjectCard` component
- [x] Implement `ExperienceItem` component
- [x] Implement `BlogPostCard` component
- [x] Implement Suspense wrappers for loading states

**Files**: `portfolio/src/components/*.rs`

## Phase 3: Frontend Sections

### Task 3.1: Implement Projects Section
- [x] Create `ProjectsSection` component
- [x] Call `get_featured_projects()` with mode parameter
- [x] Render grid of ProjectCard components
- [x] Handle empty state
- [x] Add "View All Projects →" link to GitHub
- [x] Support clickable cards with redirect_url
- [x] Add keyboard navigation (tabindex)

**Files**: `portfolio/src/components/projects.rs`

### Task 3.2: Implement Experience Section
- [x] Create `ExperienceSection` component
- [x] Call `get_featured_experience()` with mode parameter
- [x] Render list of ExperienceItem components
- [x] Style with company/role/period/description
- [x] Handle empty state
- [x] Add "View Full Resume →" link to resume.werdxz.info
- [x] Support clickable items with redirect_url
- [x] Add keyboard navigation

**Files**: `portfolio/src/components/experience.rs`

### Task 3.3: Implement Featured Writing Section
- [x] Create `WritingSection` component
- [x] Call `get_featured_posts()` with API integration
- [x] Render BlogPostCard components
- [x] Hide section if no posts (conditional rendering)
- [x] Handle API errors gracefully
- [x] Dynamically show/hide nav link based on post availability

**Files**: `portfolio/src/components/writing.rs`

### Task 3.4: Contact Form
- [ ] Not implemented (deferred - using email link instead)

### Task 3.5: Implement Hero and About Sections
- [x] Implement `Hero` component with dynamic KV content
- [x] Fetch subtitle and description per mode
- [x] Add navigation with IntersectionObserver for active section
- [x] Add social links (LinkedIn, GitHub, Blog)
- [x] Add sticky sidebar with scroll tracking
- [x] Implement `AboutSection` with dynamic KV paragraphs
- [x] Fetch paragraphs per mode from KV

**Files**: `portfolio/src/components/hero.rs`, `portfolio/src/components/about.rs`

## Phase 4: Styling and Polish

### Task 4.1: Global Styles
- [x] Import design tokens from `https://cloud.werdxz.info/shared/styles/variables.css`
- [x] Create `portfolio/style/main.css` with CSS Grid layout
- [x] Set up CSS variables for sidebar width, content width
- [x] Configure responsive breakpoint at 768px
- [x] Implement sticky sidebar with fixed left column

**Files**: `portfolio/style/main.css`

### Task 4.2: Card Design System
- [x] Create unified `.card` base class
- [x] Add left-border focus indicator (2px transparent → blue)
- [x] Implement hover states for clickable cards
- [x] Add gap-based spacing (remove min-height)
- [x] Support keyboard navigation with proper focus styles

**Files**: `portfolio/style/main.css`

### Task 4.3: Mobile Responsive Design
- [x] Test all sections on mobile (< 768px)
- [x] Convert grid layout to single column
- [x] Make sidebar non-sticky on mobile
- [x] Adjust spacing for mobile
- [x] Test touch interaction and scrolling

**Files**: `portfolio/style/main.css` (responsive section)

### Task 4.4: Footer
- [x] Add site footer with copyright "© 2025 werdxz"
- [x] Style footer with border and centered text

**Files**: `portfolio/src/app.rs`, `portfolio/style/main.css`

## Phase 5: Deployment

### Task 5.1: Production Build Configuration
- [x] Configure `cargo leptos build --release`
- [x] Set up worker-build for SSR compilation
- [x] Configure wrangler.toml with KV bindings
- [x] Test SSR rendering

**Files**: `portfolio/wrangler.toml`

### Task 5.2: KV Namespace Setup
- [x] Use shared KV namespace `ad9607c404424a8eb6949994a4383845`
- [x] Configure KV binding in wrangler.toml
- [x] Populate initial data for all 4 modes
- [x] Test shell scripts against production KV with `--remote`

**Commands**: Shell scripts in `scripts/portfolio/`

### Task 5.3: Deploy to Cloudflare Workers
- [x] Deploy with `npx wrangler deploy`
- [x] Verify deployment at https://portfolio.werdxz.workers.dev
- [x] Test all 4 mode routes (/, /fullstack, /rust, /student)
- [x] Verify KV data loads correctly
- [x] Test SSR and hydration

**Platform**: Cloudflare Workers

### Task 5.4: Domain Configuration
- [ ] Not yet configured (using workers.dev subdomain for now)
- [ ] Future: Add custom domain `portfolio.werdxz.info`

## Phase 6: Advanced Features Implemented

### Task 6.1: Multi-Mode Portfolio
- [x] Implement 4 distinct portfolio personas
- [x] Route-based mode detection (/, /fullstack, /rust, /student)
- [x] Mode-specific content fetching from KV
- [x] Mode-specific hero and about text
- [x] Mode-specific featured projects/experience/posts

**Files**: `portfolio/src/types.rs`, `portfolio/src/app.rs`

### Task 6.2: IntersectionObserver Navigation
- [x] Implement scroll-based active section tracking
- [x] Highlight active nav link based on viewport position
- [x] Use multiple threshold values for granular tracking
- [x] Calculate highest intersection ratio for best section
- [x] Configure root margin for optimal detection

**Files**: `portfolio/src/components/hero.rs`

### Task 6.3: Dynamic Content Management
- [x] Move hero/about text from hardcoded to KV
- [x] Support per-mode hero subtitle and description
- [x] Support per-mode about paragraphs
- [x] Enable content updates without redeployment

**Files**: `portfolio/src/server_functions.rs`, shell scripts

## Definition of Done

- [x] Portfolio site accessible at https://portfolio.werdxz.workers.dev
- [x] 4 portfolio modes working (/, /fullstack, /rust, /student)
- [x] Projects displaying with business-focused descriptions
- [x] Experience displaying with impact narratives
- [x] Hero and about content dynamically loaded from KV
- [x] Shell scripts for content management documented
- [x] Mobile responsive (single breakpoint at 768px)
- [x] SSR working with Cloudflare Workers
- [x] IntersectionObserver navigation tracking
- [x] Keyboard navigation support
- [ ] Contact form (deferred - using email link)
- [ ] Custom domain configuration (future)
- [ ] Performance optimization (future)

## Implementation Notes

**Shell Scripts vs Rust xtask**: We chose to implement hero/about content management as shell scripts instead of Rust xtask commands because:
1. Simpler to write and maintain
2. More straightforward for occasional content updates
3. Reduces compilation overhead
4. Direct wrangler CLI integration
5. Existing xtask commands still handle project/experience/post management

This hybrid approach keeps complex data management in Rust while allowing quick content updates via shell scripts.
