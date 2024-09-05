// use futures_util::stream::StreamExt;
// use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    // let (mut ws_stream, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
    //     .connect()
    //     .await?;

    let stdin = tokio::io::stdin();
    let mut lines = BufReader::new(stdin).lines();
    while let Some(line) = lines.next_line().await? {
        println!("{}", line);
    }
    Ok(())
}
