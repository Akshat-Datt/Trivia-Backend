use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};

#[derive(Debug)]
pub enum AppError{
    ValidationError(String),
    NotFound(String),
    Deleted,
    DatabaseError
}

impl IntoResponse for AppError{
    fn into_response(self) -> Response {
        match self {
            AppError::ValidationError(msg) => {
                (StatusCode::BAD_REQUEST, Json(msg)).into_response()
            },

            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, Json(msg)).into_response()
            },

            AppError::Deleted => {
                (StatusCode::NO_CONTENT, Json("Resource deleted".to_string())).into_response()
            },

            AppError::DatabaseError => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json("Something went wrong".to_string())).into_response()
            }
        }
    }
}