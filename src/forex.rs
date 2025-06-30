use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct TimeSeries {
    #[serde(rename = "Time Series FX (60min)")]
    time_series: std::collections::HashMap<String, ForexCandle>,
}

#[derive(Debug, Deserialize)]
struct ForexCandle {
    #[serde(rename = "4. close")]
    close: String,
}

pub async fn fetch_forex_klines(
    client: &Client,
    pair: &str,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let api_key = env::var("ALPHAVANTAGE_API_KEY")?;
    let from = &pair[0..3];
    let to = &pair[3..];

    let url = format!(
        "https://www.alphavantage.co/query?function=FX_INTRADAY&from_symbol={}&to_symbol={}&interval=60min&apikey={}",
        from, to, api_key
    );

    let resp = client.get(&url).send().await?.json::<TimeSeries>().await?;

    let mut closes: Vec<f64> = resp
        .time_series
        .iter()
        .filter_map(|(_, candle)| candle.close.parse::<f64>().ok())
        .collect();

    closes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    Ok(closes)
}
