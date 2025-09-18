use ethers::providers::{Provider, Http, Middleware}; // 👈 add Middleware here
use std::sync::Arc;
use ethers::core::types::U64;

#[derive(Clone, Debug)]
pub struct DexPrice {
    pub dex_name: String,
    pub price: f64, // price in USDC for 1 WETH (example)
}

/// Fetch dummy-but-deterministic prices using chain block number to vary them.
/// This proves the RPC works and gives you changing values to test arbitrage logic.
pub async fn fetch_prices(provider: Arc<Provider<Http>>) -> eyre::Result<Vec<DexPrice>> {
    // Get the current block number from the provider
    let block_number: U64 = provider.get_block_number().await?;
    let bn = block_number.as_u64();

    // Use block number to create two different prices (deterministic)
    // base prices around 1600 USDC for 1 WETH (example)
    let base: f64 = 1600.0;
    let p1 = base + ((bn % 13) as f64) - 3.0; // e.g., vary -3..+9
    let p2 = base + ((bn % 11) as f64) - 1.0; // e.g., vary -1..+9

    let prices = vec![
        DexPrice { dex_name: "QuickSwap".to_string(), price: (p1 * 100.0).round() / 100.0 },
        DexPrice { dex_name: "SushiSwap".to_string(), price: (p2 * 100.0).round() / 100.0 },
    ];

    Ok(prices)
}
