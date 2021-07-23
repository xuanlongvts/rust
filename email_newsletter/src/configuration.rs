#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
	pub username: String,
	pub password: String,
	pub port: i16,
	pub host: String,
	pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct Settting {
	pub database: DatabaseSettings,
	pub application_port: i16,
}

impl DatabaseSettings {
	pub fn connection_string(&self) -> String {
		format!(
			"postgres://{}:{}@{}:{}/{}",
			self.username, self.password, self.host, self.port, self.database_name
		)
	}

	pub fn connection_string_without_db(&self) -> String {
		format!(
			"postgres://{}:{}@{}:{}",
			self.username, self.password, self.host, self.port
		)
	}
}

pub fn get_configuration() -> Result<Settting, config::ConfigError> {
	let mut settings = config::Config::default();

	settings.merge(config::File::with_name("configuration"))?;
	settings.try_into()
}
