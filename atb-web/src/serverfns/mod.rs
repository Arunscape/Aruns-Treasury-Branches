pub mod login;
pub mod server_status;
pub mod transactions;

#[cfg(feature = "ssr")]
use sqlx::{any::AnyExecutor, query, query_as, query_file_as, AnyPool, PgPool};
use {atb_types::*, leptos::*, leptos_meta::*, uuid::Uuid};
pub use {login::*, server_status::*, transactions::*};

#[cfg(feature = "ssr")]
pub fn pool() -> Result<PgPool, ServerFnError> {
    use_context::<PgPool>().ok_or_else(|| ServerFnError::ServerError("No pool found".to_string()))
}
