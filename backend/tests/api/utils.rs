//! backend/tests/api/utils.rs
//! To house utility functions for testing.
use backend::{
    configuration::{get_configuration, AllSettings, ApplicationSettings},
    startup::Application,
    surrealdb_repo::Database,
    telemetry::{get_subscriber, init_subscriber},
};
use reqwest::{cookie::Cookie, Client, Response};
use serde_json::Value;
use std::future::Future;
use std::sync::OnceLock;
use surrealdb::sql::Thing;
use wiremock::MockServer;

// Can only be written to **ONCE**
static TRACING: OnceLock<()> = OnceLock::new();

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: Client,
    pub database: Database,
}

pub trait CreateQuiz<Body>
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

pub trait GetQuiz {
    async fn get_quizzes(&self) -> Response;
}

impl GetQuiz for TestApp {
    async fn get_quizzes(&self) -> Response {
        self.api_client
            .get(&format!("{}/quiz-nexus", &self.address))
            .send()
            .await
            .expect("Failed to execute GET Request")
    }
}

pub trait DestroyQuiz {
    async fn destroy_quiz(&self, quiz_id: String) -> Response;
}

impl DestroyQuiz for TestApp {
    async fn destroy_quiz(&self, quiz_id: String) -> Response {
        self.api_client
            .delete(&format!("{}/quiz-nexus?quiz={}", &self.address, quiz_id))
            .send()
            .await
            .expect("Failed to execute GET Request")
    }
}

pub trait CreateQuestions<Body>
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

pub trait GetQuestion {
    fn get_questions(&self, quiz_id: String) -> impl Future<Output = Response>;
}

impl GetQuestion for TestApp {
    async fn get_questions(&self, quiz_id: String) -> Response {
        self.api_client
            .get(&format!(
                "{}/question-forge?quiz={}",
                &self.address, quiz_id
            ))
            .send()
            .await
            .expect("Failed to execute GET Request")
    }
}

pub trait DestroyQuestion {
    fn destroy_question(&self, quest_id: String) -> impl Future<Output = Response>;
}

impl DestroyQuestion for TestApp {
    async fn destroy_question(&self, quest_id: String) -> Response {
        self.api_client
            .get(&format!(
                "{}/question-forge?quest={}",
                &self.address, quest_id
            ))
            .send()
            .await
            .expect("Failed to execute GET Request")
    }
}

/// Some helper function for the `TestApp`
/// Be sure to initialize an instance with `spawn_app()` before using these methods.
impl TestApp {
    // TODO: Maybe make into Builder like function to take in credentials or default
    /// Assuming user not created, Cleans out test database and creates a new test user.
    pub async fn create_new_test_user(&self) -> Response {
        // Clear out users
        let _: surrealdb::Result<Vec<Thing>> = self.database.client.delete("general_user").await;
        // Clear out session tokens
        let _: surrealdb::Result<Vec<Thing>> = self.database.client.delete("sessions").await;

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
        self.api_client
            .post(&format!("{}/create-user", &self.address))
            .json(&user_data)
            .send()
            .await
            .expect("Failed to create user")
    }

    /// Assuming user is created in Database, Helper method attempts to log user into application
    pub async fn log_in_test_user(&self) -> Response {
        let login_data: Value = serde_json::json!({
            "username": "testuser123",
            "password": "Password@1234"
        });

        self.api_client
            .post(&format!("{}/user-login", &self.address))
            .json(&login_data)
            .send()
            .await
            .expect("Failed to send login data")
    }
}

/// Setup function for the Test Application
/// Env var $TEST_LOG=true can send logs to standard out.
pub async fn spawn_app() -> TestApp {
    dbg!("Starting Spawn App");
    // Initiate the global Logger and Subscriber
    TRACING.get_or_init(|| {
        if std::env::var("TEST_LOG").is_ok() {
            let subscriber = get_subscriber("test".into(), "info".into(), std::io::stdout);
            init_subscriber(subscriber);
        } else {
            // `std::io::sink` is a writer that consumes all data.
            let subscriber = get_subscriber("test".into(), "info".into(), std::io::sink);
            init_subscriber(subscriber);
        }
    });

    // Get App Configurations
    let mut configuration: AllSettings =
        get_configuration().expect("Failed to Read Configuration File(s)");

    // Radomize OS Port
    configuration.application.port = 0;

    // Set database name space to testing
    configuration.database.namespace = String::from("testing");

    let application: Application = Application::from_config(configuration.clone())
        .await
        .expect("Failed to Build Application from Configuration");

    // Seperate Database Connection?
    let database: Database = Database::from_config(configuration.database.clone())
        .await
        .expect("Error connecting to database again");

    // obtain random application port
    let application_port: u16 = application.get_port();
    dbg!(application_port);

    dbg!("Starting Application");
    let _ = tokio::spawn(application.run_until_stopped());

    let client: Client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    TestApp {
        address: format!("http://127.0.0.1:{}", application_port),
        port: application_port,
        api_client: client,
        database,
    }
}
