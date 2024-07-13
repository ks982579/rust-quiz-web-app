//! backend/src/routes/create_quiz.rs
use crate::{error_chain_helper, session_wrapper::SessionWrapper, surrealdb_repo::Database};
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use models::{
    model_errors::ModelErrors,
    quiz::{Quiz, QuizJsonPkg, SurrealQuiz},
};
use surrealdb::sql::Id;
use uuid::Uuid;

// Errors
#[derive(thiserror::Error)]
pub enum CreateQuizError {
    #[error(transparent)]
    ValidationError(#[from] ModelErrors),
    #[error("{0}")]
    AuthorizationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for CreateQuizError {
    /// Custom implementation to display root cause of errors
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_helper(self, f)
    }
}

impl ResponseError for CreateQuizError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            CreateQuizError::UnexpectedError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({"msg": "Unknown Error"}))
            }
            CreateQuizError::ValidationError(err) => HttpResponse::build(StatusCode::BAD_REQUEST)
                .insert_header(ContentType::json())
                .json(serde_json::json!({ "msg": err.to_string() })),
            CreateQuizError::AuthorizationError(msg) => {
                HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({ "msg": msg }))
            }
        }
    }
}

/// ToDo: Documentation
#[tracing::instrument(
    name = "Request to Create Quiz"
    skip(db, session)
)]
pub async fn create_new_quiz(
    req: HttpRequest,
    session: SessionWrapper,
    db: web::Data<Database>,
    quiz_pkg_pt: web::Json<QuizJsonPkg>,
) -> Result<HttpResponse, CreateQuizError> {
    let quiz_data: QuizJsonPkg = quiz_pkg_pt.into_inner();
    quiz_data.validate_field()?;

    let some_user_id: Option<Uuid> = session
        .get_user_id()
        .map_err(|_| CreateQuizError::UnexpectedError(anyhow::anyhow!("A SessionGetError")))?;
    dbg!(&some_user_id);

    // Middleware should catch unauthorized users, but just in case
    let user_id: String = if let Some(id) = some_user_id {
        id.to_string()
    } else {
        return Err(CreateQuizError::AuthorizationError(
            "Session Token not found".to_string(),
        ));
    };

    let quiz_to_save: Quiz = Quiz::new(quiz_data.name, quiz_data.description, user_id);
    dbg!(&quiz_to_save);
    dbg!(Id::uuid().to_string());

    let created: Vec<SurrealQuiz> = db
        .client
        .create("quizzes")
        .content(&quiz_to_save)
        .await
        .map_err(|e| CreateQuizError::UnexpectedError(anyhow::anyhow!(e)))?;

    if created.len() == 1 {
        Ok(HttpResponse::Ok().json(&created[0]))
    } else {
        Err(CreateQuizError::UnexpectedError(anyhow::anyhow!(
            "Unsure what happened in Database"
        )))
    }
}
