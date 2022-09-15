use crate::error::{Error, Result};
use crate::model::{AssetId, Base64String, ByteString};
use crate::util::JsonDeserializer;
use crate::waves_proto::IssueTransactionData;
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
        self.is_reissuable
    }

    pub fn script(&self) -> Option<Base64String> {
        self.script.clone()
    }
}

impl TryFrom<&Value> for IssueTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let issue_transaction: IssueTransaction = value.try_into()?;
        let asset_id = AssetId::from_string(&JsonDeserializer::safe_to_string_from_field(
            value, "assetId",
        )?)?;

        Ok(IssueTransactionInfo {
            asset_id,
            name: issue_transaction.name(),
            description: issue_transaction.description(),
            quantity: issue_transaction.quantity(),
            decimals: issue_transaction.decimals(),
            is_reissuable: issue_transaction.is_reissuable(),
            script: issue_transaction.script(),
        })
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

impl TryFrom<&IssueTransaction> for IssueTransactionData {
    type Error = Error;

    fn try_from(issue_tx: &IssueTransaction) -> Result<Self> {
        let script = match issue_tx.script() {
            Some(script) => script.bytes(),
            None => vec![],
        };

        Ok(IssueTransactionData {
            name: issue_tx.name(),
            description: issue_tx.description(),
            amount: issue_tx.quantity() as i64,
            decimals: issue_tx.decimals() as i32,
            reissuable: issue_tx.is_reissuable(),
            script,
        })
    }
}

impl TryFrom<&Value> for IssueTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
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
}
