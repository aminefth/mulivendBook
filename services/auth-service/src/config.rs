use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server_address: String,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub refresh_token_expiration: i64,
    pub bcrypt_cost: u32,
    pub rate_limit_requests: u64,
    pub rate_limit_window: u64,
    pub cors_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();

        let mut cfg = config::Config::builder();

        // Set defaults
        cfg = cfg
            .set_default("server_address", "0.0.0.0:8001")?
            .set_default("jwt_expiration", 3600)? // 1 hour
            .set_default("refresh_token_expiration", 2592000)? // 30 days
            .set_default("bcrypt_cost", 12)?
            .set_default("rate_limit_requests", 100)?
            .set_default("rate_limit_window", 60)? // 1 minute
            .set_default(
                "cors_origins",
                vec!["http://localhost:3000", "http://localhost:3001"],
            )?;

        // Override with environment variables
        cfg = cfg.add_source(config::Environment::with_prefix("AUTH"));

        // Required environment variables
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| config::ConfigError::Message("DATABASE_URL must be set".to_string()))?;

        let redis_url =
            env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());

        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| config::ConfigError::Message("JWT_SECRET must be set".to_string()))?;

        cfg = cfg
            .set_override("database_url", database_url)?
            .set_override("redis_url", redis_url)?
            .set_override("jwt_secret", jwt_secret)?;

        cfg.build()?.try_deserialize()
    }
}
