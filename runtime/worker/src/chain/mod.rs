pub mod ethereum;
pub mod hyperevm;

pub use ethereum::EthereumChain;
pub use hyperevm::HyperEVMChain;

pub trait Chain {
    fn connect(&self);
}