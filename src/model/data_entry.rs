use crate::error::{Error, Result};
use crate::model::data_entry::DataEntry::{BinaryEntry, BooleanEntry, IntegerEntry, StringEntry};
use crate::util::{Base64, JsonDeserializer};
use serde_json::Value;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum DataEntry {
    IntegerEntry { key: String, value: i64 },
    BooleanEntry { key: String, value: bool },
    BinaryEntry { key: String, value: Vec<u8> },
    StringEntry { key: String, value: String },
}

impl DataEntry {
    pub fn key(&self) -> String {
        match self {
            IntegerEntry { key, .. } => key.clone(),
            BooleanEntry { key, .. } => key.clone(),
            BinaryEntry { key, .. } => key.clone(),
            StringEntry { key, .. } => key.clone(),
        }
    }
}

impl TryFrom<&Value> for DataEntry {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let key_field = JsonDeserializer::safe_to_string_from_field(value, "key")?;
        let value_field = &value["value"];
        let string_data_entry_type = JsonDeserializer::safe_to_string_from_field(value, "type")?;
        let data_entry_type = string_data_entry_type.as_str();
        match data_entry_type {
            "binary" => Ok(BinaryEntry {
                key: key_field,
                value: Base64::decode(&JsonDeserializer::safe_to_string(value_field)?)?,
            }),
            "boolean" => Ok(BooleanEntry {
                key: key_field,
                value: JsonDeserializer::safe_to_boolean(value_field)?,
            }),
            "integer" => Ok(IntegerEntry {
                key: key_field,
                value: JsonDeserializer::safe_to_int(value_field)?,
            }),
            "string" => Ok(StringEntry {
                key: key_field,
                value: JsonDeserializer::safe_to_string(value_field)?,
            }),
            _ => Err(Error::JsonParseError {
                field: data_entry_type.to_owned(),
                json: value.to_string(),
            }),
        }
    }
}
