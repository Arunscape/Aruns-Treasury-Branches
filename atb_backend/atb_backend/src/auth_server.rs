use crate::authentication::Claims;
use crate::authentication::{make_jwt, verify_jwt};
use chrono;
use chrono::offset::Utc;
use chrono::Duration;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct LoginRequest {
    uuid: String,
}

lazy_static! {
    static ref AUTH_SERVER_ADDR: String = env::var("AUTH_ADDR").unwrap_or("localhost:8081".into());
}

pub async fn auth_server() -> std::io::Result<()> {
    let mut auth_server = tide::new();

    // should only be accessible from the internal docker network
    // ie not available to the public
    auth_server
        .at("/login")
        .post(|mut req: Request<()>| async move {
            let LoginRequest { uuid } = req.body_json().await?;

            let token = make_jwt(uuid)?;
            Ok(token)
        });

    auth_server
        .listen(&*AUTH_SERVER_ADDR)
        .await
        .map_err(|e| e.into())
}
