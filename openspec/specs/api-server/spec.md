# api-server Specification

## Purpose
TBD - created by archiving change implement-api-backend. Update Purpose after archive.
## Requirements
### Requirement: Workers-rs API Runtime
The API MUST run on Cloudflare Workers using the workers-rs crate, deployed at api.werdxz.info subdomain.

#### Scenario: Health check endpoint
**GIVEN** the API is deployed
**WHEN** a GET request is made to `/health`
**THEN** respond with 200 OK and `{"status": "healthy", "version": "1.0.0"}`

#### Scenario: CORS for werdxz subdomains
**GIVEN** a request from blog.werdxz.info
**WHEN** the request includes an Origin header
**THEN** respond with Access-Control-Allow-Origin header matching the origin
**AND** include Access-Control-Allow-Methods: GET, OPTIONS
**AND** include Access-Control-Max-Age: 86400

#### Scenario: OPTIONS preflight requests
**GIVEN** a browser makes a preflight OPTIONS request
**WHEN** the request is to any API endpoint
**THEN** respond with 204 No Content and appropriate CORS headers

### Requirement: OpenAPI 3.0 Specification
The API MUST serve an OpenAPI 3.0 specification document describing all endpoints, parameters, and response schemas.

#### Scenario: Retrieve API specification
**GIVEN** the API is deployed
**WHEN** a GET request is made to `/openapi.json`
**THEN** respond with 200 OK and valid OpenAPI 3.0 JSON
**AND** include all endpoint definitions
**AND** include request/response schemas

### Requirement: Error Response Format
All error responses MUST follow a consistent JSON structure with appropriate HTTP status codes.

#### Scenario: Resource not found
**GIVEN** a request for a non-existent resource
**WHEN** the resource cannot be found
**THEN** respond with 404 Not Found
**AND** JSON body: `{"error": {"code": "NOT_FOUND", "message": "<description>", "request_id": "<id>"}}`

#### Scenario: Invalid query parameters
**GIVEN** a request with invalid query parameters
**WHEN** parameter validation fails
**THEN** respond with 400 Bad Request
**AND** JSON body describing the validation error

### Requirement: D1 Database Configuration
The API MUST connect to a D1 database named `werdxz-api-db` for metadata storage.

#### Scenario: Database connection binding
**GIVEN** the worker is deployed with D1 binding
**WHEN** the worker starts
**THEN** successfully bind to the werdxz-api-db database
**AND** be able to execute SQL queries

### Requirement: Durable Objects Configuration
The API MUST use Durable Objects for content storage with namespace `werdxz-content-store`.

#### Scenario: Durable Object namespace binding
**GIVEN** the worker is deployed with DO binding
**WHEN** accessing content storage
**THEN** successfully create/access Durable Object instances
**AND** store/retrieve content data

### Requirement: Request ID Tracking
Every API request MUST have a unique request ID for debugging and logging.

#### Scenario: Request ID in responses
**GIVEN** any API request
**WHEN** the response is generated
**THEN** include X-Request-ID header with unique identifier
**AND** include request_id in error responses

### Requirement: Rate Limiting Headers
Responses SHALL include rate limit information headers (preparatory for future rate limiting).

#### Scenario: Rate limit headers present
**GIVEN** any successful API response
**WHEN** the response is sent
**THEN** include X-RateLimit-Limit header (e.g., "1000")
**AND** include X-RateLimit-Remaining header
**AND** include X-RateLimit-Reset header with Unix timestamp

