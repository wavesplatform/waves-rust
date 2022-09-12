use crate::error::{Error, Result};
use crate::model::Address;
use crate::util::JsonDeserializer;
use crate::waves_proto::PaymentTransactionData;
use serde_json::Value;

const TYPE: u8 = 2;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PaymentTransactionInfo {
    recipient: Address,
    amount: u64,
}

impl PaymentTransactionInfo {
    pub fn new(recipient: Address, amount: u64) -> Self {
        Self { recipient, amount }
    }

    pub fn recipient(&self) -> Address {
        self.recipient.clone()
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PaymentTransaction {
    recipient: Address,
    amount: u64,
}

impl PaymentTransaction {
    pub fn new(recipient: Address, amount: u64) -> Self {
        Self { recipient, amount }
    }

    pub fn recipient(&self) -> Address {
        self.recipient.clone()
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn tx_type() -> u8 {
        TYPE
    }
}

impl TryFrom<&PaymentTransaction> for PaymentTransactionData {
    type Error = Error;

    fn try_from(value: &PaymentTransaction) -> Result<Self> {
        Ok(PaymentTransactionData {
            recipient_address: value.recipient().bytes(),
            amount: value.amount() as i64,
        })
    }
}

impl TryFrom<&Value> for PaymentTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")?;
        let recipient = JsonDeserializer::safe_to_string_from_field(value, "recipient")?;
        Ok(PaymentTransactionInfo {
            recipient: Address::from_string(&recipient)?,
            amount: amount as u64,
        })
    }
}

impl TryFrom<&Value> for PaymentTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")?;
        let recipient = JsonDeserializer::safe_to_string_from_field(value, "recipient")?;
        Ok(PaymentTransaction {
            recipient: Address::from_string(&recipient)?,
            amount: amount as u64,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::{PaymentTransaction, SignedTransaction, TransactionInfoResponse};
    use serde_json::Value;
    use std::fs;

    #[test]
    fn test_json_to_genesis_transaction() -> Result<()> {
        let data = fs::read_to_string("./tests/resources/payment_transaction_rs.json")
            .expect("Unable to read file");
        let json: &Value = &serde_json::from_str(&data).expect("failed to generate json from str");

        let payment_tx: SignedTransaction = json.try_into()?;

        let payment_tx_info: TransactionInfoResponse = json.try_into()?;

        assert_eq!(payment_tx.id()?, payment_tx_info.id());

        let payment_from_json: PaymentTransaction = json.try_into().unwrap();

        assert_eq!(910924657498, payment_from_json.amount());
        assert_eq!(
            "3PP4hNGAJaMqmx9vpdYUHk8owF3mwbUevoz",
            payment_from_json.recipient().encoded()
        );
        Ok(())
    }
}
