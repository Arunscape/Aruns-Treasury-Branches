use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use tide::{http::Request, prelude::*};

lazy_static! {
    static ref ENCODING_KEY: EncodingKey = {
    // don't worry about the private key lol it's not mine
    let encoding_key = env::var("AUTH_SECRET").unwrap_or("-----BEGIN PRIVATE KEY-----
    MC4CAQAwBQYDK2VwBCIEIGrD/e7uKYqSY4twDEsRfMMuLSrODf14dpTiTK6K1YI0
    -----END PRIVATE KEY-----".into());
    EncodingKey::from_ed_pem(&encoding_key.as_bytes()).unwrap()
    };

    static ref DECODING_KEY: DecodingKey = {
        // don't worry about the private key lol it's not mine
        let decoding_key = env::var("AUTH_SECRET").unwrap_or("-----BEGIN PUBLIC KEY-----
        MCowBQYDK2VwAyEA2+Jj2UvNCvQiUPNYRgSi0cJSPiJI6Rs6D0UTeEpQVj8=
        -----END PUBLIC KEY-----".into());
        DecodingKey::from_ed_pem(&decoding_key.as_bytes()).unwrap()
    };
    static ref HEADER: Header = Header::new(Algorithm::EdDSA);
    static ref VALIDATION: Validation = Validation::new(Algorithm::EdDSA);
}

pub fn make_jwt(uuid: String) -> Result<String, tide::Error> {
    let expiration = (Utc::now() + Duration::days(1)).timestamp();

    let claims = Claims {
        sub: uuid,
        exp: expiration as usize,
    };

    _make_jwt(&claims)
}

fn _make_jwt(claims: &Claims) -> Result<std::string::String, tide::Error> {
    encode(&*HEADER, &claims, &*ENCODING_KEY)
        .map_err(|e| tide::Error::from_str(tide::StatusCode::InternalServerError, e.to_string()))
}

pub fn refresh_jwt(token: &str) -> Result<String, tide::Error> {
    let expiration = (Utc::now() + Duration::minutes(15)).timestamp();

    let uuid = verify_jwt(token)?;

    let claims = Claims {
        sub: uuid,
        exp: expiration as usize,
    };

    _make_jwt(&claims)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn verify_jwt(token: &str) -> Result<String, tide::Error> {
    let claims = decode::<Claims>(token, &*DECODING_KEY, &*VALIDATION)
        .map_err(|e| tide::Error::from_str(tide::StatusCode::Unauthorized, e.to_string()))?;
    Ok(claims.claims.sub)
}
