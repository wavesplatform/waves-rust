use waves_rust::api::{Node, Profile};
use waves_rust::model::{
    Amount, Base64String, ChainId, PrivateKey, SetScriptTransaction, Transaction, TransactionData,
};
use waves_rust::util::get_current_epoch_millis;

const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
trigger used census";

//todo add docker private node

const COMPILED_SCRIPT: &str = "base64:AAIFAAAAAAAAAAsIAhIHCgUCBAEIEQAAAAAAAAABAAAAA2ludgEAAAAEY2FsbAAAAAUAAAACYnYAAAABYgAAAANpbnQAAAADc3RyAAAABGxpc3QEAAAABWFzc2V0CQAEQgAAAAUCAAAABUFzc2V0AgAAAAAAAAAAAAAAAAEAAAAAAAAAAAAGBAAAAAdhc3NldElkCQAEOAAAAAEFAAAABWFzc2V0BAAAAAVsZWFzZQkABEQAAAACCAUAAAADaW52AAAABmNhbGxlcgAAAAAAAAAABwQAAAAHbGVhc2VJZAkABDkAAAABBQAAAAVsZWFzZQkABEwAAAACCQEAAAALQmluYXJ5RW50cnkAAAACAgAAAANiaW4FAAAAB2Fzc2V0SWQJAARMAAAAAgkBAAAADEJvb2xlYW5FbnRyeQAAAAICAAAABGJvb2wGCQAETAAAAAIJAQAAAAxJbnRlZ2VyRW50cnkAAAACAgAAAANpbnQAAAAAAAABiJQJAARMAAAAAgkBAAAAC1N0cmluZ0VudHJ5AAAAAgIAAAAHYXNzZXRJZAkAAlgAAAABBQAAAAdhc3NldElkCQAETAAAAAIJAQAAAAtTdHJpbmdFbnRyeQAAAAICAAAAB2xlYXNlSWQJAAJYAAAAAQUAAAAHbGVhc2VJZAkABEwAAAACCQEAAAALU3RyaW5nRW50cnkAAAACAgAAAANkZWwCAAAAAAkABEwAAAACCQEAAAALRGVsZXRlRW50cnkAAAABAgAAAANkZWwJAARMAAAAAgUAAAAFYXNzZXQJAARMAAAAAgkBAAAAClNwb25zb3JGZWUAAAACBQAAAAdhc3NldElkAAAAAAAAAAABCQAETAAAAAIJAQAAAAdSZWlzc3VlAAAAAwUAAAAHYXNzZXRJZAAAAAAAAAAABAcJAARMAAAAAgkBAAAABEJ1cm4AAAACBQAAAAdhc3NldElkAAAAAAAAAAADCQAETAAAAAIJAQAAAA5TY3JpcHRUcmFuc2ZlcgAAAAMIBQAAAANpbnYAAAAGY2FsbGVyAAAAAAAAAAACBQAAAAdhc3NldElkCQAETAAAAAIFAAAABWxlYXNlCQAETAAAAAIJAQAAAAtMZWFzZUNhbmNlbAAAAAEJAAQ5AAAAAQUAAAAFbGVhc2UFAAAAA25pbAAAAAD/oHwO";

//#[tokio::test]
async fn broadcast_and_read_test() {
    let private_key =
        PrivateKey::from_seed("d", 0).expect("failed to get private ket from seed phrase");

    let transaction_data = TransactionData::SetScript(SetScriptTransaction::new(
        Base64String::from_string(COMPILED_SCRIPT).expect("failed"),
    ));

    let timestamp = get_current_epoch_millis();
    let signed_tx = Transaction::new(
        transaction_data,
        Amount::new(100000, None),
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
