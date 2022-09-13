use waves_rust::api::{Node, Profile};
use waves_rust::error::Result;

#[tokio::test]
async fn compile_script_test() -> Result<()> {
    let node = Node::from_profile(Profile::TESTNET);

    let script = r#"
        {-# STDLIB_VERSION 2 #-}
        {-# CONTENT_TYPE EXPRESSION #-}
        {-# SCRIPT_TYPE ASSET #-}

        let master = addressFromString("3masterAddress")
        match tx {
            case t: TransferTransaction =>
                t.sender == master || t.recipient == master
            case mt: MassTransferTransaction =>
                mt.sender == master
            case _: ExchangeTransaction => false
            case _ => true
        }"#;
    let script_info = node.compile_script(script, true).await?;
    println!("{:#?}", script_info);
    Ok(())
}
