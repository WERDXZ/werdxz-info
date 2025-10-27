# projects-cli Specification

## Purpose
TBD - created by archiving change implement-projects-feature. Update Purpose after archive.
## Requirements
### Requirement: Project Add Command
The CLI MUST support adding new projects via xtask.

#### Scenario: Add project with minimal fields
**GIVEN** a user wants to add a project
**WHEN** running `cargo xtask project add --name "My Project" --slug "my-project" --description "A cool project" --stage "active"`
**THEN** insert project into D1 projects table
**AND** generate UUID for project id
**AND** set created_at to current timestamp
**AND** set open_to_contributors to false by default
**AND** respond with success message including project slug

#### Scenario: Add project with all optional fields
**GIVEN** a user provides full project metadata
**WHEN** running with flags: `--readme-url`, `--open-to-contributors`, `--tags`, `--urls`
**THEN** insert project with all provided fields
**AND** parse tags as comma-separated values
**AND** normalize tags to lowercase
**AND** insert tags into tags table (if new)
**AND** create project_tags associations
**AND** parse URLs as `label=url` pairs
**AND** insert URLs into project_urls table

#### Scenario: Duplicate slug error
**GIVEN** a project with slug "my-project" exists
**WHEN** running `cargo xtask project add --slug "my-project"`
**THEN** fail with error "Project with slug 'my-project' already exists"
**AND** exit with non-zero status code

#### Scenario: Invalid slug format
**GIVEN** a user provides an invalid slug
**WHEN** running with `--slug "My Project!"` (contains uppercase and special chars)
**THEN** fail with error "Slug must be lowercase alphanumeric with hyphens only"
**AND** suggest normalized slug: "my-project"

#### Scenario: Invalid stage value
**GIVEN** a user provides an invalid stage
**WHEN** running with `--stage "unknown"`
**THEN** fail with error listing valid stages
**AND** valid stages are: planned, wip, active, maintained, archived, shelved

### Requirement: Project List Command
The CLI MUST support listing all projects.

#### Scenario: List all projects
**GIVEN** there are 3 projects in the database
**WHEN** running `cargo xtask project list`
**THEN** display table with columns: slug, name, stage, tags
**AND** order by created_at descending
**AND** show success message with count

#### Scenario: Empty list
**GIVEN** no projects exist
**WHEN** running `cargo xtask project list`
**THEN** display message "No projects found"
**AND** exit with zero status code

### Requirement: Project Update Command
The CLI MUST support updating existing projects.

#### Scenario: Update project fields
**GIVEN** a project with slug "my-project" exists
**WHEN** running `cargo xtask project update my-project --name "Updated Name" --description "New description"`
**THEN** update specified fields in database
**AND** set updated_at to current timestamp
**AND** preserve unspecified fields
**AND** respond with success message

#### Scenario: Update tags
**GIVEN** a project with tags ["rust", "cli"]
**WHEN** running `cargo xtask project update my-project --tags "rust,web,cli"`
**THEN** delete old project_tags associations
**AND** insert new tags into tags table (if needed)
**AND** create new project_tags associations
**AND** normalize tags to lowercase

#### Scenario: Update URLs
**GIVEN** a project with existing URLs
**WHEN** running with `--urls "GitHub=https://...,Demo=https://..."`
**THEN** delete old project_urls entries
**AND** insert new URLs
**AND** preserve URL order

#### Scenario: Update nonexistent project
**GIVEN** no project with slug "nonexistent" exists
**WHEN** running `cargo xtask project update nonexistent --name "New Name"`
**THEN** fail with error "Project 'nonexistent' not found"
**AND** suggest running `cargo xtask project list` to see available projects

### Requirement: Project Delete Command
The CLI MUST support deleting projects.

#### Scenario: Delete project
**GIVEN** a project with slug "my-project" exists
**WHEN** running `cargo xtask project delete my-project`
**THEN** prompt for confirmation: "Delete project 'my-project'? (y/N)"
**AND** if confirmed, delete from projects table
**AND** CASCADE DELETE removes project_tags and project_urls
**AND** respond with "Project 'my-project' deleted"

#### Scenario: Force delete without confirmation
**GIVEN** a project exists
**WHEN** running `cargo xtask project delete my-project --force`
**THEN** delete without prompting
**AND** respond with success message

#### Scenario: Delete nonexistent project
**GIVEN** no project with slug "nonexistent" exists
**WHEN** running `cargo xtask project delete nonexistent`
**THEN** fail with error "Project 'nonexistent' not found"

### Requirement: Remote Database Support
The CLI MUST support both local and remote D1 databases.

#### Scenario: Local database operations
**GIVEN** no `--remote` flag is provided
**WHEN** running any project command
**THEN** execute against local D1 database (wrangler dev)
**AND** use local connection string

#### Scenario: Remote database operations
**GIVEN** `--remote` flag is provided
**WHEN** running `cargo xtask project add my-project --remote`
**THEN** execute against production D1 database
**AND** use wrangler CLI with `--remote` flag
**AND** require wrangler authentication

#### Scenario: Database connection error
**GIVEN** D1 database is unavailable
**WHEN** running any command
**THEN** fail with error "Unable to connect to database"
**AND** provide troubleshooting hint (check wrangler config)

### Requirement: Input Validation
The CLI MUST validate all user inputs.

#### Scenario: Required field validation
**GIVEN** a user runs add command without required fields
**WHEN** running `cargo xtask project add` (missing --name, --slug, etc.)
**THEN** fail with error listing required fields
**AND** show usage example

#### Scenario: URL format validation
**GIVEN** a user provides invalid URL
**WHEN** running with `--urls "GitHub=not-a-url"`
**THEN** fail with error "Invalid URL format for 'GitHub'"
**AND** URLs must start with http:// or https://

#### Scenario: Tag format validation
**GIVEN** a user provides tags with invalid characters
**WHEN** running with `--tags "rust, web dev"` (space in tag)
**THEN** normalize by replacing spaces with hyphens
**OR** warn and skip invalid tags

### Requirement: User Experience
The CLI MUST provide clear feedback and help.

#### Scenario: Help text
**GIVEN** a user wants to learn about project commands
**WHEN** running `cargo xtask project --help`
**THEN** display usage information
**AND** list all subcommands: add, list, update, delete
**AND** show example commands

#### Scenario: Success feedback
**GIVEN** a command executes successfully
**WHEN** adding/updating/deleting a project
**THEN** print confirmation message with relevant details
**AND** use color-coded output (green for success, red for errors)

#### Scenario: Progress indication
**GIVEN** a command involves multiple steps (insert tags, insert URLs)
**WHEN** executing the command
**THEN** show progress messages: "Inserting project...", "Adding tags...", "Adding URLs...", "Done!"

