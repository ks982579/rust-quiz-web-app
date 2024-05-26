//! backend/src/main.rs
//! Entrypoint of the application where the Server is ran and
//! any parameters for said server are created
use backend::{
    configuration::{get_configuration, Settings},
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

/// Async main function, entrypoint to program.
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Setting up tracing subscriber
    let subscriber = get_subscriber("quiz-backend".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // reading configuration
    let config: Settings = get_configuration().expect("Failed to read configuration");
    let application: Application = Application::from_config(config).await?;
    application.run_until_stopped().await?;
    Ok(())
}
