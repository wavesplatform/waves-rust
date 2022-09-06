use serde_json::Value;

use crate::error::{Error, Result};
use crate::model::{Address, Amount, AssetId};
use crate::util::JsonDeserializer;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ScriptTransfer {
    recipient: Address,
    amount: Amount,
}

impl ScriptTransfer {
    pub fn new(recipient: Address, amount: Amount) -> Self {
        Self { recipient, amount }
    }

    pub fn recipient(&self) -> Address {
        self.recipient.clone()
    }

    pub fn amount(&self) -> Amount {
        self.amount.clone()
    }
}

impl TryFrom<&Value> for ScriptTransfer {
    type Error = Error;

    fn try_from(value: &Value) -> Result<ScriptTransfer> {
        let address = JsonDeserializer::safe_to_string_from_field(value, "address")?;
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")?;
        let asset_id = match value["asset"].as_str() {
            Some(asset) => Some(AssetId::from_string(asset)?),
            None => None,
        };

        Ok(ScriptTransfer {
            recipient: Address::from_string(&address)?,
            amount: Amount::new(amount as u64, asset_id),
        })
    }
}
