pub struct AtomicArbStrategy;

impl AtomicArbStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl crate::strategy::Strategy for AtomicArbStrategy {
    fn execute(&self) {
        println!("Executing Atomic Arbitrage Strategy");
    }
}
