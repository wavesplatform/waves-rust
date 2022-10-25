use crate::error::Result;
use crate::util::BinarySerializer;
use crate::{
    model::{Amount, Id, PrivateKey, Proof, PublicKey},
    util::{sign_order, Hash},
};

use super::{Order, OrderType, SignedOrder};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct OrderInfoV3 {
    id: Id,
    chain_id: u8,
    timestamp: u64,
    sender: PublicKey,
    fee: Amount,
    order_type: OrderType,
    amount: Amount,
    price: Amount,
    matcher: PublicKey,
    expiration: u64,
    proofs: Vec<Proof>,
}

impl OrderInfoV3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Id,
        chain_id: u8,
        timestamp: u64,
        sender: PublicKey,
        fee: Amount,
        order_type: OrderType,
        amount: Amount,
        price: Amount,
        matcher: PublicKey,
        expiration: u64,
        proofs: Vec<Proof>,
    ) -> Self {
        Self {
            id,
            chain_id,
            timestamp,
            sender,
            fee,
            order_type,
            amount,
            price,
            matcher,
            expiration,
            proofs,
        }
    }

    pub fn chain_id(&self) -> u8 {
        self.chain_id
    }

    pub fn version(&self) -> u8 {
        3
    }

    pub fn sender(&self) -> PublicKey {
        self.sender.clone()
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn fee(&self) -> Amount {
        self.fee.clone()
    }

    pub fn order_type(&self) -> OrderType {
        self.order_type.clone()
    }

    pub fn amount(&self) -> Amount {
        self.amount.clone()
    }

    pub fn price(&self) -> Amount {
        self.price.clone()
    }

    pub fn matcher(&self) -> PublicKey {
        self.matcher.clone()
    }

    pub fn expiration(&self) -> u64 {
        self.expiration
    }

    pub fn id(&self) -> Id {
        self.id.clone()
    }

    pub fn proofs(&self) -> Vec<Proof> {
        self.proofs.clone()
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct OrderV3 {
    chain_id: u8,
    timestamp: u64,
    sender: PublicKey,
    fee: Amount,
    order_type: OrderType,
    amount: Amount,
    price: Amount,
    matcher: PublicKey,
    expiration: u64,
}

#[allow(clippy::too_many_arguments)]
impl OrderV3 {
    pub fn new(
        chain_id: u8,
        timestamp: u64,
        sender: PublicKey,
        fee: Amount,
        order_type: OrderType,
        amount: Amount,
        price: Amount,
        matcher: PublicKey,
        expiration: u64,
    ) -> Self {
        Self {
            chain_id,
            timestamp,
            sender,
            fee,
            order_type,
            amount,
            price,
            matcher,
            expiration,
        }
    }

    pub fn chain_id(&self) -> u8 {
        self.chain_id
    }

    pub fn version(&self) -> u8 {
        3
    }

    pub fn sender(&self) -> PublicKey {
        self.sender.clone()
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn fee(&self) -> Amount {
        self.fee.clone()
    }

    pub fn order_type(&self) -> OrderType {
        self.order_type.clone()
    }

    pub fn amount(&self) -> Amount {
        self.amount.clone()
    }

    pub fn price(&self) -> Amount {
        self.price.clone()
    }

    pub fn matcher(&self) -> PublicKey {
        self.matcher.clone()
    }

    pub fn expiration(&self) -> u64 {
        self.expiration
    }

    pub fn id(&self) -> Result<Id> {
        Ok(Id::from_bytes(&Hash::blake(&self.bytes()?)?))
    }

    pub fn bytes(&self) -> Result<Vec<u8>> {
        BinarySerializer::order_body_bytes(&Order::V3(self.to_owned()))
    }

    pub fn sign(&self, private_key: &PrivateKey) -> Result<SignedOrder> {
        sign_order(&Order::V3(self.clone()), private_key)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Amount, AssetId, ByteString, PublicKey};

    use super::OrderV3;

    #[test]
    fn test_order_id_calculation() {
        let order = OrderV3 {
            chain_id: 84,
            timestamp: 1664244861345,
            sender: PublicKey::from_string("FarW7tFmnVJBsHUdDe9DMJcfUESh266UDmEm1vP6P2xE")
                .expect("failed to parse sender public key"),
            fee: Amount::new(10000000, None),
            order_type: super::OrderType::Sell,
            amount: Amount::new(10000000000, None),
            price: Amount::new(
                15000000,
                Some(
                    AssetId::from_string("25FEqEjRkqK6yCkiT7Lz6SAYz7gUFCtxfCChnrVFD5AT")
                        .expect("failed to parse asset id"),
                ),
            ),
            matcher: PublicKey::from_string("8QUAqtTckM5B8gvcuP7mMswat9SjKUuafJMusEoSn1Gy")
                .expect("failed to parse matcher public key"),
            expiration: 1666750461345,
        };

        assert_eq!(
            "H2EaCndcFAETGaWkPifGdNBL3scaZ53Pgm4Ha4xvg9wb",
            order.id().expect("failed to calculate order id").encoded()
        );
    }
}
