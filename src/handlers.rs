use crate::{
    models::{AppState, CreateDwarfUrl},
    services::{generate_url, visit_url},
};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use log::error;
use serde_json::{json, Value};

pub fn json_error(error: &str, message: &str) -> Value {
    json!({
        "error": error,
        "message": message
    })
}

pub async fn get_dwarf_url_by_slug(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let slug = path.into_inner();
    match visit_url(&state.pool, &slug).await {
        Ok(dwarf_url) => HttpResponse::Ok().json(dwarf_url),
        Err(e) => {
            error!("Unable to retrieve url: {:?}", e);
            return HttpResponse::NotFound()
                .json(json_error("Url not found", "Unable to retrieve url"));
        }
    }
}

pub async fn create_dwarf_url(
    state: Data<AppState>,
    payload: Result<Json<CreateDwarfUrl>, actix_web::Error>,
) -> impl Responder {
    let body = match payload {
        Ok(body) => body,
        Err(e) => {
            error!("Payload extraction error: {:?}", e);
            return HttpResponse::BadRequest().json(json_error(
                "Invalid body",
                "The provided data is incorrect or malformed.",
            ));
        }
    };

    let dwarf_url = match generate_url(&state.pool, &body.target, state.slug_size).await {
        Ok(dwarf_url) => dwarf_url,
        Err(e) => {
            error!("Internal error occurred: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(json_error("Internal error", "Something went wrong"));
        }
    };

    HttpResponse::Created().json(dwarf_url)
}
