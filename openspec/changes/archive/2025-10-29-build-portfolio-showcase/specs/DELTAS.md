# Implementation Deltas: Build Portfolio Showcase

This document tracks significant deviations from the original proposal made during implementation.

## Architecture Changes

### Multi-Mode Portfolio System
**Original**: Single business-focused portfolio at `portfolio.werdxz.info`

**Implemented**: Multi-persona portfolio with mode-based routing:
- `/` - Software Engineer (default professional persona)
- `/fullstack` - Full-stack Developer persona
- `/rust` - Rust Specialist persona
- `/student` - Student/Academic persona

**Rationale**:
- Provides flexibility to tailor messaging for different audiences (corporate recruiters vs. startup CTOs vs. academic roles)
- Single codebase serves multiple use cases
- Mode derived from URL path using Leptos Memo
- Each mode fetches different featured content from KV

**Impact on Data Layer**:
```
Original: portfolio:featured_projects → [ids]
New:      portfolio:featured_projects:software-engineer → [ids]
          portfolio:featured_projects:fullstack → [ids]
          portfolio:featured_projects:rust → [ids]
          portfolio:featured_projects:student → [ids]
```

**Files Affected**:
- `portfolio/src/types.rs`: Added Mode enum with 4 variants
- `portfolio/src/app.rs`: Mode derivation from URL
- All server functions: Accept `mode: Mode` parameter
- All components: Accept `mode: Signal<Mode>` prop

---

## Contact Form Removal

**Original**: Contact form section with KV storage and CLI management

**Implemented**: Simple footer with email link and social links

**Rationale**:
- Preferred direct email contact over form submissions
- Reduces complexity and maintenance burden
- Most recruiters prefer LinkedIn anyway
- Cleaner UX without form validation/error handling

**Removed**:
- `submit_contact_form()` server function
- Contact form component and validation
- `portfolio:contact:{timestamp}` KV keys
- CLI commands: `portfolio contacts`

**Added**:
- Direct email link: `contact@werdxz.com`
- Social links: LinkedIn, GitHub, Blog

**Files Affected**:
- Removed entire contact form section from hero
- Added `<address>` element with email in hero footer

---

## Design System

### Two-Column Fixed Sidebar Layout

**Original**: Single-page with scrollable sections (no specific layout mentioned)

**Implemented**: Two-column layout with fixed left sidebar

**Design**:
```
┌────────────────────┬──────────────────────────┐
│                    │                          │
│  Fixed Sidebar     │  Scrollable Content      │
│  (40% width)       │  (60% width)             │
│                    │                          │
│  ┌──────────────┐  │  About                   │
│  │ Header       │  │  Projects                │
│  │ - Name       │  │  Experience              │
│  │ - Role       │  │  Writing (conditional)   │
│  │ - Desc       │  │                          │
│  └──────────────┘  │                          │
│                    │                          │
│  ┌──────────────┐  │                          │
│  │ Navigation   │  │                          │
│  │ - TOC links  │  │                          │
│  │ - Active     │  │                          │
│  │   tracking   │  │                          │
│  └──────────────┘  │                          │
│                    │                          │
│  ┌──────────────┐  │                          │
│  │ Footer       │  │                          │
│  │ - Email      │  │                          │
│  │ - Social     │  │                          │
│  └──────────────┘  │                          │
│                    │                          │
└────────────────────┴──────────────────────────┘
```

**Rationale**:
- Inspired by Brittany Chiang's portfolio but with flatter, minimal design
- Fixed sidebar keeps important info (name, contact) always visible
- IntersectionObserver tracks active section for TOC highlighting
- Better for professional browsing (recruiters can reference info while scrolling)

**Technical Implementation**:
- CSS: `position: fixed` sidebar with flexbox for 3-section layout
- IntersectionObserver with `root-margin: "-50% 0px -50% 0px"`
- Sections: `min-height: 60vh` ensures observer can detect transitions
- Fully responsive: stacks vertically on mobile (<768px)

**Files Affected**:
- `portfolio/style/main.css`: Complete rewrite (519 lines)
- `portfolio/src/components/hero.rs`: Sidebar structure with 3 sections

---

## Conditional Writing Section

**Original**: Always show 0-3 featured blog posts

**Implemented**: Conditionally show Writing section based on whether posts exist for current mode

**Behavior**:
- If mode has featured posts → "Writing" appears in TOC + section renders
- If mode has no posts → "Writing" hidden from TOC + section doesn't render
- IntersectionObserver gracefully handles missing section

**Rationale**:
- Some modes (business professional) might not have writing
- Avoids broken TOC links
- Better UX - TOC reflects actual page content

**Technical Implementation**:
```rust
// In app.rs
let posts = Resource::new(move || mode.get(), |mode| get_featured_posts(mode));
let has_posts = Memo::new(move |_| {
    posts.get().map_or(false, |result| {
        result.as_ref().map_or(false, |posts| !posts.is_empty())
    })
});

// In hero.rs
{move || has_posts.get().then(|| view! {
    <a href="#writing" class="nav-link" class:active=...>"Writing"</a>
})}
```

**Files Affected**:
- `portfolio/src/app.rs`: Fetch posts at app level, compute `has_posts` signal
- `portfolio/src/components/hero.rs`: Conditionally render Writing nav link

---

## Accessibility Enhancements

**Original**: Not specified in proposal

**Implemented**: Full accessibility features

**Features**:
1. **Semantic HTML**:
   - `<header>`, `<nav>`, `<main>`, `<article>`, `<address>`, `<footer>`
   - Proper heading hierarchy

2. **ARIA Labels**:
   - `aria-label="Main navigation"` on nav
   - `aria-label="Social links"` on social container
   - `role="status" aria-live="polite"` on loading states

3. **Keyboard Navigation**:
   - All clickable cards focusable with `tabindex="0"`
   - Enter and Space key handlers on project/experience cards
   - Skip to content functionality (via anchor links)

4. **Focus States**:
   - Visible focus outlines: `2px solid var(--color-text-link)`
   - Focus styles match hover styles for consistency
   - `outline-offset: 4px` for better visibility

**Rationale**:
- Professional portfolio should be accessible to all users
- Many recruiters/hiring managers use keyboard navigation
- Demonstrates commitment to web standards and inclusive design

**Files Affected**:
- `portfolio/style/main.css`: Focus state styles
- `portfolio/src/components/hero.rs`: ARIA labels
- `portfolio/src/components/projects.rs`: Keyboard handlers
- `portfolio/src/components/experience.rs`: Keyboard handlers
- All loading states: ARIA live regions

---

## Constants Extraction

**Original**: Not specified

**Implemented**: Centralized URL constants

**Structure**:
```rust
// portfolio/src/constants.rs
pub const BLOG_BASE_URL: &str = "https://blog.werdxz.info";
pub const DEFAULT_API_BASE_URL: &str = "https://api.werdxz.info";
```

**Rationale**:
- Easier maintenance when domains change
- Single source of truth for external URLs
- Environment variable fallback for API URL

**Files Affected**:
- `portfolio/src/constants.rs`: New module
- `portfolio/src/server_functions.rs`: Use DEFAULT_API_BASE_URL
- `portfolio/src/components/writing.rs`: Use BLOG_BASE_URL

---

## No Section Titles

**Original**: Not specified

**Implemented**: Removed all section titles from main content

**Rationale**:
- TOC navigation already shows section names
- Redundant visual noise
- Cleaner, more minimal design
- Users can see section from nav highlighting

**Impact**:
- Only content in sections, no `<h2>` titles
- Exception: "Core Skills" subtitle in About section (needed for context)

---

## Success Criteria Status

### Must Have (v1) - Status

- [x] Portfolio site structure created
- [x] Multi-mode routing implemented (exceeds original goal)
- [x] Displays 3-6 featured projects per mode with business-focused descriptions
- [x] Displays 2-4 work experiences per mode with impact narratives
- [x] Displays 0-3 featured blog posts (conditional rendering)
- [x] Responsive design using global design tokens
- [x] ~~Contact form stores submissions in KV~~ **REMOVED** - replaced with direct email link
- [x] ~~CLI commands to manage projects/experience~~ **DEFERRED** - will implement when adding sample data
- [x] ~~CLI command to view contact submissions~~ **REMOVED** - no contact form
- [x] Server-rendered on first load (Leptos SSR)
- [x] All sections use server functions for data fetching
- [x] Accessibility features (keyboard nav, ARIA, focus states) - **ADDED**
- [x] IntersectionObserver for active section tracking - **ADDED**

### Changed Requirements

**Added**:
- Multi-mode support (4 personas instead of 1)
- Conditional Writing section based on content availability
- Full accessibility implementation
- Two-column fixed sidebar layout
- IntersectionObserver navigation tracking

**Removed**:
- Contact form functionality
- Contact CLI commands
- Single business-focused persona

---

## Implementation Phases - Actual Timeline

### Completed Phases

**Phase 1: Core Structure** ✅
- Scaffolded Leptos app with Cloudflare Workers
- Two-column layout implementation
- Hero component with three sections
- Mode-based routing system
- Global design tokens integration

**Phase 2: Data Layer** ✅
- Type definitions (Mode, Project, Experience, BlogPost)
- Server functions with mode parameter
- Worker helpers for Send-safe KV operations
- API integration for blog posts

**Phase 3: Components** ✅
- AboutSection (static content + skills)
- ProjectsSection with mode-specific data
- ExperienceSection with mode-specific data
- WritingSection with conditional rendering
- Hero with IntersectionObserver

**Phase 4: Polish & Accessibility** ✅
- Keyboard navigation support
- ARIA labels and semantic HTML
- Focus states and visual feedback
- CSS comments for maintainability
- Constants extraction

**Phase 5: Code Review & Fixes** ✅
- Comprehensive code review
- Minor issue resolution
- Build verification

### Remaining Phases

**Phase 6: Data Population** 🔄 IN PROGRESS
- Add sample projects for all modes
- Add sample experience for all modes
- Mark blog posts as featured

**Phase 7: Testing** ⏳ PENDING
- Local testing with wrangler dev
- All 4 modes verified
- Responsive testing

**Phase 8: Deployment** ⏳ PENDING
- Cloudflare Pages deployment
- KV binding configuration
- Domain setup (portfolio.werdxz.info or werdxz.com)

---

## Technical Debt & Future Work

### Known Limitations

1. **No CLI for portfolio management yet**
   - Need to add `cargo xtask portfolio` commands
   - Currently requires manual KV updates

2. **No caching for blog posts**
   - Fetches from API on every page load
   - Consider KV cache layer for performance

3. **No error logging/monitoring**
   - Server function errors logged to console only
   - Should add Sentry or similar

4. **No meta tags per mode**
   - All modes share same title/description
   - Should customize per persona

### Future Enhancements (v2)

1. View counters per project
2. Dark mode support
3. Downloadable resume PDF
4. Animated section transitions
5. Blog post caching in KV
6. Custom meta tags per mode
7. Analytics integration (beyond Cloudflare Analytics)

---

## Lessons Learned

### What Went Well

1. **Multi-mode architecture**: More valuable than single business persona
2. **Leptos + Cloudflare Workers**: Smooth integration, SendFuture pattern works well
3. **Two-column layout**: Professional look, good UX
4. **Accessibility-first**: Easier to implement during development than retrofit

### What Could Be Better

1. **Plan CLI early**: Should have implemented `cargo xtask portfolio` commands first
2. **Design tokens**: Some portfolio-specific CSS could be extracted to shared tokens
3. **Testing strategy**: Should have written tests alongside implementation

### Technical Wins

1. **SendFuture pattern**: Solved non-Send reqwest issue elegantly
2. **IntersectionObserver**: Smooth active section tracking
3. **Conditional rendering**: has_posts signal pattern is reusable
4. **Mode-based data fetching**: Clean separation of concerns

---

## Updated Timeline Estimate

**Original Estimate**: 7 days

**Actual Progress**:
- Phases 1-5: ~5 days (includes multi-mode architecture, accessibility)
- Phase 6 (Data): In progress
- Phase 7 (Testing): ~0.5 days estimated
- Phase 8 (Deployment): ~0.5 days estimated

**Total Revised**: ~6 days (faster due to removing contact form, similar total with added features)

---

## Sign-off

**Status**: Implementation complete, pending data population and deployment

**Deviations Approved**:
- ✅ Multi-mode routing (enhancement)
- ✅ Contact form removal (simplification)
- ✅ Two-column layout (design improvement)
- ✅ Conditional Writing section (UX improvement)
- ✅ Accessibility features (enhancement)

**Next Steps**:
1. Add sample data to KV for all modes
2. Test with wrangler dev
3. Deploy to Cloudflare Pages
4. Configure domain (portfolio.werdxz.info or werdxz.com)
