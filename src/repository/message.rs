use prisma_client_rust::{
    bigdecimal::ToPrimitive, prisma_errors::query_engine::UniqueKeyViolation,
};

use crate::{
    dtos::message::MessageDto,
    prisma::{message, PrismaClient},
};

pub struct MessageRepository {
    client: PrismaClient,
}

impl MessageRepository {
    pub fn new(client: PrismaClient) -> Self {
        Self { client }
    }

    pub async fn upsert_many(&self, messages: Vec<MessageDto>) {
        let messages = self
            .client
            .message()
            .create_many(
                messages
                    .iter()
                    .map(|message| {
                        message::create_unchecked(
                            message.sender_id.to_i32().unwrap(),
                            message.recipient_id.to_i32().unwrap(),
                            message.content.clone(),
                            vec![],
                        )
                    })
                    .collect(),
            )
            .exec()
            .await;

        match messages {
            Ok(_messages) => println!("Messages inserted"),
            Err(error) if error.is_prisma_error::<UniqueKeyViolation>() => println!("Prisma error"),
            Err(_error) => println!("Other error occurred"),
        }
    }
}
