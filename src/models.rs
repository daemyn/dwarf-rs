use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

pub struct AppEnv {
    pub db_url: String,
    pub app_port: u16,
    pub slug_size: u8,
    pub rate_limit: u64,
    pub rate_limit_interval: u64,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub slug_size: u8,
}

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct DwarfUrl {
    pub id: i32,
    pub slug: String,
    pub target: String,
    pub visit_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDwarfUrl {
    pub target: String,
}
