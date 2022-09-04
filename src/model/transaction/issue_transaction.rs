use crate::error::Result;
use crate::model::{AssetId, Base64String};
use crate::util::JsonDeserializer;
use serde_json::Value;

const TYPE: u8 = 3;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IssueTransactionInfo {
    asset_id: AssetId,
    name: String,
    description: String,
    quantity: u64,
    decimals: u32,
    is_reissuable: bool,
    script: Option<Base64String>,
}

impl IssueTransactionInfo {
    pub fn from_json(value: &Value) -> Result<IssueTransactionInfo> {
        let name = JsonDeserializer::safe_to_string_from_field(value, "name")?;
        let description = JsonDeserializer::safe_to_string_from_field(value, "description")?;
        let quantity = JsonDeserializer::safe_to_int_from_field(value, "quantity")? as u64;
        let decimals = JsonDeserializer::safe_to_int_from_field(value, "decimals")? as u32;
        let is_reissuable = JsonDeserializer::safe_to_boolean_from_field(value, "reissuable")?;
        let script = match value["script"].as_str() {
            Some(val) => Some(Base64String::from_string(val)?),
            None => None,
        };
        let asset_id = AssetId::from_string(&JsonDeserializer::safe_to_string_from_field(
            value, "assetId",
        )?)?;

        Ok(IssueTransactionInfo {
            asset_id,
            name,
            description,
            quantity,
            decimals,
            is_reissuable,
            script,
        })
    }

    pub fn new(
        asset_id: AssetId,
        name: String,
        description: String,
        quantity: u64,
        decimals: u32,
        is_reissuable: bool,
        script: Option<Base64String>,
    ) -> Self {
        IssueTransactionInfo {
            asset_id,
            name,
            description,
            quantity,
            decimals,
            is_reissuable,
            script,
        }
    }

    pub fn tx_type() -> u8 {
        TYPE
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
        self.is_reissuable
    }

    pub fn script(&self) -> Option<Base64String> {
        self.script.clone()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IssueTransaction {
    name: String,
    description: String,
    quantity: u64,
    decimals: u32,
    is_reissuable: bool,
    script: Option<Base64String>,
}

impl IssueTransaction {
    pub fn from_json(value: &Value) -> Result<IssueTransaction> {
        let name = JsonDeserializer::safe_to_string_from_field(value, "name")?;
        let description = JsonDeserializer::safe_to_string_from_field(value, "description")?;
        let quantity = JsonDeserializer::safe_to_int_from_field(value, "quantity")? as u64;
        let decimals = JsonDeserializer::safe_to_int_from_field(value, "decimals")? as u32;
        let is_reissuable = JsonDeserializer::safe_to_boolean_from_field(value, "reissuable")?;
        let script = match value["script"].as_str() {
            Some(val) => Some(Base64String::from_string(val)?),
            None => None,
        };

        Ok(IssueTransaction {
            name,
            description,
            quantity,
            decimals,
            is_reissuable,
            script,
        })
    }

    pub fn new(
        name: String,
        description: String,
        quantity: u64,
        decimals: u32,
        is_reissuable: bool,
        script: Option<Base64String>,
    ) -> Self {
        IssueTransaction {
            name,
            description,
            quantity,
            decimals,
            is_reissuable,
            script,
        }
    }

    pub fn tx_type() -> u8 {
        TYPE
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
        self.is_reissuable
    }

    pub fn script(&self) -> Option<Base64String> {
        self.script.clone()
    }
}
