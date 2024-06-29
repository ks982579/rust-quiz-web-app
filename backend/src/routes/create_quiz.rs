//! backend/src/routes/create_quiz.rs
use crate::{error_chain_helper, session_wrapper::SessionWrapper, surrealdb_repo::Database};
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonPkg {
    name: String,
    description: String,
}

impl JsonPkg {
    fn validate_field(&self) -> Result<(), CreateQuizError> {
        if self.name.trim().len() < 1 {
            Err(CreateQuizError::ValidationError(String::from(
                "Quiz name cannot be blank or white space",
            )))
        } else {
            Ok(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FullQuiz {
    pub id: Thing,
    pub name: String,
    pub description: String,
    pub author_id: String,
    pub questions_mc: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quiz {
    pub name: String,
    pub description: String,
    pub author_id: String,
    pub questions_mc: Vec<String>,
}

impl Quiz {
    pub fn new(name: String, description: String, author_id: String) -> Self {
        Self {
            name,
            description,
            author_id,
            questions_mc: Vec::new(),
        }
    }
}

// Errors
#[derive(thiserror::Error)]
pub enum CreateQuizError {
    #[error("{0}")]
    ValidationError(String),
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
            CreateQuizError::ValidationError(msg) => HttpResponse::build(StatusCode::BAD_REQUEST)
                .insert_header(ContentType::json())
                .json(serde_json::json!({ "msg": msg })),
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
    quiz_pkg_pt: web::Json<JsonPkg>,
) -> Result<HttpResponse, CreateQuizError> {
    let quiz_data: JsonPkg = quiz_pkg_pt.into_inner();
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

    let created: Option<FullQuiz> = db
        .client
        .create(("quizzes", Id::uuid().to_string()))
        .content(&quiz_to_save)
        .await
        .map_err(|e| CreateQuizError::ValidationError(e.to_string()))?;

    // Dummy response to shut up linter
    Ok(HttpResponse::Ok().json(&created))
}