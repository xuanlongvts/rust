use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{NewSubscriber, SubscriberName, SubscriberEmail};

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
	let name = match SubscriberName::parse(form.0.name) {
		Ok(name) => name,
		Err(_) => return HttpResponse::BadRequest().finish(),
	};
	let email = match SubscriberEmail::parse(form.0.email) {
		Ok(email) => email,
		Err(_) => return HttpResponse::BadRequest().finish()
	};
	let new_subscriber = NewSubscriber {
		email,
		name
	};
	match insert_subscribe(&pool, &new_subscriber).await {
		Ok(_) => HttpResponse::Ok().finish(),
		Err(_) => HttpResponse::InternalServerError().finish()
	}
}

#[tracing::instrument(
	name = "Saving new subscriber details in the database",
	skip(new_subscriber, pool)
)]
pub async fn insert_subscribe(pool: &PgPool, new_subscriber: &NewSubscriber) -> Result<(), sqlx::Error> {
	sqlx::query!(
		r#"
		INSERT INTO subscriptions (id, email, name, subscribed_at)
		VALUES ($1, $2, $3, $4)
		"#,
		Uuid::new_v4(),
		new_subscriber.email.as_ref(),
		new_subscriber.name.as_ref(),
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
