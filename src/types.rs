extern crate ring;
extern crate data_encoding;

use ring::{digest};
use data_encoding::BASE32;

const CHECKSUM_LEN_BYTES: usize = 4;
const HASH_LEN_BYTES: usize = digest::SHA512_256_OUTPUT_LEN;
const ALGORAND_ADDRESS_LEN: usize = 58;

pub struct Address([u8; HASH_LEN_BYTES]);

impl Address {
    pub fn new() -> Address {
        Address([0;HASH_LEN_BYTES])
    }

    pub fn encode(a: &Address) -> String {
        let checksum = digest::digest(&digest::SHA512_256, &a.0);
        let checksum = checksum.as_ref();
        // Get checksum bytes
        let checksum = &checksum[HASH_LEN_BYTES-CHECKSUM_LEN_BYTES..HASH_LEN_BYTES];
        
        // Append to address
        let mut input = a.0.clone().to_vec();
        input.append(&mut checksum.to_vec());
        
        // Convert to base 32
        BASE32.encode(&input)[0..ALGORAND_ADDRESS_LEN].to_string()
    }

    pub fn decode(s: &String) -> Address {
        
    }
}
