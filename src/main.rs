use axum::{routing::get, Router};

mod config;
mod routes;

pub struct Server {
	config: config::Server,
}

impl Server {
	/// Initializes the server using the [`Default`] implementation
	pub fn new(config: config::Server) -> Self {
		Self { config }
	}

	/// Performs any startup tasks.
	pub fn initialize(&mut self) -> &mut Self {
		self
	}

	/// Runs the server.
	pub async fn serve(&mut self) {
		let bind_addr = self.config.bind_addr();

		let router = Router::new().route("/ping", get(routes::ping));

		axum::Server::bind(&bind_addr)
			.serve(router.into_make_service())
			.await
			.unwrap();
	}
}

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();

	let config = config::get_config().expect("failed to load config");

	let server_config: config::Server =
		config::server_config(&config).expect("failed to get server config key");

	let mut server = Server::new(server_config);
	server.initialize();
	server.serve().await
}
