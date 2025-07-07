pub trait Strategy {
    fn execute(&self);
}

pub struct AtomicArbStrategy;
impl Strategy for AtomicArbStrategy {
    fn execute(&self) {
        println!("Executing Atomic Arbitrage Strategy");
    }
}

pub struct LiquidatorStrategy;
impl Strategy for LiquidatorStrategy {
    fn execute(&self) {
        println!("Executing Liquidator Strategy");
    }
}
