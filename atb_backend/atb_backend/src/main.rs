#![allow(dead_code, unused)]
use std::{net::SocketAddr, str::FromStr};
use dotenvy_macro::dotenv;
use reqwest::Client;
use atb_types::Account;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State, MatchedPath, Path},
    http::{
        self,
        header::{HeaderMap, HeaderValue},
        request::Parts,
        StatusCode,
    },
    response::{ErrorResponse, IntoResponse},
    routing::{get, delete, post},
    Json, RequestPartsExt, Router,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sqlx::{
    migrate::Migrator,
    postgres::{PgConnectOptions, PgPool, PgPoolOptions},
    ConnectOptions, Connection, PgConnection, Pool,
};

use tracing_subscriber::{
    filter::{filter_fn, LevelFilter},
    prelude::*,
};

use uuid::Uuid;

mod authentication;
mod db;

use crate::authentication::{verify_jwt, Claims, make_jwt};

// use load_dotenv::load_dotenv;

// load_dotenv!();

lazy_static! {
    static ref SERVER_ADDR: String = std::env::var("SERVER_ADDR").unwrap_or("[::]:8080".into());
    static ref AUTH_SERVER_ADDR: String = std::env::var("AUTH_ADDR").unwrap_or("[::]:8081".into());
    static ref DOMAIN: String = std::env::var("DOMAIN").unwrap_or("http://[::]:8080".into());
    static ref API_URL: String = std::env::var("API_URL").unwrap_or("http://[::]:8080".into());
    static ref DB_URL: String =
        std::env::var("DATABASE_URL").unwrap_or("postgres://postgres@localhost/postgres".into());
    static ref MCAUTH_CLIENT_ID: String = std::env::var("MCAUTH_CLIENT_ID").unwrap_or("3140686772632028258".into());
    static ref MCAUTH_CLIENT_SECRET: String = std::env::var("MCAUTH_CLIENT_SECRET").expect("MCAUTH_CLIENT_SECRET not provided");
}

#[derive(Debug, Clone, FromRef)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let x = dotenvy::dotenv().unwrap ();
    // dbg!(&x.display());
    let fmt_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        // .with(tracing_subscriber::fmt::layer())
        .with(fmt_layer.with_filter(LevelFilter::DEBUG))
        .init();

    let mut connect_opts = PgConnectOptions::from_str(&DB_URL)?;
    connect_opts.log_statements(tracing::log::LevelFilter::Debug);

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect_with(connect_opts)
        .await?;

    let app_state = AppState { pool };

    sqlx::migrate!("./src/db/queries/migrations")
        .run(&app_state.pool)
        .await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/oauth", get(get_oauth))
        // .route("/token/:id", get(get_token))
        .route("/refresh", get(refresh_jwt))
        .route("/accounts", get(get_accounts).patch(update_account_name))
        // .route("/accounts/:name", post(new_account))
        .route("/accounts/:id", delete(delete_account).post(new_account))
        .with_state(app_state);

    let server_address = SERVER_ADDR.parse()?;
    let server = axum::Server::bind(&server_address).serve(app.into_make_service());

    tracing::info!("Starting server on {server_address}");
    server.await?;

    Ok(())
}


async fn get_oauth() -> Result<impl IntoResponse, ErrorResponse> {
    // todo use a client and add it as part of appstate so you can reuse the connection
    let client = Client::new();
    let resp= client.get("https://mc-auth.com/oAuth2/authorize")
        .query(&[
            ("response_type", "code"),
            ("scope", "profile"),
            ("client_id", &*MCAUTH_CLIENT_ID),
            ("redirect_uri", &format!("{}/oauth_callback", &*DOMAIN)),
        ])
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .json()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
    Ok((StatusCode::OK, Json(resp)))
}
async fn refresh_jwt() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        "maybe I'll implement this later",
    )
}

//async fn get_accounts(State(pool): State<PgPool>) -> Result<(StatusCode, Json<Vec<Account>>), ErrorResponse> {
async fn get_accounts(State(pool): State<PgPool>) -> Result<impl IntoResponse, ErrorResponse> {
    // todo get uuid from token
    let uuid = Uuid::from_str("30652840-fcd4-48aa-b52d-306f85c0f93e").unwrap();

    let accounts = db::get_accounts_for_user(&pool, uuid)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok((StatusCode::OK, Json(accounts)))
}

async fn new_account(State(pool): State<PgPool>, Path(name): Path<String>) -> Result<impl IntoResponse, ErrorResponse> {
    // todo get uuid from token
    let uuid = Uuid::from_str("30652840-fcd4-48aa-b52d-306f85c0f93e").unwrap();

    let account = db::new_account(&pool, uuid, name)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok((StatusCode::OK, Json(account)))
}

#[derive(Deserialize, Serialize)]
struct UpdateAccountName {
    old: String,
    new: String,
}
async fn update_account_name(State(pool): State<PgPool>, Json(reqbody): Json<UpdateAccountName>) -> Result<impl IntoResponse, ErrorResponse> {
    // todo get uuid from token
    let uuid = Uuid::from_str("30652840-fcd4-48aa-b52d-306f85c0f93e").unwrap();

    let account = db::update_account_name(&pool, uuid, "old", "new")
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok((StatusCode::OK, Json(account)))

}

async fn delete_account(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, ErrorResponse> {
    // todo get uuid from token
    let uuid = Uuid::from_str("30652840-fcd4-48aa-b52d-306f85c0f93e").unwrap();

    let account = db::delete_account(&pool, id, uuid)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    match account {
        Some(account) => Ok((StatusCode::OK, Json(account))),
        None => Err((StatusCode::NOT_FOUND, "Account not found").into()),
    }

}