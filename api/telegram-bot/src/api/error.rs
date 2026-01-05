use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// Define our application's error type
#[derive(Debug)]
pub enum AppError {
    // You can add different error variants here
    InternalServerError(anyhow::Error),
    NotFound(String),
}

// Implement `IntoResponse` for our `AppError`
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalServerError(err) => {
                // Log the internal error for debugging
                tracing::error!("Internal server error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

// This allows us to use `?` to automatically convert `anyhow::Error` into `AppError`
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        AppError::InternalServerError(err.into())
    }
}
