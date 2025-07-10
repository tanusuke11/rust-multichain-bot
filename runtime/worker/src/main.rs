// Entry point for Rust worker bot

mod di;

fn main() {
    let context = di::AppContext::new();

    println!("Initializing worker bot...");

    // Setup environment
    context.environment.setup();

    // Connect to chain
    context.chain.connect();

    // Execute strategy
    context.strategy.execute();

    println!("Worker bot execution completed.");

}
