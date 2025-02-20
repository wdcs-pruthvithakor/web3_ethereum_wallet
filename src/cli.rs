// cli.rs

use clap::{Parser, Subcommand};

/// Struct to hold CLI arguments
#[derive(Parser)]
#[command(name = "web3-wallet")]
#[command(about = "A simple Web3 Ethereum wallet CLI", long_about = "This CLI allows you to generate a new Ethereum wallet, fetch the balance of an Ethereum address, and send ETH to other addresses.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Enum to define subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new Ethereum wallet
    GenerateWallet,

    /// Fetch the balance of an address
    FetchBalance(FetchBalanceArgs),

    /// Send ETH to another address
    SendTransaction(SendTransactionArgs),

    /// Start an interactive terminal interface
    Interactive,
}

/// Struct for FetchBalance command arguments
#[derive(clap::Args)]
pub struct FetchBalanceArgs {
    /// Ethereum address to fetch balance for
    #[arg(help = "The Ethereum address to fetch balance for.")]
    pub address: String,

    /// Ethereum node URL (e.g., http://localhost:8545)
    #[arg(help = "URL of the Ethereum node (e.g., Infura or local node).")]
    pub node_url: String,
}

/// Struct for SendTransaction command arguments
#[derive(clap::Args)]
pub struct SendTransactionArgs {
    /// Sender's private key (used for signing the transaction)
    #[arg(help = "The private key of the sender's wallet.")]
    pub private_key: String,

    /// Recipient's Ethereum address
    #[arg(help = "The Ethereum address of the recipient.")]
    pub to_address: String,

    /// Amount of ETH to send
    #[arg(help = "Amount of ETH to send to the recipient.")]
    pub amount: f64,

    /// Ethereum node URL (e.g., http://localhost:8545)
    #[arg(help = "URL of the Ethereum node (e.g., Infura or local node).")]
    pub node_url: String,
}
