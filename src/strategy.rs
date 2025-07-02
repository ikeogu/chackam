use ta::indicators::SimpleMovingAverage;
use ta::Next;

pub fn generate_signal(closes: &[f64]) -> &'static str {
    let mut sma_short = SimpleMovingAverage::new(10).unwrap();
    let mut sma_long = SimpleMovingAverage::new(50).unwrap();
    let mut signals = vec![];

  
    for &price in closes {
        let short = sma_short.next(price);
        let long = sma_long.next(price);
        signals.push((price, short, long));
    }

    
    if let Some(&(prev_price, prev_short, prev_long)) = signals.get(signals.len().saturating_sub(2)) {
        let (last_price, last_short, last_long) = signals.last().unwrap();

        println!(
            "📈 Latest price: {:.2}, SMA_10: {:.2}, SMA_50: {:.2}",
            last_price, last_short, last_long
        );

        if prev_short < prev_long && last_short > last_long {
            return "BUY";
        } else if prev_short > prev_long && last_short < last_long {
            return "SELL";
        }
    }



    "HOLD"
}
