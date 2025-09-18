use crate::dex::DexPrice;

pub struct ArbitrageResult {
    pub buy_dex: String,
    pub sell_dex: String,
    pub profit: f64,
}

pub fn detect_arbitrage(prices: &[DexPrice], trade_size: f64, threshold: f64) -> Option<ArbitrageResult> {
    if prices.len() < 2 {
        return None;
    }

    let mut best_buy = &prices[0];
    let mut best_sell = &prices[0];

    for p in prices.iter() {
        if p.price < best_buy.price {
            best_buy = p;
        }
        if p.price > best_sell.price {
            best_sell = p;
        }
    }

    let profit = (best_sell.price - best_buy.price) * trade_size;

    if profit > threshold {
        Some(ArbitrageResult {
            buy_dex: best_buy.dex_name.clone(),
            sell_dex: best_sell.dex_name.clone(),
            profit,
        })
    } else {
        None
    }
}
 
