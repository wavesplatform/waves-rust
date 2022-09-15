use crate::constants::{SIGNATURE_LENGTH};
use crate::error::{Error, Result};
use crate::model::account::PublicKey;
use crate::model::ByteString;
use crate::util::{Base58, Crypto};
use curve25519_dalek::montgomery::MontgomeryPoint;
use ed25519_dalek::{PublicKey as EdPublicKey, Signature, Verifier};

pub struct PrivateKey {
    // todo add https://docs.rs/secrecy/0.8.0/secrecy/ ?
    bytes: [u8; 32],
    public_key: PublicKey,
}

impl PrivateKey {
    // todo add https://docs.rs/secrecy/0.8.0/secrecy/ ?
    pub fn from_seed(seed_phrase: &str, nonce: u8) -> Result<Self> {
        let hash_seed = Crypto::get_account_seed(seed_phrase.as_bytes(), nonce)?;
        let private_key = Crypto::get_private_key(&hash_seed)?;
        let public_key = PublicKey::from_bytes(&Crypto::get_public_key(&private_key))?;
        Ok(Self {
            bytes: private_key,
            public_key,
        })
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Result<Self> {
        let public_key = PublicKey::from_bytes(&Crypto::get_public_key(&bytes))?;
        Ok(Self { bytes, public_key })
    }

    pub fn encoded(&self) -> String {
        Base58::encode(&self.bytes.to_vec(), false)
    }

    pub fn bytes(&self) -> [u8; 32] {
        self.bytes
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key.clone()
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
        Ok(Crypto::sign(&self.bytes, message))
    }

    pub fn is_signature_valid(&self, message: &[u8], signature: &[u8]) -> Result<bool> {
        let sig_arr = <[u8; SIGNATURE_LENGTH]>::try_from(signature.to_owned()).map_err(|_| {
            Error::InvalidBytesLength {
                expected_len: SIGNATURE_LENGTH,
                actual_len: signature.len(),
            }
        })?;
        let sign = sig_arr[63] & 0x80;
        let mut sig = [0u8; SIGNATURE_LENGTH];
        sig.copy_from_slice(signature);
        sig[63] &= 0x7f;

        let public_key_bytes = self.public_key.bytes();
        let mut ed_public_key =
            MontgomeryPoint(<[u8; 32]>::try_from(public_key_bytes.clone()).map_err(|_| {
                Error::InvalidBytesLength {
                    expected_len: 32,
                    actual_len: public_key_bytes.len(),
                }
            })?)
            .to_edwards(sign)
            .ok_or(Error::MontgomeryPointConversionError)?
            .compress()
            .to_bytes();
        ed_public_key[31] &= 0x7F;
        ed_public_key[31] |= sign;

        Ok(EdPublicKey::from_bytes(&ed_public_key)?
            .verify(message, &Signature::from_bytes(&sig)?)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error::InvalidBytesLength;
    use crate::error::{Error, Result};
    use crate::model::account::PrivateKey;
    use crate::model::ByteString;

    #[test]
    fn test_private_key_from_seed() -> Result<()> {
        let seed_phrase = "blame vacant regret company chase trip grant funny brisk innocent";

        let expected_private_key_with_nonce_0 = "3j2aMHzh9azPphzuW7aF3cmUefGEQC9dcWYXYCyoPcJg";
        let expected_private_key_with_nonce_128 = "HCK7dUsScMH9mTCoyaV7bVhkTxwsyCHdbMBfb9TpVhPd";
        let expected_private_key_with_nonce_255 = "5Kdsn9jH3ifWSrZ19NYqnaCN9GmaPmNpZYnuSAEE4Yga";

        let expected_public_key_from_nonce_0 = "8cj6YzvQPhSHGvnjupNTW8zrADTT8CMAAd2xTuej84gB";
        let expected_public_key_from_nonce_128 = "DTvCW1nzFr7mHrHkGf1apstRfwPp4yYL19YvjjLEAPBh";
        let expected_public_key_from_nonce_255 = "esjbpqVWSg8iCaPYQA3SoxZo3oUkdRJSi9tKLoqKQoC";

        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 0)?.encoded(),
            expected_private_key_with_nonce_0
        );
        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 128)?.encoded(),
            expected_private_key_with_nonce_128
        );
        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 255)?.encoded(),
            expected_private_key_with_nonce_255
        );

        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 0)?
                .public_key()
                .encoded(),
            expected_public_key_from_nonce_0
        );
        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 128)?
                .public_key()
                .encoded(),
            expected_public_key_from_nonce_128
        );
        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 255)?
                .public_key()
                .encoded(),
            expected_public_key_from_nonce_255
        );
        Ok(())
    }

    #[test]
    fn test_invalid_signature_size() -> Result<()> {
        let private_key = PrivateKey::from_seed("a", 0)?;
        let result = private_key.is_signature_valid(&vec![], &[0_u8; 32]);
        match result {
            Ok(_) => panic!("expected error"),
            Err(err) => match err {
                InvalidBytesLength { .. } => Ok(()),
                _ => panic!("expected error"),
            },
        }
    }

    #[test]
    fn test_private_key_from_bytes() -> Result<()> {
        let private_key = PrivateKey::from_bytes([0; 32])?;
        assert_eq!(private_key.bytes(), [0_u8; 32]);
        Ok(())
    }
}
