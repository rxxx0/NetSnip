#!/bin/bash

# NetSnip Sudo Setup Script
# This script configures passwordless sudo for NetSnip

echo "🔐 NetSnip Sudo Setup"
echo "====================="
echo ""
echo "This script will configure passwordless sudo for NetSnip."
echo "You will need to enter your password once to set this up."
echo ""

# Get the current username
USERNAME=$(whoami)
echo "Setting up for user: $USERNAME"

# Create sudoers configuration
SUDOERS_CONTENT="# NetSnip passwordless sudo configuration
$USERNAME ALL=(ALL) NOPASSWD: /Users/$USERNAME/NetSnip/target/debug/netsnip
$USERNAME ALL=(ALL) NOPASSWD: /Users/$USERNAME/NetSnip/target/release/netsnip"

# Create temporary file
TEMP_FILE=$(mktemp)
echo "$SUDOERS_CONTENT" > "$TEMP_FILE"

# Validate the sudoers syntax
echo ""
echo "Validating sudoers configuration..."
if visudo -c -f "$TEMP_FILE"; then
    echo "✅ Configuration is valid"

    # Install the configuration
    echo ""
    echo "Installing configuration (requires sudo)..."
    sudo cp "$TEMP_FILE" /etc/sudoers.d/netsnip
    sudo chmod 440 /etc/sudoers.d/netsnip

    echo "✅ Passwordless sudo configured successfully!"
    echo ""
    echo "You can now run NetSnip without entering your password:"
    echo "  npm start"
    echo "  ./start.sh"
else
    echo "❌ Configuration validation failed"
    echo "Please check the configuration and try again"
fi

# Clean up
rm -f "$TEMP_FILE"