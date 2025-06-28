// src/fetch_candles.rs

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Candle {
    pub open_time: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}

#[allow(dead_code)]
pub async fn fetch_binance_klines(symbol: &str, interval: &str, limit: u32) -> Result<Vec<f64>, reqwest::Error> {
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&interval={}&limit={}",
        symbol, interval, limit
    );

    let response = reqwest::get(&url).await?;
    let candles: Vec<Vec<serde_json::Value>> = response.json().await?;

    // Parse close prices
    let closes = candles
        .into_iter()
        .filter_map(|entry| {
            entry.get(4)?.as_str()?.parse::<f64>().ok()
        })
        .collect();

    Ok(closes)
}
