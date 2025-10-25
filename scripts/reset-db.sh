#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🗄️  Database Reset Utility${NC}"
echo ""

# Check if --remote flag is passed
REMOTE_FLAG=""
LOCATION="local"
if [[ "$1" == "--remote" ]]; then
    REMOTE_FLAG="--remote"
    LOCATION="remote"
    echo -e "${RED}⚠️  WARNING: This will reset the REMOTE database!${NC}"
    read -p "Are you sure? (yes/no): " confirm
    if [[ "$confirm" != "yes" ]]; then
        echo "Aborted."
        exit 1
    fi
fi

echo -e "${BLUE}Resetting ${LOCATION} database...${NC}"
echo ""

# Navigate to api directory
cd "$(dirname "$0")/../api"

# Get database name from wrangler.toml
DB_NAME=$(grep -A 2 'd1_databases' wrangler.toml | grep 'database_name' | cut -d'"' -f2)

if [ -z "$DB_NAME" ]; then
    echo -e "${RED}❌ Could not find database name in wrangler.toml${NC}"
    exit 1
fi

echo -e "${GREEN}Database: ${DB_NAME}${NC}"
echo ""

# Drop all tables
echo -e "${BLUE}Step 1/2: Dropping existing tables...${NC}"

TABLES=("post_tags" "posts" "tags" "d1_migrations")

for table in "${TABLES[@]}"; do
    echo "  Dropping table: $table"
    npx wrangler d1 execute "$DB_NAME" $REMOTE_FLAG \
        --command "DROP TABLE IF EXISTS $table;" 2>/dev/null || true
done

echo -e "${GREEN}✓ Tables dropped${NC}"
echo ""

# Run migrations
echo -e "${BLUE}Step 2/2: Running migrations...${NC}"
npx wrangler d1 migrations apply "$DB_NAME" $REMOTE_FLAG

echo ""
echo -e "${GREEN}✅ Database reset complete!${NC}"
