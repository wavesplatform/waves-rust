use crate::error::{Error, Result};
use crate::model::{Address, Amount, AssetId, Function, StateChanges};
use crate::util::JsonDeserializer;
use serde_json::Value;
use std::borrow::Borrow;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct InvokeAction {
    dapp: Address,
    function: Function,
    payment: Vec<Amount>,
    state_changes: StateChanges,
}

impl InvokeAction {
    pub fn new(
        dapp: Address,
        function: Function,
        payment: Vec<Amount>,
        state_changes: StateChanges,
    ) -> InvokeAction {
        InvokeAction {
            dapp,
            function,
            payment,
            state_changes,
        }
    }

    pub fn dapp(&self) -> Address {
        self.dapp.clone()
    }

    pub fn function(&self) -> Function {
        self.function.clone()
    }

    pub fn payment(&self) -> Vec<Amount> {
        self.payment.clone()
    }

    pub fn state_changes(&self) -> StateChanges {
        self.state_changes.clone()
    }
}

impl TryFrom<&Value> for InvokeAction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let dapp = JsonDeserializer::safe_to_string_from_field(value, "dApp")?;
        let function: Function = value.try_into()?;
        let payment: Vec<Amount> = map_payment(value)?;
        let state_changes: StateChanges = value["stateChanges"].borrow().try_into()?;
        Ok(InvokeAction {
            dapp: Address::from_string(&dapp)?,
            function,
            payment,
            state_changes,
        })
    }
}

////     "invokes": [
// //       {
// //         "dApp": "3MFTz4aKdjAMcvFUYFdDv7jPiKtpeUv9r3K",
// //         "call": {
// //           "function": "selfCall",
// //           "args": [
// //             {
// //               "type": "Int",
// //               "value": 1
// //             }
// //           ]
// //         },
// //         "payment": [],
// //         "stateChanges": {
// //           "data": [],
// //           "transfers": [],
// //           "issues": [],
// //           "reissues": [],
// //           "burns": [],
// //           "sponsorFees": [],
// //           "leases": [],
// //           "leaseCancels": [],
// //           "invokes": []
// //         }
// //       }
// //     ]

//todo rm copy past
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
