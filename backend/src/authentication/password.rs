//! backend/src/authentication/password.rs
//! Passwords will use the Argon2 encryption method
use anyhow::Context;
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use rand::thread_rng;
use secrecy::{ExposeSecret, Secret};

// Errors
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Credentials are invalid")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub struct UserCredentials {
    pub username: String,
    pub password: Secret<String>,
}

/// Computing Password hash for storing safely
#[tracing::instrument(name = "Computing Password Hash", skip_all)]
fn create_password_hash(pswd: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
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
fn verify_password_hash(
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
