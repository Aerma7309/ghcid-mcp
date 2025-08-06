# GHCID MCP Server

A Model Context Protocol (MCP) server that enables AI agents like Claude Code to check Haskell project compilation status in real-time. This server provides fast feedback loops for Haskell development by exposing GHC compilation checking capabilities through the MCP protocol.

## Purpose

This MCP server bridges the gap between AI development tools and Haskell compilation feedback, allowing AI agents to:

- **Check compilation status** of Haskell projects instantly
- **Get detailed error messages** when compilation fails  
- **Improve development feedback loops** with real-time compilation checking
- **Enable faster Haskell development** through AI-assisted error resolution

Perfect for AI agents working on Haskell projects that need immediate feedback on code changes and compilation errors.

## Features

- **Real-time Haskell compilation checking** via GHC integration
- **Detailed error reporting** with precise location information
- **MCP Protocol compliance** for seamless AI agent integration
- **Fast feedback loops** optimized for development workflows
- **Type-safe interfaces** with JSON schema validation
- **Async operation** for non-blocking compilation checks

## Quick Start

### Prerequisites

- **Rust toolchain** (1.70+)
- **GHC (Glasgow Haskell Compiler)** installed and available in PATH
- **Cabal** or **Stack** for Haskell project management

### Installation

#### Method 1: Direct Cargo Install
```bash
# From GitHub repository
cargo install --git https://github.com/Aerma7309/ghcid-mcp

# Or if published to crates.io
cargo install ghcid-mcp
```

#### Method 2: Build from Source
```bash
git clone https://github.com/Aerma7309/ghcid-mcp
cd ghcid-mcp
cargo build --release
```

### Claude Desktop Configuration

Add this to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`  
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`  
**Linux**: `~/.config/claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "ghcid-mcp": {
      "command": "full/path/to/ghcid-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

## Usage

Once configured, AI agents can use the following capabilities:

### Compilation Checking

Check if a Haskell project compiles:

```json
{
  "name": "check_compilation",
  "arguments": {
    "project_path": "/path/to/haskell/project",
    "module": "Main.hs"
  }
}
```

**Response on Success**:
```json
{
  "status": "success",
  "message": "Compilation successful",
  "warnings": []
}
```

**Response on Failure**:
```json
{
  "status": "error", 
  "message": "Compilation failed",
  "errors": [
    {
      "file": "src/Main.hs",
      "line": 42,
      "column": 15,
      "message": "Variable not in scope: undefinedFunction",
      "severity": "error"
    }
  ]
}
```

## Architecture

The server leverages:

- **GHCID integration** for fast compilation checking
- **MCP protocol** for AI agent communication  
- **Tokio async runtime** for non-blocking operations
- **JSON schema validation** for type safety
- **Error parsing** for detailed feedback

## Project Structure

```
src/
├── main.rs          # MCP server entry point with stdin/stdout communication
├── server.rs        # GHCIDServer implementation with compilation tools
└── ghcid/           # GHC integration and error parsing modules
```

## Development

### Building
```bash
cargo build
```

### Running
```bash
cargo run
```

The server communicates via stdin/stdout for MCP compatibility with Claude Desktop.

### Testing
Test with any MCP-compatible client or use the included examples:

```bash
# Check compilation of current directory
echo '{"method": "tools/call", "params": {"name": "check_compilation", "arguments": {"project_path": "."}}}' | cargo run
```

## MCP Tools Available

### `check_compilation`
- **Purpose**: Check if Haskell project compiles
- **Parameters**: 
  - `project_path` (string): Path to Haskell project directory
  - `module` (string, optional): Specific module to check
- **Returns**: Compilation status with errors/warnings

### `get_project_info`
- **Purpose**: Get information about Haskell project structure  
- **Parameters**:
  - `project_path` (string): Path to project
- **Returns**: Project metadata and module list

## Integration with AI Development

This server is specifically designed to enhance AI-assisted Haskell development by:

1. **Providing immediate feedback** on code changes
2. **Enabling error-driven development** where AI can fix compilation errors iteratively
3. **Supporting rapid prototyping** with instant compilation checking
4. **Facilitating learning** by providing detailed error explanations

## Dependencies

- `rmcp`: Model Context Protocol implementation for Rust
- `tokio`: Async runtime for non-blocking operations
- `serde`: JSON serialization framework
- `schemars`: JSON schema generation for type safety

## Contributing

Contributions welcome! This project aims to improve the Haskell development experience for AI agents.

## License

MIT License - see LICENSE file for details.