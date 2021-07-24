use email_newsletter::configuration::{get_configuration, DatabaseSettings};
use email_newsletter::startup::run;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::io::{sink, stdout};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
	pub address: String,
	pub db_pool: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
	let default_filter_level = "info".to_string();
	let subscriber_name = "test".to_string();

	if std::env::var("TEST_LOG").is_ok() {
		let subscriber = get_subscriber(subscriber_name, default_filter_level, stdout);
		init_subscriber(subscriber);
	} else {
		let subscriber = get_subscriber(subscriber_name, default_filter_level, sink);
		init_subscriber(subscriber);
	}
});

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
	Lazy::force(&TRACING);

	// create database
	let mut connection = PgConnection::connect(&config.connection_string_without_db())
		.await
		.expect("Failed to connect to Postgres");
	connection
		.execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
		.await
		.expect("Failed to create database.");

	// Migrate database
	let connection_pool = PgPool::connect(&config.connection_string())
		.await
		.expect("Failed to connect to Postgres.");

	sqlx::migrate!()
		.run(&connection_pool)
		.await
		.expect("Failed to migrate the database");

	connection_pool
}

async fn spawn_app() -> TestApp {
	let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");

	// We retrieve the port assigned to us by the OS
	let port = listener.local_addr().unwrap().port();
	let address = format!("http://127.0.0.1:{}", port);

	let mut configuration = get_configuration().expect("Failed to read configuration.");
	configuration.database.database_name = Uuid::new_v4().to_string();
	let connection_pool = configure_database(&configuration.database).await;

	let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
	let _ = tokio::spawn(server);

	TestApp {
		address,
		db_pool: connection_pool,
	}
}

#[actix_rt::test]
async fn health_check_works() {
	let test_app = spawn_app().await;

	let client = reqwest::Client::new();

	let response = client
		.get(&format!("{}/health_check", &test_app.address))
		.send()
		.await
		.expect("Failed to execute request.");

	// Assert
	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
	// Arrange
	let test_app = spawn_app().await;

	let client = reqwest::Client::new();
	let body = "name=le%20long&email=longle%40gmail.com";

	// Act
	let response = client
		.post(&format!("{}/subscriptions", &test_app.address))
		.header("Content-Type", "application/x-www-form-urlencoded")
		.body(body)
		.send()
		.await
		.expect("Failed to execute request.");

	// Assert
	assert_eq!(200, response.status().as_u16());

	let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
		.fetch_one(&test_app.db_pool)
		.await
		.expect("Failed to fetch saved subscription.");
	assert_eq!(saved.email, "longle@gmail.com");
	assert_eq!(saved.name, "le long");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
	// Arrange
	let test_app = spawn_app().await;
	let client = reqwest::Client::new();
	let test_cases = vec![
		("name=le%20long", "missing the email"),
		("email=longle%40gmail.com", "missing the name"),
		("", "missing both name and email"),
	];

	for (invalid_body, error_message) in test_cases {
		// Act
		let response = client
			.post(&format!("{}/subscriptions", &test_app.address))
			.header("Content-Type", "application/x-www-form-urlencoded")
			.body(invalid_body)
			.send()
			.await
			.expect("Failed to execute request");

		// Assert
		assert_eq!(
			400,
			response.status().as_u16(),
			"The API did not fail with 400 Bad Request when the payload was {}.",
			error_message
		);
	}
}
