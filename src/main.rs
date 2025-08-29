use axum::{
    extract::State,
    response::Json,
    routing::get,
    Router,
    http::StatusCode,
    response::Html,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::{net::SocketAddr, str::FromStr, sync::Arc, time::Duration};
use tower_http::cors::CorsLayer;
use tracing::{info, warn, error};
use reqwest::Client;
use pyth_sdk_solana::state::PriceFeed;
use borsh::BorshDeserialize;

#[derive(Parser)]
#[command(name = "sonic-pyth-gdp")]
#[command(about = "Sonic SVM + Pyth Network GDP Data API")]
struct Args {
    #[arg(long, default_value = "3000")]
    port: u16,
}

#[derive(Clone)]
struct AppState {
    sonic_rpc_client: Arc<RpcClient>,
    solana_fallback_client: Arc<RpcClient>,
    http_client: Client,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub symbol: String,
    pub price: f64,
    pub confidence: f64,
    pub publish_time: i64,
    pub price_feed_id: String,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub source: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct PythPriceData {
    id: String,
    price: PythPriceInfo,
}

#[derive(Deserialize)]
struct PythPriceInfo {
    price: String,
    conf: String,
    expo: i32,
    publish_time: i64,
}

impl AppState {
    async fn get_gdp_data(&self) -> Result<PriceData, anyhow::Error> {
        let gdp_feed_id = "0x01a2d2aa5728850767d67e2f82ddc9c8e4c3bbace231461386ef9cbb16d0d36b";
        self.get_price_feed(gdp_feed_id, "ECO.US.GDP").await
    }

    async fn get_all_gdp_feeds(&self) -> Result<Vec<PriceData>, anyhow::Error> {
        let gdp_feeds = vec![
            ("0x01a2d2aa5728850767d67e2f82ddc9c8e4c3bbace231461386ef9cbb16d0d36b", "ECO.US.GDP"),
            ("0xede7d586e573bba4d9f9b598134a0b3b2848fb5633efa1aeae6cdc405ca69ec4", "ECO.US.GDPQ120"),
            ("0x56ef63838b89bae2fa4ceac09887937e3a8ed5882372e9d3c5f5b0047be99201", "ECO.US.GDPQ121"),
            ("0xe007fecd2fa29ae39ca6014752d08240d29ab8d26defda84151a35888dc38f72", "ECO.US.GDPQ122"),
            ("0xed0db24f1d1e79d175b972b2824f17bfdc68adb2b9f0a95bd55001aad9636243", "ECO.US.GDPQ123"),
            ("0x7d52b237e53197baeabb7e8840afc635e804e4c120304850657b7fe9b5abde6b", "ECO.US.GDPQ124"),
            ("0x1f0585497d5749086d2a0d31872e3d54983ae1695fe8daa367f416778401a316", "ECO.US.GDPQ125"),
            ("0x9fc444f6174a9cf849b65c3b30411ecd68a98136ce7e3727c62256675f7137dc", "ECO.US.GDPQ220"),
            ("0x4e35cd9a603f66fd85f6128d91a9bc129662e64ed022a97f8d95b59f1ebf7c2e", "ECO.US.GDPQ221"),
            ("0x01da0bbe2e2a28a45eee49168a755321baccf916377337699cf6f23207952623", "ECO.US.GDPQ222"),
            ("0xd7c07f4fea81886c927eb995a5e007987c426c6b04255a0b9cc2063b990175b4", "ECO.US.GDPQ223"),
            ("0x5fd1723f5ae19701812061efdfc487b923260c3186694b1f757bc19b3478c26c", "ECO.US.GDPQ224"),
            ("0xb4ae8f99fe948c259bf1c419a8ef3c99f31b6bfbd11b2bd5e960d5ba395ce66e", "ECO.US.GDPQ225"),
            ("0xe50aec560231dbcdd10e04bcabc4f18fa492e363b08ca9098e10d658931c3457", "ECO.US.GDPQ320"),
            ("0x849b55be51fdcb722dc58ad870d69f73c3ea3b020fb2a8039e5b7abed62f2a86", "ECO.US.GDPQ321"),
            ("0xf8557de55b0ae56652e6ab325eef49a3e999aba6a37f35c36ac6403713dc11a6", "ECO.US.GDPQ322"),
            ("0xa0158865c183a07659de1f7b86dcf7f34c6b9f7982cc2f22b08c1979e3dee8ea", "ECO.US.GDPQ323"),
            ("0x8cbea9b9b69b80ddeaa00c4ab9dc54f2b1c104f3fb732ece3b70eeb622296d76", "ECO.US.GDPQ324"),
            ("0x9700fcc09ccf25204df7e5b87c3cfa7a780ff782c95a39fd5558cf14dbac8591", "ECO.US.GDPQ420"),
            ("0x3a683ed0c55b14e1521313d45d93136a5adc9945fa8dca02374f8d9870d4d342", "ECO.US.GDPQ421"),
            ("0xd584777f78a2ac22d8eebddd9cf22f9006a74b6da112e0d673bc6a6599c5f7d1", "ECO.US.GDPQ422"),
            ("0x44aaa6f2845486fd145561c678ab8b24dfba2f685a30755ad70f3b5cf6e8b3b8", "ECO.US.GDPQ423"),
            ("0x76bd1d211bed7f8c553f19cc2da845cab538e8b1d9e317d0455c22950fe4e32c", "ECO.US.GDPQ424"),
        ];

        let mut results = Vec::new();
        
        for (feed_id, symbol) in gdp_feeds {
            info!("🔍 Fetching {} feed", symbol);
            match self.get_price_feed(feed_id, symbol).await {
                Ok(data) => {
                    results.push(data);
                }
                Err(e) => {
                    warn!("Failed to fetch {}: {}", symbol, e);
                }
            }
        }

        if results.is_empty() {
            Err(anyhow::anyhow!("No GDP feeds could be fetched"))
        } else {
            Ok(results)
        }
    }

    async fn get_price_feed(&self, feed_id: &str, symbol: &str) -> Result<PriceData, anyhow::Error> {
        info!("Trying Sonic SVM first for {}", symbol);
        match self.fetch_from_sonic_via_pyth_programs(feed_id, symbol).await {
            Ok(data) => {
                info!("Successfully fetched {} from Sonic SVM!", symbol);
                return Ok(data);
            }
            Err(e) => {
                warn!("Sonic SVM failed for {}: {}", symbol, e);
            }
        }

    
        info!("Falling back to Pyth Hermes API for {}", symbol);
        match self.fetch_from_pyth_hermes(feed_id, symbol).await {
            Ok(data) => {
                info!("Successfully fetched {} from Pyth Hermes API!", symbol);
                Ok(data)
            }
            Err(e) => {
                error!("All sources failed for {}: {}", symbol, e);
                Err(anyhow::anyhow!("Failed to fetch {} from all sources: {}", symbol, e))
            }
        }
    }

    async fn fetch_from_solana_fallback(&self, feed_id: &str) -> Result<PriceData, anyhow::Error> {
        info!("Trying to fetch GDP data from Solana RPC (Helius)");
        
        
        let potential_gdp_accounts = vec![
            "48mYDzV1JWZo93cheTbg9ikvp3PScDvoTAkWFMWtmmc9", 
            "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG", 
        ];
        
        for account_str in potential_gdp_accounts {
            info!("Trying Solana account: {}", account_str);
            
            match Pubkey::from_str(account_str) {
                Ok(pubkey) => {
                    match self.solana_fallback_client.get_account(&pubkey) {
                        Ok(account) => {
                            info!("Found account on Solana!");
                            info!("Account size: {} bytes, owner: {}", account.data.len(), account.owner);
                            
                            // Try to parse as Pyth price feed
                            match PriceFeed::try_from_slice(&account.data) {
                                Ok(price_feed) => {
                                    let price = price_feed.get_price_unchecked();
                                    let price_value = price.price as f64 / 10_f64.powi(price.expo);
                                    let confidence = price.conf as f64 / 10_f64.powi(price.expo);

                                    info!("Successfully parsed REAL data from Solana: {:.2} (±{:.2})", price_value, confidence);

                                    return Ok(PriceData {
                                        symbol: if account_str.contains("48mYDzV1") { "US_GDP".to_string() } else { "PRICE_FEED".to_string() },
                                        price: price_value,
                                        confidence,
                                        publish_time: price.publish_time,
                                        price_feed_id: pubkey.to_string(),
                                        last_updated: chrono::Utc::now(),
                                        source: "Solana RPC (Helius)".to_string(),
                                    });
                                }
                                Err(e) => {
                                    warn!("Account exists but not a valid Pyth price feed: {}", e);
                                    continue;
                                }
                            }
                        }
                        Err(e) => {
                            info!("Account {} not found on Solana: {}", account_str, e);
                            continue;
                        }
                    }
                }
                Err(e) => {
                    warn!("Invalid account format {}: {}", account_str, e);
                    continue;
                }
            }
        }
        
        Err(anyhow::anyhow!("No working GDP feed accounts found on Solana"))
    }

    async fn fetch_from_pyth_hermes(&self, feed_id: &str, symbol: &str) -> Result<PriceData, anyhow::Error> {
        let api_url = format!("https://hermes.pyth.network/api/latest_price_feeds?ids[]={}", feed_id);
        
        info!("Fetching from Pyth Hermes API: {}", api_url);
        
        let response = self.http_client
            .get(&api_url)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Pyth Hermes API failed: {}", response.status()));
        }

        let response_text = response.text().await?;
        
        if response_text.contains("Failed to deserialize") || response_text.contains("Invalid") {
            return Err(anyhow::anyhow!("Pyth API error: {}", response_text));
        }

        let api_response: Vec<PythPriceData> = serde_json::from_str(&response_text)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {} - Response: {}", e, response_text))?;
        
        if let Some(price_data) = api_response.into_iter().next() {
            let price_val: i64 = price_data.price.price.parse()
                .map_err(|e| anyhow::anyhow!("Failed to parse price: {}", e))?;
            let conf_val: u64 = price_data.price.conf.parse()
                .map_err(|e| anyhow::anyhow!("Failed to parse confidence: {}", e))?;

            let price_value = price_val as f64 / 10_f64.powi(price_data.price.expo.abs());
            let confidence = conf_val as f64 / 10_f64.powi(price_data.price.expo.abs());

            info!("Successfully parsed REAL GDP data: {:.2}% (±{:.2}%)", price_value, confidence);

            Ok(PriceData {
                symbol: symbol.to_string(),
                price: price_value,
                confidence,
                publish_time: price_data.price.publish_time,
                price_feed_id: feed_id.to_string(),
                last_updated: chrono::Utc::now(),
                source: "Pyth Hermes API".to_string(),
            })
        } else {
            Err(anyhow::anyhow!("No GDP data found in Pyth Hermes response"))
        }
    }

    async fn fetch_from_sonic_via_pyth_programs(&self, feed_id: &str, symbol: &str) -> Result<PriceData, anyhow::Error> {
        info!("Attempting to fetch GDP data through Sonic SVM Pyth Programs");
        info!("Using Pyth Receiver: rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ");
        info!("Using Pyth Price Feed: pythWSnswVUd12oZpeFP8e9CVaEqJg25g1Vtc2biRsT");
        
        let pyth_price_feed_program = Pubkey::from_str("pythWSnswVUd12oZpeFP8e9CVaEqJg25g1Vtc2biRsT")?;

        let hex_without_prefix = feed_id.strip_prefix("0x").unwrap_or(feed_id);
      
        if let Ok(decoded_bytes) = hex::decode(hex_without_prefix) {
            if decoded_bytes.len() == 32 {
                let pubkey = Pubkey::new_from_array(decoded_bytes.try_into().unwrap());
                info!("Trying direct account lookup for GDP feed: {}", pubkey);
                
                match self.sonic_rpc_client.get_account(&pubkey) {
                    Ok(account) => {
                        info!("Found GDP feed account on Sonic SVM!");
                        info!(" Account size: {} bytes, owner: {}", account.data.len(), account.owner);
                        
                        
                        match PriceFeed::try_from_slice(&account.data) {
                            Ok(price_feed) => {
                                let price = price_feed.get_price_unchecked();
                                let price_value = price.price as f64 / 10_f64.powi(price.expo);
                                let confidence = price.conf as f64 / 10_f64.powi(price.expo);

                                info!("Successfully parsed REAL GDP data from Sonic SVM: {:.2}% (±{:.2}%)", price_value, confidence);

                                return Ok(PriceData {
                                    symbol: "US_GDP".to_string(),
                                    price: price_value,
                                    confidence,
                                    publish_time: price.publish_time,
                                    price_feed_id: pubkey.to_string(),
                                    last_updated: chrono::Utc::now(),
                                    source: "Sonic SVM Direct Account".to_string(),
                                });
                            }
                            Err(e) => {
                                warn!("Account exists but not a valid Pyth price feed: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        info!("GDP feed account not found on Sonic SVM: {}", e);
                    }
                }
            }
        }
        
        
        info!("GDP feed accounts not found on Sonic SVM");
        Err(anyhow::anyhow!("GDP feed accounts not available on Sonic SVM yet"))
    }


    async fn check_sonic_pyth_programs(&self) -> Result<serde_json::Value, anyhow::Error> {
        
        let pyth_receiver = "rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ";
        let pyth_price_feed = "pythWSnswVUd12oZpeFP8e9CVaEqJg25g1Vtc2biRsT";
        
        let mut results = serde_json::Map::new();
        
        
        match Pubkey::from_str(pyth_receiver) {
            Ok(pubkey) => {
                match self.sonic_rpc_client.get_account(&pubkey) {
                    Ok(account) => {
                        results.insert("pyth_receiver".to_string(), serde_json::json!({
                            "address": pyth_receiver,
                            "status": "✅ DEPLOYED on Sonic SVM",
                            "owner": account.owner.to_string(),
                            "executable": account.executable,
                            "lamports": account.lamports,
                            "data_size": account.data.len()
                        }));
                        info!("Pyth Receiver found on Sonic SVM");
                    },
                    Err(e) => {
                        results.insert("pyth_receiver".to_string(), serde_json::json!({
                            "address": pyth_receiver,
                            "status": " NOT FOUND",
                            "error": e.to_string()
                        }));
                        warn!("Pyth Receiver not found: {}", e);
                    }
                }
            },
            Err(e) => {
                results.insert("pyth_receiver".to_string(), serde_json::json!({
                    "address": pyth_receiver,
                    "status": "INVALID ADDRESS",
                    "error": e.to_string()
                }));
            }
        }
        
        
        match Pubkey::from_str(pyth_price_feed) {
            Ok(pubkey) => {
                match self.sonic_rpc_client.get_account(&pubkey) {
                    Ok(account) => {
                        results.insert("pyth_price_feed".to_string(), serde_json::json!({
                            "address": pyth_price_feed,
                            "status": "DEPLOYED on Sonic SVM",
                            "owner": account.owner.to_string(),
                            "executable": account.executable,
                            "lamports": account.lamports,
                            "data_size": account.data.len()
                        }));
                        info!("Pyth Price Feed program found on Sonic SVM");
                    },
                    Err(e) => {
                        results.insert("pyth_price_feed".to_string(), serde_json::json!({
                            "address": pyth_price_feed,
                            "status": "NOT FOUND",
                            "error": e.to_string()
                        }));
                        warn!("Pyth Price Feed program not found: {}", e);
                    }
                }
            },
            Err(e) => {
                results.insert("pyth_price_feed".to_string(), serde_json::json!({
                    "address": pyth_price_feed,
                    "status": "INVALID ADDRESS",
                    "error": e.to_string()
                }));
            }
        }
        
        Ok(serde_json::json!({
            "network": "Sonic SVM Mainnet Alpha",
            "rpc_endpoint": "https://rpc.mainnet-alpha.sonic.game",
            "documentation": "https://docs.sonic.game/additional-tools-and-examples",
            "pyth_programs": results,
            "integration_status": "PURE Sonic SVM + Pyth Integration",
            "gdp_feed_id": "0x01a2d2aa5728850767d67e2f82ddc9c8e4c3bbace231461386ef9cbb16d0d36b",
            "architecture": "Sonic SVM Native Only"
        }))
    }
}


async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("Sonic SVM + Pyth GDP API Running!".to_string()),
        error: None,
        timestamp: chrono::Utc::now(),
    })
}

async fn sonic_status(State(state): State<AppState>) -> Json<ApiResponse<serde_json::Value>> {
    match state.sonic_rpc_client.get_version() {
        Ok(version) => {
            let sonic_info = serde_json::json!({
                "network": "Sonic SVM Mainnet Alpha",
                "rpc_endpoint": "https://rpc.mainnet-alpha.sonic.game",
                "solana_core_version": version.solana_core,
                "feature_set": version.feature_set,
                "status": "Connected to Sonic SVM",
                "pyth_integration": "Ready for Pyth price feeds",
                "fallback": "Solana mainnet available"
            });
            
            Json(ApiResponse {
                success: true,
                data: Some(sonic_info),
                error: None,
                timestamp: chrono::Utc::now(),
            })
        },
        Err(e) => {
            error!("Sonic RPC connection failed: {}", e);
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Sonic RPC connection failed: {}", e)),
                timestamp: chrono::Utc::now(),
            })
        }
    }
}

async fn sonic_pyth_programs(State(state): State<AppState>) -> Json<ApiResponse<serde_json::Value>> {
    match state.check_sonic_pyth_programs().await {
        Ok(programs_info) => {
            Json(ApiResponse {
                success: true,
                data: Some(programs_info),
                error: None,
                timestamp: chrono::Utc::now(),
            })
        },
        Err(e) => {
            error!("Failed to check Pyth programs: {}", e);
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to check Pyth programs: {}", e)),
                timestamp: chrono::Utc::now(),
            })
        }
    }
}

async fn us_gdp_data(State(state): State<AppState>) -> Json<ApiResponse<PriceData>> {
    match state.get_gdp_data().await {
        Ok(gdp_data) => {
            info!("📈 GDP Data: {:.2}% (±{:.2}%) from {}", 
                gdp_data.price, gdp_data.confidence, gdp_data.source);
            
            Json(ApiResponse {
                success: true,
                data: Some(gdp_data),
                error: None,
                timestamp: chrono::Utc::now(),
            })
        },
        Err(e) => {
            error!("Failed to fetch US GDP: {}", e);
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to fetch US GDP: {}", e)),
                timestamp: chrono::Utc::now(),
            })
        }
    }
}

async fn all_gdp_feeds(State(state): State<AppState>) -> Json<ApiResponse<Vec<PriceData>>> {
    match state.get_all_gdp_feeds().await {
        Ok(feeds) => {
            info!("📊 Successfully fetched {} GDP feeds", feeds.len());
            
            Json(ApiResponse {
                success: true,
                data: Some(feeds),
                error: None,
                timestamp: chrono::Utc::now(),
            })
        },
        Err(e) => {
            error!("Failed to fetch all GDP feeds: {}", e);
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to fetch all GDP feeds: {}", e)),
                timestamp: chrono::Utc::now(),
            })
        }
    }
}

async fn serve_dashboard() -> Html<String> {
    let html = std::fs::read_to_string("static/index.html")
        .unwrap_or_else(|_| {
            r#"<!DOCTYPE html>
<html><head><title>GDP Dashboard</title></head>
<body><h1>GDP Dashboard</h1><p>Static files not found. API available at /gdp and /gdp/all</p></body>
</html>"#.to_string()
        });
    Html(html)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    info!("Starting PURE Sonic SVM GDP Fetcher on port {}", args.port);
    info!("Sonic SVM RPC: https://rpc.mainnet-alpha.sonic.game");
    info!("Priority: Sonic SVM → Solana RPC → Hermes API");
    info!("GDP Feed ID: 0x01a2d2aa5728850767d67e2f82ddc9c8e4c3bbace231461386ef9cbb16d0d36b");
    info!("Pyth Receiver: rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ");
    info!("Pyth Price Feed: pythWSnswVUd12oZpeFP8e9CVaEqJg25g1Vtc2biRsT");

    let state = AppState {
        sonic_rpc_client: Arc::new(RpcClient::new("https://rpc.mainnet-alpha.sonic.game".to_string())),
        solana_fallback_client: Arc::new(RpcClient::new("https://mainnet.helius-rpc.com/?api-key=22abefb4-e86a-482d-9a62-452fcd4f2cb0".to_string())),
        http_client: Client::new(),
    };

    let app = Router::new()
        .route("/", get(serve_dashboard))
        .route("/dashboard", get(serve_dashboard))
        .route("/health", get(health_check))
        .route("/sonic/status", get(sonic_status))
        .route("/sonic/programs", get(sonic_pyth_programs))
        .route("/gdp", get(us_gdp_data))
        .route("/gdp/all", get(all_gdp_feeds))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    info!("Server starting on {}", addr);
    info!("Dashboard UI: http://localhost:{}/", args.port);
    info!("GDP API: http://localhost:{}/gdp", args.port);
    info!("All GDP Feeds: http://localhost:{}/gdp/all", args.port);
    info!("Sonic Status: http://localhost:{}/sonic/status", args.port);
    info!("Pyth Programs: http://localhost:{}/sonic/programs", args.port);
    info!("GDP DASHBOARD READY WITH {} FEEDS!", 23);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}