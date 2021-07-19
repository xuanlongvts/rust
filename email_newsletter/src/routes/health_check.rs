use std::net::TcpListener;

fn spawn_app() -> String {
	let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");

	// We retrieve the port assigned to us by the OS
	let port = listener.local_addr().unwrap().port();
	let server = email_newsletter::run(listener).expect("Failed to bind address");
	let _ = tokio::spawn(server);

	format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn health_check_works() {
	let address = spawn_app();

	let client = reqwest::Client::new();

	let response = client
		.get(&format!("{}/health_check", &address))
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
	let address = spawn_app();
	let client = reqwest::Client::new();
	let body = "name=le%20long&email=longle%40gmail.com";

	// Act
	let response = client
		.post(&format!("{}/subscriptions", &address))
		.header("Content-Type", "application/x-www-form-urlencoded")
		.body(body)
		.send()
		.await
		.expect("Failed to execute request.");

	// Assert
	assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
	// Arrange
	let address = spawn_app();
	let client = reqwest::Client::new();
	let test_cases = vec![
		("name=le%20long", "missing the email"),
		("email=longle%40gmail.com", "missing the name"),
		("", "missing both name and email"),
	];

	for (invalid_body, error_message) in test_cases {
		// Act
		let response = client
			.post(&format!("{}/subscriptions", &address))
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
