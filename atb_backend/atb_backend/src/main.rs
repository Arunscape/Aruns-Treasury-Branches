#![cfg_attr(debug_assertions, allow(unused))]

use dotenvy::dotenv;
use futures::try_join;
use lazy_static::lazy_static;
use tide::http::Cookie;
use tide::utils::Before;
use tide::{prelude::*, utils::After};
use uuid::Uuid;

use http_types::headers::{HeaderValue, HeaderValues};
use std::vec;
use std::{env, error, io::ErrorKind};
use tide::security::{CorsMiddleware, Origin};
use tide::{http::mime, Body, Redirect, Request, Response, Server, StatusCode};
use tide_sqlx::{SQLxMiddleware, SQLxRequestExt};
use tide::log::LevelFilter;
use sqlx::postgres::{Postgres, PgConnectOptions, PgPoolOptions};
use sqlx::ConnectOptions;
use std::str::FromStr;

mod authentication;
mod db;
mod server_for_plugin;

use server_for_plugin::auth_server;

use crate::authentication::{verify_jwt, Claims};

lazy_static! {
    static ref SERVER_ADDR: String = env::var("SERVER_ADDR").unwrap_or("localhost:8080".into());
    static ref AUTH_SERVER_ADDR: String = env::var("AUTH_ADDR").unwrap_or("localhost:8081".into());
    static ref DOMAIN: String = env::var("DOMAIN").unwrap_or("http://localhost:8080".into());
    static ref API_URL: String = env::var("API_URL").unwrap_or("http://localhost:8080".into());
    static ref DB_URL: String =
        env::var("DATABASE_URL").unwrap_or("postgres://postgres@localhost/postgres".into());
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;

    #[cfg(debug_assertions)]
    tide::log::with_level(tide::log::LevelFilter::Debug);
    #[cfg(not(debug_assertions))]
    tide::log::start();
    let mut app = tide::new();
    // app.with(tide::log::LogMiddleware::new());

    app.with(After(|mut res: Response| async move {
        // dbg!(&res);
        if let Some(err) = res.error() {
            res.set_body(err.to_string());
        }
        Ok(res)
    }));

    app.with(Before(|mut req: Request<()>| async move {
        // let token: Cookie = req.cookie("jwt").ok_or(tide::Error::from_str(
        //     StatusCode::Unauthorized,
        //     "No jwt cookie",
        // ))?;

        // dbg!(&req);
        let token = req.header("Authorization");
        dbg!(&token);
        if token.is_none() {
            return req;
        }

        let token = token.unwrap().as_str().replacen("Bearer ", "", 1);
        let claims = verify_jwt(&token);

        dbg!(&claims);
        if claims.is_err() {
            return req;
        }
        let claims = claims.unwrap();
        req.set_ext(claims);
        req
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

    let mut connect_opts = PgConnectOptions::from_str(&DB_URL)?;
    connect_opts.log_statements(LevelFilter::Debug);

    let pg_pool = PgPoolOptions::new()
        .max_connections(50)
        .connect_with(connect_opts)
        .await?;

    app.with(SQLxMiddleware::from(pg_pool));

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

    // #[derive(Serialize, Deserialize)]
    // struct TokenRequest {
    //     token: String
    // }
    // app.at("/login_web").get(|req: Request<()>| async move {
    //     // let cookie = {
    //     //     let cookie = Cookie::build("jwt", token)
    //     //     // .http_only(true)
    //     //     // .secure(true)
    //     //     .same_site(tide::http::cookies::SameSite::None)
    //     //     .domain("")
    //     //     ;
    //     //     if cfg!(debug_assertions) {
    //     //         cookie
    //     //     } else {
    //     //         cookie
    //     //         .domain(&*DOMAIN)
    //     //         .secure(true)
    //     //         .http_only(true)
    //     //     }.finish()
    //     // };

    //     let token: TokenRequest = req.query()?;
    //     // .map_err(|mut e| {
    //     //     e.set_status(StatusCode::BadRequest);
    //     //     e
    //     // })?;
    //     let mut res = Response::new(StatusCode::Ok);
    //     res.set_body(json!(token));
    //     // res.insert_cookie(cookie);
    //     Ok(res)
    // });

    app.at("/accounts").get(|req: Request<()>| async move {
        let uuid = req.ext::<Claims>().ok_or(tide::Error::from_str(
            StatusCode::Unauthorized,
            "Missing token",
        ))?.uuid();

        let mut conn = req.sqlx_conn::<Postgres>().await;
        let res = db::get_accounts_for_user(&mut conn, uuid).await?;

        Ok(json!(res))
    });

    #[derive(Serialize, Deserialize)]
    struct NewAccountRequest {
        nickname: String,
    }
    app.at("/accounts").post(|mut req: Request<()>| async move {

        // dbg!(req.ext::<Claims>());
        let uuid = req.ext::<Claims>().ok_or(tide::Error::from_str(
            StatusCode::Unauthorized,
            "Missing token",
        ))?.uuid();

        let NewAccountRequest { nickname } = req.body_json().await?;

        let mut conn = req.sqlx_conn::<Postgres>().await;
        let res = db::new_account(&mut conn, uuid, nickname).await?;

        Ok(json!(res))
    });

    #[derive(Serialize, Deserialize)]
    struct ModifyAccountRequest {
        old: String,
        new: String,
    }
    app.at("/accounts").patch(|mut req: Request<()>| async move {
        let userid = req.ext::<Claims>().ok_or(tide::Error::from_str(
            StatusCode::Unauthorized,
            "Missing token",
        ))?.uuid();

        let ModifyAccountRequest { old, new } = req.body_json().await?;

        let mut conn = req.sqlx_conn::<Postgres>().await;
        let res = db::change_account_name(&mut conn, userid, &old, &new).await?;

        Ok(json!(res))
    });

    app.at("/accounts/:id").delete(|req: Request<()>| async move {
        let uuid = req.ext::<Claims>().ok_or(tide::Error::from_str(
            StatusCode::Unauthorized,
            "Missing token",
        ))?.uuid();

        let id = req.param("id")?;
        let id = Uuid::parse_str(&id)?;

        let mut conn = req.sqlx_conn::<Postgres>().await;
        let res = db::delete_account(&mut conn, id, uuid).await?;

        Ok(StatusCode::NoContent)
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

    let auth_server = auth_server().await?;

    try_join!(app, auth_server)?;
    Ok(())
}
