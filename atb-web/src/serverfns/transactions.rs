#[cfg(feature = "ssr")]
use sqlx::{any::AnyExecutor, query, query_as, query_file_as, AnyPool, PgPool};
use {atb_types::*, leptos::*, leptos_meta::*, uuid::Uuid};

#[cfg(feature = "ssr")]
pub fn pool() -> Result<PgPool, ServerFnError> {
    use_context::<PgPool>().ok_or_else(|| ServerFnError::ServerError("No pool found".to_string()))
}

#[server(GetTransactions, "/api", "GetJson", "transactions")]
pub async fn get_transactions() -> Result<Vec<Transaction>, ServerFnError> {
    let p = pool()?;

    let res = query_as!(Transaction, "SELECT * FROM transactions ORDER BY id DESC")
        .fetch_all(&p)
        .await?;

    Ok(res)
}

#[server(GetTransactionsByTicker, "/api", "GetJson", "transactions_by_item")]
pub async fn get_transactions_by_item(item: String) -> Result<Vec<Transaction>, ServerFnError> {
    let p = pool()?;

    log::debug!("get transactions by item {}", item);
    let res = query_as!(
        Transaction,
        "SELECT * FROM transactions WHERE item = $1 ORDER BY id DESC",
        item
    )
    .fetch_all(&p)
    .await?;

    Ok(res)
}

#[cfg(debug_assertions)] // not ready for prod yet, need to verify jwt or authenticate somehow
#[server(NewTransaction, "/api", "Url", "new_transaction")]
pub async fn new_transaction(
    from: Uuid,
    to: Uuid,
    item: String,
    quantity: i64,
    price: i64,
) -> Result<Transaction, ServerFnError> {
    let p = pool()?;

    let transaction = query_as!(Transaction, "INSERT INTO transactions (fromid, toid, item, quantity, price) VALUES ($1, $2, $3, $4, $5) RETURNING *", from, to, item, quantity, price).fetch_one(&p).await?;

    Ok(transaction)
}

#[cfg(feature = "ssr")]
/// Takes in a minecraft UUID
pub async fn new_user(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
    let user = query_file_as!(User, "queries/new_user.sql", id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

#[cfg(feature = "ssr")]
pub async fn new_account(
    pool: &PgPool,
    userid: Uuid,
    nickname: String,
) -> Result<Account, sqlx::Error> {
    let account = query_file_as!(Account, "queries/new_account.sql", userid, nickname,)
        .fetch_one(pool)
        .await?;

    Ok(account)
}

#[cfg(feature = "ssr")]
pub async fn update_account_name(
    pool: &PgPool,
    userid: Uuid,
    old_nickname: &str,
    new_nickname: &str,
) -> Result<Account, sqlx::Error> {
    let account = query_file_as!(
        Account,
        "queries/update_account_name.sql",
        new_nickname,
        old_nickname,
        userid,
    )
    .fetch_one(pool)
    .await?;

    Ok(account)
}
#[cfg(feature = "ssr")]

pub async fn delete_account(
    pool: &PgPool,
    accountid: Uuid,
    userid: Uuid,
) -> Result<Option<Account>, sqlx::Error> {
    let account = query_file_as!(Account, "queries/delete_account.sql", accountid, userid)
        .fetch_optional(pool)
        .await?;

    Ok(account)
}

#[cfg(feature = "ssr")]
pub async fn get_accounts_for_user(pool: &PgPool, id: Uuid) -> Result<Vec<Account>, sqlx::Error> {
    let accounts = query_file_as!(Account, "queries/get_accounts_for_user.sql", id)
        .fetch_all(pool)
        .await?;

    Ok(accounts)
}

#[cfg(feature = "ssr")]
pub async fn get_transactions_for_account(
    pool: &PgPool,
    id: Uuid,
) -> Result<Vec<Transaction>, sqlx::Error> {
    let transactions = query_file_as!(Transaction, "queries/get_transactions_for_account.sql", id)
        .fetch_all(pool)
        .await?;

    Ok(transactions)
}

#[cfg(feature = "ssr")]
pub async fn get_balances_for_account(
    pool: &PgPool,
    id: Uuid,
) -> Result<Vec<Balance>, sqlx::Error> {
    let balances = query_file_as!(Balance, "queries/get_balances_for_account.sql", id)
        .fetch_all(pool)
        .await?;

    Ok(balances)
}

//#[cfg(feature = "ssr")]
//pub async fn transfer(
//    pool: &PgPool,
//    fromid: Uuid,
//    toid: Uuid,
//    quantity: i64,
//    item: &str,
//    price: i64,
//) -> Result<Transaction, sqlx::Error> {
//    // BEGIN;
//    // UPDATE balances SET amount = (amount - $3) WHERE accountid = $1 AND item = $4;
//    // UPDATE balances SET amount = (amount + $3) WHERE accountid = $2 AND item = $4;
//
//    // INSERT INTO transactions (fromid, toid, item, quantity, price) VALUES ($1, $2, $3, $4, $5) RETURNING *;
//    // COMMIT;
//    // ",
//    // fromid,
//    // toid,
//    // quantity,
//    // item,
//    // price
//
//    let mut transaction = pool.begin().await?;
//    query!(
//        "
//        UPDATE balances SET quantity = (quantity - $1)
//        FROM accounts
//        WHERE accounts.userid = $2
//        AND accountid = $2 AND item = $3
//        ",
//        quantity,
//        fromid,
//        item
//    )
//    .execute(&mut transaction)
//    .await?;
//
//    query!(
//        "UPDATE balances SET quantity = (quantity + $1) WHERE accountid = $2 AND item = $3",
//        quantity,
//        toid,
//        item
//    )
//    .execute(&mut transaction)
//    .await?;
//
//    let t = query_as!(Transaction, "INSERT INTO transactions (fromid, toid, item, quantity, price) VALUES ($1, $2, $3, $4, $5) RETURNING *", fromid, toid, item, quantity, price)
//        .fetch_one(&mut transaction)
//        .await?;
//
//    transaction.commit().await?;
//
//    Ok(t)
//}

#[cfg(feature = "ssr")]
pub async fn deposit(
    pool: &PgPool,
    accountid: Uuid,
    item: &str,
    quantity: i64,
) -> Result<Balance, sqlx::Error> {
    let balance = query_file_as!(Balance, "queries/deposit.sql", accountid, item, quantity)
        .fetch_one(pool)
        .await?;

    Ok(balance)
}

#[cfg(feature = "ssr")]
pub async fn withdraw(
    pool: &PgPool,
    accountid: Uuid,
    item: &str,
    quantity: i64,
) -> Result<Balance, sqlx::Error> {
    let balance = query_file_as!(Balance, "queries/withdraw.sql", accountid, item, quantity)
        .fetch_one(pool)
        .await?;

    Ok(balance)
}
