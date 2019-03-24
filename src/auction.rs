extern crate serde_json;

use crate::types::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bid {
    // BidderKey identifies the bidder placing this bid.
	#[serde(rename = "bidder")]
    bidder_key: Address,

	// BidCurrency specifies how much external currency the bidder
	// is putting in with this bid.
	#[serde(rename = "cur")]
    bid_currency: u64,

	// MaxPrice specifies the maximum price, in units of external
	// currency per Algo, that the bidder is willing to pay.
	// This must be at least as high as the current price of the
	// auction in the block in which this bid appears.
	#[serde(rename = "price")]
    max_price: u64,

	// BidID identifies this bid.  The first bid by a bidder (identified
	// by BidderKey) with a particular BidID on the blockchain will be
	// considered, preventing replay of bids.  Specifying a different
	// BidID allows the bidder to place multiple bids in an auction.
	#[serde(rename = "id")]
    bid_id: u64,

	// AuctionKey specifies the auction for this bid.
    #[serde(rename = "auc")]
	auction_key: Address,

	// AuctionID identifies the auction for which this bid is intended.
    #[serde(rename = "aid")]
	auction_id: u64
}

impl Bid {
    // TODO: wrap in a result/option
    pub fn make_bid(bidder_address: &String, bid_amount: &u64, max_price: &u64, bid_id: &u64, auction_address: &String, auction_id: &u64) -> Self {
        let bidder_addr = Address::decode(&bidder_address).unwrap();
        let auction_addr = Address::decode(&auction_address).unwrap();

        Bid {
            bidder_key: bidder_addr,
            bid_currency: *bid_amount,
            max_price: *max_price,
            bid_id: *bid_id,
            auction_key: auction_addr,
            auction_id: *auction_id
        }

    } 
}