mod transactions;
mod users;

use lazy_static::lazy_static;
use atb_types::prelude::*;
use std::env;
use sqlx::query;

lazy_static!{
    static ref DB_URL: String = env::var("DATABASE_URL").unwrap_or("postgres://postgres@localhost/postgres".into());
}

struct ATBError(String);

use sqlx::postgres::PgPoolOptions;
pub struct ATBDB {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ATBDB {
    async fn new() -> Result<Self,  sqlx::Error>{
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&*DB_URL)
            .await?;
        Ok(Self { pool })
    }

    async fn new_user(&mut self, user: &User) -> Result<(), ATBError> {
        Ok(())
    }

}
