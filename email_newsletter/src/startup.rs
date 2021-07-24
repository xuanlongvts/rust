use crate::routes::{health_check, subscriber};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool_param: PgPool) -> Result<Server, std::io::Error> {
	// Wrap the connection in a smart pointer
	let db_pool = web::Data::new(db_pool_param);
	let server = HttpServer::new(move || {
		App::new()
			.wrap(TracingLogger::default())
			.route("/health_check", web::get().to(health_check))
			.route("/subscriptions", web::post().to(subscriber))
			.app_data(db_pool.clone())
	})
	.listen(listener)?
	.run();

	Ok(server)
}
