# Spec: git-deployment-hooks

## ADDED Requirements

### Requirement: A pre-commit git hook MUST automatically upload changed static assets to a private bucket and prevent them from being committed to git

A pre-commit hook SHALL detect changes to static assets in monitored directories, upload them to the configured private bucket, and prevent these files from being committed to the git repository.

#### Scenario: Hook detects changed static assets
- **Given** a developer has modified files in `/shared/styles/` or `www/public/`
- **When** they run `git commit`
- **Then** the pre-commit hook should detect the changed files
- **And** it should identify which files need to be uploaded
- **And** it should log the detected changes for transparency

#### Scenario: Hook uploads to private bucket
- **Given** changed static assets are detected
- **When** the pre-commit hook runs
- **Then** it should upload the files to the configured private bucket
- **And** it should preserve the directory structure in the bucket
- **And** it should use the correct bucket path prefix
- **And** it should verify successful upload before proceeding

#### Scenario: Hook prevents git commit of static assets
- **Given** files in `/shared/styles/` or `www/public/` are staged
- **When** the pre-commit hook runs after successful upload
- **Then** it should unstage these files from the commit
- **And** it should allow the commit to proceed without the static assets
- **And** it should display a message indicating files were uploaded to bucket

#### Scenario: Hook handles upload failures
- **Given** the bucket upload fails (network issue, auth failure, etc.)
- **When** the pre-commit hook attempts to upload
- **Then** it should display a clear error message
- **And** it should abort the commit
- **And** it should suggest troubleshooting steps (check credentials, network, bucket access)
- **And** the developer's changes should remain staged

#### Scenario: Hook skips upload if no changes to static assets
- **Given** a commit contains only code changes, no static assets
- **When** the pre-commit hook runs
- **Then** it should detect no static asset changes
- **And** it should skip the upload process
- **And** it should allow the commit to proceed normally

### Requirement: A post-pull git hook MUST automatically download the latest static assets from the private bucket after pulling changes

A post-merge hook SHALL execute after successful git pull operations to download the latest static assets from the bucket and update local directories automatically.

#### Scenario: Hook runs after git pull
- **Given** a developer runs `git pull`
- **When** the pull completes successfully
- **Then** the post-pull hook should automatically execute
- **And** it should log that it's fetching assets from the bucket

#### Scenario: Hook downloads latest assets
- **Given** the post-pull hook is running
- **When** it connects to the private bucket
- **Then** it should download files from the configured bucket paths
- **And** it should place `/shared/styles/` files in the local `/shared/styles/` directory
- **And** it should place `www/public/` files in the local `www/public/` directory
- **And** it should overwrite local files with bucket versions

#### Scenario: Hook creates directories if needed
- **Given** the post-pull hook is downloading assets
- **When** target directories don't exist locally
- **Then** it should create `/shared/` and `/shared/styles/` if needed
- **And** it should create `www/public/` if needed
- **And** it should set appropriate permissions

#### Scenario: Hook handles download failures gracefully
- **Given** the bucket download fails (network issue, auth failure, etc.)
- **When** the post-pull hook attempts to download
- **Then** it should display a warning message
- **And** it should not fail the entire pull operation
- **And** it should suggest manual download or troubleshooting
- **And** existing local files should remain unchanged

#### Scenario: Hook syncs only changed files
- **Given** the post-pull hook is running
- **When** checking for updates
- **Then** it should compare timestamps or checksums to detect changes
- **And** it should download only files that have been updated
- **And** it should skip unchanged files for efficiency

### Requirement: Git hooks MUST be easy to install and configure for new developers

Installation scripts and clear documentation SHALL enable new developers to set up git hooks quickly with minimal friction, using secure credential configuration.

#### Scenario: Hook installation script exists
- **Given** a new developer clones the repository
- **When** they need to set up hooks
- **Then** a script (e.g., `scripts/setup-hooks.sh`) should exist
- **And** the script should copy hooks to `.git/hooks/`
- **And** the script should set execute permissions on hooks
- **And** the README should document the setup process

#### Scenario: Bucket configuration is secure
- **Given** hooks need bucket credentials
- **When** a developer sets up the repository
- **Then** credentials should NOT be hardcoded in hook scripts
- **And** credentials should be read from environment variables or git config
- **And** documentation should explain how to configure credentials
- **And** a `.env.example` or similar should show required variables

#### Scenario: Hooks are executable
- **Given** hooks are installed in `.git/hooks/`
- **When** git tries to run them
- **Then** pre-commit and post-merge (for pull) hooks should have execute permissions
- **And** hooks should have proper shebang (`#!/bin/bash` or `#!/bin/sh`)
- **And** hooks should work on macOS, Linux, and Windows (Git Bash)

### Requirement: Hook scripts MUST be well-structured, documented, and easy to maintain

Hook scripts SHALL follow best practices for shell scripting with clear structure, comprehensive documentation, logging, error handling, and testability.

#### Scenario: Scripts have clear structure
- **Given** a hook script
- **When** a developer reads it
- **Then** it should have a clear header with purpose and usage
- **And** it should use functions for major steps (detect_changes, upload_to_bucket, etc.)
- **And** it should have error handling for common failure cases
- **And** it should use meaningful variable names

#### Scenario: Scripts log operations
- **Given** a hook is running
- **When** it performs operations
- **Then** it should output informative messages to the console
- **And** it should distinguish between info, warning, and error messages
- **And** it should show progress for long operations (uploads/downloads)

#### Scenario: Scripts are testable
- **Given** hook scripts exist
- **When** testing is needed
- **Then** functions should be testable in isolation
- **And** dry-run mode should be available for testing without side effects
- **And** scripts should support environment variables for test configuration

### Requirement: The bucket MUST follow a consistent path structure for organizing assets

The bucket SHALL use a logical, well-documented path structure that supports multiple apps, avoids naming conflicts, and uses URL-friendly conventions.

#### Scenario: Path structure is documented
- **Given** the project documentation
- **When** a developer needs to understand bucket organization
- **Then** the bucket path structure should be clearly documented
- **And** examples should show how local paths map to bucket paths
- **And** the structure should be logical and consistent

#### Scenario: Paths support multiple apps
- **Given** the monorepo contains multiple apps
- **When** assets are uploaded to the bucket
- **Then** shared styles should use a path like `/assets/shared/styles/`
- **And** app-specific assets should use paths like `/assets/www/public/`
- **And** paths should avoid naming conflicts between apps

#### Scenario: Paths are URL-friendly
- **Given** assets are accessed via URLs from the bucket
- **When** constructing import paths
- **Then** bucket paths should be valid URL paths
- **And** paths should not contain spaces or special characters
- **And** paths should be case-sensitive-safe

### Requirement: Hooks MUST provide clear error messages and recovery options when failures occur

Error handling SHALL provide informative messages, retry logic for transient failures, and clear guidance for manual intervention when automated recovery is not possible.

#### Scenario: Network failures are handled
- **Given** a network issue prevents bucket access
- **When** a hook attempts bucket operations
- **Then** it should retry with exponential backoff (2-3 attempts)
- **And** it should provide a clear error message after retries fail
- **And** it should suggest checking network connectivity

#### Scenario: Authentication failures are handled
- **Given** bucket credentials are missing or invalid
- **When** a hook attempts bucket operations
- **Then** it should detect authentication errors specifically
- **And** it should suggest checking credential configuration
- **And** it should reference documentation for credential setup

#### Scenario: Manual override is available
- **Given** a developer needs to bypass hook behavior
- **When** they have a legitimate reason (emergency, testing, etc.)
- **Then** they should be able to use `git commit --no-verify` to skip hooks
- **And** documentation should explain when this is appropriate
- **And** hooks should log when they are bypassed

### Requirement: Hooks MUST be fast and not significantly slow down git operations

Hook execution SHALL be optimized for speed by detecting changes quickly, skipping unnecessary operations, and using parallel uploads/downloads when appropriate.

#### Scenario: Pre-commit hook is fast
- **Given** a commit with changes to static assets
- **When** the pre-commit hook runs
- **Then** it should complete within 10 seconds for typical file sizes (< 1MB total)
- **And** it should show progress for long uploads
- **And** it should upload files in parallel if multiple files changed

#### Scenario: Post-pull hook is non-blocking
- **Given** a git pull operation completes
- **When** the post-pull hook runs
- **Then** it should complete within 15 seconds for typical downloads
- **And** it should not prevent the developer from continuing work
- **And** it should run in the background if possible

#### Scenario: Hooks skip unnecessary work
- **Given** no changes to static assets
- **When** hooks run
- **Then** they should detect this quickly (< 1 second)
- **And** they should exit early without bucket operations
- **And** they should output a brief message indicating no action needed
