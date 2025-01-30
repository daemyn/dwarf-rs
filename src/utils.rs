use dotenvy::dotenv;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::models::AppEnv;

const APP_PORT: u16 = 3000;
const SLUG_SIZE: u8 = 6;
const RATE_LIMIT: u64 = 10;
const RATE_LIMIT_INTERVAL: u64 = 60;

pub fn generate_slug(length: u8) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}

pub fn load_app_env() -> AppEnv {
    dotenv().ok();

    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or environment");

    let app_port: u16 = std::env::var("APP_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(APP_PORT);

    let slug_size: u8 = std::env::var("SLUG_SIZE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(SLUG_SIZE);

    let rate_limit: u64 = std::env::var("RATE_LIMIT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(RATE_LIMIT);

    let rate_limit_interval: u64 = std::env::var("RATE_LIMIT_INTERVAL")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(RATE_LIMIT_INTERVAL);

    AppEnv {
        app_port,
        db_url,
        slug_size,
        rate_limit,
        rate_limit_interval
    }
}
