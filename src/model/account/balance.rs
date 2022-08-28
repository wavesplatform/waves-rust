use crate::model::account::Address;

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
