#![allow(dead_code, unused)]
use std::{net::SocketAddr, str::FromStr};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{
        self,
        header::{HeaderMap, HeaderValue},
        request::Parts,
        StatusCode,
    },
    response::IntoResponse,
    routing::get,
    RequestPartsExt, Router,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sqlx::{ConnectOptions, postgres::{PgConnectOptions, PgPool, PgPoolOptions}, migrate::{Migrator}};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
use tracing::log::LevelFilter;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut connect_opts = PgConnectOptions::from_str(&DB_URL)?;
    connect_opts.log_statements(LevelFilter::Debug);

    let pg_pool = PgPoolOptions::new()
        .max_connections(50)
        .connect_with(connect_opts)
        .await?;

    sqlx::migrate!("./src/db/queries/migrations").run(&pg_pool).await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/refresh", get(refresh_jwt))
        .with_state(pg_pool);

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

async fn get_accounts(State(pg_pool): State<PgPool>) -> impl IntoResponse {
    // todo get uuid from token
    let uuid = Uuid::from_str("30652840-fcd4-48aa-b52d-306f85c0f93e");
    todo!()
}
