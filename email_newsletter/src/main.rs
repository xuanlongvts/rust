use email_newsletter::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let get_tcp_from_os = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
	run(get_tcp_from_os)?.await
}
