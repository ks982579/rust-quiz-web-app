//! backend/tests/api/create_user.rs

use crate::utils::{spawn_app, TestApp};
use reqwest::{Client, Response};

trait CreateUser<Body>
where
    Body: serde::Serialize,
{
    async fn post_create_user(&self, json: &Body) -> Response;
}

impl<Body> CreateUser<Body> for TestApp
where
    Body: serde::Serialize,
{
    async fn post_create_user(&self, json: &Body) -> Response {
        self.api_client
            .post(&format!("{}/create-user", &self.address))
            .json(json)
            .send()
            .await
            .expect("Failed to execute POST Request")
    }
}

#[tokio::test]
async fn test_create_user_200() {
    // Arrange
    let test_app: TestApp = spawn_app().await;
    // let client: Client = Client::new();

    let info: serde_json::Value = serde_json::json!({
        "name": "Joe Bob",
        "username": "joebob1234",
        "password": "Password1234"
    });

    // Act
    let response: Response = test_app.post_create_user(&info).await;

    // Assert
    dbg!(response.status());
    assert!(response.status().is_success());
}
