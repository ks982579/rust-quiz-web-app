//! backend/tests/api/create_quiz.rs
use crate::utils::{spawn_app, CreateQuiz, TestApp};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
struct SurrealRecord {
    id: Thing,
}

#[tokio::test]
async fn test_create_quiz_200() {
    // Arrange
    let test_app: TestApp = spawn_app().await;
    test_app.cleanup_db().await;

    let mut test_app_response = test_app.create_new_test_user().await;
    assert!(test_app_response.status().is_success());
    test_app_response = test_app.log_in_test_user().await;
    assert!(test_app_response.status().is_success());

    let info: serde_json::Value = serde_json::json!({
        "name": "Algorithms",
        "description": "An algorithms quiz"
    });

    for _ in 0..=5 {
        // Act
        let response: Response = test_app.post_create_quiz(&info).await;

        // Assert
        dbg!(&response);
        assert!(response.status().is_success());
    }

    // Clean up
    test_app.cleanup_db().await;
}

#[tokio::test]
async fn test_create_quiz_400() {
    // Arrange
    let test_app: TestApp = spawn_app().await;
    test_app.cleanup_db().await;

    let mut test_app_response = test_app.create_new_test_user().await;
    assert!(test_app_response.status().is_success());
    test_app_response = test_app.log_in_test_user().await;
    assert!(test_app_response.status().is_success());

    // Quiz Structure - Hopefully no questions starts and empty vector
    let info: serde_json::Value = serde_json::json!({
        "name": "  ",
        "description": "A blank quiz"
    });

    // Act
    let response: Response = test_app.post_create_quiz(&info).await;

    // Assert
    dbg!(&response);
    assert!(response.status() == 400);

    // Clean up
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
}

#[tokio::test]
async fn test_create_quiz_401() {
    // Arrange
    let test_app: TestApp = spawn_app().await;
    test_app.cleanup_db().await;

    // Not Creating a User
    // Quiz Structure
    let info: serde_json::Value = serde_json::json!({
        "name": "Algorithms",
        "description": "An algorithms quiz"
    });

    // Act
    let response: Response = test_app.post_create_quiz(&info).await;

    // Assert
    assert!(response.status() == 401);

    // Clean up
    test_app.cleanup_db().await;
}
