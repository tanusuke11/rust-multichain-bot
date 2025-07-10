pub struct EthereumChain;

impl EthereumChain {
    pub fn new() -> Self {
        Self
    }
}

impl crate::chain::Chain for EthereumChain {
    fn connect(&self) {
        println!("Connecting to Ethereum Chain");
    }
}
