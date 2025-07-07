pub struct LiquidatorStrategy;
impl crate::strategy::Strategy for LiquidatorStrategy {
    fn execute(&self) {
        println!("Executing Liquidator Strategy");
    }
}
