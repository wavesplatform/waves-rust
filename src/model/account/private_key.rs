use crate::constants::{HASH_LENGTH, SIGNATURE_LENGTH};
use crate::error::{Error, Result};
use crate::model::account::PublicKey;
use crate::model::ByteString;
use crate::util::{Base58, Crypto};
use curve25519_dalek::montgomery::MontgomeryPoint;
use ed25519_dalek::{PublicKey as EdPublicKey, Signature, Verifier};

pub struct PrivateKey {
    // todo add https://docs.rs/secrecy/0.8.0/secrecy/ ?
    bytes: Vec<u8>,
    public_key: PublicKey,
}

impl PrivateKey {
    // todo add https://docs.rs/secrecy/0.8.0/secrecy/ ?
    pub fn from_seed(seed_phrase: &str, nonce: u8) -> Result<Self> {
        let hash_seed = Crypto::get_account_seed(seed_phrase.as_bytes(), nonce)?;
        let private_key = Crypto::get_private_key(&hash_seed)?;
        let public_key = PublicKey::from_bytes(&Crypto::get_public_key(&private_key));
        Ok(Self {
            bytes: private_key,
            public_key,
        })
    }

    pub fn encoded(&self) -> String {
        Base58::encode(&self.bytes, false)
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key.clone()
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
        let private_key: [u8; HASH_LENGTH] =
            self.bytes
                .clone()
                .try_into()
                .map_err(|_| Error::InvalidBytesLength {
                    expected_len: HASH_LENGTH,
                    actual_len: self.bytes.clone().len(),
                })?;
        Ok(Crypto::sign(&private_key, message))
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
    use crate::model::account::PrivateKey;
    use crate::model::ByteString;

    #[test]
    fn test_private_key_from_seed() {
        let seed_phrase = "blame vacant regret company chase trip grant funny brisk innocent";

        let expected_private_key_with_nonce_0 = "3j2aMHzh9azPphzuW7aF3cmUefGEQC9dcWYXYCyoPcJg";
        let expected_private_key_with_nonce_128 = "HCK7dUsScMH9mTCoyaV7bVhkTxwsyCHdbMBfb9TpVhPd";
        let expected_private_key_with_nonce_255 = "5Kdsn9jH3ifWSrZ19NYqnaCN9GmaPmNpZYnuSAEE4Yga";

        let expected_public_key_from_nonce_0 = "8cj6YzvQPhSHGvnjupNTW8zrADTT8CMAAd2xTuej84gB";
        let expected_public_key_from_nonce_128 = "DTvCW1nzFr7mHrHkGf1apstRfwPp4yYL19YvjjLEAPBh";
        let expected_public_key_from_nonce_255 = "esjbpqVWSg8iCaPYQA3SoxZo3oUkdRJSi9tKLoqKQoC";

        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 0)
                .expect("failed to get private ket from seed phrase")
                .encoded(),
            expected_private_key_with_nonce_0
        );
        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 128)
                .expect("failed to get private ket from seed phrase")
                .encoded(),
            expected_private_key_with_nonce_128
        );
        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 255)
                .expect("failed to get private ket from seed phrase")
                .encoded(),
            expected_private_key_with_nonce_255
        );

        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 0)
                .expect("failed to get private ket from seed phrase")
                .public_key
                .encoded(),
            expected_public_key_from_nonce_0
        );
        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 128)
                .expect("failed to get private ket from seed phrase")
                .public_key
                .encoded(),
            expected_public_key_from_nonce_128
        );
        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 255)
                .expect("failed to get private ket from seed phrase")
                .public_key
                .encoded(),
            expected_public_key_from_nonce_255
        );
    }

    #[test]
    fn test_sign_invalid_private_key_size_error() {}
}
