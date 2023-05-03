use crate::authentication::Claims;
use crate::authentication::{make_jwt, verify_jwt};
use crate::db;
use chrono;
use chrono::offset::Utc;
use chrono::Duration;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, Postgres};
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tide::prelude::*;
use tide::Request;
use tide_sqlx::{SQLxMiddleware, SQLxRequestExt};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct LoginRequest {
    uuid: Uuid,
}

#[derive(Debug, Deserialize)]
struct DepositRequest {
    uuid: Uuid,
    item: String,
    quantity: i64,
}

#[derive(Debug, Deserialize)]
struct WithDrawRequest {
    uuid: Uuid,
    item: String,
    quantity: i64,
}

#[derive(Debug, Deserialize)]
struct BalanceRequest {
    accountid: Uuid,
}

lazy_static! {
    static ref AUTH_SERVER_ADDR: String = env::var("AUTH_ADDR").unwrap_or("localhost:8081".into());
    static ref DB_URL: String =
        env::var("DATABASE_URL").unwrap_or("postgres://postgres@localhost/postgres".into());
}

pub async fn auth_server(
) -> Result<impl futures::Future<Output = Result<(), std::io::Error>>, Box<dyn std::error::Error>> {
    let mut secret_server = tide::new();

    // let db: &'static = ATBDB::new().await.map_err(|e| {
    //     std::io::Error::new(std::io::ErrorKind::Other, "Error connecting to database")
    // })?;

    // should only be accessible from the internal docker network
    // ie not available to the public

    let mut connect_opts = PgConnectOptions::from_str(&DB_URL)?;
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await?;

    secret_server.with(SQLxMiddleware::from(pg_pool));

    secret_server
        .at("/login")
        .get(|req: Request<()>| async move {
            let LoginRequest { uuid } = req.query()?;

            let token = make_jwt(uuid)?;

            let mut conn = req.sqlx_conn::<Postgres>().await;
            db::add_user(&mut conn, uuid).await?;

            Ok(token)
        });

    secret_server
        .at("/deposit")
        .post(|mut req: Request<()>| async move {
            let DepositRequest {
                uuid,
                item,
                quantity,
            } = req.body_json().await?;

            let mut conn = req.sqlx_conn::<Postgres>().await;
            db::transfer(uuid, "", item, quantity)
            Ok(tide::StatusCode::Ok)
        });

    secret_server
        .at("/withdraw")
        .post(|mut req: Request<()>| async move {
            let WithDrawRequest {
                uuid,
                item,
                quantity,
            } = req.body_json().await?;

            Ok(tide::StatusCode::Ok)
        });

    secret_server.at("/balance").get(|req: Request<()>| async {
        let mut req = req;
        let BalanceRequest { accountid } = req.body_json().await?;

        let mut conn = req.sqlx_conn::<Postgres>().await;
        let balance = db::get_balances_for_account(&mut conn, accountid).await?;

        let balance = json!(balance);

        Ok(balance)
    });

    Ok(secret_server.listen(&*AUTH_SERVER_ADDR))
}
