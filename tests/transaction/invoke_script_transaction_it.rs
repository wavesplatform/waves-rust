use waves_rust::api::{Node, Profile};

use waves_rust::model::{
    Address, Amount, Arg, Base64String, ByteString, ChainId, Function, InvokeScriptTransaction,
    PrivateKey, Transaction, TransactionData,
};
use waves_rust::util::get_current_epoch_millis;

const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
trigger used census";

//todo add docker private node

#[ignore]
#[tokio::test]
async fn broadcast_and_read_test() {
    let private_key =
        PrivateKey::from_seed(SEED_PHRASE, 0).expect("failed to get private ket from seed phrase");

    let dapp = Address::from_string("3N2yqTEKArWS3ySs2f6t8fpXdjX6cpPuhG8")
        .expect("failed to create address from string");

    let function = Function::new(
        "storeData".to_owned(),
        vec![
            Arg::Boolean(true),
            Arg::String("some string".to_owned()),
            Arg::Integer(123),
            Arg::Binary(Base64String::from_bytes(vec![3, 5, 2, 11, 15])),
            Arg::List(vec![Arg::Integer(123), Arg::Integer(543)]),
        ],
    );

    let transaction_data = TransactionData::InvokeScript(InvokeScriptTransaction::new(
        dapp,
        function,
        vec![Amount::new(1, None), Amount::new(2, None)],
    ));

    let timestamp = get_current_epoch_millis();
    let signed_tx = Transaction::new(
        transaction_data,
        Amount::new(500000, None),
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
