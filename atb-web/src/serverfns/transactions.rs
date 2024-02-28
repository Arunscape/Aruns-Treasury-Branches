#[cfg(feature = "ssr")]
use sqlx::{any::AnyExecutor, query_as, AnyPool, PgPool};
use {atb_types::*, leptos::*, leptos_meta::*};

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

#[server(GetTransactionsByTicker, "/api", "GetJson", "transactions/:item")]
pub async fn get_transactions_by_item(item: String) -> Result<Vec<Transaction>, ServerFnError> {
    let p = pool()?;

    let res = query_as!(
        Transaction,
        "SELECT * FROM transactions WHERE item = $1 ORDER BY id",
        item
    )
    .fetch_all(&p)
    .await?;

    Ok(res)
}

//#[server(NewTransaction, "/api", "Url", "new_transaction")]
//pub async fn new_transaction(tx: Transaction) -> Result<Transaction, ServerFnError> {
//    let p = pool()?;
//
//}
