use std::{collections::HashMap, error::Error, net::SocketAddr};

use futures_util::{future, stream::TryStreamExt, SinkExt, StreamExt};
use tokio::{
    net::TcpStream,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{dtos::message::MessageDto, repository::message::MessageRepository};

type UserId = String;

pub struct MessagingService {
    message_repo: MessageRepository,
    chats: HashMap<UserId, (UnboundedSender<Message>, UnboundedReceiver<Message>)>,
}

impl MessagingService {
    pub fn new(message_repo: MessageRepository) -> Self {
        Self {
            message_repo,
            chats: HashMap::new(),
        }
    }

    pub fn register_user(&mut self, user_id: String) {
        if let Some(_) = self.chats.get(&user_id) {
            println!("Chat of {} has already been registered!", &user_id)
        }
        let (tx, rx) = unbounded_channel();
        self.chats.insert(user_id, (tx, rx));
    }

    pub fn start_live_chat(
        &self,
        ws_stream: WebSocketStream<TcpStream>,
        sender_id: String,
        recipient_id: String,
    ) -> Result<(), Error> {
        let (outgoing, incoming) = ws_stream.split();
        let Some((sender_tx, mut sender_rx)) = self.chats.get(&sender_id) else {
            println!("User {} is not registered.", &sender_id);
            return Err("User not registered");
        };
        let broadcast_incoming = incoming.try_for_each(|msg| {
            println!(
                "Received a message from {}: {}",
                &sender_id,
                &msg.to_text().unwrap()
            );
            let message: MessageDto =
                serde_json::from_slice(&msg.into_data()).expect("Failed to deserialize message");
            // save message in chat history
            sender_tx.send(msg.clone()).unwrap();

            future::ok(())
        });
        let mut recv_buffer: Vec<&str> = Vec::with_capacity(5);
        let limit = 5;
        let receive_from_others = outgoing.send_all(sender_rx);
        // let receive_from_others = sender_rx.recv_many(recv_buffer, limit).map(|msg| {
        //     if let Some(data) = msg {
        //     }
        // })
    }

    pub async fn send_message(&self, message: MessageDto) {}

    async fn save_message(&self, message: MessageDto) {
        self.message_repo.upsert_many(vec![message]).await;
    }
}

type ChatUser = (String, SocketAddr, UnboundedSender<Message>);

// Users register their conenctions
// 1. User A initiates a chat with User B
// 2. User A sends message(s) to chat
// 3. User B opens chat and send message to chat
