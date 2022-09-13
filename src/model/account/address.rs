use crate::error::{Error, Result};
use crate::model::account::PublicKey;
use crate::model::ByteString;
use crate::util::{Base58, Crypto, JsonDeserializer};
use serde_json::Value;
use std::fmt;
use std::hash::Hash;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Address {
    bytes: Vec<u8>,
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address {{ {} }}", self.encoded())
    }
}

impl Address {
    pub fn from_public_key(chain_id: u8, public_key: &PublicKey) -> Result<Address> {
        Ok(Address {
            bytes: Crypto::get_address(
                &chain_id,
                &Crypto::get_public_key_hash(&public_key.bytes())?,
            )?,
        })
    }

    pub fn from_string(address: &str) -> Result<Address> {
        Ok(Address {
            bytes: Base58::decode(address)?,
        })
    }

    pub fn chain_id(&self) -> u8 {
        self.bytes[1]
    }

    pub fn public_key_hash(&self) -> Vec<u8> {
        self.bytes[2..22].to_vec()
    }
}

impl ByteString for Address {
    fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn encoded(&self) -> String {
        Base58::encode(&self.bytes, false)
    }

    fn encoded_with_prefix(&self) -> String {
        Base58::encode(&self.bytes, true)
    }
}

impl TryFrom<&Value> for Address {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let string = JsonDeserializer::safe_to_string(value)?;
        Address::from_string(&string)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::account::{Address, PrivateKey};
    use crate::model::{ByteString, ChainId};
    use serde_json::Value;
    use std::borrow::Borrow;

    #[test]
    fn test_address_from_public_key() {
        let seed_phrase = "blame vacant regret company chase trip grant funny brisk innocent";
        let expected_address = "3Ms87NGAAaPWZux233TB9A3TXps4LDkyJWN";

        let private_key =
            PrivateKey::from_seed(seed_phrase, 0).expect("failed to get private key from seed");
        let public_key = private_key.public_key();
        let address = public_key
            .address(ChainId::TESTNET.byte())
            .expect("failed to get address from public key")
            .encoded();

        assert_eq!(address, expected_address)
    }

    #[test]
    fn test_address_from_string() {
        let expected_address = "3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW";
        let address =
            Address::from_string(expected_address).expect("failed to get address from string");
        assert_eq!(expected_address, address.encoded())
    }

    #[test]
    fn test_address_from_json() -> Result<()> {
        let expected_address = "3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW";
        let address: Address = Value::String("3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW".to_owned())
            .borrow()
            .try_into()?;
        assert_eq!(expected_address, address.encoded());
        Ok(())
    }

    #[test]
    fn test_byte_string_for_address() -> Result<()> {
        let address = Address::from_string("3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW")?;
        let expected_bytes: Vec<u8> = vec![
            1, 84, 49, 59, 204, 61, 157, 141, 148, 218, 122, 51, 43, 12, 171, 81, 190, 13, 80, 46,
            88, 199, 218, 79, 208, 145,
        ];
        let expected_encoded = "3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW";
        let expected_encoded_with_prefix = "base58:3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW";
        assert_eq!(expected_bytes, address.bytes());
        assert_eq!(expected_encoded, address.encoded());
        assert_eq!(expected_encoded_with_prefix, address.encoded_with_prefix());
        Ok(())
    }
}
