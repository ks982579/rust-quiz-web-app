//! backend/tests/api/create_quiz.rs
use crate::utils::{spawn_app, CreateQuestions, CreateQuiz, DestroyQuiz, TestApp};
use models::{
    questions::{JsonQuestion, JsonQuestionMC, QuestionJsonPkg, SurrealQuestionMC},
    quiz::SurrealQuiz,
};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
struct SurrealRecord {
    id: Thing,
}

#[tokio::test]
async fn test_user_delete_quiz_200() {
    // -- Arrange
    let test_app: TestApp = spawn_app().await;

    // clean up database
    let _: Vec<SurrealQuiz> = test_app.database.client.delete("quizzes").await.unwrap();
    let _: Vec<SurrealQuestionMC> = test_app
        .database
        .client
        .delete("questions_mc")
        .await
        .unwrap();
    // Clear out users
    let _: Vec<Thing> = test_app
        .database
        .client
        .delete("general_user")
        .await
        .unwrap();
    // Clear out session tokens
    let _: Vec<Thing> = test_app.database.client.delete("sessions").await.unwrap();

    // create user for testing
    let mut test_app_response = test_app.create_new_test_user().await;
    assert!(test_app_response.status().is_success());
    // log in user
    test_app_response = test_app.log_in_test_user().await;
    assert!(test_app_response.status().is_success());

    // Quiz Structure - Hopefully no questions starts and empty vector
    let info: serde_json::Value = serde_json::json!({
        "name": "Algorithms",
        "description": "An algorithms quiz"
    });
    let response: Response = test_app.post_create_quiz(&info).await;
    assert!(response.status().is_success());
    let quiz: SurrealQuiz = response.json().await.unwrap();

    // Creating Question!
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
    let question_response: Response = test_app.post_create_questions(&package).await;
    assert!(question_response.status() == 201);

    // Setting up query param
    let query_param: String = urlencoding::encode(&quiz.id.to_raw()).to_string();

    // Act
    let test_res: Response = test_app.destroy_quiz(query_param).await;
    assert!(test_res.status().as_u16() == 200);

    // Assert
    let actual: Vec<SurrealQuiz> = test_app.database.client.select("quizzes").await.unwrap();
    assert!(1 > actual.len());
    let actual_quest: Vec<SurrealQuestionMC> = test_app
        .database
        .client
        .select("questions_mc")
        .await
        .unwrap();
    assert!(1 > actual_quest.len());

    // Clean up
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
}

#[tokio::test]
async fn test_anon_user_delete_quiz_401() {
    // -- Arrange
    let test_app: TestApp = spawn_app().await;

    // clean up database
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
    // Clear out users
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("general_user").await;
    // Clear out session tokens
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("sessions").await;

    // create user for testing
    let mut test_app_response = test_app.create_new_test_user().await;
    assert!(test_app_response.status().is_success());
    // log in user
    test_app_response = test_app.log_in_test_user().await;
    assert!(test_app_response.status().is_success());

    // Create Quiz as Test User
    let info: serde_json::Value = serde_json::json!({
        "name": "Algorithms",
        "description": "An algorithms quiz"
    });
    let response: Response = test_app.post_create_quiz(&info).await;
    assert!(response.status().is_success());
    let quiz: SurrealQuiz = response.json().await.unwrap();

    // Setting up query param
    let query_param: String = urlencoding::encode(&quiz.id.to_raw()).to_string();

    let log_out_response: Response = test_app
        .api_client
        .get(format!("{}/user-logout", &test_app.address))
        .send()
        .await
        .expect("Failed to send log out request");
    assert!(log_out_response.status().is_success());

    // Act
    let test_res: Response = test_app.destroy_quiz(query_param).await;
    assert!(test_res.status().as_u16() == 403);

    // Assert
    let actual: Vec<SurrealQuiz> = test_app.database.client.select("quizzes").await.unwrap();
    assert!(1 == actual.len());

    // clean up database
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
    // Clear out users
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("general_user").await;
    // Clear out session tokens
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("sessions").await;
}

#[tokio::test]
async fn test_other_user_delete_quiz_403() {
    // -- Arrange
    let test_app: TestApp = spawn_app().await;

    // clean up database
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
    // Clear out users
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("general_user").await;
    // Clear out session tokens
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("sessions").await;

    // Test User Data
    let user_data: Value = serde_json::json!({
        "name": "Dummy",
        "username": "dummy123",
        "password": "Password@1234"
    });

    // Creating User via API
    let response01: Response = test_app
        .api_client
        .post(&format!("{}/create-user", &test_app.address))
        .json(&user_data)
        .send()
        .await
        .expect("Failed to create user");
    assert!(response01.status().is_success());

    let login_data: Value = serde_json::json!({
        "username": "testuser123",
        "password": "Password@1234"
    });

    // Send Login Request
    let response02: Response = test_app
        .api_client
        .post(&format!("{}/user-login", &test_app.address))
        .json(&login_data)
        .send()
        .await
        .expect("Failed to send login data");
    assert!(response02.status().is_success());

    // Create Quiz as Dummy User
    let info: serde_json::Value = serde_json::json!({
        "name": "Algorithms",
        "description": "An algorithms quiz"
    });
    let response: Response = test_app.post_create_quiz(&info).await;
    assert!(response.status().is_success());
    let quiz: SurrealQuiz = response.json().await.unwrap();

    // Setting up query param
    let query_param: String = urlencoding::encode(&quiz.id.to_raw()).to_string();

    let log_out_response: Response = test_app
        .api_client
        .get(format!("{}/user-logout", &test_app.address))
        .send()
        .await
        .expect("Failed to send log out request");
    assert!(log_out_response.status().is_success());

    // create user for testing
    let mut test_app_response = test_app.create_new_test_user().await;
    assert!(test_app_response.status().is_success());
    // log in user
    test_app_response = test_app.log_in_test_user().await;
    assert!(test_app_response.status().is_success());

    // Act
    let test_res: Response = test_app.destroy_quiz(query_param).await;
    assert!(test_res.status().as_u16() == 403);

    // Assert
    let actual: Vec<SurrealQuiz> = test_app.database.client.select("quizzes").await.unwrap();
    assert!(1 == actual.len());

    // clean up database
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
    // Clear out users
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("general_user").await;
    // Clear out session tokens
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("sessions").await;
}

#[tokio::test]
async fn test_create_quiz_400() {
    // -- Arrange
    let test_app: TestApp = spawn_app().await;

    // clean up database
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
    // Clear out users
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("general_user").await;
    // Clear out session tokens
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("sessions").await;

    // create user for testing
    let mut test_app_response = test_app.create_new_test_user().await;
    assert!(test_app_response.status().is_success());
    // log in user
    test_app_response = test_app.log_in_test_user().await;
    assert!(test_app_response.status().is_success());

    // Quiz Structure - Hopefully no questions starts and empty vector
    let info: serde_json::Value = serde_json::json!({
        "name": "Algorithms",
        "description": "An algorithms quiz"
    });
    let response: Response = test_app.post_create_quiz(&info).await;
    assert!(response.status().is_success());
    // let quiz: SurrealQuiz = response.json().await.unwrap();

    // Setting up query param
    // let query_param: String = urlencoding::encode(&quiz.id.to_raw()).to_string();
    let query_param: String = urlencoding::encode("quizzes:not-real-id-123").to_string();

    // Act
    let test_res: Response = test_app.destroy_quiz(query_param).await;
    assert!(test_res.status().as_u16() == 200);

    // Assert
    let actual: Vec<SurrealQuiz> = test_app.database.client.select("quizzes").await.unwrap();
    assert!(1 > actual.len());

    // clean up database
    let _: Vec<SurrealRecord> = test_app.database.client.delete("quizzes").await.unwrap();
    // Clear out users
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("general_user").await;
    // Clear out session tokens
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("sessions").await;
}
