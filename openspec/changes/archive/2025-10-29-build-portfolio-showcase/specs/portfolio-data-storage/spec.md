# portfolio-data-storage Delta

## ADDED Requirements

### Requirement: KV Storage with Index Pattern
The portfolio MUST store curated content in Cloudflare KV using an index-based pattern.

#### Scenario: Featured projects index and data
**GIVEN** portfolio content is stored in KV
**WHEN** reading portfolio data
**THEN** key `portfolio:featured_projects` contains array of project IDs
**AND** each ID has corresponding key `portfolio:project:{id}` with project data
**AND** project data includes: title, description, technologies[], image_url, redirect_url, links[]
**AND** links is array of {label, url} objects

#### Scenario: Featured experience index and data
**GIVEN** portfolio content is stored in KV
**WHEN** reading portfolio data
**THEN** key `portfolio:featured_experience` contains array of experience IDs
**AND** each ID has corresponding key `portfolio:experience:{id}` with experience data
**AND** experience data includes: company, role, period, location, description, technologies[], redirect_url

#### Scenario: Featured posts list
**GIVEN** featured blog posts are curated
**WHEN** reading featured posts
**THEN** key `portfolio:featured_posts` contains array of post slugs
**AND** actual post data is fetched from API (not stored in KV)

#### Scenario: Contact submissions storage
**GIVEN** user submits contact form
**WHEN** storing submission
**THEN** create key `portfolio:contact:{timestamp}` with submission data
**AND** timestamp is Unix milliseconds for uniqueness and ordering
**AND** data includes: name, email, message, timestamp

### Requirement: Data Validation
All data written to KV MUST be validated before storage.

#### Scenario: Project validation
**GIVEN** adding new project via CLI
**WHEN** project data is invalid
**THEN** reject with clear error message
**AND** do not write to KV

**Validation rules**:
- title: non-empty, max 100 characters
- description: non-empty, max 2000 characters
- technologies: array with at least 1 item
- image_url: valid HTTPS URL
- redirect_url: valid HTTPS URL if provided
- links: each link has non-empty label and valid URL

#### Scenario: Experience validation
**GIVEN** adding new experience via CLI
**WHEN** experience data is invalid
**THEN** reject with clear error message
**AND** do not write to KV

**Validation rules**:
- company: non-empty, max 100 characters
- role: non-empty, max 100 characters
- period: non-empty, max 50 characters
- description: non-empty, max 2000 characters
- technologies: array with at least 1 item
- location: max 100 characters if provided
- redirect_url: valid HTTPS URL if provided

#### Scenario: Contact form validation
**GIVEN** user submits contact form
**WHEN** validating submission
**THEN** name must be non-empty, max 100 characters
**AND** email must match email format regex
**AND** message must be non-empty, max 5000 characters

### Requirement: Atomic Index Updates
Adding or removing items MUST update both the item data and the index atomically.

#### Scenario: Add project updates index
**GIVEN** adding new project with ID "rust-api"
**WHEN** writing to KV
**THEN** write `portfolio:project:rust-api` → {data}
**AND** read current `portfolio:featured_projects` array
**AND** append "rust-api" to array
**AND** write updated `portfolio:featured_projects`
**AND** if any step fails, do not commit partial update

#### Scenario: Remove project updates index
**GIVEN** removing project with ID "rust-api"
**WHEN** deleting from KV
**THEN** delete `portfolio:project:rust-api`
**AND** read current `portfolio:featured_projects` array
**AND** remove "rust-api" from array
**AND** write updated `portfolio:featured_projects`

### Requirement: Shared KV Namespace with Prefix
Portfolio data MUST use shared KV namespace with `portfolio:*` prefix.

#### Scenario: No key collisions with API
**GIVEN** API uses same KV namespace with `api:*` prefix
**AND** portfolio uses `portfolio:*` prefix
**WHEN** both systems write to KV
**THEN** keys do not collide
**AND** each system can only access its prefixed keys

#### Scenario: KV namespace binding
**GIVEN** portfolio site is deployed
**WHEN** configuring Cloudflare Pages
**THEN** KV binding named "PORTFOLIO" points to shared namespace
**AND** same namespace ID as API uses
**AND** portfolio can read/write `portfolio:*` keys via binding

### Requirement: Graceful KV Error Handling
All KV operations MUST handle errors gracefully.

#### Scenario: KV read failure
**GIVEN** KV read operation fails
**WHEN** fetching portfolio data
**THEN** log error details
**AND** return empty array (not error to user)
**AND** page renders with no featured items

#### Scenario: KV write failure
**GIVEN** KV write operation fails
**WHEN** submitting contact form
**THEN** log error details
**AND** return error to user
**AND** preserve user's form data

