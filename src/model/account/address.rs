use std::ops::Add;
use crate::model::account::PublicKey;
use crate::util::{Base58, Crypto};

pub struct Address {
    chain_id: u8,
    public_key_hash: Vec<u8>,
}

impl Address {
    pub fn from_public_key(chain_id: u8, public_key: &PublicKey) -> Address {
        Address {
            chain_id,
            public_key_hash: Crypto::get_public_key_hash(public_key.bytes()),
        }
    }

    pub fn encoded(&self) -> String {
        Base58::encode(
            &Crypto::get_address(&self.chain_id, &self.public_key_hash),
            false
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::model::account::PrivateKey;
    use crate::model::ChainId;

    #[test]
    fn test_address_from_public_key() {
        let seed_phrase = "blame vacant regret company chase trip grant funny brisk innocent";
        let expected_address = "3Ms87NGAAaPWZux233TB9A3TXps4LDkyJWN";

        let private_key = PrivateKey::from_seed(seed_phrase, 0);
        let public_key = private_key.public_key();
        let address = public_key.address(ChainId::TESTNET.byte()).encoded();

        assert_eq!(address, expected_address)
    }
}

