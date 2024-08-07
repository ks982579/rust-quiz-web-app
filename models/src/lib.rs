use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealize_macro::Surrealize;

pub mod model_errors;
pub mod questions;
pub mod quiz;

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone)]
pub struct SurrealRecord {
    pub id: Thing,
}

/// Struct to how user UUID, to pass from middleware
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone)]
pub struct UserID(pub String);

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone, Surrealize)]
pub struct GeneralUser {
    pub uuid: String,
    pub name: String,
    pub username: String,
    pub password_hash: String,
}

impl GeneralUser {
    pub fn new(uuid: String, name: String, username: String, password_hash: String) -> Self {
        // Validation?
        Self {
            uuid,
            name,
            username,
            password_hash,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone)]
pub struct PartialUser {
    pub uuid: String,
    pub name: String,
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonMsg {
    pub msg: Option<String>,
}

impl std::default::Default for JsonMsg {
    fn default() -> Self {
        Self { msg: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_general_user() {
        // Assign
        let id = "1234-ABCD".to_string();
        let name = "Billy Joe".to_string();
        let username = String::from("billyjoe123");
        let password_hash = String::from("secret!");

        // Act
        let actual = GeneralUser::new(
            id.clone(),
            name.clone(),
            username.clone(),
            password_hash.clone(),
        );
        let expected = GeneralUser {
            uuid: id,
            name: name,
            username: username,
            password_hash: password_hash,
        };
        assert_eq!(actual, expected);
    }
}
