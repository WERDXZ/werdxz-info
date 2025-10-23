# resume-deployment Specification

## Purpose
TBD - created by archiving change implement-resume-page. Update Purpose after archive.
## Requirements
### Requirement: The resume app MUST have Cloudflare Workers configuration for deployment

The resume application SHALL include a wrangler.toml configuration file that specifies deployment settings for Cloudflare Workers, including worker name, assets directory, and compatibility date.

#### Scenario: Wrangler configuration exists
**Given** the resume/ directory
**When** checking for deployment config
**Then** a wrangler.toml file exists at resume/wrangler.toml
**And** it specifies name = "werdxz-resume"
**And** it configures assets directory pointing to public/
**And** it includes compatibility_date

#### Scenario: Deployment command works
**Given** the resume app with wrangler.toml
**When** running `npm run deploy` in resume/ directory
**Then** the command executes `wrangler deploy`
**And** deployment succeeds without errors
**And** the site is accessible at the configured route

### Requirement: The resume page MUST be accessible at resume.werdxz.info subdomain

The resume application SHALL be deployed and accessible via the resume.werdxz.info subdomain with proper DNS configuration and SSL/TLS certificate, running as an isolated Cloudflare Worker.

#### Scenario: Subdomain routing
**Given** DNS configuration and Cloudflare setup
**When** navigating to https://resume.werdxz.info
**Then** the resume page loads successfully
**And** SSL/TLS certificate is valid
**And** no redirect to a different domain occurs

#### Scenario: Isolated from main site
**Given** the resume subdomain
**When** comparing to main site at werdxz.info
**Then** resume app runs as separate Cloudflare Worker
**And** has its own deployment configuration
**And** can be deployed independently

### Requirement: The resume app MUST have a clear directory structure for static files mirroring the www app organization

The resume application SHALL organize static files in a public/ directory following the same conventions as the www app, enabling consistent development and deployment workflows.

#### Scenario: Public directory structure
**Given** the resume/ app directory
**When** examining the file structure
**Then** a resume/public/ directory contains all static files
**And** resume/public/ includes: index.html, robots.txt, CSS files, JS files
**And** the structure mirrors www/ app organization

#### Scenario: Development server
**Given** the resume/ directory with package.json
**When** running `npm run dev`
**Then** a local development server starts
**And** serves files from resume/public/
**And** enables local testing before deployment

### Requirement: The resume app MUST have npm package configuration for development and deployment scripts

The resume application SHALL include a package.json file with npm scripts for local development server and Cloudflare deployment, along with necessary dependencies.

#### Scenario: Package.json exists
**Given** the resume/ directory
**When** checking for package files
**Then** a package.json file exists
**And** it includes dev script for local development
**And** it includes deploy script running wrangler deploy
**And** wrangler is listed as a dependency or devDependency

#### Scenario: Node modules and dependencies
**Given** the resume package.json
**When** running npm install
**Then** required dependencies are installed
**And** wrangler CLI is available
**And** development server tools are available if needed

### Requirement: The resume deployment process MUST be documented in README files for future updates

The resume application SHALL have documentation explaining local development, deployment process, and subdomain configuration in both the app-specific README and the monorepo root README.

#### Scenario: README includes deployment steps
**Given** the resume/ directory
**When** checking documentation
**Then** a README.md exists explaining the resume app
**And** it documents how to run locally (`npm run dev`)
**And** it documents how to deploy (`npm run deploy`)
**And** it explains the subdomain configuration

#### Scenario: Root README updated
**Given** the root README.md
**When** checking monorepo documentation
**Then** the resume app is listed in the structure section
**And** subdomain (resume.werdxz.info) is documented
**And** tech stack (custom elements, client-side) is noted

### Requirement: The resume app MUST follow the same pattern as www for untracked public files with manual sync

The resume application SHALL exclude public/ files from git tracking and deploy them directly from the local file system using Wrangler, consistent with the www app pattern.

#### Scenario: Public files are untracked
**Given** the resume/public/ directory
**When** checking git status
**Then** HTML, CSS, JS files in resume/public/ are not tracked in git
**And** .gitignore includes appropriate patterns
**And** files are synced manually using sync scripts

#### Scenario: Deployment with latest files
**Given** resume public files updated locally
**When** deploying with `npm run deploy`
**Then** wrangler deploys files from resume/public/ directory
**And** latest local changes are included
**And** deployment succeeds with current content

### Requirement: The resume app MUST be independently configurable and deployable from other apps in the monorepo

The resume application SHALL have its own wrangler configuration, worker name, and deployment settings that operate independently without interfering with other applications.

#### Scenario: Separate wrangler config
**Given** multiple apps in the monorepo
**When** deploying the resume app
**Then** it uses resume/wrangler.toml configuration
**And** doesn't interfere with www/ deployment
**And** has its own Worker name and routes

