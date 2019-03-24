extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub enum TxType {
    // PaymentTx is the TxType for payment transactions
    Payment,
    // TxType identifies the type of the transaction
    Keyreg
}
// TODO: Wrap in enums?
// Algos are the unit of currency in Algorand
pub type Algos = u64;

// Round represents a round of the Algorand consensus protocol
pub type Round = u64;

// VotePK is the participation public key used in key registration transactions
pub type VotePK = [u8;32];

// VRFPK is the VRF public key used in key registration transactions
pub type VRFPK = [u8;32];

// MasterDerivationKey is the secret key used to derive keys in wallets
pub type MasterDerivationKey = [u8;32];
