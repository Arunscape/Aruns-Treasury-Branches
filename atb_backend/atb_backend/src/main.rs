#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use dotenv::dotenv;
use futures::try_join;
use lazy_static::lazy_static;
use tide::prelude::*;

use std::{env, error};
use tide::{http::mime, Body, Redirect, Request, Response, Server, StatusCode};

mod auth_server;
mod authentication;
use auth_server::auth_server;

lazy_static! {
    static ref SERVER_ADDR: String = env::var("SERVER_ADDR").unwrap_or("localhost:8080".into());
    static ref AUTH_SERVER_ADDR: String = env::var("AUTH_ADDR").unwrap_or("localhost:8081".into());
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    tide::log::start();

    let mut app = tide::new();
    app.at("/sse")
        .get(tide::sse::endpoint(|_req, sender| async move {
            loop {
                async_std::task::sleep(std::time::Duration::from_secs(1)).await;
                sender.send("fruit", "banana", None).await?;
                async_std::task::sleep(std::time::Duration::from_secs(5)).await;
                sender.send("fruit", "apple", None).await?;
            }
            //Ok(())
        }));

    app.at("/refresh")
        .get(|req: Request<()>| async move {
            let token = req.header("Authorization").ok_or(tide::Error::from_str(
                StatusCode::Unauthorized,
                "No Authorization header",
            ))?;
            dbg!(&token);
            let token = token.as_str().replacen("Bearer ", "", 1);
            dbg!(&token);
            let new_token = authentication::refresh_jwt(&token)?;
            Ok(new_token)
        });

    let app = {
        #[cfg(debug_assertions)]
        {
            app.listen(&*SERVER_ADDR)
        }

        #[cfg(not(debug_assertions))]
        {
            use tide_acme::rustls_acme::caches::DirCache;
            use tide_acme::{AcmeConfig, TideRustlsExt};
            app.listen(
                tide_rustls::TlsListener::build().addrs(&*SERVER_ADDR).acme(
                    AcmeConfig::new(vec!["atb.arun.gg"])
                        .contact_push("mailto:atb-acme@arun.gg")
                        .cache(DirCache::new("./acme")),
                ),
            )
        }
    };

    try_join!(app, auth_server())?;
    Ok(())
}
