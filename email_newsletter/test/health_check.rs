use std::net::TcpListener;

fn spawn_app() -> String {
	let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");

	// We retrieve the port assigned to us by the OS
	let port = listener.local_addr().unwrap().port();
	let server = email_newsletter::run(listener).expect("Failed to bind address");
	let _ = tokio::spawn(server);

	format!("http://127.0.0.1:{}", port)
}

#[actix_rs::test]
async fn health_check_works() {
	let address = spawn_app();

	let client = reqwest::Client::new();

	let response = client
		.get(&format!("{}/health_check", &address)
		.send()
		.await
		.expect("Failed to execute request.");

	// Assert
	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}
