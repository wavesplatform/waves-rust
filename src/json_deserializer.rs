use serde_json::Value;
use crate::model::{ApplicationStatus, SignedTransaction, Transaction, TransactionInfo, TransferTransaction};
use crate::model::TransactionData::Transfer;

// todo return Result<TransactionInfo, Error>
pub fn from_json(value: Value) -> TransactionInfo {

    // todo rm unwrap add handler for all reading fields
    let fee = value["fee"].as_i64().unwrap() as u64;
    let fee_asset_id = value["feeAssetId"].as_str().map(|value| value.into());
    let timestamp = value["timestamp"].as_i64().unwrap() as u64;

    let public_key = value["senderPublicKey"].as_str().unwrap().into();

    let tx_type = value["type"].as_i64().unwrap() as u8;
    let version = value["version"].as_i64().unwrap() as u8;
    let id = value["id"].as_str().unwrap().into();

    let proofs = value["proofs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let application_status = match value["applicationStatus"].as_str().unwrap() {
        "succeeded" => ApplicationStatus::Succeed,
        //todo check status
        "scriptExecutionFailed" => ApplicationStatus::ScriptExecutionFailed,
        &_ => ApplicationStatus::Unknown
    };
    let height = value["height"].as_i64().unwrap() as u32;

    let transaction_data = Transfer(TransferTransaction::from_json(value));
    let transaction = Transaction::new(
        transaction_data,
        fee,
        fee_asset_id,
        timestamp,
        public_key,
        tx_type,
        version,
    );

    let signed_transaction = SignedTransaction::new(transaction, proofs);

    TransactionInfo::new(id, signed_transaction, application_status, height)
}