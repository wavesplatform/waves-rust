use crate::errors::FieldNotFoundError;
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
    pub fn deserialize_tx_info(
        value: Value,
        chain_id: u8,
    ) -> Result<TransactionInfo, FieldNotFoundError> {
        let id = Self::safe_to_str(&value, "id")?;

        let application_status = match Self::safe_to_str(&value, "applicationStatus")?.as_str() {
            "succeeded" => ApplicationStatus::Succeed,
            //todo check statuses
            "scriptExecutionFailed" => ApplicationStatus::ScriptExecutionFailed,
            &_ => ApplicationStatus::Unknown,
        };
        let height = Self::safe_to_int(&value, "height")? as u32;
        let signed_transaction = Self::deserialize_signed_tx(&value, chain_id)?;

        Ok(TransactionInfo::new(
            id,
            signed_transaction,
            application_status,
            height,
        ))
    }

    pub fn deserialize_signed_tx(
        value: &Value,
        chain_id: u8,
    ) -> Result<SignedTransaction, FieldNotFoundError> {
        let transaction = Self::deserialize_tx(value, chain_id)?;
        let proofs_array = Self::safe_to_array(value, "proofs")?;
        let proofs = proofs_array
            .iter()
            // todo remove unwrap
            .map(|v| Base58::decode(v.as_str().unwrap()).unwrap())
            .collect::<Vec<Vec<u8>>>();
        Ok(SignedTransaction::new(transaction, proofs))
    }

    pub fn deserialize_tx(value: &Value, chain_id: u8) -> Result<Transaction, FieldNotFoundError> {
        let tx_type = Self::safe_to_int(value, "type")? as u8;
        let fee = Self::safe_to_int(value, "fee")? as u64;
        let fee_asset_id = value["feeAssetId"].as_str().map(|value| value.into());
        let transaction_data = match tx_type {
            4 => Transfer(TransferTransaction::from_json(value)),
            12 => Data(DataTransaction::from_json(value)),
            _ => panic!("unknown tx type"),
        };
        let timestamp = Self::safe_to_int(value, "timestamp")? as u64;
        let public_key = Self::safe_to_str(value, "senderPublicKey")?.try_into();
        let version = Self::safe_to_int(value, "version")? as u8;
        Ok(Transaction::new(
            transaction_data,
            Amount::new(fee, fee_asset_id),
            timestamp,
            public_key.unwrap(),
            tx_type,
            version,
            chain_id,
        ))
    }

    pub fn safe_to_str(json: &Value, field_name: &str) -> Result<String, FieldNotFoundError> {
        let string = json[field_name]
            .as_str()
            .ok_or_else(|| FieldNotFoundError::new(json, field_name.to_owned()))?;
        Ok(string.into())
    }

    pub fn safe_to_int(json: &Value, field_name: &str) -> Result<i64, FieldNotFoundError> {
        let int = json[field_name]
            .as_i64()
            .ok_or_else(|| FieldNotFoundError::new(json, field_name.to_owned()))?;
        Ok(int)
    }

    pub fn safe_to_array(json: &Value, field_name: &str) -> Result<Vec<Value>, FieldNotFoundError> {
        let array = json[field_name]
            .as_array()
            .ok_or_else(|| FieldNotFoundError::new(json, field_name.to_owned()))?;
        Ok(array.to_owned())
    }
}
