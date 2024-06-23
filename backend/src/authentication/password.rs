//! backend/src/authentication/password.rs
//! Passwords will use the Argon2 encryption method
use crate::{surrealdb_repo::Database, telemetry::spawn_blocking_and_tracing};
use actix_web::web;
use anyhow::Context;
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use rand::thread_rng;
use secrecy::{ExposeSecret, Secret};
use uuid::Uuid;
// trait for Database
use crate::routes::LookUpUser;
use serde::Deserialize;
use std::str::FromStr;

// Errors
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Credentials are invalid")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserCredentials {
    pub username: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "Validate Credentials", skip_all)]
pub async fn validate_credentials(
    credentials: UserCredentials,
    db: web::Data<Database>,
) -> Result<uuid::Uuid, AuthError> {
    // To keep return time consistent
    let mut db_user_id: Option<Uuid> = None;
    let mut expected_password_hash = Secret::new(
        "$argon2id$v=19$m=10000,t=2,p=1$\
        BOvW4laFSaAuhBGKyUq1lQ$H9mEowzY3Wj4vGRdnCzmzY15OGdlq64gytD+u/eOGrQ"
            .to_string(),
    );

    // fetch user data from database if it exists
    // Unless the database errors out, we want to hash password for consistent time.
    let db_user_opt: Option<models::GeneralUser> = db
        .get_user_by_username(credentials.username.clone())
        .await?;

    if let Some(gen_user) = db_user_opt {
        db_user_id = Some(Uuid::from_str(&gen_user.uuid).context("Failed to parse UUID")?);
        expected_password_hash = gen_user.password_hash.into();
    }

    spawn_blocking_and_tracing(move || {
        verify_password_hash(credentials.password, expected_password_hash)
    })
    .await
    .context("Failed to spawn blocking task")??;

    db_user_id
        .ok_or_else(|| anyhow::anyhow!("Invalid username"))
        .map_err(AuthError::InvalidCredentials)
}

/// Computing Password hash for storing safely
#[tracing::instrument(name = "Computing Password Hash", skip_all)]
pub fn create_password_hash(pswd: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
    // Create random salt to make password more secure
    let salt: SaltString = SaltString::generate(&mut thread_rng());

    // Hash password with default settings
    let pswd_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        // parameters are m_cost, t_cost, p_cost, output_len
        Params::new(10000, 2, 1, Some(32)).unwrap(),
    )
    .hash_password(pswd.expose_secret().as_bytes(), &salt)?
    .to_string();
    Ok(Secret::new(pswd_hash))
}

/// Function will raise an error if actual and expected passwords do not match.
pub fn verify_password_hash(
    actual_pswd_string: Secret<String>,
    expected_pswd_hash: Secret<String>,
) -> Result<(), AuthError> {
    // String to PasswordHash
    let expected_pswd_hash: PasswordHash = PasswordHash::new(expected_pswd_hash.expose_secret())
        .context("Failed to parse password hash from string to PHC string format.")?;

    Argon2::default()
        .verify_password(
            actual_pswd_string.expose_secret().as_bytes(),
            &expected_pswd_hash,
        )
        .context("Password provided is invalid")
        // Explicitly map error so it does not return unexpected varient.
        .map_err(AuthError::InvalidCredentials)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password_string: String = String::from("LuckyPassword1234!");
        let created_hash: Secret<String> =
            create_password_hash(Secret::new(password_string.clone())).unwrap();
        dbg!(&created_hash.expose_secret());
        // let pswd_hash = "$argon2id$v=19$m=10000,t=2,p=1$BOvW4laFSaAuhBGKyUq1lQ$H9mEowzY3Wj4vGRdnCzmzY15OGdlq64gytD+u/eOGrQ".to_string();
        // dbg!(ps.unwrap().expose_secret());
        let res = verify_password_hash(Secret::new(password_string), created_hash);
        dbg!(&res);
        let val: bool = if let Ok(_) = res { true } else { false };
        dbg!(val);
        assert!(val);
    }
}
