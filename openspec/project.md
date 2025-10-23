# Project Context

## Purpose
werdxz.info is a multi-stack website experiment showcasing different frontend technologies across different sections. The project serves as both a personal website and a playground for exploring various web development approaches.

## Tech Stack
- **Home Page**: Pure HTML/CSS (no framework)
- **Deployment**: Cloudflare Workers
- **Design System**: CSS Custom Properties (CSS Variables)
- **Future Sections**: React, Vue, Svelte, Web Components (planned)

## Project Conventions

### Code Style
- **HTML**: Use semantic HTML5 elements
- **CSS**: Follow BEM-like naming conventions for classes
- **Indentation**: Use consistent indentation (spaces or tabs)
- **Comments**: Add comments for complex logic or design decisions

### Architecture Patterns

#### Multi-Stack Monorepo
- Each website section is an independent app with its own tech stack
- Apps share design tokens through global CSS variables
- No code sharing between apps (except design tokens)
- Each app can be deployed independently

#### Static Asset Management via Git Hooks
- Static assets (`/shared/styles/`, `www/public/`) are NOT committed to git
- Pre-commit hook: Uploads changed assets to private bucket and unstages them
- Post-merge hook: Downloads latest assets from bucket after pull
- Local development uses files from bucket sync

**Rationale**:
- Keeps repository clean and small
- Prevents merge conflicts on binary/generated files
- Centralized storage for static assets
- Automated synchronization workflow

#### Global Styles Directory Structure
```
/shared/
  /styles/
    variables.css    # CSS custom properties (design tokens)
    README.md        # Usage documentation
```

**Usage**:
- Import in your app: `@import url('../../shared/styles/variables.css');`
- Use variables: `color: var(--color-primary-500);`
- Follow naming convention: `--category-variant-state`

**Design Token Categories**:
- Colors: Primary, secondary, neutral, semantic, contextual
- Typography: Font families, sizes, weights, line heights
- Spacing: Scale from 4px to 128px
- Layout: Breakpoints, border radius, shadows, z-index
- Transitions: Fast, base, slow

### Testing Strategy
- Manual testing on different viewport sizes (mobile, tablet, desktop)
- Accessibility testing with keyboard navigation
- Cross-browser testing (Chrome, Firefox, Safari)
- Performance testing (page load times, asset sizes)

### Git Workflow
- **Branching**: Feature branches for new features, direct commits to master for small changes
- **Commits**: Descriptive commit messages
- **Hooks**: Automated via setup-hooks.sh script
  - Pre-commit: Upload static assets to bucket
  - Post-merge: Download latest static assets
- **Setup**: New developers run `./scripts/setup-hooks.sh` and configure `.env`

## Domain Context

### Multi-Stack Philosophy
This website intentionally uses different technologies for different sections to:
1. Explore various frontend frameworks and patterns
2. Learn through practical implementation
3. Compare approaches and trade-offs
4. Maintain flexibility to adopt new technologies

### Content Strategy
- **Current**: Placeholder content with structure in place
- **Future**: Dynamic content fetched from GitHub API
- **Links**: Sections for blog, portfolio, experiments (coming soon)

## Important Constraints

### No Build Step for Home Page
- The home page (`www/`) uses pure HTML/CSS without a build process
- This keeps it simple, fast, and framework-free
- Other sections may have build steps as needed for their tech stack

### Bucket Configuration Required
- Developers need to configure `.env` with bucket credentials
- Without bucket config, hooks run in placeholder mode (log only)
- Bucket service choice is flexible (S3, R2, GCS, etc.)

### CSS Variables Compatibility
- CSS Custom Properties require modern browsers
- Fallbacks not implemented (targeting modern browsers only)
- Dark mode support via `prefers-color-scheme` media query

## External Dependencies

### Cloudflare Workers
- Hosting platform for the website
- Configuration in `www/wrangler.jsonc`
- Deployment via `wrangler deploy`
- Assets served from `www/public/` directory

### Private Bucket (To Be Configured)
- Storage for static assets (`/shared/styles/`, `www/public/`)
- Not yet configured - placeholder mode active
- Options: AWS S3, Cloudflare R2, Google Cloud Storage
- Credentials stored in `.env` (not committed)

### Future Dependencies
- GitHub API for dynamic content
- Analytics (if added)
- CDN for optimized asset delivery

## Development Workflow

### Initial Setup
1. Clone repository
2. Run `./scripts/setup-hooks.sh`
3. Copy `.env.example` to `.env`
4. Configure bucket credentials (or use placeholder mode)
5. Run `npm run dev` in `www/` to start development server

### Making Changes to Static Assets
1. Edit files in `/shared/styles/` or `www/public/`
2. Run `git add <files>`
3. Run `git commit` - pre-commit hook will upload to bucket and unstage files
4. Your code changes (not static assets) will be committed

### Pulling Changes
1. Run `git pull`
2. Post-merge hook automatically downloads latest static assets from bucket
3. Local files are synchronized

### Deploying
1. Ensure latest changes are committed
2. Run `npm run deploy` in `www/` directory
3. Cloudflare Workers will deploy the latest version
