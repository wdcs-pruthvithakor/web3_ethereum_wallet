// src/utils.rs

use anyhow::Result;
use hex;
use secp256k1::{PublicKey, SecretKey};
use tiny_keccak::{Hasher, Keccak};
use web3::types::{Address, U256};

/// Convert a public key to an Ethereum address
pub fn public_key_to_address(public_key: &PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();
    let hash = keccak256(&public_key[1..]);
    Address::from_slice(&hash[12..])
}

/// Compute the Keccak-256 hash of a byte array
pub fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    hasher.finalize(&mut output);
    output
}

/// Convert Wei to ETH
pub fn wei_to_eth(wei: U256) -> f64 {
    let wei = wei.as_u128() as f64;
    wei / 1e18
}

/// Convert hex string to secp256k1 SecretKey
pub fn hex_to_secret_key(hex_str: &str) -> Result<SecretKey> {
    let private_key = if hex_str.starts_with("0x") {
        &hex_str[2..] // Remove "0x" prefix
    } else {
        hex_str
    };
    // Decode hex string and convert to secp256k1 SecretKey
    let decoded = hex::decode(private_key).map_err(|_| secp256k1::Error::InvalidSecretKey)?; // Map the hex error to secp256k1 error

    SecretKey::from_slice(&decoded).map_err(|e| e.into()) // Convert secp256k1 error to `anyhow::Error`
}

