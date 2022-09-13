use serde_json::Value;

use crate::error::{Error, Result};
use crate::model::account::Address;
use crate::util::JsonDeserializer;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Balance {
    address: Address,
    balance: u64,
}

impl Balance {
    pub fn new(address: Address, balance: u64) -> Balance {
        Balance { address, balance }
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }
}

impl TryFrom<&Value> for Balance {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let address =
            Address::from_string(&JsonDeserializer::safe_to_string_from_field(value, "id")?)?;
        let balance = JsonDeserializer::safe_to_int_from_field(value, "balance")?;
        Ok(Balance::new(address, balance as u64))
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct BalanceDetails {
    address: Address,
    available: u64,
    regular: u64,
    generating: u64,
    effective: u64,
}

impl BalanceDetails {
    pub fn new(
        address: Address,
        available: u64,
        regular: u64,
        generating: u64,
        effective: u64,
    ) -> BalanceDetails {
        BalanceDetails {
            address,
            available,
            regular,
            generating,
            effective,
        }
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn available(&self) -> u64 {
        self.available
    }

    pub fn regular(&self) -> u64 {
        self.regular
    }

    pub fn generating(&self) -> u64 {
        self.generating
    }

    pub fn effective(&self) -> u64 {
        self.effective
    }
}

impl TryFrom<&Value> for BalanceDetails {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let address = Address::from_string(&JsonDeserializer::safe_to_string_from_field(
            value, "address",
        )?)?;
        let available = JsonDeserializer::safe_to_int_from_field(value, "available")? as u64;
        let regular = JsonDeserializer::safe_to_int_from_field(value, "regular")? as u64;
        let generating = JsonDeserializer::safe_to_int_from_field(value, "generating")? as u64;
        let effective = JsonDeserializer::safe_to_int_from_field(value, "effective")? as u64;
        Ok(BalanceDetails::new(
            address, available, regular, generating, effective,
        ))
    }
}

mod tests {
    use std::borrow::Borrow;
    use std::fs;

    use serde_json::{json, Value};

    use crate::error::Result;
    use crate::model::account::{Address, PrivateKey};
    use crate::model::{Balance, BalanceDetails, ByteString, ChainId};

    #[test]
    fn test_balance_details_from_json() -> Result<()> {
        let data = fs::read_to_string("./tests/resources/addresses/balance_details_rs.json")
            .expect("Unable to read file");
        let json: &Value = &serde_json::from_str(&data).expect("failed to generate json from str");
        let balance_details: BalanceDetails = json.try_into()?;
        assert_eq!(
            "3PAETTtuW7aSiyKtn9GuML3RgtV1xdq1mQW",
            balance_details.address().encoded()
        );
        assert_eq!(139400001, balance_details.regular());
        assert_eq!(139400002, balance_details.generating());
        assert_eq!(139400003, balance_details.available());
        assert_eq!(139400004, balance_details.effective());
        Ok(())
    }

    #[test]
    fn test_create_struct_balance_details() -> Result<()> {
        let balance_details: BalanceDetails = BalanceDetails::new(
            Address::from_string("3PAETTtuW7aSiyKtn9GuML3RgtV1xdq1mQW")?,
            139400003,
            139400001,
            139400002,
            139400004,
        );
        assert_eq!(
            "3PAETTtuW7aSiyKtn9GuML3RgtV1xdq1mQW",
            balance_details.address().encoded()
        );
        assert_eq!(139400001, balance_details.regular());
        assert_eq!(139400002, balance_details.generating());
        assert_eq!(139400003, balance_details.available());
        assert_eq!(139400004, balance_details.effective());
        Ok(())
    }

    #[test]
    fn test_balance_from_json() -> Result<()> {
        let json: &Value = &json!({
          "id": "3PAETTtuW7aSiyKtn9GuML3RgtV1xdq1mQW",
          "balance": 141700000
        });
        let balance: Balance = json.try_into()?;
        assert_eq!(
            "3PAETTtuW7aSiyKtn9GuML3RgtV1xdq1mQW",
            balance.address().encoded()
        );
        assert_eq!(141700000, balance.balance());
        Ok(())
    }

    #[test]
    fn test_create_struct_balance() -> Result<()> {
        let balance: Balance = Balance::new(
            Address::from_string("3PAETTtuW7aSiyKtn9GuML3RgtV1xdq1mQW")?,
            139400003,
        );
        assert_eq!(
            "3PAETTtuW7aSiyKtn9GuML3RgtV1xdq1mQW",
            balance.address().encoded()
        );
        assert_eq!(139400003, balance.balance());
        Ok(())
    }
}
