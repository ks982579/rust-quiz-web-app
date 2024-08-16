//! backend/tests/api/create_quiz.rs
use crate::utils::{spawn_app, CreateQuiz, GetQuiz, TestApp};
use models::quiz::SurrealQuiz;
use reqwest::Response;

#[tokio::test]
async fn test_get_quiz_200() {
    // Arrange
    let test_app: TestApp = spawn_app().await;
    test_app.cleanup_db().await;

    // Create User
    let mut test_app_response = test_app.create_new_test_user().await;
    assert!(test_app_response.status().is_success());
    // Log User In
    test_app_response = test_app.log_in_test_user().await;
    assert!(test_app_response.status().is_success());

    // Make a quiz
    let quiz_info1: serde_json::Value = serde_json::json!({
        "name": "Algorithms",
        "description": "An algorithms quiz"
    });

    let response: Response = test_app.post_create_quiz(&quiz_info1).await;
    assert!(response.status().is_success());

    // Make a quiz
    let quiz_info2: serde_json::Value = serde_json::json!({
        "name": "Rust",
        "description": "A Rust quiz"
    });

    let response: Response = test_app.post_create_quiz(&quiz_info2).await;
    assert!(response.status().is_success());

    // Act
    let response: Response = test_app.get_quizzes().await;

    // Assert
    dbg!(&response);
    assert!(response.status().is_success());
    let actual: Vec<SurrealQuiz> = response.json().await.unwrap();
    assert!(actual.len() == 2);
    if actual[0].name == "Algorithms" {
        assert!(actual[0].name == "Algorithms");
        assert!(actual[1].name == "Rust");
    } else {
        assert!(actual[0].name == "Rust");
        assert!(actual[1].name == "Algorithms");
    }

    // Clean up
    test_app.cleanup_db().await;
}
