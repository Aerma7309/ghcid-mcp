use rmcp::service::serve_server;
use tokio::io::{stdin, stdout};
use tracing::{error, info};

mod logger;
mod server;
use server::GhcidServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging system
    logger::init()?;

    info!("Starting GHCID MCP Server");

    // For MCP servers, we communicate via stdin/stdout, not TCP
    let server = GhcidServer::new();

    info!("Server initialized, beginning MCP communication");

    // Serve the MCP server over stdin/stdout for Claude Desktop compatibility
    match serve_server(server, (stdin(), stdout())).await {
        Ok(server_handle) => {
            info!("MCP server started successfully");
            match server_handle.waiting().await {
                Ok(_) => {
                    info!("MCP server shutdown gracefully");
                    Ok(())
                }
                Err(e) => {
                    error!(error = %e, "Error during server operation");
                    Err(e.into())
                }
            }
        }
        Err(e) => {
            error!(error = %e, "Failed to start MCP server");
            Err(e.into())
        }
    }
}
