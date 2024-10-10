use crate::models::auction_aggregate::{AuctionAggregate, AuctionId};

// Event-sourced model
pub struct AuctionRepository {}
impl AuctionRepository {
    pub fn load_events(auction_id: AuctionId) {
        // Get events from event store
    }

    pub fn commit_changes(auction_aggregate: AuctionAggregate) {
        // Persist changes
    }
}
