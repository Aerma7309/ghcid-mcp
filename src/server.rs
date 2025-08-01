use rmcp::{ServerHandler, model::ServerInfo, tool};
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
        request.text
    }
}

#[tool(tool_box)]
impl ServerHandler for EchoServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: Default::default(),
            capabilities: Default::default(),
            server_info: rmcp::model::Implementation {
                name: "echo-server".into(),
                version: "0.1.0".into(),
            },
            instructions: Some("A simple echo server that returns the input text.".into()),
        }
    }
}
