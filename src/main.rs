use actix_web::{middleware::Logger, web, App, HttpServer};
use dwarf_rs::{
    handlers::{create_dwarf_url, get_dwarf_url_by_slug},
    models::AppState,
    utils::load_app_env,
};
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_env = load_app_env();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&app_env.db_url)
        .await
        .expect("Failed to connect to database");

    let app_state = AppState {
        pool,
        slug_size: app_env.slug_size,
    };

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/{slug}", web::get().to(get_dwarf_url_by_slug))
            .route("/", web::post().to(create_dwarf_url))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", app_env.app_port))?
    .run()
    .await
}
