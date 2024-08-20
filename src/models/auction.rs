use crate::models::auction_item::AuctionItem;
use chrono::{DateTime, Utc};

use crate::models::price::{Currency, Price};

use super::user::{generate_user, User};

pub struct Auction<'a> {
    auction_item: &'a AuctionItem<'a>,
    bidding_rules: Vec<Box<dyn Rule>>,
    bids_received: Vec<Bid<'a>>,
    opening_bid_price: Price,
}
impl<'a> Auction<'a> {
    pub fn new(
        auction_item: &'a AuctionItem,
        opening_bid_price: Price,
        minimum_bid_increment: Option<u32>,
        headshot_bid_amount: Option<u32>,
    ) -> Self {
        let same_currency_rule = SameBidCurrency {
            currency_to_follow: opening_bid_price.currency,
        };
        let min_bid_increment_rule = MinimumBidIncrement {
            // Default min increment of 1
            min_increment: minimum_bid_increment.unwrap_or(1),
        };
        let max_bid_rule = MaximumBid {
            maximum_bid: headshot_bid_amount,
        };
        Self {
            auction_item,
            bidding_rules: vec![
                Box::new(same_currency_rule),
                Box::new(min_bid_increment_rule),
                Box::new(max_bid_rule),
            ],
            bids_received: vec![],
            opening_bid_price,
        }
    }

    pub fn place_bid(&mut self, mut bid: Bid) -> Result<(), String> {
        // TODO: Shouldn't allow owner of auction item to bid for item?
        let bid_placed = match self.bids_received.last() {
            None => {
                // First bid, should at least match opening bid price
                if bid.price < self.opening_bid_price {
                    return Err(
                        "first bid must be at least as high as the stipulated opening bid".into(),
                    );
                }
                bid.set_increment(bid.price.value);
                bid
            }
            Some(last_bid) => {
                let increment = bid.price.value - last_bid.price.value;
                bid.set_increment(increment);
                bid
            }
        };
        println!("Bid placed : {:#?}", bid_placed);
        for rule in &self.bidding_rules {
            rule.enforce(&bid_placed)?
        }
        Ok(())
    }
}

trait Rule {
    fn enforce(&self, bid: &Bid) -> Result<(), String>;
}

struct SameBidCurrency {
    currency_to_follow: Currency,
}
impl Rule for SameBidCurrency {
    fn enforce(&self, bid: &Bid) -> Result<(), String> {
        if bid.price.currency != self.currency_to_follow {
            return Err("bid's price currency does not match with set currency".into());
        }
        Ok(())
    }
}

struct MinimumBidIncrement {
    min_increment: u32,
}
impl Rule for MinimumBidIncrement {
    fn enforce(&self, bid: &Bid) -> Result<(), String> {
        if bid.increment.unwrap() < self.min_increment {
            return Err(
                "bid's increment must be at least as high as the set minimum bid increment".into(),
            );
        }
        Ok(())
    }
}

struct MaximumBid {
    maximum_bid: Option<u32>,
}
impl Rule for MaximumBid {
    fn enforce(&self, bid: &Bid) -> Result<(), String> {
        match self.maximum_bid {
            None => Ok(()),
            Some(value) => {
                if bid.price.value > value {
                    return Err("bid has exceeded the maximum bid value".into());
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct Bid<'a> {
    by: &'a User,
    created_at: DateTime<Utc>,
    increment: Option<u32>,
    pub price: Price,
}
impl<'a> Bid<'a> {
    pub fn new(by: &'a User, price: Price) -> Self {
        Self {
            by,
            created_at: Utc::now(),
            increment: None,
            price,
        }
    }

    fn set_increment(&mut self, increment: u32) {
        self.increment = Some(increment);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_bid_increment_rule() {
        let user = generate_user();
        let rule = MinimumBidIncrement { min_increment: 10 };
        let mut bid = Bid::new(&user, Price::new(Currency::SGD, 10));
        bid.set_increment(10);
        assert!(rule.enforce(&bid).is_ok());

        // Should return err if increment is less than min_increment
        bid.set_increment(8);
        assert!(rule.enforce(&bid).is_err());
    }

    #[test]
    fn invalid_bid_less_than_opening_bid() {
        let user = generate_user();
        let auction_item = AuctionItem::new(String::from("Brass Birmingham"), &user);
        let mut auction = Auction::new(&auction_item, Price::new(Currency::SGD, 10), Some(5), None);

        let invalid_bid = Bid::new(&user, Price::new(Currency::SGD, 8));
        assert!(auction.place_bid(invalid_bid).is_err());
    }
}
