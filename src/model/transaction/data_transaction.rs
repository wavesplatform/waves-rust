use crate::error::{Error, Result};
use crate::model::data_entry::DataEntry;
use crate::util::{Base64, JsonDeserializer};
use crate::waves_proto::data_transaction_data::data_entry::Value::{
    BinaryValue, BoolValue, IntValue, StringValue,
};
use crate::waves_proto::data_transaction_data::DataEntry as ProtoDataEntry;
use crate::waves_proto::DataTransactionData;
use serde_json::{Map, Value};

const TYPE: u8 = 12;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DataTransactionInfo {
    data: Vec<DataEntry>,
}

impl DataTransactionInfo {
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

impl TryFrom<&Value> for DataTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let data_transaction: DataTransaction = value.try_into()?;
        Ok(DataTransactionInfo {
            data: data_transaction.data(),
        })
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
}

impl TryFrom<&Value> for DataTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let data_array = JsonDeserializer::safe_to_array_from_field(value, "data")?;
        let data = data_array
            .iter()
            .map(|entry| entry.try_into())
            .collect::<Result<Vec<DataEntry>>>()?;

        Ok(DataTransaction { data })
    }
}

impl TryFrom<&DataTransaction> for DataTransactionData {
    type Error = Error;

    fn try_from(value: &DataTransaction) -> Result<Self> {
        let mut proto_data_entries: Vec<ProtoDataEntry> = vec![];
        let data_entries = value.data();
        for data_entry in data_entries {
            let key = data_entry.key();
            match data_entry {
                DataEntry::IntegerEntry { key: _, value } => {
                    proto_data_entries.push(ProtoDataEntry {
                        key,
                        value: Some(IntValue(value)),
                    });
                }
                DataEntry::BooleanEntry { key: _, value } => {
                    proto_data_entries.push(ProtoDataEntry {
                        key,
                        value: Some(BoolValue(value)),
                    });
                }
                DataEntry::BinaryEntry { key: _, value } => {
                    proto_data_entries.push(ProtoDataEntry {
                        key,
                        value: Some(BinaryValue(value)),
                    })
                }
                DataEntry::StringEntry { key: _, value } => {
                    proto_data_entries.push(ProtoDataEntry {
                        key,
                        value: Some(StringValue(value)),
                    })
                }
                DataEntry::DeleteEntry { key: _ } => {
                    proto_data_entries.push(ProtoDataEntry { key, value: None });
                }
            };
        }
        Ok(DataTransactionData {
            data: proto_data_entries,
        })
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
