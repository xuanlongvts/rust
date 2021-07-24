use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
	email: String,
	name: String,
}

#[tracing::instrument(
	name="Adding a new subscriber.", 
	skip(form, pool), 
	fields(email = %form.email, name = %form.name)
)]
pub async fn subscriber(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
	match insert_subscribe(&pool, &form).await {
		Ok(_) => HttpResponse::Ok().finish(),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[tracing::instrument(
	name = "Saving new subscriber details in the database",
	skip(form, pool)
)]
pub async fn insert_subscribe(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
	sqlx::query!(
		r#"
		INSERT INTO subscriptions (id, email, name, subscribed_at)
		VALUES ($1, $2, $3, $4)
		"#,
		Uuid::new_v4(),
		form.email,
		form.name,
		Utc::now()
	)
	.execute(pool)
	.await
	.map_err(|e| {
		tracing::error!("Failed to excute query: {:?}", e);
		e
	})?;
	
	Ok(())
}
