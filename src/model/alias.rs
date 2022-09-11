use crate::error::{Error, Result};
use crate::util::ByteWriter;
use regex::Regex;

const PREFIX: &str = "alias:";
const MIN_LENGTH: u8 = 4;
const MAX_LENGTH: u8 = 30;
const ALPHABET: &str = "-.0-9@_a-z";
const TYPE: u8 = 2;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Alias {
    bytes: Vec<u8>,
    name: String,
    full_name: String,
}

impl Alias {
    pub fn new(chain_id: u8, name: String) -> Result<Self> {
        if Self::is_valid(chain_id, &name) {
            let bytes = ByteWriter::new()
                .push_byte(TYPE)
                .push_byte(chain_id)
                .push_bytes(&mut name.clone().into_bytes())
                .bytes();
            let name = Self::replace_prefix(chain_id, &name);
            let full_name = format!("{}{}:{}", PREFIX, chain_id as char, &name);
            return Ok(Self {
                bytes,
                name,
                full_name,
            });
        }
        Err(Error::InvalidAliasName {
            min_length: MIN_LENGTH,
            max_length: MAX_LENGTH,
            alphabet: ALPHABET.to_owned(),
            prefix: PREFIX.to_owned(),
            chain_id: chain_id as char,
        })
    }

    pub fn chain_id(full_name: String) -> u8 {
        full_name
            .replace(PREFIX, "")
            .chars()
            .next()
            .expect("failed to get chain id from alias") as u8
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn full_name(&self) -> String {
        self.full_name.clone()
    }

    pub fn is_valid(chain_id: u8, name: &str) -> bool {
        let name = Self::replace_prefix(chain_id, name);
        Regex::new(&format!(
            r"^[{}]{{{},{}}}$",
            ALPHABET, MIN_LENGTH, MAX_LENGTH
        ))
        .expect("invalid regex")
        .is_match(&name)
    }

    fn replace_prefix(chain_id: u8, name: &str) -> String {
        Regex::new(&format!(r"^{}{}:", PREFIX, chain_id as char))
            .expect("invalid regex")
            .replace(name, "")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Alias, ChainId};

    #[test]
    fn test_is_valid() {
        let valid_alias = "alias:T:alias1662650000377".to_owned();
        assert_eq!(true, Alias::is_valid(ChainId::TESTNET.byte(), &valid_alias));
    }

    #[test]
    fn test_is_invalid() {
        let invalid_alias1 = "alias1662650000377!".to_owned();
        let invalid_alias2 = "alias 1662650000377".to_owned();
        let invalid_alias3 = "ali".to_owned();
        let invalid_alias4 = "alias1662650000377alias166265000037".to_owned();
        assert_eq!(
            false,
            Alias::is_valid(ChainId::TESTNET.byte(), &invalid_alias1)
        );
        assert_eq!(
            false,
            Alias::is_valid(ChainId::TESTNET.byte(), &invalid_alias2)
        );
        assert_eq!(
            false,
            Alias::is_valid(ChainId::TESTNET.byte(), &invalid_alias3)
        );
        assert_eq!(
            false,
            Alias::is_valid(ChainId::TESTNET.byte(), &invalid_alias4)
        )
    }

    #[test]
    fn test_create_alias() {
        let result = Alias::new(ChainId::TESTNET.byte(), "alias1662650000377".to_owned());
        match result {
            Ok(alias) => {
                assert_eq!(alias.name(), "alias1662650000377");
                assert_eq!(alias.full_name(), "alias:T:alias1662650000377");
                let mut bytes = vec![2, 84];
                bytes.append(&mut alias.name().into_bytes());
                assert_eq!(alias.bytes(), bytes);
            }
            Err(err) => {
                println!("{:?}", err);
                panic!("failed to create alias")
            }
        }
    }

    #[test]
    fn test_chain_id_from_full_name_alias() {
        let i = Alias::chain_id("alias:T:alias1662650000377".to_owned());
        assert_eq!(84, i);
    }
}
