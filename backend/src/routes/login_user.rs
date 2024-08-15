//! backend/src/routes/login_user.rs
//! Endpoint to log user into system.
//! Must set the Session Token in browser as well.
use crate::authentication::UserCredentials;
use crate::{
    authentication::{validate_credentials, AuthError},
    error_chain_helper,
    session_wrapper::SessionWrapper,
    surrealdb_repo::{Database, LookUpUser},
};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web, HttpMessage, HttpRequest, HttpResponse, ResponseError,
};
use anyhow::Context;
use models::{GeneralUser, PartialUser, UserID};

#[derive(thiserror::Error)]
pub enum UserLoginError {
    #[error("Authentication Failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Unauthenticated")]
    Unauthorized(#[source] anyhow::Error),
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
                .json(serde_json::json!({ "msg": "Incorrect username or password" })),

            UserLoginError::Unauthorized(_) => HttpResponse::build(StatusCode::UNAUTHORIZED)
                .insert_header(ContentType::json())
                .json(serde_json::json!({ "msg": "User Unauthenticated" })),
        }
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

#[tracing::instrument(name = "Check If Logged In")]
pub async fn check_login(
    req: HttpRequest,
    db: web::Data<Database>,
) -> Result<HttpResponse, UserLoginError> {
    let user: Option<PartialUser> = if let Some(user_id) = req.extensions().get::<UserID>() {
        db.client
            .select(("general_user", &user_id.0))
            .await
            .context("Error")?
    } else {
        None
    };

    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(UserLoginError::Unauthorized(anyhow::anyhow!(
            "Unauthorized"
        ))),
    }
}
