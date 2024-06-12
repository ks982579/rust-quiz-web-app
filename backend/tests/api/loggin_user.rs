//! backend/tests/api/loggin_user.rs
use crate::utils::{spawn_app, TestApp};
// need new model
use reqwest::{Client, Response};
use serde_json::Value;

#[tokio::test]
async fn test_log_in_user_200() {
    // Arrange
    let test_app: TestApp = spawn_app().await;

    // Clear out users
    let _ = test_app
        .database
        .client
        .query(
            r#"
            DELETE general_user
            "#,
        )
        .await;

    // Test User Data
    let user_data: Value = serde_json::json!({
        "name": "Test User",
        "username": "testuser123",
        "password": "Password@1234"
    });

    // Creating User via API
    let _ = test_app
        .api_client
        .post(&format!("{}/create-user", &test_app.address))
        .json(&user_data)
        .send()
        .await
        .expect("Failed to create user");

    let login_data: Value = serde_json::json!({
        "username": "Test User",
        "password": "Password@1234"
    });

    // Act
    // Send Login Request
    let response: Response = test_app
        .api_client
        .post(&format!("{}/user-login", &test_app.address))
        .json(&login_data)
        .send()
        .await
        .expect("Failed to send login data");

    // Assert
    assert!(response.status().is_success());
    // later check that token in database and stored in Cookies.

    // Clean Up
    // TODO: Code duplication
    let _ = test_app
        .database
        .client
        .query(
            r#"
            DELETE general_user
            "#,
        )
        .await;
}
