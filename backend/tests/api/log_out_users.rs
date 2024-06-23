//! backend/tests/api/log_out_users.rs
use crate::utils::{spawn_app, TestApp};
use backend::surrealdb_repo::SessionToken;
// need new model
use reqwest::{cookie::Cookie, Client, Response};
use serde_json::Value;
use surrealdb::sql::Thing;

#[tokio::test]
async fn test_log_out_user_200() {
    // Arrange
    let test_app: TestApp = spawn_app().await;
    dbg!(String::from("Spawned test app"));

    dbg!(String::from("Clearing database"));
    // Clear out users
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("general_user").await;
    // Clear out session tokens
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("sessions").await;

    dbg!(String::from("Database cleared"));

    // Test User Data
    let user_data: Value = serde_json::json!({
        "name": "Test User",
        "username": "testuser123",
        "password": "Password@1234"
    });
    dbg!(String::from("JSON Test User"));

    dbg!("Trying to create test user");
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

    // Send Login Request
    let response: Response = test_app
        .api_client
        .post(&format!("{}/user-login", &test_app.address))
        .json(&login_data)
        .send()
        .await
        .expect("Failed to send login data");

    assert!(response.status().is_success());
    let browser_cookie: Vec<Cookie> = response.cookies().collect();
    assert!(browser_cookie.len() > 0);
    let db_token: Vec<SessionToken> = test_app.database.client.select("sessions").await.unwrap();
    dbg!(&db_token);
    assert!(db_token.len() > 0);

    // Act
    let log_out_response: Response = test_app
        .api_client
        .post(format!("{}/user-logout", &test_app.address))
        .send()
        .await
        .expect("Failed to send log out request");

    // Assert
    let logout_browser_cookies: Vec<Cookie> = log_out_response.cookies().collect();
    dbg!(&logout_browser_cookies);
    assert!(browser_cookie.len() == 0);
    let db_token: Vec<SessionToken> = test_app.database.client.select("sessions").await.unwrap();
    dbg!(&db_token);
    assert!(db_token.len() == 0);

    // Clean Up
    // TODO: Code duplication
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("general_user").await;

    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("sessions").await;
}
