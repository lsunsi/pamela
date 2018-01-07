use binance::market::Market;
use binance::model::Prices;

#[derive(Debug)]
pub enum Error {
    FetchFailure,
    PriceMissing,
}

pub fn prices(market: &Market, coins: &[&str], base: &str) -> Result<Vec<f64>, Error> {
    let all_prices = match market.get_all_prices() {
        Ok(Prices::AllPrices(prices)) => prices,
        Err(_) => return Err(Error::FetchFailure),
    };

    let mut prices = vec![];

    for coin in coins {
        if *coin == base {
            prices.push(1.);
            continue;
        }

        let m = format!("{}{}", coin, base);

        let price_option = all_prices
            .iter()
            .find(|sp| sp.symbol == m)
            .and_then(|sp| sp.price.parse().ok());

        match price_option {
            Some(price) => prices.push(price),
            None => return Err(Error::PriceMissing),
        };
    }

    Ok(prices)
}
