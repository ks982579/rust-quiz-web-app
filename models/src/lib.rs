use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone)]
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
