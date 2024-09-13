use std::collections::HashMap;

use futures_util::{future, pin_mut, stream::TryStreamExt, SinkExt, StreamExt};
use tokio::{
    net::TcpStream,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{dtos::message::MessageDto, repository::message::MessageRepository};

type UserId = String;

pub struct MessagingService {
    message_repo: MessageRepository,
    chats: HashMap<
        (UserId, UserId), // Key to identify 1:1 session
        // HashMap to identify which channel to use
        HashMap<UserId, (UnboundedSender<Message>, UnboundedReceiver<Message>)>,
    >,
}

impl MessagingService {
    pub fn new(message_repo: MessageRepository) -> Self {
        Self {
            message_repo,
            chats: HashMap::new(),
        }
    }

    pub fn register_chat(&mut self, user1_id: String, user2_id: String) {
        if let Some(_) = self.chats.get(&(user1_id.clone(), user2_id.clone())) {
            println!(
                "Chat of {}-{} has already been registered!",
                &user1_id, &user2_id
            )
        }
        // Create the bidirectional channel
        let (user1_tx, user1_rx) = unbounded_channel();
        let (user2_tx, user2_rx) = unbounded_channel();
        self.chats.insert(
            (user1_id.clone(), user2_id.clone()),
            HashMap::from([
                (user1_id, (user1_tx, user1_rx)),
                (user2_id, (user2_tx, user2_rx)),
            ]),
        );
    }

    pub fn start_live_chat(
        &self,
        ws_stream: WebSocketStream<TcpStream>,
        sender_id: String,
        recipient_id: String,
    ) {
        let (mut outgoing, incoming) = ws_stream.split();
        let Some(channels) = self.chats.get(&(sender_id.clone(), recipient_id.clone())) else {
            println!(
                "Chat between {} and {} has not been registered.",
                &sender_id, &recipient_id
            );
            return;
        };
        let (sender_tx, _sender_rx) = channels.get(&sender_id).unwrap();
        let (_recipient_tx, recipient_rx) = channels.get(&recipient_id).unwrap();

        let broadcast_incoming = incoming.try_for_each(|msg| {
            println!(
                "Received a message from {}: {}",
                &sender_id,
                &msg.to_text().unwrap()
            );
            let message: MessageDto = serde_json::from_slice(&msg.clone().into_data())
                .expect("Failed to deserialize message");
            // save message in chat history
            self.save_message(message);
            sender_tx.send(Message::Binary(msg.into_data())).unwrap();

            future::ok(())
        });
        let mut receive_from_others = || async {
            while let Some(msg) = recipient_rx.recv().await {
                outgoing.send(msg).await.unwrap();
            }
        };
        let broadcast_outgoing = receive_from_others();

        pin_mut!(broadcast_incoming, broadcast_outgoing);
        future::select(broadcast_incoming, broadcast_outgoing);
    }

    async fn save_message(&self, message: MessageDto) {
        self.message_repo.upsert_many(vec![message]).await;
    }
}
