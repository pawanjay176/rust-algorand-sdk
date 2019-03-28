extern crate ed25519_dalek;
extern crate rmp_serde;
extern crate ring;
extern crate data_encoding;

use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::Sha512;
use ring::{digest};
use data_encoding::{BASE32_NOPAD};
use rmp_serde::{Deserializer, Serializer};
use crate::transaction::{Transaction, SignedTxn};
use crate::auction::Bid;

const TXID_PREFIX: &str = "TX";
const BID_PREFIX: &str = "aB";

// SignTransaction accepts a keypair and a transaction, and returns the
// bytes of a signed transaction ready to be broadcasted to the network
pub fn sign_transaction(kp: &Keypair, tx: Transaction) -> (String, Vec<u8>) {
    // Encode and prepend
	let mut encoded_tx = Vec::new();
	tx.serialize(&mut Serializer::new(&mut encoded_tx)).unwrap();
	let mut to_sign = String::from(TXID_PREFIX).as_bytes().to_vec();
	to_sign.extend(encoded_tx);

	// Sign encoded tx
	let signature = kp.sign::<Sha512>(&to_sign);
	
	// Construct signed tx
	let stx = SignedTxn {
		sig: signature,
		msig: None,
		txn: tx
	};

	// Encode signed tx
	let mut stx_bytes = Vec::new();
	stx.serialize(&mut Serializer::new(&mut stx_bytes)).unwrap();

	// Compute tx id
	let txid_bytes = digest::digest(&digest::SHA512_256, &to_sign);
    let txid_bytes = txid_bytes.as_ref();

	let txid = BASE32_NOPAD.encode(&txid_bytes).to_string();
	(txid, stx_bytes.to_vec())
}

// SignBid accepts a private key and a bid, and returns the signature of the
// bid under that key
pub fn sign_bid(kp: &Keypair, bid: Bid) -> Vec<u8> {
    // Encode and prepend
	let mut encoded_bid = Vec::new();
	bid.serialize(&mut Serializer::new(&mut encoded_bid)).unwrap();
	let mut to_sign = String::from(BID_PREFIX).as_bytes().to_vec();
	to_sign.extend(encoded_bid);

	// Sign encoded tx
	let signature = kp.sign::<Sha512>(&to_sign);
	signature.to_bytes().to_vec()
	
}