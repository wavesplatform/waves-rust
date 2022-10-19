use regex::Regex;
use std::borrow::Borrow;

use std::str::FromStr;
use std::time::Duration;

use crate::error::{Error, Result};
use reqwest::{Client, Url};
use serde_json::Value::Array;
use serde_json::{Map, Value};
use tokio::time;

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
    /// Creates node structure from [Profile]
    /// to interact with Waves node through REST-API
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    ///
    /// #[tokio::main]
    /// async fn main()  {
    ///     let  node = Node::from_profile(Profile::TESTNET);
    ///     let  addresses = node.get_addresses().await.unwrap();
    ///     println!("{:?}", addresses);
    /// }
    /// ```
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

    /// Creates node structure from [Url]
    /// to interact with Waves node through REST-API
    /// ```no_run
    /// use url::Url;
    /// use waves_rust::api::Node;
    /// use waves_rust::model::ChainId;
    ///
    /// #[tokio::main]
    /// async fn main()  {
    ///     let url = Url::parse("https://nodes-testnet.wavesnodes.com").unwrap();
    ///     let  node = Node::from_url(url, ChainId::TESTNET.byte());
    ///     let  addresses = node.get_addresses().await.unwrap();
    ///     println!("{:?}", addresses);
    /// }
    /// ```
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

    /// Get a list of account addresses in the node wallet
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    ///
    /// #[tokio::main]
    /// async fn main()  {
    ///     let  node = Node::from_profile(Profile::TESTNET);
    ///     let  addresses = node.get_addresses().await.unwrap();
    ///     println!("{:?}", addresses);
    /// }
    /// ```
    pub async fn get_addresses(&self) -> Result<Vec<Address>> {
        let get_addresses_url = format!("{}addresses", self.url().as_str());
        let rs = self.get(&get_addresses_url).await?;
        JsonDeserializer::safe_to_array(&rs)?
            .iter()
            .map(|address| address.try_into())
            .collect()
    }

    /// Get a list of account addresses in the node wallet
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let addresses = node.get_addresses_seq(0, 1).await.unwrap();
    ///     println!("{:?}", addresses);
    /// }
    /// ```
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

    /// Get the regular balance in WAVES at a given address
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let  address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    ///     let balance = node.get_balance(&address).await.unwrap();
    ///     println!("{}", balance);
    /// }
    /// ```
    pub async fn get_balance(&self, address: &Address) -> Result<u64> {
        let get_balance_url = format!(
            "{}addresses/balance/{}",
            self.url().as_str(),
            address.encoded(),
        );
        let rs = self.get(&get_balance_url).await?;
        Ok(JsonDeserializer::safe_to_int_from_field(&rs, "balance")? as u64)
    }

    /// Get the minimum regular balance at a given address for confirmations blocks back from
    /// the current height
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    ///     let balance = node.get_balance_with_confirmations(&address, 100).await.unwrap();
    ///     println!("{}", balance);
    /// }
    /// ```
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

    /// Get regular balances for multiple addresses
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let address1 = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    ///     let address2 = Address::from_string("3N2yqTEKArWS3ySs2f6t8fpXdjX6cpPuhG8").unwrap();
    ///     let balances = node.get_balances(&[address1, address2]).await.unwrap();
    ///     println!("{}", balances);
    /// }
    /// ```
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

    /// Get regular balances for multiple addresses at the given height
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let height = node.get_height().await.unwrap();
    ///     let address1 = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    ///     let address2 = Address::from_string("3N2yqTEKArWS3ySs2f6t8fpXdjX6cpPuhG8").unwrap();
    ///     let balances = node.get_balances_at_height(&[address1, address2], height - 10).await.unwrap();
    ///     println!("{}", balances);
    /// }
    /// ```
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

    /// Get the available, regular, generating, and effective balance
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    ///     let balance_details = node.get_balance_details(&address).await.unwrap();
    ///     println!("{:?}", balance_details);
    /// }
    /// ```
    pub async fn get_balance_details(&self, address: &Address) -> Result<BalanceDetails> {
        let get_balance_details_url = format!(
            "{}addresses/balance/details/{}",
            self.url().as_str(),
            address.encoded()
        );
        let rs = &self.get(&get_balance_details_url).await?;
        rs.try_into()
    }

    /// Read account data entries
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    ///     let account_data = node.get_data(&address).await.unwrap();
    ///     println!("{:?}", account_data);
    /// }
    /// ```
    pub async fn get_data(&self, address: &Address) -> Result<Vec<DataEntry>> {
        let get_data_url = format!(
            "{}addresses/data/{}",
            self.url().as_str(),
            address.encoded()
        );
        let rs = self.get(&get_data_url).await?;

        JsonDeserializer::deserialize_data_array(&rs)
    }

    /// Read account data entries by given keys
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    ///     let account_data = node.get_data_by_keys(&address, &["bool", "int"]).await.unwrap();
    ///     println!("{:?}", account_data);
    /// }
    /// ```
    pub async fn get_data_by_keys(
        &self,
        address: &Address,
        keys: &[&str],
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

    /// Read account data entries by given regular expression
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    /// use regex::Regex;
    ///
    /// #[tokio::main]
    /// async fn main() {  
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    ///     let regex = Regex::new(r"b\w+").unwrap();
    ///     let account_data = node.get_data_by_regex(&address, &regex).await.unwrap();
    ///     println!("{:?}", account_data);
    /// }
    /// ```
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

    /// Read account data entry by given key
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {  
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    ///     let account_data = node.get_data_by_key(&address, "int").await.unwrap();
    ///     println!("{:?}", account_data);
    /// }
    /// ```
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

    /// Get an account script or a dApp script with additional info by a given address
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {  
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3Mv1HwsRtMjyGKSe5DSDnbT2AoTsXAjtwZS").unwrap();
    ///     let script_info = node.get_script_info(&address).await.unwrap();
    ///     println!("{:?}", script_info);
    /// }
    /// ```
    pub async fn get_script_info(&self, address: &Address) -> Result<ScriptInfo> {
        let get_script_info_url = format!(
            "{}addresses/scriptInfo/{}",
            self.url().as_str(),
            address.encoded()
        );
        let rs = &self.get(&get_script_info_url).await?;
        rs.try_into()
    }

    /// Get an account script meta
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {  
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3Mv1HwsRtMjyGKSe5DSDnbT2AoTsXAjtwZS").unwrap();
    ///     let script_meta = node.get_script_meta(&address).await.unwrap();
    ///     println!("{:?}", script_meta);
    /// }
    /// ```
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
    /// Get a list of aliases associated with a given address
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {  
    /// let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3MxtrLkrbcG28uTvmbKmhrwGrR65ooHVYvK").unwrap();
    ///     let aliases = node.get_aliases_by_address(&address).await.unwrap();
    ///     println!("{:?}", aliases);
    /// }
    /// ```
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

    /// Get an address associated with a given alias. Alias should be plain text without an 'alias'
    /// prefix and chain ID
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{Alias, ChainId};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let alias = Alias::new(ChainId::TESTNET.byte(), "alias1662650000377").unwrap();
    ///     let address = node.get_address_by_alias(&alias).await.unwrap();
    ///     println!("{:?}", address);
    /// }
    /// ```
    pub async fn get_address_by_alias(&self, alias: &Alias) -> Result<Address> {
        let get_address_by_alias_url =
            format!("{}alias/by-alias/{}", self.url().as_str(), alias.name());
        let rs = &self.get(&get_address_by_alias_url).await?;
        Address::from_string(&JsonDeserializer::safe_to_string_from_field(rs, "address")?)
    }

    // ASSETS

    /// Get asset balance distribution by addresses at a given height
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{Address, AssetId, ChainId};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let height = node.get_height().await.unwrap();
    ///     let asset_id = AssetId::from_string("DG2xFkPdDwKUoBkzGAhQtLpSGzfXLiCYPEzeKH2Ad24p").unwrap();
    ///     let after = Address::from_string("3P2iT1nawotR2QWmjfMAm18xytUiK6cWtHt").unwrap();
    ///     let address = node.get_asset_distribution(&asset_id, height, 10, Some(after)).await.unwrap();
    ///     println!("{:?}", address);
    /// }
    /// ```
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

    /// Get account balances in all or specified assets (excluding WAVES) at a given address.
    /// Note: Full portfolio also excludes NFTs.
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{Address};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3MxtrLkrbcG28uTvmbKmhrwGrR65ooHVYvK").unwrap();
    ///     let assets_balance = node.get_assets_balance(&address).await.unwrap();
    ///     println!("{:?}", assets_balance);
    /// }
    /// ```
    pub async fn get_assets_balance(&self, address: &Address) -> Result<AssetsBalanceResponse> {
        let url = format!(
            "{}assets/balance/{}",
            self.url().as_str(),
            address.encoded()
        );
        let rs = &self.get(&url).await?;
        rs.try_into()
    }

    /// Get account balances in all or specified assets (excluding WAVES) at a given address.
    /// Note: Full portfolio also excludes NFTs.
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{Address};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     use waves_rust::model::AssetId;
    /// let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3MxtrLkrbcG28uTvmbKmhrwGrR65ooHVYvK").unwrap();
    ///     let asset_id = AssetId::from_string("8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6").unwrap();
    ///     let asset_balance = node.get_asset_balance(&address, &asset_id).await.unwrap();
    ///     println!("{:?}", asset_balance);
    /// }
    /// ```
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

    /// Get detailed information about given asset
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::AssetId;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let asset_id = AssetId::from_string("8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6").unwrap();
    ///     let asset_details = node.get_asset_details(&asset_id).await.unwrap();
    ///     println!("{:?}", asset_details);
    /// }
    /// ```
    pub async fn get_asset_details(&self, asset_id: &AssetId) -> Result<AssetDetails> {
        let url = format!(
            "{}assets/details/{}?full=true",
            self.url().as_str(),
            asset_id.encoded()
        );
        let rs = &self.get(&url).await?;
        rs.try_into()
    }

    /// Get detailed information about given assets
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::AssetId;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let asset_id1 = AssetId::from_string("8bt2MZjuUCJPmfucPfaZPTXqrxmoCHCC8gVnbjZ7bhH6").unwrap();
    ///     let asset_id2 = AssetId::from_string("973uk5Fbg5eLF8cZg2b2iKsVSoHepdJXRtCuhWcM6MsR").unwrap();
    ///     let assets_details = node.get_assets_details(&[asset_id1, asset_id2]).await.unwrap();
    ///     println!("{:?}", assets_details);
    /// }
    /// ```
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

    /// Get a list of non-fungible tokens at a given address. Max for 1000 tokens.
    /// For pagination, use the parameter {after}
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{AssetId, Address};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::MAINNET);
    ///     let address = Address::from_string("3PAETTtuW7aSiyKtn9GuML3RgtV1xdq1mQW").unwrap();
    ///     let after = AssetId::from_string("13PtvhAC28kNXXJP3Evgcba5mNMsCAQECUqCPBu5wJou").unwrap();
    ///     let nfts = node.get_nft(&address, 10, Some(after)).await.unwrap();
    ///     println!("{:?}", nfts);
    /// }
    /// ```
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

    /// Get current status of block reward
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{AssetId, Address};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let rewards = node.get_blockchain_rewards().await.unwrap();
    ///     println!("{:?}", rewards);
    /// }
    /// ```
    pub async fn get_blockchain_rewards(&self) -> Result<BlockchainRewards> {
        let get_blockchain_rewards_url = format!("{}blockchain/rewards", self.url().as_str());
        let rs = &self.get(&get_blockchain_rewards_url).await?;
        rs.try_into()
    }

    /// Get status of block reward at height
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{AssetId, Address};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let current_height = node.get_height().await.unwrap();
    ///     let rewards = node.get_blockchain_rewards_at_height(current_height - 10).await.unwrap();
    ///     println!("{:?}", rewards);
    /// }
    /// ```
    pub async fn get_blockchain_rewards_at_height(&self, height: u32) -> Result<BlockchainRewards> {
        let get_blockchain_rewards_url =
            format!("{}blockchain/rewards/{}", self.url().as_str(), height);
        let rs = &self.get(&get_blockchain_rewards_url).await?;
        rs.try_into()
    }

    // BLOCKS

    /// Get the current blockchain height
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{AssetId, Address};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let current_height = node.get_height().await.unwrap();
    ///     println!("{:?}", current_height);
    /// }
    /// ```
    pub async fn get_height(&self) -> Result<u32> {
        let get_height_url = format!("{}blocks/height", self.url().as_str());
        let rs = &self.get(&get_height_url).await?;
        Ok(JsonDeserializer::safe_to_int_from_field(rs, "height")? as u32)
    }

    /// Get the height of a block by its ID
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{AssetId, Address, Base58String};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let block_id = Base58String::from_string("oReBHRjMcUKqZxH6iVhthxQ72QndBFtfLHngV8aGW9y").unwrap();
    ///     let height = node.get_block_height_by_id(&block_id).await.unwrap();
    ///     println!("{:?}", height);
    /// }
    /// ```
    pub async fn get_block_height_by_id(&self, block_id: &Base58String) -> Result<u32> {
        let get_block_height_url = format!(
            "{}blocks/height/{}",
            self.url().as_str(),
            block_id.encoded()
        );
        let rs = &self.get(&get_block_height_url).await?;
        Ok(JsonDeserializer::safe_to_int_from_field(rs, "height")? as u32)
    }

    /// Get height of the most recent block such that its timestamp does not exceed the given {timestamp}
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{AssetId, Address};
    /// use waves_rust::util::get_current_epoch_millis;
    ///
    /// #[tokio::main]
    /// async fn main() {    
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let now = get_current_epoch_millis();
    ///     let height = node.get_block_height_by_timestamp(now - 10_000).await.unwrap();
    ///     println!("{:?}", height);
    /// }
    /// ```
    pub async fn get_block_height_by_timestamp(&self, timestamp: u64) -> Result<u32> {
        let get_block_height_url = format!(
            "{}blocks/heightByTimestamp/{}",
            self.url().as_str(),
            timestamp
        );
        let rs = &self.get(&get_block_height_url).await?;
        Ok(JsonDeserializer::safe_to_int_from_field(rs, "height")? as u32)
    }

    /// Average delay in milliseconds between last {block_num} blocks starting from block with {start_block_id}}
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Base58String;
    ///
    /// #[tokio::main]
    /// async fn main() {    
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let block_id = Base58String::from_string("oReBHRjMcUKqZxH6iVhthxQ72QndBFtfLHngV8aGW9y").unwrap();
    ///     let blocks_delay = node.get_blocks_delay(&block_id, 10).await.unwrap();
    ///     println!("{:?}", blocks_delay);
    /// }
    /// ```
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

    /// Get height of the most recent block such that its timestamp does not exceed the given {timestamp}
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    ///
    /// #[tokio::main]
    /// async fn main() {    
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let height = node.get_height().await.unwrap();
    ///     let block_headers = node.get_block_headers_at_height(height).await.unwrap();
    ///     println!("{:?}", block_headers);
    /// }
    /// ```
    pub async fn get_block_headers_at_height(&self, height: u32) -> Result<BlockHeaders> {
        let get_block_headers_url = format!("{}blocks/headers/at/{}", self.url().as_str(), height);
        let rs = &self.get(&get_block_headers_url).await?;
        rs.try_into()
    }

    /// Get headers of a given block
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Base58String;
    ///
    /// #[tokio::main]
    /// async fn main() {    
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let block_id = Base58String::from_string("oReBHRjMcUKqZxH6iVhthxQ72QndBFtfLHngV8aGW9y").unwrap();
    ///     let block_headers = node.get_block_headers_by_id(&block_id).await.unwrap();
    ///     println!("{:?}", block_headers);
    /// }
    /// ```
    pub async fn get_block_headers_by_id(&self, block_id: &Base58String) -> Result<BlockHeaders> {
        let get_block_headers_url = format!(
            "{}blocks/headers/{}",
            self.url().as_str(),
            block_id.encoded()
        );
        let rs = &self.get(&get_block_headers_url).await?;
        rs.try_into()
    }

    /// Get block headers at a given range of heights. Max range {from_height}-{to_height} is 100 blocks
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    ///
    /// #[tokio::main]
    /// async fn main() {    
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let to_height = node.get_height().await.unwrap();
    ///     let from_height = to_height.clone() - 5;
    ///     let block_headers = node.get_blocks_headers_seq(from_height, to_height).await.unwrap();
    ///     println!("{:?}", block_headers);
    /// }
    /// ```
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

    /// Get headers of the block at the current blockchain height
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    ///
    /// #[tokio::main]
    /// async fn main() {    
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let block_headers = node.get_last_block_headers().await.unwrap();
    ///     println!("{:?}", block_headers);
    /// }
    /// ```
    pub async fn get_last_block_headers(&self) -> Result<BlockHeaders> {
        let get_last_block_headers_url = format!("{}blocks/headers/last", self.url().as_str());
        let rs = &self.get(&get_last_block_headers_url).await?;
        rs.try_into()
    }

    /// Get a block at a given height
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    ///
    /// #[tokio::main]
    /// async fn main() {    
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let current_height = node.get_height().await.unwrap();
    ///     let block = node.get_block_at_height(current_height).await.unwrap();
    ///     println!("{:?}", block);
    /// }
    /// ```
    pub async fn get_block_at_height(&self, height: u32) -> Result<Block> {
        let get_block_at_height_url = format!("{}blocks/at/{}", self.url().as_str(), height);
        let rs = &self.get(&get_block_at_height_url).await?;
        rs.try_into()
    }

    /// Get a block by its ID
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Base58String;
    ///
    /// #[tokio::main]
    /// async fn main() {    
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let block_id = Base58String::from_string("oReBHRjMcUKqZxH6iVhthxQ72QndBFtfLHngV8aGW9y").unwrap();
    ///     let block = node.get_block_by_id(&block_id).await.unwrap();
    ///     println!("{:?}", block);
    /// }
    /// ```
    pub async fn get_block_by_id(&self, block_id: &Base58String) -> Result<Block> {
        let get_block_by_id_url = format!("{}blocks/{}", self.url().as_str(), block_id.encoded());
        let rs = &self.get(&get_block_by_id_url).await?;
        rs.try_into()
    }

    /// Get blocks at a given range of heights. Max range is 100 blocks
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    ///
    /// #[tokio::main]
    /// async fn main() {    
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let to_height = node.get_height().await.unwrap();
    ///     let from_height = to_height.clone() - 5;
    ///     let blocks = node.get_blocks(from_height, to_height).await.unwrap();
    ///     println!("{:?}", blocks);
    /// }
    /// ```
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

    /// Get the block at the current blockchain height
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    ///
    /// #[tokio::main]
    /// async fn main() {    
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let block = node.get_last_block().await.unwrap();
    ///     println!("{:?}", block);
    /// }
    /// ```
    pub async fn get_last_block(&self) -> Result<Block> {
        let get_last_block_url = format!("{}blocks/last", self.url().as_str());
        let rs = &self.get(&get_last_block_url).await?;
        rs.try_into()
    }

    /// Get a list of blocks forged by a given address. Max range is 100 blocks
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let generator = Address::from_string("3Mxv6Dpa1qRuyQBRFg3GwUaf3rcjHqWwNmC").unwrap();
    ///     let to_height = node.get_height().await.unwrap();
    ///     let from_height = to_height.clone() - 5;
    ///     let blocks = node.get_blocks_by_generator(&generator, from_height, to_height).await.unwrap();
    ///     println!("{:?}", blocks);
    /// }
    /// ```
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
    /// Get Waves node version
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let version = node.get_version().await.unwrap();
    ///     println!("{:?}", version);
    /// }
    /// ```
    pub async fn get_version(&self) -> Result<String> {
        let get_version_url = format!("{}node/version", self.url().as_str());
        let rs = &self.get(&get_version_url).await?;
        JsonDeserializer::safe_to_string_from_field(rs, "version")
    }

    // DEBUG
    /// Get history of the regular balance at a given address
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3Mxv6Dpa1qRuyQBRFg3GwUaf3rcjHqWwNmC").unwrap();
    ///     let history = node.get_balance_history(&address).await.unwrap();
    ///     println!("{:?}", history);
    /// }
    /// ```
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

    /// Validates a transaction and measures time spent in milliseconds
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{Address,Amount, Base58String, ChainId, Transaction, TransactionData,
    /// TransferTransaction, PrivateKey};
    /// use waves_rust::util::{Crypto, get_current_epoch_millis};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let  private_key = PrivateKey::from_seed(&Crypto::get_random_seed_phrase(12), 0).unwrap();
    ///     let signed_tx = Transaction::new(
    ///         TransactionData::Transfer(TransferTransaction::new(
    ///             Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q")?,
    ///             Amount::new(100, None),
    ///             Base58String::empty(),
    ///         )),
    ///         Amount::new(100000, None),
    ///         get_current_epoch_millis(),
    ///         private_key.public_key(),
    ///         3,
    ///         ChainId::TESTNET.byte(),
    ///     )
    ///     .sign(&private_key).unwrap();
    ///     let validation = node.validate_transaction(&signed_tx).await.unwrap();
    ///     println!("{:?}", validation);
    /// }
    /// ```
    pub async fn validate_transaction(&self, signed_tx: &SignedTransaction) -> Result<Validation> {
        let validate_url = format!("{}debug/validate", self.url().as_str());
        let rs = &self.post(&validate_url, &signed_tx.to_json()?).await?;
        rs.try_into()
    }

    // LEASING

    /// Get all active leases involving a given address
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Address;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3Mxv6Dpa1qRuyQBRFg3GwUaf3rcjHqWwNmC").unwrap();
    ///     let active_leases = node.get_active_leases(&address).await.unwrap();
    ///     println!("{:?}", active_leases);
    /// }
    /// ```
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

    /// Get lease parameters by lease ID
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{Address, Id};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let lease_id = Id::from_string("BiJR8gCxR7crGEdy31jLkYpjpLy98kq3NuxPE8Z2Uk3b").unwrap();
    ///     let lease_info = node.get_lease_info(&lease_id).await.unwrap();
    ///     println!("{:?}", lease_info);
    /// }
    /// ```
    pub async fn get_lease_info(&self, lease_id: &Id) -> Result<LeaseInfo> {
        let get_lease_info_url =
            format!("{}leasing/info/{}", self.url().as_str(), lease_id.encoded());
        let rs = &self.get(&get_lease_info_url).await?;
        rs.try_into()
    }

    /// Get lease parameters by lease IDs
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{Address, Id};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let lease_id = Id::from_string("BiJR8gCxR7crGEdy31jLkYpjpLy98kq3NuxPE8Z2Uk3b").unwrap();
    ///     let leases_info = node.get_leases_info(&[lease_id]).await.unwrap();
    ///     println!("{:?}", leases_info);
    /// }
    /// ```
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

    /// Get the minimum fee for a given transaction
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{Address,Amount, Base58String, ChainId, Transaction, TransactionData,
    /// TransferTransaction, PrivateKey};
    /// use waves_rust::util::{Crypto, get_current_epoch_millis};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let private_key = PrivateKey::from_seed(&Crypto::get_random_seed_phrase(12), 0).unwrap();
    ///     let signed_tx = Transaction::new(
    ///         TransactionData::Transfer(TransferTransaction::new(
    ///             Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q")?,
    ///             Amount::new(100, None),
    ///             Base58String::empty(),
    ///         )),
    ///         Amount::new(100000, None),
    ///         get_current_epoch_millis(),
    ///         private_key.public_key(),
    ///         3,
    ///         ChainId::TESTNET.byte(),
    ///     )
    ///     .sign(&private_key).unwrap();
    ///     let fee = node.calculate_transaction_fee(&signed_tx).await.unwrap();
    ///     println!("{:?}", fee);
    /// }
    /// ```
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

    /// Broadcast a signed transaction.
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{Address,Amount, Base58String, ChainId, Transaction, TransactionData,
    /// TransferTransaction, PrivateKey};
    /// use waves_rust::util::{Crypto, get_current_epoch_millis};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let private_key = PrivateKey::from_seed(&Crypto::get_random_seed_phrase(12), 0).unwrap();
    ///     let signed_tx = Transaction::new(
    ///         TransactionData::Transfer(TransferTransaction::new(
    ///             Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q")?,
    ///             Amount::new(100, None),
    ///             Base58String::empty(),
    ///         )),
    ///         Amount::new(100000, None),
    ///         get_current_epoch_millis(),
    ///         private_key.public_key(),
    ///         3,
    ///         ChainId::TESTNET.byte(),
    ///     )
    ///     .sign(&private_key).unwrap();
    ///     node.broadcast(&signed_tx).await.unwrap();
    /// }
    /// ```
    pub async fn broadcast(&self, signed_tx: &SignedTransaction) -> Result<SignedTransaction> {
        let broadcast_tx_url = format!("{}transactions/broadcast", self.url().as_str());
        let rs = &self.post(&broadcast_tx_url, &signed_tx.to_json()?).await?;
        rs.try_into()
    }

    /// Get a transaction by its ID
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{Address, Id};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let tx_id = Id::from_string("3kuZKAeyjcqavmezy86sWCAeXrgt3HBKa4HA8CZdT8nH").unwrap();
    ///     let tx_info = node.get_transaction_info(&tx_id).await.unwrap();
    ///     println!("{:?}", tx_info);
    /// }
    /// ```
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

    /// Get a list of the latest transactions involving a given address.
    /// For pagination, use the parameter {after_tx_id}
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::{Address, Id};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let address = Address::from_string("3Mq3pueXcAgLcuWvJzJ4ndRHfqYgjUZvL7q").unwrap();
    ///     let after_id = Some(Id::from_string(
    ///         "3p6ffM2uyseFWPRQUcXMpr3gBKkKgt7jVQ8iDGQhVpRa",
    ///     ).unwrap());
    ///     let txs_info = node.get_transactions_by_address(&address, 10, after_id).await.unwrap();
    ///     println!("{:?}", txs_info);
    /// }
    /// ```
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

    /// Get transaction status by its ID
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Id;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let id = Id::from_string("3p6ffM2uyseFWPRQUcXMpr3gBKkKgt7jVQ8iDGQhVpRa").unwrap();
    ///     let tx_status = node.get_transaction_status(&id).await.unwrap();
    ///     println!("{:?}", tx_status);
    /// }
    /// ```
    pub async fn get_transaction_status(&self, transaction_id: &Id) -> Result<TransactionStatus> {
        let get_tx_status_url = format!(
            "{}transactions/status?id={}",
            self.url().as_str(),
            transaction_id.encoded()
        );
        let rs = &self.get(&get_tx_status_url).await?;
        JsonDeserializer::safe_to_array(rs)?[0].borrow().try_into()
    }

    /// Get transaction statuses by their ID
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Id;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let id = Id::from_string("3p6ffM2uyseFWPRQUcXMpr3gBKkKgt7jVQ8iDGQhVpRa").unwrap();
    ///     let txs_statuses = node.get_transactions_statuses(&[id]).await.unwrap();
    ///     println!("{:?}", txs_statuses);
    /// }
    /// ```
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

    /// Get an unconfirmed transaction by its ID
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    /// use waves_rust::model::Id;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let id = Id::from_string("3p6ffM2uyseFWPRQUcXMpr3gBKkKgt7jVQ8iDGQhVpRa").unwrap();
    ///     let unconfirmed_tx = node.get_unconfirmed_transaction(&id).await.unwrap();
    ///     println!("{:?}", unconfirmed_tx);
    /// }
    /// ```
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

    /// Get a list of transactions in node's UTX pool
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let unconfirmed_txs = node.get_unconfirmed_transactions().await.unwrap();
    ///     println!("{:?}", unconfirmed_txs);
    /// }
    /// ```
    pub async fn get_unconfirmed_transactions(&self) -> Result<Vec<SignedTransaction>> {
        let get_unconfirmed_txs_url = format!("{}transactions/unconfirmed", self.url().as_str());
        let rs = &self.get(&get_unconfirmed_txs_url).await?;
        JsonDeserializer::safe_to_array(rs)?
            .iter()
            .map(|tx| tx.try_into())
            .collect()
    }

    /// Get the number of transactions in the UTX pool
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let utx_size = node.get_utx_size().await.unwrap();
    ///     println!("{:?}", utx_size);
    /// }
    /// ```
    pub async fn get_utx_size(&self) -> Result<u32> {
        let get_utx_size_url = format!("{}transactions/unconfirmed/size", self.url().as_str());
        let rs = &self.get(&get_utx_size_url).await?;
        Ok(JsonDeserializer::safe_to_int_from_field(rs, "size")? as u32)
    }

    // UTILS

    /// Compiles string code to base64 script representation
    /// ```no_run
    /// use waves_rust::api::{Node, Profile};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let node = Node::from_profile(Profile::TESTNET);
    ///     let script = "{-# CONTENT_TYPE EXPRESSION #-} sigVerify(tx.bodyBytes, tx.proofs[0], tx.senderPublicKey)";
    ///     let compiled_script = node.compile_script(script, true).await.unwrap();
    ///     println!("{:?}", compiled_script);
    /// }
    /// ```
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

    // WAITING
    pub async fn wait_for_transaction(
        &self,
        id: Id,
        polling_interval: Duration,
        timeout: Duration
    ) -> Result<()> {
        let mut interval = time::interval(polling_interval);
        let mut time_spent = Duration::from_millis(0);

        let mut last_error: Error = Error::NodeError {
            error: 0,
            message: "undefined".to_string()
        };

        while time_spent < timeout {
            match self.get_transaction_info(&id).await {
                Ok(_) => return Ok(()),
                Err(err) => last_error = err
            }
            interval.tick().await;
            time_spent += polling_interval;
        }
        Err(last_error)
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

    use crate::model::ChainId;

    #[test]
    fn test_create_node_from_profile() -> Result<()> {
        let _ = Node::from_profile(Profile::MAINNET);
        Ok(())
    }

    #[test]
    fn test_create_node_from_url() -> Result<()> {
        let url = Url::from_str("https://nodes.wavesnodes.com").expect("failed to parse url");
        let _ = Node::from_url(url, MAINNET.byte());
        Ok(())
    }
}
