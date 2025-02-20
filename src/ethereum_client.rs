// src/ethereum_client.rs

use web3::{transports::Http, Web3};
use anyhow::{Context, Result};

/// Struct to hold the Web3 client and the Ethereum node URL
pub struct EthereumClient {
    pub web3: Web3<Http>,
    pub node_url: String,
}

impl EthereumClient {
    /// Create a new EthereumClient instance
    pub fn new(node_url: &str) -> Result<Self> {
        let transport = Http::new(node_url)
            .context("Failed to connect to Ethereum node")?;
        let web3 = Web3::new(transport);
        Ok(EthereumClient {
            web3,
            node_url: node_url.to_string(),
        })
    }
}
