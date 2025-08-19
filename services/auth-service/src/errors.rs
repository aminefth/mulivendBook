use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Authentication failed")]
    Unauthorized,

    #[error("Access forbidden")]
    Forbidden,

    #[error("Resource not found")]
    NotFound,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Password hashing error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, error_code) = match self {
            AppError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    "DATABASE_ERROR",
                )
            }
            AppError::Redis(ref e) => {
                tracing::error!("Redis error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    "REDIS_ERROR",
                )
            }
            AppError::Validation(ref message) => {
                (StatusCode::BAD_REQUEST, message.clone(), "VALIDATION_ERROR")
            }
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Authentication required".to_string(),
                "UNAUTHORIZED",
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "Access forbidden".to_string(),
                "FORBIDDEN",
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "Resource not found".to_string(),
                "NOT_FOUND",
            ),
            AppError::Conflict(ref message) => (StatusCode::CONFLICT, message.clone(), "CONFLICT"),
            AppError::RateLimitExceeded => (
                StatusCode::TOO_MANY_REQUESTS,
                "Rate limit exceeded".to_string(),
                "RATE_LIMIT_EXCEEDED",
            ),
            AppError::Jwt(ref e) => {
                tracing::warn!("JWT error: {:?}", e);
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid token".to_string(),
                    "INVALID_TOKEN",
                )
            }
            AppError::BcryptError(ref e) => {
                tracing::error!("Bcrypt error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    "BCRYPT_ERROR",
                )
            }
            AppError::InternalServerError(ref message) => {
                tracing::error!("Internal server error: {}", message);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    "INTERNAL_SERVER_ERROR",
                )
            }
            AppError::BadRequest(ref message) => {
                (StatusCode::BAD_REQUEST, message.clone(), "BAD_REQUEST")
            }
        };

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": error_message,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        }));

        (status, body).into_response()
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let field_errors: Vec<String> = errors
                    .iter()
                    .map(|error| {
                        format!(
                            "{}: {}",
                            field,
                            error.message.as_ref().unwrap_or(&"Invalid value".into())
                        )
                    })
                    .collect();
                field_errors.join(", ")
            })
            .collect();

        AppError::Validation(error_messages.join("; "))
    }
}
