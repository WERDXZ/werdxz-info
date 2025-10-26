# Tasks: Implement Blog Frontend

## Phase 1: Project Setup and Configuration

### Task 1.1: Initialize Qwik City Project
**Estimated effort:** 30 minutes
**Delivers:** Working Qwik City scaffold with TypeScript

**Actions:**
- [ ] Run `npm create qwik@latest` in repository root, choose "blog" as directory name
- [ ] Select "Empty App" template (we'll build from scratch)
- [ ] Enable TypeScript strict mode
- [ ] Install dependencies: `npm install`
- [ ] Verify dev server runs: `npm run dev`
- [ ] Commit scaffold to git

**Validation:**
- Dev server starts without errors on http://localhost:5173
- TypeScript compilation succeeds
- Hot module replacement works

**Dependencies:** None

---

### Task 1.2: Configure Design System Integration
**Estimated effort:** 20 minutes
**Delivers:** Global styles with design system CSS variables

**Actions:**
- [ ] Create `blog/src/global.css` with design system import
- [ ] Add `@import url('https://cloud.werdxz.info/shared/styles/variables.css');`
- [ ] Import global.css in `blog/src/root.tsx`
- [ ] Add base body styles (background, font-family)
- [ ] Test dark mode support (toggle system preference)

**Validation:**
- CSS variables available in browser dev tools
- Dark mode switches colors automatically
- Fonts load correctly

**Dependencies:** Task 1.1

---

### Task 1.3: Configure Environment Variables and API Client
**Estimated effort:** 30 minutes
**Delivers:** API client utility for fetching posts

**Actions:**
- [ ] Create `.env.example` with `VITE_API_URL=https://api.werdxz.info/v1`
- [ ] Create `.env.local` for development (gitignored)
- [ ] Create `blog/src/lib/api.ts` with API client functions
- [ ] Implement `fetchPosts()` and `fetchPost(slug)` functions
- [ ] Add error handling and TypeScript types for API responses
- [ ] Test API client against local API server

**Validation:**
- API client successfully fetches posts from dev API
- TypeScript types match API response structure
- Error handling works for network failures

**Dependencies:** Task 1.1

---

## Phase 2: Routing and Layout

### Task 2.1: Create Root Layout
**Estimated effort:** 45 minutes
**Delivers:** Shared layout with header and footer

**Actions:**
- [ ] Create `blog/src/routes/layout.tsx` with header and footer
- [ ] Build `<Header>` component with navigation links
- [ ] Build `<Footer>` component with copyright and social links
- [ ] Style header and footer using design system variables
- [ ] Ensure layout is mobile-responsive
- [ ] Add accessibility (ARIA landmarks, semantic HTML)

**Validation:**
- Header and footer appear on all pages
- Navigation links are keyboard accessible
- Layout is responsive on mobile and desktop

**Dependencies:** Task 1.2

---

### Task 2.2: Implement Blog Index Route
**Estimated effort:** 1 hour
**Delivers:** Blog index page listing posts

**Actions:**
- [ ] Create `blog/src/routes/index.tsx` for blog index
- [ ] Implement `routeLoader$` to fetch posts from API
- [ ] Create `<PostCard>` component for post summaries
- [ ] Display posts in vertical list layout
- [ ] Add loading state (skeleton loaders)
- [ ] Add error state (show error message)
- [ ] Implement pagination controls (if > 10 posts)

**Validation:**
- Posts load and display correctly
- Clicking post card navigates to post detail
- Pagination works (if applicable)
- Loading and error states render correctly

**Dependencies:** Task 2.1, Task 1.3

---

### Task 2.3: Implement Post Detail Route
**Estimated effort:** 1 hour
**Delivers:** Individual post pages with full content

**Actions:**
- [ ] Create `blog/src/routes/posts/[slug]/index.tsx`
- [ ] Implement `routeLoader$` to fetch single post by slug
- [ ] Create `<PostHeader>` component (title, date, tags, read time)
- [ ] Create `<PostContent>` component (markdown rendering - placeholder for now)
- [ ] Handle 404 when post not found
- [ ] Add loading state
- [ ] Implement SSG with `onStaticGenerate`

**Validation:**
- Post detail page loads for valid slug
- 404 page shows for invalid slug
- Post metadata displays correctly
- SSG generates static HTML for all posts

**Dependencies:** Task 2.1, Task 1.3

---

### Task 2.4: Implement Tag Filter Route
**Estimated effort:** 45 minutes
**Delivers:** Tag pages showing filtered posts

**Actions:**
- [ ] Create `blog/src/routes/tags/[tag]/index.tsx`
- [ ] Implement `routeLoader$` to fetch posts filtered by tag
- [ ] Reuse `<PostCard>` component from index
- [ ] Display tag name as page heading
- [ ] Show post count for tag
- [ ] Handle empty state (no posts with tag)

**Validation:**
- Tag page shows correct filtered posts
- Tag name displays in heading
- Empty state works for non-existent tags

**Dependencies:** Task 2.2

---

## Phase 3: Content Rendering

### Task 3.1: Implement Markdown Rendering
**Estimated effort:** 1.5 hours
**Delivers:** Markdown-to-HTML conversion with styling

**Actions:**
- [ ] Install markdown-it: `npm install markdown-it @types/markdown-it`
- [ ] Create `blog/src/lib/markdown.ts` with markdown parser
- [ ] Configure markdown-it plugins (linkify, typography)
- [ ] Sanitize HTML output (use DOMPurify or built-in sanitization)
- [ ] Update `<PostContent>` to render markdown
- [ ] Style rendered HTML (headings, paragraphs, lists, blockquotes)

**Validation:**
- Markdown renders correctly to HTML
- Styles match design system
- Headings, lists, links, blockquotes format properly
- HTML output is sanitized (no XSS)

**Dependencies:** Task 2.3

---

### Task 3.2: Add Syntax Highlighting for Code Blocks
**Estimated effort:** 1 hour
**Delivers:** Highlighted code blocks in posts

**Actions:**
- [ ] Install Shiki: `npm install shiki`
- [ ] Configure Shiki with VS Code theme (dark and light)
- [ ] Integrate Shiki with markdown-it (use markdown-it-shiki plugin)
- [ ] Style code blocks (background, padding, scrolling)
- [ ] Support common languages (JS, TS, Rust, Python, Go, HTML, CSS)
- [ ] Ensure code blocks work in dark mode

**Validation:**
- Code blocks have syntax highlighting
- Multiple languages are supported
- Theme matches design system
- Horizontal scroll works for long lines

**Dependencies:** Task 3.1

---

### Task 3.3: Add Copy Button to Code Blocks
**Estimated effort:** 45 minutes
**Delivers:** Copy-to-clipboard functionality for code

**Actions:**
- [ ] Create `<CodeBlock>` component with copy button
- [ ] Implement clipboard copy using navigator.clipboard API
- [ ] Show "Copied!" confirmation message
- [ ] Style copy button (position, hover state)
- [ ] Handle copy errors gracefully

**Validation:**
- Copy button appears on code blocks
- Clicking copies code to clipboard
- Confirmation message shows briefly
- Button is keyboard accessible

**Dependencies:** Task 3.2

---

### Task 3.4: Optimize Image Handling
**Estimated effort:** 30 minutes
**Delivers:** Responsive, lazy-loaded images

**Actions:**
- [ ] Add responsive image styling (max-width: 100%)
- [ ] Implement lazy loading (loading="lazy" attribute)
- [ ] Add width/height to prevent layout shift
- [ ] Handle broken images with alt text
- [ ] Test images in markdown

**Validation:**
- Images are responsive on mobile
- Lazy loading works (images load as scrolled into view)
- Alt text displays for broken images
- No layout shift when images load

**Dependencies:** Task 3.1

---

## Phase 4: SEO and Meta Tags

### Task 4.1: Add Meta Tags to Index Page
**Estimated effort:** 30 minutes
**Delivers:** SEO meta tags for blog index

**Actions:**
- [ ] Update `blog/src/routes/index.tsx` with `DocumentHead` export
- [ ] Add title, description meta tags
- [ ] Add Open Graph tags (og:title, og:description, og:type)
- [ ] Add canonical URL
- [ ] Test with meta tag validator

**Validation:**
- Meta tags appear in HTML <head>
- Open Graph preview shows correct info (test with FB/Twitter debugger)
- Canonical URL is correct

**Dependencies:** Task 2.2

---

### Task 4.2: Add Meta Tags to Post Pages
**Estimated effort:** 45 minutes
**Delivers:** SEO meta tags for post detail pages

**Actions:**
- [ ] Update post route with dynamic `DocumentHead` based on post data
- [ ] Add title: "{post.title} | werdxz.info"
- [ ] Add description from post.summary
- [ ] Add Open Graph tags for article (og:type=article, article:published_time, article:tag)
- [ ] Add JSON-LD structured data for Article schema
- [ ] Add canonical URL

**Validation:**
- Post meta tags are dynamic (different for each post)
- Structured data validates (use Google Rich Results Test)
- Social media previews show post info

**Dependencies:** Task 2.3

---

### Task 4.3: Generate Sitemap
**Estimated effort:** 30 minutes
**Delivers:** sitemap.xml for search engines

**Actions:**
- [ ] Create `blog/src/routes/sitemap.xml/index.ts` route
- [ ] Fetch all post slugs from API
- [ ] Generate XML sitemap dynamically or at build time
- [ ] Include blog index, all post pages, tag pages
- [ ] Set appropriate lastmod and priority
- [ ] Test sitemap format (use validator)

**Validation:**
- Sitemap accessible at /sitemap.xml
- All posts included in sitemap
- XML format is valid

**Dependencies:** Task 2.3

---

## Phase 5: Deployment and Production

### Task 5.1: Configure Cloudflare Pages Deployment
**Estimated effort:** 30 minutes
**Delivers:** Cloudflare Pages configuration

**Actions:**
- [ ] Create `blog/wrangler.toml` for Cloudflare Pages
- [ ] Set build command: `npm run build`
- [ ] Set output directory: `dist`
- [ ] Configure custom domain: `blog.werdxz.info`
- [ ] Set environment variables (VITE_API_URL for production)

**Validation:**
- wrangler.toml syntax is correct
- Build command succeeds locally
- Output directory contains static files

**Dependencies:** All previous tasks

---

### Task 5.2: Production Build and Deploy
**Estimated effort:** 45 minutes
**Delivers:** Blog deployed to blog.werdxz.info

**Actions:**
- [ ] Run `npm run build` to generate production build
- [ ] Test build output locally (serve dist/ folder)
- [ ] Deploy to Cloudflare Pages: `wrangler pages deploy dist`
- [ ] Verify deployment at blog.werdxz.info
- [ ] Configure DNS for blog.werdxz.info subdomain
- [ ] Test all routes on production

**Validation:**
- Blog loads at blog.werdxz.info
- All routes work (index, posts, tags)
- API integration works in production
- SSL certificate is active

**Dependencies:** Task 5.1

---

### Task 5.3: Performance Audit with Lighthouse
**Estimated effort:** 30 minutes
**Delivers:** Performance metrics and optimization

**Actions:**
- [ ] Run Lighthouse audit on blog index (blog.werdxz.info)
- [ ] Run Lighthouse audit on post detail page
- [ ] Address any performance issues (lazy-load, code-splitting)
- [ ] Optimize images if needed
- [ ] Rerun audit to verify improvements

**Validation:**
- Lighthouse Performance score > 95
- Lighthouse Accessibility score > 95
- Lighthouse SEO score > 95
- Core Web Vitals pass (LCP < 2.5s, FID < 100ms, CLS < 0.1)

**Dependencies:** Task 5.2

---

## Phase 6: Polish and Documentation

### Task 6.1: Mobile Responsive Testing
**Estimated effort:** 30 minutes
**Delivers:** Mobile-optimized blog

**Actions:**
- [ ] Test on mobile devices (iOS, Android)
- [ ] Test on tablets (iPad, Android tablet)
- [ ] Fix any layout issues on small screens
- [ ] Test navigation menu on mobile
- [ ] Ensure touch targets are adequate (44x44px)

**Validation:**
- Blog is fully functional on mobile
- Navigation works smoothly
- Content is readable on small screens
- No horizontal scroll (except code blocks)

**Dependencies:** Task 5.2

---

### Task 6.2: Cross-Browser Testing
**Estimated effort:** 30 minutes
**Delivers:** Browser compatibility verification

**Actions:**
- [ ] Test on Chrome (latest)
- [ ] Test on Firefox (latest)
- [ ] Test on Safari (latest)
- [ ] Fix any browser-specific issues
- [ ] Test on Edge (Chromium)

**Validation:**
- Blog works identically on all browsers
- No CSS or JavaScript errors
- Fonts and styles render correctly

**Dependencies:** Task 5.2

---

### Task 6.3: Update Project Documentation
**Estimated effort:** 30 minutes
**Delivers:** README and setup instructions

**Actions:**
- [ ] Create `blog/README.md` with overview
- [ ] Document local development setup
- [ ] Document deployment process
- [ ] Add to root README.md (link to blog)
- [ ] Update `openspec/project.md` with blog details

**Validation:**
- README is clear and accurate
- New developers can set up blog following docs
- Deployment instructions work

**Dependencies:** Task 5.2

---

## Summary

**Total estimated effort:** ~12-14 hours

**Phases:**
1. Setup (1.5 hours)
2. Routing (3.5 hours)
3. Content Rendering (3.25 hours)
4. SEO (1.75 hours)
5. Deployment (1.75 hours)
6. Polish (1.5 hours)

**Parallelizable work:**
- Tasks within Phase 3 (markdown rendering tasks) can be done in parallel after Task 3.1
- SEO tasks (Phase 4) can be done independently once routes exist
- Testing tasks (Phase 6) can be done in parallel

**Critical path:**
Task 1.1 → Task 1.2 → Task 1.3 → Task 2.1 → Task 2.2/2.3 → Task 3.1 → Task 3.2 → Task 5.1 → Task 5.2 → Task 5.3
