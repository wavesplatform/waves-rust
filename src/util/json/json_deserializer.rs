use crate::model::TransactionData::{Data, Transfer};
use crate::model::{
    Amount, ApplicationStatus, DataTransaction, SignedTransaction, Transaction, TransactionInfo,
    TransferTransaction,
};
use crate::util::Base58;
use serde_json::Value;

pub struct JsonDeserializer;

// todo return Result<TransactionInfo, Error>
impl JsonDeserializer {
    pub fn deserialize_tx_info(value: Value, chain_id: u8) -> TransactionInfo {
        // todo rm unwrap add handler for all reading fields
        let id = value["id"].as_str().unwrap().into();

        let application_status = match value["applicationStatus"].as_str().unwrap() {
            "succeeded" => ApplicationStatus::Succeed,
            //todo check status
            "scriptExecutionFailed" => ApplicationStatus::ScriptExecutionFailed,
            &_ => ApplicationStatus::Unknown,
        };
        let height = value["height"].as_i64().unwrap() as u32;
        let signed_transaction = Self::deserialize_signed_tx(&value, chain_id);

        TransactionInfo::new(id, signed_transaction, application_status, height)
    }

    pub fn deserialize_tx(value: &Value, chain_id: u8) -> Transaction {
        let tx_type = value["type"].as_i64().unwrap() as u8;
        let fee = value["fee"].as_i64().unwrap() as u64;
        let fee_asset_id = value["feeAssetId"].as_str().map(|value| value.into());
        let transaction_data = match tx_type {
            4 => Transfer(TransferTransaction::from_json(value)),
            12 => Data(DataTransaction::from_json(value)),
            _ => panic!("unknown tx type"),
        };
        let timestamp = value["timestamp"].as_i64().unwrap() as u64;
        let public_key = value["senderPublicKey"].as_str().unwrap().into();
        let version = value["version"].as_i64().unwrap() as u8;
        Transaction::new(
            transaction_data,
            Amount::new(fee, fee_asset_id),
            timestamp,
            public_key,
            tx_type,
            version,
            chain_id,
        )
    }

    pub fn deserialize_signed_tx(value: &Value, chain_id: u8) -> SignedTransaction {
        let transaction = Self::deserialize_tx(value, chain_id);
        let proofs = value["proofs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| Base58::decode(v.as_str().unwrap()).unwrap())
            .collect::<Vec<Vec<u8>>>();
        SignedTransaction::new(transaction, proofs)
    }
}
