use actix_web::{web, HttpResponse, Result};
use chrono::Utc;

pub fn configure() -> actix_web::Scope {
    web::scope("")
        .route("/metrics", web::get().to(metrics))
}

async fn metrics() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "service": "catalog-service",
        "metrics": {
            "uptime": "0s",
            "timestamp": Utc::now().to_rfc3339()
        }
    })))
}
