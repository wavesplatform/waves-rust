use crate::error::{Error, Result};
use crate::model::{Address, Base58String, SignedTransaction};
use crate::util::JsonDeserializer;
use serde_json::Value;
use std::borrow::Borrow;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BlockHeaders {
    version: u8,
    timestamp: u64,
    reference: Base58String,
    nxt_consensus: NxtConsensus,
    transactions_root: Base58String,
    id: Base58String,
    features: Vec<u32>,
    desired_reward: i64,
    generator: Address,
    signature: Base58String,
    blocksize: u32,
    transaction_count: u32,
    height: u32,
    total_fee: u64,
    reward: u64,
    vrf: Base58String,
}

#[allow(clippy::too_many_arguments)]
impl BlockHeaders {
    pub fn new(
        version: u8,
        timestamp: u64,
        reference: Base58String,
        nxt_consensus: NxtConsensus,
        transactions_root: Base58String,
        id: Base58String,
        features: Vec<u32>,
        desired_reward: i64,
        generator: Address,
        signature: Base58String,
        blocksize: u32,
        transaction_count: u32,
        height: u32,
        total_fee: u64,
        reward: u64,
        vrf: Base58String,
    ) -> Self {
        Self {
            version,
            timestamp,
            reference,
            nxt_consensus,
            transactions_root,
            id,
            features,
            desired_reward,
            generator,
            signature,
            blocksize,
            transaction_count,
            height,
            total_fee,
            reward,
            vrf,
        }
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn reference(&self) -> Base58String {
        self.reference.clone()
    }

    pub fn nxt_consensus(&self) -> NxtConsensus {
        self.nxt_consensus.clone()
    }

    pub fn transactions_root(&self) -> Base58String {
        self.transactions_root.clone()
    }

    pub fn id(&self) -> Base58String {
        self.id.clone()
    }

    pub fn features(&self) -> Vec<u32> {
        self.features.clone()
    }

    pub fn desired_reward(&self) -> i64 {
        self.desired_reward
    }

    pub fn generator(&self) -> Address {
        self.generator.clone()
    }

    pub fn signature(&self) -> Base58String {
        self.signature.clone()
    }

    pub fn blocksize(&self) -> u32 {
        self.blocksize
    }

    pub fn transaction_count(&self) -> u32 {
        self.transaction_count
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn total_fee(&self) -> u64 {
        self.total_fee
    }

    pub fn reward(&self) -> u64 {
        self.reward
    }

    pub fn vrf(&self) -> Base58String {
        self.vrf.clone()
    }
}

impl TryFrom<&Value> for BlockHeaders {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let version = JsonDeserializer::safe_to_int_from_field(value, "version")?;
        let timestamp = JsonDeserializer::safe_to_int_from_field(value, "timestamp")?;
        let reference = JsonDeserializer::safe_to_string_from_field(value, "reference")?;
        let transactions_root = match value["transactionsRoot"].as_str() {
            Some(val) => Base58String::from_string(val.to_owned())?,
            None => Base58String::empty(),
        };
        let id = JsonDeserializer::safe_to_string_from_field(value, "id")?;
        let features = match value["features"].as_array() {
            Some(array) => array
                .iter()
                .filter_map(|value| value.as_i64())
                .map(|feature| feature as u32)
                .collect(),
            None => vec![],
        };

        let desired_reward = value["desiredReward"].as_i64().unwrap_or(0);
        let generator = JsonDeserializer::safe_to_string_from_field(value, "generator")?;
        let signature = JsonDeserializer::safe_to_string_from_field(value, "signature")?;
        let blocksize = JsonDeserializer::safe_to_int_from_field(value, "blocksize")?;
        let transaction_count =
            JsonDeserializer::safe_to_int_from_field(value, "transactionCount")?;
        let height = JsonDeserializer::safe_to_int_from_field(value, "height")?;
        let total_fee = JsonDeserializer::safe_to_int_from_field(value, "totalFee")?;
        let reward = value["reward"].as_i64().unwrap_or(0);

        let vrf = value["VRF"].as_str().unwrap_or("");
        Ok(BlockHeaders {
            version: version as u8,
            timestamp: timestamp as u64,
            reference: Base58String::from_string(reference)?,
            nxt_consensus: value["nxt-consensus"].borrow().try_into()?,
            transactions_root,
            id: Base58String::from_string(id)?,
            features,
            desired_reward,
            generator: Address::from_string(&generator)?,
            signature: Base58String::from_string(signature)?,
            blocksize: blocksize as u32,
            transaction_count: transaction_count as u32,
            height: height as u32,
            total_fee: total_fee as u64,
            reward: reward as u64,
            vrf: Base58String::from_string(vrf.to_owned())?,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NxtConsensus {
    base_target: u32,
    generation_signature: Base58String,
}

impl NxtConsensus {
    pub fn new(base_target: u32, generation_signature: Base58String) -> Self {
        Self {
            base_target,
            generation_signature,
        }
    }

    pub fn base_target(&self) -> u32 {
        self.base_target
    }

    pub fn generation_signature(&self) -> Base58String {
        self.generation_signature.clone()
    }
}

impl TryFrom<&Value> for NxtConsensus {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let base_target = JsonDeserializer::safe_to_int_from_field(value, "base-target")?;
        let generation_signature =
            JsonDeserializer::safe_to_string_from_field(value, "generation-signature")?;

        Ok(NxtConsensus {
            base_target: base_target as u32,
            generation_signature: Base58String::from_string(generation_signature)?,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Block {
    block_headers: BlockHeaders,
    fee: u64,
    transactions: Vec<SignedTransaction>,
}

impl Block {
    pub fn new(
        block_headers: BlockHeaders,
        fee: u64,
        transactions: Vec<SignedTransaction>,
    ) -> Self {
        Self {
            block_headers,
            fee,
            transactions,
        }
    }

    pub fn block_headers(&self) -> BlockHeaders {
        self.block_headers.clone()
    }

    pub fn fee(&self) -> u64 {
        self.fee
    }

    pub fn transactions(&self) -> Vec<SignedTransaction> {
        self.transactions.clone()
    }
}

impl TryFrom<&Value> for Block {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let block_headers = value.try_into()?;
        let fee = JsonDeserializer::safe_to_int_from_field(value, "fee")?;

        let transactions: Vec<SignedTransaction> =
            JsonDeserializer::safe_to_array_from_field(value, "transactions")?
                .iter()
                .map(|tx| tx.try_into())
                .collect::<Result<Vec<SignedTransaction>>>()?;
        Ok(Self {
            block_headers,
            fee: fee as u64,
            transactions,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::{Block, BlockHeaders, ByteString};
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    pub fn block_headers_from_response() -> Result<()> {
        let data = fs::read_to_string("./tests/resources/blocks/block_headers_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let block_headers: BlockHeaders = json.borrow().try_into()?;

        assert_eq!(5, block_headers.version());
        assert_eq!(1662965069859, block_headers.timestamp());
        assert_eq!(
            "FDVU5z5JcjU1rU6BV7MassmFwzfiabxVUBcDxzJjZTNu",
            block_headers.reference().encoded()
        );
        assert_eq!(288, block_headers.nxt_consensus().base_target());
        assert_eq!(
            "2DzzTwTrDvNnKa5BnkQyuGJ6rAmCTxrKwQNdAEiRPPiY1WkrCy5ge5FDAWsMPh3eAk3sRQq3iVB8ZVGFkP8uJU65AfR4rkQw687yNcA3wbsDsDjsoyir1nGFCWkA34jv5cUN", 
                   block_headers.nxt_consensus().generation_signature().encoded());

        assert_eq!(
            "HW2HdCDGFX1cKQC9VQHPbCi5S87jzB43m8bM1yvbjxN6",
            block_headers.transactions_root().encoded()
        );
        assert_eq!(
            "7FjvruZHvR3mi9YvTaiGQ93uvdHbarcL54ynrS6jN1Vq",
            block_headers.id().encoded()
        );
        assert_eq!(3, block_headers.features()[0]);
        assert_eq!(-1, block_headers.desired_reward());
        assert_eq!(
            "3Mxv6Dpa1qRuyQBRFg3GwUaf3rcjHqWwNmC",
            block_headers.generator().encoded()
        );
        assert_eq!("3BCrNxGjwoxRMX2vxshN2K5ucSzt8huwvkHK54qWnp6UVQLBKx1TaidWAoxtswWPxbLTdmkWAMrCEdUVUZw4o8Zr", block_headers.signature().encoded());
        assert_eq!(869, block_headers.blocksize());
        assert_eq!(1, block_headers.transaction_count());
        assert_eq!(2225531, block_headers.height());
        assert_eq!(500000, block_headers.total_fee());
        assert_eq!(600000000, block_headers.reward());
        assert_eq!(
            "BaHPQwcWxXPaaUiRgavbEg5hkAvzWdNrLYV3x5JEDgpJ",
            block_headers.vrf().encoded()
        );
        Ok(())
    }

    #[test]
    pub fn block_at_height_response() -> Result<()> {
        let data = fs::read_to_string("./tests/resources/blocks/block_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let block: Block = json.borrow().try_into()?;
        let block_headers = block.block_headers();

        assert_eq!(5, block_headers.version());
        assert_eq!(1662965069859, block_headers.timestamp());
        assert_eq!(
            "FDVU5z5JcjU1rU6BV7MassmFwzfiabxVUBcDxzJjZTNu",
            block_headers.reference().encoded()
        );
        assert_eq!(288, block_headers.nxt_consensus().base_target());
        assert_eq!(
            "2DzzTwTrDvNnKa5BnkQyuGJ6rAmCTxrKwQNdAEiRPPiY1WkrCy5ge5FDAWsMPh3eAk3sRQq3iVB8ZVGFkP8uJU65AfR4rkQw687yNcA3wbsDsDjsoyir1nGFCWkA34jv5cUN",
            block_headers.nxt_consensus().generation_signature().encoded());

        assert_eq!(
            "HW2HdCDGFX1cKQC9VQHPbCi5S87jzB43m8bM1yvbjxN6",
            block_headers.transactions_root().encoded()
        );
        assert_eq!(
            "7FjvruZHvR3mi9YvTaiGQ93uvdHbarcL54ynrS6jN1Vq",
            block_headers.id().encoded()
        );
        assert_eq!(3, block_headers.features()[0]);
        assert_eq!(-1, block_headers.desired_reward());
        assert_eq!(
            "3Mxv6Dpa1qRuyQBRFg3GwUaf3rcjHqWwNmC",
            block_headers.generator().encoded()
        );
        assert_eq!("3BCrNxGjwoxRMX2vxshN2K5ucSzt8huwvkHK54qWnp6UVQLBKx1TaidWAoxtswWPxbLTdmkWAMrCEdUVUZw4o8Zr", block_headers.signature().encoded());
        assert_eq!(869, block_headers.blocksize());
        assert_eq!(1, block_headers.transaction_count());
        assert_eq!(2225531, block_headers.height());
        assert_eq!(500000, block_headers.total_fee());
        assert_eq!(600000000, block_headers.reward());
        assert_eq!(
            "BaHPQwcWxXPaaUiRgavbEg5hkAvzWdNrLYV3x5JEDgpJ",
            block_headers.vrf().encoded()
        );

        assert_eq!(500000, block.fee());
        assert_eq!(19, block.transactions().len());

        let genesis = &block.transactions[0];
        assert_eq!(
            "3zpi4i5SeCoaiCBn1iuTUvCc5aahvtabqXBTrCXy1Y3ujUbJo56VVv6n4HQtcwiFapvg3BKV6stb5QkxsBrudTKZ", 
            genesis.id()?.encoded()
        );
        let payment = &block.transactions[1];
        assert_eq!(
            "3MBsS7S42PVEM8c1XxLsGsxzhitPsyaazDs1QoE26pCTHdRMYRv7n984wmjSFP863iZ2GR28aunSVvPC8sooEpbP",
            payment.id()?.encoded()
        );
        let issue = &block.transactions[2];
        assert_eq!(
            "3kuZKAeyjcqavmezy86sWCAeXrgt3HBKa4HA8CZdT8nH",
            issue.id()?.encoded()
        );
        let transfer = &block.transactions[3];
        assert_eq!(
            "DBozd2VWYe1FDkrdQnJgvcxh9B6mL872onqpSCjF4a7t",
            transfer.id()?.encoded()
        );
        // let reissue = &block.transactions[4];
        // assert_eq!(
        //     "44seokQaBquAwDweKC4mbmHvmu2heWrUhKNGUakwZxRf",
        //     reissue.id()?.encoded()
        // );
        let burn = &block.transactions[5];
        assert_eq!(
            "7Ruo9tnYTuBKTRwbSfG2TLooP4v6pz8SkTx1hvCgfJLU",
            burn.id()?.encoded()
        );

        println!("{:#?}", block);

        Ok(())
    }
}
