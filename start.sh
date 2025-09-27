#!/bin/bash

# NetSnip Launcher
echo "🚀 Starting NetSnip..."

# Source cargo environment if needed
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# Set environment for better logging
export RUST_LOG=info

# Check if we need sudo
if [ "$EUID" -ne 0 ]; then
    echo "NetSnip needs administrator privileges for network monitoring."
    echo "Attempting to run with sudo..."
    exec sudo "$0" "$@"
fi

echo "✅ Running NetSnip with network privileges..."

# Run the app in development mode
npm run tauri:dev