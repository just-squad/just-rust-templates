// private modules
mod handlers;
mod models;

use axum::{routing::get, Router};
use models::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn create_debug_router() -> Router {
    Router::new()
        // Liveness probe
        .route("/live", get(handlers::liveness_handler))
        // Readiness probe
        .route("/ready", get(handlers::readiness_handler))
        // The swagger UI endpoint
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc.json", ApiDoc::openapi()))
}
