// rust_trading_bot/src/state.rs

use crate::config::Config;

pub struct Position {
    pub quote_balance: f64,
    pub base_balance: f64,
    pub has_btc: bool,
}

impl Position {
    pub fn new() -> Self {
        Self {
            quote_balance: 1000.0,
            base_balance: 0.0,
            has_btc: false,
        }
    }

    pub fn buy(&mut self, price: &f64, config: &Config) {
        let cost = price * config.trade_amount;
        if self.quote_balance >= cost {
            self.quote_balance -= cost;
            self.base_balance += config.trade_amount;
            self.has_btc = true;
            println!(
                "🟢 BUY @ {:.2} — Bought {:.6} units, Spent {:.2}. New Balance → USD: {:.2}, Asset: {:.6}",
                price,
                config.trade_amount,
                cost,
                self.quote_balance,
                self.base_balance
            );
        } else {
            println!(
                "❌ Insufficient balance to BUY: Need {:.2}, Available {:.2}",
                cost,
                self.quote_balance
            );
        }
    }
    

    pub fn sell(&mut self, price: &f64, config: &Config) {
        if self.base_balance >= config.trade_amount {
            self.base_balance -= config.trade_amount;
            let revenue = price * config.trade_amount;
            self.quote_balance += revenue;
            self.has_btc = false;
            println!(
                "🔴 SELL @ {:.2} — Sold {:.6} units, Received {:.2}. New Balance → USD: {:.2}, Asset: {:.6}",
                price,
                config.trade_amount,
                revenue,
                self.quote_balance,
                self.base_balance
            );
        } else {
            println!(
                "❌ Not enough asset to SELL: Trying to sell {:.6}, Available {:.6}",
                config.trade_amount,
                self.base_balance
            );
        }
    }
    
}
