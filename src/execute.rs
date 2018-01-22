use binance::account::Account;

fn truncate(number: f64, precision: i32) -> f64 {
    let a = (10 as f64).powi(precision);
    (number * a).trunc() / a
}

pub fn execute(account: &Account, base: &str, trades: &[(String, f64)]) -> Result<(), String> {
    for &(ref coin, raw_amount) in trades {
        let amount = truncate(raw_amount, 0);
        let m = format!("{}{}", coin, base);
        println!("{:?}", m);

        let result = if amount > 0. {
            account.test_market_buy(m, amount)
        } else {
            println!("{:?}", m);
            account.test_market_sell(m, -amount)
        };

        if let Err(err) = result {
            return Err(err.description().into());
        }
    }

    Ok(())
}
