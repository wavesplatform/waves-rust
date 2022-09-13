use waves_rust::api::{Node, Profile};
use waves_rust::error::Result;
use waves_rust::model::{
    Address, Amount, Base58String, ChainId, PrivateKey, Transaction, TransactionData,
    TransferTransaction,
};
use waves_rust::util::get_current_epoch_millis;

#[ignore]
#[tokio::test]
async fn get_balance_history_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q")?;
    let balances = node.get_balance_history(&address).await?;
    println!("{:#?}", balances);
    Ok(())
}

const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
trigger used census";

#[ignore]
#[tokio::test]
async fn validate_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);

    let private_key = PrivateKey::from_seed(SEED_PHRASE, 0)?;
    let signed_tx = Transaction::new(
        TransactionData::Transfer(TransferTransaction::new(
            Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q")?,
            Amount::new(100, None),
            Base58String::empty(),
        )),
        Amount::new(100000, None),
        get_current_epoch_millis(),
        private_key.public_key(),
        3,
        ChainId::TESTNET.byte(),
    )
    .sign(&private_key)?;

    let validation = node.validate_transaction(&signed_tx).await?;
    println!("{:#?}", validation);

    Ok(())
}
