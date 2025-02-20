// src/balance.rs

use crate::ethereum_client::EthereumClient;
use crate::utils::wei_to_eth;
use anyhow::{Context, Result};
use std::str::FromStr;
use web3::types::Address;

/// Fetch the balance of an Ethereum address
pub async fn fetch_balance(client: &EthereumClient, address: &str) -> Result<()> {
    let address = Address::from_str(address)
        .context("Invalid Ethereum address format")?;
    
    let balance = client.web3.eth().balance(address, None).await
        .context("Failed to fetch balance")?;

    println!(
        "Balance: {:.4} ETH",
        wei_to_eth(balance)  // Show balance with 4 decimal places
    );

    Ok(())
}
