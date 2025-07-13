use std::sync::Arc;
use crate::module::lifi::{self, RouteSearchParams, build_best_route_transaction};
use crate::chain::Chain;
use crate::env::Environment;

pub struct AtomicArbStrategy {
    chain: Arc<dyn Chain>,
    environment: Arc<dyn Environment>,
}

impl AtomicArbStrategy {
    pub fn new(chain: Arc<dyn Chain>, environment: Arc<dyn Environment>) -> Self {
        Self {
            chain,
            environment,
        }
    }
    
    async fn find_arbitrage_routes(&self, search_params: RouteSearchParams) -> Result<(), Box<dyn std::error::Error>> {
        println!("Finding arbitrage opportunities with Li.Fi...");
        
        match lifi::find_routes(search_params).await {
            Ok(response) => {
                println!("Successfully found {} routes", response.routes.len());
                
                if response.routes.is_empty() {
                    println!("No arbitrage routes found");
                    return Ok(());
                }
                
                // ルートの詳細情報を表示
                for (idx, route) in response.routes.iter().enumerate() {
                    println!("\n===== Route {} - ID: {} =====", idx + 1, route.id);
                    println!("From: {} {} (${}) -> To: {} {} (${}) (min: {})", 
                        route.from_token.symbol, 
                        route.from_amount,
                        route.from_amount_usd,
                        route.to_token.symbol,
                        route.to_amount,
                        route.to_amount_usd,
                        route.to_amount_min
                    );
                    println!("Gas cost USD: {}", route.gas_cost_usd);
                    println!("Contains switch chain: {}", route.contains_switch_chain);
                    println!("Tags: {}", route.tags.join(", "));
                    
                    // ステップ情報も表示
                    println!("\nSteps:");
                    for (step_idx, step) in route.steps.iter().enumerate() {
                        println!("  Step {}: {} using {}", step_idx + 1, step.step_type, step.tool);
                        println!("    From {} {} -> To {} {}", 
                            step.action.from_token.symbol,
                            step.action.from_amount,
                            step.action.to_token.symbol,
                            step.estimate.to_amount
                        );
                        println!("    FromAddress: {:?}, ToAddress: {:?}", 
                            step.action.from_address,
                            step.action.to_address
                        );
                    }
                    println!("==============================\n");
                }
                
                // 最適なルート（最初のルート）からトランザクションを構築
                println!("Building transaction for the best route...");
                match build_best_route_transaction(&response, self.chain.clone()).await {
                    Ok(tx) => {
                        println!("Transaction built successfully!");
                        println!("Transaction to: {}", tx.transaction_request.to);
                        println!("Gas limit: {}", tx.transaction_request.gas_limit);
                        println!("Value: {}", tx.transaction_request.value);
                        // 実際のトランザクション実行はここに追加
                    },
                    Err(e) => {
                        println!("Failed to build transaction: {}", e);
                    }
                }
                
                Ok(())
            },
            Err(e) => {
                println!("Error finding routes: {}", e);
                Err(e)
            }
        }
    }
}

impl crate::strategy::Strategy for AtomicArbStrategy {
    fn execute(&self) {
        println!("Executing Atomic Arbitrage Strategy");
        
        // カスタムパラメータでLi.Fiのルート検索を実行
        let search_params = RouteSearchParams {
            from_chain_id: 999,
            from_amount: "1000000000000000000".to_string(),
            from_token_address: "0xBe6727B535545C67d5cAa73dEa54865B92CF7907".to_string(),
            to_chain_id: 999,
            to_token_address: "0x5555555555555555555555555555555555555555".to_string(),
            slippage: 0.003,
            fee: Some(0.0), // 0%の手数料
            ..Default::default()
        };
        
        // 非同期処理を実行するためのランタイム
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            if let Err(e) = self.find_arbitrage_routes(search_params).await {
                eprintln!("Failed to execute arbitrage strategy: {}", e);
            }
        });
        
        println!("Arbitrage execution completed");
    }
}
