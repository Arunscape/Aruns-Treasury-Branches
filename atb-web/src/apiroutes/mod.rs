use {
    axum::{
        http::{header::SET_COOKIE, HeaderMap, StatusCode},
        response::IntoResponse,
    },
    sqlx::Pool,
};
//use axum_session_auth::{SessionSqlitePool, Authentication, HasPermission};
//use bcrypt::{hash, verify, DEFAULT_COST};

pub async fn get_session_cookie(_headers: HeaderMap) -> impl IntoResponse {
    let headers = [(SET_COOKIE, "atb_session=test")];

    (StatusCode::OK, headers)
}
