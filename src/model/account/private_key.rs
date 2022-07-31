use crate::util::{Base58, Crypto};

pub struct PrivateKey {
    bytes: Vec<u8>,
}

impl PrivateKey {
    pub fn from_seed_with_nonce(seed_phrase: &str, nonce: u8) -> PrivateKey {
        PrivateKey {
            bytes: Crypto::get_private_key(
                &Crypto::get_account_seed(
                    &seed_phrase.as_bytes().to_vec(),
                    nonce,
                )
            )
        }
    }

    pub fn encoded(&self) -> String {
        Base58::encode(&self.bytes, false)
    }
}

fn account_seed(seed_phrase: Vec<u8>, nonce: u32) {
    //[0, 0, 0, -106] 150
}

//83M4HnCQxrDMzUQqwmxfTVJPTE9WdE7zjAooZZm2jCyV

#[cfg(test)]
mod tests {
    use crate::model::account::PrivateKey;

    #[test]
    fn test_private_key_from_seed() {
        let private_key = PrivateKey::from_seed_with_nonce("waves private node seed with waves tokens".into(), 0);
        let x = private_key.encoded();
        println!("{}", x)
    }
}