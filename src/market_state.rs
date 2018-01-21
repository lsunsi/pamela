use binance::account::Account;
use binance::market::Market;
use balances;
use prices;

#[derive(Debug)]
pub enum Error {
    BalancesError(balances::Error),
    PricesError(prices::Error),
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

pub fn market_state(
    market: &Market,
    account: &Account,
    coins: &[&str],
) -> Result<(Vec<f64>, Vec<f64>), Error> {
    let base = coins[0];

    let balances = balances::balances(&account, &coins)?;
    let prices = prices::prices(&market, &coins, &base)?;
    Ok((prices, balances))
}
