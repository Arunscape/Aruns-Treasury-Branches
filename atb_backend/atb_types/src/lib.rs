#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::collections::HashMap;

pub mod prelude;


#[derive(Debug)]
pub struct User {
    uuid: String,
    accounts: Vec<Account>,
}


#[derive(Debug)]
pub struct Account {
    account_id: String,
    nickname: String,
    owner: String,
    balances: HashMap<String, i64>,
}

#[derive(Debug)]
pub struct Transaction {
    id: i64,
    from_account_id: String,
    to_account_id: String,
    item: String,
    quantity: i64,
    price: i64,
}

#[derive(Debug, Eq, PartialEq)]
pub enum OrderType {
    Market,
    Limit(i64),
}
