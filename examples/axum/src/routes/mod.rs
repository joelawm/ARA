use axum::{http::StatusCode, response::{IntoResponse, Response}, routing::get, Router};

pub async fn create_routes() -> Router {
	Router::new().route("/api/health", get(get_health))
}

/// Health check endpoint
/// Returns a 200 OK status if the service is healthy
pub async fn get_health() -> Response {
	(StatusCode::OK, format!("Not Found")).into_response()
}