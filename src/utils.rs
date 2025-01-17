use dotenvy::dotenv;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::models::AppEnv;

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
        .expect("APP_PORT must be set in .env or environment")
        .parse()
        .expect("Failed to parse APP_PORT as u32");

    let slug_size: u8 = std::env::var("SLUG_SIZE")
        .expect("SLUG_SIZE must be set in .env or environment")
        .parse()
        .expect("Failed to parse SLUG_SIZE as u8");

    AppEnv {
        app_port,
        db_url,
        slug_size,
    }
}
