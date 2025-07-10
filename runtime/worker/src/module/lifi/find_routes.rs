use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use dotenv::dotenv;

// Li.Fi APIのリクエストボディ用構造体
#[derive(Debug)]
pub struct LifiRouteRequest {
    pub from_chain_id: u64,
    pub from_amount: String,
    pub from_token_address: String,
    pub to_chain_id: u64,
    pub to_token_address: String,
    pub options: LifiRouteOptions,
}

#[derive(Debug)]
pub struct LifiRouteOptions {
    pub integrator: String,
    pub referrer: String,
    pub slippage: f64,
    pub fee: f64,
    pub bridges: LifiFilterOptions,
    pub exchanges: LifiFilterOptions,
    pub allow_switch_chain: bool,
    pub order: String,
    pub max_price_impact: f64,
}

#[derive(Serialize, Debug)]
pub struct LifiFilterOptions {
    pub allow: Vec<String>,
}

// Li.FiのAPIレスポンス用構造体 (必要に応じて拡張)
#[derive(Deserialize, Debug)]
pub struct LifiRouteResponse {
    pub routes: Vec<LifiRoute>,
    #[serde(rename = "unavailableRoutes")]
    pub unavailable_routes: UnavailableRoutes,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UnavailableRoutes {
    #[serde(rename = "filteredOut")]
    pub filtered_out: Vec<serde_json::Value>,
    pub failed: Vec<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LifiRoute {
    pub id: String,
    #[serde(rename = "fromChainId")]
    pub from_chain_id: u64,
    #[serde(rename = "fromAmountUSD")]
    pub from_amount_usd: String,
    #[serde(rename = "fromAmount")]
    pub from_amount: String,
    #[serde(rename = "fromToken")]
    pub from_token: Token,
    #[serde(rename = "toChainId")]
    pub to_chain_id: u64,
    #[serde(rename = "toAmountUSD")]
    pub to_amount_usd: String,
    #[serde(rename = "toAmount")]
    pub to_amount: String,
    #[serde(rename = "toAmountMin")]
    pub to_amount_min: String,
    #[serde(rename = "toToken")]
    pub to_token: Token,
    #[serde(rename = "gasCostUSD")]
    pub gas_cost_usd: String,
    #[serde(rename = "containsSwitchChain")]
    pub contains_switch_chain: bool,
    pub steps: Vec<RouteStep>,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ToolDetails {
    pub key: String,
    pub name: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RouteAction {
    #[serde(rename = "fromToken")]
    pub from_token: Token,
    #[serde(rename = "fromAmount")]
    pub from_amount: String,
    #[serde(rename = "toToken")]
    pub to_token: Token,
    #[serde(rename = "fromChainId")]
    pub from_chain_id: u64,
    #[serde(rename = "toChainId")]
    pub to_chain_id: u64,
    pub slippage: f64,
    #[serde(rename = "fromAddress", default)]
    pub from_address: Option<String>,
    #[serde(rename = "toAddress", default)]
    pub to_address: Option<String>,
    // 追加のオプションフィールド
    #[serde(rename = "infiniteApproval", default)]
    pub infinite_approval: Option<bool>,
    #[serde(rename = "allowSwitchChain", default)]
    pub allow_switch_chain: Option<bool>,
    #[serde(rename = "executionDuration", default)]
    pub execution_duration: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Token {
    pub address: String,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    pub symbol: String,
    pub decimals: u8,
    pub name: String,
    #[serde(rename = "coinKey")]
    pub coin_key: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
    #[serde(rename = "priceUSD")]
    pub price_usd: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RouteEstimate {
    pub tool: String,
    #[serde(rename = "approvalAddress")]
    pub approval_address: String,
    #[serde(rename = "toAmountMin")]
    pub to_amount_min: String,
    #[serde(rename = "toAmount")]
    pub to_amount: String,
    #[serde(rename = "fromAmount")]
    pub from_amount: String,
    #[serde(rename = "feeCosts")]
    pub fee_costs: Vec<FeeCost>,
    #[serde(rename = "gasCosts")]
    pub gas_costs: Vec<GasCost>,
    #[serde(rename = "executionDuration")]
    pub execution_duration: u32,
    #[serde(rename = "fromAmountUSD")]
    pub from_amount_usd: String,
    #[serde(rename = "toAmountUSD")]
    pub to_amount_usd: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FeeCost {
    pub name: String,
    pub description: String,
    pub token: Token,
    pub amount: String,
    #[serde(rename = "amountUSD")]
    pub amount_usd: String,
    pub percentage: String,
    pub included: bool,
    #[serde(rename = "feeSplit", default)]
    pub fee_split: Option<FeeSplit>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FeeSplit {
    #[serde(rename = "integratorFee")]
    pub integrator_fee: String,
    #[serde(rename = "lifiFee")]
    pub lifi_fee: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GasCost {
    #[serde(rename = "type")]
    pub gas_type: String,
    pub price: String,
    pub estimate: String,
    pub limit: String,
    pub amount: String,
    #[serde(rename = "amountUSD")]
    pub amount_usd: String,
    pub token: Token,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RouteStep {
    pub id: String,
    #[serde(rename = "type")]
    pub step_type: String,
    pub tool: String,
    #[serde(rename = "toolDetails")]
    pub tool_details: ToolDetails,
    pub action: RouteAction,
    pub estimate: StepEstimate,
    #[serde(rename = "includedSteps")]
    pub included_steps: Vec<IncludedStep>,
    pub integrator: String,
    pub referrer: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IncludedStep {
    pub id: String,
    #[serde(rename = "type")]
    pub step_type: String,
    pub action: RouteAction,
    pub estimate: StepEstimate,
    pub tool: String,
    #[serde(rename = "toolDetails")]
    pub tool_details: ToolDetails,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StepEstimate {
    #[serde(rename = "fromAmount")]
    pub from_amount: String,
    #[serde(rename = "toAmount")]
    pub to_amount: String,
    #[serde(rename = "toAmountMin")]
    pub to_amount_min: String,
    pub tool: String,
    #[serde(rename = "approvalAddress")]
    pub approval_address: String,
    #[serde(rename = "executionDuration")]
    pub execution_duration: u32,
    #[serde(rename = "feeCosts")]
    pub fee_costs: Vec<FeeCost>,
    #[serde(rename = "gasCosts")]
    pub gas_costs: Vec<GasCost>,
}

// カスタムSerialize実装（JSONフィールド名をAPIに合わせる）
impl Serialize for LifiRouteRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("LifiRouteRequest", 6)?;
        state.serialize_field("fromChainId", &self.from_chain_id)?;
        state.serialize_field("fromAmount", &self.from_amount)?;
        state.serialize_field("fromTokenAddress", &self.from_token_address)?;
        state.serialize_field("toChainId", &self.to_chain_id)?;
        state.serialize_field("toTokenAddress", &self.to_token_address)?;
        state.serialize_field("options", &self.options)?;
        state.end()
    }
}

// LifiRouteOptionsのSerializeは自動で生成されるように別途実装
impl Serialize for LifiRouteOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("LifiRouteOptions", 9)?;
        state.serialize_field("integrator", &self.integrator)?;
        state.serialize_field("referrer", &self.referrer)?;
        state.serialize_field("slippage", &self.slippage)?;
        state.serialize_field("fee", &self.fee)?;
        state.serialize_field("bridges", &self.bridges)?;
        state.serialize_field("exchanges", &self.exchanges)?;
        state.serialize_field("allowSwitchChain", &self.allow_switch_chain)?;
        state.serialize_field("order", &self.order)?;
        state.serialize_field("maxPriceImpact", &self.max_price_impact)?;
        state.end()
    }
}

// ルート検索のパラメータ構造体
#[derive(Debug)]
pub struct RouteSearchParams {
    pub from_chain_id: u64,
    pub from_amount: String,
    pub from_token_address: String,
    pub to_chain_id: u64,
    pub to_token_address: String,
    pub slippage: f64,
    pub integrator: Option<String>,
    pub referrer: Option<String>,
    pub fee: Option<f64>,
    pub order: Option<String>,
    pub max_price_impact: Option<f64>,
}

impl Default for RouteSearchParams {
    fn default() -> Self {
        Self {
            from_chain_id: 999,
            from_amount: "1000000000000000000".to_string(),
            from_token_address: "0xBe6727B535545C67d5cAa73dEa54865B92CF7907".to_string(),
            to_chain_id: 999,
            to_token_address: "0x5555555555555555555555555555555555555555".to_string(),
            slippage: 0.003,
            integrator: None,
            referrer: None,
            fee: None,
            order: None,
            max_price_impact: None,
        }
    }
}

pub async fn find_routes(params: RouteSearchParams) -> Result<LifiRouteResponse, Box<dyn Error>> {
    // .envファイルを読み込む
    dotenv().ok();
    
    // 環境変数からAPIキーを取得
    let api_key = env::var("LIFI_API_KEYS").expect("LIFI_API_KEYS must be set in .env file");
    
    // ヘッダーを設定
    let mut headers = HeaderMap::new();
    headers.insert("x-lifi-api-key", HeaderValue::from_str(&api_key)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
    // リクエストボディを構築
    let request_body = LifiRouteRequest {
        from_chain_id: params.from_chain_id,
        from_amount: params.from_amount,
        from_token_address: params.from_token_address,
        to_chain_id: params.to_chain_id,
        to_token_address: params.to_token_address,
        options: LifiRouteOptions {
            integrator: params.integrator.unwrap_or_else(|| "fee-demo".to_string()),
            referrer: params.referrer.unwrap_or_else(|| "0x552008c0f6870c2f77e5cC1d2eb9bdff03e30Ea0".to_string()),
            slippage: params.slippage,
            fee: params.fee.unwrap_or(0.0),
            bridges: LifiFilterOptions {
                allow: vec!["relay".to_string()],
            },
            exchanges: LifiFilterOptions {
                allow: vec!["1inch".to_string(), "openocean".to_string()],
            },
            allow_switch_chain: true,
            order: params.order.unwrap_or_else(|| "CHEAPEST".to_string()),
            max_price_impact: params.max_price_impact.unwrap_or(0.1),
        },
    };
    
    // APIエンドポイントにPOSTリクエストを送信
    let client = reqwest::Client::new();
    let response = client
        .post("https://li.quest/v1/advanced/routes")
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;
    
    // レスポンスを処理
    if response.status().is_success() {
        // まずJSONテキストを取得
        let json_text = response.text().await?;
        println!("API Response: {}", json_text);
        
        // JSONテキストをパース
        match serde_json::from_str::<LifiRouteResponse>(&json_text) {
            Ok(routes_response) => Ok(routes_response),
            Err(e) => {
                println!("JSON parse error: {}", e);
                Err(format!("Failed to parse API response: {}", e).into())
            }
        }
    } else {
        let error_text = response.text().await?;
        Err(format!("API request failed: {}", error_text).into())
    }
}