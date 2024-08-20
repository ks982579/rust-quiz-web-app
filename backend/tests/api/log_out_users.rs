//! backend/tests/api/log_out_users.rs
use crate::utils::{spawn_app, TestApp};
use backend::surrealdb_repo::SessionToken;
use reqwest::{cookie::Cookie, Response};
use serde_json::Value;
use std::time::Duration;
use surrealdb::sql::Thing;

#[tokio::test]
async fn test_log_out_logged_in_user_200() {
    // Arrange
    let test_app: TestApp = spawn_app().await;

    // Clear out users
    test_app.cleanup_db().await;

    // Test User Data
    let user_data: Value = serde_json::json!({
        "name": "Test User",
        "username": "testuser123",
        "password": "Password@1234"
    });

    // Creating User via API
    let _ = test_app
        .api_client
        .post(&format!("{}/v01/create-user", &test_app.address))
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
        .post(&format!("{}/v01/user-login", &test_app.address))
        .json(&login_data)
        .send()
        .await
        .expect("Failed to send login data");

    assert!(response.status().is_success());
    let browser_cookie: Vec<Cookie> = response.cookies().collect();
    assert!(browser_cookie.len() > 0);
    let db_token: Vec<SessionToken> = test_app.database.client.select("sessions").await.unwrap();
    assert!(db_token.len() > 0);

    // Act
    let log_out_response: Response = test_app
        .api_client
        .get(format!("{}/v01/user-logout", &test_app.address))
        .send()
        .await
        .expect("Failed to send log out request");

    // Assert
    let mut logout_browser_cookies: Vec<Cookie> = log_out_response.cookies().collect();
    assert!(logout_browser_cookies.len() == 1);
    let this_cookie: Cookie = logout_browser_cookies.remove(0);
    // `reqwest` does not delete cookies but shows you what the server sent
    // we can verify this as follows
    assert_eq!(
        &this_cookie.max_age(),
        &Some(Duration::from_secs(0)),
        "Max-Age should be 0"
    );
    if let Some(expiry) = this_cookie.expires() {
        assert!(
            expiry < std::time::SystemTime::now(),
            "Expire date should be in past"
        );
    }
    assert!(this_cookie.value().is_empty(), "Cookie value must be empty");
    dbg!(&logout_browser_cookies);
    let logout_db_token: Vec<SessionToken> =
        test_app.database.client.select("sessions").await.unwrap();
    dbg!(&logout_db_token);
    assert!(logout_db_token.len() == 0);

    // Clean Up
    // TODO: Code duplication
    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("general_user").await;

    let _: surrealdb::Result<Vec<Thing>> = test_app.database.client.delete("sessions").await;
}

#[tokio::test]
async fn test_log_out_anonymous_user_200() {
    // Arrange
    let test_app: TestApp = spawn_app().await;
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
        .post(&format!("{}/v01/create-user", &test_app.address))
        .json(&user_data)
        .send()
        .await
        .expect("Failed to create user");

    // Not logging user into application

    // Act
    let log_out_response: Response = test_app
        .api_client
        .get(format!("{}/v01/user-logout", &test_app.address))
        .send()
        .await
        .expect("Failed to send log out request");

    // Assert
    dbg!(&log_out_response);
    // Middleware catches this
    assert!(log_out_response.status().as_u16() == 401_u16);

    // Clean Up
    test_app.cleanup_db().await;
}
