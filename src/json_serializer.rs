use serde_json::Value;
use crate::model::{ApplicationStatus, Transaction, TransactionInfo, TransferTransaction};
use crate::model::TransactionData::Transfer;

pub fn from_json(value: Value) -> TransactionInfo {
    let fee = value["fee"].as_i64().unwrap() as u64;
    let timestamp = value["timestamp"].as_i64().unwrap() as u64;
    let sender_public_key = value["senderPublicKey"].as_str().unwrap().into();
    let tx_type = value["type"].as_i64().unwrap() as u8;
    let version = value["version"].as_i64().unwrap() as u8;
    let id = value["id"].as_str().unwrap().into();

    let application_status = match value["applicationStatus"].as_str().unwrap() {
        "succeeded" => ApplicationStatus::Succeed,
        "scriptExecutionFailed" => ApplicationStatus::ScriptExecutionFailed,
        &_ => ApplicationStatus::Unknown
    };
    let height = value["height"].as_i64().unwrap() as u32;

    let transaction_data = Transfer(TransferTransaction::from_json(value));
    let transaction = Transaction::new(
        transaction_data,
        fee,
        timestamp,
        sender_public_key,
        tx_type,
        version,
    );
    TransactionInfo::new(id, transaction, application_status, height)
}