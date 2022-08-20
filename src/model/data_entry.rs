use crate::model::data_entry::DataEntry::{
    BinaryEntry, BooleanEntry, DeleteEntry, IntegerEntry, StringEntry,
};
use serde_json::Value;

#[derive(Clone)]
pub enum DataEntry {
    IntegerEntry { key: String, value: u64 },
    BooleanEntry { key: String, value: bool },
    BinaryEntry { key: String, value: Vec<u8> },
    StringEntry { key: String, value: String },
    DeleteEntry { key: String },
}

impl From<&Value> for DataEntry {
    fn from(value: &Value) -> Self {
        let key_field = value["key"].as_str().unwrap().into();
        let value_field = &value["value"];
        match value["type"].as_str().unwrap() {
            "" => DeleteEntry { key: key_field },
            "binary" => BinaryEntry {
                key: key_field,
                value: base64::decode(value_field.as_str().unwrap()).unwrap(),
            },
            "boolean" => BooleanEntry {
                key: key_field,
                value: value_field.as_bool().unwrap(),
            },
            "integer" => IntegerEntry {
                key: key_field,
                value: value_field.as_u64().unwrap(),
            },
            "string" => StringEntry {
                key: key_field,
                value: value_field.as_str().unwrap().into(),
            },
            _ => panic!("unknown type"),
        }
    }
}
