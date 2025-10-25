# Project Context

## Purpose
werdxz.info is a multi-stack website experiment showcasing different frontend technologies across different sections. The project serves as both a personal website and a playground for exploring various web development approaches.

## Tech Stack
- **Home Page** (`www/`): Pure HTML/CSS (no framework)
- **Resume Page** (`resume/`): Web Components (custom elements)
- **API Backend** (`api/`): workers-rs (Rust on Cloudflare Workers)
- **Blog** (planned): Qwik City
- **Portfolio** (planned): Leptos SSR
- **Projects** (planned): Qwik City or static generation
- **Deployment**: Cloudflare Workers for all services
- **Design System**: CSS Custom Properties served from cloud bucket
- **Data Storage**: D1 (SQLite), Durable Objects, R2 bucket

## Project Conventions

### Code Style
- **HTML**: Use semantic HTML5 elements
- **CSS**: Follow BEM-like naming conventions for classes
- **JavaScript**: ES modules, no build step for simple pages
- **Rust**: Follow Rust standard style (`cargo fmt`), use `clippy` for linting
- **Indentation**: Use consistent indentation (spaces or tabs per language convention)
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
- **Current**: Homepage with links, resume page with filtered data from cloud bucket
- **In Progress**: API backend for blog posts and resume filtering
- **Planned**: Blog (technical writing), Portfolio (curated projects), Projects (GitHub integration)
- **Audience Segmentation**:
  - **Portfolio** → Recruiters/employers (showcase best work)
  - **Projects** → Fellow developers (collaboration, contributions)
  - **Blog** → Everyone (teaching, knowledge sharing)
  - **Resume** → HR/hiring managers (credentials, printable)

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

### Cloudflare Infrastructure
- **Workers**: Hosting for www, resume, api services
- **D1**: SQLite database for API metadata (blog posts, etc.)
- **Durable Objects**: Content storage for blog markdown
- **R2 Bucket** (`cloud`): Shared assets (styles, resume data)
- **Deployment**: `wrangler deploy` per service

### Cloudflare R2 Bucket (`cloud`)
- **URL**: cloud.werdxz.info
- **Storage**:
  - `/shared/styles/variables.css` - Design tokens
  - `/resume/public/resume.json` - Resume data (not in git)
  - Static assets for www (uploaded via git hooks)
- **CORS**: Configured for all werdxz.info subdomains
- **Credentials**: Stored in `.env` (not committed)

### Future Dependencies
- GitHub API for projects page integration
- Analytics (if added)
- Search service for blog (possibly D1 FTS5)

## Development Workflow

### Initial Setup
1. Clone repository
2. Run `./scripts/setup-hooks.sh` to install git hooks
3. Copy `.env.example` to `.env`
4. Configure R2 bucket credentials in `.env`
5. Run `npm run dev` in service directories (`www/`, `resume/`, `api/`) for local development

### Making Changes to Static Assets
1. Edit files in `/shared/styles/` or `www/public/`
2. Run `git add <files>`
3. Run `git commit` - pre-commit hook will upload to bucket and unstage files
4. Your code changes (not static assets) will be committed

### Pulling Changes
1. Run `git pull`
2. Post-merge hook automatically downloads latest static assets from bucket
3. Local files are synchronized

### Deploying Services
1. Ensure latest changes are committed
2. Navigate to service directory (`www/`, `resume/`, `api/`)
3. Run `npm run deploy` or `wrangler deploy`
4. Each service deploys independently to its subdomain:
   - `www/` → werdxz.info
   - `resume/` → resume.werdxz.info
   - `api/` → api.werdxz.info

### Managing Blog Content (API)
1. Use `werdxz-cli` wrapper tool (wraps wrangler commands)
2. Create posts: `werdxz-cli posts create --title "..." --file post.md`
3. Publish: `werdxz-cli posts publish <slug>`
4. No web-based CMS (v1) - CLI-first approach for security
