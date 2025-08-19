use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    auth::{hash_password, verify_password, create_jwt_token, decode_jwt_token},
    errors::AppError,
    models::{
        LoginRequest, LoginResponse, RegisterRequest, RefreshTokenRequest, 
        User, UserRole, UserStatus, UserProfile, Claims
    },
    services::user_service,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct VerifyTokenQuery {
    token: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserProfile>, AppError> {
    // Validate input
    payload.validate()?;

    // Check if user already exists
    if user_service::get_user_by_email(&state.db, &payload.email).await.is_ok() {
        return Err(AppError::Conflict("User already exists".to_string()));
    }

    // Hash password
    let password_hash = hash_password(&payload.password, state.config.bcrypt_cost)?;

    // Create user
    let user = user_service::create_user(
        &state.db,
        &payload.email,
        &password_hash,
        &payload.first_name,
        &payload.last_name,
        payload.phone.as_deref(),
        payload.role.unwrap_or(UserRole::Customer),
    ).await?;

    // TODO: Send verification email

    Ok(Json(user.into()))
}

pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // Validate input
    payload.validate()?;

    // Get user by email
    let user = user_service::get_user_by_email(&state.db, &payload.email)
        .await
        .map_err(|_| AppError::Unauthorized)?;

    // Verify password
    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    // Check user status
    if user.status != UserStatus::Active {
        return Err(AppError::Forbidden);
    }

    // Extract user agent and IP
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    
    let ip_address = headers
        .get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Create session
    let session = user_service::create_session(
        &state.db,
        user.id,
        user_agent,
        ip_address,
        payload.remember_me.unwrap_or(false),
        &state.config,
    ).await?;

    // Create JWT tokens
    let access_token = create_jwt_token(
        &user,
        &session,
        state.config.jwt_expiration,
        &state.config.jwt_secret,
    )?;

    let refresh_token = create_jwt_token(
        &user,
        &session,
        state.config.refresh_token_expiration,
        &state.config.jwt_secret,
    )?;

    // Update last login
    user_service::update_last_login(&state.db, user.id).await?;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
        expires_in: state.config.jwt_expiration,
        user: user.into(),
    }))
}

pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // Decode and validate refresh token
    let claims = decode_jwt_token(&payload.refresh_token, &state.config.jwt_secret)?;
    
    // Get user and session
    let user = user_service::get_user_by_id(&state.db, &claims.sub.parse().unwrap())
        .await
        .map_err(|_| AppError::Unauthorized)?;

    let session = user_service::get_session_by_id(&state.db, &claims.jti.parse().unwrap())
        .await
        .map_err(|_| AppError::Unauthorized)?;

    // Check if session is still valid
    if session.expires_at < chrono::Utc::now() {
        return Err(AppError::Unauthorized);
    }

    // Create new tokens
    let access_token = create_jwt_token(
        &user,
        &session,
        state.config.jwt_expiration,
        &state.config.jwt_secret,
    )?;

    let new_refresh_token = create_jwt_token(
        &user,
        &session,
        state.config.refresh_token_expiration,
        &state.config.jwt_secret,
    )?;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token: new_refresh_token,
        expires_in: state.config.jwt_expiration,
        user: user.into(),
    }))
}

pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<StatusCode, AppError> {
    // Extract JWT token from Authorization header
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;

    // Decode token to get session ID
    let claims = decode_jwt_token(token, &state.config.jwt_secret)?;
    
    // Delete session
    user_service::delete_session(&state.db, &claims.jti.parse().unwrap()).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn verify_token(
    State(state): State<AppState>,
    Query(params): Query<VerifyTokenQuery>,
) -> Result<Json<UserProfile>, AppError> {
    // Decode and validate token
    let claims = decode_jwt_token(&params.token, &state.config.jwt_secret)?;
    
    // Get user
    let user = user_service::get_user_by_id(&state.db, &claims.sub.parse().unwrap())
        .await
        .map_err(|_| AppError::Unauthorized)?;

    // Verify session exists and is valid
    let session = user_service::get_session_by_id(&state.db, &claims.jti.parse().unwrap())
        .await
        .map_err(|_| AppError::Unauthorized)?;

    if session.expires_at < chrono::Utc::now() {
        return Err(AppError::Unauthorized);
    }

    Ok(Json(user.into()))
}
