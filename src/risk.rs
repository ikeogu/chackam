// src/risk.rs
use crate::{config::Config, state::Position};

pub async fn check_stop_loss_take_profit(
    price: f64,
    position: &mut Position,
    config: &Config,
) -> bool {
    if let Some(buy_price) = position.last_buy_price {
        let loss_trigger = buy_price * (1.0 - config.stop_loss_pct);
        let profit_trigger = buy_price * (1.0 + config.take_profit_pct);

        // 🛑 STOP LOSS
        if price <= loss_trigger {
            if position.base_balance >= config.trade_amount {
                println!("🛑 Stop loss triggered at ${:.2}", price);
                position.sell(&price, config);
                crate::alert::send_alert(&format!("🛑 STOP LOSS triggered at ${:.2}", price)).await;
                position.last_buy_price = None; // reset
                return true;
            } else {
                println!(
                    "❌ Not enough asset to SELL (STOP LOSS): Trying to sell {:.6}, Available {:.6}",
                    config.trade_amount, position.base_balance
                );
            }
        }

        // 🎯 TAKE PROFIT
        if price >= profit_trigger {
            if position.base_balance >= config.trade_amount {
                println!("🎯 Take profit triggered at ${:.2}", price);
                position.sell(&price, config);
                crate::alert::send_alert(&format!("🎯 TAKE PROFIT triggered at ${:.2}", price)).await;
                position.last_buy_price = None; // reset
                return true;
            } else {
                println!(
                    "❌ Not enough asset to SELL (TAKE PROFIT): Trying to sell {:.6}, Available {:.6}",
                    config.trade_amount, position.base_balance
                );
            }
        }
    }

    false // no stop-loss or take-profit triggered
}

