pub struct HyperEVMChain;
impl crate::chain::Chain for HyperEVMChain {
    fn connect(&self) {
        println!("Connecting to HyperEVM Chain");
    }
}
