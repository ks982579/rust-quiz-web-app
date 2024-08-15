//! backend/src/routes/get_quiz.rs
//! To fetch quizzes for a user.
use crate::error_chain_helper;
use crate::session_wrapper::SessionWrapper;
use crate::surrealdb_repo::Database;
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use models::quiz::SurrealQuiz;
use uuid::Uuid;

// Errors
#[derive(thiserror::Error)]
pub enum GetQuizError {
    #[error("{0}")]
    AuthorizationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for GetQuizError {
    /// Custom implementation to display root cause of errors
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_helper(self, f)
    }
}

impl ResponseError for GetQuizError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            GetQuizError::UnexpectedError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({"msg": "Unknown Error"}))
            }
            GetQuizError::AuthorizationError(msg) => HttpResponse::build(StatusCode::UNAUTHORIZED)
                .insert_header(ContentType::json())
                .json(serde_json::json!({ "msg": msg })),
        }
    }
}

// --- EndPoint ---
/// Route handler for fetching quizzes for a specific user.
#[tracing::instrument(name = "Request to Get Quizzes by User", skip(db, session))]
pub async fn get_my_quizzes(
    req: HttpRequest,
    session: SessionWrapper,
    db: web::Data<Database>,
) -> Result<HttpResponse, GetQuizError> {
    let some_user_id: Option<Uuid> = session
        .get_user_id()
        .map_err(|_| GetQuizError::UnexpectedError(anyhow::anyhow!("A SessionGetError")))?;
    dbg!(&some_user_id);

    // Middleware should catch unauthorized users, but just in case
    let user_id: String = if let Some(id) = some_user_id {
        id.to_string()
    } else {
        return Err(GetQuizError::AuthorizationError(
            "Session Token not found".to_string(),
        ));
    };

    // Fetch Data
    let surreal_ql = "SELECT * FROM quizzes WHERE author_id = $user_id";
    let mut surreal_response: surrealdb::Response = db
        .client
        .query(surreal_ql)
        .bind(("user_id", user_id))
        .await
        .map_err(|err| GetQuizError::UnexpectedError(anyhow::anyhow!(err)))?;

    let quizzes: Vec<SurrealQuiz> = surreal_response
        .take(0)
        .map_err(|err| GetQuizError::UnexpectedError(anyhow::anyhow!(err)))?;

    Ok(HttpResponse::Ok().json(quizzes))
}
