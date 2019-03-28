extern crate ed25519_dalek;
extern crate rand;
extern crate sha2;

use rand::rngs::OsRng;
use sha2::Sha512;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use crate::types::*;

#[derive(Debug)]
pub struct Account {
    pubkey: PublicKey,
    privkey: SecretKey,
    address: Address
}

impl Account {
    pub fn generate_account() -> Account {
        // Generate keypair
        let mut csprng: OsRng = OsRng::new().unwrap();
        let keypair: Keypair = Keypair::generate::<Sha512, _>(&mut csprng);
        
        let pk = keypair.public.clone();
        let sk = keypair.secret;
        
        // Convert public key to address
        let b = pk.to_bytes();
        let addr = Address::from_bytes(b);

        Account {
            pubkey: pk,
            privkey: sk,
            address: addr
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_test() {
        let test_acc: Account = Account::generate_account();
	    // Address should be identical to public key
        assert_eq!(test_acc.pubkey.to_bytes(), test_acc.address.to_bytes());
    }
}