use crate::error::{Error, Result};
use crate::model::account::Address;
use crate::util::JsonDeserializer;
use serde_json::Value;

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
