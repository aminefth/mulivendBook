use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::{generate_api_key, hash_api_key},
    errors::AppError,
    models::{ApiKey, ApiKeyInfo, CreateApiKeyRequest, CreateApiKeyResponse},
    AppState,
};

pub async fn list_keys(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<Vec<ApiKeyInfo>>, AppError> {
    let id = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let keys = sqlx::query_as!(
        ApiKey,
        "SELECT id, user_id, key_hash, name, scopes, last_used, expires_at, created_at 
         FROM auth.api_keys WHERE user_id = $1 ORDER BY created_at DESC",
        id
    )
    .fetch_all(&state.db)
    .await?;

    let key_infos: Vec<ApiKeyInfo> = keys
        .into_iter()
        .map(|key| ApiKeyInfo {
            id: key.id,
            name: key.name,
            scopes: key.scopes,
            last_used: key.last_used,
            expires_at: key.expires_at,
            created_at: key.created_at,
        })
        .collect();

    Ok(Json(key_infos))
}

pub async fn create_key(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<Json<CreateApiKeyResponse>, AppError> {
    payload.validate()?;

    let id = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    // Generate API key
    let api_key = generate_api_key();
    let key_hash = hash_api_key(&api_key)?;

    // Insert into database
    let key_record = sqlx::query_as!(
        ApiKey,
        r#"
        INSERT INTO auth.api_keys (user_id, key_hash, name, scopes, expires_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, user_id, key_hash, name, scopes, last_used, expires_at, created_at
        "#,
        id,
        key_hash,
        payload.name,
        &payload.scopes,
        payload.expires_at
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(CreateApiKeyResponse {
        id: key_record.id,
        name: key_record.name,
        key: api_key, // Only returned once
        scopes: key_record.scopes,
        expires_at: key_record.expires_at,
        created_at: key_record.created_at,
    }))
}

pub async fn get_key(
    State(state): State<AppState>,
    Path((user_id, key_id)): Path<(String, String)>,
) -> Result<Json<ApiKeyInfo>, AppError> {
    let uid = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;
    let kid = Uuid::parse_str(&key_id)
        .map_err(|_| AppError::BadRequest("Invalid key ID".to_string()))?;

    let key = sqlx::query_as!(
        ApiKey,
        "SELECT id, user_id, key_hash, name, scopes, last_used, expires_at, created_at 
         FROM auth.api_keys WHERE id = $1 AND user_id = $2",
        kid,
        uid
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ApiKeyInfo {
        id: key.id,
        name: key.name,
        scopes: key.scopes,
        last_used: key.last_used,
        expires_at: key.expires_at,
        created_at: key.created_at,
    }))
}

pub async fn update_key(
    State(state): State<AppState>,
    Path((user_id, key_id)): Path<(String, String)>,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<Json<ApiKeyInfo>, AppError> {
    payload.validate()?;

    let uid = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;
    let kid = Uuid::parse_str(&key_id)
        .map_err(|_| AppError::BadRequest("Invalid key ID".to_string()))?;

    let key = sqlx::query_as!(
        ApiKey,
        r#"
        UPDATE auth.api_keys 
        SET name = $3, scopes = $4, expires_at = $5
        WHERE id = $1 AND user_id = $2
        RETURNING id, user_id, key_hash, name, scopes, last_used, expires_at, created_at
        "#,
        kid,
        uid,
        payload.name,
        &payload.scopes,
        payload.expires_at
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ApiKeyInfo {
        id: key.id,
        name: key.name,
        scopes: key.scopes,
        last_used: key.last_used,
        expires_at: key.expires_at,
        created_at: key.created_at,
    }))
}

pub async fn revoke_key(
    State(state): State<AppState>,
    Path((user_id, key_id)): Path<(String, String)>,
) -> Result<StatusCode, AppError> {
    let uid = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;
    let kid = Uuid::parse_str(&key_id)
        .map_err(|_| AppError::BadRequest("Invalid key ID".to_string()))?;

    let result = sqlx::query!(
        "DELETE FROM auth.api_keys WHERE id = $1 AND user_id = $2",
        kid,
        uid
    )
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
