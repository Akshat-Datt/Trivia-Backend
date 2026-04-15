use axum::{
    Json, http::StatusCode, response::{IntoResponse, Response}
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError{
    ValidationError(String),
    NotFound(String),
    DatabaseError,
    Conflict(String)
}

#[derive(Serialize)]
struct ErrorResponse{
    error: String
}



impl IntoResponse for AppError{
    fn into_response(self) -> Response {
        match self {
            AppError::ValidationError(msg) => {
                let body = ErrorResponse {error : msg};
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            },

            AppError::NotFound(msg) => {
                let body = ErrorResponse {error : msg};
                (StatusCode::NOT_FOUND, Json(body)).into_response()
            },
            AppError::DatabaseError => {
                let body = ErrorResponse {error : "Something went wrong".to_string()};
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            },
            AppError::Conflict(msg) => {
                let body = ErrorResponse {error : msg};
                (StatusCode::CONFLICT, Json(body)).into_response()
            }
        }
    }
}