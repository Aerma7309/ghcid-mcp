#!/bin/bash
set -e

echo "🔍 Verifying ghcid-mcp installation..."

# Check if Rust/Cargo is available
if ! command -v cargo >/dev/null 2>&1; then
    echo "❌ Cargo not found. Install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Cargo found: $(cargo --version)"

# Check if binary exists in cargo bin
CARGO_BIN="$HOME/.cargo/bin/ghcid-mcp"
if [[ -f "$CARGO_BIN" ]]; then
    echo "✅ ghcid-mcp binary found: $CARGO_BIN"
elif command -v ghcid-mcp >/dev/null 2>&1; then
    echo "✅ ghcid-mcp binary found: $(which ghcid-mcp)"
else
    echo "❌ ghcid-mcp not found. Run:"
    echo "   cargo install --git https://github.com/Aerma7309/ghcid-mcp"
    exit 1
fi

# Check if it can start (timeout after 3 seconds)
echo "🧪 Testing server startup..."
timeout 3s ghcid-mcp &
SERVER_PID=$!

sleep 1

# Check if server is running
if ps -p $SERVER_PID > /dev/null; then
    echo "✅ Server started successfully"
    kill $SERVER_PID 2>/dev/null || true
    wait $SERVER_PID 2>/dev/null || true
else
    echo "❌ Server failed to start"
    exit 1
fi

echo ""
echo "🎉 Installation verified successfully!"
echo ""
echo "Next steps:"
echo "1. Add the MCP server to your Claude Desktop config"
echo "2. Copy the configuration from claude-desktop-config.json"
echo "3. Restart Claude Desktop"
echo ""
echo "Configuration path:"
echo "  macOS: ~/Library/Application Support/Claude/claude_desktop_config.json"
echo "  Linux: ~/.config/claude/claude_desktop_config.json"
echo "  Windows: %APPDATA%/Claude/claude_desktop_config.json"