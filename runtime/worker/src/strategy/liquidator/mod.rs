pub struct LiquidatorStrategy;

impl LiquidatorStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl crate::strategy::Strategy for LiquidatorStrategy {
    fn execute(&self) {
        println!("Executing Liquidator Strategy");
    }
}
