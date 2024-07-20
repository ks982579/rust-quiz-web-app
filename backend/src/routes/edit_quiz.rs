//! backend/src/routes/edit_quiz.rs
//! Endpoint to edit quiz information.
use crate::{error_chain_helper, session_wrapper::SessionWrapper, surrealdb_repo::Database};
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use models::quiz::{QuizJsonPkg, SurrealQuiz};
use serde::Deserialize;
use surrealdb::sql::{thing, Thing};
use uuid::Uuid;

// Errors
#[derive(thiserror::Error)]
pub enum EditQuizError {
    #[error("{0}")]
    ValidationError(#[source] anyhow::Error),
    #[error("{0}")]
    AuthorizationError(String),
    #[error("{0}")]
    OwnershipError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for EditQuizError {
    /// Custom implementation to display root cause of errors
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_helper(self, f)
    }
}

impl ResponseError for EditQuizError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            EditQuizError::UnexpectedError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({"msg": "Unknown Error"}))
            }
            EditQuizError::ValidationError(err) => HttpResponse::build(StatusCode::BAD_REQUEST)
                .insert_header(ContentType::json())
                .json(serde_json::json!({ "msg": err.to_string() })),
            EditQuizError::OwnershipError(anywho) => HttpResponse::build(StatusCode::FORBIDDEN)
                .insert_header(ContentType::json())
                .json(serde_json::json!({ "msg": anywho.to_string() })),
            EditQuizError::AuthorizationError(msg) => HttpResponse::build(StatusCode::UNAUTHORIZED)
                .insert_header(ContentType::json())
                .json(serde_json::json!({ "msg": msg })),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct QuizEditorQueryString {
    quiz: String,
}

/// ToDo: Documentation
#[tracing::instrument(
    name = "Request to Edit Quiz"
    skip(db, session)
)]
pub async fn edit_quiz(
    req: HttpRequest,
    session: SessionWrapper,
    db: web::Data<Database>,
    quiz: web::Query<QuizEditorQueryString>,
    quiz_pkg_pt: web::Json<QuizJsonPkg>,
) -> Result<HttpResponse, EditQuizError> {
    let quiz_data: QuizJsonPkg = quiz_pkg_pt.into_inner();

    quiz_data
        .validate_field()
        .context("Validation error")
        .map_err(|err| EditQuizError::ValidationError(err))?;

    let some_user_id: Option<Uuid> = session
        .get_user_id()
        .map_err(|_| EditQuizError::UnexpectedError(anyhow::anyhow!("A SessionGetError")))?;
    dbg!(&some_user_id);

    // Middleware should catch unauthorized users, but just in case
    let user_id: String = if let Some(id) = some_user_id {
        id.to_string()
    } else {
        return Err(EditQuizError::AuthorizationError(
            "Session Token not found".to_string(),
        ));
    };

    // Decode Query String
    let quiz_query_string: String = quiz.into_inner().quiz;
    let decoded_query_string: String = urlencoding::decode(&quiz_query_string)
        .expect("UTF-8")
        .into_owned();

    // If cannot be parsed, it cannot be in database
    let quiz_id: Thing = thing(&decoded_query_string)
        .context("Unable to parse query")
        .map_err(|err| EditQuizError::ValidationError(err))?;

    let surreal_quiz: Option<SurrealQuiz> = db
        .client
        .select(&quiz_id)
        .await
        .map_err(|err| EditQuizError::ValidationError(anyhow::anyhow!(err)))?;

    match &surreal_quiz {
        None => {
            return Err(EditQuizError::ValidationError(anyhow::anyhow!(
                "Quiz does not exist"
            )));
        }
        Some(qz) => {
            if qz.author_id != user_id {
                return Err(EditQuizError::OwnershipError(anyhow::anyhow!(
                    "User does not own quiz"
                )));
            }
        }
    }

    let created: Option<SurrealQuiz> = db
        .client
        .update(quiz_id)
        .merge(&quiz_data)
        .await
        .map_err(|e| EditQuizError::UnexpectedError(anyhow::anyhow!(e)))?;

    if let Some(qz) = created {
        Ok(HttpResponse::Ok().json(&qz))
    } else {
        Err(EditQuizError::UnexpectedError(anyhow::anyhow!(
            "Unsure what happened in Database"
        )))
    }
}
