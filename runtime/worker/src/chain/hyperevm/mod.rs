use std::error::Error;
use alloy_primitives::Address;

pub struct HyperEVMChain;

impl HyperEVMChain {
    pub fn new() -> Self {
        Self
    }
}

impl crate::chain::Chain for HyperEVMChain {
    fn connect(&self) {
        println!("Connecting to HyperEVM Chain");
    }

    fn get_primary_wallet_address(&self) -> Result<Address, Box<dyn Error>> {
        get_primary_wallet_address()
    }
}

/// Gets the first wallet address from WALLET_SECRETS_EVM for HyperEVM.
/// Returns an error if no valid addresses are found.
pub fn get_primary_wallet_address() -> Result<Address, Box<dyn Error>> {
    let addresses = crate::chain::parse_wallet_secrets_evm()?;
    addresses.into_iter().next()
        .ok_or_else(|| "No wallet addresses available for HyperEVM".into())
}
