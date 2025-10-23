#!/bin/bash
#
# Git Hooks Installation Script
#
# This script installs git hooks from scripts/hooks/ to .git/hooks/
# and sets the appropriate permissions.
#
# Usage:
#   ./scripts/setup-hooks.sh
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored messages
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in a git repository
check_git_repo() {
    if [[ ! -d .git ]]; then
        log_error "Not a git repository"
        log_info "Please run this script from the root of your git repository"
        exit 1
    fi
}

# Install hooks
install_hooks() {
    local hooks_source="scripts/hooks"
    local hooks_dest=".git/hooks"

    if [[ ! -d "$hooks_source" ]]; then
        log_error "Hooks source directory not found: $hooks_source"
        exit 1
    fi

    log_info "Installing git hooks..."

    # Copy each hook file
    local hooks_installed=0
    for hook_file in "$hooks_source"/*; do
        if [[ -f "$hook_file" ]]; then
            local hook_name=$(basename "$hook_file")
            local dest_file="$hooks_dest/$hook_name"

            # Backup existing hook if it exists
            if [[ -f "$dest_file" ]]; then
                log_warning "Backing up existing hook: $hook_name"
                mv "$dest_file" "$dest_file.backup.$(date +%Y%m%d%H%M%S)"
            fi

            # Copy hook
            cp "$hook_file" "$dest_file"

            # Set execute permissions
            chmod +x "$dest_file"

            log_success "Installed: $hook_name"
            ((hooks_installed++))
        fi
    done

    if [[ $hooks_installed -eq 0 ]]; then
        log_warning "No hooks found to install"
    else
        log_success "Installed $hooks_installed hook(s)"
    fi
}

# Verify hooks are executable
verify_hooks() {
    local hooks_dest=".git/hooks"
    local all_executable=true

    log_info "Verifying hook permissions..."

    for hook_file in "$hooks_dest"/pre-commit "$hooks_dest"/post-merge; do
        if [[ -f "$hook_file" ]]; then
            if [[ ! -x "$hook_file" ]]; then
                log_error "Hook is not executable: $(basename $hook_file)"
                all_executable=false
            else
                log_success "$(basename $hook_file) is executable"
            fi
        fi
    done

    if [[ "$all_executable" = false ]]; then
        log_error "Some hooks are not executable"
        exit 1
    fi
}

# Check for .env configuration
check_env_config() {
    log_info "Checking environment configuration..."

    if [[ ! -f .env ]]; then
        log_warning "No .env file found"
        log_info "Bucket upload/download will run in placeholder mode"
        log_info ""
        log_info "To configure bucket access:"
        log_info "  1. Copy .env.example to .env"
        log_info "  2. Fill in your bucket credentials"
        log_info "  3. Keep .env file secure and never commit it to git"
    else
        log_success "Found .env file"

        # Check for required variables
        source .env 2>/dev/null || true

        if [[ -z "$BUCKET_URL" ]]; then
            log_warning "BUCKET_URL not set in .env"
            log_info "Hooks will run in placeholder mode"
        else
            log_success "BUCKET_URL is configured"
        fi
    fi
}

# Display next steps
show_next_steps() {
    echo ""
    log_success "Git hooks installation complete!"
    echo ""
    log_info "Next steps:"
    echo ""
    echo "  1. Configure bucket credentials (if not already done):"
    echo "     - Copy .env.example to .env"
    echo "     - Set BUCKET_URL, BUCKET_ACCESS_KEY, and BUCKET_SECRET_KEY"
    echo ""
    echo "  2. Test the hooks:"
    echo "     - Make a change to shared/styles/variables.css"
    echo "     - Run: git add shared/styles/variables.css"
    echo "     - Run: git commit -m 'test hook'"
    echo "     - The hook should detect and handle the file"
    echo ""
    echo "  3. The pre-commit hook will now run automatically on git commit"
    echo ""
    echo "  4. To sync www/public/ files manually:"
    echo "     - Upload: ./scripts/sync-to-r2.sh"
    echo "     - Download: ./scripts/sync-from-r2.sh"
    echo ""
}

# Main execution
main() {
    log_info "Git Hooks Setup"
    echo ""

    # Check if we're in a git repository
    check_git_repo

    # Install hooks
    install_hooks

    # Verify hooks are executable
    verify_hooks

    # Check environment configuration
    check_env_config

    # Show next steps
    show_next_steps
}

# Run main function
main
