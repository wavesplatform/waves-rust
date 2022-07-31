pub struct Base58;

impl Base58 {
    pub fn decode(source: &str) -> bs58::decode::Result<Vec<u8>> {
        bs58::decode(source).into_vec()
    }

    pub fn encode(bytes: &Vec<u8>, with_prefix: bool) -> String {
        let base58string = bs58::encode(bytes).into_string();
        if with_prefix {
            return format!("base58:{}", base58string)
        }
        base58string
    }

    pub fn is_valid(encoded: &str) -> bool {
        bs58::decode(encoded).into_vec().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Base58;

    #[test]
    fn test_valid_base58string() {
        let base58string = "7LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2";
        assert_eq!(Base58::is_valid(base58string), true);
    }

    #[test]
    fn test_invalid_base58string() {
        let empty_string = "";
        let invalid_string = "0LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2";
        assert_eq!(Base58::is_valid(empty_string.into()), true);
        assert_eq!(Base58::is_valid(invalid_string.into()), false);
    }

    #[test]
    fn test_decode_string() {
        let test_string = "1LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2";
        let bytes = Base58::decode(test_string).unwrap();
        let base58string = Base58::encode(&bytes, false);
        let base58string_with_prefix = Base58::encode(&bytes, true);
        assert_eq!(test_string, base58string);
        assert_eq!(base58string_with_prefix, "base58:1LBopaBdBzQbgqrnwgmgCDhcSTb32MYhE96SnSHcqZC2")
    }
}