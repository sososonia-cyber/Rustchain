//! RustChain Epoch Reward Calculator Library
//! 
//! This library provides utilities for calculating RustChain mining rewards
//! based on epoch data from the RustChain network.

use serde::{Deserialize, Serialize};

/// Epoch information from RustChain API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochInfo {
    /// Current epoch number
    pub epoch: u64,
    /// Blocks per epoch
    pub blocks_per_epoch: u64,
    /// Current epoch pot (total rewards)
    #[serde(rename = "epoch_pot")]
    pub epoch_pot: f64,
    /// Total enrolled miners
    pub enrolled_miners: u64,
    /// Current slot
    pub slot: u64,
    /// Total RTC supply
    #[serde(rename = "total_supply_rtc")]
    pub total_supply_rtc: f64,
}

/// Miner information from RustChain API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerInfo {
    /// Miner wallet name
    pub name: String,
    /// Total blocks mined
    pub blocks_mined: u64,
    /// Total rewards earned
    pub rewards: f64,
    /// Attestation score
    pub score: f64,
}

/// Reward calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardCalculation {
    /// Epoch number
    pub epoch: u64,
    /// Total epoch pot
    pub epoch_pot: f64,
    /// Number of miners
    pub miner_count: u64,
    /// Average reward per miner
    pub average_reward_per_miner: f64,
    /// Estimated reward for a miner with given hashrate
    pub estimated_reward_for_miner: f64,
    /// Block reward per block
    pub block_reward: f64,
    /// Estimated daily earnings (assuming constant participation)
    pub estimated_daily_reward: f64,
    /// Estimated monthly earnings
    pub estimated_monthly_reward: f64,
}

/// Calculator for epoch rewards
pub struct EpochRewardCalculator {
    epoch_info: EpochInfo,
}

impl EpochRewardCalculator {
    /// Create a new calculator from epoch info
    pub fn new(epoch_info: EpochInfo) -> Self {
        Self { epoch_info }
    }

    /// Calculate average reward per miner
    pub fn average_reward_per_miner(&self) -> f64 {
        if self.epoch_info.enrolled_miners > 0 {
            self.epoch_info.epoch_pot / self.epoch_info.enrolled_miners as f64
        } else {
            0.0
        }
    }

    /// Calculate block reward
    pub fn block_reward(&self) -> f64 {
        if self.epoch_info.blocks_per_epoch > 0 {
            self.epoch_info.epoch_pot / self.epoch_info.blocks_per_epoch as f64
        } else {
            0.0
        }
    }

    /// Calculate estimated reward for a specific miner
    /// 
    /// # Arguments
    /// * `miner_blocks` - Number of blocks the miner contributed in the epoch
    pub fn estimate_reward_for_miner(&self, miner_blocks: u64) -> f64 {
        self.block_reward() * miner_blocks as f64
    }

    /// Calculate estimated reward based on relative hashrate
    /// 
    /// # Arguments
    /// * `relative_hashrate` - Miner hashrate as a fraction of total (0.0 to 1.0)
    pub fn estimate_reward_by_hashrate(&self, relative_hashrate: f64) -> f64 {
        self.epoch_info.epoch_pot * relative_hashrate.max(0.0).min(1.0)
    }

    /// Full reward calculation with projections
    pub fn calculate(&self, miner_blocks: u64) -> RewardCalculation {
        let avg_reward = self.average_reward_per_miner();
        let block_reward = self.block_reward();
        let miner_reward = block_reward * miner_blocks as f64;
        
        // Assume 3 epochs per day (rough estimate)
        let daily_epochs = 3.0;
        let monthly_epochs = daily_epochs * 30.0;

        RewardCalculation {
            epoch: self.epoch_info.epoch,
            epoch_pot: self.epoch_info.epoch_pot,
            miner_count: self.epoch_info.enrolled_miners,
            average_reward_per_miner: avg_reward,
            estimated_reward_for_miner: miner_reward,
            block_reward,
            estimated_daily_reward: miner_reward * daily_epochs,
            estimated_monthly_reward: miner_reward * monthly_epochs,
        }
    }
}

/// Fetch current epoch info from RustChain API
pub fn fetch_epoch_info(base_url: &str) -> Result<EpochInfo, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}/epoch", base_url.trim_end_matches('/'));
    let response = ureq::get(&url).call()?.into_json::<EpochInfo>()?;
    Ok(response)
}

/// Fetch miner info from RustChain API
pub fn fetch_miner_info(base_url: &str, wallet: &str) -> Result<MinerInfo, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}/api/miners/{}", base_url.trim_end_matches('/'), wallet);
    let response = ureq::get(&url).call()?.into_json::<MinerInfo>()?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_epoch_info() -> EpochInfo {
        EpochInfo {
            epoch: 100,
            blocks_per_epoch: 144,
            epoch_pot: 10.0,
            enrolled_miners: 100,
            slot: 5000,
            total_supply_rtc: 1000000.0,
        }
    }

    #[test]
    fn test_average_reward() {
        let info = test_epoch_info();
        let calc = EpochRewardCalculator::new(info);
        assert_eq!(calc.average_reward_per_miner(), 0.1);
    }

    #[test]
    fn test_block_reward() {
        let info = test_epoch_info();
        let calc = EpochRewardCalculator::new(info);
        assert!((calc.block_reward() - 0.06944).abs() < 0.001);
    }

    #[test]
    fn test_estimate_reward() {
        let info = test_epoch_info();
        let calc = EpochRewardCalculator::new(info);
        let reward = calc.estimate_reward_for_miner(10);
        assert!((reward - 0.6944).abs() < 0.001);
    }
}
