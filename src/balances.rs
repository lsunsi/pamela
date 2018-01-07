use binance::account::Account;

#[derive(Debug)]
pub enum Error {
    FetchFailure,
    BalanceMissing(String),
}

pub fn balances(account: &Account, coins: &[&str]) -> Result<Vec<f64>, Error> {
    let mut balances = vec![];

    let all_balances = match account.get_account() {
        Ok(account) => account.balances,
        Err(_) => return Err(Error::FetchFailure),
    };

    for coin in coins {
        let c = coin.to_string();

        let balance = all_balances
            .iter()
            .find(|balance| balance.asset == c)
            .and_then(|balance| balance.free.parse().ok());

        match balance {
            Some(balance) => balances.push(balance),
            None => return Err(Error::BalanceMissing(c)),
        };
    }

    Ok(balances)
}
