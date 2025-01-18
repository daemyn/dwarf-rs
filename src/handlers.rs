use crate::{
    errors::AppError,
    models::{AppState, CreateDwarfUrl},
    services::{generate_url, visit_url},
};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

pub async fn get_dwarf_url_by_slug(
    state: Data<AppState>,
    path: Path<String>,
) -> Result<impl Responder, AppError> {
    let slug = path.into_inner();

    let dwarf_url = visit_url(&state.pool, &slug)
        .await
        .map_err(|_| AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(dwarf_url))
}

pub async fn create_dwarf_url(
    state: Data<AppState>,
    payload: Result<Json<CreateDwarfUrl>, actix_web::Error>,
) -> Result<impl Responder, AppError> {
    let body = payload.map_err(|_| AppError::BadClientData("Invalid request body".to_string()))?;

    let dwarf_url = generate_url(&state.pool, &body.target, state.slug_size)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(HttpResponse::Created().json(dwarf_url))
}
