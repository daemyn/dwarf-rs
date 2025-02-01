use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};
use actix_web::{middleware::Logger, web, App, HttpServer};
use dwarf_rs::{
    handlers::{create_dwarf_url, get_dwarf_url_by_slug, health_check, redirect_dwarf_url_by_slug},
    models::AppState,
    utils::load_app_env,
};
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = load_app_env();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env.db_url)
        .await
        .expect("Failed to connect to database");

    let app_state = AppState {
        pool,
        slug_size: env.slug_size,
    };

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let backend = InMemoryBackend::builder().build();

    HttpServer::new(move || {
        let input = SimpleInputFunctionBuilder::new(
            Duration::from_secs(env.rate_limit_interval),
            env.rate_limit,
        )
        .real_ip_key()
        .build();
        let rate_limiter = RateLimiter::builder(backend.clone(), input)
            .add_headers()
            .build();

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/health", web::get().to(health_check))
            .route("/{slug}", web::get().to(redirect_dwarf_url_by_slug))
            .route("/api/v0/urls/{slug}", web::get().to(get_dwarf_url_by_slug))
            .route("/api/v0/urls", web::post().to(create_dwarf_url))
            .wrap(Logger::default())
            .wrap(rate_limiter)
    })
    .bind(("0.0.0.0", env.app_port))?
    .run()
    .await
}
