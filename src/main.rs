// src/main.rs

mod balance;
mod cli;
mod transaction;
mod utils;
mod wallet;
mod interactive;
mod ethereum_client;

use anyhow::{Context, Result};
use balance::fetch_balance;
use clap::Parser;
use cli::{Cli, Commands};
use transaction::{Transaction, get_transaction_receipt};
use wallet::Wallet;
use ethereum_client::EthereumClient;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateWallet => {
            let wallet = Wallet::generate()?;
            wallet.display();
        }
        Commands::FetchBalance(args) => {
            let client = EthereumClient::new(&args.node_url)?;
            fetch_balance(&client, &args.address)
                .await
                .context("Failed to fetch balance")?;
        }
        Commands::SendTransaction(args) => {
            let client = EthereumClient::new(&args.node_url)?;
            let tx = Transaction {
                from_private_key: &args.private_key,
                to_address: &args.to_address,
                amount: args.amount,
                client: &client,
            };
            tx.send().await.context("Failed to send transaction")?;
        }
        Commands::GetTransactionReceipt(args) => {
            let client = EthereumClient::new(&args.node_url)?;
            get_transaction_receipt(&client, args.hash.trim()).await.context("Failed to get transaction recipt")?;
        }
        Commands::Interactive => {
            interactive::interactive_loop().await.context("Failed to run interactive session")?;
        }
    }

    Ok(())
}
