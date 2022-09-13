use waves_rust::api::{Node, Profile};
use waves_rust::error::Result;
use waves_rust::model::{Address, AssetId};

#[ignore]
#[tokio::test]
async fn get_assets_balance_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q")?;
    let balance = node.get_assets_balance(&address).await?;

    println!("{:#?}", balance);
    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_asset_distribution_test() -> Result<()> {
    let node = Node::from_profile(Profile::MAINNET);
    let asset_id = AssetId::from_string("DG2xFkPdDwKUoBkzGAhQtLpSGzfXLiCYPEzeKH2Ad24p")?;
    let after = Address::from_string("3P2iT1nawotR2QWmjfMAm18xytUiK6cWtHt")?;
    let asset_distribution = node
        .get_asset_distribution(&asset_id, 3292600, 10, Some(after))
        .await?;

    println!("{:#?}", asset_distribution);
    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_asset_balance_test() -> Result<()> {
    let node = Node::from_profile(Profile::MAINNET);
    let asset_id = AssetId::from_string("DG2xFkPdDwKUoBkzGAhQtLpSGzfXLiCYPEzeKH2Ad24p")?;
    let address = Address::from_string("3P2iT1nawotR2QWmjfMAm18xytUiK6cWtHt")?;
    let asset_balance = node.get_asset_balance(&address, &asset_id).await?;

    println!("{:#?}", asset_balance);
    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_asset_details_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let asset_id = AssetId::from_string("CVwsbXjXmdYF2q4RCPuQKf7sLGpzhk7BNnYsxGZZJMym")?;
    let asset_details = node.get_asset_details(&asset_id).await?;

    println!("{:#?}", asset_details);
    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_assets_details_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let asset_id1 = AssetId::from_string("CVwsbXjXmdYF2q4RCPuQKf7sLGpzhk7BNnYsxGZZJMym")?;
    let asset_id2 = AssetId::from_string("5HCFX88m6Xxws4SunQuW9ghvYBmk8rK8b6xVCRL8PyAw")?;
    let assets_details = node.get_assets_details(&[asset_id1, asset_id2]).await?;

    println!("{:#?}", assets_details);
    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_nft_test() -> Result<()> {
    let node = Node::from_profile(Profile::MAINNET);
    let address = Address::from_string("3PAETTtuW7aSiyKtn9GuML3RgtV1xdq1mQW")?;
    let after = AssetId::from_string("13PtvhAC28kNXXJP3Evgcba5mNMsCAQECUqCPBu5wJou")?;
    let nfts = node.get_nft(&address, 10, Some(after)).await?;

    println!("{:#?}", nfts);
    Ok(())
}
