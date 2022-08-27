use std::str::FromStr;
use std::time::Duration;

use reqwest::{Client, Url};
use serde_json::Value;

use crate::model::{ChainId, TransactionInfo};
use crate::util::JsonDeserializer;

pub const MAINNET_URL: &str = "https://nodes.wavesnodes.com";
pub const TESTNET_URL: &str = "https://nodes-testnet.wavesnodes.com";
pub const STAGENET_URL: &str = "https://nodes-stagenet.wavesnodes.com";
pub const LOCAL_URL: &str = "http://127.0.0.1:6869";

pub struct Node {
    url: Url,
    chain_id: u8,
    http_client: Client,
}

impl Node {
    pub fn from_profile(profile: Profile) -> Node {
        Node {
            url: profile.url(),
            chain_id: profile.chain_id(),
            http_client: Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .unwrap(),
        }
    }

    pub fn url(&self) -> Url {
        self.url.clone()
    }

    pub fn chain_id(&self) -> u8 {
        self.chain_id
    }

    // todo return Result<TransactionInfo, Error>
    pub async fn get_transaction_info(&self, transaction_id: String) -> TransactionInfo {
        let get_tx_info_url = format!(
            "{}transactions/info/{}",
            self.url().as_str(),
            transaction_id
        );
        let body: Value = self
            .http_client
            .get(get_tx_info_url)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        JsonDeserializer::deserialize_tx_info(body, self.chain_id)
    }
}

pub enum Profile {
    MAINNET,
    TESTNET,
    STAGENET,
}

impl Profile {
    pub fn url(&self) -> Url {
        let url = match *self {
            Profile::MAINNET => MAINNET_URL,
            Profile::TESTNET => TESTNET_URL,
            Profile::STAGENET => STAGENET_URL,
        };
        Url::from_str(url).unwrap()
    }

    pub fn chain_id(&self) -> u8 {
        match *self {
            Profile::MAINNET => ChainId::MAINNET.byte(),
            Profile::TESTNET => ChainId::TESTNET.byte(),
            Profile::STAGENET => ChainId::STAGENET.byte(),
        }
    }
}

#[cfg(test)]
mod tests {
    use ChainId::MAINNET;

    use crate::model::data_entry::DataEntry;
    use crate::model::{ApplicationStatus, ChainId};
    use crate::node::{Node, Profile};
    use crate::util::Base58;

    #[tokio::test]
    async fn test_get_transfer_transaction_info() {
        let tx_id = "8YsBZSZ3UmWAo8bCj8RN64BvoQUTdLtd567hXqQCYDVo";

        let node = Node::from_profile(Profile::MAINNET);
        let transaction_info = node.get_transaction_info(tx_id.into()).await;

        assert_eq!(
            transaction_info.id(),
            "8YsBZSZ3UmWAo8bCj8RN64BvoQUTdLtd567hXqQCYDVo"
        );
        assert_eq!(transaction_info.status(), ApplicationStatus::Succeed);
        assert_eq!(transaction_info.height(), 3229634);

        let signed_transaction = transaction_info.signed_tx();

        let proof_from_rs = "4NiakymjU9s7mJYTBGbweGrDDwAauEXsuhMCeQJD1S28cEFL7hpjEL2LhaiVyFScq8UGVucpvCBo8PogvHQCdhrZ";
        assert_eq!(
            signed_transaction.proofs()[0],
            Base58::decode(proof_from_rs).unwrap()
        );

        let transaction = signed_transaction.tx();

        assert_eq!(transaction.timestamp(), 1659278184707);
        assert_eq!(transaction.fee().fee(), 100000);
        assert_eq!(transaction.fee().fee_asset_id(), None);
        assert_eq!(
            transaction.public_key().address(MAINNET.byte()).encoded(),
            "3P4eeU7v1LMHQFwwT2GW9W99c6vZyytHajj"
        );
        assert_eq!(
            transaction.public_key().encoded(),
            "AdZiupVsS9PMbTQK7iePWmD4Y5s8ZF6PoaQFyHKV2anj"
        );
        assert_eq!(transaction.tx_type(), 4);
        assert_eq!(transaction.version(), 1);

        let transfer_transaction = transaction.data().transfer_tx().unwrap();
        assert_eq!(transfer_transaction.attachment(), Some("".into()));
        assert_eq!(
            transfer_transaction.recipient(),
            "3PHey9P6xpUubQqP7DgMeWaza41yWQGGbHK"
        );
        assert_eq!(transfer_transaction.asset(), None);
        assert_eq!(transfer_transaction.amount(), 46095972);
    }

    #[tokio::test]
    async fn test_get_data_transaction_info() {
        let tx_id = "HcPcSma7oWeqy8g3ahhwFDzrq8YK8r739U4WC2ieB5Bs";

        let node = Node::from_profile(Profile::MAINNET);
        let transaction_info = node.get_transaction_info(tx_id.into()).await;

        assert_eq!(
            transaction_info.id(),
            "HcPcSma7oWeqy8g3ahhwFDzrq8YK8r739U4WC2ieB5Bs"
        );
        assert_eq!(transaction_info.status(), ApplicationStatus::Succeed);
        assert_eq!(transaction_info.height(), 3258212);

        let signed_transaction = transaction_info.signed_tx();

        let proof_from_rs = "25KiXB1FS3FaupiPXyEVeRquKLK4FEb3NWF36D1eHw1gpT9Y53MbLsVqnX9rJC8MPg4x9yiUxFkmxF9DDTgQruhi";
        assert_eq!(
            signed_transaction.proofs()[0],
            Base58::decode(proof_from_rs).unwrap()
        );

        let transaction = signed_transaction.tx();

        assert_eq!(transaction.timestamp(), 1660994483097);
        assert_eq!(transaction.fee().fee(), 500000);
        assert_eq!(transaction.fee().fee_asset_id(), None);
        assert_eq!(
            transaction.public_key().address(MAINNET.byte()).encoded(),
            "3P4sxdNNPJLQcitAnLqLfSwaenjxFxQvZsE"
        );
        assert_eq!(
            transaction.public_key().encoded(),
            "GTr2dXt3mxaD8tXGyNauV8YMy1hsSoi63DUuk4uyijqG"
        );
        assert_eq!(transaction.tx_type(), 12);
        assert_eq!(transaction.version(), 1);

        let data_transaction = transaction.data().data_tx().unwrap();

        let data_entries = data_transaction.data();

        match data_entries[0].clone() {
            DataEntry::IntegerEntry { key, value } => {
                assert_eq!(key, "price_ausdtlpm_20220820");
                assert_eq!(value, 1823153 as i64)
            }
            _ => panic!("failed"),
        };

        match data_entries[1].clone() {
            DataEntry::IntegerEntry { key, value } => {
                assert_eq!(key, "lastHeight_ausdtlpm");
                assert_eq!(value, 3258212 as i64)
            }
            _ => panic!("failed"),
        }
    }
}
