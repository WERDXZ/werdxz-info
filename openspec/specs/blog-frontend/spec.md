# blog-frontend Specification

## Purpose
TBD - created by archiving change implement-blog-frontend. Update Purpose after archive.
## Requirements
### Requirement: Qwik City Application Structure
The blog MUST be implemented as a Qwik City application with proper project structure.

#### Scenario: Initialize Qwik City project
**GIVEN** the blog frontend needs to be created
**WHEN** setting up the project structure
**THEN** create `blog/` directory at repository root
**AND** initialize Qwik City app with TypeScript
**AND** include package.json with dependencies: @builder.io/qwik, @builder.io/qwik-city, vite
**AND** configure vite.config.ts for Qwik
**AND** configure tsconfig.json for strict TypeScript

#### Scenario: Project directory structure
**GIVEN** the Qwik City app is initialized
**WHEN** reviewing the project structure
**THEN** have `blog/src/routes/` for file-based routing
**AND** have `blog/src/components/` for reusable components
**AND** have `blog/public/` for static assets
**AND** have `blog/src/entry.ssr.tsx` for SSR entry point
**AND** have `blog/src/root.tsx` for app root

### Requirement: Design System Integration
The blog MUST use the existing design system CSS variables.

#### Scenario: Import design tokens
**GIVEN** the blog needs consistent styling
**WHEN** setting up global styles
**THEN** import `https://cloud.werdxz.info/shared/styles/variables.css` in root layout
**AND** use CSS custom properties for colors (--color-*)
**AND** use CSS custom properties for spacing (--spacing-*)
**AND** use CSS custom properties for typography (--font-*)
**AND** support dark mode via `prefers-color-scheme` media query

#### Scenario: Component styling
**GIVEN** a blog component needs styling
**WHEN** writing component CSS
**THEN** use scoped styles (Qwik's component styles or CSS modules)
**AND** reference design tokens via `var(--token-name)`
**AND** avoid hardcoded colors, spacing, or typography
**AND** ensure mobile-first responsive design

### Requirement: API Integration
The blog MUST fetch data from the existing blog API.

#### Scenario: Configure API base URL
**GIVEN** the blog needs to fetch posts
**WHEN** configuring the application
**THEN** set API base URL via environment variable VITE_API_URL
**AND** use `https://api.werdxz.info/v1` for production
**AND** use `http://localhost:60232/v1` for development
**AND** fall back to production URL if env var not set

#### Scenario: Fetch posts list
**GIVEN** the blog index page loads
**WHEN** fetching posts from the API
**THEN** call GET `/posts` endpoint
**AND** parse JSON response with posts array and pagination
**AND** handle network errors gracefully (show error message)
**AND** implement loading states (skeleton or spinner)

#### Scenario: Fetch single post
**GIVEN** a post detail page loads
**WHEN** fetching a specific post
**THEN** call GET `/posts/{slug}` endpoint
**AND** parse JSON response with full post including content
**AND** handle 404 errors (post not found)
**AND** handle network errors gracefully

### Requirement: Cloudflare Pages Deployment
The blog MUST be deployable to Cloudflare Pages.

#### Scenario: Configure Cloudflare Pages
**GIVEN** the blog is ready for deployment
**WHEN** setting up deployment
**THEN** create `wrangler.toml` with Pages configuration
**AND** set build command to `npm run build`
**AND** set output directory to `dist`
**AND** configure custom domain `blog.werdxz.info`

#### Scenario: Build for production
**GIVEN** the blog is ready to deploy
**WHEN** running production build
**THEN** execute `npm run build`
**AND** generate static HTML for all routes
**AND** output to `dist/` directory
**AND** minify CSS and JS
**AND** generate sourcemaps for debugging

#### Scenario: Deploy to Cloudflare Pages
**GIVEN** the blog build is complete
**WHEN** deploying to production
**THEN** execute `wrangler pages deploy dist`
**AND** upload to Cloudflare Pages
**AND** make available at `blog.werdxz.info`
**AND** serve with edge caching

### Requirement: Performance Optimization
The blog MUST meet performance benchmarks.

#### Scenario: Lighthouse performance score
**GIVEN** the blog is deployed to production
**WHEN** running Lighthouse audit on blog index
**THEN** achieve Performance score > 95
**AND** achieve Accessibility score > 95
**AND** achieve Best Practices score > 90
**AND** achieve SEO score > 95

#### Scenario: Core Web Vitals
**GIVEN** the blog index page loads
**WHEN** measuring Core Web Vitals
**THEN** Largest Contentful Paint (LCP) < 2.5s
**AND** First Input Delay (FID) < 100ms
**AND** Cumulative Layout Shift (CLS) < 0.1

#### Scenario: Bundle size optimization
**GIVEN** the blog is built for production
**WHEN** analyzing bundle size
**THEN** initial JavaScript bundle < 50KB (gzipped)
**AND** use code splitting for markdown renderer
**AND** lazy-load images below the fold
**AND** tree-shake unused dependencies

### Requirement: Error Handling
The blog MUST handle errors gracefully.

#### Scenario: API network error
**GIVEN** the API is unreachable
**WHEN** fetching posts
**THEN** show error message "Unable to load posts. Please try again later."
**AND** display retry button
**AND** log error to console for debugging

#### Scenario: Post not found (404)
**GIVEN** a user navigates to `/posts/non-existent-slug`
**WHEN** the API returns 404
**THEN** show "Post not found" page
**AND** display link to blog index
**AND** return HTTP 404 status code

#### Scenario: Invalid API response
**GIVEN** the API returns malformed JSON
**WHEN** parsing the response
**THEN** catch parsing error
**AND** show generic error message
**AND** log detailed error for debugging

### Requirement: Local Development
The blog MUST support local development workflow.

#### Scenario: Run dev server
**GIVEN** a developer wants to work on the blog
**WHEN** running `npm run dev` in blog/ directory
**THEN** start Vite dev server on http://localhost:5173
**AND** enable hot module replacement (HMR)
**AND** connect to local API at http://localhost:60232/v1
**AND** show compilation errors in browser

#### Scenario: TypeScript type checking
**GIVEN** TypeScript code is written
**WHEN** running type check
**THEN** execute `npm run typecheck`
**AND** report type errors
**AND** fail build on type errors in CI/CD

#### Scenario: Code linting
**GIVEN** code needs quality checks
**WHEN** running linter
**THEN** execute `npm run lint`
**AND** use ESLint with Qwik config
**AND** enforce consistent code style
**AND** fail build on lint errors in CI/CD

