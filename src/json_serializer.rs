use serde_json::Value;
use crate::json_serializer::TransactionData::Transfer;

pub fn from_json(value: Value) -> Transaction {
    let recipient = value["recipient"].as_str().unwrap().into();
    let asset = match value["assetId"].as_str() {
        Some(value) => Some(value.into()),
        None => None
    };
    let fee_asset = match value["feeAssetId"].as_str() {
        Some(value) => Some(value.into()),
        None => None
    };
    let attachment = match value["attachment"].as_str() {
        Some(value) => Some(value.into()),
        None => None
    };

    Transaction {
        data: Transfer {
            recipient,
            asset,
            amount: value["amount"].as_i64().unwrap() as u64,
            fee_asset,
            attachment,
        },
        fee: value["fee"].as_i64().unwrap() as u64,
        timestamp: value["timestamp"].as_i64().unwrap() as u64,
        sender_public_key: value["senderPublicKey"].as_str().unwrap().into(),
        type_id: value["type"].as_i64().unwrap() as u8,
        version: value["version"].as_i64().unwrap() as u8,
    }
}

pub struct Transaction {
    data: TransactionData,
    fee: u64,
    timestamp: u64,
    sender_public_key: String,
    type_id: u8,
    version: u8,
}

impl Transaction {
    pub fn data(&self) -> &TransactionData {
        &self.data
    }
}

pub enum TransactionData {
    Transfer {
        recipient: String,
        asset: Option<String>,
        amount: u64,
        fee_asset: Option<String>,
        attachment: Option<String>,
    }
}