// DI container definition

use std::sync::Arc;

pub struct AppContext {
    pub config: Arc<Config>,
    pub strategy: Arc<dyn Strategy>,
    pub chain: Arc<dyn Chain>,
    pub environment: Arc<dyn Environment>,
}

pub struct Config {
    pub log_level: String,
    pub strategy_type: String,
    pub chain_type: String,
    pub environment_type: String,
}

impl AppContext {
    pub fn new() -> Self {
        let config = Config {
            log_level: "info".to_string(),
            strategy_type: "atomic_arb".to_string(),
            chain_type: "ethereum".to_string(),
            environment_type: "local".to_string(),
        };

        let strategy: Arc<dyn Strategy> = match config.strategy_type.as_str() {
            "atomic_arb" => Arc::new(AtomicArbStrategy::new()),
            "liquidator" => Arc::new(LiquidatorStrategy::new()),
            _ => panic!("Unknown strategy type"),
        };

        let chain: Arc<dyn Chain> = match config.chain_type.as_str() {
            "ethereum" => Arc::new(EthereumChain::new()),
            "hyperevm" => Arc::new(HyperEVMChain::new()),
            _ => panic!("Unknown chain type"),
        };

        let environment: Arc<dyn Environment> = match config.environment_type.as_str() {
            "local" => Arc::new(LocalEnvironment::new()),
            "remote" => Arc::new(RemoteEnvironment::new()),
            _ => panic!("Unknown environment type"),
        };

        AppContext {
            config: Arc::new(config),
            strategy,
            chain,
            environment,
        }
    }
}

// Traits and implementations for Strategy, Chain, and Environment
pub trait Strategy {
    fn execute(&self);
}

pub trait Chain {
    fn connect(&self);
}

pub trait Environment {
    fn setup(&self);
}
