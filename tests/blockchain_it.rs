use waves_rust::api::{Node, Profile};

#[ignore]
#[tokio::test]
async fn get_blockchain_rewards_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let rewards = node.get_blockchain_rewards().await;

    match rewards {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{:?}", err),
    }
}

#[ignore]
#[tokio::test]
async fn get_blockchain_rewards_at_height_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let rewards = node.get_blockchain_rewards_at_height(2_220_000).await;

    match rewards {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => println!("{:?}", err),
    }
}
