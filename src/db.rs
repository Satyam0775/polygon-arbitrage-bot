use rusqlite::{params, Connection, Result};
use crate::arbitrage::ArbitrageResult;

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("arbitrage_log.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS opportunities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT DEFAULT (datetime('now')),
            buy_dex TEXT NOT NULL,
            sell_dex TEXT NOT NULL,
            profit REAL NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn log_opportunity(conn: &Connection, arb: &ArbitrageResult) -> Result<()> {
    conn.execute(
        "INSERT INTO opportunities (buy_dex, sell_dex, profit) VALUES (?1, ?2, ?3)",
        params![arb.buy_dex, arb.sell_dex, arb.profit],
    )?;
    Ok(())
}
