pub mod atomic_arb;
pub mod liquidator;

pub use atomic_arb::AtomicArbStrategy;
pub use liquidator::LiquidatorStrategy;

pub trait Strategy {
    fn execute(&self);
}

