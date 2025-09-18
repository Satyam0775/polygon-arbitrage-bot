mod config;
mod dex;
mod arbitrage;
mod db;

use config::Config;
use dex::fetch_prices;
use arbitrage::detect_arbitrage;
use db::{init_db, log_opportunity};

use ethers::providers::{Provider, Http, Middleware}; // 👈 add Middleware here
use std::sync::Arc;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load config
    let cfg = Config::from_env();

    println!("🚀 Starting Polygon Arbitrage Bot");
    println!("Using RPC: {}", cfg.rpc_url);

    // 2. Connect to provider
    let provider = Provider::<Http>::try_from(cfg.rpc_url.as_str())?;
    let provider = Arc::new(provider);

    // quick RPC test: latest block
    match provider.get_block_number().await {
        Ok(bn) => println!("✅ Connected to Polygon! Latest block: {}", bn),
        Err(e) => {
            println!("❌ Failed to get block number from RPC: {:?}", e);
            return Err(e.into());
        }
    }

    // 3. Init DB
    let conn = init_db().expect("Failed to init DB");

    // 4. Main loop
    loop {
        println!("\nChecking prices...");
        match fetch_prices(provider.clone()).await {
            Ok(prices) => {
                for p in &prices {
                    println!("{} Price: {:.2} USDC", p.dex_name, p.price);
                }

                if let Some(op) = detect_arbitrage(&prices, cfg.trade_size, cfg.profit_threshold) {
                    println!("💰 Arbitrage Opportunity! Buy on {} and Sell on {} | Profit: {:.2} USDC",
                        op.buy_dex, op.sell_dex, op.profit);
                    if let Err(e) = log_opportunity(&conn, &op) {
                        println!("⚠️ Failed to log opportunity to DB: {:?}", e);
                    } else {
                        println!("✅ Logged opportunity to DB (arbitrage_log.db)");
                    }
                } else {
                    println!("No arbitrage found this cycle.");
                }
            }
            Err(e) => {
                println!("❌ Failed to fetch prices: {:?}", e);
            }
        }

        let wait = std::time::Duration::from_secs(cfg.check_interval_seconds);
        tokio::time::sleep(wait).await;
    }
}
