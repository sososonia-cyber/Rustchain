# RustChain Epoch Reward Calculator

A Rust CLI tool for calculating RustChain mining rewards based on epoch data from the network.

## Bounty

This tool addresses **Issue #674**: Build RustChain Tools & Features in Rust (Tier 1)
- **Reward**: 25-50 RTC

## Features

- Fetch current epoch information from RustChain API
- Calculate average reward per miner
- Estimate rewards based on blocks mined
- Calculate rewards based on relative hashrate
- Full calculation with daily/monthly projections

## Installation

### From Source

```bash
git clone https://github.com/sososonia-cyber/Rustchain.git
cd Rustchain/rustchain-epoch-reward-calculator
cargo build --release
./target/release/rustchain-epoch-reward-calculator --help
```

## Usage

### View Current Epoch Info

```bash
rustchain-epoch-reward-calculator epoch
```

### Calculate Average Reward per Miner

```bash
rustchain-epoch-reward-calculator average
```

### Calculate Reward for Specific Blocks

```bash
# Reward for 10 blocks
rustchain-epoch-reward-calculator reward 10
```

### Calculate by Hashrate

```bash
# 1% hashrate
rustchain-epoch-reward-calculator hashrate 0.01

# 5% hashrate
rustchain-epoch-reward-calculator hashrate 0.05
```

### Full Calculation

```bash
rustchain-epoch-reward-calculator calculate --blocks 5
```

## Example Output

```
╔══════════════════════════════════════════════════╗
║       RustChain Reward Calculation                ║
╠══════════════════════════════════════════════════╣
║ Epoch:              93                             ║
║ Epoch Pot:         1.5 RTC                         ║
║ Active Miners:      19                             ║
╠══════════════════════════════════════════════════╣
║ Block Reward:       0.0104 RTC                     ║
║ Avg Reward/Miner:   0.0789 RTC                     ║
║ Your Reward (5 block): 0.0521 RTC                  ║
╠══════════════════════════════════════════════════╣
║ Est. Daily:         0.1563 RTC                     ║
║ Est. Monthly:       4.6889 RTC                     ║
╚══════════════════════════════════════════════════╝
```

## API Reference

- **Base URL**: https://rustchain.org
- **Epoch API**: GET /epoch
- **Miners API**: GET /api/miners

## Requirements

- Rust 1.70+
- Stable Rust (latest recommended)

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

## License

MIT License

## Bounty Details

- **Issue**: [RustChain Bounties #674](https://github.com/Scottcjn/rustchain-bounties/issues/674)
- **Tier**: 1 (Utilities)
- **Category**: Epoch reward calculator

## Notes

- Epoch pot and miner counts change over time
- Daily/monthly projections are estimates based on ~3 epochs/day
- Actual rewards depend on network conditions and attestation
