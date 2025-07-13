use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::sync::Arc;
use dotenv::dotenv;

use crate::module::lifi::find_routes::LifiRouteResponse;
use crate::chain::Chain;

/// トランザクション応答の構造体（実際のAPIレスポンスに合わせて修正）
#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionResponse {
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub id: String,
    pub tool: String,
    #[serde(rename = "toolDetails")]
    pub tool_details: crate::module::lifi::find_routes::ToolDetails,
    pub action: crate::module::lifi::find_routes::RouteAction,
    pub estimate: crate::module::lifi::find_routes::StepEstimate,
    #[serde(rename = "includedSteps")]
    pub included_steps: Vec<crate::module::lifi::find_routes::IncludedStep>,
    pub integrator: String,
    pub referrer: String,
    #[serde(rename = "transactionRequest")]
    pub transaction_request: TransactionRequest,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionRequest {
    pub data: String,
    pub to: String,
    pub value: String,
    pub from: String,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<String>,
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: Option<String>,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: Option<String>,
}

/// ルートIDを指定してトランザクションを構築する関数（DI対応）
pub async fn build_transaction(
    response: &LifiRouteResponse, 
    route_id: Option<&str>,
    chain: Arc<dyn Chain>
) -> Result<TransactionResponse, Box<dyn Error>> {
    // .envファイルを読み込む
    dotenv().ok();
    
    // 環境変数からAPIキーを取得
    let api_key = env::var("LIFI_API_KEYS").expect("LIFI_API_KEYS must be set in .env file");
    
    // 使用するルートを決定
    let route = if let Some(id) = route_id {
        // 指定されたIDのルートを検索
        response.routes.iter()
            .find(|r| r.id == id)
            .ok_or_else(|| format!("Route with ID {} not found", id))?
    } else {
        // 最初のルートを使用
        response.routes.first()
            .ok_or_else(|| "No routes available")?
    };
    
    // ルートの最初のステップを使用
    if route.steps.is_empty() {
        return Err("Selected route has no steps".into());
    }
    
    // トランザクションに使用するステップをクローン
    let mut step = route.steps[0].clone();
    
    // fromAddressとtoAddressを設定
    // DIから注入されたChainを使用してEVMアドレスを取得
    let wallet_address = chain.get_primary_wallet_address()?;
    let wallet_address_str = format!("{:#x}", wallet_address);
    
    if step.action.from_address.is_none() {
        step.action.from_address = Some(wallet_address_str.clone());
    }
    
    if step.action.to_address.is_none() {
        // トークンの送信先（通常は同じアドレス）
        step.action.to_address = Some(wallet_address_str);
    }
    
    // ヘッダーを設定
    let mut headers = HeaderMap::new();
    headers.insert("x-lifi-api-key", HeaderValue::from_str(&api_key)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
    // APIエンドポイントにPOSTリクエストを送信する前に、ステップJSONをログ出力
    let step_json = serde_json::to_string_pretty(&step).unwrap_or_else(|_| format!("{:?}", step));
    println!("Sending step to API: {}", step_json);
    
    let client = reqwest::Client::new();
    let response = client
        .post("https://li.quest/v1/advanced/stepTransaction?skipSimulation=true")
        .headers(headers)
        .json(&step)
        .send()
        .await?;
    
    // レスポンスを処理
    if response.status().is_success() {
        // まずJSONテキストを取得
        let json_text = response.text().await?;
        println!("Transaction API Response: {}", json_text);
        
        // JSONテキストをパース
        match serde_json::from_str::<TransactionResponse>(&json_text) {
            Ok(tx_response) => Ok(tx_response),
            Err(e) => {
                println!("Transaction JSON parse error: {}", e);
                Err(format!("Failed to parse transaction API response: {}", e).into())
            }
        }
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        println!("Transaction API error (status {}): {}", status, error_text);
        Err(format!("API request failed with status {}: {}", status, error_text).into())
    }
}

/// 最適なルートのトランザクションを構築する関数（最初のルートを使用）（DI対応）
pub async fn build_best_route_transaction(
    response: &LifiRouteResponse,
    chain: Arc<dyn Chain>
) -> Result<TransactionResponse, Box<dyn Error>> {
    build_transaction(response, None, chain).await
}
