use crate::model::data_entry::DataEntry;
use serde_json::Value;

pub struct DataTransaction {
    data: Vec<DataEntry>,
}

impl DataTransaction {
    // todo return Result<DataTransaction, Error>
    pub fn from_json(value: Value) -> DataTransaction {
        let data = value["data"]
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| entry.into())
            .collect::<Vec<DataEntry>>();

        DataTransaction { data }
    }

    pub fn data(&self) -> Vec<DataEntry> {
        self.data.clone()
    }
}
