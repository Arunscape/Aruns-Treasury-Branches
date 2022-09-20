#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::collections::HashMap;

pub mod prelude;

#[derive(Debug, Clone)]
pub struct Account {
    uuid: UUID,
    accounts: Vec<ATBAccount>,
}

type UUID = String;

#[derive(Debug, Clone)]
pub struct ATBAccount {
    account_id: UUID,
    nickname: String,
    owner: UUID,
    balances: HashMap<String, i128>,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    // id: UUID,
    from_account_id: String,
    to_account_id: String,
    item: String,
    quantity: i128,
    price: i128,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OrderType {
    Market,
    Limit(i128),
}

impl Transaction {
    pub fn new(
        //id: &str,
        from_account_id: &str,
        to_account_id: &str,
        item: &str,
        quantity: i128,
        price: i128,
    ) -> Self {
        //let id = id.into();
        let from_account_id = from_account_id.into();
        let to_account_id = to_account_id.into();
        let item = item.into();

        Self {
            //id,
            from_account_id,
            to_account_id,
            item,
            quantity,
            price,
        }
    }
}