//! backend/src/configuration.rs
//! Logic to read configuration files and create structs to be used
//! throughout the rest of the application.
use config::ConfigError;
use secrecy::Secret;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::path::PathBuf;

/// Possible runtimes for application
pub enum AppEnv {
    Local,
    Production,
}

impl AppEnv {
    /// Enum to `String` casting
    pub fn as_str(&self) -> &str {
        match self {
            AppEnv::Local => "local",
            AppEnv::Production => "production",
        }
    }
}

/// Implementation of Associated function on `AppEnv`
/// but gives us `.try_into::<AppEnv>()` method on string for free.
impl TryFrom<String> for AppEnv {
    type Error = ConfigError;

    /// Associated function to (try) cast `String` to `AppEnv` enum.
    fn try_from(env_str: String) -> Result<Self, Self::Error> {
        // convert input and compare
        match env_str.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            unknown => Err(ConfigError::NotFound(format!(
                "`{unknown}` is not a supported runtime. \
                    Current choices =['local', 'production']"
            ))),
        }
    }
}

/// Struct for holding information regarding serving the application.
#[derive(Deserialize, Debug, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub hmac_secret: Secret<String>,
}

/// Struct to hold information regarding the database
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub name: String,
}

/// Struct for holding all settings for a convenient means of passing
/// through application.
#[derive(Deserialize, Debug, Clone)]
pub struct AllSettings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

/// Function to read from configuration files and create a `Settings` struct
/// that can be used to set up the application.
/// Values in files can be overriden with environment variables following a
/// format of:
///   - Prefixed with "QUIZAPP"
///   - structs seperated by "_"
///   - fields separated by "__"
/// e.g.) QUIZAPP_APPLICATION__HOST=127.0.0.1
/// That will set the settings.application.port = "127.0.0.1"
pub fn get_configuration() -> Result<AllSettings, ConfigError> {
    // Because we may be in workspace, must do quick look for right folder
    let base_path: PathBuf =
        std::env::current_dir().expect("Failed to determine current directory");

    // Checks if directory is there
    // else checks if it is in backend directory
    // else errors out
    let config_dir = if let true = base_path.join("configuration").exists() {
        base_path.join("configuration")
    } else if let true = base_path.join("backend").join("configuration").exists() {
        base_path.join("backend").join("configuration")
    } else {
        return Err(ConfigError::NotFound(format!(
            "Cannot find 'configuration' directory from {:?}",
            base_path
        )));
    };

    if let false = config_dir.exists() {
        return Err(ConfigError::NotFound(format!(
            "{:?} - Does not exist",
            config_dir
        )));
    };

    // Detect selected environment or default to _local_
    let app_env: AppEnv = std::env::var("APP_ENVIRONMENT")
        // default to "local"
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse env var `APP_ENVIRONMENT`");

    // reference the proper environment file
    let env_file: String = format!("{}.yaml", app_env.as_str());

    // Initialise Configuration Reader.
    let settings: config::Config = config::Config::builder()
        // Adding _base_ configuation
        .add_source(config::File::from(config_dir.join("base.yaml")))
        .add_source(config::File::from(config_dir.join(env_file)))
        // Read from environment variables
        // being last allows to override config in files
        .add_source(
            config::Environment::with_prefix("QUIZAPP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    // Try deserialize values into struct
    println!("{:?}", &settings);
    settings.try_deserialize::<AllSettings>()
}
