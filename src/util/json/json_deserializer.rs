use crate::error::{Error, Result};
use crate::model::account::{Address, Balance, BalanceDetails};
use crate::model::data_entry::DataEntry;
use crate::model::{ArgMeta, Base64String, ScriptInfo, ScriptMeta};

use serde_json::Value;
use std::collections::HashMap;

pub struct JsonDeserializer;

impl JsonDeserializer {
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
        let int = match json[field_name].as_i64() {
            Some(int) => int,
            None => match json[field_name].as_str() {
                Some(int) => int.parse().map_err(|_| Error::JsonParseError {
                    json: json.to_string(),
                    field: field_name.to_owned(),
                })?,
                None => Err(Error::JsonParseError {
                    json: json.to_string(),
                    field: field_name.to_owned(),
                })?,
            },
        };
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
