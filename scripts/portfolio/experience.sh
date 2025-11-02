#!/usr/bin/env bash
# Manage experience for portfolio

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/_common.sh"

usage() {
    cat <<EOF
Usage: $0 <command> [options]

Commands:
    get <mode> <experience-id>            Get experience by mode and ID
    set <mode> <experience-id> <json-file>  Set experience from JSON file
    list <mode>                           List all experiences for mode
    featured <mode>                       List featured experiences for mode
    set-featured <mode> <id>...           Set featured experience IDs for mode

Modes: ${MODES[*]}

Examples:
    $0 get industry fullstack-dev
    $0 set industry fullstack-dev experience.json
    $0 list industry
    $0 featured industry
    $0 set-featured industry fullstack-dev archenemy
EOF
    exit 1
}

get_experience() {
    local mode=$1
    local id=$2
    validate_mode "$mode"

    local key="portfolio:experience:$mode:$id"
    local content=$(kv_get "$key")

    if [ -z "$content" ]; then
        echo "No experience found with mode: $mode, ID: $id"
        exit 1
    fi

    echo "$content" | jq .
}

set_experience() {
    local mode=$1
    local id=$2
    local json_file=$3
    validate_mode "$mode"

    if [ ! -f "$json_file" ]; then
        echo "Error: file not found: $json_file"
        exit 1
    fi

    local json=$(cat "$json_file")
    validate_json "$json" "$SCRIPT_DIR/../../portfolio/schemas/experience.json"

    local key="portfolio:experience:$mode:$id"
    echo "Setting experience for mode $mode: $id"
    kv_put "$key" "$json"
    echo "✅ Experience updated"
}

list_experiences() {
    local mode=$1
    validate_mode "$mode"

    echo "Experiences for mode: $mode"
    echo ""

    local keys=$(kv_list "portfolio:experience:$mode:" | jq -r '.[].name')

    for key in $keys; do
        local id=$(echo "$key" | sed "s/portfolio:experience:$mode://")
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
        if [ $# -ne 2 ]; then usage; fi
        get_experience "$1" "$2"
        ;;
    set)
        if [ $# -ne 3 ]; then usage; fi
        set_experience "$1" "$2" "$3"
        ;;
    list)
        if [ $# -ne 1 ]; then usage; fi
        list_experiences "$1"
        ;;
    featured)
        if [ $# -ne 1 ]; then usage; fi
        get_featured "$1"
        ;;
    set-featured)
        if [ $# -lt 2 ]; then usage; fi
        mode=$1
        shift
        set_featured "$mode" "$@"
        ;;
    *)
        echo "Unknown command: $command"
        usage
        ;;
esac
