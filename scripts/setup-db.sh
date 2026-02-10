#!/bin/bash

# Database setup script for Recall Notes

set -e

echo "Setting up Recall Notes database..."

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo "PostgreSQL is not installed. Please install it first."
    exit 1
fi

# Default database configuration
DB_NAME="recall_notes"
DB_USER="postgres"
DB_PASSWORD="password"
DB_HOST="localhost"
DB_PORT="5432"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --db-name)
            DB_NAME="$2"
            shift 2
            ;;
        --db-user)
            DB_USER="$2"
            shift 2
            ;;
        --db-password)
            DB_PASSWORD="$2"
            shift 2
            ;;
        --db-host)
            DB_HOST="$2"
            shift 2
            ;;
        --db-port)
            DB_PORT="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Create database if it doesn't exist
echo "Creating database '$DB_NAME' if it doesn't exist..."
psql -h $DB_HOST -p $DB_PORT -U $DB_USER -c "CREATE DATABASE $DB_NAME;" 2>/dev/null || true

# Run migrations
echo "Running migrations..."
for migration_file in src-tauri/migrations/*.sql; do
    if [ -f "$migration_file" ]; then
        echo "Applying migration: $(basename $migration_file)"
        psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -f "$migration_file"
    fi
done

# Create .env file
echo "Creating .env file..."
cat > .env << EOF
# Database Configuration
DATABASE_URL=postgres://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME

# Application Configuration
APP_NAME=Recall Notes
APP_VERSION=1.0.0
APP_DEBUG=true

# Security
SECRET_KEY=$(openssl rand -hex 32)

# File Storage
MAX_UPLOAD_SIZE=10485760
ALLOWED_FILE_TYPES=jpg,jpeg,png,gif,pdf,doc,docx,txt,md

# Backup Settings
BACKUP_ENABLED=true
BACKUP_INTERVAL_HOURS=24
MAX_BACKUP_COUNT=10
EOF

echo "Database setup completed successfully!"
echo "Connection string: postgres://$DB_USER:****@$DB_HOST:$DB_PORT/$DB_NAME"