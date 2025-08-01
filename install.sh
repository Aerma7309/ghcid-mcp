#!/bin/bash
set -e

# ghcid-mcp installer script
REPO="${GITHUB_REPOSITORY:-Aerma7309/ghcid-mcp}"
BINARY_NAME="ghcid-mcp"

echo "🚀 Installing ghcid-mcp..."

# Check if Rust/Cargo is installed
if ! command -v cargo >/dev/null 2>&1; then
    echo "❌ Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "   source ~/.cargo/env"
    exit 1
fi

echo "✅ Cargo found: $(cargo --version)"

# Install using cargo
echo "📦 Installing from GitHub repository..."
echo "   This will compile the binary on your system"

if cargo install --git "https://github.com/${REPO}" --force; then
    echo "✅ ghcid-mcp installed successfully!"
else
    echo "❌ Installation failed. Please check:"
    echo "   - Internet connection"
    echo "   - Repository URL: https://github.com/${REPO}"
    echo "   - Rust toolchain is up to date: rustup update"
    exit 1
fi

# Verify installation
if command -v ghcid-mcp >/dev/null 2>&1; then
    echo "🎯 Installation verified: $(which ghcid-mcp)"
else
    echo "⚠️  ghcid-mcp installed but not found in PATH"
    echo "   Make sure ~/.cargo/bin is in your PATH:"
    echo "   export PATH=\"\$HOME/.cargo/bin:\$PATH\""
fi

echo ""
echo "🎉 Installation complete!"
echo ""
echo "Next steps:"
echo "1. Add to Claude Desktop config (see claude-desktop-config.json)"
echo "2. Run 'ghcid-mcp' to start the server"
echo "3. Run the verification script: ./scripts/verify-install.sh"