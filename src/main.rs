mod types;

use crate::types::*;

fn main() {
    let test: Address = Address::new();
    println!("{}", Address::to_str(&test));
    // println!("{}", s);
}
