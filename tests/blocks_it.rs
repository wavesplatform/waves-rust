use waves_rust::api::{Node, Profile};
use waves_rust::error::Result;
use waves_rust::model::{Address, Base58String};

#[tokio::test]
async fn get_height_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let height = node.get_height().await?;
    println!("{}", height);
    Ok(())
}

#[tokio::test]
async fn get_block_height_by_id_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let block_id =
        Base58String::from_string("oReBHRjMcUKqZxH6iVhthxQ72QndBFtfLHngV8aGW9y".to_owned())?;
    let height = node.get_block_height_by_id(&block_id).await?;
    println!("{}", height);
    Ok(())
}

#[tokio::test]
async fn get_block_height_by_timestamp_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let timestamp = 1662963400142;
    let height = node.get_block_height_by_timestamp(timestamp).await?;
    println!("{}", height);
    Ok(())
}

#[tokio::test]
async fn get_blocks_delay_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let block_id =
        Base58String::from_string("oReBHRjMcUKqZxH6iVhthxQ72QndBFtfLHngV8aGW9y".to_owned())?;
    let delay = node.get_blocks_delay(&block_id, 3).await?;
    println!("{}", delay);
    Ok(())
}

#[tokio::test]
async fn get_block_headers_at_height_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let headers = node.get_block_headers_at_height(2225531).await?;
    println!("{:#?}", headers);
    Ok(())
}

#[tokio::test]
async fn get_block_headers_by_id_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let block_id =
        Base58String::from_string("oReBHRjMcUKqZxH6iVhthxQ72QndBFtfLHngV8aGW9y".to_owned())?;
    let headers = node.get_block_headers_by_id(&block_id).await?;
    println!("{:#?}", headers);
    Ok(())
}

#[tokio::test]
async fn get_blocks_headers_seq_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let blocks_headers = node.get_blocks_headers_seq(2225585, 2225588).await?;
    println!("{:#?}", blocks_headers);
    Ok(())
}

#[tokio::test]
async fn get_last_block_headers_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let block_headers = node.get_last_block_headers().await?;
    println!("{:#?}", block_headers);
    Ok(())
}

//todo proxy error
//#[tokio::test]
async fn get_block_at_height_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let block = node.get_block_at_height(2221645).await?;
    println!("{:#?}", block);
    Ok(())
}

#[tokio::test]
async fn get_block_by_id_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let block_id =
        Base58String::from_string("E6uQ1HKHU6eTzHddBTr4Xdif1v6FGr2agxEr7qCgwREY".to_owned())?;
    let block = node.get_block_by_id(&block_id).await?;
    println!("{:#?}", block);
    Ok(())
}

//todo proxy error
//#[tokio::test]
async fn get_blocks_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let blocks = node.get_blocks(2221643, 2221645).await?;
    println!("{:#?}", blocks);
    Ok(())
}

#[tokio::test]
async fn get_last_block_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let block = node.get_last_block().await?;
    println!("{:#?}", block);
    Ok(())
}

#[tokio::test]
async fn get_blocks_by_generator_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let generator = Address::from_string("3Mxv6Dpa1qRuyQBRFg3GwUaf3rcjHqWwNmC")?;

    let block = node
        .get_blocks_by_generator(&generator, 2225770, 2225796)
        .await?;
    println!("{:#?}", block);
    Ok(())
}
