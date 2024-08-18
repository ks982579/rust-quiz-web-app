//! backend/src/routes/user_logout.rs
//! Quick endpoint to log user out of application.
//! Deletes cookie from both browser and database.
use crate::authentication::http_500;
use crate::session_wrapper::SessionWrapper;
use actix_web::{self, HttpRequest, HttpResponse};

// --- EndPoint ---
/// Route handler for Logging user out of application.
#[tracing::instrument(name = "Log User Out", skip(session))]
pub async fn user_logout(
    req: HttpRequest,
    session: SessionWrapper,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(http_500)?.is_none() {
        Ok(HttpResponse::Ok().finish())
    } else {
        session.log_out();
        Ok(HttpResponse::Ok().finish())
    }
}
