pub fn is_forex_pair(symbol: &str) -> bool {
  symbol.len() == 6 && symbol.chars().all(|c| c.is_ascii_alphabetic())
}
