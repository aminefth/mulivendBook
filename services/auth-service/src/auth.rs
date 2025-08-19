use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{Claims, Session, User},
};

pub fn hash_password(password: &str, cost: u32) -> Result<String, AppError> {
    hash(password, cost).map_err(AppError::from)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(AppError::from)
}

pub fn create_jwt_token(
    user: &User,
    session: &Session,
    expiration_seconds: i64,
    secret: &str,
) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::seconds(expiration_seconds);

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        role: user.role.clone(),
        iat: now.timestamp(),
        exp: exp.timestamp(),
        jti: session.id.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(AppError::from)
}

pub fn decode_jwt_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn generate_api_key() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const KEY_LEN: usize = 32;
    let mut rng = rand::thread_rng();

    let key: String = (0..KEY_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    format!("bm_{}", key)
}

pub fn hash_api_key(key: &str) -> Result<String, AppError> {
    hash(key, DEFAULT_COST).map_err(AppError::from)
}
