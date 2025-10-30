# werdxz.info

Monorepo for my personal website, blog, portfolio, and API backend.

## Projects

### `www/` - Homepage
Pure HTML/CSS static homepage served via Cloudflare Pages.

**Tech Stack:** HTML, CSS, Cloudflare Pages

### `api/` - API Backend
RESTful API backend for blog posts, resume data, and portfolio projects.

**Tech Stack:** Rust, Cloudflare Workers, Cloudflare D1 (SQLite), Cloudflare R2 (S3-compatible storage)

### `blog/` - Blog Frontend
SSR blog with islands architecture for interactive components.

**Tech Stack:** Deno Fresh, TypeScript, Preact, Cloudflare Pages

### `portfolio/` - Portfolio Showcase
Multi-mode portfolio site with SSR, showcasing experience, projects, and writing.

**Tech Stack:** Rust, Leptos, Cloudflare Workers, Cloudflare KV

### `xtask/` - CLI Tooling
Cargo xtask automation for managing blog posts and projects.

**Tech Stack:** Rust, cargo-xtask pattern

### `shared/` - Shared Design System
CSS primitives, design tokens, and shared styles accessible via CDN.

**Tech Stack:** CSS Variables

## CLI Commands

**Manage blog posts:**
```bash
cargo xtask blog publish --slug "my-post" --title "My Post" blog/my-post.md
cargo xtask blog list
cargo xtask blog delete my-post
```

**Manage projects:**
```bash
cargo xtask project create \
  --slug "my-project" \
  --name "My Project" \
  --description "A cool project" \
  --stage "active" \
  --readme-url "https://github.com/user/repo"

cargo xtask project list
cargo xtask project delete --slug "my-project"
```

**Manage portfolio content:**
```bash
# See scripts/portfolio/README.md for detailed usage
./scripts/portfolio/hero.sh list
./scripts/portfolio/about.sh get rust
./scripts/portfolio/project.sh list
./scripts/portfolio/experience.sh list
```

## License

MIT
