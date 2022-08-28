use waves_rust::model::account::Address;
use waves_rust::model::ChainId;
use waves_rust::node::{Node, Profile};

#[tokio::test]
async fn get_balance_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let balance = node
        .get_balance(&Address::from_string(
            "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
            ChainId::TESTNET.byte(),
        ))
        .await
        .unwrap();
    assert_eq!(true, balance > 0)
}

#[tokio::test]
async fn get_balance_with_confirmation_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let balance = node
        .get_balance_with_confirmations(
            &Address::from_string(
                "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
                ChainId::TESTNET.byte(),
            ),
            100,
        )
        .await;

    match balance {
        Ok(result) => assert_eq!(true, result > 0),
        Err(err) => println!("{}", err),
    }
}

#[tokio::test]
async fn get_balances_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let balances = node
        .get_balances(
            &vec![
                Address::from_string(
                    "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
                    ChainId::TESTNET.byte(),
                ),
                Address::from_string(
                    "3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW",
                    ChainId::TESTNET.byte(),
                ),
            ],
            ChainId::TESTNET.byte(),
        )
        .await;

    match balances {
        Ok(result) => {
            for balance in result {
                println!("{:?}", balance);
            }
        }
        Err(err) => println!("{}", err),
    }
}

#[tokio::test]
async fn get_balances_at_height_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let balances = node
        .get_balances_at_height(
            &vec![
                Address::from_string(
                    "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
                    ChainId::TESTNET.byte(),
                ),
                Address::from_string(
                    "3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW",
                    ChainId::TESTNET.byte(),
                ),
            ],
            2202560,
            ChainId::TESTNET.byte(),
        )
        .await;
    match balances {
        Ok(result) => {
            for balance in result {
                println!("{:?}", balance);
            }
        }
        Err(err) => println!("{}", err),
    }
}

#[tokio::test]
async fn get_balance_details_test() {
    let node = Node::from_profile(Profile::TESTNET);
    let balance_details = node
        .get_balance_details(&Address::from_string(
            "3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q",
            ChainId::TESTNET.byte(),
        ))
        .await;
    match balance_details {
        Ok(result) => {
            println!("address: {}", result.address().encoded());
            println!("available: {}", result.available());
            println!("regular: {}", result.regular());
            println!("generating: {}", result.generating());
            println!("effective: {}", result.effective());
        }
        Err(err) => println!("{}", err),
    }
}
