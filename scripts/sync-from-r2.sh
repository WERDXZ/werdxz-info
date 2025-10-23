#!/bin/bash
#
# Download Static Assets from R2
#
# Downloads www/public/ files from the private R2 bucket.
# Run this manually when you need to sync files locally.
#
# Note: shared/styles/ is in git (version controlled), not downloaded from R2
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

# Function to list all objects in bucket with prefix
list_bucket_objects() {
    local bucket="$1"
    local prefix="$2"

    # Check if wrangler is available
    local wrangler_cmd="npx wrangler"
    if ! command -v npx &> /dev/null; then
        if ! command -v wrangler &> /dev/null; then
            log_error "Neither npx nor wrangler found"
            return 1
        fi
        wrangler_cmd="wrangler"
    fi

    # List objects (wrangler r2 object list doesn't support prefix filtering well)
    # So we'll just try to download known paths
    echo ""
}

# Main execution
main() {
    log_info "Downloading static assets from R2..."

    # Check if wrangler is available
    local wrangler_cmd="npx wrangler"
    if ! command -v npx &> /dev/null; then
        if ! command -v wrangler &> /dev/null; then
            log_warning "Neither npx nor wrangler found"
            log_info "Install Node.js and wrangler to download files"
            exit 1
        fi
        wrangler_cmd="wrangler"
    fi

    # Create directory if it doesn't exist
    mkdir -p www/public

    # Download known files (add more as needed)
    local download_files=(
        "www/public/index.html"
        # Add more files here as needed
    )

    local download_count=0
    local fail_count=0

    for r2_path in "${download_files[@]}"; do
        local local_path="$r2_path"

        log_info "  - Downloading $r2_path"

        # Create directory if needed
        local dir=$(dirname "$local_path")
        mkdir -p "$dir"

        # Download from R2
        if $wrangler_cmd r2 object get "private/$r2_path" --file="$local_path" --remote 2>/dev/null; then
            ((download_count++))
        else
            log_warning "Failed to download $r2_path (file may not exist)"
            ((fail_count++))
        fi
    done

    # Summary
    log_info "Download complete: $download_count succeeded, $fail_count failed"

    if [[ $download_count -eq 0 ]]; then
        log_error "No files downloaded"
        exit 1
    fi

    log_success "Files synced from R2"
    exit 0
}

# Run main function
main
