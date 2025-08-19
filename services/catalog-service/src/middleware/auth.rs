use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    email: String,
    role: String,
    exp: usize,
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    
    // Skip auth for health endpoints
    if req.path().starts_with("/health") || req.path().starts_with("/metrics") {
        return Ok(req);
    }
    
    match validate_token(token) {
        Ok(claims) => {
            if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                req.extensions_mut().insert(user_id);
                Ok(req)
            } else {
                let config = req.app_data::<Config>().cloned().unwrap_or_default();
                Err((AuthenticationError::from(config).into(), req))
            }
        }
        Err(_) => {
            let config = req.app_data::<Config>().cloned().unwrap_or_default();
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
    let key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    
    let token_data = decode::<Claims>(token, &key, &validation)?;
    Ok(token_data.claims)
}

pub fn Auth() -> HttpAuthentication<BearerAuth, fn(ServiceRequest, BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)>> {
    HttpAuthentication::bearer(validator)
}
