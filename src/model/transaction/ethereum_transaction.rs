use crate::error::Error::UnsupportedOperation;
use crate::error::{Error, Result};
use crate::model::{Address, Amount, AssetId, ByteString, Function, StateChanges};
use crate::util::JsonDeserializer;
use serde_json::Value;
use std::borrow::Borrow;
use std::fmt;

const TYPE: u8 = 18;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct EthereumTransactionInfo {
    bytes: HexString,
    payload: Payload,
}

impl EthereumTransactionInfo {
    pub fn new(bytes: HexString, payload: Payload) -> Self {
        Self { bytes, payload }
    }

    pub fn bytes(&self) -> HexString {
        self.bytes.clone()
    }

    pub fn payload(&self) -> Payload {
        self.payload.clone()
    }

    pub fn tx_type() -> u8 {
        TYPE
    }
}

impl TryFrom<&Value> for EthereumTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let payload = value["payload"].borrow().try_into()?;
        let bytes = HexString::new(hex::decode(
            &JsonDeserializer::safe_to_string_from_field(value, "bytes")?[2..],
        )?);
        Ok(EthereumTransactionInfo { bytes, payload })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct EthereumTransaction {
    bytes: HexString,
}

impl EthereumTransaction {
    pub fn new(bytes: HexString) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> HexString {
        self.bytes.clone()
    }

    pub fn tx_type() -> u8 {
        TYPE
    }
}

impl TryFrom<&Value> for EthereumTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let bytes = HexString::new(hex::decode(
            &JsonDeserializer::safe_to_string_from_field(value, "bytes")?[2..],
        )?);
        Ok(EthereumTransaction { bytes })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
//todo fix it
#[allow(clippy::large_enum_variant)]
pub enum Payload {
    Invoke(InvokePayload),
    Transfer(TransferPayload),
}

impl TryFrom<&Value> for Payload {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        match JsonDeserializer::safe_to_string_from_field(value, "type")?.as_str() {
            "invocation" => {
                let invoke: InvokePayload = value.try_into()?;
                Ok(Payload::Invoke(invoke))
            }
            "transfer" => {
                let transfer: TransferPayload = value.try_into()?;
                Ok(Payload::Transfer(transfer))
            }
            _ => Err(UnsupportedOperation("unknown payload type".to_owned())),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TransferPayload {
    recipient: Address,
    amount: Amount,
}

impl TransferPayload {
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

impl TryFrom<&Value> for TransferPayload {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let recipient = JsonDeserializer::safe_to_string_from_field(value, "recipient")?;
        let asset = match value["asset"].as_str() {
            Some(asset_id) => Some(AssetId::from_string(asset_id)?),
            None => None,
        };
        let amount = JsonDeserializer::safe_to_int_from_field(value, "amount")?;

        Ok(TransferPayload {
            recipient: Address::from_string(&recipient)?,
            amount: Amount::new(amount as u64, asset),
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct InvokePayload {
    dapp: Address,
    function: Function,
    payments: Vec<Amount>,
    state_changes: StateChanges,
}

impl InvokePayload {
    pub fn new(
        dapp: Address,
        function: Function,
        payments: Vec<Amount>,
        state_changes: StateChanges,
    ) -> Self {
        Self {
            dapp,
            function,
            payments,
            state_changes,
        }
    }

    pub fn dapp(&self) -> Address {
        self.dapp.clone()
    }

    pub fn function(&self) -> Function {
        self.function.clone()
    }

    pub fn payments(&self) -> Vec<Amount> {
        self.payments.clone()
    }

    pub fn state_changes(&self) -> StateChanges {
        self.state_changes.clone()
    }
}

impl TryFrom<&Value> for InvokePayload {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let dapp = JsonDeserializer::safe_to_string_from_field(value, "dApp")?;
        let function: Function = value.try_into()?;
        let payments = map_payment(value)?;
        let state_changes: StateChanges = value["stateChanges"].borrow().try_into()?;

        Ok(InvokePayload {
            dapp: Address::from_string(&dapp)?,
            function,
            payments,
            state_changes,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct HexString {
    bytes: Vec<u8>,
}

impl fmt::Debug for HexString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HexString {{ {} }}", self.encoded())
    }
}

impl HexString {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn encoded(&self) -> String {
        hex::encode(self.bytes.clone())
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}

impl ByteString for HexString {
    fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn encoded(&self) -> String {
        hex::encode(self.bytes.clone())
    }

    fn encoded_with_prefix(&self) -> String {
        format!("0x{}", self.encoded())
    }
}

//todo rm duplicate
fn map_payment(value: &Value) -> Result<Vec<Amount>> {
    JsonDeserializer::safe_to_array_from_field(value, "payment")?
        .iter()
        .map(|payment| {
            let value = JsonDeserializer::safe_to_int_from_field(payment, "amount")?;
            let asset_id = match payment["assetId"].as_str() {
                Some(asset) => Some(AssetId::from_string(asset)?),
                None => None,
            };
            Ok(Amount::new(value as u64, asset_id))
        })
        .collect::<Result<Vec<Amount>>>()
}

#[cfg(test)]
mod tests {
    use crate::model::data_entry::DataEntry;
    use crate::model::{Arg, ByteString, EthereumTransactionInfo, Payload};
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    const INVOKE_BYTES: &str = "0xf9011186017cac99be168502540be4008307a120940ea8e14f313237aac31995f9c19a7e0f78c1cc2b80b8a409abf90e00000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000e74657374206d6574616d61736b32000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000081c9a0dcc682194d46cd3a763b352ca77a4317e9d89f10e5213379b55563cbc03619f3a02a2f26c580ab9f3d83db801bf7d556dd50d37cd69b19df8ee4a3488a6c5140c8";

    #[test]
    fn test_json_to_eth_invoke_transaction() {
        let data = fs::read_to_string("./tests/resources/ethereum_transaction_invoke_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let eth_invoke_from_json: EthereumTransactionInfo = json.borrow().try_into().unwrap();

        assert_eq!(
            INVOKE_BYTES[2..],
            hex::encode(eth_invoke_from_json.bytes().bytes())
        );

        let invoke = match eth_invoke_from_json.payload() {
            Payload::Invoke(invoke) => invoke,
            Payload::Transfer(_) => panic!("expected invoke but was transfer"),
        };

        assert_eq!(
            "3MRuzZVauiiX2DGwNyP8Tv7idDGUy1VG5bJ",
            invoke.dapp().encoded()
        );
        assert_eq!("saveString", invoke.function().name());

        match &invoke.function().args()[0] {
            Arg::String(value) => {
                assert_eq!("test metamask2", value)
            }
            _ => panic!("expected string arg"),
        }

        assert_eq!(1, invoke.payments().len());
        let payment = &invoke.payments()[0];
        assert_eq!(26434954086, payment.value());
        assert_eq!(
            "97zHFp1C3cB7qfvx8Xv5f2rWp9nUSG5UnAamfPcW6txf",
            payment.asset_id().expect("should not be empty").encoded()
        );

        match &invoke.state_changes().data()[0] {
            DataEntry::StringEntry { key, value } => {
                assert_eq!("str_1043725", key);
                assert_eq!("test metamask2", value);
            }
            _ => panic!("expected string entry"),
        }
    }

    #[test]
    fn test_json_to_eth_transfer_transaction() {
        let data = fs::read_to_string("./tests/resources/ethereum_transaction_transfer_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let eth_invoke_from_json: EthereumTransactionInfo = json.borrow().try_into().unwrap();

        let transfer = match eth_invoke_from_json.payload() {
            Payload::Transfer(transfer) => transfer,
            Payload::Invoke(_) => panic!("expected transfer but was invoke"),
        };

        assert_eq!(
            "3MVeY7NhZciZLsnwb4E47moXVd9y4gKw8S7",
            transfer.recipient().encoded()
        );
        assert_eq!(10000000, transfer.amount().value());
        assert_eq!(None, transfer.amount().asset_id());
    }
}
