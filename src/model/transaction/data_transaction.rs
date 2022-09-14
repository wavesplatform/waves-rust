use crate::error::{Error, Result};
use crate::model::data_entry::DataEntry;
use crate::util::{Base64, JsonDeserializer};
use crate::waves_proto::data_transaction_data::data_entry::Value::{
    BinaryValue, BoolValue, IntValue, StringValue,
};
use crate::waves_proto::data_transaction_data::DataEntry as ProtoDataEntry;
use crate::waves_proto::DataTransactionData;
use serde_json::{Map, Number, Value};

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

impl TryFrom<&DataTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(value: &DataTransaction) -> Result<Self> {
        let mut map: Map<String, Value> = Map::new();
        let entries = value
            .data()
            .iter()
            .map(|entry| entry.into())
            .collect::<Vec<Value>>();
        map.insert("data".to_string(), Value::Array(entries));
        Ok(map)
    }
}

impl From<&DataEntry> for Value {
    fn from(data_entry: &DataEntry) -> Self {
        let mut map: Map<String, Value> = Map::new();
        map.insert("key".to_string(), data_entry.key().into());
        match data_entry {
            DataEntry::IntegerEntry { key: _, value } => {
                map.insert("type".to_string(), "integer".into());
                map.insert("value".to_string(), Value::Number(Number::from(*value)));
            }
            DataEntry::BooleanEntry { key: _, value } => {
                map.insert("type".to_string(), "boolean".into());
                map.insert("value".to_string(), Value::Bool(*value));
            }
            DataEntry::BinaryEntry { key: _, value } => {
                map.insert("type".to_string(), "binary".into());
                map.insert("value".to_string(), Base64::encode(&value, true).into());
            }
            DataEntry::StringEntry { key: _, value } => {
                map.insert("type".to_string(), "string".into());
                map.insert("value".to_string(), Value::String(value.clone()));
            }
            DataEntry::DeleteEntry { key: _ } => {
                map.insert("value".to_string(), Value::Null);
            }
        };
        map.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::data_entry::DataEntry;
    use crate::model::DataTransaction;
    use crate::waves_proto::data_transaction_data::data_entry::Value::{
        BinaryValue, BoolValue, IntValue, StringValue,
    };
    use crate::waves_proto::DataTransactionData;
    use serde_json::{json, Map, Value};

    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_data_transaction() -> Result<()> {
        let data = fs::read_to_string("./tests/resources/data_transaction_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let data_tx_from_json: DataTransaction = json.borrow().try_into()?;
        let data_entries = data_tx_from_json.data();

        for data_entry in data_entries {
            match data_entry {
                DataEntry::IntegerEntry { key, value } => {
                    assert_eq!("int", key);
                    assert_eq!(12, value);
                }
                DataEntry::BooleanEntry { key, value } => {
                    assert_eq!("bool", key);
                    assert_eq!(false, value);
                }
                DataEntry::BinaryEntry { key, value } => {
                    assert_eq!("binary", key);
                    assert_eq!([0_u8; 12].to_vec(), value);
                }
                DataEntry::StringEntry { key, value } => {
                    assert_eq!("str", key);
                    assert_eq!("value", value);
                }
                DataEntry::DeleteEntry { key } => {
                    assert_eq!("del_str", key)
                }
            }
        }

        Ok(())
    }

    #[test]
    fn test_data_transaction_to_proto() -> Result<()> {
        let expected_vec_data = vec![
            DataEntry::IntegerEntry {
                key: "int".to_string(),
                value: 12,
            },
            DataEntry::BooleanEntry {
                key: "bool".to_string(),
                value: false,
            },
            DataEntry::BinaryEntry {
                key: "binary".to_string(),
                value: [0; 12].to_vec(),
            },
            DataEntry::StringEntry {
                key: "str".to_string(),
                value: "value".to_string(),
            },
            DataEntry::DeleteEntry {
                key: "del_str".to_string(),
            },
        ];
        let data_transaction = &DataTransaction::new(expected_vec_data.clone());
        let proto: DataTransactionData = data_transaction.try_into()?;
        assert_eq!(expected_vec_data.len(), proto.data.len());

        for data_entry in expected_vec_data {
            match data_entry {
                DataEntry::IntegerEntry { key, value } => {
                    let int_entry = &proto.data[0];
                    assert_eq!(int_entry.key, key);
                    int_entry.value.clone().map(|it| match it {
                        IntValue(int_value) => {
                            assert_eq!(int_value, value)
                        }
                        _ => panic!("expected integer"),
                    });
                }
                DataEntry::BooleanEntry { key, value } => {
                    let bool_entry = &proto.data[1];
                    assert_eq!(bool_entry.key, key);
                    bool_entry.value.clone().map(|it| match it {
                        BoolValue(bool_value) => {
                            assert_eq!(bool_value, value)
                        }
                        _ => panic!("expected integer"),
                    });
                }
                DataEntry::BinaryEntry { key, value } => {
                    let binary_entry = &proto.data[2];
                    assert_eq!(binary_entry.key, key);
                    binary_entry.value.clone().map(|it| match it {
                        BinaryValue(binary_value) => {
                            assert_eq!(binary_value, value)
                        }
                        _ => panic!("expected integer"),
                    });
                }
                DataEntry::StringEntry { key, value } => {
                    let string_entry = &proto.data[3];
                    assert_eq!(string_entry.key, key);
                    string_entry.value.clone().map(|it| match it {
                        StringValue(string_value) => {
                            assert_eq!(string_value, value)
                        }
                        _ => panic!("expected integer"),
                    });
                }
                DataEntry::DeleteEntry { key } => {
                    let delete_entry = &proto.data[4];
                    assert_eq!(delete_entry.key, key);
                }
            }
        }

        Ok(())
    }

    #[test]
    fn test_data_transaction_to_json() -> Result<()> {
        let data_transaction = &DataTransaction::new(vec![
            DataEntry::IntegerEntry {
                key: "int".to_string(),
                value: 12,
            },
            DataEntry::BooleanEntry {
                key: "bool".to_string(),
                value: false,
            },
            DataEntry::BinaryEntry {
                key: "binary".to_string(),
                value: [0; 12].to_vec(),
            },
            DataEntry::StringEntry {
                key: "str".to_string(),
                value: "value".to_string(),
            },
            DataEntry::DeleteEntry {
                key: "del_str".to_string(),
            },
        ]);

        let map: Map<String, Value> = data_transaction.try_into()?;
        let json: Value = map.into();
        let expected_json = json!({"data": [
                {
                  "key": "int",
                  "type": "integer",
                  "value": 12
                },
                {
                  "key": "bool",
                  "type": "boolean",
                  "value": false
                },
                {
                  "key": "binary",
                  "type": "binary",
                  "value": "base64:AAAAAAAAAAAAAAAA"
                },
                {
                  "key": "str",
                  "type": "string",
                  "value": "value"
                },
                {
                  "key": "del_str",
                  "value": null
                }
        ]});
        assert_eq!(expected_json, json);
        Ok(())
    }
}
