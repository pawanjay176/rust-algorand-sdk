mod types;
use crate::types::*;

fn main() {
    let test: Address = Address::new();
    let encoded: String = Address::encode(&test);
    println!("{}", encoded);
    let decoded = Address::decode(&encoded);
    println!("{:?}", decoded.unwrap().0);
}
