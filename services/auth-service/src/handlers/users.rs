use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::{hash_password, verify_password},
    errors::AppError,
    models::{ChangePasswordRequest, UpdateUserRequest, UserProfile},
    services::user_service,
    AppState,
};

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<UserProfile>, AppError> {
    let user = user_service::get_user_by_id(&state.db, &user_id).await?;
    Ok(Json(user.into()))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserProfile>, AppError> {
    payload.validate()?;

    let id = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let user = sqlx::query_as!(
        crate::models::User,
        r#"
        UPDATE auth.users 
        SET first_name = COALESCE($2, first_name),
            last_name = COALESCE($3, last_name),
            phone = COALESCE($4, phone),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, email, password_hash, first_name, last_name, phone,
                  role as "role: crate::models::UserRole", 
                  status as "status: crate::models::UserStatus",
                  email_verified, phone_verified, last_login, created_at, updated_at
        "#,
        id,
        payload.first_name,
        payload.last_name,
        payload.phone
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(user.into()))
}

pub async fn change_password(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, AppError> {
    payload.validate()?;

    let user = user_service::get_user_by_id(&state.db, &user_id).await?;

    // Verify current password
    if !verify_password(&payload.current_password, &user.password_hash)? {
        return Err(AppError::BadRequest("Current password is incorrect".to_string()));
    }

    // Hash new password
    let new_password_hash = hash_password(&payload.new_password, state.config.bcrypt_cost)?;

    // Update password
    sqlx::query!(
        "UPDATE auth.users SET password_hash = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2",
        new_password_hash,
        user.id
    )
    .execute(&state.db)
    .await?;

    // Invalidate all sessions for this user (force re-login)
    sqlx::query!("DELETE FROM auth.sessions WHERE user_id = $1", user.id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
