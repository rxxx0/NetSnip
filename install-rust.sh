#!/bin/bash

echo "Installing Rust for NetSnip..."
echo ""

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Source the environment
source "$HOME/.cargo/env"

# Verify installation
echo ""
echo "Verifying Rust installation..."
rustc --version
cargo --version

echo ""
echo "✅ Rust installation complete!"
echo ""
echo "Now you can run:"
echo "  npm run tauri:dev"