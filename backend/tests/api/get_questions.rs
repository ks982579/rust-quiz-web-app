//! backend/tests/api/create_questions.rs
use crate::utils::{spawn_app, CreateQuestions, CreateQuiz, GetQuestion, TestApp};
use models::{
    questions::{AllQuestions, JsonQuestion, JsonQuestionMC, QuestionJsonPkg},
    quiz::SurrealQuiz,
};
use reqwest::Response;

#[tokio::test]
async fn test_get_questions_200() {
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
    dbg!(&response);
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

    let package1: QuestionJsonPkg = QuestionJsonPkg {
        quiz_id: quiz.id.clone(),
        question: q1,
    };
    let question_response1: Response = test_app.post_create_questions(&package1).await;
    assert!(question_response1.status() == 201);

    let q2 = JsonQuestion::MultipleChoice(JsonQuestionMC {
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
    });

    let package2: QuestionJsonPkg = QuestionJsonPkg {
        quiz_id: quiz.id.clone(),
        question: q2,
    };
    let question_response2: Response = test_app.post_create_questions(&package2).await;
    assert!(question_response2.status() == 201);

    let q3 = JsonQuestion::MultipleChoice(JsonQuestionMC {
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
    });

    let package3: QuestionJsonPkg = QuestionJsonPkg {
        quiz_id: quiz.id.clone(),
        question: q3,
    };
    let question_response3: Response = test_app.post_create_questions(&package3).await;
    assert!(question_response3.status() == 201);
    let query_param: String = urlencoding::encode(&quiz.id.to_raw()).to_string();

    //Act
    let res: Response = test_app.get_questions(query_param).await;
    dbg!(&res);

    // Assert
    assert!(res.status() == 200);
    let everything: AllQuestions = res.json().await.unwrap();
    assert!(everything.mc.len() == 3);

    // Clean UP
    test_app.cleanup_db().await;
}
