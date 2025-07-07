pub trait Chain {
    fn connect(&self);
}

pub struct EthereumChain;
impl Chain for EthereumChain {
    fn connect(&self) {
        println!("Connecting to Ethereum Chain");
    }
}

pub struct HyperEVMChain;
impl Chain for HyperEVMChain {
    fn connect(&self) {
        println!("Connecting to HyperEVM Chain");
    }
}
