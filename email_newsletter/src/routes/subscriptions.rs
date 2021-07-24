use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing;
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
	email: String,
	name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
	let request_id = Uuid::new_v4();
	let request_span = tracing::info_span!(
		"Adding a new subscriber.",
		%request_id,
		email = %form.email,
		name = %form.name
	);
	let _request_span_guard = request_span.enter();
	let query_span = tracing::info_span!("Saving new subscriber details in the database");
	// tracing::info!("Saving new subscriber details in the database");
	match sqlx::query!(
		r#"
		INSERT INTO subscriptions (id, email, name, subscribed_at)
		VALUES ($1, $2, $3, $4)
		"#,
		request_id,
		form.email,
		form.name,
		Utc::now()
	)
	.execute(pool.as_ref())
	.instrument(query_span)
	.await
	{
		Ok(_) => HttpResponse::Ok().finish(),
		Err(e) => {
			tracing::error!("Failed to execute query: {:?}", e);
			HttpResponse::InternalServerError().finish()
		}
	}
}
