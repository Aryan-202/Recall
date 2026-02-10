#!/bin/bash

# Build script for Recall Notes

set -e

echo "Building Recall Notes..."

# Check if database is configured
if [ ! -f .env ]; then
    echo "Error: .env file not found. Run setup-db.sh first."
    exit 1
fi

# Load environment variables
export $(cat .env | grep -v '^#' | xargs)

# Build frontend
echo "Building frontend..."
bun run build

# Build backend
echo "Building backend..."
cd src-tauri
cargo build --release
cd ..

# Create distribution directory
echo "Creating distribution..."
mkdir -p dist

# Copy built files
cp -r build/* dist/ 2>/dev/null || true
cp -r src-tauri/target/release/*.exe dist/ 2>/dev/null || true
cp -r src-tauri/target/release/*.app dist/ 2>/dev/null || true
cp -r src-tauri/target/release/*.deb dist/ 2>/dev/null || true
cp -r src-tauri/target/release/*.rpm dist/ 2>/dev/null || true

echo "Build completed successfully!"
echo "Output files are in the 'dist' directory."