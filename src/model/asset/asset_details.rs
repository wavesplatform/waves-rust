use crate::error::{Error, Result};
use crate::model::{Address, AssetId, Base64String, Id, PublicKey};
use crate::util::JsonDeserializer;
use serde_json::Value;
use std::borrow::Borrow;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct AssetDetails {
    asset_id: AssetId,
    issue_height: u32,
    issue_timestamp: u64,
    issuer: Address,
    issuer_public_key: PublicKey,
    name: String,
    description: String,
    decimals: u32,
    reissuable: bool,
    quantity: u64,
    scripted: bool,
    min_sponsored_asset_fee: u64,
    origin_transaction_id: Id,
    script_details: ScriptDetails,
}

#[allow(clippy::too_many_arguments)]
impl AssetDetails {
    pub fn new(
        asset_id: AssetId,
        issue_height: u32,
        issue_timestamp: u64,
        issuer: Address,
        issuer_public_key: PublicKey,
        name: String,
        description: String,
        decimals: u32,
        reissuable: bool,
        quantity: u64,
        scripted: bool,
        min_sponsored_asset_fee: u64,
        origin_transaction_id: Id,
        script_details: ScriptDetails,
    ) -> Self {
        Self {
            asset_id,
            issue_height,
            issue_timestamp,
            issuer,
            issuer_public_key,
            name,
            description,
            decimals,
            reissuable,
            quantity,
            scripted,
            min_sponsored_asset_fee,
            origin_transaction_id,
            script_details,
        }
    }

    pub fn asset_id(&self) -> AssetId {
        self.asset_id.clone()
    }

    pub fn issue_height(&self) -> u32 {
        self.issue_height
    }

    pub fn issue_timestamp(&self) -> u64 {
        self.issue_timestamp
    }

    pub fn issuer(&self) -> Address {
        self.issuer.clone()
    }

    pub fn issuer_public_key(&self) -> PublicKey {
        self.issuer_public_key.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn decimals(&self) -> u32 {
        self.decimals
    }

    pub fn reissuable(&self) -> bool {
        self.reissuable
    }

    pub fn quantity(&self) -> u64 {
        self.quantity
    }

    pub fn scripted(&self) -> bool {
        self.scripted
    }

    pub fn min_sponsored_asset_fee(&self) -> u64 {
        self.min_sponsored_asset_fee
    }

    pub fn origin_transaction_id(&self) -> Id {
        self.origin_transaction_id.clone()
    }

    pub fn script_details(&self) -> ScriptDetails {
        self.script_details.clone()
    }
}

impl TryFrom<&Value> for AssetDetails {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let asset_id = JsonDeserializer::safe_to_string_from_field(value, "assetId")?;
        let issue_height = JsonDeserializer::safe_to_int_from_field(value, "issueHeight")?;
        let issue_timestamp = JsonDeserializer::safe_to_int_from_field(value, "issueTimestamp")?;
        let issuer = JsonDeserializer::safe_to_string_from_field(value, "issuer")?;
        let issuer_public_key =
            JsonDeserializer::safe_to_string_from_field(value, "issuerPublicKey")?;
        let name = JsonDeserializer::safe_to_string_from_field(value, "name")?;
        let description = JsonDeserializer::safe_to_string_from_field(value, "description")?;
        let decimals = JsonDeserializer::safe_to_int_from_field(value, "decimals")?;
        let reissuable = JsonDeserializer::safe_to_boolean_from_field(value, "reissuable")?;
        let quantity = JsonDeserializer::safe_to_int_from_field(value, "quantity")?;
        let scripted = JsonDeserializer::safe_to_boolean_from_field(value, "scripted")?;
        let min_sponsored_asset_fee =
            JsonDeserializer::safe_to_int_from_field(value, "minSponsoredAssetFee").unwrap_or(0);
        let origin_transaction_id =
            JsonDeserializer::safe_to_string_from_field(value, "originTransactionId")?;
        let script_details: ScriptDetails = value["scriptDetails"].borrow().try_into()?;

        Ok(AssetDetails {
            asset_id: AssetId::from_string(&asset_id)?,
            issue_height: issue_height as u32,
            issue_timestamp: issue_timestamp as u64,
            issuer: Address::from_string(&issuer)?,
            issuer_public_key: PublicKey::from_string(&issuer_public_key)?,
            name,
            description,
            decimals: decimals as u32,
            reissuable,
            quantity: quantity as u64,
            scripted,
            min_sponsored_asset_fee: min_sponsored_asset_fee as u64,
            origin_transaction_id: Id::from_string(&origin_transaction_id)?,
            script_details,
        })
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ScriptDetails {
    script: Base64String,
    complexity: u32,
}

impl ScriptDetails {
    pub fn new(script: Base64String, complexity: u32) -> Self {
        Self { script, complexity }
    }

    pub fn script(&self) -> Base64String {
        self.script.clone()
    }

    pub fn complexity(&self) -> u32 {
        self.complexity
    }
}

impl TryFrom<&Value> for ScriptDetails {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let script = match value["script"].as_str() {
            Some(script) => script,
            None => {
                return Ok(ScriptDetails {
                    script: Base64String::empty(),
                    complexity: 0,
                })
            }
        };
        let complexity = JsonDeserializer::safe_to_int_from_field(value, "scriptComplexity")?;
        Ok(ScriptDetails {
            script: Base64String::from_string(script)?,
            complexity: complexity as u32,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::asset::asset_details::{AssetDetails, ScriptDetails};
    use crate::model::{Base64String, ByteString};
    use serde_json::{json, Value};
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_asset_details() -> Result<()> {
        let data = fs::read_to_string("./tests/resources/assets/asset_details_rs.json")
            .expect("Unable to read file");
        let json: &Value = &serde_json::from_str(&data).expect("failed to convert");

        let asset_details: AssetDetails = json.try_into()?;

        assert_eq!(
            "CVwsbXjXmdYF2q4RCPuQKf7sLGpzhk7BNnYsxGZZJMym",
            asset_details.asset_id().encoded()
        );
        assert_eq!(2221593, asset_details.issue_height());
        assert_eq!(1662728397110, asset_details.issue_timestamp());
        assert_eq!(
            "3Ms6jp75u5qnfmAgWpxbt9xHv7znBp7RHnq",
            asset_details.issuer().encoded()
        );
        assert_eq!(
            "ASA4fMdz5FirDREfB34PPi67QxLHMt8tvzRQDT64juiM",
            asset_details.issuer_public_key().encoded()
        );
        assert_eq!("AssetWithScript", asset_details.name());
        assert_eq!("", asset_details.description());
        assert_eq!(0, asset_details.decimals());
        assert_eq!(true, asset_details.reissuable());
        assert_eq!(10000, asset_details.quantity());
        assert_eq!(true, asset_details.scripted());
        assert_eq!(0, asset_details.min_sponsored_asset_fee());
        assert_eq!(
            "CVwsbXjXmdYF2q4RCPuQKf7sLGpzhk7BNnYsxGZZJMym",
            asset_details.origin_transaction_id().encoded()
        );
        assert_eq!(127, asset_details.script_details().complexity());

        let expected_script = "AgQAAAAHbWFzdGVyMQkBAAAAEWFkZHJlc3NGcm9tU3RyaW5nAAAAAQIAAAAQMzMzbWFzdGVyQWRkcmVzcwQAAAAHJG1hdGNoMAUAAAACdHgDCQAAAQAAAAIFAAAAByRtYXRjaDACAAAAE1RyYW5zZmVyVHJhbnNhY3Rpb24EAAAAAXQFAAAAByRtYXRjaDADCQAAAAAAAAIIBQAAAAF0AAAABnNlbmRlcgUAAAAHbWFzdGVyMQYJAAAAAAAAAggFAAAAAXQAAAAJcmVjaXBpZW50BQAAAAdtYXN0ZXIxAwkAAAEAAAACBQAAAAckbWF0Y2gwAgAAABdNYXNzVHJhbnNmZXJUcmFuc2FjdGlvbgQAAAACbXQFAAAAByRtYXRjaDAJAAAAAAAAAggFAAAAAm10AAAABnNlbmRlcgUAAAAHbWFzdGVyMQMJAAABAAAAAgUAAAAHJG1hdGNoMAIAAAATRXhjaGFuZ2VUcmFuc2FjdGlvbgcGFLbwIw==";
        assert_eq!(
            expected_script,
            asset_details.script_details().script().encoded()
        );
        Ok(())
    }

    #[test]
    fn test_if_script_details_null() -> Result<()> {
        let json = json!({ "scriptDetails": null });
        let script_details: ScriptDetails = json["scriptDetails"].borrow().try_into()?;
        assert_eq!(script_details.script(), Base64String::empty());
        assert_eq!(script_details.complexity(), 0);
        Ok(())
    }
}
