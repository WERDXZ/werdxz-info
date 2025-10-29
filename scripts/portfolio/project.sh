#!/usr/bin/env bash
# Manage projects for portfolio

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/_common.sh"

usage() {
    cat <<EOF
Usage: $0 <command> [options]

Commands:
    get <project-id>              Get project by ID
    set <project-id> <json-file>  Set project from JSON file
    list                          List all projects
    featured <mode>               List featured projects for mode
    set-featured <mode> <id>...   Set featured project IDs for mode

Modes: ${MODES[*]}

Examples:
    $0 get chico-rs
    $0 set chico-rs project.json
    $0 list
    $0 featured software-engineer
    $0 set-featured rust chico-rs wlrs archenemy
EOF
    exit 1
}

get_project() {
    local id=$1
    local key="portfolio:project:$id"
    local content=$(kv_get "$key")

    if [ -z "$content" ]; then
        echo "No project found with ID: $id"
        exit 1
    fi

    echo "$content" | jq .
}

set_project() {
    local id=$1
    local json_file=$2

    if [ ! -f "$json_file" ]; then
        echo "Error: file not found: $json_file"
        exit 1
    fi

    local json=$(cat "$json_file")
    validate_json "$json" "$SCRIPT_DIR/../../portfolio/schemas/project.json"

    local key="portfolio:project:$id"
    echo "Setting project: $id"
    kv_put "$key" "$json"
    echo "✅ Project updated"
}

list_projects() {
    echo "All projects:"
    echo ""

    local keys=$(kv_list "portfolio:project:" | jq -r '.[].name')

    for key in $keys; do
        local id=$(echo "$key" | sed 's/portfolio:project://')
        local content=$(kv_get "$key")
        local title=$(echo "$content" | jq -r '.title')
        echo "  $id: $title"
    done
}

get_featured() {
    local mode=$1
    validate_mode "$mode"

    local key="portfolio:featured_projects:$mode"
    local content=$(kv_get "$key")

    if [ -z "$content" ]; then
        echo "[]"
    else
        echo "$content" | jq .
    fi
}

set_featured() {
    local mode=$1
    shift
    validate_mode "$mode"

    if [ $# -eq 0 ]; then
        echo "Error: at least one project ID is required"
        usage
    fi

    local json=$(jq -n --args '$ARGS.positional' -- "$@")

    local key="portfolio:featured_projects:$mode"
    echo "Setting featured projects for mode: $mode"
    kv_put "$key" "$json"
    echo "✅ Featured projects updated"
}

# Main
if [ $# -lt 1 ]; then
    usage
fi

command=$1
shift

case "$command" in
    get)
        if [ $# -ne 1 ]; then usage; fi
        get_project "$1"
        ;;
    set)
        if [ $# -ne 2 ]; then usage; fi
        set_project "$1" "$2"
        ;;
    list)
        list_projects
        ;;
    featured)
        if [ $# -ne 1 ]; then usage; fi
        get_featured "$1"
        ;;
    set-featured)
        if [ $# -lt 2 ]; then usage; fi
        set_featured "$@"
        ;;
    *)
        echo "Unknown command: $command"
        usage
        ;;
esac
