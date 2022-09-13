use regex::Regex;
use std::borrow::Borrow;

use std::str::FromStr;
use std::time::Duration;

use crate::error::{Error, Result};
use reqwest::{Client, Url};
use serde_json::Value::Array;
use serde_json::{Map, Value};

use crate::model::account::{Address, Balance, BalanceDetails};
use crate::model::asset::asset_details::AssetDetails;
use crate::model::asset::asset_distribution::AssetDistribution;
use crate::model::asset::balance::AssetsBalanceResponse;
use crate::model::data_entry::DataEntry;
use crate::model::{
    Alias, AliasesByAddressResponse, Amount, AssetId, Base58String, Block, BlockHeaders,
    BlockchainRewards, ByteString, ChainId, HistoryBalance, Id, LeaseInfo, ScriptInfo, ScriptMeta,
    SignedTransaction, TransactionInfoResponse, TransactionStatus, Validation,
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

    pub fn from_url(url: Url, chain_id: u8) -> Node {
        Node {
            url,
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
        JsonDeserializer::safe_to_array(&rs)?
            .iter()
            .map(|address| address.try_into())
            .collect()
    }

    pub async fn get_addresses_seq(&self, from_index: u64, to_index: u64) -> Result<Vec<Address>> {
        let get_addresses_seq_url = format!(
            "{}addresses/seq/{}/{}",
            self.url().as_str(),
            from_index,
            to_index
        );
        let rs = self.get(&get_addresses_seq_url).await?;
        JsonDeserializer::safe_to_array(&rs)?
            .iter()
            .map(|address| address.try_into())
            .collect()
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
        let rs = &self.post(&get_balances_url, &json_addresses.into()).await?;
        JsonDeserializer::safe_to_array(rs)?
            .iter()
            .map(|balance| balance.try_into())
            .collect()
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
        let rs = &self.post(&get_balances_url, &json_addresses.into()).await?;
        JsonDeserializer::safe_to_array(rs)?
            .iter()
            .map(|balance| balance.try_into())
            .collect()
    }

    pub async fn get_balance_details(&self, address: &Address) -> Result<BalanceDetails> {
        let get_balance_details_url = format!(
            "{}addresses/balance/details/{}",
            self.url().as_str(),
            address.encoded()
        );
        let rs = &self.get(&get_balance_details_url).await?;
        rs.try_into()
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
        rs.try_into()
    }

    pub async fn get_script_meta(&self, address: &Address) -> Result<ScriptMeta> {
        let get_script_meta_url = format!(
            "{}addresses/scriptInfo/{}/meta",
            self.url().as_str(),
            address.encoded()
        );
        let rs = &self.get(&get_script_meta_url).await?;
        rs.try_into()
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

    // ASSETS

    pub async fn get_asset_distribution(
        &self,
        asset_id: &AssetId,
        height: u32,
        limit: u16,
        after: Option<Address>,
    ) -> Result<AssetDistribution> {
        let url = format!(
            "{}assets/{}/distribution/{}/limit/{}",
            self.url().as_str(),
            asset_id.encoded(),
            height,
            limit
        );
        let rs = match after {
            Some(after_address) => {
                let url_with_cursor = format!("{}?after={}", &url, after_address.encoded());
                self.get(&url_with_cursor).await?
            }
            None => self.get(&url).await?,
        };
        rs.borrow().try_into()
    }

    pub async fn get_assets_balance(&self, address: &Address) -> Result<AssetsBalanceResponse> {
        let url = format!(
            "{}assets/balance/{}",
            self.url().as_str(),
            address.encoded()
        );
        let rs = &self.get(&url).await?;
        rs.try_into()
    }

    pub async fn get_asset_balance(&self, address: &Address, asset_id: &AssetId) -> Result<u64> {
        let url = format!(
            "{}assets/balance/{}/{}",
            self.url().as_str(),
            address.encoded(),
            asset_id.encoded()
        );
        let rs = &self.get(&url).await?;
        Ok(JsonDeserializer::safe_to_int_from_field(rs, "balance")? as u64)
    }

    pub async fn get_asset_details(&self, asset_id: &AssetId) -> Result<AssetDetails> {
        let url = format!(
            "{}assets/details/{}?full=true",
            self.url().as_str(),
            asset_id.encoded()
        );
        let rs = &self.get(&url).await?;
        rs.try_into()
    }

    pub async fn get_assets_details(&self, asset_ids: &[AssetId]) -> Result<Vec<AssetDetails>> {
        let url = format!("{}assets/details", self.url().as_str());
        let mut ids: Map<String, Value> = Map::new();
        ids.insert(
            "ids".to_owned(),
            Array(
                asset_ids
                    .iter()
                    .map(|it| Value::String(it.encoded()))
                    .collect(),
            ),
        );
        let rs = &self.post(&url, &ids.into()).await?;
        JsonDeserializer::safe_to_array(rs)?
            .iter()
            .map(|asset| asset.try_into())
            .collect()
    }

    pub async fn get_nft(
        &self,
        address: &Address,
        limit: u16,
        after: Option<AssetId>,
    ) -> Result<Vec<AssetDetails>> {
        let url = format!(
            "{}assets/nft/{}/limit/{}",
            self.url().as_str(),
            address.encoded(),
            limit
        );
        let rs = match after {
            Some(after_id) => {
                let url_with_cursor = format!("{}?after={}", &url, after_id.encoded());
                self.get(&url_with_cursor).await?
            }
            None => self.get(&url).await?,
        };
        JsonDeserializer::safe_to_array(&rs)?
            .iter()
            .map(|asset| asset.try_into())
            .collect()
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

    pub async fn get_block_height_by_id(&self, block_id: &Base58String) -> Result<u32> {
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
        start_block_id: &Base58String,
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

    pub async fn get_block_headers_by_id(&self, block_id: &Base58String) -> Result<BlockHeaders> {
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

    pub async fn get_block_by_id(&self, block_id: &Base58String) -> Result<Block> {
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
        generator: &Address,
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

    // NODE
    pub async fn get_version(&self) -> Result<String> {
        let get_version_url = format!("{}node/version", self.url().as_str(),);
        let rs = &self.get(&get_version_url).await?;
        JsonDeserializer::safe_to_string_from_field(rs, "version")
    }

    // DEBUG
    pub async fn get_balance_history(&self, address: &Address) -> Result<Vec<HistoryBalance>> {
        let get_balance_history_url = format!(
            "{}debug/balances/history/{}",
            self.url().as_str(),
            address.encoded()
        );
        let rs = &self.get(&get_balance_history_url).await?;
        JsonDeserializer::safe_to_array(rs)?
            .iter()
            .map(|balance| balance.try_into())
            .collect()
    }

    pub async fn validate_transaction(&self, signed_tx: &SignedTransaction) -> Result<Validation> {
        let validate_url = format!("{}debug/validate", self.url().as_str());
        let rs = &self.post(&validate_url, &signed_tx.to_json()?).await?;
        rs.try_into()
    }

    // LEASING

    pub async fn get_active_leases(&self, address: &Address) -> Result<Vec<LeaseInfo>> {
        let get_active_leases_url = format!(
            "{}leasing/active/{}",
            self.url().as_str(),
            address.encoded()
        );
        let rs = &self.get(&get_active_leases_url).await?;
        JsonDeserializer::safe_to_array(rs)?
            .iter()
            .map(|lease| lease.try_into())
            .collect()
    }

    pub async fn get_lease_info(&self, lease_id: &Id) -> Result<LeaseInfo> {
        let get_lease_info_url =
            format!("{}leasing/info/{}", self.url().as_str(), lease_id.encoded());
        let rs = &self.get(&get_lease_info_url).await?;
        rs.try_into()
    }

    pub async fn get_leases_info(&self, lease_ids: &[Id]) -> Result<Vec<LeaseInfo>> {
        let get_leases_info_url = format!("{}leasing/info", self.url().as_str());
        let mut ids: Map<String, Value> = Map::new();
        ids.insert(
            "ids".to_owned(),
            Array(
                lease_ids
                    .iter()
                    .map(|it| Value::String(it.encoded()))
                    .collect(),
            ),
        );
        let rs = &self.post(&get_leases_info_url, &ids.into()).await?;
        JsonDeserializer::safe_to_array(rs)?
            .iter()
            .map(|lease| lease.try_into())
            .collect()
    }

    // TRANSACTIONS

    pub async fn calculate_transaction_fee(
        &self,
        transaction: &SignedTransaction,
    ) -> Result<Amount> {
        let get_lease_info_url = format!("{}transactions/calculateFee", self.url().as_str());
        let rs = &self
            .post(&get_lease_info_url, &transaction.to_json()?)
            .await?;
        Ok(Amount::new(
            JsonDeserializer::safe_to_int_from_field(rs, "feeAmount")? as u64,
            JsonDeserializer::asset_id_from_json(rs, "feeAssetId")?,
        ))
    }

    pub async fn broadcast(&self, signed_tx: &SignedTransaction) -> Result<SignedTransaction> {
        let broadcast_tx_url = format!("{}transactions/broadcast", self.url().as_str());
        let rs = &self.post(&broadcast_tx_url, &signed_tx.to_json()?).await?;
        rs.try_into()
    }

    pub async fn get_transaction_info(
        &self,
        transaction_id: &Id,
    ) -> Result<TransactionInfoResponse> {
        let get_tx_info_url = format!(
            "{}transactions/info/{}",
            self.url().as_str(),
            transaction_id.encoded()
        );
        let rs = &self.get(&get_tx_info_url).await?;
        rs.try_into()
    }

    pub async fn get_transactions_by_address(
        &self,
        address: &Address,
        limit: u16,
        after_tx_id: Option<Id>,
    ) -> Result<Vec<TransactionInfoResponse>> {
        let get_tx_info_by_address = format!(
            "{}transactions/address/{}/limit/{}",
            self.url().as_str(),
            address.encoded(),
            limit
        );
        let rs = match after_tx_id {
            Some(after_id) => {
                let url_with_cursor =
                    format!("{}?after={}", &get_tx_info_by_address, after_id.encoded());
                self.get(&url_with_cursor).await?
            }
            None => self.get(&get_tx_info_by_address).await?,
        };
        JsonDeserializer::safe_to_array(&JsonDeserializer::safe_to_array(&rs)?[0])?
            .iter()
            .map(|tx| tx.try_into())
            .collect()
    }

    pub async fn get_transaction_status(&self, transaction_id: &Id) -> Result<TransactionStatus> {
        let get_tx_status_url = format!(
            "{}transactions/status?id={}",
            self.url().as_str(),
            transaction_id.encoded()
        );
        let rs = &self.get(&get_tx_status_url).await?;
        JsonDeserializer::safe_to_array(rs)?[0].borrow().try_into()
    }

    pub async fn get_transactions_statuses(
        &self,
        transaction_ids: &[Id],
    ) -> Result<Vec<TransactionStatus>> {
        let get_tx_status_url = format!("{}transactions/status", self.url().as_str());
        let mut ids = Map::new();
        ids.insert(
            "ids".to_owned(),
            Array(
                transaction_ids
                    .iter()
                    .map(|id| Value::String(id.encoded()))
                    .collect(),
            ),
        );
        let rs = &self.post(&get_tx_status_url, &ids.into()).await?;
        JsonDeserializer::safe_to_array(rs)?
            .iter()
            .map(|status| status.try_into())
            .collect()
    }

    pub async fn get_unconfirmed_transaction(
        &self,
        transaction_id: &Id,
    ) -> Result<SignedTransaction> {
        let get_unconfirmed_tx_url = format!(
            "{}transactions/unconfirmed/info/{}",
            self.url().as_str(),
            transaction_id.encoded()
        );
        let rs = &self.get(&get_unconfirmed_tx_url).await?;
        rs.try_into()
    }

    pub async fn get_unconfirmed_transactions(&self) -> Result<Vec<SignedTransaction>> {
        let get_unconfirmed_txs_url = format!("{}transactions/unconfirmed", self.url().as_str());
        let rs = &self.get(&get_unconfirmed_txs_url).await?;
        JsonDeserializer::safe_to_array(rs)?
            .iter()
            .map(|tx| tx.try_into())
            .collect()
    }

    pub async fn get_utx_size(&self) -> Result<u32> {
        let get_utx_size_url = format!("{}transactions/unconfirmed/size", self.url().as_str());
        let rs = &self.get(&get_utx_size_url).await?;
        Ok(JsonDeserializer::safe_to_int_from_field(rs, "size")? as u32)
    }

    // UTILS

    pub async fn compile_script(
        &self,
        source: &str,
        enable_compaction: bool,
    ) -> Result<ScriptInfo> {
        let compile_script_url = format!(
            "{}utils/script/compileCode?compact={}",
            self.url().as_str(),
            enable_compaction
        );
        let rs = &self
            .post_plain_text(&compile_script_url, source.to_owned())
            .await?;
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

    async fn post_plain_text(&self, url: &str, body: String) -> Result<Value> {
        let response = self.http_client.post(url).body(body.clone()).send().await?;
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
    use std::str::FromStr;
    use url::Url;
    use ChainId::MAINNET;

    use crate::api::node::{Node, Profile};
    use crate::error::Result;
    use crate::model::data_entry::DataEntry;
    use crate::model::{ApplicationStatus, ByteString, ChainId, Id};

    #[tokio::test]
    async fn test_get_transfer_transaction_info() -> Result<()> {
        let node = Node::from_profile(Profile::MAINNET);
        let _ = node.get_addresses();
        Ok(())
    }

    #[tokio::test]
    async fn test_get_data_transaction_info() -> Result<()> {
        let url = Url::from_str("https://nodes.wavesnodes.com").expect("failed to parse url");
        let node = Node::from_url(url, MAINNET.byte());
        let _ = node.get_addresses();
        Ok(())
    }
}
