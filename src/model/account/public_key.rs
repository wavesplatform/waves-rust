use crate::error::Error::InvalidBytesLength;
use crate::error::{Error, Result};
use crate::model::account::Address;
use crate::model::ByteString;
use crate::util::Base58;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PublicKey {
    bytes: Vec<u8>,
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey {{ {} }}", self.encoded())
    }
}

impl PublicKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey> {
        if bytes.len() != 32 {
            Err(InvalidBytesLength {
                expected_len: 32,
                actual_len: bytes.len(),
            })?;
        }
        Ok(PublicKey {
            bytes: Vec::from(bytes),
        })
    }

    pub fn from_string(base58string: &str) -> Result<PublicKey> {
        let bytes = Base58::decode(base58string)?;
        Ok(PublicKey { bytes })
    }

    pub fn address(&self, chain_id: u8) -> Result<Address> {
        Address::from_public_key(chain_id, self)
    }
}

impl ByteString for PublicKey {
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
    use crate::error::Error::InvalidBytesLength;
    use crate::error::Result;
    use crate::model::account::{PrivateKey, PublicKey};
    use crate::model::ByteString;
    use crate::util::{Base58, Crypto};

    #[test]
    fn test_public_key_from_bytes() -> Result<()> {
        let seed_phrase = "blame vacant regret company chase trip grant funny brisk innocent";

        let expected_public_key_from_nonce_0 = "8cj6YzvQPhSHGvnjupNTW8zrADTT8CMAAd2xTuej84gB";
        let expected_public_key_from_nonce_128 = "DTvCW1nzFr7mHrHkGf1apstRfwPp4yYL19YvjjLEAPBh";
        let expected_public_key_from_nonce_255 = "esjbpqVWSg8iCaPYQA3SoxZo3oUkdRJSi9tKLoqKQoC";

        assert_eq!(
            PublicKey::from_bytes(&public_key_bytes(seed_phrase, 0))?.encoded(),
            expected_public_key_from_nonce_0
        );
        assert_eq!(
            PublicKey::from_bytes(&public_key_bytes(seed_phrase, 128))?.encoded(),
            expected_public_key_from_nonce_128
        );
        assert_eq!(
            PublicKey::from_bytes(&public_key_bytes(seed_phrase, 255))?.encoded(),
            expected_public_key_from_nonce_255
        );
        Ok(())
    }

    fn public_key_bytes(seed_phrase: &str, nonce: u8) -> Vec<u8> {
        let bytes = PrivateKey::from_seed(seed_phrase, nonce)
            .expect("failed to get private ket from seed phrase")
            .bytes()
            .clone();
        println!("{}", Base58::encode(&bytes.to_vec(), false));
        Crypto::get_public_key(&bytes)
    }

    #[test]
    fn test_invalid_bytes_len_for_public_key() -> Result<()> {
        let public_key = PublicKey::from_bytes(&vec![]);
        match public_key {
            Ok(_) => panic!("expected error"),
            Err(err) => match err {
                InvalidBytesLength { .. } => Ok(()),
                _ => panic!("expected InvalidBytesLength error"),
            },
        }
    }

    #[test]
    fn test_public_key_from_str() -> Result<()> {
        let expected_str = "8cj6YzvQPhSHGvnjupNTW8zrADTT8CMAAd2xTuej84gB";
        let public_key: PublicKey = expected_str.try_into()?;
        assert_eq!(expected_str, public_key.encoded());
        Ok(())
    }

    #[test]
    fn test_public_key_from_string() -> Result<()> {
        let expected_string = "8cj6YzvQPhSHGvnjupNTW8zrADTT8CMAAd2xTuej84gB".to_owned();
        let public_key: PublicKey = expected_string.clone().try_into()?;
        assert_eq!(expected_string, public_key.encoded());
        Ok(())
    }

    #[test]
    fn test_byte_string_for_public_key() -> Result<()> {
        let public_key: PublicKey = "8cj6YzvQPhSHGvnjupNTW8zrADTT8CMAAd2xTuej84gB".try_into()?;
        let expected_bytes = vec![
            113, 40, 188, 13, 166, 104, 24, 229, 65, 157, 176, 205, 96, 187, 101, 62, 170, 97, 253,
            32, 117, 73, 107, 139, 119, 67, 237, 157, 117, 22, 27, 36,
        ];
        assert_eq!(expected_bytes, public_key.bytes());
        assert_eq!(
            "8cj6YzvQPhSHGvnjupNTW8zrADTT8CMAAd2xTuej84gB",
            public_key.encoded()
        );
        assert_eq!(
            "base58:8cj6YzvQPhSHGvnjupNTW8zrADTT8CMAAd2xTuej84gB",
            public_key.encoded_with_prefix()
        );
        Ok(())
    }
}
