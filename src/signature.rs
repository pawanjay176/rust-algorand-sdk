extern crate ed25519_dalek;
extern crate serde_json;

use ed25519_dalek::Signature;
use ed25519_dalek::PublicKey;

// MultisigSubsig contains a single public key and, optionally, a signature
#[derive(Debug, Serialize, Deserialize)]
pub struct MultisigSubsig  {
    #[serde(rename = "pk")]
	pub_key: PublicKey,

    #[serde(rename = "s")]
	signature: Signature
}

// MultisigSig holds multiple Subsigs, as well as threshold and version info
#[derive(Debug, Serialize, Deserialize)]
pub struct MultisigSig {
    #[serde(rename = "v")]
	version: u8,

    #[serde(rename = "thr")]
    threshold: u8,

    // #[serde(rename = "subsig")]
    subsigs: Vec<MultisigSubsig>
}