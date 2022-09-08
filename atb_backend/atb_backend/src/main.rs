use dotenv::dotenv;
use lazy_static::lazy_static;
use tide::prelude::*;

use std::env;
use tide::{http::mime, Body, Redirect, Request, Response, Server, StatusCode};

lazy_static! {
    static ref PORT: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    static ref ADDRESS: String = env::var("ADDRESS").unwrap_or("0.0.0.0".into());

    static ref CLIENT_ID: &'static str = "b65a4f90-a35f-4345-a755-fa4c05c7a8d9";
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tide::log::start();
    let mut app = tide::new();
    app.listen(format!("{}:{}", *ADDRESS, *PORT)).await?;
    Ok(())
}
