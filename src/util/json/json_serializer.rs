use serde_json::{Map, Value};

use crate::error::{Error, Result};
use crate::model::{
    BurnTransaction, ByteString, CreateAliasTransaction, DataTransaction, EthereumTransaction,
    ExchangeTransaction, GenesisTransaction, InvokeScriptTransaction, IssueTransaction,
    LeaseCancelTransaction, LeaseTransaction, MassTransferTransaction, PaymentTransaction,
    ReissueTransaction, SetAssetScriptTransaction, SetScriptTransaction, SignedTransaction,
    SponsorFeeTransaction, Transaction, TransactionData, TransferTransaction,
    UpdateAssetInfoTransaction,
};

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
        TransactionData::Genesis(_) => Err(Error::UnsupportedOperation(
            "broadcasting genesis transaction".to_owned(),
        ))?,
        TransactionData::Payment(_) => Err(Error::UnsupportedOperation(
            "broadcasting payment transaction".to_owned(),
        ))?,
        TransactionData::Transfer(transfer_tx) => {
            json_props.append(&mut transfer_tx.try_into()?);
        }
        TransactionData::Data(data_tx) => {
            json_props.append(&mut data_tx.try_into()?);
        }
        TransactionData::Issue(issue_tx) => {
            json_props.append(&mut issue_tx.try_into()?);
        }
        TransactionData::InvokeScript(invoke_tx) => {
            json_props.append(&mut invoke_tx.try_into()?);
        }
        TransactionData::Exchange(exchange_tx) => {
            json_props.append(&mut exchange_tx.try_into()?);
        }
        TransactionData::Reissue(reissue_tx) => {
            json_props.append(&mut reissue_tx.try_into()?);
        }
        TransactionData::Burn(burn_tx) => {
            json_props.append(&mut burn_tx.try_into()?);
        }
        TransactionData::Lease(lease_tx) => {
            json_props.append(&mut lease_tx.try_into()?);
        }
        TransactionData::LeaseCancel(lease_cancel_tx) => {
            json_props.append(&mut lease_cancel_tx.try_into()?);
        }
        TransactionData::CreateAlias(create_alias_tx) => {
            json_props.append(&mut create_alias_tx.try_into()?);
        }
        TransactionData::MassTransfer(mass_transfer_tx) => {
            json_props.append(&mut mass_transfer_tx.try_into()?);
        }
        TransactionData::SetScript(set_script_tx) => {
            json_props.append(&mut set_script_tx.try_into()?);
        }
        TransactionData::SetAssetScript(set_asset_script_tx) => {
            json_props.append(&mut set_asset_script_tx.try_into()?);
        }
        TransactionData::SponsorFee(sponsor_fee_tx) => {
            json_props.append(&mut sponsor_fee_tx.try_into()?);
        }
        TransactionData::UpdateAssetInfo(update_asset_info_tx) => {
            json_props.append(&mut update_asset_info_tx.try_into()?);
        }
        TransactionData::Ethereum(_) => Err(Error::UnsupportedOperation(
            "broadcasting ethereum transaction".to_owned(),
        ))?,
    };
    Ok(json_props.clone())
}

fn tx_type(tx: &Transaction) -> u8 {
    match tx.data() {
        TransactionData::Genesis(_) => GenesisTransaction::tx_type(),
        TransactionData::Payment(_) => PaymentTransaction::tx_type(),
        TransactionData::Transfer(_) => TransferTransaction::tx_type(),
        TransactionData::Data(_) => DataTransaction::tx_type(),
        TransactionData::Issue(_) => IssueTransaction::tx_type(),
        TransactionData::InvokeScript(_) => InvokeScriptTransaction::tx_type(),
        TransactionData::Exchange(_) => ExchangeTransaction::tx_type(),
        TransactionData::Reissue(_) => ReissueTransaction::tx_type(),
        TransactionData::Burn(_) => BurnTransaction::tx_type(),
        TransactionData::Lease(_) => LeaseTransaction::tx_type(),
        TransactionData::LeaseCancel(_) => LeaseCancelTransaction::tx_type(),
        TransactionData::CreateAlias(_) => CreateAliasTransaction::tx_type(),
        TransactionData::MassTransfer(_) => MassTransferTransaction::tx_type(),
        TransactionData::SetScript(_) => SetScriptTransaction::tx_type(),
        TransactionData::SetAssetScript(_) => SetAssetScriptTransaction::tx_type(),
        TransactionData::SponsorFee(_) => SponsorFeeTransaction::tx_type(),
        TransactionData::UpdateAssetInfo(_) => UpdateAssetInfoTransaction::tx_type(),
        TransactionData::Ethereum(_) => EthereumTransaction::tx_type(),
    }
}

fn proofs(sign_tx: &SignedTransaction) -> Vec<String> {
    sign_tx
        .proofs()
        .iter()
        .map(|proof| proof.encoded())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::account::PublicKey;
    use crate::model::data_entry::DataEntry;
    use crate::model::{
        Amount, ChainId, DataTransaction, Proof, SignedTransaction, Transaction, TransactionData,
    };
    use crate::util::{Base58, JsonSerializer};

    #[test]
    fn test_data_transaction_to_json() -> Result<()> {
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

        let public_key = PublicKey::from_string("8jDzNuHZwuTTo6WvZMdSoNc8ydY6a7UnxvwHZ8kooMuS")?;
        let signed_transaction = SignedTransaction::new(
            Transaction::new(
                transaction_data,
                Amount::new(100000, None),
                1661456063029,
                public_key,
                2,
                ChainId::TESTNET.byte(),
            ),
            vec![
                Proof::new(Base58::decode(
                    "4nDUCnVw9j9D5bTBSLfFCHR9CtvS32mSdxctccChRAohfLwz3ng3ps5ffUiy4NtRmXG7vDHRMW57ABxzkMW64tzC"
                )?)
            ],
        );

        let json = &JsonSerializer::serialize_signed_tx(&signed_transaction)?;
        let signed_tx_from_json: SignedTransaction = json.try_into()?;
        assert_eq!(signed_transaction, signed_tx_from_json);
        Ok(())
    }
}
