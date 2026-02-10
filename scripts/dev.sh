#!/bin/bash

# Development script for Recall Notes

set -e

echo "Starting Recall Notes in development mode..."

# Check if database is running
if ! pg_isready -q; then
    echo "PostgreSQL is not running. Starting it..."
    
    # Try to start PostgreSQL (this command varies by OS)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        brew services start postgresql
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo service postgresql start
    fi
    
    sleep 2
fi

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Install frontend dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "Installing frontend dependencies..."
    bun install
fi

# Install Rust dependencies if needed
if [ ! -d "src-tauri/target" ]; then
    echo "Installing Rust dependencies..."
    cd src-tauri
    cargo fetch
    cd ..
fi

# Run database migrations
echo "Running database migrations..."
cd src-tauri
cargo run --bin migrate 2>/dev/null || echo "Migrations already applied"
cd ..

# Start the application
echo "Starting Tauri application..."
bun run tauri dev