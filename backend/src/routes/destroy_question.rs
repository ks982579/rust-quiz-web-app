//! backend/src/routes/destroy_question.rs
//! to delete a question from the database.
use crate::error_chain_helper;
use crate::session_wrapper::SessionWrapper;
use crate::surrealdb_repo::Database;
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use models::questions::SurrealQuestionMC;
use models::{model_errors::ModelErrors, quiz::SurrealQuiz};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{thing, Thing};
use uuid::Uuid;

// Errors
#[derive(thiserror::Error)]
pub enum DestroyQuestError {
    #[error("{0}")]
    AuthorizationError(String),
    #[error("{0}")]
    OwnershipError(#[source] anyhow::Error),
    #[error("{0}")]
    ValidationError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for DestroyQuestError {
    /// Custom implementation to display root cause of errors
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_helper(self, f)
    }
}

impl ResponseError for DestroyQuestError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            DestroyQuestError::UnexpectedError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({"msg": "Unknown Error"}))
            }
            DestroyQuestError::AuthorizationError(msg) => {
                HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({ "msg": msg }))
            }
            DestroyQuestError::OwnershipError(anywho) => HttpResponse::build(StatusCode::FORBIDDEN)
                .insert_header(ContentType::json())
                .json(serde_json::json!({ "msg": anywho.to_string() })),
            DestroyQuestError::ValidationError(anywho) => {
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({ "msg": anywho.to_string() }))
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct QuestDestroyerQueryString {
    quest: String,
}

/// TODO: Documentation
#[tracing::instrument(name = "Request to Destroy User's Quiz by User", skip(db, session))]
pub async fn destroy_my_quest(
    req: HttpRequest,
    session: SessionWrapper,
    db: web::Data<Database>,
    quest: web::Query<QuestDestroyerQueryString>,
) -> Result<HttpResponse, DestroyQuestError> {
    let some_user_id: Option<Uuid> = session
        .get_user_id()
        .map_err(|_| DestroyQuestError::UnexpectedError(anyhow::anyhow!("A SessionGetError")))?;

    // Middleware should catch unauthorized users, but just in case
    let user_id: String = if let Some(id) = some_user_id {
        id.to_string()
    } else {
        return Err(DestroyQuestError::AuthorizationError(
            "Session Token not found".to_string(),
        ));
    };

    // Decode Query String
    let quest_query_string: String = quest.into_inner().quest;
    let decoded_query_str: String = urlencoding::decode(&quest_query_string)
        .expect("UTF-8")
        .into_owned();

    // If cannot be parsed, it cannot be in database
    let quest_id: Thing = thing(&decoded_query_str)
        .context("Unable to parse query")
        .map_err(|err| DestroyQuestError::ValidationError(err))?;

    // Checking  -- Error returned from database indicates no ID exists.
    let surreal_quest: Option<SurrealQuestionMC> = db
        .client
        .select(&quest_id)
        .await
        .map_err(|err| DestroyQuestError::ValidationError(anyhow::anyhow!(err)))?;

    // Sanity checks
    match &surreal_quest {
        None => {
            return Err(DestroyQuestError::ValidationError(anyhow::anyhow!(
                "Question does not exist"
            )));
        }
        Some(qz) => {
            if qz.author_id != user_id {
                return Err(DestroyQuestError::OwnershipError(anyhow::anyhow!(
                    "User does not own question"
                )));
            }
        }
    }

    // Delete Quiz
    let deleted_quest: Option<SurrealQuestionMC> = db
        .client
        .delete(&quest_id)
        .await
        .map_err(|err| DestroyQuestError::UnexpectedError(anyhow::anyhow!(err)))?;

    // After removing vector to track questions on Quiz, nothing more to do

    Ok(HttpResponse::Ok().json(deleted_quest))
}
