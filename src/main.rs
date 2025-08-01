use rmcp::service::serve_server;
use tokio::net::TcpListener;

mod server;
use server::EchoServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8001").await?;
    println!("Echo MCP Server listening on 127.0.0.1:8001");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        let server = EchoServer::new();
        tokio::spawn(async move {
            if let Err(e) = serve_server(server, stream).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}
