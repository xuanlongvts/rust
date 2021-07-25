use email_newsletter::configuration::get_configuration;
use email_newsletter::startup::run;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
	init_subscriber(subscriber);

	let get_configuration = get_configuration().expect("Failed to bind address");
	let connection_pool = PgPool::connect(&get_configuration.database.connection_string())
		.await
		.expect("Failed to connect to Postgres.");

	let address = format!(
		"{}:{}",
		get_configuration.application.host, get_configuration.application.port
	);
	let listener = TcpListener::bind(address)?;
	run(listener, connection_pool)?.await?;
	Ok(())
}
