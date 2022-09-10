use crate::error::{Error, Result};
use crate::model::Address;
use crate::util::JsonDeserializer;
use serde_json::Value;

pub struct GenesisTransactionInfo {
    recipient: Address,
    amount: u64,
}

impl TryFrom<&Value> for GenesisTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")?;
        let recipient = JsonDeserializer::safe_to_string_from_field(value, "recipient")?;
        Ok(GenesisTransactionInfo {
            recipient: Address::from_string(&recipient)?,
            amount: amount as u64,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::GenesisTransactionInfo;
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_burn_transaction() {
        let data =
            fs::read_to_string("./tests/resources/genesis_rs.json").expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let genesis_from_json: GenesisTransactionInfo = json.borrow().try_into().unwrap();

        assert_eq!("400000000000000", genesis_from_json.amount)
    }
}
