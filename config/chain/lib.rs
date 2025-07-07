// Chain trait + module integration

pub trait Chain {
    fn connect(&self);
}

pub struct EthereumChain;
impl EthereumChain {
    pub fn new() -> Self {
        EthereumChain
    }
}
impl Chain for EthereumChain {
    fn connect(&self) {
        println!("Connecting to Ethereum Chain with RPC configurations...");
    }
}

pub struct HyperEVMChain;
impl HyperEVMChain {
    pub fn new() -> Self {
        HyperEVMChain
    }
}
impl Chain for HyperEVMChain {
    fn connect(&self) {
        println!("Connecting to HyperEVM Chain with specific configurations...");
    }
}
