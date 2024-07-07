//! backend/tests/api/create_questions.rs
use std::future::Future;

use crate::utils::{spawn_app, TestApp};
use models::{
    questions::{JsonQuestion, JsonQuestionMC, QuestionJsonPkg, SurrealQuestionMC},
    quiz::SurrealQuiz,
    SurrealRecord,
};
use reqwest::Response;

trait CreateQuiz<Body>
where
    Body: serde::Serialize,
{
    async fn post_create_quiz(&self, json: &Body) -> Response;
}

impl<Body> CreateQuiz<Body> for TestApp
where
    Body: serde::Serialize,
{
    async fn post_create_quiz(&self, json: &Body) -> Response {
        self.api_client
            .post(&format!("{}/quiz-nexus", &self.address))
            .json(json)
            .send()
            .await
            .expect("Failed to execute POST Request")
    }
}

trait CreateQuestions<Body>
where
    Body: serde::Serialize,
{
    fn post_create_questions(&self, json: &Body) -> impl Future<Output = Response>;
}
impl<Body> CreateQuestions<Body> for TestApp
where
    Body: serde::Serialize,
{
    async fn post_create_questions(&self, json: &Body) -> Response {
        self.api_client
            .post(&format!("{}/question-forge", &self.address))
            .json(json)
            .send()
            .await
            .expect("Failed to execute POST Request")
    }
}

#[tokio::test]
async fn test_create_question_201() {
    // Arrange
    let test_app: TestApp = spawn_app().await;

    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
    let _: Vec<SurrealRecord> = test_app
        .database
        .client
        .delete("questions_mc")
        .await
        .unwrap();

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
    // let client: Client = Client::new();

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

    let mut package: QuestionJsonPkg = QuestionJsonPkg {
        quiz_id: quiz.id.clone(),
        question: q1,
    };

    // package
    //     .questions
    //     .push(JsonQuestion::MultipleChoice(JsonQuestionMC {
    //     question: String::from(
    //         "In Big O notation, which of the following represents the most efficient algorithm for large inputs?",
    //     ),
    //     hint: None,
    //     answer: String::from("O(log(n))"),
    //     choices: vec![
    //         String::from("O(n^2)"),
    //         String::from("O(n*log(n))"),
    //         String::from("O(n)"),
    //     ],
    // }));
    // package
    //     .questions
    //     .push(JsonQuestion::MultipleChoice(JsonQuestionMC {
    //     question: String::from(
    //         "Which data structure would be mostt efficient for implementing a priority queue?",
    //     ),
    //     hint: Some(String::from(
    //         "This data structure allows for efficient insertion and deletion of the highest (or lowest) priority element.",
    //     )),
    //     answer: String::from("Heap"),
    //     choices: vec![
    //         String::from("Array"),
    //         String::from("Linked List"),
    //         String::from("Binary Search Tree"),
    //     ],
    // }));

    //Act
    let question_response: Response = test_app.post_create_questions(&package).await;

    // Assert
    assert!(question_response.status() == 201);
    let this: SurrealQuestionMC = question_response.json().await.unwrap();
    dbg!(this);
    // TODO: Also want to compare `response.json().await` to database query.

    // Clean up
    // let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
    // let _: Vec<SurrealRecord> = test_app
    //     .database
    //     .client
    //     .delete("questions_mc")
    //     .await
    //     .unwrap();
}

#[tokio::test]
async fn test_create_questions_400() {
    // Arrange
    let test_app: TestApp = spawn_app().await;

    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
    let _: Vec<SurrealRecord> = test_app
        .database
        .client
        .delete("questions_mc")
        .await
        .unwrap();

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
    // let client: Client = Client::new();

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

    // package
    //     .questions
    //     .push(JsonQuestion::MultipleChoice(JsonQuestionMC {
    //     question: String::from(
    //         "In Big O notation, which of the following represents the most efficient algorithm for large inputs?",
    //     ),
    //     hint: None,
    //     answer: String::from("O(log(n))"),
    //     choices: vec![
    //         String::from("O(n^2)"),
    //         String::from("O(n*log(n))"),
    //         String::from("O(n)"),
    //     ],
    // }));
    // package
    //     .questions
    //     .push(JsonQuestion::MultipleChoice(JsonQuestionMC {
    //     question: String::from(
    //         "Which data structure would be mostt efficient for implementing a priority queue?",
    //     ),
    //     hint: Some(String::from(
    //         "This data structure allows for efficient insertion and deletion of the highest (or lowest) priority element.",
    //     )),
    //     answer: String::from("Heap"),
    //     choices: vec![
    //         String::from("Array"),
    //         String::from("Linked List"),
    //         String::from("Binary Search Tree"),
    //     ],
    // }));

    //Act
    let question_response: Response = test_app.post_create_questions(&package).await;

    // Assert
    assert!(question_response.status().is_client_error());

    // Clean up
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
    let _: Vec<SurrealRecord> = test_app
        .database
        .client
        .delete("questions_mc")
        .await
        .unwrap();
}
