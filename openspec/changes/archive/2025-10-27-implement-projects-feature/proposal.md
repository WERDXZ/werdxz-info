# Proposal: Implement Projects Showcase Feature

## Summary
Add a projects showcase at `projects.werdxz.info` that displays open-source projects for technical audience, targeting collaboration and contributions.

## Why
The projects page serves a distinct audience from the portfolio (recruiters) - it targets fellow developers for collaboration and open-source contributions. Projects need their own dedicated space to:
- Showcase technical work-in-progress and experimental projects
- Signal which projects are actively seeking contributors
- Provide easy access to GitHub repos, live demos, and documentation
- Allow technical readers to explore project READMEs without leaving the site

This aligns with the multi-stack philosophy of exploring different frameworks (Deno Fresh) while maintaining design consistency across all properties.

## Problem
Currently, there's no dedicated space to showcase open-source projects. The projects page needs to:
- Display project metadata (name, description, stage, tags)
- Indicate if projects are open to contributors
- Link to external resources (GitHub, live demos, documentation, package registries)
- Support project detail pages with rendered READMEs from external sources

## Proposed Solution
Implement a projects feature with three main components:

1. **API Backend** (Rust/Workers):
   - D1 database with normalized schema for projects, tags, and URLs
   - REST endpoints for listing and fetching project details
   - xtask CLI commands for project management

2. **Frontend** (Deno Fresh):
   - Projects listing page at `projects.werdxz.info`
   - Project detail pages at `/[slug]` with external README rendering
   - Pure CSS styling using shared design tokens

3. **Data Management**:
   - CLI-based project management via `cargo xtask project`
   - Normalized D1 schema similar to blog posts architecture
   - External README links (GitHub raw URLs)

## Capabilities Affected
- **NEW**: Projects API (backend endpoints and D1 schema)
- **NEW**: Projects Frontend (Deno Fresh application)
- **NEW**: Projects CLI Management (xtask commands)

## Out of Scope
- Web-based admin interface (CLI-only for MVP)
- Automatic GitHub API integration
- Project analytics or metrics
- Comments or discussions on projects

## Dependencies
- Existing API infrastructure (Cloudflare Workers, D1)
- Existing xtask framework
- Shared CSS design tokens from cloud bucket
- Cloudflare Pages deployment for Fresh app

## Success Criteria
- Projects can be added/updated via `cargo xtask project`
- API serves project list and details at `/v1/projects`
- Frontend displays projects at `projects.werdxz.info`
- Project detail pages render external READMEs
- Design is consistent with blog and resume pages

## Timeline Estimate
- API Backend: 2-3 hours
- Frontend (Fresh): 3-4 hours
- CLI Tooling (xtask): 1-2 hours
- Total: ~6-9 hours

## Open Questions
None - design is clear based on previous blog architecture.
