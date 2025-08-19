use axum::{extract::State, response::Response, http::header};
use prometheus::{Encoder, TextEncoder, Counter, Histogram, Gauge, register_counter, register_histogram, register_gauge};
use std::sync::Arc;
use lazy_static::lazy_static;

use crate::{errors::AppError, AppState};

lazy_static! {
    static ref AUTH_REQUESTS_TOTAL: Counter = register_counter!(
        "auth_requests_total",
        "Total number of authentication requests"
    ).unwrap();
    
    static ref AUTH_REQUEST_DURATION: Histogram = register_histogram!(
        "auth_request_duration_seconds",
        "Authentication request duration in seconds"
    ).unwrap();
    
    static ref ACTIVE_SESSIONS: Gauge = register_gauge!(
        "auth_active_sessions",
        "Number of active user sessions"
    ).unwrap();
    
    static ref FAILED_LOGINS_TOTAL: Counter = register_counter!(
        "auth_failed_logins_total",
        "Total number of failed login attempts"
    ).unwrap();
}

pub async fn metrics(State(state): State<AppState>) -> Result<Response, AppError> {
    // Update active sessions gauge
    let active_sessions_count: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM auth.sessions WHERE expires_at > CURRENT_TIMESTAMP"
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);
    
    ACTIVE_SESSIONS.set(active_sessions_count as f64);

    // Encode metrics
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, encoder.format_type())
        .body(buffer.into())
        .unwrap())
}

// Helper functions to update metrics from other handlers
pub fn increment_auth_requests() {
    AUTH_REQUESTS_TOTAL.inc();
}

pub fn record_auth_duration(duration: f64) {
    AUTH_REQUEST_DURATION.observe(duration);
}

pub fn increment_failed_logins() {
    FAILED_LOGINS_TOTAL.inc();
}
