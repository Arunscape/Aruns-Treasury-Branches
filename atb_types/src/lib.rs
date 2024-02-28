#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use {
    anyhow::Error,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};
#[cfg(feature = "ssr")]
use {
    async_trait::async_trait,
    axum_session::{DatabasePool, Session, SessionConfig, SessionLayer, SessionPgPool},
    axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication, HasPermission},
    sqlx::{
        any::{AnyConnectOptions, AnyPoolOptions},
        postgres::{PgConnectOptions, PgPoolOptions},
        AnyPool, ConnectOptions, PgPool,
    },
    std::net::SocketAddr,
};

pub mod prelude;

#[derive(
    Debug,
    // sqlx::FromRow
    Clone,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Account {
    pub id: Uuid,
    pub userid: Uuid,
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Balance {
    pub accountid: Uuid,
    pub item: String,
    pub quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct McItem {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Transaction {
    pub id: Uuid,
    pub time_processed: chrono::DateTime<chrono::Utc>,
    pub fromid: Uuid,
    pub toid: Uuid,
    pub item: String,
    pub quantity: i64,
    pub price: i64,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum OrderType {
    Market,
    Limit(i64),
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct McServerStatus {
    pub version: String,
    pub max_players: usize,
    pub online_players: usize,
    /// (name, id)
    pub sample: Vec<(String, String)>,
    pub favicon: Vec<u8>,
}

#[cfg(feature = "ssr")]
// This is only used if you want to use Token based Authentication checks
#[async_trait]
impl HasPermission<PgPool> for User {
    async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
        true
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl Authentication<User, Uuid, PgPool> for User {
    // This is ran when the user has logged in and has not yet been Cached in the system.
    // Once ran it will load and cache the user.
    async fn load_user(id: Uuid, _pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
        Ok(User { id })
    }

    // This function is used internally to deturmine if they are logged in or not.
    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        true
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl HasPermission<AnyPool> for User {
    async fn has(&self, perm: &str, _pool: &Option<&AnyPool>) -> bool {
        true
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl Authentication<User, Uuid, AnyPool> for User {
    // This is ran when the user has logged in and has not yet been Cached in the system.
    // Once ran it will load and cache the user.
    async fn load_user(id: Uuid, _pool: Option<&AnyPool>) -> Result<User, anyhow::Error> {
        Ok(User { id })
    }

    // This function is used internally to deturmine if they are logged in or not.
    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        true
    }
}
