// Entry point for Rust worker bot

mod di;
mod chain;

fn main() {
    // Initialize dotenv to load .env file
    dotenv::dotenv().ok();
    
    println!("Initializing worker bot...");

    // Test wallet address conversion
    println!("Testing wallet address conversion...");
    match chain::hyperevm::get_primary_wallet_address() {
        Ok(address) => {
            println!("✅ Primary wallet address: {:#x}", address);
        }
        Err(e) => {
            println!("❌ Failed to get primary wallet address: {}", e);
        }
    }

    let context = di::AppContext::new();

    // Setup environment
    context.environment.setup();

    // Connect to chain
    context.chain.connect();

    // Execute strategy
    context.strategy.execute();

    println!("Worker bot execution completed.");
}
