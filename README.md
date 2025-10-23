# Mono Repo for [werdxz.info](https://werdxz.info)

## Structure

- `www/`: The home page of my website (pure HTML/CSS)
- `shared/styles/`: Shared CSS primitives and design tokens (committed to git)
- `wrangler.toml`: Root config for Cloudflare resources (R2 buckets, etc.)

## Setup for New Developers

1. Clone the repository
2. Install dependencies: `cd www && npm install`
3. Run `npm run dev` in `www/` to start development server

### Optional: Git Hooks for R2 Content Sync

4. Authenticate with Cloudflare: `npx wrangler login`
5. Run `./scripts/setup-hooks.sh` to install git hooks
6. Git hooks will automatically sync content to R2 buckets:
   - `cloud` - Public bucket for images, media, fonts
   - `private` - Private bucket for HTML files

### Deployment to Cloudflare

Each app has its own `wrangler.toml` for deployment:

```bash
# One-time: Login to Cloudflare
npx wrangler login

# Deploy www (homepage)
cd www && npm run deploy

# List R2 buckets
npx wrangler r2 bucket list
```

### What's in Git vs R2 Buckets?

**Committed to Git (source code) AND R2 `cloud` bucket:**
- Shared files (`shared/styles/`) - Git for version control, `cloud` bucket for public CDN access
- Configuration files (`wrangler.toml`, `package.json`)
- Documentation

**Git ignored, R2 `private` bucket (sync only, no git bloat):**
- HTML files (`www/public/*.html`)
- Images (`www/public/images/`)
- Media files (`www/public/media/`)
- Fonts (`www/public/fonts/`)
- Any frequently changing content

**Bucket Strategy:**
- `cloud` - **Public**: Only `shared/` files that all frontends need to access via CDN
- `private` - **Private**: Everything else, just for sync to avoid bloating git history

**Why this split?**
- Shared CSS in `cloud` bucket: All frontends can `@import url('https://cloud.werdxz.info/shared/styles/variables.css')`
- App-specific files in `private` bucket: Synced locally via git hooks, no commit history bloat

For more details on the shared design system, see `shared/styles/README.md`
