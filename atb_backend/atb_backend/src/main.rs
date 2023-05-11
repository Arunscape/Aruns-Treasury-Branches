#![allow(dead_code)]
use std::{net::SocketAddr, str::FromStr};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
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
use uuid::Uuid;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let server_address = SERVER_ADDR.parse()?;
    let server = axum::Server::bind(&server_address).serve(app.into_make_service());

    tracing::info!("Starting server on {server_address}");
    server.await?;

    Ok(())
}
