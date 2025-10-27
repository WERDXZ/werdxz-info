# Spec: Projects API

## ADDED Requirements

### Requirement: D1 Database Schema for Projects
The API MUST store project metadata in D1 with normalized schema for projects, tags, and URLs.

#### Scenario: Projects table structure
**GIVEN** the API is deployed
**WHEN** querying the D1 database
**THEN** `projects` table exists with columns: id (PRIMARY KEY), slug (UNIQUE), name, description, stage, open_to_contributors (BOOLEAN), readme_url, created_at, updated_at
**AND** slug has UNIQUE constraint
**AND** stage is constrained to valid values: 'planned', 'wip', 'active', 'maintained', 'archived', 'shelved'
**AND** index exists on slug for fast lookups

#### Scenario: Normalized tags table
**GIVEN** the API is deployed
**WHEN** querying the D1 database
**THEN** `tags` table exists with columns: id (AUTOINCREMENT PRIMARY KEY), name (UNIQUE)
**AND** tags are normalized to lowercase
**AND** tags are shared between blog posts and projects (same table)

#### Scenario: Project-Tags junction table
**GIVEN** the API is deployed
**WHEN** querying the D1 database
**THEN** `project_tags` table exists with columns: project_id (FK to projects.id), tag_id (FK to tags.id)
**AND** composite PRIMARY KEY on (project_id, tag_id)
**AND** FOREIGN KEY constraints with CASCADE DELETE
**AND** indexes on both project_id and tag_id

#### Scenario: Project URLs table
**GIVEN** the API is deployed
**WHEN** querying the D1 database
**THEN** `project_urls` table exists with columns: id (AUTOINCREMENT PRIMARY KEY), project_id (FK), label, url
**AND** label is human-readable (e.g., "GitHub", "Live Demo", "Documentation", "npm")
**AND** FOREIGN KEY constraint on project_id with CASCADE DELETE
**AND** index on project_id for efficient lookups

### Requirement: List Projects Endpoint
The API MUST provide an endpoint to list all projects with metadata.

#### Scenario: List all projects
**GIVEN** there are 5 projects in the database
**WHEN** GET /v1/projects is requested
**THEN** respond with 200 OK
**AND** return array of projects with: id, slug, name, description, stage, open_to_contributors, tags, urls
**AND** tags is array of tag names
**AND** urls is array of {label, url} objects
**AND** projects ordered by created_at descending (newest first)
**AND** return all projects (no pagination - expected max ~20 projects)

#### Scenario: Empty projects list
**GIVEN** no projects exist
**WHEN** GET /v1/projects is requested
**THEN** respond with 200 OK
**AND** return empty array `[]`

### Requirement: Get Single Project Endpoint
The API MUST provide an endpoint to retrieve a single project by slug.

#### Scenario: Retrieve project by slug
**GIVEN** a project with slug "my-project" exists
**WHEN** GET /v1/projects/my-project is requested
**THEN** respond with 200 OK
**AND** return project with full metadata: id, slug, name, description, stage, open_to_contributors, readme_url, tags, urls, created_at, updated_at
**AND** tags is array of tag names
**AND** urls is array of {label, url} objects

#### Scenario: Project not found
**GIVEN** no project with slug "nonexistent" exists
**WHEN** GET /v1/projects/nonexistent is requested
**THEN** respond with 404 Not Found
**AND** error code "NOT_FOUND"
**AND** message "Project with slug 'nonexistent' not found"

### Requirement: Tag Normalization
Project tags MUST be normalized to lowercase for consistent filtering.

#### Scenario: Tag storage normalization
**GIVEN** a project is created with tags ["Rust", "TypeScript", "Web"]
**WHEN** storing tags in the database
**THEN** normalize all tags to lowercase: ["rust", "typescript", "web"]
**AND** insert into `tags` table if not exists
**AND** create associations in `project_tags` junction table

#### Scenario: Tag query normalization
**GIVEN** a request to filter by tags
**WHEN** GET /v1/projects?tags=Rust,TYPESCRIPT is requested
**THEN** normalize query tags to lowercase before matching
**AND** return projects matching any of the normalized tags

### Requirement: URL Management
Projects MUST support multiple URLs with custom labels.

#### Scenario: Store multiple URLs
**GIVEN** a project has GitHub, demo, and docs URLs
**WHEN** inserting the project
**THEN** store each URL in `project_urls` table
**AND** associate with project_id
**AND** preserve custom labels (e.g., "Production Demo", "Staging Demo")

#### Scenario: Retrieve URLs with project
**GIVEN** a project has 3 URLs
**WHEN** fetching the project
**THEN** include all URLs in response
**AND** format as array: `[{label: "GitHub", url: "..."}, ...]`
**AND** order URLs by insertion order (id ascending)

### Requirement: OpenAPI Documentation
Projects endpoints MUST be documented in OpenAPI spec.

#### Scenario: Projects endpoints in OpenAPI
**GIVEN** the API serves OpenAPI spec at /openapi.json
**WHEN** fetching the spec
**THEN** include `/v1/projects` GET endpoint (no query parameters)
**AND** include `/v1/projects/{slug}` GET endpoint
**AND** include schema definitions for Project, ProjectUrl, Tag
**AND** document response codes: 200, 404, 500

### Requirement: Error Handling
The API MUST handle errors gracefully.

#### Scenario: Database connection error
**GIVEN** D1 database is unavailable
**WHEN** GET /v1/projects is requested
**THEN** respond with 500 Internal Server Error
**AND** error code "INTERNAL_ERROR"
**AND** message "Unable to load projects"

