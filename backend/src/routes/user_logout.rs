//! backend/src/routes/user_logout.rs
//! Quick endpoint to log user out of application.
//! Hopefully it deletes cookie from both browser and database.
use actix_web::{self, HttpRequest, HttpResponse};

#[tracing::instrument(name = "Log User Out")]
pub async fn user_logout(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}
