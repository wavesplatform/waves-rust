use crate::error::{Error, Result};
use crate::util::JsonDeserializer;
use serde_json::Value;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Validation {
    valid: bool,
    validation_time: u64,
    error: Option<String>,
}

impl Validation {
    pub fn new(valid: bool, validation_time: u64, error: Option<String>) -> Self {
        Self {
            valid,
            validation_time,
            error,
        }
    }

    pub fn valid(&self) -> bool {
        self.valid
    }

    pub fn validation_time(&self) -> u64 {
        self.validation_time
    }

    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

impl TryFrom<&Value> for Validation {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let valid = JsonDeserializer::safe_to_boolean_from_field(value, "valid")?;
        let validation_time = JsonDeserializer::safe_to_int_from_field(value, "validationTime")?;
        let error = value["error"].as_str().map(|it| it.to_owned());
        Ok(Validation {
            valid,
            validation_time: validation_time as u64,
            error,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::{
        Address, ByteString, GenesisTransaction, GenesisTransactionInfo, SignedTransaction,
        TransactionInfoResponse, Validation,
    };
    use crate::waves_proto::GenesisTransactionData;
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_validation() -> Result<()> {
        let data = fs::read_to_string("./tests/resources/validation_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let validation: Validation = json.borrow().try_into()?;

        assert_eq!(validation.valid(), true);
        assert_eq!(validation.validation_time(), 3);
        assert_eq!(validation.error().unwrap(), "some error");
        Ok(())
    }
}
