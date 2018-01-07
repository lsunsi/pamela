extern crate binance;
#[macro_use]
extern crate error_chain;

pub mod allocation;
pub mod rebalance;
pub mod balances;
pub mod execute;
pub mod trades;
pub mod prices;

use allocation::allocation;
use binance::api::Binance;
use rebalance::rebalance;
use balances::balances;
use execute::execute;
use prices::prices;
use trades::trades;

fn main() {
    let market = Binance::new(None, None);
    let account = Binance::new(Some("api-key".into()), Some("secret".into()));

    let coins = ["BTC", "ADA", "BNB", "XLM"];
    let target = [0.0, 0.0, 0.0, 1.0];

    let result = rebalance(&market, &account, &coins, &target, 0.0);

    println!("{:?}", result);
}
