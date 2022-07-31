use curve25519_dalek::constants;
use curve25519_dalek::scalar::Scalar;
use crate::util::{Bytes, Hash};

pub struct Crypto;

impl Crypto {
    pub fn get_account_seed(seed_phrase: &Vec<u8>, nonce: u8) -> Vec<u8> {
        Hash::secure_hash(
            &Bytes::concat(
                vec![
                    Bytes::from_nonce(nonce),
                    seed_phrase.to_vec(),
                ]
            )
        )
    }

    pub fn get_private_key(account_seed: &Vec<u8>) -> Vec<u8> {
        let mut private_key = [0u8; 32];
        let hashed_account_seed = Hash::sha256(account_seed);
        private_key.copy_from_slice(&hashed_account_seed);
        private_key[0] &= 248;
        private_key[31] &= 127;
        private_key[31] |= 64;

        private_key.to_vec()
    }

    pub fn get_public_key(private_key: &Vec<u8>) -> Vec<u8> {
        let mut pk = [0u8; 32];
        pk.copy_from_slice(private_key.as_slice());
        let ed_pk = &Scalar::from_bits(pk) * &constants::ED25519_BASEPOINT_TABLE;
        ed_pk.to_montgomery().to_bytes().to_vec()
    }
}


#[cfg(test)]
mod tests {
    use crate::util::{Base58, Crypto};

    #[test]
    fn test_get_private_key() {
        let seed_phrase = "blame vacant regret company chase trip grant funny brisk innocent"
            .as_bytes()
            .to_vec();
        let expected_private_key = "3j2aMHzh9azPphzuW7aF3cmUefGEQC9dcWYXYCyoPcJg";
        let account_seed = Crypto::get_account_seed(&seed_phrase, 0);
        let private_key = Crypto::get_private_key(&account_seed);
        let encoded_private_key = Base58::encode(&private_key, false);
        assert_eq!(encoded_private_key, expected_private_key)
    }

    #[test]
    fn test_get_public_key() {
        let seed_phrase = "blame vacant regret company chase trip grant funny brisk innocent";

        let expected_public_key_from_nonce_0 = "8cj6YzvQPhSHGvnjupNTW8zrADTT8CMAAd2xTuej84gB";
        let expected_public_key_from_nonce_128 = "DTvCW1nzFr7mHrHkGf1apstRfwPp4yYL19YvjjLEAPBh";
        let expected_public_key_from_nonce_255 = "esjbpqVWSg8iCaPYQA3SoxZo3oUkdRJSi9tKLoqKQoC";
        assert_eq!(
            Crypto::get_public_key(
                &private_key(seed_phrase, 0)
            ),
            Base58::decode(expected_public_key_from_nonce_0).unwrap()
        );
        assert_eq!(
            Crypto::get_public_key(
                &private_key(seed_phrase, 128)
            ),
            Base58::decode(expected_public_key_from_nonce_128).unwrap()
        );
        assert_eq!(
            Crypto::get_public_key(
                &private_key(seed_phrase, 255)
            ),
            Base58::decode(expected_public_key_from_nonce_255).unwrap()
        );
    }

    fn private_key(seed_phrase: &str, nonce: u8) -> Vec<u8> {
        let account_seed = Crypto::get_account_seed(
            &seed_phrase.as_bytes().to_vec(),
            nonce,
        );
        Crypto::get_private_key(&account_seed)
    }
}