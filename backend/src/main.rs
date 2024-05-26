//! backend/src/main.rs
//! Entrypoint of the application where the Server is ran and
//! any parameters for said server are created
use backend::{
    startup::{Application, Settings},
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Setting up tracing subscriber
    let subscriber = get_subscriber("quiz-backend".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config: Settings = Settings();
    let application: Application = Application::build(config).await?;
    application.run_until_stopped().await?;
    Ok(())
}
