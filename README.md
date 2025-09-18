# Polygon Arbitrage Opportunity Detector Bot

## 📌 Introduction
This is a Rust-based bot that detects potential arbitrage opportunities on the **Polygon (Matic) Network**.  
Arbitrage means finding a situation where a token (e.g., USDC, WETH, WBTC) can be bought cheaply on one DEX and sold at a higher price on another DEX.

---

## 🎯 Goal
- Periodically check token prices (example: **WETH/USDC**) on two DEXes (QuickSwap, SushiSwap).
- Detect arbitrage opportunities if the price difference exceeds a defined threshold.
- Simulate profit for a fixed trade size.
- Log opportunities into a local SQLite database (`arbitrage_log.db`).
- Configurable RPC URL, token pair, thresholds via `.env` file.

---

## 📦 Technology Stack
- **Blockchain**: Polygon PoS  
- **DEX Interaction**: QuickSwap, SushiSwap (Uniswap V2-like ABIs assumed)  
- **Tokens**: WETH, WBTC, USDC (demo uses deterministic price simulation)  
- **Language**: Rust  
- **Database**: SQLite  

---

## ⚙️ Setup

### 1. Install Rust
```bash
https://www.rust-lang.org/tools/install
2. Clone the repository
bash
Copy code
git clone https://github.com/Satyam0775/polygon-arbitrage-bot.git
cd polygon-arbitrage-bot
3. Configure .env
Create a .env file in the root folder:

ini
Copy code
POLYGON_RPC_URL=https://polygon-mainnet.g.alchemy.com/v2/<your-api-key>
TRADE_SIZE=1.0
PROFIT_THRESHOLD=5.0
CHECK_INTERVAL_SECONDS=10
⚠️ Replace <your-api-key> with your free Alchemy Polygon RPC key.

4. Run the bot
bash
Copy code
cargo run
🖥️ Sample Output (Demo)
yaml
Copy code
🚀 Starting Polygon Arbitrage Bot
Using RPC: https://polygon-mainnet.g.alchemy.com/v2/<your-key>
✅ Connected to Polygon! Latest block: 40785212

Checking prices...
QuickSwap Price: 1597.00 USDC
SushiSwap Price: 1604.00 USDC
💰 Arbitrage Opportunity! Buy on QuickSwap and Sell on SushiSwap | Profit: 7.00 USDC
✅ Logged opportunity to DB (arbitrage_log.db)

Checking prices...
QuickSwap Price: 1602.00 USDC
SushiSwap Price: 1600.00 USDC
No arbitrage found this cycle.

Checking prices...
QuickSwap Price: 1589.00 USDC
SushiSwap Price: 1603.00 USDC
💰 Arbitrage Opportunity! Buy on QuickSwap and Sell on SushiSwap | Profit: 14.00 USDC
✅ Logged opportunity to DB (arbitrage_log.db)
🗄️ Database Example (arbitrage_log.db)
id	timestamp	buy_dex	sell_dex	profit
1	2025-09-18 10:25:42	QuickSwap	SushiSwap	7.0
2	2025-09-18 10:26:11	QuickSwap	SushiSwap	14.0

📑 Deliverables Achieved
✅ Multi-DEX Price Fetching (RPC + simulated logic)

✅ Arbitrage Detection (compare prices, threshold check)

✅ Simulated Profit Calculation

✅ Config Management via .env

✅ SQLite Logging

✅ Sample Output + DB Schema

🚀 Notes
On Linux/Mac this project runs without file-lock issues.

On Windows, if you face os error 32 (file lock), use WSL/Ubuntu for smooth compilation.

This bot is for educational/demo purposes only, not financial advice.

🏁 Conclusion
This project demonstrates how to:

Connect to Polygon via RPC,

Fetch multi-DEX token prices,

Detect arbitrage opportunities,

Simulate profits,

Log opportunities for later review.
