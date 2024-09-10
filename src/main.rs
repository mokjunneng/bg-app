pub mod prisma;

use bgapp::request::parse_request;
use std::fs;
use std::io;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::time;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Listen for incoming TCP connections on localhost port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    // Block forever, handling each request that arrives at this IP address
    loop {
        let (socket, _) = listener.accept().await?;
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            let _ = handle_connection(socket).await;
        });
    }
}

// TODO:
// * Parse request - method, header, payload, path
// * Route request to correct handler

async fn handle_connection(stream: TcpStream) -> io::Result<()> {
    stream.readable().await?;
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.try_read(&mut buffer).unwrap();
    println!("{}", String::from_utf8(buffer.to_vec()).unwrap());

    let request = parse_request(&buffer);

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    // let response = format!("{status_line}{contents}");
    // stream.try_write(response.as_bytes()).unwrap();
    Ok(())
}
