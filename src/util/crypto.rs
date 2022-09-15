use crate::constants::{ADDRESS_LENGTH, ADDRESS_VERSION, SIGNATURE_LENGTH};
use crate::error::Result;
use crate::util::{Bytes, Hash};
use curve25519_dalek::constants;
use curve25519_dalek::scalar::Scalar;
use rand::Rng;
use sha2::digest::Update;
use sha2::Sha512;

pub struct Crypto;

impl Crypto {
    pub fn get_account_seed(seed_phrase: &[u8], nonce: u8) -> Result<Vec<u8>> {
        Hash::secure_hash(&Bytes::concat(vec![
            Bytes::from_nonce(nonce),
            seed_phrase.to_vec(),
        ]))
    }

    pub fn get_private_key(account_seed: &Vec<u8>) -> Result<[u8; 32]> {
        let mut private_key = [0u8; 32];
        let hashed_account_seed = Hash::sha256(account_seed);
        private_key.copy_from_slice(&hashed_account_seed);
        private_key[0] &= 248;
        private_key[31] &= 127;
        private_key[31] |= 64;

        Ok(private_key)
    }

    pub fn get_public_key(private_key: &[u8; 32]) -> Vec<u8> {
        let mut pk = [0u8; 32];
        pk.copy_from_slice(private_key);
        let ed_pk = &Scalar::from_bits(pk) * &constants::ED25519_BASEPOINT_TABLE;
        ed_pk.to_montgomery().to_bytes().to_vec()
    }

    pub fn get_public_key_hash(public_key: &[u8]) -> Result<Vec<u8>> {
        let hash = Hash::secure_hash(public_key)?;
        Ok(hash[0..20].to_vec())
    }

    pub fn get_address(chain_id: &u8, public_key_hash: &[u8]) -> Result<Vec<u8>> {
        let mut buf = [0u8; ADDRESS_LENGTH];
        buf[0] = ADDRESS_VERSION;
        buf[1] = *chain_id;
        buf[2..22].copy_from_slice(public_key_hash);
        let checksum = &Hash::secure_hash(&buf[..22])?[..4];
        buf[22..].copy_from_slice(checksum);
        Ok(buf.to_vec())
    }

    pub fn sign(private_key: &[u8; 32], message: &[u8]) -> Vec<u8> {
        let mut hash = Sha512::default();

        hash.update(&INITBUF);

        hash.update(private_key);
        hash.update(message);

        let mut rand = rand::thread_rng();
        let mut rndbuf: Vec<u8> = vec![0; 64];
        (0..63).for_each(|i| rndbuf[i] = rand.gen::<u8>());

        hash.update(&rndbuf);

        let rsc = Scalar::from_hash(hash);
        let r = (&rsc * &constants::ED25519_BASEPOINT_TABLE)
            .compress()
            .to_bytes();

        let ed_public_key = constants::ED25519_BASEPOINT_POINT * Scalar::from_bits(*private_key);
        let public_key = ed_public_key.compress().to_bytes();

        hash = Sha512::default();
        hash.update(&r);
        hash.update(&public_key);
        hash.update(message);
        let s = (Scalar::from_hash(hash) * Scalar::from_bits(*private_key)) + rsc;

        let sign = public_key[31] & 0x80;
        let mut result = [0; SIGNATURE_LENGTH];
        result[..32].copy_from_slice(&r);
        result[32..].copy_from_slice(&s.to_bytes());
        result[63] &= 0x7F;
        result[63] |= sign;
        result.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::ChainId;
    use crate::util::{Base58, Crypto};

    #[test]
    fn test_get_private_key() {
        let seed_phrase = "blame vacant regret company chase trip grant funny brisk innocent"
            .as_bytes()
            .to_vec();
        let expected_private_key = "3j2aMHzh9azPphzuW7aF3cmUefGEQC9dcWYXYCyoPcJg";
        let account_seed =
            Crypto::get_account_seed(&seed_phrase, 0).expect("failed to get account seed");
        let private_key =
            Crypto::get_private_key(&account_seed).expect("failed to get private key");
        let encoded_private_key = Base58::encode(&private_key.to_vec(), false);
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
                &private_key(seed_phrase, 0).expect("failed to get private key")
            ),
            Base58::decode(expected_public_key_from_nonce_0).expect("Failed to decode str")
        );
        assert_eq!(
            Crypto::get_public_key(
                &private_key(seed_phrase, 128).expect("failed to get private key")
            ),
            Base58::decode(expected_public_key_from_nonce_128).expect("Failed to decode str")
        );
        assert_eq!(
            Crypto::get_public_key(
                &private_key(seed_phrase, 255).expect("failed to get private key")
            ),
            Base58::decode(expected_public_key_from_nonce_255).expect("Failed to decode str")
        );
    }

    #[test]
    fn test_get_address() {
        let seed_phrase = "blame vacant regret company chase trip grant funny brisk innocent";

        let expected_address = "3Ms87NGAAaPWZux233TB9A3TXps4LDkyJWN";

        let public_key = Crypto::get_public_key(
            &private_key(seed_phrase, 0).expect("failed to get private key"),
        );
        let public_key_hash =
            Crypto::get_public_key_hash(&public_key).expect("failed to get public key hash");

        let address = Crypto::get_address(&ChainId::TESTNET.byte(), &public_key_hash)
            .expect("failed to get address");
        let encoded_address = Base58::encode(&address, false);

        assert_eq!(encoded_address, expected_address)
    }

    fn private_key(seed_phrase: &str, nonce: u8) -> Result<[u8; 32]> {
        let account_seed = Crypto::get_account_seed(seed_phrase.as_bytes(), nonce)
            .expect("failed to get account seed");
        Crypto::get_private_key(&account_seed)
    }
}

static INITBUF: [u8; 32] = [
    0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];
