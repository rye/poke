use axum::{routing::get, Router};

mod config;
mod routes;

pub struct Server {
	config: config::Server,
}

impl Server {
	/// Initializes the server using the [`Default`] implementation
	pub(crate) fn new(config: config::Server) -> Self {
		Self { config }
	}

	/// Performs any startup tasks.
	pub(crate) fn initialize(&mut self) -> &mut Self {
		self
	}

	/// Runs the server.
	pub(crate) async fn serve(&mut self) -> Result<(), RuntimeError> {
		let bind_addr = self.config.bind_addr();

		let router = Router::new().route("/ping", get(routes::ping));

		Ok(
			axum::Server::bind(&bind_addr)
				.serve(router.into_make_service())
				.await?,
		)
	}
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
	#[error("hyper error")]
	Hyper(#[from] hyper::Error),

	#[error("configuration error")]
	Config(#[from] ConfigError),
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
	#[error("general config error")]
	General(::config::ConfigError),

	#[error("server configuration error")]
	Server(::config::ConfigError),
}

#[tokio::main]
async fn main() -> Result<(), RuntimeError> {
	tracing_subscriber::fmt::init();

	let config = config::get_config().map_err(ConfigError::General)?;

	let server_config: config::Server =
		config::server_config(&config).map_err(ConfigError::Server)?;

	let mut server = Server::new(server_config);
	server.initialize();
	server.serve().await
}
