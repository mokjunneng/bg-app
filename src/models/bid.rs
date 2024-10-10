use chrono::{DateTime, Utc};

use super::price::Price;

#[derive(Debug, Clone)]
pub struct Bid {
    created_at: DateTime<Utc>,
    increment: Option<u32>,
    pub price: Price,
}
impl Bid {
    pub fn new(price: Price) -> Self {
        Self {
            created_at: Utc::now(),
            increment: None,
            price,
        }
    }

    fn set_increment(&mut self, increment: u32) {
        self.increment = Some(increment);
    }
}
