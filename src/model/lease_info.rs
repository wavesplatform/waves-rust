use crate::error::{Error, Result};
use crate::model::{Address, Id};
use crate::util::JsonDeserializer;
use serde_json::Value;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct LeaseInfo {
    id: Id,
    origin_transaction_id: Id,
    sender: Address,
    recipient: Address,
    amount: u64,
    height: u32,
    status: LeaseStatus,
    cancel_height: Option<u32>,
    cancel_transaction_id: Option<Id>,
}

#[allow(clippy::too_many_arguments)]
impl LeaseInfo {
    pub fn new(
        id: Id,
        origin_transaction_id: Id,
        sender: Address,
        recipient: Address,
        amount: u64,
        height: u32,
        status: LeaseStatus,
        cancel_height: Option<u32>,
        cancel_transaction_id: Option<Id>,
    ) -> LeaseInfo {
        LeaseInfo {
            id,
            origin_transaction_id,
            sender,
            recipient,
            amount,
            height,
            status,
            cancel_height,
            cancel_transaction_id,
        }
    }

    pub fn id(&self) -> Id {
        self.id.clone()
    }

    pub fn origin_transaction_id(&self) -> Id {
        self.origin_transaction_id.clone()
    }

    pub fn sender(&self) -> Address {
        self.sender.clone()
    }

    pub fn recipient(&self) -> Address {
        self.recipient.clone()
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn status(&self) -> LeaseStatus {
        self.status.clone()
    }

    pub fn cancel_height(&self) -> Option<u32> {
        self.cancel_height
    }

    pub fn cancel_transaction_id(&self) -> Option<Id> {
        self.cancel_transaction_id.clone()
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum LeaseStatus {
    Active,
    Canceled,
}

impl TryFrom<&Value> for LeaseInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let id = JsonDeserializer::safe_to_string_from_field(value, "id")?;
        let origin_tx_id =
            JsonDeserializer::safe_to_string_from_field(value, "originTransactionId")?;
        let sender = JsonDeserializer::safe_to_string_from_field(value, "sender")?;
        let recipient = JsonDeserializer::safe_to_string_from_field(value, "recipient")?;
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")?;
        let height = JsonDeserializer::safe_to_int_from_field(value, "height")?;
        let status = match JsonDeserializer::safe_to_string_from_field(value, "status")?.as_str() {
            "active" => LeaseStatus::Active,
            "canceled" => LeaseStatus::Canceled,
            _ => panic!("unknown lease type"),
        };
        let cancel_height = value["cancelHeight"].as_i64();
        let cancel_tx_id = match value["cancelTransactionId"].as_str() {
            Some(id) => Some(Id::from_string(id)?),
            None => None,
        };
        Ok(LeaseInfo {
            id: Id::from_string(&id)?,
            origin_transaction_id: Id::from_string(&origin_tx_id)?,
            sender: Address::from_string(&sender)?,
            recipient: Address::from_string(&recipient)?,
            amount: amount as u64,
            height: height as u32,
            status,
            cancel_height: cancel_height.map(|it| it as u32),
            cancel_transaction_id: cancel_tx_id,
        })
    }
}
