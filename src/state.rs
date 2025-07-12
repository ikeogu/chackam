// rust_trading_bot/src/state.rs

use crate::config::Config;
use crate::alert::send_alert;

pub struct Position {
    pub quote_balance: f64,
    pub base_balance: f64,
    pub has_btc: bool,
    pub last_buy_price: Option<f64>,
}

impl Position {
    pub fn new(config: &Config) -> Self {
        Self {
            quote_balance: config.start_balance,
            base_balance: 0.0,
            has_btc: false,
            last_buy_price: None,
        }
    }

    pub async fn buy(&mut self, price: &f64, _config: &Config) {
        let max_affordable_amount = self.quote_balance / price;

        if max_affordable_amount >= 0.000001 {
            let cost = max_affordable_amount * price;
            self.quote_balance -= cost;
            self.base_balance += max_affordable_amount;
            self.has_btc = true;
            self.last_buy_price = Some(*price);

            let message = format!(
                "🟢 BUY @ {:.2} — Bought {:.6} units, Spent {:.2}. New Balance → USDT: {:.2}, BTC: {:.6}",
                price,
                max_affordable_amount,
                cost,
                self.quote_balance,
                self.base_balance
            );

            println!("{message}");
            send_alert(&message).await;
        } else {
            let msg = format!(
                "❌ Insufficient funds: Cannot buy minimum amount with {:.2} USDT",
                self.quote_balance
            );

            println!("{msg}");
            send_alert(&msg).await;
        }
    }
    

    pub async fn sell(&mut self, price: &f64, config: &Config) {
        if self.base_balance >= 0.000001 {
            let revenue = self.base_balance * price;
            let message = format!(
                "🔴 SELL @ {:.2} — Sold {:.6} units, Received {:.2}.\n💰 New Balance → USDT: {:.2}, BTC: {:.6}",
                price,
                self.base_balance,
                revenue,
                self.quote_balance + revenue,
                0.0
            );

            self.quote_balance += revenue;
            self.base_balance = 0.0;
            self.has_btc = false;
            self.last_buy_price = None;

            println!("{message}");
            send_alert(&message).await;
        } else {
            let msg = format!(
                "❌ Not enough asset to SELL: Balance = {:.6}",
                self.base_balance
            );
            println!("{msg}");
            send_alert(&msg).await;
        }
    }

    
}
