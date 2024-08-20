use crate::models::user::User;

pub struct AuctionItem<'a> {
    name: String,
    owner: &'a User,
}
impl<'a> AuctionItem<'a> {
    pub fn new(name: String, owner: &'a User) -> Self {
        Self { name, owner }
    }
}
