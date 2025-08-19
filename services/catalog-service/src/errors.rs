use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),
    
    #[error("Search engine error: {0}")]
    Search(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error occurred"
            })),
            AppError::Validation(errors) => HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Validation failed",
                "details": errors
            })),
            AppError::Search(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Search service error"
            })),
            AppError::Storage(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Storage service error"
            })),
            AppError::Auth(msg) => HttpResponse::Unauthorized().json(serde_json::json!({
                "error": msg
            })),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(serde_json::json!({
                "error": msg
            })),
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(serde_json::json!({
                "error": msg
            })),
            AppError::Internal(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })),
        }
    }
}
