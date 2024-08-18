//! backend/tests/api/create_questions.rs
use crate::utils::{spawn_app, CreateQuestions, CreateQuiz, TestApp};
use models::{
    questions::{JsonQuestion, JsonQuestionMC, QuestionJsonPkg},
    quiz::SurrealQuiz,
};
use reqwest::Response;

#[tokio::test]
async fn test_create_question_201() {
    // Arrange
    let test_app: TestApp = spawn_app().await;
    test_app.cleanup_db().await;

    // Create User for testing
    let mut test_app_response = test_app.create_new_test_user().await;
    assert!(
        test_app_response.status().is_success(),
        "Failed to create new Test User"
    );
    test_app_response = test_app.log_in_test_user().await;
    assert!(
        test_app_response.status().is_success(),
        "Failed to log user in"
    );

    // Quiz Structure
    let info: serde_json::Value = serde_json::json!({
        "name": "Algorithms",
        "description": "An algorithms quiz"
    });

    let response: Response = test_app.post_create_quiz(&info).await;
    assert!(response.status().is_success(), "Failed to create Quiz");
    let quiz: SurrealQuiz = response.json().await.unwrap();

    let q1 = JsonQuestion::MultipleChoice(JsonQuestionMC {
        question: String::from(
            "Which sorting algorithm has an average and worst-case time complexity of O(n log(n))?",
        ),
        hint: Some(String::from(
            "This algorithm uses a divide-and-conquer strategy and is often implement recursively.",
        )),
        answer: String::from("Merge Sort"),
        choices: vec![
            String::from("Bubble Sort"),
            String::from("Quick Sort"),
            String::from("Selection Sort"),
        ],
    });

    let package: QuestionJsonPkg = QuestionJsonPkg {
        quiz_id: quiz.id.clone(),
        question: q1,
    };

    //Act
    let question_response: Response = test_app.post_create_questions(&package).await;

    // Assert
    assert!(question_response.status() == 201);
    test_app.cleanup_db().await;
}

#[tokio::test]
async fn test_create_questions_400() {
    // Arrange
    let test_app: TestApp = spawn_app().await;
    test_app.cleanup_db().await;

    // Create User for testing
    let mut test_app_response = test_app.create_new_test_user().await;
    assert!(
        test_app_response.status().is_success(),
        "Failed to create new Test User"
    );
    test_app_response = test_app.log_in_test_user().await;
    assert!(
        test_app_response.status().is_success(),
        "Failed to log user in"
    );

    // Quiz Structure
    let info: serde_json::Value = serde_json::json!({
        "name": "Algorithms",
        "description": "An algorithms quiz"
    });

    let response: Response = test_app.post_create_quiz(&info).await;
    assert!(response.status().is_success(), "Failed to create Quiz");
    let quiz: SurrealQuiz = response.json().await.unwrap();

    let q1 = JsonQuestion::MultipleChoice(JsonQuestionMC {
        question: String::from(""),
        hint: Some(String::from(
            "This algorithm uses a divide-and-conquer strategy and is often implement recursively.",
        )),
        answer: String::from("Merge Sort"),
        choices: vec![
            String::from("Bubble Sort"),
            String::from("Quick Sort"),
            String::from("Selection Sort"),
        ],
    });
    let package: QuestionJsonPkg = QuestionJsonPkg {
        quiz_id: quiz.id.clone(),
        question: q1,
    };

    //Act
    let question_response: Response = test_app.post_create_questions(&package).await;

    // Assert
    assert!(question_response.status().is_client_error());

    // Clean up
    test_app.cleanup_db().await;
}
