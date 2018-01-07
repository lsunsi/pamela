pub fn trades(
    total: f64,
    prices: &[f64],
    current: &[f64],
    target: &[f64],
    coins: &[&str],
) -> Vec<(String, f64)> {
    let mut trades = vec![];

    for i in (1..prices.len()).rev() {
        let diff = target[i] - current[i];
        if diff != 0. {
            let value = total * diff / prices[i];
            trades.push((coins[i].to_owned(), value));
        }
    }

    trades.sort_unstable_by(|&(_, v1), &(_, v2)| v1.partial_cmp(&v2).unwrap());
    trades
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let total = 5.;
        let prices = [1., 0.5, 0.01, 0.9];
        let current = [0.4, 0.25, 0.25, 0.1];
        let target = [0.0, 0.15, 0.75, 0.1];
        let coins = ["BTC", "ETH", "XLM", "ADA"];

        let trades = generate(total, &prices, &current, &target, &coins);

        assert_eq!(trades, [("ETH".into(), -1.), ("XLM".into(), 250.)]);
    }
}
