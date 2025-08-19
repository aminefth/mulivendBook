use actix_web::{web, HttpResponse, Result};
use validator::Validate;
use crate::{AppState, models::*, errors::AppError};

pub fn configure() -> actix_web::Scope {
    web::scope("/categories")
        .route("", web::post().to(create_category))
        .route("", web::get().to(list_categories))
}

async fn create_category(
    state: web::Data<AppState>,
    req: web::Json<CreateCategoryRequest>,
) -> Result<HttpResponse, AppError> {
    req.validate()?;
    
    let category = state.database.create_category(&req).await?;
    
    Ok(HttpResponse::Created().json(category))
}

async fn list_categories(
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let categories = state.database.get_categories().await?;
    
    Ok(HttpResponse::Ok().json(categories))
}
