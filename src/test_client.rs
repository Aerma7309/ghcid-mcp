// Simple test client to validate the echo tool
use rmcp::service::serve_client;
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
struct TestClient;

impl rmcp::ClientHandler for TestClient {
    fn get_peer(&self) -> Option<rmcp::service::Peer<rmcp::service::RoleClient>> {
        None
    }

    fn set_peer(&mut self, _peer: rmcp::service::Peer<rmcp::service::RoleClient>) {
        // No-op for simple test
    }

    fn get_info(&self) -> rmcp::model::ClientInfo {
        rmcp::model::ClientInfo {
            protocol_version: Default::default(),
            capabilities: Default::default(),
            client_info: rmcp::model::Implementation {
                name: "test-client".into(),
                version: "0.1.0".into(),
            },
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the server
    let stream = TcpStream::connect("127.0.0.1:8001").await?;
    let client = TestClient;

    // Start the client service
    let service = serve_client(client, stream).await?;

    // List available tools
    let tools = service.peer().list_tools(Default::default()).await?;
    println!("Available tools: {:?}", tools);

    // Test the echo tool
    let mut params = serde_json::Map::new();
    params.insert(
        "text".to_string(),
        serde_json::Value::String("Hello, World!".to_string()),
    );

    let result = service
        .peer()
        .call_tool(rmcp::model::CallToolRequestParam {
            name: "echo".into(),
            arguments: Some(params),
        })
        .await?;

    println!("Echo result: {:?}", result);

    Ok(())
}
