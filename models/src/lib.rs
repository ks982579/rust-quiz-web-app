use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone)]
pub struct GeneralUser {
    pub uuid: String,
    pub name: String,
    pub username: String,
    pub password: String,
}

impl GeneralUser {
    pub fn new(uuid: String, name: String, username: String, password: String) -> Self {
        // Validation?
        Self {
            uuid,
            name,
            username,
            password,
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
        let password = String::from("secret!");

        // Act
        let actual = GeneralUser::new(id.clone(), name.clone(), username.clone(), password.clone());
        let expected = GeneralUser {
            uuid: id,
            name: name,
            username: username,
            password: password,
        };
        assert_eq!(actual, expected);
    }
}
