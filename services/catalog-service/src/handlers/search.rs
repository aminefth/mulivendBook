use actix_web::{web, HttpResponse, Result};
use validator::Validate;
use crate::{AppState, models::*, errors::AppError};

pub fn configure() -> actix_web::Scope {
    web::scope("/search")
        .route("", web::post().to(search_products))
        .route("/suggestions", web::get().to(get_suggestions))
}

async fn search_products(
    state: web::Data<AppState>,
    req: web::Json<SearchRequest>,
) -> Result<HttpResponse, AppError> {
    let response = state.search_engine.search(&req).await?;
    
    Ok(HttpResponse::Ok().json(response))
}

async fn get_suggestions(
    _state: web::Data<AppState>,
    query: web::Query<SuggestionQuery>,
) -> Result<HttpResponse, AppError> {
    // TODO: Implement search suggestions
    let suggestions = vec![
        format!("{} programming", query.q),
        format!("{} tutorial", query.q),
        format!("{} guide", query.q),
    ];
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "suggestions": suggestions
    })))
}

#[derive(serde::Deserialize)]
struct SuggestionQuery {
    q: String,
}
