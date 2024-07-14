//! backend/src/routes/get_quiz.rs
//! To fetch quizzes for a user.
use crate::error_chain_helper;
use crate::session_wrapper::SessionWrapper;
use crate::surrealdb_repo::Database;
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use models::{model_errors::ModelErrors, quiz::SurrealQuiz};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{thing, Thing};
use uuid::Uuid;

// Errors
#[derive(thiserror::Error)]
pub enum DestroyQuizError {
    #[error("{0}")]
    AuthorizationError(String),
    #[error("{0}")]
    OwnershipError(#[source] anyhow::Error),
    #[error("{0}")]
    ValidationError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for DestroyQuizError {
    /// Custom implementation to display root cause of errors
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_helper(self, f)
    }
}

impl ResponseError for DestroyQuizError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            DestroyQuizError::UnexpectedError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({"msg": "Unknown Error"}))
            }
            DestroyQuizError::AuthorizationError(msg) => {
                HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({ "msg": msg }))
            }
            DestroyQuizError::OwnershipError(anywho) => HttpResponse::build(StatusCode::FORBIDDEN)
                .insert_header(ContentType::json())
                .json(serde_json::json!({ "msg": anywho.to_string() })),
            DestroyQuizError::ValidationError(anywho) => {
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({ "msg": anywho.to_string() }))
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct QuizDestroyerQueryString {
    quiz: String,
}

/// TODO: Documentation
#[tracing::instrument(name = "Request to Destroy User's Quiz by User", skip(db, session))]
pub async fn destroy_my_quiz(
    req: HttpRequest,
    session: SessionWrapper,
    db: web::Data<Database>,
    quiz: web::Query<QuizDestroyerQueryString>,
) -> Result<HttpResponse, DestroyQuizError> {
    let some_user_id: Option<Uuid> = session
        .get_user_id()
        .map_err(|_| DestroyQuizError::UnexpectedError(anyhow::anyhow!("A SessionGetError")))?;

    // Middleware should catch unauthorized users, but just in case
    let user_id: String = if let Some(id) = some_user_id {
        id.to_string()
    } else {
        return Err(DestroyQuizError::AuthorizationError(
            "Session Token not found".to_string(),
        ));
    };

    // Decode Query String
    let quiz_query_str: String = quiz.into_inner().quiz;
    let decoded_query_str: String = urlencoding::decode(&quiz_query_str)
        .expect("UTF-8")
        .into_owned();

    // If cannot be parsed, it cannot be in database
    let quiz_id: Thing = thing(&decoded_query_str)
        .context("Unable to parse query")
        .map_err(|err| DestroyQuizError::ValidationError(err))?;

    // Checking  -- Error returned from database indicates no ID exists.
    let mut surreal_quiz: Option<SurrealQuiz> = db
        .client
        .select(&quiz_id)
        .await
        .map_err(|err| DestroyQuizError::ValidationError(anyhow::anyhow!(err)))?;

    dbg!(&surreal_quiz);

    // Sanity checks
    match &surreal_quiz {
        None => {
            return Err(DestroyQuizError::ValidationError(anyhow::anyhow!(
                "Quiz does not exist"
            )));
        }
        Some(qz) => {
            if qz.author_id != user_id {
                return Err(DestroyQuizError::OwnershipError(anyhow::anyhow!(
                    "User does not own quiz"
                )));
            }
        }
    }

    dbg!(&surreal_quiz);

    // Delete Quiz
    let _deleted_quiz: Option<SurrealQuiz> = db
        .client
        .delete(&quiz_id)
        .await
        .map_err(|err| DestroyQuizError::UnexpectedError(anyhow::anyhow!(err)))?;
    // Delete related questions

    // Delete from MC table
    let surreal_ql = "DELETE type::table($table) WHERE author_id = $user_id";
    let mut surreal_response: surrealdb::Response = db
        .client
        .query(surreal_ql)
        .bind(("table", "questions_mc"))
        .bind(("user_id", user_id))
        .await
        .map_err(|err| DestroyQuizError::UnexpectedError(anyhow::anyhow!(err)))?;

    let quizzes: Vec<SurrealQuiz> = surreal_response
        .take(0)
        .map_err(|err| DestroyQuizError::UnexpectedError(anyhow::anyhow!(err)))?;

    Ok(HttpResponse::Ok().json(quizzes))
}
