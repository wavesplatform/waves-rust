use crate::error::{Error, Result};
use crate::model::{AssetId, Base64String};
use crate::util::JsonDeserializer;
use serde_json::Value;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct IssueAction {
    asset_id: AssetId,
    name: String,
    description: String,
    quantity: u64,
    decimals: u32,
    reissuable: bool,
    script: Base64String,
    nonce: u32,
}

#[allow(clippy::too_many_arguments)]
impl IssueAction {
    pub fn new(
        asset_id: AssetId,
        name: String,
        description: String,
        quantity: u64,
        decimals: u32,
        reissuable: bool,
        script: Base64String,
        nonce: u32,
    ) -> Self {
        Self {
            asset_id,
            name,
            description,
            quantity,
            decimals,
            reissuable,
            script,
            nonce,
        }
    }

    pub fn asset_id(&self) -> AssetId {
        self.asset_id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn quantity(&self) -> u64 {
        self.quantity
    }

    pub fn decimals(&self) -> u32 {
        self.decimals
    }

    pub fn is_reissuable(&self) -> bool {
        self.reissuable
    }

    pub fn script(&self) -> Base64String {
        self.script.clone()
    }

    pub fn nonce(&self) -> u32 {
        self.nonce
    }
}

impl TryFrom<&Value> for IssueAction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let asset_id = JsonDeserializer::safe_to_string_from_field(value, "assetId")?;
        let name = JsonDeserializer::safe_to_string_from_field(value, "name")?;
        let description = JsonDeserializer::safe_to_string_from_field(value, "description")?;
        let quantity = JsonDeserializer::safe_to_int_from_field(value, "quantity")?;
        let decimals = JsonDeserializer::safe_to_int_from_field(value, "decimals")?;
        let reissuable = JsonDeserializer::safe_to_boolean_from_field(value, "isReissuable")?;
        let script = JsonDeserializer::safe_to_string_from_field(value, "compiledScript")
            .unwrap_or_else(|_| "".to_owned());
        let nonce = JsonDeserializer::safe_to_int_from_field(value, "nonce")?;

        Ok(IssueAction {
            asset_id: AssetId::from_string(&asset_id)?,
            name,
            description,
            quantity: quantity as u64,
            decimals: decimals as u32,
            reissuable,
            script: Base64String::from_string(&script)?,
            nonce: nonce as u32,
        })
    }
}
