use crate::error::{Error, Result};
use crate::model::account::Address;
use crate::util::Base58;
use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub struct PublicKey {
    bytes: Vec<u8>,
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey {{ {} }}", self.encoded())
    }
}

impl PublicKey {
    pub fn from_bytes(bytes: &[u8]) -> PublicKey {
        PublicKey {
            bytes: Vec::from(bytes),
        }
    }

    pub fn from_string(base58string: &str) -> Result<PublicKey> {
        let bytes = Base58::decode(base58string)?;
        Ok(PublicKey { bytes })
    }

    pub fn encoded(&self) -> String {
        Base58::encode(&self.bytes, false)
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn address(&self, chain_id: u8) -> Result<Address> {
        Address::from_public_key(chain_id, self)
    }
}

impl TryFrom<&str> for PublicKey {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        PublicKey::from_string(value)
    }
}

impl TryFrom<String> for PublicKey {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        PublicKey::from_string(&value)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::account::{PrivateKey, PublicKey};
    use crate::util::{Base58, Crypto};

    #[test]
    fn test_public_key_from_bytes() {
        let seed_phrase = "blame vacant regret company chase trip grant funny brisk innocent";

        let expected_public_key_from_nonce_0 = "8cj6YzvQPhSHGvnjupNTW8zrADTT8CMAAd2xTuej84gB";
        let expected_public_key_from_nonce_128 = "DTvCW1nzFr7mHrHkGf1apstRfwPp4yYL19YvjjLEAPBh";
        let expected_public_key_from_nonce_255 = "esjbpqVWSg8iCaPYQA3SoxZo3oUkdRJSi9tKLoqKQoC";

        assert_eq!(
            PublicKey::from_bytes(&public_key_bytes(seed_phrase, 0)).encoded(),
            expected_public_key_from_nonce_0
        );
        assert_eq!(
            PublicKey::from_bytes(&public_key_bytes(seed_phrase, 128)).encoded(),
            expected_public_key_from_nonce_128
        );
        assert_eq!(
            PublicKey::from_bytes(&public_key_bytes(seed_phrase, 255)).encoded(),
            expected_public_key_from_nonce_255
        )
    }

    fn public_key_bytes(seed_phrase: &str, nonce: u8) -> Vec<u8> {
        let bytes = PrivateKey::from_seed(seed_phrase, nonce)
            .expect("failed to get private ket from seed phrase")
            .bytes()
            .clone();
        println!("{}", Base58::encode(&bytes, false));
        Crypto::get_public_key(&bytes)
    }
}
