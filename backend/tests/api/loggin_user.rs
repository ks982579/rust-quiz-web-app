//! backend/tests/api/loggin_user.rs
use crate::utils::{spawn_app, TestApp};
use backend::surrealdb_repo::SessionToken;
use reqwest::{cookie::Cookie, Response};
use serde_json::Value;
use surrealdb::sql::Thing;

#[tokio::test]
async fn test_log_in_user_200() {
    // Arrange
    let test_app: TestApp = spawn_app().await;
    // Clear out users
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("general_user").await;
    // Clear out session tokens
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("sessions").await;

    // Test User Data
    let user_data: Value = serde_json::json!({
        "name": "Test User",
        "username": "testuser123",
        "password": "Password@1234"
    });

    // Creating User via API
    let _ = test_app
        .api_client
        .post(&format!("{}/create-user", &test_app.address))
        .json(&user_data)
        .send()
        .await
        .expect("Failed to create user");

    let login_data: Value = serde_json::json!({
        "username": "testuser123",
        "password": "Password@1234"
    });

    // Act
    // Send Login Request
    let response: Response = test_app
        .api_client
        .post(&format!("{}/user-login", &test_app.address))
        .json(&login_data)
        .send()
        .await
        .expect("Failed to send login data");

    // Assert
    assert!(response.status().is_success());
    let browser_cookie: Vec<Cookie> = response.cookies().collect();
    assert!(browser_cookie.len() > 0);
    // Must Explicitly return SessionToken because it declares an ID
    let db_token: Vec<SessionToken> = test_app.database.client.select("sessions").await.unwrap();
    assert!(db_token.len() > 0);

    // Clean Up
    // TODO: Code duplication
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("general_user").await;

    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("sessions").await;
}

#[tokio::test]
async fn test_log_in_user_400() {
    // Arrange
    let test_app: TestApp = spawn_app().await;
    // Clear out users
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("general_user").await;
    // Clear out session tokens
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("sessions").await;

    // Create Test User
    let user_data: Value = serde_json::json!({
        "name": "Test User",
        "username": "testuser123",
        "password": "Password@1234"
    });
    // Creating User via API
    let _ = test_app
        .api_client
        .post(&format!("{}/create-user", &test_app.address))
        .json(&user_data)
        .send()
        .await
        .expect("Failed to create user");

    let missing_username: serde_json::Value = serde_json::json!({
        "username": "",
        "password": "Password1234"
    });
    let missing_password: serde_json::Value = serde_json::json!({
        "username": "joebob1234",
        "password": ""
    });
    let incorrect_username: serde_json::Value = serde_json::json!({
        "name": "Joe Bob",
        "username": "joebob1234",
        "password": "12345"
    });
    let incorrect_password: serde_json::Value = serde_json::json!({
        "name": "Joe Bob",
        "username": "joebob1234",
        "password": "12345"
    });

    let test_cases: Vec<(serde_json::Value, &str)> = vec![
        (missing_username, "Missing username"),
        (missing_password, "Missing password"),
        (incorrect_username, "incorrect username"),
        (incorrect_password, "incorrect password"),
    ];

    for (bad_data, err_msg) in test_cases {
        // Act
        // could probably life into a trait
        let response: Response = test_app
            .api_client
            .post(&format!("{}/user-login", &test_app.address))
            .json(&bad_data)
            .send()
            .await
            .expect("Failed to send login data");

        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with 400 when payload was {}.",
            err_msg
        );
        let browser_cookie: Vec<Cookie> = response.cookies().collect();
        assert!(browser_cookie.len() == 0);
        let db_token: Vec<SessionToken> =
            test_app.database.client.select("sessions").await.unwrap();
        assert!(db_token.len() == 0);
    }

    // Clean Up
    // TODO: Code duplication
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("general_user").await;

    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("sessions").await;
}
