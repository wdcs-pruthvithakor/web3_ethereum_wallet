// src/wallet.rs

use crate::utils::public_key_to_address;
use anyhow::{Context, Result};
use secp256k1::Secp256k1;
use web3::signing::SecretKey as Web3SecretKey;
use web3::types::Address;
use hex;

/// Struct to hold wallet details
pub struct Wallet {
    pub private_key: Web3SecretKey,
    pub address: Address,
}

impl Wallet {
    /// Generate a new Ethereum wallet
    pub fn generate() -> Result<Self> {
        let secp = Secp256k1::new();
        let mut rng = rand::rngs::OsRng;
        let (secp_secret_key, secp_public_key) = secp.generate_keypair(&mut rng);
        let web3_secret_key = Web3SecretKey::from_slice(secp_secret_key.as_ref())
            .context("Failed to convert secret key")?;
        let address = public_key_to_address(&secp_public_key);

        Ok(Wallet {
            private_key: web3_secret_key,
            address,
        })
    }

    /// Display the wallet details
    pub fn display(&self) {
        println!("\n--- Wallet Generated ---");
        println!("Address: {:?}", self.address);
        println!("Private Key: {:?}", hex::encode(self.private_key.as_ref()));
        println!("Make sure to keep your private key safe and secure.");
    }
}
