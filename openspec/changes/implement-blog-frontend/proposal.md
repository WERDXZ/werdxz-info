# Proposal: Implement Blog Frontend with Qwik City

## Change ID
`implement-blog-frontend`

## Summary
Build a blog frontend using Qwik City to consume the existing blog API (`api.werdxz.info/v1/posts`). The blog will be deployed to `blog.werdxz.info` on Cloudflare Pages with static site generation for optimal performance.

## Why
The blog API backend is complete but there's no user-facing interface to read posts. Without a frontend, the content stored in R2 and indexed in D1 is inaccessible to readers. This change delivers the final piece needed for a functional blog, enabling content publishing and consumption.

## Motivation
The API backend for blog posts is complete (implemented in `implement-api-backend`), but there's no frontend to display the content. Users cannot read blog posts without a dedicated blog interface.

**Goals:**
- Provide a fast, SEO-friendly blog interface
- Leverage Qwik's resumability for minimal JavaScript
- Use static generation for posts where possible
- Integrate with existing design system (CSS variables from `cloud.werdxz.info`)
- Deploy to Cloudflare Pages for edge performance

**Non-goals:**
- Content management UI (use CLI for now)
- Comments or social features (future consideration)
- Search functionality (can add later with D1 FTS5)

## Scope
This change introduces:
1. **Blog Frontend Application** - New `blog/` directory with Qwik City app
2. **Blog Routing** - File-based routes for index, post detail, tag filtering
3. **Content Display** - Components for rendering markdown posts with syntax highlighting

**Out of scope:**
- RSS feed (separate future change)
- Blog search (separate future change)
- Admin UI for post management (CLI-first approach)

## Dependencies
- **Depends on:** `implement-api-backend` (completed) - Blog API must exist
- **Blocks:** Future blog enhancement features (search, RSS, analytics)

## Architecture Overview
See `design.md` for detailed architecture decisions.

**High-level approach:**
- Qwik City for frontend framework (resumability, performance)
- Static Site Generation (SSG) for published posts
- Incremental Static Regeneration (ISR) for new posts
- Markdown rendering with syntax highlighting (shiki or highlight.js)
- Cloudflare Pages deployment with edge caching

## Affected Capabilities
New specs:
- `blog-frontend` - Core blog application structure
- `blog-routing` - Routes and navigation
- `blog-content-display` - Markdown rendering and post display

Modified specs:
- None (this is a new independent application)

## Risk Assessment
**Low Risk** - This is an isolated frontend application that doesn't modify existing services.

**Risks:**
- Qwik is relatively new (mitigation: well-documented, active community)
- Static generation requires rebuild for new posts (mitigation: use ISR or deploy triggers)

**Rollback:**
- Easy rollback - just unpublish the `blog.werdxz.info` Cloudflare Pages deployment
- No impact on API or other services

## Success Criteria
- [ ] Blog index page loads in < 1s (LCP)
- [ ] Individual posts render with proper formatting and syntax highlighting
- [ ] Tag filtering works correctly
- [ ] Pagination works for > 10 posts
- [ ] SEO meta tags present (title, description, OG tags)
- [ ] Mobile-responsive design using design system
- [ ] Lighthouse score > 95 for performance
- [ ] Deployed to `blog.werdxz.info` on Cloudflare Pages

## Validation Plan
See `tasks.md` for detailed task breakdown.

**Testing approach:**
- Manual testing on localhost during development
- Visual regression testing for design system consistency
- Lighthouse audit for performance metrics
- Cross-browser testing (Chrome, Firefox, Safari)
- Mobile testing (responsive breakpoints)

## Open Questions
None - architecture is straightforward given existing patterns.
