pub struct AtomicArbStrategy;
impl crate::strategy::Strategy for AtomicArbStrategy {
    fn execute(&self) {
        println!("Executing Atomic Arbitrage Strategy");
    }
}
