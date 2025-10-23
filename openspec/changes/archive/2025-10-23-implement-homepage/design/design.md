# Design: implement-homepage

## Architecture Overview

This design establishes a unique multi-stack website architecture with automated asset management through git hooks.

## Key Design Decisions

### 1. Static-First Home Page
**Decision**: Implement the home page as pure HTML/CSS without a build step.

**Rationale**:
- Minimal complexity for a link hub and informational page
- Fast loading times with no JavaScript bundle
- Easy to maintain and update
- Aligns with the goal of different tech stacks for different sections

**Trade-offs**:
- No reactive components (acceptable for this use case)
- Manual DOM manipulation if interactivity is needed later
- Template repetition if structure becomes complex

### 2. Global Styles Architecture
**Decision**: Create a `/shared/styles/` directory at the monorepo root for CSS variables and common styles.

**Rationale**:
- Single source of truth for design tokens (colors, spacing, typography)
- Can be imported by any app regardless of tech stack
- Deployed to bucket so all apps can reference it
- Promotes visual consistency across different sections

**Structure**:
```
/shared/
  /styles/
    variables.css      # CSS custom properties (colors, spacing, etc.)
    reset.css         # Optional CSS reset/normalize
    utilities.css     # Optional common utility classes
```

**Trade-offs**:
- Need to ensure proper bucket paths for imports
- Changes require syncing to bucket
- Not type-safe like CSS-in-JS solutions (acceptable for simplicity)

### 3. Git Hook Deployment System
**Decision**: Use pre-commit and post-pull hooks to sync static assets with a private bucket.

**Workflow**:
```
Pre-commit hook:
1. Check for changes in /shared/styles/ or www/public/
2. Upload changed files to private bucket
3. Prevent commit of these files to git

Post-pull hook:
1. Download latest assets from private bucket
2. Place in appropriate local directories
3. Ensure local dev environment is up-to-date
```

**Rationale**:
- Keeps repository clean and small
- Automated synchronization reduces manual steps
- Private bucket provides secure storage
- Team members always have latest assets locally

**Trade-offs**:
- Requires initial bucket setup and credentials
- Hook failures could block commits (need good error handling)
- First-time setup requires hook installation
- Need fallback if bucket is unavailable

**Alternatives Considered**:
- Git LFS: Requires GitHub LFS quota, less flexible
- Submodules: More complex, harder to maintain
- CDN only: No local development copies

### 4. Deployment to Cloudflare Workers
**Decision**: Continue using existing Cloudflare Workers setup with static assets served from `www/public/`.

**Rationale**:
- Already configured in wrangler.jsonc
- Global edge network for fast delivery
- Integrates well with other Cloudflare services
- Simple deployment with `wrangler deploy`

**Flow**:
```
Development: Bucket → Local /public/ → Cloudflare Workers dev
Production: Bucket → CI/CD → Cloudflare Workers production
```

## Data Flow

```
┌─────────────────┐
│  Developer      │
│  Changes Code   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Pre-commit     │
│  Hook Runs      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Upload to      │
│  Private Bucket │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Prevent Git    │
│  Commit         │
└─────────────────┘

On pull:
┌─────────────────┐
│  git pull       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Post-pull      │
│  Hook Runs      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Download from  │
│  Bucket         │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Update Local   │
│  Files          │
└─────────────────┘
```

## Security Considerations

1. **Bucket Access**: Credentials stored in `.git/config` or environment variables, never committed
2. **HTTPS Only**: All bucket operations over HTTPS
3. **Access Control**: Private bucket with restricted access
4. **Validation**: Hooks validate file types and sizes before upload

## Scalability

This architecture supports:
- Multiple apps with different tech stacks (React, Vue, Svelte, etc.)
- Shared assets accessible to all apps
- Independent deployment of each app
- Easy addition of new sections

## Migration Path

Future enhancements:
1. Add TypeScript for type-safe style tokens
2. Implement GitHub API integration for dynamic content
3. Add build step for optimization if needed
4. Consider CDN caching strategies

## Open Questions

1. Which private bucket service? (S3, R2, GCS, etc.)
2. Bucket path structure? (e.g., `/assets/shared/styles/`, `/assets/www/public/`)
3. Versioning strategy for global styles?
4. Fallback behavior if bucket is unavailable?
