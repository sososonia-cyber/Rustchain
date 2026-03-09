# RustChain Quick Start Guide

Welcome to RustChain! This guide will help you get started in minutes.

## Node Information

- **Main Node:** https://50.28.86.131
- **Explorer:** https://50.28.86.131/explorer
- **Health Check:** https://50.28.86.131/health

---

## Choose Your Path

### 1. Wallet/User Path

#### Check Your Balance

```bash
# Using curl
curl https://50.28.86.131/api/balance/YOUR_WALLET_ID

# Example with a test wallet
curl https://50.28.86.131/api/balance/RTC-testwallet123
```

#### Understanding Wallet IDs

**Important:** RustChain uses its own wallet system. Your RustChain wallet ID is NOT an Ethereum, Solana, or Base address.

- **RustChain Wallet ID format:** `RTC-{identifier}` (e.g., `RTC-miner_abc123`)
- **NOT an ETH address:** Do NOT use 0x... addresses
- **NOT a Solana address:** Do NOT use So... addresses
- **NOT a Base address:** Do NOT use 0x... on Base network

#### View Transaction History

```bash
# Check explorer for transactions
curl https://50.28.86.131/explorer/transactions?wallet=YOUR_WALLET_ID
```

---

### 2. Miner Path

#### System Requirements

- Python 3.8+
- Internet connection
- Static IP recommended

#### Start Mining

```bash
# Clone the miner repository
git clone https://github.com/Scottcjn/rustchain-bounties.git
cd rustchain-bounties

# Install dependencies
pip install -r requirements.txt

# Run the miner
python miners/your_miner.py --wallet YOUR_RTC_WALLET_ID --node https://50.28.86.131
```

#### Check Miner Status

```bash
# Check node health
curl https://50.28.86.131/health

# List active miners
curl https://50.28.86.131/api/miners
```

---

### 3. Developer Path

#### API Endpoints

| Endpoint | Description |
|----------|-------------|
| `/health` | Node health status |
| `/epoch` | Current epoch info |
| `/api/balance/{wallet}` | Check wallet balance |
| `/api/miners` | List active miners |
| `/wallet/balance?miner_id={id}` | Alternative balance check |

#### Example: Check Node Health

```bash
curl -k https://50.28.86.131/health
```

Response:
```json
{
  "ok": true,
  "version": "1.0.0",
  "uptime_s": 123456,
  "db_rw": true
}
```

#### Example: Get Wallet Balance

```bash
curl -k "https://50.28.86.131/wallet/balance?miner_id=RTC-testwallet123"
```

#### Example: Python Client

```python
import requests

# Disable SSL warning for self-signed cert
requests.packages.urllib3.disable_warnings()

BASE_URL = "https://50.28.86.131"

# Check health
health = requests.get(f"{BASE_URL}/health", verify=False).json()
print(f"Node status: {health['ok']}")

# Check balance
wallet = "RTC-yourwallet"
balance = requests.get(
    f"{BASE_URL}/wallet/balance",
    params={"miner_id": wallet},
    verify=False
).json()
print(f"Balance: {balance.get('amount_rtc', 0)} RTC")
```

**Note:** Use `verify=False` when connecting to the node with self-signed certificates.

---

## Important Notes

### Self-Signed Certificates

The node uses a self-signed SSL certificate. When connecting via HTTPS:

- Use `-k` flag with curl
- Use `verify=False` in Python requests
- Your browser will show a security warning - this is normal

### Wallet ID vs External Addresses

**CRITICAL:** Never send funds to an Ethereum, Solana, or Base address expecting to receive RustChain tokens. RustChain has its own native wallet system.

- RustChain wallet IDs start with `RTC-`
- They are NOT compatible with ETH, SOL, or Base addresses

---

## Get Help

- **Bounties:** https://github.com/Scottcjn/rustchain-bounties
- **Issues:** Report bugs on the relevant repo
- **Community:** Join discussions on the project

---

*Last updated: 2026-03-09*
