//! backend/src/routes/create_user.rs
//! Endpoint used for user creation given credentials.
use actix_web::http::header::ContentType;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserPayload {
    name: String,
    username: String,
    password: String,
}

#[tracing::instrument(name = "Request to Create User")]
pub async fn create_user(
    req: HttpRequest,
    // user_info: web::Json<CreateUserPayload>,
) -> HttpResponse {
    // pub async fn create_user() -> HttpResponse {
    for (key, value) in req.headers().iter() {
        println!("{}: {}", key.as_str(), value.to_str().unwrap_or_default());
    }
    // println!("{user_info:?}");
    HttpResponse::Ok()
        .content_type(ContentType::json())
        // .body("data")
        .finish()
}
