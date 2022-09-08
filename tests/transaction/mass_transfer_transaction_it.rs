use waves_rust::model::{
    Address, Amount, AssetId, Base58String, BurnTransaction, ChainId, IssueTransaction,
    MassTransferTransaction, PrivateKey, ReissueTransaction, Transaction, TransactionData,
    Transfer,
};
use waves_rust::node::{Node, Profile};
use waves_rust::util::get_current_epoch_millis;
use waves_rust::waves_proto::invoke_script_result::Burn;

const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
trigger used census";

//todo add docker private node

#[tokio::test]
async fn broadcast_and_read_test() {
    let private_key =
        PrivateKey::from_seed("b", 0).expect("failed to get private ket from seed phrase");

    let transaction_data = TransactionData::MassTransfer(MassTransferTransaction::new(
        Some(AssetId::from_string("8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6").expect("failed")),
        vec![
            Transfer::new(
                Address::from_string("3MxtrLkrbcG28uTvmbKmhrwGrR65ooHVYvK").expect("failed"),
                10,
            ),
            Transfer::new(
                Address::from_string("3MxjhrvCr1nnDxvNJiCQfSC557gd8QYEhDx").expect("faield"),
                12,
            ),
        ],
        Base58String::from_bytes(vec![1, 2, 3]),
    ));

    let timestamp = get_current_epoch_millis();
    let signed_tx = Transaction::new(
        transaction_data,
        Amount::new(200000, None),
        timestamp,
        private_key.public_key(),
        BurnTransaction::tx_type(),
        3,
        ChainId::TESTNET.byte(),
    )
    .sign(&private_key)
    .expect("failed to sign transaction");

    let node = Node::from_profile(Profile::TESTNET);
    let signed_tx_from_rs = node.broadcast(&signed_tx).await;

    match signed_tx_from_rs {
        Ok(signed_tx_from_rs) => {
            assert_eq!(
                signed_tx_from_rs
                    .id()
                    .expect("failed to calculate tx id")
                    .encoded(),
                signed_tx.id().expect("failed to calculate id").encoded()
            )
        }
        Err(err) => println!("{:?}", err),
    }
}
