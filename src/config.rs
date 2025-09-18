use dotenv::dotenv;
use std::env;

pub struct Config {
    pub rpc_url: String,
    pub trade_size: f64,
    pub profit_threshold: f64,
    pub check_interval_seconds: u64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let rpc_url = env::var("POLYGON_RPC_URL").expect("POLYGON_RPC_URL must be set in .env");
        let trade_size = env::var("TRADE_SIZE").unwrap_or_else(|_| "1.0".to_string()).parse().expect("TRADE_SIZE must be a number");
        let profit_threshold = env::var("PROFIT_THRESHOLD").unwrap_or_else(|_| "5.0".to_string()).parse().expect("PROFIT_THRESHOLD must be a number");
        let check_interval_seconds = env::var("CHECK_INTERVAL_SECONDS").unwrap_or_else(|_| "10".to_string()).parse().expect("CHECK_INTERVAL_SECONDS must be a number");

        Self {
            rpc_url,
            trade_size,
            profit_threshold,
            check_interval_seconds,
        }
    }
}
 
