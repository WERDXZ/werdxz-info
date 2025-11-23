## Why

Blog posts currently have no way for readers to engage or provide feedback. Adding a comment system will enable discussions and community interaction without building custom backend infrastructure.

## What Changes

- Add Giscus comment widget to blog post pages
- Comments stored in GitHub Discussions at `WERDXZ/blog` repository
- Custom Gruvbox theme CSS served from API worker for faster loading
- Theme supports both light and dark mode via `prefers-color-scheme`

## Impact

- Affected specs: `blog-content-display`
- Affected code:
  - `blog/src/routes/posts/[slug]/index.tsx` - Giscus widget integration
  - `blog/src/routes/posts/[slug]/index.module.css` - Comments section styling
  - `blog/public/giscus-theme.css` - Custom Gruvbox theme (new file, same-origin for faster loading)
- External dependency: Giscus (GitHub Discussions-based comments)
- New GitHub repository: `WERDXZ/blog` (CC BY-SA 4.0 license)
