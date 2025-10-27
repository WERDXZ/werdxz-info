# Spec: Projects Frontend

## ADDED Requirements

### Requirement: Deno Fresh Application Setup
The frontend MUST be built with Deno Fresh and deployed to Cloudflare Pages.

#### Scenario: Fresh application structure
**GIVEN** the projects frontend is scaffolded
**WHEN** inspecting the project structure
**THEN** use Deno Fresh framework
**AND** deploy to `projects.werdxz.info` subdomain
**AND** configure for Cloudflare Pages deployment
**AND** use pure CSS (no Tailwind or CSS frameworks)
**AND** import shared CSS variables from `https://cloud.werdxz.info/shared/styles/variables.css`

### Requirement: Projects Listing Page
The frontend MUST display all projects in a grid layout.

#### Scenario: Display project cards
**GIVEN** the API returns 5 projects
**WHEN** visiting `projects.werdxz.info`
**THEN** display all projects in flat list layout
**AND** show single column on all screen sizes
**AND** order projects by created_at descending (newest first)
**AND** no pagination (expected max ~20 projects total)

#### Scenario: Project card content
**GIVEN** a project card is displayed
**WHEN** rendering the card
**THEN** show project name as h2 heading with link to detail page
**AND** show description text (truncate to ~150 characters if needed)
**AND** show stage badge with color coding: planned (gray), wip (blue), active (green), maintained (green), archived (orange), shelved (gray)
**AND** show "Open to Contributors" badge if open_to_contributors is true
**AND** show tags as flat text with middot separators (like blog)
**AND** show URL links with labels (GitHub, Demo, Docs, etc.)

#### Scenario: Empty projects state
**GIVEN** the API returns no projects
**WHEN** visiting the projects page
**THEN** show message "No projects yet. Check back soon!"
**AND** do not show empty grid

### Requirement: Project Detail Page
The frontend MUST display full project details with rendered README.

#### Scenario: Display project detail
**GIVEN** a project with slug "my-project" exists
**WHEN** visiting `/my-project`
**THEN** fetch project from `/v1/projects/my-project`
**AND** display project name as h1
**AND** show full description
**AND** show all metadata (stage, tags, contributor status)
**AND** show all URLs as clickable links
**AND** fetch and render README from `readme_url`

#### Scenario: Render external README
**GIVEN** a project has `readme_url` = "https://raw.githubusercontent.com/..."
**WHEN** displaying the project detail
**THEN** fetch markdown content from readme_url
**AND** parse markdown using `marked` library (same as blog)
**AND** render as HTML with styling matching blog content
**AND** support headings, lists, code blocks, links, images
**AND** handle fetch errors gracefully (show "README unavailable")

#### Scenario: Project not found
**GIVEN** no project with slug "nonexistent" exists
**WHEN** visiting `/nonexistent`
**THEN** display 404 page
**AND** show message "Project not found"
**AND** provide link back to projects list

### Requirement: Navigation and Layout
The frontend MUST have consistent header and navigation.

#### Scenario: Site header
**GIVEN** any page on projects.werdxz.info
**WHEN** displaying the page
**THEN** show header with site logo/name
**AND** include link to homepage (werdxz.info)
**AND** include link to blog, resume
**AND** use consistent styling with blog layout

#### Scenario: Breadcrumb navigation
**GIVEN** a project detail page
**WHEN** displaying the page
**THEN** show breadcrumb: "Projects / [Project Name]"
**AND** link "Projects" back to listing page

### Requirement: Tag Display
The frontend MUST display tags for visual organization.

#### Scenario: Display tags on cards
**GIVEN** a project has tags ["rust", "cli", "web"]
**WHEN** rendering the project card
**THEN** display tags as flat text with middot separators
**AND** style tags consistently with blog tags
**AND** tags are non-interactive (display only, no filtering)

### Requirement: Responsive Design
The frontend MUST be mobile-responsive.

#### Scenario: Mobile layout
**GIVEN** viewing on mobile device (<768px)
**WHEN** displaying projects
**THEN** use single column grid
**AND** stack project card elements vertically
**AND** ensure touch-friendly tap targets (≥44x44px)
**AND** allow horizontal scroll for long URLs/tags

#### Scenario: Desktop layout
**GIVEN** viewing on desktop (>768px)
**WHEN** displaying projects
**THEN** use single column list
**AND** constrain content width for readability
**AND** center content on page

### Requirement: Loading and Error States
The frontend MUST handle loading and error states.

#### Scenario: Loading state
**GIVEN** fetching projects from API
**WHEN** data is not yet loaded
**THEN** show skeleton loaders matching card layout
**AND** prevent layout shift when content loads

#### Scenario: API error handling
**GIVEN** API request fails
**WHEN** displaying the page
**THEN** show error message "Unable to load projects"
**AND** provide retry button
**AND** log error to console for debugging

### Requirement: Accessibility
The frontend MUST be accessible.

#### Scenario: Semantic HTML
**GIVEN** any page on the site
**WHEN** inspecting the markup
**THEN** use semantic HTML5 elements (header, nav, main, article, section)
**AND** use proper heading hierarchy (h1 → h2 → h3)
**AND** include alt text for any images

#### Scenario: Keyboard navigation
**GIVEN** a user navigating with keyboard
**WHEN** tabbing through the page
**THEN** all interactive elements are focusable
**AND** focus indicators are visible
**AND** tab order is logical (top to bottom, left to right)

### Requirement: Performance
The frontend MUST load quickly and efficiently.

#### Scenario: Asset optimization
**GIVEN** the site is deployed
**WHEN** measuring performance
**THEN** use server-side rendering (Fresh default)
**AND** minimize JavaScript bundle size
**AND** lazy-load README markdown rendering
**AND** achieve LCP < 2.5s, FID < 100ms

#### Scenario: Caching strategy
**GIVEN** API responses
**WHEN** fetching data
**THEN** use Fresh's data loading patterns
**AND** leverage CDN caching for static assets
**AND** set appropriate cache headers
