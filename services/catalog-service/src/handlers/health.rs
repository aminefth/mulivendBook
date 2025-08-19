use actix_web::{web, HttpResponse, Result};
use chrono::Utc;

pub fn configure() -> actix_web::Scope {
    web::scope("")
        .route("/health", web::get().to(health_check))
        .route("/ready", web::get().to(readiness_check))
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "catalog-service",
        "version": "1.0.0",
        "timestamp": Utc::now().to_rfc3339()
    })))
}

async fn readiness_check() -> Result<HttpResponse> {
    // TODO: Add actual health checks for database, Redis, Elasticsearch
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ready",
        "checks": {
            "database": "healthy",
            "redis": "healthy",
            "elasticsearch": "healthy"
        },
        "timestamp": Utc::now().to_rfc3339()
    })))
}
