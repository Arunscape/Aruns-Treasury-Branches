use chrono;
use chrono::offset::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct LoginRequest {
    uuid: String,
    server_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

lazy_static! {
    static ref AUTH_SERVER_ADDR: String = env::var("AUTH_ADDR").unwrap_or("localhost:8081".into());
    static ref HEADER: Header = Header::new(Algorithm::EdDSA);
    static ref KEY: EncodingKey = {
        let auth_secret = env::var("AUTH_SECRET").unwrap_or("changeme".into());
        EncodingKey::from_secret(auth_secret.as_ref())
    };
}

fn make_jwt(uuid: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let expiration = Utc::now()
        .checked_add_signed(
            // 60s * 10 = 10 minutes
            chrono::Duration::seconds(60 * 10)
            ).ok_or("failed to add 10 minutes to current time. I see we have somehow overflowed 64 bit time. Congrats? And hello future aliens!")?
        .timestamp();

    let claims = Claims {
        sub: uuid.to_owned(),
        exp: expiration as usize,
    };

    encode(&*HEADER, &claims, &*KEY).map_err(|e| e.into())
}



pub async fn auth_server() -> std::io::Result<()> {
    let mut auth_server = tide::new();

    auth_server
        .at("/login")
        .post(|mut req: Request<()>| async move {
            let LoginRequest {
                uuid,
                server_secret,
            } = req.body_json().await?;

            let token = make_jwt(&uuid);

            #[cfg(debug_assertions)]
            {
                // I AM CONFUSION
                fn type_of<T>(_: &T) {
                    dbg!(std::any::type_name::<T>());
                }
                type_of(&token);
            }

            let token = token.map_err(|e| Err(format!("{}", e)))?;

            let login_url = format!("https://atb.arun.gg/login/{token}");

            Ok(json!({ "uuid": uuid }))
        });

    auth_server
        .listen(&*AUTH_SERVER_ADDR)
        .await
        .map_err(|e| e.into())
}
