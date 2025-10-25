# Root Endpoint
GET {{base_url}}/

HTTP 200
[Asserts]
header "Content-Type" contains "application/json"
header "Access-Control-Allow-Origin" exists
header "X-Request-ID" exists
jsonpath "$.service" == "werdxz-api"
jsonpath "$.version" == "0.1.0"
jsonpath "$.api_version" == "v1"
jsonpath "$.endpoints.health" == "/v1/health"
jsonpath "$.endpoints.posts" == "/v1/posts"
jsonpath "$.endpoints.resume" == "/v1/resume"
jsonpath "$.endpoints.openapi" == "/openapi.json"


# Health Check
GET {{base_url}}/v1/health

HTTP 200
[Asserts]
header "Content-Type" contains "application/json"
header "Access-Control-Allow-Origin" == "*"
header "X-Request-ID" exists
jsonpath "$.status" == "healthy"
jsonpath "$.timestamp" exists


# Posts - Empty List (no data yet)
GET {{base_url}}/v1/posts

HTTP 200
[Asserts]
header "Content-Type" contains "application/json"
header "Access-Control-Allow-Origin" == "*"
header "X-Request-ID" exists
jsonpath "$.posts" isCollection
jsonpath "$.posts" count == 0
jsonpath "$.pagination.page" == 1
jsonpath "$.pagination.limit" == 10
jsonpath "$.pagination.total" == 0
jsonpath "$.pagination.has_next" == false


# Posts - With Pagination Parameters
GET {{base_url}}/v1/posts?page=2&limit=5

HTTP 200
[Asserts]
jsonpath "$.pagination.page" == 2
jsonpath "$.pagination.limit" == 5


# Posts - With Sort Parameters
GET {{base_url}}/v1/posts?sort=title&order=asc

HTTP 200
[Asserts]
jsonpath "$.posts" isCollection


# Resume - Full
GET {{base_url}}/v1/resume

HTTP 200
[Asserts]
header "Content-Type" contains "application/json"
header "Access-Control-Allow-Origin" == "*"
header "Cache-Control" == "public, max-age=3600"
header "X-Request-ID" exists
jsonpath "$['$schema']" exists
jsonpath "$.personal" exists
jsonpath "$.experience" isCollection
jsonpath "$.education" isCollection
jsonpath "$.projects" isCollection
jsonpath "$.extracurricular" isCollection


# Resume - Section Filtering (experience only)
GET {{base_url}}/v1/resume?sections=experience

HTTP 200
[Asserts]
jsonpath "$.personal" not exists
jsonpath "$.experience" isCollection
jsonpath "$.education" not exists
jsonpath "$.projects" not exists
jsonpath "$.extracurricular" not exists


# Resume - Multiple Sections
GET {{base_url}}/v1/resume?sections=personal,experience,education

HTTP 200
[Asserts]
jsonpath "$.personal" exists
jsonpath "$.experience" isCollection
jsonpath "$.education" isCollection
jsonpath "$.projects" not exists
jsonpath "$.extracurricular" not exists


# Resume - Limit Items
GET {{base_url}}/v1/resume?limit=2

HTTP 200
[Asserts]
jsonpath "$.experience" count <= 2
jsonpath "$.education" count <= 2
jsonpath "$.projects" count <= 2
jsonpath "$.extracurricular" count <= 2


# Resume - Tag Filtering (rust)
GET {{base_url}}/v1/resume?tags=rust

HTTP 200
[Asserts]
jsonpath "$.experience" isCollection
jsonpath "$.projects" isCollection
jsonpath "$.experience" count >= 1
jsonpath "$.projects" count >= 1


# Resume - Minimal Format
GET {{base_url}}/v1/resume?format=minimal

HTTP 200
[Asserts]
jsonpath "$.experience[0]" exists
jsonpath "$.experience[0].title" exists
jsonpath "$.experience[0].organization" exists
# Bullets and description should be empty/missing in minimal format


# Resume - Combined Filters
GET {{base_url}}/v1/resume?sections=experience,projects&tags=rust&limit=3

HTTP 200
[Asserts]
jsonpath "$.personal" not exists
jsonpath "$.education" not exists
jsonpath "$.extracurricular" not exists
jsonpath "$.experience" count <= 3
jsonpath "$.projects" count <= 3


# Resume - Section personal only
GET {{base_url}}/v1/resume?sections=personal

HTTP 200
[Asserts]
jsonpath "$.personal" exists
jsonpath "$.personal.firstName" exists
jsonpath "$.personal.lastName" exists
jsonpath "$.personal.email" exists
jsonpath "$.experience" not exists
jsonpath "$.education" not exists
jsonpath "$.projects" not exists
jsonpath "$.extracurricular" not exists


# Resume - Projects section with structure check
GET {{base_url}}/v1/resume?sections=projects

HTTP 200
[Asserts]
jsonpath "$.projects" isCollection
jsonpath "$.projects[0].title" exists
jsonpath "$.projects[0].date" exists
jsonpath "$.projects[0].status" exists


# CORS Preflight - OPTIONS
OPTIONS {{base_url}}/v1/resume

HTTP 204
[Asserts]
header "Access-Control-Allow-Origin" exists
header "Access-Control-Allow-Methods" contains "GET"
header "Access-Control-Allow-Methods" contains "OPTIONS"
header "Access-Control-Allow-Headers" contains "Content-Type"
header "Access-Control-Max-Age" == "86400"


# 404 - Unknown Route
GET {{base_url}}/unknown

HTTP 404


# 404 - Post Not Found
GET {{base_url}}/v1/posts/non-existent-slug

HTTP 404
[Asserts]
jsonpath "$.error.code" == "NOT_FOUND"
