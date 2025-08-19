use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{auth::decode_jwt_token, errors::AppError, AppState};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Skip auth for health checks and public endpoints
    let path = request.uri().path();
    if path == "/health" 
        || path == "/ready" 
        || path == "/metrics"
        || path.starts_with("/auth/register")
        || path.starts_with("/auth/login")
        || path.starts_with("/auth/verify") {
        return Ok(next.run(request).await);
    }

    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let token = match auth_header {
        Some(token) => token,
        None => return Err(AppError::Unauthorized),
    };

    // Decode and validate JWT token
    let claims = decode_jwt_token(token, &state.config.jwt_secret)?;

    // Verify session is still valid
    let session = crate::services::user_service::get_session_by_id(&state.db, &claims.jti)
        .await
        .map_err(|_| AppError::Unauthorized)?;

    if session.expires_at < chrono::Utc::now() {
        return Err(AppError::Unauthorized);
    }

    // Add user info to request extensions for use in handlers
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
