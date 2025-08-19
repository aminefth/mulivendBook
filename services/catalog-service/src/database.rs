use sqlx::{PgPool, Row};
use anyhow::Result;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::*;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;
        
        // Run migrations if needed
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Database { pool })
    }

    pub async fn create_product(&self, vendor_id: Uuid, product: &CreateProductRequest) -> Result<Product> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let row = sqlx::query!(
            r#"
            INSERT INTO marketplace.products (
                id, vendor_id, title, description, isbn, author, publisher, 
                publication_date, language, pages, category_id, price, 
                discount_price, stock_quantity, status, images, metadata, 
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
            RETURNING *
            "#,
            id,
            vendor_id,
            product.title,
            product.description,
            product.isbn,
            product.author,
            product.publisher,
            product.publication_date,
            product.language,
            product.pages,
            product.category_id,
            product.price,
            product.discount_price,
            product.stock_quantity,
            ProductStatus::Draft as ProductStatus,
            &Vec::<String>::new(),
            product.metadata.clone().unwrap_or(serde_json::json!({})),
            now,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Product {
            id: row.id,
            vendor_id: row.vendor_id,
            title: row.title,
            description: row.description,
            isbn: row.isbn,
            author: row.author,
            publisher: row.publisher,
            publication_date: row.publication_date,
            language: row.language,
            pages: row.pages,
            category_id: row.category_id,
            price: row.price,
            discount_price: row.discount_price,
            stock_quantity: row.stock_quantity,
            status: ProductStatus::Draft,
            images: vec![],
            metadata: row.metadata,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    pub async fn get_product(&self, id: Uuid) -> Result<Option<Product>> {
        let row = sqlx::query_as!(
            Product,
            r#"
            SELECT id, vendor_id, title, description, isbn, author, publisher,
                   publication_date, language, pages, category_id, price,
                   discount_price, stock_quantity, 
                   status as "status: ProductStatus",
                   images, metadata, created_at, updated_at
            FROM marketplace.products 
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn update_product(&self, id: Uuid, vendor_id: Uuid, update: &UpdateProductRequest) -> Result<Option<Product>> {
        let now = Utc::now();
        
        let row = sqlx::query_as!(
            Product,
            r#"
            UPDATE marketplace.products 
            SET title = COALESCE($3, title),
                description = COALESCE($4, description),
                isbn = COALESCE($5, isbn),
                author = COALESCE($6, author),
                publisher = COALESCE($7, publisher),
                publication_date = COALESCE($8, publication_date),
                language = COALESCE($9, language),
                pages = COALESCE($10, pages),
                category_id = COALESCE($11, category_id),
                price = COALESCE($12, price),
                discount_price = COALESCE($13, discount_price),
                stock_quantity = COALESCE($14, stock_quantity),
                status = COALESCE($15, status),
                metadata = COALESCE($16, metadata),
                updated_at = $17
            WHERE id = $1 AND vendor_id = $2
            RETURNING id, vendor_id, title, description, isbn, author, publisher,
                      publication_date, language, pages, category_id, price,
                      discount_price, stock_quantity, 
                      status as "status: ProductStatus",
                      images, metadata, created_at, updated_at
            "#,
            id,
            vendor_id,
            update.title.as_ref(),
            update.description.as_ref(),
            update.isbn.as_ref(),
            update.author.as_ref(),
            update.publisher.as_ref(),
            update.publication_date,
            update.language.as_ref(),
            update.pages,
            update.category_id,
            update.price,
            update.discount_price,
            update.stock_quantity,
            update.status.as_ref() as Option<&ProductStatus>,
            update.metadata.as_ref(),
            now
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn list_products(&self, vendor_id: Option<Uuid>, category_id: Option<Uuid>, page: i32, limit: i32) -> Result<(Vec<Product>, i64)> {
        let offset = (page - 1) * limit;
        
        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT id, vendor_id, title, description, isbn, author, publisher,
                   publication_date, language, pages, category_id, price,
                   discount_price, stock_quantity, 
                   status as "status: ProductStatus",
                   images, metadata, created_at, updated_at
            FROM marketplace.products 
            WHERE ($1::uuid IS NULL OR vendor_id = $1)
              AND ($2::uuid IS NULL OR category_id = $2)
              AND status = 'active'
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#,
            vendor_id,
            category_id,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        let total = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM marketplace.products 
            WHERE ($1::uuid IS NULL OR vendor_id = $1)
              AND ($2::uuid IS NULL OR category_id = $2)
              AND status = 'active'
            "#,
            vendor_id,
            category_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((products, total.count.unwrap_or(0)))
    }

    pub async fn delete_product(&self, id: Uuid, vendor_id: Uuid) -> Result<bool> {
        let result = sqlx::query!(
            "DELETE FROM marketplace.products WHERE id = $1 AND vendor_id = $2",
            id,
            vendor_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // Category methods
    pub async fn create_category(&self, category: &CreateCategoryRequest) -> Result<Category> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let row = sqlx::query_as!(
            Category,
            r#"
            INSERT INTO marketplace.categories (id, name, slug, description, parent_id, image_url, is_active, sort_order, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, true, $7, $8, $9)
            RETURNING *
            "#,
            id,
            category.name,
            category.slug,
            category.description,
            category.parent_id,
            category.image_url,
            category.sort_order.unwrap_or(0),
            now,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        let categories = sqlx::query_as!(
            Category,
            "SELECT * FROM marketplace.categories WHERE is_active = true ORDER BY sort_order, name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }

    // Review methods
    pub async fn create_review(&self, product_id: Uuid, user_id: Uuid, review: &CreateReviewRequest) -> Result<Review> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let row = sqlx::query_as!(
            Review,
            r#"
            INSERT INTO marketplace.reviews (id, product_id, user_id, rating, title, comment, is_verified, helpful_count, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, false, 0, $7, $8)
            RETURNING *
            "#,
            id,
            product_id,
            user_id,
            review.rating,
            review.title,
            review.comment,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn get_product_reviews(&self, product_id: Uuid, page: i32, limit: i32) -> Result<(Vec<Review>, i64)> {
        let offset = (page - 1) * limit;
        
        let reviews = sqlx::query_as!(
            Review,
            "SELECT * FROM marketplace.reviews WHERE product_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            product_id,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        let total = sqlx::query!(
            "SELECT COUNT(*) as count FROM marketplace.reviews WHERE product_id = $1",
            product_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((reviews, total.count.unwrap_or(0)))
    }
}
