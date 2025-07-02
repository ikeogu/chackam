// rust_trading_bot/src/main.rs

mod binance;
mod strategy;
mod alert;
mod backtest;
mod config;
mod state;
mod fetch_candles;
mod risk;
mod forex;
mod utils;

use crate::binance::{fetch_klines, place_market_order};
use crate::strategy::generate_signal;
use crate::alert::send_alert;
use tokio::time::{sleep, Duration};
use crate::forex::fetch_forex_klines;
use crate::utils::is_forex_pair;

use crate::{config::*, state::*};
use reqwest::Client;
use dotenv::dotenv;
use crate::risk::check_stop_loss_take_profit;
//use fetch_candles::fetch_binance_klines;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let client = Client::new();

    let settings = Config::load();
    let mut position = Position::new(&settings);

    // Uncomment this for backtesting
  /*   let prices = fetch_binance_klines("BTCUSDT", "1h", 5000).await?;
    println!("Fetched {} candles from Binance", prices.len()); 
    backtest(&prices);
    return Ok(()); */ 


    loop {
        // Fetch latest prices
      
        let prices = match if is_forex_pair(&settings.trading_pair) {
            fetch_forex_klines(&client, &settings.trading_pair).await
        } else {
            fetch_klines(&client, &settings.trading_pair).await
        } {
            Ok(p) => p,
            Err(e) => {
                eprintln!("❌ Failed to fetch prices: {}", e);
                sleep(Duration::from_secs(30)).await;
                continue;
            }
        };

        if let Some(&latest_price) = prices.last() {
            println!("📈 Latest price: {:.2}", latest_price);
            if check_stop_loss_take_profit(latest_price, &mut position, &settings).await {
                continue;
            }
        }

        // Generate signal
        let signal = generate_signal(&prices);
        
        println!("📡 Signal: {}", signal);

        // Handle signal
        match signal.as_ref() {
            "BUY" => {
                if position.has_btc {
                    println!("⚠️ Already holding asset, skipping BUY.");
                } else {
                    let price = *prices.last().unwrap();
                    println!("📥 Executing BUY at {:.2}", price);
                    position.buy(&price, &settings);

                    if settings.live_mode {
                        place_market_order(&client, &settings.trading_pair, "BUY", settings.trade_amount).await?;
                    }

                    send_alert(&format!("🟢 BUY signal executed at ${:.2}", price)).await;
                }
            }

            "SELL" => {
                if position.has_btc {
                    let price = *prices.last().unwrap();
                    println!("📤 Executing SELL at {:.2}", price);
                    position.sell(&price, &settings);

                    if settings.live_mode {
                        place_market_order(&client, &settings.trading_pair, "SELL", settings.trade_amount).await?;
                    }

                    send_alert(&format!("🔴 SELL signal executed at ${:.2}", price)).await;
                } else {
                    println!("⚠️ No BTC to sell, skipping SELL.");
                }
            }

            _ => {
                println!("⏸️ No actionable signal.");
            }
        }

        // Print current balance
       
        let trade_pair = std::env::var("TRADE_PAIR").unwrap_or_else(|_| "BTCUSDT".to_string());
        let (base, quote) = parse_trade_pair(&trade_pair);

        println!(
            "💰 {}: {:.2}, {}: {:.6}",
            quote,
            position.quote_balance,
            base,
            position.base_balance
        );

        // Wait for the next interval
        sleep(Duration::from_secs(settings.interval_secs)).await;
    }

}

fn parse_trade_pair(pair: &str) -> (String, String) {
    let mut quote_assets = known_quote_assets();
    quote_assets.sort_by(|a, b| b.len().cmp(&a.len())); // sort longest first

    for quote in &quote_assets {
        if pair.ends_with(quote) {
            let base = &pair[..pair.len() - quote.len()];
            return (base.to_string(), quote.to_string());
        }
    }

    ("UNKNOWN".to_string(), "UNKNOWN".to_string())
}

fn known_quote_assets() -> Vec<&'static str> {
    vec![
        "USDT", "USDC", "BUSD", "USD", "BTC", "ETH", "BNB", "TRY", "EUR", "TUSD",
        "FDUSD", "DAI", "AUD", "GBP", "RUB", "BRL", "NGN", "IDR", "PHP", "ARS", "ZAR", "JPY", "SOL"
    ]
}
