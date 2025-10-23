# Proposal: implement-resume-page

## Summary
Implement a resume page at `resume.werdxz.info` subdomain using client-side rendering with custom elements. The page will display professional resume content while being blocked from search engine indexing.

## Motivation
- Provide a shareable digital resume for recruiters and employers
- Experiment with custom elements and client-side rendering approach
- Keep resume content separate from main site with subdomain isolation
- Maintain control over search engine visibility (noindex)

## Goals
- Create resume page with custom elements and client-side data fetching
- Deploy to Cloudflare Pages/Workers at resume.werdxz.info subdomain
- Block search engine crawlers with robots.txt and meta tags
- Use shared design system from cloud.werdxz.info CDN
- Keep implementation simple with minimal dependencies

## Non-Goals
- SEO optimization (intentionally blocked)
- Server-side rendering (using pure client-side approach)
- Complex state management or routing
- Authentication/password protection (may add later)

## Proposed Changes

### 1. Resume Page Implementation (NEW spec: `resume-page`)
Create a new `resume/` directory with:
- HTML page using custom elements for resume sections
- Client-side JavaScript to fetch and render resume data
- CSS styling using shared design system variables
- Robots.txt and noindex meta tags to block crawlers
- Resume data structure (JSON or embedded)

### 2. Resume Deployment Configuration (NEW spec: `resume-deployment`)
- Configure Cloudflare Workers/Pages for resume subdomain
- Set up wrangler.toml for resume.werdxz.info
- Add deployment scripts and documentation
- Configure DNS/routing for subdomain

## Dependencies
- Shared design system (global-styles) - already implemented
- Custom elements API (browser native, no framework)
- Cloudflare Workers/Pages platform

## Risks and Mitigations
- **Risk**: Custom elements browser support
  - **Mitigation**: Target modern browsers only (same as shared styles)
- **Risk**: Client-side rendering SEO
  - **Mitigation**: Not a concern, intentionally blocking crawlers
- **Risk**: Resume data format/source
  - **Mitigation**: Start simple with embedded/static JSON, can enhance later

## Alternatives Considered
1. **SSR with framework** - Rejected: Overkill for resume page, adds complexity
2. **Pure HTML/CSS** - Rejected: Want to experiment with custom elements
3. **Include in main site** - Rejected: Want subdomain isolation and different stack

## Open Questions
1. Resume data structure - embedded in HTML or separate JSON file?
2. Custom elements naming convention - `<resume-section>`, `<r-section>`, etc.?
3. Should we include PDF download functionality in v1?
4. Typography/design details - waiting for user input

## Success Metrics
- Resume page loads and renders correctly on modern browsers
- Properly blocked from search engines (verified with robots checker)
- Uses shared design system consistently
- Deployed successfully to resume.werdxz.info subdomain
