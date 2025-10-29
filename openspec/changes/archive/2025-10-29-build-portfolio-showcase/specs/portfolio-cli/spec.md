# portfolio-cli Delta

## ADDED Requirements

### Requirement: Portfolio Project Management Commands
The CLI MUST provide commands to manage featured projects.

#### Scenario: Add project command
**GIVEN** user wants to add a featured project
**WHEN** running `cargo xtask portfolio project add <id> --title "..." --description "..." --technologies "rust,typescript" --image "https://..." --link "GitHub:https://..."`
**THEN** validate all required fields are provided
**AND** validate data format (URLs, max lengths)
**AND** write `portfolio:project:{id}` to KV
**AND** append {id} to `portfolio:featured_projects` array
**AND** display success message with project ID

#### Scenario: List projects command
**GIVEN** there are featured projects in KV
**WHEN** running `cargo xtask portfolio project list`
**THEN** read `portfolio:featured_projects` array
**AND** fetch each project data in parallel
**AND** display table with: ID, title, technologies, image URL
**AND** show count of projects

#### Scenario: Remove project command
**GIVEN** project "rust-api" exists in KV
**WHEN** running `cargo xtask portfolio project remove rust-api`
**THEN** confirm deletion with user (Y/n prompt)
**AND** delete `portfolio:project:rust-api`
**AND** remove "rust-api" from `portfolio:featured_projects` array
**AND** display success message

#### Scenario: Add project with invalid data
**GIVEN** user provides invalid project data
**WHEN** running add command
**THEN** display validation errors
**AND** do not write to KV
**AND** exit with non-zero code

### Requirement: Portfolio Experience Management Commands
The CLI MUST provide commands to manage work experience.

#### Scenario: Add experience command
**GIVEN** user wants to add work experience
**WHEN** running `cargo xtask portfolio experience add <id> --company "..." --role "..." --period "..." --description "..." --technologies "python,react"`
**THEN** validate all required fields
**AND** write `portfolio:experience:{id}` to KV
**AND** append {id} to `portfolio:featured_experience` array
**AND** display success message

#### Scenario: List experience command
**GIVEN** there are experiences in KV
**WHEN** running `cargo xtask portfolio experience list`
**THEN** read `portfolio:featured_experience` array
**AND** fetch each experience data
**AND** display table with: ID, company, role, period
**AND** show count of experiences

#### Scenario: Remove experience command
**GIVEN** experience "meta-2024" exists
**WHEN** running `cargo xtask portfolio experience remove meta-2024`
**THEN** confirm deletion
**AND** delete `portfolio:experience:meta-2024`
**AND** remove from index array
**AND** display success message

### Requirement: Portfolio Post Management Commands
The CLI MUST provide commands to manage featured blog posts list.

#### Scenario: Add post to featured list
**GIVEN** blog post with slug "rust-api-guide" exists in API
**WHEN** running `cargo xtask portfolio post add rust-api-guide`
**THEN** verify post exists by calling GET /v1/posts/rust-api-guide
**AND** read current `portfolio:featured_posts` array
**AND** append "rust-api-guide" if not already present
**AND** write updated array to KV
**AND** display success message

#### Scenario: List featured posts
**GIVEN** there are featured post slugs in KV
**WHEN** running `cargo xtask portfolio post list`
**THEN** read `portfolio:featured_posts` array
**AND** fetch each post from API
**AND** display table with: slug, title, published date
**AND** show count of featured posts

#### Scenario: Remove post from featured list
**GIVEN** "rust-api-guide" is in featured posts
**WHEN** running `cargo xtask portfolio post remove rust-api-guide`
**THEN** read current `portfolio:featured_posts` array
**AND** remove "rust-api-guide" from array
**AND** write updated array to KV
**AND** display success message

#### Scenario: Add non-existent post
**GIVEN** post "fake-slug" does not exist in API
**WHEN** running `cargo xtask portfolio post add fake-slug`
**THEN** display error "Post not found: fake-slug"
**AND** do not update KV
**AND** exit with non-zero code

### Requirement: Contact Submissions Viewing
The CLI MUST provide command to view contact form submissions.

#### Scenario: List contact submissions
**GIVEN** there are contact submissions in KV
**WHEN** running `cargo xtask portfolio contact list`
**THEN** list all keys matching `portfolio:contact:*`
**AND** fetch each submission
**AND** display table with: timestamp, name, email, message preview
**AND** sort by timestamp descending (newest first)
**AND** truncate message to 50 characters in preview

#### Scenario: No contact submissions
**GIVEN** no submissions exist
**WHEN** running `cargo xtask portfolio contact list`
**THEN** display message "No contact submissions yet"
**AND** exit with zero code

### Requirement: CLI Error Handling
CLI commands MUST handle errors gracefully with helpful messages.

#### Scenario: KV access error
**GIVEN** KV credentials are invalid
**WHEN** running any portfolio command
**THEN** display error "Failed to access KV: [error details]"
**AND** suggest checking wrangler authentication
**AND** exit with non-zero code

#### Scenario: API server unreachable
**GIVEN** API server is down
**WHEN** running `cargo xtask portfolio post add ...`
**THEN** display error "API server unreachable"
**AND** suggest checking API_BASE_URL
**AND** exit with non-zero code

#### Scenario: Missing required flag
**GIVEN** user omits required flag
**WHEN** running `cargo xtask portfolio project add test-id --title "Test"`
**THEN** display error "Missing required flag: --description"
**AND** show usage help
**AND** exit with non-zero code

### Requirement: CLI Environment Configuration
CLI MUST support both local and remote KV access.

#### Scenario: Local development mode
**GIVEN** `.dev.vars` contains KV configuration
**WHEN** running CLI commands
**THEN** use local KV namespace for testing
**AND** display "[LOCAL]" prefix in output

#### Scenario: Remote production mode
**GIVEN** `--remote` flag is provided
**WHEN** running CLI commands
**THEN** use production KV namespace
**AND** display "[REMOTE]" prefix in output
**AND** require extra confirmation for destructive operations

