//! frontend/src/store.rs
//! A container for global state set into context
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use leptos::*;

#[derive(Clone, Copy, Debug)]
pub struct AuthState {
    is_authenticated: RwSignal<bool>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            is_authenticated: create_rw_signal(false),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.is_authenticated.get()
    }

    pub fn set_authenticated(&self, authenticated: bool) {
        self.is_authenticated.set(authenticated);
    }
}

#[derive(Clone, Debug)]
pub struct AppSettings {
    pub backend_url: String,
}

impl AppSettings {
    pub fn init() -> Self {
        let mut settings = Self::load_from_file();
        Self::load_from_env(&mut settings);
        settings
    }

    /// Initializes the settings from .env file
    fn load_from_file() -> Self {
        let env_file: File = File::open(".env").expect("Failed to open .env file");
        let env_reader: BufReader<File> = BufReader::new(env_file);
        let mut tmp_settings: HashMap<String, String> = HashMap::new();

        for line in env_reader.lines() {
            if let Ok(line) = line {
                if !line.starts_with("#") && !line.is_empty() {
                    // using `split_once` in case "=" appears in value
                    if let Some((key, val)) = line.split_once("=") {
                        tmp_settings.insert(key.trim().to_string(), val.trim().to_string());
                    }
                }
            }
        }

        AppSettings {
            backend_url: tmp_settings.get("BACKEND_URL").cloned().unwrap_or_default(),
        }
    }

    /// Initialize from file first, then this mutates the settings in place.
    fn load_from_env(tmp_settings: &mut Self) {
        if let Ok(url) = std::env::var("APP__BACKEND_URL") {
            tmp_settings.backend_url = url.trim().to_string();
        }
    }
}
