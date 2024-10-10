use std::collections::HashMap;

use tokio::sync::{broadcast, oneshot};
use tokio_tungstenite::tungstenite::Message;

use crate::dtos::message::MessageDto;

type Responder<T> = oneshot::Sender<T>;
#[derive(Debug)]
pub enum Command {
    SaveMessage {
        message: MessageDto,
        // TODO: DB response
        resp: Responder<String>,
    },
}

type UserId = u32;
type ChatId = u32;
type Channel = (broadcast::Sender<Message>, broadcast::Receiver<Message>);

pub struct ChatSession {
    id: ChatId,
    users: Vec<UserId>,
    channels: HashMap<UserId, Channel>,
}
impl ChatSession {
    pub fn new(id: ChatId, users: Vec<UserId>) -> ChatSession {
        let mut channels = HashMap::new();
        for user in users.iter() {
            let (tx, rx) = broadcast::channel(2);
            channels.insert(*user, (tx, rx));
        }
        ChatSession {
            id,
            users,
            channels,
        }
    }

    pub fn get_user_receiving_channel(&self, user_id: UserId) -> broadcast::Receiver<Message> {
        let (tx, _) = self.channels.get(&user_id).unwrap();
        tx.subscribe()
    }

    pub fn get_broadcast_channels(&self) -> Vec<broadcast::Sender<Message>> {
        let mut channels = vec![];
        for user in self.users.iter() {
            let (tx, _) = self.channels.get(&user).unwrap();
            channels.push(tx.clone());
        }
        channels
    }
}

pub struct MessagingService {
    next_chat_id: ChatId,
    // TechDebt: is there a better data structure?
    one_on_one_chat_registry: HashMap<(UserId, UserId), ChatId>,
    chat_sessions: HashMap<ChatId, ChatSession>,
}

impl MessagingService {
    pub fn new() -> Self {
        Self {
            next_chat_id: 1,
            one_on_one_chat_registry: HashMap::new(),
            chat_sessions: HashMap::new(),
        }
    }

    pub fn register_one_on_one_chat(&mut self, user1_id: UserId, user2_id: UserId) -> ChatId {
        match self.one_on_one_chat_registry.get(&(user1_id, user2_id)) {
            Some(chat_id) => {
                println!(
                    "Found existing chat between {} and {}. Skip registration.",
                    &user1_id, &user2_id
                );
                *chat_id
            }
            None => match self.one_on_one_chat_registry.get(&(user2_id, user1_id)) {
                Some(chat_id) => {
                    println!(
                        "Found existing chat between {} and {}. Skip registration.",
                        &user1_id, &user2_id
                    );
                    *chat_id
                }
                None => {
                    self.one_on_one_chat_registry
                        .insert((user1_id, user2_id), self.next_chat_id);
                    let chat_session =
                        ChatSession::new(self.next_chat_id, vec![user1_id, user2_id]);
                    let chat_session_id = self.next_chat_id;
                    self.next_chat_id += 1;
                    self.chat_sessions.insert(chat_session.id, chat_session);
                    println!("Registered chat for {} and {}", &user1_id, &user2_id);
                    chat_session_id
                }
            },
        }
    }

    pub fn get_chat_session(&self, chat_id: ChatId) -> &ChatSession {
        self.chat_sessions.get(&chat_id).unwrap()
    }
}
