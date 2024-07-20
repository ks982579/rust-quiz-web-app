//! backend/src/routes/create_quesstions.rs
//! The plural indicates handling a vector of sorts
use crate::{error_chain_helper, session_wrapper::SessionWrapper, surrealdb_repo::Database};
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use models::{
    model_errors::ModelErrors,
    questions::{
        EditQuestionJsonPkg, JsonQuestion, QuestionMC, SurrealGenericQuestionData,
        SurrealQuestionMC,
    },
    SurrealRecord,
};
use serde::Deserialize;
use surrealdb::opt::PatchOp;
use surrealdb::sql::{thing, Thing};
use uuid::Uuid;

// -- Errors --
#[derive(thiserror::Error)]
pub enum EditQuestionError {
    #[error("{0}")]
    ValidationError(#[source] anyhow::Error),
    #[error("{0}")]
    OwnershipError(#[source] anyhow::Error),
    #[error("{0}")]
    AuthorizationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for EditQuestionError {
    /// Custom implementation to display root cause of errors
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_helper(self, f)
    }
}

impl ResponseError for EditQuestionError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            EditQuestionError::UnexpectedError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({"msg": "Unknown Error"}))
            }
            EditQuestionError::ValidationError(anywho) => {
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({ "msg": anywho.to_string() }))
            }
            EditQuestionError::OwnershipError(anywho) => HttpResponse::build(StatusCode::FORBIDDEN)
                .insert_header(ContentType::json())
                .json(serde_json::json!({ "msg": anywho.to_string() })),
            EditQuestionError::AuthorizationError(msg) => {
                HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({ "msg": msg }))
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct EditQuestQueryString {
    quest: String,
}

// --- EndPoint ---
/// ToDo: Documentation
#[tracing::instrument(
    name = "Request to Edit Questions"
    skip(db, session)
)]
pub async fn edit_question(
    req: HttpRequest,
    session: SessionWrapper,
    db: web::Data<Database>,
    quest_qp: web::Query<EditQuestQueryString>,
    question_pkg_pt: web::Json<EditQuestionJsonPkg>,
) -> Result<HttpResponse, EditQuestionError> {
    // parse JSON package
    let question_data: EditQuestionJsonPkg = question_pkg_pt.into_inner();
    question_data
        .validate_fields()
        .map_err(|err| EditQuestionError::ValidationError(anyhow::anyhow!(err)))?;

    let question: JsonQuestion = question_data.question;

    // Get User Session ID
    let some_user_id: Option<Uuid> = session
        .get_user_id()
        .map_err(|_| EditQuestionError::UnexpectedError(anyhow::anyhow!("A SessionGetError")))?;

    // Middleware should catch unauthorized users, but just in case
    let user_id: String = if let Some(id) = some_user_id {
        id.to_string()
    } else {
        return Err(EditQuestionError::AuthorizationError(
            "Session Token not found".to_string(),
        ));
    };

    // Decode Query String
    let quest_query_string: String = quest_qp.into_inner().quest;
    let decoded_query_string: String = urlencoding::decode(&quest_query_string)
        .expect("UTF-8")
        .into_owned();

    // If cannot be parsed, it cannot be in database
    let quest_id: Thing = thing(&decoded_query_string)
        .context("Unable to parse query")
        .map_err(|err| EditQuestionError::ValidationError(err))?;

    // Checking  -- Error returned from database indicates no ID exists.
    let surreal_quest: Option<SurrealGenericQuestionData> = db
        .client
        .select(&quest_id)
        .await
        .map_err(|err| EditQuestionError::ValidationError(anyhow::anyhow!(err)))?;

    // Sanity checks
    match &surreal_quest {
        None => {
            return Err(EditQuestionError::ValidationError(anyhow::anyhow!(
                "Question does not exist"
            )));
        }
        Some(qz) => {
            if qz.author_id != user_id {
                return Err(EditQuestionError::OwnershipError(anyhow::anyhow!(
                    "User does not own question"
                )));
            }
        }
    }

    // If you have more questions, put into more lists
    // Each should follow same format as first, update and return response
    match question {
        JsonQuestion::MultipleChoice(what) => {
            // -- Save Question into Database
            let updated: Option<SurrealQuestionMC> = db
                .client
                .update(quest_id)
                .merge(&what)
                .await
                .map_err(|e| EditQuestionError::UnexpectedError(anyhow::anyhow!(e)))?;
            // Check it returned correctly
            if let Some(qst) = updated {
                Ok(HttpResponse::Ok().json(&qst))
            } else {
                Err(EditQuestionError::UnexpectedError(anyhow::anyhow!(
                    "Error updating question"
                )))
            }
        }
    }
}
