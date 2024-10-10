use super::{
    bid::Bid,
    price::{Currency, Price},
};

pub type AuctionId = u32;

pub struct AuctionAggregate {
    state: AuctionState,
    domain_events: Vec<AuctionEvent>,
}
impl AuctionAggregate {
    pub fn new(events: Vec<AuctionEvent>) -> Self {
        // TODO: Randomly generate auction id
        let mut state = AuctionState::new();
        // Rehydrate state from events
        for event in events.iter() {
            println!(
                "Applying event id {} of type {:?}..",
                &event.event_id, &event.event_type
            );
            state.apply(event);
        }
        println!("Rehydrated state: {:?}", state);
        Self {
            state,
            domain_events: events,
        }
    }

    pub fn execute(&self, cmd: Box<dyn Command>) {
        match cmd.get_type() {
            AuctionCommandType::CreateAuction => self.create_auction(),
            AuctionCommandType::StartAuction => self.start_auction(),
            AuctionCommandType::CloseAuction => self.close_auction(),
            AuctionCommandType::MakeBidOffer => self.offer_bid_for_auction(),
        }
    }

    fn create_auction(&self) {}

    fn start_auction(&self) {}

    fn close_auction(&self) {}

    fn offer_bid_for_auction(&self) {}
}

// All events added to aggregate's events collection are passed to the state projection logic
// under this class where the relevant field's values are mutated to the events' data
#[derive(Debug)]
pub struct AuctionState {
    id: AuctionId,
    bids: Vec<Bid>,
}
impl AuctionState {
    pub fn new() -> Self {
        Self {
            id: 0,
            bids: vec![],
        }
    }

    pub fn apply(&mut self, event: &AuctionEvent) {
        match event.event_type {
            AuctionEventType::AuctionCreated => self.id = event.auction_id,
            AuctionEventType::AuctionStarted => println!("Auction started"),
            AuctionEventType::AuctionClosed => println!("Auction closed!"),
            AuctionEventType::BidOffered => match &event.bid {
                Some(bid) => self.bids.push(bid.clone()),
                None => println!("No bid found for event"),
            },
            _ => println!("Received unknown event"),
        }
    }
}

// NOTE: Does it make sense to generalize with trait like that?
pub trait Command {
    fn get_type(&self) -> AuctionCommandType;
}

pub struct CreateAuctionCommand {
    command_type: AuctionCommandType,
}
impl Command for CreateAuctionCommand {
    fn get_type(&self) -> AuctionCommandType {
        self.command_type.clone()
    }
}

pub struct StartAuctionCommand {
    command_type: AuctionCommandType,
}
impl Command for StartAuctionCommand {
    fn get_type(&self) -> AuctionCommandType {
        self.command_type.clone()
    }
}

pub struct CloseAuctionCommand {
    command_type: AuctionCommandType,
}
impl Command for CloseAuctionCommand {
    fn get_type(&self) -> AuctionCommandType {
        self.command_type.clone()
    }
}

pub struct MakeBidOfferCommand {
    command_type: AuctionCommandType,
}
impl Command for MakeBidOfferCommand {
    fn get_type(&self) -> AuctionCommandType {
        self.command_type.clone()
    }
}

// COMMANDS
#[derive(Clone)]
pub enum AuctionCommandType {
    CreateAuction,
    StartAuction,
    CloseAuction,
    MakeBidOffer,
}

// DOMAIN EVENTS

// BidReceived

pub struct AuctionEvent {
    // Event information
    event_id: EventId,
    event_type: AuctionEventType,

    // Auction information
    auction_id: AuctionId,

    // Bid information for auction
    bid: Option<Bid>,
}

type EventId = u32;

#[derive(Debug)]
enum AuctionEventType {
    AuctionCreated,
    AuctionStarted,
    AuctionInProgress,
    // Auction's deadline approaching
    AuctionEnding,
    // Auction's deadline reached
    AuctionEnded,
    // Auction is closed by owner prematurely
    AuctionClosed,
    BidOffered,
    HeadshotBidPlaced, // TODO: Maybe don't need this as can infer from BidPlaced
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_auction_aggregate_from_events() {
        let auction_created_event = AuctionEvent {
            event_id: 1,
            event_type: AuctionEventType::AuctionCreated,
            auction_id: 1,
            bid: None,
        };
        let auction_started_event = AuctionEvent {
            event_id: 2,
            event_type: AuctionEventType::AuctionStarted,
            auction_id: 1,
            bid: None,
        };
        let bid_offered_event = AuctionEvent {
            event_id: 3,
            event_type: AuctionEventType::BidOffered,
            auction_id: 1,
            bid: Some(Bid::new(Price::new(Currency::MYR, 100))),
        };

        let domain_events = vec![
            auction_created_event,
            auction_started_event,
            bid_offered_event,
        ];

        let _auction_aggregate = AuctionAggregate::new(domain_events);
    }
}
