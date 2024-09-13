use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

use bgapp::{
    dtos::message::MessageDto, prisma::PrismaClient, repository::message::MessageRepository,
    services::messaging::MessagingService,
};

type Tx = UnboundedSender<Message>;

type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
type AddrMap = Arc<Mutex<HashMap<String, SocketAddr>>>;
type AsyncMessagingService<'a> = Arc<MessagingService>;

// TODO: Key-value store to store chat history
// TODO: Chat server that managed 1:1 chat connections
// TODO: Push notification service to alert new messages
// TODO: DTO and model for message

#[tokio::main]
async fn main() -> Result<(), IoError> {
    // initiate database client connection
    let db_client = PrismaClient::_builder()
        .build()
        .await
        .expect("Failed to initialize db client");
    let message_repo = MessageRepository::new(db_client);
    let messaging_service: AsyncMessagingService = Arc::new(MessagingService::new(message_repo));

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let peer_map = PeerMap::new(Mutex::new(HashMap::new()));
    let addr_map = AddrMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        // Registers user in the chat pool
        tokio::spawn(handle_connection(
            peer_map.clone(),
            addr_map.clone(),
            stream,
            addr,
            Arc::clone(&messaging_service),
        ));
    }

    Ok(())
}

async fn handle_connection<'a>(
    peer_map: PeerMap,
    addr_map: AddrMap,
    raw_stream: TcpStream,
    addr: SocketAddr,
    messaging_service: AsyncMessagingService<'a>,
) {
    println!("Incoming TCP connection from: {}", addr);
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| async {
        println!(
            "Received a message from {}: {}",
            addr,
            msg.to_text().unwrap()
        );
        let message: MessageDto = serde_json::from_slice(&(msg.clone()).into_data())
            .expect("Failed to deserialize message");
        addr_map
            .lock()
            .unwrap()
            .insert(message.sender_id.clone(), addr);
        messaging_service.save_message(message).await;

        let peers = peer_map.lock().unwrap();

        // broadcast the message to everyone except ourselves.
        let broadcast_recipients = peers
            .iter()
            .filter(|(peer_addr, _)| peer_addr != &&addr)
            .map(|(_, ws_sink)| ws_sink);

        for recp in broadcast_recipients {
            recp.unbounded_send(msg.clone()).unwrap();
        }

        Ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}
