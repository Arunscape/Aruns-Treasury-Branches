#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod prelude;

#[derive(
    Debug,
    // sqlx::FromRow
)]
pub struct User {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub userid: Uuid,
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub accountid: Uuid,
    pub item: String,
    pub quantity: i64,
}

#[derive(Debug)]
pub struct McItem {
    pub id: String,
}

#[derive(Debug)]
pub struct Transaction {
    pub id: Uuid,
    pub time_processed: chrono::DateTime<chrono::Utc>,
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
