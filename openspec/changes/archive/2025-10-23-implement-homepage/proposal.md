# Proposal: implement-homepage

## Why
The website needs a home page that serves as a central hub with links to different sections and explains the multi-stack architecture philosophy. Additionally, a shared styles system is needed to maintain design consistency across different apps in the monorepo, and an automated deployment workflow is required to keep static assets out of git while making them accessible to all applications.

## What Changes
- Create static HTML/CSS home page in www/public/ with links section and design goals content
- Establish /shared/styles/ directory with CSS variables for colors, typography, spacing, and breakpoints
- Implement pre-commit hook to upload changed static assets to private bucket and prevent git commits
- Implement post-pull hook to download latest static assets from private bucket
- Add .gitignore rules for /shared/styles/ and document bucket deployment architecture
- Update project.md with global styles conventions and git hooks workflow

## Impact
- **Affected specs**: homepage (new), global-styles (new), git-deployment-hooks (new)
- **Affected code**:
  - www/public/index.html - complete rewrite from placeholder to functional home page
  - New /shared/styles/variables.css - centralized design tokens
  - New .git/hooks/pre-commit - bucket upload and commit prevention
  - New .git/hooks/post-merge - bucket download after pull
  - .gitignore - add /shared/styles/ exclusion
  - openspec/project.md - document new conventions
- **Infrastructure**: Requires private bucket configuration (S3, R2, or similar) with credentials
- **Developer workflow**: New developers must run setup-hooks.sh script and configure bucket credentials
