#!/bin/bash
#
# Sync Static Assets to R2
#
# Uploads all monitored files to R2 buckets:
#   - shared/ → cloud bucket (public CDN)
#   - www/public/ → private bucket (sync only)
#
# Run this manually whenever you want to sync files to R2.
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

# Function to upload files to R2 bucket
upload_to_bucket() {
    local file="$1"
    local bucket="$2"
    local r2_path="$3"

    # Check if wrangler is available
    local wrangler_cmd="npx wrangler"
    if ! command -v npx &> /dev/null; then
        if ! command -v wrangler &> /dev/null; then
            log_error "Neither npx nor wrangler found"
            return 1
        fi
        wrangler_cmd="wrangler"
    fi

    log_info "  - Uploading $file to $bucket/$r2_path"

    # Upload to R2 using wrangler
    if ! $wrangler_cmd r2 object put "$bucket/$r2_path" --file="$file" --remote 2>/dev/null; then
        log_error "Failed to upload $file"
        return 1
    fi

    return 0
}

# Main execution
main() {
    log_info "Syncing static assets to R2..."

    local upload_count=0
    local fail_count=0

    # Upload shared styles to cloud bucket
    if [[ -d "shared/styles" ]]; then
        log_info "Uploading shared/styles/ to cloud bucket..."
        while IFS= read -r file; do
            if upload_to_bucket "$file" "cloud" "$file"; then
                ((upload_count++))
            else
                ((fail_count++))
            fi
        done < <(find shared/styles -type f)
    fi

    # Upload www/public/ to private bucket
    if [[ -d "www/public" ]]; then
        log_info "Uploading www/public/ to private bucket..."
        while IFS= read -r file; do
            if upload_to_bucket "$file" "private" "$file"; then
                ((upload_count++))
            else
                ((fail_count++))
            fi
        done < <(find www/public -type f)
    fi

    # Summary
    log_info "Upload complete: $upload_count succeeded, $fail_count failed"

    if [[ $fail_count -gt 0 ]]; then
        log_error "Some uploads failed"
        exit 1
    fi

    log_success "All files synced to R2"
    exit 0
}

# Run main function
main
