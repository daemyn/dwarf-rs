mod helpers;

use actix_web::{http::StatusCode, test, web, App};
use dwarf_rs::{
    handlers::{create_dwarf_url, get_dwarf_url_by_slug, redirect_dwarf_url_by_slug},
    models::{AppState, CreateDwarfUrl},
    services::generate_url,
};

use crate::helpers::test_database::TestDatabase;


#[actix_web::test]
async fn test_create_dwarf_url() {
    let test_db = TestDatabase::new().await;
    let app_state = AppState {
        pool: test_db.pool.clone(),
        slug_size: 6,
    };

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .route("/", web::post().to(create_dwarf_url)),
    )
    .await;

    let payload = CreateDwarfUrl {
        target: "https://example.com".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let slug = body
                        .get("slug")
                        .expect("Missing 'slug' field")
                        .as_str()
                        .expect("'slug' is not a string");
    assert_eq!(slug.len(), 6, "Slug length is not 6");
}

#[actix_web::test]
async fn test_get_dwarf_url_by_slug() {

    let test_db = TestDatabase::new().await;
    let app_state = AppState {
        pool: test_db.pool.clone(),
        slug_size: 6,
    };

    let created_url = generate_url(&test_db.pool, "https://example.com", app_state.slug_size)
        .await
        .expect("Failed to generate URL");

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .route("/{slug}/details", web::get().to(get_dwarf_url_by_slug)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/{}/details", created_url.slug))
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["slug"], created_url.slug);
    assert_eq!(body["target"], "https://example.com");
}

#[actix_web::test]
async fn test_redirect_dwarf_url_by_slug() {
    let test_db = TestDatabase::new().await;
    let app_state = AppState {
        pool: test_db.pool.clone(),
        slug_size: 6,
    };

    let created_url = generate_url(&test_db.pool, "https://example.com", app_state.slug_size)
        .await
        .expect("Failed to generate URL");

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .route("/{slug}", web::get().to(redirect_dwarf_url_by_slug)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/{}", created_url.slug))
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::MOVED_PERMANENTLY);

    let location_header = resp.headers().get("Location").unwrap();
    assert_eq!(location_header, "https://example.com");
}
