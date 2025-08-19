use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    middleware,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, warn};
use uuid::Uuid;

mod auth;
mod config;
mod database;
mod errors;
mod handlers;
mod middleware as auth_middleware;
mod models;
mod services;

use config::Config;
use errors::AppError;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: redis::Client,
    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("auth_service=debug,tower_http=debug")
        .init();

    info!("Starting BookMarket Auth Service");

    // Load configuration
    let config = Arc::new(Config::from_env()?);

    // Initialize database connection
    let db = database::connect(&config.database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&db).await?;

    // Initialize Redis connection
    let redis_client = redis::Client::open(config.redis_url.as_str())?;

    let state = AppState {
        db,
        redis: redis_client,
        config: config.clone(),
    };

    // Build application routes
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind(&config.server_address).await?;
    info!("Auth service listening on {}", config.server_address);

    axum::serve(listener, app).await?;

    Ok(())
}

fn create_app(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        
        // Authentication routes
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/refresh", post(handlers::auth::refresh_token))
        .route("/auth/logout", post(handlers::auth::logout))
        .route("/auth/verify", get(handlers::auth::verify_token))
        
        // MFA routes
        .route("/auth/mfa/setup", post(handlers::mfa::setup_mfa))
        .route("/auth/mfa/verify", post(handlers::mfa::verify_mfa))
        .route("/auth/mfa/disable", post(handlers::mfa::disable_mfa))
        
        // User management
        .route("/users/:id", get(handlers::users::get_user))
        .route("/users/:id", put(handlers::users::update_user))
        .route("/users/:id/password", put(handlers::users::change_password))
        
        // API key management
        .route("/api-keys", get(handlers::api_keys::list_keys))
        .route("/api-keys", post(handlers::api_keys::create_key))
        .route("/api-keys/:id", get(handlers::api_keys::get_key))
        .route("/api-keys/:id", put(handlers::api_keys::update_key))
        .route("/api-keys/:id/revoke", post(handlers::api_keys::revoke_key))
        
        // Admin routes
        .route("/admin/users", get(handlers::admin::list_users))
        .route("/admin/users/:id/suspend", post(handlers::admin::suspend_user))
        .route("/admin/users/:id/activate", post(handlers::admin::activate_user))
        
        // Metrics
        .route("/metrics", get(handlers::metrics::metrics))
        
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware::auth_middleware,
        ))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn health_check() -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "service": "auth-service",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

async fn readiness_check(State(state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {
    // Check database connectivity
    let db_status = match sqlx::query("SELECT 1").fetch_one(&state.db).await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    // Check Redis connectivity
    let mut redis_conn = state.redis.get_async_connection().await.map_err(|_| {
        AppError::InternalServerError("Failed to connect to Redis".to_string())
    })?;
    
    let redis_status = match redis::cmd("PING").query_async::<_, String>(&mut redis_conn).await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    let overall_status = if db_status == "healthy" && redis_status == "healthy" {
        "ready"
    } else {
        "not_ready"
    };

    Ok(Json(serde_json::json!({
        "status": overall_status,
        "checks": {
            "database": db_status,
            "redis": redis_status
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
