# RustChain Wallet CLI

A command-line interface for managing RTC tokens on the RustChain blockchain.

## Features

- Check wallet balance
- Send RTC to other wallets
- View transaction history
- Generate new wallet addresses
- Check network health status
- Browse recent transactions

## Installation

```bash
# Build from source
cargo build --release

# Or install directly
cargo install --path .
```

## Usage

### Check Wallet Balance

```bash
rustchain-wallet balance my_wallet
```

### Send RTC

```bash
rustchain-wallet send --from sender_wallet --to receiver_wallet --amount 10.0
```

### View Transaction History

```bash
rustchain-wallet history my_wallet
```

### Check Network Health

```bash
rustchain-wallet health
```

### Generate New Wallet

```bash
rustchain-wallet generate new_wallet_name
```

### Get Wallet Info

```bash
rustchain-wallet info my_wallet
```

### Browse Recent Transactions

```bash
rustchain-wallet transactions --limit 20
```

## Options

- `--base-url, -b`: Set the RustChain API base URL (default: https://rustchain.org)

## Example

```bash
# Check balance
$ rustchain-wallet balance founder_community

🟡 Wallet: founder_community
   Address: founder_community
   Balance: 1234.5678 RTC

# Check network health
$ rustchain-wallet health

🔗 RustChain Network Status

   Status:    healthy
   Epoch:     12345
   Miners:    42
```

## Development

```bash
# Run tests
cargo test

# Build in debug mode
cargo build
```

## License

MIT
