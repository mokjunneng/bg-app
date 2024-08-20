use std::fmt;

#[derive(Copy, Clone)]
enum Currency {
    MYR,
    SGD,
}

enum ItemCondition {
    HeavilyUsed,
    WellUsed,
    LikeNew,
    New,
}

#[derive(Copy, Clone)]
struct Price {
    currency: Currency,
    value: u32,
}

struct AuctionItem {
    name: String,
    owner: String, // TODO: create new Owner struct
    description: String,
    condition: ItemCondition, // TODO: Maybe consider more verbose struct
}

struct Auction {
    item: AuctionItem,
    created_at: String,
    starting_bid: Price,
    headshot_bid: Price,
    bids: Vec<Price>,
}

impl fmt::Display for Auction {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "({}, {})", self.longitude, self.latitude)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Auction for item(name: {}, owner: {}, condition: {:?}). Bid structure ({:?}, {:?})",
            self.item.name,
            self.item.owner,
            self.item.condition,
            self.starting_bid,
            self.headshot_bid
        )
    }
}

impl Auction {
    fn place_bid(&mut self, bid: u32) {
        self.bids.push(Price {
            currency: self.starting_bid.currency,
            value: bid,
        });
    }
}

// Features
// - create new auction
// - bid for auction
