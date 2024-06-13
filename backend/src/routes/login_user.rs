//! backend/src/routes/login_user.rs
//! Endpoint to log user into system.
//! Must set the Session Token in browser as well.
use crate::authentication::UserCredentials;
use crate::{
    authentication::{validate_credentials, verify_password_hash, AuthError},
    error_chain_helper,
    session_wrapper::SessionWrapper,
    surrealdb_repo::Database,
};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web, HttpRequest, HttpResponse, ResponseError,
};
use anyhow::Context;
use models::GeneralUser;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use surrealdb::sql::Uuid;

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
// Compiler suggest not making public async trait...
pub trait LookUpUser {
    fn get_user_by_username(
        &self,
        username: String,
    ) -> impl std::future::Future<Output = Result<Option<GeneralUser>, anyhow::Error>> + Send;
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
    skip(db, session)
)]
pub async fn user_login(
    req: HttpRequest, // for tracing
    db: web::Data<Database>,
    user_info_ptr: web::Json<UserCredentials>,
    session: SessionWrapper,
) -> Result<HttpResponse, UserLoginError> {
    let user_data: UserCredentials = user_info_ptr.into_inner();

    match validate_credentials(user_data, db).await {
        Ok(user_uuid) => {
            tracing::Span::current().record("UUID", &tracing::field::display(&user_uuid));
            // Setting Cookies
            // Renew help prevent fixation attacks
            session.renew();
            session
                .insert_user_id(user_uuid)
                .map_err(|_| anyhow::anyhow!("Failed to insert user UUID"))?;
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => UserLoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => UserLoginError::UnexpectedError(e.into()),
            };
            return Err(e);
        }
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(serde_json::json!({"msg": "Login Successful"})))
}
