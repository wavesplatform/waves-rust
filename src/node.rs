use std::str::FromStr;
use reqwest::Url;
use serde_json::Value;
use crate::json_serializer::{from_json, TransactionInfo};
use crate::model::ChainId;

pub const MAINNET_URL: &str = "https://nodes.wavesnodes.com";
pub const TESTNET_URL: &str = "https://nodes-testnet.wavesnodes.com";
pub const STAGENET_URL: &str = "https://nodes-stagenet.wavesnodes.com";
pub const LOCAL_URL: &str = "http://127.0.0.1:6869";

pub struct Node {
    url: Url,
    chain_id: u8,
}

impl Node {
    pub fn from_profile(profile: Profile) -> Node {
        Node {
            url: profile.url(),
            chain_id: profile.chain_id(),
        }
    }

    pub fn url(&self) -> Url {
        self.url.clone()
    }

    pub fn get_transaction_info(&self, transaction_id: String) -> TransactionInfo {
        let get_tx_info_url = format!("{}transactions/info/{}", self.url().as_str(), transaction_id);
        let body: Value = reqwest::blocking::get(get_tx_info_url)
            .unwrap()
            .json()
            .unwrap();
        from_json(body)
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
            Profile::MAINNET => "https://nodes.wavesnodes.com",
            Profile::TESTNET => "https://nodes-testnet.wavesnodes.com",
            Profile::STAGENET => "https://nodes-stagenet.wavesnodes.com",
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
    use crate::json_serializer::ApplicationStatus;
    use crate::node::{Node, Profile};

    #[test]
    fn test_get_transaction_info() {
        let tx_id = "8YsBZSZ3UmWAo8bCj8RN64BvoQUTdLtd567hXqQCYDVo";

        let node = Node::from_profile(Profile::MAINNET);
        let transaction_info = node.get_transaction_info(tx_id.into());

        assert_eq!(transaction_info.id(), "8YsBZSZ3UmWAo8bCj8RN64BvoQUTdLtd567hXqQCYDVo");
        assert_eq!(transaction_info.status(), ApplicationStatus::Succeed);
        assert_eq!(transaction_info.height(), 3229634);

        let transaction = transaction_info.tx();
        assert_eq!(transaction.timestamp(), 1659278184707);
        assert_eq!(transaction.sender_public_key(), "AdZiupVsS9PMbTQK7iePWmD4Y5s8ZF6PoaQFyHKV2anj");
        assert_eq!(transaction.tx_type(), 4);
        assert_eq!(transaction.version(), 1);

        let transfer_transaction = transaction.data().transfer().unwrap();
        assert_eq!(transfer_transaction.attachment(), Some("".into()));
        assert_eq!(transfer_transaction.recipient(), "3PHey9P6xpUubQqP7DgMeWaza41yWQGGbHK");
        assert_eq!(transfer_transaction.asset(), None);
        assert_eq!(transfer_transaction.amount(), 46095972);
    }
}