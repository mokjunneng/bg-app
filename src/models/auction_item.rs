use crate::models::user::User;

pub type AuctionItemId = u32;

pub struct AuctionItem<'a> {
    id: AuctionItemId,
    name: String,
    owner: &'a User,
}

impl<'a> AuctionItem<'a> {
    pub fn new(id: AuctionItemId, name: String, owner: &'a User) -> Self {
        Self { id, name, owner }
    }

    pub fn get_id(&self) -> AuctionItemId {
        self.id
    }
}
