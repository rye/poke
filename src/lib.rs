use std::{net::IpAddr, str::FromStr};

use axum::{routing::get, Router};

mod routes;

pub struct Server {}

impl Server {
	/// Initializes the server using the [`Default`] implementation
	pub fn new() -> Self {
		Default::default()
	}

	/// Performs any startup tasks.
	pub fn initialize(&mut self) -> &mut Self {
		self
	}

	/// Runs the server.
	pub async fn serve(&mut self) {
		let bind = IpAddr::from_str("::0").unwrap();
		let port = 3030;

		let bind_addr = (bind, port).into();

		let router = Router::new().route("/ping", get(routes::ping));

		axum::Server::bind(&bind_addr)
			.serve(router.into_make_service())
			.await
			.unwrap();
	}
}

impl Default for Server {
	fn default() -> Self {
		Self {}
	}
}
