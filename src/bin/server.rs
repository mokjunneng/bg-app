use std::{env, io::Error as IoError, net::SocketAddr, sync::Arc};

use futures_util::{future, pin_mut, SinkExt, StreamExt, TryStreamExt};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot, RwLock},
};

use bgapp::{
    dtos::message::MessageDto,
    prisma::PrismaClient,
    repository::message::MessageRepository,
    services::messaging::{Command, MessagingService},
};
use tokio_stream::wrappers::BroadcastStream;
use tokio_tungstenite::tungstenite::Message;

// TODO: Push notification service to alert new messages
// TODO: DTO and model for message

#[tokio::main]
async fn main() -> Result<(), IoError> {
    // Create mpsc channel and spawn tokio task to handle commands
    let (commands_tx, mut commands_rx) = mpsc::channel::<Command>(32);
    tokio::spawn(async move {
        let db_client = PrismaClient::_builder()
            .build()
            .await
            .expect("Failed to initialize db client");
        let message_repo = MessageRepository::new(db_client);

        while let Some(cmd) = commands_rx.recv().await {
            match cmd {
                Command::SaveMessage { message, resp } => {
                    let _res = message_repo.upsert_many(vec![message]).await;
                    let _ = resp.send("success!".to_string());
                }
            }
        }
    });

    let messaging_service = Arc::new(RwLock::new(MessagingService::new()));

    // Handle all incoming WS connections
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    // Create the event loop and TCP listener we'll accept connections on.
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(
            stream,
            addr,
            Arc::clone(&messaging_service),
            commands_tx.clone(),
        ));
    }

    Ok(())
}

async fn handle_connection(
    raw_stream: TcpStream,
    addr: SocketAddr,
    messaging_service: Arc<RwLock<MessagingService>>,
    commands_tx: mpsc::Sender<Command>,
) {
    println!("Incoming TCP connection from: {}", addr);
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);
    let (mut outgoing, mut incoming) = ws_stream.split();

    let first_msg = incoming.next().await.unwrap().unwrap();
    let init_message: MessageDto = serde_json::from_slice(&first_msg.into_data()).unwrap();
    println!("Received init message {:?}", init_message);

    // TODO: Default registration of one on one chat now; Handle group chat too
    println!("Retrieving write lock to register chat");

    // Get write lock to register chat
    let chat_session_id;
    {
        println!("Getting write lock on messaging service to register chat.");
        let mut messaging_service_write_lock = messaging_service.write().await;
        chat_session_id = messaging_service_write_lock
            .register_one_on_one_chat(init_message.sender_id, init_message.recipient_id);
    }

    let sender_id = init_message.sender_id;
    let broadcast_targets;
    let sender_rx;
    {
        let messaging_service_read_lock = messaging_service.read().await;
        let chat_session = messaging_service_read_lock.get_chat_session(chat_session_id);
        broadcast_targets = chat_session.get_broadcast_channels(sender_id);
        sender_rx = chat_session.get_user_receiving_channel(sender_id);
    }

    // let broadcast_incoming = incoming.try_for_each(|msg| async move {
    //     println!(
    //         "Received a message from {}: {}",
    //         &sender_id,
    //         &msg.to_text().unwrap()
    //     );
    //
    //     broadcast_targets.into_iter().for_each(|target| {
    //         target
    //             .send(Message::Binary(msg.clone().into_data()))
    //             .unwrap();
    //     });
    //
    //     let message: MessageDto =
    //         serde_json::from_slice(&msg.into_data()).expect("Failed to deserialize message");
    //
    //     // save message
    //     let (resp_tx, resp_rx) = oneshot::channel();
    //     let cmd = Command::SaveMessage {
    //         message,
    //         resp: resp_tx,
    //     };
    //     commands_tx.send(cmd).await.unwrap();
    //
    //     let res = resp_rx.await;
    //     println!("Received response from SaveMessage command: {:?}", res);
    //     Ok(())
    // });
    let receive_from_others = BroadcastStream::new(sender_rx).try_for_each(|msg| {
        let _ = outgoing.send(msg);
        future::ok(())
    });
    let _ = receive_from_others.await;

    while let msg = incoming.next().await.unwrap().unwrap() {
        println!(
            "Received a message from {}: {}",
            &sender_id,
            &msg.to_text().unwrap()
        );

        broadcast_targets.iter().for_each(|target| {
            target
                .send(Message::Binary(msg.clone().into_data()))
                .unwrap();
        });

        let message: MessageDto =
            serde_json::from_slice(&msg.into_data()).expect("Failed to deserialize message");

        // save message
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::SaveMessage {
            message,
            resp: resp_tx,
        };
        commands_tx.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("Received response from SaveMessage command: {:?}", res);
    }
}
