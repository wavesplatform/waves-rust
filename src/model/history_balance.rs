use crate::error::{Error, Result};
use crate::util::JsonDeserializer;
use serde_json::Value;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct HistoryBalance {
    height: u32,
    balance: u64,
}

impl HistoryBalance {
    pub fn new(height: u32, balance: u64) -> Self {
        Self { height, balance }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }
}

impl TryFrom<&Value> for HistoryBalance {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let height = JsonDeserializer::safe_to_int_from_field(value, "height")?;
        let balance = JsonDeserializer::safe_to_int_from_field(value, "balance")?;
        Ok(HistoryBalance {
            height: height as u32,
            balance: balance as u64,
        })
    }
}
