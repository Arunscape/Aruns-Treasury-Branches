use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, MatchedPath, Path, State},
    http::{
        self,
        header::{HeaderMap, HeaderValue},
        request::Parts,
        StatusCode,
    },
    response::{ErrorResponse, IntoResponse},
    routing::{delete, get, post},
    Json, RequestPartsExt, Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub async fn handle_auth_callback(//    Query(query): Query<AuthRequest>,
//    State(store): State<MemoryStore>,
) -> Result<impl IntoResponse, ErrorResponse> {
    Ok(())
}
