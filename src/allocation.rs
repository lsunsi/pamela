pub fn allocation(balances: &[f64], prices: &[f64]) -> (f64, Vec<f64>) {
    let mut base_balances = vec![];

    for (balance, price) in balances.iter().zip(prices) {
        base_balances.push(balance * price);
    }

    let total: f64 = base_balances.iter().sum();

    let mut allocation = vec![];

    for base_balance in base_balances {
        allocation.push(base_balance / total);
    }

    (total, allocation)
}
