//! backend/src/routes/login_user.rs
//! Endpoint to log user into system.
//! Must set the Session Token in browser as well.
use crate::{authentication::verify_password_hash, error_chain_helper, surrealdb_repo::Database};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web, HttpRequest, HttpResponse, ResponseError,
};
use anyhow::Context;
use models::GeneralUser;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

/// Input struct for JSON
#[derive(Debug, Clone, Deserialize)]
pub struct UserCredentials {
    username: String,
    // Secret to hide password from telemetry
    password: Secret<String>,
}

#[derive(thiserror::Error)]
pub enum UserLoginError {
    #[error("Authentication Failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Unexpected error")]
    UnexpectedError(#[from] anyhow::Error),
}

/// Custom Debug implementation to log the root cause.
impl std::fmt::Debug for UserLoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_helper(self, f)
    }
}

impl ResponseError for UserLoginError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            UserLoginError::UnexpectedError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .insert_header(ContentType::json())
                    .json(serde_json::json!({"msg": "Unknown Error"}))
            }
            // not passing information through so slightly harder to guess a username
            UserLoginError::AuthError(_) => HttpResponse::build(StatusCode::BAD_REQUEST)
                .insert_header(ContentType::json())
                .json(serde_json::json!({ "msg": "Incorrect username of password" })),
        }
    }
}

// -- Traits for DB
trait LookUpUser {
    async fn get_user_by_username(
        &self,
        username: String,
    ) -> Result<Option<GeneralUser>, anyhow::Error>;
}

impl LookUpUser for Database {
    async fn get_user_by_username(
        &self,
        username: String,
    ) -> Result<Option<GeneralUser>, anyhow::Error> {
        let query: &str = r#"
        SELECT * FROM type::table($table)
        WHERE username IS $username
        "#;

        // How it works?
        // SurrealDB::Error implements the Error trait.
        // anyhow::Error implements From<Error> and Rust converts for us
        let mut response: surrealdb::Response = self
            .client
            .query(query)
            .bind(("table", "general_user"))
            .bind(("username", username))
            .await?;

        let user: Option<GeneralUser> = response.take(0)?;
        Ok(user)
    }
}

#[tracing::instrument(
    name = "User Login"
    skip(db)
)]
pub async fn user_login(
    req: HttpRequest, // for tracing
    db: web::Data<Database>,
    user_info_ptr: web::Json<UserCredentials>,
) -> Result<HttpResponse, UserLoginError> {
    let user_data: UserCredentials = user_info_ptr.into_inner();
    // TODO: Validation of credentials
    let maybe_surreal_user: Option<GeneralUser> =
        db.get_user_by_username(user_data.username).await?;
    let surreal_user: GeneralUser = if let Some(user) = maybe_surreal_user {
        user
    } else {
        return Err(UserLoginError::AuthError(anyhow::anyhow!(
            "Did not find user"
        )));
    };

    let _ = verify_password_hash(user_data.password, surreal_user.password_hash.into())
        .context("Invalid password")?;

    // Setting Cookies
    //

    Ok(HttpResponse::ImATeapot().finish())
}
