# resume-page Specification

## Purpose
TBD - created by archiving change implement-resume-page. Update Purpose after archive.
## Requirements
### Requirement: The resume page MUST use semantic HTML with custom elements for modular resume sections

The resume page SHALL be built with semantic HTML5 structure and use Web Components (custom elements) to encapsulate and render different resume sections, providing a clean separation of concerns and reusable components.

#### Scenario: Resume page loads with custom elements
**Given** a user navigates to resume.werdxz.info
**When** the page loads
**Then** the HTML contains custom element tags like `<resume-header>`, `<resume-experience>`, `<resume-education>`
**And** custom elements are defined and registered in JavaScript
**And** the page uses semantic HTML structure (main, section, article)

#### Scenario: Custom elements render resume data
**Given** custom elements are registered
**When** the DOM is parsed
**Then** each custom element renders its corresponding resume section
**And** data is passed to custom elements via attributes or properties
**And** elements use shadow DOM for style encapsulation (optional)

### Requirement: The resume page MUST be blocked from search engine indexing using robots.txt and meta tags

The resume page SHALL prevent search engine crawlers from indexing the page through multiple blocking mechanisms including robots.txt file and HTML meta tags.

#### Scenario: Robots.txt blocks all crawlers
**Given** resume/public/robots.txt exists
**When** a search engine bot requests /robots.txt
**Then** the file contains `User-agent: *` and `Disallow: /`
**And** no sitemap reference is included

#### Scenario: Meta tags prevent indexing
**Given** the resume HTML head section
**When** the page is rendered
**Then** it contains `<meta name="robots" content="noindex, nofollow">`
**And** no conflicting meta tags allow indexing

### Requirement: The resume page MUST import and use CSS variables from the shared design system

The resume page styling SHALL import CSS custom properties from the cloud-hosted shared design system and use these variables for all colors, spacing, typography, and other design values.

#### Scenario: Import global CSS variables
**Given** the resume page CSS
**When** styles are applied
**Then** CSS imports `@import url('https://cloud.werdxz.info/shared/styles/variables.css');`
**And** all colors, spacing, typography use CSS custom properties
**And** no hard-coded design values are used

#### Scenario: Responsive layout using shared breakpoints
**Given** the resume page on different viewport sizes
**When** the page is rendered
**Then** media queries use shared breakpoint variables (--breakpoint-md, --breakpoint-lg)
**And** the layout adapts appropriately for mobile, tablet, desktop

### Requirement: The resume page MUST have a clear data structure for experience, education, and skills

The resume SHALL organize professional data in a structured format that can be easily accessed by custom elements, including work experience, education history, and technical skills.

#### Scenario: Resume data is accessible to custom elements
**Given** the resume application
**When** custom elements need to render data
**Then** data is available either embedded in HTML, in a separate JSON file, or inline in script
**And** data structure includes: experience (company, role, dates, description), education (institution, degree, dates), skills (categories and items)
**And** data format is consistent and easy to update

### Requirement: The resume page MUST render entirely client-side without server-side processing

The resume page SHALL be implemented as a static HTML file with client-side JavaScript that handles all dynamic rendering, requiring no server-side templating or preprocessing.

#### Scenario: Static HTML with JavaScript enhancement
**Given** the resume HTML file
**When** served from Cloudflare
**Then** the HTML is static and contains no server-side templates
**And** JavaScript handles all dynamic rendering
**And** the page works without a build step

#### Scenario: Progressive enhancement
**Given** a browser with JavaScript disabled
**When** the resume page loads
**Then** basic HTML content is still visible
**Or** a fallback message indicates JavaScript is required
**And** no errors are thrown

### Requirement: The resume page MUST target modern browsers only, consistent with the shared design system

The resume page SHALL be developed for modern evergreen browsers without polyfills or fallbacks for older browsers, aligning with the design system's browser support policy.

#### Scenario: Custom elements browser support
**Given** the resume page uses custom elements API
**When** loaded in modern browsers (Chrome, Firefox, Safari, Edge - latest versions)
**Then** all features work correctly
**And** no polyfills are included for older browsers
**And** CSS custom properties are used without fallbacks

### Requirement: The resume page MUST have professional styling with clear visual hierarchy using shared design tokens

The resume page SHALL apply professional typography, spacing, and layout using values from the shared design system to create a clear visual hierarchy appropriate for a professional resume.

#### Scenario: Typography and spacing
**Given** the resume page content
**When** rendered with styles
**Then** typography uses shared font variables (--font-family-sans, --font-size-*)
**And** spacing uses shared spacing scale (--spacing-*)
**And** visual hierarchy is clear (headings, sections, lists)

#### Scenario: Single-page layout
**Given** the resume content
**When** the page is rendered
**Then** all content fits on a single scrollable page
**And** sections are clearly separated visually
**And** the layout is print-friendly (for future PDF generation)

