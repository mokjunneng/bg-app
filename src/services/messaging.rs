use std::{collections::HashMap, net::SocketAddr};

use futures_util::StreamExt;
use tokio::{
    net::TcpStream,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{dtos::message::MessageDto, repository::message::MessageRepository};

pub struct MessagingService {
    message_repo: MessageRepository,
    chats: HashMap<ChatUser, Vec<ChatUser>>,
    peer_map: HashMap<
        String,
        (
            SocketAddr,
            UnboundedReceiver<Message>,
            UnboundedSender<Message>,
        ),
    >,
}

impl MessagingService {
    pub fn new(message_repo: MessageRepository) -> Self {
        Self {
            message_repo,
            chats: HashMap::new(),
            peer_map: HashMap::new(),
        }
    }

    pub fn register_user(&mut self, user_id: String, socket_addr: SocketAddr) {
        let (tx, rx) = unbounded_channel();
        self.peer_map.insert(user_id, (socket_addr, rx, tx));
    }

    pub fn start_chat(
        &self,
        ws_stream: WebSocketStream<TcpStream>,
        sender_id: String,
        recipient_id: String,
    ) {
        let (outgoing, incoming) = ws_stream.split();
        let (sender_socket_addr, sender_rx, sender_tx) = self.peer_map.get(&sender_id);
        let (recipient_socket_addr, recipient_rx, recipient_tx) = self.peer_map.get(&recipient_id);
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
