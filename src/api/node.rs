use regex::Regex;

use std::str::FromStr;
use std::time::Duration;

use crate::error::{Error, Result};
use reqwest::{Client, Url};
use serde_json::Value::Array;
use serde_json::{Map, Value};

use crate::model::account::{Address, Balance, BalanceDetails};
use crate::model::asset::balance::AssetsBalanceResponse;
use crate::model::data_entry::DataEntry;
use crate::model::{
    Alias, AliasesByAddressResponse, Base58String, Block, BlockHeaders, BlockchainRewards,
    ByteString, ChainId, ScriptInfo, ScriptMeta, SignedTransaction, TransactionInfoResponse,
};
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
                .expect("Failed to create http client for struct Node"),
        }
    }

    pub fn from_url(url: &str, chain_id: u8) -> Node {
        Node {
            url: Url::from_str(url).expect("failed to parse url"),
            chain_id,
            http_client: Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .expect("Failed to create http client for struct Node"),
        }
    }

    pub fn url(&self) -> Url {
        self.url.clone()
    }

    pub fn chain_id(&self) -> u8 {
        self.chain_id
    }

    // ADDRESSES

    pub async fn get_addresses(&self) -> Result<Vec<Address>> {
        let get_addresses_url = format!("{}addresses", self.url().as_str());
        let rs = self.get(&get_addresses_url).await?;
        JsonDeserializer::deserialize_addresses(&rs)
    }

    pub async fn get_addresses_seq(&self, from_index: u64, to_index: u64) -> Result<Vec<Address>> {
        let get_addresses_seq_url = format!(
            "{}addresses/seq/{}/{}",
            self.url().as_str(),
            from_index,
            to_index
        );
        let rs = self.get(&get_addresses_seq_url).await?;
        JsonDeserializer::deserialize_addresses(&rs)
    }

    pub async fn get_balance(&self, address: &Address) -> Result<u64> {
        let get_balance_url = format!(
            "{}addresses/balance/{}",
            self.url().as_str(),
            address.encoded(),
        );
        let rs = self.get(&get_balance_url).await?;
        Ok(JsonDeserializer::safe_to_int_from_field(&rs, "balance")? as u64)
    }

    pub async fn get_balance_with_confirmations(
        &self,
        address: &Address,
        confirmations: u32,
    ) -> Result<u64> {
        let get_balance_url = format!(
            "{}addresses/balance/{}/{}",
            self.url().as_str(),
            address.encoded(),
            confirmations
        );
        let rs = self.get(&get_balance_url).await?;
        Ok(JsonDeserializer::safe_to_int_from_field(&rs, "balance")? as u64)
    }

    pub async fn get_balances(&self, addresses: &[Address]) -> Result<Vec<Balance>> {
        let get_balances_url = format!("{}addresses/balance", self.url().as_str(),);
        let mut json_addresses: Map<String, Value> = Map::new();
        json_addresses.insert(
            "addresses".to_owned(),
            Array(
                addresses
                    .iter()
                    .map(|address| Value::String(address.encoded()))
                    .collect(),
            ),
        );
        let rs = self.post(&get_balances_url, &json_addresses.into()).await?;
        JsonDeserializer::deserialize_balances(&rs)
    }

    pub async fn get_balances_at_height(
        &self,
        addresses: &[Address],
        height: u32,
    ) -> Result<Vec<Balance>> {
        let get_balances_url = format!("{}addresses/balance", self.url().as_str());
        let mut json_addresses: Map<String, Value> = Map::new();
        json_addresses.insert(
            "addresses".to_owned(),
            Array(
                addresses
                    .iter()
                    .map(|address| Value::String(address.encoded()))
                    .collect(),
            ),
        );
        json_addresses.insert("height".to_owned(), height.into());
        let rs = self.post(&get_balances_url, &json_addresses.into()).await?;
        JsonDeserializer::deserialize_balances(&rs)
    }

    pub async fn get_balance_details(&self, address: &Address) -> Result<BalanceDetails> {
        let get_balance_details_url = format!(
            "{}addresses/balance/details/{}",
            self.url().as_str(),
            address.encoded()
        );
        let rs = self.get(&get_balance_details_url).await?;
        JsonDeserializer::deserialize_balance_details(&rs)
    }

    pub async fn get_data(&self, address: &Address) -> Result<Vec<DataEntry>> {
        let get_data_url = format!(
            "{}addresses/data/{}",
            self.url().as_str(),
            address.encoded()
        );
        let rs = self.get(&get_data_url).await?;

        JsonDeserializer::deserialize_data_array(&rs)
    }

    pub async fn get_data_by_keys(
        &self,
        address: &Address,
        keys: &[String],
    ) -> Result<Vec<DataEntry>> {
        let get_data_url = format!(
            "{}addresses/data/{}",
            self.url().as_str(),
            address.encoded()
        );
        let mut json_keys: Map<String, Value> = Map::new();
        json_keys.insert("keys".to_owned(), keys.into());
        let rs = self.post(&get_data_url, &json_keys.into()).await?;
        JsonDeserializer::deserialize_data_array(&rs)
    }

    pub async fn get_data_by_regex(
        &self,
        address: &Address,
        regex: &Regex,
    ) -> Result<Vec<DataEntry>> {
        let get_data_url = format!(
            "{}addresses/data/{}?matches={}",
            self.url().as_str(),
            address.encoded(),
            urlencoding::encode(regex.as_str())
        );
        let rs = self.get(&get_data_url).await?;
        JsonDeserializer::deserialize_data_array(&rs)
    }

    pub async fn get_data_by_key(&self, address: &Address, key: &str) -> Result<DataEntry> {
        let get_data_by_key_url = format!(
            "{}addresses/data/{}/{}",
            self.url().as_str(),
            address.encoded(),
            key
        );
        let rs = &self.get(&get_data_by_key_url).await?;
        rs.try_into()
    }

    pub async fn get_script_info(&self, address: &Address) -> Result<ScriptInfo> {
        let get_script_info_url = format!(
            "{}addresses/scriptInfo/{}",
            self.url().as_str(),
            address.encoded()
        );
        let rs = &self.get(&get_script_info_url).await?;
        JsonDeserializer::deserialize_script_info(rs)
    }

    pub async fn get_script_meta(&self, address: &Address) -> Result<ScriptMeta> {
        let get_script_meta_url = format!(
            "{}addresses/scriptInfo/{}/meta",
            self.url().as_str(),
            address.encoded()
        );
        let rs = &self.get(&get_script_meta_url).await?;
        JsonDeserializer::deserialize_script_meta(rs)
    }

    // ALIAS
    pub async fn get_aliases_by_address(
        &self,
        address: &Address,
    ) -> Result<AliasesByAddressResponse> {
        let get_aliases_by_address_url = format!(
            "{}alias/by-address/{}",
            self.url().as_str(),
            address.encoded()
        );
        let rs = &self.get(&get_aliases_by_address_url).await?;
        rs.try_into()
    }

    pub async fn get_address_by_alias(&self, alias: &Alias) -> Result<Address> {
        let get_address_by_alias_url =
            format!("{}alias/by-alias/{}", self.url().as_str(), alias.name());
        let rs = &self.get(&get_address_by_alias_url).await?;
        Address::from_string(&JsonDeserializer::safe_to_string_from_field(rs, "address")?)
    }

    // BLOCKCHAIN

    pub async fn get_blockchain_rewards(&self) -> Result<BlockchainRewards> {
        let get_blockchain_rewards_url = format!("{}blockchain/rewards", self.url().as_str());
        let rs = &self.get(&get_blockchain_rewards_url).await?;
        rs.try_into()
    }

    pub async fn get_blockchain_rewards_at_height(&self, height: u32) -> Result<BlockchainRewards> {
        let get_blockchain_rewards_url =
            format!("{}blockchain/rewards/{}", self.url().as_str(), height);
        let rs = &self.get(&get_blockchain_rewards_url).await?;
        rs.try_into()
    }

    // BLOCKS

    pub async fn get_height(&self) -> Result<u32> {
        let get_height_url = format!("{}blocks/height", self.url().as_str());
        let rs = &self.get(&get_height_url).await?;
        Ok(JsonDeserializer::safe_to_int_from_field(rs, "height")? as u32)
    }

    pub async fn get_block_height_by_id(&self, block_id: Base58String) -> Result<u32> {
        let get_block_height_url = format!(
            "{}blocks/height/{}",
            self.url().as_str(),
            block_id.encoded()
        );
        let rs = &self.get(&get_block_height_url).await?;
        Ok(JsonDeserializer::safe_to_int_from_field(rs, "height")? as u32)
    }

    pub async fn get_block_height_by_timestamp(&self, timestamp: u64) -> Result<u32> {
        let get_block_height_url = format!(
            "{}blocks/heightByTimestamp/{}",
            self.url().as_str(),
            timestamp
        );
        let rs = &self.get(&get_block_height_url).await?;
        Ok(JsonDeserializer::safe_to_int_from_field(rs, "height")? as u32)
    }

    pub async fn get_blocks_delay(
        &self,
        start_block_id: Base58String,
        block_num: u32,
    ) -> Result<u32> {
        let get_blocks_delay_url = format!(
            "{}blocks/delay/{}/{}",
            self.url().as_str(),
            start_block_id.encoded(),
            block_num
        );
        let rs = &self.get(&get_blocks_delay_url).await?;
        Ok(JsonDeserializer::safe_to_int_from_field(rs, "delay")? as u32)
    }

    pub async fn get_block_headers_at_height(&self, height: u32) -> Result<BlockHeaders> {
        let get_block_headers_url = format!("{}blocks/headers/at/{}", self.url().as_str(), height);
        let rs = &self.get(&get_block_headers_url).await?;
        rs.try_into()
    }

    pub async fn get_block_headers_by_id(&self, block_id: Base58String) -> Result<BlockHeaders> {
        let get_block_headers_url = format!(
            "{}blocks/headers/{}",
            self.url().as_str(),
            block_id.encoded()
        );
        let rs = &self.get(&get_block_headers_url).await?;
        rs.try_into()
    }

    pub async fn get_blocks_headers_seq(
        &self,
        from_height: u32,
        to_height: u32,
    ) -> Result<Vec<BlockHeaders>> {
        let get_blocks_headers_seq_url = format!(
            "{}blocks/headers/seq/{}/{}",
            self.url().as_str(),
            from_height,
            to_height
        );
        let rs = &self.get(&get_blocks_headers_seq_url).await?;
        JsonDeserializer::safe_to_array(rs)?
            .iter()
            .map(|block| block.try_into())
            .collect()
    }

    pub async fn get_last_block_headers(&self) -> Result<BlockHeaders> {
        let get_last_block_headers_url = format!("{}blocks/headers/last", self.url().as_str());
        let rs = &self.get(&get_last_block_headers_url).await?;
        rs.try_into()
    }

    pub async fn get_block_at_height(&self, height: u32) -> Result<Block> {
        let get_block_at_height_url = format!("{}blocks/at/{}", self.url().as_str(), height);
        let rs = &self.get(&get_block_at_height_url).await?;
        rs.try_into()
    }

    pub async fn get_block_by_id(&self, block_id: Base58String) -> Result<Block> {
        let get_block_by_id_url = format!("{}blocks/{}", self.url().as_str(), block_id.encoded());
        let rs = &self.get(&get_block_by_id_url).await?;
        rs.try_into()
    }

    pub async fn get_blocks(&self, from_height: u32, to_height: u32) -> Result<Vec<Block>> {
        let get_blocks_url = format!(
            "{}blocks/seq/{}/{}",
            self.url().as_str(),
            from_height,
            to_height
        );
        let rs = &self.get(&get_blocks_url).await?;
        JsonDeserializer::safe_to_array(rs)?
            .iter()
            .map(|block| block.try_into())
            .collect()
    }

    pub async fn get_last_block(&self) -> Result<Block> {
        let get_last_block_url = format!("{}blocks/last", self.url().as_str());
        let rs = &self.get(&get_last_block_url).await?;
        rs.try_into()
    }

    pub async fn get_blocks_by_generator(
        &self,
        generator: Address,
        from_height: u32,
        to_height: u32,
    ) -> Result<Vec<Block>> {
        let get_blocks_by_generator_url = format!(
            "{}blocks/address/{}/{}/{}",
            self.url().as_str(),
            generator.encoded(),
            from_height,
            to_height
        );
        let rs = &self.get(&get_blocks_by_generator_url).await?;
        JsonDeserializer::safe_to_array(rs)?
            .iter()
            .map(|block| block.try_into())
            .collect()
    }

    pub async fn get_transaction_info(
        &self,
        transaction_id: &str,
    ) -> Result<TransactionInfoResponse> {
        let get_tx_info_url = format!(
            "{}transactions/info/{}",
            self.url().as_str(),
            transaction_id
        );
        let rs = &self.get(&get_tx_info_url).await?;
        rs.try_into()
    }

    pub async fn get_assets_balance(&self, address: &Address) -> Result<AssetsBalanceResponse> {
        let url = format!(
            "{}assets/balance/{}",
            self.url().as_str(),
            address.encoded()
        );
        let rs = self.get(&url).await?;
        rs.try_into()
    }

    pub async fn broadcast(&self, signed_tx: &SignedTransaction) -> Result<SignedTransaction> {
        let broadcast_tx_url = format!("{}transactions/broadcast", self.url().as_str());
        let rs = &self.post(&broadcast_tx_url, &signed_tx.to_json()?).await?;
        rs.try_into()
    }

    async fn get(&self, url: &str) -> Result<Value> {
        let response = self.http_client.get(url).send().await?;
        let rs = response.json().await?;
        Self::error_check(&rs)?;
        Ok(rs)
    }

    async fn post(&self, url: &str, body: &Value) -> Result<Value> {
        let response = self.http_client.post(url).json(body).send().await?;
        let rs = response.json().await?;
        Self::error_check(&rs)?;
        Ok(rs)
    }

    fn error_check(rs: &Value) -> Result<()> {
        let error = rs["error"].as_i64();
        if let Some(err) = error {
            let message = rs["message"].as_str().unwrap_or("");
            return Err(Error::NodeError {
                error: err as u32,
                message: message.to_owned(),
            });
        }
        Ok(())
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
        Url::from_str(url).expect("Invalid url")
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

    use crate::api::node::{Node, Profile};
    use crate::model::data_entry::DataEntry;
    use crate::model::{ApplicationStatus, ByteString, ChainId};

    #[tokio::test]
    async fn test_get_transfer_transaction_info() {
        let tx_id = "8YsBZSZ3UmWAo8bCj8RN64BvoQUTdLtd567hXqQCYDVo";

        let node = Node::from_profile(Profile::MAINNET);
        let transaction_info = node
            .get_transaction_info(tx_id)
            .await
            .expect("failed to get transaction info");

        assert_eq!(
            transaction_info.id().encoded(),
            "8YsBZSZ3UmWAo8bCj8RN64BvoQUTdLtd567hXqQCYDVo"
        );
        assert_eq!(transaction_info.status(), ApplicationStatus::Succeed);
        assert_eq!(transaction_info.height(), 3229634);

        let proof_from_rs = "4NiakymjU9s7mJYTBGbweGrDDwAauEXsuhMCeQJD1S28cEFL7hpjEL2LhaiVyFScq8UGVucpvCBo8PogvHQCdhrZ";
        assert_eq!(transaction_info.proofs()[0].encoded(), proof_from_rs);

        assert_eq!(transaction_info.timestamp(), 1659278184707);
        assert_eq!(transaction_info.fee().value(), 100000);
        assert_eq!(transaction_info.fee().asset_id(), None);
        assert_eq!(
            transaction_info
                .public_key()
                .address(MAINNET.byte())
                .expect("failed to get address from public key")
                .encoded(),
            "3P4eeU7v1LMHQFwwT2GW9W99c6vZyytHajj"
        );
        assert_eq!(
            transaction_info.public_key().encoded(),
            "AdZiupVsS9PMbTQK7iePWmD4Y5s8ZF6PoaQFyHKV2anj"
        );
        assert_eq!(transaction_info.tx_type(), 4);
        assert_eq!(transaction_info.version(), 1);

        let data_info = transaction_info.data();
        let transfer_transaction = data_info
            .transfer_tx()
            .expect("failed to get transfer transaction");
        assert_eq!(transfer_transaction.attachment().encoded(), "".to_owned());
        assert_eq!(
            transfer_transaction.recipient().encoded(),
            "3PHey9P6xpUubQqP7DgMeWaza41yWQGGbHK"
        );
        assert_eq!(transfer_transaction.amount().asset_id(), None);
        assert_eq!(transfer_transaction.amount().value(), 46095972);
    }

    #[tokio::test]
    async fn test_get_data_transaction_info() {
        let tx_id = "HcPcSma7oWeqy8g3ahhwFDzrq8YK8r739U4WC2ieB5Bs";

        let node = Node::from_profile(Profile::MAINNET);
        let transaction_info = node
            .get_transaction_info(tx_id)
            .await
            .expect("failed to get transaction info");

        assert_eq!(
            transaction_info.id().encoded(),
            "HcPcSma7oWeqy8g3ahhwFDzrq8YK8r739U4WC2ieB5Bs"
        );
        assert_eq!(transaction_info.status(), ApplicationStatus::Succeed);
        assert_eq!(transaction_info.height(), 3258212);

        let proof_from_rs = "25KiXB1FS3FaupiPXyEVeRquKLK4FEb3NWF36D1eHw1gpT9Y53MbLsVqnX9rJC8MPg4x9yiUxFkmxF9DDTgQruhi";
        assert_eq!(transaction_info.proofs()[0].encoded(), proof_from_rs);

        assert_eq!(transaction_info.timestamp(), 1660994483097);
        assert_eq!(transaction_info.fee().value(), 500000);
        assert_eq!(transaction_info.fee().asset_id(), None);
        assert_eq!(
            transaction_info
                .public_key()
                .address(MAINNET.byte())
                .expect("failed to get address from public key")
                .encoded(),
            "3P4sxdNNPJLQcitAnLqLfSwaenjxFxQvZsE"
        );
        assert_eq!(
            transaction_info.public_key().encoded(),
            "GTr2dXt3mxaD8tXGyNauV8YMy1hsSoi63DUuk4uyijqG"
        );
        assert_eq!(transaction_info.tx_type(), 12);
        assert_eq!(transaction_info.version(), 1);

        let data_info = transaction_info.data();
        let data_transaction = data_info
            .data_tx()
            .expect("failed to get data transaction from string");

        let data_entries = data_transaction.data();

        match data_entries[0].clone() {
            DataEntry::IntegerEntry { key, value } => {
                assert_eq!(key, "price_ausdtlpm_20220820");
                assert_eq!(value, 1823153_i64)
            }
            _ => panic!("failed"),
        };

        match data_entries[1].clone() {
            DataEntry::IntegerEntry { key, value } => {
                assert_eq!(key, "lastHeight_ausdtlpm");
                assert_eq!(value, 3258212_i64)
            }
            _ => panic!("failed"),
        }
    }
}
