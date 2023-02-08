#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use dotenvy::dotenv;
use futures::try_join;
use lazy_static::lazy_static;
use tide::http::Cookie;
use tide::{prelude::*, utils::After};
use uuid::Uuid;

use http_types::headers::{HeaderValue, HeaderValues};
use std::vec;
use std::{env, error, io::ErrorKind};
use tide::security::{CorsMiddleware, Origin};
use tide::{http::mime, Body, Redirect, Request, Response, Server, StatusCode};

mod authentication;
mod db;
mod server_for_plugin;

use server_for_plugin::auth_server;

use crate::authentication::verify_jwt;

lazy_static! {
    static ref SERVER_ADDR: String = env::var("SERVER_ADDR").unwrap_or("localhost:8080".into());
    static ref AUTH_SERVER_ADDR: String = env::var("AUTH_ADDR").unwrap_or("localhost:8081".into());
    static ref DOMAIN: String = env::var("DOMAIN").unwrap_or("http://localhost:8080".into());
    static ref API_URL: String = env::var("API_URL").unwrap_or("http://localhost:8080".into());
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;

    tide::log::start();
    let mut app = tide::new();
    // app.with(tide::log::LogMiddleware::new());

    app.with(After(|mut res: Response| async move {
        dbg!(&res);
        if let Some(err) = res.error() {
            res.set_body(err.to_string());
        }
        Ok(res)
    }));

    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from(vec![
            "http://localhost:3000",
            "https://atb.arun.gg",
        ]))
        .allow_credentials(true)
        .allow_headers(HeaderValues::from(vec![
            "Authorization".try_into().unwrap(),
            "Content-Type".try_into().unwrap(),
        ]));

    app.with(cors);

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

    app.at("/refresh").get(|req: Request<()>| async move {
        let _token = req.header("Authorization").ok_or(tide::Error::from_str(
            StatusCode::Unauthorized,
            "No Authorization header",
        ))?;

        // let token = token.as_str().replacen("Bearer ", "", 1);
        // let new_token = authentication::refresh_jwt(&token)?;
        // Ok(new_token)

        let mut res = Response::new(tide::StatusCode::NotImplemented);
        res.set_body("maybe I'll implement this later");
        Ok(res)
    });

    app.at("/login_web").get(|req: Request<()>| async move {
        let token = req.header("Authorization").ok_or(tide::Error::from_str(
            StatusCode::Unauthorized,
            "No Authorization header",
        ))?;
        let token = token.as_str().replacen("Bearer ", "", 1);

        // let cookie = {

        //     let cookie = Cookie::build("jwt", token)
        //     // .http_only(true)
        //     // .secure(true)
        //     .same_site(tide::http::cookies::SameSite::None)
        //     .domain("")
        //     ;

        //     if cfg!(debug_assertions) {
        //         cookie
        //     } else {
        //         cookie
        //         .domain(&*DOMAIN)
        //         .secure(true)
        //         .http_only(true)
        //     }.finish()
        // };

        let mut res = Response::new(StatusCode::Ok);
        res.set_body(json!({
            "token": token.as_str()
        }));
        // res.insert_cookie(cookie);
        Ok(res)
    });

    app.at("/hello").get(|_| async { Ok("Hello, world!") });

    app.at("/accounts").get(|req: Request<()>| async move {
        let token: Cookie = req.cookie("jwt").unwrap();
        let token = token.value();

        let uuid = verify_jwt(token)?;
        let uuid: Uuid = uuid.parse()?;

        let mut db = db::ATBDB::new().await?;
        let res = db.get_accounts_for_user(uuid).await.unwrap();

        Ok(json!(res))
    });

    app.at("/users").get(|_| async {
        let db = db::ATBDB::new().await.unwrap();
        Ok("users")
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
                    AcmeConfig::new(vec![&*DOMAIN])
                        .contact_push("mailto:atb-acme@arun.gg")
                        .cache(DirCache::new("./acme")),
                ),
            )
        }
    };

    try_join!(app, auth_server())?;
    Ok(())
}
