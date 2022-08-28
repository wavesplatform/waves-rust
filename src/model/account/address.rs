use crate::model::account::PublicKey;
use crate::util::{Base58, Crypto};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Address {
    chain_id: u8,
    bytes: Vec<u8>,
}

impl Address {
    pub fn from_public_key(chain_id: u8, public_key: &PublicKey) -> Address {
        Address {
            chain_id,
            bytes: Crypto::get_address(&chain_id, &Crypto::get_public_key_hash(public_key.bytes())),
        }
    }

    pub fn encoded(&self) -> String {
        Base58::encode(&self.bytes, false)
    }

    pub fn from_string(address: &str, chain_id: u8) -> Address {
        Address {
            chain_id,
            bytes: Base58::decode(address).unwrap(),
        }
    }

    pub fn chain_id(&self) -> u8 {
        self.chain_id
    }
}

#[cfg(test)]
mod tests {
    use crate::model::account::{Address, PrivateKey};
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

    #[test]
    fn test_address_from_string() {
        let expected_address = "3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW";
        let address = Address::from_string(expected_address, ChainId::TESTNET.byte());
        assert_eq!(expected_address, address.encoded())
    }
}
