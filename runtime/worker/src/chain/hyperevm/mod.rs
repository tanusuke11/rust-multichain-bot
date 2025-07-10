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
}
