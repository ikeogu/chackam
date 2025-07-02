use reqwest::Client;
use std::{env, time::{SystemTime, UNIX_EPOCH}};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;
use url::form_urlencoded;
use serde_json::Value;


pub async fn fetch_klines(client: &Client, symbol: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
  let res = client
      .get("https://api.binance.com/api/v3/klines")
      .query(&[
          ("symbol", symbol),
          ("interval", "5m"),
          ("limit", "100"),
      ])
      .send()
      .await?
      .json::<Vec<Vec<Value>>>()
      .await?;

  let prices = res
      .iter()
      .filter_map(|row| row.get(4))              // 5th column = close price
      .filter_map(|val| val.as_str())            // it’s a string
      .filter_map(|s| s.parse::<f64>().ok())     // convert to float
      .collect();

  Ok(prices)
}
fn sign_query(secret: &str, query: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(query.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

fn get_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}

pub async fn place_market_order(
    client: &Client,
    symbol: &str,
    side: &str,
    quantity: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let key = env::var("BINANCE_API_KEY")?;
    let secret = env::var("BINANCE_SECRET_KEY")?;
    let timestamp = get_timestamp();

    let raw_query = form_urlencoded::Serializer::new(String::new())
        .append_pair("symbol", symbol)
        .append_pair("side", side)
        .append_pair("type", "MARKET")
        .append_pair("quantity", &quantity.to_string())
        .append_pair("timestamp", &timestamp.to_string())
        .finish();

    let signature = sign_query(&secret, &raw_query);
    let url = format!("https://api.binance.com/api/v3/order?{}&signature={}", raw_query, signature);

    let res = client
        .post(&url)
        .header("X-MBX-APIKEY", key)
        .send()
        .await?;

    println!("Order Response: {}", res.text().await?);
    Ok(())
}
