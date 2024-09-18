use std::{
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use futures_util::StreamExt;

use tokio::{
    net::{TcpListener, TcpStream},
    sync::oneshot,
};

use bgapp::{
    dtos::message::MessageDto,
    prisma::PrismaClient,
    repository::message::MessageRepository,
    services::messaging::{Command, MessagingService},
};

// TODO: Push notification service to alert new messages
// TODO: DTO and model for message

#[tokio::main]
async fn main() -> Result<(), IoError> {
    // initiate database client connection
    // TODO: May need to arc mutex this shit
    let db_client = PrismaClient::_builder()
        .build()
        .await
        .expect("Failed to initialize db client");
    let message_repo = MessageRepository::new(db_client);
    let messaging_service = Arc::new(RwLock::new(MessagingService::new(message_repo)));
    messaging_service.clone().write().unwrap().start().await;

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, messaging_service.clone()));
    }

    Ok(())
}

async fn handle_connection(
    raw_stream: TcpStream,
    addr: SocketAddr,
    messaging_service: Arc<RwLock<MessagingService>>,
) {
    println!("Incoming TCP connection from: {}", addr);
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);
    let (outgoing, mut incoming) = ws_stream.split();
    let first_msg = incoming.next().await.unwrap().unwrap();
    let init_message: MessageDto = serde_json::from_slice(&first_msg.into_data()).unwrap();
    println!("Received init message {:?}", init_message);

    let msg_service_tx = messaging_service.read().unwrap().get_tx_channel();
    tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::SaveMessage {
            message: init_message,
            resp: resp_tx,
        };
        msg_service_tx.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });
}
