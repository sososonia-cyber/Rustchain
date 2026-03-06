//! RustChain Epoch Reward Calculator - CLI Tool
//! 
//! A command-line tool for calculating RustChain mining rewards
//! based on epoch data from the network.
//! 
//! Bounty: #674 - Build RustChain Tools & Features in Rust (Tier 1)
//! Reward: 25-50 RTC

use clap::{Parser, Subcommand};
use epoch_calculator::{fetch_epoch_info, EpochRewardCalculator};
use std::process::ExitCode;

const DEFAULT_BASE_URL: &str = "https://rustchain.org";

#[derive(Parser)]
#[command(name = "rustchain-epoch-reward-calculator")]
#[command(about = "Calculate RustChain mining rewards for epochs", long_about = None)]
#[command(version = "1.0.0")]
struct Cli {
    /// Base URL for RustChain API
    #[arg(short, long, default_value = DEFAULT_BASE_URL)]
    url: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show current epoch information
    Epoch,
    
    /// Calculate average reward per miner for current epoch
    Average,
    
    /// Calculate reward for a specific number of blocks
    Reward {
        /// Number of blocks mined in the epoch
        blocks: u64,
    },
    
    /// Calculate reward based on relative hashrate (0.0-1.0)
    Hashrate {
        /// Relative hashrate as decimal (e.g., 0.01 for 1%)
        hashrate: f64,
    },
    
    /// Full calculation with projections
    Calculate {
        /// Number of blocks mined in the epoch
        #[arg(default_value = "1")]
        blocks: u64,
    },
}

fn run_cli(url: &str, command: Commands) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match command {
        Commands::Epoch => {
            let info = fetch_epoch_info(url)?;
            println!("╔══════════════════════════════════════════╗");
            println!("║     RustChain Epoch Information          ║");
            println!("╠══════════════════════════════════════════╣");
            println!("║ Epoch:           {:>24} ║", info.epoch);
            println!("║ Blocks/Epoch:    {:>24} ║", info.blocks_per_epoch);
            println!("║ Epoch Pot:       {:>23} RTC ║", info.epoch_pot);
            println!("║ Enrolled Miners: {:>24} ║", info.enrolled_miners);
            println!("║ Total Supply:    {:>23} RTC ║", info.total_supply_rtc);
            println!("║ Current Slot:    {:>24} ║", info.slot);
            println!("╚══════════════════════════════════════════╝");
        }
        
        Commands::Average => {
            let info = fetch_epoch_info(url)?;
            let calc = EpochRewardCalculator::new(info);
            let avg = calc.average_reward_per_miner();
            println!("Average reward per miner for current epoch: {:.4} RTC", avg);
        }
        
        Commands::Reward { blocks } => {
            let info = fetch_epoch_info(url)?;
            let calc = EpochRewardCalculator::new(info);
            let reward = calc.estimate_reward_for_miner(blocks);
            println!("Estimated reward for {} block(s): {:.4} RTC", blocks, reward);
            println!("Block reward: {:.4} RTC", calc.block_reward());
        }
        
        Commands::Hashrate { hashrate } => {
            let info = fetch_epoch_info(url)?;
            let calc = EpochRewardCalculator::new(info);
            let reward = calc.estimate_reward_by_hashrate(hashrate);
            let percentage = hashrate * 100.0;
            println!("Estimated reward for {:.2}% hashrate: {:.4} RTC", percentage, reward);
        }
        
        Commands::Calculate { blocks } => {
            let info = fetch_epoch_info(url)?;
            let calc = EpochRewardCalculator::new(info);
            let result = calc.calculate(blocks);
            
            println!("╔══════════════════════════════════════════════════╗");
            println!("║       RustChain Reward Calculation                ║");
            println!("╠══════════════════════════════════════════════════╣");
            println!("║ Epoch:              {:>30} ║", result.epoch);
            println!("║ Epoch Pot:          {:>28} RTC ║", result.epoch_pot);
            println!("║ Active Miners:      {:>30} ║", result.miner_count);
            println!("╠══════════════════════════════════════════════════╣");
            println!("║ Block Reward:       {:>28} RTC ║", result.block_reward);
            println!("║ Avg Reward/Miner:  {:>28} RTC ║", result.average_reward_per_miner);
            println!("║ Your Reward ({} block): {:>22} RTC ║", blocks, result.estimated_reward_for_miner);
            println!("╠══════════════════════════════════════════════════╣");
            println!("║ Est. Daily:         {:>27} RTC ║", result.estimated_daily_reward);
            println!("║ Est. Monthly:       {:>27} RTC ║", result.estimated_monthly_reward);
            println!("╚══════════════════════════════════════════════════╝");
        }
    }
    
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    
    match run_cli(&cli.url, cli.command) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
