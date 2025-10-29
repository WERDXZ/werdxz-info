#!/usr/bin/env bash
# Common utilities for portfolio management scripts

set -euo pipefail

# KV namespace ID
NAMESPACE="ad9607c404424a8eb6949994a4383845"

# Valid modes
MODES=("software-engineer" "fullstack" "rust" "student")

# Check if mode is valid
validate_mode() {
    local mode=$1
    for valid_mode in "${MODES[@]}"; do
        if [ "$mode" = "$valid_mode" ]; then
            return 0
        fi
    done
    echo "Error: Invalid mode '$mode'. Valid modes: ${MODES[*]}"
    exit 1
}

# Put value to KV (remote by default)
kv_put() {
    local key=$1
    local value=$2
    local remote_flag="${3:---remote}"

    npx wrangler kv key put $remote_flag --namespace-id="$NAMESPACE" "$key" "$value"
}

# Get value from KV
kv_get() {
    local key=$1
    local remote_flag="${2:---remote}"

    npx wrangler kv key get $remote_flag --namespace-id="$NAMESPACE" "$key" 2>/dev/null || echo ""
}

# List keys with prefix
kv_list() {
    local prefix=$1
    local remote_flag="${2:---remote}"

    npx wrangler kv key list $remote_flag --namespace-id="$NAMESPACE" --prefix="$prefix"
}

# Delete key from KV
kv_delete() {
    local key=$1
    local remote_flag="${2:---remote}"

    npx wrangler kv key delete $remote_flag --namespace-id="$NAMESPACE" "$key"
}

# Validate JSON against schema (requires jq)
validate_json() {
    local json=$1
    local schema_path=$2

    if ! command -v jq &> /dev/null; then
        echo "Warning: jq not installed, skipping JSON validation"
        return 0
    fi

    # Basic JSON syntax check
    if ! echo "$json" | jq empty 2>/dev/null; then
        echo "Error: Invalid JSON syntax"
        return 1
    fi

    # TODO: Add proper JSON schema validation if needed
    return 0
}
