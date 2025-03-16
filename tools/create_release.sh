#!/bin/bash
# Simple shell script wrapper for the create_release Rust script

# Default version if not provided
VERSION=${1:-0.1.0}

# Build the release binary if needed
if [ ! -f "target/release/push-to-whisper" ]; then
    echo "Building release binary..."
    cargo build --release
fi

# Build and run the create_release script
echo "Creating release package for version $VERSION..."
cargo run --release --bin create_release "$VERSION" 