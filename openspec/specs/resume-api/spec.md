# resume-api Specification

## Purpose
TBD - created by archiving change implement-api-backend. Update Purpose after archive.
## Requirements
### Requirement: Resume Data Retrieval
The API MUST fetch resume data from the cloud bucket and provide filtering capabilities via query parameters.

#### Scenario: Get full resume
**GIVEN** resume.json exists in cloud bucket
**WHEN** GET /resume is requested with no parameters
**THEN** respond with 200 OK
**AND** return complete resume JSON from cloud.werdxz.info/resume/public/resume.json
**AND** include all sections: personal, education, experience, projects, extracurricular

#### Scenario: Cloud bucket fetch failure
**GIVEN** the cloud bucket is unavailable
**WHEN** GET /resume is requested
**THEN** respond with 503 Service Unavailable
**AND** error code "UPSTREAM_ERROR"
**AND** message indicating resume data unavailable

### Requirement: Section Filtering
The API MUST allow filtering resume sections via query parameter.

#### Scenario: Filter to specific sections
**GIVEN** full resume data exists
**WHEN** GET /resume?sections=experience,education is requested
**THEN** respond with 200 OK
**AND** return only personal, experience, and education sections
**AND** exclude projects and extracurricular sections
**AND** always include personal section (not filterable)

#### Scenario: Invalid section name
**GIVEN** a request with invalid section
**WHEN** GET /resume?sections=invalid is requested
**THEN** respond with 400 Bad Request
**AND** error indicating invalid section name
**AND** list valid section names: education, experience, projects, extracurricular

#### Scenario: Empty sections parameter
**GIVEN** a request with empty sections
**WHEN** GET /resume?sections= is requested
**THEN** respond with full resume (treat as no filter)

### Requirement: Technology Tag Filtering
The API MUST allow filtering resume items by technology tags.

#### Scenario: Filter by single tag
**GIVEN** resume items tagged with various technologies
**WHEN** GET /resume?tags=rust is requested
**THEN** respond with 200 OK
**AND** filter experience items to those with "rust" in tags array
**AND** filter projects to those with "rust" in tags
**AND** filter extracurricular to those with "rust" in tags
**AND** always include education (no tag filtering)

#### Scenario: Filter by multiple tags (OR logic)
**GIVEN** resume items with various tags
**WHEN** GET /resume?tags=rust,typescript is requested
**THEN** return items tagged with "rust" OR "typescript"
**AND** match is case-insensitive

#### Scenario: No matching tags
**GIVEN** a request for tags with no matches
**WHEN** GET /resume?tags=nonexistent is requested
**THEN** respond with 200 OK
**AND** return personal and education sections
**AND** return empty arrays for experience, projects, extracurricular

### Requirement: Output Format Options
The API MUST support different output formats for various use cases.

#### Scenario: Minimal format
**GIVEN** a request for minimal format
**WHEN** GET /resume?format=minimal is requested
**THEN** respond with 200 OK
**AND** exclude description and bullets fields from all items
**AND** include only: title, organization, dates, tags for experience
**AND** include only: title, date, status, tags for projects

#### Scenario: Full format (default)
**GIVEN** a request without format parameter
**WHEN** GET /resume is requested
**THEN** return full resume with all fields including descriptions and bullets

#### Scenario: Invalid format
**GIVEN** a request with unsupported format
**WHEN** GET /resume?format=invalid is requested
**THEN** respond with 400 Bad Request
**AND** error listing valid formats: full, minimal

### Requirement: Item Limit Per Section
The API MUST allow limiting the number of items returned per section.

#### Scenario: Limit items per section
**GIVEN** a resume with 10 experience items
**WHEN** GET /resume?limit=3 is requested
**THEN** respond with 200 OK
**AND** return at most 3 items per section (experience, projects, extracurricular)
**AND** items ordered by date descending (most recent first)

#### Scenario: Limit with section filter
**GIVEN** a resume with multiple items
**WHEN** GET /resume?sections=experience&limit=2 is requested
**THEN** return only experience section
**AND** limit experience items to 2 most recent

#### Scenario: Invalid limit
**GIVEN** a request with invalid limit
**WHEN** GET /resume?limit=0 is requested
**THEN** respond with 400 Bad Request
**AND** error indicating limit must be positive integer

### Requirement: Combined Filtering
The API MUST support combining multiple filters.

#### Scenario: Section + tag filtering
**GIVEN** full resume data
**WHEN** GET /resume?sections=experience,projects&tags=rust is requested
**THEN** return personal section
**AND** return experience items tagged with "rust"
**AND** return projects tagged with "rust"
**AND** exclude education and extracurricular

#### Scenario: Section + tag + limit
**GIVEN** multiple matching items
**WHEN** GET /resume?sections=projects&tags=rust&limit=2 is requested
**THEN** return personal section
**AND** return 2 most recent projects tagged with "rust"

#### Scenario: All filters + minimal format
**GIVEN** full resume data
**WHEN** GET /resume?sections=experience&tags=rust&limit=3&format=minimal is requested
**THEN** return 3 most recent rust-tagged experience items
**AND** exclude description and bullets fields
**AND** include only title, organization, dates, tags

### Requirement: Caching Headers
Resume responses SHALL include appropriate caching headers since data changes infrequently.

#### Scenario: Cache-Control headers
**GIVEN** any successful resume response
**WHEN** the response is sent
**THEN** include Cache-Control: public, max-age=300 (5 minutes)
**AND** include ETag header based on content hash

#### Scenario: Conditional requests
**GIVEN** a request with If-None-Match header
**WHEN** the ETag matches current content
**THEN** respond with 304 Not Modified
**AND** no body content

