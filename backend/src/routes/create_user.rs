//! backend/src/routes/create_user.rs
//! Endpoint used for user creation given credentials.
use crate::error_chain_helper;
use crate::surrealdb_repo::Database;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use models::GeneralUser;
use serde::Deserialize;
use uuid::Uuid;

// Errors
#[derive(thiserror::Error)]
pub enum CreateUserError {
    // #[error("{0}")]
    // PasswordsDontMatch(String),
    // #[error("{0}")]
    // MissingName(String),
    // #[error("{0}")]
    // MissingUsername(String),
    // #[error("{0}")]
    // MissingPassword(String),
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for CreateUserError {
    /// Custom implementation to display root cause of errors
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_helper(self, f)
    }
}

impl ResponseError for CreateUserError {
    // not required method actually...
    // fn status_code(&self) -> StatusCode {
    //     match *self {
    //         CreateUserError::ValidationError(_) => StatusCode::BAD_REQUEST,
    //         CreateUserError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    //     }
    // }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            CreateUserError::UnexpectedError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({"msg": "Unknown Error"}))
            }
            CreateUserError::ValidationError(msg) => HttpResponse::build(StatusCode::BAD_REQUEST)
                .insert_header(ContentType::json())
                .json(serde_json::json!({ "msg": msg })),
        }
    }
}

// Structs for JSON
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserPayload {
    name: String,
    username: String,
    password: String,
}

impl Into<GeneralUser> for CreateUserPayload {
    fn into(self) -> GeneralUser {
        let uuid: String = Uuid::new_v4()
            .hyphenated()
            .encode_lower(&mut Uuid::encode_buffer())
            .to_string();
        GeneralUser::new(uuid, self.name, self.username, self.password)
    }
}

impl CreateUserPayload {
    /// Main purpose is to be used with ? to escape logic if fields are not
    /// correctly filled in.
    fn validate_fields(&self) -> Result<(), CreateUserError> {
        if self.username.trim().len() < 1 {
            Err(CreateUserError::ValidationError(String::from(
                "Username is required, cannot be empty space.",
            )))
        } else if self.name.trim().len() < 1 {
            Err(CreateUserError::ValidationError(String::from(
                "Name is required, cannot be empty space.",
            )))
        } else if self.password.len() < 6 {
            Err(CreateUserError::ValidationError(
                "Password must be at least 6 characters long".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

/// Takes in JSON with user information and stores in database.
/// If successful, returns 201 CREATED.
#[tracing::instrument(name = "Request to Create User")]
pub async fn create_user(
    req: HttpRequest,
    db: web::Data<Database>,
    user_info_pt: web::Json<CreateUserPayload>,
) -> Result<HttpResponse, CreateUserError> {
    let user_data = user_info_pt.into_inner();

    // Checking Data
    user_data.validate_fields()?;
    // Is username unique?
    let _ = unique_username(&db, &user_data.username).await?;

    let new_user_opt: Option<GeneralUser> = db.add_general_user(user_data.into()).await;
    let new_user = new_user_opt.unwrap();

    // println!("{user_info:?}");
    Ok(HttpResponse::Created()
        .content_type(ContentType::json())
        .json(new_user))
}

async fn unique_username(
    db: &web::Data<Database>,
    username: &str,
) -> Result<bool, CreateUserError> {
    let users = db
        .count_users(&username)
        .await
        // returns Anyhow error which converts to UnknownError
        .context("Issue performing count")?;

    if users > 0 {
        Err(CreateUserError::ValidationError(String::from(
            "Username already exists",
        )))
    } else {
        Ok(true)
    }
}
