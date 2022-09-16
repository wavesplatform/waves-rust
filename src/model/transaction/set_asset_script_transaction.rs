use crate::error::{Error, Result};
use crate::model::{AssetId, Base64String, ByteString};
use crate::util::JsonDeserializer;
use crate::waves_proto::SetAssetScriptTransactionData;
use serde_json::{Map, Value};

const TYPE: u8 = 15;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SetAssetScriptTransactionInfo {
    asset_id: AssetId,
    script: Base64String,
}

impl SetAssetScriptTransactionInfo {
    pub fn new(asset_id: AssetId, script: Base64String) -> Self {
        Self { asset_id, script }
    }

    pub fn script(&self) -> Base64String {
        self.script.clone()
    }

    pub fn asset_id(&self) -> AssetId {
        self.asset_id.clone()
    }
}

impl TryFrom<&Value> for SetAssetScriptTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let script = JsonDeserializer::safe_to_string_from_field(value, "script")?;
        let asset_id = JsonDeserializer::safe_to_string_from_field(value, "assetId")?;

        Ok(SetAssetScriptTransactionInfo {
            asset_id: AssetId::from_string(&asset_id)?,
            script: Base64String::from_string(&script)?,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SetAssetScriptTransaction {
    asset_id: AssetId,
    script: Base64String,
}

impl SetAssetScriptTransaction {
    pub fn new(asset_id: AssetId, script: Base64String) -> Self {
        Self { asset_id, script }
    }

    pub fn script(&self) -> Base64String {
        self.script.clone()
    }

    pub fn asset_id(&self) -> AssetId {
        self.asset_id.clone()
    }

    pub fn tx_type() -> u8 {
        TYPE
    }
}

impl TryFrom<&Value> for SetAssetScriptTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let script = JsonDeserializer::safe_to_string_from_field(value, "script")?;
        let asset_id = JsonDeserializer::safe_to_string_from_field(value, "assetId")?;

        Ok(SetAssetScriptTransaction {
            asset_id: AssetId::from_string(&asset_id)?,
            script: Base64String::from_string(&script)?,
        })
    }
}

impl TryFrom<&SetAssetScriptTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(value: &SetAssetScriptTransaction) -> Result<Self> {
        let mut set_script_tx_json = Map::new();
        set_script_tx_json.insert("assetId".to_owned(), value.asset_id.encoded().into());
        set_script_tx_json.insert("script".to_owned(), value.script.encoded().into());
        Ok(set_script_tx_json)
    }
}

impl TryFrom<&SetAssetScriptTransaction> for SetAssetScriptTransactionData {
    type Error = Error;

    fn try_from(value: &SetAssetScriptTransaction) -> Result<Self> {
        Ok(SetAssetScriptTransactionData {
            asset_id: value.asset_id.bytes(),
            script: value.script.bytes(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::{AssetId, Base64String, ByteString, SetAssetScriptTransaction};
    use crate::waves_proto::SetAssetScriptTransactionData;
    use serde_json::{json, Map, Value};
    use std::borrow::Borrow;
    use std::fs;

    const COMPILED_ASSET_SCRIPT: &str = "base64:AgQAAAAHbWFzdGVyMQkBAAAAEWFkZHJlc3NGcm9tU3RyaW5nAAAAAQIAAAAQMzMzbWFzdGVyQWRkcmVzcwQAAAAHJG1hdGNoMAUAAAACdHgDCQAAAQAAAAIFAAAAByRtYXRjaDACAAAAE1RyYW5zZmVyVHJhbnNhY3Rpb24EAAAAAXQFAAAAByRtYXRjaDADCQAAAAAAAAIIBQAAAAF0AAAABnNlbmRlcgUAAAAHbWFzdGVyMQYJAAAAAAAAAggFAAAAAXQAAAAJcmVjaXBpZW50BQAAAAdtYXN0ZXIxAwkAAAEAAAACBQAAAAckbWF0Y2gwAgAAABdNYXNzVHJhbnNmZXJUcmFuc2FjdGlvbgQAAAACbXQFAAAAByRtYXRjaDAJAAAAAAAAAggFAAAAAm10AAAABnNlbmRlcgUAAAAHbWFzdGVyMQMJAAABAAAAAgUAAAAHJG1hdGNoMAIAAAATRXhjaGFuZ2VUcmFuc2FjdGlvbgcGFLbwIw==";

    #[test]
    fn test_json_to_set_asset_script_transaction() {
        let data = fs::read_to_string("./tests/resources/set_asset_script_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let set_script_from_json: SetAssetScriptTransaction = json.borrow().try_into().unwrap();

        assert_eq!(
            "CVwsbXjXmdYF2q4RCPuQKf7sLGpzhk7BNnYsxGZZJMym",
            set_script_from_json.asset_id.encoded()
        );

        assert_eq!(
            set_script_from_json.script().encoded_with_prefix(),
            COMPILED_ASSET_SCRIPT
        );
    }

    #[test]
    fn test_set_asset_script_to_proto() -> Result<()> {
        let set_asset_script_tx = &SetAssetScriptTransaction::new(
            AssetId::from_string("CVwsbXjXmdYF2q4RCPuQKf7sLGpzhk7BNnYsxGZZJMym")?,
            Base64String::from_string(COMPILED_ASSET_SCRIPT)?,
        );
        let proto: SetAssetScriptTransactionData = set_asset_script_tx.try_into()?;

        assert_eq!(proto.asset_id, set_asset_script_tx.asset_id().bytes());
        assert_eq!(proto.script, set_asset_script_tx.script().bytes());

        Ok(())
    }

    #[test]
    fn test_set_asset_script_to_json() -> Result<()> {
        let set_asset_script_tx = &SetAssetScriptTransaction::new(
            AssetId::from_string("CVwsbXjXmdYF2q4RCPuQKf7sLGpzhk7BNnYsxGZZJMym")?,
            Base64String::from_string(COMPILED_ASSET_SCRIPT)?,
        );

        let map: Map<String, Value> = set_asset_script_tx.try_into()?;
        let json: Value = map.into();
        let expected_json = json!({
            "assetId": "CVwsbXjXmdYF2q4RCPuQKf7sLGpzhk7BNnYsxGZZJMym",
            "script": "AgQAAAAHbWFzdGVyMQkBAAAAEWFkZHJlc3NGcm9tU3RyaW5nAAAAAQIAAAAQMzMzbWFzdGVyQWRkcmVzcwQAAAAHJG1hdGNoMAUAAAACdHgDCQAAAQAAAAIFAAAAByRtYXRjaDACAAAAE1RyYW5zZmVyVHJhbnNhY3Rpb24EAAAAAXQFAAAAByRtYXRjaDADCQAAAAAAAAIIBQAAAAF0AAAABnNlbmRlcgUAAAAHbWFzdGVyMQYJAAAAAAAAAggFAAAAAXQAAAAJcmVjaXBpZW50BQAAAAdtYXN0ZXIxAwkAAAEAAAACBQAAAAckbWF0Y2gwAgAAABdNYXNzVHJhbnNmZXJUcmFuc2FjdGlvbgQAAAACbXQFAAAAByRtYXRjaDAJAAAAAAAAAggFAAAAAm10AAAABnNlbmRlcgUAAAAHbWFzdGVyMQMJAAABAAAAAgUAAAAHJG1hdGNoMAIAAAATRXhjaGFuZ2VUcmFuc2FjdGlvbgcGFLbwIw=="
        });
        assert_eq!(expected_json, json);
        Ok(())
    }
}
