//! backend/src/routes/create_user.rs
//! Endpoint used for user creation given credentials.
use crate::authentication::create_password_hash;
use crate::error_chain_helper;
use crate::surrealdb_repo::Database;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use models::GeneralUser;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use uuid::Uuid;

// Errors
#[derive(thiserror::Error)]
pub enum CreateUserError {
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

// TODO -> Implement a "secure" struct
// Structs for JSON
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserPayload {
    name: String,
    username: String,
    // Need secrect to hid password in logs
    password: Secret<String>,
}

impl From<CreateUserPayload> for GeneralUser {
    fn from(value: CreateUserPayload) -> Self {
        // hash should not fail
        let password_hash = value.hash_password().unwrap();
        let uuid_s: String = Uuid::new_v4()
            .hyphenated()
            .encode_lower(&mut Uuid::encode_buffer())
            .to_string();
        GeneralUser::new(
            uuid_s,
            value.name,
            value.username,
            password_hash.expose_secret().to_string(),
        )
    }
}

// impl Into<GeneralUser> for CreateUserPayload {
//     fn into(self) -> GeneralUser {
//         GeneralUser::new(
//             uuid,
//             self.name,
//             self.username,
//             self.password.expose_secret().to_string(),
//         )
//     }
// }

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
        } else if self.password.expose_secret().len() < 6 {
            Err(CreateUserError::ValidationError(
                "Password must be at least 6 characters long".to_string(),
            ))
        } else {
            Ok(())
        }
    }
    pub fn hash_password(&self) -> Result<Secret<String>, anyhow::Error> {
        create_password_hash(self.password.clone())
    }
}

/// Takes in JSON with user information and stores in database.
/// If successful, returns 201 CREATED.
#[tracing::instrument(name = "Request to Create User", skip(db))]
pub async fn create_user(
    req: HttpRequest, // for tracing
    db: web::Data<Database>,
    user_info_pt: web::Json<CreateUserPayload>,
) -> Result<HttpResponse, CreateUserError> {
    let user_data = user_info_pt.into_inner();

    // Checking Data
    user_data.validate_fields()?;
    // Is username unique?
    let _ = unique_username(&db, &user_data.username).await?;

    // Do not return, General User has hashed password
    let _: Option<GeneralUser> = db.add_general_user(user_data.into()).await;

    // println!("{user_info:?}");
    // Unless Something comes up, no good reason to return JSON information
    Ok(HttpResponse::Created()
        .content_type(ContentType::json())
        .finish())
    // .json(new_user))
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
