//! backend/tests/api/utils.rs
//! To house utility functions for testing.
use backend::{
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

    TestApp {
        address: "127.0.0.1".into(),
        port: 8000,
        api_client: reqwest::Client::new(),
    }
}
