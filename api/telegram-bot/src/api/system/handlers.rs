use axum::{http::StatusCode, response::IntoResponse, Json};

/// Liveness probe endpoint.
pub async fn liveness_handler() -> impl IntoResponse {
    const MESSAGE: &str = "healthy";

    Json(MESSAGE)
}

/// Readiness probe endpoint.
pub async fn readiness_handler() -> impl IntoResponse {
    // TODO: Check database connection and other dependencies before returning OK.
    StatusCode::OK
}
