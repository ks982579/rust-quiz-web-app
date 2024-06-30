//! backend/tests/api/create_questions.rs
use std::future::Future;

use crate::utils::{spawn_app, TestApp};
use models::{
    questions::{JsonQuestion, JsonQuestionMC, QuestionJsonPkg, QuestionMC, SurrealQuestionMC},
    quiz::SurrealQuiz,
    SurrealRecord,
};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Output, Thing};

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
async fn test_create_questions_200() {
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

    let mut package: QuestionJsonPkg = QuestionJsonPkg {
        quiz_id: quiz.id.clone(),
        questions: Vec::new(),
    };

    // Loading Questions
    package
        .questions
        .push(JsonQuestion::MultipleChoice(JsonQuestionMC {
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
    }));
    package
        .questions
        .push(JsonQuestion::MultipleChoice(JsonQuestionMC {
        question: String::from(
            "In Big O notation, which of the following represents the most efficient algorithm for large inputs?",
        ),
        hint: None,
        answer: String::from("O(log(n))"),
        choices: vec![
            String::from("O(n^2)"),
            String::from("O(n*log(n))"),
            String::from("O(n)"),
        ],
    }));
    package
        .questions
        .push(JsonQuestion::MultipleChoice(JsonQuestionMC {
        question: String::from(
            "Which data structure would be mostt efficient for implementing a priority queue?",
        ),
        hint: Some(String::from(
            "This data structure allows for efficient insertion and deletion of the highest (or lowest) priority element.",
        )),
        answer: String::from("Heap"),
        choices: vec![
            String::from("Array"),
            String::from("Linked List"),
            String::from("Binary Search Tree"),
        ],
    }));

    //Act
    let question_response: Response = test_app.post_create_questions(&package).await;
    // Assert
    assert!(question_response.status().is_success());
    // TODO: Also want to compare `response.json().await` to database query.

    // assert!(false);
    // Clean up
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
    let _: Vec<SurrealRecord> = test_app
        .database
        .client
        .delete("questions_mc")
        .await
        .unwrap();
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

    let mut package: QuestionJsonPkg = QuestionJsonPkg {
        quiz_id: quiz.id.clone(),
        questions: Vec::new(),
    };

    // Loading Questions
    package
        .questions
        .push(JsonQuestion::MultipleChoice(JsonQuestionMC {
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
    }));
    package
        .questions
        .push(JsonQuestion::MultipleChoice(JsonQuestionMC {
        question: String::from(
            "In Big O notation, which of the following represents the most efficient algorithm for large inputs?",
        ),
        hint: None,
        answer: String::from("O(log(n))"),
        choices: vec![
            String::from("O(n^2)"),
            String::from("O(n*log(n))"),
            String::from("O(n)"),
        ],
    }));
    package
        .questions
        .push(JsonQuestion::MultipleChoice(JsonQuestionMC {
        question: String::from(
            "Which data structure would be mostt efficient for implementing a priority queue?",
        ),
        hint: Some(String::from(
            "This data structure allows for efficient insertion and deletion of the highest (or lowest) priority element.",
        )),
        answer: String::from("Heap"),
        choices: vec![
            String::from("Array"),
            String::from("Linked List"),
            String::from("Binary Search Tree"),
        ],
    }));

    //Act
    let question_response: Response = test_app.post_create_questions(&package).await;
    // Assert
    assert!(question_response.status().is_client_error());
    // TODO: Also want to compare `response.json().await` to database query.

    // assert!(false);
    // Clean up
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
    let _: Vec<SurrealRecord> = test_app
        .database
        .client
        .delete("questions_mc")
        .await
        .unwrap();
}
/*
#[tokio::test]
async fn test_create_quiz_400() {
    // Arrange
    let test_app: TestApp = spawn_app().await;

    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();

    let mut test_app_response = test_app.create_new_test_user().await;
    assert!(test_app_response.status().is_success());
    test_app_response = test_app.log_in_test_user().await;
    assert!(test_app_response.status().is_success());
    // let client: Client = Client::new();

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
    // TODO: Also want to compare `response.json().await` to database query.

    // assert!(false);
    // Clean up
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
}

#[tokio::test]
async fn test_create_quiz_401() {
    // Arrange
    let test_app: TestApp = spawn_app().await;

    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();

    // Not Creating a User
    // let mut test_app_response = test_app.create_new_test_user().await;
    // assert!(test_app_response.status().is_success());
    // test_app_response = test_app.log_in_test_user().await;
    // assert!(test_app_response.status().is_success());
    // let client: Client = Client::new();

    // Quiz Structure - Hopefully no questions starts and empty vector
    let info: serde_json::Value = serde_json::json!({
        "name": "Algorithms",
        "description": "An algorithms quiz"
    });

    // Act
    let response: Response = test_app.post_create_quiz(&info).await;

    // Assert
    dbg!(&response);
    assert!(response.status() == 401);
    // TODO: Also want to compare `response.json().await` to database query.

    // Clean up
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
}*/
/* Other Tests that shouldn't be too demanding:
* - incomplete information is rejected
* - username already taken
*/

/*
#[tokio::test]
async fn test_create_user_400_incomplete_data() {
    // Arrange
    let test_app: TestApp = spawn_app().await;

    // Clean database first
    let _: surrealdb::Result<Vec<GeneralUser>> =
        test_app.database.client.delete("general_user").await;

    let missing_name: serde_json::Value = serde_json::json!({
        "name": "",
        "username": "joebob1234",
        "password": "Password1234"
    });
    let missing_username: serde_json::Value = serde_json::json!({
        "name": "Joe Bob",
        "username": "",
        "password": "Password1234"
    });
    let missing_password: serde_json::Value = serde_json::json!({
        "name": "Joe Bob",
        "username": "joebob1234",
        "password": ""
    });
    let short_password: serde_json::Value = serde_json::json!({
        "name": "Joe Bob",
        "username": "joebob1234",
        "password": "12345"
    });

    let test_cases: Vec<(serde_json::Value, &str)> = vec![
        (missing_name, "Missing person name"),
        (missing_username, "Missing username"),
        (missing_password, "Missing password"),
        (short_password, "Password under 6 characters"),
    ];

    for (bad_data, err_msg) in test_cases {
        // Act
        let response: Response = test_app.post_create_user(&bad_data).await;

        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with 400 when payload was {}.",
            err_msg
        );
    }

    let qry = r#"
    SELECT count() FROM general_user
    "#;
    let mut response_res = test_app.database.client.query(qry).await;
    let count = if let Ok(mut surreal_res) = response_res {
        if let Ok(gen_user_cnt_opt) = surreal_res.take(0) {
            if let Some(gen_user_count) = gen_user_cnt_opt {
                gen_user_count
            } else {
                0
            }
        } else {
            42
        }
    } else {
        42
    };
    assert_eq!(count, 0);
}
*/
