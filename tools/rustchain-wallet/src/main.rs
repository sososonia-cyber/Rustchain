//! RustChain Wallet CLI
//! A command-line interface for managing RTC tokens on RustChain

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;
use ureq::{Agent, Response};

// ============ Error Types ============

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("Network error: {0}")]
    Network(#[from] ureq::Error),
    #[error("API error: {0}")]
    Api(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Wallet not found: {0}")]
    WalletNotFound(String),
}

impl From<serde_json::Error> for WalletError {
    fn from(e: serde_json::Error) -> Self {
        WalletError::Api(e.to_string())
    }
}

// ============ API Types ============

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub name: String,
    pub balance: f64,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub timestamp: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub epoch: u64,
    pub total_miners: u64,
}

// ============ CLI Types ============

#[derive(Parser)]
#[command(name = "rustchain-wallet")]
#[command(about = "RustChain CLI Wallet - Manage your RTC tokens", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, default_value = "https://rustchain.org")]
    base_url: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Check wallet balance
    Balance { wallet_name: String },
    
    /// Send RTC to another wallet
    Send {
        from_wallet: String,
        to_wallet: String,
        amount: f64,
    },
    
    /// Get transaction history
    History { wallet_name: String },
    
    /// Get network health status
    Health,
    
    /// List recent transactions
    Transactions { limit: Option<u32> },
    
    /// Generate a new wallet address
    Generate { wallet_name: String },
    
    /// Get wallet info
    Info { wallet_name: String },
}

// ============ API Client ============

pub struct RustChainClient {
    agent: Agent,
    base_url: String,
}

impl RustChainClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            agent: Agent::new(),
            base_url: base_url.to_string(),
        }
    }

    fn request(&self, method: &str, path: &str) -> Result<Response, WalletError> {
        let url = format!("{}{}", self.base_url, path);
        match method {
            "GET" => self.agent.get(&url).call().map_err(WalletError::Network),
            "POST" => self.agent.post(&url).call().map_err(WalletError::Network),
            _ => Err(WalletError::Api("Unknown method".to_string())),
        }
    }

    pub async fn get_health(&self) -> Result<HealthResponse, WalletError> {
        let response = self.request("GET", "/health")?;
        
        if !response.status() == 200 {
            return Err(WalletError::Api(format!("HTTP {}", response.status())));
        }
        
        Ok(response.into_json()?)
    }

    pub async fn get_wallet(&self, name: &str) -> Result<Wallet, WalletError> {
        let url = format!("/wallet/{}", name);
        let response = self.request("GET", &url)?;
        
        if response.status() == 404 {
            return Err(WalletError::WalletNotFound(name.to_string()));
        }
        
        if !response.status() == 200 {
            return Err(WalletError::Api(format!("HTTP {}", response.status())));
        }
        
        let api_response: ApiResponse<Wallet> = response.into_json()?;
        
        if let Some(data) = api_response.data {
            Ok(data)
        } else {
            Err(WalletError::Api(api_response.message.unwrap_or_default()))
        }
    }

    pub async fn send_transaction(
        &self,
        from: &str,
        to: &str,
        amount: f64,
    ) -> Result<Transaction, WalletError> {
        let body = serde_json::json!({
            "from": from,
            "to": to,
            "amount": amount
        });
        
        let response = self.agent
            .post(&format!("{}/wallet/send", self.base_url))
            .send_json(body)
            .map_err(WalletError::Network)?;
        
        if !response.status() == 200 {
            let error: ApiResponse<()> = response.into_json()?;
            return Err(WalletError::Api(error.message.unwrap_or_default()));
        }
        
        let api_response: ApiResponse<Transaction> = response.into_json()?;
        
        if let Some(data) = api_response.data {
            Ok(data)
        } else {
            Err(WalletError::Api(api_response.message.unwrap_or_default()))
        }
    }

    pub async fn get_transactions(&self, wallet: &str, limit: u32) -> Result<Vec<Transaction>, WalletError> {
        let url = format!("/wallet/{}/transactions?limit={}", wallet, limit);
        let response = self.request("GET", &url)?;
        
        if !response.status() == 200 {
            return Err(WalletError::Api(format!("HTTP {}", response.status())));
        }
        
        let api_response: ApiResponse<Vec<Transaction>> = response.into_json()?;
        
        Ok(api_response.data.unwrap_or_default())
    }
}

// ============ Commands ============

async fn cmd_balance(client: &RustChainClient, wallet_name: &str) -> Result<(), WalletError> {
    let wallet = client.get_wallet(wallet_name).await?;
    
    println!("\n🟡 Wallet: {}", wallet.name);
    println!("   Address: {}", wallet.address);
    println!("   Balance: {:.4} RTC\n", wallet.balance);
    
    Ok(())
}

async fn cmd_send(
    client: &RustChainClient,
    from_wallet: &str,
    to_wallet: &str,
    amount: f64,
) -> Result<(), WalletError> {
    println!("\n📤 Sending {:.4} RTC from {} to {}", amount, from_wallet, to_wallet);
    
    let tx = client.send_transaction(from_wallet, to_wallet, amount).await?;
    
    println!("✅ Transaction submitted!");
    println!("   Hash: {}", tx.tx_hash);
    println!("   Status: {}\n", tx.status);
    
    Ok(())
}

async fn cmd_history(client: &RustChainClient, wallet_name: &str) -> Result<(), WalletError> {
    let transactions = client.get_transactions(wallet_name, 20).await?;
    
    println!("\n📜 Transaction History for {}\n", wallet_name);
    
    if transactions.is_empty() {
        println!("   No transactions found.");
    } else {
        for tx in transactions {
            println!("   {} | {} → {} | {:.4} RTC | {}",
                tx.timestamp,
                &tx.from[..8.min(tx.from.len())],
                &tx.to[..8.min(tx.to.len())],
                tx.amount,
                tx.status
            );
        }
    }
    println!();
    
    Ok(())
}

async fn cmd_health(client: &RustChainClient) -> Result<(), WalletError> {
    let health = client.get_health().await?;
    
    println!("\n🔗 RustChain Network Status\n");
    println!("   Status:    {}", health.status);
    println!("   Epoch:     {}", health.epoch);
    println!("   Miners:    {}\n", health.total_miners);
    
    Ok(())
}

async fn cmd_transactions(client: &RustChainClient, limit: Option<u32>) -> Result<(), WalletError> {
    let limit = limit.unwrap_or(10);
    let url = format!("/transactions?limit={}", limit);
    let response = client.request("GET", &url)?;
    
    if !response.status() == 200 {
        return Err(WalletError::Api(format!("HTTP {}", response.status())));
    }
    
    let api_response: ApiResponse<Vec<Transaction>> = response.into_json()?;
    let transactions = api_response.data.unwrap_or_default();
    
    println!("\n📊 Recent Transactions ({})\n", limit);
    
    if transactions.is_empty() {
        println!("   No transactions found.");
    } else {
        for tx in transactions {
            println!("   {} | {} → {} | {:.4} RTC",
                &tx.timestamp[..19.min(tx.timestamp.len())],
                &tx.from[..8.min(tx.from.len())],
                &tx.to[..8.min(tx.to.len())],
                tx.amount
            );
        }
    }
    println!();
    
    Ok(())
}

async fn cmd_generate(client: &RustChainClient, wallet_name: &str) -> Result<(), WalletError> {
    let body = serde_json::json!({
        "name": wallet_name
    });
    
    let response = client.agent
        .post(&format!("{}/wallet/create", client.base_url))
        .send_json(body)
        .map_err(WalletError::Network)?;
    
    if !response.status() == 200 {
        let error: ApiResponse<()> = response.into_json()?;
        return Err(WalletError::Api(error.message.unwrap_or_default()));
    }
    
    let api_response: ApiResponse<Wallet> = response.into_json()?;
    
    if let Some(wallet) = api_response.data {
        println!("\n✅ Wallet created successfully!");
        println!("   Name:    {}", wallet.name);
        println!("   Address: {}\n", wallet.address);
    } else {
        return Err(WalletError::Api("Failed to create wallet".to_string()));
    }
    
    Ok(())
}

async fn cmd_info(client: &RustChainClient, wallet_name: &str) -> Result<(), WalletError> {
    let wallet = client.get_wallet(wallet_name).await?;
    
    println!("\n🟡 Wallet Information\n");
    println!("   Name:     {}", wallet.name);
    println!("   Address:  {}", wallet.address);
    println!("   Balance:  {:.4} RTC\n", wallet.balance);
    
    Ok(())
}

// ============ Main ============

#[tokio::main]
async fn main() -> Result<(), WalletError> {
    let cli = Cli::parse();
    let client = RustChainClient::new(&cli.base_url);
    
    match cli.command {
        Commands::Balance { wallet_name } => {
            cmd_balance(&client, &wallet_name).await?;
        }
        Commands::Send { from_wallet, to_wallet, amount } => {
            cmd_send(&client, &from_wallet, &to_wallet, amount).await?;
        }
        Commands::History { wallet_name } => {
            cmd_history(&client, &wallet_name).await?;
        }
        Commands::Health => {
            cmd_health(&client).await?;
        }
        Commands::Transactions { limit } => {
            cmd_transactions(&client, limit).await?;
        }
        Commands::Generate { wallet_name } => {
            cmd_generate(&client, &wallet_name).await?;
        }
        Commands::Info { wallet_name } => {
            cmd_info(&client, &wallet_name).await?;
        }
    }
    
    Ok(())
}
