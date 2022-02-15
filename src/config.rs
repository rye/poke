use config::Config;

pub(crate) fn get_config() -> Result<Config, config::ConfigError> {
	let mut config = Config::default();

	config.set_default("server.host", "::")?;
	config.set_default("server.port", 3030)?;

	config.merge(config::File::with_name("config"))?;

	Ok(config)
}
