#!/usr/bin/env bash
# Manage hero content for portfolio

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/_common.sh"

usage() {
    cat <<EOF
Usage: $0 <command> [options]

Commands:
    get <mode>                          Get hero content for mode
    set <mode> <subtitle> <description> Set hero content for mode
    list                                List all hero content

Modes: ${MODES[*]}

Examples:
    $0 get software-engineer
    $0 set rust "Rust Developer" "Building systems with Rust"
    $0 list
EOF
    exit 1
}

get_hero() {
    local mode=$1
    validate_mode "$mode"

    local key="portfolio:hero_content:$mode"
    local content=$(kv_get "$key")

    if [ -z "$content" ]; then
        echo "No hero content found for mode: $mode"
        exit 1
    fi

    echo "$content" | jq .
}

set_hero() {
    local mode=$1
    local subtitle=$2
    local description=$3

    validate_mode "$mode"

    if [ -z "$subtitle" ] || [ -z "$description" ]; then
        echo "Error: subtitle and description are required"
        usage
    fi

    local json=$(jq -n \
        --arg subtitle "$subtitle" \
        --arg description "$description" \
        '{subtitle: $subtitle, description: $description}')

    validate_json "$json" "$SCRIPT_DIR/../../portfolio/schemas/hero-content.json"

    local key="portfolio:hero_content:$mode"
    echo "Setting hero content for mode: $mode"
    kv_put "$key" "$json"
    echo "✅ Hero content updated"
}

list_hero() {
    echo "Hero content for all modes:"
    echo ""

    for mode in "${MODES[@]}"; do
        local key="portfolio:hero_content:$mode"
        local content=$(kv_get "$key")

        if [ -n "$content" ]; then
            echo "[$mode]"
            echo "$content" | jq -r '"  Subtitle: \(.subtitle)\n  Description: \(.description)"'
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
        get_hero "$1"
        ;;
    set)
        if [ $# -ne 3 ]; then usage; fi
        set_hero "$1" "$2" "$3"
        ;;
    list)
        list_hero
        ;;
    *)
        echo "Unknown command: $command"
        usage
        ;;
esac
