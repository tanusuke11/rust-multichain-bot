use std::env;
use std::error::Error;
use alloy_primitives::{Address, keccak256};
use k256::ecdsa::SigningKey;

pub mod ethereum;
pub mod hyperevm;

pub use ethereum::EthereumChain;
pub use hyperevm::HyperEVMChain;

pub trait Chain {
    fn connect(&self);
    fn get_primary_wallet_address(&self) -> Result<Address, Box<dyn Error>>;
}

/// Converts a private key to an Ethereum address using k256 and alloy-primitives
fn private_key_to_address_evm(private_key: &str) -> Result<Address, Box<dyn Error>> {
    let key_str = private_key.strip_prefix("0x").unwrap_or(private_key);
    let key_bytes = hex::decode(key_str)?;
    
    if key_bytes.len() != 32 {
        return Err("Private key must be 32 bytes".into());
    }
    
    // Use k256 for efficient key operations
    let signing_key = SigningKey::from_slice(&key_bytes)?;
    let public_key = signing_key.verifying_key().to_encoded_point(false);
    let public_key_bytes = &public_key.as_bytes()[1..]; // Remove 0x04 prefix
    
    // Hash and extract address
    let hash = keccak256(public_key_bytes);
    let address_bytes: [u8; 20] = hash[12..].try_into()?;
    
    Ok(Address::from(address_bytes))
}

/// Parses WALLET_SECRETS_EVM and converts private keys to addresses
pub fn parse_wallet_secrets_evm() -> Result<Vec<Address>, Box<dyn Error>> {
    let secrets = env::var("WALLET_SECRETS_EVM")?;
    
    let addresses: Vec<Address> = secrets
        .split(',')
        .filter_map(|key| {
            let key = key.trim();
            if key.is_empty() { return None; }
            
            // Try private key conversion first, then direct address parsing
            private_key_to_address_evm(key)
                .or_else(|_| key.parse::<Address>())
                .ok()
        })
        .collect();
    
    if addresses.is_empty() {
        Err("No valid wallet addresses found".into())
    } else {
        Ok(addresses)
    }
}