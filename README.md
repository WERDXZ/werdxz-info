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
cargo xtask blog delete --slug "my-post"
```

**Manage projects:**
```bash
cargo xtask projects create \
  --slug "my-project" \
  --name "My Project" \
  --description "A cool project" \
  --stage "active" \
  --readme-url "https://github.com/user/repo"

cargo xtask projects list
cargo xtask projects delete --slug "my-project"
```

## License

MIT
