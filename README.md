# Echo MCP Server

A simple Model Context Protocol (MCP) server implemented in Rust using the `rmcp` library. This server provides a single "echo" tool that returns the input text back to the client.

## Features

- **Echo Tool**: Returns the provided text input unchanged
- **TCP Transport**: Supports TCP connections on port 8001
- **Async/Await**: Full asynchronous implementation using Tokio
- **Type Safety**: JSON schema validation for tool parameters
- **MCP Compliant**: Follows the Model Context Protocol specification
- **Easy Installation**: Multiple installation methods supported

## Quick Start

### Installation

#### Method 1: Quick Install Script (Recommended)
```bash
curl -fsSL https://raw.githubusercontent.com/Aerma7309/ghcid-mcp/main/install.sh | sh
```

#### Method 2: Direct Cargo Install
```bash
# From GitHub (compiles on your machine)
cargo install --git https://github.com/Aerma7309/ghcid-mcp

# From crates.io (if published)
cargo install ghcid-mcp
```

#### Method 3: Claude MCP Integration
```bash
# Using Claude MCP manager (similar to Serena pattern)
claude mcp add ghcid-echo -- cargo install --git https://github.com/Aerma7309/ghcid-mcp
```

**Prerequisites**: Rust toolchain required (installs automatically with script)

### Claude Desktop Configuration

Add this to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`  
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`  
**Linux**: `~/.config/claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "ghcid-echo": {
      "command": "ghcid-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

### Development Setup

## Project Structure

```
src/
├── main.rs          # TCP server entry point
├── server.rs        # EchoServer implementation with tools
└── test_client.rs   # Example test client
```

## Building from Source

```bash
cargo build
```

## Running the Server

```bash
cargo run
```

This will start the default `ghcid-mcp` binary. The server will start listening on `127.0.0.1:8001`.

## Testing

You can test the server using any MCP-compatible client, or run the included test client:

```bash
cargo run --bin test_client
```

## Tool Definition

### Echo Tool

**Name**: `echo`  
**Description**: Echo back the provided text  
**Parameters**:
- `text` (string, required): Text to echo back

**Example Usage**:
```json
{
  "name": "echo",
  "arguments": {
    "text": "Hello, World!"
  }
}
```

**Response**:
```json
{
  "content": [
    {
      "type": "text",
      "text": "Hello, World!"
    }
  ],
  "is_error": false
}
```

## Architecture

The server uses the `rmcp` library's macro system for automatic tool registration:

- `#[tool(tool_box)]` on the implementation generates tool metadata
- `#[tool(description = "...")]` on methods creates callable tools
- Automatic JSON schema generation from Rust types
- Built-in MCP protocol handling

## Dependencies

- `rmcp`: Core MCP library for Rust
- `tokio`: Async runtime
- `serde`: Serialization framework
- `schemars`: JSON schema generation

## MCP Protocol Compliance

This implementation fully complies with the MCP specification:

- ✅ Server initialization and handshake
- ✅ Tool discovery via `tools/list`
- ✅ Tool execution via `tools/call`
- ✅ JSON schema validation
- ✅ Proper error handling
- ✅ Concurrent client support
- ✅ Simple installation via cargo (no pre-built binaries needed)