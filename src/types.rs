extern crate ring;
extern crate data_encoding;

extern crate arrayref;

use ring::{digest};
use data_encoding::{BASE32_NOPAD};
use arrayref::array_ref;

const CHECKSUM_LEN_BYTES: usize = 4;
const HASH_LEN_BYTES: usize = digest::SHA512_256_OUTPUT_LEN;
const ALGORAND_ADDRESS_LEN: usize = 58;

#[derive(Debug, Serialize, Deserialize)]
pub struct Address(pub [u8; HASH_LEN_BYTES]);

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
        BASE32_NOPAD.encode(&input)[0..ALGORAND_ADDRESS_LEN].to_string()
    }


    // TODO: Return a result type by defining decoding error
    pub fn decode(s: &String) -> Option <Address> {
        // Decode
        let decoded = BASE32_NOPAD.decode(s.as_bytes()).unwrap();

        let addr = &decoded[0..HASH_LEN_BYTES];
        let exp_checksum = &decoded[HASH_LEN_BYTES..];

        let checksum = digest::digest(&digest::SHA512_256, addr);
        let checksum = checksum.as_ref();
        // Get checksum bytes
        let checksum = &checksum[HASH_LEN_BYTES-CHECKSUM_LEN_BYTES..HASH_LEN_BYTES];
        
        // Validate checksum
        if checksum == exp_checksum {
            Some(Address(array_ref!(*addr,0,32).clone()))
        }
        else {
            None
        }
        
    }
}
