extern crate serde_json;
extern crate ed25519_dalek;

use crate::basics::*;
use crate::signature::*;
use crate::types::Address;
use ed25519_dalek::Signature;

// Transaction describes a transaction that can appear in a block.
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
	// Type of transaction
    #[serde(rename = "type")]
	tx_type: TxType,

	// Common fields for all types of transactions
	header: Header,

	// Fields for different types of transactions
    // TODO: should be wrapped in an Option?
    #[serde(skip_serializing_if="Option::is_none")]
    keyreg_fields: Option<KeyregTxnFields>,

    #[serde(skip_serializing_if="Option::is_none")]
	payment_fields: Option<PaymentTxnFields>
}

// SignedTxn wraps a transaction and a signature. The encoding of this struct
// is suitable to broadcast on the network
#[derive(Debug, Serialize, Deserialize)]
pub struct SignedTxn {
    // TODO: should be wrapped in an Option?
    pub sig: Signature,
    pub msig: Option<MultisigSig>,
	pub txn:  Transaction
}

// KeyregTxnFields captures the fields used for key registration transactions.
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyregTxnFields {
	
    #[serde(rename = "votekey")]
    vote_pk: VotePK,

    #[serde(rename = "selkey")]
	selection_pk: VRFPK
}

// PaymentTxnFields captures the fields used by payment transactions.
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTxnFields {
    #[serde(rename = "rcv")]
    receiver: Address,

    #[serde(rename = "amt")]
	amount: Algos,

	// When close_remainder_to is set, it indicates that the
	// transaction is requesting that the account should be
	// closed, and all remaining funds be transferred to this
	// address.
    #[serde(rename = "close")]
    #[serde(skip_serializing_if="Option::is_none")]
	close_remainder_to: Option<Address>
}

// Header captures the fields common to every transaction type.
#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    
    #[serde(rename = "snd")]
    sender: Address,

    #[serde(rename = "fee")]
    fee: Algos,

    #[serde(rename = "fv")]
    first_valid: Round,
    
    #[serde(rename = "lv")]
    last_valid: Round,

    note: Vec<u8>,

    #[serde(rename = "gen")]
    genesis_id: String
}


impl Transaction {
    // TODO: wrap in option/result
    pub fn make_payment_tx(from: &String, to: &String, fee: &u64, amount: &u64, first_round: &u64, last_round: &u64, note: &Vec<u8>, close_remainder_to: &Option<String>, genesis_id: &String) -> Self {
        let from_addr = Address::decode(&from).unwrap();
        let to_addr = Address::decode(&to).unwrap();
        let close_addr = match close_remainder_to {
            Some(i) => Address::decode(&i),
            None => None
        };

        Transaction {
            tx_type: TxType::Payment,
            header: Header {
                sender: from_addr,
                fee: *fee,
                first_valid: *first_round,
                last_valid: *last_round,
                note: note.clone(),
                genesis_id: genesis_id.clone()
            },
            payment_fields: Some(PaymentTxnFields {
                receiver: to_addr,
                amount: *amount,
                close_remainder_to: close_addr
            }),
            keyreg_fields: None
        }
    }
}