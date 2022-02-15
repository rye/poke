use std::{net::IpAddr, str::FromStr};

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

	/// Sets up the filters to use when handling requests.
	///
	/// Any work done here is done at startup before the server begins serving requests.
	#[deprecated = "warp is being removed"]
	fn filters(
		&self,
	) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
		use warp::Filter;

		let logger = warp::log("slowpoke");

		let ping_filter = warp::path!("ping")
			.then(routes::ping::ping)
			.with(logger)
			.boxed();

		ping_filter
	}

	/// Runs the server.
	pub async fn serve(&mut self) {
		let bind = IpAddr::from_str("::0").unwrap();
		let port = 3030;

		let bind_addr = (bind, port).into();

		let router = axum::Router::new().route("/ping", axum::routing::get(routes::ping::ping));

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
