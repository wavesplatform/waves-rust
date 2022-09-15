use waves_rust::api::{Node, Profile};
use waves_rust::error::Result;
use waves_rust::model::{
    Address, Amount, Base58String, ChainId, Id, PrivateKey, Transaction, TransactionData,
    TransferTransaction,
};
use waves_rust::util::get_current_epoch_millis;

const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
trigger used census";

#[ignore]
#[tokio::test]
async fn calculate_transaction_fee_test() -> Result<()> {
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

    let amount = node.calculate_transaction_fee(&signed_tx).await?;
    println!("{:#?}", amount);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_transaction_info_test() -> Result<()> {
    let testnet_node = Node::from_profile(Profile::TESTNET);
    let mainnet_node = Node::from_profile(Profile::MAINNET);
    let stagenet_node = Node::from_profile(Profile::STAGENET);

    let genesis_tx_id = Id::from_string(
        "3zpi4i5SeCoaiCBn1iuTUvCc5aahvtabqXBTrCXy1Y3ujUbJo56VVv6n4HQtcwiFapvg3BKV6stb5QkxsBrudTKZ",
    )?;
    let genesis_tx_info = testnet_node.get_transaction_info(&genesis_tx_id).await?;
    println!("{:#?}", genesis_tx_info);

    let payment_tx_id = Id::from_string(
        "3MBsS7S42PVEM8c1XxLsGsxzhitPsyaazDs1QoE26pCTHdRMYRv7n984wmjSFP863iZ2GR28aunSVvPC8sooEpbP",
    )?;
    let payment_tx_info = mainnet_node.get_transaction_info(&payment_tx_id).await?;
    println!("{:#?}", payment_tx_info);

    let issue_tx_id = Id::from_string("3kuZKAeyjcqavmezy86sWCAeXrgt3HBKa4HA8CZdT8nH")?;
    let issue_tx_info = testnet_node.get_transaction_info(&issue_tx_id).await?;
    println!("{:#?}", issue_tx_info);

    let transfer_tx_id = Id::from_string("DBozd2VWYe1FDkrdQnJgvcxh9B6mL872onqpSCjF4a7t")?;
    let transfer_tx_info = testnet_node.get_transaction_info(&transfer_tx_id).await?;
    println!("{:#?}", transfer_tx_info);

    let reissue_tx_id = Id::from_string("44seokQaBquAwDweKC4mbmHvmu2heWrUhKNGUakwZxRf")?;
    let reissue_tx_info = testnet_node.get_transaction_info(&reissue_tx_id).await?;
    println!("{:#?}", reissue_tx_info);

    let burn_tx_id = Id::from_string("7Ruo9tnYTuBKTRwbSfG2TLooP4v6pz8SkTx1hvCgfJLU")?;
    let burn_tx_info = testnet_node.get_transaction_info(&burn_tx_id).await?;
    println!("{:#?}", burn_tx_info);

    let exchange_tx_id = Id::from_string("2dn5KbBN4itxU2eYVmFheEyZEwRC9DMVkY3MNizEnXkX")?;
    let exchange_tx_info = testnet_node.get_transaction_info(&exchange_tx_id).await?;
    println!("{:#?}", exchange_tx_info);

    let lease_tx_id = Id::from_string("FL9juc4i2e5L2LnnrcagWQf7LYBmcJrxrxQdBrxNkwjx")?;
    let lease_tx_info = testnet_node.get_transaction_info(&lease_tx_id).await?;
    println!("{:#?}", lease_tx_info);

    let lease_cancel_tx_id = Id::from_string("FoPVrSqzK74bwt8hgCDsEb48HJv7g2nvjeCW5wBoWpXb")?;
    let lease_cancel_tx_info = testnet_node
        .get_transaction_info(&lease_cancel_tx_id)
        .await?;
    println!("{:#?}", lease_cancel_tx_info);

    let create_alias_tx_id = Id::from_string("5Hri2XC3QFqP4MNHa84rdyoAAQKiL8ijhVy2WmPjRwdv")?;
    let create_alias_tx_info = testnet_node
        .get_transaction_info(&create_alias_tx_id)
        .await?;
    println!("{:#?}", create_alias_tx_info);

    let mass_transfer_tx_id = Id::from_string("JCcN8JZn35ww6VJSATdyPxsdSgSdj4ZzzLFdTNtiwKXh")?;
    let mass_transfer_tx_info = testnet_node
        .get_transaction_info(&mass_transfer_tx_id)
        .await?;
    println!("{:#?}", mass_transfer_tx_info);

    let data_tx_id = Id::from_string("Aui38ZYPbNAEz8K2dvfN1bMT6FzXziqjjeSceCCizRmJ")?;
    let data_tx_info = testnet_node.get_transaction_info(&data_tx_id).await?;
    println!("{:#?}", data_tx_info);

    let set_script_tx_id = Id::from_string("65TyhCmJjseze6WvXstDDEZ5YvLzGcNiBJqkfFsjR3C2")?;
    let set_script_tx_info = testnet_node.get_transaction_info(&set_script_tx_id).await?;
    println!("{:#?}", set_script_tx_info);

    let sponsor_fee_tx_id = Id::from_string("5y8knLkSH9C6xnd7SKvsa2VzVm4kowFHJHwHUj27gdZ9")?;
    let sponsor_fee_tx_info = testnet_node
        .get_transaction_info(&sponsor_fee_tx_id)
        .await?;
    println!("{:#?}", sponsor_fee_tx_info);

    let set_asset_script_tx_id = Id::from_string("AcyDhTrLZA1B5QRMWGPz4KZ1oQWyiKAKU4NSdcuiThZi")?;
    let set_asset_script_tx_info = testnet_node
        .get_transaction_info(&set_asset_script_tx_id)
        .await?;
    println!("{:#?}", set_asset_script_tx_info);

    let invoke_tx_id = Id::from_string("7QT8tS7eC3Krzc65GVBdzGyfeCk8kDy9y2BTp6fMr6vx")?;
    let invoke_tx_info = testnet_node.get_transaction_info(&invoke_tx_id).await?;
    println!("{:#?}", invoke_tx_info);

    let update_asset_info_tx_id = Id::from_string("A8xxTmhe8PDiggtJtK64maaseq7kp35tHqJHGjZ98xmo")?;
    let update_asset_info_tx_info = stagenet_node
        .get_transaction_info(&update_asset_info_tx_id)
        .await?;
    println!("{:#?}", update_asset_info_tx_info);

    let eth_transfer_tx_id = Id::from_string("CWuFY42te67sLmc5gwt4NxwHmFjVfJdHkKuLyshTwEct")?;
    let eth_transfer_tx_info = stagenet_node
        .get_transaction_info(&eth_transfer_tx_id)
        .await?;
    println!("{:#?}", eth_transfer_tx_info);

    let eth_invoke_tx_id = Id::from_string("CWuFY42te67sLmc5gwt4NxwHmFjVfJdHkKuLyshTwEct")?;
    let eth_invoke_tx_info = stagenet_node
        .get_transaction_info(&eth_invoke_tx_id)
        .await?;
    println!("{:#?}", eth_invoke_tx_info);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_transactions_by_address_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);

    let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q")?;

    let after_id = Some(Id::from_string(
        "3p6ffM2uyseFWPRQUcXMpr3gBKkKgt7jVQ8iDGQhVpRa",
    )?);
    let transactions = node
        .get_transactions_by_address(&address, 10, after_id)
        .await?;
    println!("{:#?}", transactions);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_transaction_status_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);

    let tx_id = Id::from_string("3p6ffM2uyseFWPRQUcXMpr3gBKkKgt7jVQ8iDGQhVpRa")?;
    let status = node.get_transaction_status(&tx_id).await?;
    println!("{:#?}", status);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_transactions_statuses_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);

    let tx_id1 = Id::from_string("3p6ffM2uyseFWPRQUcXMpr3gBKkKgt7jVQ8iDGQhVpRa")?;
    let tx_id2 = Id::from_string("7QT8tS7eC3Krzc65GVBdzGyfeCk8kDy9y2BTp6fMr6vx")?;
    let statuses = node.get_transactions_statuses(&[tx_id1, tx_id2]).await?;
    println!("{:#?}", statuses);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_unconfirmed_transaction_test() -> Result<()> {
    let node = Node::from_profile(Profile::MAINNET);

    let tx_id = Id::from_string("3p6ffM2uyseFWPRQUcXMpr3gBKkKgt7jVQ8iDGQhVpRa")?;
    let tx = node.get_unconfirmed_transaction(&tx_id).await?;
    println!("{:#?}", tx);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_unconfirmed_transactions_test() -> Result<()> {
    let node = Node::from_profile(Profile::MAINNET);
    let txs = node.get_unconfirmed_transactions().await?;
    println!("{:#?}", txs);
    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_utx_size_test() -> Result<()> {
    let node = Node::from_profile(Profile::MAINNET);
    let size = node.get_utx_size().await?;
    println!("{:#?}", size);
    Ok(())
}
