use crate::model::account::PublicKey;
use crate::util::{Base58, Crypto};

pub struct PrivateKey {
    bytes: Vec<u8>,
    public_key: PublicKey,
}

impl PrivateKey {
    pub fn from_seed(seed_phrase: &str, nonce: u8) -> PrivateKey {
        let hash_seed = Crypto::get_account_seed(
            &seed_phrase.as_bytes().to_vec(),
            nonce,
        );
        let private_key = Crypto::get_private_key(
            &hash_seed
        );
        let public_key = PublicKey::from_bytes(&Crypto::get_public_key(&private_key));
        PrivateKey {
            bytes: private_key,
            public_key,
        }
    }

    pub fn encoded(&self) -> String {
        Base58::encode(&self.bytes, false)
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
}

#[cfg(test)]
mod tests {
    use crate::model::account::PrivateKey;

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
            PrivateKey::from_seed(seed_phrase, 0).encoded(),
            expected_private_key_with_nonce_0
        );
        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 128).encoded(),
            expected_private_key_with_nonce_128
        );
        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 255).encoded(),
            expected_private_key_with_nonce_255
        );

        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 0).public_key.encoded(),
            expected_public_key_from_nonce_0
        );
        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 128).public_key.encoded(),
            expected_public_key_from_nonce_128
        );
        assert_eq!(
            PrivateKey::from_seed(seed_phrase, 255).public_key.encoded(),
            expected_public_key_from_nonce_255
        );
    }
}