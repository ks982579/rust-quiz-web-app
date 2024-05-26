//! backend/src/startup.rs
//! Holds application level information and functions.
use crate::{configuration::Settings, routes::health_check};
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// To hold necessary application level information.
pub struct Application {
    port: u16,
    server: Server,
}

// pub struct Settings();

impl Application {
    /// Builder pattern for `Application` struct to set up application
    /// based on configuration setting from files or environment variables.
    pub async fn build(config: Settings) -> Result<Self, anyhow::Error> {
        // todo!();
        // TODO - SETUP DATABASE HERE

        // Update port based on settings
        let address: String = format! {
            "{}:{}",
            // host address, 0.0.0.0 or 127.0.0.1
            config.application.host,
            // port
            config.application.port,
        };

        // Need listener to obtain randomly selected port
        let listener: TcpListener = TcpListener::bind(address)?;
        let port: u16 = listener.local_addr().unwrap().port();

        let server: Server = run(listener).await?;

        Ok(Self { port, server })
    }

    /// Final method to consume the Application and return the running server.
    /// The error is specifically `std::io::Error`, cannot use `anyhow::Error`
    pub async fn run_until_stopped(self) -> std::io::Result<()> {
        self.server.await
    }
}

/// Builds an Actix-Web Server, from `HttpServer::new()` provided a TcpListener.
/// Tracing is added, along with other middleware.
/// Other pieces of application state will also be included in the returned server.
pub async fn run(listener: TcpListener) -> Result<Server, anyhow::Error> {
    let server: Server = HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health-check", web::get().to(health_check))
    })
    .listen(listener)?
    // .bind(("0.0.0.0", 8000))?
    .run();
    Ok(server)
}
