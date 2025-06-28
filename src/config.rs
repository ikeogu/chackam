// src/config.rs

use std::env;

pub struct Config {
    pub trading_pair: String,
    pub trade_amount: f64,
    pub interval_secs: u64,
    pub live_mode: bool,
}

impl Config {
    pub fn load() -> Self {
        dotenv::dotenv().ok();

        let trading_pair = env::var("TRADE_PAIR").unwrap_or_else(|_| "BTCUSDT".into());

        let trade_amount = env::var("TRADE_AMOUNT")
            .unwrap_or_else(|_| "100.0".into())
            .parse::<f64>()
            .unwrap_or_else(|_| {
                eprintln!("⚠️  Invalid TRADE_AMOUNT. Using default of 100.0");
                100.0
            });

        let interval_secs = env::var("INTERVAL_SECS")
            .unwrap_or_else(|_| "60".into())
            .parse::<u64>()
            .unwrap_or_else(|_| {
                eprintln!("⚠️  Invalid INTERVAL_SECS. Using default of 60 seconds.");
                60
            });

        let live_mode = env::var("LIVE_MODE").unwrap_or_else(|_| "false".into()) == "true";

        Self {
            trading_pair,
            trade_amount,
            interval_secs,
            live_mode,
        }
    }
}
