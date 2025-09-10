use axum::{
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson, Response},
};
use serde_json::json;
use tracing::error;

/// Custom error type that automatically handles HTTP responses and logging
#[derive(Debug)]
pub enum AppError {
    /// Internal server errors - logged with full context, return generic error to user
    Internal(anyhow::Error),
    /// User errors with custom error codes - returned directly to user
    BadRequest { message: String, code: String },
    /// Not found errors - returned directly to user  
    NotFound(String),
}

impl AppError {
    /// Create a bad request error with code
    pub fn bad_request(message: impl Into<String>, code: impl Into<String>) -> Self {
        Self::BadRequest {
            message: message.into(),
            code: code.into(),
        }
    }

    /// Create a not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Internal(err) => {
                // Log the full error context for internal errors
                error!("Internal server error: {:?}", err);
                
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseJson(json!({
                        "error": "Internal server error"
                    }))
                ).into_response()
            }
            AppError::BadRequest { message, code } => {
                (
                    StatusCode::BAD_REQUEST,
                    ResponseJson(json!({
                        "error": message,
                        "code": code
                    }))
                ).into_response()
            }
            AppError::NotFound(message) => {
                (
                    StatusCode::NOT_FOUND,
                    ResponseJson(json!({
                        "error": message
                    }))
                ).into_response()
            }
        }
    }
}

/// Automatically convert anyhow::Error to AppError::Internal
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err)
    }
}

/// Automatically convert sqlx::Error to AppError::Internal via anyhow
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Internal(err.into())
    }
}

/// Convenience type alias for Results that can be converted to HTTP responses
pub type AppResult<T> = Result<T, AppError>;