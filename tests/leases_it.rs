use waves_rust::api::{Node, Profile};
use waves_rust::error::Result;
use waves_rust::model::{Address, Id};

#[ignore]
#[tokio::test]
async fn get_active_leases_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q")?;
    let leases = node.get_active_leases(&address).await?;
    println!("{:#?}", leases);
    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_lease_info_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let lease_id = Id::from_string("BiJR8gCxR7crGEdy31jLkYpjpLy98kq3NuxPE8Z2Uk3b")?;
    let lease = node.get_lease_info(&lease_id).await?;
    println!("{:#?}", lease);
    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_leases_info_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);
    let lease_id1 = Id::from_string("BiJR8gCxR7crGEdy31jLkYpjpLy98kq3NuxPE8Z2Uk3b")?;
    let lease_id2 = Id::from_string("5EWudZk4xXaqRezrh26zqjbNeAzvEzDATjs4paKdyhGy")?;
    let leases = node.get_leases_info(&[lease_id1, lease_id2]).await?;
    println!("{:#?}", leases);
    Ok(())
}
