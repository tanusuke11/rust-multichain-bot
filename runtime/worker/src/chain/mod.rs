pub trait Chain {
    fn connect(&self);
}

pub struct EthereumChain;
pub struct HyperEVMChain;