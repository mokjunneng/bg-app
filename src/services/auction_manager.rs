pub struct AuctionManagerService {}
impl AuctionManagerService {
    pub fn create_auction() {}

    pub fn place_bid_for_auction() {
        // Get aution aggregate from repository, rehydrate its state from events
        // Execute PlaceBid command on auction aggregate
        // Save auction via repository
    }
}
