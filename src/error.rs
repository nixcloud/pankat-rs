use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use diesel::result::Error as DieselError;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed")]
    AuthError,
    #[error("Database error: {0}")]
    DatabaseError(#[from] DieselError),
    #[error("Internal server error")]
    InternalError,
    #[error("Not found")]
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::AuthError => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::NotFound => {
                return (
                    StatusCode::NOT_FOUND,
                    [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                    crate::renderer::html::create_not_found_page(),
                )
                    .into_response();
            }
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
