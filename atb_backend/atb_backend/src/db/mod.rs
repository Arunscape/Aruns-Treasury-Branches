use atb_types::prelude::*;
use lazy_static::lazy_static;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    query, query_as, query_file_as, PgConnection,
};
use std::env;
use uuid::Uuid;
//use sqlx::{Acquire, Connection};
use sqlx::Connection;

lazy_static! {
    static ref DB_URL: String =
        env::var("DATABASE_URL").unwrap_or("postgres://postgres@localhost/postgres".into());
}

/// Takes in a minecraft UUID
pub async fn new_user(conn: &mut PgConnection, id: Uuid) -> Result<User, sqlx::Error> {
    let user = query_file_as!(User, "./src/db/queries/new_user.sql", id)
        .fetch_one(conn)
        .await?;

    Ok(user)
}

pub async fn new_account(
    conn: &mut PgConnection,
    userid: Uuid,
    nickname: String,
) -> Result<Account, sqlx::Error> {
    let account = query_file_as!(
        Account,
        "./src/db/queries/new_account.sql",
        userid,
        nickname,
    )
    .fetch_one(conn)
    .await?;

    Ok(account)
}

pub async fn update_account_name(
    conn: &mut PgConnection,
    userid: Uuid,
    old_nickname: &str,
    new_nickname: &str,
) -> Result<Account, sqlx::Error> {
    let account = query_file_as!(
        Account,
        "./src/db/queries/update_account_name.sql",
        new_nickname,
        old_nickname,
        userid,
    )
    .fetch_one(conn)
    .await?;

    Ok(account)
}

pub async fn delete_account(
    conn: &mut PgConnection,
    accountid: Uuid,
    userid: Uuid,
) -> Result<Option<Account>, sqlx::Error> {
    let account = query_file_as!(
        Account,
        "./src/db/queries/delete_account.sql",
        accountid,
        userid
    )
    .fetch_optional(conn)
    .await?;

    Ok(account)
}

pub async fn get_accounts_for_user(
    conn: &mut PgConnection,
    id: Uuid,
) -> Result<Vec<Account>, sqlx::Error> {
    let accounts = query_file_as!(Account, "./src/db/queries/get_accounts_for_user.sql", id)
        .fetch_all(conn)
        .await?;

    Ok(accounts)
}

pub async fn get_transactions_for_account(
    conn: &mut PgConnection,
    id: Uuid,
) -> Result<Vec<Transaction>, sqlx::Error> {
    let transactions = query_file_as!(
        Transaction,
        "./src/db/queries/get_transactions_for_account.sql",
        id
    )
    .fetch_all(conn)
    .await?;

    Ok(transactions)
}

pub async fn get_balances_for_account(
    conn: &mut PgConnection,
    id: Uuid,
) -> Result<Vec<Balance>, sqlx::Error> {
    let balances = query_file_as!(Balance, "./src/db/queries/get_balances_for_account.sql", id)
        .fetch_all(conn)
        .await?;

    Ok(balances)
}

pub async fn transfer(
    conn: &mut PgConnection,
    fromid: Uuid,
    toid: Uuid,
    quantity: i64,
    item: &str,
    price: i64,
) -> Result<Transaction, sqlx::Error> {
    // BEGIN;
    // UPDATE balances SET amount = (amount - $3) WHERE accountid = $1 AND item = $4;
    // UPDATE balances SET amount = (amount + $3) WHERE accountid = $2 AND item = $4;

    // INSERT INTO transactions (fromid, toid, item, quantity, price) VALUES ($1, $2, $3, $4, $5) RETURNING *;
    // COMMIT;
    // ",
    // fromid,
    // toid,
    // quantity,
    // item,
    // price

    let mut transaction = conn.begin().await?;
    query!(
        "
        UPDATE balances SET quantity = (quantity - $1)
        FROM accounts
        WHERE accounts.userid = $2
        AND accountid = $2 AND item = $3
        ",
        quantity,
        fromid,
        item
    )
    .execute(&mut transaction)
    .await?;

    query!(
        "UPDATE balances SET quantity = (quantity + $1) WHERE accountid = $2 AND item = $3",
        quantity,
        toid,
        item
    )
    .execute(&mut transaction)
    .await?;

    let t = query_as!(Transaction, "INSERT INTO transactions (fromid, toid, item, quantity, price) VALUES ($1, $2, $3, $4, $5) RETURNING *", fromid, toid, item, quantity, price)
        .fetch_one(&mut transaction)
        .await?;

    transaction.commit().await?;

    Ok(t)
}

pub async fn deposit(
    conn: &mut PgConnection,
    accountid: Uuid,
    item: &str,
    quantity: i64,
) -> Result<Balance, sqlx::Error> {
    let balance = query_file_as!(
        Balance,
        "./src/db/queries/deposit.sql",
        accountid,
        item,
        quantity
    )
    .fetch_one(conn)
    .await?;

    Ok(balance)
}

pub async fn withdraw(
    conn: &mut PgConnection,
    accountid: Uuid,
    item: &str,
    quantity: i64,
) -> Result<Balance, sqlx::Error> {
    let balance = query_file_as!(
        Balance,
        "./src/db/queries/withdraw.sql",
        accountid,
        item,
        quantity
    )
    .fetch_one(conn)
    .await?;

    Ok(balance)
}
