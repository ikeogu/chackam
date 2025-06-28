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
            println!("Paper BUY executed @ {:.2}", price);
        } else {
            println!("Insufficient balance to BUY.");
        }
    }

    pub fn sell(&mut self, price: &f64, config: &Config) {
        if self.base_balance >= config.trade_amount {
            self.base_balance -= config.trade_amount;
            self.quote_balance += price * config.trade_amount;
            self.has_btc = false;
            println!("Paper SELL executed @ {:.2}", price);
        } else {
            println!("Not enough BTC to SELL.");
        }
    }
}
