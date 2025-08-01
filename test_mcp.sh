#!/bin/bash
set -e

echo "🧪 Testing GHCID MCP Server functionality..."

# Check if binary exists
if ! command -v ghcid-mcp >/dev/null 2>&1 && [[ ! -f "$HOME/.cargo/bin/ghcid-mcp" ]]; then
    echo "❌ ghcid-mcp not found. Please install first:"
    echo "   cargo install --path ."
    exit 1
fi

BINARY="${HOME}/.cargo/bin/ghcid-mcp"
if [[ ! -f "$BINARY" ]]; then
    BINARY="ghcid-mcp"
fi

echo "✅ Using binary: $BINARY"

# Test 1: Initialize
echo "📋 Test 1: Initialize request"
INIT_RESPONSE=$(echo '{"jsonrpc": "2.0", "method": "initialize", "id": 1, "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}}' | $BINARY 2>/dev/null || echo "FAILED")

if [[ "$INIT_RESPONSE" == *"echo-server"* ]]; then
    echo "✅ Initialize successful"
else
    echo "❌ Initialize failed"
    echo "Response: $INIT_RESPONSE"
    exit 1
fi

# Test 2: List tools
echo "📋 Test 2: List tools request"
TOOLS_RESPONSE=$(echo -e '{"jsonrpc": "2.0", "method": "initialize", "id": 1, "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}}\n{"jsonrpc": "2.0", "method": "tools/list", "id": 2, "params": {}}' | $BINARY 2>/dev/null | tail -1 || echo "FAILED")

if [[ "$TOOLS_RESPONSE" == *"echo"* ]]; then
    echo "✅ Tools list successful"
else
    echo "❌ Tools list failed"
    echo "Response: $TOOLS_RESPONSE"
    exit 1
fi

echo ""
echo "🎉 All tests passed! Your MCP server is working correctly."
echo ""
echo "Next steps:"
echo "1. Add to Claude Desktop config:"
echo "   {\"mcpServers\": {\"ghcid-echo\": {\"command\": \"ghcid-mcp\", \"args\": [], \"env\": {}}}}"
echo "2. Restart Claude Desktop"
echo "3. The 'echo' tool should now be available in Claude conversations"