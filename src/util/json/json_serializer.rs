use serde_json::{Map, Value};

use crate::error::Result;
use crate::model::{
    Arg, ByteString, DataTransaction, InvokeScriptTransaction, IssueTransaction, SignedTransaction,
    Transaction, TransactionData, TransferTransaction,
};
use crate::util::Base58;

pub struct JsonSerializer;

impl JsonSerializer {
    pub fn serialize_signed_tx(sign_tx: &SignedTransaction) -> Result<Value> {
        let mut json_props: Map<String, Value> = Map::new();
        let mut json_props_with_default_values = add_default_fields(sign_tx, &mut json_props)?;
        let json_props_with_additional_fields =
            add_additional_fields(sign_tx.tx().data(), &mut json_props_with_default_values)?;
        Ok(json_props_with_additional_fields.into())
    }
}

fn add_default_fields(
    sign_tx: &SignedTransaction,
    json_props: &mut Map<String, Value>,
) -> Result<Map<String, Value>> {
    let tx = sign_tx.tx();
    json_props.insert("type".to_string(), tx_type(tx).into());
    json_props.insert("version".to_string(), tx.version().into());
    json_props.insert("chainId".to_string(), tx.chain_id().into());
    json_props.insert(
        "senderPublicKey".to_string(),
        tx.public_key().encoded().into(),
    );
    json_props.insert(
        "sender".to_string(),
        tx.public_key()
            .address(sign_tx.tx().chain_id())?
            .encoded()
            .into(),
    );
    json_props.insert("fee".to_string(), tx.fee().value().into());
    json_props.insert("feeAssetId".to_string(), tx.fee().asset_id().into());
    json_props.insert("timestamp".to_string(), tx.timestamp().into());
    json_props.insert("proofs".to_string(), proofs(sign_tx).into());
    Ok(json_props.clone())
}

fn add_additional_fields(
    tx_data: &TransactionData,
    json_props: &mut Map<String, Value>,
) -> Result<Map<String, Value>> {
    match tx_data {
        TransactionData::Transfer(transfer_tx) => {
            json_props.insert(
                "recipient".to_owned(),
                transfer_tx.recipient().encoded().into(),
            );
            json_props.insert("amount".to_owned(), transfer_tx.amount().value().into());
            json_props.insert("assetId".to_owned(), transfer_tx.amount().asset_id().into());
            json_props.insert(
                "attachment".to_owned(),
                transfer_tx.attachment().encoded().into(),
            );
        }
        TransactionData::Data(data_tx) => {
            json_props.insert("data".to_string(), data_tx.data().into());
        }
        TransactionData::Issue(issue_tx) => {
            json_props.insert("name".to_string(), issue_tx.name().into());
            json_props.insert("description".to_string(), issue_tx.description().into());
            json_props.insert("quantity".to_string(), issue_tx.quantity().into());
            json_props.insert("decimals".to_string(), issue_tx.decimals().into());
            json_props.insert("reissuable".to_string(), issue_tx.is_reissuable().into());
            json_props.insert(
                "script".to_string(),
                issue_tx.script().map(|it| it.encoded()).into(),
            );
        }
        TransactionData::InvokeScript(invoke_tx) => invoke_to_json(invoke_tx, json_props),
    };
    Ok(json_props.clone())
}

fn tx_type(tx: &Transaction) -> u8 {
    match tx.data() {
        TransactionData::Transfer(_) => TransferTransaction::tx_type(),
        TransactionData::Data(_) => DataTransaction::tx_type(),
        TransactionData::Issue(_) => IssueTransaction::tx_type(),
        TransactionData::InvokeScript(_) => InvokeScriptTransaction::tx_type(),
    }
}

fn proofs(sign_tx: &SignedTransaction) -> Vec<String> {
    sign_tx
        .proofs()
        .iter()
        .map(|proof| Base58::encode(proof, false))
        .collect()
}

fn invoke_to_json(invoke_tx: &InvokeScriptTransaction, json: &mut Map<String, Value>) {
    json.insert("dApp".to_owned(), invoke_tx.dapp().encoded().into());
    let mut call: Map<String, Value> = Map::new();
    call.insert("function".to_owned(), invoke_tx.function().name().into());
    let mut args = vec![];
    args_to_json(invoke_tx.function().args(), &mut args);
    call.insert("args".to_owned(), Value::Array(args));
    json.insert("call".to_owned(), call.into());
    let payments: Vec<Value> = invoke_tx
        .payment()
        .iter()
        .map(|arg| {
            let mut map = Map::new();
            map.insert("amount".to_owned(), arg.value().into());
            map.insert(
                "asset_id".to_owned(),
                arg.asset_id().map(|it| it.encoded()).into(),
            );
            map.into()
        })
        .collect();
    json.insert("payments".to_owned(), payments.into());
}

fn args_to_json(args: Vec<Arg>, json_args: &mut Vec<Value>) {
    for arg in args {
        let mut arg_map = Map::new();
        match arg {
            Arg::Binary(binary) => {
                arg_map.insert("type".to_owned(), "binary".into());
                arg_map.insert("value".to_owned(), binary.encoded_with_prefix().into());
            }
            Arg::Boolean(boolean) => {
                arg_map.insert("type".to_owned(), "boolean".into());
                arg_map.insert("value".to_owned(), boolean.into());
            }
            Arg::Integer(integer) => {
                arg_map.insert("type".to_owned(), "integer".into());
                arg_map.insert("value".to_owned(), integer.into());
            }
            Arg::String(string) => {
                arg_map.insert("type".to_owned(), "string".into());
                arg_map.insert("value".to_owned(), string.into());
            }
            Arg::List(list) => {
                arg_map.insert("type".to_owned(), "list".into());
                let mut list_args = vec![];
                args_to_json(list, &mut list_args);
                arg_map.insert("value".to_owned(), Value::Array(list_args));
            }
        };
        json_args.push(arg_map.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::account::PublicKey;
    use crate::model::data_entry::DataEntry;
    use crate::model::{
        Amount, ChainId, DataTransaction, SignedTransaction, Transaction, TransactionData,
    };
    use crate::util::json::json_deserializer::JsonDeserializer;
    use crate::util::{Base58, JsonSerializer};

    #[test]
    fn test_data_transaction_to_json() {
        let binary_value: [u8; 12] = [0; 12];

        let transaction_data = TransactionData::Data(DataTransaction::new(vec![
            DataEntry::IntegerEntry {
                key: "int".to_string(),
                value: 12,
            },
            DataEntry::BooleanEntry {
                key: "bool".to_string(),
                value: false,
            },
            DataEntry::BinaryEntry {
                key: "binary".to_string(),
                value: binary_value.to_vec(),
            },
            DataEntry::StringEntry {
                key: "str".to_string(),
                value: "value".to_string(),
            },
        ]));

        let public_key = PublicKey::from_string("8jDzNuHZwuTTo6WvZMdSoNc8ydY6a7UnxvwHZ8kooMuS")
            .expect("failed to get public key from string");
        let signed_transaction = SignedTransaction::new(
            Transaction::new(
                transaction_data,
                Amount::new(100000, None),
                1661456063029,
                public_key,
                DataTransaction::tx_type(),
                2,
                ChainId::TESTNET.byte(),
            ),
            vec![
                Base58::decode(
                    "4nDUCnVw9j9D5bTBSLfFCHR9CtvS32mSdxctccChRAohfLwz3ng3ps5ffUiy4NtRmXG7vDHRMW57ABxzkMW64tzC"
                ).expect("Failed to decode base58 string")
            ],
        );

        let json = JsonSerializer::serialize_signed_tx(&signed_transaction)
            .expect("failed to serialize signed transaction");
        let signed_tx_from_json =
            JsonDeserializer::deserialize_signed_tx(&json, ChainId::TESTNET.byte())
                .expect("Failed to deserialize signed tx");
        assert_eq!(signed_transaction, signed_tx_from_json);
    }
}
