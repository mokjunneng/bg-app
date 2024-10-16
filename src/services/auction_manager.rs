use crate::repository::auction::AuctionRepository;

pub struct AuctionManagerService {
    auction_repo: AuctionRepository,
}
impl AuctionManagerService {
    pub fn new(auction_repo: AuctionRepository) -> Self {
        Self { auction_repo }
    }

    pub fn create_auction() {}

    pub fn place_bid_for_auction() {
        // Get aution aggregate from repository, rehydrate its state from events
        // Execute PlaceBid command on auction aggregate
        // Save auction via repository
    }
}

#[cfg(test)]
mod tests {
    use crate::{prisma::PrismaClient, repository::auction::AuctionRepository};

    use super::*;

    #[tokio::test]
    async fn create_auction() {
        let db_client = PrismaClient::_builder()
            .build()
            .await
            .expect("Failed to initialize db client");
        let auction_repo = AuctionRepository::new(db_client);
        let auction_manager_service = AuctionManagerService::new(auction_repo);
    }
}
