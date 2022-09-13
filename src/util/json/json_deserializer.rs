use crate::error::{Error, Result};
use crate::model::account::{Address, Balance, BalanceDetails};
use crate::model::data_entry::DataEntry;
use crate::model::{ArgMeta, AssetId, Base64String, ScriptInfo, ScriptMeta};

use serde_json::Value;
use std::collections::HashMap;

pub struct JsonDeserializer;

impl JsonDeserializer {

    pub fn deserialize_data_array(value: &Value) -> Result<Vec<DataEntry>> {
        let data_array = Self::safe_to_array(value)?;
        data_array
            .iter()
            .map(|entry| entry.try_into())
            .collect::<Result<Vec<DataEntry>>>()
    }

    pub fn asset_id_from_json(json: &Value, field_name: &str) -> Result<Option<AssetId>> {
        let asset_id = match json[field_name].as_str() {
            Some(asset_id) => Some(AssetId::from_string(asset_id)?),
            None => None,
        };
        Ok(asset_id)
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
