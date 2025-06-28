use ta::indicators::SimpleMovingAverage;
use ta::Next;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn load_csv(path: &str) -> Vec<f64> {
  let file = File::open(path).expect("Could not open file");
  let reader = BufReader::new(file);

  reader
      .lines()
      .skip(1) // Skip the header row
      .filter_map(|line| {
          if let Ok(l) = line {
              let cols: Vec<&str> = l.split(',').collect();
              if cols.len() > 4 {
                  return cols[4].parse::<f64>().ok(); // Safely parse close price
              }
          }
          None
      })
      .collect()
}

#[allow(dead_code)]
pub fn backtest(prices: &[f64]) {
    let mut sma_short = SimpleMovingAverage::new(10).unwrap();
    let mut sma_long = SimpleMovingAverage::new(50).unwrap();

    let mut balance = 1000.0;
    let mut btc = 0.0;
    let mut trade_count = 0;
    let mut buy_count = 0;
    let mut sell_count = 0;


    println!("Starting backtest with ${:.2} and 0 BTC", balance);

    for i in 1..prices.len() {
        let price = prices[i];
       // let _prev_price = prices[i - 1];

        let short_prev = sma_short.next(prices[i - 1]);
        let long_prev = sma_long.next(prices[i - 1]);

        let short = sma_short.next(price);
        let long = sma_long.next(price);

        // Detect crossover
        if short_prev < long_prev && short > long {
            println!("BUY @ {:.2} — crossover", price);
            // BUY
            if balance > 0.0 {
                let buy_btc = balance / price;
                btc += buy_btc;
                println!("BUY @ {:.2} → Bought {:.6} BTC", price, buy_btc);
                balance = 0.0;
                //last_signal = "BUY";
                buy_count += 1;
                trade_count += 1;

            }
        } else if short_prev > long_prev && short < long {
            println!("SELL @ {:.2} — crossunder", price);
            // SELL
            if btc > 0.0 {
                let sell_value = btc * price;
                balance += sell_value;
                println!("SELL @ {:.2} → Sold {:.6} BTC", price, btc);
                btc = 0.0;
               // last_signal = "SELL";
                trade_count += 1;
                sell_count += 1;
            }
        }
    }


    let final_value = balance + btc * prices.last().unwrap();

    println!("🟢 Buys: {}, 🔴 Sells: {}", buy_count, sell_count);
    println!("🔁 Total Trades: {}", trade_count);
    println!("📈 Total Profit: {:.2}%", (final_value - 1000.0) / 1000.0 * 100.0);

    println!("\n🔚 Final Value: ${:.2}", final_value);
    println!("Remaining USD: ${:.2}, BTC: {:.6}", balance, btc);
}

