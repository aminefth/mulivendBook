use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use validator::Validate;
use crate::{AppState, models::*, errors::AppError};

pub fn configure() -> actix_web::Scope {
    web::scope("/reviews")
        .route("/{id}/helpful", web::post().to(mark_helpful))
}

async fn mark_helpful(
    _state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let _review_id = path.into_inner();
    
    // TODO: Implement helpful marking logic
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Review marked as helpful"
    })))
}
