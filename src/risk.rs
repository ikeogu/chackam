// src/risk.rs
use crate::{config::Config, state::Position};
use std::time::Duration;

pub async fn check_stop_loss_take_profit(
    price: f64,
    position: &mut Position,
    config: &Config,
) -> bool {
    let buy_price = position.quote_balance;
    let loss_trigger = buy_price * (1.0 - config.stop_loss_pct);
    let profit_trigger = buy_price * (1.0 + config.take_profit_pct);

    if price <= loss_trigger {
        println!("🛑 Stop loss triggered at ${:.2}", price);
        position.sell(&price, config);
        crate::alert::send_alert(&format!("🛑 STOP LOSS triggered at ${:.2}", price)).await;
        return true; // exit trade
    }

    if price >= profit_trigger {
        println!("🎯 Take profit triggered at ${:.2}", price);
        position.sell(&price, config);
        crate::alert::send_alert(&format!("🎯 TAKE PROFIT triggered at ${:.2}", price)).await;
        return true; // exit trade
    }

    false // no exit triggered
}
