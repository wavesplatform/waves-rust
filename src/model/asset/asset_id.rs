use crate::error::{Error, Result};
use crate::model::ByteString;
use crate::util::Base58;
use serde_json::Value;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct AssetId {
    bytes: Vec<u8>,
}

impl fmt::Debug for AssetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AssetId {{ {} }}", self.encoded())
    }
}

impl AssetId {
    pub fn from_string(asset_id: &str) -> Result<AssetId> {
        Ok(Self::from_bytes(Base58::decode(asset_id)?))
    }

    pub fn from_bytes(bytes: Vec<u8>) -> AssetId {
        AssetId { bytes }
    }
}

impl ByteString for AssetId {
    fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn encoded(&self) -> String {
        Base58::encode(&self.bytes, false)
    }

    fn encoded_with_prefix(&self) -> String {
        Base58::encode(&self.bytes, true)
    }
}

impl TryFrom<&str> for AssetId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        AssetId::from_string(value)
    }
}

impl From<AssetId> for Value {
    fn from(value: AssetId) -> Self {
        Value::String(value.encoded())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::{AssetId, ByteString};
    use serde_json::{json, Value};

    #[test]
    fn test_byte_string_for_asset_id() -> Result<()> {
        let asset_id: AssetId = "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6".try_into()?;
        let expected_bytes = vec![
            112, 241, 63, 143, 135, 230, 99, 221, 151, 63, 90, 167, 36, 212, 184, 216, 209, 139,
            159, 7, 132, 61, 191, 30, 148, 44, 33, 177, 250, 231, 193, 117,
        ];
        assert_eq!(expected_bytes, asset_id.bytes());
        assert_eq!(
            "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
            asset_id.encoded()
        );
        assert_eq!(
            "base58:8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
            asset_id.encoded_with_prefix()
        );
        Ok(())
    }

    #[test]
    fn test_asset_id_to_json() -> Result<()> {
        let asset_id: AssetId = "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6".try_into()?;
        let json_value: Value = asset_id.into();
        assert_eq!(
            "8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6",
            json_value.as_str().unwrap_or("")
        );
        Ok(())
    }
}
