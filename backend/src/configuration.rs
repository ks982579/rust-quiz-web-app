//! backend/src/configuration.rs
//! Logic to read configuration files and create structs to be used
//! throughout the rest of the application.
use config::ConfigError;
use config::Environment;
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

pub fn get_configuration() -> Result<Settings, ConfigError> {
    // Might need check if running in "backend" or parent dir
    let base_path: PathBuf =
        std::env::current_dir().expect("Failed to determine current directory");
    let config_dir: PathBuf = base_path.join("configuration");

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
            config::Environment::with_prefix("QUIZ")
                .prefix("_")
                .separator("__"),
        )
        .build()?;

    // Try deserialize values into struct
    settings.try_deserialize::<Settings>()
}
