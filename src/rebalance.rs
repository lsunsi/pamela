use trades::trades;
use binance::account::Account;
use execute;

#[derive(Debug)]
pub enum Error {
    ExecuteError(String),
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::ExecuteError(err)
    }
}

pub fn rebalance(
    account: &Account,
    prices: &[f64],
    allocation: &[f64],
    total: &f64,
    coins: &[&str],
    target: &[f64],
    margin: f64,
) -> Result<Vec<(String, f64)>, Error> {
    let trades = trades(total * (1. - margin), prices, allocation, target, coins);
    execute(&account, coins[0], &trades)?;
    Ok(trades)
}
