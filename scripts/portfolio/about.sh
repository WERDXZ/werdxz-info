#!/usr/bin/env bash
# Manage about content for portfolio

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/_common.sh"

usage() {
    cat <<EOF
Usage: $0 <command> [options]

Commands:
    get <mode>                 Get about content for mode
    set <mode> <paragraph>...  Set about content for mode (multiple paragraphs)
    list                       List all about content

Modes: ${MODES[*]}

Examples:
    $0 get software-engineer
    $0 set rust "Paragraph 1" "Paragraph 2"
    $0 list
EOF
    exit 1
}

get_about() {
    local mode=$1
    validate_mode "$mode"

    local key="portfolio:about_content:$mode"
    local content=$(kv_get "$key")

    if [ -z "$content" ]; then
        echo "No about content found for mode: $mode"
        exit 1
    fi

    echo "$content" | jq .
}

set_about() {
    local mode=$1
    shift
    validate_mode "$mode"

    if [ $# -eq 0 ]; then
        echo "Error: at least one paragraph is required"
        usage
    fi

    # Build JSON array from paragraphs
    local json=$(jq -n --args '{paragraphs: $ARGS.positional}' -- "$@")

    validate_json "$json" "$SCRIPT_DIR/../../portfolio/schemas/about-content.json"

    local key="portfolio:about_content:$mode"
    echo "Setting about content for mode: $mode"
    kv_put "$key" "$json"
    echo "✅ About content updated"
}

list_about() {
    echo "About content for all modes:"
    echo ""

    for mode in "${MODES[@]}"; do
        local key="portfolio:about_content:$mode"
        local content=$(kv_get "$key")

        if [ -n "$content" ]; then
            echo "[$mode]"
            echo "$content" | jq -r '.paragraphs | to_entries[] | "  \(.key + 1). \(.value)"'
            echo ""
        fi
    done
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
        get_about "$1"
        ;;
    set)
        if [ $# -lt 2 ]; then usage; fi
        set_about "$@"
        ;;
    list)
        list_about
        ;;
    *)
        echo "Unknown command: $command"
        usage
        ;;
esac
