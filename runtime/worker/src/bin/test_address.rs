// Test program to verify private key to address conversion

use dotenv::dotenv;
use std::env;
use std::error::Error;
use alloy_primitives::{Address, keccak256};
use k256::ecdsa::SigningKey;

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

fn main() {
    // Load .env file
    dotenv().ok();
    
    println!("Testing private key to EVM address conversion...");
    
    // Test with the private key from .env
    if let Ok(secrets) = env::var("WALLET_SECRETS_EVM") {
        let first_key = secrets.split(',').next().unwrap_or("").trim();
        
        if !first_key.is_empty() {
            println!("Input private key: {}", first_key);
            
            match private_key_to_address_evm(first_key) {
                Ok(address) => {
                    let address_str = format!("{:#x}", address);
                    println!("✅ Derived address: {}", address_str);
                    
                    // Check if it matches the target
                    let target = "0x1325a0828ca572261eb557058f352a5072006b7c";
                    if address_str.to_lowercase() == target.to_lowercase() {
                        println!("🎉 SUCCESS: Address matches target!");
                    } else {
                        println!("❌ MISMATCH: Expected {}, got {}", target, address_str);
                    }
                }
                Err(e) => {
                    println!("❌ Error converting private key: {}", e);
                }
            }
        } else {
            println!("❌ No private key found in WALLET_SECRETS_EVM");
        }
    } else {
        println!("❌ WALLET_SECRETS_EVM environment variable not found");
    }
}
