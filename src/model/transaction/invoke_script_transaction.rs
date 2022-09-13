use crate::error::{Error, Result};
use crate::model::{Address, Amount, AssetId, Base64String, StateChanges};
use crate::util::{ByteWriter, JsonDeserializer};
use crate::waves_proto::InvokeScriptTransactionData;
use crate::waves_proto::{recipient, Amount as ProtoAmount, Recipient};
use serde_json::Value;
use std::borrow::Borrow;

const TYPE: u8 = 16;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct InvokeScriptTransactionInfo {
    dapp: Address,
    function: Function,
    payment: Vec<Amount>,
    state_changes: StateChanges,
}

impl InvokeScriptTransactionInfo {
    pub fn new(
        dapp: Address,
        function: Function,
        payment: Vec<Amount>,
        state_changes: StateChanges,
    ) -> InvokeScriptTransactionInfo {
        InvokeScriptTransactionInfo {
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

impl TryFrom<&Value> for InvokeScriptTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let dapp = JsonDeserializer::safe_to_string_from_field(value, "dApp")?;
        let function: Function = value.try_into()?;
        let payment = map_payment(value)?;
        let state_changes = value["stateChanges"].borrow().try_into()?;

        Ok(InvokeScriptTransactionInfo {
            dapp: Address::from_string(&dapp)?,
            function,
            payment,
            state_changes,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct InvokeScriptTransaction {
    dapp: Address,
    function: Function,
    payment: Vec<Amount>,
}

impl TryFrom<&InvokeScriptTransaction> for InvokeScriptTransactionData {
    type Error = Error;

    fn try_from(invoke_tx: &InvokeScriptTransaction) -> Result<Self> {
        let dapp = Some(Recipient {
            recipient: Some(recipient::Recipient::PublicKeyHash(
                invoke_tx.dapp().public_key_hash(),
            )),
        });
        let payments: Vec<ProtoAmount> = invoke_tx
            .payment()
            .iter()
            .map(|amount| {
                let asset_id = match amount.asset_id() {
                    Some(asset) => asset.bytes(),
                    None => vec![],
                };
                ProtoAmount {
                    asset_id,
                    amount: amount.value() as i64,
                }
            })
            .collect();
        Ok(InvokeScriptTransactionData {
            d_app: dapp,
            function_call: ByteWriter::bytes_from_function(&invoke_tx.function()),
            payments,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Function {
    name: String,
    args: Vec<Arg>,
}

impl Function {
    pub fn new(name: String, args: Vec<Arg>) -> Self {
        Function { name, args }
    }

    pub fn args(&self) -> Vec<Arg> {
        self.args.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn is_default(&self) -> bool {
        self.name == "default" && self.args.is_empty()
    }
}

impl TryFrom<&Value> for Function {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let call = JsonDeserializer::safe_to_map_from_field(value, "call")?;
        let function_name = match call.get("function") {
            Some(func_name) => JsonDeserializer::safe_to_string(func_name)?,
            None => "".to_owned(),
        };
        let args = match call.get("args") {
            Some(args) => map_args(args)?,
            None => vec![],
        };

        Ok(Function {
            name: function_name,
            args,
        })
    }
}

impl InvokeScriptTransaction {
    pub fn from_json(value: &Value) -> Result<InvokeScriptTransaction> {
        let dapp =
            Address::from_string(&JsonDeserializer::safe_to_string_from_field(value, "dApp")?)?;
        let function: Function = value.try_into()?;
        let payments = map_payment(value)?;

        Ok(InvokeScriptTransaction {
            dapp,
            function,
            payment: payments,
        })
    }

    pub fn new(dapp: Address, function: Function, payment: Vec<Amount>) -> Self {
        InvokeScriptTransaction {
            dapp,
            function,
            payment,
        }
    }

    pub fn tx_type() -> u8 {
        TYPE
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
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Arg {
    Binary(Base64String),
    Boolean(bool),
    Integer(i64),
    String(String),
    List(Vec<Arg>),
}

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

fn map_args(value: &Value) -> Result<Vec<Arg>> {
    let mut args: Vec<Arg> = vec![];
    for arg in JsonDeserializer::safe_to_array(value)? {
        let arg = match JsonDeserializer::safe_to_string_from_field(&arg, "type")?.as_str() {
            "boolean" | "Boolean" => {
                Arg::Boolean(JsonDeserializer::safe_to_boolean_from_field(&arg, "value")?)
            }
            "string" | "String" => {
                Arg::String(JsonDeserializer::safe_to_string_from_field(&arg, "value")?)
            }
            "integer" | "Int" => {
                Arg::Integer(JsonDeserializer::safe_to_int_from_field(&arg, "value")?)
            }
            "binary" | "ByteVector" => Arg::Binary(Base64String::from_string(
                &JsonDeserializer::safe_to_string_from_field(&arg, "value")?,
            )?),
            "list" | "List" => {
                let result = map_args(&arg["value"])?;
                Arg::List(result)
            }
            _ => panic!("unknown type"),
        };
        args.push(arg);
    }
    Ok(args)
}

#[cfg(test)]
mod tests {
    use crate::model::data_entry::DataEntry;
    use crate::model::{Arg, Base64String, ByteString, InvokeScriptTransactionInfo, LeaseStatus};
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_invoke_script_transaction() {
        let data = fs::read_to_string("./tests/resources/invoke_script_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let invoke_script_from_json: InvokeScriptTransactionInfo =
            json.borrow().try_into().unwrap();
        let function = invoke_script_from_json.function();
        assert_eq!("checkPointAndPoligon", function.name());
        assert_eq!(
            "3MQ833eGnNM5dtRWGBaKFpmRfxfrnmeKd9G",
            match &function.args()[0] {
                Arg::String(value) => value,
                _ => panic!("wrong type"),
            }
        );

        let state_changes = invoke_script_from_json.state_changes();
        let data_entries = state_changes.data();
        assert_eq!(5, data_entries.len());
        for data_entry in data_entries {
            match data_entry {
                DataEntry::IntegerEntry { key, value } => {
                    assert_eq!("int", key);
                    assert_eq!(2514, value);
                }
                DataEntry::BooleanEntry { key, value } => {
                    assert_eq!("bool", key);
                    assert_eq!(true, value)
                }
                DataEntry::BinaryEntry { key, value } => {
                    assert_eq!("bin", key);
                    assert_eq!("mmXJ", Base64String::from_bytes(value).encoded());
                }
                DataEntry::StringEntry { key, value } => {
                    assert_eq!("str", key);
                    assert_eq!("", value)
                }
                DataEntry::DeleteEntry { key } => {
                    assert_eq!("str", key)
                }
            }
        }

        let transfers = state_changes.transfers();
        assert_eq!(1, transfers.len());
        let transfer = &transfers[0];
        assert_eq!(
            "3MQ833eGnNM5dtRWGBaKFpmRfxfrnmeKd9G",
            transfer.recipient().encoded()
        );
        assert_eq!(
            "AuEwc87bodoeofX5pdbt9ebU7K5zrz85frwDwoFeuQoa",
            transfer.amount().asset_id().expect("failed").encoded()
        );
        assert_eq!(1, transfer.amount().value());

        let issues = state_changes.issues();
        assert_eq!(1, issues.len());
        let issue = &issues[0];
        assert_eq!(
            "AuEwc87bodoeofX5pdbt9ebU7K5zrz85frwDwoFeuQoa",
            issue.asset_id().encoded()
        );
        assert_eq!("Asset", issue.name());
        assert_eq!("", issue.description());
        assert_eq!(1, issue.quantity());
        assert_eq!(0, issue.decimals());
        assert_eq!(true, issue.is_reissuable());
        assert_eq!("", issue.script().encoded());
        assert_eq!(0, issue.nonce());

        let reissues = state_changes.reissues();
        assert_eq!(1, reissues.len());
        let reissue = &reissues[0];
        assert_eq!(
            "AuEwc87bodoeofX5pdbt9ebU7K5zrz85frwDwoFeuQoa",
            reissue.asset_id().encoded()
        );
        assert_eq!(false, reissue.is_reissuable());
        assert_eq!(1, reissue.quantity());

        let burns = state_changes.burns();
        assert_eq!(1, burns.len());
        let burn = &burns[0];
        assert_eq!(
            "AuEwc87bodoeofX5pdbt9ebU7K5zrz85frwDwoFeuQoa",
            burn.asset_id().encoded()
        );
        assert_eq!(1, burn.amount());

        let sponsor_fees = state_changes.sponsor_fees();
        assert_eq!(1, sponsor_fees.len());
        let sponsor_fee = &sponsor_fees[0];
        assert_eq!(
            "GyH2wqKQcjHtz6KgkUNzUpDYYy1azqZdYHZ2awXHWqYx",
            sponsor_fee.asset_id().encoded()
        );
        assert_eq!(1, sponsor_fee.min_sponsored_asset_fee());

        let leases = state_changes.leases();
        assert_eq!(1, leases.len());
        let lease_info = &leases[0];
        assert_eq!(
            "9zzpWBv63hh91FDdBnaeTDRVhgvqE4vdnwtYkGU9SvNb",
            lease_info.id().encoded()
        );
        assert_eq!(
            "4XFVLLMBjBMPwGivgyLhw374kViANoToLAYUdEXWLsBJ",
            lease_info.origin_transaction_id().encoded()
        );
        assert_eq!(
            "3MwjNKQ9aAoAdBKGAR9cmsq8sRicQVitGVz",
            lease_info.sender().encoded()
        );
        assert_eq!(
            "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
            lease_info.recipient().encoded()
        );
        assert_eq!(7, lease_info.amount());
        assert_eq!(2217333, lease_info.height());
        assert_eq!(LeaseStatus::Canceled, lease_info.status());
        assert_eq!(Some(2217333), lease_info.cancel_height());
        assert_eq!(
            Some("4XFVLLMBjBMPwGivgyLhw374kViANoToLAYUdEXWLsBJ".to_owned()),
            lease_info.cancel_transaction_id().map(|it| it.encoded())
        );

        let lease_cancels = state_changes.lease_cancels();
        assert_eq!(1, lease_cancels.len());
        let lease_cancel_info = &lease_cancels[0];
        assert_eq!(
            "9zzpWBv63hh91FDdBnaeTDRVhgvqE4vdnwtYkGU9SvNb",
            lease_cancel_info.id().encoded()
        );
        assert_eq!(
            "4XFVLLMBjBMPwGivgyLhw374kViANoToLAYUdEXWLsBJ",
            lease_cancel_info.origin_transaction_id().encoded()
        );
        assert_eq!(
            "3MwjNKQ9aAoAdBKGAR9cmsq8sRicQVitGVz",
            lease_cancel_info.sender().encoded()
        );
        assert_eq!(
            "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
            lease_cancel_info.recipient().encoded()
        );
        assert_eq!(7, lease_cancel_info.amount());
        assert_eq!(2217333, lease_cancel_info.height());
        assert_eq!(LeaseStatus::Canceled, lease_cancel_info.status());
        assert_eq!(Some(2217333), lease_cancel_info.cancel_height());
        assert_eq!(
            Some("4XFVLLMBjBMPwGivgyLhw374kViANoToLAYUdEXWLsBJ".to_owned()),
            lease_cancel_info
                .cancel_transaction_id()
                .map(|it| it.encoded())
        );

        let invokes = state_changes.invokes();
        assert_eq!(2, invokes.len());
        let first_invoke = &invokes[0];
        assert_eq!(
            "3MFTz4aKdjAMcvFUYFdDv7jPiKtpeUv9r3K",
            first_invoke.dapp().encoded()
        );
        assert_eq!("selfCall", first_invoke.function().name());
        assert_eq!(1, first_invoke.function().args().len());
        let inner_invoke = &first_invoke.state_changes().invokes()[0];
        assert_eq!(
            "3MFTz4aKdjAMcvFUYFdDv7jPiKtpeUv9r3K",
            inner_invoke.dapp().encoded()
        );
        assert_eq!("selfCall2", inner_invoke.function().name());
        assert_eq!(1, inner_invoke.function().args().len());
        let second_invoke = &invokes[1];
        assert_eq!(
            "3MFTz4aKdjAMcvFUYFdDv7jPiKtpeUv9r3K",
            second_invoke.dapp().encoded()
        );
        assert_eq!("selfCall1", second_invoke.function().name());
        assert_eq!(1, second_invoke.function().args().len());
    }
}
