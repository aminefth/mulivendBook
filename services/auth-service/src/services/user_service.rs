use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::Config,
    errors::AppError,
    models::{Session, User, UserRole, UserStatus},
};

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    first_name: &str,
    last_name: &str,
    phone: Option<&str>,
    role: UserRole,
) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO auth.users (email, password_hash, first_name, last_name, phone, role)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, email, password_hash, first_name, last_name, phone, 
                  role as "role: UserRole", status as "status: UserStatus", 
                  email_verified, phone_verified, last_login, created_at, updated_at
        "#,
        email,
        password_hash,
        first_name,
        last_name,
        phone,
        role as UserRole
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password_hash, first_name, last_name, phone,
               role as "role: UserRole", status as "status: UserStatus",
               email_verified, phone_verified, last_login, created_at, updated_at
        FROM auth.users
        WHERE email = $1
        "#,
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: &str) -> Result<User, AppError> {
    let id = Uuid::parse_str(user_id).map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;
    
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password_hash, first_name, last_name, phone,
               role as "role: UserRole", status as "status: UserStatus",
               email_verified, phone_verified, last_login, created_at, updated_at
        FROM auth.users
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn update_last_login(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE auth.users SET last_login = CURRENT_TIMESTAMP WHERE id = $1",
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_session(
    pool: &PgPool,
    user_id: Uuid,
    user_agent: Option<String>,
    ip_address: Option<String>,
    remember_me: bool,
    config: &Config,
) -> Result<Session, AppError> {
    let expires_at = if remember_me {
        Utc::now() + Duration::seconds(config.refresh_token_expiration)
    } else {
        Utc::now() + Duration::seconds(config.jwt_expiration * 2) // 2x access token lifetime
    };

    // Generate a random token hash for the session
    let token_hash = uuid::Uuid::new_v4().to_string();

    let session = sqlx::query_as!(
        Session,
        r#"
        INSERT INTO auth.sessions (user_id, token_hash, expires_at, user_agent, ip_address)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, user_id, token_hash, expires_at, user_agent, ip_address, created_at
        "#,
        user_id,
        token_hash,
        expires_at,
        user_agent,
        ip_address
    )
    .fetch_one(pool)
    .await?;

    Ok(session)
}

pub async fn get_session_by_id(pool: &PgPool, session_id: &str) -> Result<Session, AppError> {
    let id = Uuid::parse_str(session_id).map_err(|_| AppError::BadRequest("Invalid session ID".to_string()))?;
    
    let session = sqlx::query_as!(
        Session,
        r#"
        SELECT id, user_id, token_hash, expires_at, user_agent, ip_address, created_at
        FROM auth.sessions
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(session)
}

pub async fn delete_session(pool: &PgPool, session_id: &str) -> Result<(), AppError> {
    let id = Uuid::parse_str(session_id).map_err(|_| AppError::BadRequest("Invalid session ID".to_string()))?;
    
    sqlx::query!("DELETE FROM auth.sessions WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn cleanup_expired_sessions(pool: &PgPool) -> Result<u64, AppError> {
    let result = sqlx::query!(
        "DELETE FROM auth.sessions WHERE expires_at < CURRENT_TIMESTAMP"
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
