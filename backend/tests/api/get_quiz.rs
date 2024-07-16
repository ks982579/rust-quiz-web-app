//! backend/tests/api/create_quiz.rs
use crate::utils::{spawn_app, CreateQuiz, GetQuiz, TestApp};
use models::quiz::SurrealQuiz;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

// trait CreateQuiz<Body>
// where
//     Body: serde::Serialize,
// {
//     async fn post_create_quiz(&self, json: &Body) -> Response;
// }
//
// impl<Body> CreateQuiz<Body> for TestApp
// where
//     Body: serde::Serialize,
// {
//     async fn post_create_quiz(&self, json: &Body) -> Response {
//         self.api_client
//             .post(&format!("{}/quiz-nexus", &self.address))
//             .json(json)
//             .send()
//             .await
//             .expect("Failed to execute POST Request")
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
struct SurrealRecord {
    id: Thing,
}

#[tokio::test]
async fn test_get_quiz_200() {
    // Arrange
    let test_app: TestApp = spawn_app().await;

    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();

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
    // TODO: Also want to compare `response.json().await` to database query.

    // Clean up
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
}

// #[tokio::test]
// async fn test_create_quiz_400() {
//     // Arrange
//     let test_app: TestApp = spawn_app().await;
//
//     let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
//
//     let mut test_app_response = test_app.create_new_test_user().await;
//     assert!(test_app_response.status().is_success());
//     test_app_response = test_app.log_in_test_user().await;
//     assert!(test_app_response.status().is_success());
//
//     // Quiz Structure - Hopefully no questions starts and empty vector
//     let info: serde_json::Value = serde_json::json!({
//         "name": "  ",
//         "description": "A blank quiz"
//     });
//
//     // Act
//     let response: Response = test_app.post_create_quiz(&info).await;
//
//     // Assert
//     dbg!(&response);
//     assert!(response.status() == 400);
//
//     // Clean up
//     let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
// }
//
// #[tokio::test]
// async fn test_create_quiz_401() {
//     // Arrange
//     let test_app: TestApp = spawn_app().await;
//
//     let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
//
//     // Not Creating a User
//     // Quiz Structure
//     let info: serde_json::Value = serde_json::json!({
//         "name": "Algorithms",
//         "description": "An algorithms quiz"
//     });
//
//     // Act
//     let response: Response = test_app.post_create_quiz(&info).await;
//
//     // Assert
//     dbg!(&response);
//     assert!(response.status() == 401);
//
//     // Clean up
//     let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
// }
