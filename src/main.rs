// main.rs

mod balance;
mod cli;
mod transaction;
mod utils;
mod wallet;
mod interactive;

use anyhow::{Context, Result};
use balance::fetch_balance;
use clap::Parser;
use cli::{Cli, Commands};
use transaction::Transaction;
use wallet::Wallet;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateWallet => {
            let wallet = Wallet::generate()?;
            wallet.display();
        }
        Commands::FetchBalance(args) => {
            fetch_balance(&args.address, &args.node_url)
                .await
                .context("Failed to fetch balance")?;
        }
        Commands::SendTransaction(args) => {
            let tx = Transaction {
                from_private_key: &args.private_key,
                to_address: &args.to_address,
                amount: args.amount,
                node_url: &args.node_url,
            };
            tx.send().await.context("Failed to send transaction")?;
        }
        Commands::Interactive => {
            interactive::interactive_loop().await.context("Failed to run interactive session")?;
        }
    }

    Ok(())
}
