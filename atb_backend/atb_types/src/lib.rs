#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::collections::HashMap;
use uuid::Uuid;

pub mod prelude;

#[derive(Debug, 
    // sqlx::FromRow
)] 
pub struct User {
    pub id: Uuid,
}

#[derive(Debug)]
pub struct Account {
    pub id: Uuid,
    pub userid: Uuid,
    pub nickname: String,
}

#[derive(Debug)]
pub struct Balance {
    pub accountid: Uuid,
    pub item: String,
    pub amount: i64,
}

#[derive(Debug)]
pub struct McItem {
    pub id: String
}

#[derive(Debug)]
pub struct Transaction {
    pub id: Uuid,
    pub time_millis: chrono::DateTime<chrono::Utc>,
    pub fromid: Uuid,
    pub toid: Uuid,
    pub item: String,
    pub quantity: i64,
    pub price: i64,
}

#[derive(Debug, Eq, PartialEq)]
pub enum OrderType {
    Market,
    Limit(i64),
}
