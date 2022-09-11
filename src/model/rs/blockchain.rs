use crate::error::{Error, Result};
use crate::util::JsonDeserializer;
use serde_json::Value;
use std::borrow::Borrow;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct BlockchainRewards {
    height: u32,
    current_reward: u64,
    total_waves_amount: u64,
    min_increment: u64,
    term: u32,
    next_check: u32,
    voting_interval_start: u32,
    voting_interval: u32,
    voting_threshold: u32,
    votes: Votes,
}

#[allow(clippy::too_many_arguments)]
impl BlockchainRewards {
    pub fn new(
        height: u32,
        current_reward: u64,
        total_waves_amount: u64,
        min_increment: u64,
        term: u32,
        next_check: u32,
        voting_interval_start: u32,
        voting_interval: u32,
        voting_threshold: u32,
        votes: Votes,
    ) -> Self {
        Self {
            height,
            current_reward,
            total_waves_amount,
            min_increment,
            term,
            next_check,
            voting_interval_start,
            voting_interval,
            voting_threshold,
            votes,
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn current_reward(&self) -> u64 {
        self.current_reward
    }

    pub fn total_waves_amount(&self) -> u64 {
        self.total_waves_amount
    }

    pub fn min_increment(&self) -> u64 {
        self.min_increment
    }

    pub fn term(&self) -> u32 {
        self.term
    }

    pub fn next_check(&self) -> u32 {
        self.next_check
    }

    pub fn voting_interval_start(&self) -> u32 {
        self.voting_interval_start
    }

    pub fn voting_interval(&self) -> u32 {
        self.voting_interval
    }

    pub fn voting_threshold(&self) -> u32 {
        self.voting_threshold
    }

    pub fn votes(&self) -> Votes {
        self.votes.clone()
    }
}

impl TryFrom<&Value> for BlockchainRewards {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let height = JsonDeserializer::safe_to_int_from_field(value, "height")?;
        let current_reward = JsonDeserializer::safe_to_int_from_field(value, "currentReward")?;
        let total_waves_amount =
            JsonDeserializer::safe_to_int_from_field(value, "totalWavesAmount")?;
        let min_increment = JsonDeserializer::safe_to_int_from_field(value, "minIncrement")?;
        let term = JsonDeserializer::safe_to_int_from_field(value, "term")?;
        let next_check = JsonDeserializer::safe_to_int_from_field(value, "nextCheck")?;
        let voting_interval_start =
            JsonDeserializer::safe_to_int_from_field(value, "votingIntervalStart")?;
        let voting_interval = JsonDeserializer::safe_to_int_from_field(value, "votingInterval")?;
        let voting_threshold = JsonDeserializer::safe_to_int_from_field(value, "votingThreshold")?;
        let votes: Votes = value["votes"].borrow().try_into()?;

        Ok(BlockchainRewards {
            height: height as u32,
            current_reward: current_reward as u64,
            total_waves_amount: total_waves_amount as u64,
            min_increment: min_increment as u64,
            term: term as u32,
            next_check: next_check as u32,
            voting_interval_start: voting_interval_start as u32,
            voting_interval: voting_interval as u32,
            voting_threshold: voting_threshold as u32,
            votes,
        })
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Votes {
    increase: u32,
    decrease: u32,
}

impl Votes {
    pub fn new(increase: u32, decrease: u32) -> Self {
        Self { increase, decrease }
    }

    pub fn increase(&self) -> u32 {
        self.increase
    }

    pub fn decrease(&self) -> u32 {
        self.decrease
    }
}

impl TryFrom<&Value> for Votes {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let increase = JsonDeserializer::safe_to_int_from_field(value, "increase")?;
        let decrease = JsonDeserializer::safe_to_int_from_field(value, "decrease")?;
        Ok(Votes {
            increase: increase as u32,
            decrease: decrease as u32,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::BlockchainRewards;
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_blockchain_rewards() {
        let data = fs::read_to_string("./tests/resources/blockchain/blockchain_rewards.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let rewards_from_json: BlockchainRewards = json.borrow().try_into().unwrap();

        assert_eq!(2224756, rewards_from_json.height());
        assert_eq!(10904654200000000, rewards_from_json.total_waves_amount());
        assert_eq!(600000000, rewards_from_json.current_reward());
        assert_eq!(50000000, rewards_from_json.min_increment());
        assert_eq!(100000, rewards_from_json.term());
        assert_eq!(2316999, rewards_from_json.next_check());
        assert_eq!(2307000, rewards_from_json.voting_interval_start());
        assert_eq!(10000, rewards_from_json.voting_interval());
        assert_eq!(5001, rewards_from_json.voting_threshold());
        assert_eq!(0, rewards_from_json.votes().increase());
        assert_eq!(1, rewards_from_json.votes().decrease());
    }
}
