use crate::model::data_entry::DataEntry::{BinaryEntry, BooleanEntry, IntegerEntry, StringEntry};
use crate::util::Base64;
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

impl From<&Value> for DataEntry {
    fn from(value: &Value) -> Self {
        let key_field = value["key"].as_str().unwrap().into();
        let value_field = &value["value"];
        match value["type"].as_str().unwrap() {
            "binary" => BinaryEntry {
                key: key_field,
                value: Base64::decode(value_field.as_str().unwrap()).unwrap(),
            },
            "boolean" => BooleanEntry {
                key: key_field,
                value: value_field.as_bool().unwrap(),
            },
            "integer" => IntegerEntry {
                key: key_field,
                value: value_field.as_i64().unwrap(),
            },
            "string" => StringEntry {
                key: key_field,
                value: value_field.as_str().unwrap().into(),
            },
            _ => panic!("unknown type"),
        }
    }
}
