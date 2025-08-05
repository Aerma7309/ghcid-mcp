use rmcp::{
    ServerHandler,
    model::{ProtocolVersion, ServerCapabilities, ServerInfo},
    tool,
};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EchoRequest {
    #[schemars(description = "Text to echo back")]
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct EchoServer;

impl EchoServer {
    pub fn new() -> Self {
        Self
    }
}

#[tool(tool_box)]
impl EchoServer {
    #[tool(description = "Echo back the provided text")]
    async fn echo(&self, #[tool(aggr)] request: EchoRequest) -> String {
        "response from mcp server ".to_string() + request.text.as_str()
    }
}

#[tool(tool_box)]
impl ServerHandler for EchoServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: rmcp::model::Implementation {
                name: "echo-server".into(),
                version: "0.1.0".into(),
            },
            instructions: Some("A simple echo server that returns the input text.".into()),
        }
    }
}
