use waves_rust::api::{Node, Profile};
use waves_rust::model::{
    Amount, AssetId, ByteString, ChainId, PrivateKey, ReissueTransaction, Transaction,
    TransactionData,
};
use waves_rust::util::get_current_epoch_millis;

const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
trigger used census";

//todo add docker private node

#[ignore]
#[tokio::test]
async fn broadcast_and_read_test() {
    let private_key =
        PrivateKey::from_seed("b", 0).expect("failed to get private ket from seed phrase");

    let transaction_data = TransactionData::Reissue(ReissueTransaction::new(
        Amount::new(
            12,
            Some(
                AssetId::from_string("8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6")
                    .expect("failed"),
            ),
        ),
        true,
    ));

    let timestamp = get_current_epoch_millis();
    let signed_tx = Transaction::new(
        transaction_data,
        Amount::new(100000000, None),
        timestamp,
        private_key.public_key(),
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
