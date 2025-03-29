use axum::{http::StatusCode, response::{IntoResponse, Response}, routing::get, Router, Extension};
use sqlx::{Pool, Postgres};

pub async fn create_routes() -> Router {
	Router::new().route("/api/health", get(get_health))
}

/// Health check endpoint
/// Returns a 200 OK status if the service is healthy
pub async fn get_health() -> Response {
	(StatusCode::OK, format!("Not Found")).into_response()
}

pub async fn get_list(headers: HeaderMap, Extension(pool): Extension<Pool<Postgres>>) -> Result<Response, Response> {
	// Get headers
	let user_agent: String = match headers.get(USER_AGENT) {
		Some(context) => context.to_str().unwrap_or_default().to_string(),
		None => return Err(CreateOauthUser::MissingUserAgent.into_response()),
	};

	// Get Database connection
	let mut conn = match pool.acquire().await {
		Ok(context) => context,
		Err(e) => {
			tracing::error!("Error getting a connection from the pool: {:?}", e);
			return Err(Error::PoolConnection.into_response())
		}
	};
}