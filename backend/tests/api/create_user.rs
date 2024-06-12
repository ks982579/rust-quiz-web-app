//! backend/tests/api/create_user.rs

use crate::utils::{spawn_app, TestApp};
use models::GeneralUser;
use reqwest::{Client, Response};

pub trait CreateUser<Body>
where
    Body: serde::Serialize,
{
    async fn post_create_user(&self, json: &Body) -> Response;
}

impl<Body> CreateUser<Body> for TestApp
where
    Body: serde::Serialize,
{
    async fn post_create_user(&self, json: &Body) -> Response {
        self.api_client
            .post(&format!("{}/create-user", &self.address))
            .json(json)
            .send()
            .await
            .expect("Failed to execute POST Request")
    }
}

#[tokio::test]
async fn test_create_user_200() {
    // Arrange
    let test_app: TestApp = spawn_app().await;
    // let client: Client = Client::new();

    let info: serde_json::Value = serde_json::json!({
        "name": "Joe Bob",
        "username": "joebob1234",
        "password": "Password1234"
    });

    // Act
    let response: Response = test_app.post_create_user(&info).await;

    // Assert
    dbg!(response.status());
    assert!(response.status().is_success());

    // Clean up
    test_app
        .database
        .client
        .query(
            r#"
            DELETE general_user
            WHERE username is 'joebob1234'
            "#,
        )
        .await;
}

/* Other Tests that shouldn't be too demanding:
* - incomplete information is rejected
* - username already taken
*/

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
