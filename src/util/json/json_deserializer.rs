use crate::constants::HASH_LENGTH;
use crate::error::{Error, Result};
use crate::model::account::{Address, Balance, BalanceDetails};
use crate::model::data_entry::DataEntry;
use crate::model::{
    Amount, ApplicationStatus, ArgMeta, AssetId, Base64String, DataTransaction,
    DataTransactionInfo, Id, InvokeScriptTransaction, IssueTransaction, IssueTransactionInfo,
    PublicKey, ScriptInfo, ScriptMeta, SignedTransaction, Transaction, TransactionData,
    TransactionDataInfo, TransactionInfoResponse, TransferTransaction, TransferTransactionInfo,
};
use crate::util::Base58;
use serde_json::Value;
use std::collections::HashMap;

pub struct JsonDeserializer;

impl JsonDeserializer {
    pub fn deserialize_tx_info(value: &Value, chain_id: u8) -> Result<TransactionInfoResponse> {
        let id = Id::from_string(&Self::safe_to_string_from_field(value, "id")?)?;

        let application_status =
            match Self::safe_to_string_from_field(value, "applicationStatus")?.as_str() {
                "succeeded" => ApplicationStatus::Succeed,
                //todo check statuses in wavesJ
                "scriptExecutionFailed" => ApplicationStatus::ScriptExecutionFailed,
                &_ => ApplicationStatus::Unknown,
            };
        let height = Self::safe_to_int_from_field(value, "height")? as u32;
        let proofs_array = Self::safe_to_array_from_field(value, "proofs")?;
        let tx_type = Self::safe_to_int_from_field(value, "type")? as u8;
        let fee = Self::safe_to_int_from_field(value, "fee")? as u64;
        let fee_asset_id = match value["feeAssetId"].as_str() {
            Some(val) => Some(AssetId::from_string(val)?),
            None => None,
        };
        let transaction_data = match tx_type {
            3 => TransactionDataInfo::Issue(IssueTransactionInfo::from_json(value)?),
            4 => TransactionDataInfo::Transfer(TransferTransactionInfo::from_json(value)?),
            5 => TransactionDataInfo::Reissue(value.try_into()?),
            6 => TransactionDataInfo::Burn(value.try_into()?),
            7 => TransactionDataInfo::Exchange(value.try_into()?),
            8 => TransactionDataInfo::Lease(value.try_into()?),
            9 => TransactionDataInfo::LeaseCancel(value.try_into()?),
            10 => TransactionDataInfo::CreateAlias(value.try_into()?),
            11 => TransactionDataInfo::MassTransfer(value.try_into()?),
            12 => TransactionDataInfo::Data(DataTransactionInfo::from_json(value)?),
            13 => TransactionDataInfo::SetScript(value.try_into()?),
            14 => TransactionDataInfo::SponsorFee(value.try_into()?),
            15 => TransactionDataInfo::SetAssetScript(value.try_into()?),
            16 => TransactionDataInfo::Invoke(value.try_into()?),
            17 => TransactionDataInfo::UpdateAssetInfo(value.try_into()?),
            _ => panic!("unknown tx type"),
        };
        let timestamp = Self::safe_to_int_from_field(value, "timestamp")? as u64;
        let public_key = Self::safe_to_string_from_field(value, "senderPublicKey")?.try_into()?;
        let version = Self::safe_to_int_from_field(value, "version")? as u8;

        let proofs = proofs_array
            .iter()
            .map(|v| Base58::decode(&Self::safe_to_string(v)?))
            .collect::<Result<Vec<Vec<u8>>>>()?;

        Ok(TransactionInfoResponse::new(
            id,
            application_status,
            transaction_data,
            Amount::new(fee, fee_asset_id),
            timestamp,
            public_key,
            tx_type,
            version,
            chain_id,
            height,
            proofs,
        ))
    }

    pub fn deserialize_signed_tx(value: &Value, chain_id: u8) -> Result<SignedTransaction> {
        let transaction = Self::deserialize_tx(value, chain_id)?;

        let proofs_array = if transaction.tx_type() == 1 {
            vec![Value::String(JsonDeserializer::safe_to_string_from_field(
                value,
                "signature",
            )?)]
        } else {
            Self::safe_to_array_from_field(value, "proofs")?
        };

        let proofs = proofs_array
            .iter()
            .map(|v| Base58::decode(&Self::safe_to_string(v)?))
            .collect::<Result<Vec<Vec<u8>>>>()?;
        Ok(SignedTransaction::new(transaction, proofs))
    }

    pub fn deserialize_tx(value: &Value, chain_id: u8) -> Result<Transaction> {
        let tx_type = Self::safe_to_int_from_field(value, "type")? as u8;
        let fee = Self::safe_to_int_from_field(value, "fee")? as u64;
        let fee_asset_id = match value["feeAssetId"].as_str() {
            Some(val) => Some(AssetId::from_string(val)?),
            None => None,
        };
        let transaction_data = match tx_type {
            3 => TransactionData::Issue(IssueTransaction::from_json(value)?),
            4 => TransactionData::Transfer(TransferTransaction::from_json(value)?),
            5 => TransactionData::Reissue(value.try_into()?),
            6 => TransactionData::Burn(value.try_into()?),
            7 => TransactionData::Exchange(value.try_into()?),
            8 => TransactionData::Lease(value.try_into()?),
            9 => TransactionData::LeaseCancel(value.try_into()?),
            10 => TransactionData::CreateAlias(value.try_into()?),
            11 => TransactionData::MassTransfer(value.try_into()?),
            12 => TransactionData::Data(DataTransaction::from_json(value)?),
            13 => TransactionData::SetScript(value.try_into()?),
            14 => TransactionData::SponsorFee(value.try_into()?),
            15 => TransactionData::SetAssetScript(value.try_into()?),
            16 => TransactionData::InvokeScript(InvokeScriptTransaction::from_json(value)?),
            17 => TransactionData::UpdateAssetInfo(value.try_into()?),
            _ => todo!(),
        };
        let timestamp = Self::safe_to_int_from_field(value, "timestamp")? as u64;
        let public_key = if tx_type == 1 {
            PublicKey::from_bytes(&[0; HASH_LENGTH])
        } else {
            Self::safe_to_string_from_field(value, "senderPublicKey")?.try_into()?
        };
        let version = Self::safe_to_int_from_field(value, "version")? as u8;
        Ok(Transaction::new(
            transaction_data,
            Amount::new(fee, fee_asset_id),
            timestamp,
            public_key,
            version,
            chain_id,
        ))
    }

    pub fn deserialize_addresses(value: &Value) -> Result<Vec<Address>> {
        let array = Self::safe_to_array(value)?;
        array.iter().map(Self::deserialize_address).collect()
    }

    pub fn deserialize_address(value: &Value) -> Result<Address> {
        let string = Self::safe_to_string(value)?;
        Address::from_string(&string)
    }

    pub fn deserialize_balances(value: &Value) -> Result<Vec<Balance>> {
        let array = Self::safe_to_array(value)?;
        array.iter().map(Self::deserialize_balance).collect()
    }

    pub fn deserialize_balance(value: &Value) -> Result<Balance> {
        let address = Address::from_string(&Self::safe_to_string_from_field(value, "id")?)?;
        let balance = Self::safe_to_int_from_field(value, "balance")?;
        Ok(Balance::new(address, balance as u64))
    }

    pub fn deserialize_balance_details(value: &Value) -> Result<BalanceDetails> {
        let address = Address::from_string(&Self::safe_to_string_from_field(value, "address")?)?;
        let available = Self::safe_to_int_from_field(value, "available")? as u64;
        let regular = Self::safe_to_int_from_field(value, "regular")? as u64;
        let generating = Self::safe_to_int_from_field(value, "generating")? as u64;
        let effective = Self::safe_to_int_from_field(value, "effective")? as u64;
        Ok(BalanceDetails::new(
            address, available, regular, generating, effective,
        ))
    }

    pub fn deserialize_data_array(value: &Value) -> Result<Vec<DataEntry>> {
        let data_array = Self::safe_to_array(value)?;
        data_array
            .iter()
            .map(|entry| entry.try_into())
            .collect::<Result<Vec<DataEntry>>>()
    }

    pub fn deserialize_script_info(value: &Value) -> Result<ScriptInfo> {
        let script = Base64String::from_string(
            &Self::safe_to_string_from_field(value, "script").unwrap_or_else(|_| "".to_owned()),
        )?;
        let complexity = Self::safe_to_int_from_field(value, "complexity")? as u32;
        let verifier_complexity = Self::safe_to_int_from_field(value, "verifierComplexity")? as u32;
        let callable_complexities: HashMap<String, u32> =
            Self::safe_to_map_from_field(value, "callableComplexities")?
                .into_iter()
                .map(|entry| {
                    Ok((
                        entry.0.to_owned(),
                        JsonDeserializer::safe_to_int(&entry.1)? as u32,
                    ))
                })
                .collect::<Result<HashMap<String, u32>>>()?;
        let extra_fee = Self::safe_to_int_from_field(value, "extraFee")? as u64;
        let script_text =
            Self::safe_to_string_from_field(value, "scriptText").unwrap_or_else(|_| "".to_owned());
        Ok(ScriptInfo::new(
            script,
            complexity,
            verifier_complexity,
            callable_complexities,
            extra_fee,
            script_text,
        ))
    }

    pub fn deserialize_script_meta(value: &Value) -> Result<ScriptMeta> {
        let meta_version: u32 = Self::safe_to_string_from_field(&value["meta"], "version")
            .unwrap_or_else(|_| "0".to_string())
            .parse()
            .unwrap_or(0);
        if meta_version == 0 {
            return Ok(ScriptMeta::new(meta_version, HashMap::new()));
        }
        let callable_func_types =
            Self::safe_to_map_from_field(&value["meta"], "callableFuncTypes")?;

        let callable_functions: HashMap<String, Vec<ArgMeta>> = callable_func_types
            .into_iter()
            .map(|entry| {
                let arg_meta = Self::safe_to_array(&entry.1)
                    .unwrap_or_default()
                    .iter()
                    .map(|arg| {
                        let arg_name = Self::safe_to_string_from_field(arg, "name")
                            .unwrap_or_else(|_| "".to_owned());
                        let arg_type = Self::safe_to_string_from_field(arg, "type")
                            .unwrap_or_else(|_| "".to_owned());
                        ArgMeta::new(arg_name, arg_type)
                    })
                    .collect();
                (entry.0, arg_meta)
            })
            .collect();
        Ok(ScriptMeta::new(meta_version, callable_functions))
    }

    pub fn safe_to_string_from_field(json: &Value, field_name: &str) -> Result<String> {
        let string = json[field_name]
            .as_str()
            .ok_or_else(|| Error::JsonParseError {
                json: json.to_string(),
                field: field_name.to_owned(),
            })?;
        Ok(string.into())
    }

    pub fn safe_to_int_from_field(json: &Value, field_name: &str) -> Result<i64> {
        let int = json[field_name]
            .as_i64()
            .ok_or_else(|| Error::JsonParseError {
                json: json.to_string(),
                field: field_name.to_owned(),
            })?;
        Ok(int)
    }

    pub fn safe_to_array_from_field(json: &Value, field_name: &str) -> Result<Vec<Value>> {
        let array = json[field_name]
            .as_array()
            .ok_or_else(|| Error::JsonParseError {
                json: json.to_string(),
                field: field_name.to_owned(),
            })?;
        Ok(array.to_owned())
    }

    pub fn safe_to_map_from_field(
        json: &Value,
        field_name: &str,
    ) -> Result<serde_json::Map<String, Value>> {
        let map = json[field_name]
            .as_object()
            .ok_or_else(|| Error::JsonParseError {
                json: json.to_string(),
                field: field_name.to_owned(),
            })?;
        Ok(map.to_owned())
    }

    pub fn safe_to_boolean_from_field(json: &Value, field_name: &str) -> Result<bool> {
        let bool = json[field_name]
            .as_bool()
            .ok_or_else(|| Error::JsonParseError {
                json: json.to_string(),
                field: field_name.to_owned(),
            })?;
        Ok(bool)
    }

    pub fn safe_to_string(json: &Value) -> Result<String> {
        let string = json.as_str().ok_or_else(|| Error::JsonParseError {
            json: json.to_string(),
            field: "String".to_owned(),
        })?;
        Ok(string.to_owned())
    }

    pub fn safe_to_int(json: &Value) -> Result<i64> {
        let int = json.as_i64().ok_or_else(|| Error::JsonParseError {
            json: json.to_string(),
            field: "i64".to_owned(),
        })?;
        Ok(int.to_owned())
    }

    pub fn safe_to_boolean(json: &Value) -> Result<bool> {
        let bool = json.as_bool().ok_or_else(|| Error::JsonParseError {
            json: json.to_string(),
            field: "bool".to_owned(),
        })?;
        Ok(bool)
    }

    pub fn safe_to_array(json: &Value) -> Result<Vec<Value>> {
        let array = json.as_array().ok_or_else(|| Error::JsonParseError {
            json: json.to_string(),
            field: "Vec<Value>".to_owned(),
        })?;
        Ok(array.to_owned())
    }
}
