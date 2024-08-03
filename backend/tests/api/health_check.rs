//! backend/tests/api/health_check.rs

use crate::utils::{spawn_app, TestApp};
use reqwest::{Client, Response};

#[tokio::test]
async fn test_health_check_200() {
    // Arrange
    let test_app: TestApp = spawn_app().await;
    let client: Client = Client::new();

    // Act
    let response: Response = client
        .get(&format!("{}/v01/health-check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
}
