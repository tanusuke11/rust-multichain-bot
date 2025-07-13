use std::sync::Arc;
use crate::chain::Chain;
use crate::env::Environment;

pub struct LiquidatorStrategy {
    chain: Arc<dyn Chain>,
    environment: Arc<dyn Environment>,
}

impl LiquidatorStrategy {
    pub fn new(chain: Arc<dyn Chain>, environment: Arc<dyn Environment>) -> Self {
        Self {
            chain,
            environment,
        }
    }
}

impl crate::strategy::Strategy for LiquidatorStrategy {
    fn execute(&self) {
        println!("Executing Liquidator Strategy");
    }
}
