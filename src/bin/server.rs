use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_websockets::{ServerBuilder, WebSocketStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let ws_stream = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream).await
        });
    }
}

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    while let Some(Ok(msg)) = ws_stream.next().await {
        if let Some(text) = msg.as_text() {
            println!("Received message from client: {text}")
        }
    }
    Ok(())
}
