use binance::account::Account;
use binance::market::Market;
use trades::trades;
use allocation;
use balances;
use execute;
use prices;

#[derive(Debug)]
pub enum Error {
    BalancesError(balances::Error),
    PricesError(prices::Error),
    ExecuteError(String),
}

impl From<balances::Error> for Error {
    fn from(err: balances::Error) -> Error {
        Error::BalancesError(err)
    }
}

impl From<prices::Error> for Error {
    fn from(err: prices::Error) -> Error {
        Error::PricesError(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::ExecuteError(err)
    }
}

pub fn rebalance(
    market: &Market,
    account: &Account,
    coins: &[&str],
    target: &[f64],
    margin: f64,
) -> Result<Vec<(String, f64)>, Error> {
    let base = coins[0];

    let balances = balances::balances(account, coins)?;
    let prices = prices::prices(market, coins, base)?;
    let (total, allocation) = allocation::allocation(&balances, &prices);

    let trades = trades(total * (1. - margin), &prices, &allocation, target, coins);
    // execute(&account, &base, &trades)?;

    Ok(trades)
}
