#!/usr/bin/env bash
# Manage experience for portfolio

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/_common.sh"

usage() {
    cat <<EOF
Usage: $0 <command> [options]

Commands:
    get <experience-id>              Get experience by ID
    set <experience-id> <json-file>  Set experience from JSON file
    list                             List all experiences
    featured <mode>                  List featured experiences for mode
    set-featured <mode> <id>...      Set featured experience IDs for mode

Modes: ${MODES[*]}

Examples:
    $0 get rust-club
    $0 set rust-club experience.json
    $0 list
    $0 featured software-engineer
    $0 set-featured fullstack fullstack-dev rust-club
EOF
    exit 1
}

get_experience() {
    local id=$1
    local key="portfolio:experience:$id"
    local content=$(kv_get "$key")

    if [ -z "$content" ]; then
        echo "No experience found with ID: $id"
        exit 1
    fi

    echo "$content" | jq .
}

set_experience() {
    local id=$1
    local json_file=$2

    if [ ! -f "$json_file" ]; then
        echo "Error: file not found: $json_file"
        exit 1
    fi

    local json=$(cat "$json_file")
    validate_json "$json" "$SCRIPT_DIR/../../portfolio/schemas/experience.json"

    local key="portfolio:experience:$id"
    echo "Setting experience: $id"
    kv_put "$key" "$json"
    echo "✅ Experience updated"
}

list_experiences() {
    echo "All experiences:"
    echo ""

    local keys=$(kv_list "portfolio:experience:" | jq -r '.[].name')

    for key in $keys; do
        local id=$(echo "$key" | sed 's/portfolio:experience://')
        local content=$(kv_get "$key")
        local role=$(echo "$content" | jq -r '.role')
        local company=$(echo "$content" | jq -r '.company')
        echo "  $id: $role at $company"
    done
}

get_featured() {
    local mode=$1
    validate_mode "$mode"

    local key="portfolio:featured_experience:$mode"
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
        echo "Error: at least one experience ID is required"
        usage
    fi

    local json=$(jq -n --args '$ARGS.positional' -- "$@")

    local key="portfolio:featured_experience:$mode"
    echo "Setting featured experiences for mode: $mode"
    kv_put "$key" "$json"
    echo "✅ Featured experiences updated"
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
        get_experience "$1"
        ;;
    set)
        if [ $# -ne 2 ]; then usage; fi
        set_experience "$1" "$2"
        ;;
    list)
        list_experiences
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
