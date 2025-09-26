#!/bin/bash

echo "======================================"
echo "NetSnip Setup Script"
echo "======================================"
echo ""

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "✅ Rust is already installed ($(rustc --version))"
fi

# Ensure cargo is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Install Tauri CLI
echo "📦 Installing Tauri CLI..."
cargo install tauri-cli --version "^2.0.0-beta"

# Install Node dependencies
echo "📦 Installing Node dependencies..."
npm install

# Install additional Tauri v2 dependencies
echo "📦 Installing Tauri v2 API..."
npm install @tauri-apps/api@next @tauri-apps/cli@next --save

# Build icons (create placeholder if needed)
if [ ! -d "src-tauri/icons" ]; then
    echo "📦 Creating icon placeholders..."
    mkdir -p src-tauri/icons
    # Create a simple 32x32 PNG placeholder
    echo "iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAACXBIWXMAAAsTAAALEwEAmpwYAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAZSURBVHgB7cEBAQAAAIIg/69uSEABAAAA7w0EAAAB3nXJuQAAAABJRU5ErkJggg==" | base64 -d > src-tauri/icons/32x32.png
    cp src-tauri/icons/32x32.png src-tauri/icons/128x128.png
    cp src-tauri/icons/32x32.png src-tauri/icons/128x128@2x.png
    cp src-tauri/icons/32x32.png src-tauri/icons/icon.png
    touch src-tauri/icons/icon.icns
    touch src-tauri/icons/icon.ico
fi

# Build the Rust backend
echo "📦 Building Rust backend..."
cd src-tauri && cargo build --release
cd ..

echo ""
echo "======================================"
echo "✨ Setup complete!"
echo "======================================"
echo ""
echo "To run the application:"
echo "  npm run tauri:dev   # Development mode"
echo "  npm run tauri:build # Build for production"
echo ""