use waves_rust::api::{Node, Profile};
use waves_rust::error::Result;

#[tokio::test]
async fn get_version_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let version = node.get_version().await?;
    println!("{:#?}", version);
    Ok(())
}
