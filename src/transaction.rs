// src/transaction.rs

use crate::ethereum_client::EthereumClient;
use crate::utils::{hex_to_secret_key, wei_to_eth};
use anyhow::{Context, Result};
use std::str::FromStr;
use web3::signing::SecretKey as Web3SecretKey;
use web3::types::{Address, BlockNumber, TransactionParameters, U256};

/// Struct for holding transaction details
pub struct Transaction<'a> {
    pub from_private_key: &'a str,
    pub to_address: &'a str,
    pub amount: f64,
    pub client: &'a EthereumClient,
}

impl<'a> Transaction<'a> {
    /// Send ETH to another address
    pub async fn send(&self) -> Result<()> {
        let secp_secret_key = hex_to_secret_key(self.from_private_key)
            .context("Failed to decode private key")?;
        let web3_secret_key = Web3SecretKey::from_slice(secp_secret_key.as_ref())?;
        let to_address = Address::from_str(self.to_address)
            .context("Invalid recipient address format")?;

        // Estimate gas for the transaction
        let gas_estimate = self.client.web3
        .eth()
        .estimate_gas(TransactionParameters {
            to: Some(to_address),
            value: U256::from((self.amount * 1e18) as u64),
            gas: U256::from(21000),
            gas_price: Some(U256::from(1000000000u64)), // 1 Gwei
            ..Default::default()
        }.into(),
        Some(BlockNumber::Latest)
        )
            .await
            .context("Failed to estimate gas")?;

        // Calculate estimated transaction fee
        let gas_price = U256::from(1000000000u64); // 1 Gwei
        let fee_in_wei = gas_estimate * gas_price;
        let fee_in_eth = wei_to_eth(fee_in_wei);

        println!(
            "Estimated transaction fee: {:.6} ETH",
            fee_in_eth
        );

        // Proceed to sign and send the transaction
        let tx_object = TransactionParameters {
            to: Some(to_address),
            value: U256::from((self.amount * 1e18) as u64),
            gas: gas_estimate,
            gas_price: Some(gas_price),
            ..Default::default()
        };

        let signed = self.client.web3
            .accounts()
            .sign_transaction(tx_object, &web3_secret_key)
            .await
            .context("Failed to sign transaction")?;
        let tx_hash = self.client.web3
            .eth()
            .send_raw_transaction(signed.raw_transaction)
            .await
            .context("Failed to send transaction")?;

        println!("Transaction Hash: {:?}", tx_hash);

        Ok(())
    }
}
