use crate::util::{Bytes, Hash};

pub struct Crypto;

impl Crypto {
    pub fn get_account_seed(seed_phrase: &Vec<u8>, nonce: u8) -> Vec<u8> {
        Hash::secure_hash(
            Bytes::concat(
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
}


#[cfg(test)]
mod tests {
    use crate::util::Base58;

    #[test]
    fn test() {}
}