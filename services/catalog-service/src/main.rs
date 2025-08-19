use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use tracing_actix_web::TracingLogger;
use std::env;

mod config;
mod models;
mod handlers;
mod services;
mod database;
mod search;
mod storage;
mod middleware;
mod errors;

use config::Config;
use database::Database;
use search::SearchEngine;
use storage::StorageService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    
    tracing::info!("Starting BookMarket Catalog Service");

    // Initialize database
    let database = Database::new(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // Initialize Redis
    let redis_client = redis::Client::open(config.redis_url.as_str())
        .expect("Failed to create Redis client");
    let redis_manager = redis::aio::ConnectionManager::new(redis_client)
        .await
        .expect("Failed to create Redis connection manager");

    // Initialize search engine
    let search_engine = SearchEngine::new(&config.elasticsearch_url)
        .await
        .expect("Failed to initialize search engine");

    // Initialize storage service
    let storage_service = StorageService::new(&config).await
        .expect("Failed to initialize storage service");

    // Create application state
    let app_state = web::Data::new(AppState {
        database,
        redis: redis_manager,
        search_engine,
        storage_service,
        config: config.clone(),
    });

    let bind_address = format!("0.0.0.0:{}", config.port);
    tracing::info!("Catalog service listening on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(TracingLogger::default())
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .wrap(middleware::auth::Auth)
            .service(
                web::scope("/api/v1")
                    .service(handlers::products::configure())
                    .service(handlers::categories::configure())
                    .service(handlers::search::configure())
                    .service(handlers::reviews::configure())
            )
            .service(handlers::health::configure())
            .service(handlers::metrics::configure())
    })
    .bind(&bind_address)?
    .run()
    .await
}

pub struct AppState {
    pub database: Database,
    pub redis: redis::aio::ConnectionManager,
    pub search_engine: SearchEngine,
    pub storage_service: StorageService,
    pub config: Config,
}
