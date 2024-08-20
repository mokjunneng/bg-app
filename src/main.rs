use std::error::Error;

use bgapp::models::{auction, auction_item, price, user};

fn main() -> Result<(), Box<dyn Error>> {
    let user = user::generate_user();
    let auction_item = auction_item::AuctionItem::new(String::from("Brass Birmingham"), &user);
    let mut curr_auction = auction::Auction::new(
        &auction_item,
        price::Price::new(price::Currency::SGD, 10),
        Some(5),
        None,
    );

    let bid = auction::Bid::new(&user, price::Price::new(price::Currency::SGD, 10));
    Ok(curr_auction.place_bid(bid)?)
}
