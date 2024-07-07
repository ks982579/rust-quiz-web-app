//! backend/src/routes/get_question.rs
//! To fetch questions to a quiz.
//! This endpoint is only designed currently to handle the only one type of question, multiple
//! choice. A Breaking API changes will come in future when adding other questions types.
use crate::error_chain_helper;
use crate::surrealdb_repo::Database;
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use models::questions::{AllQuestions, SurrealQuestionMC};
use serde::Deserialize;
use surrealdb::sql::{thing, Thing};

// -- Errors --
#[derive(thiserror::Error)]
pub enum GetQuestionError {
    #[error("{0}")]
    AuthorizationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for GetQuestionError {
    /// Custom implementation to display root cause of errors
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_helper(self, f)
    }
}

impl ResponseError for GetQuestionError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            GetQuestionError::UnexpectedError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({"msg": "Unknown Error"}))
            }
            GetQuestionError::AuthorizationError(msg) => {
                HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({ "msg": msg }))
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct QuestionsQueryString {
    quiz: String,
}

/// Per documentation, 400 response returned if cannot serialize query.
#[tracing::instrument(name = "Request to Get Quizzes by User", skip(db))]
pub async fn get_questions(
    req: HttpRequest,
    db: web::Data<Database>,
    quiz: web::Query<QuestionsQueryString>,
) -> Result<HttpResponse, GetQuestionError> {
    let quiz_query_str: String = quiz.into_inner().quiz;
    let decoded_query_str: String = urlencoding::decode(&quiz_query_str)
        .expect("UTF-8")
        .into_owned();
    let qid: Thing = thing(&decoded_query_str).context("Unable to parse query string")?;

    let surreal_ql: &str = "SELECT * FROM questions_mc WHERE parent_quiz = $quiz_id";
    let mut surreal_response: surrealdb::Response = db
        .client
        .query(surreal_ql)
        .bind(("quiz_id", qid))
        .await
        .map_err(|err| GetQuestionError::UnexpectedError(anyhow::anyhow!(err)))?;

    let mc_questions: Vec<SurrealQuestionMC> = surreal_response
        .take(0)
        .map_err(|err| GetQuestionError::UnexpectedError(anyhow::anyhow!(err)))?;

    let all_questions: AllQuestions = AllQuestions { mc: mc_questions };

    Ok(HttpResponse::Ok().json(all_questions))
}
