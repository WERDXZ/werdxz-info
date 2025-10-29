# portfolio-frontend Delta

## ADDED Requirements

### Requirement: Leptos SSR Single-Page Portfolio Site
The portfolio site MUST be a server-side rendered Leptos application deployed on Cloudflare Pages.

#### Scenario: Portfolio site is accessible
**GIVEN** the portfolio site is deployed
**WHEN** visiting https://portfolio.werdxz.info
**THEN** respond with 200 OK
**AND** page is server-rendered (no loading spinners on initial load)
**AND** HTML contains all sections: Hero, Projects, Experience, Writing, About, Contact

#### Scenario: Mobile responsive layout
**GIVEN** the portfolio site is loaded
**WHEN** viewing on mobile device (< 768px)
**THEN** layout adapts to single column
**AND** navigation is touch-friendly
**AND** images scale appropriately

### Requirement: Featured Projects Section
The portfolio MUST display 3-6 featured projects with business-focused messaging.

#### Scenario: Projects display with business focus
**GIVEN** there are 5 featured projects in KV
**WHEN** the Projects section renders
**THEN** display 5 project cards in grid layout
**AND** each card shows: title, description, technologies, image, links
**AND** description emphasizes business value and impact
**AND** technologies displayed as tags
**AND** links render as action buttons (Website, GitHub, etc.)

#### Scenario: Project card click behavior
**GIVEN** a project has redirect_url set
**WHEN** user clicks the project card
**THEN** navigate to redirect_url in new tab
**AND** preserve portfolio page state

#### Scenario: No featured projects
**GIVEN** portfolio:featured_projects returns empty array
**WHEN** the Projects section renders
**THEN** display placeholder message "No featured projects yet"
**AND** section remains visible (not hidden)

### Requirement: Work Experience Section
The portfolio MUST display 2-4 work experiences with impact narratives.

#### Scenario: Experience items display
**GIVEN** there are 3 featured experiences in KV
**WHEN** the Experience section renders
**THEN** display 3 experience items
**AND** each item shows: company, role, period, location, description, technologies
**AND** description emphasizes measurable impact
**AND** period displayed as human-readable text ("Summer 2024")
**AND** technologies displayed as tags

#### Scenario: Experience click behavior
**GIVEN** an experience has redirect_url set
**WHEN** user clicks the company name
**THEN** navigate to redirect_url in new tab

### Requirement: Featured Writing Section
The portfolio MUST display 0-3 featured blog posts fetched from the API.

#### Scenario: Featured posts display
**GIVEN** there are 2 featured post slugs in KV
**AND** API returns post data for those slugs
**WHEN** the Featured Writing section renders
**THEN** display 2 blog post cards
**AND** each card shows: title, summary, published date, tags
**AND** clicking card navigates to blog post on blog.werdxz.info

#### Scenario: No featured posts
**GIVEN** portfolio:featured_posts returns empty array
**WHEN** page renders
**THEN** Featured Writing section is hidden entirely
**AND** other sections remain unaffected

#### Scenario: API failure graceful degradation
**GIVEN** API server is unavailable
**WHEN** fetching featured posts fails
**THEN** hide Featured Writing section
**AND** log error to console
**AND** do not show error message to user
**AND** other sections load normally

### Requirement: Contact Form
The portfolio MUST provide a contact form that stores submissions in KV.

#### Scenario: Contact form submission success
**GIVEN** user is on the portfolio page
**WHEN** user fills name, email, message fields
**AND** submits the form
**THEN** validate input server-side
**AND** store submission in KV with timestamp
**AND** display success message
**AND** clear form fields

#### Scenario: Contact form validation
**GIVEN** user submits contact form
**WHEN** name is empty OR email is invalid OR message is empty
**THEN** display validation error
**AND** do not submit to server
**AND** highlight invalid fields

#### Scenario: Contact form submission failure
**GIVEN** KV write fails
**WHEN** user submits contact form
**THEN** display error message "Unable to submit. Please try again."
**AND** preserve form field values
**AND** log error details

### Requirement: Server Functions for Data Fetching
The portfolio MUST use Leptos server functions for all data operations.

#### Scenario: Server function returns projects
**GIVEN** KV contains featured projects
**WHEN** get_featured_projects() server function is called
**THEN** read portfolio:featured_projects index
**AND** fetch each project in parallel
**AND** return Vec<Project>
**AND** handle KV errors gracefully

#### Scenario: Server function returns experience
**GIVEN** KV contains featured experiences
**WHEN** get_featured_experience() server function is called
**THEN** read portfolio:featured_experience index
**AND** fetch each experience in parallel
**AND** return Vec<Experience>

#### Scenario: Server function fetches posts from API
**GIVEN** KV contains featured post slugs
**WHEN** get_featured_posts() server function is called
**THEN** read portfolio:featured_posts from KV
**AND** fetch each post from GET /v1/posts/{slug}
**AND** filter out failed fetches (404, 500)
**AND** return Vec<BlogPost> with only successful fetches

### Requirement: Performance Targets
The portfolio MUST meet performance benchmarks for business audiences.

#### Scenario: Fast initial page load
**GIVEN** portfolio site is deployed to production
**WHEN** loading page from edge location
**THEN** Time to First Byte (TTFB) < 200ms
**AND** First Contentful Paint (FCP) < 1s
**AND** Largest Contentful Paint (LCP) < 2s

#### Scenario: Small bundle size
**GIVEN** Leptos app is built for production
**WHEN** measuring WASM bundle size
**THEN** total bundle < 150KB gzipped
**AND** CSS < 20KB gzipped

