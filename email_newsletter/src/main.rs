use email_newsletter::configuration::get_configuration;
use email_newsletter::startup::run;
use env_logger::{Builder, Env};
use sqlx::postgres::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	Builder::from_env(Env::default().default_filter_or("info")).init();
	let get_configuration = get_configuration().expect("Failed to bind address");

	let connection_pool = PgPool::connect(&get_configuration.database.connection_string())
		.await
		.expect("Failed to connect to Postgres.");

	let address = format!("127.0.0.1:{}", get_configuration.application_port);
	let listener = TcpListener::bind(address)?;
	run(listener, connection_pool)?.await?;
	Ok(())
}
