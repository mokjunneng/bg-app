use std::collections::HashMap;

use futures_util::{
    future, pin_mut,
    stream::{SplitSink, SplitStream},
    Future, SinkExt, StreamExt, TryStreamExt,
};
use tokio::{
    net::TcpStream,
    sync::{
        broadcast::{self, Receiver, Sender},
        mpsc, oneshot,
    },
};
use tokio_stream::wrappers::BroadcastStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{dtos::message::MessageDto, repository::message::MessageRepository};

type Responder<T> = oneshot::Sender<T>;
#[derive(Debug)]
pub enum Command {
    SaveMessage {
        message: MessageDto,
        // TODO: DB response
        resp: Responder<String>,
    },
}

pub struct MessagingService {
    message_repo: MessageRepository,
    commands_tx: mpsc::Sender<Command>,
    commands_rx: mpsc::Receiver<Command>,
}

impl MessagingService {
    pub fn new(message_repo: MessageRepository) -> Self {
        // TODO: Define proper channel capacity
        let (tx, rx) = mpsc::channel(32);
        Self {
            message_repo,
            commands_tx: tx,
            commands_rx: rx,
        }
    }

    pub fn get_tx_channel(&self) -> mpsc::Sender<Command> {
        self.commands_tx.clone()
    }

    pub async fn start(&mut self) {
        while let Some(cmd) = self.commands_rx.recv().await {
            match cmd {
                Command::SaveMessage { message, resp } => {
                    let _res = self.message_repo.upsert_many(vec![message]).await;
                    // TODO: Return proper DB response
                    let _ = resp.send("success!".to_string());
                }
            }
        }
    }
}

// type UserId = u32;
// type ChatId = u32;
// type Channel = (Sender<Message>, Receiver<Message>);
//
// pub struct ChatSession {
//     id: ChatId,
//     users: Vec<UserId>,
//     channels: HashMap<UserId, Channel>,
// }
// impl ChatSession {
//     pub fn new(id: ChatId, users: Vec<UserId>) -> ChatSession {
//         let mut channels = HashMap::new();
//         for user in users.iter() {
//             let (tx, rx) = broadcast::channel(2);
//             channels.insert(*user, (tx, rx));
//         }
//         ChatSession {
//             id,
//             users,
//             channels,
//         }
//     }
//
//     pub fn get_user_receiving_channel(&self, user_id: UserId) -> Receiver<Message> {
//         let (tx, _) = self.channels.get(&user_id).unwrap();
//         tx.subscribe()
//     }
//
//     // Return all users' channels except the sender's channel to broadcast into
//     pub fn get_broadcast_channels(&self, user_id: UserId) -> Vec<&Sender<Message>> {
//         self.users
//             .clone()
//             .into_iter()
//             .filter(|id| *id != user_id)
//             .map(|id| &self.channels.get(&id).unwrap().0)
//             .collect()
//     }
// }
//
// pub struct MessagingService {
//     message_repo: MessageRepository,
//     next_chat_id: ChatId,
//     // TechDebt: is there a better data structure?
//     one_on_one_chat_registry: HashMap<(UserId, UserId), ChatId>,
//     chat_sessions: HashMap<ChatId, ChatSession>,
// }
//
// impl MessagingService {
//     pub fn new(message_repo: MessageRepository) -> Self {
//         Self {
//             message_repo,
//             next_chat_id: 1,
//             one_on_one_chat_registry: HashMap::new(),
//             chat_sessions: HashMap::new(),
//         }
//     }
//
//     pub fn register_one_on_one_chat(&mut self, user1_id: UserId, user2_id: UserId) -> ChatId {
//         match self.one_on_one_chat_registry.get(&(user1_id, user2_id)) {
//             Some(chat_id) => *chat_id,
//             None => match self.one_on_one_chat_registry.get(&(user2_id, user1_id)) {
//                 Some(chat_id) => *chat_id,
//                 None => {
//                     let chat_session =
//                         ChatSession::new(self.next_chat_id, vec![user1_id, user2_id]);
//                     let chat_session_id = self.next_chat_id;
//                     self.next_chat_id += 1;
//                     self.chat_sessions.insert(chat_session.id, chat_session);
//                     chat_session_id
//                 }
//             },
//         }
//     }
//
//     pub async fn start_live_chat(
//         &self,
//         mut outgoing: SplitSink<WebSocketStream<TcpStream>, Message>,
//         incoming: SplitStream<WebSocketStream<TcpStream>>,
//         chat_session_id: ChatId,
//         sender_id: UserId,
//     ) {
//         let Some(chat_session) = self.chat_sessions.get(&chat_session_id) else {
//             println!("Chat {} has not been registered.", chat_session_id);
//             return;
//         };
//
//         let broadcast_incoming = incoming.try_for_each(|msg| async {
//             println!(
//                 "Received a message from {}: {}",
//                 &sender_id,
//                 &msg.to_text().unwrap()
//             );
//
//             let broadcast_targets = chat_session.get_broadcast_channels(sender_id);
//             broadcast_targets.into_iter().for_each(|target| {
//                 target
//                     .send(Message::Binary(msg.clone().into_data()))
//                     .unwrap();
//             });
//
//             let message: MessageDto =
//                 serde_json::from_slice(&msg.into_data()).expect("Failed to deserialize message");
//             // save message in chat history
//             self.save_message(message).await;
//             Ok(())
//         });
//
//         let sender_rx = BroadcastStream::new(chat_session.get_user_receiving_channel(sender_id));
//         let receive_from_others = sender_rx.try_for_each(|msg| {
//             outgoing.send(msg);
//             future::ok(())
//         });
//
//         pin_mut!(broadcast_incoming, receive_from_others);
//         future::select(broadcast_incoming, receive_from_others).await;
//     }
//
//     async fn save_message(&self, message: MessageDto) {
//         self.message_repo.upsert_many(vec![message]).await;
//     }
// }
