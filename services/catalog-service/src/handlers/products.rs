use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use validator::Validate;
use crate::{AppState, models::*, errors::AppError};

pub fn configure() -> actix_web::Scope {
    web::scope("/products")
        .route("", web::post().to(create_product))
        .route("", web::get().to(list_products))
        .route("/{id}", web::get().to(get_product))
        .route("/{id}", web::put().to(update_product))
        .route("/{id}", web::delete().to(delete_product))
        .route("/{id}/images", web::post().to(upload_images))
        .route("/{id}/reviews", web::get().to(get_product_reviews))
        .route("/{id}/reviews", web::post().to(create_review))
}

async fn create_product(
    state: web::Data<AppState>,
    req: web::Json<CreateProductRequest>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    req.validate()?;
    
    let vendor_id = *user_id.into_inner();
    let product = state.database.create_product(vendor_id, &req).await?;
    
    // Index in search engine
    state.search_engine.index_product(&product).await?;
    
    Ok(HttpResponse::Created().json(product))
}

async fn get_product(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let product_id = path.into_inner();
    
    match state.database.get_product(product_id).await? {
        Some(product) => Ok(HttpResponse::Ok().json(product)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Product not found"
        }))),
    }
}

async fn list_products(
    state: web::Data<AppState>,
    query: web::Query<ProductListQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20).min(100);
    
    let (products, total) = state.database
        .list_products(query.vendor_id, query.category_id, page, limit)
        .await?;
    
    let total_pages = (total as f64 / limit as f64).ceil() as i32;
    
    let response = ProductListResponse {
        products,
        total,
        page,
        limit,
        total_pages,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

async fn update_product(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateProductRequest>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    req.validate()?;
    
    let product_id = path.into_inner();
    let vendor_id = *user_id.into_inner();
    
    match state.database.update_product(product_id, vendor_id, &req).await? {
        Some(product) => {
            // Update search index
            state.search_engine.index_product(&product).await?;
            Ok(HttpResponse::Ok().json(product))
        },
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Product not found or access denied"
        }))),
    }
}

async fn delete_product(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    let product_id = path.into_inner();
    let vendor_id = *user_id.into_inner();
    
    let deleted = state.database.delete_product(product_id, vendor_id).await?;
    
    if deleted {
        // Remove from search index
        state.search_engine.delete_product(product_id).await?;
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Product not found or access denied"
        })))
    }
}

async fn upload_images(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    payload: actix_web::web::Payload,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    let product_id = path.into_inner();
    let vendor_id = *user_id.into_inner();
    
    // Verify product ownership
    match state.database.get_product(product_id).await? {
        Some(product) if product.vendor_id == vendor_id => {
            // Handle file upload
            let image_urls = state.storage_service.upload_product_images(product_id, payload).await?;
            
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "Images uploaded successfully",
                "urls": image_urls
            })))
        },
        Some(_) => Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied"
        }))),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Product not found"
        }))),
    }
}

async fn get_product_reviews(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, AppError> {
    let product_id = path.into_inner();
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20).min(50);
    
    let (reviews, total) = state.database.get_product_reviews(product_id, page, limit).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "reviews": reviews,
        "total": total,
        "page": page,
        "limit": limit,
        "total_pages": (total as f64 / limit as f64).ceil() as i32
    })))
}

async fn create_review(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: web::Json<CreateReviewRequest>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    req.validate()?;
    
    let product_id = path.into_inner();
    let user_id = *user_id.into_inner();
    
    let review = state.database.create_review(product_id, user_id, &req).await?;
    
    Ok(HttpResponse::Created().json(review))
}

#[derive(serde::Deserialize)]
struct ProductListQuery {
    vendor_id: Option<Uuid>,
    category_id: Option<Uuid>,
    page: Option<i32>,
    limit: Option<i32>,
}

#[derive(serde::Deserialize)]
struct PaginationQuery {
    page: Option<i32>,
    limit: Option<i32>,
}
