use std::{str::FromStr, net::SocketAddr};

use axum::{routing::get, Router};
use lazy_static::lazy_static;

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
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let server_address: SocketAddr = SERVER_ADDR.parse()?;
    axum::Server::bind(&server_address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
