// DI container definition

use std::fs;
use std::path::Path;
use std::sync::Arc;
use toml::Value;
use worker::strategy::{AtomicArbStrategy, LiquidatorStrategy};
use worker::chain::{EthereumChain, HyperEVMChain};
use worker::env::{local::LocalEnvironment, vps::VPSEnvironment};


pub struct AppContext {
    pub config: Arc<Config>,
    pub strategy: Arc<dyn Strategy>,
    pub chain: Arc<dyn Chain>,
    pub environment: Arc<dyn Environment>,
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
            "vps" => Arc::new(VPSEnvironment::new()),
            _ => panic!("Unknown environment type"),
        };

        AppContext {
            config: Arc::new(config),
            strategy,
            chain,
            environment,
        }
    }

    pub fn from_file(config_path: &str) -> Self {
        let config = Config::from_file(config_path);

        let strategy: Arc<dyn Strategy> = match config.strategy_type.as_str() {
            "atomic_arb" => Arc::new(AtomicArbStrategy::new()),
            "liquidator" => Arc::new(LiquidatorStrategy::new()),
            _ => panic!("Unknown strategy type: {}", config.strategy_type),
        };

        let chain: Arc<dyn Chain> = match config.chain_type.as_str() {
            "ethereum" => Arc::new(EthereumChain::new()),
            "hyperevm" => Arc::new(HyperEVMChain::new()),
            _ => panic!("Unknown chain type: {}", config.chain_type),
        };

        let environment: Arc<dyn Environment> = match config.environment_type.as_str() {
            "local" => Arc::new(LocalEnvironment::new()),
            "vps" => Arc::new(VPSEnvironment::new()),
            _ => panic!("Unknown environment type: {}", config.environment_type),
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

pub struct Config {
    pub log_level: String,
    pub strategy_type: String,
    pub chain_type: String,
    pub environment_type: String,
}

impl Config {
    pub fn from_file(config_path: &str) -> Self {
        let config_content = fs::read_to_string(Path::new(config_path))
            .expect("Failed to read config file");
        let config: Value = config_content.parse::<Value>()
            .expect("Failed to parse config file");

        Config {
            log_level: config["log_level"].as_str().unwrap_or("info").to_string(),
            strategy_type: config["strategy_type"].as_str().unwrap_or("atomic_arb").to_string(),
            chain_type: config["chain_type"].as_str().unwrap_or("ethereum").to_string(),
            environment_type: config["environment_type"].as_str().unwrap_or("local").to_string(),
        }
    }
}
