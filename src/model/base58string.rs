use crate::error::Result;
use std::fmt;
use std::fmt::{Display, Formatter};

pub trait ByteString {
    fn bytes(&self) -> Vec<u8>;
    fn encoded(&self) -> String;
    fn encoded_with_prefix(&self) -> String;
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Base58String {
    bytes: Vec<u8>,
}

impl fmt::Debug for Base58String {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Base58String {{ {} }}", self.encoded())
    }
}

impl Base58String {
    pub fn from_bytes(bytes: Vec<u8>) -> Base58String {
        Base58String { bytes }
    }

    pub fn from_string(encoded: &str) -> Result<Base58String> {
        let bytes: Vec<u8> = bs58::decode(encoded).into_vec()?;
        Ok(Base58String { bytes })
    }

    pub fn empty() -> Base58String {
        Base58String { bytes: vec![] }
    }
}

impl ByteString for Base58String {
    fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn encoded(&self) -> String {
        bs58::encode(&self.bytes).into_string()
    }

    fn encoded_with_prefix(&self) -> String {
        format!("base58:{}", bs58::encode(&self.bytes).into_string())
    }
}

impl Display for Base58String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.encoded())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::base58string::ByteString;
    use crate::model::Base58String;

    #[test]
    fn test_base58string_from_string() {
        let base58string =
            Base58String::from_string("7LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2")
                .expect("failed to get base58string from string");

        assert_eq!(
            base58string.encoded(),
            "7LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2"
        );
        assert_eq!(
            base58string.encoded_with_prefix(),
            "base58:7LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2"
        )
    }

    #[test]
    fn test_base58string_from_bytes() {
        let bytes = bs58::decode("7LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2")
            .into_vec()
            .expect("Failed to parse base58 string");
        let base58string = Base58String::from_bytes(bytes);

        assert_eq!(
            base58string.encoded(),
            "7LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2"
        );
        assert_eq!(
            base58string.encoded_with_prefix(),
            "base58:7LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2"
        )
    }

    #[test]
    fn test_eq_between_base58_strings() {
        let bytes = bs58::decode("7LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2")
            .into_vec()
            .expect("Failed to parse base58 string");
        let base58string1 = Base58String::from_bytes(bytes);

        let base58string2 =
            Base58String::from_string("7LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2")
                .expect("failed to get base58string from string");

        let base58string3 =
            Base58String::from_string("8LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2")
                .expect("failed to get base58string from string");

        assert_eq!(base58string1 == base58string2, true);
        assert_eq!(base58string2 == base58string3, false)
    }

    #[test]
    fn test_pretty_print() {
        let base58string =
            Base58String::from_string("7LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2")
                .expect("failed to get base58string from string");
        let formatted_string = format!("{}", base58string);
        assert_eq!(
            "7LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2",
            formatted_string
        )
    }
}
