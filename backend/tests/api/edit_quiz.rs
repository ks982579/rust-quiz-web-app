//! backend/tests/api/edit_quiz.rs
use crate::utils::{spawn_app, CreateQuestions, CreateQuiz, EditQuiz, TestApp};
use models::{
    questions::{JsonQuestion, JsonQuestionMC, QuestionJsonPkg, SurrealQuestionMC},
    quiz::SurrealQuiz,
};
use reqwest::Response;

#[tokio::test]
async fn test_edit_quiz_200() {
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

    let package: QuestionJsonPkg = QuestionJsonPkg {
        quiz_id: quiz.id.clone(),
        question: q1,
    };
    let question_response: Response = test_app.post_create_questions(&package).await;
    assert!(question_response.status() == 201);

    // Setting up query param
    let query_param: String = urlencoding::encode(&quiz.id.to_raw()).to_string();

    // Setting up Body of Put
    let updated_info: serde_json::Value = serde_json::json!({
        "name": "Algorithms",
        "description": "testing edit"
    });

    // Act
    let test_res: Response = test_app.edit_quiz(query_param, &updated_info).await;
    assert!(test_res.status().as_u16() == 200);

    // Assert
    let actual: Vec<SurrealQuiz> = test_app.database.client.select("quizzes").await.unwrap();
    assert!(1 == actual.len());
    let actual_quest: Vec<SurrealQuestionMC> = test_app
        .database
        .client
        .select("questions_mc")
        .await
        .unwrap();
    assert!(
        1 == actual_quest.len(),
        "Something happened to the question"
    );

    assert!(
        actual[0].name == "Algorithms",
        "Name updated for some reason"
    );
    assert!(
        actual[0].description == "testing edit",
        "Description did not update correctly"
    );

    // clean up database
    test_app.cleanup_db().await;
}

/*
* TODO: Implement remaining tests:
*   400 -> weird quiz id
*   401 -> anonymous user cannot edit
*   403 -> the wrong user cannot edit
*/
