use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{User, UserProfile, UserRole, UserStatus},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    page: Option<u32>,
    limit: Option<u32>,
    role: Option<UserRole>,
    status: Option<UserStatus>,
    search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedUsers {
    users: Vec<UserProfile>,
    total: i64,
    page: u32,
    limit: u32,
    total_pages: u32,
}

pub async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<ListUsersQuery>,
) -> Result<Json<PaginatedUsers>, AppError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20).min(100); // Max 100 per page
    let offset = (page - 1) * limit;

    let mut query = "SELECT id, email, password_hash, first_name, last_name, phone, role, status, email_verified, phone_verified, last_login, created_at, updated_at FROM auth.users".to_string();
    let mut conditions = Vec::new();
    let mut bind_params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
    let mut param_count = 0;

    // Add filters
    if let Some(role) = &params.role {
        param_count += 1;
        conditions.push(format!("role = ${}", param_count));
        bind_params.push(Box::new(role.clone()));
    }

    if let Some(status) = &params.status {
        param_count += 1;
        conditions.push(format!("status = ${}", param_count));
        bind_params.push(Box::new(status.clone()));
    }

    if let Some(search) = &params.search {
        param_count += 1;
        conditions.push(format!("(email ILIKE ${} OR first_name ILIKE ${} OR last_name ILIKE ${})", param_count, param_count, param_count));
        let search_pattern = format!("%{}%", search);
        bind_params.push(Box::new(search_pattern));
    }

    if !conditions.is_empty() {
        query.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
    }

    // Get total count
    let count_query = query.replace("SELECT id, email, password_hash, first_name, last_name, phone, role, status, email_verified, phone_verified, last_login, created_at, updated_at", "SELECT COUNT(*)");
    let total: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(&state.db)
        .await?;

    // Add pagination
    query.push_str(&format!(" ORDER BY created_at DESC LIMIT {} OFFSET {}", limit, offset));

    let users = sqlx::query_as!(
        User,
        &query,
        // Note: In a real implementation, you'd need to properly bind the parameters
        // This is simplified for demonstration
    )
    .fetch_all(&state.db)
    .await?;

    let user_profiles: Vec<UserProfile> = users.into_iter().map(|u| u.into()).collect();
    let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;

    Ok(Json(PaginatedUsers {
        users: user_profiles,
        total,
        page,
        limit,
        total_pages,
    }))
}

pub async fn suspend_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let id = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let result = sqlx::query!(
        "UPDATE auth.users SET status = 'suspended', updated_at = CURRENT_TIMESTAMP WHERE id = $1",
        id
    )
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    // Invalidate all sessions for this user
    sqlx::query!("DELETE FROM auth.sessions WHERE user_id = $1", id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn activate_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let id = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let result = sqlx::query!(
        "UPDATE auth.users SET status = 'active', updated_at = CURRENT_TIMESTAMP WHERE id = $1",
        id
    )
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
