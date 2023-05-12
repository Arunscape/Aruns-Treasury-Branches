#![allow(dead_code, unused)]
use std::{net::SocketAddr, str::FromStr};

use atb_types::Account;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{
        self,
        header::{HeaderMap, HeaderValue},
        request::Parts,
        StatusCode,
    },
    response::{ErrorResponse, IntoResponse},
    routing::get,
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

use crate::authentication::{verify_jwt, Claims};

lazy_static! {
    static ref SERVER_ADDR: String = std::env::var("SERVER_ADDR").unwrap_or("[::]:8080".into());
    static ref AUTH_SERVER_ADDR: String = std::env::var("AUTH_ADDR").unwrap_or("[::]:8081".into());
    static ref DOMAIN: String = std::env::var("DOMAIN").unwrap_or("http://[::]:8080".into());
    static ref API_URL: String = std::env::var("API_URL").unwrap_or("http://[::]:8080".into());
    static ref DB_URL: String =
        std::env::var("DATABASE_URL").unwrap_or("postgres://postgres@localhost/postgres".into());
}

#[derive(Debug, Clone, FromRef)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .route("/refresh", get(refresh_jwt))
        .route("/accounts", get(get_accounts))
        .with_state(app_state);

    let server_address = SERVER_ADDR.parse()?;
    let server = axum::Server::bind(&server_address).serve(app.into_make_service());

    tracing::info!("Starting server on {server_address}");
    server.await?;

    Ok(())
}

async fn refresh_jwt() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        "maybe I'll implement this later",
    )
}

#[axum::debug_handler]
async fn get_accounts(State(pool): State<PgPool>) -> Result<(StatusCode, Json<Vec<Account>>), ErrorResponse> {
    // todo get uuid from token
    //let uuid = Uuid::from_str("30652840-fcd4-48aa-b52d-306f85c0f93e").unwrap();
    let uuid = Uuid::from_str("f5253be5-8a09-41d3-b0d9-f788b5f55783").unwrap();

    let accounts = db::get_accounts_for_user(&pool, uuid)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok((StatusCode::OK, Json(accounts)))
}
