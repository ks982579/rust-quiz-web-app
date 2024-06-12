//! backend/src/routes/login_user.rs
//! Endpoint to log user into system.
//! Must set the Session Token in browser as well.
use crate::surrealdb_repo::Database;
use actix_web::{
    http::{header::ContentType, StatusCode},
    web, HttpRequest, HttpResponse, ResponseError,
};
use secrecy::{ExposeSecret, Secret};

pub async fn user_login(
    req: HttpRequest, // for tracing
    db: web::Data<Database>,
    user_info_ptr: web::Json<_>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    todo!();
}
