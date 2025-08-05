use rmcp::service::serve_server;
use tokio::io::{stdin, stdout};
use tracing_subscriber::EnvFilter;
mod server;
use server::Counter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .json()
        .with_current_span(false)
        .init();
    // For MCP servers, we communicate via stdin/stdout, not TCP
    let server = Counter::new();

    // Serve the MCP server over stdin/stdout for Claude Desktop compatibility
    serve_server(server, (stdin(), stdout())).await?;

    Ok(())
}
