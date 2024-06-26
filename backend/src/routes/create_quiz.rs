//! backend/src/routes/create_quiz.rs
use crate::surrealdb_repo::Database;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonPkg {
    name: String,
}

#[tracing::instrument(
    name = "Request to Create Quiz"
    skip(db)
)]
pub async fn create_new_quiz(
    req: HttpRequest,
    db: web::Data<Database>,
    quiz_pkg_pt: web::Json<JsonPkg>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
