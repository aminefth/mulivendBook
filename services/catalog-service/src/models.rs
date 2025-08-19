use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Product {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub title: String,
    pub description: String,
    pub isbn: Option<String>,
    pub author: String,
    pub publisher: String,
    pub publication_date: Option<DateTime<Utc>>,
    pub language: String,
    pub pages: Option<i32>,
    pub category_id: Uuid,
    pub price: rust_decimal::Decimal,
    pub discount_price: Option<rust_decimal::Decimal>,
    pub stock_quantity: i32,
    pub status: ProductStatus,
    pub images: Vec<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "product_status", rename_all = "lowercase")]
pub enum ProductStatus {
    Draft,
    Active,
    Inactive,
    OutOfStock,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1, max = 2000))]
    pub description: String,
    pub isbn: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub author: String,
    #[validate(length(min = 1, max = 255))]
    pub publisher: String,
    pub publication_date: Option<DateTime<Utc>>,
    pub language: String,
    pub pages: Option<i32>,
    pub category_id: Uuid,
    #[validate(range(min = 0.01))]
    pub price: rust_decimal::Decimal,
    pub discount_price: Option<rust_decimal::Decimal>,
    #[validate(range(min = 0))]
    pub stock_quantity: i32,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateProductRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub isbn: Option<String>,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub publication_date: Option<DateTime<Utc>>,
    pub language: Option<String>,
    pub pages: Option<i32>,
    pub category_id: Option<Uuid>,
    pub price: Option<rust_decimal::Decimal>,
    pub discount_price: Option<rust_decimal::Decimal>,
    pub stock_quantity: Option<i32>,
    pub status: Option<ProductStatus>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub image_url: Option<String>,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 100))]
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub image_url: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Review {
    pub id: Uuid,
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
    pub title: Option<String>,
    pub comment: Option<String>,
    pub is_verified: bool,
    pub helpful_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateReviewRequest {
    #[validate(range(min = 1, max = 5))]
    pub rating: i32,
    pub title: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductListResponse {
    pub products: Vec<Product>,
    pub total: i64,
    pub page: i32,
    pub limit: i32,
    pub total_pages: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub category_id: Option<Uuid>,
    pub min_price: Option<rust_decimal::Decimal>,
    pub max_price: Option<rust_decimal::Decimal>,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub language: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub products: Vec<Product>,
    pub total: i64,
    pub page: i32,
    pub limit: i32,
    pub total_pages: i32,
    pub facets: serde_json::Value,
    pub suggestions: Vec<String>,
}
