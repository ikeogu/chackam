// rust_trading_bot/src/state.rs

use crate::config::Config;

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

    pub fn buy(&mut self, price: &f64, _config: &Config) {
        let max_affordable_amount = self.quote_balance / price;

        if max_affordable_amount >= 0.000001 {
            let cost = max_affordable_amount * price;
            self.quote_balance -= cost;
            self.base_balance += max_affordable_amount;
            self.has_btc = true;
            self.last_buy_price = Some(*price);

            println!(
                "🟢 BUY @ {:.2} — Bought {:.6} units, Spent {:.2}. New Balance → USDT: {:.2}, BTC: {:.6}",
                price,
                max_affordable_amount,
                cost,
                self.quote_balance,
                self.base_balance
            );
        } else {
            println!(
                "❌ Insufficient funds: Cannot buy minimum amount with {:.2} USDT",
                self.quote_balance
            );
    }
    }
    

    pub fn sell(&mut self, price: &f64, _config: &Config) {
        if self.base_balance >= 0.000001 {
            let revenue = self.base_balance * price;
            println!(
                "🔴 SELL @ {:.2} — Sold {:.6} units, Received {:.2}.",
                price,
                self.base_balance,
                revenue
            );
            self.quote_balance += revenue;
            self.base_balance = 0.0;
            self.has_btc = false;
            self.last_buy_price = None;
    
            println!(
                "💰 New Balance → USDT: {:.2}, BTC: {:.6}",
                self.quote_balance,
                self.base_balance
            );
        } else {
            println!(
                "❌ Not enough asset to SELL: Balance = {:.6}",
                self.base_balance
            );
        }
    
    }
    
}
