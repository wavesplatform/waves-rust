use crate::model::ByteString;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Base64String {
    bytes: Vec<u8>,
}

impl Base64String {
    pub fn from_bytes(bytes: Vec<u8>) -> Base64String {
        Base64String { bytes }
    }

    pub fn from_string(encoded: &str) -> Base64String {
        let base64str = if encoded.starts_with("base64:") {
            encoded.replace("base64:", "")
        } else {
            encoded.to_owned()
        };
        Base64String {
            bytes: base64::decode(base64str).expect("Failed to parse base64 string"),
        }
    }
}

impl ByteString for Base64String {
    fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    fn encoded(&self) -> String {
        base64::encode(&self.bytes)
    }

    fn encoded_with_prefix(&self) -> String {
        format!("base58:{}", bs58::encode(&self.bytes).into_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Base64String, ByteString};

    #[test]
    fn base64string_test() {
        let binary_value: [u8; 12] = [0; 12];
        let base64string = Base64String::from_bytes(binary_value.to_vec());
        assert_eq!("AAAAAAAAAAAAAAAA", base64string.encoded())
    }
}
