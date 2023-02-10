use crate::authentication::Claims;
use crate::authentication::{make_jwt, verify_jwt};
use crate::db::{ATBDB, self};
use chrono;
use chrono::offset::Utc;
use chrono::Duration;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tide::prelude::*;
use tide::Request;
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
    static ref DB: ATBDB = {
        let x = async_std::task::block_on(ATBDB::new());

        x.unwrap()
    };
}

pub async fn auth_server() -> std::io::Result<()> {
    let mut secret_server = tide::new();

    // let db: &'static = ATBDB::new().await.map_err(|e| {
    //     std::io::Error::new(std::io::ErrorKind::Other, "Error connecting to database")
    // })?;

    // should only be accessible from the internal docker network
    // ie not available to the public
    secret_server
        .at("/login")
        .get(|req: Request<()>| async move {
            let LoginRequest { uuid } = req.query()?;

            let db = db::ATBDB::new();
            let token = make_jwt(uuid)?;

            let mut db = db.await?;
            db.add_user(uuid).await?;

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

        let balance = DB.clone().get_balances_for_account(accountid).await?;

        let balance = json!(balance);

        Ok(balance)
    });

    secret_server
        .listen(&*AUTH_SERVER_ADDR)
        .await
        .map_err(|e| e.into())
}
