// balance.rs

use crate::utils::wei_to_eth;
use anyhow::{Context, Result};
use std::str::FromStr;
use web3::{transports::Http, types::Address, Web3};

/// Fetch the balance of an Ethereum address
pub async fn fetch_balance(address: &str, node_url: &str) -> Result<()> {
    let transport = Http::new(node_url)
        .context("Failed to connect to Ethereum node")?;
    let web3 = Web3::new(transport);

    let address = Address::from_str(address)
        .context("Invalid Ethereum address format")?;
    let balance = web3.eth().balance(address, None).await
        .context("Failed to fetch balance")?;

    println!(
        "Balance: {:.4} ETH",
        wei_to_eth(balance)  // Show balance with 4 decimal places
    );

    Ok(())
}
