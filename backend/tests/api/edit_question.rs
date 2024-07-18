//! backend/tests/api/edit_question.rs
use crate::utils::{spawn_app, CreateQuestions, CreateQuiz, EditQuestion, TestApp};
use models::{
    questions::{JsonQuestion, JsonQuestionMC, QuestionJsonPkg, SurrealQuestionMC},
    quiz::SurrealQuiz,
    SurrealRecord,
};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::Thing;

#[tokio::test]
async fn test_edit_question_200() {
    // Arrange
    let test_app: TestApp = spawn_app().await;

    // Clean database
    test_app.cleanup_db().await;

    // Create Test User
    let mut test_app_response = test_app.create_new_test_user().await;
    assert!(test_app_response.status().is_success());

    // Log in Said User
    test_app_response = test_app.log_in_test_user().await;
    assert!(test_app_response.status().is_success());

    // Quiz Structure
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
    let surreal_quest: SurrealQuestionMC = question_response.json().await.unwrap();

    // Setting up query param
    let query_param: String = urlencoding::encode(&surreal_quest.id.to_raw()).to_string();

    // Setting up Body of Put
    let updated_quest = JsonQuestion::MultipleChoice(JsonQuestionMC {
        question: String::from("Testing"),
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

    // Act
    let test_res: Response = test_app.edit_question(query_param, &updated_quest).await;
    assert!(test_res.status().as_u16() == 200);

    // Assert
    let actual_quizzes: Vec<SurrealQuiz> =
        test_app.database.client.select("quizzes").await.unwrap();
    assert!(1 == actual_quizzes.len());
    let actual_quests: Vec<SurrealQuestionMC> = test_app
        .database
        .client
        .select("questions_mc")
        .await
        .unwrap();
    assert!(
        1 == actual_quests.len(),
        "Something happened to the question"
    );

    assert!(actual_quests[0].question == "Testing", "Did not update");
    assert!(
        actual_quests[0].answer == "Merge Sort",
        "Other field incorrectly updated"
    );

    // clean up database
    test_app.cleanup_db().await;
}

/*
* TODO: Implement remaining tests:
*   400 -> weird question id
*   401 -> anonymous user cannot edit
*   403 -> the wrong user cannot edit
*/

// #[tokio::test]
// async fn test_edit_quiz_400() {
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
// async fn test_edit_quiz_401() {
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
