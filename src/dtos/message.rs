use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDto {
    pub sender_id: u32,
    pub recipient_id: u32,
    // TODO: Encode content semantics as type - e.g. @ mentions, reply to
    pub content: String,
    // created_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_message_dto_from_bytes() {
        let mut stdin = std::io::stdin();
        let mut buf = vec![0; 1024];
        let n = stdin.read(&mut buf).expect("Error reading stdin");
        buf.truncate(n);
        let message_dto: MessageDto = serde_json::from_slice(&buf).expect("Failed to deserialize");
        println!("Deserialized message DTO = {:?}", message_dto);
    }

    #[test]
    fn serialize_message_dto_into_bytes() {
        let message_dto = MessageDto {
            sender_id: 1,
            recipient_id: 2,
            content: "Hello back!".to_string(),
        };
        let buf = serde_json::to_vec(&message_dto).expect("Failed to serialize");
        println!("Serialized message DTO = {:?}", buf);
    }
}
