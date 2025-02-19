use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::derive::Display;
use serde_json::json;

#[derive(Debug, Display)]
pub enum AppError {
    #[display("Internal error")]
    InternalError,

    #[display("Service unavailable")]
    ServiceUnavailable,

    #[display("Bad request")]
    BadClientData(String),

    #[display("Not found")]
    NotFound,

    #[display("Max attempts reached")]
    MaxAttemptsReached,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let message = match self {
            AppError::InternalError => "Internal error occurred",
            AppError::BadClientData(msg) => msg,
            AppError::NotFound => "Resource not found",
            AppError::MaxAttemptsReached => "Max attempts reached",
            AppError::ServiceUnavailable => "Service unavailable",
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(json!({
                "error": self.to_string(),
                "message": message
            }))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadClientData(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::MaxAttemptsReached => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}
