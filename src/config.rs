use std::net::IpAddr;

use config::Config;

#[derive(serde::Deserialize)]
pub(crate) struct Server {
	host: IpAddr,
	port: u16,
}

impl Server {
	pub(crate) fn bind_addr(&self) -> std::net::SocketAddr {
		(self.host, self.port).into()
	}
}

pub(crate) fn get_config() -> Result<Config, config::ConfigError> {
	let mut config = Config::default();

	config.set_default("server.host", "::")?;
	config.set_default("server.port", 3030)?;

	config.merge(config::File::with_name("config"))?;

	Ok(config)
}

pub(crate) fn server_config(config: &Config) -> Result<Server, config::ConfigError> {
	config.get("server")
}
