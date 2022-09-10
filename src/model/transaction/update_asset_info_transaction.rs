use crate::error::{Error, Result};
use crate::model::AssetId;
use crate::util::JsonDeserializer;
use crate::waves_proto::UpdateAssetInfoTransactionData;
use serde_json::{Map, Value};

const TYPE: u8 = 17;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UpdateAssetInfoTransactionInfo {
    asset_id: AssetId,
    name: String,
    description: String,
}

impl UpdateAssetInfoTransactionInfo {
    pub fn new(asset_id: AssetId, name: String, description: String) -> Self {
        Self {
            asset_id,
            name,
            description,
        }
    }

    pub fn asset_id(&self) -> AssetId {
        self.asset_id.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl TryFrom<&Value> for UpdateAssetInfoTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let asset_id = JsonDeserializer::safe_to_string_from_field(value, "assetId")?;
        let name = JsonDeserializer::safe_to_string_from_field(value, "name")?;
        let description = JsonDeserializer::safe_to_string_from_field(value, "description")?;

        Ok(UpdateAssetInfoTransactionInfo {
            asset_id: AssetId::from_string(&asset_id)?,
            name,
            description,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UpdateAssetInfoTransaction {
    asset_id: AssetId,
    name: String,
    description: String,
}

impl UpdateAssetInfoTransaction {
    pub fn new(asset_id: AssetId, name: String, description: String) -> Self {
        Self {
            asset_id,
            name,
            description,
        }
    }

    pub fn asset_id(&self) -> AssetId {
        self.asset_id.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn tx_type() -> u8 {
        TYPE
    }
}

impl TryFrom<&Value> for UpdateAssetInfoTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let asset_id = JsonDeserializer::safe_to_string_from_field(value, "assetId")?;
        let name = JsonDeserializer::safe_to_string_from_field(value, "name")?;
        let description = JsonDeserializer::safe_to_string_from_field(value, "description")?;

        Ok(UpdateAssetInfoTransaction {
            asset_id: AssetId::from_string(&asset_id)?,
            name,
            description,
        })
    }
}

impl TryFrom<&UpdateAssetInfoTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(value: &UpdateAssetInfoTransaction) -> Result<Self> {
        let mut update_asset_info_tx_json = Map::new();
        update_asset_info_tx_json.insert("assetId".to_owned(), value.asset_id().encoded().into());
        update_asset_info_tx_json.insert("name".to_owned(), value.name().into());
        update_asset_info_tx_json.insert("description".to_owned(), value.description().into());
        Ok(update_asset_info_tx_json)
    }
}

impl TryFrom<&UpdateAssetInfoTransaction> for UpdateAssetInfoTransactionData {
    type Error = Error;

    fn try_from(value: &UpdateAssetInfoTransaction) -> Result<Self> {
        Ok(UpdateAssetInfoTransactionData {
            asset_id: value.asset_id().bytes(),
            name: value.name(),
            description: value.description(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::UpdateAssetInfoTransactionInfo;
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_sponsor_fee_transaction() {
        let data = fs::read_to_string("./tests/resources/update_asset_info_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let update_asset_info_from_json: UpdateAssetInfoTransactionInfo =
            json.borrow().try_into().unwrap();

        assert_eq!(
            "7qhc24Cq53DiaHUzmcaYMUKq8kidaVW8ZAvKrTtADozG",
            update_asset_info_from_json.asset_id().encoded()
        );
        assert_eq!("UpdatedAsset", update_asset_info_from_json.name());
        assert_eq!(
            "updated description",
            update_asset_info_from_json.description()
        );
    }
}
