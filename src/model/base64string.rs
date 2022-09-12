use crate::error::Result;
use crate::model::ByteString;
use std::fmt;

#[derive(Eq, PartialEq, Clone)]
pub struct Base64String {
    bytes: Vec<u8>,
}

impl fmt::Debug for Base64String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Base64String {{ {} }}", self.encoded())
    }
}

impl Base64String {
    pub fn from_bytes(bytes: Vec<u8>) -> Base64String {
        Base64String { bytes }
    }

    pub fn from_string(encoded: &str) -> Result<Base64String> {
        let base64str = if encoded.starts_with("base64:") {
            encoded.replace("base64:", "")
        } else {
            encoded.to_owned()
        };
        Ok(Base64String {
            bytes: base64::decode(base64str)?,
        })
    }
}

impl ByteString for Base64String {
    fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn encoded(&self) -> String {
        base64::encode(&self.bytes)
    }

    fn encoded_with_prefix(&self) -> String {
        format!("base64:{}", base64::encode(&self.bytes))
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
