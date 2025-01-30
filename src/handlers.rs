use crate::{
    errors::AppError,
    models::{AppState, CreateDwarfUrl},
    services::{generate_url, get_url_by_slug, service_health_check, visit_url},
};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use url::Url;


pub async fn health_check(state: Data<AppState>) -> Result<impl Responder, AppError> {
    service_health_check(&state.pool).await?;
    Ok(HttpResponse::Ok())
}

pub async fn get_dwarf_url_by_slug(
    state: Data<AppState>,
    path: Path<String>,
) -> Result<impl Responder, AppError> {
    let slug = path.into_inner();

    let dwarf_url = get_url_by_slug(&state.pool, &slug).await?;

    Ok(HttpResponse::Ok().json(dwarf_url))
}

pub async fn redirect_dwarf_url_by_slug(
    state: Data<AppState>,
    path: Path<String>,
) -> Result<impl Responder, AppError> {
    let slug = path.into_inner();

    let dwarf_url = visit_url(&state.pool, &slug).await?;

    Ok(HttpResponse::MovedPermanently()
        .append_header(("Location", dwarf_url.target))
        .finish())
}

pub async fn create_dwarf_url(
    state: Data<AppState>,
    payload: Result<Json<CreateDwarfUrl>, actix_web::Error>,
) -> Result<impl Responder, AppError> {
    let body = payload.map_err(|_| AppError::BadClientData("Invalid request body".to_string()))?;

    if Url::parse(&body.target).is_err() {
        return Err(AppError::BadClientData(
            "Invalid URL provided in target field".to_string(),
        ));
    }

    let dwarf_url = generate_url(&state.pool, &body.target, state.slug_size).await?;

    Ok(HttpResponse::Created().json(dwarf_url))
}
