// Strategy trait + common utilities

pub trait Strategy {
    fn execute(&self);
}

pub struct AtomicArbStrategy;
impl AtomicArbStrategy {
    pub fn new() -> Self {
        AtomicArbStrategy
    }
}
impl Strategy for AtomicArbStrategy {
    fn execute(&self) {
        println!("Executing Atomic Arbitrage Strategy with specific logic...");
    }
}

pub struct LiquidatorStrategy;
impl LiquidatorStrategy {
    pub fn new() -> Self {
        LiquidatorStrategy
    }
}
impl Strategy for LiquidatorStrategy {
    fn execute(&self) {
        println!("Executing Liquidator Strategy with specific logic...");
    }
}
