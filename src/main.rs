extern crate binance;
#[macro_use]
extern crate error_chain;

pub mod allocation;
pub mod rebalance;
pub mod balances;
pub mod execute;
pub mod trades;
pub mod prices;
pub mod pamr;
pub mod market_state;

use allocation::allocation;
use binance::api::Binance;
use rebalance::rebalance;
use balances::balances;
use execute::execute;
use prices::prices;
use pamr::pamr;
use market_state::market_state;

fn main() {
    let market = Binance::new(None, None);
    let account = Binance::new(
        Some("Z8EUkK26VxSbFY3CYutpEcP4R0FPfjVMND5gn7qXjAxiRcW2smsb1SbOFeOdunQ5".into()),
        Some("X0iPDL8FUlFPFFlNfysk338M3BOp7iYAsAKQNFhy1B51oW0GUjdlSMRqvKhLn3aL".into()),
    );

    let coins = ["BTC", "ADA", "BNB", "XLM"];

    let (prices, balances) = match market_state(&market, &account, &coins) {
        Ok(prices_balances) => prices_balances,
        Err(_) => (Vec::new(), Vec::new()),
    };
    println!("{:?}", prices);
    println!("{:?}", balances);
    let (total, current_allocation) = allocation(&balances, &prices);
    println!("{:?}", total);
    println!("{:?}", current_allocation);

    let target = pamr(0.7, 0.3, &prices, &prices, &current_allocation);
    println!("{:?}", target);
    let result = rebalance(
        &account,
        &prices,
        &current_allocation,
        &total,
        &coins,
        &target,
        0.0,
    );

    println!("{:?}", target);
    println!("{:?}", result);
}
