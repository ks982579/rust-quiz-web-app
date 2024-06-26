//! backend/src/startup.rs
//! Holds application level information and functions.
use crate::{
    authentication::AuthCookie,
    configuration::AllSettings,
    routes::{check_login, create_user, health_check, user_login},
    surrealdb_repo::Database,
};
use actix_cors::Cors;
use actix_session::{config::PersistentSession, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key, SameSite},
    dev::Server,
    http::header,
    web, App, HttpServer,
};
use secrecy::{ExposeSecret, Secret};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Builds an Actix-Web Server, from `HttpServer::new()` provided a TcpListener.
/// Tracing is added, along with other middleware.
/// Other pieces of application state will also be included in the returned server.
pub async fn run(
    listener: TcpListener,
    database: Database,
    hmac_secret: Secret<String>,
) -> Result<Server, anyhow::Error> {
    // Wrap connection in Smart Pointer
    // ideally we want separate database for cookies, but should be OK for small project
    let db_connect: web::Data<Database> = web::Data::new(database);
    // Key for cookies
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());

    let server: Server = HttpServer::new(move || {
        App::new()
            // consider versioning like /api/v1/
            .wrap(
                SessionMiddleware::builder(db_connect.as_ref().clone(), secret_key.clone())
                    .cookie_http_only(true)
                    .cookie_name(String::from("sessionid"))
                    .cookie_same_site(SameSite::None)
                    .cookie_secure(true)
                    // .cookie_domain(Some(String::from("localhost:8000")))
                    .cookie_content_security(actix_session::config::CookieContentSecurity::Signed)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl_extension_policy(
                                actix_session::config::TtlExtensionPolicy::OnStateChanges,
                            )
                            .session_ttl(Duration::days(7)),
                    )
                    .build(),
            )
            .wrap(
                Cors::default()
                    // front-end URL
                    // .allowed_origin("http://localhost:8080")
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    // .allowed_header(header::CONTENT_TYPE)
                    // .allowed_headers(vec![
                    //     header::AUTHORIZATION,
                    //     header::ACCEPT,
                    //     header::CONTENT_TYPE,
                    //     header::ORIGIN,
                    //     header::WWW_AUTHENTICATE,
                    //     header::X_,
                    //     header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    // ])
                    // Allows inclusion of cookies and HTTP Authentication Info
                    .supports_credentials()
                    .max_age(3600),
            )
            // This checks if authorized
            .wrap(TracingLogger::default())
            .route("/health-check", web::get().to(health_check))
            .route("/create-user", web::post().to(create_user))
            .route("/user-login", web::post().to(user_login))
            .service(
                web::scope("")
                    // .wrap(SessionMiddleware::new(database.clone(), secret_key.clone()))
                    .wrap(AuthCookie)
                    .route("/check-login", web::get().to(check_login)),
            )
            // setting
            .app_data(
                web::JsonConfig::default().content_type(|_| "application/json".parse().unwrap()),
            )
            .app_data(db_connect.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

/// To hold necessary application level information.
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    /// Initialization for `Application` struct to set up application
    /// based on configuration setting from files or environment variables.
    pub async fn from_config(config: AllSettings) -> Result<Self, anyhow::Error> {
        // TODO: Set up proper configuration
        let database: Database = Database::from_config(config.database)
            .await // Result<Database, Error>
            .expect("Unable to Connect to Database");

        // Update port based on settings
        let address: String = format! {
            "{}:{}",
            // host address, 0.0.0.0 or 127.0.0.1
            config.application.host,
            // port
            config.application.port,
        };
        println!("Running on {:?}", &address);

        // Need listener to obtain randomly selected port
        let listener: TcpListener = TcpListener::bind(address)?;
        let port: u16 = listener.local_addr().unwrap().port();

        let server: Server = run(listener, database, config.application.hmac_secret).await?;

        Ok(Self { port, server })
    }

    /// Returns a copy of the application port, if needed in other parts of application.
    pub fn get_port(&self) -> u16 {
        self.port.clone()
    }

    /// Final method to consume the Application and return the running server.
    /// The error is specifically `std::io::Error`, cannot use `anyhow::Error`
    pub async fn run_until_stopped(self) -> std::io::Result<()> {
        self.server.await
    }
}
