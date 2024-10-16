use crate::{
    models::auction_aggregate::{AuctionAggregate, AuctionId},
    prisma::PrismaClient,
};

// Event-sourced model
pub struct AuctionRepository {
    db_client: PrismaClient,
}
impl AuctionRepository {
    pub fn new(db_client: PrismaClient) -> Self {
        Self { db_client }
    }

    pub fn load_events(&self, auction_id: AuctionId) {
        // Get events from event store
    }

    pub fn commit_changes(&self, auction_aggregate: AuctionAggregate) {
        // Persist changes
    }
}
