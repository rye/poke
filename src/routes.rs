use axum::{response::IntoResponse, routing::get, Router};

/// Replies with `"pong"`
pub(crate) async fn ping() -> impl IntoResponse {
	"pong".to_string()
}

pub(crate) fn router(webhooks: &crate::config::WebhookSet) -> axum::Router {
	let router = Router::new().route("/ping", get(ping));

	router
}
