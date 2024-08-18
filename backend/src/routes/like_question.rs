//! backend/src/routes/like_question.rs
//! endpoint to toggle like status on a question.
use actix_web::{HttpRequest, HttpResponse};

// TODO: Finish Implementation
/// Feature did not make it into initial release.
#[tracing::instrument(name = "Request to Toggle Question Like Status")]
pub async fn toggle_like(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}
