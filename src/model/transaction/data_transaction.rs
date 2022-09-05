use crate::error::Result;
use crate::model::data_entry::DataEntry;
use crate::util::{Base64, JsonDeserializer};
use serde_json::{Map, Value};

const TYPE: u8 = 12;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DataTransactionInfo {
    data: Vec<DataEntry>,
}

impl DataTransactionInfo {
    pub fn from_json(value: &Value) -> Result<DataTransactionInfo> {
        let data_array = JsonDeserializer::safe_to_array_from_field(value, "data")?;
        let data = data_array
            .iter()
            .map(|entry| entry.try_into())
            .collect::<Result<Vec<DataEntry>>>()?;

        Ok(DataTransactionInfo { data })
    }

    pub fn new(data: Vec<DataEntry>) -> Self {
        DataTransactionInfo { data }
    }

    pub fn tx_type() -> u8 {
        TYPE
    }

    pub fn data(&self) -> Vec<DataEntry> {
        self.data.clone()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DataTransaction {
    data: Vec<DataEntry>,
}

impl DataTransaction {
    pub fn new(data: Vec<DataEntry>) -> Self {
        DataTransaction { data }
    }

    pub fn tx_type() -> u8 {
        TYPE
    }

    pub fn data(&self) -> Vec<DataEntry> {
        self.data.clone()
    }

    pub fn from_json(value: &Value) -> Result<DataTransaction> {
        let data_array = JsonDeserializer::safe_to_array_from_field(value, "data")?;
        let data = data_array
            .iter()
            .map(|entry| entry.try_into())
            .collect::<Result<Vec<DataEntry>>>()?;

        Ok(DataTransaction { data })
    }
}

impl From<DataEntry> for Value {
    fn from(data_entry: DataEntry) -> Self {
        let mut map: Map<String, Value> = Map::new();
        map.insert("key".to_string(), data_entry.key().into());
        match data_entry {
            DataEntry::IntegerEntry { key: _, value } => {
                map.insert("type".to_string(), "integer".into());
                map.insert("value".to_string(), value.into());
            }
            DataEntry::BooleanEntry { key: _, value } => {
                map.insert("type".to_string(), "boolean".into());
                map.insert("value".to_string(), value.into());
            }
            DataEntry::BinaryEntry { key: _, value } => {
                map.insert("type".to_string(), "binary".into());
                map.insert("value".to_string(), Base64::encode(&value, true).into());
            }
            DataEntry::StringEntry { key: _, value } => {
                map.insert("type".to_string(), "string".into());
                map.insert("value".to_string(), value.into());
            }
            DataEntry::DeleteEntry { key: _ } => {
                map.insert("value".to_string(), Value::Null);
            }
        };
        map.into()
    }
}
