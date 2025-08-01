use rmcp::service::serve_server;
use tokio::io::{stdin, stdout};

mod server;
use server::EchoServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // For MCP servers, we communicate via stdin/stdout, not TCP
    let server = EchoServer::new();

    // Serve the MCP server over stdin/stdout for Claude Desktop compatibility
    serve_server(server, (stdin(), stdout())).await?;

    Ok(())
}
