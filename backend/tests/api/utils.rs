//! backend/tests/api/utils.rs
//! To house utility functions for testing.
use backend::{
    configuration::{get_configuration, AllSettings, ApplicationSettings},
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use reqwest::{Client, Response};
use std::sync::OnceLock;
use wiremock::MockServer;

// Can only be written to **ONCE**
static TRACING: OnceLock<()> = OnceLock::new();

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: Client,
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

    let application: Application = Application::from_config(configuration.clone())
        .await
        .expect("Failed to Build Application from Configuration");

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
    }
}
