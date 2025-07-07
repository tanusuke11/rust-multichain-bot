pub struct EthereumChain;
impl crate::chain::Chain for EthereumChain {
    fn connect(&self) {
        println!("Connecting to Ethereum Chain");
    }
}
