use axum::{routing::get, Router};

/// Replies with `"pong"`
pub(crate) async fn ping() -> String {
	"pong".to_string()
}

pub(crate) fn router() -> axum::Router {
	let router = Router::new().route("/ping", get(ping));

	router
}
