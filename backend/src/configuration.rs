//! backend/src/configuration.rs
//! Logic to read configuration files and create structs to be used
//! throughout the rest of the application.
use config::ConfigError;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::path::PathBuf;

/// Possible runtimes for application
pub enum AppEnv {
    Local,
    Production,
}

impl AppEnv {
    pub fn as_str(&self) -> &str {
        match self {
            AppEnv::Local => "local",
            AppEnv::Production => "production",
        }
    }
}

impl TryFrom<String> for AppEnv {
    type Error = ConfigError;

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

#[derive(Deserialize, Debug, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
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
pub fn get_configuration() -> Result<Settings, ConfigError> {
    // Because we may be in workspace, must do quick look for right folder
    let mut base_path: PathBuf =
        std::env::current_dir().expect("Failed to determine current directory");
    // let dir_list: std::fs::ReadDir = match base_path.read_dir() {
    //     Ok(dir_iter) => dir_iter,
    //     Err(_) => {
    //         return Err(ConfigError::Message(
    //             "Error reading current directory".into(),
    //         ))
    //     }
    // };
    // if
    if !base_path.ends_with("backend") {
        base_path = base_path.join("backend");
    };
    let config_dir: PathBuf = base_path.join("configuration");

    if let false = config_dir.exists() {
        return Err(ConfigError::NotFound(
            "Unable to find configuration directory".into(),
        ));
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
                .prefix("_")
                .separator("__"),
        )
        .build()?;

    // Try deserialize values into struct
    settings.try_deserialize::<Settings>()
}
