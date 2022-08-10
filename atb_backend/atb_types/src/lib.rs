use std::collections::HashMap;

pub mod prelude;

#[derive(Debug)]
pub struct Account {
    uuid: UUID,
    accounts: Vec<ATBAccount>,
}

type UUID = String;

#[derive(Debug)]
pub struct ATBAccount {
    account_id: UUID,
    nickname: String,
    owner: UUID,
    balances: HashMap<String, i128>,
}

#[derive(Debug)]
pub struct Transaction {
    id: UUID,
    from_account_id: String,
    to_account_id: String,
    item: String,
    quantity: String,
}

#[derive(Debug, Eq, PartialEq)]
pub enum OrderType {
    MarketBuy,
    MarketSell,
    LimitBuy(i128),
    LimitSell(i128),
}
