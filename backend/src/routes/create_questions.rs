//! backend/src/routes/create_quesstions.rs
//! The plural indicates handling a vector of sorts
use crate::{error_chain_helper, session_wrapper::SessionWrapper, surrealdb_repo::Database};
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use models::questions::{QuestionMC, SurrealQuestionMC};
use models::SurrealRecord;
use serde::{Deserialize, Serialize};
use surrealdb::opt::PatchOp;
use surrealdb::sql::{Id, Thing};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonPkg {
    pub quiz_id: Thing,
    pub questions: Vec<JsonQuestion>,
}

/// To allow for the easy transporation of data
/// If adding another type, be sure to update the `JsonPkg::validate_fields()` method.
#[derive(Serialize, Deserialize, Debug)]
pub enum JsonQuestion {
    MultipleChoice(JsonQuestionMC),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonQuestionMC {
    pub question: String,
    pub hint: Option<String>,
    pub answer: String,
    pub choices: Vec<String>,
}

impl JsonPkg {
    fn validate_fields(&self) -> Result<(), CreateQuestionError> {
        // The type system ensures quiz_id isn't empty
        use JsonQuestion::*;
        for (num, what) in self.questions.iter().enumerate() {
            match what {
                MultipleChoice(qmc) => {
                    if qmc.question.trim().len() < 1 {
                        return Err(CreateQuestionError::ValidationError(format!(
                            "Q{}: Question cannot be empty",
                            num
                        )));
                    } else if qmc.answer.trim().len() < 1 {
                        return Err(CreateQuestionError::ValidationError(format!(
                            "Q{}: Question needs valid answer",
                            num
                        )));
                    } else if qmc.choices.len() < 1 {
                        return Err(CreateQuestionError::ValidationError(format!(
                            "Q{}: Question needs at least one additional choice",
                            num
                        )));
                    }
                    // Could loop through choices to ensure they are also not blank
                }
            }
        }
        Ok(())
    }
}

// -- Errors --
#[derive(thiserror::Error)]
pub enum CreateQuestionError {
    #[error("{0}")]
    ValidationError(String),
    #[error("{0}")]
    AuthorizationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for CreateQuestionError {
    /// Custom implementation to display root cause of errors
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_helper(self, f)
    }
}

impl ResponseError for CreateQuestionError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            CreateQuestionError::UnexpectedError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({"msg": "Unknown Error"}))
            }
            CreateQuestionError::ValidationError(msg) => {
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({ "msg": msg }))
            }
            CreateQuestionError::AuthorizationError(msg) => {
                HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({ "msg": msg }))
            }
        }
    }
}

// --- EndPoint ---
/// ToDo: Documentation
#[tracing::instrument(
    name = "Request to Create Questions"
    skip(db, session)
)]
pub async fn create_new_questions(
    req: HttpRequest,
    session: SessionWrapper,
    db: web::Data<Database>,
    question_pkg_pt: web::Json<JsonPkg>,
) -> Result<HttpResponse, CreateQuestionError> {
    let question_data: JsonPkg = question_pkg_pt.into_inner();
    question_data.validate_fields()?;

    let quiz_id: Thing = question_data.quiz_id;
    let questions: Vec<JsonQuestion> = question_data.questions;
    // `question_data` is no longer valid FYI

    let some_user_id: Option<Uuid> = session
        .get_user_id()
        .map_err(|_| CreateQuestionError::UnexpectedError(anyhow::anyhow!("A SessionGetError")))?;
    dbg!(&some_user_id);

    // Middleware should catch unauthorized users, but just in case
    let user_id: String = if let Some(id) = some_user_id {
        id.to_string()
    } else {
        return Err(CreateQuestionError::AuthorizationError(
            "Session Token not found".to_string(),
        ));
    };

    // Lists of certain question types here to have data pushed to.
    let mut questions_to_save_mc: Vec<QuestionMC> = Vec::new();

    // If you have more questions, put into more lists
    for q in questions {
        match q {
            JsonQuestion::MultipleChoice(what) => questions_to_save_mc.push(QuestionMC {
                question: what.question,
                hint: what.hint,
                author_id: user_id.clone(),
                parent_quiz: quiz_id.clone(),
                answer: what.answer,
                choices: what.choices,
            }),
        }
    }

    // Load Questions Here
    let uploaded_questions_mc: Vec<SurrealRecord> = db
        .client
        .insert("questions_mc")
        .content(questions_to_save_mc)
        .await
        .map_err(|e| CreateQuestionError::UnexpectedError(anyhow::anyhow!(e)))?;

    let thing_list: Vec<Thing> = uploaded_questions_mc.into_iter().map(|r| r.id).collect();

    // Try Update quiz
    let _: Option<SurrealRecord> = db
        .client
        .update(&quiz_id)
        .patch(PatchOp::replace("/questions_mc", thing_list))
        .await
        .map_err(|e| CreateQuestionError::UnexpectedError(anyhow::anyhow!(e)))?;

    // Dummy response to shut up linter
    Ok(HttpResponse::Created().json(serde_json::json!({"msg": "ok"})))
}
